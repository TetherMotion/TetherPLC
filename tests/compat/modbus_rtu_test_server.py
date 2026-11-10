#!/usr/bin/env python3
"""modbus_rtu_test_server.py — minimal Modbus RTU slave for the
tc2_modbusrtu e2e. Opens a pty pair, writes the slave path to the
file given as argv[1], then serves Modbus RTU on the master side:
unit|func|data|crc16 frames in, crc16-terminated responses out.
Answers functions 1/2/3/4/5/6/8/15/16/23 with deterministic data
(same PDU semantics as modbus_test_server.py) for unit id 1, and
returns exception 2 for reads at addr >= 99.
Usage: modbus_rtu_test_server.py SLAVE_PATH_FILE"""
import os
import pty
import select
import sys
import time


def be16(b, o):
    return (b[o] << 8) | b[o + 1]


def crc16(data):
    c = 0xFFFF
    for b in data:
        c ^= b
        for _ in range(8):
            c = (c >> 1) ^ 0xA001 if c & 1 else c >> 1
    return c


def reg_pattern(addr, qty):
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
        return bytes([func]) + d[:4]
    if func in (15, 16):
        return bytes([func]) + d[:4]
    if func == 23:
        raddr, rqty = be16(d, 0), be16(d, 2)
        if raddr >= 99:
            return bytes([func | 0x80, 2])
        data = reg_pattern(raddr, rqty)
        return bytes([func, len(data)]) + data
    if func == 8:
        return bytes([func]) + d[:4]
    return bytes([func | 0x80, 1])


def req_frame_len(buf):
    """Expected RTU request size once determinable; 0 = need more
    bytes, -1 = not a request we understand (gap framing)."""
    if len(buf) < 2:
        return 0
    func = buf[1]
    if func in (1, 2, 3, 4, 5, 6):
        return 8
    if func == 8:
        return 6
    if func in (15, 16):
        if len(buf) < 7:
            return 0
        return 9 + buf[6]
    if func == 23:
        if len(buf) < 11:
            return 0
        return 13 + buf[10]
    return -1


def main():
    m, s = pty.openpty()
    slave = os.ttyname(s)
    with open(sys.argv[1], "w") as f:
        f.write(slave)
    os.set_blocking(m, False)
    buf = bytearray()
    last_rx = time.monotonic()
    while True:
        r, _, _ = select.select([m], [], [], 0.05)
        if r:
            try:
                data = os.read(m, 512)
            except OSError:
                data = b""
            if data:
                buf += data
                last_rx = time.monotonic()
        # extract one complete request frame
        while len(buf) >= 4:
            need = req_frame_len(buf)
            done = False
            if need > 0:
                done = len(buf) >= need
            elif need < 0 and time.monotonic() - last_rx > 0.15:
                done = True  # unknown func: silence gap ends frame
            if not done:
                break
            frame = bytes(buf[:need] if need > 0 else buf)
            del buf[:len(frame)]
            if len(frame) < 4 or \
                    crc16(frame[:-2]) != (frame[-2] | frame[-1] << 8):
                continue  # bad crc: drop frame
            unit, func = frame[0], frame[1]
            if unit != 1:
                continue  # not our station address
            pdu = bytes([func]) + frame[2:-2]
            resp_pdu = handle_pdu(pdu)
            rsp = bytes([unit]) + resp_pdu
            c = crc16(rsp)
            rsp += bytes([c & 0xFF, c >> 8])
            try:
                os.write(m, rsp)
            except OSError:
                pass


if __name__ == "__main__":
    main()
