# resume.yml schema (quick reference)

Run `mkcv schema --format json` for the machine-readable version. Only
`profile.name` (or `first_name` + `last_name`) is required; everything else is
optional and omitted sections don't render.

**Markdown in free-text fields** (`summary`, `details`, `description`, `text`,
`quote`, cover-letter `opening`/`closing`/`body`, and `bullets`/`items`):
- Inline: `**bold**` / `__bold__`, `*italic*` / `_italic_`, `` `code` ``,
  `~~strikethrough~~`, `[label](url)`.
- Blocks: a blank line starts a new paragraph; end a line with `\` (or two
  spaces) for a hard line break.
- `bullets` also support **nested sub-bullets** — either indented `- ` lines
  inside one bullet string, or separate indented `- ` array items after their
  parent:
  ```yaml
  bullets:
    - "Led the platform migration:\n  - moved 200 services\n  - zero downtime"
  ```
Intraword underscores (`snake_case`) and lone `~`/`#`/`$` stay literal — only the
patterns above are interpreted, and everything else is escaped (no raw Typst).

```yaml
meta:
  template: "modern"        # see `mkcv templates`; or a local ./file.typ
                            # (customize: `mkcv eject <name> -o my.typ`, then edit)
  kind: "resume"            # resume | cover-letter
  color: "orange"   # named preset or #hex
  section_highlight: "full" # full | three-letter | none
  paper: "a4"               # a4 | letter

profile:
  name: "Full Name"         # or first_name + last_name
  positions: ["Role A", "Role B"]   # or a single `title`
  address: "…"; email: "…"; phone: "…"   # phone alias: mobile
  location: "…"; website: "…"            # website alias: homepage
  github: "handle"; linkedin: "handle"; twitter: "handle"
  quote: "…"
  photo: { path: "me.jpg", shape: "circle", side: "right", edge: true }

summary: "paragraph (markup)"
experience: [{ company, role, location, period, bullets: [] }]
education:  [{ institution, degree, location, period, details }]
skills:     [{ category, items: [] }]        # or { category, text: "markup" }
projects:   [{ name, description, link, bullets: [] }]
honors:     [{ subsection, items: [{ award, event, location, date }] }]
extracurricular: [{ heading, subheading, location, date, bullets: [] }]
order:  [summary, experience, projects, education, skills, honors, extracurricular]
footer: { left: "{today}", center: "Name — Resume", right: "{page}" }

# Cover letter (meta.kind: cover-letter):
letter:
  recipient_name: "…"; recipient_address: "line1\nline2"; date: "{today}"
  title: "subject"; opening: "Dear …,"
  sections: [{ title, body: ["paras"] }]   # or flat: body: ["paras"]
  closing: "Sincerely,"; enclosure: "Resume"; enclosure_label: "Attached"
```
