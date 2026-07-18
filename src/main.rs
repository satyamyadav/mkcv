//! mkcv — a lightning-fast, zero-dependency CV compiler.
//!
//! Compiles a single YAML data file into a pixel-perfect, typeset PDF using an
//! embedded Typst engine and fonts. No LaTeX, no headless browser, no network.
//!
//! The CLI exposes an MCP-tool-like surface: discrete operations (`build`,
//! `init`, `templates`, `schema`, `validate`) with structured `--format json`
//! output that an agent can parse.

mod build;
mod engine;
mod parser;
mod template;
mod watcher;

use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde_json::json;

use build::Overrides;

/// Boilerplate resume written out by `mkcv init`.
const EXAMPLE_YAML: &str = include_str!("../assets/resume.example.yml");

/// Output format for machine vs. human consumption.
#[derive(ValueEnum, Clone, Copy, PartialEq, Eq)]
enum Format {
    /// Human-readable text (default).
    Text,
    /// One structured JSON object per command.
    Json,
}

#[derive(Parser)]
#[command(
    name = "mkcv",
    version,
    about = "Compile a YAML resume into a pixel-perfect PDF — fast and dependency-free."
)]
struct Cli {
    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text, global = true)]
    format: Format,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Write a boilerplate resume.yml into the current directory.
    Init {
        /// Path of the config file to create.
        #[arg(short, long, default_value = "resume.yml")]
        output: PathBuf,
        /// Overwrite the file if it already exists.
        #[arg(short, long)]
        force: bool,
    },

    /// Compile a PDF from a resume data file (or inline `--yaml`).
    Build {
        /// Input YAML data file.
        #[arg(short, long, default_value = "resume.yml")]
        input: PathBuf,
        /// Inline YAML string (overrides --input; paths resolve from the cwd).
        #[arg(long)]
        yaml: Option<String>,
        /// Output PDF file.
        #[arg(short, long, default_value = "resume.pdf")]
        output: PathBuf,
        /// Override meta.template.
        #[arg(short, long)]
        template: Option<String>,
        /// Override meta.kind ("resume" or "cover-letter").
        #[arg(short, long)]
        kind: Option<String>,
    },

    /// Watch the data file and rebuild the PDF on every save.
    Watch {
        #[arg(short, long, default_value = "resume.yml")]
        input: PathBuf,
        #[arg(short, long, default_value = "resume.pdf")]
        output: PathBuf,
        #[arg(short, long)]
        template: Option<String>,
        #[arg(short, long)]
        kind: Option<String>,
    },

    /// List the available templates and how to select them.
    Templates,

    /// Describe the YAML data schema.
    Schema,

    /// Check that a resume file (or inline `--yaml`) parses.
    Validate {
        #[arg(short, long, default_value = "resume.yml")]
        input: PathBuf,
        #[arg(long)]
        yaml: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let format = cli.format;
    if let Err(e) = run(cli) {
        match format {
            Format::Json => println!("{}", json!({ "ok": false, "error": format!("{e:#}") })),
            Format::Text => eprintln!("Error: {e:#}"),
        }
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let fmt = cli.format;
    match cli.command {
        Command::Init { output, force } => cmd_init(&output, force, fmt),
        Command::Build { input, yaml, output, template, kind } => {
            cmd_build(&input, yaml.as_deref(), &output, template.as_deref(), kind.as_deref(), fmt)
        }
        Command::Watch { input, output, template, kind } => watcher::watch(
            &input,
            &output,
            Overrides { template: template.as_deref(), kind: kind.as_deref() },
        ),
        Command::Templates => cmd_templates(fmt),
        Command::Schema => cmd_schema(fmt),
        Command::Validate { input, yaml } => cmd_validate(&input, yaml.as_deref(), fmt),
    }
}

fn cmd_init(output: &Path, force: bool, fmt: Format) -> Result<()> {
    if output.exists() && !force {
        bail!("{} already exists (use --force to overwrite)", output.display());
    }
    std::fs::write(output, EXAMPLE_YAML)
        .with_context(|| format!("could not write {}", output.display()))?;
    match fmt {
        Format::Json => println!("{}", json!({ "ok": true, "path": output.display().to_string() })),
        Format::Text => {
            println!("✓ Wrote {}", output.display());
            println!("  Next: mkcv build --input {}", output.display());
        }
    }
    Ok(())
}

fn cmd_build(
    input: &Path,
    yaml: Option<&str>,
    output: &Path,
    template: Option<&str>,
    kind: Option<&str>,
    fmt: Format,
) -> Result<()> {
    let overrides = Overrides { template, kind };
    let report = match yaml {
        Some(y) => build::build_yaml(y, Path::new("."), output, overrides)?,
        None => build::build_once(input, output, overrides)?,
    };
    let ms = report.elapsed.as_secs_f64() * 1000.0;
    match fmt {
        Format::Json => println!(
            "{}",
            json!({ "ok": true, "output": output.display().to_string(), "pages": report.pages, "ms": ms })
        ),
        Format::Text => {
            let src = if yaml.is_some() { "<inline>".to_string() } else { input.display().to_string() };
            println!("✓ Compiled {} → {} in {:.1}ms", src, output.display(), ms);
        }
    }
    Ok(())
}

fn cmd_templates(fmt: Format) -> Result<()> {
    let entries = template::catalog();
    match fmt {
        Format::Json => {
            let arr: Vec<_> = entries
                .iter()
                .map(|t| {
                    json!({
                        "name": t.name, "category": t.category, "kind": t.kind,
                        "default": t.default, "note": t.note,
                        "preview": format!("{}{}", template::PREVIEW_BASE, t.preview),
                    })
                })
                .collect();
            println!("{}", json!({ "templates": arr }));
        }
        Format::Text => {
            println!("Available templates (set meta.template; for letters also meta.kind: cover-letter):\n");
            for cat in ["resume", "cv", "coverletter"] {
                println!("{}:", cat);
                for t in entries.iter().filter(|t| t.category == cat) {
                    let def = if t.default { "  (default)" } else { "" };
                    println!("  {:<12}{} — {}", t.name, def, t.note);
                }
                println!();
            }
            println!("Preview screenshots: {}<name>.png", template::PREVIEW_BASE);
        }
    }
    Ok(())
}

fn cmd_schema(fmt: Format) -> Result<()> {
    let schema = json!({
        "meta": {
            "template": "template name (see `templates`); or a local ./path.typ",
            "kind": "resume (default) | cover-letter",
            "color": "named preset (orange, …) or #hex",
            "accent_color": "#hex (legacy alias for color)",
            "section_highlight": "full | three-letter | none",
            "paper": "a4 | letter",
            "dark_text": "#hex override",
        },
        "profile": {
            "name": "full name (or first_name + last_name)",
            "first_name": "string", "last_name": "string",
            "title": "single tagline", "positions": ["list of positions"],
            "address": "string", "email": "string",
            "phone": "string (alias: mobile)", "location": "string",
            "website": "string (alias: homepage)",
            "github": "handle", "linkedin": "handle", "twitter": "handle",
            "quote": "italic quote",
            "photo": { "path": "image path (rel. to yaml)", "shape": "circle | rect", "side": "left | right", "edge": "bool" },
        },
        "summary": "paragraph (markup allowed)",
        "experience": [{ "company": "s", "role": "s", "location": "s", "period": "s", "bullets": ["markup"] }],
        "education": [{ "institution": "s", "degree": "s", "location": "s", "period": "s", "details": "markup" }],
        "skills": [{ "category": "s", "items": ["list"], "text": "OR free-form markup" }],
        "projects": [{ "name": "s", "description": "s", "link": "url", "bullets": ["markup"] }],
        "honors": [{ "subsection": "optional heading", "items": [{ "award": "s", "event": "s", "location": "s", "date": "s" }] }],
        "extracurricular": [{ "heading": "s", "subheading": "s", "location": "s", "date": "s", "bullets": ["markup"] }],
        "order": ["explicit section order"],
        "footer": { "left": "{today}", "center": "s", "right": "{page}" },
        "letter": {
            "recipient_name": "s", "recipient_address": "newline-separated", "date": "{today}",
            "title": "subject", "opening": "greeting",
            "sections": [{ "title": "s", "body": ["paragraphs"] }], "body": ["OR flat paragraphs"],
            "closing": "s", "enclosure": "s", "enclosure_label": "s",
        },
        "notes": "Free-text fields accept **bold**, *italic*, [links](url). Only profile name is required.",
    });
    match fmt {
        Format::Json => println!("{schema}"),
        Format::Text => println!("{}", serde_json::to_string_pretty(&schema)?),
    }
    Ok(())
}

fn cmd_validate(input: &Path, yaml: Option<&str>, fmt: Format) -> Result<()> {
    let text = match yaml {
        Some(y) => y.to_string(),
        None => std::fs::read_to_string(input)
            .with_context(|| format!("could not read input file {}", input.display()))?,
    };
    let result = parser::Resume::from_yaml(&text);
    let (ok, errors) = match &result {
        Ok(_) => (true, Vec::new()),
        Err(e) => (false, vec![format!("{e:#}")]),
    };
    match fmt {
        Format::Json => println!("{}", json!({ "ok": ok, "errors": errors })),
        Format::Text => {
            if ok {
                println!("✓ valid");
            } else {
                eprintln!("✗ {}", errors.join("; "));
            }
        }
    }
    if !ok {
        std::process::exit(1);
    }
    Ok(())
}
