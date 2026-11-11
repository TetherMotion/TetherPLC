#!/usr/bin/env python3
"""Minimal SMTP test peer for tc2_sms_sms e2e (RFC 5321 subset).

Listens on 127.0.0.1:2525. One client at a time is fine for the
test. EHLO advertises AUTH LOGIN; AUTH LOGIN accepts any
credentials (235). RCPT containing "reject" gets a 550 so the
consumer can exercise the fail path. DATA captures the dot-
unstuffed message and prints a one-line summary per message:
  MSG from=<..> to=<..> subj=<..> body=<first line>
"""
import socket
import threading

HOST, PORT = "127.0.0.1", 2525


def reply(w, code, text):
    w.write(f"{code} {text}\r\n".encode())
    w.flush()


def multiline(w, cap, lines):
    for i, ln in enumerate(lines):
        tag = "-" if i < len(lines) - 1 else " "
        w.write(f"{cap}{tag}{ln}\r\n".encode())
    w.flush()


def handle(c):
    f = c.makefile("rb")
    w = c.makefile("wb")
    try:
        reply(w, 220, "tplc-test ESMTP")
        mail_from = rcpt_to = subject = ""
        in_data = False
        data_lines = []
        auth_step = 0   # 0=none 1=want b64 user 2=want b64 pass
        while True:
            line = f.readline()
            if not line:
                break
            s = line.decode("latin1").rstrip("\r\n")
            u = s.upper()
            if auth_step == 1:
                auth_step = 2
                reply(w, 334, "UGFzc3dvcmQ6")  # "Password:"
                continue
            if auth_step == 2:
                auth_step = 0
                reply(w, 235, "authenticated")
                continue
            if in_data:
                if s == ".":
                    in_data = False
                    # find headers
                    hdr = {}
                    body = []
                    seen_blank = False
                    for dl in data_lines:
                        if not seen_blank and ":" in dl:
                            k, _, v = dl.partition(":")
                            hdr[k.strip().lower()] = v.strip()
                        elif not seen_blank and dl == "":
                            seen_blank = True
                        else:
                            body.append(dl)
                    mail_from = hdr.get("from", mail_from)
                    rcpt_to = hdr.get("to", rcpt_to)
                    subject = hdr.get("subject", subject)
                    first = body[0] if body else ""
                    print(f"MSG from={mail_from} to={rcpt_to} "
                          f"subj={subject} body={first}", flush=True)
                    reply(w, 250, "queued")
                    data_lines = []
                elif s.startswith(".."):
                    data_lines.append(s[1:])
                else:
                    data_lines.append(s)
                continue
            if u.startswith("EHLO") or u.startswith("HELO"):
                multiline(w, 250, [
                    "tplc-test", "SIZE 1024", "AUTH LOGIN",
                ])
            elif u == "AUTH LOGIN":
                auth_step = 1
                reply(w, 334, "VXNlcm5hbWU6")  # "Username:"
            elif u.startswith("AUTH"):
                reply(w, 235, "authenticated")
            elif u.startswith("MAIL FROM:"):
                mail_from = s.split(":", 1)[1].strip()
                reply(w, 250, "ok")
            elif u.startswith("RCPT TO:"):
                rcpt_to = s.split(":", 1)[1].strip()
                if "reject" in rcpt_to.lower():
                    reply(w, 550, "no such user")
                else:
                    reply(w, 250, "ok")
            elif u == "DATA":
                reply(w, 354, "end with .")
                in_data = True
                data_lines = []
            elif u == "RSET":
                mail_from = rcpt_to = subject = ""
                reply(w, 250, "ok")
            elif u == "NOOP":
                reply(w, 250, "ok")
            elif u == "QUIT":
                reply(w, 221, "bye")
                break
            else:
                reply(w, 250, "ok")
    except (BrokenPipeError, ConnectionResetError):
        pass
    finally:
        c.close()


def main():
    s = socket.socket()
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind((HOST, PORT))
    s.listen(4)
    print(f"smtp peer on {PORT}", flush=True)
    while True:
        c, _ = s.accept()
        threading.Thread(target=handle, args=(c,), daemon=True).start()


if __name__ == "__main__":
    main()
