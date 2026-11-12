use std::{
    sync::{
        LazyLock,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

use crate::FrameDemand;
use anyhow::{Context, Result};
use gpui_util::ResultExt;
use windows::Win32::{
    Foundation::HWND,
    Graphics::Dwm::{DWM_TIMING_INFO, DwmFlush, DwmGetCompositionTimingInfo},
    System::Performance::QueryPerformanceFrequency,
};

static QPC_TICKS_PER_SECOND: LazyLock<u64> = LazyLock::new(|| {
    let mut frequency = 0;
    // On systems that run Windows XP or later, the function will always succeed and
    // will thus never return zero.
    unsafe { QueryPerformanceFrequency(&mut frequency).unwrap() };
    frequency as u64
});

const VSYNC_INTERVAL_THRESHOLD: Duration = Duration::from_millis(1);
const DEFAULT_VSYNC_INTERVAL: Duration = Duration::from_micros(16_666); // ~60Hz

impl FrameDemand {
    pub(crate) fn request(&self, hwnd: HWND) {
        self.request_window(hwnd.0 as isize);
    }
}

pub(crate) struct VSyncProvider {
    interval: Duration,
    f: Box<dyn Fn() -> bool>,
    consecutive_fast_waits: AtomicUsize,
}

impl VSyncProvider {
    pub(crate) fn new() -> Self {
        let interval = get_dwm_interval()
            .context("Failed to get DWM interval")
            .log_err()
            .unwrap_or(DEFAULT_VSYNC_INTERVAL);
        let f = Box::new(|| unsafe { DwmFlush().is_ok() });
        Self {
            interval,
            f,
            consecutive_fast_waits: AtomicUsize::new(0),
        }
    }

    pub(crate) fn wait_for_vsync(&self) {
        let vsync_start = Instant::now();
        let wait_succeeded = (self.f)();
        let elapsed = vsync_start.elapsed();

        if !wait_succeeded {
            std::thread::sleep(self.interval);
            self.consecutive_fast_waits.store(0, Ordering::Relaxed);
            return;
        }

        if elapsed < VSYNC_INTERVAL_THRESHOLD {
            let count = self.consecutive_fast_waits.fetch_add(1, Ordering::Relaxed) + 1;
            // When DwmFlush returns immediately because we caught the beginning of the
            // compositor's refresh cycle, do not inject an artificial 16ms sleep.
            // Only back off with a sleep if DwmFlush returns immediately consecutively
            // (e.g. desktop occlusion or monitor sleep).
            if count >= 2 {
                std::thread::sleep(self.interval);
            }
        } else {
            self.consecutive_fast_waits.store(0, Ordering::Relaxed);
        }
    }
}

fn get_dwm_interval() -> Result<Duration> {
    let mut timing_info = DWM_TIMING_INFO {
        cbSize: std::mem::size_of::<DWM_TIMING_INFO>() as u32,
        ..Default::default()
    };
    if unsafe { DwmGetCompositionTimingInfo(HWND::default(), &mut timing_info) }.is_ok() {
        let interval = retrieve_duration(timing_info.qpcRefreshPeriod, *QPC_TICKS_PER_SECOND);
        if interval >= VSYNC_INTERVAL_THRESHOLD {
            return Ok(interval);
        }
        if timing_info.rateRefresh.uiNumerator > 0 && timing_info.rateRefresh.uiDenominator > 0 {
            let hz = timing_info.rateRefresh.uiNumerator as u64
                / timing_info.rateRefresh.uiDenominator as u64;
            if hz > 0 {
                return Ok(Duration::from_nanos(1_000_000_000 / hz));
            }
        }
    }

    // Fall back to querying the primary display's actual refresh rate via EnumDisplaySettingsW
    let mut devmode = windows::Win32::Graphics::Gdi::DEVMODEW::default();
    devmode.dmSize = std::mem::size_of::<windows::Win32::Graphics::Gdi::DEVMODEW>() as u16;
    if unsafe {
        windows::Win32::Graphics::Gdi::EnumDisplaySettingsW(
            None,
            windows::Win32::Graphics::Gdi::ENUM_CURRENT_SETTINGS,
            &mut devmode,
        )
    }
    .as_bool()
    {
        if devmode.dmDisplayFrequency > 0 {
            let hz = devmode.dmDisplayFrequency as u64;
            log::info!("Detected display refresh rate via EnumDisplaySettings: {} Hz", hz);
            return Ok(Duration::from_nanos(1_000_000_000 / hz));
        }
    }

    Ok(DEFAULT_VSYNC_INTERVAL)
}

#[inline]
fn retrieve_duration(counts: u64, ticks_per_second: u64) -> Duration {
    let ticks_per_microsecond = ticks_per_second / 1_000_000;
    Duration::from_micros(counts / ticks_per_microsecond)
}
