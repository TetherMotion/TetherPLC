#!/usr/bin/env python3
"""mqtt_broker_fake.py — minimal MQTT 3.1.1 broker for the
tplc-mqtt e2e.  Answers CONNECT->CONNACK, SUBSCRIBE->SUBACK, and
echoes every PUBLISH back to the client (so a subscribe+publish
roundtrips the payload).  UNSUBSCRIBE->UNSUBACK, PINGREQ->PINGRESP.
Not a real broker — just enough for the client e2e."""
import socket, struct, sys, threading

def read_varint(s):
    mult, val = 1, 0
    for _ in range(4):
        b = s.recv(1)
        if not b: return None
        val += (b[0] & 0x7F) * mult
        if not (b[0] & 0x80): return val
        mult *= 128
    return val

def read_packet(s):
    h = s.recv(1)
    if not h: return None, None
    rl = read_varint(s)
    if rl is None: return None, None
    body = b''
    while len(body) < rl:
        c = s.recv(rl - len(body))
        if not c: return None, None
        body += c
    return h[0], body

def send(s, hdr, body):
    rl = len(body); enc = b''
    while True:
        d = rl % 128; rl //= 128
        enc += bytes([d | (0x80 if rl else 0)])
        if not rl: break
    s.sendall(bytes([hdr]) + enc + body)

def handle(conn):
    subs = set()
    try:
        while True:
            hdr, body = read_packet(conn)
            if hdr is None: break
            ptype = hdr >> 4
            if ptype == 1:      # CONNECT -> CONNACK (session 0, rc 0)
                send(conn, 0x20, b'\x00\x00')
            elif ptype == 8:    # SUBSCRIBE -> SUBACK
                mid = struct.unpack('>H', body[:2])[0]
                # topic filter at [2 + 2-byte len ..]
                tl = struct.unpack('>H', body[2:4])[0]
                topic = body[4:4+tl].decode()
                subs.add(topic)
                send(conn, 0x90, body[:2] + b'\x00')  # granted qos0
            elif ptype == 3:    # PUBLISH -> PUBACK (qos1) + echo to subs
                tl = struct.unpack('>H', body[:2])[0]
                topic = body[2:2+tl].decode()
                rest = body[2+tl:]
                # qos1 has a msgid; qos0 doesn't
                flags = hdr & 0x0F
                mid = None
                payload = rest
                if (flags >> 1) & 3:  # qos>0
                    mid = struct.unpack('>H', rest[:2])[0]
                    payload = rest[2:]
                if mid is not None:
                    send(conn, 0x40, struct.pack('>H', mid))
                # echo back a qos0 PUBLISH on each subscribed topic
                for t in subs:
                    send(conn, 0x30, struct.pack('>H', len(t)) + t.encode() + payload)
            elif ptype == 10:   # UNSUBSCRIBE -> UNSUBACK
                send(conn, 0xB0, body[:2])
            elif ptype == 12:   # PINGREQ -> PINGRESP
                send(conn, 0xD0, b'')
            elif ptype == 14:   # DISCONNECT
                break
    except Exception:
        pass
    conn.close()

def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 11883
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(('127.0.0.1', port)); s.listen(4)
    print(f'mqtt_broker_fake on {port}', flush=True)
    while True:
        c, _ = s.accept()
        threading.Thread(target=handle, args=(c,), daemon=True).start()

if __name__ == '__main__':
    main()
