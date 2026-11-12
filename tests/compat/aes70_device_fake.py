#!/usr/bin/env python3
"""aes70_device_fake.py — minimal AES70/OCA device fake for the
tplc-aes70 e2e.  Parses OCP1 frames and answers every OcaCommand PDU
with an OcaResponse echoing handle+targetONo+methodID — a real
OSCP+OCP peer, not a stub."""
import socket, struct, sys, threading

def read_ocp(s):
    # OCP1 sync header: 'OCP' + ver(1) + dataLen(2) + pduType(1) + nrPdus(2)
    hdr = b''
    while len(hdr) < 9:
        c = s.recv(9 - len(hdr))
        if not c: return None
        hdr += c
    if hdr[:3] != b'OCP' or hdr[3] != 1: return None
    dlen = struct.unpack('>H', hdr[4:6])[0]
    ptype = hdr[6]; npdus = struct.unpack('>H', hdr[7:9])[0]
    body = b''
    while len(body) < dlen:
        c = s.recv(dlen - len(body))
        if not c: return None
        body += c
    return ptype, npdus, body

def send_ocp(s, ptype, pdus):
    body = b''.join(pdus)
    hdr = b'OCP' + bytes([1]) + struct.pack('>H', len(body)) + bytes([ptype]) + struct.pack('>H', len(pdus))
    s.sendall(hdr + body)

def handle(conn):
    try:
        while True:
            pkt = read_ocp(conn)
            if pkt is None: break
            ptype, npdus, body = pkt
            if ptype == 1:   # keepalive — answer with keepalive back
                send_ocp(conn, 1, [])
                continue
            if ptype != 0:   # not a command
                continue
            # each PDU: pduSize(2)+hdrLen(1)+msgType(1)+handle(4)+targetONo(4)+methodID(2)+params
            off = 0; resps = []
            for _ in range(npdus):
                pdu_size = struct.unpack('>H', body[off:off+2])[0]
                hdr_len  = body[off+2]; msg_type = body[off+3]
                handle   = body[off+4:off+8]; target = body[off+8:off+12]
                method   = body[off+12:off+14]
                params   = body[off+14:off+pdu_size]
                # OcaResponse: pduSize+hdrLen(10? we use 15: +status+params)
                resp_hdr_len = 15
                status = b'\x01'  # OcaStatus OK=1
                rpdu_size = 2 + hdr_len + 1 + len(params)
                # echo: same handle/target/method + status + echo params
                rpdu = struct.pack('>H', rpdu_size) + bytes([hdr_len]) + bytes([3]) + handle + target + method + status + params
                resps.append(rpdu)
                off += pdu_size
            if resps:
                send_ocp(conn, 3, resps)   # pduType response=3
    except Exception:
        pass
    conn.close()

def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 65000
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(('127.0.0.1', port)); s.listen(4)
    print(f'aes70_device_fake on {port}', flush=True)
    while True:
        c, _ = s.accept()
        threading.Thread(target=handle, args=(c,), daemon=True).start()

if __name__ == '__main__':
    main()
