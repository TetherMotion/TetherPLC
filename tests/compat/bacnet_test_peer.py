#!/usr/bin/env python3
"""bacnet_test_peer.py — minimal BACnet/IP (ASHRAE 135) server for
the tc3_bacnet e2e. Binds UDP 47808 and answers:
  - Who-Is (unconfirmed svc 8) with one I-Am for device
    analogInput(0).1 / analogOutput(1).1 / device(8).1234 when the
    requested instance range covers 1234.
  - ReadProperty (svc 12): Complex-ACK with a REAL present-value
    for analogInput.1, a characterString object-name for
    device.1234, an UNSIGNED for analogOutput.1, and a BACnet
    Error for instance 9999.
  - WriteProperty (svc 15): Simple-ACK.
BVLC 0x81 ucast 0x0A / bcast 0x0B, NPDU v1, APDU encodings mirror
runtime/host/src/BacnetService.cpp (verified against bacpypes3).
Usage: bacnet_test_peer.py [PORT]"""
import socket
import struct
import sys

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 47808
DEV_OID = (8 << 22) | 1234          # device objectIdentifier
AI_OID = (0 << 22) | 1              # analogInput.1
AO_OID = (1 << 22) | 1              # analogOutput.1
VENDOR = 260                        # test vendor id
MAX_APDU = 1476
SEG = 0                             # no segmentation
REAL_215 = struct.pack(">f", 21.5)
REAL_225 = struct.pack(">f", 22.5)
UNSIGNED_7 = bytes([7])             # minimal BE unsigned
OBJNAME = b"TETH-PLC"


def u32be(b):
    return int.from_bytes(b, "big")


def rd_tag(buf, i):
    """BACnet ASN.1 tag: returns (ctx, num, payload, next_i)."""
    o = buf[i]
    i += 1
    ctx = bool(o & 0x08)
    num = o >> 4
    if num == 0x0F:
        num = buf[i]
        i += 1
    l = o & 0x07
    if l in (6, 7):
        return ctx, num, b"", i
    if l == 5:
        l = buf[i]
        i += 1
        if l == 254:
            l = (buf[i] << 8) | buf[i + 1]
            i += 2
        elif l == 255:
            l = int.from_bytes(buf[i:i + 4], "big")
            i += 4
    d = buf[i:i + l]
    return ctx, num, d, i + l


def put_app(num, payload):
    """application-class tag, payload verbatim"""
    l = len(payload)
    if l < 5:
        return bytes([(num << 4) | l]) + payload
    if l <= 253:
        return bytes([(num << 4) | 5, l]) + payload
    return bytes([(num << 4) | 5, 254, l >> 8, l & 0xFF]) + payload


def put_ctx(num, payload):
    l = len(payload)
    if l < 5:
        return bytes([0x08 | (num << 4) | l]) + payload
    if l <= 253:
        return bytes([0x08 | (num << 4) | 5, l]) + payload
    return bytes([0x08 | (num << 4) | 5, 254, l >> 8, l & 0xFF]) + payload


def npdu(payload):
    return bytes([1, 0x00]) + payload          # ver 1, no expecting-reply


def bvll(func, payload):
    body = npdu(payload)
    return bytes([0x81, func]) + struct.pack(">H", 4 + len(body)) + body


def iam(iid_unused=None):
    p = put_app(12, DEV_OID.to_bytes(4, "big"))
    p += put_app(2, _min_be(MAX_APDU))
    p += put_app(2, _min_be(SEG))
    p += put_app(2, _min_be(VENDOR))
    return bvll(0x0A, bytes([0x10, 0x00]) + p)  # unconfirmed, svc I-Am


def _min_be(v):
    if v == 0:
        return b"\x00"
    out = bytearray()
    while v:
        out.insert(0, v & 0xFF)
        v >>= 8
    return bytes(out)


def parse_confirmed(pkt):
    """pkt starts at APDU. Returns (invoke_id, service, params)."""
    b0 = pkt[0]
    if b0 >> 4 != 0:
        return None
    iid = pkt[2]
    svc = pkt[3]
    params = []
    i = 4
    while i < len(pkt):
        ctx, num, d, i = rd_tag(pkt, i)
        params.append((ctx, num, d))
    return iid, svc, params


def complex_ack(iid, svc, oid, propid, value_app):
    p = put_ctx(0, oid.to_bytes(4, "big"))
    p += put_ctx(1, _min_be(propid))
    p += value_app                            # Any: application tag
    return bvll(0x0A, bytes([0x30, iid, svc]) + p)


def simple_ack(iid, svc):
    return bvll(0x0A, bytes([0x20, iid, svc]))


def error_ack(iid, svc):
    # Error: type 5, invoke, service, error-class(enum), error-code(enum)
    p = put_app(9, b"\x01")                    # class: device(1)
    p += put_app(9, b"\x1F")                   # code: unknownObject(31)
    return bvll(0x0A, bytes([0x50, iid, svc]) + p)


def handle(pkt, addr, sock):
    if len(pkt) < 6 or pkt[0] != 0x81:
        return
    func = pkt[1]
    blen = (pkt[2] << 8) | pkt[3]
    if blen < 4 or blen > len(pkt):
        return
    i = 4
    if i + 2 > len(pkt) or pkt[i] != 1:
        return
    ctrl = pkt[i + 1]
    i += 2
    if ctrl & 0x20:                            # DNET present
        dlen = pkt[i + 2]
        i += 3 + dlen + 1
    if ctrl & 0x08:                            # SNET present
        slen = pkt[i + 2]
        i += 3 + slen
    if i >= len(pkt):
        return
    b0 = pkt[i]
    atype = b0 >> 4
    body = pkt[i:]

    if atype == 1:                             # unconfirmed request
        svc = body[1] if len(body) > 1 else None
        if svc == 8:                           # Who-Is
            lo, hi = 0, 0xFFFFFFFF
            j = 2
            while j < len(body):
                ctx, num, d, j = rd_tag(body, j)
                if ctx and num == 0:
                    lo = u32be(d)
                elif ctx and num == 1:
                    hi = u32be(d)
            inst = DEV_OID & 0x3FFFFF
            if lo <= inst <= hi:
                sock.sendto(iam(), addr)
        return

    if atype != 0:                             # only confirmed reqs left
        return
    parsed = parse_confirmed(body)
    if not parsed:
        return
    iid, svc, params = parsed
    objid = propid = None
    aval = None
    for ctx, num, d in params:
        if ctx and num == 0 and len(d) == 4:
            objid = u32be(d)
        elif ctx and num == 1:
            propid = u32be(d)
        elif not ctx:
            aval = (num, d)
    if svc == 12:                              # ReadProperty
        if objid == (0 << 22) | 9999 or objid == 9999:
            sock.sendto(error_ack(iid, svc), addr)
        elif objid == AI_OID and propid == 85:
            sock.sendto(complex_ack(iid, svc, objid, propid,
                                    put_app(4, REAL_215)), addr)
        elif objid == AO_OID and propid == 85:
            sock.sendto(complex_ack(iid, svc, objid, propid,
                                    put_app(2, UNSIGNED_7)), addr)
        elif objid == DEV_OID and propid == 77:
            sock.sendto(complex_ack(iid, svc, objid, propid,
                                    put_app(7, OBJNAME)), addr)
        else:
            sock.sendto(error_ack(iid, svc), addr)
    elif svc == 15:                            # WriteProperty
        sock.sendto(simple_ack(iid, svc), addr)


def main():
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    try:
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
    except OSError:
        pass
    sock.bind(("0.0.0.0", PORT))
    sock.settimeout(0.2)
    print(f"bacnet peer on udp/{PORT}", flush=True)
    while True:
        try:
            pkt, addr = sock.recvfrom(4096)
        except socket.timeout:
            continue
        try:
            handle(pkt, addr, sock)
        except Exception as e:
            print(f"peer error: {e}", flush=True)


if __name__ == "__main__":
    main()
