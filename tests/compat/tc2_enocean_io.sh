#!/usr/bin/env bash
# tc2_enocean_io.sh — wired EnOcean ESP3 e2e: builds
# compat/tc2/tc2_enocean.st as a --library package, starts the
# PTY transceiver fake (enocean_fake.py), and replays
# tc2_enocean_case.st through tether-plc-sim.  Skips (77) when
# python3, the fake, or the toolchain is missing.  The case
# opens /tmp/eno_pty — the fake creates that symlink to its
# PTY slave.
set -u
cd "$(dirname "$0")/../.."
TPLCC=${TPLCC:-compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
FAKE=${ENOCEAN_FAKE:-tests/compat/enocean_fake.py}
[ -x "$TPLCC" ] || { echo "tplcc missing: $TPLCC" >&2; exit 77; }
[ -x "$BUILD/tether-plc-sim" ] || { echo "tether-plc-sim missing" >&2; exit 77; }
command -v python3 >/dev/null 2>&1 || { echo "python3 missing" >&2; exit 77; }
[ -f "$FAKE" ] || { echo "enocean_fake missing" >&2; exit 77; }
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'kill $SRV_PID 2>/dev/null; rm -f /tmp/eno_pty; rm -rf "$WORK"' EXIT

python3 "$FAKE" "$WORK/rx.log" >"$WORK/fake.out" 2>&1 &
SRV_PID=$!
sleep 0.8
kill -0 "$SRV_PID" 2>/dev/null || { bad "enocean fake start"; cat "$WORK/fake.out"; echo "== $pass pass, $fail fail"; exit 1; }
PTY=$(grep '^pty:' "$WORK/rx.log" | cut -d: -f2)
[ -n "$PTY" ] || { bad "enocean fake pty"; cat "$WORK/fake.out"; echo "== $pass pass, $fail fail"; exit 1; }
ln -sf "$PTY" /tmp/eno_pty

"$TPLCC" build --library --name Tc2_EnOcean \
    compat/tc2/tc2_enocean.st -o "$WORK/tc2_enocean.tplc" >/dev/null 2>&1 \
    && ok "Tc2_EnOcean library" || { bad "Tc2_EnOcean library"; \
         echo "== $pass pass, $fail fail"; exit 1; }

"$TPLCC" build tests/compat/tc2_enocean_case.st \
    --lib "$WORK/tc2_enocean.tplc" \
    -o "$WORK/tc2_enocean_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 300); do
        t=$((i * 10000000))
        if [ "$i" -eq 300 ]; then
            echo "step $i time=$t in 0=00 expect 0=1f"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/eno.steps"

IMG=/tplc_tc2eno_$$; CTL="$WORK/eno.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_enocean_case.tplc" \
    --steps "$WORK/eno.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 2000 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: EnOcean open/send/radio/ptm/close"
else
    bad "sim replay: EnOcean open/send/radio/ptm/close"
    tail -20 "$WORK/sim.log"
    cat "$WORK/rx.log" 2>/dev/null
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
