#!/usr/bin/env python3
"""fake_profinet_device.py — in-process PROFINET device for
tplc-profinet e2e.  UDP transport carrying DCP PDUs (12-byte header
+ option blocks) plus the 'PN'-tagged cyclic record primitive.
"""
import socket
import sys

NAME = b"tplc-dev-1"
VENDOR = 0x002A
DEVICE = 0xBEEF
ROLE = 1            # IO Device
INSTANCE = 7
OEM = 0xCAFEBABE
ALIAS = b"AB"

FID_GETSET = 0xFEFD
FID_IDENT_REQ = 0xFEFE
FID_IDENT_RESP = 0xFEFF
SVC_GET, SVC_SET, SVC_IDENT = 0x03, 0x04, 0x05
OPT_IP, OPT_PROP = 0x01, 0x02
SUB_NAME, SUB_DEVID, SUB_ROLE, SUB_ALIAS, SUB_INST, SUB_OEM = \
    0x02, 0x03, 0x04, 0x06, 0x07, 0x08


def rd16(b, o):
    return (b[o] << 8) | b[o + 1]


def wr16(v):
    return bytes([(v >> 8) & 0xFF, v & 0xFF])


def wr32(v):
    return bytes([(v >> 24) & 0xFF, (v >> 16) & 0xFF,
                  (v >> 8) & 0xFF, v & 0xFF])


def block(opt, sub, info, data):
    """response block: opt,sub,len=2+dlen,info,data,pad-to-even"""
    blen = 2 + len(data)
    out = bytes([opt, sub]) + wr16(blen) + wr16(info) + data
    if blen & 1:
        out += b"\x00"
    return out


def dcp_response(fid, svc, xid, blocks):
    body = b"".join(blocks)
    return (wr16(fid) + bytes([svc, 0x01]) + wr32(xid) +
            wr16(0) + wr16(len(body)) + body)


def prop_blocks():
    return [
        block(OPT_PROP, SUB_NAME, 1, NAME),
        block(OPT_PROP, SUB_DEVID, 1,
              wr16(VENDOR) + wr16(DEVICE)),
        block(OPT_PROP, SUB_ROLE, 1, bytes([ROLE, 0])),
        block(OPT_PROP, SUB_INST, 1, wr16(INSTANCE)),
        block(OPT_PROP, SUB_OEM, 1, wr32(OEM)),
        block(OPT_PROP, SUB_ALIAS, 1, ALIAS),
    ]


def parse_blocks(body):
    out = []
    off = 0
    while off + 4 <= len(body):
        opt, sub = body[off], body[off + 1]
        blen = rd16(body, off + 2)
        if blen < 2 or off + 4 + blen > len(body):
            break
        info = rd16(body, off + 4)
        out.append((opt, sub, info, body[off + 6:off + 4 + blen]))
        off += 4 + blen + (blen & 1)
    return out


def parse_request_blocks(body):
    """request-side blocks: selector blocks carry blocklen=0 (no
    info/data); value blocks carry info+data like responses."""
    out = []
    off = 0
    while off + 4 <= len(body):
        opt, sub = body[off], body[off + 1]
        blen = rd16(body, off + 2)
        if blen == 0:
            out.append((opt, sub, 0, b""))
            off += 4
            continue
        if blen < 2 or off + 4 + blen > len(body):
            break
        info = rd16(body, off + 4)
        out.append((opt, sub, info, body[off + 6:off + 4 + blen]))
        off += 4 + blen + (blen & 1)
    return out


def get_data(opt, sub):
    if opt == OPT_PROP and sub == SUB_NAME:
        return NAME
    if opt == OPT_PROP and sub == SUB_ALIAS:
        return ALIAS
    if opt == OPT_PROP and sub == SUB_DEVID:
        return wr16(VENDOR) + wr16(DEVICE)
    return None


IOCR_CYCLE = [0]


def handle(data):
    # cyclic IO record primitive: 'PN' + len + payload -> transform
    if data[:2] == b"PN" and len(data) >= 3:
        n = data[2]
        payload = data[3:3 + n]
        reply = bytes([(b + 1) & 0xFF for b in payload])
        return b"PN" + bytes([len(reply)]) + reply
    # IOCR APDU: [frame_id u16][C-SDU >=40]['PN' rec][pad]
    #             [cycle u16][dataStatus][transferStatus]
    # DCP FIDs are 0xFExx; the test frame_id is 0x8000.
    if len(data) >= 46 and data[0] != 0xFE:
        fid = rd16(data, 0)
        csdu = data[2:-4]
        tr = data[-4:]
        if csdu[:2] == b"PN" and len(csdu) >= 40:
            n = csdu[2]
            payload = csdu[3:3 + n]
            reply = bytes([(b + 1) & 0xFF for b in payload])
            rcsdu = bytearray(40)
            rcsdu[0:2] = b"PN"
            rcsdu[2] = len(reply)
            rcsdu[3:3 + len(reply)] = reply
            IOCR_CYCLE[0] = (IOCR_CYCLE[0] + 1) & 0xFFFF
            return (fid.to_bytes(2, "big") + bytes(rcsdu) +
                    IOCR_CYCLE[0].to_bytes(2, "big") +
                    bytes([0x35, 0x00]))
        return None
    if len(data) < 12:
        return None
    fid = rd16(data, 0)
    svc, stype = data[2], data[3]
    xid = int.from_bytes(data[4:8], "big")
    if fid == FID_IDENT_REQ and svc == SVC_IDENT:
        return dcp_response(FID_IDENT_RESP, SVC_IDENT, xid,
                            prop_blocks())
    if fid == FID_GETSET and svc == SVC_GET:
        blocks = parse_request_blocks(data[12:12 + rd16(data, 10)])
        resp = []
        for opt, sub, _info, _d in blocks:
            val = get_data(opt, sub)
            resp.append(block(opt, sub, 1 if val is not None else 2,
                              val if val is not None else b""))
        return dcp_response(FID_GETSET, SVC_GET, xid, resp)
    if fid == FID_GETSET and svc == SVC_SET:
        blocks = parse_request_blocks(data[12:12 + rd16(data, 10)])
        global ALIAS
        for opt, sub, _info, d in blocks:
            if opt == OPT_PROP and sub == SUB_ALIAS:
                ALIAS = d
        resp = [block(opt, sub, 1, b"") for opt, sub, _i, _d in blocks]
        return dcp_response(FID_GETSET, SVC_SET, xid, resp)
    return None


def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 14511
    srv = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", port))
    print(f"fake profinet device on 127.0.0.1:{port}", flush=True)
    while True:
        data, peer = srv.recvfrom(2048)
        resp = handle(data)
        if resp:
            srv.sendto(resp, peer)


if __name__ == "__main__":
    main()
