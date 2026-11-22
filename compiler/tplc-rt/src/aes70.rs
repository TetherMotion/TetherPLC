//! tplc-rt builtin FB: aes70.
use crate::*;

// ---------------------------------------------------------------------------
// FB_Aes70 — OCA/AES70 audio-control client over tplc-aes70 (OSCP TCP).
// ---------------------------------------------------------------------------

// FB_Aes70 member offsets (see builtin_fb_layout("FB_Aes70")). Strings
// are len-prefixed u8len+data.
pub(crate) const A7_OP: usize = 0;
pub(crate) const A7_HANDLE: usize = 4;
pub(crate) const A7_HOST_LEN: usize = 8;
pub(crate) const A7_HOST: usize = 9;
pub(crate) const A7_PORT: usize = 72;
pub(crate) const A7_TARGET: usize = 76;
pub(crate) const A7_METH: usize = 80;
pub(crate) const A7_PARAMS_LEN: usize = 84;
pub(crate) const A7_PARAMS: usize = 85;
pub(crate) const A7_HANDLE2: usize = 340;
pub(crate) const A7_STATUSCODE: usize = 344;
pub(crate) const A7_RXPAYLOAD_LEN: usize = 348;
pub(crate) const A7_RXPAYLOAD: usize = 349;
pub(crate) const A7_STATE: usize = 604;
pub(crate) const A7_RXTOT: usize = 608;
pub(crate) const A7_TXTOT: usize = 612;
pub(crate) const A7_LASTERR: usize = 616;
pub(crate) const A7_STATUS: usize = 620;
pub(crate) const A7_ERRID: usize = 624;

// tplc-aes70 service vtable offsets (TplcAes70Service):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const A7_SVC_CONNECT: usize = 16;
pub(crate) const A7_SVC_DISCONNECT: usize = 24;
pub(crate) const A7_SVC_STATUS: usize = 32;
pub(crate) const A7_SVC_CALL: usize = 40;
pub(crate) const A7_SVC_KEEPALIVE: usize = 48;
pub(crate) const A7_SVC_RECEIVE: usize = 56;

/// service lookup for "tplc-aes70" (10 chars)
pub(crate) unsafe fn aes70_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-aes70".as_ptr() as *const u8, 10) as *const u8
}

#[inline]
pub(crate) unsafe fn a7fn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FAes70Connect = unsafe extern "C" fn(*const u8, *const u8, u32, *mut u32) -> i32;
pub(crate) type FAes70Disconnect = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FAes70Status = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FAes70Call = unsafe extern "C" fn(*const u8, u32, u32, u32, *const u8, u32, *mut u32) -> i32;
pub(crate) type FAes70Keepalive = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FAes70Receive = unsafe extern "C" fn(*const u8, u32, *mut u32, *mut u32, *mut u8, u32) -> i32;

/// `__tplc_rt_FB_Aes70` — synchronous op dispatch over the tplc-aes70
/// OCA client. OP: 1=connect (HOST/PORT -> HANDLE), 2=disconnect,
/// 3=status (-> STATE/RXTOT/TXTOT/LASTERR), 4=call (TARGET/METHOD/
/// PARAMS -> HANDLE2), 5=keepalive, 6=receive (-> HANDLE2/STATUSCODE/
/// RXPAYLOAD or NOMSG). STATUS carries the service return code,
/// ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Aes70(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(A7_OP));
    wi32(this.add(A7_STATUS), 0);
    wu32(this.add(A7_ERRID), 0);
    let svc = aes70_service(ctx);
    if svc.is_null() {
        wi32(this.add(A7_STATUS), -2);
        wu32(this.add(A7_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(A7_STATUS), rc);
        wu32(this.add(A7_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 64];
            uacstr(this, A7_HOST_LEN, A7_HOST, 63, buf.as_mut_ptr());
            let f: FAes70Connect = core::mem::transmute(a7fn(svc, A7_SVC_CONNECT));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), ru32(this.add(A7_PORT)), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(A7_HANDLE), h);
            }
        }
        2 => {
            let f: FAes70Disconnect = core::mem::transmute(a7fn(svc, A7_SVC_DISCONNECT));
            set_rc(f(svc, ru32(this.add(A7_HANDLE))));
        }
        3 => {
            // TplcAes70Status (48B): state@0 rx_total@4 tx_total@8
            // last_error@12
            let mut st = [0u8; 48];
            let f: FAes70Status = core::mem::transmute(a7fn(svc, A7_SVC_STATUS));
            let rc = f(svc, ru32(this.add(A7_HANDLE)), st.as_mut_ptr());
            wi32(this.add(A7_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(A7_STATE), rd(0));
                wu32(this.add(A7_RXTOT), rd(4) as u32);
                wu32(this.add(A7_TXTOT), rd(8) as u32);
                wi32(this.add(A7_LASTERR), rd(12));
            } else {
                wu32(this.add(A7_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let n = (*this.add(A7_PARAMS_LEN) as usize).min(255);
            let f: FAes70Call = core::mem::transmute(a7fn(svc, A7_SVC_CALL));
            let mut h2: u32 = 0;
            let rc = f(svc, ru32(this.add(A7_HANDLE)),
                       ru32(this.add(A7_TARGET)), ru32(this.add(A7_METH)),
                       this.add(A7_PARAMS), n as u32, &mut h2);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(A7_HANDLE2), h2);
            }
        }
        5 => {
            let f: FAes70Keepalive = core::mem::transmute(a7fn(svc, A7_SVC_KEEPALIVE));
            set_rc(f(svc, ru32(this.add(A7_HANDLE))));
        }
        6 => {
            let mut out = [0u8; 255];
            let mut h2: u32 = 0;
            let mut sc: u32 = 0;
            let f: FAes70Receive = core::mem::transmute(a7fn(svc, A7_SVC_RECEIVE));
            let rc = f(svc, ru32(this.add(A7_HANDLE)),
                       &mut h2, &mut sc, out.as_mut_ptr(), 255);
            wi32(this.add(A7_STATUS), rc);
            wu32(this.add(A7_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                wu32(this.add(A7_HANDLE2), h2);
                wu32(this.add(A7_STATUSCODE), sc);
                let n = (rc as usize).min(255);
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(A7_RXPAYLOAD), n);
                *this.add(A7_RXPAYLOAD_LEN) = n as u8;
            }
        }
        _ => {
            wi32(this.add(A7_STATUS), -1);
            wu32(this.add(A7_ERRID), 1);
        }
    }
    0
}
