//! tplc-rt builtin FB: kin.
use crate::*;

// FB_Kin — kinematic transformation objects over the tplc-kin
// service (Tc2_NcKinematicTransformation).
// ---------------------------------------------------------------------------

// FB_Kin member offsets (see builtin_fb_layout("FB_Kin")).
pub(crate) const FK_OP: usize = 0;
pub(crate) const FK_OBJ: usize = 4;
pub(crate) const FK_TYPE: usize = 8;
pub(crate) const FK_PRESET: usize = 12;
pub(crate) const FK_NCFG: usize = 16;
pub(crate) const FK_NIN: usize = 20;
pub(crate) const FK_NACS: usize = 24;
pub(crate) const FK_NMCS: usize = 28;
pub(crate) const FK_CMD: usize = 32;
pub(crate) const FK_ROP: usize = 36;
pub(crate) const FK_CHN: usize = 40;
pub(crate) const FK_LOCKED: usize = 44;
pub(crate) const FK_STATUS: usize = 48;
pub(crate) const FK_ERRID: usize = 52;
pub(crate) const FK_STATE: usize = 56;
pub(crate) const FK_KTYPE: usize = 60;
pub(crate) const FK_DOF: usize = 64;
pub(crate) const FK_ERR: usize = 68;
pub(crate) const FK_CHNST: usize = 72;
pub(crate) const FK_CFG: usize = 80;   // f64[32]
pub(crate) const FK_IN: usize = 336;   // f64[8]
pub(crate) const FK_OUT: usize = 400;  // f64[8]
pub(crate) const FK_Q: usize = 464;    // f64[8]
pub(crate) const FK_ACS: usize = 528;  // u32[8]
pub(crate) const FK_MCS: usize = 560;  // u32[8]
pub(crate) const FK_ROT: usize = 592;  // f64[3]

// tplc-kin service vtable offsets (TplcKinService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const FK_SVC_CONFIGURE: usize = 16;
pub(crate) const FK_SVC_SET_AXES: usize = 24;
pub(crate) const FK_SVC_COMMAND: usize = 32;
pub(crate) const FK_SVC_CALC_INPUT: usize = 40;
pub(crate) const FK_SVC_RESULT: usize = 48;
pub(crate) const FK_SVC_SET_LOCKED: usize = 56;
pub(crate) const FK_SVC_ROTATION: usize = 64;
pub(crate) const FK_SVC_STATUS: usize = 72;
pub(crate) const FK_SVC_SET_CHANNEL: usize = 80;
pub(crate) const FK_SVC_GET_AXES: usize = 88;

/// service lookup for "tplc-kin" (8 chars)
pub(crate) unsafe fn kin_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-kin".as_ptr() as *const u8, 8) as *const u8
}

#[inline]
pub(crate) unsafe fn kinfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FNkinConfigure = unsafe extern "C" fn(
    *const u8, u32, i32, u32, *const f64, u32,
) -> i32;
pub(crate) type FNkinSetAxes = unsafe extern "C" fn(
    *const u8, u32, *const u32, u32, *const u32, u32,
) -> i32;
pub(crate) type FNkinCommand = unsafe extern "C" fn(*const u8, u32, u32) -> i32;
pub(crate) type FNkinCalcInput =
    unsafe extern "C" fn(*const u8, u32, *const f64, u32) -> i32;
pub(crate) type FNkinResult =
    unsafe extern "C" fn(*const u8, u32, *mut f64, *mut f64) -> i32;
pub(crate) type FNkinSetLocked = unsafe extern "C" fn(*const u8, u32, u32) -> i32;
pub(crate) type FNkinRotation = unsafe extern "C" fn(
    *const u8, u32, u32, *const f64, *mut f64,
) -> i32;
pub(crate) type FNkinStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FNkinSetChannel = unsafe extern "C" fn(*const u8, u32, u32) -> i32;
pub(crate) type FNkinGetAxes = unsafe extern "C" fn(
    *const u8, u32, *mut u32, *mut u32, *mut u32, *mut u32,
) -> i32;

/// `__tplc_rt_FB_Kin` — synchronous op dispatch over the tplc-kin
/// kinematic transformation object. OP: 1=configure, 2=set axes,
/// 3=command (CMD), 4=calc input, 5=result, 6=set locked,
/// 7=rotation (ROP + ROT), 8=status, 9=set channel (CHN),
/// 10=get axes (ACS/MCS + NACS/NMCS).
/// STATUS carries the service return code, ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Kin(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(FK_OP));
    wi32(this.add(FK_STATUS), 0);
    wu32(this.add(FK_ERRID), 0);
    let svc = kin_service(ctx);
    if svc.is_null() {
        wi32(this.add(FK_STATUS), -2);
        wu32(this.add(FK_ERRID), 99);
        return 0;
    }
    let obj = ru32(this.add(FK_OBJ));
    let fail = |this: *mut u8, rc: i32| {
        wi32(this.add(FK_STATUS), rc);
        wu32(this.add(FK_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let f: FNkinConfigure =
                core::mem::transmute(kinfn(svc, FK_SVC_CONFIGURE));
            let rc = f(
                svc,
                obj,
                ri32(this.add(FK_TYPE)),
                ru32(this.add(FK_PRESET)),
                this.add(FK_CFG) as *const f64,
                ru32(this.add(FK_NCFG)).min(32),
            );
            fail(this, rc);
        }
        2 => {
            let f: FNkinSetAxes =
                core::mem::transmute(kinfn(svc, FK_SVC_SET_AXES));
            let rc = f(
                svc,
                obj,
                this.add(FK_ACS) as *const u32,
                ru32(this.add(FK_NACS)).min(8),
                this.add(FK_MCS) as *const u32,
                ru32(this.add(FK_NMCS)).min(8),
            );
            fail(this, rc);
        }
        3 => {
            let f: FNkinCommand =
                core::mem::transmute(kinfn(svc, FK_SVC_COMMAND));
            let rc = f(svc, obj, ru32(this.add(FK_CMD)));
            fail(this, rc);
        }
        4 => {
            let f: FNkinCalcInput =
                core::mem::transmute(kinfn(svc, FK_SVC_CALC_INPUT));
            let rc = f(
                svc,
                obj,
                this.add(FK_IN) as *const f64,
                ru32(this.add(FK_NIN)).min(8),
            );
            fail(this, rc);
        }
        5 => {
            let f: FNkinResult =
                core::mem::transmute(kinfn(svc, FK_SVC_RESULT));
            let rc = f(
                svc,
                obj,
                this.add(FK_OUT) as *mut f64,
                this.add(FK_Q) as *mut f64,
            );
            fail(this, rc);
        }
        6 => {
            let f: FNkinSetLocked =
                core::mem::transmute(kinfn(svc, FK_SVC_SET_LOCKED));
            let rc = f(svc, obj, ru32(this.add(FK_LOCKED)));
            fail(this, rc);
        }
        7 => {
            let f: FNkinRotation =
                core::mem::transmute(kinfn(svc, FK_SVC_ROTATION));
            let rot = this.add(FK_ROT) as *mut f64;
            let rc = f(
                svc,
                obj,
                ru32(this.add(FK_ROP)),
                rot as *const f64,
                rot,
            );
            fail(this, rc);
        }
        8 => {
            // TplcKinStatus: state@0 type@4 dof@8 nacs@12 nmcs@16
            // locked@20 err@24 chnstate@28 chn@32.
            let mut st = [0u8; 64];
            let f: FNkinStatus =
                core::mem::transmute(kinfn(svc, FK_SVC_STATUS));
            let rc = f(svc, obj, st.as_mut_ptr());
            wi32(this.add(FK_STATUS), rc);
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
                let rdu = |off: usize| -> u32 { rd(off) as u32 };
                wi32(this.add(FK_STATE), rd(0));
                wi32(this.add(FK_KTYPE), rd(4));
                wi32(this.add(FK_DOF), rd(8));
                wu32(this.add(FK_NACS), rdu(12));
                wu32(this.add(FK_NMCS), rdu(16));
                wu32(this.add(FK_LOCKED), rdu(20));
                wi32(this.add(FK_ERR), rd(24));
                wi32(this.add(FK_CHNST), rd(28));
                wu32(this.add(FK_CHN), rdu(32));
            } else {
                wu32(this.add(FK_ERRID), (-rc).max(0) as u32);
            }
        }
        9 => {
            let f: FNkinSetChannel =
                core::mem::transmute(kinfn(svc, FK_SVC_SET_CHANNEL));
            let rc = f(svc, obj, ru32(this.add(FK_CHN)));
            fail(this, rc);
        }
        10 => {
            let mut na = 0u32;
            let mut nm = 0u32;
            let f: FNkinGetAxes =
                core::mem::transmute(kinfn(svc, FK_SVC_GET_AXES));
            let rc = f(
                svc,
                obj,
                this.add(FK_ACS) as *mut u32,
                &mut na,
                this.add(FK_MCS) as *mut u32,
                &mut nm,
            );
            wi32(this.add(FK_STATUS), rc);
            if rc == 0 {
                wu32(this.add(FK_NACS), na);
                wu32(this.add(FK_NMCS), nm);
            } else {
                wu32(this.add(FK_ERRID), (-rc).max(0) as u32);
            }
        }
        _ => {
            wi32(this.add(FK_STATUS), -1);
            wu32(this.add(FK_ERRID), 1);
        }
    }
    0
}
