//! The core build pipeline: YAML (file or string) → parsed resume → Typst
//! source → PDF.

use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};

use crate::engine::{prime_local_offset, render_pdf};
use crate::parser::Resume;
use crate::template::{build_source, photo_vpath};

/// The result of a successful build.
pub struct BuildReport {
    pub pages: usize,
    pub elapsed: Duration,
}

/// Optional `meta.template` / `meta.kind` overrides supplied on the CLI.
#[derive(Default, Clone, Copy)]
pub struct Overrides<'a> {
    pub template: Option<&'a str>,
    pub kind: Option<&'a str>,
}

/// Read `input`, compile it, and write the PDF to `output`.
pub fn build_once(input: &Path, output: &Path, overrides: Overrides) -> Result<BuildReport> {
    let yaml = fs::read_to_string(input)
        .with_context(|| format!("could not read input file {}", input.display()))?;
    let base = input.parent().filter(|p| !p.as_os_str().is_empty());
    build_yaml(&yaml, base.unwrap_or_else(|| Path::new(".")), output, overrides)
}

/// Compile a resume from an in-memory YAML string. Relative paths (photo,
/// custom template) resolve against `base_dir`.
pub fn build_yaml(
    yaml: &str,
    base_dir: &Path,
    output: &Path,
    overrides: Overrides,
) -> Result<BuildReport> {
    // Resolve the local timezone offset here, on the main thread, before Typst
    // compilation spawns worker threads (the lookup is only sound single-threaded).
    prime_local_offset();

    let start = Instant::now();

    let mut resume = Resume::from_yaml(yaml)?;
    if let Some(t) = overrides.template {
        resume.meta.template = t.to_string();
    }
    if let Some(k) = overrides.kind {
        resume.meta.kind = Some(k.to_string());
    }

    let custom = load_custom_template(&resume, base_dir)?;
    let source = build_source(&resume, custom.as_deref())?;
    let assets = collect_assets(&resume, base_dir)?;
    let (pdf, pages) = render_pdf(source, assets)?;

    fs::write(output, &pdf)
        .with_context(|| format!("could not write output file {}", output.display()))?;

    Ok(BuildReport { pages, elapsed: start.elapsed() })
}

/// If `meta.template` is a local `.typ` path, read it (relative to `base_dir`)
/// so it can be compiled as a custom template body. Otherwise returns `None`
/// and the embedded templates are used.
fn load_custom_template(resume: &Resume, base_dir: &Path) -> Result<Option<String>> {
    let template = &resume.meta.template;
    if !template.ends_with(".typ") {
        return Ok(None);
    }
    let path = base_dir.join(template);
    let body = fs::read_to_string(&path)
        .with_context(|| format!("could not read custom template {}", path.display()))?;
    Ok(Some(body))
}

/// Read any binary assets referenced by the resume (currently the profile
/// photo), resolving their paths relative to `base_dir`.
fn collect_assets(resume: &Resume, base_dir: &Path) -> Result<Vec<(String, Vec<u8>)>> {
    let mut assets = Vec::new();
    if let Some(photo) = &resume.profile.photo {
        let photo_path = base_dir.join(&photo.path);
        let bytes = fs::read(&photo_path)
            .with_context(|| format!("could not read profile photo {}", photo_path.display()))?;
        assets.push((photo_vpath(&photo.path), bytes));
    }
    Ok(assets)
}
