<div align="center">

<img src="packaging/icons/dovy.png" width="128" alt="Dovy File Manager logo">

# Dovy File Manager

**A fast, local-first file manager built with Rust and GPUI.**

[![License: AGPL 3.0](https://img.shields.io/badge/License-AGPL_3.0-blue.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built_with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Latest Release](https://img.shields.io/github/v/release/prakhyat798/dovy-file-manager?color=brightgreen&label=Version)](https://github.com/prakhyat798/dovy-file-manager/releases)

<br>

[![Download Windows Installer](https://img.shields.io/badge/⬇️_Download_Windows_Installer-v0.1.3-0078D4?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/prakhyat798/dovy-file-manager/releases/download/v0.1.3/Dovy-File-Manager-Setup-v0.1.3.exe)

</div>

Dovy is a desktop file manager for Windows and Linux. It combines everyday file operations with multi-pane browsing, rich previews, archive tools, local conversion, encrypted vaults, cloud access, disk analysis, and an extension system.

## Features

### Browsing and layout

- Five views with Details, Grid, List, Tree, and Miller Columns
- Tabs, detachable tabs, horizontal and vertical split panes, pane resizing, and equalized layouts
- Sidebar with bookmarks, recent folders, drives, tags, cloud locations, and configurable sections
- Breadcrumb and direct-path navigation with back, forward, up, and history controls
- Recursive branch browsing, hidden-file controls, per-pane zoom, grouping, sorting, and filtering
- Session restore and optional automatic workspace saving

### File operations

- Copy, move, paste, rename, batch rename, create, delete, restore, and permanent deletion
- Recycle Bin browsing, restore, selective purge, and empty-bin actions
- Undo and redo for supported file operations
- Drag and drop within Dovy, between panes, to the sidebar, and to or from Windows applications
- Conflict handling for overwrite, skip, rename, and folder merges
- Native system context menus, properties, Open With, shortcuts, and terminal launch
- Operation progress, cancellation, error reporting, and completion feedback

### Search and organization

- Fast in-place name filtering and fuzzy matching
- Folder and subfolder search with a background volume index
- Content search powered by local search services
- Color tags, tag-based results, bookmarks, and recent locations
- Selection tools including select all, invert selection, keyboard range selection, and Vim-style navigation

### Preview and inspection

- Image previews and thumbnails with zoom and fit controls
- Text and source previews with syntax highlighting, optional line numbers, editing, and save
- Rendered Markdown with headings, lists, code, quotes, and links
- Audio metadata and embedded artwork
- Video frame, codec, resolution, and duration previews
- PDF page previews
- CSV, TSV, XLS, XLSX, and ODS table previews
- DOCX and PPTX content previews
- PSD and PSB previews through an optional local tool
- Built-in software previews for GLB, glTF, FBX, OBJ, and STL models
- Directory summaries, metadata inspection, and multi-selection thumbnail previews
- Preview support for files inside archives and supported cloud locations

### Archives

- Browse archives as virtual folders without extracting first
- ZIP, RAR, 7Z, TAR, and compressed TAR support
- Create archives, extract selected items, extract whole archives, and choose destinations
- Archive-to-archive conversion for supported formats

### File conversion

- Batch conversion from the selection with progress, cancellation, output-folder choice, quality, and size controls
- Images including PNG, JPG, WebP, BMP, TIFF, GIF, ICO, SVG, and PDF routes
- Documents including PDF, DOCX, TXT, Markdown, HTML, EPUB, ODT, and RTF routes
- Spreadsheets and structured data including XLSX, ODS, CSV, TSV, JSON, JSONL, TOML, and vCard
- Audio and video routes including MP3, WAV, FLAC, M4A, OGG, Opus, AAC, MP4, WebM, MKV, MOV, and AVI
- Subtitle conversion between SRT, VTT, ASS, LRC, SAMI, TTML, and text
- Ebook conversion for EPUB, FB2, MOBI, and AZW3
- Optional local engines such as ImageMagick, FFmpeg, Pandoc, LibreOffice, Calibre, Ghostscript, Poppler, and Opus
- Originals are kept and output names avoid overwriting existing files

### Cloud and network

- SFTP, FTP, WebDAV, SMB, Google Drive, and OneDrive connections
- Direct access mode and optional mounted-drive mode through rclone and WinFsp
- Upload, download, remote copy and move, rename, folder creation, and deletion
- Transfer queue with pause, resume, retry, cancel, and progress controls
- Cloud previews and storage quota display

### Privacy and security

- Local-first operation with no required file upload
- Password-protected encrypted vaults using authenticated encryption and memory-hard password derivation
- Create, unlock, auto-lock, import, export, restore, and profile-based vault access
- Optional native mounted vault access on Windows
- Private-file handling that prevents unsupported previews, conversions, or external leakage

### Storage and maintenance

- Space Lens disk visualization for finding large folders and files
- Windows storage map and cleanup review
- Cleanup categories for temporary files, browser caches, crash data, and development caches
- Protected paths, review-before-delete, progress, cancellation, reports, and cleanup history
- File and folder analyzer with size and content summaries

### Customization and extensibility

- Themes including glass styles, configurable opacity, fonts, spacing, thumbnail size, and icon styles
- Multiple profiles with separate layout, view, bookmark, tag, and preview preferences
- Command palette, configurable shortcuts, Vim mode, and first-run guide
- Smooth-scrolling control, reduced or disabled animations, eco mode, and inactive-pane dimming
- 22 bundled interface languages
- Extension manager with install, enable, disable, and uninstall flows
- Declarative file tools, preview providers, icon providers, conversion engines, and sidecar providers
- Built-in extension manifests for cloud drives, document previews, media thumbnails, file tools, icons, and conversion tools

## Build from source

Dovy uses the Rust toolchain pinned in [rust-toolchain.toml](rust-toolchain.toml). Cargo downloads the locked registry dependencies, while the project-specific GPUI forks remain included under `vendor`.

```powershell
cargo check --workspace
cargo run --release
```

The main executable is named `dovy`. See [ARCHITECTURE.md](ARCHITECTURE.md) for the workspace structure and [CONTRIBUTING.md](CONTRIBUTING.md) for development guidance.

## Project structure

- `crates/atlas-model` contains serializable state and domain models
- `crates/atlas-core` contains filesystem, search, conversion, archive, cloud, vault, cleanup, and extension backends
- `crates/atlas-ui` contains the GPUI interface, views, input handling, previews, and overlays
- `crates/atlas-win-space` contains Windows integration
- `extensions` contains the bundled extension manifests
- `packaging` contains application icons and platform packaging files

## License and attribution

Dovy is distributed under the [GNU Affero General Public License v3.0 or later](LICENSE). This codebase is derived from Atlas File Manager and retains its AGPL license and third-party notices. Dependency licenses and notices are stored under `legal`.
