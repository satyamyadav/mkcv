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
                    Some("bullets") => {
                        // Bullets additionally support Markdown sub-bullets,
                        // which fold into a nested `#list(...)` under their
                        // parent (either indented `- ` lines within one string
                        // or separate indented `- ` array items).
                        if let Value::Sequence(seq) = v {
                            transform_bullet_seq(seq);
                        } else {
                            transform_markup(v);
                        }
                    }
                    Some(k) if is_markup_list_key(k) => {
                        if let Value::Sequence(seq) = v {
                            // `items`/`body` hold either plain strings (convert)
                            // or objects like honors (recurse so their own fields
                            // are still processed).
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

/// Convert a free-text scalar into a Typst content literal `[ ... ]`, handling
/// block structure (paragraphs and hard line breaks) as well as inline markup.
fn markup_to_typst(s: &str) -> String {
    format!("[{}]", blocks_to_typst(s))
}

/// Convert a multi-line free-text value into Typst content: a blank line becomes
/// `#parbreak()`, a hard line break (a line ending in `\` or two spaces) becomes
/// `#linebreak()`, and inline markup within each line is converted. A plain
/// single-line string passes straight through `markup_inner`.
fn blocks_to_typst(s: &str) -> String {
    let paras = split_paragraphs(s);
    paras
        .iter()
        .map(|p| paragraph_to_typst(p))
        .collect::<Vec<_>>()
        .join("#parbreak()")
}

/// Split text into paragraphs on blank (whitespace-only) lines. Always returns
/// at least one (possibly empty) paragraph so callers get a stable shape.
fn split_paragraphs(s: &str) -> Vec<String> {
    let mut paras = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    for line in s.lines() {
        if line.trim().is_empty() {
            if !cur.is_empty() {
                paras.push(cur.join("\n"));
                cur.clear();
            }
        } else {
            cur.push(line);
        }
    }
    if !cur.is_empty() {
        paras.push(cur.join("\n"));
    }
    if paras.is_empty() {
        paras.push(String::new());
    }
    paras
}

/// Render one paragraph: soft line wraps become spaces; a line ending in `\` or
/// two spaces becomes a `#linebreak()`.
fn paragraph_to_typst(p: &str) -> String {
    let lines: Vec<&str> = p.lines().collect();
    let mut out = String::new();
    for (idx, line) in lines.iter().enumerate() {
        let last = idx + 1 == lines.len();
        // A hard-break marker only makes sense when another line follows; on the
        // final line, keep the text as-is so a trailing `\` isn't silently lost.
        let (text, hard) = if last { (*line, false) } else { strip_hard_break(line) };
        out.push_str(&markup_inner(text.trim_end()));
        if !last {
            out.push_str(if hard { "#linebreak()" } else { " " });
        }
    }
    out
}

/// Detect a Markdown hard line break: a trailing backslash or two trailing
/// spaces. Returns the line without the break marker and whether it was hard.
fn strip_hard_break(line: &str) -> (&str, bool) {
    if let Some(stripped) = line.strip_suffix('\\') {
        (stripped, true)
    } else if line.ends_with("  ") {
        (line.trim_end(), true)
    } else {
        (line, false)
    }
}

/// Convert one line of inline markup (no newlines) to inner Typst content.
/// Supports `[label](url)`, `` `code` ``, `**b**`/`__b__`, `*i*`/`_i_`, and
/// `~~strike~~`. Every non-markup character is escaped, so no arbitrary Typst
/// is ever evaluated from user input.
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
        // Inline code: `code` (content taken literally, not re-parsed).
        if b[i] == b'`' {
            if let Some((code, next)) = parse_code(s, i) {
                out.push_str(&format!("#raw({})", escape_string(&code)));
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
        // Bold: __...__ (underscore; guarded so `snake_case` stays literal).
        if b[i] == b'_' && i + 1 < s.len() && b[i + 1] == b'_' && underscore_open_ok(s, i) {
            if let Some((inner, next)) = parse_delim_guarded(s, i, "__") {
                out.push_str(&format!("#strong[{}]", markup_inner(&inner)));
                i = next;
                continue;
            }
        }
        // Strikethrough: ~~...~~
        if b[i] == b'~' && i + 1 < s.len() && b[i + 1] == b'~' {
            if let Some((inner, next)) = parse_delim(s, i, "~~") {
                out.push_str(&format!("#strike[{}]", markup_inner(&inner)));
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
        // Italic: _..._ (underscore; guarded so `snake_case` stays literal).
        if b[i] == b'_' && underscore_open_ok(s, i) {
            if let Some((inner, next)) = parse_delim_guarded(s, i, "_") {
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

/// Parse an inline-code span at `start` (a run of N backticks matched by an
/// equal-length run). Returns (code, index past the closing run) or None.
fn parse_code(s: &str, start: usize) -> Option<(String, usize)> {
    let b = s.as_bytes();
    let mut run = 0;
    while start + run < s.len() && b[start + run] == b'`' {
        run += 1;
    }
    let fence = "`".repeat(run);
    let content_start = start + run;
    let rel = s[content_start..].find(&fence)?;
    if rel == 0 {
        return None; // empty span, e.g. ``
    }
    let code = s[content_start..content_start + rel].trim().to_string();
    Some((code, content_start + rel + run))
}

/// Whether an underscore emphasis run may *open* at `at`: only at the start or
/// after whitespace or an opening bracket/quote. This is a simplified CommonMark
/// left-flank rule that keeps intraword underscores (`snake_case`) literal.
fn underscore_open_ok(s: &str, at: usize) -> bool {
    match s[..at].chars().next_back() {
        None => true,
        Some(c) => c.is_whitespace() || matches!(c, '(' | '[' | '{' | '"' | '\''),
    }
}

/// Like `parse_delim`, but additionally requires the closing delimiter to be at
/// a right word boundary (end, whitespace, or closing punctuation) and the run
/// to not be padded with spaces — so `_i_` emphasizes but `a _ b` does not.
fn parse_delim_guarded(s: &str, start: usize, delim: &str) -> Option<(String, usize)> {
    let (inner, next) = parse_delim(s, start, delim)?;
    if inner.starts_with(char::is_whitespace) || inner.ends_with(char::is_whitespace) {
        return None;
    }
    let ok_right = match s[next..].chars().next() {
        None => true,
        Some(c) => c.is_whitespace() || matches!(c, '.' | ',' | ';' | ':' | '!' | '?' | ')' | ']' | '}' | '"' | '\''),
    };
    if ok_right {
        Some((inner, next))
    } else {
        None
    }
}

/// Convert a `bullets` sequence, folding Markdown sub-bullets into a nested
/// `#list(...)` under their parent. Sub-bullets come either as indented `- `
/// lines inside a single bullet string, or as separate indented `- ` array
/// items following their parent. Non-string items (e.g. honor objects) are
/// recursed and preserved in place.
fn transform_bullet_seq(seq: &mut Vec<Value>) {
    // Accumulated top-level bullets as (main text, sub-bullet texts).
    let mut pending: Vec<(String, Vec<String>)> = Vec::new();
    let mut out: Vec<Value> = Vec::with_capacity(seq.len());

    let flush = |pending: &mut Vec<(String, Vec<String>)>, out: &mut Vec<Value>| {
        for (main, subs) in pending.drain(..) {
            let mut inner = markup_inner(&main);
            if !subs.is_empty() {
                let items: Vec<String> =
                    subs.iter().map(|x| format!("[{}]", markup_inner(x))).collect();
                if !inner.is_empty() {
                    inner.push(' ');
                }
                inner.push_str(&format!("#list({})", items.join(", ")));
            }
            out.push(raw_value(format!("[{}]", inner)));
        }
    };

    for item in std::mem::take(seq) {
        match item {
            Value::String(s) if !s.is_empty() => {
                // A separate array item that is an indented `- ` line attaches
                // to the previous top-level bullet.
                if let Some(sub) = as_indented_subbullet(&s) {
                    if let Some(last) = pending.last_mut() {
                        last.1.push(sub);
                        continue;
                    }
                    pending.push((sub, Vec::new()));
                } else {
                    let (main, subs) = split_bullet(&s);
                    pending.push((main, subs));
                }
            }
            other => {
                flush(&mut pending, &mut out);
                let mut o = other;
                transform_markup(&mut o);
                out.push(o);
            }
        }
    }
    flush(&mut pending, &mut out);
    *seq = out;
}

/// Split a single bullet string into (main text, sub-bullet texts): the first
/// line(s) are the main text (soft-joined), and any `- `/`* ` lines after that
/// become nested list items.
fn split_bullet(s: &str) -> (String, Vec<String>) {
    let mut main: Vec<String> = Vec::new();
    let mut subs: Vec<String> = Vec::new();
    for line in s.lines() {
        match strip_bullet_marker(line) {
            // A `- ` line after the main text has started is a sub-bullet.
            Some(rest) if !main.is_empty() || !subs.is_empty() => subs.push(rest),
            // A `- ` line that opens the string is the parent's own text — strip
            // the marker so it isn't re-interpreted as a Typst list item.
            Some(rest) => main.push(rest),
            None => main.push(line.trim().to_string()),
        }
    }
    (main.join(" ").trim().to_string(), subs)
}

/// If `line` begins (after optional whitespace) with a `- ` or `* ` marker,
/// return the text after it.
fn strip_bullet_marker(line: &str) -> Option<String> {
    let t = line.trim_start();
    for m in ["- ", "* "] {
        if let Some(rest) = t.strip_prefix(m) {
            return Some(rest.trim().to_string());
        }
    }
    None
}

/// A standalone array item counts as a sub-bullet only when it is *indented*
/// and marked — so a normal top-level bullet is never accidentally demoted.
fn as_indented_subbullet(s: &str) -> Option<String> {
    if s.starts_with([' ', '\t']) {
        strip_bullet_marker(s)
    } else {
        None
    }
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
        '\\' | '#' | '[' | ']' | '*' | '_' | '`' | '@' | '<' | '>' | '$' | '=' | '~' => {
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
        // A lone tilde is a non-breaking space in Typst — must be escaped.
        assert_eq!(markup_to_typst("~1000 users"), "[\\~1000 users]");
    }

    #[test]
    fn inline_code() {
        assert_eq!(markup_to_typst("run `cargo test`"), "[run #raw(\"cargo test\")]");
        // Content is literal: markup chars inside code are not interpreted.
        assert_eq!(markup_to_typst("`a*b`"), "[#raw(\"a*b\")]");
    }

    #[test]
    fn strikethrough() {
        assert_eq!(markup_to_typst("~~old~~ new"), "[#strike[old] new]");
    }

    #[test]
    fn underscore_emphasis_and_intraword() {
        assert_eq!(markup_to_typst("_i_"), "[#emph[i]]");
        assert_eq!(markup_to_typst("__b__"), "[#strong[b]]");
        // Intraword underscores stay literal (snake_case must not emphasize).
        assert_eq!(markup_to_typst("snake_case_name"), "[snake\\_case\\_name]");
    }

    #[test]
    fn paragraphs_and_line_breaks() {
        // Blank line -> parbreak.
        assert_eq!(markup_to_typst("a\n\nb"), "[a#parbreak()b]");
        // Soft wrap -> space.
        assert_eq!(markup_to_typst("a\nb"), "[a b]");
        // Trailing backslash -> hard line break.
        assert_eq!(markup_to_typst("a\\\nb"), "[a#linebreak()b]");
    }

    fn raw_of(v: &Value) -> String {
        if let Value::Mapping(m) = v {
            if let Some(Value::String(raw)) = m.get(Value::String(RAW_KEY.to_string())) {
                return raw.clone();
            }
        }
        panic!("not a raw value: {v:?}");
    }

    #[test]
    fn sub_bullets_within_a_string() {
        let mut seq = vec![Value::String("Led migration\n- moved db\n- no downtime".into())];
        transform_bullet_seq(&mut seq);
        assert_eq!(seq.len(), 1);
        assert_eq!(raw_of(&seq[0]), "[Led migration #list([moved db], [no downtime])]");
    }

    #[test]
    fn sub_bullets_as_indented_items() {
        let mut seq = vec![
            Value::String("Led migration".into()),
            Value::String("  - moved db".into()),
            Value::String("  - no downtime".into()),
        ];
        transform_bullet_seq(&mut seq);
        assert_eq!(seq.len(), 1);
        assert_eq!(raw_of(&seq[0]), "[Led migration #list([moved db], [no downtime])]");
    }

    #[test]
    fn plain_bullet_unchanged_shape() {
        let mut seq = vec![Value::String("Shipped **feature** X".into())];
        transform_bullet_seq(&mut seq);
        assert_eq!(raw_of(&seq[0]), "[Shipped #strong[feature] X]");
    }

    #[test]
    fn leading_marker_on_parent_line_is_stripped() {
        // A parent bullet written with its own leading `- ` must not leak the
        // marker into the content (it would render as a stray Typst list item).
        let mut seq = vec![Value::String("- Led migration\n- moved db".into())];
        transform_bullet_seq(&mut seq);
        assert_eq!(raw_of(&seq[0]), "[Led migration #list([moved db])]");
    }

    #[test]
    fn trailing_backslash_on_final_line_is_kept() {
        // A trailing backslash on the last line must be preserved, not dropped.
        assert_eq!(markup_to_typst("a\\"), "[a\\\\]");
        assert_eq!(markup_to_typst("path C:\\"), "[path C:\\\\]");
    }
}



