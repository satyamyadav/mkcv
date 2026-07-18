---
name: mkcv
description: >-
  Compile a YAML data file into a pixel-perfect PDF resume, CV, or cover letter
  using the embedded-Typst `mkcv` engine. Use when the user wants to create,
  update, or re-style a resume / CV / cover letter from structured data, switch
  templates, or validate a resume.yml. Drives the `mkcv` CLI, whose commands are
  discrete tool-like operations with `--format json` output.
---

# mkcv — compile a resume/CV/cover letter from YAML

`mkcv` turns one `resume.yml` into a typeset PDF. It is a single self-contained
binary (embedded Typst engine + embedded fonts) — no LaTeX, no browser, no
network at compile time. Every command accepts `--format json`, so you parse
results instead of scraping text.

## Entry criteria

Use this skill when any of these hold:
- The user asks for a new resume, academic CV, or cover letter.
- The user pastes unstructured career history and wants a clean printable PDF.
- The user wants to restyle an existing `resume.yml` (switch template/color) or
  check that it is valid.

## Workflow

Follow the steps in order. Prefer `--format json` and read the `ok` field.

### 1. Get the binary
Run the bundled bootstrap and **capture its stdout** as the binary path (it
resolves `$MKCV_BIN` → `mkcv` on PATH → cache → the repo's committed
`dist/<target>/mkcv`, downloading once if needed):
```bash
BIN=$(bash "$CLAUDE_SKILL_DIR/scripts/ensure-binary.sh") || { echo "$BIN"; exit 1; }
```
If it prints an error (e.g. an unsupported platform), report it and stop.

### 2. Discover templates + schema (don't guess field names)
```bash
"$BIN" templates --format json   # names + categories
"$BIN" schema --format json      # the full YAML schema
```
Templates — resume: `modern` (default), `crisp`, `serif`, `split`; cv:
`formal`, `sidebar`; cover letters: `modern`, `classic` (set
`meta.kind: cover-letter`). `meta.template` may also be a local `./file.typ`.

### 3. Author or edit `resume.yml`
- No file yet → `"$BIN" init` writes a complete boilerplate to `resume.yml`.
- Only `profile.name` (or `first_name`+`last_name`) is required; omit unknown
  fields — the layout adapts. Free-text fields accept `**bold**`, `*italic*`,
  `[links](url)`. See `reference/schema.md`.

### 4. Validate, then build
```bash
"$BIN" validate --input resume.yml --format json     # {"ok":true} or {"ok":false,"errors":[…]}
"$BIN" build --input resume.yml --output resume.pdf --format json
# → {"ok":true,"output":"resume.pdf","pages":1,"ms":18.0}
```
Useful build flags: `--template <name-or-./file.typ>`, `--kind cover-letter`,
`--yaml '<inline yaml>'`, `--output <path>`.

## Anti-rationalization

Do not take these shortcuts:

| Tempting shortcut | Do this instead |
| --- | --- |
| "I'll just output HTML/Markdown and let them print it." | No. The user wants a real typeset PDF. Run `mkcv build`. |
| "This custom section isn't in the schema, I'll drop the file / invent a field." | No. Every field except the name is optional; simply omit what doesn't map — the layout adapts. Check `mkcv schema` for the real fields. |
| "The YAML looks fine, I'll skip `validate`." | No. Run `mkcv validate --format json` first so a schema error is a clear `ok:false`, not a rendering failure. |
| "I'll hand-write a `.typ` and hope it compiles." | Only via `meta.template: ./file.typ`; a custom template gets `data` + `_core.typ` helpers and must be self-contained. Otherwise use a built-in template. |

## Exit criteria

Consider the task done only when:
1. `validate` returned `{"ok":true}` (or the user accepted a known warning).
2. `build` returned `{"ok":true, "output": …, "pages": N}` and the output file
   exists.
3. You reported the output path and page count to the user (build time is a
   metric to mention, not a pass/fail threshold).

If any command returned `ok:false`, surface its `error`/`errors` to the user and
fix the input before retrying.
