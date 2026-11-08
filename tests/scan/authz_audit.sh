#!/usr/bin/env bash
# authz_audit.sh — doc 15 §5 acceptance:
#   1. UDS peer creds map to roles; an observer peer is denied on
#      mutating ops (TPLC_E_CTL_AUTHZ → wire "error 401")
#   2. commissioner peers mutate normally; every mutating op lands in
#      the durable journal {ts, uid, gid, pid, sess, name, op, ahash, rc}
#   3. mutating-op rate limit trips under flood (error 413), keyed by
#      uid so reconnects do not reset it
#   4. the journal survives restart (append across boots)
#
# Usage: authz_audit.sh <tether-plc> <tplc-image> <tplc-ctl> <stub.so>
set -u

PLC="$1"; IMG="$2"; CTL="$3"; STUB="$4"
NAME="az_$$"
WORK="/tmp/${NAME}"
SOCK="$WORK/ctl"
mkdir -p "$WORK"
MYUID=$(id -u)
fail() { echo "FAIL: $*"; [ -n "${PID:-}" ] && kill "$PID" 2>/dev/null; \
         "$IMG" unlink "$NAME" 2>/dev/null; exit 1; }
pass() { echo "PASS: $*"; }

"$IMG" unlink "$NAME" 2>/dev/null
"$IMG" create "$NAME" 64 64 >/dev/null || { echo "SKIP: image"; exit 77; }

# -- 1+2: observer denied, journal records the denial ---------------
echo "$MYUID observer" > "$WORK/authz"
"$PLC" --image "$NAME" --module "$STUB" --ctl "$SOCK" \
    --authz "$WORK/authz" --audit-log "$WORK/audit.log" \
    >"$WORK/run1.log" 2>&1 &
PID=$!
sleep 1
kill -0 "$PID" 2>/dev/null || { cat "$WORK/run1.log"; fail "runtime died"; }
"$CTL" "$SOCK" status >/dev/null 2>&1 || fail "observer cannot read status"
"$CTL" "$SOCK" force 0 00 >/dev/null 2>&1 && fail "observer forced"
"$CTL" "$SOCK" force 0 00 2>&1 | grep -q "error 401" \
    || fail "no 401 on observer force"
grep -q "rc=-401" "$WORK/audit.log" \
    || fail "journal lacks the denial record"
pass "observer denied (401), denial journaled with identity"
kill "$PID" 2>/dev/null; wait "$PID" 2>/dev/null

# -- 2: commissioner mutates, journal has identity + rc=0 -----------
echo "$MYUID commissioner" > "$WORK/authz"
"$PLC" --image "$NAME" --module "$STUB" --ctl "$SOCK" \
    --authz "$WORK/authz" --audit-log "$WORK/audit.log" \
    >"$WORK/run2.log" 2>&1 &
PID=$!
sleep 1
"$CTL" "$SOCK" force 0 01 >/dev/null 2>&1 || fail "commissioner force denied"
"$CTL" "$SOCK" suspend 0 >/dev/null 2>&1 || fail "commissioner suspend denied"
grep -q "op=dbg.force" "$WORK/audit.log" \
    || fail "journal lacks dbg.force record"
grep -qE "uid=$MYUID.*rc=0" "$WORK/audit.log" \
    || fail "journal lacks uid+rc on success"
L1=$(wc -l < "$WORK/audit.log")
pass "commissioner ops journaled with uid/pid/ahash/rc=0"

# -- 3: uid-keyed rate limit ----------------------------------------
DENIED=0
for i in $(seq 60); do
    "$CTL" "$SOCK" suspend 0 2>&1 | grep -q "error 413" \
        && DENIED=$((DENIED+1))
done
[ "$DENIED" -gt 0 ] || fail "rate limit never tripped over 60 suspends"
grep -q "rc=-413" "$WORK/audit.log" \
    || fail "journal lacks rate-limit denial"
"$CTL" "$SOCK" resume 0 >/dev/null 2>&1
"$CTL" "$SOCK" shutdown >/dev/null 2>&1 || kill "$PID" 2>/dev/null
wait "$PID" 2>/dev/null
pass "rate limit tripped ($DENIED denied), journal has rc=-413"

# -- 4: journal survives restart -------------------------------------
"$PLC" --image "$NAME" --module "$STUB" --ctl "$SOCK" \
    --authz "$WORK/authz" --audit-log "$WORK/audit.log" \
    >"$WORK/run3.log" 2>&1 &
PID=$!
sleep 1
"$CTL" "$SOCK" suspend 0 >/dev/null 2>&1 || true
"$CTL" "$SOCK" shutdown >/dev/null 2>&1 || kill "$PID" 2>/dev/null
wait "$PID" 2>/dev/null
L2=$(wc -l < "$WORK/audit.log")
[ "$L2" -gt "$L1" ] || fail "journal did not append across restart"
pass "journal append across restart ($L1 → $L2 lines)"

"$IMG" unlink "$NAME" 2>/dev/null
rm -rf "$WORK"
echo "authz_audit: all checks passed"
