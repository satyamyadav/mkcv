//! Filesystem listener that rebuilds the PDF on every save of the input file.

use std::path::Path;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use notify::{Event, RecursiveMode, Watcher};

use crate::build::{build_once, BuildReport, Overrides};

/// Watch `input` and rebuild `output` on each change until interrupted.
pub fn watch(input: &Path, output: &Path, overrides: Overrides) -> Result<()> {
    // Initial build so the output is fresh the moment watching starts.
    report(build_once(input, output, overrides));

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })
    .context("failed to initialise filesystem watcher")?;

    // Watch the parent directory: many editors save by replacing the file,
    // which drops a watch placed directly on the file itself.
    let dir = input.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
    watcher
        .watch(dir, RecursiveMode::NonRecursive)
        .with_context(|| format!("failed to watch {}", dir.display()))?;

    println!("👀 Watching {} — press Ctrl+C to stop.", input.display());

    let mut last = Instant::now() - Duration::from_secs(1);
    for res in rx {
        let event: Event = match res {
            Ok(e) => e,
            Err(e) => {
                eprintln!("watch error: {e}");
                continue;
            }
        };

        let touched = event.paths.iter().any(|p| same_file(p, input));
        if !touched {
            continue;
        }

        // Debounce: editors often emit several events per save.
        if last.elapsed() < Duration::from_millis(80) {
            continue;
        }
        last = Instant::now();

        report(build_once(input, output, overrides));
    }

    Ok(())
}

/// Compare two paths by file name and, when possible, canonical form.
fn same_file(a: &Path, b: &Path) -> bool {
    if a == b {
        return true;
    }
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(x), Ok(y)) => x == y,
        _ => a.file_name() == b.file_name(),
    }
}

fn report(result: Result<BuildReport>) {
    match result {
        Ok(r) => println!("✓ Rebuilt in {:.1}ms", r.elapsed.as_secs_f64() * 1000.0),
        Err(e) => eprintln!("✗ {e:#}"),
    }
}
