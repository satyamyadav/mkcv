//! Serde schemas mapping the YAML resume file into strongly-typed structures.
//!
//! Optional fields default to empty so the layout can adapt dynamically
//! rather than rendering blank placeholders.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resume {
    #[serde(default)]
    pub meta: Meta,
    pub profile: Profile,
    /// Explicit section render order. Empty means the template's default order.
    #[serde(default)]
    pub order: Vec<String>,
    /// Optional three-part page footer.
    #[serde(default)]
    pub footer: Option<Footer>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub experience: Vec<Experience>,
    #[serde(default)]
    pub education: Vec<Education>,
    #[serde(default)]
    pub skills: Vec<SkillGroup>,
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub honors: Vec<HonorGroup>,
    #[serde(default)]
    pub extracurricular: Vec<Entry>,
    /// Cover-letter content (used when `meta.kind` is "cover-letter").
    #[serde(default)]
    pub letter: Option<Letter>,
}

/// Cover-letter content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Letter {
    #[serde(default)]
    pub recipient_name: Option<String>,
    /// Recipient address; newline-separated lines are stacked.
    #[serde(default)]
    pub recipient_address: Option<String>,
    /// Letter date; `{today}` is substituted.
    #[serde(default)]
    pub date: Option<String>,
    /// Subject / title line (underlined).
    #[serde(default)]
    pub title: Option<String>,
    /// Greeting, e.g. "Dear Hiring Manager,".
    #[serde(default)]
    pub opening: Option<String>,
    /// Titled body sections (each with its own paragraphs).
    #[serde(default)]
    pub sections: Vec<LetterSection>,
    /// Flat body paragraphs (used when `sections` is empty).
    #[serde(default)]
    pub body: Vec<String>,
    /// Sign-off, e.g. "Sincerely,".
    #[serde(default)]
    pub closing: Option<String>,
    #[serde(default)]
    pub enclosure: Option<String>,
    /// Label preceding the enclosure (default "Enclosure").
    #[serde(default)]
    pub enclosure_label: Option<String>,
}

/// A titled section within a cover letter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetterSection {
    pub title: String,
    #[serde(default)]
    pub body: Vec<String>,
}

/// Three-part footer. Each cell may contain `{today}` / `{page}` placeholders,
/// which the template substitutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Footer {
    #[serde(default)]
    pub left: Option<String>,
    #[serde(default)]
    pub center: Option<String>,
    #[serde(default)]
    pub right: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    #[serde(default = "default_template")]
    pub template: String,
    #[serde(default = "default_language")]
    pub language: String,
    /// Accent color as a hex string, e.g. "#dc3522". Kept for back-compat;
    /// prefer `color`, which also accepts named presets. When unset, each
    /// template applies its own default accent.
    #[serde(default)]
    pub accent_color: Option<String>,
    /// Document kind: "resume" (default) or "cover-letter".
    #[serde(default)]
    pub kind: Option<String>,
    /// Named color preset (e.g. "orange") or a `#hex`. Overrides
    /// `accent_color` when set.
    #[serde(default)]
    pub color: Option<String>,
    /// Section-title highlighting: "full" | "three-letter" | "none".
    #[serde(default)]
    pub section_highlight: Option<String>,
    /// Paper size: "a4" | "letter".
    #[serde(default)]
    pub paper: Option<String>,
    /// Optional override for the dark heading/title text color (`#hex`).
    #[serde(default)]
    pub dark_text: Option<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            template: default_template(),
            language: default_language(),
            accent_color: None,
            kind: None,
            color: None,
            section_highlight: None,
            paper: None,
            dark_text: None,
        }
    }
}

fn default_template() -> String {
    "modern".to_string()
}
fn default_language() -> String {
    "en".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Full name. Optional if `first_name`/`last_name` are given instead.
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    /// Single tagline (back-compat) — `positions` takes precedence when set.
    #[serde(default)]
    pub title: Option<String>,
    /// One or more positions, rendered joined by " · ".
    #[serde(default)]
    pub positions: Vec<String>,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    /// Alias for `phone` (alternate name).
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    /// Alias for `website` (alternate name).
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub github: Option<String>,
    #[serde(default)]
    pub linkedin: Option<String>,
    #[serde(default)]
    pub twitter: Option<String>,
    /// Optional italic quote under the header.
    #[serde(default)]
    pub quote: Option<String>,
    /// Optional profile photo.
    #[serde(default)]
    pub photo: Option<Photo>,
}

/// A profile photo shown in the header.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    /// Image path, relative to the input YAML file.
    pub path: String,
    /// "circle" (default) or "rect".
    #[serde(default)]
    pub shape: Option<String>,
    /// "left" or "right" (default).
    #[serde(default)]
    pub side: Option<String>,
    /// Draw a thin accent border around the photo.
    #[serde(default)]
    pub edge: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub company: String,
    pub role: String,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub period: Option<String>,
    #[serde(default)]
    pub bullets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub institution: String,
    #[serde(default)]
    pub degree: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub period: Option<String>,
    #[serde(default)]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillGroup {
    pub category: String,
    /// A list of discrete skills (joined by " · ").
    #[serde(default)]
    pub items: Vec<String>,
    /// Free-form skill text (markup allowed). Takes precedence over `items`.
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub bullets: Vec<String>,
}

/// A generic CV entry (used by `extracurricular`): a bold heading, a small-caps
/// subheading, an accent location, a date, and bullet points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub heading: String,
    #[serde(default)]
    pub subheading: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub bullets: Vec<String>,
}

/// A group of honors, optionally under a subsection heading (e.g.
/// "International" / "Domestic").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HonorGroup {
    #[serde(default)]
    pub subsection: Option<String>,
    #[serde(default)]
    pub items: Vec<Honor>,
}

/// A single honor/award line: `<award>, <event> ... <location> <date>`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Honor {
    pub award: String,
    #[serde(default)]
    pub event: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
}

impl Resume {
    /// Parse a resume from a YAML string.
    pub fn from_yaml(yaml: &str) -> Result<Resume> {
        let resume: Resume = serde_yaml::from_str(yaml)
            .context("failed to parse resume YAML (check schema and indentation)")?;
        let pr = &resume.profile;
        if !has(&pr.name) && !has(&pr.first_name) && !has(&pr.last_name) {
            anyhow::bail!("profile is missing a name: set `name`, or `first_name`/`last_name`");
        }
        Ok(resume)
    }
}

/// True if an optional string is present and non-empty.
fn has(v: &Option<String>) -> bool {
    v.as_deref().is_some_and(|s| !s.trim().is_empty())
}
