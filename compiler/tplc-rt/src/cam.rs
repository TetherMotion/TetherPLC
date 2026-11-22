//! tplc-rt builtin FB: cam.
use crate::*;

// ---------------------------------------------------------------------------
// FB_Cam — tplc-cam service bridge (Tc3_Mc3Camming cam objects and
// per-slave cam bindings).
// ---------------------------------------------------------------------------

// FB_Cam member offsets (see builtin_fb_layout("FB_Cam")).
pub(crate) const CM_OP: usize = 0;
pub(crate) const CM_HANDLE: usize = 4;
pub(crate) const CM_AUX: usize = 8;
pub(crate) const CM_AUX2: usize = 12;
pub(crate) const CM_FLAGS: usize = 16;
pub(crate) const CM_STATUS: usize = 20;
pub(crate) const CM_ERRID: usize = 24;
pub(crate) const CM_NPTS: usize = 28;
pub(crate) const CM_F1: usize = 32; // f64
pub(crate) const CM_F2: usize = 40; // f64
pub(crate) const CM_F3: usize = 48; // f64
pub(crate) const CM_F4: usize = 56; // f64

// tplc-cam service vtable offsets (TplcCamService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const CM_SVC_CREATE: usize = 16;
pub(crate) const CM_SVC_CLEAR: usize = 24;
pub(crate) const CM_SVC_WRITE: usize = 32;
pub(crate) const CM_SVC_CONFIG: usize = 40;
pub(crate) const CM_SVC_EVAL: usize = 48;
pub(crate) const CM_SVC_INFO: usize = 56;
pub(crate) const CM_SVC_BIND: usize = 64;
pub(crate) const CM_SVC_UNBIND: usize = 72;
pub(crate) const CM_SVC_BOUND: usize = 80;
pub(crate) const CM_SVC_SCALING: usize = 88;

/// service lookup for "tplc-cam" (8 chars)
pub(crate) unsafe fn cam_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-cam".as_ptr() as *const u8, 8) as *const u8
}

#[inline]
pub(crate) unsafe fn cmfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FNcmCreate = unsafe extern "C" fn(*const u8, *mut u32) -> i32;
pub(crate) type FNcmCamOp = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FNcmWrite =
    unsafe extern "C" fn(*const u8, u32, u32, f64, f64) -> i32;
pub(crate) type FNcmConfig = unsafe extern "C" fn(*const u8, u32, u32, u32) -> i32;
pub(crate) type FNcmEval =
    unsafe extern "C" fn(*const u8, u32, f64, *mut f64) -> i32;
pub(crate) type FNcmInfo =
    unsafe extern "C" fn(*const u8, u32, *mut u32, *mut u32) -> i32;
pub(crate) type FNcmBind = unsafe extern "C" fn(
    *const u8, u32, u32, u32, f64, f64, f64, f64,
) -> i32;
pub(crate) type FNcmUnbind = unsafe extern "C" fn(*const u8, u32, u32) -> i32;
pub(crate) type FNcmBound = unsafe extern "C" fn(
    *const u8, u32, *mut u32, *mut u32, *mut f64, *mut f64,
    *mut f64, *mut f64,
) -> i32;
pub(crate) type FNcmScaling = unsafe extern "C" fn(
    *const u8, u32, f64, f64, f64, f64,
) -> i32;

/// `__tplc_rt_FB_Cam` — synchronous op dispatch over the tplc-cam
/// cam-object table and slave-binding registry. OP: 0=create,
/// 1=clear, 2=write point, 3=config, 4=eval, 5=info,
/// 6=bind, 7=bound query, 8=unbind, 9=scaling.
/// STATUS carries the service return code, ERRID its magnitude.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Cam(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(CM_OP));
    wi32(this.add(CM_STATUS), 0);
    wu32(this.add(CM_ERRID), 0);
    let svc = cam_service(ctx);
    if svc.is_null() {
        wi32(this.add(CM_STATUS), -2);
        wu32(this.add(CM_ERRID), 2);
        return 0;
    }
    let fail = |this: *mut u8, rc: i32| {
        wi32(this.add(CM_STATUS), rc);
        wu32(this.add(CM_ERRID), (-rc).max(0) as u32);
    };
    match op {
        0 => {
            let mut out = 0u32;
            let f: FNcmCreate =
                core::mem::transmute(cmfn(svc, CM_SVC_CREATE));
            let rc = f(svc, &mut out);
            if rc == 0 {
                wu32(this.add(CM_HANDLE), out);
            }
            fail(this, rc);
        }
        1 => {
            let f: FNcmCamOp =
                core::mem::transmute(cmfn(svc, CM_SVC_CLEAR));
            fail(this, f(svc, ru32(this.add(CM_HANDLE))));
        }
        2 => {
            let f: FNcmWrite =
                core::mem::transmute(cmfn(svc, CM_SVC_WRITE));
            fail(this, f(svc, ru32(this.add(CM_HANDLE)),
                         ru32(this.add(CM_AUX)),
                         f64::from_bits(ru64(this.add(CM_F1))),
                         f64::from_bits(ru64(this.add(CM_F2)))));
        }
        3 => {
            let f: FNcmConfig =
                core::mem::transmute(cmfn(svc, CM_SVC_CONFIG));
            fail(this, f(svc, ru32(this.add(CM_HANDLE)),
                         ru32(this.add(CM_AUX)),
                         ru32(this.add(CM_FLAGS))));
        }
        4 => {
            let mut out = 0.0f64;
            let f: FNcmEval =
                core::mem::transmute(cmfn(svc, CM_SVC_EVAL));
            let rc = f(svc, ru32(this.add(CM_HANDLE)),
                       f64::from_bits(ru64(this.add(CM_F1))), &mut out);
            fail(this, rc);
            if rc == 0 {
                wu64(this.add(CM_F2), out.to_bits());
            }
        }
        5 => {
            let mut n = 0u32;
            let mut fl = 0u32;
            let f: FNcmInfo =
                core::mem::transmute(cmfn(svc, CM_SVC_INFO));
            let rc = f(svc, ru32(this.add(CM_HANDLE)), &mut n, &mut fl);
            fail(this, rc);
            if rc == 0 {
                wu32(this.add(CM_NPTS), n);
                wu32(this.add(CM_FLAGS), fl);
            }
        }
        6 => {
            let f: FNcmBind =
                core::mem::transmute(cmfn(svc, CM_SVC_BIND));
            fail(this, f(svc, ru32(this.add(CM_AUX)),
                         ru32(this.add(CM_AUX2)),
                         ru32(this.add(CM_HANDLE)),
                         f64::from_bits(ru64(this.add(CM_F1))),
                         f64::from_bits(ru64(this.add(CM_F2))),
                         f64::from_bits(ru64(this.add(CM_F3))),
                         f64::from_bits(ru64(this.add(CM_F4)))));
        }
        7 => {
            let mut cam = 0u32;
            let mut master = 0u32;
            let mut ms = 0.0f64;
            let mut mo = 0.0f64;
            let mut ss = 0.0f64;
            let mut so = 0.0f64;
            let f: FNcmBound =
                core::mem::transmute(cmfn(svc, CM_SVC_BOUND));
            let rc = f(svc, ru32(this.add(CM_AUX)), &mut cam,
                       &mut master, &mut ms, &mut mo, &mut ss, &mut so);
            wi32(this.add(CM_STATUS), rc);
            if rc == 0 {
                wu32(this.add(CM_HANDLE), cam);
                wu32(this.add(CM_AUX2), master);
                wu64(this.add(CM_F1), ms.to_bits());
                wu64(this.add(CM_F2), mo.to_bits());
                wu64(this.add(CM_F3), ss.to_bits());
                wu64(this.add(CM_F4), so.to_bits());
            } else {
                wu32(this.add(CM_ERRID), (-rc).max(0) as u32);
            }
        }
        8 => {
            let f: FNcmUnbind =
                core::mem::transmute(cmfn(svc, CM_SVC_UNBIND));
            fail(this, f(svc, ru32(this.add(CM_AUX)),
                         ru32(this.add(CM_HANDLE))));
        }
        9 => {
            let f: FNcmScaling =
                core::mem::transmute(cmfn(svc, CM_SVC_SCALING));
            fail(this, f(svc, ru32(this.add(CM_AUX)),
                         f64::from_bits(ru64(this.add(CM_F1))),
                         f64::from_bits(ru64(this.add(CM_F2))),
                         f64::from_bits(ru64(this.add(CM_F3))),
                         f64::from_bits(ru64(this.add(CM_F4)))));
        }
        _ => {
            wi32(this.add(CM_STATUS), -1);
            wu32(this.add(CM_ERRID), 1);
        }
    }
    0
}
