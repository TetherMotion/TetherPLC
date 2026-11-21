#!/usr/bin/env python3
"""Minimal TetherPLC ctl client for the MQTT journal ops (0x0F20/0x0F21).
Usage: ctl_mqtt_journal.py <ctl-socket> — prints parsed results."""
import socket, struct, sys, time

OP_HELLO = 0x0101
OP_MQTT_STATUS = 0x0F20
OP_MQTT_JOURNAL = 0x0F21

def frame(t, flags, req, payload):
    return struct.pack('<HHIQ', t, flags, len(payload), req) + payload

def read_exact(s, n):
    buf = b''
    while len(buf) < n:
        chunk = s.recv(n - len(buf))
        if not chunk:
            raise EOFError('ctl closed')
        buf += chunk
    return buf

def read_frame(s):
    hdr = read_exact(s, 16)
    t, fl, ln, req = struct.unpack('<HHIQ', hdr)
    return t, fl, req, read_exact(s, ln)

def rd_str(b, o):
    n = struct.unpack_from('<H', b, o)[0]; o += 2
    return b[o:o+n].decode('utf-8', 'replace'), o + n

def main():
    sock_path = sys.argv[1]
    wait_s = float(sys.argv[2]) if len(sys.argv) > 2 else 0.0
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    s.settimeout(10)
    s.connect(sock_path)
    # HELLO {u16 proto_ver, u16 pad, u32 caps} + str name
    name = b'ctl_mqtt_journal'
    hello = struct.pack('<HHI', 3, 0, 0) + struct.pack('<H', len(name)) + name
    s.sendall(frame(OP_HELLO, 0, 1, hello))
    t, fl, req, body = read_frame(s)
    assert t == 0x0102, f'hello ack expected, got {t:#x}'
    if wait_s:
        time.sleep(wait_s)

    s.sendall(frame(OP_MQTT_STATUS, 0, 2, b''))
    t, fl, req, body = read_frame(s)
    assert t == OP_MQTT_STATUS, f'status: type {t:#x} flags {fl:#x} {body[:64]}'
    n = struct.unpack_from('<I', body, 0)[0]
    o = 4
    conns = []
    for _ in range(n):
        h, st, jc, port, tr, tls, mv = struct.unpack_from('<7I', body, o); o += 28
        host, o = rd_str(body, o)
        cid, o = rd_str(body, o)
        conns.append(dict(handle=h, state=st, journal=jc, port=port,
                          transport=tr, tls=tls, mver=mv,
                          host=host, cid=cid))
    print('CONNS', conns)

    ok = True
    if not conns:
        print('FAIL: no mqtt connections')
        ok = False
    else:
        c = conns[0]
        if c['state'] != 2:
            print(f"FAIL: conn state {c['state']} != 2")
            ok = False
        h = c['handle']
        s.sendall(frame(OP_MQTT_JOURNAL, 0, 3,
                        struct.pack('<III', h, 0, 64)))
        t, fl, req, body = read_frame(s)
        assert t == OP_MQTT_JOURNAL, f'journal: type {t:#x} {body[:64]}'
        total = struct.unpack_from('<i', body, 0)[0]
        cnt = struct.unpack_from('<I', body, 4)[0]
        o = 8
        entries = []
        for _ in range(cnt):
            ts, qos, ret, mid = struct.unpack_from('<QIII', body, o); o += 20
            topic, o = rd_str(body, o)
            plen = struct.unpack_from('<I', body, o)[0]; o += 4
            payload = body[o:o+plen]; o += plen
            entries.append((ts, topic, payload, qos, ret, mid))
        print(f'JOURNAL total={total} n={cnt}')
        for e in entries[:8]:
            print('  ', e[1], repr(e[2][:40]), 'qos', e[3], 'ret', e[4])
        topics = {e[1] for e in entries}
        payloads = {e[2] for e in entries}
        if total < 3:
            print(f'FAIL: journal total {total} < 3')
            ok = False
        if not ({'jrn/a', 'jrn/b', 'jrn/c'} <= topics):
            print(f'FAIL: missing topics, have {topics}')
            ok = False
        if not ({b'MSG1', b'MSG2', b'MSG3'} & payloads):
            print('FAIL: no expected payloads')
            ok = False
    print('RESULT', 'PASS' if ok else 'FAIL')
    sys.exit(0 if ok else 1)

main()
