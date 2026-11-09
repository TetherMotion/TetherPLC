#!/usr/bin/env bash
# od_surface.sh — CiA-405-style object dictionary over ctl (doc 16 §7
# iteration-2 item): LIST/READ/WRITE against the management state
# model, entry-level authz, and the reserved MGMT_* codes.
#   1. LIST enumerates the dictionary
#   2. READ returns typed values for known indices; unknown → MGMT_OP
#   3. WRITE to a read-only object → MGMT_SCHEMA
#   4. WRITE below the object's role floor → MGMT_AUTHZ
#   5. WRITE the retain-flush object as commissioner flushes + journals
#
# Usage: od_surface.sh <tether-plc> <tplc-image> <tplc-ctl> <stub.so>
set -u

PLC="$1"; IMG="$2"; CTL="$3"; STUB="$4"
NAME="od_$$"
WORK="/tmp/${NAME}"
SOCK="$WORK/ctl"
mkdir -p "$WORK" "$WORK/retain"
MYUID=$(id -u)
PID=
fail() { echo "FAIL: $*"; [ -n "$PID" ] && kill "$PID" 2>/dev/null; exit 1; }
pass() { echo "PASS: $*"; }
trap '[ -n "$PID" ] && kill "$PID" 2>/dev/null; "$IMG" unlink "$NAME" 2>/dev/null; rm -rf "$WORK"' EXIT

boot() {  # boot <role>
    "$IMG" create "$NAME" 256 256 >/dev/null 2>&1
    echo "$MYUID $1" > "$WORK/authz"
    "$PLC" --image "$NAME" --module "$STUB" --ctl "$SOCK" \
        --authz "$WORK/authz" --audit-log "$WORK/audit.log" \
        --retain "$WORK/retain" >"$WORK/run.log" 2>&1 &
    PID=$!
    sleep 1
}

# -- 1: observer lists and reads ------------------------------------
boot observer
out=$("$CTL" "$SOCK" od-list 2>&1)
echo "$out" | grep -qE 'od [0-9]+ entries' \
    || fail "od-list: $out"
echo "$out" | grep -q '0x2000/0.*state' \
    || fail "od-list lacks state row: $out"
echo "$out" | grep -q '0x2100/1.*task_stats' \
    || fail "od-list lacks per-task row: $out"
pass "od-list enumerates the dictionary"

out=$("$CTL" "$SOCK" od-read 0x2000 0 2>&1)
echo "$out" | grep -q 'type u8' || fail "od-read state: $out"
out=$("$CTL" "$SOCK" od-read 0x2002 0 2>&1)
echo "$out" | grep -q 'type u64' || fail "od-read cycles: $out"
out=$("$CTL" "$SOCK" od-read 0x2301 0 2>&1)
echo "$out" | grep -q 'len 32' || fail "od-read layout_hash: $out"
pass "od-read returns typed values"

out=$("$CTL" "$SOCK" od-read 0x9999 0 2>&1)
echo "$out" | grep -q 'error 500' \
    || fail "unknown index should be MGMT_OP (500): $out"
pass "unknown object → MGMT_OP (500)"

out=$("$CTL" "$SOCK" od-write 0x2000 0 01 2>&1)
echo "$out" | grep -q 'error 401' \
    || fail "observer write should be gated (401): $out"
kill "$PID" 2>/dev/null; wait "$PID" 2>/dev/null; PID=

# -- 2: engineer hits the object's own role floor -------------------
boot engineer
out=$("$CTL" "$SOCK" od-read 0x2000 0 2>&1)
echo "$out" | grep -q 'type u8' || fail "engineer read: $out"
out=$("$CTL" "$SOCK" od-write 0x2000 0 01 2>&1)
echo "$out" | grep -q 'error 503' \
    || fail "read-only object should be MGMT_SCHEMA (503): $out"
out=$("$CTL" "$SOCK" od-write 0x2400 1 01 2>&1)
echo "$out" | grep -q 'error 502' \
    || fail "engineer below object floor should be MGMT_AUTHZ (502): $out"
pass "read-only → MGMT_SCHEMA (503); below floor → MGMT_AUTHZ (502)"
kill "$PID" 2>/dev/null; wait "$PID" 2>/dev/null; PID=

# -- 3: commissioner flushes retain through the dictionary ----------
boot commissioner
out=$("$CTL" "$SOCK" od-write 0x2400 1 01 2>&1)
echo "$out" | grep -q '^ok' \
    || fail "commissioner retain_flush should succeed: $out"
grep -q 'op=od.write' "$WORK/audit.log" \
    || fail "journal lacks od.write record"
pass "od-write retain_flush flushes + journals"

out=$("$CTL" "$SOCK" od-write 0x2400 1 0102 2>&1)
echo "$out" | grep -q 'error 503' \
    || fail "wrong-length write should be MGMT_SCHEMA: $out"
pass "type/length mismatch → MGMT_SCHEMA (503)"

kill "$PID" 2>/dev/null; wait "$PID" 2>/dev/null; PID=
echo "od_surface: all checks passed"
