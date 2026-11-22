#!/usr/bin/env python3
"""Minimal RFC6455 client for `tplc serve` /ws smoke tests.

Usage: ws_client.py <port> <script.py> — imports `WS` from this file
is awkward across processes, so the test embeds the ops directly:
this file is exec'd with the port as argv[1] and runs the whole check.
"""
import base64
import json
import os
import socket
import struct
import sys


class WS:
    def __init__(self, port, path="/ws"):
        self.s = socket.create_connection(("127.0.0.1", port), timeout=120)
        key = base64.b64encode(os.urandom(16)).decode()
        req = (
            f"GET {path} HTTP/1.1\r\n"
            f"Host: 127.0.0.1:{port}\r\n"
            "Upgrade: websocket\r\n"
            "Connection: Upgrade\r\n"
            f"Sec-WebSocket-Key: {key}\r\n"
            "Sec-WebSocket-Version: 13\r\n\r\n"
        )
        self.s.sendall(req.encode())
        buf = b""
        while b"\r\n\r\n" not in buf:
            chunk = self.s.recv(4096)
            if not chunk:
                raise RuntimeError("handshake: connection closed")
            buf += chunk
        head, self.buf = buf.split(b"\r\n\r\n", 1)
        status = head.split(b"\r\n", 1)[0]
        if b" 101" not in status:
            raise RuntimeError(f"handshake failed: {status!r}")

    def _read(self, n):
        while len(self.buf) < n:
            chunk = self.s.recv(1 << 16)
            if not chunk:
                raise EOFError("ws closed")
            self.buf += chunk
        out, self.buf = self.buf[:n], self.buf[n:]
        return out

    def send(self, data, op=0x1):
        if isinstance(data, str):
            data = data.encode()
        mask = os.urandom(4)
        n = len(data)
        hdr = bytes([0x80 | op])
        if n < 126:
            hdr += bytes([0x80 | n])
        elif n <= 0xFFFF:
            hdr += bytes([0x80 | 126]) + struct.pack(">H", n)
        else:
            hdr += bytes([0x80 | 127]) + struct.pack(">Q", n)
        hdr += mask
        masked = bytes(b ^ mask[i % 4] for i, b in enumerate(data))
        self.s.sendall(hdr + masked)

    def recv(self):
        """(opcode, payload) — assumes server frames are unfragmented."""
        h = self._read(2)
        op = h[0] & 0x0F
        ln = h[1] & 0x7F
        if ln == 126:
            ln = struct.unpack(">H", self._read(2))[0]
        elif ln == 127:
            ln = struct.unpack(">Q", self._read(8))[0]
        return op, self._read(ln)

    def meta(self, op_name, fields=None, want_id=None):
        """send a JSON meta-op, return the `data` payload (or raise)."""
        mid = getattr(self, "_mid", 0) + 1
        self._mid = mid
        self.send(json.dumps({"id": mid, "op": op_name, **(fields or {})}))
        while True:
            op, payload = self.recv()
            if op != 0x1:
                continue  # binary ctl traffic — skip
            j = json.loads(payload)
            if j.get("id") != mid:
                continue
            if not j.get("ok"):
                raise RuntimeError(f"meta {op_name}: {j.get('error')}")
            return j.get("data")

    def ping_ctl(self):
        """send a raw ctl PING frame; return the reply frame bytes."""
        req = 0xdeadbeef
        frame = struct.pack("<HHIQ", 0x0103, 0, 0, req)
        self.send(frame, op=0x2)
        while True:
            op, payload = self.recv()
            if op == 0x2 and len(payload) >= 16:
                t, fl, ln = struct.unpack("<HHI", payload[:8])
                rid = struct.unpack("<Q", payload[8:16])[0]
                if rid == req:
                    return t, fl, payload[16:16 + ln]
