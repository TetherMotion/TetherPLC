#!/usr/bin/env bash
# ensure_matiec.sh — fetch + build the matiec iec2c oracle at a pinned
# commit into MATIEC_HOME (default ~/.cache/tplc-oracle/matiec), outside
# the repository tree. matiec is GPL: it is a subprocess oracle only —
# never vendored, never linked, its outputs are data. Idempotent.
set -euo pipefail

PIN=01a3a6c0c29b9aa0322ab6aba6258ebef2e36155
HOME_DIR="${MATIEC_HOME:-$HOME/.cache/tplc-oracle/matiec}"
IEC2C="$HOME_DIR/src/build/bin/iec2c"

if [ -x "$IEC2C" ]; then
    echo "$IEC2C"
    exit 0
fi

mkdir -p "$HOME_DIR"
if [ ! -d "$HOME_DIR/src/.git" ]; then
    git clone --quiet https://github.com/beremiz/matiec "$HOME_DIR/src"
fi
cd "$HOME_DIR/src"
git checkout --quiet "$PIN"
git submodule update --init 2>/dev/null || true

cmake -S . -B build -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release >/dev/null
cmake --build build --target iec2c -j8 >/dev/null

[ -x "$IEC2C" ] || { echo "ensure_matiec: iec2c build failed" >&2; exit 1; }
echo "$IEC2C"
