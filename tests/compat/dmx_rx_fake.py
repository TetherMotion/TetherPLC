#!/usr/bin/env python3
"""dmx_rx_fake.py — PTY fake DMX receiver for the tplc-dmx e2e.
Creates a PTY pair, prints the slave path, and reads the raw bytes
the service writes after the break+MAB — the start code and the
512 channel bytes are a real serial byte stream on the master side.
Writes 'seen:<n>:<bytes>' to a log file so the e2e can verify."""
import os, sys, pty, time

def main():
    log = sys.argv[1] if len(sys.argv) > 1 else '/tmp/dmx_rx.log'
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
            except OSError:
                break
            time.sleep(0.005)

if __name__ == '__main__':
    main()
