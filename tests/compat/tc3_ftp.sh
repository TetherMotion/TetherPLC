#!/usr/bin/env bash
# tc3_ftp.sh — Tc3_FTP + Tc2_FTP compat (plan 21 C4):
# build compat/tc3/tc3_ftp.st as a --library package, consume
# it into tests/compat/tc3_ftp_case.st via --lib, and replay
# an autonomous FTP session through tether-plc-sim against
# the real tplc-ftp PASV client on TCP 2121.
# tests/compat/ftp_test_peer.py serves an in-memory filesystem
# (/data.txt 68B, /bin.dat, dirs / and /pub) with USER/PASS,
# MKD/RMD/DELE/SIZE/RNFR/RNTO and RETR/STOR/LIST over PASV.
# The program packs 12 checks into %QD0; the last step expects
# 0xfff.
# Runs without --scan: the PLC free-runs at its task period and
# each step samples the next committed scan, so 500 steps span
# several seconds of wall time — enough for the login plus
# eleven operations (each xfer runs TYPE+PASV+data+final).
set -u
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

python3 tests/compat/ftp_test_peer.py >"$WORK/peer.log" 2>&1 &
PEER=$!
trap 'kill $PEER 2>/dev/null; rm -rf "$WORK"' EXIT
sleep 0.5

"$TPLCC" build --library --name Tc3_FTP \
    compat/tc3/tc3_ftp.st -o "$WORK/tc3_ftp.tplc" >/dev/null 2>&1 \
    && ok "Tc3_FTP library" || bad "Tc3_FTP library"

"$TPLCC" build tests/compat/tc3_ftp_case.st \
    --lib "$WORK/tc3_ftp.tplc" \
    -o "$WORK/tc3_ftp_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 499); do
        t=$((i * 10000000))
        if [ "$i" -eq 499 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff0f0000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/ftp.steps"

IMG=/tplc_tc3ftp_$$; CTL="$WORK/ftp.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc3_ftp_case.tplc" \
    --steps "$WORK/ftp.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: conn/size/mkd/retr/list/stor/sizeup/fex/dex/rename/connerr/close"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
    tail -8 "$WORK/peer.log"
fi

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
