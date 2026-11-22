//! tplc-rt builtin FB: ocpp.
use crate::*;

// FB_Ocpp — OCPP-J client over tplc-ocpp (WebSocket + JSON-RPC).
// ---------------------------------------------------------------------------

// FB_Ocpp member offsets (see builtin_fb_layout("FB_Ocpp")). Strings
// are len-prefixed u8len+data.
pub(crate) const OC_OP: usize = 0;
pub(crate) const OC_HANDLE: usize = 4;
pub(crate) const OC_HOST_LEN: usize = 8;
pub(crate) const OC_HOST: usize = 9;
pub(crate) const OC_PORT: usize = 72;
pub(crate) const OC_PATH_LEN: usize = 76;
pub(crate) const OC_PATH: usize = 77;
pub(crate) const OC_OCPPID_LEN: usize = 140;
pub(crate) const OC_OCPPID: usize = 141;
pub(crate) const OC_MSGID_LEN: usize = 204;
pub(crate) const OC_MSGID: usize = 205;
pub(crate) const OC_ACT_LEN: usize = 268;
pub(crate) const OC_ACT: usize = 269;
pub(crate) const OC_CODE_LEN: usize = 332;
pub(crate) const OC_CODE: usize = 333;
pub(crate) const OC_DESC_LEN: usize = 396;
pub(crate) const OC_DESC: usize = 397;
pub(crate) const OC_PAYLOAD_LEN: usize = 524;
pub(crate) const OC_PAYLOAD: usize = 525;
pub(crate) const OC_RXACTION_LEN: usize = 780;
pub(crate) const OC_RXACTION: usize = 781;
pub(crate) const OC_RXPAYLOAD_LEN: usize = 844;
pub(crate) const OC_RXPAYLOAD: usize = 845;
pub(crate) const OC_RXMSGTYPE: usize = 1100;
pub(crate) const OC_STATE: usize = 1104;
pub(crate) const OC_RXTOT: usize = 1108;
pub(crate) const OC_TXTOT: usize = 1112;
pub(crate) const OC_LASTERR: usize = 1116;
pub(crate) const OC_STATUS: usize = 1120;
pub(crate) const OC_ERRID: usize = 1124;

// tplc-ocpp service vtable offsets (TplcOcppService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const OC_SVC_CONNECT: usize = 16;
pub(crate) const OC_SVC_DISCONNECT: usize = 24;
pub(crate) const OC_SVC_STATUS: usize = 32;
pub(crate) const OC_SVC_CALL: usize = 40;
pub(crate) const OC_SVC_RESULT: usize = 48;
pub(crate) const OC_SVC_CALLERROR: usize = 56;
pub(crate) const OC_SVC_RECEIVE: usize = 64;

/// service lookup for "tplc-ocpp" (9 chars)
pub(crate) unsafe fn ocpp_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-ocpp".as_ptr() as *const u8, 9) as *const u8
}

#[inline]
pub(crate) unsafe fn ocfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FOcppConnect = unsafe extern "C" fn(*const u8, *const u8, u32, *const u8, *const u8, *mut u32) -> i32;
pub(crate) type FOcppDisconnect = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FOcppStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FOcppCall = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, *const u8, u32) -> i32;
pub(crate) type FOcppResult = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, u32) -> i32;
pub(crate) type FOcppCallError = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, *const u8, *const u8, u32) -> i32;
pub(crate) type FOcppReceive = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;

/// `__tplc_rt_FB_Ocpp` — synchronous op dispatch over the tplc-ocpp
/// OCPP-J client. OP: 1=connect (HOST/PORT/PATH/OCPPID -> HANDLE),
/// 2=disconnect, 3=status (-> STATE/RXTOT/TXTOT/LASTERR), 4=call
/// (MSGID/ACTION/PAYLOAD), 5=result (MSGID/PAYLOAD), 6=callerror
/// (MSGID/CODE/DESC/PAYLOAD), 7=receive (-> RXACTION/RXPAYLOAD/
/// RXMSGTYPE or NOMSG). STATUS carries the service return code,
/// ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Ocpp(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(OC_OP));
    wi32(this.add(OC_STATUS), 0);
    wu32(this.add(OC_ERRID), 0);
    let svc = ocpp_service(ctx);
    if svc.is_null() {
        wi32(this.add(OC_STATUS), -2);
        wu32(this.add(OC_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(OC_STATUS), rc);
        wu32(this.add(OC_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut host = [0u8; 64];
            let mut path = [0u8; 64];
            let mut oid = [0u8; 64];
            uacstr(this, OC_HOST_LEN, OC_HOST, 63, host.as_mut_ptr());
            uacstr(this, OC_PATH_LEN, OC_PATH, 63, path.as_mut_ptr());
            uacstr(this, OC_OCPPID_LEN, OC_OCPPID, 63, oid.as_mut_ptr());
            let f: FOcppConnect = core::mem::transmute(ocfn(svc, OC_SVC_CONNECT));
            let mut h: u32 = 0;
            let rc = f(svc, host.as_ptr(), ru32(this.add(OC_PORT)),
                       path.as_ptr(), oid.as_ptr(), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(OC_HANDLE), h);
            }
        }
        2 => {
            let f: FOcppDisconnect = core::mem::transmute(ocfn(svc, OC_SVC_DISCONNECT));
            set_rc(f(svc, ru32(this.add(OC_HANDLE))));
        }
        3 => {
            // TplcOcppStatus (48B): state@0 rx_total@4 tx_total@8
            // last_error@12
            let mut st = [0u8; 48];
            let f: FOcppStatus = core::mem::transmute(ocfn(svc, OC_SVC_STATUS));
            let rc = f(svc, ru32(this.add(OC_HANDLE)), st.as_mut_ptr());
            wi32(this.add(OC_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(OC_STATE), rd(0));
                wu32(this.add(OC_RXTOT), rd(4) as u32);
                wu32(this.add(OC_TXTOT), rd(8) as u32);
                wi32(this.add(OC_LASTERR), rd(12));
            } else {
                wu32(this.add(OC_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let mut mid = [0u8; 64];
            let mut act = [0u8; 64];
            uacstr(this, OC_MSGID_LEN, OC_MSGID, 63, mid.as_mut_ptr());
            uacstr(this, OC_ACT_LEN, OC_ACT, 63, act.as_mut_ptr());
            let n = (*this.add(OC_PAYLOAD_LEN) as usize).min(255);
            let f: FOcppCall = core::mem::transmute(ocfn(svc, OC_SVC_CALL));
            let rc = f(svc, ru32(this.add(OC_HANDLE)),
                       mid.as_ptr(), act.as_ptr(),
                       this.add(OC_PAYLOAD), n as u32);
            set_rc(rc);
        }
        5 => {
            let mut mid = [0u8; 64];
            uacstr(this, OC_MSGID_LEN, OC_MSGID, 63, mid.as_mut_ptr());
            let n = (*this.add(OC_PAYLOAD_LEN) as usize).min(255);
            let f: FOcppResult = core::mem::transmute(ocfn(svc, OC_SVC_RESULT));
            let rc = f(svc, ru32(this.add(OC_HANDLE)),
                       mid.as_ptr(), this.add(OC_PAYLOAD), n as u32);
            set_rc(rc);
        }
        6 => {
            let mut mid = [0u8; 64];
            let mut code = [0u8; 64];
            let mut desc = [0u8; 128];
            uacstr(this, OC_MSGID_LEN, OC_MSGID, 63, mid.as_mut_ptr());
            uacstr(this, OC_CODE_LEN, OC_CODE, 63, code.as_mut_ptr());
            uacstr(this, OC_DESC_LEN, OC_DESC, 127, desc.as_mut_ptr());
            let n = (*this.add(OC_PAYLOAD_LEN) as usize).min(255);
            let f: FOcppCallError = core::mem::transmute(ocfn(svc, OC_SVC_CALLERROR));
            let rc = f(svc, ru32(this.add(OC_HANDLE)),
                       mid.as_ptr(), code.as_ptr(), desc.as_ptr(),
                       this.add(OC_PAYLOAD), n as u32);
            set_rc(rc);
        }
        7 => {
            // TplcOcppRx: action_cap@0 payload_cap@4 action_len@8
            // payload_len@12 msg_type@16 reserved@20 action@24 ptr
            // payload@32 ptr
            let mut rx = [0u8; 40];
            let mut wu = |off: usize, v: u32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), rx.as_mut_ptr().add(off), 4);
            };
            wu(0, 63);
            wu(4, 255);
            let ap = this.add(OC_RXACTION) as usize;
            let pp = this.add(OC_RXPAYLOAD) as usize;
            core::ptr::copy_nonoverlapping(
                ap.to_le_bytes().as_ptr(), rx.as_mut_ptr().add(24), 8);
            core::ptr::copy_nonoverlapping(
                pp.to_le_bytes().as_ptr(), rx.as_mut_ptr().add(32), 8);
            let f: FOcppReceive = core::mem::transmute(ocfn(svc, OC_SVC_RECEIVE));
            let rc = f(svc, ru32(this.add(OC_HANDLE)), rx.as_mut_ptr());
            wi32(this.add(OC_STATUS), rc);
            wu32(this.add(OC_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let rd = |off: usize| -> u32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(rx.as_ptr().add(off), b.as_mut_ptr(), 4);
                    u32::from_le_bytes(b)
                };
                let al = rd(8).min(63) as usize;
                *this.add(OC_RXACTION_LEN) = al as u8;
                *this.add(OC_RXACTION + al) = 0;
                let pl = rd(12).min(255) as usize;
                *this.add(OC_RXPAYLOAD_LEN) = pl as u8;
                wu32(this.add(OC_RXMSGTYPE), rd(16));
            }
        }
        _ => {
            wi32(this.add(OC_STATUS), -1);
            wu32(this.add(OC_ERRID), 1);
        }
    }
    0
}
