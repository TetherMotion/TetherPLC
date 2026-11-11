#!/usr/bin/env python3
"""iec870_test_peer.py — minimal IEC 60870-5-104 slave for e2e tests.

Listens on a TCP port (default 2480), accepts one connection, performs
the STARTDT handshake, then exchanges I-frames carrying ASDUs.

For each received ASDU it replies with a scripted response:
  - C_IC_NA_1 (type 100, interrogation) -> M_SP_NA_1 + M_ME_NA_1 burst
  - C_CS_NA_1 (type 103, clock sync)    -> C_CS_NA_1 response
  - anything else                        -> S-frame ack only

It also answers TESTFR act with TESTFR con and logs every frame to
stderr for debugging.

Usage: iec870_test_peer.py [port]
"""

import socket
import struct
import sys
import threading

# U-format control octets
U_STARTDT_ACT = 0x07
U_STARTDT_CON = 0x0B
U_STOPDT_ACT = 0x13
U_STOPDT_CON = 0x23
U_TESTFR_ACT = 0x43
U_TESTFR_CON = 0x83
S_FORMAT = 0x01

SEQ_MOD = 32768


def log(msg):
    sys.stderr.write(f"[iec870-peer] {msg}\n")
    sys.stderr.flush()


def build_u(func):
    return bytes([0x68, 0x04, func, 0x00, 0x00, 0x00])


def encode_seq(seq):
    v = (seq * 2) & 0xFFFF
    return bytes([v & 0xFF, (v >> 8) & 0xFF])


def decode_seq(b):
    return ((b[1] << 8) | (b[0] & 0xFE)) // 2


def build_s(recv_count):
    return bytes([0x68, 0x04, 0x01, 0x00]) + encode_seq(recv_count)


def build_i(send_count, recv_count, asdu):
    ctrl = encode_seq(send_count) + encode_seq(recv_count)
    body = ctrl + asdu
    return bytes([0x68, len(body)]) + body


def build_asdu(type_id, vsq, cot, ca, payload):
    """ASDU with 2-octet COT (low=cause, high=OA) and 2-octet CA."""
    hdr = bytes([type_id, vsq, cot & 0xFF, (cot >> 8) & 0xFF,
                 ca & 0xFF, (ca >> 8) & 0xFF])
    return hdr + payload


class Peer:
    def __init__(self, port):
        self.port = port
        self.send_count = 0   # next I-frame send seq (slave->master)
        self.recv_count = 0   # next expected I-frame send seq (master->slave)
        self.unconfirmed_rx = 0
        self.active = False
        self.stop = False

    def recv_exact(self, conn, n):
        buf = b""
        while len(buf) < n:
            chunk = conn.recv(n - len(buf))
            if not chunk:
                raise ConnectionError("peer closed")
            buf += chunk
        return buf

    def recv_frame(self, conn):
        """Read one APCI frame, return (fmt, control_bytes, asdu_or_none)."""
        start = self.recv_exact(conn, 1)
        if start != b"\x68":
            raise ValueError(f"bad start byte {start.hex()}")
        ln = self.recv_exact(conn, 1)[0]
        body = self.recv_exact(conn, ln)
        return body

    def send_frame(self, conn, data):
        conn.sendall(data)

    def handle_asdu(self, conn, asdu):
        """Parse ASDU header, dispatch, return list of response ASDUs."""
        if len(asdu) < 6:
            log(f"short ASDU {asdu.hex()}")
            return []
        type_id = asdu[0]
        vsq = asdu[1]
        cot = asdu[2] | (asdu[3] << 8)
        ca = asdu[4] | (asdu[5] << 8)
        payload = asdu[6:]
        log(f"ASDU type={type_id} vsq={vsq:02x} cot={cot:04x} "
            f"ca={ca:04x} payload={payload.hex()}")

        responses = []
        if type_id == 100:  # C_IC_NA_1 interrogation
            # respond with M_SP_NA_1 (type 1) + M_ME_NA_1 (type 9)
            # single-point: IOA(3) + SPI/QDS(1) = 4 bytes per object
            sp = bytes([0x01, 0x01, 0x03, 0x00,  # type,vsq,cot_lo,cot_hi
                        0x01, 0x00,             # CA=1
                        0x00, 0x00, 0x01,      # IOA=1
                        0x01])                 # SPI=1, QDS ok
            responses.append(sp)
            # measured value normalized: IOA(3) + int16 + QDS = 6 bytes
            me = bytes([0x09, 0x01, 0x03, 0x00,
                        0x01, 0x00,
                        0x00, 0x00, 0x02,      # IOA=2
                        0x34, 0x12,            # value = 0x1234 = 4660
                        0x00])                 # QDS ok
            responses.append(me)
        elif type_id == 103:  # C_CS_NA_1 clock sync
            # echo back C_CS_NA_1 with same CA
            cs = bytes([0x67, 0x01, cot & 0xFF, (cot >> 8) & 0xFF,
                        ca & 0xFF, (ca >> 8) & 0xFF]) + payload
            responses.append(cs)
        elif type_id == 45:  # C_SC_NA_1 single command
            # respond C_SC_NA_1 act-con (COT=7) echoing IOA+SCS
            sc = bytes([type_id, 0x01, 0x07, 0x00,
                        ca & 0xFF, (ca >> 8) & 0xFF]) + payload
            responses.append(sc)
        return responses

    def run(self, conn):
        log(f"connection from {conn.getpeername()}")
        conn.settimeout(30)
        try:
            while not self.stop:
                body = self.recv_frame(conn)
                if len(body) < 1:
                    continue
                cf1 = body[0]

                if (cf1 & 1) == 0:
                    # I format
                    if len(body) < 5:
                        log("short I-frame")
                        continue
                    sseq = decode_seq(body[0:2])
                    rseq = decode_seq(body[2:4])
                    asdu = body[4:]
                    log(f"I-frame sseq={sseq} rseq={rseq} "
                        f"asdu={asdu.hex()}")
                    if sseq != self.recv_count:
                        log(f"seq error: expected {self.recv_count}, "
                            f"got {sseq}")
                        break
                    self.recv_count = (self.recv_count + 1) % SEQ_MOD
                    self.unconfirmed_rx += 1
                    # acknowledge via S-frame if >= 8, else let the
                    # response I-frame carry the ack
                    responses = self.handle_asdu(conn, asdu)
                    for resp_asdu in responses:
                        frame = build_i(self.send_count, self.recv_count,
                                        resp_asdu)
                        self.send_frame(conn, frame)
                        self.send_count = (self.send_count + 1) % SEQ_MOD
                        self.unconfirmed_rx = 0
                    if self.unconfirmed_rx >= 8:
                        self.send_frame(conn, build_s(self.recv_count))
                        self.unconfirmed_rx = 0
                elif (cf1 & 3) == 3:
                    # U format
                    func = cf1
                    log(f"U-frame func={func:02x}")
                    if func == U_STARTDT_ACT:
                        self.send_frame(conn, build_u(U_STARTDT_CON))
                        self.active = True
                        log("STARTDT con sent, active")
                    elif func == U_STARTDT_CON:
                        self.active = True
                    elif func == U_TESTFR_ACT:
                        self.send_frame(conn, build_u(U_TESTFR_CON))
                    elif func == U_STOPDT_ACT:
                        self.send_frame(conn, build_u(U_STOPDT_CON))
                        self.active = False
                        log("STOPDT con sent")
                elif cf1 == S_FORMAT:
                    # S frame: ack our sends
                    rseq = decode_seq(body[2:4])
                    log(f"S-frame rseq={rseq}")
                else:
                    log(f"unknown frame cf1={cf1:02x} body={body.hex()}")
        except (ConnectionError, socket.timeout, OSError) as e:
            log(f"connection ended: {e}")
        finally:
            conn.close()
            log("closed")


def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 2480
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", port))
    srv.listen(1)
    log(f"listening on 127.0.0.1:{port}")
    conn, addr = srv.accept()
    peer = Peer(port)
    peer.run(conn)
    srv.close()


if __name__ == "__main__":
    main()
