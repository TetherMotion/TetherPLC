#!/usr/bin/env python3
"""mbus_slave_fake.py — PTY fake M-Bus slave for the tplc-mbus e2e.
Creates a PTY pair, prints the slave path, and answers real M-Bus
frames: SND_NKE -> E5 ack; REQ_UD2 -> a 68.. long data frame — a
real M-Bus byte stream on the master side."""
import os, sys, pty, time, struct

def frame_long(c, a, ci, data):
    L = 3 + len(data)
    body = bytes([c, a, ci]) + data
    cs = sum(body) & 0xFF
    return bytes([0x68, L, L, 0x68]) + body + bytes([cs, 0x16])

def main():
    log = sys.argv[1] if len(sys.argv) > 1 else '/tmp/mbus_rx.log'
    master, slave = pty.openpty()
    path = os.ttyname(slave)
    with open(log, 'w') as f:
        f.write(f'pty:{path}\n')
        f.flush()
        buf = b''
        t_end = time.time() + 30
        while time.time() < t_end:
            try:
                c = os.read(master, 4096)
                if c:
                    buf += c
                    f.write(f'rx {len(buf)} {buf.hex()}\n')
                    f.flush()
                    # short frame: 10 C A CS 16
                    while len(buf) >= 5 and buf[0] == 0x10:
                        ctrl, addr = buf[1], buf[2]
                        f.write(f'short c={ctrl:02x} a={addr:02x}\n')
                        f.flush()
                        if ctrl == 0x40:          # SND_NKE -> E5
                            os.write(master, b'\xe5')
                        elif ctrl in (0x5b, 0x7b):# REQ_UD2/UD1 -> data
                            fr = frame_long(0x08, addr, 0x72,
                                        b'\x01\x02\x03\x04')
                            n = os.write(master, fr)
                            f.write(f'wrote {n}B {fr.hex()}\n')
                            f.flush()
                        buf = buf[5:]
                        break
                    # long frame: 68 L L 68 ...
                    while len(buf) >= 4 and buf[0] == 0x68:
                        L = buf[1]
                        if len(buf) < L + 6:
                            break
                        os.write(master, b'\xe5')
                        f.write(f'long L={L}\n'); f.flush()
                        buf = buf[L + 6:]
                        break
                    if len(buf) and buf[0] not in (0x10, 0x68):
                        buf = buf[1:]
            except OSError:
                break
            time.sleep(0.005)

if __name__ == '__main__':
    main()
