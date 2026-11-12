#!/usr/bin/env bash
# tc3_xmlsrv_io.sh — Tc2_XmlDataSrv e2e: builds
# compat/tc3/tc3_xmlsrv.st as a --library package and replays
# tc3_xmlsrv_case.st through tether-plc-sim (file-backed XML store:
# alloc/write/read/realloc-preserve/range-error/dealloc/counters).
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

"$TPLCC" build --library --name Tc2_XmlDataSrv \
    compat/tc3/tc3_xmlsrv.st -o "$WORK/tc3_xml.tplc" >/dev/null 2>&1 \
    && ok "Tc2_XmlDataSrv library" || { bad "Tc2_XmlDataSrv library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_xmlsrv_case.st \
    --lib "$WORK/tc3_xml.tplc" \
    -o "$WORK/xml_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 80); do
        t=$((i * 10000000))
        if [ "$i" -eq 80 ]; then
            echo "step $i time=$t in 0=00 expect 0=7f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/xml.steps"

IMG=/tplc_tc3xs_$$; CTL="$WORK/dm.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/xml_case.tplc" \
    --steps "$WORK/xml.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: write/read/ByName/missing-key/obfuscation"
else
    bad "sim replay: write/read/ByName/missing-key/obfuscation"
    tail -20 "$WORK/sim.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
