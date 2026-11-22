//! tplc-rt builtin FB: modbus.
use crate::*;

/* ---------------- FB_Modbus (tplc-modbus service bridge) ---------------- */

// TplcModbusService fn ptrs start @16, eight bytes apart.
pub(crate) const MB_SVC_BEGIN: usize = 16;
pub(crate) const MB_SVC_POLL: usize = 24;
pub(crate) const MB_SVC_BEGIN_UDP: usize = 40;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Modbus"
pub(crate) const MB_HOST_LEN: usize = 0;
pub(crate) const MB_HOST: usize = 1;
pub(crate) const MB_PORT: usize = 16;
pub(crate) const MB_UNIT: usize = 20;
pub(crate) const MB_TIMEOUT: usize = 24;
pub(crate) const MB_FUNC: usize = 32;
pub(crate) const MB_DATA: usize = 36;
pub(crate) const MB_RLEN: usize = 292;
pub(crate) const MB_DLEN: usize = 296;
pub(crate) const MB_HANDLE: usize = 300;
pub(crate) const MB_RESULT: usize = 304;
pub(crate) const MB_EXC: usize = 308;
pub(crate) const MB_ERRID: usize = 312;
pub(crate) const MB_EXEC: usize = 316;
pub(crate) const MB_BUSY: usize = 317;
pub(crate) const MB_DONE: usize = 318;
pub(crate) const MB_LAST_EXEC: usize = 319;
pub(crate) const MB_UDP: usize = 320;

// mirrored TplcModbusService result codes, shifted so ERRID 2 stays
// "no service" like the other service bridges
pub(crate) const MBERR_PARAM: u32 = 1;
pub(crate) const MBERR_NOSERVICE: u32 = 2;
pub(crate) const MBERR_IO: u32 = 3;
pub(crate) const MBERR_CONNECT: u32 = 4;
pub(crate) const MBERR_TIMEOUT: u32 = 5;
pub(crate) const MBERR_CLOSED: u32 = 6;
pub(crate) const MBERR_PROTO: u32 = 7;
pub(crate) const MBERR_STATE: u32 = 8;
pub(crate) const MBERR_RANGE: u32 = 9;

pub(crate) unsafe fn mb_errid(rc: i32) -> u32 {
    match -rc {
        1 => MBERR_PARAM,
        2 => MBERR_IO,
        3 => MBERR_CONNECT,
        4 => MBERR_TIMEOUT,
        5 => MBERR_CLOSED,
        6 => MBERR_PROTO,
        7 => MBERR_STATE,
        8 => MBERR_RANGE,
        _ => MBERR_IO,
    }
}

pub(crate) type FMbBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
pub(crate) type FMbPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;

pub(crate) unsafe fn mb_fail(this: *mut u8, errid: u32) {
    wb(this.add(MB_BUSY), false);
    wb(this.add(MB_DONE), true);
    wu32(this.add(MB_ERRID), errid);
    wi32(this.add(MB_RESULT), 0);
    wu32(this.add(MB_HANDLE), 0);
}

/// resolve the tplc-modbus service table through ctx->svc->get_service
pub(crate) unsafe fn modbus_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-modbus".as_ptr() as *const u8, 11) as *const u8
}

/// load a TplcModbusService slot as a raw pointer-sized value
#[inline]
pub(crate) unsafe fn mbfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

/// `__tplc_rt_FB_Modbus` — EXEC rising edge begins one Modbus TCP
/// transaction (FUNC + DATA request bytes); while BUSY the transaction
/// is polled each scan until DONE or the service-side TIMEOUT.
/// RESULT is 0=ok / 1=exception (EXC = exception code); ERRID carries
/// transport/service errors. DATA is overwritten with the response.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Modbus(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(MB_EXEC));
    let last = rb(this.add(MB_LAST_EXEC));
    wb(this.add(MB_LAST_EXEC), exec);

    if !rb(this.add(MB_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        // rising edge: arm and begin the transaction
        wb(this.add(MB_DONE), false);
        wu32(this.add(MB_ERRID), 0);
        wi32(this.add(MB_RESULT), 0);
        wu32(this.add(MB_EXC), 0);
        wu32(this.add(MB_DLEN), 0);
        wb(this.add(MB_BUSY), true);

        let svc = modbus_service(ctx);
        if svc.is_null() {
            mb_fail(this, MBERR_NOSERVICE);
            return 0;
        }
        /* TplcModbusReq (32 bytes): host[16] @0 NUL-term, port u16le
        @16, timeout_ms u16le @18, unit u8 @20, func u8 @21,
        req_len u16le @22, req ptr u64le @24 */
        let mut req = [0u8; 32];
        let hn = (*this.add(MB_HOST_LEN) as usize).min(15);
        core::ptr::copy_nonoverlapping(this.add(MB_HOST), req.as_mut_ptr(), hn);
        let port = ru32(this.add(MB_PORT)) as u16;
        req[16..18].copy_from_slice(&port.to_le_bytes());
        let tns = ri(this.add(MB_TIMEOUT));
        let tms = if tns > 0 {
            (tns / 1_000_000).min(65535) as u16
        } else {
            0
        };
        req[18..20].copy_from_slice(&tms.to_le_bytes());
        req[20] = ru32(this.add(MB_UNIT)) as u8;
        req[21] = ru32(this.add(MB_FUNC)) as u8;
        let rlen = ru32(this.add(MB_RLEN)).min(256) as u16;
        req[22..24].copy_from_slice(&rlen.to_le_bytes());
        let datap = this.add(MB_DATA) as usize as u64;
        req[24..32].copy_from_slice(&datap.to_le_bytes());
        let mut h: u32 = 0;
        /* UDP flag selects mb_begin_udp (vtable slot 40); both share
        mb_poll/mb_abort and the handle namespace */
        let foff = if rb(this.add(MB_UDP)) {
            MB_SVC_BEGIN_UDP
        } else {
            MB_SVC_BEGIN
        };
        let f: FMbBegin = core::mem::transmute(mbfn(svc, foff));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            mb_fail(this, mb_errid(rc));
            return 0;
        }
        wu32(this.add(MB_HANDLE), h);
    }

    if !rb(this.add(MB_BUSY)) {
        return 0;
    }
    let svc = modbus_service(ctx);
    if svc.is_null() {
        mb_fail(this, MBERR_NOSERVICE);
        return 0;
    }
    /* TplcModbusResp (16 bytes): data ptr u64le @0, cap u16le @8,
    len u16le @10, status u8 @12, exc u8 @13 */
    let mut resp = [0u8; 16];
    let datap = this.add(MB_DATA) as usize as u64;
    resp[0..8].copy_from_slice(&datap.to_le_bytes());
    resp[8..10].copy_from_slice(&256u16.to_le_bytes());
    let f: FMbPoll = core::mem::transmute(mbfn(svc, MB_SVC_POLL));
    let rc = f(svc, ru32(this.add(MB_HANDLE)), resp.as_mut_ptr());
    match rc {
        1 => {} // TPLC_MB_BUSY — keep polling
        0 => {
            let rlen = u16::from_le_bytes([resp[10], resp[11]]);
            wu32(this.add(MB_DLEN), rlen as u32);
            wu32(this.add(MB_EXC), resp[13] as u32);
            wi32(this.add(MB_RESULT), resp[12] as i32);
            wb(this.add(MB_BUSY), false);
            wb(this.add(MB_DONE), true);
            wu32(this.add(MB_HANDLE), 0);
        }
        e => mb_fail(this, mb_errid(e)),
    }
    0
}
