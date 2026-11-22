//! tplc-rt builtin FB: rtmon.
use crate::*;

/* ---------------- FB_RtMon (tplc-rtmon service bridge) ---------------- */

// TplcRtmonService fn ptrs start @16, eight bytes apart.
pub(crate) const RTM_SVC_MARK: usize = 16;
pub(crate) const RTM_SVC_COUNT: usize = 24;
pub(crate) const RTM_SVC_GET: usize = 32;
pub(crate) const RTM_SVC_CLEAR: usize = 40;

// member offsets — keep in lockstep with builtin_fb_layout "FB_RtMon"
pub(crate) const RTM_OP: usize = 0;
pub(crate) const RTM_CONTEXT: usize = 4;
pub(crate) const RTM_GROUP: usize = 8;
pub(crate) const RTM_EVENT: usize = 12;
pub(crate) const RTM_MARKCTRL: usize = 16;
pub(crate) const RTM_AUX: usize = 20;
pub(crate) const RTM_RESULT: usize = 24;
pub(crate) const RTM_ERRID: usize = 28;
pub(crate) const RTM_EXEC: usize = 32;
pub(crate) const RTM_BUSY: usize = 33;
pub(crate) const RTM_DONE: usize = 34;
pub(crate) const RTM_LAST_EXEC: usize = 35;

pub(crate) const RTM_MARK: i32 = 1;
pub(crate) const RTM_COUNT: i32 = 2;
pub(crate) const RTM_GET: i32 = 3;
pub(crate) const RTM_CLEAR: i32 = 4;

// mirrored TplcRtmonService result codes (positive form for ERRID);
// range errors (-3) arrive via the service rc and map the same way
pub(crate) const RTMERR_PARAM: u32 = 1;
pub(crate) const RTMERR_NOSERVICE: u32 = 2;

pub(crate) type FRtmMark = unsafe extern "C" fn(*const u8, u32, u32, u32, u32) -> i32;
pub(crate) type FRtmCount = unsafe extern "C" fn(*const u8, *mut u32) -> i32;
pub(crate) type FRtmGet = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FRtmClear = unsafe extern "C" fn(*const u8) -> i32;

pub(crate) unsafe fn rtm_fail(this: *mut u8, errid: u32) {
    wb(this.add(RTM_BUSY), false);
    wb(this.add(RTM_DONE), true);
    wu32(this.add(RTM_ERRID), errid);
    wi32(this.add(RTM_RESULT), 0);
}

pub(crate) unsafe fn rtm_done(this: *mut u8, result: i32) {
    wb(this.add(RTM_BUSY), false);
    wb(this.add(RTM_DONE), true);
    wu32(this.add(RTM_ERRID), 0);
    wi32(this.add(RTM_RESULT), result);
}

/// resolve the tplc-rtmon service table through ctx->svc->get_service
pub(crate) unsafe fn rtmon_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-rtmon".as_ptr() as *const u8, 10) as *const u8
}

/// load a TplcRtmonService slot as a raw pointer-sized value
#[inline]
pub(crate) unsafe fn rtmfn(rtmon: *const u8, off: usize) -> usize {
    (rtmon.add(off) as *const usize).read()
}

/// `__tplc_rt_FB_RtMon` — EXEC rising edge performs one synchronous
/// op (mark append, count, or indexed read) and sets DONE the same
/// scan. RESULT carries the op result; ERRID carries service errors.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_RtMon(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(RTM_EXEC));
    let last = rb(this.add(RTM_LAST_EXEC));
    wb(this.add(RTM_LAST_EXEC), exec);
    if !(exec && !last) {
        return 0;
    }
    wb(this.add(RTM_DONE), false);
    wu32(this.add(RTM_ERRID), 0);
    wi32(this.add(RTM_RESULT), 0);
    wb(this.add(RTM_BUSY), true);

    let rtmon = rtmon_service(ctx);
    if rtmon.is_null() {
        rtm_fail(this, RTMERR_NOSERVICE);
        return 0;
    }

    match ri32(this.add(RTM_OP)) {
        RTM_MARK => {
            let f: FRtmMark = core::mem::transmute(rtmfn(rtmon, RTM_SVC_MARK));
            let rc = f(
                rtmon,
                ru32(this.add(RTM_CONTEXT)),
                ru32(this.add(RTM_GROUP)),
                ru32(this.add(RTM_EVENT)),
                ru32(this.add(RTM_MARKCTRL)),
            );
            if rc < 0 {
                rtm_fail(this, (-rc) as u32);
            } else {
                rtm_done(this, rc);
            }
        }
        RTM_COUNT => {
            let mut n: u32 = 0;
            let f: FRtmCount = core::mem::transmute(rtmfn(rtmon, RTM_SVC_COUNT));
            let rc = f(rtmon, &mut n);
            if rc < 0 {
                rtm_fail(this, (-rc) as u32);
            } else {
                rtm_done(this, rc);
                /* RESULT is DINT; counts past i32::MAX wrap, which is
                acceptable for a diagnostic ring. Written after
                rtm_done so the HRESULT slot doesn't clobber it. */
                wi32(this.add(RTM_RESULT), n as i32);
            }
        }
        RTM_GET => {
            /* TplcRtmonMark: u64 ts @0, u32 context @8, group @12,
            event @16, markctrl @20 (24 bytes) */
            let mut m = [0u8; 24];
            let f: FRtmGet = core::mem::transmute(rtmfn(rtmon, RTM_SVC_GET));
            let rc = f(rtmon, ru32(this.add(RTM_AUX)), m.as_mut_ptr());
            if rc < 0 {
                rtm_fail(this, (-rc) as u32);
            } else {
                wu32(this.add(RTM_CONTEXT), ru32(m.as_ptr().add(8)));
                wu32(this.add(RTM_GROUP), ru32(m.as_ptr().add(12)));
                wu32(this.add(RTM_EVENT), ru32(m.as_ptr().add(16)));
                wu32(this.add(RTM_MARKCTRL), ru32(m.as_ptr().add(20)));
                rtm_done(this, rc);
            }
        }
        RTM_CLEAR => {
            let f: FRtmClear = core::mem::transmute(rtmfn(rtmon, RTM_SVC_CLEAR));
            let rc = f(rtmon);
            if rc < 0 {
                rtm_fail(this, (-rc) as u32);
            } else {
                rtm_done(this, rc);
            }
        }
        _ => rtm_fail(this, RTMERR_PARAM),
    }
    0
}
