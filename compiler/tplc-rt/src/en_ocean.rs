//! tplc-rt builtin FB: en ocean.
use crate::*;

// ---- FB_EnOcean builtin — EnOcean ESP3 transceiver (tplc-enocean) ----
// Member offsets (see builtin_fb_layout("FB_EnOcean")). Strings are
// len-prefixed.
pub(crate) const ENO_OP: usize = 0;
pub(crate) const ENO_HANDLE: usize = 4;
pub(crate) const ENO_PATH_LEN: usize = 8;
pub(crate) const ENO_PATH: usize = 9;
pub(crate) const ENO_ORG: usize = 136;
pub(crate) const ENO_SENDER: usize = 140;
pub(crate) const ENO_DEST: usize = 144;
pub(crate) const ENO_DATA_LEN: usize = 148;
pub(crate) const ENO_DATA: usize = 149;
pub(crate) const ENO_PTYPE: usize = 404;
pub(crate) const ENO_RORG: usize = 408;
pub(crate) const ENO_RSENDER: usize = 412;
pub(crate) const ENO_RXPAYLOAD_LEN: usize = 416;
pub(crate) const ENO_RXPAYLOAD: usize = 417;
pub(crate) const ENO_STATE: usize = 672;
pub(crate) const ENO_TXTOT: usize = 676;
pub(crate) const ENO_RXTOT: usize = 680;
pub(crate) const ENO_LASTERR: usize = 684;
pub(crate) const ENO_STATUS: usize = 688;
pub(crate) const ENO_ERRID: usize = 692;
pub(crate) const ENO_RXB0: usize = 696;

// TplcEnoceanService vtable (tplc_enocean.h): size@0 abi@4 host@8
pub(crate) const ENO_SVC_OPEN: usize = 16;
pub(crate) const ENO_SVC_CLOSE: usize = 24;
pub(crate) const ENO_SVC_STATUS: usize = 32;
pub(crate) const ENO_SVC_SEND: usize = 40;
pub(crate) const ENO_SVC_RECEIVE: usize = 48;

pub(crate) unsafe fn enocean_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-enocean".as_ptr() as *const u8, 12) as *const u8
}

pub(crate) type FEnoOpen = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FEnoClose = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FEnoStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FEnoSend = unsafe extern "C" fn(*const u8, u32, u32, u32, u32, *const u8, u32) -> i32;
pub(crate) type FEnoReceive = unsafe extern "C" fn(*const u8, u32, *mut u32, *mut u32, *mut u32, *mut u8, u32) -> i32;

/// `__tplc_rt_FB_EnOcean` — synchronous op dispatch over the
/// tplc-enocean ESP3 transceiver. OP: 1=open (PATH -> HANDLE),
/// 2=close, 3=status (-> STATE/TXTOT/RXTOT/LASTERR),
/// 4=send_radio (ORG/SENDER/DEST/DATA), 5=receive
/// (-> PTYPE/RORG/RSENDER/RXPAYLOAD or NOMSG). STATUS carries the
/// service return code, ERRID its positive form.  The serial port
/// is only opened when the surface is used.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_EnOcean(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(ENO_OP));
    wi32(this.add(ENO_STATUS), 0);
    wu32(this.add(ENO_ERRID), 0);
    let svc = enocean_service(ctx);
    if svc.is_null() {
        wi32(this.add(ENO_STATUS), -2);
        wu32(this.add(ENO_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(ENO_STATUS), rc);
        wu32(this.add(ENO_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 128];
            uacstr(this, ENO_PATH_LEN, ENO_PATH, 127, buf.as_mut_ptr());
            let f: FEnoOpen = core::mem::transmute(mbusfn(svc, ENO_SVC_OPEN));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(ENO_HANDLE), h);
            }
        }
        2 => {
            let f: FEnoClose = core::mem::transmute(mbusfn(svc, ENO_SVC_CLOSE));
            set_rc(f(svc, ru32(this.add(ENO_HANDLE))));
        }
        3 => {
            let mut st = [0u8; 48];
            let f: FEnoStatus = core::mem::transmute(mbusfn(svc, ENO_SVC_STATUS));
            let rc = f(svc, ru32(this.add(ENO_HANDLE)), st.as_mut_ptr());
            wi32(this.add(ENO_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(ENO_STATE), rd(0));
                wu32(this.add(ENO_TXTOT), rd(4) as u32);
                wu32(this.add(ENO_RXTOT), rd(8) as u32);
                wi32(this.add(ENO_LASTERR), rd(12));
            } else {
                wu32(this.add(ENO_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let n = (*this.add(ENO_DATA_LEN) as usize).min(255);
            let f: FEnoSend = core::mem::transmute(mbusfn(svc, ENO_SVC_SEND));
            set_rc(f(svc, ru32(this.add(ENO_HANDLE)),
                     ru32(this.add(ENO_ORG)), ru32(this.add(ENO_SENDER)),
                     ru32(this.add(ENO_DEST)),
                     this.add(ENO_DATA), n as u32));
        }
        5 => {
            let mut out = [0u8; 255];
            let mut ptype: u32 = 0;
            let mut org: u32 = 0;
            let mut sender: u32 = 0;
            let f: FEnoReceive = core::mem::transmute(mbusfn(svc, ENO_SVC_RECEIVE));
            let rc = f(svc, ru32(this.add(ENO_HANDLE)),
                       &mut ptype, &mut org, &mut sender,
                       out.as_mut_ptr(), 255);
            wi32(this.add(ENO_STATUS), rc);
            wu32(this.add(ENO_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                wu32(this.add(ENO_PTYPE), ptype);
                wu32(this.add(ENO_RORG), org);
                wu32(this.add(ENO_RSENDER), sender);
                let n = (rc as usize).min(255);
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(ENO_RXPAYLOAD), n);
                *this.add(ENO_RXPAYLOAD_LEN) = n as u8;
                wu32(this.add(ENO_RXB0), if n > 0 { out[0] as u32 } else { 0 });
            }
        }
        _ => {
            wi32(this.add(ENO_STATUS), -1);
            wu32(this.add(ENO_ERRID), 1);
        }
    }
    0
}
