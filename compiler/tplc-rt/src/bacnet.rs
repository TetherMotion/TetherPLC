//! tplc-rt builtin FB: bacnet.
use crate::*;

/* ---------------- FB_Bacnet (tplc-bacnet bridge) ---------------- */

// member offsets — keep in lockstep with builtin_fb_layout
// "FB_Bacnet"
pub(crate) const BAC_HOST_LEN: usize = 0;
pub(crate) const BAC_HOST: usize = 1;
pub(crate) const BAC_PORT: usize = 16;
pub(crate) const BAC_FUNC: usize = 20;
pub(crate) const BAC_OBJ_TYPE: usize = 24;
pub(crate) const BAC_OBJ_INST: usize = 28;
pub(crate) const BAC_PROP_ID: usize = 32;
pub(crate) const BAC_HAS_IDX: usize = 36;
pub(crate) const BAC_HAS_PRIO: usize = 37;
pub(crate) const BAC_HAS_RANGE: usize = 38;
pub(crate) const BAC_PROP_IDX: usize = 40;
pub(crate) const BAC_PRIORITY: usize = 44;
pub(crate) const BAC_RANGE_LO: usize = 48;
pub(crate) const BAC_RANGE_HI: usize = 52;
pub(crate) const BAC_VAL_TYPE: usize = 56;
pub(crate) const BAC_VLEN: usize = 60;
pub(crate) const BAC_TIMEOUT: usize = 64;
pub(crate) const BAC_VAL: usize = 72;
pub(crate) const BAC_HANDLE: usize = 328;
pub(crate) const BAC_STATUS: usize = 332;
pub(crate) const BAC_R_SERVICE: usize = 336;
pub(crate) const BAC_R_APDU: usize = 340;
pub(crate) const BAC_R_OBJ_TYPE: usize = 344;
pub(crate) const BAC_R_OBJ_INST: usize = 348;
pub(crate) const BAC_R_PROP_ID: usize = 352;
pub(crate) const BAC_R_VTYPE: usize = 356;
pub(crate) const BAC_R_VLEN: usize = 360;
pub(crate) const BAC_R_VAL: usize = 364;
pub(crate) const BAC_DEV_COUNT: usize = 620;
pub(crate) const BAC_DEV_ID: usize = 624;
pub(crate) const BAC_DEV_IP: usize = 656;
pub(crate) const BAC_DEV_VENDOR: usize = 688;
pub(crate) const BAC_DEV_MAX_APDU: usize = 692;
pub(crate) const BAC_DEV_SEG: usize = 696;
pub(crate) const BAC_ERRID: usize = 700;
pub(crate) const BAC_EXEC: usize = 704;
pub(crate) const BAC_BUSY: usize = 705;
pub(crate) const BAC_DONE: usize = 706;
pub(crate) const BAC_LAST_EXEC: usize = 707;

pub(crate) const BAC_SVC_BEGIN: usize = 16;
pub(crate) const BAC_SVC_POLL: usize = 24;

/// service lookup for "tplc-bacnet" (11 chars)
pub(crate) unsafe fn bacnet_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-bacnet".as_ptr() as *const u8, 11) as *const u8
}

/// load a TplcBacnetService slot as a raw pointer-sized value
#[inline]
pub(crate) unsafe fn bacfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FBacBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FBacPoll =
    unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32, *mut u32, *mut u8, u32) -> i32;
#[inline]
pub(crate) unsafe fn bac_fail(this: *mut u8, errid: u32) {
    wb(this.add(BAC_BUSY), false);
    wb(this.add(BAC_DONE), true);
    wu32(this.add(BAC_HANDLE), 0);
    wu32(this.add(BAC_ERRID), errid);
    wi32(this.add(BAC_STATUS), -1);
}

/// `__tplc_rt_FB_Bacnet` — EXEC rising edge begins one BACnet/IP
/// transaction (FUNC + request fields); while BUSY the transaction
/// is polled each scan until STATUS leaves PENDING or the
/// service-side TIMEOUT. R_*/DEV_* outputs mirror the response.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Bacnet(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(BAC_EXEC));
    let last = rb(this.add(BAC_LAST_EXEC));
    wb(this.add(BAC_LAST_EXEC), exec);

    if !rb(this.add(BAC_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        // rising edge: arm and begin the transaction
        wb(this.add(BAC_DONE), false);
        wu32(this.add(BAC_ERRID), 0);
        wi32(this.add(BAC_STATUS), 0);
        wb(this.add(BAC_BUSY), true);

        let svc = bacnet_service(ctx);
        if svc.is_null() {
            bac_fail(this, 99);
            return 0;
        }
        /* TplcBacnetReq (316 bytes): func u32le @0, peer.host[16]
        @4, peer.port u16le @20, peer.reserved u16 @22, obj_type
        @24, obj_inst @28, prop_id @32, prop_idx @36, val_type @40,
        flags @44, aux0 @48, aux1 @52, val_len u16 @56, timeout_ms
        u16 @58, val[256] @60 */
        let mut req = [0u8; 316];
        req[0..4].copy_from_slice(&ru32(this.add(BAC_FUNC)).to_le_bytes());
        let hn = (*this.add(BAC_HOST_LEN) as usize).min(15);
        core::ptr::copy_nonoverlapping(this.add(BAC_HOST), req.as_mut_ptr().add(4), hn);
        let port = ru32(this.add(BAC_PORT)) as u16;
        req[20..22].copy_from_slice(&port.to_le_bytes());
        req[24..28].copy_from_slice(&ru32(this.add(BAC_OBJ_TYPE)).to_le_bytes());
        req[28..32].copy_from_slice(&ru32(this.add(BAC_OBJ_INST)).to_le_bytes());
        req[32..36].copy_from_slice(&ru32(this.add(BAC_PROP_ID)).to_le_bytes());
        req[36..40].copy_from_slice(&ru32(this.add(BAC_PROP_IDX)).to_le_bytes());
        req[40..44].copy_from_slice(&ru32(this.add(BAC_VAL_TYPE)).to_le_bytes());
        let mut flags: u32 = 0;
        if rb(this.add(BAC_HAS_IDX)) {
            flags |= 1;
        }
        if rb(this.add(BAC_HAS_PRIO)) {
            flags |= 2;
        }
        if rb(this.add(BAC_HAS_RANGE)) {
            flags |= 4;
        }
        req[44..48].copy_from_slice(&flags.to_le_bytes());
        /* aux0 @48: write priority (F_HAS_PRIO) or who-is range
        low (F_HAS_RANGE); aux1 @52: who-is range high */
        if flags & 4 != 0 {
            req[48..52].copy_from_slice(&ru32(this.add(BAC_RANGE_LO)).to_le_bytes());
        } else {
            req[48..52].copy_from_slice(&ru32(this.add(BAC_PRIORITY)).to_le_bytes());
        }
        req[52..56].copy_from_slice(&ru32(this.add(BAC_RANGE_HI)).to_le_bytes());
        req[56..58].copy_from_slice(&(ru32(this.add(BAC_VLEN)).min(256) as u16).to_le_bytes());
        let tns = ri(this.add(BAC_TIMEOUT));
        let tms = if tns > 0 {
            (tns / 1_000_000).min(65535) as u16
        } else {
            0
        };
        req[58..60].copy_from_slice(&tms.to_le_bytes());
        let vlen = ru32(this.add(BAC_VLEN)).min(256) as usize;
        if vlen > 0 {
            core::ptr::copy_nonoverlapping(this.add(BAC_VAL), req.as_mut_ptr().add(60), vlen);
        }
        let mut h: u32 = 0;
        let f: FBacBegin = core::mem::transmute(bacfn(svc, BAC_SVC_BEGIN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            bac_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(BAC_HANDLE), h);
    }

    if !rb(this.add(BAC_BUSY)) {
        return 0;
    }
    let svc = bacnet_service(ctx);
    if svc.is_null() {
        bac_fail(this, 99);
        return 0;
    }
    /* bac_poll(h, info64, val, val_cap, dev_ids, dev_ips, dev_cap)
    writes straight into the instance block: info -> stack copy
    for scalar fields, payloads -> R_VAL/DEV_ID/DEV_IP. */
    let mut info = [0u8; 64];
    let f: FBacPoll = core::mem::transmute(bacfn(svc, BAC_SVC_POLL));
    let rc = f(
        svc,
        ru32(this.add(BAC_HANDLE)),
        info.as_mut_ptr(),
        this.add(BAC_R_VAL),
        256,
        this.add(BAC_DEV_ID) as *mut u32,
        this.add(BAC_DEV_IP),
        8,
    );
    if rc != 0 {
        bac_fail(this, (-rc) as u32);
        return 0;
    }
    /* TplcBacnetRespInfo (64 bytes): status i32 @0, service @4,
    apdu @8, obj_type @12, obj_inst @16, prop_id @20, val_type
    @24, val_len @28, dev_count @32, dev_vendor @36,
    dev_max_apdu @40, dev_seg @44 */
    let st = ri32(info.as_ptr());
    wi32(this.add(BAC_STATUS), st);
    wu32(this.add(BAC_R_SERVICE), ru32(info.as_ptr().add(4)));
    wu32(this.add(BAC_R_APDU), ru32(info.as_ptr().add(8)));
    wu32(this.add(BAC_R_OBJ_TYPE), ru32(info.as_ptr().add(12)));
    wu32(this.add(BAC_R_OBJ_INST), ru32(info.as_ptr().add(16)));
    wu32(this.add(BAC_R_PROP_ID), ru32(info.as_ptr().add(20)));
    wu32(this.add(BAC_R_VTYPE), ru32(info.as_ptr().add(24)));
    wu32(this.add(BAC_R_VLEN), ru32(info.as_ptr().add(28)));
    wu32(this.add(BAC_DEV_COUNT), ru32(info.as_ptr().add(32)));
    wu32(this.add(BAC_DEV_VENDOR), ru32(info.as_ptr().add(36)));
    wu32(this.add(BAC_DEV_MAX_APDU), ru32(info.as_ptr().add(40)));
    wu32(this.add(BAC_DEV_SEG), ru32(info.as_ptr().add(44)));
    if st != 0 {
        wb(this.add(BAC_BUSY), false);
        wb(this.add(BAC_DONE), true);
        wu32(this.add(BAC_HANDLE), 0);
        wu32(this.add(BAC_ERRID), 0);
    }
    0
}
