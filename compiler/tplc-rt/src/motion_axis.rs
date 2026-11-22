//! tplc-rt builtin FB: motion axis.
use crate::*;

// FB_MotionAxis member offsets (see builtin_fb_layout("FB_MotionAxis"))
// all members are i32/u32 (4 bytes), declaration order.
pub(crate) const MA_OP: usize = 0;
pub(crate) const MA_AXIS: usize = 4;
pub(crate) const MA_UNITS_PER_REV: usize = 8;
pub(crate) const MA_MAX_VEL: usize = 12;
pub(crate) const MA_MAX_ACCEL: usize = 16;
pub(crate) const MA_MAX_DECEL: usize = 20;
pub(crate) const MA_POS_MIN: usize = 24;
pub(crate) const MA_POS_MAX: usize = 28;
pub(crate) const MA_HOME_OFFSET: usize = 32;
pub(crate) const MA_FOLLOW_WINDOW: usize = 36;
pub(crate) const MA_FLAGS: usize = 40;
pub(crate) const MA_INDEX: usize = 44;
pub(crate) const MA_VALUE: usize = 48;
pub(crate) const MA_STATUS: usize = 52;
pub(crate) const MA_ERRID: usize = 56;
pub(crate) const MA_MODE: usize = 60;
pub(crate) const MA_FAULT: usize = 64;
pub(crate) const MA_TARGET_REACHED: usize = 68;
pub(crate) const MA_BUSY: usize = 72;
pub(crate) const MA_POS_ACTUAL: usize = 76;
pub(crate) const MA_VEL_ACTUAL: usize = 80;

// tplc-motion service vtable offsets (TplcMotionService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const MA_SVC_CREATE: usize = 16;
pub(crate) const MA_SVC_DESTROY: usize = 24;
pub(crate) const MA_SVC_POLL: usize = 32;
pub(crate) const MA_SVC_INFO: usize = 40;
pub(crate) const MA_SVC_OBJ_READ: usize = 48;
pub(crate) const MA_SVC_OBJ_WRITE: usize = 56;

/// service lookup for "tplc-motion" (11 chars)
pub(crate) unsafe fn motion_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-motion".as_ptr() as *const u8, 11) as *const u8
}

#[inline]
pub(crate) unsafe fn motionfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FMAxisCreate = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FMAxisDestroy = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FMAxisPoll = unsafe extern "C" fn(*const u8) -> i32;
pub(crate) type FMAxisInfo = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FMObjRead = unsafe extern "C" fn(*const u8, u32, u32, *mut i32) -> i32;
pub(crate) type FMObjWrite = unsafe extern "C" fn(*const u8, u32, u32, i32) -> i32;

/// `__tplc_rt_FB_MotionAxis` — synchronous op dispatch over the
/// tplc-motion simulated CiA402 axis. OP selects the operation:
/// 1=create (CFG fields -> AXIS handle), 2=destroy, 3=poll,
/// 4=read object (INDEX -> VALUE), 5=write object (INDEX+VALUE),
/// 6=info (axis snapshot into MODE/FAULT/TARGET_REACHED/BUSY/
/// POS_ACTUAL/VEL_ACTUAL). STATUS carries the
/// service return code (for OP=6 the DS402 state). All calls are
/// immediate; the compat layer gates create/destroy timing.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_MotionAxis(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(MA_OP));
    wi32(this.add(MA_STATUS), 0);
    wu32(this.add(MA_ERRID), 0);
    let svc = motion_service(ctx);
    if svc.is_null() {
        wi32(this.add(MA_STATUS), -2);
        wu32(this.add(MA_ERRID), 99);
        return 0;
    }
    match op {
        1 => {
            // TPLC_MotionAxisCfg (64 bytes): 8xi32 cfg @0..31,
            // flags u32 @32, reserved[7] u32 @36..63
            let mut cfg = [0u8; 64];
            let mut put = |off: usize, v: i32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), cfg.as_mut_ptr().add(off), 4);
            };
            put(0, ri32(this.add(MA_UNITS_PER_REV)));
            put(4, ri32(this.add(MA_MAX_VEL)));
            put(8, ri32(this.add(MA_MAX_ACCEL)));
            put(12, ri32(this.add(MA_MAX_DECEL)));
            put(16, ri32(this.add(MA_POS_MIN)));
            put(20, ri32(this.add(MA_POS_MAX)));
            put(24, ri32(this.add(MA_HOME_OFFSET)));
            put(28, ri32(this.add(MA_FOLLOW_WINDOW)));
            let fl = ru32(this.add(MA_FLAGS)).to_le_bytes();
            core::ptr::copy_nonoverlapping(fl.as_ptr(), cfg.as_mut_ptr().add(32), 4);
            let mut h: u32 = 0;
            let f: FMAxisCreate = core::mem::transmute(motionfn(svc, MA_SVC_CREATE));
            let rc = f(svc, cfg.as_ptr(), &mut h);
            wi32(this.add(MA_STATUS), rc);
            wu32(this.add(MA_ERRID), (-rc).max(0) as u32);
            wu32(this.add(MA_AXIS), h);
        }
        2 => {
            let f: FMAxisDestroy = core::mem::transmute(motionfn(svc, MA_SVC_DESTROY));
            let rc = f(svc, ru32(this.add(MA_AXIS)));
            wi32(this.add(MA_STATUS), rc);
            wu32(this.add(MA_ERRID), (-rc).max(0) as u32);
        }
        3 => {
            let f: FMAxisPoll = core::mem::transmute(motionfn(svc, MA_SVC_POLL));
            let rc = f(svc);
            wi32(this.add(MA_STATUS), rc);
        }
        4 => {
            let mut v: i32 = 0;
            let f: FMObjRead = core::mem::transmute(motionfn(svc, MA_SVC_OBJ_READ));
            let rc = f(
                svc,
                ru32(this.add(MA_AXIS)),
                ru32(this.add(MA_INDEX)),
                &mut v,
            );
            wi32(this.add(MA_STATUS), rc);
            wi32(this.add(MA_VALUE), v);
            wu32(this.add(MA_ERRID), (-rc).max(0) as u32);
        }
        5 => {
            let f: FMObjWrite = core::mem::transmute(motionfn(svc, MA_SVC_OBJ_WRITE));
            let rc = f(
                svc,
                ru32(this.add(MA_AXIS)),
                ru32(this.add(MA_INDEX)),
                ri32(this.add(MA_VALUE)),
            );
            wi32(this.add(MA_STATUS), rc);
            wu32(this.add(MA_ERRID), (-rc).max(0) as u32);
        }
        6 => {
            // TPLC_MotionAxisInfo (64 bytes): status@0 mode@4 fault@8
            // target_reached@12 busy@16 pos@20 vel@24
            let mut info = [0u8; 64];
            let f: FMAxisInfo = core::mem::transmute(motionfn(svc, MA_SVC_INFO));
            let rc = f(svc, ru32(this.add(MA_AXIS)), info.as_mut_ptr());
            wi32(this.add(MA_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(info.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(MA_STATUS), rd(0));
                wi32(this.add(MA_MODE), rd(4));
                wi32(this.add(MA_FAULT), rd(8));
                wi32(this.add(MA_TARGET_REACHED), rd(12));
                wi32(this.add(MA_BUSY), rd(16));
                wi32(this.add(MA_POS_ACTUAL), rd(20));
                wi32(this.add(MA_VEL_ACTUAL), rd(24));
            } else {
                wu32(this.add(MA_ERRID), (-rc).max(0) as u32);
            }
        }
        _ => {
            wi32(this.add(MA_STATUS), -1);
            wu32(this.add(MA_ERRID), 1);
        }
    }
    0
}
