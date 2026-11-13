#!/usr/bin/env python3
"""mqtt_broker_fake.py — minimal MQTT 3.1.1 broker for the
tplc-mqtt e2e.  Answers CONNECT->CONNACK (parsing and storing any
last-will), SUBSCRIBE->SUBACK, broadcasts PUBLISH to every
subscriber across connections, stores retained publishes, and
publishes a connection's will when it closes WITHOUT a DISCONNECT
(ungraceful drop).  UNSUBSCRIBE->UNSUBACK, PINGREQ->PINGRESP.
Not a real broker — just enough for the client e2e."""
import socket, ssl, struct, sys, threading, time

SUBS = {}       # conn -> {topic filter: granted qos} (all connections)
RETAINED = {}   # topic -> payload
LOCK = threading.Lock()
MSGID = [1]     # broker->client packet ids for qos>0 publishes
CONNS = set()   # live client sockets
SRV = [None]    # listener socket
TLSCTX = [None] # ssl context when --tls
BOUNCE = [None] # {'topic','delay'} one-shot self-bounce
RESPONDER = [None]  # topic that triggers a req/resp reply

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

VERSIONS = {}    # conn -> protocol level (4 = 3.1.1, 5 = 5.0)

def publish_to(topic, payload, qos=0, props_raw=b'\x00'):
    """broadcast a PUBLISH to every matching subscriber at
    min(pub qos, granted qos); qos>0 carries a packet id.
    v5 subscribers get the publisher's property block
    (props_raw, default empty)."""
    with LOCK:
        targets = [(c, fs, VERSIONS.get(c, 4)) for c, fs in SUBS.items()]
        mid = MSGID[0]; MSGID[0] += 1
    for conn, fs, ver in targets:
        for f, gq in fs.items():
            if topic_match(f, topic):
                eff = min(qos, gq)
                body = struct.pack('>H', len(topic)) + topic.encode()
                if eff:
                    body += struct.pack('>H', mid)
                if ver == 5:
                    body += props_raw
                body += payload
                try:
                    send(conn, 0x30 | (eff << 1), body)
                except Exception:
                    pass
                break

def skip_props(body, pos):
    """MQTT5: properties = varint length + bytes; return pos after"""
    plen, p = 0, pos
    mult = 1
    while p < len(body):
        b = body[p]; p += 1
        plen += (b & 0x7F) * mult
        if not (b & 0x80):
            break
        mult *= 128
    return p + plen

def parse_will(body):
    """extract last-will + protocol version from a CONNECT body
    (MQTT 3.1.1 and 5.0)"""
    try:
        plen = struct.unpack('>H', body[0:2])[0]
        pos = 2 + plen                     # protocol name
        level = body[pos]
        flags = body[pos + 1]              # connect flags
        pos += 4                           # level+flags+keepalive
        if level == 5:
            pos = skip_props(body, pos)    # connect properties
        cid_len = struct.unpack('>H', body[pos:pos+2])[0]
        pos += 2 + cid_len                 # client id
        if not (flags & 0x04):
            return {'will': None, 'version': level}
        if level == 5:
            pos = skip_props(body, pos)    # will properties
        wt_len = struct.unpack('>H', body[pos:pos+2])[0]
        pos += 2
        will_topic = body[pos:pos+wt_len].decode()
        pos += wt_len
        wm_len = struct.unpack('>H', body[pos:pos+2])[0]
        pos += 2
        will_msg = body[pos:pos+wm_len]
        return {'will': {'topic': will_topic, 'payload': will_msg,
                         'qos': (flags >> 3) & 3,
                         'retain': (flags >> 5) & 1},
                'version': level}
    except Exception:
        return {'will': None, 'version': 4}

def parse_publish_props(body, pos):
    """parse MQTT5 PUBLISH properties at body[pos]; returns
    (props_dict, payload_start).  Only the fields the e2e checks
    are decoded; unknown ids stop the walk."""
    # properties = varint length + bytes
    q = pos
    plen = 0
    mult = 1
    while q < len(body):
        b = body[q]; q += 1
        plen += (b & 0x7F) * mult
        if not (b & 0x80):
            break
        mult *= 128
    p = q
    end = p + plen
    props = {}
    def take(n):
        nonlocal p
        v = body[p:p+n]; p += n; return v
    def u16(): return struct.unpack('>H', take(2))[0]
    def u32(): return struct.unpack('>I', take(4))[0]
    def u8(): return take(1)[0]
    def s():
        n = u16(); return take(n).decode('utf-8', 'replace')
    while p < end:
        pid = u8()
        if pid == 0x01: props['fmt'] = u8()
        elif pid == 0x02: props['expiry'] = u32()
        elif pid == 0x03: props['ctype'] = s()
        elif pid == 0x08: props['rtopic'] = s()
        elif pid == 0x09:
            n = u16(); props['corr'] = take(n).hex()
        elif pid == 0x26:
            k = s(); v = s()
            props.setdefault('uprops', []).append(f'{k}={v}')
        else:
            break
    return props, end

def handle(conn):
    will = None
    clean = False
    v5 = False
    try:
        while True:
            hdr, body = read_packet(conn)
            if hdr is None: break
            ptype = hdr >> 4
            if ptype == 1:      # CONNECT -> CONNACK (session 0, rc 0)
                parsed = parse_will(body)
                will = parsed['will']
                v5 = parsed['version'] == 5
                VERSIONS[conn] = parsed['version']
                if v5:
                    # CONNACK v5: ack flags + reason + properties
                    send(conn, 0x20, b'\x00\x00\x00')
                else:
                    send(conn, 0x20, b'\x00\x00')
            elif ptype == 8:    # SUBSCRIBE -> SUBACK + retained delivery
                # body: packet id + [v5 props] + repeated
                # (topic len, topic, opts, [v5 props])
                sprows = []
                if v5:
                    # v5 inserts a properties field after the packet id
                    pos = skip_props(body, 2)
                    # capture the props block for logging
                    q = 2; plen = 0; mult = 1
                    while q < len(body):
                        b = body[q]; q += 1
                        plen += (b & 0x7F) * mult
                        if not (b & 0x80):
                            break
                        mult *= 128
                    pe = q + plen
                    pp = q
                    while pp < pe:
                        pid = body[pp]; pp += 1
                        if pid == 0x0B:   # subscription identifier (varint)
                            sid = 0; mv = 1
                            while pp < pe:
                                b = body[pp]; pp += 1
                                sid += (b & 0x7F) * mv
                                if not (b & 0x80):
                                    break
                                mv *= 128
                            sprows.append(f'sid={sid}')
                        elif pid == 0x26:  # user property
                            kl = struct.unpack('>H', body[pp:pp+2])[0]; pp += 2
                            k = body[pp:pp+kl].decode(); pp += kl
                            vl = struct.unpack('>H', body[pp:pp+2])[0]; pp += 2
                            v = body[pp:pp+vl].decode(); pp += vl
                            sprows.append(f'up={k}={v}')
                        elif pid in (0x01, 0x11, 0x13, 0x21, 0x22, 0x23,
                                     0x24, 0x25, 0x27, 0x28, 0x29, 0x2A):
                            pp += 1
                        elif pid in (0x02, 0x03, 0x08, 0x09, 0x12, 0x15,
                                     0x16, 0x17, 0x18, 0x19, 0x1A, 0x1C,
                                     0x1F, 0x20, 0x2B):
                            break
                        else:
                            break
                else:
                    pos = 2
                granted = []
                new_topics = []
                while pos + 2 <= len(body):
                    tl = struct.unpack('>H', body[pos:pos+2])[0]
                    pos += 2
                    topic = body[pos:pos+tl].decode()
                    pos += tl
                    if v5:
                        opts = body[pos]; pos += 1
                        req = opts & 0x03
                        if opts & 0x04:   # no local
                            sprows.append('nolocal=1')
                        if opts & 0x08:   # retain as published
                            sprows.append('rap=1')
                        rh = (opts >> 4) & 0x03
                        if rh:
                            sprows.append(f'rh={rh}')
                        pos = skip_props(body, pos)  # per-topic props
                    else:
                        req = body[pos] if pos < len(body) else 0
                        pos += 1
                    gq = min(req, 2)
                    granted.append(gq)
                    new_topics.append(topic)
                    with LOCK:
                        SUBS.setdefault(conn, {})[topic] = gq
                if sprows:
                    print('SUBPROPS', ' '.join(sprows), flush=True)
                if v5:
                    # SUBACK v5: packet id + props + reason codes
                    send(conn, 0x90,
                         body[:2] + b'\x00' + bytes(granted))
                else:
                    send(conn, 0x90, body[:2] + bytes(granted))
                for rt, rp in RETAINED.items():
                    if any(topic_match(t, rt) for t in new_topics):
                        rbody = struct.pack('>H', len(rt)) + rt.encode()
                        if v5:
                            rbody += b'\x00'    # empty properties
                        send(conn, 0x31,  # PUBLISH qos0 retain=1
                             rbody + rp)
            elif ptype == 3:    # PUBLISH -> qos ack + broadcast
                tl = struct.unpack('>H', body[:2])[0]
                topic = body[2:2+tl].decode()
                rest = body[2+tl:]
                flags = hdr & 0x0F
                qos = (flags >> 1) & 3
                mid = None
                if qos:
                    mid = struct.unpack('>H', rest[:2])[0]
                    rest = rest[2:]
                props_raw = b'\x00'
                if v5:
                    # v5 inserts a properties field before the payload
                    pos = 2 + tl + (2 if qos else 0)
                    pstart = pos
                    pprops, pos = parse_publish_props(body, pos)
                    props_raw = body[pstart:pos]
                    if pprops:
                        print('PUBPROPS', topic, pprops, flush=True)
                    payload = body[pos:]
                else:
                    payload = rest
                if qos == 1:
                    if v5:
                        send(conn, 0x40, struct.pack('>H', mid) + b'\x00\x00')
                    else:
                        send(conn, 0x40, struct.pack('>H', mid))   # PUBACK
                elif qos == 2:
                    if v5:
                        send(conn, 0x50, struct.pack('>H', mid) + b'\x00\x00')
                    else:
                        send(conn, 0x50, struct.pack('>H', mid))   # PUBREC
                if flags & 1:  # retain flag: store for later subscribers
                    with LOCK:
                        RETAINED[topic] = payload
                publish_to(topic, payload, qos, props_raw)
                # request/response responder: a v5 publish carrying
                # ResponseTopic + CorrelationData gets a 'pong' reply
                # on the response topic with the correlation echoed
                if (v5 and RESPONDER[0] and topic == RESPONDER[0]
                        and pprops.get('rtopic')):
                    rt = pprops['rtopic']
                    corr = bytes.fromhex(pprops.get('corr', ''))
                    rprops = b'\x00'
                    if corr:
                        pb = b'\x09' + struct.pack('>H', len(corr)) + corr
                        rprops = bytes([len(pb)]) + pb
                    print('RESPONDER reply ->', rt, flush=True)
                    publish_to(rt, b'pong', 0, rprops)
                # one-shot self-bounce: after broadcasting the trigger
                # topic, drop every client + the listener, then rebind
                # after `delay` seconds (deterministic reconnect e2e)
                if BOUNCE[0] and topic == BOUNCE[0]['topic']:
                    b = BOUNCE[0]; BOUNCE[0] = None
                    threading.Thread(target=bounce, args=(b,),
                                     daemon=True).start()
            elif ptype == 6:    # PUBREL (outbound qos2) -> PUBCOMP
                send(conn, 0x70, body[:2])
            elif ptype == 5:    # PUBREC (inbound qos2) -> PUBREL
                send(conn, 0x62, body[:2])
            elif ptype == 10:   # UNSUBSCRIBE -> UNSUBACK
                pos = skip_props(body, 2) if v5 else 2
                ntopics = 0
                while pos + 2 <= len(body):
                    tl = struct.unpack('>H', body[pos:pos+2])[0]
                    pos += 2
                    with LOCK:
                        SUBS.get(conn, {}).pop(body[pos:pos+tl].decode(),
                                               None)
                    pos += tl
                    ntopics += 1
                if v5:
                    # UNSUBACK v5: packet id + props + reason codes
                    send(conn, 0xB0,
                         body[:2] + b'\x00' + b'\x00' * ntopics)
                else:
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
        CONNS.discard(conn)
        VERSIONS.pop(conn, None)
    try:
        conn.close()
    except Exception:
        pass
    # ungraceful drop: broker publishes the registered will
    if will is not None and not clean:
        wt = will['topic']
        if will['retain']:
            with LOCK:
                RETAINED[wt] = will['payload']
        publish_to(wt, will['payload'], will['qos'])

def accept_loop(s):
    while True:
        try:
            c, _ = s.accept()
        except Exception:
            return
        with LOCK:
            CONNS.add(c)
        threading.Thread(target=handle, args=(c,), daemon=True).start()

def bounce(b):
    """drop every client conn so they see an ungraceful close; the
    listener stays up so reconnecting clients find a live broker.
    (Closing the listener does not wake a thread blocked in
    accept(), so it is left alone.)"""
    with LOCK:
        conns = list(CONNS)
    for c in conns:
        try:
            c.shutdown(socket.SHUT_RDWR)
        except Exception:
            pass
        try:
            c.close()
        except Exception:
            pass
    print('mqtt_broker_fake bounced', flush=True)

def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 11883
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(('127.0.0.1', port)); s.listen(4)
    # optional TLS: --tls <cert.pem> <key.pem> wraps the listener
    if len(sys.argv) > 4 and sys.argv[2] == '--tls':
        ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        ctx.load_cert_chain(sys.argv[3], sys.argv[4])
        TLSCTX[0] = ctx
        s = ctx.wrap_socket(s, server_side=True)
        print(f'mqtt_broker_fake TLS on {port}', flush=True)
    else:
        print(f'mqtt_broker_fake on {port}', flush=True)
    # optional self-bounce: --bounce <topic> <delay_s>
    if '--bounce' in sys.argv:
        i = sys.argv.index('--bounce')
        BOUNCE[0] = {'topic': sys.argv[i + 1],
                     'delay': float(sys.argv[i + 2]),
                     'port': port}
    # optional req/resp responder: --responder <request-topic>
    if '--responder' in sys.argv:
        RESPONDER[0] = sys.argv[sys.argv.index('--responder') + 1]
    SRV[0] = s
    accept_loop(s)

if __name__ == '__main__':
    main()
