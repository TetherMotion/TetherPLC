#!/usr/bin/env python3
"""Minimal EtherNet/IP adapter for the tplc-eip e2e: answers the
encapsulation layer (RegisterSession 0x65, ListIdentity 0x63,
UnRegisterSession 0x66) and SendRRData (0x6F) carrying CIP
Get/Set_Attribute_Single on a tiny object table. Wire-level peer for
EipService — not a real device."""
import socket, struct, sys, threading

OBJ = {}          # (class, inst, attr) -> bytes
SESS = 0x1234ABCD

def encap(cmd, sess, status, body):
    return struct.pack('<HHI', cmd, len(body), sess) + struct.pack('<I', status) + b'\0'*12 + body

def seg_read(p, off):
    """read one logical segment -> (value16, new_off)."""
    t = p[off]
    if t & 0x01:           # 16-bit form
        return struct.unpack_from('<H', p, off+2)[0], off + 4
    return p[off+1], off + 2

def handle_cip(cip):
    svc = cip[0]
    nwords = cip[1]
    path = cip[2:2 + nwords*2]
    data = cip[2 + nwords*2:]
    cls, o = seg_read(path, 0)
    inst, o = seg_read(path, o)
    attr, o = seg_read(path, o)
    if svc == 0x0E:        # Get_Attribute_Single
        val = OBJ.get((cls, inst, attr))
        if val is None:
            return bytes([svc | 0x80, 0, 0x14, 0])      # BadAttribute
        return bytes([svc | 0x80, 0, 0, 0]) + val
    if svc == 0x10:        # Set_Attribute_Single
        OBJ[(cls, inst, attr)] = data
        return bytes([svc | 0x80, 0, 0, 0])
    return bytes([svc | 0x80, 0, 0x08, 0])              # ServiceNotSupported

def conn(c):
    try:
        sess = 0
        while True:
            hdr = b''
            while len(hdr) < 24:
                r = c.recv(24 - len(hdr))
                if not r: return
                hdr += r
            cmd, ln, sess_h, status = struct.unpack('<HHII', hdr[:12])
            body = b''
            while len(body) < ln:
                r = c.recv(ln - len(body))
                if not r: return
                body += r
            print(f'cmd=0x{cmd:02x} len={ln}', flush=True)
            if cmd == 0x65:                    # RegisterSession
                sess = SESS
                c.sendall(encap(0x65, sess, 0, struct.pack('<HH', 1, 0)))
            elif cmd == 0x66:                  # UnRegisterSession
                return
            elif cmd == 0x63:                  # ListIdentity
                name = b'TetherEipAdapter'
                ident = struct.pack('<H', 1)   # protocolVer
                ident += struct.pack('>H', 44818) + socket.inet_aton('127.0.0.1')
                ident += b'\0'*10
                ident += struct.pack('<HHH', 1, 12, 99)  # vendor,devtype,prodcode
                ident += bytes([1, 2]) + struct.pack('<HI', 0x30, 0xABCD)
                ident += bytes([len(name)]) + name + b'\x00'
                item = struct.pack('<HH', 0x0C, len(ident)) + ident
                c.sendall(encap(0x63, 0, 0, struct.pack('<H', 1) + item))
            elif cmd == 0x6F:                  # SendRRData
                ih, to, nitems = struct.unpack_from('<IHH', body, 0)
                off = 8
                for _ in range(nitems):
                    t, il = struct.unpack_from('<HH', body, off)
                    if t == 0x00B2:
                        cip = body[off+4:off+4+il]
                        resp = handle_cip(cip)
                        out = struct.pack('<IHH', 0, 0, 2)
                        out += struct.pack('<HH', 0x0000, 0)
                        out += struct.pack('<HH', 0x00B2, len(resp)) + resp
                        c.sendall(encap(0x6F, sess, 0, out))
                        break
                    off += 4 + il
            else:
                c.sendall(encap(cmd, sess, 1, b''))
    except OSError:
        pass
    finally:
        c.close()

def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 44818
    OBJ[(1, 1, 3)] = b'TetherEipAdapter'
    OBJ[(4, 100, 3)] = struct.pack('<I', 0x5A5A1234)
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind(('127.0.0.1', port))
    s.listen(8)
    print(f'eip fake on {port}', flush=True)
    while True:
        c, _ = s.accept()
        threading.Thread(target=conn, args=(c,), daemon=True).start()

if __name__ == '__main__':
    main()
