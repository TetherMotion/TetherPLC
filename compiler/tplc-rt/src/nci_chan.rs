//! tplc-rt builtin FB: nci chan.
use crate::*;

// FB_NciChan member offsets (see builtin_fb_layout("FB_NciChan")).
// Strings are len-prefixed: PATH u8len@8+data@9 (cap127), PROG
// u8len@136+data@137 (cap4095); ints aligned to 4.
pub(crate) const NC_OP: usize = 0;
pub(crate) const NC_CHAN: usize = 4;
pub(crate) const NC_PATH_LEN: usize = 8;
pub(crate) const NC_PATH: usize = 9;
pub(crate) const NC_PROG_LEN: usize = 136;
pub(crate) const NC_PROG: usize = 137;
pub(crate) const NC_CMD: usize = 4232;
pub(crate) const NC_STATUS: usize = 4236;
pub(crate) const NC_ERRID: usize = 4240;
pub(crate) const NC_STATE: usize = 4244;
pub(crate) const NC_LINE: usize = 4248;
pub(crate) const NC_TOTAL: usize = 4252;
pub(crate) const NC_NERR: usize = 4256;
pub(crate) const NC_PROGRESS: usize = 4260;
pub(crate) const NC_DURATION: usize = 4264;
pub(crate) const NC_AUX: usize = 4268;
pub(crate) const NC_RVAL: usize = 4272;
pub(crate) const NC_ZSX: usize = 4280;
pub(crate) const NC_ZSY: usize = 4288;
pub(crate) const NC_ZSZ: usize = 4296;
pub(crate) const NC_MFUNC: usize = 4304;
pub(crate) const NC_FIFOLEN: usize = 4308;
pub(crate) const NC_FIFOSEQ: usize = 4312;

// tplc-nci service vtable offsets (TplcNciService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const NC_SVC_LOAD_TEXT: usize = 16;
pub(crate) const NC_SVC_LOAD_FILE: usize = 24;
pub(crate) const NC_SVC_COMMAND: usize = 32;
pub(crate) const NC_SVC_STATUS: usize = 40;
pub(crate) const NC_SVC_FIFO: usize = 48;
pub(crate) const NC_SVC_CONFIRM: usize = 56;
pub(crate) const NC_SVC_RPARAM: usize = 64;
pub(crate) const NC_SVC_ZSHIFT: usize = 72;

/// service lookup for "tplc-nci" (8 chars)
pub(crate) unsafe fn nci_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-nci".as_ptr() as *const u8, 8) as *const u8
}

#[inline]
pub(crate) unsafe fn ncifn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FNciLoadText = unsafe extern "C" fn(*const u8, u32, *const u8, u32) -> i32;
pub(crate) type FNciCommand = unsafe extern "C" fn(*const u8, u32, u32) -> i32;
pub(crate) type FNciStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FNciConfirm = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FNciRparam = unsafe extern "C" fn(*const u8, u32, u32, i32, *mut f64) -> i32;
pub(crate) type FNciZshift = unsafe extern "C" fn(*const u8, u32, i32, *mut f64) -> i32;

/// `__tplc_rt_FB_NciChan` — synchronous op dispatch over the tplc-nci
/// NC interpreter channel. OP: 1=load program text (PROG), 2=load a
/// program file on the server host (PATH), 3=command (CMD code),
/// 4=status (fills STATE/LINE/TOTAL/NERR/PROGRESS/DURATION/MFUNC/
/// FIFOLEN/FIFOSEQ), 5=FIFO block append (PROG), 6=confirm pending
/// M-function, 7=R-param write (AUX index, RVAL value), 8=R-param
/// read (AUX -> RVAL), 9=zero-shift write (ZSX/ZSY/ZSZ). STATUS
/// carries the service return code, ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_NciChan(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(NC_OP));
    wi32(this.add(NC_STATUS), 0);
    wu32(this.add(NC_ERRID), 0);
    let svc = nci_service(ctx);
    if svc.is_null() {
        wi32(this.add(NC_STATUS), -2);
        wu32(this.add(NC_ERRID), 99);
        return 0;
    }
    match op {
        1 | 2 => {
            let (len_off, data_off, cap) = if op == 1 {
                (NC_PROG_LEN, NC_PROG, 4095usize)
            } else {
                (NC_PATH_LEN, NC_PATH, 127usize)
            };
            let n = (*this.add(len_off) as usize).min(cap);
            let f: FNciLoadText = core::mem::transmute(ncifn(
                svc,
                if op == 1 {
                    NC_SVC_LOAD_TEXT
                } else {
                    NC_SVC_LOAD_FILE
                },
            ));
            let rc = f(svc, ru32(this.add(NC_CHAN)), this.add(data_off), n as u32);
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
        }
        3 => {
            let f: FNciCommand = core::mem::transmute(ncifn(svc, NC_SVC_COMMAND));
            let rc = f(svc, ru32(this.add(NC_CHAN)), ru32(this.add(NC_CMD)));
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
        }
        4 => {
            // TplcNciStatus (64 bytes): state@0 finished@4 errcode@8
            // line@12 total@16 progress_ms@20 duration_ms@24
            let mut st = [0u8; 64];
            let f: FNciStatus = core::mem::transmute(ncifn(svc, NC_SVC_STATUS));
            let rc = f(svc, ru32(this.add(NC_CHAN)), st.as_mut_ptr());
            wi32(this.add(NC_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(NC_STATE), rd(0));
                wi32(this.add(NC_NERR), rd(8));
                wu32(this.add(NC_LINE), rd(12) as u32);
                wu32(this.add(NC_TOTAL), rd(16) as u32);
                wi32(this.add(NC_PROGRESS), rd(20));
                wi32(this.add(NC_DURATION), rd(24));
                wu32(this.add(NC_MFUNC), rd(28) as u32);
                wu32(this.add(NC_FIFOLEN), rd(32) as u32);
                wu32(this.add(NC_FIFOSEQ), rd(36) as u32);
            } else {
                wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
            }
        }
        5 => {
            let n = (*this.add(NC_PROG_LEN) as usize).min(4095);
            let f: FNciLoadText = core::mem::transmute(ncifn(svc, NC_SVC_FIFO));
            let rc = f(
                svc,
                ru32(this.add(NC_CHAN)),
                this.add(NC_PROG),
                n as u32,
            );
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
        }
        6 => {
            let f: FNciConfirm = core::mem::transmute(ncifn(svc, NC_SVC_CONFIRM));
            let rc = f(svc, ru32(this.add(NC_CHAN)));
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
        }
        7 | 8 => {
            let mut v: f64 = core::ptr::read_unaligned(this.add(NC_RVAL) as *const f64);
            let f: FNciRparam = core::mem::transmute(ncifn(svc, NC_SVC_RPARAM));
            let rc = f(
                svc,
                ru32(this.add(NC_CHAN)),
                ru32(this.add(NC_AUX)),
                if op == 7 { 1 } else { 0 },
                &mut v,
            );
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
            if rc == 0 && op == 8 {
                core::ptr::write_unaligned(this.add(NC_RVAL) as *mut f64, v);
            }
        }
        9 => {
            let mut xyz = [
                core::ptr::read_unaligned(this.add(NC_ZSX) as *const f64),
                core::ptr::read_unaligned(this.add(NC_ZSY) as *const f64),
                core::ptr::read_unaligned(this.add(NC_ZSZ) as *const f64),
            ];
            let f: FNciZshift = core::mem::transmute(ncifn(svc, NC_SVC_ZSHIFT));
            let rc = f(svc, ru32(this.add(NC_CHAN)), 1, xyz.as_mut_ptr());
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
        }
        _ => {
            wi32(this.add(NC_STATUS), -1);
            wu32(this.add(NC_ERRID), 1);
        }
    }
    0
}
