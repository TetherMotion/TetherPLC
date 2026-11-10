#!/usr/bin/env python3
"""Minimal ADS/AMS-TCP test server for tc2_system_ads.sh.

Speaks the Beckhoff ADS wire format: 6-byte TCP header (u16 reserved,
u32 ams_len), 32-byte AMS header, ADS payload, all little-endian.
Answers READ/WRITE/READ_STATE/WRITE_CONTROL on 127.0.0.1:18765.
"""
import socket
import struct
import sys

HOST, PORT = "127.0.0.1", 18765
AMS_HDR = 32
TCP_HDR = 6

CMD_READ, CMD_WRITE, CMD_READ_STATE, CMD_WRITE_CTRL = 0x02, 0x03, 0x04, 0x05


def recvn(conn, n):
    buf = b""
    while len(buf) < n:
        chunk = conn.recv(n - len(buf))
        if not chunk:
            raise ConnectionError("eof")
        buf += chunk
    return buf


def handle(conn):
    try:
        while True:
            hdr = recvn(conn, TCP_HDR)
            reserved, ams_len = struct.unpack("<HI", hdr)
            ams = recvn(conn, ams_len)
            if len(ams) < AMS_HDR:
                return
            tgt_nid, tgt_port, src_nid, src_port = (
                ams[0:6], ams[6:8], ams[8:14], ams[14:16])
            cmd, state, dlen, err, invoke = struct.unpack(
                "<HHIII", ams[16:32])
            data = ams[32:32 + dlen]
            print(f"cmd={cmd:#06x} state={state:#06x} dlen={dlen} "
                  f"invoke={invoke} data={data[:16].hex()}", flush=True)

            if cmd == CMD_READ:
                grp, off, length = struct.unpack("<III", data[:12])
                payload = struct.pack("<II", 0, length)
                payload += bytes(0x41 + (i % 26) for i in range(length))
                # 'ABCDEFGH...' pattern
            elif cmd == CMD_WRITE:
                grp, off, length = struct.unpack("<III", data[:12])
                payload = struct.pack("<I", 0)
            elif cmd == CMD_READ_STATE:
                payload = struct.pack("<IHH", 0, 5, 0)  # RUN, dev 0
            elif cmd == CMD_WRITE_CTRL:
                payload = struct.pack("<I", 0)
            else:
                payload = struct.pack("<I", 0x701)  # unknown service

            resp_ams = src_nid + src_port + tgt_nid + tgt_port
            resp_ams += struct.pack("<HHIII", cmd, state | 0x0001,
                                    len(payload), 0, invoke)
            resp_ams += payload
            conn.sendall(struct.pack("<HI", 0, len(resp_ams)) + resp_ams)
    except (ConnectionError, OSError, struct.error):
        pass
    finally:
        conn.close()


def main():
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind((HOST, PORT))
    srv.listen(4)
    print(f"ads-test-server listening on {HOST}:{PORT}", flush=True)
    while True:
        conn, _ = srv.accept()
        handle(conn)


if __name__ == "__main__":
    main()
