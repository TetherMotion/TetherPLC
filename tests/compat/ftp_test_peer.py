#!/usr/bin/env python3
"""Minimal FTP test peer for tc3_ftp e2e (RFC 959, PASV only).

Serves a fixed in-memory filesystem on 127.0.0.1:2121:
  files: /data.txt (68B), /bin.dat (1024B pattern)
  dirs:  /, /pub, /pub/sub
Supports USER/PASS/SYST/TYPE/PWD/CWD/MKD/RMD/DELE/SIZE/PASV/
RETR/STOR/LIST/NLST/RNFR/RNTO/NOOP/QUIT. STOR stores the
received bytes; RNFR/RNTO rename files and dirs.
"""
import socket
import threading

FILES = {
    "/data.txt": b"Hello FTP World!\n" * 4,
    "/bin.dat": bytes(range(256)) * 4,
}
DIRS = {"/", "/pub", "/pub/sub"}
RNFR = [None]  # pending rename source per connection is local;
# use a global for simplicity (one client at a time in tests)


def norm(p):
    if not p:
        return "/"
    if not p.startswith("/"):
        p = "/" + p
    while "//" in p:
        p = p.replace("//", "/")
    if p != "/" and p.endswith("/"):
        p = p[:-1]
    return p or "/"


class Conn:
    def __init__(self, sock):
        self.s = sock
        self.f = sock.makefile("rb")
        self.w = sock.makefile("wb")
        self.cwd = "/"
        self.pending_data = None
        self.rnfr = None

    def send(self, code, msg):
        self.w.write(f"{code} {msg}\r\n".encode("latin1"))
        self.w.flush()

    def cmd(self):
        line = self.f.readline()
        if not line:
            return None, None
        parts = line.decode("latin1").rstrip("\r\n").split(" ", 1)
        return parts[0].upper(), (parts[1] if len(parts) > 1 else "")

    def path(self, p):
        p = norm(p)
        if p.startswith("/"):
            return p
        base = self.cwd if self.cwd != "/" else ""
        return norm(base + "/" + p)


def handle(sock, addr):
    c = Conn(sock)
    try:
        sock.settimeout(15)
        c.send(220, "tplc ftp test peer")
        while True:
            verb, arg = c.cmd()
            if verb is None:
                return
            if verb == "USER":
                c.send(331, "password required")
            elif verb == "PASS":
                c.send(230, "logged in")
            elif verb == "SYST":
                c.send(215, "UNIX Type: L8")
            elif verb == "TYPE":
                c.send(200, "type set")
            elif verb == "PWD":
                c.send(257, f'"{c.cwd}"')
            elif verb == "CWD":
                p = c.path(arg)
                if p in DIRS:
                    c.cwd = p
                    c.send(250, "cwd ok")
                else:
                    c.send(550, "no such directory")
            elif verb == "MKD":
                p = c.path(arg)
                if p in DIRS:
                    c.send(550, "exists")
                else:
                    DIRS.add(p)
                    c.send(257, f'"{p}" created')
            elif verb == "RMD":
                p = c.path(arg)
                if p in DIRS and p != "/":
                    DIRS.discard(p)
                    c.send(250, "removed")
                else:
                    c.send(550, "no such directory")
            elif verb == "DELE":
                p = c.path(arg)
                if p in FILES:
                    del FILES[p]
                    c.send(250, "deleted")
                else:
                    c.send(550, "no such file")
            elif verb == "RNFR":
                p = c.path(arg)
                if p in FILES or p in DIRS:
                    c.rnfr = p
                    c.send(350, "ready for RNTO")
                else:
                    c.rnfr = None
                    c.send(550, "no such file or directory")
            elif verb == "RNTO":
                if c.rnfr is None:
                    c.send(503, "RNFR first")
                    continue
                dst = c.path(arg)
                if c.rnfr in FILES:
                    FILES[dst] = FILES.pop(c.rnfr)
                elif c.rnfr in DIRS:
                    DIRS.discard(c.rnfr)
                    DIRS.add(dst)
                else:
                    c.send(550, "source vanished")
                    c.rnfr = None
                    continue
                c.rnfr = None
                c.send(250, "renamed")
            elif verb == "SIZE":
                p = c.path(arg)
                if p in FILES:
                    c.send(213, str(len(FILES[p])))
                else:
                    c.send(550, "no such file")
            elif verb == "PASV":
                ls = socket.socket()
                ls.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
                ls.bind(("127.0.0.1", 0))
                ls.listen(1)
                port = ls.getsockname()[1]
                c.send(
                    227,
                    f"Entering Passive Mode (127,0,0,1,{port // 256},{port % 256})",
                )
                dc, _ = ls.accept()
                ls.close()
                c.pending_data = dc
            elif verb in ("RETR", "STOR", "LIST", "NLST"):
                dc = c.pending_data
                c.pending_data = None
                if dc is None:
                    c.send(425, "no data connection")
                    continue
                p = c.path(arg) if arg else c.cwd
                if verb == "RETR":
                    if p not in FILES:
                        c.send(550, "no such file")
                        dc.close()
                        continue
                    c.send(150, "opening data")
                    dc.sendall(FILES[p])
                    dc.close()
                    c.send(226, "transfer complete")
                elif verb == "STOR":
                    c.send(150, "ready to receive")
                    data = bytearray()
                    while True:
                        chunk = dc.recv(4096)
                        if not chunk:
                            break
                        data += chunk
                    dc.close()
                    FILES[p] = bytes(data)
                    c.send(226, "stored")
                else:  # LIST / NLST
                    c.send(150, "ok")
                    target = p if p in DIRS else c.cwd
                    lines = []
                    for f in sorted(FILES):
                        if f.rsplit("/", 1)[0] == target or (
                            target == "/" and "/" not in f[1:]
                        ):
                            lines.append(
                                f"-rw-r--r-- 1 u g {len(FILES[f])} Jan 1 00:00 {f}\r\n"
                            )
                    for d in sorted(DIRS):
                        if d != target and d.rsplit("/", 1)[0] == target:
                            lines.append(
                                f"drwxr-xr-x 2 u g 0 Jan 1 00:00 {d}\r\n"
                            )
                    if not lines:
                        lines.append("total 0\r\n")
                    dc.sendall("".join(lines).encode("latin1"))
                    dc.close()
                    c.send(226, "done")
            elif verb == "NOOP":
                c.send(200, "ok")
            elif verb == "QUIT":
                c.send(221, "bye")
                return
            else:
                c.send(502, "not implemented")
    except (ConnectionError, OSError):
        pass
    finally:
        try:
            sock.close()
        except OSError:
            pass


def main():
    srv = socket.socket()
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", 2121))
    srv.listen(4)
    print("ftp_test_peer on 127.0.0.1:2121", flush=True)
    while True:
        s, a = srv.accept()
        threading.Thread(target=handle, args=(s, a), daemon=True).start()


if __name__ == "__main__":
    main()
