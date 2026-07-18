# mkcv skill — install

A Claude agent skill that compiles a `resume.yml` into a PDF resume / CV / cover
letter via the `mkcv` CLI. It works out of the box: on first use the skill
downloads the `mkcv` binary from the repo
(`https://raw.githubusercontent.com/satyamyadav/mkcv/main/dist/<target>/mkcv`)
and caches it under `~/.cache/mkcv`.

Install it any of three ways.

## 1. Command (one-liner)

With the [`skills`](https://skills.addy.ie) CLI (installs into your active agent
platform):

```bash
npx skills add satyamyadav/mkcv --skill mkcv
```

Or run the remote installer (`curl | sh`, installs into
`~/.claude/skills/mkcv`; override with `MKCV_SKILL_DIR=…`):

```bash
curl -fsSL https://raw.githubusercontent.com/satyamyadav/mkcv/main/skills/mkcv/install.sh | sh
```

## 2. Git repo / plugin

Clone the repo (it bundles this skill under `skills/mkcv/` **and** the binary
under `dist/`, so no download is needed):

```bash
git clone https://github.com/satyamyadav/mkcv && cd mkcv
export MKCV_REPO="$PWD"     # skill uses the checked-out dist/<target>/mkcv
```

Then register the repo as a Claude Code plugin per your plugin workflow.

## 3. Copy-paste

Create `~/.claude/skills/mkcv/` and paste the contents of
[`SKILL.md`](./SKILL.md) and [`scripts/ensure-binary.sh`](./scripts/ensure-binary.sh).
The bootstrap will download the binary on first use.

## Binary resolution

`scripts/ensure-binary.sh` finds the binary in order: `$MKCV_BIN` → `mkcv` on
`PATH` → a cached copy → a repo clone (`$MKCV_REPO` or auto-detected) →
**download** from `$MKCV_REPO_RAW` (default: the mkcv repo's `main`).

- Set `MKCV_BIN=/path/to/mkcv` to use a specific binary.
- Set `MKCV_REPO_RAW=https://raw.githubusercontent.com/<owner>/<branch-base>` to
  pull from a fork/branch.
- v1 ships a Linux x86_64 (`gnu`) binary; other platforms build from source
  (`cargo build --release`) until v2's release automation lands.
