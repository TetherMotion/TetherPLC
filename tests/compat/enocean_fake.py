#!/usr/bin/env python3
"""enocean_fake.py — PTY fake EnOcean ESP3 transceiver for the
tplc-enocean e2e.  Creates a PTY pair, prints the slave path,
and acts as a TCM: on any inbound byte it answers with an ESP3
RESPONSE packet, and ~200ms after start it emits one RADIO_ERP1
telegram (F6 RPS switch, sender 0x00112233, pressed action 2).
Real ESP3 framing with CRC8H/CRC8D."""
import os, sys, pty, time

def crc8(data):
    crc = 0
    for b in data:
        crc ^= b
        for _ in range(8):
            crc = ((crc << 1) ^ 0x07) & 0xFF if (crc & 0x80) else (crc << 1) & 0xFF
    return crc

def esp3(ptype, data, opt=b''):
    hdr = bytes([len(data) >> 8, len(data) & 0xFF, len(opt), ptype])
    return b'\x55' + hdr + bytes([crc8(hdr)]) + data + opt + bytes([crc8(data + opt)])

def radio_ptm(sender=0x00112233, action=2, dest=0xFFFFFFFF):
    # RORG F6, payload [action|energy-bow], sender 4B, status 0x00
    d = bytes([0xF6, action | 0x10])
    d += sender.to_bytes(4, 'big') + b'\x00'
    opt = dest.to_bytes(4, 'big') + b'\x30\x00'
    return esp3(1, d, opt)

def main():
    log = sys.argv[1] if len(sys.argv) > 1 else '/tmp/eno_rx.log'
    master, slave = pty.openpty()
    path = os.ttyname(slave)
    sent_radio = False
    t0 = time.time()
    with open(log, 'w') as f:
        f.write(f'pty:{path}\n')
        f.flush()
        t_end = t0 + 60
        while time.time() < t_end:
            try:
                c = os.read(master, 4096)
                if c:
                    f.write(f'rx {len(c)} {c.hex()}\n'); f.flush()
                    # answer with a RESPONSE packet (code 0 OK),
                    # then emit our inbound RADIO telegram once.
                    os.write(master, esp3(2, b'\x00'))
                    if not sent_radio:
                        fr = radio_ptm()
                        os.write(master, fr)
                        f.write(f'radio {fr.hex()}\n'); f.flush()
                        sent_radio = True
            except OSError:
                break
            time.sleep(0.005)

if __name__ == '__main__':
    main()
