//! tplc-rt builtin FB: mbus.
use crate::*;

// ---------------------------------------------------------------------------
// FB_Mbus — wired M-Bus (EN 13757) master over tplc-mbus (opt-in serial).
// ---------------------------------------------------------------------------

// FB_Mbus member offsets (see builtin_fb_layout("FB_Mbus")). Strings
// are len-prefixed u8len+data.
pub(crate) const MBUS_OP: usize = 0;
pub(crate) const MBUS_HANDLE: usize = 4;
pub(crate) const MBUS_PATH_LEN: usize = 8;
pub(crate) const MBUS_PATH: usize = 9;
pub(crate) const MBUS_CONTROL: usize = 136;
pub(crate) const MBUS_ADDRESS: usize = 140;
pub(crate) const MBUS_CI: usize = 144;
pub(crate) const MBUS_DATA_LEN: usize = 148;
pub(crate) const MBUS_DATA: usize = 149;
pub(crate) const MBUS_KIND: usize = 404;
pub(crate) const MBUS_RXPAYLOAD_LEN: usize = 408;
pub(crate) const MBUS_RXPAYLOAD: usize = 409;
pub(crate) const MBUS_STATE: usize = 664;
pub(crate) const MBUS_TXTOT: usize = 668;
pub(crate) const MBUS_RXTOT: usize = 672;
pub(crate) const MBUS_LASTERR: usize = 676;
pub(crate) const MBUS_STATUS: usize = 680;
pub(crate) const MBUS_ERRID: usize = 684;

// tplc-mbus service vtable offsets (TplcMbusService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const MB_SVC_OPEN: usize = 16;
pub(crate) const MB_SVC_CLOSE: usize = 24;
pub(crate) const MB_SVC_STATUS: usize = 32;
pub(crate) const MB_SVC_SHORT: usize = 40;
pub(crate) const MB_SVC_LONG: usize = 48;
pub(crate) const MB_SVC_RECEIVE: usize = 56;

/// service lookup for "tplc-mbus" (9 chars)
pub(crate) unsafe fn mbus_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-mbus".as_ptr() as *const u8, 9) as *const u8
}

#[inline]
pub(crate) unsafe fn mbusfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FMbusOpen = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FMbusClose = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FMbusStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FMbusShort = unsafe extern "C" fn(*const u8, u32, u32, u32) -> i32;
pub(crate) type FMbusLong = unsafe extern "C" fn(*const u8, u32, u32, u32, u32, *const u8, u32) -> i32;
pub(crate) type FMbusReceive = unsafe extern "C" fn(*const u8, u32, *mut u32, *mut u8, u32) -> i32;

/// `__tplc_rt_FB_Mbus` — synchronous op dispatch over the tplc-mbus
/// wired M-Bus master. OP: 1=open (PATH -> HANDLE), 2=close,
/// 3=status (-> STATE/TXTOT/RXTOT/LASTERR), 4=short frame
/// (CONTROL/ADDRESS), 5=long frame (CONTROL/ADDRESS/CI/DATA),
/// 6=receive (-> KIND/RXPAYLOAD or NOMSG). STATUS carries the
/// service return code, ERRID its positive form.  The serial port
/// is only opened when the surface is used.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Mbus(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(MBUS_OP));
    wi32(this.add(MBUS_STATUS), 0);
    wu32(this.add(MBUS_ERRID), 0);
    let svc = mbus_service(ctx);
    if svc.is_null() {
        wi32(this.add(MBUS_STATUS), -2);
        wu32(this.add(MBUS_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(MBUS_STATUS), rc);
        wu32(this.add(MBUS_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 128];
            uacstr(this, MBUS_PATH_LEN, MBUS_PATH, 127, buf.as_mut_ptr());
            let f: FMbusOpen = core::mem::transmute(mbusfn(svc, MB_SVC_OPEN));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(MBUS_HANDLE), h);
            }
        }
        2 => {
            let f: FMbusClose = core::mem::transmute(mbusfn(svc, MB_SVC_CLOSE));
            set_rc(f(svc, ru32(this.add(MBUS_HANDLE))));
        }
        3 => {
            // TplcMbusStatus (48B): state@0 tx@4 rx@8 last_error@12
            let mut st = [0u8; 48];
            let f: FMbusStatus = core::mem::transmute(mbusfn(svc, MB_SVC_STATUS));
            let rc = f(svc, ru32(this.add(MBUS_HANDLE)), st.as_mut_ptr());
            wi32(this.add(MBUS_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(MBUS_STATE), rd(0));
                wu32(this.add(MBUS_TXTOT), rd(4) as u32);
                wu32(this.add(MBUS_RXTOT), rd(8) as u32);
                wi32(this.add(MBUS_LASTERR), rd(12));
            } else {
                wu32(this.add(MBUS_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let f: FMbusShort = core::mem::transmute(mbusfn(svc, MB_SVC_SHORT));
            set_rc(f(svc, ru32(this.add(MBUS_HANDLE)),
                     ru32(this.add(MBUS_CONTROL)), ru32(this.add(MBUS_ADDRESS))));
        }
        5 => {
            let n = (*this.add(MBUS_DATA_LEN) as usize).min(255);
            let f: FMbusLong = core::mem::transmute(mbusfn(svc, MB_SVC_LONG));
            set_rc(f(svc, ru32(this.add(MBUS_HANDLE)),
                     ru32(this.add(MBUS_CONTROL)), ru32(this.add(MBUS_ADDRESS)),
                     ru32(this.add(MBUS_CI)),
                     this.add(MBUS_DATA), n as u32));
        }
        6 => {
            let mut out = [0u8; 255];
            let mut kind: u32 = 0;
            let f: FMbusReceive = core::mem::transmute(mbusfn(svc, MB_SVC_RECEIVE));
            let rc = f(svc, ru32(this.add(MBUS_HANDLE)),
                       &mut kind, out.as_mut_ptr(), 255);
            wi32(this.add(MBUS_STATUS), rc);
            wu32(this.add(MBUS_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                wu32(this.add(MBUS_KIND), kind);
                let n = (rc as usize).min(255);
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(MBUS_RXPAYLOAD), n);
                *this.add(MBUS_RXPAYLOAD_LEN) = n as u8;
            }
        }
        _ => {
            wi32(this.add(MBUS_STATUS), -1);
            wu32(this.add(MBUS_ERRID), 1);
        }
    }
    0
}
