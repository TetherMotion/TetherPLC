#!/usr/bin/env python3
"""modbus_test_server.py — minimal Modbus TCP/UDP server for the
tc2_modbussrv e2e. Accepts one TCP transaction at a time (the
client connects per transaction), answers functions
1/2/3/4/5/6/8/15/16/23 with deterministic data, and returns
exception 2 for reads at addr >= 99. UDP mode serves the same ADU
per datagram on TCP_PORT+1.
Usage: modbus_test_server.py PORT"""
import socket
import struct
import sys
import threading


def be16(b, o):
    return (b[o] << 8) | b[o + 1]


def reg_pattern(addr, qty):
    # deterministic register image: reg[i] = 0x1000 + addr + i
    out = bytearray()
    for i in range(qty):
        v = 0x1000 + ((addr + i) & 0x0FFF)
        out += bytes([(v >> 8) & 0xFF, v & 0xFF])
    return bytes(out)


def handle_pdu(pdu):
    func = pdu[0]
    d = pdu[1:]
    if func in (1, 2, 3, 4):
        addr, qty = be16(d, 0), be16(d, 2)
        if addr >= 99:
            return bytes([func | 0x80, 2])
        if func in (3, 4):
            data = reg_pattern(addr, qty)
            return bytes([func, len(data)]) + data
        if func == 2:
            nbytes = (qty + 7) // 8
            return bytes([func, nbytes]) + bytes([0x5A] * nbytes)
        nbytes = (qty + 7) // 8
        return bytes([func, nbytes]) + bytes([0xA5] * nbytes)
    if func in (5, 6):
        return bytes([func]) + d[:4]          # echo addr+value
    if func in (15, 16):
        addr, qty = be16(d, 0), be16(d, 2)
        return bytes([func]) + d[:4]          # echo addr+qty
    if func == 23:
        raddr, rqty = be16(d, 0), be16(d, 2)
        waddr, wqty = be16(d, 4), be16(d, 6)
        if raddr >= 99:
            return bytes([func | 0x80, 2])
        data = reg_pattern(raddr, rqty)
        return bytes([func, len(data)]) + data
    if func == 8:
        return bytes([func]) + d[:4]          # echo subfunc+data
    return bytes([func | 0x80, 1])            # illegal function


def serve(conn):
    try:
        while True:
            hdr = b""
            while len(hdr) < 7:
                chunk = conn.recv(7 - len(hdr))
                if not chunk:
                    return
                hdr += chunk
            mlen = be16(hdr, 4)
            body = b""
            while len(body) < mlen - 1:
                chunk = conn.recv(mlen - 1 - len(body))
                if not chunk:
                    return
                body += chunk
            tid, proto, unit = hdr[:2], hdr[2:4], hdr[6]
            resp_pdu = handle_pdu(body)
            adu = tid + proto + struct.pack(">H", len(resp_pdu) + 1) \
                + bytes([unit]) + resp_pdu
            conn.sendall(adu)
    except OSError:
        pass
    finally:
        conn.close()


def serve_udp(port):
    us = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    us.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    us.bind(("127.0.0.1", port))
    while True:
        adu, peer = us.recvfrom(2048)
        if len(adu) < 8:
            continue
        mlen = be16(adu, 4)
        body = adu[7:6 + mlen]
        tid, proto, unit = adu[:2], adu[2:4], adu[6]
        resp_pdu = handle_pdu(body)
        rsp = tid + proto + struct.pack(">H", len(resp_pdu) + 1) \
            + bytes([unit]) + resp_pdu
        us.sendto(rsp, peer)


def main():
    port = int(sys.argv[1])
    threading.Thread(target=serve_udp, args=(port + 1,),
                     daemon=True).start()
    ls = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    ls.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    ls.bind(("127.0.0.1", port))
    ls.listen(8)
    while True:
        c, _ = ls.accept()
        threading.Thread(target=serve, args=(c,), daemon=True).start()


if __name__ == "__main__":
    main()
