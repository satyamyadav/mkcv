// split.typ — a two-column resume: a big two-tone name, then a narrow left
// column (education, skills, honors) beside a wide right column (experience,
// projects).
//
// Prepended with `#let data = (...)` and `_core.typ`.

#let accent2 = resolve-color(
  firstof(data.meta.at("color", default: none), data.meta.at("accent_color", default: none)),
  fallback: "#2C3E50",
)
#let sans = "Source Sans 3"
#let dark = rgb("#2b2b2b")
#let gray = rgb("#5d5d5d")
#let light = rgb("#8a8a8a")

#let first-name = {
  let n = orelse(p.at("name", default: none), "")
  if n != "" { n.split(" ").first() } else { orelse(p.at("first_name", default: none), "") }
}
#let last-name = {
  let n = orelse(p.at("name", default: none), "")
  if n != "" { n.split(" ").slice(1).join(" ") } else { orelse(p.at("last_name", default: none), "") }
}
#let full-name = (first-name + " " + last-name).trim()

#set document(title: full-name + " — Resume", author: full-name)
#set page(paper: paper, margin: (x: 1.4cm, top: 1.2cm, bottom: 1.2cm),
  footer: context {
    let f = data.at("footer", default: none)
    if f != none {
      set text(font: sans, size: 8pt, fill: light)
      grid(columns: (1fr, 1fr, 1fr), align: (left, center, right),
        footer-part(f.at("left", default: none)),
        footer-part(f.at("center", default: none)),
        footer-part(f.at("right", default: none)))
    }
  })
#set text(font: sans, size: 9pt, fill: dark)
#set par(leading: 0.58em)

#let ticon(name, ..a) = text(font: "tabler-icons", ..a)[#ti-glyphs.at(name)]

// --- Header (big two-tone name) -----------------------------

#{
  text(size: 32pt, weight: "light", fill: gray)[#first-name]
  text(size: 32pt, weight: "bold", fill: dark)[ #last-name]
}
#{
  let positions = p.at("positions", default: ())
  let title = if positions.len() > 0 { positions.join("  //  ") } else { orelse(p.at("title", default: none), "") }
  let phone = firstof(p.at("phone", default: none), p.at("mobile", default: none))
  let site = firstof(p.at("website", default: none), p.at("homepage", default: none))
  let bits = ()
  if has(p.at("email", default: none)) { bits.push(link("mailto:" + p.email)[#p.email]) }
  if has(phone) { bits.push([#phone]) }
  if has(site) { bits.push(link(site)[#site]) }
  if has(p.at("github", default: none)) { bits.push(link("https://github.com/" + p.github)[#p.github]) }
  if has(p.at("linkedin", default: none)) { bits.push(link("https://linkedin.com/in/" + p.linkedin)[#p.linkedin]) }
  v(1pt)
  grid(columns: (1fr, auto), align: (left + horizon, right + horizon), column-gutter: 1em,
    if title != "" { text(size: 10pt, fill: accent2)[#smallcaps(title)] } else { [] },
    text(size: 8pt, fill: gray)[#bits.join(text(fill: light)[  ·  ])])
}
#v(4pt)
#line(length: 100%, stroke: 1.2pt + accent2)
#v(6pt)

// --- Section + entry helpers -------------------------------------------------

#let section(title) = {
  v(4pt)
  text(size: 12pt, weight: "bold", fill: accent2)[#smallcaps(title)]
  v(2pt)
}

#let entry(title: none, subtitle: none, location: none, date: none, body: none) = {
  grid(columns: (1fr, auto), align: (left, right),
    text(size: 10pt, weight: "bold")[#orelse(title, "")],
    text(size: 8.5pt, fill: gray)[#orelse(date, "")])
  if has(subtitle) or has(location) {
    grid(columns: (1fr, auto), align: (left, right),
      text(size: 9pt, fill: accent2, style: "italic")[#orelse(subtitle, "")],
      text(size: 8.5pt, fill: light, style: "italic")[#orelse(location, "")])
  }
  if body != none { body }
  v(3pt)
}

#let bullets(items) = if items.len() > 0 {
  v(1pt); set text(size: 8.5pt, fill: rgb("#3a3a3a")); set par(justify: true, leading: 0.48em)
  for b in items { grid(columns: (0.9em, 1fr), align: (left + top, left), text(fill: accent2)[•], [#b]); v(0.3pt) }
}

// --- Renderers: right (wide) column ------------------------------------------

#let r-summary() = if has(data.at("summary", default: none)) { section("Profile"); set par(justify: true); text(size: 9pt)[#data.summary]; v(3pt) }
#let r-experience() = if data.at("experience", default: ()).len() > 0 {
  section("Experience")
  for e in data.experience { entry(title: e.company, subtitle: e.at("role", default: none), location: e.at("location", default: none), date: e.at("period", default: none), body: bullets(e.at("bullets", default: ()))) }
}
#let r-projects() = if data.at("projects", default: ()).len() > 0 {
  section("Projects")
  for pr in data.projects { entry(title: pr.name, subtitle: pr.at("description", default: none), date: if has(pr.at("link", default: none)) { pr.link } else { none }, body: bullets(pr.at("bullets", default: ()))) }
}
#let r-extracurricular() = if data.at("extracurricular", default: ()).len() > 0 {
  section("Extracurricular")
  for e in data.extracurricular { entry(title: e.heading, subtitle: e.at("subheading", default: none), location: e.at("location", default: none), date: e.at("date", default: none), body: bullets(e.at("bullets", default: ()))) }
}

// --- Renderers: left (narrow) column -----------------------------------------

#let r-education() = if data.at("education", default: ()).len() > 0 {
  section("Education")
  for ed in data.education {
    text(size: 10pt, weight: "bold")[#ed.institution]; linebreak()
    if has(ed.at("degree", default: none)) { text(size: 9pt, fill: accent2, style: "italic")[#ed.degree]; linebreak() }
    set text(size: 8pt, fill: light)
    [#ed.at("location", default: "")]
    if has(ed.at("location", default: none)) and has(ed.at("period", default: none)) { [ · ] }
    [#ed.at("period", default: "")]
    if has(ed.at("details", default: none)) { linebreak(); text(size: 8.5pt, fill: gray)[#ed.details] }
    v(4pt)
  }
}
#let r-skills() = if data.at("skills", default: ()).len() > 0 {
  section("Skills")
  for g in data.skills {
    text(size: 9pt, weight: "bold", fill: accent2)[#g.category]; linebreak()
    let value = if has(g.at("text", default: none)) { g.text } else { g.at("items", default: ()).join(" · ") }
    text(size: 8.5pt)[#value]
    v(4pt)
  }
}
#let r-honors() = if data.at("honors", default: ()).len() > 0 {
  section("Honors")
  for grp in data.honors {
    for h in grp.at("items", default: ()) {
      text(size: 9pt, weight: "bold")[#h.award]
      if has(h.at("date", default: none)) { text(size: 8pt, fill: light)[ · #h.date] }
      linebreak()
      text(size: 8.5pt, fill: gray)[#h.at("event", default: "")]
      v(3pt)
    }
  }
}

// --- Two-column assembly -----------------------------------------------------

#let left-renderers = (education: r-education, skills: r-skills, honors: r-honors)
#let right-renderers = (summary: r-summary, experience: r-experience, projects: r-projects, extracurricular: r-extracurricular)

#grid(columns: (0.82fr, 1.18fr), column-gutter: 1cm,
  { for (k, f) in left-renderers { f() } },
  { for (k, f) in right-renderers { f() } },
)
