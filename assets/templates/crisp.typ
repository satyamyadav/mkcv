// crisp.typ — a polished, professional single-column layout.
//
// The engine prepends `#let data = (...)` and `_core.typ` (helpers, `accent`,
// `presets`, `paper`, `footer-part`, `ti-glyphs`). This file adds its palette,
// Roboto/Source Sans fonts, and a crisp type scale: a two-tone name, small-caps
// section headers with an accent on the first letters, and org-bold entries.

// --- Palette -----------------------------------------------------------------

#let textcol = rgb("#333333")    // text
#let graytext = rgb("#5D5D5D")   // positions, dates
#let lighttext = rgb("#999999")  // address, social, footer
#let darktext = rgb(orelse(data.meta.at("dark_text", default: none), "#414141"))
#let sec-mode = orelse(data.meta.at("section_highlight", default: none), "three-letter")

// --- Fonts -------------------------------------------------------------------

#let headerfont = "Roboto"
#let bodyfont = "Source Sans 3"

// --- Full name ---------------------------------------------------------------

#let first-name = {
  let n = orelse(p.at("name", default: none), "")
  if n != "" { n.split(" ").first() } else { orelse(p.at("first_name", default: none), "") }
}
#let last-name = {
  let n = orelse(p.at("name", default: none), "")
  if n != "" { n.split(" ").slice(1).join(" ") } else { orelse(p.at("last_name", default: none), "") }
}
#let full-name = (first-name + " " + last-name).trim()

// --- Icons -------------------------------------------------------------------
// Contact codepoints from core, plus a round bullet marker.
#let glyphs = ti-glyphs + (bullet: "\u{2022}")
#let icon(name, size: 6.8pt) = text(font: "tabler-icons", size: size)[#glyphs.at(name)]

// --- Page / document ---------------------------------------------------------

#set document(
  title: full-name + (if kind == "cover-letter" { " — Cover Letter" } else { " — Resume" }),
  author: full-name,
)

#let footer-content = {
  let f = data.at("footer", default: none)
  if f == none { none } else {
    set text(font: bodyfont, size: 8pt, fill: lighttext)
    smallcaps(grid(
      columns: (1fr, 1fr, 1fr),
      align: (left + horizon, center + horizon, right + horizon),
      footer-part(f.at("left", default: none)),
      footer-part(f.at("center", default: none)),
      footer-part(f.at("right", default: none)),
    ))
  }
}

#set page(paper: paper, margin: (left: 1.4cm, right: 1.4cm, top: 0.8cm, bottom: 1.8cm), footer: footer-content)
#set text(font: bodyfont, size: 9pt, fill: textcol)
#set par(justify: false, leading: 0.6em)

// --- Header ------------------------------------------------------------------

#let contact-item(name, value, url: none) = {
  if has(value) {
    let c = box(baseline: 15%)[#icon(name)#h(3pt)#value]
    if url != none { link(url)[#c] } else { c }
  }
}

#let make-header() = {
  set align(center)
  // Name: first (Roboto Thin, gray) + last (Roboto Bold, dark).
  text(size: 30pt, font: headerfont)[
    #text(weight: "thin", fill: graytext)[#first-name]
    #h(0.2em)
    #text(weight: "bold", fill: textcol)[#last-name]
  ]

  let positions = p.at("positions", default: ())
  let pos = if positions.len() > 0 { positions } else if has(p.at("title", default: none)) { (p.title,) } else { () }
  if pos.len() > 0 {
    v(0.4mm)
    text(size: 7.8pt, font: bodyfont, fill: accent)[
      #smallcaps(pos.map(x => [#x]).join([ · ]))
    ]
  }

  if has(p.at("address", default: none)) {
    v(0.4mm)
    text(size: 8pt, font: headerfont, style: "italic", fill: lighttext)[#p.address]
  }
  v(2.2mm)

  let phone = firstof(p.at("phone", default: none), p.at("mobile", default: none))
  let website = firstof(p.at("website", default: none), p.at("homepage", default: none))
  let items = ()
  if has(phone) { items.push(contact-item("phone", phone)) }
  if has(p.at("email", default: none)) { items.push(contact-item("mail", p.email, url: "mailto:" + p.email)) }
  if has(website) { items.push(contact-item("website", website, url: website)) }
  if has(p.at("github", default: none)) { items.push(contact-item("github", p.github, url: "https://github.com/" + p.github)) }
  if has(p.at("linkedin", default: none)) { items.push(contact-item("linkedin", p.linkedin, url: "https://linkedin.com/in/" + p.linkedin)) }
  if has(p.at("twitter", default: none)) { items.push(contact-item("twitter", p.twitter, url: "https://twitter.com/" + p.twitter)) }
  block[
    #set text(size: 6.8pt, font: headerfont, fill: lighttext)
    #items.filter(x => x != none).join(text(fill: lighttext)[#h(0.5em)#sym.bar.v#h(0.5em)])
  ]

  if has(p.at("quote", default: none)) {
    v(5mm)
    text(size: 9pt, font: bodyfont, style: "italic", fill: darktext)[“#p.quote”]
  }
}

// --- Sections ----------------------------------------------------------------

// Section heading; in "three-letter" mode the first letters take the accent.
#let section(title) = {
  v(3mm)
  let render = if sec-mode == "none" {
    text(size: 16pt, weight: "bold", fill: textcol)[#title]
  } else if sec-mode == "full" {
    text(size: 16pt, weight: "bold", fill: accent)[#title]
  } else {
    let cl = title.clusters()
    let k = calc.min(3, cl.len())
    let head = cl.slice(0, k).join()
    let rest = if cl.len() > k { cl.slice(k).join() } else { "" }
    text(size: 16pt, weight: "bold")[#text(fill: accent)[#head]#text(fill: textcol)[#rest]]
  }
  box(width: 100%)[
    #render #h(0.6em) #box(width: 1fr, baseline: -0.15em, line(length: 100%, stroke: 0.5pt + lighttext))
  ]
  v(2.5mm)
}

#let subsection(title) = {
  v(1mm)
  text(size: 12pt, font: bodyfont, fill: textcol)[#smallcaps(title)]
  v(1mm)
}

// A cventry: title/location on top, position/date below, then description.
#let cv-entry(title: none, subtitle: none, location: none, date: none, body: none) = {
  grid(
    columns: (1fr, 4.5cm),
    row-gutter: 1.2pt,
    align: (left, right),
    text(size: 10pt, weight: "bold", fill: darktext)[#orelse(title, "")],
    text(size: 9pt, weight: "light", style: "italic", fill: accent)[#orelse(location, "")],
    text(size: 8pt, fill: graytext)[#smallcaps(orelse(subtitle, ""))],
    text(size: 8pt, weight: "light", style: "italic", fill: graytext)[#orelse(date, "")],
  )
  if body != none { body }
}

#let cv-items(items) = {
  if items.len() > 0 {
    v(1mm)
    set text(size: 9pt, weight: "light", fill: textcol)
    set par(justify: true, leading: 0.55em)
    for b in items {
      grid(
        columns: (1.6em, 1fr),
        align: (left + top, left),
        text(fill: textcol)[#glyphs.bullet], [#b],
      )
      v(0.3mm)
    }
  }
}

// --- Section renderers -------------------------------------------------------

#let render-summary() = {
  if has(data.at("summary", default: none)) {
    section("Summary")
    set par(justify: true)
    text(size: 9pt, weight: "light")[#data.summary]
  }
}

#let render-experience() = {
  if data.at("experience", default: ()).len() > 0 {
    section("Experience")
    for (i, e) in data.experience.enumerate() {
      if i > 0 { v(2mm) }
      cv-entry(
        title: e.company, subtitle: e.at("role", default: none),
        location: e.at("location", default: none), date: e.at("period", default: none),
        body: cv-items(e.at("bullets", default: ())),
      )
    }
  }
}

#let render-projects() = {
  if data.at("projects", default: ()).len() > 0 {
    section("Projects")
    for (i, pr) in data.projects.enumerate() {
      if i > 0 { v(2mm) }
      let name = if has(pr.at("link", default: none)) { link(pr.link)[#pr.name] } else { pr.name }
      cv-entry(
        title: name, subtitle: pr.at("description", default: none),
        date: if has(pr.at("link", default: none)) { pr.link } else { none },
        body: cv-items(pr.at("bullets", default: ())),
      )
    }
  }
}

#let render-education() = {
  if data.at("education", default: ()).len() > 0 {
    section("Education")
    for (i, ed) in data.education.enumerate() {
      if i > 0 { v(2mm) }
      cv-entry(
        title: ed.institution, subtitle: ed.at("degree", default: none),
        location: ed.at("location", default: none), date: ed.at("period", default: none),
        body: if has(ed.at("details", default: none)) {
          v(1mm); text(size: 9pt, weight: "light")[#ed.details]
        } else { none },
      )
    }
  }
}

#let render-skills() = {
  if data.at("skills", default: ()).len() > 0 {
    section("Skills")
    for g in data.skills {
      let value = if has(g.at("text", default: none)) {
        g.text
      } else { g.at("items", default: ()).map(x => [#x]).join([, ]) }
      grid(
        columns: (30%, 1fr),
        column-gutter: 1em,
        align: (right + top, left),
        text(size: 10pt, weight: "bold", fill: darktext)[#g.category],
        text(size: 9pt, weight: "light")[#value],
      )
      v(1mm)
    }
  }
}

#let honor-line(h) = {
  grid(
    columns: (1.5cm, 1fr, 2.5cm),
    column-gutter: 6pt,
    align: (left + top, left + top, right + top),
    text(size: 9pt, fill: graytext)[#orelse(h.at("date", default: none), "")],
    {
      set text(size: 9pt)
      text(weight: "bold", fill: darktext)[#h.award]
      if has(h.at("event", default: none)) [#text(fill: graytext)[, #h.event]]
    },
    text(size: 9pt, weight: "light", style: "italic", fill: accent)[#orelse(h.at("location", default: none), "")],
  )
  v(0.6mm)
}

#let render-honors() = {
  if data.at("honors", default: ()).len() > 0 {
    section("Honors & Awards")
    for g in data.honors {
      if has(g.at("subsection", default: none)) { subsection(g.subsection) }
      for h in g.at("items", default: ()) { honor-line(h) }
    }
  }
}

#let render-extracurricular() = {
  if data.at("extracurricular", default: ()).len() > 0 {
    section("Extracurricular")
    for (i, e) in data.extracurricular.enumerate() {
      if i > 0 { v(2mm) }
      cv-entry(
        title: e.heading, subtitle: e.at("subheading", default: none),
        location: e.at("location", default: none), date: e.at("date", default: none),
        body: cv-items(e.at("bullets", default: ())),
      )
    }
  }
}

// --- Body assembly -----------------------------------------------------------

#let renderers = (
  summary: render-summary, experience: render-experience, projects: render-projects,
  education: render-education, skills: render-skills, honors: render-honors,
  extracurricular: render-extracurricular,
)
#let default-order = (
  "summary", "experience", "projects", "education", "skills", "honors", "extracurricular",
)
#let order = { let o = data.at("order", default: ()); if o.len() > 0 { o } else { default-order } }

#make-header()
#for name in order {
  let r = renderers.at(name, default: none)
  if r != none { r() }
}
