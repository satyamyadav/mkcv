#!/usr/bin/env sh
# Install the mkcv agent skill into your agent's skills directory.
#
#   curl -fsSL https://raw.githubusercontent.com/satyamyadav/mkcv/main/skills/mkcv/install.sh | sh
#
# Overrides:
#   MKCV_SKILL_DIR   where to install (default: ~/.claude/skills/mkcv)
#   MKCV_SKILL_BASE  raw URL base to fetch from (default: the mkcv repo, main)
set -eu

DEST="${MKCV_SKILL_DIR:-$HOME/.claude/skills/mkcv}"
BASE="${MKCV_SKILL_BASE:-https://raw.githubusercontent.com/satyamyadav/mkcv/main/skills/mkcv}"

command -v curl >/dev/null 2>&1 || { echo "mkcv: curl is required" >&2; exit 1; }

mkdir -p "$DEST/scripts" "$DEST/reference"
curl -fsSL "$BASE/SKILL.md"                 -o "$DEST/SKILL.md"
curl -fsSL "$BASE/scripts/ensure-binary.sh" -o "$DEST/scripts/ensure-binary.sh"
curl -fsSL "$BASE/reference/schema.md"      -o "$DEST/reference/schema.md"
chmod +x "$DEST/scripts/ensure-binary.sh"

echo "mkcv skill installed to $DEST"
echo "The binary is fetched automatically on first use (or set MKCV_BIN)."
