// classic-letter.typ — a plain, formal business cover letter set in a serif
// face: sender block, date, recipient, salutation, justified body, closing,
// and signature. A traditional alternative to the header-styled `modern` letter.
//
// Prepended with `#let data = (...)` and `_core.typ`. (meta.kind: cover-letter,
// meta.template: classic)

#let serif = "Liberation Serif"
#let ink = rgb("#1a1a1a")
#let letter = data.at("letter", default: (:))

#let full-name = {
  let n = orelse(p.at("name", default: none), "")
  if n != "" { n } else {
    (orelse(p.at("first_name", default: none), "") + " " + orelse(p.at("last_name", default: none), "")).trim()
  }
}

#set document(title: full-name + " — Cover Letter", author: full-name)
#set page(paper: paper, margin: (x: 2cm, top: 2cm, bottom: 2cm),
  footer: context {
    let f = data.at("footer", default: none)
    if f != none {
      set text(font: serif, size: 8pt, fill: rgb("#666666"))
      grid(columns: (1fr, 1fr, 1fr), align: (left, center, right),
        footer-part(f.at("left", default: none)),
        footer-part(f.at("center", default: none)),
        footer-part(f.at("right", default: none)))
    }
  })
#set text(font: serif, size: 11pt, fill: ink)
#set par(justify: true, leading: 0.62em, first-line-indent: 0pt)

#let stacked(s) = s.split("\n").map(l => l.trim()).filter(l => l != "").map(l => [#l]).join(linebreak())

// --- Sender block (top, centered) --------------------------------------------

#align(center)[
  #text(size: 16pt, weight: "bold")[#full-name]
  #{
    let phone = firstof(p.at("phone", default: none), p.at("mobile", default: none))
    let site = firstof(p.at("website", default: none), p.at("homepage", default: none))
    let bits = ()
    if has(p.at("address", default: none)) { bits.push(p.address) }
    if has(phone) { bits.push(phone) }
    if has(p.at("email", default: none)) { bits.push(p.email) }
    if has(site) { bits.push(site) }
    if bits.len() > 0 { linebreak(); text(size: 9.5pt, fill: rgb("#444444"))[#bits.join("  •  ")] }
  }
]
#v(2pt)
#line(length: 100%, stroke: 0.5pt + rgb("#999999"))
#v(10pt)

// --- Date + recipient --------------------------------------------------------

#if has(letter.at("date", default: none)) {
  footer-part(letter.date)
  v(10pt)
}
#if has(letter.at("recipient_name", default: none)) {
  text(weight: "bold")[#letter.recipient_name]; linebreak()
}
#if has(letter.at("recipient_address", default: none)) {
  stacked(letter.recipient_address)
}
#v(12pt)

// --- Subject + salutation ----------------------------------------------------

#if has(letter.at("title", default: none)) {
  text(weight: "bold")[Re: #letter.title]
  parbreak()
}
#if has(letter.at("opening", default: none)) { letter.opening; parbreak() }

// --- Body --------------------------------------------------------------------

#let paras = {
  let secs = letter.at("sections", default: ())
  if secs.len() > 0 { secs.map(s => s.at("body", default: ())).flatten() } else { letter.at("body", default: ()) }
}
#for para in paras { block(below: 0.9em, para) }

// --- Closing -----------------------------------------------------------------

#v(6pt)
#if has(letter.at("closing", default: none)) { letter.closing; parbreak(); v(16pt) }
#text(weight: "bold")[#full-name]
#if has(letter.at("enclosure", default: none)) {
  linebreak(); v(2pt)
  text(size: 9.5pt, style: "italic", fill: rgb("#666666"))[#orelse(letter.at("enclosure_label", default: none), "Enclosure"): #letter.enclosure]
}
