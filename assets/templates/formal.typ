// formal.typ — a classic CV: a colored name header with a right-aligned
// contact block, colored section titles, and dates in a left margin column.
// Defaults to a professional blue.
//
// Prepended with `#let data = (...)` and `_core.typ`.

// Use the user's color if set, else a professional blue default.
#let blue = resolve-color(
  firstof(data.meta.at("color", default: none), data.meta.at("accent_color", default: none)),
  fallback: "#3873B3",
)
#let sans = "Source Sans 3"
#let gray = rgb("#5d5d5d")
#let dark = rgb("#2b2b2b")

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
#set page(paper: paper, margin: (x: 1.6cm, top: 1.2cm, bottom: 1.4cm),
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

#let dcol = 2.5cm // left "hint" column width for dates

// --- Header ------------------------------------------------------------------

#let icon(name) = text(font: "tabler-icons", size: 8pt, fill: blue)[#ti-glyphs.at(name)]
#let cline(name, value) = if has(value) { box[#icon(name)#h(3pt)#value]; linebreak() }

#let header() = {
  grid(columns: (1fr, auto), align: (left + horizon, right + horizon), column-gutter: 1em,
    {
      text(size: 26pt, fill: blue)[#first-name #text(weight: "bold")[#last-name]]
      let positions = p.at("positions", default: ())
      let title = if positions.len() > 0 { positions.join(" · ") } else { orelse(p.at("title", default: none), "") }
      if title != "" { linebreak(); text(size: 11pt, fill: gray)[#title] }
    },
    {
      set align(right)
      set text(size: 8.5pt, fill: gray)
      let phone = firstof(p.at("phone", default: none), p.at("mobile", default: none))
      let site = firstof(p.at("website", default: none), p.at("homepage", default: none))
      cline("location", p.at("address", default: none))
      cline("location", p.at("location", default: none))
      cline("phone", phone)
      if has(p.at("email", default: none)) { box[#icon("mail")#h(3pt)#link("mailto:" + p.email)[#p.email]]; linebreak() }
      if has(site) { box[#icon("website")#h(3pt)#link(site)[#site]]; linebreak() }
      if has(p.at("github", default: none)) { box[#icon("github")#h(3pt)#link("https://github.com/" + p.github)[#p.github]]; linebreak() }
      if has(p.at("linkedin", default: none)) { box[#icon("linkedin")#h(3pt)#link("https://linkedin.com/in/" + p.linkedin)[#p.linkedin]] }
    },
  )
  v(4pt)
  line(length: 100%, stroke: 0.8pt + blue)
  if has(p.at("quote", default: none)) { v(3pt); set align(center); text(style: "italic", fill: gray)[#p.quote] }
}

// --- Section + entry ---------------------------------------------------------

#let section(title) = {
  v(7pt)
  grid(columns: (dcol, 1fr), column-gutter: 0.6em, align: (left + horizon, left + horizon),
    text(size: 12pt, weight: "bold", fill: blue)[#title],
    line(length: 100%, stroke: 0.5pt + blue.lighten(40%)))
  v(3pt)
}

// An entry: date in the left hint column, content on the right.
#let entry(date: none, title: none, org: none, location: none, body: none) = {
  grid(columns: (dcol, 1fr), column-gutter: 0.6em, align: (right + top, left),
    text(size: 8.5pt, fill: gray)[#orelse(date, "")],
    {
      let head = ()
      if has(title) { head.push(text(weight: "bold")[#title]) }
      if has(org) { head.push(text(fill: blue)[#org]) }
      head.join(text(fill: gray)[, ])
      if has(location) { text(fill: gray, size: 9pt)[ — #location] }
      if body != none { body }
    },
  )
  v(2pt)
}

#let bullets(items) = if items.len() > 0 {
  v(1pt); set text(size: 9pt); set par(justify: true, leading: 0.5em)
  for b in items { grid(columns: (1em, 1fr), align: (left + top, left), text(fill: blue)[•], [#b]); v(0.3pt) }
}

// --- Renderers ---------------------------------------------------------------

#let r-summary() = if has(data.at("summary", default: none)) { section("Summary"); grid(columns: (dcol, 1fr), column-gutter: 0.6em, [], { set par(justify: true); data.summary }) }
#let r-experience() = if data.at("experience", default: ()).len() > 0 {
  section("Experience")
  for e in data.experience { entry(date: e.at("period", default: none), title: e.company, org: e.at("role", default: none), location: e.at("location", default: none), body: bullets(e.at("bullets", default: ()))) }
}
#let r-education() = if data.at("education", default: ()).len() > 0 {
  section("Education")
  for ed in data.education { entry(date: ed.at("period", default: none), title: ed.institution, org: ed.at("degree", default: none), location: ed.at("location", default: none), body: if has(ed.at("details", default: none)) { v(1pt); text(size: 9pt)[#ed.details] } else { none }) }
}
#let r-projects() = if data.at("projects", default: ()).len() > 0 {
  section("Projects")
  for pr in data.projects { entry(date: none, title: pr.name, org: pr.at("description", default: none), body: bullets(pr.at("bullets", default: ()))) }
}
#let r-skills() = if data.at("skills", default: ()).len() > 0 {
  section("Skills")
  for g in data.skills {
    let value = if has(g.at("text", default: none)) { g.text } else { g.at("items", default: ()).join(", ") }
    grid(columns: (dcol, 1fr), column-gutter: 0.6em, align: (right + top, left),
      text(size: 9pt, weight: "bold", fill: gray)[#g.category], text(size: 9pt)[#value])
    v(1.5pt)
  }
}
#let r-honors() = if data.at("honors", default: ()).len() > 0 {
  section("Honors & Awards")
  for grp in data.honors {
    for h in grp.at("items", default: ()) {
      entry(date: h.at("date", default: none), title: h.award, org: h.at("event", default: none), location: h.at("location", default: none))
    }
  }
}
#let r-extracurricular() = if data.at("extracurricular", default: ()).len() > 0 {
  section("Extracurricular")
  for e in data.extracurricular { entry(date: e.at("date", default: none), title: e.heading, org: e.at("subheading", default: none), location: e.at("location", default: none), body: bullets(e.at("bullets", default: ()))) }
}

#let renderers = (summary: r-summary, experience: r-experience, education: r-education, projects: r-projects, skills: r-skills, honors: r-honors, extracurricular: r-extracurricular)
#let order = { let o = data.at("order", default: ()); if o.len() > 0 { o } else { ("summary", "experience", "education", "projects", "skills", "honors", "extracurricular") } }

#header()
#for name in order { let r = renderers.at(name, default: none); if r != none { r() } }
