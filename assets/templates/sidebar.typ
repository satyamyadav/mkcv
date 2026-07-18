// sidebar.typ — a name banner over a two-column body: a main column of
// experience/education and a colored sidebar with photo, contact, and skill
// tags.
//
// Prepended with `#let data = (...)` and `_core.typ`.

#let accent2 = resolve-color(
  firstof(data.meta.at("color", default: none), data.meta.at("accent_color", default: none)),
  fallback: "#2D6A9F",
)
#let sans = "Source Sans 3"
#let dark = rgb("#2b2b2b")
#let gray = rgb("#5d5d5d")

#let first-name = {
  let n = orelse(p.at("name", default: none), "")
  if n != "" { n.split(" ").first() } else { orelse(p.at("first_name", default: none), "") }
}
#let last-name = {
  let n = orelse(p.at("name", default: none), "")
  if n != "" { n.split(" ").slice(1).join(" ") } else { orelse(p.at("last_name", default: none), "") }
}
#let full-name = (first-name + " " + last-name).trim()

#set document(title: full-name + " — CV", author: full-name)
#set page(paper: paper, margin: (x: 1.3cm, top: 1.2cm, bottom: 1.2cm),
  footer: context {
    let f = data.at("footer", default: none)
    if f != none {
      set text(font: sans, size: 8pt, fill: gray)
      grid(columns: (1fr, 1fr, 1fr), align: (left, center, right),
        footer-part(f.at("left", default: none)),
        footer-part(f.at("center", default: none)),
        footer-part(f.at("right", default: none)))
    }
  })
#set text(font: sans, size: 9.5pt, fill: dark)
#set par(leading: 0.6em)

// --- Shared bits -------------------------------------------------------------

#let ticon(name, ..a) = text(font: "tabler-icons", ..a)[#ti-glyphs.at(name)]

#let section(title) = {
  v(6pt)
  text(size: 13pt, weight: "bold", fill: accent2)[#title]
  v(1pt)
  line(length: 100%, stroke: 1.2pt + accent2.lighten(30%))
  v(3pt)
}

#let entry(title: none, org: none, location: none, date: none, body: none) = {
  text(size: 10.5pt, weight: "bold")[#orelse(title, "")]
  if has(org) { text(fill: accent2)[  #org] }
  if has(date) or has(location) {
    linebreak()
    set text(size: 8.5pt, fill: gray, style: "italic")
    [#orelse(date, "")]
    if has(location) and has(date) { [ · ] }
    [#orelse(location, "")]
  }
  if body != none { v(1pt); body }
  v(3pt)
}

#let bullets(items) = if items.len() > 0 {
  set text(size: 9pt); set par(justify: true, leading: 0.5em)
  for b in items { grid(columns: (1em, 1fr), align: (left + top, left), text(fill: accent2)[•], [#b]); v(0.4pt) }
}

// --- Main column renderers ---------------------------------------------------

#let r-summary() = if has(data.at("summary", default: none)) { section("Profile"); set par(justify: true); data.summary }
#let r-experience() = if data.at("experience", default: ()).len() > 0 {
  section("Experience")
  for e in data.experience { entry(title: e.company, org: e.at("role", default: none), location: e.at("location", default: none), date: e.at("period", default: none), body: bullets(e.at("bullets", default: ()))) }
}
#let r-projects() = if data.at("projects", default: ()).len() > 0 {
  section("Projects")
  for pr in data.projects { entry(title: pr.name, org: pr.at("description", default: none), date: if has(pr.at("link", default: none)) { pr.link } else { none }, body: bullets(pr.at("bullets", default: ()))) }
}
#let r-education() = if data.at("education", default: ()).len() > 0 {
  section("Education")
  for ed in data.education { entry(title: ed.institution, org: ed.at("degree", default: none), location: ed.at("location", default: none), date: ed.at("period", default: none), body: if has(ed.at("details", default: none)) { text(size: 9pt)[#ed.details] } else { none }) }
}
#let r-honors() = if data.at("honors", default: ()).len() > 0 {
  section("Honors & Awards")
  for grp in data.honors { for h in grp.at("items", default: ()) { entry(title: h.award, org: h.at("event", default: none), location: h.at("location", default: none), date: h.at("date", default: none)) } }
}
#let r-extracurricular() = if data.at("extracurricular", default: ()).len() > 0 {
  section("Extracurricular")
  for e in data.extracurricular { entry(title: e.heading, org: e.at("subheading", default: none), location: e.at("location", default: none), date: e.at("date", default: none), body: bullets(e.at("bullets", default: ()))) }
}
#let main-renderers = (summary: r-summary, experience: r-experience, projects: r-projects, education: r-education, honors: r-honors, extracurricular: r-extracurricular)
#let main-order = { let o = data.at("order", default: ()); let d = ("summary", "experience", "projects", "education", "honors", "extracurricular"); (if o.len() > 0 { o } else { d }).filter(n => n in main-renderers) }

// --- Sidebar -----------------------------------------------------------------

#let sidebar-heading(t) = { v(4pt); text(size: 12pt, weight: "bold", fill: accent2)[#t]; v(1pt); line(length: 100%, stroke: 1pt + accent2.lighten(40%)); v(3pt) }
#let tag(t) = box(fill: accent2.lighten(78%), inset: (x: 5pt, y: 2.5pt), radius: 3pt, outset: (y: 2pt))[#text(size: 8.5pt, fill: accent2.darken(10%))[#t]]

#let sidebar() = {
  // Photo
  let ph = p.at("photo", default: none)
  if ph != none and has(ph.at("src", default: none)) {
    align(center, box(clip: true, radius: 50%, width: 3cm, height: 3cm, image(ph.src, width: 3cm, height: 3cm, fit: "cover")))
    v(6pt)
  }
  // Contact
  sidebar-heading("Contact")
  set text(size: 8.5pt)
  let cl(name, value, url: none) = if has(value) {
    grid(columns: (1.2em, 1fr), align: (center + top, left), ticon(name, size: 9pt, fill: accent2),
      if url != none { link(url)[#value] } else { [#value] })
    v(1.5pt)
  }
  let phone = firstof(p.at("phone", default: none), p.at("mobile", default: none))
  let site = firstof(p.at("website", default: none), p.at("homepage", default: none))
  cl("location", firstof(p.at("address", default: none), p.at("location", default: none)))
  cl("phone", phone)
  cl("mail", p.at("email", default: none), url: if has(p.at("email", default: none)) { "mailto:" + p.email } else { none })
  cl("website", site, url: site)
  cl("github", p.at("github", default: none), url: if has(p.at("github", default: none)) { "https://github.com/" + p.github } else { none })
  cl("linkedin", p.at("linkedin", default: none), url: if has(p.at("linkedin", default: none)) { "https://linkedin.com/in/" + p.linkedin } else { none })

  // Skills as tags
  if data.at("skills", default: ()).len() > 0 {
    sidebar-heading("Skills")
    for g in data.skills {
      text(size: 8.5pt, weight: "bold", fill: gray)[#g.category]
      v(2pt)
      let items = g.at("items", default: ())
      if items.len() > 0 { box(width: 100%)[#items.map(tag).join(h(4pt))] } else if has(g.at("text", default: none)) { text(size: 8.5pt)[#g.text] }
      v(4pt)
    }
  }
}

// --- Assembly ----------------------------------------------------------------

// Name banner (full width)
#text(size: 30pt, fill: accent2)[#first-name #text(weight: "bold")[#last-name]]
#{
  let positions = p.at("positions", default: ())
  let title = if positions.len() > 0 { positions.join(" · ") } else { orelse(p.at("title", default: none), "") }
  if title != "" { linebreak(); text(size: 11pt, fill: gray, weight: "medium")[#upper(title)] }
}
#v(8pt)

#grid(columns: (1fr, 6cm), column-gutter: 1.1cm,
  { for name in main-order { main-renderers.at(name)() } },
  block(fill: accent2.lighten(92%), inset: 10pt, radius: 4pt, width: 100%, sidebar()),
)
