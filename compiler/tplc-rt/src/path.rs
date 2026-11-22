//! tplc-rt builtin FB: path.
use crate::*;

// ---------------------------------------------------------------------------
// FB_Path — tplc-path service bridge (Tc3_McCoordinatedMotion path
// objects executed by MC_MovePath).
// ---------------------------------------------------------------------------

// FB_Path member offsets (see builtin_fb_layout("FB_Path")).
pub(crate) const PT_OP: usize = 0;
pub(crate) const PT_HANDLE: usize = 4;
pub(crate) const PT_AUX: usize = 8;
pub(crate) const PT_AUX2: usize = 12;
pub(crate) const PT_FLAGS: usize = 16;
pub(crate) const PT_STATUS: usize = 20;
pub(crate) const PT_ERRID: usize = 24;
pub(crate) const PT_NSEG: usize = 28;
pub(crate) const PT_DONE: usize = 32;
pub(crate) const PT_NOW: usize = 40;  // f64
pub(crate) const PT_F1: usize = 48;   // f64
pub(crate) const PT_F2: usize = 56;   // f64
pub(crate) const PT_F3: usize = 64;   // f64
pub(crate) const PT_POS: usize = 72;  // f64[16]
pub(crate) const PT_AUXP: usize = 200; // f64[16]
pub(crate) const PT_TRID: usize = 328; // u32[10]
pub(crate) const PT_TACT: usize = 368; // u32[10]

// tplc-path service vtable offsets (TplcPathService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const PT_SVC_CREATE: usize = 16;
pub(crate) const PT_SVC_CLEAR: usize = 24;
pub(crate) const PT_SVC_ADD: usize = 32;
pub(crate) const PT_SVC_SET_START: usize = 40;
pub(crate) const PT_SVC_EVAL: usize = 48;
pub(crate) const PT_SVC_INFO: usize = 56;
pub(crate) const PT_SVC_ADDCIRC: usize = 64;
pub(crate) const PT_SVC_RELEASE: usize = 72;
pub(crate) const PT_SVC_BLOCKER: usize = 80;
pub(crate) const PT_SVC_BINDGROUP: usize = 88;
pub(crate) const PT_SVC_TRAJ: usize = 96;
pub(crate) const PT_SVC_TRAFO: usize = 104;

/// service lookup for "tplc-path" (9 chars)
pub(crate) unsafe fn path_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-path".as_ptr() as *const u8, 9) as *const u8
}

#[inline]
pub(crate) unsafe fn ptfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FNptCreate = unsafe extern "C" fn(*const u8, *mut u32) -> i32;
pub(crate) type FNptPathOp = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FNptAdd = unsafe extern "C" fn(
    *const u8, u32, u32, *const f64, u32, f64, f64, f64,
) -> i32;
pub(crate) type FNptSetStart =
    unsafe extern "C" fn(*const u8, u32, *const f64, u32) -> i32;
pub(crate) type FNptEval = unsafe extern "C" fn(
    *const u8, u32, f64, *mut f64, u32, *mut u32, *mut i32,
    *mut u32,
) -> i32;
pub(crate) type FNptInfo =
    unsafe extern "C" fn(*const u8, u32, *mut u32, *mut f64) -> i32;
pub(crate) type FNptAddCirc = unsafe extern "C" fn(
    *const u8, u32, u32, *const f64, u32, *const f64, u32, f64,
    f64, f64,
) -> i32;
pub(crate) type FNptRelease = unsafe extern "C" fn(*const u8, f64) -> i32;
pub(crate) type FNptBlocker =
    unsafe extern "C" fn(*const u8, *mut f64, *mut i32) -> i32;
pub(crate) type FNptBindGroup = unsafe extern "C" fn(*const u8, u32, u32) -> i32;
pub(crate) type FNptTraj = unsafe extern "C" fn(
    *const u8, u32, *mut f64, *mut f64, *mut u32, *mut u32, u32,
    *mut u32,
) -> i32;
pub(crate) type FNptTrafo =
    unsafe extern "C" fn(*const u8, u32, *mut f64, *mut u32) -> i32;

/// `__tplc_rt_FB_Path` — synchronous op dispatch over the tplc-path
/// path-object table. OP: 0=create, 1=clear, 2=add segment,
/// 3=set start, 4=eval, 5=info. NOW is refreshed from ctx->now_ns
/// on every call. STATUS carries the service return code, ERRID its
/// magnitude.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Path(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(PT_OP));
    wi32(this.add(PT_STATUS), 0);
    wu32(this.add(PT_ERRID), 0);
    /* NOW: ctx->now_ns as seconds (f64) */
    let now_s = (now_ns(ctx) as f64) / 1.0e9;
    wu64(this.add(PT_NOW), now_s.to_bits());
    let svc = path_service(ctx);
    if svc.is_null() {
        wi32(this.add(PT_STATUS), -2);
        wu32(this.add(PT_ERRID), 2);
        return 0;
    }
    let fail = |this: *mut u8, rc: i32| {
        wi32(this.add(PT_STATUS), rc);
        wu32(this.add(PT_ERRID), (-rc).max(0) as u32);
    };
    match op {
        0 => {
            let mut out = 0u32;
            let f: FNptCreate =
                core::mem::transmute(ptfn(svc, PT_SVC_CREATE));
            let rc = f(svc, &mut out);
            if rc == 0 {
                wu32(this.add(PT_HANDLE), out);
            }
            fail(this, rc);
        }
        1 => {
            let f: FNptPathOp =
                core::mem::transmute(ptfn(svc, PT_SVC_CLEAR));
            fail(this, f(svc, ru32(this.add(PT_HANDLE))));
        }
        2 => {
            let f: FNptAdd =
                core::mem::transmute(ptfn(svc, PT_SVC_ADD));
            fail(this, f(svc, ru32(this.add(PT_HANDLE)),
                         ru32(this.add(PT_AUX)),
                         this.add(PT_POS) as *const f64,
                         ru32(this.add(PT_AUX2)),
                         f64::from_bits(ru64(this.add(PT_F1))),
                         f64::from_bits(ru64(this.add(PT_F2))),
                         f64::from_bits(ru64(this.add(PT_F3)))));
        }
        3 => {
            let f: FNptSetStart =
                core::mem::transmute(ptfn(svc, PT_SVC_SET_START));
            fail(this, f(svc, ru32(this.add(PT_HANDLE)),
                         this.add(PT_POS) as *const f64,
                         ru32(this.add(PT_AUX))));
        }
        4 => {
            let mut seg = 0u32;
            let mut done = 0i32;
            let mut nout = 0u32;
            let f: FNptEval =
                core::mem::transmute(ptfn(svc, PT_SVC_EVAL));
            let rc = f(svc, ru32(this.add(PT_HANDLE)),
                       f64::from_bits(ru64(this.add(PT_F1))),
                       this.add(PT_POS) as *mut f64, 16,
                       &mut seg, &mut done, &mut nout);
            fail(this, rc);
            if rc == 0 {
                wu32(this.add(PT_AUX), seg);
                wu32(this.add(PT_DONE), done.max(0) as u32);
                wu32(this.add(PT_NSEG), nout);
            }
        }
        5 => {
            let mut ns = 0u32;
            let mut dur = 0.0f64;
            let f: FNptInfo =
                core::mem::transmute(ptfn(svc, PT_SVC_INFO));
            let rc = f(svc, ru32(this.add(PT_HANDLE)), &mut ns, &mut dur);
            fail(this, rc);
            if rc == 0 {
                wu32(this.add(PT_NSEG), ns);
                wu64(this.add(PT_F1), dur.to_bits());
            }
        }
        6 => {
            let f: FNptAddCirc =
                core::mem::transmute(ptfn(svc, PT_SVC_ADDCIRC));
            fail(this, f(svc, ru32(this.add(PT_HANDLE)),
                         ru32(this.add(PT_AUX)),
                         this.add(PT_AUXP) as *const f64,
                         ru32(this.add(PT_FLAGS)),
                         this.add(PT_POS) as *const f64,
                         ru32(this.add(PT_AUX2)),
                         f64::from_bits(ru64(this.add(PT_F1))),
                         f64::from_bits(ru64(this.add(PT_F2))),
                         f64::from_bits(ru64(this.add(PT_F3)))));
        }
        7 => {
            /* blocker wait query: AUX=waiting, F1=blocker id */
            let mut id = 0.0f64;
            let mut w = 0i32;
            let f: FNptBlocker =
                core::mem::transmute(ptfn(svc, PT_SVC_BLOCKER));
            let rc = f(svc, &mut id, &mut w);
            fail(this, rc);
            if rc == 0 {
                wu32(this.add(PT_AUX), w.max(0) as u32);
                wu64(this.add(PT_F1), id.to_bits());
            }
        }
        8 => {
            /* release blocker id carried in F1 */
            let f: FNptRelease =
                core::mem::transmute(ptfn(svc, PT_SVC_RELEASE));
            fail(this, f(svc, f64::from_bits(ru64(this.add(PT_F1)))));
        }
        9 => {
            /* bind path handle to group AUX */
            let f: FNptBindGroup =
                core::mem::transmute(ptfn(svc, PT_SVC_BINDGROUP));
            fail(this, f(svc, ru32(this.add(PT_HANDLE)),
                         ru32(this.add(PT_AUX))));
        }
        11 => {
            /* pending coordinate transform: F1=id, AUX2=gen */
            let mut id = 0.0f64;
            let mut gen = 0u32;
            let f: FNptTrafo =
                core::mem::transmute(ptfn(svc, PT_SVC_TRAFO));
            let rc = f(svc, ru32(this.add(PT_HANDLE)), &mut id,
                       &mut gen);
            fail(this, rc);
            if rc == 0 {
                wu64(this.add(PT_F1), id.to_bits());
                wu32(this.add(PT_AUX2), gen);
            }
        }
        10 => {
            /* trajectory readback for group AUX: POS=remLen f64[16],
               AUXP=remDur f64[16], TRID=u32[10] ids, TACT=u32[10]
               active, NSEG=count */
            let mut cnt = 0u32;
            let f: FNptTraj =
                core::mem::transmute(ptfn(svc, PT_SVC_TRAJ));
            let rc = f(svc, ru32(this.add(PT_AUX)),
                       this.add(PT_POS) as *mut f64,
                       this.add(PT_AUXP) as *mut f64,
                       this.add(PT_TRID) as *mut u32,
                       this.add(PT_TACT) as *mut u32, 10, &mut cnt);
            fail(this, rc);
            if rc == 0 {
                wu32(this.add(PT_NSEG), cnt);
            }
        }
        _ => {
            wi32(this.add(PT_STATUS), -1);
            wu32(this.add(PT_ERRID), 1);
        }
    }
    0
}
