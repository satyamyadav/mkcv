---
name: mkcv
description: >-
  Create, improve, and tailor a resume, CV, or cover letter as a guided coaching
  workflow — beat the blank page with an instant first draft, pick a template
  visually, interview the user (pulling from their old resume, LinkedIn, docs,
  images, or a job posting), then refine the content into a strong, job-aligned
  PDF via the embedded-Typst `mkcv` engine. Use this whenever the user wants to
  build a resume/CV/cover letter, restyle or fix an existing one, turn their
  career history into a PDF, tailor a resume to a specific job, or asks for help
  writing bullet points or a summary — even if they just paste a job title and
  say "make me a resume." Prefer this over hand-writing HTML/Markdown: it
  produces a real typeset PDF and coaches the content.
---

# mkcv — build a resume/CV/cover letter, and coach the content

`mkcv` compiles one `resume.yml` into a typeset PDF (embedded Typst engine +
fonts — no LaTeX, no browser, no network). But the PDF is the easy part. The
real job of this skill is to **get a strong resume out of a person who may not
know how to write one** — so treat yourself as a resume coach who happens to
have a fast typesetter, not a form-filler.

The guiding principle is **show, don't interrogate**: a real person facing a
blank resume freezes. A complete draft they can react to unfreezes them. So you
lead with a draft, then improve it together — never open with a wall of
questions.

## When this applies

Any of: "make me a resume / CV / cover letter", pasting career history and
wanting a PDF, "improve/fix my resume", "tailor this to this job", "help me
write bullets / a summary", or restyling an existing `resume.yml`. If the user
gives almost nothing ("backend engineer, 5 years, was at Stripe"), that is still
enough to start — go to Phase 1.

## The engine (mechanics you'll use throughout)

Every command takes `--format json`; read the `ok` field instead of scraping text.

**Get the binary once, reuse `$BIN`:**
```bash
BIN="$(sh ~/.claude/skills/mkcv/install.sh --bin)" || { echo "$BIN"; exit 1; }
```
Use the actual install path if different (don't rely on `$CLAUDE_SKILL_DIR` — it
may be unset). If it errors (no prebuilt for this platform), report and stop.

**Commands** (all discrete, JSON-out):
- `"$BIN" templates --format json` — catalog; each entry has a `preview` image URL.
- `"$BIN" schema --format json` — the full YAML schema. Don't guess field names.
- `"$BIN" init` — writes boilerplate `resume.yml`.
- `"$BIN" validate --input resume.yml --format json` — `{"ok":true}` or errors.
- `"$BIN" build --input resume.yml --output resume.pdf --format json` — compiles.
  Flags: `--template <name-or-./file.typ>`, `--kind cover-letter`, `--yaml '<inline>'`.
- `"$BIN" eject <name> --output my.typ` — copy a built-in template's source to a
  local `.typ` to **customize** (add `--kind cover-letter` for a letter variant).
  Prefer this over hand-writing a template: eject, tweak, then set
  `meta.template: "./my.typ"`. The ejected file is self-contained (the prelude is
  inlined when needed) and receives `data` + `_core.typ` helpers.

Free-text fields accept a safe Markdown subset — `**bold**`/`__bold__`,
`*italic*`/`_italic_`, `` `code` ``, `~~strike~~`, `[links](url)`, blank-line
paragraphs, and nested sub-bullets in `bullets`. Only `profile.name` (or
`first_name`+`last_name`) is required — omit anything that doesn't apply; the
layout adapts. Full field map: `reference/schema.md`.

---

# The workflow

Six phases. They're a spine, not a cage — if the user arrives mid-way (e.g. hands
you a finished `resume.yml` and says "just restyle it"), jump to the phase that
fits. But for anyone starting from scratch, do them in order, because the order
is what beats the blank page.

## Phase 0 — Kickoff (30 seconds, not an interview)

Two quick things before drafting, because they shape everything downstream:

1. **What are they aiming at?** Always ask for the **target role or job**, because
   a resume aimed at a specific job is dramatically stronger than a generic one.
   A title is enough to start ("Senior Backend Engineer roles"); a pasted job
   posting or URL is even better — you'll mine it in Phase 5. If they truly have
   no target, proceed generically but note that tailoring is the biggest lever
   you can pull later.
2. **What do they already have?** Ask if they can point you at an existing
   resume, a LinkedIn profile/export, a screenshot, or notes — anything. If yes,
   ingest it now (see *Ingesting sources* below) so the first draft is built from
   real material instead of guesses.

Then go straight to Phase 1. Do **not** keep asking questions here — one target,
one "what do you have", then draft.

## Phase 1 — Beat the blank page (draft first, always)

Produce a **complete-looking** first-draft PDF from whatever you have, even if
that's one sentence. A full page the user can react to is worth more than a
correct-but-empty skeleton.

- `init` a `resume.yml`, then fill **every** standard section (summary,
  experience, skills, education) so the page looks finished.
- Where you don't have real information, **invent plausible, realistic content**
  — but mark every invented or uncertain value inline with the token `‹?›` so it
  is impossible to miss in the PDF. Example bullet:
  `"Scaled the payments API to 10k req/s ‹?› — confirm real numbers"`.
  A guessed job title becomes `"Senior Backend Engineer ‹?›"`.
- `validate`, then `build`, then **show the user the PDF path** and say plainly:
  "Here's a full first draft. Everything marked `‹?›` is my guess — we'll replace
  those together. React to it: what's wrong, what's missing?"

Why inline `‹?›` instead of a clean draft: it makes fabrication visible, so the
user can never accidentally send out invented numbers, and it turns editing into
a concrete hunt-and-replace rather than open-ended writing. Clearing all `‹?›`
markers is part of the exit criteria.

## Phase 2 — Pick the look

Show the template options so the choice is visual, not abstract:
- Run `templates --format json`; each entry has a `preview` URL — offer those.
- Even better once a draft exists: `build` the user's *own* draft in 2–3
  candidate templates (`--template <name> --output preview-<name>.pdf`) and show
  those, so they compare real pages with their content.
- Resume: `modern` (default), `crisp`, `serif`, `split`. CV: `formal`, `sidebar`.
  Cover letters: `modern`, `classic` (set `meta.kind: cover-letter`). Set the
  pick in `meta.template`. Colors via `meta.color` (preset or `#hex`).

Selection tactics (which template suits whom) are in `reference/coaching.md`.

## Phase 3 — Interview until the context is clear

Now fill in the truth behind the draft. This is iterative: ask, ingest, update
the YAML, rebuild, repeat. Keep going until the resume tells a coherent story —
not merely until fields are non-empty. You're done interviewing when you could
explain *why this person is a strong candidate for their target* in two
sentences.

**Ingesting sources** (pull structure from whatever they have; don't make them
retype):
- **Local files** — an old resume, LinkedIn PDF export, a `.txt`/`.md` of notes:
  `Read` the file and extract roles, dates, achievements into the schema. For
  `.docx`, extract text (e.g. `unzip -p file.docx word/document.xml` then strip
  tags, or a converter if available) rather than asking them to paste it.
- **Images / screenshots** — a photo of a profile, a certificate, handwritten
  notes: `Read` the image and pull the content in.
- **URLs** — a job posting, a GitHub profile, a portfolio: `WebFetch` it. Note
  LinkedIn profile URLs usually hit an auth wall; if fetch fails, ask the user to
  paste the text or export the PDF instead. Don't loop on a blocked URL.
- **A header photo** — the `sidebar`/`split` look supports one via
  `meta.photo` (see `reference/schema.md`). Offer it; it's optional.

**What to ask** — target the gaps that make a resume weak. Don't ask everything;
ask what's missing and what's vague. The high-leverage questions (surfacing
quantified impact, seniority signals, scope) are in `reference/coaching.md` —
consult it when you're deciding what to probe. After each answer, update the YAML
and rebuild so the user always sees progress.

## Phase 4 — Review the collected information

Before polishing, **play the assembled picture back** and get it confirmed —
this catches wrong dates, misremembered scope, and anything you inferred. Summarize
the structured content (roles, dates, headline achievements, skills, the
narrative) in the chat and ask: "Here's what I have — is any of it wrong or
missing before I tighten the wording?" Fix, rebuild, and only then move on. No
`‹?›` markers should survive this phase.

## Phase 5 — Refine and coach (the part that makes it *good*)

This is where you earn the "coach" role — the user may not know how to make a
resume strong, so contribute actively rather than transcribing. Read
`reference/coaching.md` for the full playbook; the essentials:

- **Rewrite weak bullets** into *action verb → what you did → quantified impact*.
  Turn "Responsible for the database" into "Cut p99 query latency 40% by
  redesigning the indexing strategy for a 2 TB Postgres cluster." Where a number
  is missing, ask for it — impact without magnitude is the most common weakness.
- **Generate content** the user is struggling to produce: a summary, a skills
  taxonomy, a bullet for an experience they described only vaguely.
- **Align to the target job** (Phase 0's role/JD). Mine the posting for its
  priorities and language, then reorder sections and bullets so the most relevant
  evidence is on top, and mirror the posting's real keywords where the user
  genuinely has that experience (never fabricate it). If they gave a JD, tailoring
  is your single biggest quality lever — do it explicitly and tell them what you
  changed and why.
- Keep length honest (1 page early-career, 2 max unless it's an academic CV) and
  keep iterating — build after each change so the user sees each improvement.

## Anti-rationalization

| Tempting shortcut | Do this instead |
| --- | --- |
| Open with a long questionnaire. | No — draft first (Phase 1). Questions land better against a concrete draft. |
| Leave sections empty because you lack info. | Fill them with plausible `‹?›`-marked content so the page looks whole; replace during the interview. |
| Ship a draft with `‹?›` markers still in it. | Those are unconfirmed guesses — resolve every one before calling it done. |
| Transcribe what the user says verbatim into weak bullets. | Coach it: action + impact + number. Suggest, don't just record. |
| Skip asking for a target job. | Always ask (Phase 0) — a tailored resume is the biggest quality win available. |
| Make the user retype their old resume / LinkedIn. | Ingest it: `Read` the file/image or `WebFetch` the URL. |
| Output HTML/Markdown "they can print." | No — run `mkcv build` for a real typeset PDF. |
| Skip `validate` before `build`. | Validate first so a schema error is a clear `ok:false`, not a render failure. |
| Invent a field that isn't in the schema. | Check `schema`; omit what doesn't map — the layout adapts. |

## Exit criteria

Done when:
1. `validate` → `{"ok":true}` and `build` → `{"ok":true, "output":…, "pages":N}`,
   output file exists, path reported.
2. **No `‹?›` markers remain** — every value is confirmed or intentionally kept.
3. The user has confirmed the content (Phase 4) and it's tailored to their target
   (or they explicitly opted out of tailoring).
4. Weak/impact-free bullets have been coached, not left as-is.

If any command returns `ok:false`, surface its `error`/`errors` and fix the input
before retrying.

## Reference files

- `reference/schema.md` — the YAML field map (sections, profile, photo, letter).
- `reference/coaching.md` — the coaching playbook: interview question bank, the
  bullet formula, job-alignment tactics, and template selection guidance. Read it
  during Phases 3 and 5.
