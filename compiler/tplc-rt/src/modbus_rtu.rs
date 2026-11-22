//! tplc-rt builtin FB: modbus rtu.
use crate::*;

/* ---------------- FB_ModbusRtu (tplc-modbus RTU bridge) ---------------- */

// member offsets — keep in lockstep with builtin_fb_layout
// "FB_ModbusRtu"
pub(crate) const MRT_DEV_LEN: usize = 0;
pub(crate) const MRT_DEV: usize = 1;
pub(crate) const MRT_BAUD: usize = 64;
pub(crate) const MRT_PARITY: usize = 68;
pub(crate) const MRT_DBITS: usize = 72;
pub(crate) const MRT_SBITS: usize = 76;
pub(crate) const MRT_UNIT: usize = 80;
pub(crate) const MRT_TIMEOUT: usize = 88;
pub(crate) const MRT_FUNC: usize = 96;
pub(crate) const MRT_DATA: usize = 100;
pub(crate) const MRT_RLEN: usize = 356;
pub(crate) const MRT_DLEN: usize = 360;
pub(crate) const MRT_HANDLE: usize = 364;
pub(crate) const MRT_RESULT: usize = 368;
pub(crate) const MRT_EXC: usize = 372;
pub(crate) const MRT_ERRID: usize = 376;
pub(crate) const MRT_EXEC: usize = 380;
pub(crate) const MRT_BUSY: usize = 381;
pub(crate) const MRT_DONE: usize = 382;
pub(crate) const MRT_LAST_EXEC: usize = 383;

pub(crate) const MB_SVC_BEGIN_RTU: usize = 48;

pub(crate) unsafe fn mrt_fail(this: *mut u8, errid: u32) {
    wb(this.add(MRT_BUSY), false);
    wb(this.add(MRT_DONE), true);
    wu32(this.add(MRT_ERRID), errid);
    wi32(this.add(MRT_RESULT), 0);
    wu32(this.add(MRT_HANDLE), 0);
}

/// `__tplc_rt_FB_ModbusRtu` — EXEC rising edge begins one Modbus RTU
/// transaction over the serial device in DEV (BAUD/PARITY/DBITS/
/// SBITS termios settings, UNIT station address); while BUSY the
/// transaction is polled each scan until DONE or the service-side
/// TIMEOUT. Same RESULT/EXC/ERRID contract as FB_Modbus.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_ModbusRtu(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(MRT_EXEC));
    let last = rb(this.add(MRT_LAST_EXEC));
    wb(this.add(MRT_LAST_EXEC), exec);

    if !rb(this.add(MRT_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        wb(this.add(MRT_DONE), false);
        wu32(this.add(MRT_ERRID), 0);
        wi32(this.add(MRT_RESULT), 0);
        wu32(this.add(MRT_EXC), 0);
        wu32(this.add(MRT_DLEN), 0);
        wb(this.add(MRT_BUSY), true);

        let svc = modbus_service(ctx);
        if svc.is_null() {
            mrt_fail(this, MBERR_NOSERVICE);
            return 0;
        }
        /* TplcModbusRtuReq (88 bytes): dev[64] @0 NUL-term,
        baud u32le @64, timeout_ms u32le @68, unit u8 @72,
        func u8 @73, parity u8 @74, databits u8 @75, stopbits u8 @76,
        req_len u16le @78, req ptr u64le @80 */
        let mut req = [0u8; 88];
        let dn = (*this.add(MRT_DEV_LEN) as usize).min(63);
        core::ptr::copy_nonoverlapping(this.add(MRT_DEV), req.as_mut_ptr(), dn);
        req[64..68].copy_from_slice(&ru32(this.add(MRT_BAUD)).to_le_bytes());
        let tns = ri(this.add(MRT_TIMEOUT));
        let tms = if tns > 0 {
            (tns / 1_000_000).min(u32::MAX as i64) as u32
        } else {
            0
        };
        req[68..72].copy_from_slice(&tms.to_le_bytes());
        req[72] = ru32(this.add(MRT_UNIT)) as u8;
        req[73] = ru32(this.add(MRT_FUNC)) as u8;
        req[74] = ru32(this.add(MRT_PARITY)) as u8;
        req[75] = ru32(this.add(MRT_DBITS)) as u8;
        req[76] = ru32(this.add(MRT_SBITS)) as u8;
        let rlen = ru32(this.add(MRT_RLEN)).min(256) as u16;
        req[78..80].copy_from_slice(&rlen.to_le_bytes());
        let datap = this.add(MRT_DATA) as usize as u64;
        req[80..88].copy_from_slice(&datap.to_le_bytes());
        let mut h: u32 = 0;
        let f: FMbBegin = core::mem::transmute(mbfn(svc, MB_SVC_BEGIN_RTU));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            mrt_fail(this, mb_errid(rc));
            return 0;
        }
        wu32(this.add(MRT_HANDLE), h);
    }

    if !rb(this.add(MRT_BUSY)) {
        return 0;
    }
    let svc = modbus_service(ctx);
    if svc.is_null() {
        mrt_fail(this, MBERR_NOSERVICE);
        return 0;
    }
    /* TplcModbusResp (16 bytes): data ptr u64le @0, cap u16le @8,
    len u16le @10, status u8 @12, exc u8 @13 */
    let mut resp = [0u8; 16];
    let datap = this.add(MRT_DATA) as usize as u64;
    resp[0..8].copy_from_slice(&datap.to_le_bytes());
    resp[8..10].copy_from_slice(&256u16.to_le_bytes());
    let f: FMbPoll = core::mem::transmute(mbfn(svc, MB_SVC_POLL));
    let rc = f(svc, ru32(this.add(MRT_HANDLE)), resp.as_mut_ptr());
    match rc {
        1 => {} // TPLC_MB_BUSY — keep polling
        0 => {
            let rlen = u16::from_le_bytes([resp[10], resp[11]]);
            wu32(this.add(MRT_DLEN), rlen as u32);
            wu32(this.add(MRT_EXC), resp[13] as u32);
            wi32(this.add(MRT_RESULT), resp[12] as i32);
            wb(this.add(MRT_BUSY), false);
            wb(this.add(MRT_DONE), true);
            wu32(this.add(MRT_HANDLE), 0);
        }
        e => mrt_fail(this, mb_errid(e)),
    }
    0
}
