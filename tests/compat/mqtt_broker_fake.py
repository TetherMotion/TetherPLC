#!/usr/bin/env python3
"""mqtt_broker_fake.py — minimal MQTT 3.1.1 broker for the
tplc-mqtt e2e.  Answers CONNECT->CONNACK (parsing and storing any
last-will), SUBSCRIBE->SUBACK, broadcasts PUBLISH to every
subscriber across connections, stores retained publishes, and
publishes a connection's will when it closes WITHOUT a DISCONNECT
(ungraceful drop).  UNSUBSCRIBE->UNSUBACK, PINGREQ->PINGRESP.
Not a real broker — just enough for the client e2e."""
import socket, struct, sys, threading

SUBS = {}       # conn -> set of topic filters (all connections)
RETAINED = {}   # topic -> payload
LOCK = threading.Lock()

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

def topic_match(flt, topic):
    """minimal filter match: exact, '#'-suffix prefix, or '#' alone"""
    if flt == topic or flt == '#':
        return True
    if flt.endswith('/#') and topic.startswith(flt[:-1]):
        return True
    return False

def publish_to(topic, payload):
    """broadcast a qos0 PUBLISH to every matching subscriber"""
    pkt = struct.pack('>H', len(topic)) + topic.encode() + payload
    with LOCK:
        targets = list(SUBS.items())
    for conn, fs in targets:
        for f in fs:
            if topic_match(f, topic):
                try:
                    send(conn, 0x30, pkt)
                except Exception:
                    pass
                break

def parse_will(body):
    """extract last-will from a CONNECT body (MQTT 3.1.1)"""
    try:
        plen = struct.unpack('>H', body[0:2])[0]
        pos = 2 + plen                     # protocol name
        flags = body[pos + 1]              # connect flags
        pos += 4                           # level+flags+keepalive
        cid_len = struct.unpack('>H', body[pos:pos+2])[0]
        pos += 2 + cid_len                 # client id
        if not (flags & 0x04):
            return None
        wt_len = struct.unpack('>H', body[pos:pos+2])[0]
        pos += 2
        will_topic = body[pos:pos+wt_len].decode()
        pos += wt_len
        wm_len = struct.unpack('>H', body[pos:pos+2])[0]
        pos += 2
        will_msg = body[pos:pos+wm_len]
        return {'topic': will_topic, 'payload': will_msg,
                'qos': (flags >> 3) & 3, 'retain': (flags >> 5) & 1}
    except Exception:
        return None

def handle(conn):
    will = None
    clean = False
    try:
        while True:
            hdr, body = read_packet(conn)
            if hdr is None: break
            ptype = hdr >> 4
            if ptype == 1:      # CONNECT -> CONNACK (session 0, rc 0)
                will = parse_will(body)
                send(conn, 0x20, b'\x00\x00')
            elif ptype == 8:    # SUBSCRIBE -> SUBACK + retained delivery
                mid = struct.unpack('>H', body[:2])[0]
                tl = struct.unpack('>H', body[2:4])[0]
                topic = body[4:4+tl].decode()
                with LOCK:
                    SUBS.setdefault(conn, set()).add(topic)
                send(conn, 0x90, body[:2] + b'\x00')  # granted qos0
                for rt, rp in RETAINED.items():
                    if topic_match(topic, rt):
                        send(conn, 0x31,  # PUBLISH qos0 retain=1
                             struct.pack('>H', len(rt)) + rt.encode() + rp)
            elif ptype == 3:    # PUBLISH -> PUBACK (qos1) + broadcast
                tl = struct.unpack('>H', body[:2])[0]
                topic = body[2:2+tl].decode()
                rest = body[2+tl:]
                flags = hdr & 0x0F
                mid = None
                payload = rest
                if (flags >> 1) & 3:  # qos>0
                    mid = struct.unpack('>H', rest[:2])[0]
                    payload = rest[2:]
                if mid is not None:
                    send(conn, 0x40, struct.pack('>H', mid))
                if flags & 1:  # retain flag: store for later subscribers
                    with LOCK:
                        RETAINED[topic] = payload
                publish_to(topic, payload)
            elif ptype == 10:   # UNSUBSCRIBE -> UNSUBACK
                tl = struct.unpack('>H', body[2:4])[0]
                with LOCK:
                    SUBS.get(conn, set()).discard(body[4:4+tl].decode())
                send(conn, 0xB0, body[:2])
            elif ptype == 12:   # PINGREQ -> PINGRESP
                send(conn, 0xD0, b'')
            elif ptype == 14:   # DISCONNECT (graceful: no will)
                clean = True
                break
    except Exception:
        pass
    with LOCK:
        SUBS.pop(conn, None)
    conn.close()
    # ungraceful drop: broker publishes the registered will
    if will is not None and not clean:
        wt = will['topic']
        if will['retain']:
            with LOCK:
                RETAINED[wt] = will['payload']
        publish_to(wt, will['payload'])

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
