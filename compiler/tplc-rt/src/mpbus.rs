//! tplc-rt builtin FB: mpbus.
use crate::*;

// ---- FB_Mpbus builtin — MP-Bus master (tplc-mpbus) ----
// Member offsets (see builtin_fb_layout("FB_Mpbus")) — same
// field plan as FB_Genibus plus RCMD.
pub(crate) const MPB_OP: usize = 0;
pub(crate) const MPB_HANDLE: usize = 4;
pub(crate) const MPB_PATH_LEN: usize = 8;
pub(crate) const MPB_PATH: usize = 9;
pub(crate) const MPB_DA: usize = 136;
pub(crate) const MPB_CMD: usize = 140;
pub(crate) const MPB_APDU_LEN: usize = 144;
pub(crate) const MPB_APDU: usize = 145;
pub(crate) const MPB_RDA: usize = 396;
pub(crate) const MPB_RCMD: usize = 400;
pub(crate) const MPB_RXAPDU_LEN: usize = 404;
pub(crate) const MPB_RXAPDU: usize = 405;
pub(crate) const MPB_STATE: usize = 656;
pub(crate) const MPB_TXTOT: usize = 660;
pub(crate) const MPB_RXTOT: usize = 664;
pub(crate) const MPB_LASTERR: usize = 668;
pub(crate) const MPB_STATUS: usize = 672;
pub(crate) const MPB_ERRID: usize = 676;
pub(crate) const MPB_RXB0: usize = 680;

// TplcMpbusService vtable (tplc_mpbus.h): size@0 abi@4 host@8
pub(crate) const MPB_SVC_OPEN: usize = 16;
pub(crate) const MPB_SVC_CLOSE: usize = 24;
pub(crate) const MPB_SVC_STATUS: usize = 32;
pub(crate) const MPB_SVC_TRANSACT: usize = 40;
pub(crate) const MPB_SVC_RECEIVE: usize = 48;

pub(crate) unsafe fn mpbus_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-mpbus".as_ptr() as *const u8, 10) as *const u8
}

pub(crate) type FMpOpen = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FMpClose = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FMpStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FMpTransact = unsafe extern "C" fn(*const u8, u32, u32, u32, *const u8, u32, *mut u32, *mut u32, *mut u8, u32) -> i32;
pub(crate) type FMpReceive = unsafe extern "C" fn(*const u8, u32, *mut u32, *mut u32, *mut u8, u32) -> i32;

/// `__tplc_rt_FB_Mpbus` — synchronous op dispatch over the
/// tplc-mpbus service.  OP: 1=open (PATH -> HANDLE), 2=close,
/// 3=status (-> STATE/TXTOT/RXTOT/LASTERR), 4=transact
/// (DA/CMD/APDU -> RDA/RCMD/RXAPDU/RXB0 or NOMSG), 5=receive.
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Mpbus(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(MPB_OP));
    wi32(this.add(MPB_STATUS), 0);
    wu32(this.add(MPB_ERRID), 0);
    let svc = mpbus_service(ctx);
    if svc.is_null() {
        wi32(this.add(MPB_STATUS), -2);
        wu32(this.add(MPB_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(MPB_STATUS), rc);
        wu32(this.add(MPB_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 128];
            uacstr(this, MPB_PATH_LEN, MPB_PATH, 127, buf.as_mut_ptr());
            let f: FMpOpen = core::mem::transmute(mbusfn(svc, MPB_SVC_OPEN));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(MPB_HANDLE), h);
            }
        }
        2 => {
            let f: FMpClose = core::mem::transmute(mbusfn(svc, MPB_SVC_CLOSE));
            set_rc(f(svc, ru32(this.add(MPB_HANDLE))));
        }
        3 => {
            let mut st = [0u8; 48];
            let f: FMpStatus = core::mem::transmute(mbusfn(svc, MPB_SVC_STATUS));
            let rc = f(svc, ru32(this.add(MPB_HANDLE)), st.as_mut_ptr());
            wi32(this.add(MPB_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(MPB_STATE), rd(0));
                wu32(this.add(MPB_TXTOT), rd(4) as u32);
                wu32(this.add(MPB_RXTOT), rd(8) as u32);
                wi32(this.add(MPB_LASTERR), rd(12));
            } else {
                wu32(this.add(MPB_ERRID), (-rc).max(0) as u32);
            }
        }
        4 | 5 => {
            let mut out = [0u8; 250];
            let mut da: u32 = 0;
            let mut rc2: u32 = 0;
            let rc: i32 = if op == 4 {
                let n = (*this.add(MPB_APDU_LEN) as usize).min(250);
                let f: FMpTransact = core::mem::transmute(mbusfn(svc, MPB_SVC_TRANSACT));
                f(svc, ru32(this.add(MPB_HANDLE)),
                  ru32(this.add(MPB_DA)), ru32(this.add(MPB_CMD)),
                  this.add(MPB_APDU), n as u32,
                  &mut da, &mut rc2, out.as_mut_ptr(), 250)
            } else {
                let f: FMpReceive = core::mem::transmute(mbusfn(svc, MPB_SVC_RECEIVE));
                f(svc, ru32(this.add(MPB_HANDLE)),
                  &mut da, &mut rc2, out.as_mut_ptr(), 250)
            };
            wi32(this.add(MPB_STATUS), rc);
            wu32(this.add(MPB_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                wu32(this.add(MPB_RDA), da);
                wu32(this.add(MPB_RCMD), rc2);
                let n = (rc as usize).min(250);
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(MPB_RXAPDU), n);
                *this.add(MPB_RXAPDU_LEN) = n as u8;
                wu32(this.add(MPB_RXB0), if n > 0 { out[0] as u32 } else { 0 });
            }
        }
        _ => {
            wi32(this.add(MPB_STATUS), -1);
            wu32(this.add(MPB_ERRID), 1);
        }
    }
    0
}
