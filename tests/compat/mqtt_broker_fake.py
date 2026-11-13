#!/usr/bin/env python3
"""mqtt_broker_fake.py — minimal MQTT 3.1.1 broker for the
tplc-mqtt e2e.  Answers CONNECT->CONNACK (parsing and storing any
last-will), SUBSCRIBE->SUBACK, broadcasts PUBLISH to every
subscriber across connections, stores retained publishes, and
publishes a connection's will when it closes WITHOUT a DISCONNECT
(ungraceful drop).  UNSUBSCRIBE->UNSUBACK, PINGREQ->PINGRESP.
Not a real broker — just enough for the client e2e."""
import socket, struct, sys, threading

SUBS = {}       # conn -> {topic filter: granted qos} (all connections)
RETAINED = {}   # topic -> payload
LOCK = threading.Lock()
MSGID = [1]     # broker->client packet ids for qos>0 publishes

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

def publish_to(topic, payload, qos=0):
    """broadcast a PUBLISH to every matching subscriber at
    min(pub qos, granted qos); qos>0 carries a packet id"""
    with LOCK:
        targets = list(SUBS.items())
        mid = MSGID[0]; MSGID[0] += 1
    for conn, fs in targets:
        for f, gq in fs.items():
            if topic_match(f, topic):
                eff = min(qos, gq)
                body = struct.pack('>H', len(topic)) + topic.encode()
                if eff:
                    body += struct.pack('>H', mid)
                body += payload
                try:
                    send(conn, 0x30 | (eff << 1), body)
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
                # body: packet id + repeated (topic len, topic, qos)
                pos = 2
                granted = []
                new_topics = []
                while pos + 2 <= len(body):
                    tl = struct.unpack('>H', body[pos:pos+2])[0]
                    pos += 2
                    topic = body[pos:pos+tl].decode()
                    pos += tl
                    req = body[pos] if pos < len(body) else 0
                    pos += 1
                    gq = min(req, 2)
                    granted.append(gq)
                    new_topics.append(topic)
                    with LOCK:
                        SUBS.setdefault(conn, {})[topic] = gq
                send(conn, 0x90, body[:2] + bytes(granted))
                for rt, rp in RETAINED.items():
                    if any(topic_match(t, rt) for t in new_topics):
                        send(conn, 0x31,  # PUBLISH qos0 retain=1
                             struct.pack('>H', len(rt)) + rt.encode() + rp)
            elif ptype == 3:    # PUBLISH -> qos ack + broadcast
                tl = struct.unpack('>H', body[:2])[0]
                topic = body[2:2+tl].decode()
                rest = body[2+tl:]
                flags = hdr & 0x0F
                qos = (flags >> 1) & 3
                mid = None
                payload = rest
                if qos:
                    mid = struct.unpack('>H', rest[:2])[0]
                    payload = rest[2:]
                if qos == 1:
                    send(conn, 0x40, struct.pack('>H', mid))   # PUBACK
                elif qos == 2:
                    send(conn, 0x50, struct.pack('>H', mid))   # PUBREC
                if flags & 1:  # retain flag: store for later subscribers
                    with LOCK:
                        RETAINED[topic] = payload
                publish_to(topic, payload, qos)
            elif ptype == 6:    # PUBREL (outbound qos2) -> PUBCOMP
                send(conn, 0x70, body[:2])
            elif ptype == 5:    # PUBREC (inbound qos2) -> PUBREL
                send(conn, 0x62, body[:2])
            elif ptype == 10:   # UNSUBSCRIBE -> UNSUBACK
                pos = 2
                while pos + 2 <= len(body):
                    tl = struct.unpack('>H', body[pos:pos+2])[0]
                    pos += 2
                    with LOCK:
                        SUBS.get(conn, {}).pop(body[pos:pos+tl].decode(),
                                               None)
                    pos += tl
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
        publish_to(wt, will['payload'], will['qos'])

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
