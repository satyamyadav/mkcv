//! Template inserter: serialises the parsed resume into a native Typst value
//! and stitches it together with the embedded `.typ` layout source.
//!
//! Rather than doing fragile string interpolation, we emit the resume as a
//! real Typst dictionary literal bound to `data`, which the template layout
//! consumes via `data.profile.name`, `data.experience`, etc.

use anyhow::{Context, Result};
use serde_yaml::Value;

use crate::parser::Resume;

/// Font/palette-agnostic shared utilities, prepended before every template.
const CORE: &str = include_str!("../assets/templates/_core.typ");
/// Shared palette + header for the "modern"/cover-letter templates.
const PRELUDE: &str = include_str!("../assets/templates/_prelude.typ");

// Resume / CV bodies.
const MODERN: &str = include_str!("../assets/templates/modern.typ");
const CRISP: &str = include_str!("../assets/templates/crisp.typ");
const SERIF: &str = include_str!("../assets/templates/serif.typ");
const FORMAL: &str = include_str!("../assets/templates/formal.typ");
const SIDEBAR: &str = include_str!("../assets/templates/sidebar.typ");
const SPLIT: &str = include_str!("../assets/templates/split.typ");

// Cover-letter bodies (used when `meta.kind` is "cover-letter").
const COVERLETTER: &str = include_str!("../assets/templates/coverletter.typ");
const CLASSIC_LETTER: &str = include_str!("../assets/templates/classic-letter.typ");

/// Build the full Typst source string.
///
/// If `custom_body` is `Some` (a user's local `.typ` template read from disk),
/// it is used directly with only `_core.typ` prepended. Otherwise the body is
/// selected from the embedded templates by `meta.kind` + `meta.template`, with
/// the shared prelude prepended for the templates that use it.
pub fn build_source(resume: &Resume, custom_body: Option<&str>) -> Result<String> {
    let mut value =
        serde_yaml::to_value(resume).context("failed to serialise resume for rendering")?;
    // Convert light markup in free-text fields into raw Typst content.
    transform_markup(&mut value);
    // Tell the template the virtual path under which the photo is served.
    if let Some(photo) = &resume.profile.photo {
        let vpath = photo_vpath(&photo.path);
        if let Some(m) = value
            .get_mut("profile")
            .and_then(|p| p.get_mut("photo"))
            .and_then(|ph| ph.as_mapping_mut())
        {
            m.insert(Value::String("src".to_string()), Value::String(vpath));
        }
    }
    let data_literal = to_typst(&value);

    // Custom local template: `data` + core helpers + the user's body.
    if let Some(body) = custom_body {
        return Ok(format!("#let data = {data_literal}\n{CORE}\n{body}"));
    }

    let template = resume.meta.template.as_str();
    let is_letter = resume.meta.kind.as_deref() == Some("cover-letter");

    // (body, uses_shared_prelude)
    let (body, use_prelude) = if is_letter {
        // Cover-letter category: the resume template name picks the letter style.
        match template {
            "classic" => (CLASSIC_LETTER, false),
            "modern" | "crisp" | "serif" | "split" | "formal" | "sidebar" => (COVERLETTER, true),
            other => anyhow::bail!(
                "unknown cover-letter template '{other}' (available: modern, classic)"
            ),
        }
    } else {
        // Resume / CV category.
        match template {
            "modern" => (MODERN, true),
            "crisp" => (CRISP, false),
            "serif" => (SERIF, false),
            "formal" => (FORMAL, false),
            "sidebar" => (SIDEBAR, false),
            "split" => (SPLIT, false),
            other => anyhow::bail!(
                "unknown template '{other}' (available: modern, crisp, serif, \
                 split, formal, sidebar, or a local ./path.typ)"
            ),
        }
    };
    let prelude = if use_prelude { PRELUDE } else { "" };

    Ok(format!("#let data = {data_literal}\n{CORE}\n{prelude}\n{body}"))
}

/// One entry in the template catalog, for the `templates` discovery command.
pub struct TemplateInfo {
    pub name: &'static str,
    /// "resume" | "cv" | "coverletter".
    pub category: &'static str,
    /// The `meta.kind` needed to select it: "resume" | "cover-letter".
    pub kind: &'static str,
    pub default: bool,
    pub note: &'static str,
    /// Preview screenshot filename (under `PREVIEW_BASE`), so an agent can offer
    /// a visual choice. Use `PREVIEW_BASE` + this to form the URL.
    pub preview: &'static str,
}

/// Base URL for the committed preview screenshots.
pub const PREVIEW_BASE: &str =
    "https://raw.githubusercontent.com/satyamyadav/mkcv/main/skills/mkcv/previews/";

/// The full template catalog. Keep in sync with the routing in `build_source`.
pub fn catalog() -> &'static [TemplateInfo] {
    &[
        TemplateInfo { name: "modern", category: "resume", kind: "resume", default: true, note: "Minimalist, Liberation Sans.", preview: "modern.png" },
        TemplateInfo { name: "crisp", category: "resume", kind: "resume", default: false, note: "Polished professional: two-tone name, first-letters section accent, org-bold entries (Roboto + Source Sans).", preview: "crisp.png" },
        TemplateInfo { name: "serif", category: "resume", kind: "resume", default: false, note: "Single-column serif, ATS-friendly and compact.", preview: "serif.png" },
        TemplateInfo { name: "split", category: "resume", kind: "resume", default: false, note: "Two-column: big two-tone name, narrow-left / wide-right.", preview: "split.png" },
        TemplateInfo { name: "formal", category: "cv", kind: "resume", default: false, note: "Classic CV: colored header, left-margin dates.", preview: "formal.png" },
        TemplateInfo { name: "sidebar", category: "cv", kind: "resume", default: false, note: "Two-column with a colored sidebar, photo, and skill tags.", preview: "sidebar.png" },
        TemplateInfo { name: "modern", category: "coverletter", kind: "cover-letter", default: true, note: "Header-styled cover letter.", preview: "modern-letter.png" },
        TemplateInfo { name: "classic", category: "coverletter", kind: "cover-letter", default: false, note: "Formal serif business letter.", preview: "classic-letter.png" },
    ]
}

/// The virtual path under which a profile photo at `path` is served to Typst.
/// The extension is preserved so Typst can infer the image format.
pub fn photo_vpath(path: &str) -> String {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("img");
    format!("/__photo.{ext}")
}

/// Sentinel mapping key marking a value that `to_typst` should emit verbatim
/// (already-formed Typst content), rather than as a quoted string.
const RAW_KEY: &str = "__typst_raw__";

/// Convert a `serde_yaml::Value` into a Typst literal expression.
fn to_typst(value: &Value) -> String {
    match value {
        Value::Null => "none".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => escape_string(s),
        Value::Sequence(seq) => {
            if seq.is_empty() {
                // An empty Typst array is `()`; `(,)` is a syntax error.
                return "()".to_string();
            }
            let items: Vec<String> = seq.iter().map(to_typst).collect();
            // A trailing comma makes single-element arrays unambiguous in Typst.
            format!("({},)", items.join(", "))
        }
        Value::Mapping(map) => {
            // Raw-content sentinel: `{__typst_raw__: "<typst>"}` emits verbatim.
            if map.len() == 1 {
                if let Some(Value::String(raw)) = map.get(Value::String(RAW_KEY.to_string())) {
                    return raw.clone();
                }
            }
            if map.is_empty() {
                return "(:)".to_string();
            }
            let entries: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    let key = match k {
                        Value::String(s) => s.clone(),
                        other => to_typst(other),
                    };
                    format!("{}: {}", sanitize_key(&key), to_typst(v))
                })
                .collect();
            format!("({})", entries.join(", "))
        }
        // serde_yaml may surface tagged values; render their inner content.
        Value::Tagged(tagged) => to_typst(&tagged.value),
    }
}

/// Escape a string into a Typst double-quoted string literal.
fn escape_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

/// Ensure a mapping key is a valid Typst identifier; otherwise quote it.
fn sanitize_key(key: &str) -> String {
    let valid = !key.is_empty()
        && key.chars().next().map(|c| c.is_ascii_alphabetic() || c == '_').unwrap_or(false)
        && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if valid {
        key.to_string()
    } else {
        escape_string(key)
    }
}

// -----------------------------------------------------------------------------
// Inline markup (WS2)
//
// Free-text fields may use a small, safe Markdown-ish subset — `**bold**`,
// `*italic*`, and `[label](url)`. We convert these to Typst content *in Rust*
// and inject them as raw content, so no arbitrary Typst is ever evaluated on
// user input: every non-markup character is escaped.
// -----------------------------------------------------------------------------

/// Scalar fields whose string value is rendered as inline markup.
fn is_markup_scalar_key(k: &str) -> bool {
    matches!(
        k,
        "summary" | "quote" | "details" | "description" | "text" | "opening" | "closing"
    )
}

/// List fields whose string elements are each rendered as inline markup.
fn is_markup_list_key(k: &str) -> bool {
    matches!(k, "bullets" | "items" | "body")
}

/// Wrap already-formed Typst content in the raw sentinel mapping.
fn raw_value(typst: String) -> Value {
    let mut m = serde_yaml::Mapping::new();
    m.insert(Value::String(RAW_KEY.to_string()), Value::String(typst));
    Value::Mapping(m)
}

/// Walk the data tree and replace designated free-text strings with raw Typst
/// content produced from their light markup. Empty strings are left untouched
/// so the template's presence checks still work.
fn transform_markup(value: &mut Value) {
    match value {
        Value::Mapping(map) => {
            for (k, v) in map.iter_mut() {
                let key = if let Value::String(s) = k { Some(s.as_str()) } else { None };
                match key {
                    Some(k) if is_markup_scalar_key(k) => {
                        if let Value::String(s) = v {
                            if !s.is_empty() {
                                *v = raw_value(markup_to_typst(s));
                            }
                        }
                    }
                    Some(k) if is_markup_list_key(k) => {
                        if let Value::Sequence(seq) = v {
                            // A markup-list key (e.g. `items`) can hold either
                            // plain strings (convert) or objects like honors
                            // (recurse so their own fields are still processed).
                            for item in seq.iter_mut() {
                                match item {
                                    Value::String(s) if !s.is_empty() => {
                                        *item = raw_value(markup_to_typst(s));
                                    }
                                    other => transform_markup(other),
                                }
                            }
                        } else {
                            transform_markup(v);
                        }
                    }
                    _ => transform_markup(v),
                }
            }
        }
        Value::Sequence(seq) => seq.iter_mut().for_each(transform_markup),
        _ => {}
    }
}

/// Convert a markup string into a Typst content literal `[ ... ]`.
fn markup_to_typst(s: &str) -> String {
    format!("[{}]", markup_inner(s))
}

/// Convert markup to the inner Typst markup (no surrounding brackets).
fn markup_inner(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = String::with_capacity(s.len() + 8);
    let mut i = 0;
    while i < s.len() {
        // Link: [label](url)
        if b[i] == b'[' {
            if let Some((label, url, next)) = parse_link(s, i) {
                out.push_str(&format!("#link({})[{}]", escape_string(&url), markup_inner(&label)));
                i = next;
                continue;
            }
        }
        // Bold: **...**
        if b[i] == b'*' && i + 1 < s.len() && b[i + 1] == b'*' {
            if let Some((inner, next)) = parse_delim(s, i, "**") {
                out.push_str(&format!("#strong[{}]", markup_inner(&inner)));
                i = next;
                continue;
            }
        }
        // Italic: *...*
        if b[i] == b'*' {
            if let Some((inner, next)) = parse_delim(s, i, "*") {
                out.push_str(&format!("#emph[{}]", markup_inner(&inner)));
                i = next;
                continue;
            }
        }
        let ch = s[i..].chars().next().unwrap();
        push_escaped(&mut out, ch);
        i += ch.len_utf8();
    }
    out
}

/// Parse `[label](url)` starting at `open` (the `[`). Returns (label, url,
/// index past the closing `)`), or None if the shape doesn't match.
fn parse_link(s: &str, open: usize) -> Option<(String, String, usize)> {
    let bytes = s.as_bytes();
    let close = s[open + 1..].find(']')? + open + 1;
    if close + 1 >= s.len() || bytes[close + 1] != b'(' {
        return None;
    }
    let paren_open = close + 1;
    // Find the matching ')' , allowing balanced parens inside the URL (e.g.
    // a Wikipedia link ending in "(disambiguation)"). Parens are ASCII, so
    // byte scanning stays on char boundaries.
    let mut depth = 1usize;
    let mut j = paren_open + 1;
    let paren_close = loop {
        if j >= s.len() {
            return None; // unbalanced — treat '[' as literal text
        }
        match bytes[j] {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    break j;
                }
            }
            _ => {}
        }
        j += 1;
    };
    let label = s[open + 1..close].to_string();
    let url = s[paren_open + 1..paren_close].to_string();
    Some((label, url, paren_close + 1))
}

/// Parse a delimited run: `<delim>inner<delim>` starting at `start`. Returns
/// (inner, index past the closing delimiter), or None if unterminated/empty.
fn parse_delim(s: &str, start: usize, delim: &str) -> Option<(String, usize)> {
    let content_start = start + delim.len();
    let rel = s[content_start..].find(delim)?;
    if rel == 0 {
        return None; // empty run, e.g. "**" or "*"
    }
    let inner = s[content_start..content_start + rel].to_string();
    Some((inner, content_start + rel + delim.len()))
}

/// Escape a single character for Typst *markup* context.
fn push_escaped(out: &mut String, ch: char) {
    match ch {
        '\\' | '#' | '[' | ']' | '*' | '_' | '`' | '@' | '<' | '>' | '$' | '=' => {
            out.push('\\');
            out.push(ch);
        }
        _ => out.push(ch),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_is_escaped() {
        assert_eq!(markup_to_typst("a # b [c]"), "[a \\# b \\[c\\]]");
    }

    #[test]
    fn bold_italic_link() {
        assert_eq!(markup_to_typst("**b**"), "[#strong[b]]");
        assert_eq!(markup_to_typst("*i*"), "[#emph[i]]");
        assert_eq!(
            markup_to_typst("[gh](https://x.com)"),
            "[#link(\"https://x.com\")[gh]]"
        );
    }

    #[test]
    fn mixed_and_realistic() {
        assert_eq!(
            markup_to_typst("*Technology*: Rust & Typst."),
            "[#emph[Technology]: Rust & Typst.]"
        );
    }

    #[test]
    fn link_with_nested_parens() {
        assert_eq!(
            markup_to_typst("[x](https://e.com/Foo_(bar))"),
            "[#link(\"https://e.com/Foo_(bar)\")[x]]"
        );
    }

    #[test]
    fn injection_is_neutralized() {
        // A lone/unterminated marker and code-mode chars must be escaped, not run.
        assert_eq!(markup_to_typst("#set page()"), "[\\#set page()]");
        assert_eq!(markup_to_typst("a * b"), "[a \\* b]");
    }
}

