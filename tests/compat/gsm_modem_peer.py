#!/usr/bin/env python3
"""Minimal GSM modem test peer for tc2_sms_sms e2e.

Opens a POSIX pty pair, writes the slave path to argv[1] (the
PLC opens the slave via tplc-serial). Answers a small AT subset
on the master side:
  AT            -> OK
  AT+CMGF=1     -> OK   (text mode)
  AT+CMGS="n"   -> >    (send prompt)
  <text> SUB    -> +CMGS: <id> / OK, logs `SMS to=<n> text=<t>`
Anything else -> ERROR.
"""
import os
import pty
import re
import select
import sys

m, s = pty.openpty()
slave = os.ttyname(s)
with open(sys.argv[1], "w") as f:
    f.write(slave)
os.set_blocking(m, False)

buf = b""
mode = "cmd"          # cmd | body
pending_to = ""
msg_id = 0


def respond(data):
    print(f"ATTX {data!r}", flush=True)
    os.write(m, data)


while True:
    r, _, _ = select.select([m], [], [], 0.05)
    if not r:
        continue
    try:
        chunk = os.read(m, 4096)
    except OSError:
        continue
    if not chunk:
        break
    buf += chunk
    while True:
        if mode == "body":
            i = buf.find(b"\x1a")
            if i < 0:
                break
            body = buf[:i]
            buf = buf[i + 1:]
            mode = "cmd"
            msg_id += 1
            text = body.decode("latin1", "replace")
            print(f"SMSBODY {text!r}", flush=True)
            print(f"SMS to={pending_to} text={text}", flush=True)
            respond(b"\r\n+CMGS: %d\r\nOK\r\n" % msg_id)
            continue
        i = buf.find(b"\r")
        if i < 0:
            break
        cmd = buf[:i].strip()
        buf = buf[i + 1:]
        print(f"ATRX {cmd!r}", flush=True)
        if cmd == b"AT":
            respond(b"\r\nOK\r\n")
        elif cmd == b"AT+CMGF=1":
            respond(b"\r\nOK\r\n")
        elif cmd.startswith(b"AT+CMGS="):
            mm = re.search(rb'"([^"]*)"', cmd)
            pending_to = mm.group(1).decode("latin1") if mm else ""
            mode = "body"
            respond(b"\r\n> ")
        elif cmd == b"":
            pass
        else:
            respond(b"\r\nERROR\r\n")
