#!/usr/bin/env python3
"""ocpp_csms_fake.py — minimal OCPP-J central-system fake for the
tplc-ocpp e2e.  Answers the RFC6455 upgrade, then echoes every
OCPP-J call back as a result ([3,<same-id>,{"echo":...}]) so the
client sees a real roundtrip over a real WebSocket frame."""
import socket, struct, sys, threading, base64, hashlib

GUID = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"

def ws_accept(key):
    return base64.b64encode(hashlib.sha1(key.encode() + GUID.encode()).digest()).decode()

def send_ws(s, text):
    data = text.encode(); hdr = bytes([0x81])
    n = len(data)
    if n < 126: hdr += bytes([n])
    elif n < 65536: hdr += bytes([126]) + struct.pack('>H', n)
    else: hdr += bytes([127]) + struct.pack('>Q', n)
    s.sendall(hdr + data)

def recv_ws(s):
    h = s.recv(2)
    if len(h) < 2: return None, None
    op = h[0] & 0x0F; n = h[1] & 0x7F; masked = h[1] & 0x80
    if n == 126: n = struct.unpack('>H', s.recv(2))[0]
    elif n == 127: n = struct.unpack('>Q', s.recv(8))[0]
    mask = s.recv(4) if masked else None
    payload = b''
    while len(payload) < n:
        c = s.recv(n - len(payload))
        if not c: return None, None
        payload += c
    if mask:
        payload = bytes(b ^ mask[i & 3] for i, b in enumerate(payload))
    return op, payload

def handle(conn):
    try:
        # HTTP upgrade handshake
        req = b''
        while b'\r\n\r\n' not in req:
            c = conn.recv(4096)
            if not c: return
            req += c
        headers = dict(l.split(': ', 1) for l in
                       req.decode().split('\r\n') if ': ' in l)
        key = headers.get('Sec-WebSocket-Key', '')
        resp = ("HTTP/1.1 101 Switching Protocols\r\n"
                "Upgrade: websocket\r\nConnection: Upgrade\r\n"
                f"Sec-WebSocket-Accept: {ws_accept(key)}\r\n"
                "Sec-WebSocket-Protocol: ocpp1.6\r\n\r\n")
        conn.sendall(resp.encode())
        # OCPP-J: echo every call [2,id,action,payload] -> result
        while True:
            op, payload = recv_ws(conn)
            if op is None or op == 8: break
            if op != 1: continue
            import json
            msg = json.loads(payload.decode())
            if isinstance(msg, list) and msg and msg[0] == 2:
                send_ws(conn, json.dumps([3, msg[1], {"echo": msg[3] if len(msg) > 3 else {}}]))
    except Exception:
        pass
    conn.close()

def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 11890
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(('127.0.0.1', port)); s.listen(4)
    print(f'ocpp_csms_fake on {port}', flush=True)
    while True:
        c, _ = s.accept()
        threading.Thread(target=handle, args=(c,), daemon=True).start()

if __name__ == '__main__':
    main()
