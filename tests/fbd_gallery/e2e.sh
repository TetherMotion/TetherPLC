#!/usr/bin/env bash
# FBD gallery conformance (doc 06 corpus): each .st/.fbd.json pair runs
# the same input trace and must produce bit-identical %Q images, plus
# the expect_q expectations. Exit 77 = prerequisites missing → skip.
set -uo pipefail
cd "$(dirname "$0")/../.."

# tplcc needs an LLVM-21 toolchain — locate one if not already on PATH
if [ -z "${LLVM_SYS_211_PREFIX:-}" ]; then
    for p in /home/uli/llvm21/usr/lib/llvm-21 /home/uli/llvm21/usr /opt/llvm21 /usr/lib/llvm-21; do
        if [ -x "$p/bin/llvm-config" ]; then
            export LLVM_SYS_211_PREFIX=$p
            export LLVM_CONFIG=$p/bin/llvm-config
            export PATH="$p/bin:$PATH"
            break
        fi
    done
fi
(cd compiler && cargo build -q -p tplcc 2>/dev/null) || true
TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
[ -x "$TPLCC" ] || { echo "SKIP: no tplcc"; exit 77; }

"$TPLCC" vectors run tests/fbd_gallery/vectors.jsonl --src tests/fbd_gallery
