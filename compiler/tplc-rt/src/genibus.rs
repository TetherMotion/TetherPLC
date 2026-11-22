//! tplc-rt builtin FB: genibus.
use crate::*;

// ---- FB_Genibus builtin — GENIbus telegrams (tplc-genibus) ----
// Member offsets (see builtin_fb_layout("FB_Genibus")). Strings
// are len-prefixed.
pub(crate) const GEN_OP: usize = 0;
pub(crate) const GEN_HANDLE: usize = 4;
pub(crate) const GEN_PATH_LEN: usize = 8;
pub(crate) const GEN_PATH: usize = 9;
pub(crate) const GEN_DA: usize = 136;
pub(crate) const GEN_SA: usize = 140;
pub(crate) const GEN_APDU_LEN: usize = 144;
pub(crate) const GEN_APDU: usize = 145;
pub(crate) const GEN_RDA: usize = 396;
pub(crate) const GEN_RSA: usize = 400;
pub(crate) const GEN_RXAPDU_LEN: usize = 404;
pub(crate) const GEN_RXAPDU: usize = 405;
pub(crate) const GEN_STATE: usize = 656;
pub(crate) const GEN_TXTOT: usize = 660;
pub(crate) const GEN_RXTOT: usize = 664;
pub(crate) const GEN_LASTERR: usize = 668;
pub(crate) const GEN_STATUS: usize = 672;
pub(crate) const GEN_ERRID: usize = 676;
pub(crate) const GEN_RXB0: usize = 680;

// TplcGenibusService vtable (tplc_genibus.h): size@0 abi@4 host@8
pub(crate) const GEN_SVC_OPEN: usize = 16;
pub(crate) const GEN_SVC_CLOSE: usize = 24;
pub(crate) const GEN_SVC_STATUS: usize = 32;
pub(crate) const GEN_SVC_SEND: usize = 40;
pub(crate) const GEN_SVC_RECEIVE: usize = 48;

pub(crate) unsafe fn genibus_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-genibus".as_ptr() as *const u8, 12) as *const u8
}

pub(crate) type FGenOpen = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FGenClose = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FGenStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FGenSend = unsafe extern "C" fn(*const u8, u32, u32, u32, *const u8, u32) -> i32;
pub(crate) type FGenReceive = unsafe extern "C" fn(*const u8, u32, *mut u32, *mut u32, *mut u8, u32) -> i32;

/// `__tplc_rt_FB_Genibus` — synchronous op dispatch over the
/// tplc-genibus service.  OP: 1=open (PATH -> HANDLE), 2=close,
/// 3=status (-> STATE/TXTOT/RXTOT/LASTERR), 4=send (DA/SA/APDU),
/// 5=receive (-> RDA/RSA/RXAPDU/RXB0 or NOMSG).  The serial port
/// is only opened when the surface is used.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Genibus(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(GEN_OP));
    wi32(this.add(GEN_STATUS), 0);
    wu32(this.add(GEN_ERRID), 0);
    let svc = genibus_service(ctx);
    if svc.is_null() {
        wi32(this.add(GEN_STATUS), -2);
        wu32(this.add(GEN_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(GEN_STATUS), rc);
        wu32(this.add(GEN_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 128];
            uacstr(this, GEN_PATH_LEN, GEN_PATH, 127, buf.as_mut_ptr());
            let f: FGenOpen = core::mem::transmute(mbusfn(svc, GEN_SVC_OPEN));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(GEN_HANDLE), h);
            }
        }
        2 => {
            let f: FGenClose = core::mem::transmute(mbusfn(svc, GEN_SVC_CLOSE));
            set_rc(f(svc, ru32(this.add(GEN_HANDLE))));
        }
        3 => {
            let mut st = [0u8; 48];
            let f: FGenStatus = core::mem::transmute(mbusfn(svc, GEN_SVC_STATUS));
            let rc = f(svc, ru32(this.add(GEN_HANDLE)), st.as_mut_ptr());
            wi32(this.add(GEN_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(GEN_STATE), rd(0));
                wu32(this.add(GEN_TXTOT), rd(4) as u32);
                wu32(this.add(GEN_RXTOT), rd(8) as u32);
                wi32(this.add(GEN_LASTERR), rd(12));
            } else {
                wu32(this.add(GEN_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let n = (*this.add(GEN_APDU_LEN) as usize).min(250);
            let f: FGenSend = core::mem::transmute(mbusfn(svc, GEN_SVC_SEND));
            set_rc(f(svc, ru32(this.add(GEN_HANDLE)),
                     ru32(this.add(GEN_DA)), ru32(this.add(GEN_SA)),
                     this.add(GEN_APDU), n as u32));
        }
        5 => {
            let mut out = [0u8; 250];
            let mut da: u32 = 0;
            let mut sa: u32 = 0;
            let f: FGenReceive = core::mem::transmute(mbusfn(svc, GEN_SVC_RECEIVE));
            let rc = f(svc, ru32(this.add(GEN_HANDLE)),
                       &mut da, &mut sa, out.as_mut_ptr(), 250);
            wi32(this.add(GEN_STATUS), rc);
            wu32(this.add(GEN_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                wu32(this.add(GEN_RDA), da);
                wu32(this.add(GEN_RSA), sa);
                let n = (rc as usize).min(250);
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(GEN_RXAPDU), n);
                *this.add(GEN_RXAPDU_LEN) = n as u8;
                wu32(this.add(GEN_RXB0), if n > 0 { out[0] as u32 } else { 0 });
            }
        }
        _ => {
            wi32(this.add(GEN_STATUS), -1);
            wu32(this.add(GEN_ERRID), 1);
        }
    }
    0
}
