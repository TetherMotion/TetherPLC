//! tplc-rt builtin FB: mcgroup.
use crate::*;

// ---------------------------------------------------------------------------
// FB_McGroup — tplc-mcgroup service bridge
// (Tc3_McCoordinatedMotion group administration).
// ---------------------------------------------------------------------------

// FB_McGroup member offsets (see builtin_fb_layout("FB_McGroup")).
pub(crate) const MG_OP: usize = 0;
pub(crate) const MG_GRP: usize = 4;
pub(crate) const MG_AXIS: usize = 8;
pub(crate) const MG_IDENT: usize = 12;
pub(crate) const MG_ENABLE: usize = 16;
pub(crate) const MG_SOURCE: usize = 20;
pub(crate) const MG_COORD: usize = 24;
pub(crate) const MG_STATUS: usize = 28;
pub(crate) const MG_ERRID: usize = 32;
pub(crate) const MG_STATE: usize = 36;
pub(crate) const MG_NAXES: usize = 40;
pub(crate) const MG_ENABLED: usize = 44;
#[allow(dead_code)] // reserved ABI slot
pub(crate) const MG_SRC: usize = 48;
pub(crate) const MG_OVR: usize = 56;    // f64
pub(crate) const MG_OVRACT: usize = 64; // f64
pub(crate) const MG_AXES: usize = 72;   // u32[16]

// tplc-mcgroup service vtable offsets (TplcMcGroupService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const MG_SVC_ENSURE: usize = 16;
pub(crate) const MG_SVC_ADD_AXIS: usize = 24;
pub(crate) const MG_SVC_REMOVE_AXIS: usize = 32;
pub(crate) const MG_SVC_UNGROUP: usize = 40;
pub(crate) const MG_SVC_SET_STATE: usize = 48;
pub(crate) const MG_SVC_STATUS: usize = 56;
pub(crate) const MG_SVC_SET_OVERRIDE: usize = 64;
pub(crate) const MG_SVC_SET_SOURCE: usize = 72;
pub(crate) const MG_SVC_SET_TRANSFORM: usize = 80;
#[allow(dead_code)] // reserved ABI slot
pub(crate) const MG_SVC_GET_SOURCE: usize = 88;
pub(crate) const MG_SVC_GET_TRANSFORM: usize = 96;
#[allow(dead_code)] // reserved ABI slot
pub(crate) const MG_SVC_GET_OVERRIDE: usize = 104;
pub(crate) const MG_SVC_AXES: usize = 112;

/// service lookup for "tplc-mcgroup" (11 chars)
pub(crate) unsafe fn mcgroup_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-mcgroup".as_ptr() as *const u8, 11) as *const u8
}

#[inline]
pub(crate) unsafe fn mgfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FNmgEnsure = unsafe extern "C" fn(*const u8, u32, *mut u32) -> i32;
pub(crate) type FNmgAxisOp = unsafe extern "C" fn(*const u8, u32, u32, u32) -> i32;
pub(crate) type FNmgUngroup = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FNmgSetState = unsafe extern "C" fn(*const u8, u32, i32) -> i32;
pub(crate) type FNmgStatus = unsafe extern "C" fn(
    *const u8, u32, *mut i32, *mut i32, *mut i32, *mut i32,
) -> i32;
pub(crate) type FNmgSetOverride =
    unsafe extern "C" fn(*const u8, u32, f64, *mut f64) -> i32;
pub(crate) type FNmgSetSource = unsafe extern "C" fn(*const u8, u32, i32) -> i32;
pub(crate) type FNmgSetTransform = unsafe extern "C" fn(*const u8, u32, u32) -> i32;
#[allow(dead_code)] // reserved ABI slot
pub(crate) type FNmgGetI32 = unsafe extern "C" fn(*const u8, u32, *mut i32) -> i32;
pub(crate) type FNmgGetU32 = unsafe extern "C" fn(*const u8, u32, *mut u32) -> i32;
#[allow(dead_code)] // reserved ABI slot
pub(crate) type FNmgGetF64 = unsafe extern "C" fn(*const u8, u32, *mut f64) -> i32;
pub(crate) type FNmgAxes = unsafe extern "C" fn(
    *const u8, u32, *mut u32, u32, *mut i32,
) -> i32;

/// `__tplc_rt_FB_McGroup` — synchronous op dispatch over the
/// tplc-mcgroup coordinated-motion group table. OP: 1=ensure,
/// 2=add axis, 3=remove axis, 4=ungroup, 5=set state
/// (ENABLE: 1 enable / 0 disable / 2 reset), 6=status,
/// 7=set override, 8=set source, 9=set transform,
/// 10=get axes.
/// STATUS carries the service return code, ERRID its magnitude.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_McGroup(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(MG_OP));
    wi32(this.add(MG_STATUS), 0);
    wu32(this.add(MG_ERRID), 0);
    let svc = mcgroup_service(ctx);
    if svc.is_null() {
        wi32(this.add(MG_STATUS), -2);
        wu32(this.add(MG_ERRID), 2);
        return 0;
    }
    let grp = ru32(this.add(MG_GRP));
    let fail = |this: *mut u8, rc: i32| {
        wi32(this.add(MG_STATUS), rc);
        wu32(this.add(MG_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut out = 0u32;
            let f: FNmgEnsure =
                core::mem::transmute(mgfn(svc, MG_SVC_ENSURE));
            let rc = f(svc, grp, &mut out);
            if rc == 0 {
                wu32(this.add(MG_GRP), out);
            }
            fail(this, rc);
        }
        2 => {
            let f: FNmgAxisOp =
                core::mem::transmute(mgfn(svc, MG_SVC_ADD_AXIS));
            fail(this, f(svc, grp, ru32(this.add(MG_AXIS)),
                         ru32(this.add(MG_IDENT))));
        }
        3 => {
            let f: FNmgAxisOp =
                core::mem::transmute(mgfn(svc, MG_SVC_REMOVE_AXIS));
            fail(this, f(svc, grp, ru32(this.add(MG_AXIS)),
                         ru32(this.add(MG_IDENT))));
        }
        4 => {
            let f: FNmgUngroup =
                core::mem::transmute(mgfn(svc, MG_SVC_UNGROUP));
            fail(this, f(svc, grp));
        }
        5 => {
            let f: FNmgSetState =
                core::mem::transmute(mgfn(svc, MG_SVC_SET_STATE));
            fail(this, f(svc, grp, ri32(this.add(MG_ENABLE))));
        }
        6 => {
            let mut st = 0i32;
            let mut n = 0i32;
            let mut en = 0i32;
            let mut err = 0i32;
            let f: FNmgStatus =
                core::mem::transmute(mgfn(svc, MG_SVC_STATUS));
            let rc = f(svc, grp, &mut st, &mut n, &mut en, &mut err);
            wi32(this.add(MG_STATUS), rc);
            if rc == 0 {
                wi32(this.add(MG_STATE), st);
                wi32(this.add(MG_NAXES), n);
                wi32(this.add(MG_ENABLED), en);
                wu32(this.add(MG_ERRID), err.max(0) as u32);
            } else {
                wu32(this.add(MG_ERRID), (-rc).max(0) as u32);
            }
        }
        7 => {
            let ovr = f64::from_bits(ru64(this.add(MG_OVR)));
            let mut act = 0.0f64;
            let f: FNmgSetOverride =
                core::mem::transmute(mgfn(svc, MG_SVC_SET_OVERRIDE));
            let rc = f(svc, grp, ovr, &mut act);
            fail(this, rc);
            if rc == 0 {
                wu64(this.add(MG_OVRACT), act.to_bits());
            }
        }
        8 => {
            let f: FNmgSetSource =
                core::mem::transmute(mgfn(svc, MG_SVC_SET_SOURCE));
            fail(this, f(svc, grp, ri32(this.add(MG_SOURCE))));
        }
        9 => {
            let f: FNmgSetTransform =
                core::mem::transmute(mgfn(svc, MG_SVC_SET_TRANSFORM));
            fail(this, f(svc, grp, ru32(this.add(MG_COORD))));
        }
        11 => {
            let f: FNmgGetU32 =
                core::mem::transmute(mgfn(svc, MG_SVC_GET_TRANSFORM));
            let mut c = 0u32;
            let rc = f(svc, grp, &mut c);
            wi32(this.add(MG_STATUS), rc);
            if rc == 0 {
                wu32(this.add(MG_COORD), c);
            } else {
                wu32(this.add(MG_ERRID), (-rc).max(0) as u32);
            }
        }
        10 => {
            let mut n = 0i32;
            let f: FNmgAxes =
                core::mem::transmute(mgfn(svc, MG_SVC_AXES));
            let rc = f(svc, grp, this.add(MG_AXES) as *mut u32,
                       16, &mut n);
            wi32(this.add(MG_STATUS), rc);
            if rc == 0 {
                wi32(this.add(MG_NAXES), n);
            } else {
                wu32(this.add(MG_ERRID), (-rc).max(0) as u32);
            }
        }
        _ => {
            wi32(this.add(MG_STATUS), -1);
            wu32(this.add(MG_ERRID), 1);
        }
    }
    0
}
