#!/usr/bin/env bash
# gpl_hygiene.sh — assert the tools/oracle-* quarantine (doc 14 §3):
# GPL oracles are subprocess data-producers only; nothing they touch
# may end up compiled into, linked with, or embedded in shipped paths.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
fail=0

say_fail() { echo "HYGIENE-FAIL: $1"; fail=1; }

# 1) shipped source must not reference oracle internals (matiec's
#    generated-code API). Naming the tools in comments/docs/test names
#    is fine — oracle outputs are data; only these tokens mean code
#    coupling.
viol="$(git grep -lE "iec_std_lib|accessor\.h|__LOCATED_VAR|iec2c " \
        -- include/ runtime/ compiler/ tools/ \
           ':!tools/oracle-matiec' ':!tools/oracle-beremiz' \
           ':!tools/oracle' || true)"
[ -z "$viol" ] || say_fail "oracle internals referenced outside quarantine: $viol"

# 2) no shipped binary embeds oracle strings.
for b in tether-plc tether-plc-sim tplc-ctl tplc-image; do
    [ -f "build/$b" ] || continue
    if strings "build/$b" | grep -iqE "matiec|iec2c|beremiz"; then
        say_fail "$b embeds oracle strings"
    fi
done

# 3) no oracle file is a compiled source in the project build:
#    CMakeLists may register them only behind add_test/bash/python.
if grep -nE "add_(executable|library)|target_sources" CMakeLists.txt \
        | grep -qE "oracle"; then
    say_fail "oracle file listed as a compiled source in CMakeLists.txt"
fi

if [ "$fail" = 0 ]; then
    echo "PASS gpl_hygiene: oracle quarantine intact"
    exit 0
fi
exit 1
