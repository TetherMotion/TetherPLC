//! tplc-rt builtin FB: ads.
use crate::*;

/* ---------------- FB_Ads (tplc-ads service bridge) ---------------- */

// TplcAdsService fn ptrs start @16, eight bytes apart.
pub(crate) const ADS_OPEN: usize = 16;
pub(crate) const ADS_READ: usize = 24;
pub(crate) const ADS_WRITE: usize = 32;
pub(crate) const ADS_READ_STATE: usize = 40;
pub(crate) const ADS_WRITE_CTRL: usize = 48;
pub(crate) const ADS_CLOSE: usize = 56;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Ads"
pub(crate) const AD_OP: usize = 0;
pub(crate) const AD_HOST_LEN: usize = 4;
pub(crate) const AD_HOST: usize = 5;
pub(crate) const AD_PORT: usize = 20;
pub(crate) const AD_IDXGRP: usize = 24;
pub(crate) const AD_IDXOFF: usize = 28;
pub(crate) const AD_STATE: usize = 32;
pub(crate) const AD_DEVSTATE: usize = 36;
pub(crate) const AD_TIMEOUT: usize = 48;
pub(crate) const AD_DATA_LEN: usize = 56;
pub(crate) const AD_DATA: usize = 57;
pub(crate) const AD_DLEN: usize = 316;
pub(crate) const AD_HANDLE: usize = 320;
pub(crate) const AD_RESULT: usize = 324;
pub(crate) const AD_ERRID: usize = 328;
pub(crate) const AD_EXEC: usize = 332;
pub(crate) const AD_BUSY: usize = 333;
pub(crate) const AD_DONE: usize = 334;
pub(crate) const AD_LAST_EXEC: usize = 335;
pub(crate) const AD_DEADLINE: usize = 336;
pub(crate) const AD_SENT: usize = 344;

pub(crate) const ADS_OP_OPEN: i32 = 1;
pub(crate) const ADS_OP_READ: i32 = 2;
pub(crate) const ADS_OP_WRITE: i32 = 3;
pub(crate) const ADS_OP_READ_STATE: i32 = 4;
pub(crate) const ADS_OP_WRITE_CTRL: i32 = 5;
pub(crate) const ADS_OP_CLOSE: i32 = 6;

// mirrored TplcAdsService result codes (positive form for ERRID)
pub(crate) const ADSERR_PARAM: u32 = 1;
pub(crate) const ADSERR_NOSERVICE: u32 = 2;
pub(crate) const ADSERR_TIMEOUT: u32 = 5;

pub(crate) type FAdsOpen = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
pub(crate) type FAdsRead =
    unsafe extern "C" fn(*const u8, u32, u32, u32, u32, *mut u8, u32, *mut u32, *mut u32) -> i32;
pub(crate) type FAdsWrite = unsafe extern "C" fn(*const u8, u32, u32, u32, *const u8, u32, *mut u32) -> i32;
pub(crate) type FAdsReadState = unsafe extern "C" fn(*const u8, u32, *mut u32, *mut u32, *mut u32) -> i32;
pub(crate) type FAdsWriteCtrl = unsafe extern "C" fn(*const u8, u32, u32, u32, *mut u32) -> i32;
pub(crate) type FAdsClose = unsafe extern "C" fn(*const u8, u32) -> i32;

pub(crate) unsafe fn ads_fail(this: *mut u8, errid: u32) {
    wb(this.add(AD_BUSY), false);
    wb(this.add(AD_DONE), true);
    wu32(this.add(AD_ERRID), errid);
    wi32(this.add(AD_RESULT), 0);
}

pub(crate) unsafe fn ads_done(this: *mut u8, result: i32) {
    wb(this.add(AD_BUSY), false);
    wb(this.add(AD_DONE), true);
    wu32(this.add(AD_ERRID), 0);
    wi32(this.add(AD_RESULT), result);
}

/// copy the len-prefixed HOST member into a NUL-terminated buffer
pub(crate) unsafe fn ad_host_cstr(this: *const u8, out: *mut u8) {
    let n = (*this.add(AD_HOST_LEN) as usize).min(15);
    core::ptr::copy_nonoverlapping(this.add(AD_HOST), out, n);
    *out.add(n) = 0;
}

/// resolve the tplc-ads service table through ctx->svc->get_service
pub(crate) unsafe fn ads_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-ads".as_ptr() as *const u8, 8) as *const u8
}

/// load a TplcAdsService slot as a raw pointer-sized value
#[inline]
pub(crate) unsafe fn adsfn(ads: *const u8, off: usize) -> usize {
    (ads.add(off) as *const usize).read()
}

/// `__tplc_rt_FB_Ads` — EXEC rising edge arms an op; while BUSY the
/// op is polled each scan until DONE or the TIMEOUT deadline. RESULT
/// carries the ADS-layer error code; ERRID carries service errors.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Ads(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(AD_EXEC));
    let last = rb(this.add(AD_LAST_EXEC));
    let now = now_ns(ctx);

    if !rb(this.add(AD_BUSY)) {
        if !(exec && !last) {
            wb(this.add(AD_LAST_EXEC), exec);
            return 0;
        }
        // rising edge: arm the op
        wb(this.add(AD_DONE), false);
        wu32(this.add(AD_ERRID), 0);
        wi32(this.add(AD_RESULT), 0);
        wu32(this.add(AD_SENT), 0);
        wb(this.add(AD_BUSY), true);
        let to = ri(this.add(AD_TIMEOUT));
        wi(
            this.add(AD_DEADLINE),
            if to > 0 {
                now.saturating_add(to)
            } else {
                i64::MAX
            },
        );
    }
    wb(this.add(AD_LAST_EXEC), exec);

    let ads = ads_service(ctx);
    if ads.is_null() {
        ads_fail(this, ADSERR_NOSERVICE);
        return 0;
    }
    if now > ri(this.add(AD_DEADLINE)) {
        ads_fail(this, ADSERR_TIMEOUT);
        return 0;
    }

    let op = ri32(this.add(AD_OP));
    let mut hbuf = [0u8; 16];
    match op {
        ADS_OP_OPEN => {
            ad_host_cstr(this, hbuf.as_mut_ptr());
            let mut out: u32 = 0;
            let f: FAdsOpen = core::mem::transmute(adsfn(ads, ADS_OPEN));
            let rc = f(ads, hbuf.as_ptr(), ru32(this.add(AD_PORT)) as u16, &mut out);
            if rc < 0 {
                ads_fail(this, (-rc) as u32);
            } else {
                wu32(this.add(AD_HANDLE), out);
                ads_done(this, 0);
            }
        }
        ADS_OP_READ => {
            let req_len = ru32(this.add(AD_DLEN)).min(256);
            let mut rlen: u32 = 0;
            let mut adserr: u32 = 0;
            let f: FAdsRead = core::mem::transmute(adsfn(ads, ADS_READ));
            let rc = f(
                ads,
                ru32(this.add(AD_HANDLE)),
                ru32(this.add(AD_IDXGRP)),
                ru32(this.add(AD_IDXOFF)),
                req_len,
                this.add(AD_DATA),
                256,
                &mut rlen,
                &mut adserr,
            );
            match rc {
                0 => {
                    *this.add(AD_DATA_LEN) = rlen.min(256) as u8;
                    wu32(this.add(AD_DLEN), rlen);
                    ads_done(this, adserr as i32);
                }
                -6 => {}
                e => ads_fail(this, (-e) as u32),
            }
        }
        ADS_OP_WRITE => {
            let dlen = ru32(this.add(AD_DLEN)) as usize;
            let slen = (*this.add(AD_DATA_LEN) as usize).min(256);
            let total = if dlen > 0 { dlen.min(256) } else { slen };
            let mut adserr: u32 = 0;
            let f: FAdsWrite = core::mem::transmute(adsfn(ads, ADS_WRITE));
            let rc = f(
                ads,
                ru32(this.add(AD_HANDLE)),
                ru32(this.add(AD_IDXGRP)),
                ru32(this.add(AD_IDXOFF)),
                this.add(AD_DATA),
                total as u32,
                &mut adserr,
            );
            match rc {
                0 => ads_done(this, adserr as i32),
                -6 => {}
                e => ads_fail(this, (-e) as u32),
            }
        }
        ADS_OP_READ_STATE => {
            let mut ads_state: u32 = 0;
            let mut dev_state: u32 = 0;
            let mut adserr: u32 = 0;
            let f: FAdsReadState = core::mem::transmute(adsfn(ads, ADS_READ_STATE));
            let rc = f(
                ads,
                ru32(this.add(AD_HANDLE)),
                &mut ads_state,
                &mut dev_state,
                &mut adserr,
            );
            match rc {
                0 => {
                    wi32(this.add(AD_STATE), ads_state as i32);
                    wi32(this.add(AD_DEVSTATE), dev_state as i32);
                    ads_done(this, adserr as i32);
                }
                -6 => {}
                e => ads_fail(this, (-e) as u32),
            }
        }
        ADS_OP_WRITE_CTRL => {
            let mut adserr: u32 = 0;
            let f: FAdsWriteCtrl = core::mem::transmute(adsfn(ads, ADS_WRITE_CTRL));
            let rc = f(
                ads,
                ru32(this.add(AD_HANDLE)),
                ru32(this.add(AD_STATE)),
                ru32(this.add(AD_DEVSTATE)),
                &mut adserr,
            );
            match rc {
                0 => ads_done(this, adserr as i32),
                -6 => {}
                e => ads_fail(this, (-e) as u32),
            }
        }
        ADS_OP_CLOSE => {
            let f: FAdsClose = core::mem::transmute(adsfn(ads, ADS_CLOSE));
            f(ads, ru32(this.add(AD_HANDLE)));
            ads_done(this, 0);
        }
        _ => ads_fail(this, ADSERR_PARAM),
    }
    0
}
