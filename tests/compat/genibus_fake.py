#!/usr/bin/env python3
"""genibus_fake.py — PTY fake GENIbus slave for the
tplc-genibus e2e.  Creates a PTY pair, prints the slave path,
and answers GENIbus telegrams addressed to unit 0x20:
ping (empty APDU) -> empty-APDU ack; any APDU -> an APDU
response.  Real GENIbus framing with CRC16-CCITT."""
import os, sys, pty, time

def crc16(data):
    crc = 0xFFFF
    for b in data:
        crc ^= b << 8
        for _ in range(8):
            crc = ((crc << 1) ^ 0x1021) & 0xFFFF if (crc & 0x8000) else (crc << 1) & 0xFFFF
    return crc

def telegram(da, sa, apdu=b''):
    le = 2 + len(apdu)
    body = bytes([le, le, da, sa]) + apdu
    crc = crc16(body)
    return b'\x27' + body + bytes([crc >> 8, crc & 0xFF])

def main():
    unit = 0x20
    log = sys.argv[1] if len(sys.argv) > 1 else '/tmp/gen_rx.log'
    master, slave = pty.openpty()
    path = os.ttyname(slave)
    buf = b''
    with open(log, 'w') as f:
        f.write(f'pty:{path}\n')
        f.flush()
        t_end = time.time() + 60
        while time.time() < t_end:
            try:
                c = os.read(master, 4096)
                if c:
                    buf += c
                    f.write(f'rx {len(c)} {c.hex()}\n'); f.flush()
            except OSError:
                break
            # parse complete telegrams: 27 LE LEr [le] crc2
            while True:
                i = buf.find(b'\x27')
                if i < 0:
                    buf = b''; break
                buf = buf[i:]
                if len(buf) < 5: break
                le = buf[1]
                if buf[2] != le or le < 2 or le > 250:
                    buf = buf[1:]; continue
                need = 3 + le + 2
                if len(buf) < need: break
                body = buf[1:3 + le]
                crc = buf[3 + le] << 8 | buf[4 + le]
                if crc16(body) != crc:
                    buf = buf[1:]; continue
                da, sa = body[2], body[3]
                apdu = body[4:]
                buf = buf[need:]
                if da != unit:
                    continue
                # ack: empty APDU -> empty APDU; else echo data
                r = telegram(sa, unit, apdu if apdu else b'')
                os.write(master, r)
                f.write(f'tx {r.hex()}\n'); f.flush()
            time.sleep(0.005)

if __name__ == '__main__':
    main()
