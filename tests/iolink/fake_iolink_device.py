#!/usr/bin/env python3
"""fake_iolink_device.py — in-process IO-Link device for tplc-iolink e2e.

Speaks the device side of the IO-Link data-link protocol (IEC 61131-9)
over a TCP byte stream: parses [MC][CKT][body], answers with
[data...][CKS].  DPP1 page record advertises 4-byte process data in
both directions; ISDU params live in a table addressed by
(index, subindex); cyclic frames get a deterministic PD-in pattern.
Wakeup bytes (0x55) are skipped.

ISDU support mirrors the reference device stack (iolinki): the
I-Service octet stream is segmented over ISDU-channel M-sequences
with FlowCTRL START/COUNT/IDLE in the MC address bits and two OD
octets per type-2 M-sequence (PD-free profile).  MasterCommand
writes to the page channel address the port state.
"""
import socket
import sys

PDOUT_LEN = 4
PDIN_LEN = 4
ISDU_OD = 2

# DPP1 identification record served on the page channel (16 octets).
DPP1 = bytes([
    0x00,             # [0] reserved
    0x00, 0x02,       # [1..2] minimum cycle time
    0x03,             # [3] M-sequence capability (ISDU supported)
    0x01,             # [4] revision
    0x84,             # [5] PD-in descriptor: 4 octets
    0x84,             # [6] PD-out descriptor: 4 octets
    0x34, 0x12,       # [7..8] vendor ID 0x1234 (LE)
    0x78, 0x56,       # [9..10] device ID 0x5678 (LE)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
])

# Legacy single-octet ISDU table (OP 6/7 path).
PARAMS = [0] * 16

# Spec ISDU object store: (index, subindex) -> bytes.
ISDU_PARAMS = {}

FLOW_START = 0x10
FLOW_IDLE = 0x11
FLOW_IDLE2 = 0x12
FLOW_ABORT = 0x1F


def cksum6(data):
    """A.1.6 checksum: XOR seed 0x52, fold 8->6 bits."""
    ck = 0x52
    for b in data:
        ck ^= b
    b = lambda i: (ck >> i) & 1
    r = ((b(7) ^ b(5) ^ b(3) ^ b(1)) << 5) | \
        ((b(6) ^ b(4) ^ b(2) ^ b(0)) << 4) | \
        ((b(7) ^ b(6)) << 3) | \
        ((b(5) ^ b(4)) << 2) | \
        ((b(3) ^ b(2)) << 1) | \
        (b(1) ^ b(0))
    return r & 0x3F


def cks_frame(data, flags=0):
    """Append CKS: checksum over data + CKS-position byte holding only
    the flag bits (bits 0-5 zeroed)."""
    ck = cksum6(bytes(data) + bytes([flags & 0xC0]))
    return bytes(data) + bytes([(flags & 0xC0) | ck])


def recvn(conn, n):
    buf = b""
    while len(buf) < n:
        chunk = conn.recv(n - len(buf))
        if not chunk:
            raise ConnectionError("eof")
        buf += chunk
    return buf


def xor_all(data):
    x = 0
    for b in data:
        x ^= b
    return x


class IsduDevice:
    """Device-side segmented-ISDU transport (A.5, Tables 52-54)."""

    def __init__(self):
        self.reset()

    def reset(self):
        self.state = "idle"      # idle | ready
        self.req = bytearray()
        self.req_total = 0
        self.resp = bytearray()
        self.resp_total = 0
        self.resp_idx = 0
        self.last_fc = -1
        self.flowctrl_seen = False
        self.last_read_fc = -1
        self.last_read_od = None

    def declared_total(self):
        if not self.req:
            return 0
        ln = self.req[0] & 0x0F
        if ln == 0:
            return 1
        if ln == 1:
            if len(self.req) < 2:
                return 0
            ext = self.req[1]
            return ext if 17 <= ext <= 238 else 0
        return ln

    def write_od(self, fc, od):
        """Feed two OD octets of a write M-sequence."""
        if fc >= FLOW_IDLE:
            return
        if fc == FLOW_START:
            self.req = bytearray()
            self.req_total = 0
            self.flowctrl_seen = True
            self.last_fc = fc
            self.state = "collect"
        else:
            if not self.flowctrl_seen or self.state != "collect":
                self.reset()
                return
            if fc == self.last_fc:
                return  # repeated FlowCTRL: payload ignored
            if fc != ((self.last_fc + 1) & 0x0F):
                self.reset()
                return
            self.last_fc = fc
        for b in od:
            self.req.append(b)
            if self.req_total == 0:
                self.req_total = self.declared_total()
            if self.req_total and len(self.req) >= self.req_total:
                break
        if self.req_total and len(self.req) >= self.req_total:
            self.execute()

    def read_od(self, fc):
        """Return the two OD octets for a read M-sequence."""
        if fc in (FLOW_IDLE, FLOW_IDLE2, FLOW_ABORT):
            self.reset()
            return [0, 0]
        if fc == FLOW_START:
            if self.state != "ready":
                # idle or mid-request: No Service (0x00)
                self.resp = bytearray([0x00])
                self.resp_total = 1
                self.resp_idx = 0
            else:
                self.resp_idx = 0  # restart from first octet
            self.last_read_fc = fc
            self.flowctrl_seen = True
        elif fc < FLOW_START:
            if self.last_read_od is None:
                return [0, 0]
            if fc == self.last_read_fc:
                return list(self.last_read_od)  # repeat segment
            exp = (self.last_read_fc + 1) & 0x0F
            if (fc != exp or self.state != "ready"
                    or self.resp_idx >= self.resp_total):
                self.reset()
                return [0, 0]
            self.last_read_fc = fc
        else:
            return [0, 0]  # reserved FlowCTRL
        out = []
        for _ in range(ISDU_OD):
            if self.resp_idx < self.resp_total:
                out.append(self.resp[self.resp_idx])
                self.resp_idx += 1
            else:
                out.append(0)
        if self.state == "ready" and self.resp_idx >= self.resp_total:
            self.reset()
        self.last_read_od = bytes(out)
        return out

    def _respond(self, body):
        self.resp = bytearray(body)
        self.resp.append(xor_all(body) & 0xFF)
        self.resp_total = len(self.resp)
        self.resp_idx = 0
        self.state = "ready"

    def execute(self):
        req = bytes(self.req[:self.req_total])
        svc = req[0] >> 4
        pos = 1
        if (req[0] & 0x0F) == 1:
            pos += 1  # ExtLength
        payload_end = len(req) - 1  # exclude CHKPDU
        if svc in (3, 11):           # 16-bit index + subindex
            if pos + 1 >= payload_end:
                self._respond(bytes([0xC4 if svc == 11 else 0x44,
                                     0x80, 0x12]))
                return
            idx = (req[pos] << 8) | req[pos + 1]
            pos += 2
        else:
            if pos >= payload_end:
                self._respond(bytes([0xC4 if svc in (9, 10) else 0x44,
                                     0x80, 0x12]))
                return
            idx = req[pos]
            pos += 1
        sub = 0
        if svc in (2, 3, 10, 11):
            if pos >= payload_end:
                self._respond(bytes([0xC4 if svc in (9, 10, 11) else 0x44,
                                     0x80, 0x12]))
                return
            sub = req[pos]
            pos += 1
        data = req[pos:payload_end]
        wr = svc in (1, 2, 3)
        if xor_all(req) != 0:
            self._respond(bytes([0x44 if wr else 0xC4, 0x80, 0x00]))
            return
        key = (idx, sub)
        if wr:
            ISDU_PARAMS[key] = bytes(data)
            self._respond(bytes([0x52]))       # write response (+)
            return
        if key in ISDU_PARAMS:
            payload = ISDU_PARAMS[key]
            direct = len(payload) + 2
            if direct <= 15:
                body = bytes([0xD0 | direct]) + payload
            else:
                body = bytes([0xD1, len(payload) + 3]) + payload
            self._respond(body)
        else:
            # negative read response: ErrorCode 0x80 (APP_DEV),
            # AdditionalCode 0x11 (index not available)
            self._respond(bytes([0xC4, 0x80, 0x11]))


def handle(conn):
    global PARAMS
    isdu = IsduDevice()
    pd_count = 0
    while True:
        # frame start: skip wakeup bytes
        mc = recvn(conn, 1)[0]
        while mc == 0x55:
            mc = recvn(conn, 1)[0]
        ckt = recvn(conn, 1)[0]
        channel = (mc >> 5) & 0x03
        is_read = bool(mc & 0x80)
        mseq = ckt >> 6
        addr = mc & 0x1F

        if channel == 0x01:            # PAGE, type 0
            if is_read:
                body = b""
            else:
                body = recvn(conn, 1)   # MasterCommand octet
            if is_read:
                conn.sendall(cks_frame(bytes([DPP1[addr & 0x0F]])))
            else:
                # MasterCommand accepted; DeviceOperate (0x99) moves
                # the port to OPERATE — tracked implicitly.
                conn.sendall(cks_frame(b""))
        elif channel == 0x03 and mseq == 0:
            # legacy single-octet ISDU (OP 6/7 profile)
            body = b"" if is_read else recvn(conn, 1)
            a = mc & 0x0F
            if is_read:
                conn.sendall(cks_frame(bytes([PARAMS[a]])))
            else:
                PARAMS[a] = body[0]
                conn.sendall(cks_frame(body))
        elif channel == 0x03 and mseq == 2:
            # segmented ISDU on the type-2 profile: two OD octets
            fc = addr & 0x1F
            if is_read:
                od = isdu.read_od(fc)
                conn.sendall(cks_frame(bytes(od)))
            else:
                od = recvn(conn, ISDU_OD)
                isdu.write_od(fc, od)
                conn.sendall(cks_frame(b""))  # write reply: CKS only
        elif channel == 0x00:          # PROCESS, type 2_V
            body = recvn(conn, PDOUT_LEN)
            pd_count += 1
            pdi = bytes([
                0x42,
                (body[0] + 1) & 0xFF,
                0x99,
                pd_count & 0xFF,
            ])
            conn.sendall(cks_frame(pdi))
        else:                          # DIAGNOSIS: unsupported
            conn.sendall(cks_frame(b"\x00"))


def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 14411
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", port))
    srv.listen(1)
    print(f"fake iolink device on 127.0.0.1:{port}", flush=True)
    while True:
        conn, _ = srv.accept()
        try:
            handle(conn)
        except (ConnectionError, OSError):
            pass
        finally:
            conn.close()


if __name__ == "__main__":
    main()
