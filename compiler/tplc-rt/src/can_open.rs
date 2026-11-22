//! tplc-rt builtin FB: can open.
use crate::*;

// FB_CanOpen — CANopen/CoE CiA 301 SDO over tplc-canopen (Tether IO
// machine.sdo.* functions; EtherCAT CoE carries the CANopen object
// dictionary).
// ---------------------------------------------------------------------------

// FB_CanOpen member offsets (see builtin_fb_layout("FB_CanOpen")).
pub(crate) const CO_OP: usize = 0;
pub(crate) const CO_SLAVE: usize = 4;
pub(crate) const CO_INDEX: usize = 8;
pub(crate) const CO_SUB: usize = 12;
pub(crate) const CO_VERIFY: usize = 16;
pub(crate) const CO_DLEN: usize = 20;
pub(crate) const CO_ABORT: usize = 24;
pub(crate) const CO_STATUS: usize = 28;
pub(crate) const CO_ERRID: usize = 32;
pub(crate) const CO_DATA: usize = 36; // u8[256]

// tplc-canopen service vtable offsets (TplcCanopenService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B),
// co_state@16, co_sdo_read@24, co_sdo_write@32.
pub(crate) const CO_SVC_STATE: usize = 16;
pub(crate) const CO_SVC_READ: usize = 24;
pub(crate) const CO_SVC_WRITE: usize = 32;
pub(crate) const CO_DATA_CAP: u32 = 256;

/// service lookup for "tplc-canopen" (12 chars)
pub(crate) unsafe fn canopen_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-canopen".as_ptr() as *const u8, 12) as *const u8
}

/// `__tplc_rt_FB_CanOpen` — OP 1=sdo_read, 2=sdo_write, 3=state.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_CanOpen(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(CO_OP));
    wi32(this.add(CO_STATUS), 0);
    wu32(this.add(CO_ERRID), 0);
    wu32(this.add(CO_ABORT), 0);
    let svc = canopen_service(ctx);
    if svc.is_null() {
        wi32(this.add(CO_STATUS), -3);
        wu32(this.add(CO_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(CO_STATUS), rc);
        wu32(this.add(CO_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            /* co_sdo_read(svc, slave u16, index u16, sub u8,
               out ptr, cap u32, out_len* u32, abort* u32) -> i32 */
            let f: unsafe extern "C" fn(
                *const u8, u16, u16, u8, *mut u8, u32, *mut u32,
                *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(CO_SVC_READ) as *const usize).read(),
            );
            let mut out_len: u32 = 0;
            let mut abort: u32 = 0;
            let mut cap = ru32(this.add(CO_DLEN));
            if cap == 0 || cap > CO_DATA_CAP {
                cap = CO_DATA_CAP;
            }
            let rc = f(
                svc,
                ru32(this.add(CO_SLAVE)) as u16,
                ru32(this.add(CO_INDEX)) as u16,
                ru32(this.add(CO_SUB)) as u8,
                this.add(CO_DATA),
                cap,
                &mut out_len,
                &mut abort,
            );
            wu32(this.add(CO_DLEN), out_len);
            wu32(this.add(CO_ABORT), abort);
            set_rc(rc);
        }
        2 => {
            /* co_sdo_write(svc, slave u16, index u16, sub u8,
               data ptr, len u32, verify u8, abort* u32) -> i32 */
            let f: unsafe extern "C" fn(
                *const u8, u16, u16, u8, *const u8, u32, u8,
                *mut u32,
            ) -> i32 = core::mem::transmute(
                (svc.add(CO_SVC_WRITE) as *const usize).read(),
            );
            let mut abort: u32 = 0;
            let len = ru32(this.add(CO_DLEN)).min(CO_DATA_CAP);
            let rc = f(
                svc,
                ru32(this.add(CO_SLAVE)) as u16,
                ru32(this.add(CO_INDEX)) as u16,
                ru32(this.add(CO_SUB)) as u8,
                this.add(CO_DATA),
                len,
                ru32(this.add(CO_VERIFY)) as u8,
                &mut abort,
            );
            wu32(this.add(CO_ABORT), abort);
            set_rc(rc);
        }
        3 => {
            let f: unsafe extern "C" fn(*const u8) -> i32 =
                core::mem::transmute(
                    (svc.add(CO_SVC_STATE) as *const usize).read(),
                );
            set_rc(f(svc));
        }
        _ => {
            wi32(this.add(CO_STATUS), -1);
            wu32(this.add(CO_ERRID), 1);
        }
    }
    0
}
