#!/usr/bin/env python3
"""fake_iolink_device.py — in-process IO-Link device for tplc-iolink e2e.

Speaks the device side of the IO-Link data-link protocol (IEC 61131-9)
over a TCP byte stream: parses [MC][CKT][body], answers with
[data...][CKS].  DPP1 page record advertises 4-byte process data in
both directions; ISDU params live in a small table; cyclic frames get
a deterministic PD-in pattern.  Wakeup bytes (0x55) are skipped.
"""
import socket
import sys

PDOUT_LEN = 4
PDIN_LEN = 4

# DPP1 identification record served on the page channel (16 octets).
DPP1 = bytes([
    0x00,             # [0] reserved
    0x00, 0x02,       # [1..2] minimum cycle time
    0x03,             # [3] M-sequence capability
    0x01,             # [4] revision
    0x84,             # [5] PD-in descriptor: 4 octets
    0x84,             # [6] PD-out descriptor: 4 octets
    0x34, 0x12,       # [7..8] vendor ID 0x1234 (LE)
    0x78, 0x56,       # [9..10] device ID 0x5678 (LE)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
])

PARAMS = [0] * 16


def cksum6(data):
    """A.1.6 checksum: XOR seed 0x52, fold 8->6 bits."""
    ck = 0x52
    for b in data:
        ck ^= b
    b = lambda i: (ck >> i) & 1
    r = ((b(7) ^ b(5) ^ b(3) ^ b(1)) << 5) | \
        ((b(6) ^ b(4) ^ b(2) ^ b(0)) << 4) | \
        ((b(7) ^ b(6)) << 3) | \
        ((b(5) ^ b(4)) << 2) | \
        ((b(3) ^ b(2)) << 1) | \
        (b(1) ^ b(0))
    return r & 0x3F


def cks_frame(data, flags=0):
    """Append CKS: checksum over data + CKS-position byte holding only
    the flag bits (bits 0-5 zeroed)."""
    ck = cksum6(bytes(data) + bytes([flags & 0xC0]))
    return bytes(data) + bytes([(flags & 0xC0) | ck])


def recvn(conn, n):
    buf = b""
    while len(buf) < n:
        chunk = conn.recv(n - len(buf))
        if not chunk:
            raise ConnectionError("eof")
        buf += chunk
    return buf


def handle(conn):
    global PARAMS
    pd_count = 0
    while True:
        # frame start: skip wakeup bytes
        mc = recvn(conn, 1)[0]
        while mc == 0x55:
            mc = recvn(conn, 1)[0]
        ckt = recvn(conn, 1)[0]
        channel = (mc >> 5) & 0x03
        is_read = bool(mc & 0x80)
        mseq = ckt >> 6

        if channel == 0x01:            # PAGE, type 0: no body
            body = b""
        elif channel == 0x03:          # ISDU, type 0
            body = b"" if is_read else recvn(conn, 1)
        elif channel == 0x00:          # PROCESS, type 2_V
            body = recvn(conn, PDOUT_LEN)
        else:                          # DIAGNOSIS: unsupported
            conn.sendall(cks_frame(b"\x00"))
            continue

        if channel == 0x01:
            addr = mc & 0x1F
            conn.sendall(cks_frame(bytes([DPP1[addr & 0x0F]])))
        elif channel == 0x03:
            addr = mc & 0x0F
            if is_read:
                conn.sendall(cks_frame(bytes([PARAMS[addr]])))
            else:
                PARAMS[addr] = body[0]
                conn.sendall(cks_frame(body))
        else:
            pd_count += 1
            pdi = bytes([
                0x42,
                (body[0] + 1) & 0xFF,
                0x99,
                pd_count & 0xFF,
            ])
            conn.sendall(cks_frame(pdi))


def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 14411
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", port))
    srv.listen(1)
    print(f"fake iolink device on 127.0.0.1:{port}", flush=True)
    while True:
        conn, _ = srv.accept()
        try:
            handle(conn)
        except (ConnectionError, OSError):
            pass
        finally:
            conn.close()


if __name__ == "__main__":
    main()
