// _core.typ — font/palette-agnostic shared utilities, prepended before every
// template. Owns the pieces that were previously duplicated across
// `_prelude.typ` and `crisp.typ`.

// --- Small helpers -----------------------------------------------------------

#let has(v) = v != none and v != ""
#let firstof(a, b) = if has(a) { a } else if has(b) { b } else { none }
#let orelse(v, d) = if has(v) { v } else { d }

#let p = data.profile
#let kind = orelse(data.meta.at("kind", default: none), "resume")

// --- Color presets + resolution ----------------------------------------------

#let presets = (
  "emerald": "#00A388",
  "skyblue": "#0395DE",
  "red": "#DC3522",
  "pink": "#EF4089",
  "orange": "#FF6138",
  "nephritis": "#27AE60",
  "concrete": "#95A5A6",
  "darknight": "#131A28",
)
// `v` is a named preset or a "#hex".
#let resolve-color(v, fallback: "#DC3522") = {
  let c = orelse(v, fallback)
  if c.starts-with("#") { rgb(c) } else { rgb(presets.at(c, default: fallback)) }
}
// The resolved accent color for this document (`meta.color` or `meta.accent_color`).
#let accent = resolve-color(
  firstof(data.meta.at("color", default: none), data.meta.at("accent_color", default: none)),
)

// --- Page helpers ------------------------------------------------------------

#let paper = if orelse(data.meta.at("paper", default: none), "a4") == "letter" { "us-letter" } else { "a4" }

// Footer token substitution: `{today}` / `{page}`, else literal text.
#let footer-part(s) = {
  if not has(s) { return [] }
  if s == "{today}" {
    let d = datetime.today()
    if d != none { d.display("[day padding:none] [month repr:short] [year]") } else { [] }
  } else if s == "{page}" {
    context counter(page).display()
  } else { s }
}

// --- Icons (Tabler Icons, family "tabler-icons") -----------------------------
// Contact-field codepoints from Tabler Icons 3.45.0 (the exact bundled font).
// Templates extend this with their own bullet/marker glyphs.
#let ti-glyphs = (
  location: "\u{eae8}", // map-pin
  phone: "\u{eb09}",    // phone
  mail: "\u{eae5}",     // mail
  website: "\u{f38f}",  // world-www
  github: "\u{ec1c}",   // brand-github
  linkedin: "\u{ec8c}", // brand-linkedin
  twitter: "\u{ec27}",  // brand-twitter
)
