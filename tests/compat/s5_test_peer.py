#!/usr/bin/env python3
"""s5_test_peer.py — minimal Siemens S5 RK512/3964 TCP peer.

Listens on TCP (default 1103) and speaks the TetherPLC RK512
subset: 3964 frames `STX <rk512 msg> <bcc> ETX` with BCC = XOR of
message bytes plus ETX.

RK512 command telegram (10-byte header):
  [0..1] telegram id (00 00), [2] order type ('E' FETCH / 'A'
  SEND), [3] area ('D' = DB), [4] DB number, [5] word offset,
  [6..7] data length in words (big-endian), [8..9] flags (0xFF).
SEND appends nwords*2 payload bytes; FETCH replies with a 4-byte
header (id echo, type echo, status) plus nwords*2 data bytes.
Status 0x00 = ok, 0x01 = unknown DB.
"""
import socket
import sys
import threading

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 1103
STX, ETX = 0x02, 0x03

DBS = {
    1: bytearray((i * 7 + 3) & 0xFF for i in range(256)),
    2: bytearray(0x40 + (i & 0x3F) for i in range(256)),
}


def wrap(msg):
    bcc = ETX
    for b in msg:
        bcc ^= b
    return bytes([STX]) + msg + bytes([bcc, ETX])


def recvn(conn, n):
    buf = b""
    while len(buf) < n:
        chunk = conn.recv(n - len(buf))
        if not chunk:
            raise ConnectionError("eof")
        buf += chunk
    return buf


def read_frame(conn, buf):
    """Return (msg, rest) for the first BCC-valid 3964 frame."""
    while True:
        # drop bytes until a STX head
        while buf and buf[0] != STX:
            buf = buf[1:]
        if not buf:
            more = conn.recv(4096)
            if not more:
                raise ConnectionError("eof")
            buf += more
            continue
        # find an ETX whose preceding byte is a valid BCC
        for i in range(2, len(buf)):
            if buf[i] != ETX:
                continue
            want = ETX
            for j in range(1, i - 1):
                want ^= buf[j]
            if buf[i - 1] == want:
                return buf[1:i - 1], buf[i + 1:]
        more = conn.recv(4096)
        if not more:
            raise ConnectionError("eof")
        buf += more


def handle(conn):
    buf = b""
    while True:
        try:
            msg, buf = read_frame(conn, buf)
        except ConnectionError:
            return
        if len(msg) < 10:
            continue
        if msg[0] != 0 or msg[1] != 0 or msg[3] != ord('D'):
            continue
        otype = msg[2]
        db, woff = msg[4], msg[5]
        nwords = (msg[6] << 8) | msg[7]
        nbytes = nwords * 2
        if otype == ord('E'):
            if db not in DBS:
                resp = bytes([0, 0, otype, 1])
            else:
                start = woff * 2
                data = bytes(DBS[db][start:start + nbytes])
                resp = bytes([0, 0, otype, 0]) + data
            conn.sendall(wrap(resp))
        elif otype == ord('A'):
            payload = msg[10:10 + nbytes]
            if db not in DBS:
                resp = bytes([0, 0, otype, 1])
            else:
                start = woff * 2
                need = start + nbytes
                if need > len(DBS[db]):
                    DBS[db].extend(b"\x00" * (need - len(DBS[db])))
                DBS[db][start:start + nbytes] = payload
                resp = bytes([0, 0, otype, 0])
            conn.sendall(wrap(resp))


def main():
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", PORT))
    srv.listen(4)
    print(f"s5 peer on {PORT}", flush=True)
    while True:
        conn, _ = srv.accept()
        threading.Thread(target=handle, args=(conn,), daemon=True).start()


if __name__ == "__main__":
    main()
