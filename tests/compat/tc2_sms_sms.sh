#!/usr/bin/env bash
# tc2_sms_sms.sh — Tc2_SMS + Tc2_SMTP compat (plan 21 C4):
# build compat/tc2/tc2_serialcom.st and compat/tc2/tc2_sms_sms.st
# as --library packages (the SMS library consumes the serial
# wrappers via --lib), consume both into
# tests/compat/tc2_sms_sms_case.st, and replay an autonomous
# sequence through tether-plc-sim:
#   pty serial open -> FB_SendSMS (AT state machine vs a pty
#   GSM modem peer) -> FB_SmtpV3 plain send -> FB_SmtpV3_Full
#   AUTH LOGIN send -> RCPT reject (550) -> FB_SmtpAttach
#   unsupported -> version -> refused connect -> close.
# tests/compat/smtp_test_peer.py serves RFC 5321 on TCP 2525;
# tests/compat/gsm_modem_peer.py answers AT commands on a pty.
# The program packs 9 checks into %QD0; the last step expects
# 0x1ff.
# Runs without --scan: the PLC free-runs at its task period and
# each step samples the next committed scan.
set -u
cd "$(dirname "$0")/../.."

TPLCC=${TPLCC:-./compiler/target/debug/tplcc}
BUILD=${BUILD:-./build}
pass=0; fail=0
ok()  { echo "PASS $1"; pass=$((pass+1)); }
bad() { echo "FAIL $1"; fail=$((fail+1)); }

WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT

[ -x "$TPLCC" ] && [ -x "$BUILD/tether-plc" ] \
    && [ -x "$BUILD/tether-plc-sim" ] && command -v python3 >/dev/null \
    || { echo "SKIP: no tplcc/build tree or python3"; exit 77; }

# GSM modem on a pty pair; it writes the slave path for us.
python3 tests/compat/gsm_modem_peer.py "$WORK/pty_slave" \
    >"$WORK/modem.log" 2>&1 &
MODEM=$!
for i in $(seq 1 50); do [ -s "$WORK/pty_slave" ] && break; sleep 0.05; done
SLAVE=$(cat "$WORK/pty_slave" 2>/dev/null)
[ -n "$SLAVE" ] || { bad "gsm modem pty"; echo "== $pass pass, $fail fail"; exit 1; }

python3 tests/compat/smtp_test_peer.py >"$WORK/smtp.log" 2>&1 &
SMTP=$!
sleep 0.5
trap 'kill $MODEM $SMTP 2>/dev/null; rm -rf "$WORK"' EXIT

"$TPLCC" build --library --name Tc2_SerialCom \
    compat/tc2/tc2_serialcom.st -o "$WORK/tc2_serialcom.tplc" >/dev/null 2>&1 \
    && ok "Tc2_SerialCom library" || bad "Tc2_SerialCom library"

"$TPLCC" build --library --name Tc2_SmsSmtp \
    compat/tc2/tc2_sms_sms.st --lib "$WORK/tc2_serialcom.tplc" \
    -o "$WORK/tc2_sms_sms.tplc" >/dev/null 2>&1 \
    && ok "Tc2_SmsSmtp library" || bad "Tc2_SmsSmtp library"

sed "s|__PTY_SLAVE__|$SLAVE|" tests/compat/tc2_sms_sms_case.st \
    > "$WORK/tc2_sms_sms_case.st"

"$TPLCC" build "$WORK/tc2_sms_sms_case.st" \
    --lib "$WORK/tc2_serialcom.tplc" \
    --lib "$WORK/tc2_sms_sms.tplc" \
    -o "$WORK/tc2_sms_sms_case.tplc" >/dev/null 2>&1 \
    && ok "consumer build with --lib" \
    || { bad "consumer build with --lib"; \
         echo "== $pass pass, $fail fail"; exit 1; }

{
    for i in $(seq 0 699); do
        t=$((i * 10000000))
        if [ "$i" -eq 699 ]; then
            echo "step $i time=$t in 0=00 expect 0=ff010000"
        else
            echo "step $i time=$t in 0=00"
        fi
    done
} > "$WORK/sms.steps"

IMG=/tplc_tc2sms_$$; CTL="$WORK/sms.ctl"
"$BUILD/tether-plc-sim" --image "$IMG" --package "$WORK/tc2_sms_sms_case.tplc" \
    --steps "$WORK/sms.steps" --plc "$BUILD/tether-plc" \
    --ctl "$CTL" --settle-ms 800 >"$WORK/sim.log" 2>&1
rc=$?
if [ $rc -eq 0 ] && grep -q "all matched" "$WORK/sim.log"; then
    ok "sim replay: serial/sms/smtp-plain/smtp-auth/reject/attach/version/dead/close"
else
    bad "sim replay"
    tail -8 "$WORK/sim.log"
fi
# peer-side evidence: the modem logged the SMS, the SMTP peer
# logged both queued messages.
grep -q "SMS to=+4915123456 text=Hello TetherPLC" "$WORK/modem.log" \
    && ok "modem saw SMS" || bad "modem saw SMS"
grep -q "subj=Plain Send" "$WORK/smtp.log" \
    && grep -q "subj=Auth Send" "$WORK/smtp.log" \
    && ok "smtp peer saw both messages" || bad "smtp peer saw both messages"

echo "== $pass pass, $fail fail"
[ $fail -eq 0 ]
