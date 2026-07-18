//! The Typst compilation engine.
//!
//! Implements a minimal in-memory [`World`] over a single detached source file
//! and the fonts baked into the binary, then compiles straight to PDF bytes.

use std::collections::HashMap;
use std::sync::OnceLock;

use anyhow::{anyhow, Result};
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};
use typst_layout::PagedDocument;
use typst_pdf::PdfOptions;

/// Fonts embedded directly into the binary (`include_bytes!`), guaranteeing the
/// compiler runs identically on a fresh OS with no network or system fonts.
///
/// Liberation Sans covers body text; Tabler Icons (MIT-licensed, family name
/// `tabler-icons`) supplies the crisp, single-weight contact/section glyphs.
static EMBEDDED_FONTS: &[&[u8]] = &[
    // "modern" template body.
    include_bytes!("../assets/fonts/LiberationSans-Regular.ttf"),
    include_bytes!("../assets/fonts/LiberationSans-Bold.ttf"),
    include_bytes!("../assets/fonts/LiberationSans-Italic.ttf"),
    include_bytes!("../assets/fonts/LiberationSans-BoldItalic.ttf"),
    // Contact/section icons (both templates).
    include_bytes!("../assets/fonts/tabler-icons.ttf"),
    // "crisp" template: Roboto (header) + Source Sans 3 (body).
    include_bytes!("../assets/fonts/Roboto-Thin.ttf"),
    include_bytes!("../assets/fonts/Roboto-ThinItalic.ttf"),
    include_bytes!("../assets/fonts/Roboto-Light.ttf"),
    include_bytes!("../assets/fonts/Roboto-LightItalic.ttf"),
    include_bytes!("../assets/fonts/Roboto-Regular.ttf"),
    include_bytes!("../assets/fonts/Roboto-Italic.ttf"),
    include_bytes!("../assets/fonts/Roboto-Medium.ttf"),
    include_bytes!("../assets/fonts/Roboto-MediumItalic.ttf"),
    include_bytes!("../assets/fonts/Roboto-Bold.ttf"),
    include_bytes!("../assets/fonts/Roboto-BoldItalic.ttf"),
    include_bytes!("../assets/fonts/SourceSans3-VF.ttf"),
    include_bytes!("../assets/fonts/SourceSans3-Italic-VF.ttf"),
    // "serif" / "classic" templates: a Times-like serif.
    include_bytes!("../assets/fonts/LiberationSerif-Regular.ttf"),
    include_bytes!("../assets/fonts/LiberationSerif-Bold.ttf"),
    include_bytes!("../assets/fonts/LiberationSerif-Italic.ttf"),
    include_bytes!("../assets/fonts/LiberationSerif-BoldItalic.ttf"),
];

/// Typst's fallback fonts, only compiled in with the `fallback-fonts` feature.
#[cfg(feature = "fallback-fonts")]
fn fallback_fonts() -> impl Iterator<Item = Bytes> {
    typst_assets::fonts().map(Bytes::new)
}

#[cfg(not(feature = "fallback-fonts"))]
fn fallback_fonts() -> impl Iterator<Item = Bytes> {
    std::iter::empty()
}

/// Build a project-rooted [`FileId`] from an absolute virtual path like
/// `"/main.typ"`.
fn project_file_id(vpath: &str) -> FileId {
    let vp = VirtualPath::new(vpath).expect("valid virtual path");
    FileId::new(RootedPath::new(VirtualRoot::Project, vp))
}

/// An in-memory Typst environment for compiling a single resume source, plus
/// any extra binary assets (e.g. a profile photo) served by virtual path.
pub struct ResumeWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main: FileId,
    source: Source,
    assets: HashMap<FileId, Bytes>,
}

impl ResumeWorld {
    /// Build a world from a Typst source string and virtual-path assets.
    /// Each asset is `(virtual_path, bytes)`, referenced from Typst via
    /// `image(virtual_path)`.
    pub fn new(source_text: String, assets: Vec<(String, Vec<u8>)>) -> ResumeWorld {
        let mut book = FontBook::new();
        let mut fonts = Vec::new();

        // Our embedded families first, then (optionally) Typst's fallback fonts
        // for glyphs Liberation Sans and Tabler Icons lack.
        let sources = EMBEDDED_FONTS
            .iter()
            .copied()
            .map(Bytes::new)
            .chain(fallback_fonts());

        for bytes in sources {
            for font in Font::iter(bytes) {
                book.push(font.info().clone());
                fonts.push(font);
            }
        }

        // Give the main file a real virtual path so absolute asset paths resolve.
        let main = project_file_id("/main.typ");
        let assets = assets
            .into_iter()
            .map(|(vpath, bytes)| (project_file_id(&vpath), Bytes::new(bytes)))
            .collect();

        ResumeWorld {
            library: LazyHash::new(Library::builder().build()),
            book: LazyHash::new(book),
            fonts,
            main,
            source: Source::new(main, source_text),
            assets,
        }
    }
}

impl World for ResumeWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(id.vpath().get_without_slash().into()))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.assets
            .get(&id)
            .cloned()
            .ok_or_else(|| FileError::NotFound(id.vpath().get_without_slash().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let utc = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
        // Shift into local time so the printed date matches the user's clock.
        let local = utc + local_utc_offset_seconds();
        let (y, m, d) = civil_from_days(local.div_euclid(86_400));
        Datetime::from_ymd(y, m, d)
    }
}

/// The local timezone's offset from UTC, in seconds.
///
/// Determining the local offset is only sound on a single-threaded process, so
/// it is resolved once via [`prime_local_offset`] (called before Typst spawns
/// its worker threads) and cached. Falls back to 0 (UTC) if undetermined.
static LOCAL_OFFSET: OnceLock<i64> = OnceLock::new();

/// Resolve and cache the local UTC offset. Call this on the main thread before
/// compiling (see `build_once`); doing so keeps the lookup sound.
pub fn prime_local_offset() {
    LOCAL_OFFSET.get_or_init(|| {
        time::UtcOffset::current_local_offset()
            .map(|o| o.whole_seconds() as i64)
            .unwrap_or(0)
    });
}

fn local_utc_offset_seconds() -> i64 {
    LOCAL_OFFSET.get().copied().unwrap_or(0)
}

/// Convert a count of days since the Unix epoch into a (year, month, day) civil
/// date, using Howard Hinnant's algorithm (valid for the proleptic Gregorian
/// calendar). Avoids pulling in a date/time crate.
fn civil_from_days(z: i64) -> (i32, u8, u8) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u8; // [1, 31]
    let m: i64 = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u8, d)
}

/// Compile a full Typst source string (plus any virtual-path assets) into PDF
/// bytes and the number of pages produced.
pub fn render_pdf(source_text: String, assets: Vec<(String, Vec<u8>)>) -> Result<(Vec<u8>, usize)> {
    let world = ResumeWorld::new(source_text, assets);

    let result = typst::compile::<PagedDocument>(&world);
    let document = result.output.map_err(|diags| {
        let msg = diags
            .iter()
            .map(|d| d.message.to_string())
            .collect::<Vec<_>>()
            .join("\n  ");
        anyhow!("Typst compilation failed:\n  {msg}")
    })?;

    let pages = document.pages().len();
    let pdf = typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|diags| anyhow!("PDF export failed: {} error(s)", diags.len()))?;

    Ok((pdf, pages))
}
