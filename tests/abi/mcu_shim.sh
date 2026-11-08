#!/usr/bin/env bash
# MCU static-link shim gate (plan/18-mcu-path.md §7 criterion 3).
# Runs the shim host (both module images linked in, static function
# tables) and asserts the binary pulls in NO dynamic-loading,
# filesystem, mmap or thread symbols — the ABI must never require
# a Linux host surface.
set -uo pipefail
HOST=${1:-}
[ -x "$HOST" ] || { echo "SKIP no shim host"; exit 77; }

pass=0; fail=0
ok()  { pass=$((pass+1)); echo "PASS $1"; }
bad() { fail=$((fail+1)); echo "FAIL $1"; }

out=$("$HOST" 2>&1); rc=$?
echo "$out" | grep -c '^PASS' | xargs -I{} echo "shim checks: {}"
[ "$rc" -eq 0 ] && ok "shim host conformance run" \
    || bad "shim host run (rc=$rc): $out"
echo "$out" | grep -q '^FAIL' && bad "shim checks failed: $out"

# no dynamic loading / files / mmap / threads in the undefined set
und=$(nm -u "$HOST" 2>/dev/null)
for sym in dlopen dlsym dlclose mmap munmap pthread_create \
           shm_open open fopen; do
    echo "$und" | grep -qw "$sym" \
        && bad "shim imports $sym (ABI drifted to Linux assumptions)" \
        || ok "no $sym import"
done

echo "== $pass pass, $fail fail"
[ "$fail" -eq 0 ]
