// coverletter.typ — cover-letter body for mkcv (meta.kind: cover-letter).
//
// The engine prepends `#let data = (...)` and `_prelude.typ`, so `data`,
// `header()`, `heading-with-rule()`, `full-name`, `footer-part()`, and the
// helpers are all in scope here.

#let letter = data.at("letter", default: (:))

// Render a multi-line address string (newline-separated) as stacked lines.
#let addr-lines(s) = {
  s.split("\n").map(l => l.trim()).filter(l => l != "").map(l => [#l]).join(linebreak())
}

// A cover-letter section heading: like the resume section but larger with a
// heavier gray rule.
#let lettersection(title) = {
  v(6pt)
  heading-with-rule(title, size: 13pt, rule-color: gray, rule-weight: 0.9pt)
}

#header()

// --- Title block -------------------------------------------------------------

#v(8pt)
#grid(
  columns: (1fr, auto),
  align: (left + horizon, right + horizon),
  text(size: 11pt, weight: "bold", fill: darktext)[#orelse(letter.at("recipient_name", default: none), "")],
  text(size: 9pt, fill: gray, style: "italic")[#footer-part(letter.at("date", default: none))],
)
#if has(letter.at("recipient_address", default: none)) {
  v(2pt)
  text(size: 9pt, fill: gray)[#smallcaps(addr-lines(letter.recipient_address))]
}

#v(10pt)
#if has(letter.at("title", default: none)) {
  text(size: 10pt, weight: "bold", fill: darktext)[#underline(letter.title)]
  parbreak()
}
#if has(letter.at("opening", default: none)) {
  text(size: 10pt, fill: gray)[#letter.opening]
  parbreak()
}

// --- Body --------------------------------------------------------------------

#set text(size: 10pt, fill: gray)
#set par(justify: true, leading: 0.65em)

#let render-paras(paras) = {
  for para in paras {
    block(below: 0.8em)[#para]
  }
}

#let sections = letter.at("sections", default: ())
#if sections.len() > 0 {
  for s in sections {
    lettersection(s.at("title", default: ""))
    render-paras(s.at("body", default: ()))
  }
} else {
  render-paras(letter.at("body", default: ()))
}

// --- Closing -----------------------------------------------------------------

#v(6pt)
#if has(letter.at("closing", default: none)) {
  text(fill: gray)[#letter.closing]
  parbreak()
  v(6pt)
}
#text(size: 10pt, weight: "bold", fill: darktext)[#full-name]
#if has(letter.at("enclosure", default: none)) {
  parbreak()
  v(2pt)
  let lbl = orelse(letter.at("enclosure_label", default: none), "Enclosure")
  text(size: 9pt, fill: lightgray, style: "italic")[#lbl: #letter.enclosure]
}
