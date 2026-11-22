#!/usr/bin/env bash
# e2e_all.sh — run every standalone e2e script under tests/ and report
# a pass/skip/fail tally. Used by the linux-x86_64-e2e CI job; also
# runnable locally (scripts self-skip when prerequisites are absent).
#
#   rc 0   -> PASS
#   rc 77  -> SKIP (missing prerequisite — same convention as ctest)
#   rc 124 -> FAIL (hit $E2E_TIMEOUT, default 600 s per script)
#   else   -> FAIL
#
# Env: E2E_TIMEOUT=<sec>   per-script timeout (default 600)
#      E2E_EXCLUDE=<regex> scripts matching are skipped up front
#      E2E_ONLY=<regex>     run only matching scripts
set -u
cd "$(dirname "$0")/.."

TIMEOUT=${E2E_TIMEOUT:-600}
EXCLUDE=${E2E_EXCLUDE:-}
ONLY=${E2E_ONLY:-}

pass=0; fail=0; skip=0
failed=()
while IFS= read -r t; do
    [ "$t" = "tests/e2e_all.sh" ] && continue
    rel=${t#tests/}
    if [ -n "$ONLY" ] && ! echo "$rel" | grep -qE "$ONLY"; then continue; fi
    if [ -n "$EXCLUDE" ] && echo "$rel" | grep -qE "$EXCLUDE"; then
        printf 'SKIP   %s (excluded)\n' "$rel"; skip=$((skip+1)); continue
    fi
    printf '== %s\n' "$rel"
    timeout "$TIMEOUT" bash "$t"
    rc=$?
    case $rc in
        0)   printf 'PASS   %s\n' "$rel"; pass=$((pass+1));;
        77)  printf 'SKIP   %s\n' "$rel"; skip=$((skip+1));;
        124) printf 'FAIL   %s (timeout %ss)\n' "$rel" "$TIMEOUT"; fail=$((fail+1)); failed+=("$rel (timeout)");;
        *)   printf 'FAIL   %s (exit %d)\n' "$rel" "$rc"; fail=$((fail+1)); failed+=("$rel (exit $rc)");;
    esac
done < <(find tests -name '*.sh' -type f | sort)

echo "=================================================="
echo "== e2e tally: $pass pass, $skip skip, $fail fail"
if [ "$fail" -gt 0 ]; then
    printf '== failed: %s\n' "${failed[@]}"
    exit 1
fi
