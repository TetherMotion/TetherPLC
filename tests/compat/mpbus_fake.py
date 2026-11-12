#!/usr/bin/env python3
"""mpbus_fake.py — PTY fake Belimo MP-Bus slave (node 0x05)
for the tplc-mpbus e2e.  Answers request frames
[DA][CMD][LEN][data][CHK8-xor]: CMD 0x01 -> ack with the
written position byte; CMD 0x02 -> reply with actual pos
(0x80); CMD 0x10 -> reply with measured value 0x42."""
import os, sys, pty, time

def chk8(b):
    c = 0
    for x in b: c ^= x
    return c

def frame(da, cmd, data=b''):
    f = bytes([da, cmd, len(data)]) + data
    return f + bytes([chk8(f)])

def main():
    unit = 0x05
    log = sys.argv[1] if len(sys.argv) > 1 else '/tmp/mpb_rx.log'
    master, slave = pty.openpty()
    path = os.ttyname(slave)
    buf = b''
    with open(log, 'w') as f:
        f.write(f'pty:{path}\n'); f.flush()
        t_end = time.time() + 60
        while time.time() < t_end:
            try:
                c = os.read(master, 4096)
                if c:
                    buf += c
                    f.write(f'rx {len(c)} {c.hex()}\n'); f.flush()
            except OSError:
                break
            i = 0
            while i + 4 <= len(buf):
                ln = buf[i + 2]
                if ln > 250: i += 1; continue
                need = i + 4 + ln
                if need > len(buf): break
                fr = buf[i:need]
                if chk8(fr[:3 + ln]) != fr[3 + ln]: i += 1; continue
                buf = buf[need:]
                da, cmd, data = fr[0], fr[1], fr[3:3 + ln]
                if da != unit: continue
                if cmd == 1:
                    r = frame(unit, cmd, data[:1] or b'\x00')
                elif cmd == 2:
                    r = frame(unit, cmd, b'\x80')
                elif cmd == 0x10:
                    r = frame(unit, cmd, b'\x42')
                else:
                    r = frame(unit, cmd, b'')
                os.write(master, r)
                f.write(f'tx {r.hex()}\n'); f.flush()
            time.sleep(0.005)

if __name__ == '__main__':
    main()
