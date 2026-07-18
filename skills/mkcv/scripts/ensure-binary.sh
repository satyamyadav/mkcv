#!/usr/bin/env bash
# Resolve (and if needed download) the mkcv binary, printing its path on
# stdout. On failure, prints an error and exits non-zero.
#
# Resolution order:
#   1. $MKCV_BIN (explicit override)
#   2. `mkcv` on PATH
#   3. cached copy under $CACHE_DIR
#   4. the repo's dist/<target>/mkcv — from a local clone ($MKCV_REPO)
#      or downloaded from a raw URL ($MKCV_REPO_RAW)
set -euo pipefail

CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/mkcv"

# --- Map the host to a Rust target triple ------------------------------------
os="$(uname -s)"; arch="$(uname -m)"
case "$os/$arch" in
  Linux/x86_64)        target="x86_64-unknown-linux-gnu" ;;
  Linux/aarch64)       target="aarch64-unknown-linux-gnu" ;;
  Darwin/arm64)        target="aarch64-apple-darwin" ;;
  Darwin/x86_64)       target="x86_64-apple-darwin" ;;
  *) echo "mkcv: no prebuilt binary for $os/$arch (build from source: cargo build --release)" >&2; exit 1 ;;
esac
rel="dist/$target/mkcv"

usable() { [ -x "$1" ] && "$1" --version >/dev/null 2>&1; }

# 1. Explicit override
if [ -n "${MKCV_BIN:-}" ] && usable "$MKCV_BIN"; then echo "$MKCV_BIN"; exit 0; fi

# 2. On PATH
if command -v mkcv >/dev/null 2>&1 && usable "$(command -v mkcv)"; then
  command -v mkcv; exit 0
fi

# 3. Cached
cached="$CACHE_DIR/$target/mkcv"
if usable "$cached"; then echo "$cached"; exit 0; fi

# 4a. Local clone of the repo
if [ -n "${MKCV_REPO:-}" ] && usable "$MKCV_REPO/$rel"; then echo "$MKCV_REPO/$rel"; exit 0; fi
# The skill lives at skills/mkcv/ inside the repo; try ../../dist too.
here="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
if usable "$here/$rel"; then echo "$here/$rel"; exit 0; fi

# 4b. Download from the repo (default: the mkcv repo's raw URL; override with
#     MKCV_REPO_RAW to point at a fork/branch).
base="${MKCV_REPO_RAW:-https://raw.githubusercontent.com/satyamyadav/mkcv/main}"
mkdir -p "$CACHE_DIR/$target"
if command -v curl >/dev/null 2>&1; then
  curl -fsSL "$base/$rel" -o "$cached"
elif command -v wget >/dev/null 2>&1; then
  wget -qO "$cached" "$base/$rel"
else
  echo "mkcv: need curl or wget to download the binary" >&2; exit 1
fi
chmod +x "$cached"
if usable "$cached"; then echo "$cached"; exit 0; fi
echo "mkcv: downloaded binary is not runnable ($cached)" >&2; exit 1
