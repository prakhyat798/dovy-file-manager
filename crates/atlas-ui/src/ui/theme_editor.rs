//! Custom theme copies are persisted independently of bundled presets.
use crate::app::App;
use gpui::prelude::*;
use gpui::{
    div, px, AnyElement, ColorExt as _, Context, Entity, Hsla, MouseButton, SharedString,
    Subscription, Window,
};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::color_picker::{ColorPicker, ColorPickerEvent, ColorPickerState};
use gpui_component::StyledExt as _;
use gpui_component::{
    h_flex, v_flex, ActiveTheme, Colorize, Disableable, Sizable, ThemeConfig, ThemeRegistry,
    ThemeSet,
};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Clone, Copy)]
enum Role {
    Accent,
    Background,
    Foreground,
}

impl Role {
    fn label(self) -> String {
        rust_i18n::t!(match self {
            Self::Accent => "theme_color_accent",
            Self::Background => "theme_color_background",
            Self::Foreground => "theme_color_foreground",
        })
        .to_string()
    }
    fn get(self, config: &ThemeConfig) -> Option<&SharedString> {
        match self {
            Self::Accent => config.colors.accent.as_ref(),
            Self::Background => config.colors.background.as_ref(),
            Self::Foreground => config.colors.foreground.as_ref(),
        }
    }
    fn set(self, config: &mut ThemeConfig, color: Hsla) {
        let value = Some(color.to_hex().into());
        match self {
            Self::Accent => config.colors.accent = value,
            Self::Background => config.colors.background = value,
            Self::Foreground => config.colors.foreground = value,
        }
    }
}

pub(crate) struct ThemeEditor {
    config: ThemeConfig,
    file_name: String,
    controls: Vec<(Role, Entity<ColorPickerState>)>,
    dirty: bool,
    error: Option<String>,
    _subscriptions: Vec<Subscription>,
}

fn fork_config(base: &ThemeConfig, existing: impl Fn(&str) -> bool) -> ThemeConfig {
    let (stem, suffix) = base
        .name
        .strip_suffix(" Glass")
        .map_or((base.name.as_ref(), ""), |stem| (stem, " Glass"));
    let mut number = 1u64;
    let name = loop {
        let candidate = if number == 1 {
            format!("{stem} Copy{suffix}")
        } else {
            format!("{stem} Copy {number}{suffix}")
        };
        if !existing(&candidate) {
            break candidate;
        }
        number += 1;
    };
    let mut config = base.clone();
    config.name = name.into();
    config.is_default = false;
    config
}

fn theme_set(config: &ThemeConfig) -> ThemeSet {
    ThemeSet {
        name: config.name.clone(),
        themes: vec![config.clone()],
        ..ThemeSet::default()
    }
}

impl App {
    fn fork_active_theme(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let registry = ThemeRegistry::global(cx);
        let Some(base) = registry
            .themes()
            .get(self.state.profiles.active().theme.as_str())
        else {
            self.theme_fork_error = Some(rust_i18n::t!("theme_fork_unavailable").to_string());
            cx.notify();
            return;
        };
        let config = fork_config(base, |name| registry.themes().contains_key(name));
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let file_name = format!(
            "custom-{timestamp:x}-{}-{}.json",
            std::process::id(),
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
        if let Err(error) =
            atlas_core::util::config::save_custom_theme(&file_name, &theme_set(&config))
        {
            self.theme_fork_error =
                Some(rust_i18n::t!("theme_save_error", error = error.to_string()).to_string());
            cx.notify();
            return;
        }
        let mut editor = ThemeEditor {
            config: config.clone(),
            file_name,
            controls: Vec::new(),
            dirty: false,
            error: None,
            _subscriptions: Vec::new(),
        };
        for role in [Role::Accent, Role::Background, Role::Foreground] {
            let fallback = match role {
                Role::Accent => cx.theme().accent,
                Role::Background => cx.theme().background,
                Role::Foreground => cx.theme().foreground,
            };
            let color = role
                .get(&config)
                .and_then(|value| Hsla::parse_hex(value).ok())
                .unwrap_or(fallback);
            let animate = !self.state.settings.disable_animations;
            let control = cx.new(|cx| {
                ColorPickerState::new(window, cx)
                    .default_value(color)
                    .default_angular()
                    .angular_animation(animate)
            });
            editor._subscriptions.push(cx.subscribe(
                &control,
                move |this, _, event: &ColorPickerEvent, cx| {
                    if let ColorPickerEvent::Change(Some(color)) = event {
                        if let Some(editor) = &mut this.theme_editor {
                            role.set(&mut editor.config, *color);
                            editor.dirty = true;
                            editor.error = None;
                        }
                        cx.notify();
                    }
                },
            ));
            editor.controls.push((role, control));
        }
        ThemeRegistry::global_mut(cx).register_custom_theme(config.clone());
        self.set_theme(config.name.to_string());
        self.theme_editor = Some(editor);
        self.theme_fork_error = None;
        cx.notify();
    }

    fn save_theme_editor(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = &mut self.theme_editor else {
            return;
        };
        if !editor.dirty {
            return;
        }
        if let Err(error) = atlas_core::util::config::save_custom_theme(
            &editor.file_name,
            &theme_set(&editor.config),
        ) {
            editor.error =
                Some(rust_i18n::t!("theme_save_error", error = error.to_string()).to_string());
        } else {
            ThemeRegistry::global_mut(cx).register_custom_theme(editor.config.clone());
            editor.dirty = false;
            editor.error = None;
            let name = editor.config.name.to_string();
            self.set_theme(name);
        }
        cx.notify();
    }

    pub(crate) fn render_theme_editor(&self, cx: &mut Context<Self>) -> AnyElement {
        let fg = cx.theme().foreground;
        let muted = cx.theme().muted_foreground;
        let danger = cx.theme().danger;
        let accent = cx.theme().accent;
        let accent_fg = cx.theme().accent_foreground;
        let surface = cx.theme().secondary;
        let border = cx.theme().border;
        let hover = cx.theme().secondary_hover;

        let active_theme = self.state.profiles.active().theme.clone();
        let registry = ThemeRegistry::global(cx);
        let sorted = registry.sorted_themes();
        let themes: Vec<_> = sorted
            .into_iter()
            .filter(|cfg| crate::app::theme::theme_is_available(cfg.name.as_ref()))
            .cloned()
            .collect();

        let mut glass_themes = Vec::new();
        let mut solid_themes = Vec::new();
        let mut other_themes = Vec::new();

        for cfg in themes {
            if cfg.name.starts_with("Atlas ") && cfg.name.ends_with(" Glass") {
                glass_themes.push(cfg);
            } else if cfg.name.starts_with("Atlas ") {
                solid_themes.push(cfg);
            } else {
                other_themes.push(cfg);
            }
        }

        let mut content = v_flex().w_full().gap_2().py_2().child(
            h_flex()
                .w_full()
                .items_center()
                .justify_between()
                .gap_2()
                .child(
                    v_flex()
                        .gap_1()
                        .child(
                            rust_i18n::t!(
                                "theme_active",
                                name = self.state.profiles.active().theme.clone()
                            )
                            .to_string(),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(muted)
                                .child(rust_i18n::t!("theme_fork_description").to_string()),
                        ),
                )
                .child(
                    Button::new("fork-active-theme")
                        .small()
                        .label(rust_i18n::t!("theme_fork").to_string())
                        .disabled(self.theme_editor.is_some())
                        .on_click(
                            cx.listener(|this, _, window, cx| this.fork_active_theme(window, cx)),
                        ),
                ),
        );

        // Glass (Frosted Translucent) Section
        if !glass_themes.is_empty() {
            let mut glass_row = h_flex().w_full().gap(px(6.0)).flex_wrap();
            for cfg in &glass_themes {
                let name = cfg.name.to_string();
                let is_selected = active_theme == name;
                let display_name = if cfg.name.starts_with("Atlas ") {
                    &cfg.name[6..]
                } else {
                    cfg.name.as_ref()
                };
                let accent_color = cfg
                    .colors
                    .accent
                    .as_ref()
                    .and_then(|h| Hsla::parse_hex(h).ok())
                    .unwrap_or(accent);

                let theme_name = name.clone();
                glass_row = glass_row.child(
                    div()
                        .id(SharedString::from(format!("theme-chip-{}", cfg.name)))
                        .cursor_pointer()
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .border_1()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .text_xs()
                        .when(is_selected, |d| {
                            d.bg(accent)
                                .border_color(accent)
                                .text_color(accent_fg)
                                .font_semibold()
                        })
                        .when(!is_selected, |d| {
                            d.bg(surface)
                                .border_color(border)
                                .text_color(fg)
                                .hover(|h| h.bg(hover))
                        })
                        .child(
                            div()
                                .size(px(8.0))
                                .rounded_full()
                                .bg(accent_color)
                                .border_1()
                                .border_color(if is_selected {
                                    accent_fg.opacity(0.4)
                                } else {
                                    border
                                }),
                        )
                        .child(display_name.to_string())
                        .when(is_selected, |d| d.child(div().ml(px(2.0)).child("✓")))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.set_theme(theme_name.clone());
                                cx.notify();
                            }),
                        ),
                );
            }

            content = content.child(
                v_flex()
                    .gap(px(4.0))
                    .mt_1()
                    .child(
                        div()
                            .text_xs()
                            .font_semibold()
                            .text_color(fg)
                            .child("Translucent Themes (Glass Blur)"),
                    )
                    .child(glass_row),
            );
        }

        // Solid (Opaque) Section
        if !solid_themes.is_empty() {
            let mut solid_row = h_flex().w_full().gap(px(6.0)).flex_wrap();
            for cfg in &solid_themes {
                let name = cfg.name.to_string();
                let is_selected = active_theme == name;
                let display_name = if cfg.name.starts_with("Atlas ") {
                    &cfg.name[6..]
                } else {
                    cfg.name.as_ref()
                };
                let accent_color = cfg
                    .colors
                    .accent
                    .as_ref()
                    .and_then(|h| Hsla::parse_hex(h).ok())
                    .unwrap_or(accent);

                let theme_name = name.clone();
                solid_row = solid_row.child(
                    div()
                        .id(SharedString::from(format!("theme-chip-{}", cfg.name)))
                        .cursor_pointer()
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .border_1()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .text_xs()
                        .when(is_selected, |d| {
                            d.bg(accent)
                                .border_color(accent)
                                .text_color(accent_fg)
                                .font_semibold()
                        })
                        .when(!is_selected, |d| {
                            d.bg(surface)
                                .border_color(border)
                                .text_color(fg)
                                .hover(|h| h.bg(hover))
                        })
                        .child(
                            div()
                                .size(px(8.0))
                                .rounded_full()
                                .bg(accent_color)
                                .border_1()
                                .border_color(if is_selected {
                                    accent_fg.opacity(0.4)
                                } else {
                                    border
                                }),
                        )
                        .child(display_name.to_string())
                        .when(is_selected, |d| d.child(div().ml(px(2.0)).child("✓")))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.set_theme(theme_name.clone());
                                cx.notify();
                            }),
                        ),
                );
            }

            content = content.child(
                v_flex()
                    .gap(px(4.0))
                    .mt_2()
                    .child(
                        div()
                            .text_xs()
                            .font_semibold()
                            .text_color(fg)
                            .child("Solid Themes (Opaque)"),
                    )
                    .child(solid_row),
            );
        }

        // Custom & Community Themes Section (if any present)
        if !other_themes.is_empty() {
            let mut other_row = h_flex().w_full().gap(px(6.0)).flex_wrap();
            for cfg in &other_themes {
                let name = cfg.name.to_string();
                let is_selected = active_theme == name;
                let display_name = if cfg.name.starts_with("Atlas ") {
                    &cfg.name[6..]
                } else {
                    cfg.name.as_ref()
                };
                let accent_color = cfg
                    .colors
                    .accent
                    .as_ref()
                    .and_then(|h| Hsla::parse_hex(h).ok())
                    .unwrap_or(accent);

                let theme_name = name.clone();
                other_row = other_row.child(
                    div()
                        .id(SharedString::from(format!("theme-chip-{}", cfg.name)))
                        .cursor_pointer()
                        .px_2()
                        .py_1()
                        .rounded_md()
                        .border_1()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .text_xs()
                        .when(is_selected, |d| {
                            d.bg(accent)
                                .border_color(accent)
                                .text_color(accent_fg)
                                .font_semibold()
                        })
                        .when(!is_selected, |d| {
                            d.bg(surface)
                                .border_color(border)
                                .text_color(fg)
                                .hover(|h| h.bg(hover))
                        })
                        .child(
                            div()
                                .size(px(8.0))
                                .rounded_full()
                                .bg(accent_color)
                                .border_1()
                                .border_color(if is_selected {
                                    accent_fg.opacity(0.4)
                                } else {
                                    border
                                }),
                        )
                        .child(display_name.to_string())
                        .when(is_selected, |d| d.child(div().ml(px(2.0)).child("✓")))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _, _, cx| {
                                this.set_theme(theme_name.clone());
                                cx.notify();
                            }),
                        ),
                );
            }

            content = content.child(
                v_flex()
                    .gap(px(4.0))
                    .mt_2()
                    .child(
                        div()
                            .text_xs()
                            .font_semibold()
                            .text_color(fg)
                            .child("Community & Custom Themes"),
                    )
                    .child(other_row),
            );
        }
        if let Some(error) = &self.theme_fork_error {
            content = content.child(div().text_sm().text_color(danger).child(error.clone()));
        }
        if let Some(editor) = &self.theme_editor {
            content = content.child(div().text_sm().text_color(fg).child(
                rust_i18n::t!("theme_editing", name = editor.config.name.to_string()).to_string(),
            ));
            content = content.child(
                div()
                    .text_xs()
                    .text_color(muted)
                    .child(rust_i18n::t!("theme_editor_description").to_string()),
            );
            for (role, control) in &editor.controls {
                let hex = control
                    .read(cx)
                    .value()
                    .map(|value| value.to_hex())
                    .unwrap_or_default();
                content = content.child(
                    h_flex()
                        .w_full()
                        .items_center()
                        .justify_between()
                        .gap_2()
                        .child(div().text_sm().text_color(fg).child(role.label()))
                        .child(ColorPicker::new(control).small().label(hex)),
                );
            }
            if let Some(error) = &editor.error {
                content = content.child(div().text_sm().text_color(danger).child(error.clone()));
            }
            content = content.child(
                h_flex()
                    .gap_2()
                    .mt(px(4.0))
                    .child(
                        Button::new("save-custom-theme")
                            .primary()
                            .small()
                            .label(rust_i18n::t!("theme_save").to_string())
                            .disabled(!editor.dirty)
                            .on_click(cx.listener(|this, _, _, cx| this.save_theme_editor(cx))),
                    )
                    .child(
                        Button::new("close-theme-editor")
                            .small()
                            .label(
                                rust_i18n::t!(if editor.dirty {
                                    "theme_discard"
                                } else {
                                    "theme_close_editor"
                                })
                                .to_string(),
                            )
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.theme_editor = None;
                                cx.notify();
                            })),
                    ),
            );
        }
        content.into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn theme_fork_preserves_material_fonts_and_colors_with_a_unique_name() {
        let set: ThemeSet =
            serde_json::from_str(include_str!("../../assets/themes/atlas-themes.json")).unwrap();
        let source = set
            .themes
            .iter()
            .find(|theme| theme.name.ends_with(" Glass"))
            .unwrap();
        let first = fork_config(source, |_| false);
        let second = fork_config(source, |name| name == first.name.as_ref());
        assert_ne!(first.name, second.name);
        assert!(first.name.ends_with(" Glass"));
        assert!(second.name.ends_with(" Glass"));
        assert!(!first.is_default);
        assert_eq!(
            serde_json::to_value(&first.colors).unwrap(),
            serde_json::to_value(&source.colors).unwrap()
        );
        assert_eq!(first.font_family, source.font_family);
        assert_eq!(first.mode, source.mode);
        let serialized = serde_json::to_string(&theme_set(&first)).unwrap();
        let restored: ThemeSet = serde_json::from_str(&serialized).unwrap();
        assert_eq!(restored.themes[0].name, first.name);
    }
}
