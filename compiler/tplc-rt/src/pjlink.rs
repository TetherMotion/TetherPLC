//! tplc-rt builtin FB: pjlink.
use crate::*;

/* ---------------- FB_Pjlink (tplc-pjlink bridge) ---------------- */

// member offsets — keep in lockstep with builtin_fb_layout
// "FB_Pjlink"
pub(crate) const PJL_HOST_LEN: usize = 0;
pub(crate) const PJL_HOST: usize = 1;
pub(crate) const PJL_PORT: usize = 16;
pub(crate) const PJL_CLS: usize = 20;
pub(crate) const PJL_CMD_LEN: usize = 24;
pub(crate) const PJL_CMD: usize = 25;
pub(crate) const PJL_PARAM_LEN: usize = 30;
pub(crate) const PJL_PARAM: usize = 31;
pub(crate) const PJL_PASS_LEN: usize = 160;
pub(crate) const PJL_PASS: usize = 161;
pub(crate) const PJL_TIMEOUT: usize = 200;
pub(crate) const PJL_HANDLE: usize = 208;
pub(crate) const PJL_STATUS: usize = 212;
pub(crate) const PJL_ERRID: usize = 216;
pub(crate) const PJL_R_ERR: usize = 220;
pub(crate) const PJL_R_CLS: usize = 224;
pub(crate) const PJL_R_CMD: usize = 228;
pub(crate) const PJL_R_VLEN: usize = 232;
pub(crate) const PJL_R_VALUE_LEN: usize = 236;
pub(crate) const PJL_R_VALUE: usize = 237;
pub(crate) const PJL_AUTHED: usize = 364;
pub(crate) const PJL_EXEC: usize = 365;
pub(crate) const PJL_BUSY: usize = 366;
pub(crate) const PJL_DONE: usize = 367;
pub(crate) const PJL_LAST_EXEC: usize = 368;

pub(crate) const PJL_SVC_BEGIN: usize = 16;
pub(crate) const PJL_SVC_POLL: usize = 24;

/// service lookup for "tplc-pjlink" (11 chars)
pub(crate) unsafe fn pjlink_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-pjlink".as_ptr() as *const u8, 11) as *const u8
}

#[inline]
pub(crate) unsafe fn pjlfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FPjBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FPjPoll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
#[inline]
pub(crate) unsafe fn pjl_fail(this: *mut u8, errid: u32) {
    wb(this.add(PJL_BUSY), false);
    wb(this.add(PJL_DONE), true);
    wu32(this.add(PJL_HANDLE), 0);
    wu32(this.add(PJL_ERRID), errid);
    wi32(this.add(PJL_STATUS), -1);
}

/// `__tplc_rt_FB_Pjlink` — EXEC rising edge begins one PJLink
/// transaction; while BUSY the transaction is polled each scan
/// until STATUS leaves PENDING or the service-side TIMEOUT.
/// R_VALUE is the NUL-terminated response value text stored as an
/// IEC STRING (len byte + chars), R_CLS/R_CMD echo the response
/// header (R_CMD packed big-endian ASCII).
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Pjlink(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(PJL_EXEC));
    let last = rb(this.add(PJL_LAST_EXEC));
    wb(this.add(PJL_LAST_EXEC), exec);

    if !rb(this.add(PJL_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        // rising edge: arm and begin the transaction
        wb(this.add(PJL_DONE), false);
        wu32(this.add(PJL_ERRID), 0);
        wi32(this.add(PJL_STATUS), 0);
        wb(this.add(PJL_BUSY), true);

        let svc = pjlink_service(ctx);
        if svc.is_null() {
            pjl_fail(this, 99);
            return 0;
        }
        /* TplcPjlinkReq (196 bytes): host[16] @0, port u16 @16,
        reserved @18, cls u32 @20, cmd[5] @24, param[129] @29,
        timeout_ms u16 @158, reserved2 @160, password[33] @162 */
        let mut req = [0u8; 196];
        let hn = (*this.add(PJL_HOST_LEN) as usize).min(15);
        core::ptr::copy_nonoverlapping(this.add(PJL_HOST), req.as_mut_ptr(), hn);
        let port = ru32(this.add(PJL_PORT)) as u16;
        req[16..18].copy_from_slice(&port.to_le_bytes());
        req[20..24].copy_from_slice(&ru32(this.add(PJL_CLS)).to_le_bytes());
        let cn = (*this.add(PJL_CMD_LEN) as usize).min(4);
        if cn > 0 {
            core::ptr::copy_nonoverlapping(this.add(PJL_CMD), req.as_mut_ptr().add(24), cn);
        }
        let pn = (*this.add(PJL_PARAM_LEN) as usize).min(128);
        if pn > 0 {
            core::ptr::copy_nonoverlapping(this.add(PJL_PARAM), req.as_mut_ptr().add(29), pn);
        }
        let tns = ri(this.add(PJL_TIMEOUT));
        let tms = if tns > 0 {
            (tns / 1_000_000).min(65535) as u16
        } else {
            0
        };
        req[158..160].copy_from_slice(&tms.to_le_bytes());
        let wn = (*this.add(PJL_PASS_LEN) as usize).min(32);
        if wn > 0 {
            core::ptr::copy_nonoverlapping(this.add(PJL_PASS), req.as_mut_ptr().add(162), wn);
        }
        let mut h: u32 = 0;
        let f: FPjBegin = core::mem::transmute(pjlfn(svc, PJL_SVC_BEGIN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            pjl_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(PJL_HANDLE), h);
    }

    if !rb(this.add(PJL_BUSY)) {
        return 0;
    }
    let svc = pjlink_service(ctx);
    if svc.is_null() {
        pjl_fail(this, 99);
        return 0;
    }
    /* pjl_poll(h, info64, val, val_cap) — info to stack, value text
    through a scratch buffer then into the R_VALUE string slot. */
    let mut info = [0u8; 64];
    let mut vbuf = [0u8; 160];
    let f: FPjPoll = core::mem::transmute(pjlfn(svc, PJL_SVC_POLL));
    let rc = f(
        svc,
        ru32(this.add(PJL_HANDLE)),
        info.as_mut_ptr(),
        vbuf.as_mut_ptr(),
        160,
    );
    if rc != 0 {
        pjl_fail(this, (-rc) as u32);
        return 0;
    }
    /* TplcPjlinkRespInfo (64 bytes): status i32 @0, err i32 @4,
    resp_cls u32 @8, resp_cmd u32 @12, value_len u32 @16,
    authed u32 @20 */
    let st = ri32(info.as_ptr());
    wi32(this.add(PJL_STATUS), st);
    wi32(this.add(PJL_R_ERR), ri32(info.as_ptr().add(4)));
    wu32(this.add(PJL_R_CLS), ru32(info.as_ptr().add(8)));
    wu32(this.add(PJL_R_CMD), ru32(info.as_ptr().add(12)));
    wu32(this.add(PJL_R_VLEN), ru32(info.as_ptr().add(16)));
    wb(this.add(PJL_AUTHED), ru32(info.as_ptr().add(20)) != 0);
    /* copy the NUL-terminated value into R_VALUE (len byte + chars) */
    let mut vl = 0usize;
    while vl < 127 && vbuf[vl] != 0 {
        vl += 1;
    }
    *this.add(PJL_R_VALUE_LEN) = vl as u8;
    if vl > 0 {
        core::ptr::copy_nonoverlapping(vbuf.as_ptr(), this.add(PJL_R_VALUE), vl);
    }
    if st != 0 {
        wb(this.add(PJL_BUSY), false);
        wb(this.add(PJL_DONE), true);
        wu32(this.add(PJL_HANDLE), 0);
        wu32(this.add(PJL_ERRID), 0);
    }
    0
}
