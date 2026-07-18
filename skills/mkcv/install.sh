#!/usr/bin/env sh
# mkcv — one script: remote installer + binary resolver (all platforms).
#
#   curl -fsSL https://raw.githubusercontent.com/satyamyadav/mkcv/main/skills/mkcv/install.sh | sh
#       Install the mkcv skill (SKILL.md + this script) and fetch the binary.
#
#   sh install.sh --bin
#       Ensure the binary is present and print ONLY its path (used by the skill).
#
# Env overrides:
#   MKCV_BIN        use this binary, skip detection/download
#   MKCV_SKILL_DIR  where to install the skill (default: ~/.claude/skills/mkcv)
#   MKCV_REPO       a local repo clone (uses its dist/ binary)
#   MKCV_REPO_RAW   raw URL base to fetch from (default: the mkcv repo, main)
set -eu

RAW="${MKCV_REPO_RAW:-https://raw.githubusercontent.com/satyamyadav/mkcv/main}"
CACHE="${XDG_CACHE_HOME:-$HOME/.cache}/mkcv"
SKILL_DIR="${MKCV_SKILL_DIR:-$HOME/.claude/skills/mkcv}"
QUIET=0

log()  { [ "$QUIET" = 1 ] || echo "$@" >&2; }
die()  { echo "mkcv: $*" >&2; exit 1; }
usable() { [ -x "$1" ] && "$1" --version >/dev/null 2>&1; }

# Map the host to the committed dist artifact (short names). Returns non-zero
# for platforms we don't publish a prebuilt for.
artifact() {
  case "$(uname -s) $(uname -m)" in
    "Linux x86_64")  echo "mkcv" ;;
    "Darwin arm64")  echo "mkcv-macos-arm64" ;;   # published by CI later
    "Darwin x86_64") echo "mkcv-macos-x64" ;;     # published by CI later
    *) return 1 ;;
  esac
}

fetch() { # url dest  (returns non-zero on HTTP/transport error)
  if   command -v curl >/dev/null 2>&1; then curl -fsSL "$1" -o "$2"
  elif command -v wget >/dev/null 2>&1; then wget -qO "$2" "$1"
  else die "need curl or wget"; fi
}

no_prebuilt() {
  die "no prebuilt binary for $(uname -s)/$(uname -m) yet — build from source \
('cargo build --release') and set MKCV_BIN=/path/to/mkcv"
}

# Ensure a runnable binary exists; echo its path on stdout.
ensure_bin() {
  # 1. explicit override, 2. on PATH — work on any platform.
  if [ -n "${MKCV_BIN:-}" ] && usable "$MKCV_BIN"; then echo "$MKCV_BIN"; return 0; fi
  if command -v mkcv >/dev/null 2>&1 && usable "$(command -v mkcv)"; then command -v mkcv; return 0; fi

  art="$(artifact)" || no_prebuilt
  cached="$CACHE/mkcv"

  # 3. cache
  if usable "$cached"; then echo "$cached"; return 0; fi
  # 4. local repo clone
  if [ -n "${MKCV_REPO:-}" ] && usable "$MKCV_REPO/dist/$art"; then echo "$MKCV_REPO/dist/$art"; return 0; fi
  # 5. download
  mkdir -p "$CACHE"
  fetch "$RAW/dist/$art" "$cached" 2>/dev/null || { rm -f "$cached"; no_prebuilt; }
  chmod +x "$cached"
  # Clear macOS Gatekeeper quarantine on downloaded binaries.
  [ "$(uname -s)" = "Darwin" ] && xattr -d com.apple.quarantine "$cached" >/dev/null 2>&1 || true
  usable "$cached" || die "downloaded binary is not runnable ($cached)"
  echo "$cached"
}

# --- --bin mode: quiet, print only the path (used by the skill) --------------
if [ "${1:-}" = "--bin" ]; then
  QUIET=1
  ensure_bin
  exit 0
fi

# --- install mode: skill files + binary --------------------------------------
log "Installing the mkcv skill -> $SKILL_DIR"
mkdir -p "$SKILL_DIR/reference"
fetch "$RAW/skills/mkcv/SKILL.md"            "$SKILL_DIR/SKILL.md"
fetch "$RAW/skills/mkcv/reference/schema.md" "$SKILL_DIR/reference/schema.md"
fetch "$RAW/skills/mkcv/install.sh"          "$SKILL_DIR/install.sh"
chmod +x "$SKILL_DIR/install.sh"

bin="$(ensure_bin)"
log "Binary ready: $bin"

# Make `mkcv` runnable as a command for direct/standalone use (best-effort).
if [ "$bin" = "$CACHE/mkcv" ]; then
  mkdir -p "$HOME/.local/bin"
  ln -sf "$bin" "$HOME/.local/bin/mkcv"
  log "Linked -> ~/.local/bin/mkcv (add ~/.local/bin to PATH to run 'mkcv' directly)"
fi

log "Done. Ask your agent to build your resume, or run: mkcv init && mkcv build"
