#!/usr/bin/env python3
"""s7_test_peer.py — minimal Siemens S7comm (RFC1006) server peer.

Listens on TCP 1102 and speaks TPKT + COTP + S7comm:
  - COTP CR -> CC, then S7 Setup Communication job -> ACK_DATA
  - Read Var (0x04) / Write Var (0x05) jobs -> ACK_DATA
  - DB1 (256B pattern), MK area index 0 (128B pattern) are the
    registered memory areas; unknown areas get item rc 0x0A.
Bit access follows the S7 convention: a bit read returns one
byte with the value right-justified (bit 0); a bit write merges
data bit 0 into the addressed bit.
"""
import socket
import struct
import sys
import threading

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 1102
AREAS = {
    0x84: {1: bytearray((i * 7 + 3) & 0xFF for i in range(256))},
    0x83: {0: bytearray(0xA0 + (i & 0xF) for i in range(128))},
}


def recvn(conn, n):
    buf = b""
    while len(buf) < n:
        chunk = conn.recv(n - len(buf))
        if not chunk:
            raise ConnectionError("eof")
        buf += chunk
    return buf


def get16(b, o):
    return (b[o] << 8) | b[o + 1]


def put16(v):
    return struct.pack(">H", v)


def tpkt(payload):
    return b"\x03\x00" + put16(len(payload) + 4) + payload


def ack_data(ref, param, data, err=(0, 0)):
    return (
        b"\x32\x03\x00\x00" + put16(ref) + put16(len(param)) +
        put16(len(data)) + bytes(err) + param + data
    )


def cotp_cc():
    # LI=6, CC, dst-ref=1, src-ref=1, class 0 (7-byte COTP PDU)
    return tpkt(b"\x06\xd0\x00\x01\x00\x01\x00")


def parse_items(param, data):
    """Return (func, [(ts,count,db,area,bitaddr)], data_section).
    param = func nitems + per-item 12-byte S7-ANY specs
    `12 0A 10 ts count16 db16 area addr24`."""
    func = param[0]
    n = param[1]
    items = []
    q = param[2:]
    for _ in range(n):
        # spec: 12 0A 10 ts count16 db16 area addr24
        ts = q[3]
        count = get16(q, 4)
        db = get16(q, 6)
        area = q[8]
        addr = (q[9] << 16) | (q[10] << 8) | q[11]
        items.append((ts, count, db, area, addr))
        q = q[12:]
    return func, items, data


def handle_s7(conn, pdu):
    ros = pdu[1]
    ref = get16(pdu, 4)
    plen = get16(pdu, 6)
    dlen = get16(pdu, 8)
    param = pdu[10:10 + plen]
    data = pdu[10 + plen:10 + plen + dlen]
    if ros != 0x01:
        conn.sendall(tpkt(b"\x02\xf0\x80" + ack_data(ref, b"", b"", (0x81, 1))))
        return
    func = param[0]
    if func == 0xF0:  # setup communication
        conn.sendall(tpkt(b"\x02\xf0\x80" + ack_data(ref, param, b"")))
        return
    if func == 0x04:  # read var
        _, items, _ = parse_items(param, data)
        ritems = []
        for ts, count, db, area, addr in items:
            mem = AREAS.get(area, {}).get(db)
            if mem is None:
                ritems.append(bytes([0x0A, 0x00, 0x00, 0x00]))
                continue
            if ts == 0x01:  # BIT: addr = byte*8 + bit
                byte = addr >> 3
                bit = addr & 7
                if byte >= len(mem):
                    ritems.append(bytes([0x05, 0x00, 0x00, 0x00]))
                    continue
                val = (mem[byte] >> bit) & 1
                ritems.append(bytes([0xFF, 0x03, 0x00, 0x01, val]))
            else:  # BYTE family: addr = byte*8
                byte = addr >> 3
                if byte + count > len(mem):
                    ritems.append(bytes([0x05, 0x00, 0x00, 0x00]))
                    continue
                payload = bytes(mem[byte:byte + count])
                ritems.append(
                    bytes([0xFF, 0x04]) + put16(len(payload) * 8) + payload
                )
        out = b"".join(ritems)
        conn.sendall(tpkt(b"\x02\xf0\x80" + ack_data(ref, b"\x04\x01", out)))
        return
    if func == 0x05:  # write var
        _, items, _ = parse_items(param, data)
        # data section: 00 ts len16 payload
        dts = data[1]
        dlen16 = get16(data, 2)
        payload = data[4:]
        rcs = []
        for ts, count, db, area, addr in items:
            mem = AREAS.get(area, {}).get(db)
            if mem is None:
                rcs.append(0x0A)
                continue
            if ts == 0x01:
                byte = addr >> 3
                bit = addr & 7
                if byte >= len(mem):
                    rcs.append(0x05)
                    continue
                v = payload[0] & 1
                mem[byte] = (mem[byte] & ~(1 << bit)) | (v << bit)
            else:
                byte = addr >> 3
                if byte + count > len(mem):
                    rcs.append(0x05)
                    continue
                mem[byte:byte + count] = payload[:count]
            rcs.append(0xFF)
        out = bytes(rcs)
        conn.sendall(tpkt(b"\x02\xf0\x80" + ack_data(ref, b"\x05\x01", out)))
        return
    conn.sendall(tpkt(b"\x02\xf0\x80" + ack_data(ref, b"", b"", (0x81, 1))))


def handle_conn(conn):
    try:
        conn.settimeout(10)
        while True:
            hdr = recvn(conn, 4)
            if hdr[0] != 0x03:
                return
            total = get16(hdr, 2)
            rest = recvn(conn, total - 4)
            payload = rest  # COTP PDU after the 4-byte TPKT header
            if len(payload) < 2:
                return
            if payload[1] == 0xE0:  # COTP CR
                conn.sendall(cotp_cc())
                continue
            if payload[0] == 0x02 and payload[1] == 0xF0:  # COTP DT
                s7 = payload[3:]
                if len(s7) >= 10:
                    handle_s7(conn, s7)
                continue
            return
    except (ConnectionError, socket.timeout, OSError, AssertionError):
        pass
    finally:
        conn.close()


def main():
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", PORT))
    srv.listen(4)
    print(f"s7 peer on {PORT}", flush=True)
    while True:
        c, _ = srv.accept()
        threading.Thread(target=handle_conn, args=(c,), daemon=True).start()


if __name__ == "__main__":
    main()
