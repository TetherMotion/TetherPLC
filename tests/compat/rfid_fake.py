#!/usr/bin/env python3
"""rfid_fake.py — PTY fake RFID reader for the tc2_rfid e2e.
The reader protocol is byte-transparent at this layer, so the
fake answers any command with a tag record: "TAG:ABC12345\r\n".
Usage: rfid_fake.py <logfile>"""
import os, sys, pty, time, termios, tty

def main():
    log = sys.argv[1] if len(sys.argv) > 1 else '/tmp/rfid_rx.log'
    master, slave = pty.openpty()
    # reader links are raw: no echo of the PLC's command
    tty.setraw(slave)
    attrs = termios.tcgetattr(slave)
    attrs[3] &= ~(termios.ECHO | termios.ICANON)
    termios.tcsetattr(slave, termios.TCSANOW, attrs)
    path = os.ttyname(slave)
    with open(log, 'w') as f:
        f.write(f'pty:{path}\n'); f.flush()
        t_end = time.time() + 60
        while time.time() < t_end:
            try:
                c = os.read(master, 4096)
            except OSError:
                break
            if not c:
                time.sleep(0.01)
                continue
            f.write(f'rx {len(c)} {c.hex()}\n'); f.flush()
            os.write(master, b'TAG:ABC12345\r\n')
    return 0

if __name__ == '__main__':
    sys.exit(main())
