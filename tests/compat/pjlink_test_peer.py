#!/usr/bin/env python3
"""PJLink test peer for tc3_pjlink — TCP 4352.

Connection-count driven greeting so one listener covers every
phase of the consumer program:
  conn 1-3 : `PJLINK 0\\r`   (no auth)
  conn 4   : `PJLINK 1 a1b2c3d4\\r`, accepts
             md5("a1b2c3d4"+"secret") digest
  conn 5   : `PJLINK 1 deadbeef\\r`, rejects every digest with
             `PJLINK ERRA\\r`
  conn 6+  : `PJLINK 0\\r`, answers `ERR1` to unknown commands

Commands are `%<cls><CMD4> <param>\\r` (optionally prefixed by a
32-char lowercase md5 hex digest); responses are
`%<cls><CMD4>=<value>\\r`.
"""
import hashlib
import socket
import sys
import threading

PORT = 4352
NONCE_OK = "a1b2c3d4"
PASSWORD = "secret"

# command -> query response value (set commands get "OK")
QUERIES = {
    b"POWR": b"1",     # power on
    b"INPT": b"31",    # input 31
    b"AVMT": b"31",    # AV mute on
    b"ERST": b"000000",
}
SETS_OK = {b"POWR", b"INPT", b"AVMT"}

state = {"conn": 0}
lock = threading.Lock()


def greeting_for(n):
    if n == 4:
        return b"PJLINK 1 " + NONCE_OK.encode() + b"\r"
    if n == 5:
        return b"PJLINK 1 deadbeef\r"
    return b"PJLINK 0\r"


def handle(conn, n):
    try:
        conn.settimeout(5.0)
        conn.sendall(greeting_for(n))
        buf = b""
        while True:
            try:
                chunk = conn.recv(512)
            except socket.timeout:
                return
            if not chunk:
                return
            buf += chunk
            while b"\r" in buf:
                line, buf = buf.split(b"\r", 1)
                if not line:
                    continue
                # strip a possible auth digest prefix
                digest = None
                if len(line) >= 32 and all(
                    c in b"0123456789abcdef" for c in line[:32]
                ):
                    digest = line[:32]
                    line = line[32:]
                if not line.startswith(b"%"):
                    continue
                # %<cls><cmd4> <param>
                if len(line) < 7 or line[6:7] != b" ":
                    conn.sendall(b"%1ERRX=ERR1\r")
                    continue
                cls = line[1:2]
                cmd = line[2:6]
                param = line[7:]
                if n == 5:
                    # auth phase: every digest is wrong
                    conn.sendall(b"PJLINK ERRA\r")
                    return
                if n == 4:
                    want = hashlib.md5(
                        (NONCE_OK + PASSWORD).encode()
                    ).hexdigest().encode()
                    if digest != want:
                        conn.sendall(b"PJLINK ERRA\r")
                        return
                # strip auth digest effect: command proceeds
                if param == b"?":
                    val = QUERIES.get(cmd, b"ERR1")
                elif cmd in SETS_OK:
                    val = b"OK"
                else:
                    val = b"ERR1"
                conn.sendall(b"%" + cls + cmd + b"=" + val + b"\r")
    except OSError:
        pass
    finally:
        conn.close()


def main():
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", PORT))
    srv.listen(8)
    print(f"pjlink peer on {PORT}", flush=True)
    while True:
        conn, _ = srv.accept()
        with lock:
            state["conn"] += 1
            n = state["conn"]
        print(f"conn {n}", flush=True)
        threading.Thread(target=handle, args=(conn, n),
                         daemon=True).start()


if __name__ == "__main__":
    sys.exit(main())
