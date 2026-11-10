#!/usr/bin/env python3
"""sACN (ANSI E1.31) test peer for the tc3_sacn compat test.

Sends crafted data packets to the universe-1 multicast group on UDP
5568 on a wall-clock schedule:

  t=1.5s  HIPRI  priority 10, name 'HIPRI', dmx 33 44 55  (x3, seq++)
  t=3.0s  LOWPR  priority 200, name 'LOWPR', dmx 99       (once)
  then    HIPRI  priority 10 every 0.4s until killed

The receiver under test must accept HIPRI (priority 0 is most
important), drop LOWPR and the test's own priority-100 source once
HIPRI has been seen.
"""
import socket
import sys
import time

MCAST = "239.255.0.1"
PORT = 5568


def data_packet(cid, name, priority, sync_addr, seq, options,
                universe, start_code, dmx):
    total = 126 + len(dmx)
    p = bytearray()
    p += bytes([0x00, 0x10, 0x00, 0x00]) + b"ASC-E1.17" + bytes(3)
    p += bytes([0x70 | (((total - 16) >> 8) & 0x0F),
                (total - 16) & 0xFF])
    p += (4).to_bytes(4, "big")
    p += cid
    p += bytes([0x70 | (((total - 38) >> 8) & 0x0F),
                (total - 38) & 0xFF])
    p += (2).to_bytes(4, "big")
    p += name.encode()[:63].ljust(64, b"\0")
    p += bytes([priority])
    p += sync_addr.to_bytes(2, "big")
    p += bytes([seq & 0xFF])
    p += bytes([options])
    p += universe.to_bytes(2, "big")
    dmp_len = total - 115
    p += bytes([0x70 | ((dmp_len >> 8) & 0x0F), dmp_len & 0xFF])
    p += b"\x02"
    p += bytes([0xA1, 0x00, 0x00, 0x00, 0x01])
    p += (len(dmx) + 1).to_bytes(2, "big")
    p += bytes([start_code])
    p += dmx
    assert len(p) == total, (len(p), total)
    return bytes(p)


def main():
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    sock.bind(("", PORT))
    sock.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_LOOP, 1)
    sock.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_TTL, 8)
    # join the universe-1 group so we can see the DUT's own source
    # stream; HIPRI is only injected after SRC1 has been seen, which
    # keeps the receiver's phase-A (own-source) window deterministic
    mreq = socket.inet_aton(MCAST) + socket.inet_aton("0.0.0.0")
    sock.setsockopt(socket.IPPROTO_IP, socket.IP_ADD_MEMBERSHIP, mreq)
    sock.settimeout(10.0)
    dst = (MCAST, PORT)
    cid = bytes(range(16))

    def send(name, prio, seq, dmx):
        pkt = data_packet(cid, name, prio, 0, seq, 0, 1, 0, dmx)
        sock.sendto(pkt, dst)
        print(f"sent {name} prio={prio} seq={seq} "
              f"len={len(pkt)}", file=sys.stderr, flush=True)

    # wait for the DUT source ('SRC1') to appear on the wire
    seen = 0
    try:
        while seen < 4:
            data, _ = sock.recvfrom(2048)
            if b"SRC1" in data:
                seen += 1
    except socket.timeout:
        print("timeout waiting for SRC1", file=sys.stderr)
        return
    print(f"saw {seen} SRC1 packets", file=sys.stderr, flush=True)
    # let the receiver latch a few more own-source frames so the
    # program's phase-A check is guaranteed to run before preemption;
    # kept short — the sim's step count is the wall-clock budget
    time.sleep(0.3)
    for i in range(3):
        send("HIPRI", 10, 3 + i, bytes([0x33, 0x44, 0x55]))
        time.sleep(0.4)
    time.sleep(0.3)
    send("LOWPR", 200, 9, bytes([0x99]))
    seq = 10
    while True:
        time.sleep(0.4)
        send("HIPRI", 10, seq, bytes([0x33, 0x44, 0x55]))
        seq += 1


if __name__ == "__main__":
    main()
