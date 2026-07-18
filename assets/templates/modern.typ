// modern.typ — the minimalist resume body for mkcv.
//
// The engine prepends `#let data = (...)` and `_prelude.typ` before this file,
// so `data`, `accent`, `header()`, `heading-with-rule()`, and the helpers are
// all in scope here.

// --- Sections ----------------------------------------------------------------

#let section(title) = {
  v(9pt)
  heading-with-rule(title)
}

// A small-caps subsection heading used to group content within a section.
#let subsection(title) = {
  v(4pt)
  text(size: 10.5pt, fill: darktext, tracking: 0.4pt)[#smallcaps(title)]
  v(2pt)
}

// A two-column entry heading: title/subtitle left, meta/date right.
#let entry-head(left-top, left-bottom: none, right-top: none, right-bottom: none) = {
  grid(
    columns: (1fr, auto),
    align: (left, right),
    [
      #text(size: 10pt, weight: "bold", fill: darktext)[#left-top]
      #if left-bottom != none [ \ #text(size: 9pt, fill: accent, style: "italic")[#left-bottom] ]
    ],
    [
      #if right-top != none [ #text(size: 9pt, fill: gray, weight: "medium")[#right-top] ]
      #if right-bottom != none [ \ #text(size: 8.5pt, fill: lightgray, style: "italic")[#right-bottom] ]
    ],
  )
}

#let bullets(items) = {
  if items.len() > 0 {
    v(2pt)
    set text(size: 9pt, fill: rgb("#4a4a4a"))
    for b in items {
      grid(
        columns: (10pt, 1fr),
        align: (left, left),
        box(baseline: 15%)[#icon("bullet", size: 8pt, fill: accent)], [#b],
      )
      v(1pt)
    }
  }
}

// --- Section renderers -------------------------------------------------------

#let render-summary() = {
  if has(data.at("summary", default: none)) {
    section("Summary")
    text(fill: rgb("#4a4a4a"))[#data.summary]
  }
}

#let render-experience() = {
  if data.at("experience", default: ()).len() > 0 {
    section("Experience")
    for (i, e) in data.experience.enumerate() {
      if i > 0 { v(6pt) }
      entry-head(
        e.company,
        left-bottom: e.at("role", default: none),
        right-top: e.at("period", default: none),
        right-bottom: e.at("location", default: none),
      )
      bullets(e.at("bullets", default: ()))
    }
  }
}

#let render-projects() = {
  if data.at("projects", default: ()).len() > 0 {
    section("Projects")
    for (i, pr) in data.projects.enumerate() {
      if i > 0 { v(6pt) }
      let name = if has(pr.at("link", default: none)) { link(pr.link)[#pr.name] } else { pr.name }
      entry-head(
        name,
        left-bottom: pr.at("description", default: none),
        right-top: if has(pr.at("link", default: none)) { pr.link } else { none },
      )
      bullets(pr.at("bullets", default: ()))
    }
  }
}

#let render-education() = {
  if data.at("education", default: ()).len() > 0 {
    section("Education")
    for (i, ed) in data.education.enumerate() {
      if i > 0 { v(6pt) }
      entry-head(
        ed.institution,
        left-bottom: ed.at("degree", default: none),
        right-top: ed.at("period", default: none),
        right-bottom: ed.at("location", default: none),
      )
      if has(ed.at("details", default: none)) {
        v(2pt)
        text(size: 9pt, fill: rgb("#4a4a4a"))[#ed.details]
      }
    }
  }
}

#let render-skills() = {
  if data.at("skills", default: ()).len() > 0 {
    section("Skills")
    for g in data.skills {
      // Free-form `text` (markup content) wins over a discrete `items` list.
      let value = if has(g.at("text", default: none)) {
        g.text
      } else {
        g.at("items", default: ()).join(text(fill: lightgray)[ · ])
      }
      grid(
        columns: (auto, 1fr),
        column-gutter: 8pt,
        align: (right + top, left),
        text(weight: "bold", fill: darktext)[#g.category],
        [#value],
      )
      v(3pt)
    }
  }
}

// A compact honor line: date | <award>, <event> | location.
#let honor-line(h) = {
  grid(
    columns: (auto, 1fr, auto),
    column-gutter: 10pt,
    align: (left + top, left + top, right + top),
    text(size: 9pt, fill: gray)[#orelse(h.at("date", default: none), "")],
    {
      set text(size: 9pt)
      text(weight: "bold", fill: darktext)[#h.award]
      if has(h.at("event", default: none)) [#text(fill: gray)[, #h.event]]
    },
    text(size: 9pt, fill: accent, style: "italic")[#orelse(h.at("location", default: none), "")],
  )
  v(2pt)
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
      if i > 0 { v(6pt) }
      entry-head(
        e.heading,
        left-bottom: e.at("subheading", default: none),
        right-top: e.at("date", default: none),
        right-bottom: e.at("location", default: none),
      )
      bullets(e.at("bullets", default: ()))
    }
  }
}

// --- Body assembly (order-driven) --------------------------------------------

#let renderers = (
  summary: render-summary,
  experience: render-experience,
  projects: render-projects,
  education: render-education,
  skills: render-skills,
  honors: render-honors,
  extracurricular: render-extracurricular,
)
#let default-order = (
  "summary", "experience", "projects", "education", "skills", "honors", "extracurricular",
)
#let order = {
  let o = data.at("order", default: ())
  if o.len() > 0 { o } else { default-order }
}

#header()

#for name in order {
  let r = renderers.at(name, default: none)
  if r != none { r() }
}
