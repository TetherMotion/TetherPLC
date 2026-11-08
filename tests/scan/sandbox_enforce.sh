#!/usr/bin/env bash
# sandbox_enforce.sh — doc 15 §3 acceptance: the seccomp profile is
# active in the running runtime and forbids non-UDS sockets.
#
#   1. a valid module (stub_v1) runs under the sandbox and serves ctl
#   2. /proc/<pid>/status shows Seccomp: 2 (filter mode)
#   3. probe: socket(AF_INET) after installSandbox → SIGSYS death
#   4. probe: socket(AF_UNIX) → succeeds
#
# Usage: sandbox_enforce.sh <tether-plc> <tplc-image> <tplc-ctl>
#          <sandbox_probe> <stub_v1.so>
set -u

PLC="$1"; IMG="$2"; CTL="$3"; PROBE="$4"; GOOD="$5"
NAME="sbx_$$"
CTL_SOCK="/tmp/${NAME}.ctl"
PID=
fail() { echo "FAIL: $*"; [ -n "$PID" ] && kill "$PID" 2>/dev/null; "$IMG" unlink "$NAME" 2>/dev/null; exit 1; }
pass() { echo "PASS: $*"; }

"$IMG" unlink "$NAME" 2>/dev/null
"$IMG" create "$NAME" 64 64 >/dev/null || { echo "SKIP: image create"; exit 77; }

# -- positive control: good module boots under the sandbox ----------
"$PLC" --image "$NAME" --module "$GOOD" --ctl "$CTL_SOCK" \
    >/tmp/${NAME}.log 2>&1 &
PID=$!
sleep 1
kill -0 "$PID" 2>/dev/null || { cat /tmp/${NAME}.log; fail "good module died under sandbox"; }
grep -q "sandbox on" /tmp/${NAME}.log || {
    echo "SKIP: sandbox inactive (no libseccomp at build time)"
    kill "$PID" 2>/dev/null; "$IMG" unlink "$NAME"; exit 77; }
[ "$(awk '/^Seccomp:/{print $2}' /proc/$PID/status)" = "2" ] \
    || fail "proc status lacks Seccomp:2"
"$CTL" "$CTL_SOCK" ping >/dev/null 2>&1 || fail "ctl dead under sandbox"
pass "good module runs, Seccomp:2, ctl live"
"$CTL" "$CTL_SOCK" shutdown >/dev/null 2>&1
wait "$PID" 2>/dev/null

# -- filter enforcement ----------------------------------------------
"$PROBE" unix || fail "AF_UNIX socket denied (rc=$?)"
"$PROBE" inet
RC=$?
[ "$RC" = "159" ] || fail "AF_INET not SIGSYS-killed (rc=$RC)"
pass "AF_UNIX allowed, AF_INET killed by SIGSYS"

"$IMG" unlink "$NAME" 2>/dev/null
rm -f /tmp/${NAME}.log "$CTL_SOCK"
echo "sandbox_enforce: all checks passed"
