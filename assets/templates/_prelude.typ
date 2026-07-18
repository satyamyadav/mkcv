// _prelude.typ — shared setup for the "modern" and cover-letter templates.
//
// The engine prepends `#let data = (...)` and `_core.typ` before this file, so
// the helpers, `accent`, `presets`, `resolve-color`, `paper`, `footer-part`,
// and `ti-glyphs` are already in scope. This file adds the modern palette,
// header, and section kit.

// --- Modern palette ----------------------------------------------------------
#let darktext = rgb(orelse(data.meta.at("dark_text", default: none), "#333333"))
#let gray = rgb("#5d5d5d")
#let lightgray = rgb("#999999")
#let sec-mode = orelse(data.meta.at("section_highlight", default: none), "full")

// --- Full name ---------------------------------------------------------------
#let full-name = {
  let n = orelse(p.at("name", default: none), "")
  if n != "" { n } else {
    (orelse(p.at("first_name", default: none), "") + " " + orelse(p.at("last_name", default: none), "")).trim()
  }
}

// --- Icons -------------------------------------------------------------------
// Contact codepoints from core, plus a chevron bullet marker.
#let glyphs = ti-glyphs + (bullet: "\u{ea61}") // chevron-right
#let icon(name, size: 9pt, fill: black) = {
  text(font: "tabler-icons", size: size, fill: fill)[#glyphs.at(name)]
}

// --- Document & page ---------------------------------------------------------
#let doc-suffix = if kind == "cover-letter" { " — Cover Letter" } else { " — Resume" }
#set document(title: full-name + doc-suffix, author: full-name)

// Footer: three parts, each substituted via core's `footer-part`.
#let footer-content = {
  let f = data.at("footer", default: none)
  if f == none { none } else {
    set text(size: 8pt, fill: lightgray, tracking: 0.4pt)
    smallcaps(grid(
      columns: (1fr, 1fr, 1fr),
      align: (left + horizon, center + horizon, right + horizon),
      footer-part(f.at("left", default: none)),
      footer-part(f.at("center", default: none)),
      footer-part(f.at("right", default: none)),
    ))
  }
}

#set page(paper: paper, margin: (x: 1.4cm, top: 1.5cm, bottom: 1.4cm), footer: footer-content)
#set text(font: "Liberation Sans", size: 9pt, fill: rgb("#333333"))
#set par(justify: false, leading: 0.6em)

// --- Shared header -----------------------------------------------------------

#let rule() = line(length: 100%, stroke: 0.5pt + rgb("#cccccc"))

// A section heading with the accent treatment + a trailing rule. Shared by the
// resume `section()` and the cover-letter `lettersection()`.
#let heading-with-rule(title, size: 12pt, rule-color: rgb("#cccccc"), rule-weight: 0.5pt) = {
  let t = upper(title)
  if sec-mode == "none" {
    text(size: size, weight: "bold", fill: darktext, tracking: 0.5pt)[#t]
  } else if sec-mode == "three-letter" {
    let cl = t.clusters()
    let k = calc.min(3, cl.len())
    let head = cl.slice(0, k).join()
    let rest = if cl.len() > k { cl.slice(k).join() } else { "" }
    text(size: size, weight: "bold", tracking: 0.5pt)[#text(fill: accent)[#head]#text(fill: darktext)[#rest]]
  } else {
    text(size: size, weight: "bold", fill: accent, tracking: 0.5pt)[#t]
  }
  v(2pt)
  line(length: 100%, stroke: rule-weight + rule-color)
  v(4pt)
}

#let contact-item(icon-name, value, link-target: none) = {
  if has(value) {
    let content = box(baseline: 20%)[
      #icon(icon-name, size: 9pt, fill: accent)#h(3pt)#value
    ]
    if link-target != none { link(link-target)[#content] } else { content }
  }
}

// Render the profile photo (served by the engine at photo.src).
#let photo-block(ph) = {
  let dim = 2.6cm
  let radius = if orelse(ph.at("shape", default: none), "circle") == "rect" { 4pt } else { 50% }
  let stroke = if ph.at("edge", default: false) == true { 1pt + accent } else { none }
  box(
    clip: true,
    radius: radius,
    stroke: stroke,
    width: dim,
    height: dim,
    image(ph.src, width: dim, height: dim, fit: "cover"),
  )
}

// The header text (name, positions, address, contacts, quote), with no outer
// alignment set — the caller places it (centered, or beside a photo).
#let header-text() = {
  // Name: light throughout, or light first + bold last when split.
  let name-block = {
    let n = orelse(p.at("name", default: none), "")
    if n != "" {
      text(size: 26pt, weight: "light", fill: rgb("#333333"))[#n]
    } else {
      text(size: 26pt)[
        #text(weight: "light", fill: rgb("#333333"))[#orelse(p.at("first_name", default: none), "")]
        #text(weight: "bold", fill: darktext)[#orelse(p.at("last_name", default: none), "")]
      ]
    }
  }
  name-block

  // Positions (joined by " · "), or the single title.
  let positions = p.at("positions", default: ())
  let pos-line = if positions.len() > 0 {
    positions.map(x => upper(x)).join(text(fill: lightgray)[ · ])
  } else if has(p.at("title", default: none)) { upper(p.title) } else { none }
  if pos-line != none {
    v(2pt)
    text(size: 10pt, fill: gray, tracking: 1pt)[#pos-line]
  }

  if has(p.at("address", default: none)) {
    v(2pt)
    text(size: 8pt, style: "italic", fill: lightgray)[#p.address]
  }
  v(6pt)

  let phone = firstof(p.at("phone", default: none), p.at("mobile", default: none))
  let website = firstof(p.at("website", default: none), p.at("homepage", default: none))
  let items = ()
  if has(p.at("location", default: none)) { items.push(contact-item("location", p.location)) }
  if has(phone) { items.push(contact-item("phone", phone)) }
  if has(p.at("email", default: none)) {
    items.push(contact-item("mail", p.email, link-target: "mailto:" + p.email))
  }
  if has(website) { items.push(contact-item("website", website, link-target: website)) }
  if has(p.at("github", default: none)) {
    items.push(contact-item("github", "github.com/" + p.github, link-target: "https://github.com/" + p.github))
  }
  if has(p.at("linkedin", default: none)) {
    items.push(contact-item("linkedin", "linkedin.com/in/" + p.linkedin, link-target: "https://linkedin.com/in/" + p.linkedin))
  }
  if has(p.at("twitter", default: none)) {
    items.push(contact-item("twitter", "@" + p.twitter, link-target: "https://twitter.com/" + p.twitter))
  }

  block[
    #set text(size: 8.5pt, fill: gray)
    #items.filter(x => x != none).join(text(fill: lightgray)[#h(6pt) | #h(6pt)])
  ]

  if has(p.at("quote", default: none)) {
    v(6pt)
    text(size: 9pt, style: "italic", fill: darktext)[“#p.quote”]
  }
}

// Header: centered text, or text beside a profile photo when one is set.
#let header() = {
  let ph = p.at("photo", default: none)
  if ph != none and has(ph.at("src", default: none)) {
    let side = orelse(ph.at("side", default: none), "right")
    let text-col = align(left, header-text())
    let photo-col = align(center + horizon, photo-block(ph))
    if side == "left" {
      grid(columns: (auto, 1fr), column-gutter: 14pt, align: horizon, photo-col, text-col)
    } else {
      grid(columns: (1fr, auto), column-gutter: 14pt, align: horizon, text-col, photo-col)
    }
  } else {
    align(center, header-text())
  }
}
