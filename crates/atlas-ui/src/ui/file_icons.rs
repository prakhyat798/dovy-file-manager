use std::path::{Path, PathBuf};
use std::sync::Arc;

use gpui::{img, AnyElement, Hsla, IntoElement, RenderImage, Styled};

use crate::app::App;
use crate::ui::icons::{file_type_icon, is_windows_theme, AtlasIcon};
use atlas_model::file_entry::FileKind;
use atlas_model::settings::FileIconStyle;

#[derive(Clone)]
pub(crate) struct FileIconRef {
    pub(crate) path: PathBuf,
    pub(crate) kind: FileKind,
    pub(crate) extension: Option<gpui::SharedString>,
}

#[derive(Clone)]
pub(crate) enum IconTexture {
    Single(Arc<RenderImage>),
    Multi(Arc<crate::icon_theme::MultiResIcon>),
}

impl IconTexture {
    #[inline]
    pub(crate) fn for_size(&self, px: f32) -> Arc<RenderImage> {
        match self {
            Self::Single(tex) => tex.clone(),
            Self::Multi(multi) => multi.for_size(px),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn single(&self) -> Arc<RenderImage> {
        match self {
            Self::Single(tex) => tex.clone(),
            Self::Multi(multi) => multi.full.clone(),
        }
    }
}

impl From<Arc<RenderImage>> for IconTexture {
    fn from(tex: Arc<RenderImage>) -> Self {
        Self::Single(tex)
    }
}

impl From<Arc<crate::icon_theme::MultiResIcon>> for IconTexture {
    fn from(multi: Arc<crate::icon_theme::MultiResIcon>) -> Self {
        Self::Multi(multi)
    }
}

#[derive(Clone)]
pub(crate) enum FileIcon {
    PreviewTexture(Arc<RenderImage>),
    IconTexture(IconTexture),
    Builtin {
        kind: FileKind,
        extension: Option<gpui::SharedString>,
    },
    Vector(AtlasIcon),
}

impl FileIcon {
    pub(crate) fn element(self, size: f32, color: Hsla) -> AnyElement {
        match self {
            Self::PreviewTexture(tex) => img(tex)
                .size(crate::ui::rem_px(size))
                .flex_none()
                .into_any_element(),
            Self::IconTexture(tex) => img(tex.for_size(crate::ui::BASE_PX * size))
                .size(crate::ui::rem_px(size))
                .flex_none()
                .into_any_element(),
            Self::Builtin { kind, extension } => file_type_icon(&kind, extension.as_deref(), size),
            Self::Vector(icon) => crate::ui::icons::icon(icon, size, color),
        }
    }

    pub(crate) fn texture(&self) -> Option<Arc<RenderImage>> {
        self.texture_for_size(24.0)
    }

    pub(crate) fn texture_for_size(&self, px: f32) -> Option<Arc<RenderImage>> {
        match self {
            Self::PreviewTexture(tex) => Some(tex.clone()),
            Self::IconTexture(tex) => Some(tex.for_size(px)),
            Self::Builtin { .. } | Self::Vector(_) => None,
        }
    }
}

pub(crate) fn vector_atlas_icon(kind: &FileKind, extension: Option<&str>) -> AtlasIcon {
    match kind {
        FileKind::Dir => AtlasIcon::Folder,
        FileKind::Symlink => AtlasIcon::File,
        FileKind::File => {
            let ext = extension.unwrap_or("");
            if ext.eq_ignore_ascii_case("pdf")
                || ext.eq_ignore_ascii_case("doc")
                || ext.eq_ignore_ascii_case("docx")
                || ext.eq_ignore_ascii_case("txt")
                || ext.eq_ignore_ascii_case("md")
                || ext.eq_ignore_ascii_case("json")
                || ext.eq_ignore_ascii_case("toml")
                || ext.eq_ignore_ascii_case("rs")
                || ext.eq_ignore_ascii_case("py")
                || ext.eq_ignore_ascii_case("js")
                || ext.eq_ignore_ascii_case("ts")
            {
                AtlasIcon::TextFile
            } else if ext.eq_ignore_ascii_case("png")
                || ext.eq_ignore_ascii_case("jpg")
                || ext.eq_ignore_ascii_case("jpeg")
                || ext.eq_ignore_ascii_case("gif")
                || ext.eq_ignore_ascii_case("webp")
                || ext.eq_ignore_ascii_case("svg")
            {
                AtlasIcon::Image
            } else if ext.eq_ignore_ascii_case("mp3")
                || ext.eq_ignore_ascii_case("wav")
                || ext.eq_ignore_ascii_case("flac")
                || ext.eq_ignore_ascii_case("ogg")
                || ext.eq_ignore_ascii_case("m4a")
                || ext.eq_ignore_ascii_case("aac")
            {
                AtlasIcon::Audio
            } else if ext.eq_ignore_ascii_case("mp4")
                || ext.eq_ignore_ascii_case("mkv")
                || ext.eq_ignore_ascii_case("mov")
                || ext.eq_ignore_ascii_case("avi")
                || ext.eq_ignore_ascii_case("webm")
            {
                AtlasIcon::Video
            } else {
                AtlasIcon::File
            }
        }
    }
}

impl App {
    pub(crate) fn file_icon_ref_for_path(path: PathBuf, is_dir: bool) -> FileIconRef {
        let kind = if is_dir {
            FileKind::Dir
        } else {
            FileKind::File
        };
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase());
        FileIconRef {
            path,
            kind,
            extension: extension.map(gpui::SharedString::from),
        }
    }

    #[inline]
    pub(crate) fn should_use_shell_icons(&self) -> bool {
        #[cfg(target_os = "windows")]
        {
            match self.state.settings.file_icon_style {
                FileIconStyle::System => true,
                FileIconStyle::LiquidGlass | FileIconStyle::Vector => false,
                FileIconStyle::Auto => is_windows_theme(&self.state.profiles.active().theme),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    }

    /// Icon for compact rows (details/list/tree/miller/palette): in Windows themes
    /// or System icon style always the plain Shell file-type icon — never a thumbnail or content peek.
    pub(crate) fn row_icon_for_entry(
        &self,
        path: &Path,
        kind: &FileKind,
        extension: Option<&str>,
    ) -> FileIcon {
        #[cfg(target_os = "windows")]
        if self.should_use_shell_icons() {
            let tex = if matches!(kind, FileKind::Dir) {
                self.shell_folder_icon()
            } else {
                self.shell_type_icon(path)
            };
            if let Some(tex) = tex {
                return FileIcon::IconTexture(IconTexture::Multi(tex));
            }
        } else if let Some(tex) = self.grid_thumbnail(path) {
            return FileIcon::PreviewTexture(tex);
        }
        self.file_icon_fallback(kind, extension)
    }

    fn file_icon_fallback(&self, kind: &FileKind, extension: Option<&str>) -> FileIcon {
        match self.state.settings.file_icon_style {
            FileIconStyle::LiquidGlass => {
                FileIcon::IconTexture(IconTexture::Single(self.liquid_glass_icon(kind, extension)))
            }
            FileIconStyle::Vector => {
                if let Some(multi) = self.multi_res_icon_for_entry(kind, extension) {
                    FileIcon::IconTexture(IconTexture::Multi(multi))
                } else {
                    FileIcon::Vector(vector_atlas_icon(kind, extension))
                }
            }
            FileIconStyle::System | FileIconStyle::Auto => {
                if let Some(multi) = self.multi_res_icon_for_entry(kind, extension) {
                    return FileIcon::IconTexture(IconTexture::Multi(multi));
                }
                FileIcon::IconTexture(IconTexture::Single(self.liquid_glass_icon(kind, extension)))
            }
        }
    }

    pub(crate) fn file_icon_for_entry(
        &self,
        path: &Path,
        kind: &FileKind,
        extension: Option<&str>,
    ) -> FileIcon {
        if let Some(tex) = self.grid_thumbnail(path) {
            return FileIcon::PreviewTexture(tex);
        }
        #[cfg(target_os = "windows")]
        if self.should_use_shell_icons() {
            if matches!(kind, FileKind::Dir) {
                if let Some(tex) = self.shell_folder_icon() {
                    return FileIcon::IconTexture(IconTexture::Multi(tex));
                }
            } else if let Some(tex) = self.shell_type_icon(path) {
                return FileIcon::IconTexture(IconTexture::Multi(tex));
            }
        }
        self.file_icon_fallback(kind, extension)
    }

    pub(crate) fn file_icon(&self, file: &FileIconRef) -> FileIcon {
        self.row_icon_for_entry(&file.path, &file.kind, file.extension.as_deref())
    }

    pub(crate) fn maybe_request_file_icon(&mut self, file: &FileIconRef) {
        #[cfg(target_os = "windows")]
        if self.should_use_shell_icons() {
            if matches!(file.kind, FileKind::Dir) {
                self.request_shell_folder_icon();
            } else {
                self.request_shell_type_icon(&file.path);
            }
        }
    }
}
