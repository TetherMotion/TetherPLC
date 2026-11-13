#!/usr/bin/env bash
# tc3_jsonxml_io.sh — Tc3_JsonXml e2e: builds
# compat/tc3/tc3_jsonxml.st as a --library package and replays
# tc3_jsonxml_case.st through tether-plc-sim (pure-ST JSON codec:
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

"$TPLCC" build --library --name Tc3_JsonXml \
    compat/tc3/tc3_jsonxml.st -o "$WORK/tc3_json.tplc" >/dev/null 2>&1 \
    && ok "Tc3_JsonXml library" || { bad "Tc3_JsonXml library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc3_jsonxml_case.st \
    --lib "$WORK/tc3_json.tplc" \
    -o "$WORK/json_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 80); do
        t=$((i * 10000000))
        if [ "$i" -eq 80 ]; then
            echo "step $i time=$t in 0=00 expect 0=ffff01"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/json.steps"

IMG=/tplc_tc3js_$$; CTL="$WORK/dm.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/json_case.tplc" \
    --steps "$WORK/json.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: write-shape/read-back int-real-bool-string/missing-key"
else
    bad "sim replay: write-shape/read-back int-real-bool-string/missing-key"
    tail -20 "$WORK/sim.log"
fi

"$TPLCC" build tests/compat/tc3_jsonxml_xml_case.st \
    --lib "$WORK/tc3_json.tplc" \
    -o "$WORK/xml_case.tplc" >/dev/null 2>&1 \
    && ok "XML consumer build with --lib" \
    || { bad "XML consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 55); do
        t=$((i * 10000000))
        if [ "$i" -eq 55 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff3f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/xml.steps"

IMG=/tplc_tc3xj_$$; CTL="$WORK/dmx.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/xml_case.tplc" \
    --steps "$WORK/xml.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/simx.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/simx.log"; then
    ok "sim replay: xml load/root/childbyname/attr/text/nodexml/toxml/build/error"
else
    bad "sim replay: xml load/root/childbyname/attr/text/nodexml/toxml/build/error"
    tail -20 "$WORK/simx.log"
fi

"$TPLCC" build tests/compat/tc3_jsonxml_pretty_case.st \
    --lib "$WORK/tc3_json.tplc" \
    -o "$WORK/pretty_case.tplc" >/dev/null 2>&1 \
    && ok "pretty consumer build with --lib" \
    || { bad "pretty consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 29); do
        t=$((i * 10000000))
        if [ "$i" -eq 29 ]; then
            echo "step $i time=$t in 0=00 expect 0=3f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/pretty.steps"

IMG=/tplc_tc3pj_$$; CTL="$WORK/dmp.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/pretty_case.tplc" \
    --steps "$WORK/pretty.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 1500 >"$WORK/simp.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/simp.log"; then
    ok "sim replay: pretty writer indent/nested/array/close"
else
    bad "sim replay: pretty writer indent/nested/array/close"
    tail -20 "$WORK/simp.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
