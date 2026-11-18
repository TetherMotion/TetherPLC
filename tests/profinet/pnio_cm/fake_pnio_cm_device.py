# Fake PNIO device: answers Connect/Write/Control-PrmEnd on UDP, sends AppReady.
import socket, struct, sys, threading, time

import sys
PORT = int(sys.argv[1]) if len(sys.argv)>1 else 34964
DEV_IFACE = bytes.fromhex('dea000016c9711d1827100a02442df7d')
# wire form is LE for uuid fields; device responds BE so store canonical
IFACE_CANON = bytes.fromhex('dea000016c9711d1827100a02442df7d')
CTRL_IFACE = bytes.fromhex('dea000026c9711d1827100a02442df7d')
DEV_OBJ   = bytes.fromhex('dea000006c9711d18271000100020493')

def uuid_wire(canon, le):
    # canon: 16 bytes text order; wire: u32,u16,u16 in drep, rest verbatim
    b = canon
    if le:
        return b[3::-1] + b[5:3:-1] + b[7:5:-1] + b[8:]
    return b

def hdr(ptype, flags, drep_le, obj, iface, act, seq, opnum, fraglen, serial=0):
    h = bytearray(80)
    h[0]=4; h[1]=ptype; h[2]=flags; h[3]=0
    h[4]=0x10 if drep_le else 0x00
    h[7]=serial>>8
    h[8:24]=uuid_wire(obj,drep_le); h[24:40]=uuid_wire(iface,drep_le); h[40:56]=uuid_wire(act,drep_le)
    pk=lambda o,v: h.__setitem__(slice(o,o+4),struct.pack('<I' if drep_le else '>I',v))
    ph=lambda o,v: h.__setitem__(slice(o,o+2),struct.pack('<H' if drep_le else '>H',v))
    pk(56,0); pk(60,1); pk(64,seq); ph(68,opnum); ph(70,0xffff); ph(72,0xffff); ph(74,fraglen); ph(76,0); h[78]=0; h[79]=serial&0xff
    return bytes(h)

def ndr_req(blocks): return struct.pack('<IIIII',len(blocks),)*0+struct.pack('<IIIII',0,0,0,0,0) # placeholder
def ndr_request(bl, le=True):
    f='<5I' if le else '>5I'
    return struct.pack(f, len(bl),len(bl),len(bl),0,len(bl))
def ndr_response(status, bl, req_args_max, le=False):
    f='<5I' if le else '>5I'
    return struct.pack(f, status,len(bl),req_args_max,0,len(bl))

def blk(t, body):
    return struct.pack('>HHBB',t,len(body)+2,1,0)+body

def blocks_ar_res(ar_uuid, key, mac, port=0x8892):
    return blk(0x8101, struct.pack('>H',1)+ar_uuid+struct.pack('>H',key)+mac+struct.pack('>H',port))
def blocks_iocr_res(t,ref,fid): return blk(0x8102, struct.pack('>HHH',t,ref,fid))
def blocks_alarm_res(): return blk(0x8103, struct.pack('>HHH',1,0,200))
def blocks_server_res(name=b'pnet-dev'):
    body=struct.pack('>H',len(name))+name
    pad=(- (6+len(body)))%4; body+=b'\0'*pad
    return blk(0x8106,body)
def ctrl_block(t, ar_uuid, key, cmd):
    return blk(t, struct.pack('>H',0)+ar_uuid+struct.pack('>H',key)+struct.pack('>H',0)+struct.pack('>H',cmd)+struct.pack('>H',0))
def write_res_for(reqblocks):
    # parse req records, echo IODWriteResHeader per record
    out=b''; p=0
    while p<len(reqblocks):
        t,bl=struct.unpack('>HH',reqblocks[p:p+4])[0],struct.unpack('>H',reqblocks[p+2:p+4])[0]
        body=reqblocks[p+6:p+6+bl-2]
        if t!=0x0008: break
        seq=struct.unpack('>H',body[0:2])[0]; ar=body[2:18]
        api,=struct.unpack('>I',body[18:22]); slot,sub=struct.unpack('>HH',body[22:26])
        idx,=struct.unpack('>H',body[28:30]); rdl,=struct.unpack('>I',body[30:34])
        out+=blk(0x8008, struct.pack('>H',seq)+ar+struct.pack('>I',api)+struct.pack('>HH',slot,sub)+struct.pack('>H',0)+struct.pack('>H',idx)+struct.pack('>I',0)+struct.pack('>HH',0,0)+struct.pack('>I',0)+b'\0'*16)
        p+=6+bl-2+rdl
        p=(p+3)&~3
    return out

def handle(data, addr, sock):
    if len(data)<100: return
    ptype=data[1]; flags=data[2]; le=bool(data[4]&0x10)
    u32=lambda o: struct.unpack('<I' if le else '>I',data[o:o+4])[0]
    u16=lambda o: struct.unpack('<H' if le else '>H',data[o:o+2])[0]
    opnum=u16(68); seq=u32(64); act=data[40:56]; obj=data[8:24]
    ndr=data[80:100]
    ns=lambda o: struct.unpack('<I' if le else '>I',ndr[o:o+4])[0]
    status,alen,mx,off,actual=ns(0),ns(4),ns(8),ns(12),ns(16)
    blocks=data[100:100+actual] if len(data)>=100+actual else data[100:]
    # find ar_uuid in connect blocks (first block ARBlockReq 0x0101)
    if opnum==0 and ptype==0:
        # parse ARBlockReq: hdr6 + arType2+uuid16+key2+mac6+objuuid16+props4+atf2+port2+namelen2+name
        b=blocks
        t,bl=struct.unpack('>HH',b[:4])[0],struct.unpack('>H',b[2:4])[0]
        body=b[6:6+bl-2]
        ar_uuid=body[2:18]; key=struct.unpack('>H',body[18:20])[0]
        res=(blocks_ar_res(ar_uuid,key,bytes.fromhex('8cf319cd19f8'))+
             blocks_iocr_res(1,1,0x8000)+blocks_iocr_res(2,2,0x8001)+
             blocks_alarm_res()+blocks_server_res())
        payload=hdr(2,0x28,False,DEV_OBJ_CANON,IFACE_CANON,act,seq,0,len(res)+20)+ndr_response(0,res,mx)+res
        sock.sendto(payload,addr); print('connect -> res',len(res),'B',flush=True)
        # then device sends AppReady after PrmEnd (handled below); schedule now? do after prmend
    elif opnum==3 and ptype==0:
        res=write_res_for(blocks)
        payload=hdr(2,0x28,False,DEV_OBJ_CANON,IFACE_CANON,act,seq,3,len(res)+20)+ndr_response(0,res,mx)+res
        sock.sendto(payload,addr); print('write -> res',len(res),'B',flush=True)
    elif opnum==4 and ptype==0:
        t=struct.unpack('>H',blocks[0:2])[0]
        if t==0x0110:  # PrmEnd -> Done, then send AppReady
            ar_uuid=blocks[8:24]; key=struct.unpack('>H',blocks[26:28])[0]
            res=ctrl_block(0x8110,ar_uuid,key,0x0008)
            payload=hdr(2,0x28,False,DEV_OBJ_CANON,IFACE_CANON,act,seq,4,len(res)+20)+ndr_response(0,res,mx)+res
            sock.sendto(payload,addr); print('prmend -> done',flush=True)
            # device->controller AppReady on controller iface, BE, opnum4 seq0? use same act, iface=CTRL
            app=ctrl_block(0x0112,ar_uuid,key,0x0002)
            # controller iface uuid canon; object = controller obj (any)
            p2=hdr(0,0x20,False,b'\x01'*16,CTRL_IFACE,act,0,4,len(app)+20)+ndr_request(app,le=False)+app
            time.sleep(0.05); sock.sendto(p2,addr); print('appready -> sent',flush=True)
        elif t==0x0114:
            ar_uuid=blocks[8:24]; key=struct.unpack('>H',blocks[26:28])[0]
            res=ctrl_block(0x8114,ar_uuid,key,0x0008)
            payload=hdr(2,0x28,False,DEV_OBJ_CANON,IFACE_CANON,act,seq,1,len(res)+20)+ndr_response(0,res,mx)+res
            sock.sendto(payload,addr); print('release -> done',flush=True)
    elif opnum in (2,5) and ptype==0:
        # IODReadReqHeader 0x0009 -> IODReadResHeader 0x8009 + IM0 record
        t=struct.unpack('>H',blocks[0:2])[0]
        if t==0x0009:
            body=blocks[6:6+struct.unpack('>H',blocks[2:4])[0]-2]
            bseq=struct.unpack('>H',body[0:2])[0]; ar=body[2:18]
            api,=struct.unpack('>I',body[18:22]); slot,sub=struct.unpack('>HH',body[22:26])
            idx,=struct.unpack('>H',body[28:30])
            # IM0 record: block 0x0020, 54B body
            im0=struct.pack('>H',0x0493)+b'PNU-RT-200'.ljust(20)+b'SN12345'.ljust(16)+struct.pack('>H',0x0100)+b'V'+bytes([1,0,0])+struct.pack('>H',0)+struct.pack('>HH',0x0001,0x0001)+bytes([1,1])+struct.pack('>H',0x000E)
            rec=struct.pack('>HHBB',0x0020,len(im0)+2,1,0)+im0
            resbody=struct.pack('>H',bseq)+ar+struct.pack('>I',api)+struct.pack('>HH',slot,sub)+struct.pack('>H',0)+struct.pack('>H',idx)+struct.pack('>I',len(rec))+struct.pack('>HH',0,0)+b'\0'*20
            res=blk(0x8009,resbody)+rec
            payload=hdr(2,0x28,False,DEV_OBJ_CANON,IFACE_CANON,act,seq,opnum,len(res)+20)+ndr_response(0,res,mx)+res
            sock.sendto(payload,addr); print('read -> res',len(res),'B idx',hex(idx),flush=True)
    elif opnum==1 and ptype==0:
        t=struct.unpack('>H',blocks[0:2])[0]
        if t==0x0114:
            ar_uuid=blocks[8:24]; key=struct.unpack('>H',blocks[26:28])[0]
            res=ctrl_block(0x8114,ar_uuid,key,0x0008)
            payload=hdr(2,0x28,False,DEV_OBJ_CANON,IFACE_CANON,act,seq,1,len(res)+20)+ndr_response(0,res,mx)+res
            sock.sendto(payload,addr); print('release -> done',flush=True)
    elif opnum==4 and ptype==2:
        print('appready res received (Done)',flush=True)

DEV_OBJ_CANON=DEV_OBJ
s=socket.socket(socket.AF_INET,socket.SOCK_DGRAM); s.bind(('127.0.0.1',PORT))
print('fake dev on',PORT,flush=True)
while True:
    d,a=s.recvfrom(65535); handle(d,a,s)
