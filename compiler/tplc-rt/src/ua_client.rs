//! tplc-rt builtin FB: ua client.
use crate::*;

// ---------------------------------------------------------------------------
// FB_UaClient — OPC UA client over the tplc-opcua service.
// ---------------------------------------------------------------------------

// FB_UaClient member offsets (see builtin_fb_layout("FB_UaClient")).
// Strings are len-prefixed: URL u8len@8+data@9 (cap127), NODE
// u8len@136+data@137 (cap255), SVAL u8len@392+data@393 (cap255).
pub(crate) const UA_OP: usize = 0;
pub(crate) const UA_HANDLE: usize = 4;
pub(crate) const UA_URL_LEN: usize = 8;
pub(crate) const UA_URL: usize = 9;
pub(crate) const UA_NODE_LEN: usize = 136;
pub(crate) const UA_NODE: usize = 137;
pub(crate) const UA_SVAL_LEN: usize = 392;
pub(crate) const UA_SVAL: usize = 393;
pub(crate) const UA_VAL: usize = 648;
pub(crate) const UA_TYPE: usize = 656;
pub(crate) const UA_STATE: usize = 660;
pub(crate) const UA_IDX: usize = 664;
pub(crate) const UA_STATUS: usize = 668;
pub(crate) const UA_ERRID: usize = 672;

// tplc-opcua service vtable offsets (TplcOpcUaService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const UA_SVC_CONNECT: usize = 16;
pub(crate) const UA_SVC_DISCONNECT: usize = 24;
pub(crate) const UA_SVC_STATUS: usize = 32;
pub(crate) const UA_SVC_READ: usize = 40;
pub(crate) const UA_SVC_WRITE: usize = 48;
pub(crate) const UA_SVC_NS_INDEX: usize = 56;

// TplcOpcUaValue (248 bytes): type@0 i32, reserved@4, union@8
// (i64/u64/f64), str@16..248.
pub(crate) const UAV_TYPE: usize = 0;
pub(crate) const UAV_F64: usize = 8;
pub(crate) const UAV_I64: usize = 8;
pub(crate) const UAV_U64: usize = 8;
pub(crate) const UAV_STR: usize = 16;

/// service lookup for "tplc-opcua" (10 chars)
pub(crate) unsafe fn opcua_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-opcua".as_ptr() as *const u8, 10) as *const u8
}

#[inline]
pub(crate) unsafe fn uafn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FUaConnect = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FUaDisconnect = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FUaStatus = unsafe extern "C" fn(*const u8, u32, *mut i32) -> i32;
pub(crate) type FUaRead = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u8) -> i32;
pub(crate) type FUaWrite = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8) -> i32;
pub(crate) type FUaNsIndex = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;

/// Copy a len-prefixed ST string (u8 len + data) into a NUL-terminated
/// stack buffer; returns the buffer with `n` set to the length.
pub(crate) unsafe fn uacstr(this: *const u8, len_off: usize, data_off: usize,
                 cap: usize, buf: *mut u8) -> u32 {
    let n = (*this.add(len_off) as usize).min(cap);
    core::ptr::copy_nonoverlapping(this.add(data_off), buf, n);
    *buf.add(n) = 0;
    n as u32
}

/// `__tplc_rt_FB_UaClient` — synchronous op dispatch over the
/// tplc-opcua OPC UA client. OP: 1=connect (URL -> HANDLE),
/// 2=disconnect, 3=status (HANDLE -> STATE), 4=read (NODE -> TYPE +
/// VAL or SVAL), 5=write (NODE + TYPE + VAL/SVAL), 6=namespace index
/// lookup (URL used as the URI -> IDX). STATUS carries the service
/// return code, ERRID its positive form. Numeric kinds widen into the
/// LREAL VAL (precision above 2^53 is lost — an honest limit).
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_UaClient(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(UA_OP));
    wi32(this.add(UA_STATUS), 0);
    wu32(this.add(UA_ERRID), 0);
    let svc = opcua_service(ctx);
    if svc.is_null() {
        wi32(this.add(UA_STATUS), -2);
        wu32(this.add(UA_ERRID), 99);
        return 0;
    }
    match op {
        1 => {
            let mut buf = [0u8; 128];
            uacstr(this, UA_URL_LEN, UA_URL, 127, buf.as_mut_ptr());
            let f: FUaConnect = core::mem::transmute(uafn(svc, UA_SVC_CONNECT));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), &mut h);
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                wu32(this.add(UA_HANDLE), h);
            }
        }
        2 => {
            let f: FUaDisconnect = core::mem::transmute(uafn(svc, UA_SVC_DISCONNECT));
            let rc = f(svc, ru32(this.add(UA_HANDLE)));
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
        }
        3 => {
            let f: FUaStatus = core::mem::transmute(uafn(svc, UA_SVC_STATUS));
            let mut st: i32 = 0;
            let rc = f(svc, ru32(this.add(UA_HANDLE)), &mut st);
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                wi32(this.add(UA_STATE), st);
            }
        }
        4 => {
            let mut buf = [0u8; 256];
            uacstr(this, UA_NODE_LEN, UA_NODE, 255, buf.as_mut_ptr());
            let mut v = [0u8; 248];
            let f: FUaRead = core::mem::transmute(uafn(svc, UA_SVC_READ));
            let rc = f(svc, ru32(this.add(UA_HANDLE)),
                       buf.as_ptr(), v.as_mut_ptr());
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let vt = ri32(v.as_ptr().add(UAV_TYPE));
                wu32(this.add(UA_TYPE), vt as u32);
                match vt {
                    1 => { // BOOL
                        let b = (v.as_ptr().add(UAV_U64) as *const u64).read_unaligned();
                        core::ptr::write_unaligned(this.add(UA_VAL) as *mut f64, b as f64);
                    }
                    2 => { // I64
                        let i = (v.as_ptr().add(UAV_I64) as *const i64).read_unaligned();
                        core::ptr::write_unaligned(this.add(UA_VAL) as *mut f64, i as f64);
                    }
                    3 => { // U64
                        let u = (v.as_ptr().add(UAV_U64) as *const u64).read_unaligned();
                        core::ptr::write_unaligned(this.add(UA_VAL) as *mut f64, u as f64);
                    }
                    4 => { // F64
                        let d = (v.as_ptr().add(UAV_F64) as *const f64).read_unaligned();
                        core::ptr::write_unaligned(this.add(UA_VAL) as *mut f64, d);
                    }
                    5 => { // STRING
                        let mut n2 = 0usize;
                        while n2 < 231 && *v.as_ptr().add(UAV_STR + n2) != 0 {
                            n2 += 1;
                        }
                        let n = n2.min(255);
                        core::ptr::copy_nonoverlapping(
                            v.as_ptr().add(UAV_STR),
                            this.add(UA_SVAL),
                            n,
                        );
                        *this.add(UA_SVAL_LEN) = n as u8;
                        *this.add(UA_SVAL + n) = 0;
                    }
                    _ => {}
                }
            }
        }
        5 => {
            let mut buf = [0u8; 256];
            uacstr(this, UA_NODE_LEN, UA_NODE, 255, buf.as_mut_ptr());
            let mut v = [0u8; 248];
            let vt = ru32(this.add(UA_TYPE)) as i32;
            wi32(v.as_mut_ptr().add(UAV_TYPE), vt);
            let d = core::ptr::read_unaligned(this.add(UA_VAL) as *const f64);
            match vt {
                1 | 3 => core::ptr::write_unaligned(
                    v.as_mut_ptr().add(UAV_U64) as *mut u64,
                    d as u64,
                ),
                2 => core::ptr::write_unaligned(
                    v.as_mut_ptr().add(UAV_I64) as *mut i64,
                    d as i64,
                ),
                _ => core::ptr::write_unaligned(
                    v.as_mut_ptr().add(UAV_F64) as *mut f64,
                    d,
                ),
            }
            if vt == 5 {
                let sn = (*this.add(UA_SVAL_LEN) as usize).min(231);
                core::ptr::copy_nonoverlapping(
                    this.add(UA_SVAL),
                    v.as_mut_ptr().add(UAV_STR),
                    sn,
                );
                *v.as_mut_ptr().add(UAV_STR + sn) = 0;
            }
            let f: FUaWrite = core::mem::transmute(uafn(svc, UA_SVC_WRITE));
            let rc = f(svc, ru32(this.add(UA_HANDLE)),
                       buf.as_ptr(), v.as_ptr());
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
        }
        6 => {
            let mut buf = [0u8; 128];
            uacstr(this, UA_URL_LEN, UA_URL, 127, buf.as_mut_ptr());
            let f: FUaNsIndex = core::mem::transmute(uafn(svc, UA_SVC_NS_INDEX));
            let mut idx: u32 = 0;
            let rc = f(svc, ru32(this.add(UA_HANDLE)),
                       buf.as_ptr(), &mut idx);
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                wu32(this.add(UA_IDX), idx);
            }
        }
        _ => {
            wi32(this.add(UA_STATUS), -1);
            wu32(this.add(UA_ERRID), 1);
        }
    }
    0
}
