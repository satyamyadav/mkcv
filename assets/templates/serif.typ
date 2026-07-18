// serif.typ — a clean, single-column, ATS-friendly resume set in a Times-like
// serif face: centered name, a contact line, ruled small-caps section headers.
//
// Prepended with `#let data = (...)` and `_core.typ`.

#let serif = "Liberation Serif"
#let ink = rgb("#000000")
#let full-name = {
  let n = orelse(p.at("name", default: none), "")
  if n != "" { n } else {
    (orelse(p.at("first_name", default: none), "") + " " + orelse(p.at("last_name", default: none), "")).trim()
  }
}

#set document(title: full-name + " — Resume", author: full-name)
#set page(
  paper: paper,
  margin: (x: 1.5cm, top: 1.1cm, bottom: 1.1cm),
  footer: context {
    let f = data.at("footer", default: none)
    if f != none {
      set text(font: serif, size: 8pt, fill: rgb("#666666"))
      grid(columns: (1fr, 1fr, 1fr), align: (left, center, right),
        footer-part(f.at("left", default: none)),
        footer-part(f.at("center", default: none)),
        footer-part(f.at("right", default: none)))
    }
  },
)
#set text(font: serif, size: 10pt, fill: ink)
#set par(leading: 0.55em)

// --- Header ------------------------------------------------------------------

#let sep = text(fill: rgb("#444444"))[ | ]
#let header() = {
  set align(center)
  text(size: 24pt)[#full-name]
  v(3pt)
  let phone = firstof(p.at("phone", default: none), p.at("mobile", default: none))
  let site = firstof(p.at("website", default: none), p.at("homepage", default: none))
  let bits = ()
  if has(phone) { bits.push([#phone]) }
  if has(p.at("email", default: none)) { bits.push(link("mailto:" + p.email, underline(p.email))) }
  if has(site) { bits.push(link(site, underline(site))) }
  if has(p.at("linkedin", default: none)) {
    bits.push(link("https://linkedin.com/in/" + p.linkedin, underline("linkedin.com/in/" + p.linkedin)))
  }
  if has(p.at("github", default: none)) {
    bits.push(link("https://github.com/" + p.github, underline("github.com/" + p.github)))
  }
  text(size: 9.5pt)[#bits.join(sep)]
}

// --- Section + entry helpers -------------------------------------------------

#let section(title) = {
  v(6pt)
  text(size: 12pt, weight: "bold")[#smallcaps(title)]
  v(1pt)
  line(length: 100%, stroke: 0.6pt + ink)
  v(3pt)
}

#let entry(title: none, subtitle: none, location: none, date: none, body: none) = {
  grid(columns: (1fr, auto), align: (left, right),
    text(weight: "bold")[#orelse(title, "")],
    text(fill: rgb("#333333"))[#orelse(date, "")],
  )
  if has(subtitle) or has(location) {
    grid(columns: (1fr, auto), align: (left, right),
      text(style: "italic")[#orelse(subtitle, "")],
      text(style: "italic", fill: rgb("#333333"))[#orelse(location, "")],
    )
  }
  if body != none { body }
}

#let bullets(items) = {
  if items.len() > 0 {
    v(1pt)
    set text(size: 9.5pt)
    set par(justify: true, leading: 0.5em)
    for b in items {
      grid(columns: (1.4em, 1fr), align: (left + top, left),
        [•], [#b])
      v(0.4pt)
    }
  }
}

// --- Section renderers -------------------------------------------------------

#let r-summary() = if has(data.at("summary", default: none)) {
  section("Summary")
  set par(justify: true)
  data.summary
}
#let r-experience() = if data.at("experience", default: ()).len() > 0 {
  section("Experience")
  for (i, e) in data.experience.enumerate() {
    if i > 0 { v(4pt) }
    entry(title: e.company, subtitle: e.at("role", default: none),
      location: e.at("location", default: none), date: e.at("period", default: none),
      body: bullets(e.at("bullets", default: ())))
  }
}
#let r-projects() = if data.at("projects", default: ()).len() > 0 {
  section("Projects")
  for (i, pr) in data.projects.enumerate() {
    if i > 0 { v(4pt) }
    let name = if has(pr.at("link", default: none)) { link(pr.link)[#pr.name] } else { pr.name }
    entry(title: name, subtitle: pr.at("description", default: none),
      date: if has(pr.at("link", default: none)) { pr.link } else { none },
      body: bullets(pr.at("bullets", default: ())))
  }
}
#let r-education() = if data.at("education", default: ()).len() > 0 {
  section("Education")
  for (i, ed) in data.education.enumerate() {
    if i > 0 { v(4pt) }
    entry(title: ed.institution, subtitle: ed.at("degree", default: none),
      location: ed.at("location", default: none), date: ed.at("period", default: none),
      body: if has(ed.at("details", default: none)) { v(1pt); text(size: 9.5pt)[#ed.details] } else { none })
  }
}
#let r-skills() = if data.at("skills", default: ()).len() > 0 {
  section("Technical Skills")
  for g in data.skills {
    let value = if has(g.at("text", default: none)) { g.text } else { g.at("items", default: ()).join(", ") }
    text(size: 9.5pt)[#text(weight: "bold")[#g.category: ]#value]
    v(1.5pt)
  }
}
#let r-honors() = if data.at("honors", default: ()).len() > 0 {
  section("Honors & Awards")
  for grp in data.honors {
    if has(grp.at("subsection", default: none)) { text(style: "italic")[#grp.subsection]; v(1pt) }
    for h in grp.at("items", default: ()) {
      grid(columns: (1fr, auto), align: (left, right),
        [#text(weight: "bold")[#h.award] — #h.at("event", default: "")],
        text(fill: rgb("#333333"))[#h.at("date", default: "")])
      v(0.5pt)
    }
  }
}
#let r-extracurricular() = if data.at("extracurricular", default: ()).len() > 0 {
  section("Extracurricular")
  for (i, e) in data.extracurricular.enumerate() {
    if i > 0 { v(4pt) }
    entry(title: e.heading, subtitle: e.at("subheading", default: none),
      location: e.at("location", default: none), date: e.at("date", default: none),
      body: bullets(e.at("bullets", default: ())))
  }
}

#let renderers = (
  summary: r-summary, experience: r-experience, projects: r-projects,
  education: r-education, skills: r-skills, honors: r-honors, extracurricular: r-extracurricular,
)
#let order = {
  let o = data.at("order", default: ())
  if o.len() > 0 { o } else { ("summary", "education", "experience", "projects", "skills", "honors", "extracurricular") }
}

#header()
#for name in order { let r = renderers.at(name, default: none); if r != none { r() } }
