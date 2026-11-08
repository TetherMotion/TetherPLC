#!/usr/bin/env bash
# sanitizer passes (doc 14 §5): ASan+UBSan over the conformance suite,
# TSan over the seqlock image protocol. Each sanitizer gets its own
# build dir; only the C-test targets compile (ctl_smoke /
# fault_injection need the full tether-plc binary — out of scope).
# Requires: cmake + a compiler with the sanitizers (exit 77 skips).
set -uo pipefail
cd "$(dirname "$0")/.."

command -v cmake >/dev/null || { echo "SKIP: no cmake"; exit 77; }
command -v cc  >/dev/null || { echo "SKIP: no cc";  exit 77; }

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

run_suite() { # <name> <sanitizer-flags> <tests...>
    local name="$1" flags="$2"; shift 2
    local dir="build-san-$name"
    cmake -S . -B "$dir" -DCMAKE_BUILD_TYPE=Debug \
        -DCMAKE_C_FLAGS="-fsanitize=$flags -fno-omit-frame-pointer" \
        -DCMAKE_CXX_FLAGS="-fsanitize=$flags -fno-omit-frame-pointer" \
        -DCMAKE_EXE_LINKER_FLAGS="-fsanitize=$flags" \
        -DCMAKE_MODULE_LINKER_FLAGS="-fsanitize=$flags" \
        -DCMAKE_SHARED_LINKER_FLAGS="-fsanitize=$flags" \
        >"$dir.log" 2>&1 || { bad "$name: configure"; return; }
    cmake --build "$dir" -j8 --target "$@" >>"$dir.log" 2>&1 \
        || { bad "$name: build"; return; }
    local t ok_t=0
    for t in "$@"; do
        if ASAN_OPTIONS=detect_leaks=1 UBSAN_OPTIONS=halt_on_error=1 \
           "./$dir/$t" >>"$dir.log" 2>&1; then
            ok_t=$((ok_t+1))
        else
            bad "$name: $t"
        fi
    done
    [ "$ok_t" -gt 0 ] && ok "$name: $ok_t targets clean"
}

run_suite asan "address,undefined" \
    image_conformance abi_conformance pkg_conformance retain_store
run_suite tsan "thread" image_conformance

echo "== sanitizers: $pass pass, $fail fail"
[ "$fail" -eq 0 ]
