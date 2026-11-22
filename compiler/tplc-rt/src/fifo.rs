//! tplc-rt builtin FB: fifo.
use crate::*;

// ---------------------------------------------------------------------------
// FB_Fifo — FiFoAxes numeric setpoint FIFO over the tplc-fifo service.
// ---------------------------------------------------------------------------

// FB_Fifo member offsets (see builtin_fb_layout("FB_Fifo")).
// ROWS is a flat LREAL array: row r column c at index r*8+c.
pub(crate) const FF_OP: usize = 0;
pub(crate) const FF_CHAN: usize = 4;
pub(crate) const FF_AUX: usize = 8;
pub(crate) const FF_NROWS: usize = 12;
pub(crate) const FF_VAL: usize = 16;
pub(crate) const FF_STATE: usize = 24;
pub(crate) const FF_STATUS: usize = 28;
pub(crate) const FF_ERRID: usize = 32;
pub(crate) const FF_QLEN: usize = 36;
pub(crate) const FF_SEQ: usize = 40;
pub(crate) const FF_NAXES: usize = 44;
pub(crate) const FF_MAXROWS: usize = 48;
pub(crate) const FF_ROWS: usize = 56;

// tplc-fifo service vtable offsets (TplcFifoService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const FF_SVC_INTEGRATE: usize = 16;
pub(crate) const FF_SVC_DISINTEGRATE: usize = 24;
pub(crate) const FF_SVC_WRITE: usize = 32;
pub(crate) const FF_SVC_COMMAND: usize = 40;
pub(crate) const FF_SVC_STATUS: usize = 48;
pub(crate) const FF_SVC_SET_OVERRIDE: usize = 56;
pub(crate) const FF_SVC_POLL: usize = 64;

/// service lookup for "tplc-fifo" (9 chars)
pub(crate) unsafe fn fifo_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-fifo".as_ptr() as *const u8, 9) as *const u8
}

#[inline]
pub(crate) unsafe fn fifofn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FNfifoIntegrate = unsafe extern "C" fn(*const u8, u32, u32, u32) -> i32;
pub(crate) type FNfifoDisintegrate = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FNfifoWrite =
    unsafe extern "C" fn(*const u8, u32, *const u8, u32, i32) -> i32;
pub(crate) type FNfifoCommand = unsafe extern "C" fn(*const u8, u32, u32) -> i32;
pub(crate) type FNfifoStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FNfifoSetOverride = unsafe extern "C" fn(*const u8, u32, f64) -> i32;
pub(crate) type FNfifoPoll = unsafe extern "C" fn(*const u8, u32) -> i32;

/// `__tplc_rt_FB_Fifo` — synchronous op dispatch over the tplc-fifo
/// FiFoAxes channel. OP: 1=integrate axis (AUX = axis | gpos<<16),
/// 2=disintegrate, 3=append rows (NROWS + ROWS blob), 4=overwrite
/// rows, 5=command (AUX = cmd code), 6=status+poll (fills
/// STATE/QLEN/SEQ/NAXES/MAXROWS), 7=set override (VAL), 8=status
/// without poll. STATUS carries the service return code, ERRID its
/// positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Fifo(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(FF_OP));
    wi32(this.add(FF_STATUS), 0);
    wu32(this.add(FF_ERRID), 0);
    let svc = fifo_service(ctx);
    if svc.is_null() {
        wi32(this.add(FF_STATUS), -2);
        wu32(this.add(FF_ERRID), 99);
        return 0;
    }
    match op {
        1 => {
            let f: FNfifoIntegrate =
                core::mem::transmute(fifofn(svc, FF_SVC_INTEGRATE));
            let rc = f(
                svc,
                ru32(this.add(FF_CHAN)),
                ru32(this.add(FF_AUX)) & 0xFFFF,
                ru32(this.add(FF_AUX)) >> 16,
            );
            wi32(this.add(FF_STATUS), rc);
            wu32(this.add(FF_ERRID), (-rc).max(0) as u32);
        }
        2 => {
            let f: FNfifoDisintegrate =
                core::mem::transmute(fifofn(svc, FF_SVC_DISINTEGRATE));
            let rc = f(svc, ru32(this.add(FF_CHAN)));
            wi32(this.add(FF_STATUS), rc);
            wu32(this.add(FF_ERRID), (-rc).max(0) as u32);
        }
        3 | 4 => {
            // Wire blob: u32 nrows + nrows*8 f64 LE. ROWS is already
            // contiguous f64s; cap at the 32-row buffer.
            let nrows = (ru32(this.add(FF_NROWS)) as usize).min(32);
            let mut blob = [0u8; 4 + 32 * 8 * 8];
            let nr = nrows as u32;
            blob[0..4].copy_from_slice(&nr.to_le_bytes());
            core::ptr::copy_nonoverlapping(
                this.add(FF_ROWS),
                blob.as_mut_ptr().add(4),
                nrows * 8 * 8,
            );
            let f: FNfifoWrite =
                core::mem::transmute(fifofn(svc, FF_SVC_WRITE));
            let rc = f(
                svc,
                ru32(this.add(FF_CHAN)),
                blob.as_ptr(),
                (4 + nrows * 8 * 8) as u32,
                if op == 4 { 1 } else { 0 },
            );
            wi32(this.add(FF_STATUS), rc);
            wu32(this.add(FF_ERRID), (-rc).max(0) as u32);
        }
        5 => {
            let f: FNfifoCommand =
                core::mem::transmute(fifofn(svc, FF_SVC_COMMAND));
            let rc = f(svc, ru32(this.add(FF_CHAN)), ru32(this.add(FF_AUX)));
            wi32(this.add(FF_STATUS), rc);
            wu32(this.add(FF_ERRID), (-rc).max(0) as u32);
        }
        6 | 8 => {
            if op == 6 {
                let f: FNfifoPoll =
                    core::mem::transmute(fifofn(svc, FF_SVC_POLL));
                f(svc, ru32(this.add(FF_CHAN)));
            }
            // TplcFifoStatus (56 bytes): state@0 naxes@4 maxrows@8
            // queued@12 seq@16 override@24
            let mut st = [0u8; 56];
            let f: FNfifoStatus =
                core::mem::transmute(fifofn(svc, FF_SVC_STATUS));
            let rc = f(svc, ru32(this.add(FF_CHAN)), st.as_mut_ptr());
            wi32(this.add(FF_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(
                        st.as_ptr().add(off),
                        b.as_mut_ptr(),
                        4,
                    );
                    i32::from_le_bytes(b)
                };
                wi32(this.add(FF_STATE), rd(0));
                wi32(this.add(FF_NAXES), rd(4));
                wu32(this.add(FF_MAXROWS), rd(8) as u32);
                wu32(this.add(FF_QLEN), rd(12) as u32);
                wu32(this.add(FF_SEQ), rd(16) as u32);
            } else {
                wu32(this.add(FF_ERRID), (-rc).max(0) as u32);
            }
        }
        7 => {
            let v = core::ptr::read_unaligned(this.add(FF_VAL) as *const f64);
            let f: FNfifoSetOverride =
                core::mem::transmute(fifofn(svc, FF_SVC_SET_OVERRIDE));
            let rc = f(svc, ru32(this.add(FF_CHAN)), v);
            wi32(this.add(FF_STATUS), rc);
            wu32(this.add(FF_ERRID), (-rc).max(0) as u32);
        }
        _ => {
            wi32(this.add(FF_STATUS), -1);
            wu32(this.add(FF_ERRID), 1);
        }
    }
    0
}
