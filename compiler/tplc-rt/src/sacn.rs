//! tplc-rt builtin FB: sacn.
use crate::*;

/* ------------- FB_SacnSource / FB_SacnReceiver (tplc-sacn) ------------- */

// TplcSacnService fn ptrs start @16, eight bytes apart.
pub(crate) const SACN_SVC_SRC_BEGIN: usize = 16;
pub(crate) const SACN_SVC_SRC_POLL: usize = 24;
pub(crate) const SACN_SVC_SRC_CLOSE: usize = 32;
pub(crate) const SACN_SVC_RCV_BEGIN: usize = 40;
pub(crate) const SACN_SVC_RCV_POLL: usize = 48;
pub(crate) const SACN_SVC_RCV_CLOSE: usize = 56;

// member offsets — keep in lockstep with builtin_fb_layout
// "FB_SacnSource"
pub(crate) const SRC_ENABLE: usize = 0;
pub(crate) const SRC_UNIVERSE: usize = 4;
pub(crate) const SRC_PRIORITY: usize = 8;
pub(crate) const SRC_SYNC_ADDR: usize = 12;
pub(crate) const SRC_INTERVAL: usize = 16;
pub(crate) const SRC_TTL: usize = 20;
pub(crate) const SRC_OPTIONS: usize = 24;
pub(crate) const SRC_FLAGS: usize = 25;
pub(crate) const SRC_START_CODE: usize = 26;
pub(crate) const SRC_NAME_LEN: usize = 28;
pub(crate) const SRC_NAME: usize = 29;
pub(crate) const SRC_CID: usize = 92;
pub(crate) const SRC_DLEN: usize = 108;
pub(crate) const SRC_DMX: usize = 112;
pub(crate) const SRC_HANDLE: usize = 624;
pub(crate) const SRC_SEQ: usize = 628;
pub(crate) const SRC_ERRID: usize = 632;
pub(crate) const SRC_LAST_ERR: usize = 636;
pub(crate) const SRC_SENT: usize = 640;
pub(crate) const SRC_AUX: usize = 648;
pub(crate) const SRC_BUSY: usize = 656;
pub(crate) const SRC_ERR_LATCH: usize = 657;

// member offsets — keep in lockstep with builtin_fb_layout
// "FB_SacnReceiver"
pub(crate) const RCV_ENABLE: usize = 0;
pub(crate) const RCV_UNIVERSE: usize = 4;
pub(crate) const RCV_SYNC_UNIVERSE: usize = 8;
pub(crate) const RCV_PORT: usize = 12;
pub(crate) const RCV_DATA_RX: usize = 16;
pub(crate) const RCV_DATA_DROP: usize = 24;
pub(crate) const RCV_SYNC_RX: usize = 32;
pub(crate) const RCV_DISC_RX: usize = 40;
pub(crate) const RCV_REMOTE_IP: usize = 48;
pub(crate) const RCV_HANDLE: usize = 52;
pub(crate) const RCV_ERRID: usize = 56;
pub(crate) const RCV_DLEN: usize = 60;
pub(crate) const RCV_SYNC_ADDR: usize = 64;
pub(crate) const RCV_UNI: usize = 66;
pub(crate) const RCV_SYNC_ADDR_SEEN: usize = 68;
pub(crate) const RCV_UNI_COUNT: usize = 70;
pub(crate) const RCV_START_CODE: usize = 72;
pub(crate) const RCV_SEQ: usize = 73;
pub(crate) const RCV_PRIORITY: usize = 74;
pub(crate) const RCV_SYNC_SEQ: usize = 75;
pub(crate) const RCV_UNI_PAGE: usize = 76;
pub(crate) const RCV_UNI_LAST: usize = 77;
pub(crate) const RCV_HAVE_DATA: usize = 78;
pub(crate) const RCV_PREVIEW: usize = 79;
pub(crate) const RCV_STREAM_TERM: usize = 80;
pub(crate) const RCV_FORCE_SYNC: usize = 81;
pub(crate) const RCV_HAVE_SYNC: usize = 82;
pub(crate) const RCV_HAVE_DISC: usize = 83;
pub(crate) const RCV_BUSY: usize = 84;
pub(crate) const RCV_NAME_LEN: usize = 86;
pub(crate) const RCV_NAME: usize = 87;
pub(crate) const RCV_DMX: usize = 150;
pub(crate) const RCV_UNIS: usize = 662;
pub(crate) const RCV_ERR_LATCH: usize = 790;

// TplcSacnRcvInfo scalar offsets (64-byte fixed layout)
pub(crate) const RI_DATA_RX: usize = 0;
pub(crate) const RI_DATA_DROP: usize = 8;
pub(crate) const RI_SYNC_RX: usize = 16;
pub(crate) const RI_DISC_RX: usize = 24;
pub(crate) const RI_UNI_COUNT: usize = 32;
pub(crate) const RI_UNI_PAGE: usize = 34;
pub(crate) const RI_UNI_LAST: usize = 36;
pub(crate) const RI_SYNC_ADDR: usize = 38;
pub(crate) const RI_UNIVERSE: usize = 40;
pub(crate) const RI_DMX_LEN: usize = 42;
pub(crate) const RI_SYNC_ADDR_SEEN: usize = 44;
pub(crate) const RI_HAVE_DATA: usize = 46;
pub(crate) const RI_PREVIEW: usize = 47;
pub(crate) const RI_STREAM_TERM: usize = 48;
pub(crate) const RI_FORCE_SYNC: usize = 49;
pub(crate) const RI_START_CODE: usize = 50;
pub(crate) const RI_SEQ: usize = 51;
pub(crate) const RI_PRIORITY: usize = 52;
pub(crate) const RI_HAVE_SYNC: usize = 53;
pub(crate) const RI_SYNC_SEQ: usize = 54;
pub(crate) const RI_HAVE_DISC: usize = 55;
pub(crate) const RI_REMOTE_IP: usize = 56;

// mirrored TplcSacnService result codes; ERRID 2 stays "no service"
pub(crate) const SACNERR_PARAM: u32 = 1;
pub(crate) const SACNERR_NOSERVICE: u32 = 2;
pub(crate) const SACNERR_IO: u32 = 3;
pub(crate) const SACNERR_STATE: u32 = 4;
pub(crate) const SACNERR_RANGE: u32 = 5;

pub(crate) unsafe fn sacn_errid(rc: i32) -> u32 {
    match -rc {
        1 => SACNERR_PARAM,
        2 => SACNERR_IO,
        3 => SACNERR_STATE,
        4 => SACNERR_RANGE,
        _ => SACNERR_IO,
    }
}

/// resolve the tplc-sacn service table through ctx->svc->get_service
pub(crate) unsafe fn sacn_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-sacn".as_ptr() as *const u8, 9) as *const u8
}

/// load a TplcSacnService slot as a raw pointer-sized value
#[inline]
pub(crate) unsafe fn sacnfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FSacnSrcBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FSacnSrcPoll = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u8) -> i32;
pub(crate) type FSacnSrcClose = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FSacnRcvBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FSacnRcvPoll =
    unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u16, *mut u16, u16, *mut u8, u32) -> i32;
pub(crate) type FSacnRcvClose = unsafe extern "C" fn(*const u8, u32) -> i32;

/// copy a Str{63} blob field (len@off, data@off+1) into a fixed
/// NUL-terminated C buffer; returns the byte count copied
#[inline]
pub(crate) unsafe fn sacn_copy_name(
    this: *const u8,
    len_off: usize,
    data_off: usize,
    dst: *mut u8,
    cap: usize,
) -> usize {
    let n = (*this.add(len_off) as usize).min(cap.saturating_sub(1));
    core::ptr::copy_nonoverlapping(this.add(data_off), dst, n);
    *dst.add(n) = 0;
    n
}

/// `__tplc_rt_FB_SacnSource` — ENABLE rising edge opens the sACN
/// source (config latched at begin), falling edge closes; while the
/// handle is live the service transmits the DMX frame at INTERVAL ms
/// and SENT/AUX/SEQ/LAST_ERR mirror the service counters.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_SacnSource(this: *mut u8, ctx: *mut u8) -> i32 {
    let en = rb(this.add(SRC_ENABLE));
    let mut handle = ru32(this.add(SRC_HANDLE));

    if en && handle == 0 && !rb(this.add(SRC_ERR_LATCH)) {
        let svc = sacn_service(ctx);
        if svc.is_null() {
            wu32(this.add(SRC_ERRID), SACNERR_NOSERVICE);
            wb(this.add(SRC_ERR_LATCH), true);
        } else {
            /* TplcSacnSrcReq (104 bytes): universe u32le @0,
            priority @4, sync_addr @8, interval_ms @12, ttl @16,
            options u8 @20, flags u8 @21, name[64] @24, cid[16] @88 */
            let mut req = [0u8; 104];
            req[0..4].copy_from_slice(&ru32(this.add(SRC_UNIVERSE)).to_le_bytes());
            req[4..8].copy_from_slice(&ru32(this.add(SRC_PRIORITY)).to_le_bytes());
            req[8..12].copy_from_slice(&ru32(this.add(SRC_SYNC_ADDR)).to_le_bytes());
            req[12..16].copy_from_slice(&ru32(this.add(SRC_INTERVAL)).to_le_bytes());
            req[16..20].copy_from_slice(&ru32(this.add(SRC_TTL)).to_le_bytes());
            req[20] = *this.add(SRC_OPTIONS);
            req[21] = *this.add(SRC_FLAGS);
            sacn_copy_name(this, SRC_NAME_LEN, SRC_NAME, req.as_mut_ptr().add(24), 64);
            core::ptr::copy_nonoverlapping(this.add(SRC_CID), req.as_mut_ptr().add(88), 16);
            let mut h: u32 = 0;
            let f: FSacnSrcBegin = core::mem::transmute(sacnfn(svc, SACN_SVC_SRC_BEGIN));
            let rc = f(svc, req.as_ptr(), &mut h);
            if rc == 0 {
                handle = h;
                wu32(this.add(SRC_HANDLE), h);
                wu32(this.add(SRC_ERRID), 0);
            } else {
                wu32(this.add(SRC_ERRID), sacn_errid(rc));
                wb(this.add(SRC_ERR_LATCH), true);
            }
        }
    }
    if !en {
        wb(this.add(SRC_ERR_LATCH), false);
        if handle != 0 {
            let svc = sacn_service(ctx);
            if !svc.is_null() {
                let f: FSacnSrcClose = core::mem::transmute(sacnfn(svc, SACN_SVC_SRC_CLOSE));
                f(svc, handle);
            }
            wu32(this.add(SRC_HANDLE), 0);
            handle = 0;
        }
    }
    if handle != 0 {
        let svc = sacn_service(ctx);
        if svc.is_null() {
            wu32(this.add(SRC_HANDLE), 0);
            handle = 0;
            wb(this.add(SRC_ERR_LATCH), true);
            wu32(this.add(SRC_ERRID), SACNERR_NOSERVICE);
        } else {
            /* TplcSacnFrame (16 bytes): dmx ptr u64le @0,
            dmx_len u16le @8, start_code u8 @10 */
            let mut frame = [0u8; 16];
            let dmxp = this.add(SRC_DMX) as usize as u64;
            frame[0..8].copy_from_slice(&dmxp.to_le_bytes());
            let dlen = ru32(this.add(SRC_DLEN)).min(512) as u16;
            frame[8..10].copy_from_slice(&dlen.to_le_bytes());
            frame[10] = *this.add(SRC_START_CODE);
            /* TplcSacnSrcInfo (24 bytes): sent u64 @0, aux u64 @8,
            seq u32 @16, last_err i32 @20 */
            let mut info = [0u8; 24];
            let f: FSacnSrcPoll = core::mem::transmute(sacnfn(svc, SACN_SVC_SRC_POLL));
            let rc = f(svc, handle, frame.as_ptr(), info.as_mut_ptr());
            if rc == 0 {
                wu64(this.add(SRC_SENT), ru64(info.as_ptr()));
                wu64(this.add(SRC_AUX), ru64(info.as_ptr().add(8)));
                wu32(this.add(SRC_SEQ), ru32(info.as_ptr().add(16)));
                wi32(this.add(SRC_LAST_ERR), ri32(info.as_ptr().add(20)));
            } else {
                wu32(this.add(SRC_HANDLE), 0);
                handle = 0;
                wb(this.add(SRC_ERR_LATCH), true);
                wu32(this.add(SRC_ERRID), sacn_errid(rc));
            }
        }
    }
    wb(this.add(SRC_BUSY), handle != 0);
    0
}

/// `__tplc_rt_FB_SacnReceiver` — ENABLE rising edge binds the sACN
/// port and joins the universe multicast group(s), falling edge
/// closes; while the handle is live each scan drains pending
/// datagrams into the output fields.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_SacnReceiver(this: *mut u8, ctx: *mut u8) -> i32 {
    let en = rb(this.add(RCV_ENABLE));
    let mut handle = ru32(this.add(RCV_HANDLE));

    if en && handle == 0 && !rb(this.add(RCV_ERR_LATCH)) {
        let svc = sacn_service(ctx);
        if svc.is_null() {
            wu32(this.add(RCV_ERRID), SACNERR_NOSERVICE);
            wb(this.add(RCV_ERR_LATCH), true);
        } else {
            /* TplcSacnRcvReq (12 bytes): universe u32le @0,
            sync_universe @4, port @8 */
            let mut req = [0u8; 12];
            req[0..4].copy_from_slice(&ru32(this.add(RCV_UNIVERSE)).to_le_bytes());
            req[4..8].copy_from_slice(&ru32(this.add(RCV_SYNC_UNIVERSE)).to_le_bytes());
            req[8..12].copy_from_slice(&ru32(this.add(RCV_PORT)).to_le_bytes());
            let mut h: u32 = 0;
            let f: FSacnRcvBegin = core::mem::transmute(sacnfn(svc, SACN_SVC_RCV_BEGIN));
            let rc = f(svc, req.as_ptr(), &mut h);
            if rc == 0 {
                handle = h;
                wu32(this.add(RCV_HANDLE), h);
                wu32(this.add(RCV_ERRID), 0);
            } else {
                wu32(this.add(RCV_ERRID), sacn_errid(rc));
                wb(this.add(RCV_ERR_LATCH), true);
            }
        }
    }
    if !en {
        wb(this.add(RCV_ERR_LATCH), false);
        if handle != 0 {
            let svc = sacn_service(ctx);
            if !svc.is_null() {
                let f: FSacnRcvClose = core::mem::transmute(sacnfn(svc, SACN_SVC_RCV_CLOSE));
                f(svc, handle);
            }
            wu32(this.add(RCV_HANDLE), 0);
            handle = 0;
        }
    }
    if handle != 0 {
        let svc = sacn_service(ctx);
        if svc.is_null() {
            wu32(this.add(RCV_HANDLE), 0);
            handle = 0;
            wb(this.add(RCV_ERR_LATCH), true);
            wu32(this.add(RCV_ERRID), SACNERR_NOSERVICE);
        } else {
            /* TplcSacnRcvInfo (64 bytes) into a stack copy; DMX,
            universe window, and source name go straight into the
            instance block. */
            let mut info = [0u8; 64];
            let dmxp = this.add(RCV_DMX);
            let unisp = this.add(RCV_UNIS) as *mut u16;
            let namep = this.add(RCV_NAME);
            let f: FSacnRcvPoll = core::mem::transmute(sacnfn(svc, SACN_SVC_RCV_POLL));
            let rc = f(
                svc,
                handle,
                info.as_mut_ptr(),
                dmxp,
                512,
                unisp,
                64,
                namep,
                63,
            );
            if rc == 0 {
                let ip = info.as_ptr();
                wu64(this.add(RCV_DATA_RX), ru64(ip.add(RI_DATA_RX)));
                wu64(this.add(RCV_DATA_DROP), ru64(ip.add(RI_DATA_DROP)));
                wu64(this.add(RCV_SYNC_RX), ru64(ip.add(RI_SYNC_RX)));
                wu64(this.add(RCV_DISC_RX), ru64(ip.add(RI_DISC_RX)));
                wu32(this.add(RCV_REMOTE_IP), {
                    let b = ip.add(RI_REMOTE_IP);
                    ((*b as u32) << 24)
                        | ((*b.add(1) as u32) << 16)
                        | ((*b.add(2) as u32) << 8)
                        | (*b.add(3) as u32)
                });
                wu32(this.add(RCV_DLEN), ru16(ip.add(RI_DMX_LEN)) as u32);
                wu16(this.add(RCV_SYNC_ADDR), ru16(ip.add(RI_SYNC_ADDR)));
                wu16(this.add(RCV_UNI), ru16(ip.add(RI_UNIVERSE)));
                wu16(
                    this.add(RCV_SYNC_ADDR_SEEN),
                    ru16(ip.add(RI_SYNC_ADDR_SEEN)),
                );
                wu16(this.add(RCV_UNI_COUNT), ru16(ip.add(RI_UNI_COUNT)));
                *this.add(RCV_START_CODE) = *ip.add(RI_START_CODE);
                *this.add(RCV_SEQ) = *ip.add(RI_SEQ);
                *this.add(RCV_PRIORITY) = *ip.add(RI_PRIORITY);
                *this.add(RCV_SYNC_SEQ) = *ip.add(RI_SYNC_SEQ);
                *this.add(RCV_UNI_PAGE) = *ip.add(RI_UNI_PAGE);
                *this.add(RCV_UNI_LAST) = *ip.add(RI_UNI_LAST);
                wb(this.add(RCV_HAVE_DATA), *ip.add(RI_HAVE_DATA) != 0);
                wb(this.add(RCV_PREVIEW), *ip.add(RI_PREVIEW) != 0);
                wb(this.add(RCV_STREAM_TERM), *ip.add(RI_STREAM_TERM) != 0);
                wb(this.add(RCV_FORCE_SYNC), *ip.add(RI_FORCE_SYNC) != 0);
                wb(this.add(RCV_HAVE_SYNC), *ip.add(RI_HAVE_SYNC) != 0);
                wb(this.add(RCV_HAVE_DISC), *ip.add(RI_HAVE_DISC) != 0);
                /* refresh the Str len byte from the NUL-terminated
                name the service just wrote */
                let mut nl = 0usize;
                while nl < 62 && *namep.add(nl) != 0 {
                    nl += 1;
                }
                *this.add(RCV_NAME_LEN) = nl as u8;
            } else {
                wu32(this.add(RCV_HANDLE), 0);
                handle = 0;
                wb(this.add(RCV_ERR_LATCH), true);
                wu32(this.add(RCV_ERRID), sacn_errid(rc));
            }
        }
    }
    wb(this.add(RCV_BUSY), handle != 0);
    0
}
