//! `tplc-rt` — clean-room IEC 61131-3 standard FB implementations
//! (doc 07 Appendix C semantics). Compiled to bitcode and linked into
//! deployable artifacts; also callable directly in tests.
//!
//! Every fn takes `(this, ctx)`: `this` = the FB instance's byte block
//! inside the program state blob (member offsets from
//! `tplc_mir::layout::builtin_fb_layout`), `ctx` = TplcCtx*
//! (now_ns @ +48). Returns 0 — faults use the checkpoint path, not
//! per-FB codes.
//!
//! Member offsets (must stay in lockstep with builtin_fb_layout):
//!   TON/TOF/TP : IN@0 u8, PT@8 i64, Q@16 u8, ET@24 i64,
//!                __in_last@32 u8, __start@40 i64, __run@48 u8  (size 56)
//!   R_/F_TRIG  : CLK@0 u8, Q@1 u8, __clk_last@2 u8            (size 8)
//!   CTU/CTD/CTUD: CU@0 CD@1 R@2 LD@3 u8, PV@4 i32, QU@8 QD@9 u8,
//!                CV@12 i32, __cu_last@16 __cd_last@17 u8      (size 24)
//!   SR/RS      : S@0 R1@1 S1@2 R@3 Q1@4 u8                   (size 8)
//!   FB_Socket  : OP@0 i32, HOST@4(str), PORT@20 u32,
//!                TIMEOUT@24 i64, DATA@32(str), DLEN@292 u32,
//!                HANDLE@296 u32, AUX@300 u32, RESULT@304 i32,
//!                ERRID@308 u32, EXEC@312 u8, BUSY@313 u8,
//!                DONE@314 u8, __last_exec@315 u8,
//!                __deadline@320 i64, __sent@328 u32,
//!                __recvd@332 u32, __phase@336 i32   (size 344)
//!   FB_Serial  : OP@0 i32, DEV@4(str), BAUD@68 u32,
//!                PARITY@72 u32, DBITS@76 u32, SBITS@80 u32,
//!                FLAGS@84 u32, AUX@88 u32, TIMEOUT@96 i64,
//!                DATA@104(str), DLEN@364 u32, HANDLE@368 u32,
//!                RESULT@372 i32, ERRID@376 u32, EXEC@380 u8,
//!                BUSY@381 u8, DONE@382 u8, __last_exec@383 u8,
//!                __deadline@384 i64, __sent@392 u32,
//!                __recvd@396 u32, __phase@400 i32   (size 408)
//!   FB_Ads     : OP@0 i32, HOST@4(str), PORT@20 u32,
//!                IDXGRP@24 u32, IDXOFF@28 u32, STATE@32 i32,
//!                DEVSTATE@36 i32, AUX@40 u32,
//!                TIMEOUT@48 i64, DATA@56(str), DLEN@316 u32,
//!                HANDLE@320 u32, RESULT@324 i32, ERRID@328 u32,
//!                EXEC@332 u8, BUSY@333 u8, DONE@334 u8,
//!                __last_exec@335 u8, __deadline@336 i64,
//!                __sent@344 u32, __recvd@348 u32,
//!                __phase@352 i32   (size 360)
//!   FB_RtMon   : OP@0 i32, CONTEXT@4 u32, GROUP@8 u32,
//!                EVENT@12 u32, MARKCTRL@16 u32, AUX@20 u32,
//!                RESULT@24 i32, ERRID@28 u32, EXEC@32 u8,
//!                BUSY@33 u8, DONE@34 u8, __last_exec@35 u8 (size 40)
//!   FB_Modbus  : HOST@0(str), PORT@16 u32, UNIT@20 u32,
//!                TIMEOUT@24 i64, FUNC@32 u32, DATA@36 (256B array),
//!                RLEN@292 u32, DLEN@296 u32, HANDLE@300 u32,
//!                RESULT@304 i32, EXC@308 u32, ERRID@312 u32,
//!                EXEC@316 u8, BUSY@317 u8, DONE@318 u8,
//!                __last_exec@319 u8, UDP@320 u8    (size 328)
//!   FB_ModbusRtu: DEV@0(str64), BAUD@64 u32, PARITY@68 u32,
//!                DBITS@72 u32, SBITS@76 u32, UNIT@80 u32,
//!                TIMEOUT@88 i64, FUNC@96 u32, DATA@100 (256B array),
//!                RLEN@356 u32, DLEN@360 u32, HANDLE@364 u32,
//!                RESULT@368 i32, EXC@372 u32, ERRID@376 u32,
//!                EXEC@380 u8, BUSY@381 u8, DONE@382 u8,
//!                __last_exec@383 u8            (size 384)

#![cfg_attr(feature = "nostd", no_std)]

#[cfg(feature = "nostd")]
#[panic_handler]
fn __tplc_rt_panic(_: &core::panic::PanicInfo) -> ! {
    unsafe extern "C" {
        fn abort() -> !;
    }
    unsafe { abort() }
}
const CTX_NOW_NS: usize = 48;

#[inline]
unsafe fn now_ns(ctx: *const u8) -> i64 {
    (ctx.add(CTX_NOW_NS) as *const i64).read()
}
#[inline]
unsafe fn rb(p: *const u8) -> bool {
    p.read() != 0
}
#[inline]
unsafe fn wb(p: *mut u8, v: bool) {
    p.write(v as u8);
}
#[inline]
unsafe fn ri(p: *const u8) -> i64 {
    (p as *const i64).read()
}
#[inline]
unsafe fn wi(p: *mut u8, v: i64) {
    (p as *mut i64).write(v);
}
#[inline]
unsafe fn ri32(p: *const u8) -> i32 {
    (p as *const i32).read()
}
#[inline]
unsafe fn wi32(p: *mut u8, v: i32) {
    (p as *mut i32).write(v);
}
#[inline]
unsafe fn ru32(p: *const u8) -> u32 {
    (p as *const u32).read()
}
#[inline]
unsafe fn wu32(p: *mut u8, v: u32) {
    (p as *mut u32).write(v);
}
#[inline]
unsafe fn ru16(p: *const u8) -> u16 {
    (p as *const u16).read()
}
#[inline]
unsafe fn wu16(p: *mut u8, v: u16) {
    (p as *mut u16).write(v);
}
#[inline]
unsafe fn ru64(p: *const u8) -> u64 {
    (p as *const u64).read()
}
#[inline]
unsafe fn wu64(p: *mut u8, v: u64) {
    (p as *mut u64).write(v);
}

/* ---------------- timers (shared block) ---------------- */

/// TON: IN↑ captures start; while IN, ET=min(now-start,PT), Q=(ET>=PT).
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_TON(this: *mut u8, ctx: *mut u8) -> i32 {
    let inn = rb(this);
    let pt = ri(this.add(8));
    let last = rb(this.add(32));
    let now = now_ns(ctx);
    if inn && !last {
        wi(this.add(40), now);
    }
    let (q, et) = if inn {
        let el = now - ri(this.add(40));
        (el >= pt, el.min(pt))
    } else {
        (false, 0)
    };
    wb(this.add(16), q);
    wi(this.add(24), et);
    wb(this.add(32), inn);
    0
}

/// TOF: IN → Q=1 immediately, ET=0; IN↓ starts off-timer, Q drops at PT.
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_TOF(this: *mut u8, ctx: *mut u8) -> i32 {
    let inn = rb(this);
    let pt = ri(this.add(8));
    let last = rb(this.add(32));
    let now = now_ns(ctx);
    if inn {
        wb(this.add(16), true);
        wi(this.add(24), 0);
        wb(this.add(48), false); // off-running cancelled
    } else {
        if last {
            wi(this.add(40), now); // falling edge: capture off-start
            wb(this.add(48), true);
        }
        if rb(this.add(48)) {
            let et = (now - ri(this.add(40))).min(pt);
            wi(this.add(24), et);
            if et >= pt {
                wb(this.add(16), false);
                wb(this.add(48), false);
            }
        }
    }
    wb(this.add(32), inn);
    0
}

/// TP: IN↑ starts a non-retriggerable PT pulse; Q high until ET>=PT.
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_TP(this: *mut u8, ctx: *mut u8) -> i32 {
    let inn = rb(this);
    let pt = ri(this.add(8));
    let last = rb(this.add(32));
    let now = now_ns(ctx);
    if inn && !last && !rb(this.add(48)) {
        wb(this.add(48), true);
        wi(this.add(40), now);
        wb(this.add(16), true);
    }
    if rb(this.add(48)) {
        let et = (now - ri(this.add(40))).min(pt);
        wi(this.add(24), et);
        if et >= pt {
            wb(this.add(16), false);
            wb(this.add(48), false);
        }
    }
    wb(this.add(32), inn);
    0
}

/* ---------------- edge detectors ---------------- */

#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_R_TRIG(this: *mut u8, _ctx: *mut u8) -> i32 {
    let clk = rb(this);
    let last = rb(this.add(2));
    wb(this.add(1), clk && !last);
    wb(this.add(2), clk);
    0
}

#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_F_TRIG(this: *mut u8, _ctx: *mut u8) -> i32 {
    let clk = rb(this);
    let last = rb(this.add(2));
    wb(this.add(1), !clk && last);
    wb(this.add(2), clk);
    0
}

/* ---------------- counters ---------------- */

unsafe fn ctud_step(this: *mut u8) {
    let cu = rb(this);
    let cd = rb(this.add(1));
    let r = rb(this.add(2));
    let ld = rb(this.add(3));
    let pv = ri32(this.add(4));
    let cu_last = rb(this.add(16));
    let cd_last = rb(this.add(17));
    let mut cv = ri32(this.add(12));
    if ld {
        cv = pv;
    }
    if r {
        cv = 0;
    }
    if !r && !ld {
        if cu && !cu_last {
            cv = cv.saturating_add(1);
        }
        if cd && !cd_last {
            cv = cv.saturating_sub(1);
        }
    }
    wi32(this.add(12), cv);
    wb(this.add(8), cv >= pv); // QU
    wb(this.add(9), cv <= 0); // QD
    wb(this.add(16), cu);
    wb(this.add(17), cd);
}

#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_CTU(this: *mut u8, _ctx: *mut u8) -> i32 {
    ctud_step(this);
    0
}
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_CTD(this: *mut u8, _ctx: *mut u8) -> i32 {
    ctud_step(this);
    0
}
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_CTUD(this: *mut u8, _ctx: *mut u8) -> i32 {
    ctud_step(this);
    0
}

/* ---------------- bistables ---------------- */

/// SR: set-dominant (S1 wins over R).
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_SR(this: *mut u8, _ctx: *mut u8) -> i32 {
    let s1 = rb(this.add(2));
    let r = rb(this.add(3));
    let q1 = if s1 {
        true
    } else if r {
        false
    } else {
        rb(this.add(4))
    };
    wb(this.add(4), q1);
    0
}

/// RS: reset-dominant (R1 wins over S).
#[no_mangle]
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
pub unsafe extern "C" fn __tplc_rt_RS(this: *mut u8, _ctx: *mut u8) -> i32 {
    let s = rb(this);
    let r1 = rb(this.add(1));
    let q1 = if r1 {
        false
    } else if s {
        true
    } else {
        rb(this.add(4))
    };
    wb(this.add(4), q1);
    0
}

/* ---------------- FB_Socket (tplc-net service bridge) ---------------- */

// TplcCtx.svc @72; TplcServices.get_service @48, .host @56.
// TplcNetService fn ptrs start @16, eight bytes apart.
const CTX_SVC: usize = 72;
const SVC_GET_SERVICE: usize = 48;
const SVC_HOST: usize = 56;
const NET_TCP_CONNECT: usize = 16;
const NET_TCP_CONNECT_POLL: usize = 24;
const NET_TCP_LISTEN: usize = 32;
const NET_TCP_ACCEPT: usize = 40;
const NET_TCP_SEND: usize = 48;
const NET_TCP_RECV: usize = 56;
const NET_TCP_CLOSE: usize = 64;
const NET_UDP_BIND: usize = 72;
const NET_UDP_SENDTO: usize = 80;
const NET_UDP_RECVFROM: usize = 88;
const NET_UDP_CLOSE: usize = 96;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Socket"
const SO_OP: usize = 0;
const SO_HOST_LEN: usize = 4;
const SO_HOST: usize = 5;
const SO_PORT: usize = 20;
const SO_TIMEOUT: usize = 24;
const SO_DATA_LEN: usize = 32;
const SO_DATA: usize = 33;
const SO_DLEN: usize = 292;
const SO_HANDLE: usize = 296;
const SO_AUX: usize = 300;
const SO_RESULT: usize = 304;
const SO_ERRID: usize = 308;
const SO_EXEC: usize = 312;
const SO_BUSY: usize = 313;
const SO_DONE: usize = 314;
const SO_LAST_EXEC: usize = 315;
const SO_DEADLINE: usize = 320;
const SO_SENT: usize = 328;
const SO_PHASE: usize = 336;

const SOCK_OP_CONNECT: i32 = 1;
const SOCK_OP_LISTEN: i32 = 2;
const SOCK_OP_ACCEPT: i32 = 3;
const SOCK_OP_SEND: i32 = 4;
const SOCK_OP_RECV: i32 = 5;
const SOCK_OP_CLOSE: i32 = 6;
const SOCK_OP_UDP_BIND: i32 = 7;
const SOCK_OP_UDP_SENDTO: i32 = 8;
const SOCK_OP_UDP_RECV: i32 = 9;
const SOCK_OP_UDP_CLOSE: i32 = 10;

// mirrored TplcNetService result codes (positive form for ERRID);
// service errors arrive as negatives and are stored as -rc
const NETERR_PARAM: u32 = 1;
const NETERR_NOSERVICE: u32 = 2;
const NETERR_TIMEOUT: u32 = 5;

type FNetConnect = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
type FNetConnectPoll = unsafe extern "C" fn(*const u8, u32) -> i32;
type FNetListen = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
type FNetAccept = unsafe extern "C" fn(*const u8, u32, *mut u32) -> i32;
type FNetSend = unsafe extern "C" fn(*const u8, u32, *const u8, u32, *mut u32) -> i32;
type FNetRecv = unsafe extern "C" fn(*const u8, u32, *mut u8, u32, *mut u32) -> i32;
type FNetClose = unsafe extern "C" fn(*const u8, u32) -> i32;
type FNetBind = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
type FNetSendTo =
    unsafe extern "C" fn(*const u8, u32, *const u8, u16, *const u8, u32, *mut u32) -> i32;
type FNetRecvFrom =
    unsafe extern "C" fn(*const u8, u32, *mut u8, u32, *mut u8, *mut u16, *mut u32) -> i32;

unsafe fn sock_fail(this: *mut u8, errid: u32) {
    wb(this.add(SO_BUSY), false);
    wb(this.add(SO_DONE), true);
    wu32(this.add(SO_ERRID), errid);
    wi32(this.add(SO_RESULT), 0);
}

unsafe fn sock_done(this: *mut u8, result: i32) {
    wb(this.add(SO_BUSY), false);
    wb(this.add(SO_DONE), true);
    wu32(this.add(SO_ERRID), 0);
    wi32(this.add(SO_RESULT), result);
}

/// copy the len-prefixed HOST member into a NUL-terminated buffer
unsafe fn host_cstr(this: *const u8, out: *mut u8) {
    let n = (*this.add(SO_HOST_LEN) as usize).min(15);
    core::ptr::copy_nonoverlapping(this.add(SO_HOST), out, n);
    *out.add(n) = 0;
}

/// resolve the tplc-net service table through ctx->svc->get_service
unsafe fn net_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-net".as_ptr() as *const u8, 8) as *const u8
}

/// load a TplcNetService slot as a raw pointer-sized value
#[inline]
unsafe fn netfn(net: *const u8, off: usize) -> usize {
    (net.add(off) as *const usize).read()
}

/// `__tplc_rt_FB_Socket` — EXEC rising edge arms an op; while BUSY the
/// op is polled each scan until DONE or the TIMEOUT deadline. OP codes
/// mirror TPLC_SOCK_OP_*; ERRID carries the positive service code.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Socket(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(SO_EXEC));
    let last = rb(this.add(SO_LAST_EXEC));
    let now = now_ns(ctx);

    if !rb(this.add(SO_BUSY)) {
        if !(exec && !last) {
            wb(this.add(SO_LAST_EXEC), exec);
            return 0;
        }
        // rising edge: arm the op
        wb(this.add(SO_DONE), false);
        wu32(this.add(SO_ERRID), 0);
        wi32(this.add(SO_RESULT), 0);
        wu32(this.add(SO_SENT), 0);
        wb(this.add(SO_BUSY), true);
        let to = ri(this.add(SO_TIMEOUT));
        wi(
            this.add(SO_DEADLINE),
            if to > 0 {
                now.saturating_add(to)
            } else {
                i64::MAX
            },
        );
    }
    wb(this.add(SO_LAST_EXEC), exec);

    let net = net_service(ctx);
    if net.is_null() {
        sock_fail(this, NETERR_NOSERVICE);
        return 0;
    }
    if now > ri(this.add(SO_DEADLINE)) {
        sock_fail(this, NETERR_TIMEOUT);
        return 0;
    }

    let op = ri32(this.add(SO_OP));
    let mut hbuf = [0u8; 16];
    match op {
        SOCK_OP_CONNECT => {
            if ri32(this.add(SO_PHASE)) == 0 {
                host_cstr(this, hbuf.as_mut_ptr());
                let mut out: u32 = 0;
                let f: FNetConnect = core::mem::transmute(netfn(net, NET_TCP_CONNECT));
                let rc = f(net, hbuf.as_ptr(), ru32(this.add(SO_PORT)) as u16, &mut out);
                if rc < 0 {
                    sock_fail(this, (-rc) as u32);
                    return 0;
                }
                wu32(this.add(SO_HANDLE), out);
                wi32(this.add(SO_PHASE), 1);
            }
            let f: FNetConnectPoll = core::mem::transmute(netfn(net, NET_TCP_CONNECT_POLL));
            match f(net, ru32(this.add(SO_HANDLE))) {
                0 => sock_done(this, 0),
                -7 => {} // still pending
                e => sock_fail(this, (-e) as u32),
            }
        }
        SOCK_OP_LISTEN => {
            host_cstr(this, hbuf.as_mut_ptr());
            let mut out: u32 = 0;
            let f: FNetListen = core::mem::transmute(netfn(net, NET_TCP_LISTEN));
            let rc = f(net, hbuf.as_ptr(), ru32(this.add(SO_PORT)) as u16, &mut out);
            if rc < 0 {
                sock_fail(this, (-rc) as u32);
            } else {
                wu32(this.add(SO_HANDLE), out);
                sock_done(this, 0);
            }
        }
        SOCK_OP_ACCEPT => {
            let mut out: u32 = 0;
            let f: FNetAccept = core::mem::transmute(netfn(net, NET_TCP_ACCEPT));
            match f(net, ru32(this.add(SO_AUX)), &mut out) {
                0 => {
                    wu32(this.add(SO_HANDLE), out);
                    sock_done(this, 0);
                }
                -7 => {}
                e => sock_fail(this, (-e) as u32),
            }
        }
        SOCK_OP_SEND => {
            let dlen = ru32(this.add(SO_DLEN)) as usize;
            let slen = (*this.add(SO_DATA_LEN) as usize).min(256);
            let total = if dlen > 0 { dlen.min(256) } else { slen };
            let mut sent = ru32(this.add(SO_SENT)) as usize;
            let f: FNetSend = core::mem::transmute(netfn(net, NET_TCP_SEND));
            for _ in 0..8 {
                if sent >= total {
                    break;
                }
                let mut done: u32 = 0;
                match f(
                    net,
                    ru32(this.add(SO_HANDLE)),
                    this.add(SO_DATA).add(sent),
                    (total - sent) as u32,
                    &mut done,
                ) {
                    0 => {
                        sent += done as usize;
                        if done == 0 {
                            break;
                        }
                    }
                    -7 => break,
                    e => {
                        sock_fail(this, (-e) as u32);
                        return 0;
                    }
                }
            }
            wu32(this.add(SO_SENT), sent as u32);
            if sent >= total {
                sock_done(this, sent as i32);
            }
        }
        SOCK_OP_RECV => {
            // DLEN is a pure output here: recv always reads up to the
            // full DATA capacity so a re-armed op cannot inherit a
            // stale cap from the previous transfer.
            let mut done: u32 = 0;
            let f: FNetRecv = core::mem::transmute(netfn(net, NET_TCP_RECV));
            match f(
                net,
                ru32(this.add(SO_HANDLE)),
                this.add(SO_DATA),
                256,
                &mut done,
            ) {
                0 => {
                    *this.add(SO_DATA_LEN) = done.min(256) as u8;
                    wu32(this.add(SO_DLEN), done);
                    sock_done(this, done as i32);
                }
                -7 => {}
                e => sock_fail(this, (-e) as u32),
            }
        }
        SOCK_OP_CLOSE => {
            let f: FNetClose = core::mem::transmute(netfn(net, NET_TCP_CLOSE));
            f(net, ru32(this.add(SO_HANDLE)));
            sock_done(this, 0);
        }
        SOCK_OP_UDP_BIND => {
            host_cstr(this, hbuf.as_mut_ptr());
            let mut out: u32 = 0;
            let f: FNetBind = core::mem::transmute(netfn(net, NET_UDP_BIND));
            let rc = f(net, hbuf.as_ptr(), ru32(this.add(SO_PORT)) as u16, &mut out);
            if rc < 0 {
                sock_fail(this, (-rc) as u32);
            } else {
                wu32(this.add(SO_HANDLE), out);
                sock_done(this, 0);
            }
        }
        SOCK_OP_UDP_SENDTO => {
            host_cstr(this, hbuf.as_mut_ptr());
            let dlen = ru32(this.add(SO_DLEN)) as usize;
            let slen = (*this.add(SO_DATA_LEN) as usize).min(256);
            let total = if dlen > 0 { dlen.min(256) } else { slen };
            let mut done: u32 = 0;
            let f: FNetSendTo = core::mem::transmute(netfn(net, NET_UDP_SENDTO));
            let rc = f(
                net,
                ru32(this.add(SO_HANDLE)),
                hbuf.as_ptr(),
                ru32(this.add(SO_PORT)) as u16,
                this.add(SO_DATA),
                total as u32,
                &mut done,
            );
            match rc {
                0 => sock_done(this, done as i32),
                e => sock_fail(this, (-e) as u32),
            }
        }
        SOCK_OP_UDP_RECV => {
            let mut peer = [0u8; 16];
            let mut pport: u16 = 0;
            let mut done: u32 = 0;
            let f: FNetRecvFrom = core::mem::transmute(netfn(net, NET_UDP_RECVFROM));
            match f(
                net,
                ru32(this.add(SO_HANDLE)),
                this.add(SO_DATA),
                256,
                peer.as_mut_ptr(),
                &mut pport,
                &mut done,
            ) {
                0 => {
                    *this.add(SO_DATA_LEN) = done.min(256) as u8;
                    wu32(this.add(SO_DLEN), done);
                    // peer endpoint -> HOST string + AUX port
                    let mut n = 0usize;
                    while n < 15 && peer[n] != 0 {
                        n += 1;
                    }
                    *this.add(SO_HOST_LEN) = n as u8;
                    core::ptr::copy_nonoverlapping(peer.as_ptr(), this.add(SO_HOST), n);
                    wu32(this.add(SO_AUX), pport as u32);
                    sock_done(this, done as i32);
                }
                -7 => {}
                e => sock_fail(this, (-e) as u32),
            }
        }
        SOCK_OP_UDP_CLOSE => {
            let f: FNetClose = core::mem::transmute(netfn(net, NET_UDP_CLOSE));
            f(net, ru32(this.add(SO_HANDLE)));
            sock_done(this, 0);
        }
        _ => sock_fail(this, NETERR_PARAM),
    }
    0
}

/* ---------------- FB_Serial (tplc-serial service bridge) ---------------- */

// TplcSerialService fn ptrs start @16 (after struct_size/abi_version/
// host), eight bytes apart.
const SER_OPEN: usize = 16;
const SER_WRITE: usize = 24;
const SER_READ: usize = 32;
const SER_FLUSH: usize = 40;
const SER_RS485: usize = 48;
const SER_CLOSE: usize = 56;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Serial"
const SE_OP: usize = 0;
const SE_DEV_LEN: usize = 4;
const SE_DEV: usize = 5;
const SE_BAUD: usize = 68;
const SE_PARITY: usize = 72;
const SE_DBITS: usize = 76;
const SE_SBITS: usize = 80;
const SE_FLAGS: usize = 84;
const SE_AUX: usize = 88;
const SE_TIMEOUT: usize = 96;
const SE_DATA_LEN: usize = 104;
const SE_DATA: usize = 105;
const SE_DLEN: usize = 364;
const SE_HANDLE: usize = 368;
const SE_RESULT: usize = 372;
const SE_ERRID: usize = 376;
const SE_EXEC: usize = 380;
const SE_BUSY: usize = 381;
const SE_DONE: usize = 382;
const SE_LAST_EXEC: usize = 383;
const SE_DEADLINE: usize = 384;
const SE_SENT: usize = 392;

const SER_OP_OPEN: i32 = 1;
const SER_OP_WRITE: i32 = 2;
const SER_OP_READ: i32 = 3;
const SER_OP_CLOSE: i32 = 4;
const SER_OP_FLUSH: i32 = 5;
const SER_OP_RS485: i32 = 6;

// mirrored TplcSerialService result codes (positive form for ERRID)
const SERERR_PARAM: u32 = 1;
const SERERR_NOSERVICE: u32 = 2;
const SERERR_TIMEOUT: u32 = 5;

type FSerOpen = unsafe extern "C" fn(*const u8, *const u8, u32, u8, u8, u8, u32, *mut u32) -> i32;
type FSerWrite = unsafe extern "C" fn(*const u8, u32, *const u8, u32, *mut u32) -> i32;
type FSerRead = unsafe extern "C" fn(*const u8, u32, *mut u8, u32, *mut u32) -> i32;
type FSerFlush = unsafe extern "C" fn(*const u8, u32, u8) -> i32;
type FSerRs485 = unsafe extern "C" fn(*const u8, u32, u8) -> i32;
type FSerClose = unsafe extern "C" fn(*const u8, u32) -> i32;

unsafe fn ser_fail(this: *mut u8, errid: u32) {
    wb(this.add(SE_BUSY), false);
    wb(this.add(SE_DONE), true);
    wu32(this.add(SE_ERRID), errid);
    wi32(this.add(SE_RESULT), 0);
}

unsafe fn ser_done(this: *mut u8, result: i32) {
    wb(this.add(SE_BUSY), false);
    wb(this.add(SE_DONE), true);
    wu32(this.add(SE_ERRID), 0);
    wi32(this.add(SE_RESULT), result);
}

/// copy the len-prefixed DEV member into a NUL-terminated buffer
unsafe fn dev_cstr(this: *const u8, out: *mut u8) {
    let n = (*this.add(SE_DEV_LEN) as usize).min(63);
    core::ptr::copy_nonoverlapping(this.add(SE_DEV), out, n);
    *out.add(n) = 0;
}

/// resolve the tplc-serial service table through ctx->svc->get_service
unsafe fn ser_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-serial".as_ptr() as *const u8, 11) as *const u8
}

/// load a TplcSerialService slot as a raw pointer-sized value
#[inline]
unsafe fn serfn(ser: *const u8, off: usize) -> usize {
    (ser.add(off) as *const usize).read()
}

/// `__tplc_rt_FB_Serial` — EXEC rising edge arms an op; while BUSY the
/// op is polled each scan until DONE or the TIMEOUT deadline. OP codes
/// mirror the TPLC_SERIAL_* op constants; ERRID carries the positive
/// service code.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Serial(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(SE_EXEC));
    let last = rb(this.add(SE_LAST_EXEC));
    let now = now_ns(ctx);

    if !rb(this.add(SE_BUSY)) {
        if !(exec && !last) {
            wb(this.add(SE_LAST_EXEC), exec);
            return 0;
        }
        // rising edge: arm the op
        wb(this.add(SE_DONE), false);
        wu32(this.add(SE_ERRID), 0);
        wi32(this.add(SE_RESULT), 0);
        wu32(this.add(SE_SENT), 0);
        wb(this.add(SE_BUSY), true);
        let to = ri(this.add(SE_TIMEOUT));
        wi(
            this.add(SE_DEADLINE),
            if to > 0 {
                now.saturating_add(to)
            } else {
                i64::MAX
            },
        );
    }
    wb(this.add(SE_LAST_EXEC), exec);

    let ser = ser_service(ctx);
    if ser.is_null() {
        ser_fail(this, SERERR_NOSERVICE);
        return 0;
    }
    if now > ri(this.add(SE_DEADLINE)) {
        ser_fail(this, SERERR_TIMEOUT);
        return 0;
    }

    let op = ri32(this.add(SE_OP));
    let mut devbuf = [0u8; 64];
    match op {
        SER_OP_OPEN => {
            dev_cstr(this, devbuf.as_mut_ptr());
            let mut out: u32 = 0;
            let f: FSerOpen = core::mem::transmute(serfn(ser, SER_OPEN));
            let rc = f(
                ser,
                devbuf.as_ptr(),
                ru32(this.add(SE_BAUD)),
                ru32(this.add(SE_DBITS)) as u8,
                ru32(this.add(SE_PARITY)) as u8,
                ru32(this.add(SE_SBITS)) as u8,
                ru32(this.add(SE_FLAGS)),
                &mut out,
            );
            if rc < 0 {
                ser_fail(this, (-rc) as u32);
            } else {
                wu32(this.add(SE_HANDLE), out);
                ser_done(this, 0);
            }
        }
        SER_OP_WRITE => {
            let dlen = ru32(this.add(SE_DLEN)) as usize;
            let slen = (*this.add(SE_DATA_LEN) as usize).min(256);
            let total = if dlen > 0 { dlen.min(256) } else { slen };
            let mut sent = ru32(this.add(SE_SENT)) as usize;
            let f: FSerWrite = core::mem::transmute(serfn(ser, SER_WRITE));
            for _ in 0..8 {
                if sent >= total {
                    break;
                }
                let mut done: u32 = 0;
                match f(
                    ser,
                    ru32(this.add(SE_HANDLE)),
                    this.add(SE_DATA).add(sent),
                    (total - sent) as u32,
                    &mut done,
                ) {
                    0 => {
                        sent += done as usize;
                        if done == 0 {
                            break;
                        }
                    }
                    -6 => break, // E_AGAIN: nothing moved this scan
                    e => {
                        ser_fail(this, (-e) as u32);
                        return 0;
                    }
                }
            }
            wu32(this.add(SE_SENT), sent as u32);
            if sent >= total {
                ser_done(this, sent as i32);
            }
        }
        SER_OP_READ => {
            let mut done: u32 = 0;
            let f: FSerRead = core::mem::transmute(serfn(ser, SER_READ));
            match f(
                ser,
                ru32(this.add(SE_HANDLE)),
                this.add(SE_DATA),
                256,
                &mut done,
            ) {
                0 => {
                    *this.add(SE_DATA_LEN) = done.min(256) as u8;
                    wu32(this.add(SE_DLEN), done);
                    ser_done(this, done as i32);
                }
                -6 => {}
                e => ser_fail(this, (-e) as u32),
            }
        }
        SER_OP_CLOSE => {
            let f: FSerClose = core::mem::transmute(serfn(ser, SER_CLOSE));
            f(ser, ru32(this.add(SE_HANDLE)));
            ser_done(this, 0);
        }
        SER_OP_FLUSH => {
            let f: FSerFlush = core::mem::transmute(serfn(ser, SER_FLUSH));
            let rc = f(ser, ru32(this.add(SE_HANDLE)), ru32(this.add(SE_AUX)) as u8);
            if rc < 0 {
                ser_fail(this, (-rc) as u32);
            } else {
                ser_done(this, 0);
            }
        }
        SER_OP_RS485 => {
            let f: FSerRs485 = core::mem::transmute(serfn(ser, SER_RS485));
            let rc = f(ser, ru32(this.add(SE_HANDLE)), ru32(this.add(SE_AUX)) as u8);
            if rc < 0 {
                ser_fail(this, (-rc) as u32);
            } else {
                ser_done(this, 0);
            }
        }
        _ => ser_fail(this, SERERR_PARAM),
    }
    0
}

/* ---------------- FB_Ads (tplc-ads service bridge) ---------------- */

// TplcAdsService fn ptrs start @16, eight bytes apart.
const ADS_OPEN: usize = 16;
const ADS_READ: usize = 24;
const ADS_WRITE: usize = 32;
const ADS_READ_STATE: usize = 40;
const ADS_WRITE_CTRL: usize = 48;
const ADS_CLOSE: usize = 56;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Ads"
const AD_OP: usize = 0;
const AD_HOST_LEN: usize = 4;
const AD_HOST: usize = 5;
const AD_PORT: usize = 20;
const AD_IDXGRP: usize = 24;
const AD_IDXOFF: usize = 28;
const AD_STATE: usize = 32;
const AD_DEVSTATE: usize = 36;
const AD_TIMEOUT: usize = 48;
const AD_DATA_LEN: usize = 56;
const AD_DATA: usize = 57;
const AD_DLEN: usize = 316;
const AD_HANDLE: usize = 320;
const AD_RESULT: usize = 324;
const AD_ERRID: usize = 328;
const AD_EXEC: usize = 332;
const AD_BUSY: usize = 333;
const AD_DONE: usize = 334;
const AD_LAST_EXEC: usize = 335;
const AD_DEADLINE: usize = 336;
const AD_SENT: usize = 344;

const ADS_OP_OPEN: i32 = 1;
const ADS_OP_READ: i32 = 2;
const ADS_OP_WRITE: i32 = 3;
const ADS_OP_READ_STATE: i32 = 4;
const ADS_OP_WRITE_CTRL: i32 = 5;
const ADS_OP_CLOSE: i32 = 6;

// mirrored TplcAdsService result codes (positive form for ERRID)
const ADSERR_PARAM: u32 = 1;
const ADSERR_NOSERVICE: u32 = 2;
const ADSERR_TIMEOUT: u32 = 5;

type FAdsOpen = unsafe extern "C" fn(*const u8, *const u8, u16, *mut u32) -> i32;
type FAdsRead =
    unsafe extern "C" fn(*const u8, u32, u32, u32, u32, *mut u8, u32, *mut u32, *mut u32) -> i32;
type FAdsWrite = unsafe extern "C" fn(*const u8, u32, u32, u32, *const u8, u32, *mut u32) -> i32;
type FAdsReadState = unsafe extern "C" fn(*const u8, u32, *mut u32, *mut u32, *mut u32) -> i32;
type FAdsWriteCtrl = unsafe extern "C" fn(*const u8, u32, u32, u32, *mut u32) -> i32;
type FAdsClose = unsafe extern "C" fn(*const u8, u32) -> i32;

unsafe fn ads_fail(this: *mut u8, errid: u32) {
    wb(this.add(AD_BUSY), false);
    wb(this.add(AD_DONE), true);
    wu32(this.add(AD_ERRID), errid);
    wi32(this.add(AD_RESULT), 0);
}

unsafe fn ads_done(this: *mut u8, result: i32) {
    wb(this.add(AD_BUSY), false);
    wb(this.add(AD_DONE), true);
    wu32(this.add(AD_ERRID), 0);
    wi32(this.add(AD_RESULT), result);
}

/// copy the len-prefixed HOST member into a NUL-terminated buffer
unsafe fn ad_host_cstr(this: *const u8, out: *mut u8) {
    let n = (*this.add(AD_HOST_LEN) as usize).min(15);
    core::ptr::copy_nonoverlapping(this.add(AD_HOST), out, n);
    *out.add(n) = 0;
}

/// resolve the tplc-ads service table through ctx->svc->get_service
unsafe fn ads_service(ctx: *const u8) -> *const u8 {
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
unsafe fn adsfn(ads: *const u8, off: usize) -> usize {
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

/* ---------------- FB_RtMon (tplc-rtmon service bridge) ---------------- */

// TplcRtmonService fn ptrs start @16, eight bytes apart.
const RTM_SVC_MARK: usize = 16;
const RTM_SVC_COUNT: usize = 24;
const RTM_SVC_GET: usize = 32;
const RTM_SVC_CLEAR: usize = 40;

// member offsets — keep in lockstep with builtin_fb_layout "FB_RtMon"
const RTM_OP: usize = 0;
const RTM_CONTEXT: usize = 4;
const RTM_GROUP: usize = 8;
const RTM_EVENT: usize = 12;
const RTM_MARKCTRL: usize = 16;
const RTM_AUX: usize = 20;
const RTM_RESULT: usize = 24;
const RTM_ERRID: usize = 28;
const RTM_EXEC: usize = 32;
const RTM_BUSY: usize = 33;
const RTM_DONE: usize = 34;
const RTM_LAST_EXEC: usize = 35;

const RTM_MARK: i32 = 1;
const RTM_COUNT: i32 = 2;
const RTM_GET: i32 = 3;
const RTM_CLEAR: i32 = 4;

// mirrored TplcRtmonService result codes (positive form for ERRID);
// range errors (-3) arrive via the service rc and map the same way
const RTMERR_PARAM: u32 = 1;
const RTMERR_NOSERVICE: u32 = 2;

type FRtmMark = unsafe extern "C" fn(*const u8, u32, u32, u32, u32) -> i32;
type FRtmCount = unsafe extern "C" fn(*const u8, *mut u32) -> i32;
type FRtmGet = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FRtmClear = unsafe extern "C" fn(*const u8) -> i32;

unsafe fn rtm_fail(this: *mut u8, errid: u32) {
    wb(this.add(RTM_BUSY), false);
    wb(this.add(RTM_DONE), true);
    wu32(this.add(RTM_ERRID), errid);
    wi32(this.add(RTM_RESULT), 0);
}

unsafe fn rtm_done(this: *mut u8, result: i32) {
    wb(this.add(RTM_BUSY), false);
    wb(this.add(RTM_DONE), true);
    wu32(this.add(RTM_ERRID), 0);
    wi32(this.add(RTM_RESULT), result);
}

/// resolve the tplc-rtmon service table through ctx->svc->get_service
unsafe fn rtmon_service(ctx: *const u8) -> *const u8 {
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
unsafe fn rtmfn(rtmon: *const u8, off: usize) -> usize {
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

/* ---------------- FB_Modbus (tplc-modbus service bridge) ---------------- */

// TplcModbusService fn ptrs start @16, eight bytes apart.
const MB_SVC_BEGIN: usize = 16;
const MB_SVC_POLL: usize = 24;
const MB_SVC_BEGIN_UDP: usize = 40;

// member offsets — keep in lockstep with builtin_fb_layout "FB_Modbus"
const MB_HOST_LEN: usize = 0;
const MB_HOST: usize = 1;
const MB_PORT: usize = 16;
const MB_UNIT: usize = 20;
const MB_TIMEOUT: usize = 24;
const MB_FUNC: usize = 32;
const MB_DATA: usize = 36;
const MB_RLEN: usize = 292;
const MB_DLEN: usize = 296;
const MB_HANDLE: usize = 300;
const MB_RESULT: usize = 304;
const MB_EXC: usize = 308;
const MB_ERRID: usize = 312;
const MB_EXEC: usize = 316;
const MB_BUSY: usize = 317;
const MB_DONE: usize = 318;
const MB_LAST_EXEC: usize = 319;
const MB_UDP: usize = 320;

// mirrored TplcModbusService result codes, shifted so ERRID 2 stays
// "no service" like the other service bridges
const MBERR_PARAM: u32 = 1;
const MBERR_NOSERVICE: u32 = 2;
const MBERR_IO: u32 = 3;
const MBERR_CONNECT: u32 = 4;
const MBERR_TIMEOUT: u32 = 5;
const MBERR_CLOSED: u32 = 6;
const MBERR_PROTO: u32 = 7;
const MBERR_STATE: u32 = 8;
const MBERR_RANGE: u32 = 9;

unsafe fn mb_errid(rc: i32) -> u32 {
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

type FMbBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FMbPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;

unsafe fn mb_fail(this: *mut u8, errid: u32) {
    wb(this.add(MB_BUSY), false);
    wb(this.add(MB_DONE), true);
    wu32(this.add(MB_ERRID), errid);
    wi32(this.add(MB_RESULT), 0);
    wu32(this.add(MB_HANDLE), 0);
}

/// resolve the tplc-modbus service table through ctx->svc->get_service
unsafe fn modbus_service(ctx: *const u8) -> *const u8 {
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
unsafe fn mbfn(svc: *const u8, off: usize) -> usize {
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

/* ---------------- FB_ModbusRtu (tplc-modbus RTU bridge) ---------------- */

// member offsets — keep in lockstep with builtin_fb_layout
// "FB_ModbusRtu"
const MRT_DEV_LEN: usize = 0;
const MRT_DEV: usize = 1;
const MRT_BAUD: usize = 64;
const MRT_PARITY: usize = 68;
const MRT_DBITS: usize = 72;
const MRT_SBITS: usize = 76;
const MRT_UNIT: usize = 80;
const MRT_TIMEOUT: usize = 88;
const MRT_FUNC: usize = 96;
const MRT_DATA: usize = 100;
const MRT_RLEN: usize = 356;
const MRT_DLEN: usize = 360;
const MRT_HANDLE: usize = 364;
const MRT_RESULT: usize = 368;
const MRT_EXC: usize = 372;
const MRT_ERRID: usize = 376;
const MRT_EXEC: usize = 380;
const MRT_BUSY: usize = 381;
const MRT_DONE: usize = 382;
const MRT_LAST_EXEC: usize = 383;

const MB_SVC_BEGIN_RTU: usize = 48;

unsafe fn mrt_fail(this: *mut u8, errid: u32) {
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

/* ------------- FB_SacnSource / FB_SacnReceiver (tplc-sacn) ------------- */

// TplcSacnService fn ptrs start @16, eight bytes apart.
const SACN_SVC_SRC_BEGIN: usize = 16;
const SACN_SVC_SRC_POLL: usize = 24;
const SACN_SVC_SRC_CLOSE: usize = 32;
const SACN_SVC_RCV_BEGIN: usize = 40;
const SACN_SVC_RCV_POLL: usize = 48;
const SACN_SVC_RCV_CLOSE: usize = 56;

// member offsets — keep in lockstep with builtin_fb_layout
// "FB_SacnSource"
const SRC_ENABLE: usize = 0;
const SRC_UNIVERSE: usize = 4;
const SRC_PRIORITY: usize = 8;
const SRC_SYNC_ADDR: usize = 12;
const SRC_INTERVAL: usize = 16;
const SRC_TTL: usize = 20;
const SRC_OPTIONS: usize = 24;
const SRC_FLAGS: usize = 25;
const SRC_START_CODE: usize = 26;
const SRC_NAME_LEN: usize = 28;
const SRC_NAME: usize = 29;
const SRC_CID: usize = 92;
const SRC_DLEN: usize = 108;
const SRC_DMX: usize = 112;
const SRC_HANDLE: usize = 624;
const SRC_SEQ: usize = 628;
const SRC_ERRID: usize = 632;
const SRC_LAST_ERR: usize = 636;
const SRC_SENT: usize = 640;
const SRC_AUX: usize = 648;
const SRC_BUSY: usize = 656;
const SRC_ERR_LATCH: usize = 657;

// member offsets — keep in lockstep with builtin_fb_layout
// "FB_SacnReceiver"
const RCV_ENABLE: usize = 0;
const RCV_UNIVERSE: usize = 4;
const RCV_SYNC_UNIVERSE: usize = 8;
const RCV_PORT: usize = 12;
const RCV_DATA_RX: usize = 16;
const RCV_DATA_DROP: usize = 24;
const RCV_SYNC_RX: usize = 32;
const RCV_DISC_RX: usize = 40;
const RCV_REMOTE_IP: usize = 48;
const RCV_HANDLE: usize = 52;
const RCV_ERRID: usize = 56;
const RCV_DLEN: usize = 60;
const RCV_SYNC_ADDR: usize = 64;
const RCV_UNI: usize = 66;
const RCV_SYNC_ADDR_SEEN: usize = 68;
const RCV_UNI_COUNT: usize = 70;
const RCV_START_CODE: usize = 72;
const RCV_SEQ: usize = 73;
const RCV_PRIORITY: usize = 74;
const RCV_SYNC_SEQ: usize = 75;
const RCV_UNI_PAGE: usize = 76;
const RCV_UNI_LAST: usize = 77;
const RCV_HAVE_DATA: usize = 78;
const RCV_PREVIEW: usize = 79;
const RCV_STREAM_TERM: usize = 80;
const RCV_FORCE_SYNC: usize = 81;
const RCV_HAVE_SYNC: usize = 82;
const RCV_HAVE_DISC: usize = 83;
const RCV_BUSY: usize = 84;
const RCV_NAME_LEN: usize = 86;
const RCV_NAME: usize = 87;
const RCV_DMX: usize = 150;
const RCV_UNIS: usize = 662;
const RCV_ERR_LATCH: usize = 790;

// TplcSacnRcvInfo scalar offsets (64-byte fixed layout)
const RI_DATA_RX: usize = 0;
const RI_DATA_DROP: usize = 8;
const RI_SYNC_RX: usize = 16;
const RI_DISC_RX: usize = 24;
const RI_UNI_COUNT: usize = 32;
const RI_UNI_PAGE: usize = 34;
const RI_UNI_LAST: usize = 36;
const RI_SYNC_ADDR: usize = 38;
const RI_UNIVERSE: usize = 40;
const RI_DMX_LEN: usize = 42;
const RI_SYNC_ADDR_SEEN: usize = 44;
const RI_HAVE_DATA: usize = 46;
const RI_PREVIEW: usize = 47;
const RI_STREAM_TERM: usize = 48;
const RI_FORCE_SYNC: usize = 49;
const RI_START_CODE: usize = 50;
const RI_SEQ: usize = 51;
const RI_PRIORITY: usize = 52;
const RI_HAVE_SYNC: usize = 53;
const RI_SYNC_SEQ: usize = 54;
const RI_HAVE_DISC: usize = 55;
const RI_REMOTE_IP: usize = 56;

// mirrored TplcSacnService result codes; ERRID 2 stays "no service"
const SACNERR_PARAM: u32 = 1;
const SACNERR_NOSERVICE: u32 = 2;
const SACNERR_IO: u32 = 3;
const SACNERR_STATE: u32 = 4;
const SACNERR_RANGE: u32 = 5;

unsafe fn sacn_errid(rc: i32) -> u32 {
    match -rc {
        1 => SACNERR_PARAM,
        2 => SACNERR_IO,
        3 => SACNERR_STATE,
        4 => SACNERR_RANGE,
        _ => SACNERR_IO,
    }
}

/// resolve the tplc-sacn service table through ctx->svc->get_service
unsafe fn sacn_service(ctx: *const u8) -> *const u8 {
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
unsafe fn sacnfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FSacnSrcBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FSacnSrcPoll = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u8) -> i32;
type FSacnSrcClose = unsafe extern "C" fn(*const u8, u32) -> i32;
type FSacnRcvBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FSacnRcvPoll =
    unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u16, *mut u16, u16, *mut u8, u32) -> i32;
type FSacnRcvClose = unsafe extern "C" fn(*const u8, u32) -> i32;

/// copy a Str{63} blob field (len@off, data@off+1) into a fixed
/// NUL-terminated C buffer; returns the byte count copied
#[inline]
unsafe fn sacn_copy_name(
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

/* ---------------- FB_Bacnet (tplc-bacnet bridge) ---------------- */

// member offsets — keep in lockstep with builtin_fb_layout
// "FB_Bacnet"
const BAC_HOST_LEN: usize = 0;
const BAC_HOST: usize = 1;
const BAC_PORT: usize = 16;
const BAC_FUNC: usize = 20;
const BAC_OBJ_TYPE: usize = 24;
const BAC_OBJ_INST: usize = 28;
const BAC_PROP_ID: usize = 32;
const BAC_HAS_IDX: usize = 36;
const BAC_HAS_PRIO: usize = 37;
const BAC_HAS_RANGE: usize = 38;
const BAC_PROP_IDX: usize = 40;
const BAC_PRIORITY: usize = 44;
const BAC_RANGE_LO: usize = 48;
const BAC_RANGE_HI: usize = 52;
const BAC_VAL_TYPE: usize = 56;
const BAC_VLEN: usize = 60;
const BAC_TIMEOUT: usize = 64;
const BAC_VAL: usize = 72;
const BAC_HANDLE: usize = 328;
const BAC_STATUS: usize = 332;
const BAC_R_SERVICE: usize = 336;
const BAC_R_APDU: usize = 340;
const BAC_R_OBJ_TYPE: usize = 344;
const BAC_R_OBJ_INST: usize = 348;
const BAC_R_PROP_ID: usize = 352;
const BAC_R_VTYPE: usize = 356;
const BAC_R_VLEN: usize = 360;
const BAC_R_VAL: usize = 364;
const BAC_DEV_COUNT: usize = 620;
const BAC_DEV_ID: usize = 624;
const BAC_DEV_IP: usize = 656;
const BAC_DEV_VENDOR: usize = 688;
const BAC_DEV_MAX_APDU: usize = 692;
const BAC_DEV_SEG: usize = 696;
const BAC_ERRID: usize = 700;
const BAC_EXEC: usize = 704;
const BAC_BUSY: usize = 705;
const BAC_DONE: usize = 706;
const BAC_LAST_EXEC: usize = 707;

const BAC_SVC_BEGIN: usize = 16;
const BAC_SVC_POLL: usize = 24;

/// service lookup for "tplc-bacnet" (11 chars)
unsafe fn bacnet_service(ctx: *const u8) -> *const u8 {
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
unsafe fn bacfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FBacBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FBacPoll =
    unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32, *mut u32, *mut u8, u32) -> i32;
#[inline]
unsafe fn bac_fail(this: *mut u8, errid: u32) {
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

/* ---------------- FB_Pjlink (tplc-pjlink bridge) ---------------- */

// member offsets — keep in lockstep with builtin_fb_layout
// "FB_Pjlink"
const PJL_HOST_LEN: usize = 0;
const PJL_HOST: usize = 1;
const PJL_PORT: usize = 16;
const PJL_CLS: usize = 20;
const PJL_CMD_LEN: usize = 24;
const PJL_CMD: usize = 25;
const PJL_PARAM_LEN: usize = 30;
const PJL_PARAM: usize = 31;
const PJL_PASS_LEN: usize = 160;
const PJL_PASS: usize = 161;
const PJL_TIMEOUT: usize = 200;
const PJL_HANDLE: usize = 208;
const PJL_STATUS: usize = 212;
const PJL_ERRID: usize = 216;
const PJL_R_ERR: usize = 220;
const PJL_R_CLS: usize = 224;
const PJL_R_CMD: usize = 228;
const PJL_R_VLEN: usize = 232;
const PJL_R_VALUE_LEN: usize = 236;
const PJL_R_VALUE: usize = 237;
const PJL_AUTHED: usize = 364;
const PJL_EXEC: usize = 365;
const PJL_BUSY: usize = 366;
const PJL_DONE: usize = 367;
const PJL_LAST_EXEC: usize = 368;

const PJL_SVC_BEGIN: usize = 16;
const PJL_SVC_POLL: usize = 24;

/// service lookup for "tplc-pjlink" (11 chars)
unsafe fn pjlink_service(ctx: *const u8) -> *const u8 {
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
unsafe fn pjlfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FPjBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FPjPoll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
#[inline]
unsafe fn pjl_fail(this: *mut u8, errid: u32) {
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

// ---------------------------------------------------------------------------
// FB_S7Conn / FB_S7Io — tplc-s7 service bridges (Siemens S7comm,
// RFC1006 ISO-on-TCP). FB_S7Conn owns the persistent COTP session
// (open handshake polled via BUSY, CLOSE edge tears it down);
// FB_S7Io runs one Read Var / Write Var transaction per EXEC edge
// against a live CONN_HANDLE (one outstanding io per connection).
// ---------------------------------------------------------------------------

// FB_S7Conn member offsets (see builtin_fb_layout("FB_S7Conn"))
const S7C_HOST_LEN: usize = 0;
const S7C_HOST: usize = 1;
const S7C_PORT: usize = 16;
const S7C_RACK: usize = 20;
const S7C_SLOT: usize = 24;
const S7C_TIMEOUT: usize = 32;
const S7C_CONN_HANDLE: usize = 40;
const S7C_STATUS: usize = 44;
const S7C_ERRID: usize = 48;
const S7C_CLOSE: usize = 52;
const S7C_EXEC: usize = 53;
const S7C_BUSY: usize = 54;
const S7C_DONE: usize = 55;
const S7C_LAST_EXEC: usize = 56;
const S7C_LAST_CLOSE: usize = 57;

// FB_S7Io member offsets (see builtin_fb_layout("FB_S7Io"))
const S7I_CONN_HANDLE: usize = 0;
const S7I_MODE: usize = 4;
const S7I_AREA: usize = 8;
const S7I_DB: usize = 12;
const S7I_BYTE_OFF: usize = 16;
const S7I_BIT_ACCESS: usize = 20;
const S7I_BIT_NO: usize = 24;
const S7I_LEN: usize = 28;
const S7I_TIMEOUT: usize = 32;
const S7I_HANDLE: usize = 40;
const S7I_STATUS: usize = 44;
const S7I_ITEM_RC: usize = 48;
const S7I_DATA_LEN: usize = 52;
const S7I_ERRID: usize = 56;
const S7I_DATA: usize = 60;
const S7I_R_DATA: usize = 300;
const S7I_EXEC: usize = 540;
const S7I_BUSY: usize = 541;
const S7I_DONE: usize = 542;
const S7I_LAST_EXEC: usize = 543;

// TplcS7Service vtable slots
const S7_SVC_OPEN: usize = 16;
const S7_SVC_CONN_POLL: usize = 24;
const S7_SVC_READ: usize = 32;
const S7_SVC_WRITE: usize = 40;
const S7_SVC_POLL: usize = 48;
const S7_SVC_CLOSE: usize = 64;

/// service lookup for "tplc-s7" (7 chars)
unsafe fn s7_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-s7".as_ptr() as *const u8, 7) as *const u8
}

#[inline]
unsafe fn s7fn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FS7Open = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FS7ConnPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FS7IoBegin = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;
type FS7Poll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
type FS7Close = unsafe extern "C" fn(*const u8, u32) -> i32;

#[inline]
unsafe fn s7c_fail(this: *mut u8, errid: u32) {
    wb(this.add(S7C_BUSY), false);
    wb(this.add(S7C_DONE), true);
    wu32(this.add(S7C_ERRID), errid);
    wi32(this.add(S7C_STATUS), 2);
}

#[inline]
unsafe fn s7i_fail(this: *mut u8, errid: u32) {
    wb(this.add(S7I_BUSY), false);
    wb(this.add(S7I_DONE), true);
    wu32(this.add(S7I_HANDLE), 0);
    wu32(this.add(S7I_ERRID), errid);
    wi32(this.add(S7I_STATUS), 3);
}

/// ns -> ms for the service timeout fields (0 = service default)
#[inline]
unsafe fn s7_timeout_ms(tns: i64) -> u32 {
    if tns <= 0 {
        0
    } else {
        ((tns / 1_000_000).min(60000)) as u32
    }
}

/// close a live connection handle through the service (best effort)
#[inline]
unsafe fn s7_close_handle(svc: *const u8, h: u32) {
    if h == 0 || svc.is_null() {
        return;
    }
    let f: FS7Close = core::mem::transmute(s7fn(svc, S7_SVC_CLOSE));
    f(svc, h);
}

/// `__tplc_rt_FB_S7Conn` — EXEC rising edge opens a persistent S7
/// connection; BUSY stays set while the TCP/COTP/Setup-Comm
/// handshake is driven through s7_conn_poll. STATUS mirrors the
/// conn info (0 pending, 1 ready, 2 error); DONE pulses on the
/// terminal poll. CLOSE rising edge closes the session.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_S7Conn(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(S7C_EXEC));
    let last = rb(this.add(S7C_LAST_EXEC));
    wb(this.add(S7C_LAST_EXEC), exec);
    let cls = rb(this.add(S7C_CLOSE));
    let clast = rb(this.add(S7C_LAST_CLOSE));
    wb(this.add(S7C_LAST_CLOSE), cls);

    // CLOSE rising edge: tear down any live session
    if cls && !clast {
        let h = ru32(this.add(S7C_CONN_HANDLE));
        let svc = s7_service(ctx);
        s7_close_handle(svc, h);
        wu32(this.add(S7C_CONN_HANDLE), 0);
        wb(this.add(S7C_BUSY), false);
        wb(this.add(S7C_DONE), true);
        wi32(this.add(S7C_STATUS), 0);
        wu32(this.add(S7C_ERRID), 0);
    }

    if !rb(this.add(S7C_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        // EXEC rising edge: drop a stale handle, open a fresh conn
        let old = ru32(this.add(S7C_CONN_HANDLE));
        let svc = s7_service(ctx);
        if svc.is_null() {
            s7c_fail(this, 99);
            return 0;
        }
        s7_close_handle(svc, old);
        wu32(this.add(S7C_CONN_HANDLE), 0);

        /* TplcS7OpenReq (32 bytes): host[16] @0, port u16 @16,
        rack u16 @18, slot u16 @20, reserved u16 @22,
        timeout_ms u32 @24, reserved2 u32 @28 */
        let mut req = [0u8; 32];
        let hn = (*this.add(S7C_HOST_LEN) as usize).min(15);
        if hn > 0 {
            core::ptr::copy_nonoverlapping(this.add(S7C_HOST), req.as_mut_ptr(), hn);
        }
        req[16..18].copy_from_slice(&(ru32(this.add(S7C_PORT)) as u16).to_le_bytes());
        req[18..20].copy_from_slice(&(ru32(this.add(S7C_RACK)) as u16).to_le_bytes());
        req[20..22].copy_from_slice(&(ru32(this.add(S7C_SLOT)) as u16).to_le_bytes());
        req[24..28].copy_from_slice(&s7_timeout_ms(ri(this.add(S7C_TIMEOUT))).to_le_bytes());
        let mut h: u32 = 0;
        let f: FS7Open = core::mem::transmute(s7fn(svc, S7_SVC_OPEN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            s7c_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(S7C_CONN_HANDLE), h);
        wb(this.add(S7C_DONE), false);
        wu32(this.add(S7C_ERRID), 0);
        wi32(this.add(S7C_STATUS), 0);
        wb(this.add(S7C_BUSY), true);
    }

    if !rb(this.add(S7C_BUSY)) {
        return 0;
    }
    let svc = s7_service(ctx);
    if svc.is_null() {
        s7c_fail(this, 99);
        return 0;
    }
    /* s7_conn_poll(h, info64) — TplcS7ConnInfo: status i32 @0,
    err i32 @4 */
    let mut info = [0u8; 64];
    let f: FS7ConnPoll = core::mem::transmute(s7fn(svc, S7_SVC_CONN_POLL));
    let rc = f(svc, ru32(this.add(S7C_CONN_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        s7c_fail(this, (-rc) as u32);
        return 0;
    }
    wi32(this.add(S7C_STATUS), ri32(info.as_ptr()));
    wu32(this.add(S7C_ERRID), ri32(info.as_ptr().add(4)) as u32);
    if ri32(info.as_ptr()) != 0 {
        wb(this.add(S7C_BUSY), false);
        wb(this.add(S7C_DONE), true);
    }
    0
}

/// `__tplc_rt_FB_S7Io` — EXEC rising edge begins one S7 Read Var
/// (MODE=0) or Write Var (MODE=1) transaction on CONN_HANDLE;
/// BUSY stays set while s7_poll drives it. R_DATA receives the
/// read payload (DATA_LEN bytes); ITEM_RC/ERRID mirror the
/// response item return code and transport/S7 error word.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_S7Io(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(S7I_EXEC));
    let last = rb(this.add(S7I_LAST_EXEC));
    wb(this.add(S7I_LAST_EXEC), exec);

    if !rb(this.add(S7I_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = s7_service(ctx);
        if svc.is_null() {
            s7i_fail(this, 99);
            return 0;
        }
        let is_write = ru32(this.add(S7I_MODE)) != 0;
        let bit = rb(this.add(S7I_BIT_ACCESS));
        let bit_no = ru32(this.add(S7I_BIT_NO));

        /* TplcS7Addr (24 bytes): area u32 @0, db u32 @4,
        byte_off u32 @8, bit_off u32 @12, len u32 @16,
        reserved u32 @20 — little-endian scalars */
        let fill_addr = |p: *mut u8| {
            core::ptr::write_bytes(p, 0, 24);
            let w = |off: usize, v: u32| {
                core::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), p.add(off), 4);
            };
            w(0, ru32(this.add(S7I_AREA)));
            w(4, ru32(this.add(S7I_DB)));
            w(8, ru32(this.add(S7I_BYTE_OFF)));
            w(12, if bit { bit_no } else { 0xFFFF_FFFF });
            w(16, ru32(this.add(S7I_LEN)));
        };

        let mut h: u32 = 0;
        let rc = if is_write {
            /* TplcS7WriteReq (276 bytes): addr @0..24, dlen u32
            @24, timeout_ms u32 @28, reserved u32 @32,
            data[240] @36 */
            let mut req = [0u8; 276];
            fill_addr(req.as_mut_ptr());
            let dlen = ru32(this.add(S7I_LEN)).min(240);
            let mut w = |off: usize, v: u32| {
                core::ptr::copy_nonoverlapping(
                    v.to_le_bytes().as_ptr(),
                    req.as_mut_ptr().add(off),
                    4,
                );
            };
            w(24, dlen);
            w(28, s7_timeout_ms(ri(this.add(S7I_TIMEOUT))));
            if dlen > 0 {
                core::ptr::copy_nonoverlapping(
                    this.add(S7I_DATA),
                    req.as_mut_ptr().add(36),
                    dlen as usize,
                );
            }
            let f: FS7IoBegin = core::mem::transmute(s7fn(svc, S7_SVC_WRITE));
            f(svc, ru32(this.add(S7I_CONN_HANDLE)), req.as_ptr(), &mut h)
        } else {
            /* TplcS7ReadReq (32 bytes): addr @0..24,
            timeout_ms u32 @24, reserved u32 @28 */
            let mut req = [0u8; 32];
            fill_addr(req.as_mut_ptr());
            core::ptr::copy_nonoverlapping(
                s7_timeout_ms(ri(this.add(S7I_TIMEOUT)))
                    .to_le_bytes()
                    .as_ptr(),
                req.as_mut_ptr().add(24),
                4,
            );
            let f: FS7IoBegin = core::mem::transmute(s7fn(svc, S7_SVC_READ));
            f(svc, ru32(this.add(S7I_CONN_HANDLE)), req.as_ptr(), &mut h)
        };
        if rc != 0 {
            s7i_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(S7I_HANDLE), h);
        wb(this.add(S7I_DONE), false);
        wu32(this.add(S7I_ERRID), 0);
        wu32(this.add(S7I_ITEM_RC), 0);
        wu32(this.add(S7I_DATA_LEN), 0);
        wi32(this.add(S7I_STATUS), 0);
        wb(this.add(S7I_BUSY), true);
    }

    if !rb(this.add(S7I_BUSY)) {
        return 0;
    }
    let svc = s7_service(ctx);
    if svc.is_null() {
        s7i_fail(this, 99);
        return 0;
    }
    /* s7_poll(h, info64, val, val_cap) — TplcS7RespInfo:
    status i32 @0, err i32 @4, item_rc u32 @8, data_len u32 @12 */
    let mut info = [0u8; 64];
    let mut vbuf = [0u8; 240];
    let f: FS7Poll = core::mem::transmute(s7fn(svc, S7_SVC_POLL));
    let rc = f(
        svc,
        ru32(this.add(S7I_HANDLE)),
        info.as_mut_ptr(),
        vbuf.as_mut_ptr(),
        240,
    );
    if rc != 0 {
        s7i_fail(this, (-rc) as u32);
        return 0;
    }
    let st = ri32(info.as_ptr());
    wi32(this.add(S7I_STATUS), st);
    wu32(this.add(S7I_ERRID), ri32(info.as_ptr().add(4)) as u32);
    wu32(this.add(S7I_ITEM_RC), ru32(info.as_ptr().add(8)));
    let dlen = ru32(info.as_ptr().add(12));
    wu32(this.add(S7I_DATA_LEN), dlen);
    if ru32(this.add(S7I_MODE)) == 0 && dlen > 0 {
        let cp = (dlen as usize).min(240);
        core::ptr::copy_nonoverlapping(vbuf.as_ptr(), this.add(S7I_R_DATA), cp);
    }
    if st != 0 {
        wb(this.add(S7I_BUSY), false);
        wb(this.add(S7I_DONE), true);
        wu32(this.add(S7I_HANDLE), 0);
    }
    0
}

// ---------------------------------------------------------------------------
// FB_FtpConn / FB_FtpCmd / FB_FtpXfer — tplc-ftp service bridges (FTP
// client, RFC 959 over TCP with PASV data connections). FB_FtpConn owns
// the control session (open/login polled via BUSY, CLOSE edge tears it
// down); FB_FtpCmd runs one control command per EXEC edge;
// FB_FtpXfer runs one RETR/STOR/LIST transfer per EXEC edge (downloads
// stream into BUF, uploads feed from UP_DATA automatically).
// ---------------------------------------------------------------------------

// FB_FtpConn member offsets (see builtin_fb_layout("FB_FtpConn"))
const FTPC_HOST_LEN: usize = 0;
const FTPC_HOST: usize = 1;
const FTPC_PORT: usize = 16;
const FTPC_USER_LEN: usize = 20;
const FTPC_USER: usize = 21;
const FTPC_PASS_LEN: usize = 52;
const FTPC_PASS: usize = 53;
const FTPC_TIMEOUT: usize = 88;
const FTPC_CONN_HANDLE: usize = 96;
const FTPC_STATUS: usize = 100;
const FTPC_ERRID: usize = 104;
const FTPC_CLOSE: usize = 108;
const FTPC_EXEC: usize = 109;
const FTPC_BUSY: usize = 110;
const FTPC_DONE: usize = 111;
const FTPC_LAST_EXEC: usize = 112;
const FTPC_LAST_CLOSE: usize = 113;

// FB_FtpCmd member offsets
const FTC_CONN_HANDLE: usize = 0;
const FTC_CMD_LEN: usize = 4;
const FTC_CMD: usize = 5;
const FTC_TIMEOUT: usize = 136;
const FTC_HANDLE: usize = 144;
const FTC_STATUS: usize = 148;
const FTC_RESP_CODE: usize = 152;
const FTC_ERRID: usize = 156;
const FTC_TEXT_LEN: usize = 160;
const FTC_TEXT: usize = 161;
const FTC_EXEC: usize = 288;
const FTC_BUSY: usize = 289;
const FTC_DONE: usize = 290;
const FTC_LAST_EXEC: usize = 291;

// FB_FtpXfer member offsets
const FTX_CONN_HANDLE: usize = 0;
const FTX_OP: usize = 4;
const FTX_PATH_LEN: usize = 8;
const FTX_PATH: usize = 9;
const FTX_TIMEOUT: usize = 136;
const FTX_HANDLE: usize = 144;
const FTX_STATUS: usize = 148;
const FTX_RESP_CODE: usize = 152;
const FTX_CHUNK: usize = 156;
const FTX_TOTAL: usize = 160;
const FTX_ERRID: usize = 164;
const FTX_UP_LEN: usize = 168;
const FTX_UP_OFF: usize = 172;
const FTX_BUF: usize = 176;
const FTX_UP_DATA: usize = 688;
const FTX_EXEC: usize = 1200;
const FTX_BUSY: usize = 1201;
const FTX_DONE: usize = 1202;
const FTX_LAST_EXEC: usize = 1203;
const FTX_FEED_DONE: usize = 1204;

// TplcFtpService vtable offsets (struct_size@0, abi@4, host@8)
const FTP_SVC_OPEN: usize = 16;
const FTP_SVC_CONN_POLL: usize = 24;
const FTP_SVC_CMD: usize = 32;
const FTP_SVC_TXN_POLL: usize = 40;
const FTP_SVC_XFER_BEGIN: usize = 48;
const FTP_SVC_XFER_POLL: usize = 56;
const FTP_SVC_XFER_FEED: usize = 64;
const FTP_SVC_XFER_FINISH: usize = 72;
const FTP_SVC_CLOSE: usize = 88;

unsafe fn ftp_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-ftp".as_ptr() as *const u8, 8) as *const u8
}

#[inline]
unsafe fn ftpfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FFtpOpen = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FFtpConnPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FFtpCmd = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;
type FFtpTxnPoll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
type FFtpXferBegin = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;
type FFtpXferPoll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
type FFtpXferFeed = unsafe extern "C" fn(*const u8, u32, *const u8, u32) -> i32;
type FFtpXferFinish = unsafe extern "C" fn(*const u8, u32) -> i32;
type FFtpClose = unsafe extern "C" fn(*const u8, u32) -> i32;

/// copy an FB Str field (len byte + chars) into a NUL-terminated
/// fixed char buffer for the service request structs
#[inline]
unsafe fn ftp_str_to_buf(
    this: *const u8,
    len_off: usize,
    ch_off: usize,
    str_cap: usize,
    dst: *mut u8,
    dst_cap: usize,
) {
    let n = (*this.add(len_off) as usize).min(str_cap).min(dst_cap - 1);
    if n > 0 {
        core::ptr::copy_nonoverlapping(this.add(ch_off), dst, n);
    }
    *dst.add(n) = 0;
}

/// ns -> ms for the service timeout fields (0 = service default)
#[inline]
unsafe fn ftp_timeout_ms(tns: i64) -> u32 {
    if tns <= 0 {
        0
    } else {
        ((tns / 1_000_000).min(300000)) as u32
    }
}

#[inline]
unsafe fn ftpc_fail(this: *mut u8, errid: u32) {
    wb(this.add(FTPC_BUSY), false);
    wb(this.add(FTPC_DONE), true);
    wu32(this.add(FTPC_ERRID), errid);
    wi32(this.add(FTPC_STATUS), 2);
}

#[inline]
unsafe fn ftpcmd_fail(this: *mut u8, errid: u32) {
    wb(this.add(FTC_BUSY), false);
    wb(this.add(FTC_DONE), true);
    wu32(this.add(FTC_HANDLE), 0);
    wu32(this.add(FTC_ERRID), errid);
    wi32(this.add(FTC_STATUS), 4);
}

#[inline]
unsafe fn ftx_fail(this: *mut u8, errid: u32) {
    wb(this.add(FTX_BUSY), false);
    wb(this.add(FTX_DONE), true);
    wu32(this.add(FTX_HANDLE), 0);
    wu32(this.add(FTX_ERRID), errid);
    wi32(this.add(FTX_STATUS), 4);
}

/// close a live connection handle through the service (best effort)
#[inline]
unsafe fn ftp_close_handle(svc: *const u8, h: u32) {
    if h == 0 || svc.is_null() {
        return;
    }
    let f: FFtpClose = core::mem::transmute(ftpfn(svc, FTP_SVC_CLOSE));
    f(svc, h);
}

/// `__tplc_rt_FB_FtpConn` — EXEC rising edge opens an FTP control
/// connection; BUSY stays set while the TCP connect + USER/PASS
/// login is driven through ftp_conn_poll. STATUS mirrors the conn
/// info (0 pending, 1 ready, 2 error); DONE pulses on the terminal
/// poll. CLOSE rising edge closes the session.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_FtpConn(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(FTPC_EXEC));
    let last = rb(this.add(FTPC_LAST_EXEC));
    wb(this.add(FTPC_LAST_EXEC), exec);
    let cls = rb(this.add(FTPC_CLOSE));
    let clast = rb(this.add(FTPC_LAST_CLOSE));
    wb(this.add(FTPC_LAST_CLOSE), cls);

    if cls && !clast {
        let h = ru32(this.add(FTPC_CONN_HANDLE));
        let svc = ftp_service(ctx);
        ftp_close_handle(svc, h);
        wu32(this.add(FTPC_CONN_HANDLE), 0);
        wb(this.add(FTPC_BUSY), false);
        wb(this.add(FTPC_DONE), true);
        wi32(this.add(FTPC_STATUS), 0);
        wu32(this.add(FTPC_ERRID), 0);
    }

    if !rb(this.add(FTPC_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let old = ru32(this.add(FTPC_CONN_HANDLE));
        let svc = ftp_service(ctx);
        if svc.is_null() {
            ftpc_fail(this, 99);
            return 0;
        }
        ftp_close_handle(svc, old);
        wu32(this.add(FTPC_CONN_HANDLE), 0);

        /* TplcFtpOpenReq (280 bytes): host[16] @0, port u16 @16,
        reserved u16 @18, timeout_ms u32 @20, user[128] @24,
        pass[128] @152 */
        let mut req = [0u8; 280];
        ftp_str_to_buf(this, FTPC_HOST_LEN, FTPC_HOST, 15, req.as_mut_ptr(), 16);
        core::ptr::copy_nonoverlapping(
            (ru32(this.add(FTPC_PORT)) as u16).to_le_bytes().as_ptr(),
            req.as_mut_ptr().add(16),
            2,
        );
        core::ptr::copy_nonoverlapping(
            ftp_timeout_ms(ri(this.add(FTPC_TIMEOUT)))
                .to_le_bytes()
                .as_ptr(),
            req.as_mut_ptr().add(20),
            4,
        );
        ftp_str_to_buf(
            this,
            FTPC_USER_LEN,
            FTPC_USER,
            31,
            req.as_mut_ptr().add(24),
            128,
        );
        ftp_str_to_buf(
            this,
            FTPC_PASS_LEN,
            FTPC_PASS,
            31,
            req.as_mut_ptr().add(152),
            128,
        );
        let mut h: u32 = 0;
        let f: FFtpOpen = core::mem::transmute(ftpfn(svc, FTP_SVC_OPEN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            ftpc_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(FTPC_CONN_HANDLE), h);
        wb(this.add(FTPC_DONE), false);
        wu32(this.add(FTPC_ERRID), 0);
        wi32(this.add(FTPC_STATUS), 0);
        wb(this.add(FTPC_BUSY), true);
    }

    if !rb(this.add(FTPC_BUSY)) {
        return 0;
    }
    let svc = ftp_service(ctx);
    if svc.is_null() {
        ftpc_fail(this, 99);
        return 0;
    }
    /* ftp_conn_poll(h, info64) — TplcFtpConnInfo: status i32 @0,
    err i32 @4, resp_code u32 @8 */
    let mut info = [0u8; 64];
    let f: FFtpConnPoll = core::mem::transmute(ftpfn(svc, FTP_SVC_CONN_POLL));
    let rc = f(svc, ru32(this.add(FTPC_CONN_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        ftpc_fail(this, (-rc) as u32);
        return 0;
    }
    wi32(this.add(FTPC_STATUS), ri32(info.as_ptr()));
    wu32(this.add(FTPC_ERRID), ri32(info.as_ptr().add(4)) as u32);
    if ri32(info.as_ptr()) != 0 {
        wb(this.add(FTPC_BUSY), false);
        wb(this.add(FTPC_DONE), true);
    }
    0
}

/// `__tplc_rt_FB_FtpCmd` — EXEC rising edge begins one FTP control
/// command transaction on CONN_HANDLE; BUSY stays set while
/// ftp_txn_poll drives it. RESP_CODE is the final 3-digit reply
/// code; TEXT receives the reply text. STATUS mirrors the txn
/// info (0 pending, 1 ok/2xx, 2 fail/4xx-5xx, 3 timeout,
/// 4 error, 5 badresp).
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_FtpCmd(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(FTC_EXEC));
    let last = rb(this.add(FTC_LAST_EXEC));
    wb(this.add(FTC_LAST_EXEC), exec);

    if !rb(this.add(FTC_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = ftp_service(ctx);
        if svc.is_null() {
            ftpcmd_fail(this, 99);
            return 0;
        }
        /* TplcFtpCmdReq (136 bytes): cmd[128] @0,
        timeout_ms u32 @128, reserved u32 @132 */
        let mut req = [0u8; 136];
        ftp_str_to_buf(this, FTC_CMD_LEN, FTC_CMD, 127, req.as_mut_ptr(), 128);
        core::ptr::copy_nonoverlapping(
            ftp_timeout_ms(ri(this.add(FTC_TIMEOUT)))
                .to_le_bytes()
                .as_ptr(),
            req.as_mut_ptr().add(128),
            4,
        );
        let mut h: u32 = 0;
        let f: FFtpCmd = core::mem::transmute(ftpfn(svc, FTP_SVC_CMD));
        let rc = f(svc, ru32(this.add(FTC_CONN_HANDLE)), req.as_ptr(), &mut h);
        if rc != 0 {
            ftpcmd_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(FTC_HANDLE), h);
        wb(this.add(FTC_DONE), false);
        wu32(this.add(FTC_ERRID), 0);
        wu32(this.add(FTC_RESP_CODE), 0);
        wi32(this.add(FTC_STATUS), 0);
        wb(this.add(FTC_BUSY), true);
    }

    if !rb(this.add(FTC_BUSY)) {
        return 0;
    }
    let svc = ftp_service(ctx);
    if svc.is_null() {
        ftpcmd_fail(this, 99);
        return 0;
    }
    /* ftp_txn_poll(h, info64, textbuf, text_cap) — TplcFtpTxnInfo:
    status i32 @0, err i32 @4, resp_code u32 @8, total u64 @16,
    chunk u32 @24 */
    let mut info = [0u8; 64];
    let mut tbuf = [0u8; 128];
    let f: FFtpTxnPoll = core::mem::transmute(ftpfn(svc, FTP_SVC_TXN_POLL));
    let rc = f(
        svc,
        ru32(this.add(FTC_HANDLE)),
        info.as_mut_ptr(),
        tbuf.as_mut_ptr(),
        128,
    );
    if rc != 0 {
        ftpcmd_fail(this, (-rc) as u32);
        return 0;
    }
    let st = ri32(info.as_ptr());
    wi32(this.add(FTC_STATUS), st);
    wu32(this.add(FTC_ERRID), ri32(info.as_ptr().add(4)) as u32);
    wu32(this.add(FTC_RESP_CODE), ru32(info.as_ptr().add(8)));
    /* copy the NUL-terminated reply text into TEXT (len + chars) */
    let mut tl = 0usize;
    while tl < 127 && tbuf[tl] != 0 {
        tl += 1;
    }
    *this.add(FTC_TEXT_LEN) = tl as u8;
    if tl > 0 {
        core::ptr::copy_nonoverlapping(tbuf.as_ptr(), this.add(FTC_TEXT), tl);
    }
    if st != 0 {
        wb(this.add(FTC_BUSY), false);
        wb(this.add(FTC_DONE), true);
        wu32(this.add(FTC_HANDLE), 0);
    }
    0
}

/// `__tplc_rt_FB_FtpXfer` — EXEC rising edge begins one FTP data
/// transfer (OP 0=RETR, 1=STOR, 2=LIST) on CONN_HANDLE; BUSY stays
/// set while ftp_xfer_poll drives it. For RETR/LIST each scan copies
/// up to 512 received bytes into BUF (CHUNK bytes this scan, TOTAL
/// cumulative); STATUS 6=more while data flows, terminal 1=ok /
/// 2=fail / 3=timeout / 4=error / 5=badresp. For STOR the rt feeds
/// UP_DATA[0..UP_LEN) automatically and signals finish.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_FtpXfer(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(FTX_EXEC));
    let last = rb(this.add(FTX_LAST_EXEC));
    wb(this.add(FTX_LAST_EXEC), exec);

    if !rb(this.add(FTX_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = ftp_service(ctx);
        if svc.is_null() {
            ftx_fail(this, 99);
            return 0;
        }
        /* TplcFtpXferReq (136 bytes): op u32 @0, timeout_ms u32 @4,
        path[128] @8 */
        let mut req = [0u8; 136];
        core::ptr::copy_nonoverlapping(
            ru32(this.add(FTX_OP)).to_le_bytes().as_ptr(),
            req.as_mut_ptr(),
            4,
        );
        core::ptr::copy_nonoverlapping(
            ftp_timeout_ms(ri(this.add(FTX_TIMEOUT)))
                .to_le_bytes()
                .as_ptr(),
            req.as_mut_ptr().add(4),
            4,
        );
        ftp_str_to_buf(
            this,
            FTX_PATH_LEN,
            FTX_PATH,
            127,
            req.as_mut_ptr().add(8),
            128,
        );
        let mut h: u32 = 0;
        let f: FFtpXferBegin = core::mem::transmute(ftpfn(svc, FTP_SVC_XFER_BEGIN));
        let rc = f(svc, ru32(this.add(FTX_CONN_HANDLE)), req.as_ptr(), &mut h);
        if rc != 0 {
            ftx_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(FTX_HANDLE), h);
        wb(this.add(FTX_DONE), false);
        wb(this.add(FTX_FEED_DONE), false);
        wu32(this.add(FTX_UP_OFF), 0);
        wu32(this.add(FTX_ERRID), 0);
        wu32(this.add(FTX_RESP_CODE), 0);
        wu32(this.add(FTX_CHUNK), 0);
        wu32(this.add(FTX_TOTAL), 0);
        wi32(this.add(FTX_STATUS), 0);
        wb(this.add(FTX_BUSY), true);
    }

    if !rb(this.add(FTX_BUSY)) {
        return 0;
    }
    let svc = ftp_service(ctx);
    if svc.is_null() {
        ftx_fail(this, 99);
        return 0;
    }
    let is_stor = ru32(this.add(FTX_OP)) == 1;
    /* TplcFtpTxnInfo (64 bytes): status i32 @0, err i32 @4,
    resp_code u32 @8, total u64 @16, chunk u32 @24 */
    let mut info = [0u8; 64];
    if is_stor {
        /* drive the transfer, then feed remaining UP_DATA */
        let fp: FFtpXferPoll = core::mem::transmute(ftpfn(svc, FTP_SVC_XFER_POLL));
        let rc = fp(
            svc,
            ru32(this.add(FTX_HANDLE)),
            info.as_mut_ptr(),
            core::ptr::null_mut(),
            0,
        );
        if rc != 0 {
            ftx_fail(this, (-rc) as u32);
            return 0;
        }
        let ulen = ru32(this.add(FTX_UP_LEN)).min(512);
        let mut uoff = ru32(this.add(FTX_UP_OFF)).min(ulen);
        if !rb(this.add(FTX_FEED_DONE)) {
            if uoff < ulen {
                let ff: FFtpXferFeed = core::mem::transmute(ftpfn(svc, FTP_SVC_XFER_FEED));
                let n = ff(
                    svc,
                    ru32(this.add(FTX_HANDLE)),
                    this.add(FTX_UP_DATA).add(uoff as usize),
                    ulen - uoff,
                );
                if n > 0 {
                    uoff += n as u32;
                }
            }
            if uoff >= ulen {
                let ff: FFtpXferFinish = core::mem::transmute(ftpfn(svc, FTP_SVC_XFER_FINISH));
                ff(svc, ru32(this.add(FTX_HANDLE)));
                wb(this.add(FTX_FEED_DONE), true);
            }
            wu32(this.add(FTX_UP_OFF), uoff);
        }
    } else {
        /* RETR/LIST: poll copies up to 512 bytes into BUF */
        let fp: FFtpXferPoll = core::mem::transmute(ftpfn(svc, FTP_SVC_XFER_POLL));
        let rc = fp(
            svc,
            ru32(this.add(FTX_HANDLE)),
            info.as_mut_ptr(),
            this.add(FTX_BUF),
            512,
        );
        if rc != 0 {
            ftx_fail(this, (-rc) as u32);
            return 0;
        }
    }
    let st = ri32(info.as_ptr());
    wi32(this.add(FTX_STATUS), st);
    wu32(this.add(FTX_ERRID), ri32(info.as_ptr().add(4)) as u32);
    wu32(this.add(FTX_RESP_CODE), ru32(info.as_ptr().add(8)));
    wu32(this.add(FTX_CHUNK), ru32(info.as_ptr().add(24)));
    let tot = u64::from_le_bytes(info[16..24].try_into().unwrap_or([0; 8]));
    wu32(this.add(FTX_TOTAL), tot.min(u64::from(u32::MAX)) as u32);
    /* terminal: ok/fail/timeout/error/badresp (1..=5); 0 pending
    and 6 more-data keep BUSY */
    if (1..=5).contains(&st) {
        wb(this.add(FTX_BUSY), false);
        wb(this.add(FTX_DONE), true);
        wu32(this.add(FTX_HANDLE), 0);
    }
    0
}

// ---------------------------------------------------------------------------
// FB_SmtpSend — tplc-smtp service bridge (SMTP client, RFC 5321 over TCP
// 25/587, plain text + optional AUTH LOGIN). EXEC rising edge begins
// one send transaction; the rt feeds BODY[0..BODY_LEN) automatically
// and signals finish. STATUS mirrors the service (0 pending, 1 ok,
// 2 fail, 3 timeout, 4 error, 5 badresp, 6 more).
// ---------------------------------------------------------------------------

// FB_SmtpSend member offsets (see builtin_fb_layout("FB_SmtpSend"))
const SMTP_HOST_LEN: usize = 0;
const SMTP_HOST: usize = 1;
const SMTP_PORT: usize = 16;
const SMTP_USER_LEN: usize = 20;
const SMTP_USER: usize = 21;
const SMTP_PASS_LEN: usize = 52;
const SMTP_PASS: usize = 53;
const SMTP_FROM_LEN: usize = 84;
const SMTP_FROM: usize = 85;
const SMTP_TO_LEN: usize = 148;
const SMTP_TO: usize = 149;
const SMTP_SUBJECT_LEN: usize = 212;
const SMTP_SUBJECT: usize = 213;
const SMTP_TIMEOUT: usize = 280;
const SMTP_HANDLE: usize = 288;
const SMTP_STATUS: usize = 292;
const SMTP_RESP_CODE: usize = 296;
const SMTP_ERRID: usize = 300;
const SMTP_BODY: usize = 304;
const SMTP_EXEC: usize = 560;
const SMTP_BUSY: usize = 561;
const SMTP_DONE: usize = 562;
const SMTP_LAST_EXEC: usize = 563;
const SMTP_FEED_DONE: usize = 564;

// TplcSmtpService vtable offsets (struct_size@0, abi@4, host@8)
const SMTP_SVC_BEGIN: usize = 16;
const SMTP_SVC_POLL: usize = 24;
const SMTP_SVC_FEED: usize = 32;
const SMTP_SVC_FEED_FINISH: usize = 40;

unsafe fn smtp_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-smtp".as_ptr() as *const u8, 9) as *const u8
}

#[inline]
unsafe fn smtpfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FSmtpBegin = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FSmtpPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FSmtpFeed = unsafe extern "C" fn(*const u8, u32, *const u8, u32) -> i32;
type FSmtpFeedFinish = unsafe extern "C" fn(*const u8, u32) -> i32;

#[inline]
unsafe fn smtp_fail(this: *mut u8, errid: u32) {
    wb(this.add(SMTP_BUSY), false);
    wb(this.add(SMTP_DONE), true);
    wu32(this.add(SMTP_HANDLE), 0);
    wu32(this.add(SMTP_ERRID), errid);
    wi32(this.add(SMTP_STATUS), 4);
}

/// `__tplc_rt_FB_SmtpSend` — EXEC rising edge begins one SMTP send
/// transaction; BUSY stays set while smtp_poll drives it. The rt
/// feeds BODY[0..BODY_LEN) once the DATA phase opens and calls
/// feed_finish when the body is fully queued. STATUS 6=more while
/// the body streams; terminal 1=ok / 2=fail / 3=timeout /
/// 4=error / 5=badresp.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_SmtpSend(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(SMTP_EXEC));
    let last = rb(this.add(SMTP_LAST_EXEC));
    wb(this.add(SMTP_LAST_EXEC), exec);

    if !rb(this.add(SMTP_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = smtp_service(ctx);
        if svc.is_null() {
            smtp_fail(this, 99);
            return 0;
        }
        /* TplcSmtpReq (344 bytes): host[16] @0, port u16 @16,
        reserved u16 @18, timeout_ms u32 @20, user[64] @24,
        pass[64] @88, from[64] @152, to[64] @216, subject[64] @280 */
        let mut req = [0u8; 344];
        ftp_str_to_buf(this, SMTP_HOST_LEN, SMTP_HOST, 15, req.as_mut_ptr(), 16);
        core::ptr::copy_nonoverlapping(
            (ru32(this.add(SMTP_PORT)) as u16).to_le_bytes().as_ptr(),
            req.as_mut_ptr().add(16),
            2,
        );
        core::ptr::copy_nonoverlapping(
            ftp_timeout_ms(ri(this.add(SMTP_TIMEOUT)))
                .to_le_bytes()
                .as_ptr(),
            req.as_mut_ptr().add(20),
            4,
        );
        ftp_str_to_buf(
            this,
            SMTP_USER_LEN,
            SMTP_USER,
            31,
            req.as_mut_ptr().add(24),
            64,
        );
        ftp_str_to_buf(
            this,
            SMTP_PASS_LEN,
            SMTP_PASS,
            31,
            req.as_mut_ptr().add(88),
            64,
        );
        ftp_str_to_buf(
            this,
            SMTP_FROM_LEN,
            SMTP_FROM,
            63,
            req.as_mut_ptr().add(152),
            64,
        );
        ftp_str_to_buf(
            this,
            SMTP_TO_LEN,
            SMTP_TO,
            63,
            req.as_mut_ptr().add(216),
            64,
        );
        ftp_str_to_buf(
            this,
            SMTP_SUBJECT_LEN,
            SMTP_SUBJECT,
            63,
            req.as_mut_ptr().add(280),
            64,
        );
        let mut h: u32 = 0;
        let f: FSmtpBegin = core::mem::transmute(smtpfn(svc, SMTP_SVC_BEGIN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            smtp_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(SMTP_HANDLE), h);
        wb(this.add(SMTP_DONE), false);
        wb(this.add(SMTP_FEED_DONE), false);
        wu32(this.add(SMTP_ERRID), 0);
        wu32(this.add(SMTP_RESP_CODE), 0);
        wi32(this.add(SMTP_STATUS), 0);
        wb(this.add(SMTP_BUSY), true);
    }

    if !rb(this.add(SMTP_BUSY)) {
        return 0;
    }
    let svc = smtp_service(ctx);
    if svc.is_null() {
        smtp_fail(this, 99);
        return 0;
    }
    /* TplcSmtpInfo (64 bytes): status i32 @0, err i32 @4,
    resp_code u32 @8, body_total u64 @16 */
    let mut info = [0u8; 64];
    let fp: FSmtpPoll = core::mem::transmute(smtpfn(svc, SMTP_SVC_POLL));
    let rc = fp(svc, ru32(this.add(SMTP_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        smtp_fail(this, (-rc) as u32);
        return 0;
    }
    /* feed BODY once the DATA phase is streaming (status 6);
    BODY is a STRING[255] (len byte @SMTP_BODY, chars @+1) */
    let st = ri32(info.as_ptr());
    if st == 6 && !rb(this.add(SMTP_FEED_DONE)) {
        let blen = (*this.add(SMTP_BODY) as usize).min(255);
        if blen > 0 {
            let ff: FSmtpFeed = core::mem::transmute(smtpfn(svc, SMTP_SVC_FEED));
            ff(
                svc,
                ru32(this.add(SMTP_HANDLE)),
                this.add(SMTP_BODY).add(1),
                blen as u32,
            );
        }
        let ff: FSmtpFeedFinish = core::mem::transmute(smtpfn(svc, SMTP_SVC_FEED_FINISH));
        ff(svc, ru32(this.add(SMTP_HANDLE)));
        wb(this.add(SMTP_FEED_DONE), true);
    }
    wi32(this.add(SMTP_STATUS), ri32(info.as_ptr()));
    wu32(this.add(SMTP_ERRID), ri32(info.as_ptr().add(4)) as u32);
    wu32(this.add(SMTP_RESP_CODE), ru32(info.as_ptr().add(8)));
    let st2 = ri32(info.as_ptr());
    if (1..=5).contains(&st2) {
        wb(this.add(SMTP_BUSY), false);
        wb(this.add(SMTP_DONE), true);
        wu32(this.add(SMTP_HANDLE), 0);
    }
    0
}

// ---------------------------------------------------------------------------
// FB_Iec870Conn / FB_Iec870Tx / FB_Iec870Rx — tplc-iec870 service bridges
// (IEC 60870-5-104 client, TCP 2404). FB_Iec870Conn owns the session
// (open handshake polled via BUSY, CLOSE edge sends STOPDT + closes);
// FB_Iec870Tx runs one ASDU send per EXEC edge; FB_Iec870Rx polls the
// received-ASDU queue every scan (consume-on-read, DONE pulses per ASDU).
// ---------------------------------------------------------------------------

// FB_Iec870Conn member offsets (see builtin_fb_layout("FB_Iec870Conn"))
const I70C_HOST_LEN: usize = 0;
const I70C_HOST: usize = 1;
const I70C_PORT: usize = 16;
const I70C_CA_SIZE: usize = 20;
const I70C_COT_SIZE: usize = 24;
const I70C_TIMEOUT: usize = 32;
const I70C_CONN_HANDLE: usize = 40;
const I70C_STATUS: usize = 44;
const I70C_SUBSTATE: usize = 48;
const I70C_ERRID: usize = 52;
const I70C_RX_DROPPED: usize = 56;
const I70C_CLOSE: usize = 60;
const I70C_EXEC: usize = 61;
const I70C_BUSY: usize = 62;
const I70C_DONE: usize = 63;
const I70C_LAST_EXEC: usize = 64;
const I70C_LAST_CLOSE: usize = 65;

// FB_Iec870Tx member offsets (see builtin_fb_layout("FB_Iec870Tx"))
const I70T_CONN_HANDLE: usize = 0;
const I70T_TYPE_ID: usize = 4;
const I70T_VSQ: usize = 8;
const I70T_COT: usize = 12;
const I70T_CA: usize = 16;
const I70T_LEN: usize = 20;
const I70T_HANDLE: usize = 24;
const I70T_STATUS: usize = 28;
const I70T_ERRID: usize = 32;
const I70T_PAYLOAD: usize = 36;
const I70T_EXEC: usize = 276;
const I70T_BUSY: usize = 277;
const I70T_DONE: usize = 278;
const I70T_LAST_EXEC: usize = 279;

// FB_Iec870Rx member offsets (see builtin_fb_layout("FB_Iec870Rx"))
const I70R_CONN_HANDLE: usize = 0;
const I70R_TYPE_ID: usize = 4;
const I70R_VSQ: usize = 8;
const I70R_COT: usize = 12;
const I70R_CA: usize = 16;
const I70R_LEN: usize = 20;
const I70R_I_FRAME_SEQ: usize = 24;
const I70R_ERRID: usize = 28;
const I70R_PAYLOAD: usize = 32;
const I70R_DONE: usize = 272;

// TplcIec870Service vtable slots
const I70_SVC_OPEN: usize = 16;
const I70_SVC_CONN_POLL: usize = 24;
const I70_SVC_SEND: usize = 32;
const I70_SVC_TX_POLL: usize = 40;
const I70_SVC_RX_POLL: usize = 56;
const I70_SVC_CLOSE: usize = 64;

/// service lookup for "tplc-iec870" (11 chars)
unsafe fn iec870_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-iec870".as_ptr() as *const u8, 11) as *const u8
}

#[inline]
unsafe fn iec870fn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FI70Open = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FI70ConnPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FI70Send = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;
type FI70TxPoll = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FI70RxPoll = unsafe extern "C" fn(*const u8, u32, *mut u8, *mut u8, u32) -> i32;
type FI70Close = unsafe extern "C" fn(*const u8, u32) -> i32;

#[inline]
unsafe fn i70c_fail(this: *mut u8, errid: u32) {
    wb(this.add(I70C_BUSY), false);
    wb(this.add(I70C_DONE), true);
    wu32(this.add(I70C_ERRID), errid);
    wi32(this.add(I70C_STATUS), 2);
}

#[inline]
unsafe fn i70t_fail(this: *mut u8, errid: u32) {
    wb(this.add(I70T_BUSY), false);
    wb(this.add(I70T_DONE), true);
    wu32(this.add(I70T_HANDLE), 0);
    wu32(this.add(I70T_ERRID), errid);
    wi32(this.add(I70T_STATUS), 2);
}

/// ns -> ms for the service timeout fields (0 = service default)
#[inline]
unsafe fn i70_timeout_ms(tns: i64) -> u32 {
    if tns <= 0 {
        0
    } else {
        ((tns / 1_000_000).min(60000)) as u32
    }
}

/// close a live connection handle through the service (best effort)
#[inline]
unsafe fn i70_close_handle(svc: *const u8, h: u32) {
    if h == 0 || svc.is_null() {
        return;
    }
    let f: FI70Close = core::mem::transmute(iec870fn(svc, I70_SVC_CLOSE));
    f(svc, h);
}

/// `__tplc_rt_FB_Iec870Conn` — EXEC rising edge opens an IEC
/// 60870-5-104 connection; BUSY stays set while the TCP/STARTDT
/// handshake is driven through iec870_conn_poll. STATUS mirrors
/// the conn info (0 pending, 1 ready, 2 error); SUBSTATE
/// 0=none 1=tcp 2=startdt 3=active; DONE pulses on the terminal
/// poll. CLOSE rising edge sends STOPDT act and closes.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Iec870Conn(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(I70C_EXEC));
    let last = rb(this.add(I70C_LAST_EXEC));
    wb(this.add(I70C_LAST_EXEC), exec);
    let cls = rb(this.add(I70C_CLOSE));
    let clast = rb(this.add(I70C_LAST_CLOSE));
    wb(this.add(I70C_LAST_CLOSE), cls);

    // CLOSE rising edge: tear down any live session
    if cls && !clast {
        let h = ru32(this.add(I70C_CONN_HANDLE));
        let svc = iec870_service(ctx);
        i70_close_handle(svc, h);
        wu32(this.add(I70C_CONN_HANDLE), 0);
        wb(this.add(I70C_BUSY), false);
        wb(this.add(I70C_DONE), true);
        wi32(this.add(I70C_STATUS), 0);
        wi32(this.add(I70C_SUBSTATE), 0);
        wu32(this.add(I70C_ERRID), 0);
    }

    if !rb(this.add(I70C_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        // EXEC rising edge: drop a stale handle, open a fresh conn
        let old = ru32(this.add(I70C_CONN_HANDLE));
        let svc = iec870_service(ctx);
        if svc.is_null() {
            i70c_fail(this, 99);
            return 0;
        }
        i70_close_handle(svc, old);
        wu32(this.add(I70C_CONN_HANDLE), 0);

        /* TplcIec870OpenReq (36 bytes): host[16] @0, port u16
        @16, ca_size u16 @18, cot_size u16 @20, reserved u16 @22,
        timeout_ms u32 @24, keepalive_ms u32 @28, reserved2 @32 */
        let mut req = [0u8; 36];
        let hn = (*this.add(I70C_HOST_LEN) as usize).min(15);
        if hn > 0 {
            core::ptr::copy_nonoverlapping(this.add(I70C_HOST), req.as_mut_ptr(), hn);
        }
        let mut w16 = |off: usize, v: u16| {
            core::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), req.as_mut_ptr().add(off), 2);
        };
        w16(16, ru32(this.add(I70C_PORT)) as u16);
        w16(18, ru32(this.add(I70C_CA_SIZE)) as u16);
        w16(20, ru32(this.add(I70C_COT_SIZE)) as u16);
        let mut w32 = |off: usize, v: u32| {
            core::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), req.as_mut_ptr().add(off), 4);
        };
        w32(24, i70_timeout_ms(ri(this.add(I70C_TIMEOUT))));
        w32(28, 0); /* keepalive: service default */
        let mut h: u32 = 0;
        let f: FI70Open = core::mem::transmute(iec870fn(svc, I70_SVC_OPEN));
        let rc = f(svc, req.as_ptr(), &mut h);
        if rc != 0 {
            i70c_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(I70C_CONN_HANDLE), h);
        wb(this.add(I70C_DONE), false);
        wu32(this.add(I70C_ERRID), 0);
        wi32(this.add(I70C_STATUS), 0);
        wi32(this.add(I70C_SUBSTATE), 0);
        wb(this.add(I70C_BUSY), true);
    }

    if !rb(this.add(I70C_BUSY)) {
        return 0;
    }
    let svc = iec870_service(ctx);
    if svc.is_null() {
        i70c_fail(this, 99);
        return 0;
    }
    /* iec870_conn_poll(h, info64) — TplcIec870ConnInfo: status i32
    @0, substate i32 @4, err i32 @8, rx_dropped u32 @12 */
    let mut info = [0u8; 64];
    let f: FI70ConnPoll = core::mem::transmute(iec870fn(svc, I70_SVC_CONN_POLL));
    let rc = f(svc, ru32(this.add(I70C_CONN_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        i70c_fail(this, (-rc) as u32);
        return 0;
    }
    wi32(this.add(I70C_STATUS), ri32(info.as_ptr()));
    wi32(this.add(I70C_SUBSTATE), ri32(info.as_ptr().add(4)));
    wu32(this.add(I70C_ERRID), ri32(info.as_ptr().add(8)) as u32);
    wu32(this.add(I70C_RX_DROPPED), ru32(info.as_ptr().add(12)));
    if ri32(info.as_ptr()) != 0 {
        wb(this.add(I70C_BUSY), false);
        wb(this.add(I70C_DONE), true);
    }
    0
}

/// `__tplc_rt_FB_Iec870Tx` — EXEC rising edge begins one ASDU
/// send on CONN_HANDLE; BUSY stays set while iec870_tx_poll
/// drives the I-frame flush. STATUS 0=pending 1=ok 2=error;
/// ERRID the transport errno.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Iec870Tx(this: *mut u8, ctx: *mut u8) -> i32 {
    let exec = rb(this.add(I70T_EXEC));
    let last = rb(this.add(I70T_LAST_EXEC));
    wb(this.add(I70T_LAST_EXEC), exec);

    if !rb(this.add(I70T_BUSY)) {
        if !(exec && !last) {
            return 0;
        }
        let svc = iec870_service(ctx);
        if svc.is_null() {
            i70t_fail(this, 99);
            return 0;
        }

        /* TplcIec870TxReq (264 bytes): type_id u32 @0, vsq @4,
        cot @8, ca @12, plen @16, reserved @20, payload[240] @24 */
        let mut req = [0u8; 264];
        let mut w = |off: usize, v: u32| {
            core::ptr::copy_nonoverlapping(v.to_le_bytes().as_ptr(), req.as_mut_ptr().add(off), 4);
        };
        w(0, ru32(this.add(I70T_TYPE_ID)));
        w(4, ru32(this.add(I70T_VSQ)));
        w(8, ru32(this.add(I70T_COT)));
        w(12, ru32(this.add(I70T_CA)));
        let plen = ru32(this.add(I70T_LEN)).min(240);
        w(16, plen);
        if plen > 0 {
            core::ptr::copy_nonoverlapping(
                this.add(I70T_PAYLOAD),
                req.as_mut_ptr().add(24),
                plen as usize,
            );
        }
        let mut h: u32 = 0;
        let f: FI70Send = core::mem::transmute(iec870fn(svc, I70_SVC_SEND));
        let rc = f(svc, ru32(this.add(I70T_CONN_HANDLE)), req.as_ptr(), &mut h);
        if rc != 0 {
            i70t_fail(this, (-rc) as u32);
            return 0;
        }
        wu32(this.add(I70T_HANDLE), h);
        wb(this.add(I70T_DONE), false);
        wu32(this.add(I70T_ERRID), 0);
        wi32(this.add(I70T_STATUS), 0);
        wb(this.add(I70T_BUSY), true);
    }

    if !rb(this.add(I70T_BUSY)) {
        return 0;
    }
    let svc = iec870_service(ctx);
    if svc.is_null() {
        i70t_fail(this, 99);
        return 0;
    }
    /* iec870_tx_poll(h, info64) — TplcIec870TxInfo: status i32
    @0, err i32 @4 */
    let mut info = [0u8; 64];
    let f: FI70TxPoll = core::mem::transmute(iec870fn(svc, I70_SVC_TX_POLL));
    let rc = f(svc, ru32(this.add(I70T_HANDLE)), info.as_mut_ptr());
    if rc != 0 {
        i70t_fail(this, (-rc) as u32);
        return 0;
    }
    wi32(this.add(I70T_STATUS), ri32(info.as_ptr()));
    wu32(this.add(I70T_ERRID), ri32(info.as_ptr().add(4)) as u32);
    if ri32(info.as_ptr()) != 0 {
        wb(this.add(I70T_BUSY), false);
        wb(this.add(I70T_DONE), true);
        wu32(this.add(I70T_HANDLE), 0);
    }
    0
}

/// `__tplc_rt_FB_Iec870Rx` — polls the received-ASDU queue every
/// scan. DONE pulses TRUE on the scan an ASDU is delivered
/// (consume-on-read); TYPE_ID/VSQ/COT/CA/LEN/I_FRAME_SEQ and
/// PAYLOAD describe it.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Iec870Rx(this: *mut u8, ctx: *mut u8) -> i32 {
    wb(this.add(I70R_DONE), false);
    let svc = iec870_service(ctx);
    if svc.is_null() {
        wu32(this.add(I70R_ERRID), 99);
        return 0;
    }
    /* iec870_rx_poll(ch, info64, payload, cap) — TplcIec870RxInfo:
    status i32 @0, err i32 @4, type_id @8, vsq @12, cot @16,
    ca @20, payload_len @24, i_frame_seq @28 */
    let mut info = [0u8; 64];
    let f: FI70RxPoll = core::mem::transmute(iec870fn(svc, I70_SVC_RX_POLL));
    let rc = f(
        svc,
        ru32(this.add(I70R_CONN_HANDLE)),
        info.as_mut_ptr(),
        this.add(I70R_PAYLOAD),
        240,
    );
    if rc != 0 {
        wu32(this.add(I70R_ERRID), (-rc) as u32);
        return 0;
    }
    wu32(this.add(I70R_ERRID), ri32(info.as_ptr().add(4)) as u32);
    if ri32(info.as_ptr()) == 1 {
        let rd = |off: usize| -> u32 {
            let mut b = [0u8; 4];
            core::ptr::copy_nonoverlapping(info.as_ptr().add(off), b.as_mut_ptr(), 4);
            u32::from_le_bytes(b)
        };
        wu32(this.add(I70R_TYPE_ID), rd(8));
        wu32(this.add(I70R_VSQ), rd(12));
        wu32(this.add(I70R_COT), rd(16));
        wu32(this.add(I70R_CA), rd(20));
        wu32(this.add(I70R_LEN), rd(24));
        wu32(this.add(I70R_I_FRAME_SEQ), rd(28));
        wb(this.add(I70R_DONE), true);
    }
    0
}

// FB_MotionAxis member offsets (see builtin_fb_layout("FB_MotionAxis"))
// all members are i32/u32 (4 bytes), declaration order.
const MA_OP: usize = 0;
const MA_AXIS: usize = 4;
const MA_UNITS_PER_REV: usize = 8;
const MA_MAX_VEL: usize = 12;
const MA_MAX_ACCEL: usize = 16;
const MA_MAX_DECEL: usize = 20;
const MA_POS_MIN: usize = 24;
const MA_POS_MAX: usize = 28;
const MA_HOME_OFFSET: usize = 32;
const MA_FOLLOW_WINDOW: usize = 36;
const MA_FLAGS: usize = 40;
const MA_INDEX: usize = 44;
const MA_VALUE: usize = 48;
const MA_STATUS: usize = 52;
const MA_ERRID: usize = 56;
const MA_MODE: usize = 60;
const MA_FAULT: usize = 64;
const MA_TARGET_REACHED: usize = 68;
const MA_BUSY: usize = 72;
const MA_POS_ACTUAL: usize = 76;
const MA_VEL_ACTUAL: usize = 80;

// tplc-motion service vtable offsets (TplcMotionService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
const MA_SVC_CREATE: usize = 16;
const MA_SVC_DESTROY: usize = 24;
const MA_SVC_POLL: usize = 32;
const MA_SVC_INFO: usize = 40;
const MA_SVC_OBJ_READ: usize = 48;
const MA_SVC_OBJ_WRITE: usize = 56;

/// service lookup for "tplc-motion" (11 chars)
unsafe fn motion_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-motion".as_ptr() as *const u8, 11) as *const u8
}

#[inline]
unsafe fn motionfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FMAxisCreate = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FMAxisDestroy = unsafe extern "C" fn(*const u8, u32) -> i32;
type FMAxisPoll = unsafe extern "C" fn(*const u8) -> i32;
type FMAxisInfo = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FMObjRead = unsafe extern "C" fn(*const u8, u32, u32, *mut i32) -> i32;
type FMObjWrite = unsafe extern "C" fn(*const u8, u32, u32, i32) -> i32;

/// `__tplc_rt_FB_MotionAxis` — synchronous op dispatch over the
/// tplc-motion simulated CiA402 axis. OP selects the operation:
/// 1=create (CFG fields -> AXIS handle), 2=destroy, 3=poll,
/// 4=read object (INDEX -> VALUE), 5=write object (INDEX+VALUE),
/// 6=info (axis snapshot into MODE/FAULT/TARGET_REACHED/BUSY/
/// POS_ACTUAL/VEL_ACTUAL). STATUS carries the
/// service return code (for OP=6 the DS402 state). All calls are
/// immediate; the compat layer gates create/destroy timing.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_MotionAxis(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(MA_OP));
    wi32(this.add(MA_STATUS), 0);
    wu32(this.add(MA_ERRID), 0);
    let svc = motion_service(ctx);
    if svc.is_null() {
        wi32(this.add(MA_STATUS), -2);
        wu32(this.add(MA_ERRID), 99);
        return 0;
    }
    match op {
        1 => {
            // TPLC_MotionAxisCfg (64 bytes): 8xi32 cfg @0..31,
            // flags u32 @32, reserved[7] u32 @36..63
            let mut cfg = [0u8; 64];
            let mut put = |off: usize, v: i32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), cfg.as_mut_ptr().add(off), 4);
            };
            put(0, ri32(this.add(MA_UNITS_PER_REV)));
            put(4, ri32(this.add(MA_MAX_VEL)));
            put(8, ri32(this.add(MA_MAX_ACCEL)));
            put(12, ri32(this.add(MA_MAX_DECEL)));
            put(16, ri32(this.add(MA_POS_MIN)));
            put(20, ri32(this.add(MA_POS_MAX)));
            put(24, ri32(this.add(MA_HOME_OFFSET)));
            put(28, ri32(this.add(MA_FOLLOW_WINDOW)));
            let fl = ru32(this.add(MA_FLAGS)).to_le_bytes();
            core::ptr::copy_nonoverlapping(fl.as_ptr(), cfg.as_mut_ptr().add(32), 4);
            let mut h: u32 = 0;
            let f: FMAxisCreate = core::mem::transmute(motionfn(svc, MA_SVC_CREATE));
            let rc = f(svc, cfg.as_ptr(), &mut h);
            wi32(this.add(MA_STATUS), rc);
            wu32(this.add(MA_ERRID), (-rc).max(0) as u32);
            wu32(this.add(MA_AXIS), h);
        }
        2 => {
            let f: FMAxisDestroy = core::mem::transmute(motionfn(svc, MA_SVC_DESTROY));
            let rc = f(svc, ru32(this.add(MA_AXIS)));
            wi32(this.add(MA_STATUS), rc);
            wu32(this.add(MA_ERRID), (-rc).max(0) as u32);
        }
        3 => {
            let f: FMAxisPoll = core::mem::transmute(motionfn(svc, MA_SVC_POLL));
            let rc = f(svc);
            wi32(this.add(MA_STATUS), rc);
        }
        4 => {
            let mut v: i32 = 0;
            let f: FMObjRead = core::mem::transmute(motionfn(svc, MA_SVC_OBJ_READ));
            let rc = f(
                svc,
                ru32(this.add(MA_AXIS)),
                ru32(this.add(MA_INDEX)),
                &mut v,
            );
            wi32(this.add(MA_STATUS), rc);
            wi32(this.add(MA_VALUE), v);
            wu32(this.add(MA_ERRID), (-rc).max(0) as u32);
        }
        5 => {
            let f: FMObjWrite = core::mem::transmute(motionfn(svc, MA_SVC_OBJ_WRITE));
            let rc = f(
                svc,
                ru32(this.add(MA_AXIS)),
                ru32(this.add(MA_INDEX)),
                ri32(this.add(MA_VALUE)),
            );
            wi32(this.add(MA_STATUS), rc);
            wu32(this.add(MA_ERRID), (-rc).max(0) as u32);
        }
        6 => {
            // TPLC_MotionAxisInfo (64 bytes): status@0 mode@4 fault@8
            // target_reached@12 busy@16 pos@20 vel@24
            let mut info = [0u8; 64];
            let f: FMAxisInfo = core::mem::transmute(motionfn(svc, MA_SVC_INFO));
            let rc = f(svc, ru32(this.add(MA_AXIS)), info.as_mut_ptr());
            wi32(this.add(MA_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(info.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(MA_STATUS), rd(0));
                wi32(this.add(MA_MODE), rd(4));
                wi32(this.add(MA_FAULT), rd(8));
                wi32(this.add(MA_TARGET_REACHED), rd(12));
                wi32(this.add(MA_BUSY), rd(16));
                wi32(this.add(MA_POS_ACTUAL), rd(20));
                wi32(this.add(MA_VEL_ACTUAL), rd(24));
            } else {
                wu32(this.add(MA_ERRID), (-rc).max(0) as u32);
            }
        }
        _ => {
            wi32(this.add(MA_STATUS), -1);
            wu32(this.add(MA_ERRID), 1);
        }
    }
    0
}

// FB_NciChan member offsets (see builtin_fb_layout("FB_NciChan")).
// Strings are len-prefixed: PATH u8len@8+data@9 (cap127), PROG
// u8len@136+data@137 (cap4095); ints aligned to 4.
const NC_OP: usize = 0;
const NC_CHAN: usize = 4;
const NC_PATH_LEN: usize = 8;
const NC_PATH: usize = 9;
const NC_PROG_LEN: usize = 136;
const NC_PROG: usize = 137;
const NC_CMD: usize = 4232;
const NC_STATUS: usize = 4236;
const NC_ERRID: usize = 4240;
const NC_STATE: usize = 4244;
const NC_LINE: usize = 4248;
const NC_TOTAL: usize = 4252;
const NC_NERR: usize = 4256;
const NC_PROGRESS: usize = 4260;
const NC_DURATION: usize = 4264;
const NC_AUX: usize = 4268;
const NC_RVAL: usize = 4272;
const NC_ZSX: usize = 4280;
const NC_ZSY: usize = 4288;
const NC_ZSZ: usize = 4296;
const NC_MFUNC: usize = 4304;
const NC_FIFOLEN: usize = 4308;
const NC_FIFOSEQ: usize = 4312;

// tplc-nci service vtable offsets (TplcNciService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
const NC_SVC_LOAD_TEXT: usize = 16;
const NC_SVC_LOAD_FILE: usize = 24;
const NC_SVC_COMMAND: usize = 32;
const NC_SVC_STATUS: usize = 40;
const NC_SVC_FIFO: usize = 48;
const NC_SVC_CONFIRM: usize = 56;
const NC_SVC_RPARAM: usize = 64;
const NC_SVC_ZSHIFT: usize = 72;

/// service lookup for "tplc-nci" (8 chars)
unsafe fn nci_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-nci".as_ptr() as *const u8, 8) as *const u8
}

#[inline]
unsafe fn ncifn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FNciLoadText = unsafe extern "C" fn(*const u8, u32, *const u8, u32) -> i32;
type FNciCommand = unsafe extern "C" fn(*const u8, u32, u32) -> i32;
type FNciStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FNciConfirm = unsafe extern "C" fn(*const u8, u32) -> i32;
type FNciRparam = unsafe extern "C" fn(*const u8, u32, u32, i32, *mut f64) -> i32;
type FNciZshift = unsafe extern "C" fn(*const u8, u32, i32, *mut f64) -> i32;

/// `__tplc_rt_FB_NciChan` — synchronous op dispatch over the tplc-nci
/// NC interpreter channel. OP: 1=load program text (PROG), 2=load a
/// program file on the server host (PATH), 3=command (CMD code),
/// 4=status (fills STATE/LINE/TOTAL/NERR/PROGRESS/DURATION/MFUNC/
/// FIFOLEN/FIFOSEQ), 5=FIFO block append (PROG), 6=confirm pending
/// M-function, 7=R-param write (AUX index, RVAL value), 8=R-param
/// read (AUX -> RVAL), 9=zero-shift write (ZSX/ZSY/ZSZ). STATUS
/// carries the service return code, ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_NciChan(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(NC_OP));
    wi32(this.add(NC_STATUS), 0);
    wu32(this.add(NC_ERRID), 0);
    let svc = nci_service(ctx);
    if svc.is_null() {
        wi32(this.add(NC_STATUS), -2);
        wu32(this.add(NC_ERRID), 99);
        return 0;
    }
    match op {
        1 | 2 => {
            let (len_off, data_off, cap) = if op == 1 {
                (NC_PROG_LEN, NC_PROG, 4095usize)
            } else {
                (NC_PATH_LEN, NC_PATH, 127usize)
            };
            let n = (*this.add(len_off) as usize).min(cap);
            let f: FNciLoadText = core::mem::transmute(ncifn(
                svc,
                if op == 1 {
                    NC_SVC_LOAD_TEXT
                } else {
                    NC_SVC_LOAD_FILE
                },
            ));
            let rc = f(svc, ru32(this.add(NC_CHAN)), this.add(data_off), n as u32);
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
        }
        3 => {
            let f: FNciCommand = core::mem::transmute(ncifn(svc, NC_SVC_COMMAND));
            let rc = f(svc, ru32(this.add(NC_CHAN)), ru32(this.add(NC_CMD)));
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
        }
        4 => {
            // TplcNciStatus (64 bytes): state@0 finished@4 errcode@8
            // line@12 total@16 progress_ms@20 duration_ms@24
            let mut st = [0u8; 64];
            let f: FNciStatus = core::mem::transmute(ncifn(svc, NC_SVC_STATUS));
            let rc = f(svc, ru32(this.add(NC_CHAN)), st.as_mut_ptr());
            wi32(this.add(NC_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(NC_STATE), rd(0));
                wi32(this.add(NC_NERR), rd(8));
                wu32(this.add(NC_LINE), rd(12) as u32);
                wu32(this.add(NC_TOTAL), rd(16) as u32);
                wi32(this.add(NC_PROGRESS), rd(20));
                wi32(this.add(NC_DURATION), rd(24));
                wu32(this.add(NC_MFUNC), rd(28) as u32);
                wu32(this.add(NC_FIFOLEN), rd(32) as u32);
                wu32(this.add(NC_FIFOSEQ), rd(36) as u32);
            } else {
                wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
            }
        }
        5 => {
            let n = (*this.add(NC_PROG_LEN) as usize).min(4095);
            let f: FNciLoadText = core::mem::transmute(ncifn(svc, NC_SVC_FIFO));
            let rc = f(
                svc,
                ru32(this.add(NC_CHAN)),
                this.add(NC_PROG),
                n as u32,
            );
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
        }
        6 => {
            let f: FNciConfirm = core::mem::transmute(ncifn(svc, NC_SVC_CONFIRM));
            let rc = f(svc, ru32(this.add(NC_CHAN)));
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
        }
        7 | 8 => {
            let mut v: f64 = core::ptr::read_unaligned(this.add(NC_RVAL) as *const f64);
            let f: FNciRparam = core::mem::transmute(ncifn(svc, NC_SVC_RPARAM));
            let rc = f(
                svc,
                ru32(this.add(NC_CHAN)),
                ru32(this.add(NC_AUX)),
                if op == 7 { 1 } else { 0 },
                &mut v,
            );
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
            if rc == 0 && op == 8 {
                core::ptr::write_unaligned(this.add(NC_RVAL) as *mut f64, v);
            }
        }
        9 => {
            let mut xyz = [
                core::ptr::read_unaligned(this.add(NC_ZSX) as *const f64),
                core::ptr::read_unaligned(this.add(NC_ZSY) as *const f64),
                core::ptr::read_unaligned(this.add(NC_ZSZ) as *const f64),
            ];
            let f: FNciZshift = core::mem::transmute(ncifn(svc, NC_SVC_ZSHIFT));
            let rc = f(svc, ru32(this.add(NC_CHAN)), 1, xyz.as_mut_ptr());
            wi32(this.add(NC_STATUS), rc);
            wu32(this.add(NC_ERRID), (-rc).max(0) as u32);
        }
        _ => {
            wi32(this.add(NC_STATUS), -1);
            wu32(this.add(NC_ERRID), 1);
        }
    }
    0
}

// ---------------------------------------------------------------------------
// FB_UaClient — OPC UA client over the tplc-opcua service.
// ---------------------------------------------------------------------------

// FB_UaClient member offsets (see builtin_fb_layout("FB_UaClient")).
// Strings are len-prefixed: URL u8len@8+data@9 (cap127), NODE
// u8len@136+data@137 (cap255), SVAL u8len@392+data@393 (cap255).
const UA_OP: usize = 0;
const UA_HANDLE: usize = 4;
const UA_URL_LEN: usize = 8;
const UA_URL: usize = 9;
const UA_NODE_LEN: usize = 136;
const UA_NODE: usize = 137;
const UA_SVAL_LEN: usize = 392;
const UA_SVAL: usize = 393;
const UA_VAL: usize = 648;
const UA_TYPE: usize = 656;
const UA_STATE: usize = 660;
const UA_IDX: usize = 664;
const UA_STATUS: usize = 668;
const UA_ERRID: usize = 672;

// tplc-opcua service vtable offsets (TplcOpcUaService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
const UA_SVC_CONNECT: usize = 16;
const UA_SVC_DISCONNECT: usize = 24;
const UA_SVC_STATUS: usize = 32;
const UA_SVC_READ: usize = 40;
const UA_SVC_WRITE: usize = 48;
const UA_SVC_NS_INDEX: usize = 56;

// TplcOpcUaValue (248 bytes): type@0 i32, reserved@4, union@8
// (i64/u64/f64), str@16..248.
const UAV_TYPE: usize = 0;
const UAV_F64: usize = 8;
const UAV_I64: usize = 8;
const UAV_U64: usize = 8;
const UAV_STR: usize = 16;

/// service lookup for "tplc-opcua" (10 chars)
unsafe fn opcua_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-opcua".as_ptr() as *const u8, 10) as *const u8
}

#[inline]
unsafe fn uafn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FUaConnect = unsafe extern "C" fn(*const u8, *const u8, *mut u32) -> i32;
type FUaDisconnect = unsafe extern "C" fn(*const u8, u32) -> i32;
type FUaStatus = unsafe extern "C" fn(*const u8, u32, *mut i32) -> i32;
type FUaRead = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u8) -> i32;
type FUaWrite = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8) -> i32;
type FUaNsIndex = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u32) -> i32;

/// Copy a len-prefixed ST string (u8 len + data) into a NUL-terminated
/// stack buffer; returns the buffer with `n` set to the length.
unsafe fn uacstr(this: *const u8, len_off: usize, data_off: usize,
                 cap: usize, buf: *mut u8) -> u32 {
    let n = (*this.add(len_off) as usize).min(cap);
    core::ptr::copy_nonoverlapping(this.add(data_off), buf, n);
    *buf.add(n) = 0;
    n as u32
}

/// `__tplc_rt_FB_UaClient` — synchronous op dispatch over the
/// tplc-opcua OPC UA client. OP: 1=connect (URL -> HANDLE),
/// 2=disconnect, 3=status (HANDLE -> STATE), 4=read (NODE -> TYPE +
/// VAL or SVAL), 5=write (NODE + TYPE + VAL/SVAL), 6=namespace index
/// lookup (URL used as the URI -> IDX). STATUS carries the service
/// return code, ERRID its positive form. Numeric kinds widen into the
/// LREAL VAL (precision above 2^53 is lost — an honest limit).
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_UaClient(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(UA_OP));
    wi32(this.add(UA_STATUS), 0);
    wu32(this.add(UA_ERRID), 0);
    let svc = opcua_service(ctx);
    if svc.is_null() {
        wi32(this.add(UA_STATUS), -2);
        wu32(this.add(UA_ERRID), 99);
        return 0;
    }
    match op {
        1 => {
            let mut buf = [0u8; 128];
            uacstr(this, UA_URL_LEN, UA_URL, 127, buf.as_mut_ptr());
            let f: FUaConnect = core::mem::transmute(uafn(svc, UA_SVC_CONNECT));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), &mut h);
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                wu32(this.add(UA_HANDLE), h);
            }
        }
        2 => {
            let f: FUaDisconnect = core::mem::transmute(uafn(svc, UA_SVC_DISCONNECT));
            let rc = f(svc, ru32(this.add(UA_HANDLE)));
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
        }
        3 => {
            let f: FUaStatus = core::mem::transmute(uafn(svc, UA_SVC_STATUS));
            let mut st: i32 = 0;
            let rc = f(svc, ru32(this.add(UA_HANDLE)), &mut st);
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                wi32(this.add(UA_STATE), st);
            }
        }
        4 => {
            let mut buf = [0u8; 256];
            uacstr(this, UA_NODE_LEN, UA_NODE, 255, buf.as_mut_ptr());
            let mut v = [0u8; 248];
            let f: FUaRead = core::mem::transmute(uafn(svc, UA_SVC_READ));
            let rc = f(svc, ru32(this.add(UA_HANDLE)),
                       buf.as_ptr(), v.as_mut_ptr());
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let vt = ri32(v.as_ptr().add(UAV_TYPE));
                wu32(this.add(UA_TYPE), vt as u32);
                match vt {
                    1 => { // BOOL
                        let b = (v.as_ptr().add(UAV_U64) as *const u64).read_unaligned();
                        core::ptr::write_unaligned(this.add(UA_VAL) as *mut f64, b as f64);
                    }
                    2 => { // I64
                        let i = (v.as_ptr().add(UAV_I64) as *const i64).read_unaligned();
                        core::ptr::write_unaligned(this.add(UA_VAL) as *mut f64, i as f64);
                    }
                    3 => { // U64
                        let u = (v.as_ptr().add(UAV_U64) as *const u64).read_unaligned();
                        core::ptr::write_unaligned(this.add(UA_VAL) as *mut f64, u as f64);
                    }
                    4 => { // F64
                        let d = (v.as_ptr().add(UAV_F64) as *const f64).read_unaligned();
                        core::ptr::write_unaligned(this.add(UA_VAL) as *mut f64, d);
                    }
                    5 => { // STRING
                        let mut n2 = 0usize;
                        while n2 < 231 && *v.as_ptr().add(UAV_STR + n2) != 0 {
                            n2 += 1;
                        }
                        let n = n2.min(255);
                        core::ptr::copy_nonoverlapping(
                            v.as_ptr().add(UAV_STR),
                            this.add(UA_SVAL),
                            n,
                        );
                        *this.add(UA_SVAL_LEN) = n as u8;
                        *this.add(UA_SVAL + n) = 0;
                    }
                    _ => {}
                }
            }
        }
        5 => {
            let mut buf = [0u8; 256];
            uacstr(this, UA_NODE_LEN, UA_NODE, 255, buf.as_mut_ptr());
            let mut v = [0u8; 248];
            let vt = ru32(this.add(UA_TYPE)) as i32;
            wi32(v.as_mut_ptr().add(UAV_TYPE), vt);
            let d = core::ptr::read_unaligned(this.add(UA_VAL) as *const f64);
            match vt {
                1 | 3 => core::ptr::write_unaligned(
                    v.as_mut_ptr().add(UAV_U64) as *mut u64,
                    d as u64,
                ),
                2 => core::ptr::write_unaligned(
                    v.as_mut_ptr().add(UAV_I64) as *mut i64,
                    d as i64,
                ),
                _ => core::ptr::write_unaligned(
                    v.as_mut_ptr().add(UAV_F64) as *mut f64,
                    d,
                ),
            }
            if vt == 5 {
                let sn = (*this.add(UA_SVAL_LEN) as usize).min(231);
                core::ptr::copy_nonoverlapping(
                    this.add(UA_SVAL),
                    v.as_mut_ptr().add(UAV_STR),
                    sn,
                );
                *v.as_mut_ptr().add(UAV_STR + sn) = 0;
            }
            let f: FUaWrite = core::mem::transmute(uafn(svc, UA_SVC_WRITE));
            let rc = f(svc, ru32(this.add(UA_HANDLE)),
                       buf.as_ptr(), v.as_ptr());
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
        }
        6 => {
            let mut buf = [0u8; 128];
            uacstr(this, UA_URL_LEN, UA_URL, 127, buf.as_mut_ptr());
            let f: FUaNsIndex = core::mem::transmute(uafn(svc, UA_SVC_NS_INDEX));
            let mut idx: u32 = 0;
            let rc = f(svc, ru32(this.add(UA_HANDLE)),
                       buf.as_ptr(), &mut idx);
            wi32(this.add(UA_STATUS), rc);
            wu32(this.add(UA_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                wu32(this.add(UA_IDX), idx);
            }
        }
        _ => {
            wi32(this.add(UA_STATUS), -1);
            wu32(this.add(UA_ERRID), 1);
        }
    }
    0
}

// ---------------------------------------------------------------------------
// FB_Eip — EtherNet/IP explicit-messaging client over tplc-eip.
// ---------------------------------------------------------------------------

// FB_Eip member offsets (see builtin_fb_layout("FB_Eip")). Strings are
// len-prefixed: HOST u8len@8+data@9 (cap63), DATA u8len@92+data@93
// (cap255).
const EIP_OP: usize = 0;
const EIP_HANDLE: usize = 4;
const EIP_HOST_LEN: usize = 8;
const EIP_HOST: usize = 9;
const EIP_PORT: usize = 72;
const EIP_CLS: usize = 76;
const EIP_INST: usize = 80;
const EIP_ATTR: usize = 84;
const EIP_DATALEN: usize = 88;
const EIP_DATA_LEN: usize = 92;
const EIP_DATA: usize = 93;
const EIP_STATE: usize = 348;
const EIP_SESSION: usize = 352;
const EIP_LASTCIP: usize = 356;
const EIP_STATUS: usize = 360;
const EIP_ERRID: usize = 364;

// tplc-eip service vtable offsets (TplcEipService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
const EIP_SVC_OPEN: usize = 16;
const EIP_SVC_CLOSE: usize = 24;
const EIP_SVC_STATUS: usize = 32;
const EIP_SVC_LISTID: usize = 40;
const EIP_SVC_GET_ATTR: usize = 48;
const EIP_SVC_SET_ATTR: usize = 56;

/// service lookup for "tplc-eip" (8 chars)
unsafe fn eip_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-eip".as_ptr() as *const u8, 8) as *const u8
}

#[inline]
unsafe fn eipfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FEipOpen = unsafe extern "C" fn(*const u8, *const u8, u32, *mut u32) -> i32;
type FEipClose = unsafe extern "C" fn(*const u8, u32) -> i32;
type FEipStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FEipListId = unsafe extern "C" fn(*const u8, *const u8, u32, *mut u8, u32) -> i32;
type FEipGetAttr = unsafe extern "C" fn(*const u8, u32, u32, u32, u32, *mut u8, u32) -> i32;
type FEipSetAttr = unsafe extern "C" fn(*const u8, u32, u32, u32, u32, *const u8, u32) -> i32;

/// `__tplc_rt_FB_Eip` — synchronous op dispatch over the tplc-eip
/// EtherNet/IP client. OP: 1=open+RegisterSession (HOST/PORT -> HANDLE),
/// 2=close+unregister, 3=status (-> STATE/SESSION/LASTCIP),
/// 4=ListIdentity (HOST/PORT -> DATA), 5=Get_Attribute_Single
/// (CLS/INST/ATTR -> DATA + DATALEN), 6=Set_Attribute_Single
/// (CLS/INST/ATTR + DATA/DATALEN). STATUS carries the service return
/// code, ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Eip(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(EIP_OP));
    wi32(this.add(EIP_STATUS), 0);
    wu32(this.add(EIP_ERRID), 0);
    let svc = eip_service(ctx);
    if svc.is_null() {
        wi32(this.add(EIP_STATUS), -2);
        wu32(this.add(EIP_ERRID), 99);
        return 0;
    }
    match op {
        1 => {
            let mut buf = [0u8; 64];
            uacstr(this, EIP_HOST_LEN, EIP_HOST, 63, buf.as_mut_ptr());
            let f: FEipOpen = core::mem::transmute(eipfn(svc, EIP_SVC_OPEN));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), ru32(this.add(EIP_PORT)), &mut h);
            wi32(this.add(EIP_STATUS), rc);
            wu32(this.add(EIP_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                wu32(this.add(EIP_HANDLE), h);
            }
        }
        2 => {
            let f: FEipClose = core::mem::transmute(eipfn(svc, EIP_SVC_CLOSE));
            let rc = f(svc, ru32(this.add(EIP_HANDLE)));
            wi32(this.add(EIP_STATUS), rc);
            wu32(this.add(EIP_ERRID), (-rc).max(0) as u32);
        }
        3 => {
            // TplcEipStatus (32 bytes): state@0 session@4 last_cip@8
            let mut st = [0u8; 32];
            let f: FEipStatus = core::mem::transmute(eipfn(svc, EIP_SVC_STATUS));
            let rc = f(svc, ru32(this.add(EIP_HANDLE)), st.as_mut_ptr());
            wi32(this.add(EIP_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(EIP_STATE), rd(0));
                wu32(this.add(EIP_SESSION), rd(4) as u32);
                wu32(this.add(EIP_LASTCIP), rd(8) as u32);
            } else {
                wu32(this.add(EIP_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let mut hbuf = [0u8; 64];
            uacstr(this, EIP_HOST_LEN, EIP_HOST, 63, hbuf.as_mut_ptr());
            let mut out = [0u8; 255];
            let f: FEipListId = core::mem::transmute(eipfn(svc, EIP_SVC_LISTID));
            let rc = f(svc, hbuf.as_ptr(), ru32(this.add(EIP_PORT)),
                       out.as_mut_ptr(), 255);
            wi32(this.add(EIP_STATUS), rc);
            wu32(this.add(EIP_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let mut n = 0usize;
                while n < 255 && out[n] != 0 { n += 1; }
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(EIP_DATA), n);
                *this.add(EIP_DATA_LEN) = n as u8;
                *this.add(EIP_DATA + n) = 0;
            }
        }
        5 => {
            let mut buf = [0u8; 255];
            let f: FEipGetAttr = core::mem::transmute(eipfn(svc, EIP_SVC_GET_ATTR));
            let rc = f(svc, ru32(this.add(EIP_HANDLE)),
                       ru32(this.add(EIP_CLS)), ru32(this.add(EIP_INST)),
                       ru32(this.add(EIP_ATTR)), buf.as_mut_ptr(), 255);
            wi32(this.add(EIP_STATUS), rc);
            wu32(this.add(EIP_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                let n = (rc as usize).min(255);
                core::ptr::copy_nonoverlapping(buf.as_ptr(), this.add(EIP_DATA), n);
                *this.add(EIP_DATA_LEN) = n as u8;
                *this.add(EIP_DATA + n) = 0;
                wu32(this.add(EIP_DATALEN), n as u32);
            }
        }
        6 => {
            let n = (*this.add(EIP_DATA_LEN) as usize).min(255);
            let f: FEipSetAttr = core::mem::transmute(eipfn(svc, EIP_SVC_SET_ATTR));
            let rc = f(svc, ru32(this.add(EIP_HANDLE)),
                       ru32(this.add(EIP_CLS)), ru32(this.add(EIP_INST)),
                       ru32(this.add(EIP_ATTR)),
                       this.add(EIP_DATA), n as u32);
            wi32(this.add(EIP_STATUS), rc);
            wu32(this.add(EIP_ERRID), (-rc).max(0) as u32);
        }
        _ => {
            wi32(this.add(EIP_STATUS), -1);
            wu32(this.add(EIP_ERRID), 1);
        }
    }
    0
}

// ---------------------------------------------------------------------------
// FB_Iec61850 — IEC 61850 MMS client over tplc-iec61850 (libiec61850).
// ---------------------------------------------------------------------------

// FB_Iec61850 member offsets (see builtin_fb_layout("FB_Iec61850")).
// Strings are len-prefixed: HOST u8len@8+data@9 cap63, DOMAIN
// u8len@76+data@77 cap63, ITEM u8len@140+data@141 cap127, VAL
// u8len@268+data@269 cap255.
const IEC_OP: usize = 0;
const IEC_HANDLE: usize = 4;
const IEC_HOST_LEN: usize = 8;
const IEC_HOST: usize = 9;
const IEC_PORT: usize = 72;
const IEC_DOMAIN_LEN: usize = 76;
const IEC_DOMAIN: usize = 77;
const IEC_ITEM_LEN: usize = 140;
const IEC_ITEM: usize = 141;
const IEC_VAL_LEN: usize = 268;
const IEC_VAL: usize = 269;
const IEC_STATE: usize = 524;
const IEC_INVOKE: usize = 528;
const IEC_LASTERR: usize = 532;
const IEC_STATUS: usize = 536;
const IEC_ERRID: usize = 540;

// tplc-iec61850 service vtable offsets (TplcIec61850Service):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
const IEC_SVC_CONNECT: usize = 16;
const IEC_SVC_CLOSE: usize = 24;
const IEC_SVC_STATUS: usize = 32;
const IEC_SVC_READ: usize = 40;
const IEC_SVC_WRITE: usize = 48;
const IEC_SVC_GETNAMELIST: usize = 56;

/// service lookup for "tplc-iec61850" (12 chars)
unsafe fn iec_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-iec61850".as_ptr() as *const u8, 12) as *const u8
}

#[inline]
unsafe fn iecfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FIecConnect = unsafe extern "C" fn(*const u8, *const u8, u32, *mut u32) -> i32;
type FIecClose = unsafe extern "C" fn(*const u8, u32) -> i32;
type FIecStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FIecRead = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, *mut u8, u32) -> i32;
type FIecWrite = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, *const u8, u32) -> i32;
type FIecGetNameList = unsafe extern "C" fn(*const u8, u32, *const u8, *mut u8, u32) -> i32;

/// `__tplc_rt_FB_Iec61850` — synchronous op dispatch over the
/// tplc-iec61850 MMS client. OP: 1=connect+associate (HOST/PORT ->
/// HANDLE), 2=close+release, 3=status (-> STATE/INVOKE/LASTERR),
/// 4=read (DOMAIN/ITEM -> VAL), 5=write (DOMAIN/ITEM + VAL),
/// 6=GetNameList (DOMAIN -> VAL). STATUS carries the service return
/// code, ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Iec61850(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(IEC_OP));
    wi32(this.add(IEC_STATUS), 0);
    wu32(this.add(IEC_ERRID), 0);
    let svc = iec_service(ctx);
    if svc.is_null() {
        wi32(this.add(IEC_STATUS), -2);
        wu32(this.add(IEC_ERRID), 99);
        return 0;
    }
    match op {
        1 => {
            let mut buf = [0u8; 64];
            uacstr(this, IEC_HOST_LEN, IEC_HOST, 63, buf.as_mut_ptr());
            let f: FIecConnect = core::mem::transmute(iecfn(svc, IEC_SVC_CONNECT));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), ru32(this.add(IEC_PORT)), &mut h);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                wu32(this.add(IEC_HANDLE), h);
            }
        }
        2 => {
            let f: FIecClose = core::mem::transmute(iecfn(svc, IEC_SVC_CLOSE));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)));
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
        }
        3 => {
            // TplcIec61850Status (32 bytes): state@0 invoke_id@4 last_error@8
            let mut st = [0u8; 32];
            let f: FIecStatus = core::mem::transmute(iecfn(svc, IEC_SVC_STATUS));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)), st.as_mut_ptr());
            wi32(this.add(IEC_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(IEC_STATE), rd(0));
                wu32(this.add(IEC_INVOKE), rd(4) as u32);
                wu32(this.add(IEC_LASTERR), rd(8) as u32);
            } else {
                wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let mut dbuf = [0u8; 64];
            let mut ibuf = [0u8; 128];
            uacstr(this, IEC_DOMAIN_LEN, IEC_DOMAIN, 63, dbuf.as_mut_ptr());
            uacstr(this, IEC_ITEM_LEN, IEC_ITEM, 127, ibuf.as_mut_ptr());
            let mut out = [0u8; 255];
            let f: FIecRead = core::mem::transmute(iecfn(svc, IEC_SVC_READ));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       dbuf.as_ptr(), ibuf.as_ptr(), out.as_mut_ptr(), 255);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                let n = (rc as usize).min(255);
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(IEC_VAL), n);
                *this.add(IEC_VAL_LEN) = n as u8;
                *this.add(IEC_VAL + n) = 0;
            }
        }
        5 => {
            let mut dbuf = [0u8; 64];
            let mut ibuf = [0u8; 128];
            uacstr(this, IEC_DOMAIN_LEN, IEC_DOMAIN, 63, dbuf.as_mut_ptr());
            uacstr(this, IEC_ITEM_LEN, IEC_ITEM, 127, ibuf.as_mut_ptr());
            let n = (*this.add(IEC_VAL_LEN) as usize).min(255);
            let f: FIecWrite = core::mem::transmute(iecfn(svc, IEC_SVC_WRITE));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       dbuf.as_ptr(), ibuf.as_ptr(),
                       this.add(IEC_VAL), n as u32);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
        }
        6 => {
            let mut dbuf = [0u8; 64];
            uacstr(this, IEC_DOMAIN_LEN, IEC_DOMAIN, 63, dbuf.as_mut_ptr());
            let mut out = [0u8; 255];
            let f: FIecGetNameList = core::mem::transmute(iecfn(svc, IEC_SVC_GETNAMELIST));
            let rc = f(svc, ru32(this.add(IEC_HANDLE)),
                       dbuf.as_ptr(), out.as_mut_ptr(), 255);
            wi32(this.add(IEC_STATUS), rc);
            wu32(this.add(IEC_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                let mut n = 0usize;
                while n < 255 && out[n] != 0 { n += 1; }
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(IEC_VAL), n);
                *this.add(IEC_VAL_LEN) = n as u8;
                *this.add(IEC_VAL + n) = 0;
            }
        }
        _ => {
            wi32(this.add(IEC_STATUS), -1);
            wu32(this.add(IEC_ERRID), 1);
        }
    }
    0
}

// ---------------------------------------------------------------------------
// FB_Mqtt — MQTT client over tplc-mqtt (libpaho-mqtt3c).
// ---------------------------------------------------------------------------

// FB_Mqtt member offsets (see builtin_fb_layout("FB_Mqtt")).
// Strings are len-prefixed: HOST u8len@8+data@9 cap63, TOPIC
// u8len@84+data@85 cap127, PAYLOAD u8len@212+data@213 cap255,
// RXTOPIC u8len@476+data@477 cap127, RXPAYLOAD u8len@604+data@605
// cap255.
const MQ_OP: usize = 0;
const MQ_HANDLE: usize = 4;
const MQ_HOST_LEN: usize = 8;
const MQ_HOST: usize = 9;
const MQ_PORT: usize = 72;
const MQ_CLEAN: usize = 76;
const MQ_KEEPALIVE: usize = 80;
const MQ_TOPIC_LEN: usize = 84;
const MQ_TOPIC: usize = 85;
const MQ_PAYLOAD_LEN: usize = 212;
const MQ_PAYLOAD: usize = 213;
const MQ_QOS: usize = 468;
const MQ_FLG_RETAIN: usize = 472;
const MQ_RXTOPIC_LEN: usize = 476;
const MQ_RXTOPIC: usize = 477;
const MQ_RXPAYLOAD_LEN: usize = 604;
const MQ_RXPAYLOAD: usize = 605;
const MQ_RXQOS: usize = 860;
const MQ_RXMSGID: usize = 864;
const MQ_RXPLEN: usize = 868;
const MQ_STATE: usize = 872;
const MQ_LASTMSGID: usize = 876;
const MQ_LASTERR: usize = 880;
const MQ_STATUS: usize = 884;
const MQ_ERRID: usize = 888;
const MQ_CONNECTED: usize = 892;

// tplc-mqtt service vtable offsets (TplcMqttService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
const MQ_SVC_CONNECT: usize = 16;
const MQ_SVC_DISCONNECT: usize = 24;
const MQ_SVC_STATUS: usize = 32;
const MQ_SVC_PUBLISH: usize = 40;
const MQ_SVC_SUBSCRIBE: usize = 48;
const MQ_SVC_UNSUBSCRIBE: usize = 56;
const MQ_SVC_RECEIVE: usize = 64;
const MQ_SVC_IS_CONNECTED: usize = 72;

/// service lookup for "tplc-mqtt" (9 chars)
unsafe fn mqtt_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-mqtt".as_ptr() as *const u8, 9) as *const u8
}

#[inline]
unsafe fn mqfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FMqttConnect = unsafe extern "C" fn(*const u8, *const u8, u32, *const u8, *mut u32) -> i32;
type FMqttDisconnect = unsafe extern "C" fn(*const u8, u32) -> i32;
type FMqttStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FMqttPublish = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, u32, u32, u32) -> i32;
type FMqttSubscribe = unsafe extern "C" fn(*const u8, u32, *const u8, u32) -> i32;
type FMqttReceive = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FMqttIsConnected = unsafe extern "C" fn(*const u8, u32) -> i32;

/// `__tplc_rt_FB_Mqtt` — synchronous op dispatch over the tplc-mqtt
/// MQTT client. OP: 1=connect (HOST/PORT/CLEAN/KEEPALIVE -> HANDLE),
/// 2=disconnect, 3=status (-> STATE/LASTMSGID/LASTERR), 4=publish
/// (TOPIC/PAYLOAD/QOS/RETAIN), 5=subscribe (TOPIC/QOS),
/// 6=unsubscribe (TOPIC), 7=receive (-> RXTOPIC/RXPAYLOAD/RXQOS/
/// RXMSGID/RXPLEN or NOMSG), 8=isConnected (-> CONNECTED). STATUS
/// carries the service return code, ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Mqtt(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(MQ_OP));
    wi32(this.add(MQ_STATUS), 0);
    wu32(this.add(MQ_ERRID), 0);
    let svc = mqtt_service(ctx);
    if svc.is_null() {
        wi32(this.add(MQ_STATUS), -2);
        wu32(this.add(MQ_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(MQ_STATUS), rc);
        wu32(this.add(MQ_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 64];
            uacstr(this, MQ_HOST_LEN, MQ_HOST, 63, buf.as_mut_ptr());
            // TplcMqttConnectCfg: clean@0 i32, keepalive@4 u32,
            // version@8 u32, timeout@12 u32, username@16 ptr,
            // password@24 ptr, client_id@32 ptr
            let mut cfg = [0u8; 40];
            let mut wi = |off: usize, v: i32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), cfg.as_mut_ptr().add(off), 4);
            };
            wi(0, ru32(this.add(MQ_CLEAN)) as i32);
            wi(4, ru32(this.add(MQ_KEEPALIVE)) as i32);
            wi(8, 3);       // MQTT 3.1.1 default
            wi(12, 5000);   // connect timeout ms
            // username/password/client_id ptrs left null
            let f: FMqttConnect = core::mem::transmute(mqfn(svc, MQ_SVC_CONNECT));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), ru32(this.add(MQ_PORT)),
                       cfg.as_ptr(), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(MQ_HANDLE), h);
            }
        }
        2 => {
            let f: FMqttDisconnect = core::mem::transmute(mqfn(svc, MQ_SVC_DISCONNECT));
            set_rc(f(svc, ru32(this.add(MQ_HANDLE))));
        }
        3 => {
            // TplcMqttStatus (48B): state@0 conn_flags@4 last_msg_id@8
            // last_qos@12 last_error@16
            let mut st = [0u8; 48];
            let f: FMqttStatus = core::mem::transmute(mqfn(svc, MQ_SVC_STATUS));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)), st.as_mut_ptr());
            wi32(this.add(MQ_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(MQ_STATE), rd(0));
                wu32(this.add(MQ_LASTMSGID), rd(8) as u32);
                wi32(this.add(MQ_LASTERR), rd(16));
            } else {
                wu32(this.add(MQ_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let mut tbuf = [0u8; 128];
            uacstr(this, MQ_TOPIC_LEN, MQ_TOPIC, 127, tbuf.as_mut_ptr());
            let n = (*this.add(MQ_PAYLOAD_LEN) as usize).min(255);
            let f: FMqttPublish = core::mem::transmute(mqfn(svc, MQ_SVC_PUBLISH));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)),
                       tbuf.as_ptr(), this.add(MQ_PAYLOAD), n as u32,
                       ru32(this.add(MQ_QOS)), ru32(this.add(MQ_FLG_RETAIN)));
            set_rc(rc);
        }
        5 => {
            let mut tbuf = [0u8; 128];
            uacstr(this, MQ_TOPIC_LEN, MQ_TOPIC, 127, tbuf.as_mut_ptr());
            let f: FMqttSubscribe = core::mem::transmute(mqfn(svc, MQ_SVC_SUBSCRIBE));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)),
                       tbuf.as_ptr(), ru32(this.add(MQ_QOS)));
            set_rc(rc);
        }
        6 => {
            let mut tbuf = [0u8; 128];
            uacstr(this, MQ_TOPIC_LEN, MQ_TOPIC, 127, tbuf.as_mut_ptr());
            let f: FMqttSubscribe = core::mem::transmute(mqfn(svc, MQ_SVC_UNSUBSCRIBE));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)), tbuf.as_ptr(), 0);
            set_rc(rc);
        }
        7 => {
            // TplcMqttRx: topic_cap@0 payload_cap@4 topic_len@8
            // payload_len@12 qos@16 message_id@20 topic@24 ptr
            // payload@32 ptr
            let mut rx = [0u8; 40];
            let mut wu = |off: usize, v: u32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), rx.as_mut_ptr().add(off), 4);
            };
            wu(0, 127);
            wu(4, 255);
            // topic ptr @24, payload ptr @32 (usize on 64-bit)
            let tp = this.add(MQ_RXTOPIC) as usize;
            let pp = this.add(MQ_RXPAYLOAD) as usize;
            core::ptr::copy_nonoverlapping(
                (tp as usize).to_le_bytes().as_ptr(),
                rx.as_mut_ptr().add(24), 8);
            core::ptr::copy_nonoverlapping(
                (pp as usize).to_le_bytes().as_ptr(),
                rx.as_mut_ptr().add(32), 8);
            let f: FMqttReceive = core::mem::transmute(mqfn(svc, MQ_SVC_RECEIVE));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)), rx.as_mut_ptr());
            wi32(this.add(MQ_STATUS), rc);
            wu32(this.add(MQ_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let rd = |off: usize| -> u32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(rx.as_ptr().add(off), b.as_mut_ptr(), 4);
                    u32::from_le_bytes(b)
                };
                let tl = rd(8).min(127) as usize;
                *this.add(MQ_RXTOPIC_LEN) = tl as u8;
                *this.add(MQ_RXTOPIC + tl) = 0;
                let pl = rd(12).min(255) as usize;
                *this.add(MQ_RXPAYLOAD_LEN) = pl as u8;
                wu32(this.add(MQ_RXQOS), rd(16));
                wu32(this.add(MQ_RXMSGID), rd(20));
                wu32(this.add(MQ_RXPLEN), rd(12));
            }
        }
        8 => {
            let f: FMqttIsConnected = core::mem::transmute(mqfn(svc, MQ_SVC_IS_CONNECTED));
            let rc = f(svc, ru32(this.add(MQ_HANDLE)));
            wi32(this.add(MQ_STATUS), rc);
            wu32(this.add(MQ_CONNECTED), if rc == 1 { 1 } else { 0 });
        }
        _ => {
            wi32(this.add(MQ_STATUS), -1);
            wu32(this.add(MQ_ERRID), 1);
        }
    }
    0
}

// ---------------------------------------------------------------------------
// FB_Ocpp — OCPP-J client over tplc-ocpp (WebSocket + JSON-RPC).
// ---------------------------------------------------------------------------

// FB_Ocpp member offsets (see builtin_fb_layout("FB_Ocpp")). Strings
// are len-prefixed u8len+data.
const OC_OP: usize = 0;
const OC_HANDLE: usize = 4;
const OC_HOST_LEN: usize = 8;
const OC_HOST: usize = 9;
const OC_PORT: usize = 72;
const OC_PATH_LEN: usize = 76;
const OC_PATH: usize = 77;
const OC_OCPPID_LEN: usize = 140;
const OC_OCPPID: usize = 141;
const OC_MSGID_LEN: usize = 204;
const OC_MSGID: usize = 205;
const OC_ACT_LEN: usize = 268;
const OC_ACT: usize = 269;
const OC_CODE_LEN: usize = 332;
const OC_CODE: usize = 333;
const OC_DESC_LEN: usize = 396;
const OC_DESC: usize = 397;
const OC_PAYLOAD_LEN: usize = 524;
const OC_PAYLOAD: usize = 525;
const OC_RXACTION_LEN: usize = 780;
const OC_RXACTION: usize = 781;
const OC_RXPAYLOAD_LEN: usize = 844;
const OC_RXPAYLOAD: usize = 845;
const OC_RXMSGTYPE: usize = 1100;
const OC_STATE: usize = 1104;
const OC_RXTOT: usize = 1108;
const OC_TXTOT: usize = 1112;
const OC_LASTERR: usize = 1116;
const OC_STATUS: usize = 1120;
const OC_ERRID: usize = 1124;

// tplc-ocpp service vtable offsets (TplcOcppService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
const OC_SVC_CONNECT: usize = 16;
const OC_SVC_DISCONNECT: usize = 24;
const OC_SVC_STATUS: usize = 32;
const OC_SVC_CALL: usize = 40;
const OC_SVC_RESULT: usize = 48;
const OC_SVC_CALLERROR: usize = 56;
const OC_SVC_RECEIVE: usize = 64;

/// service lookup for "tplc-ocpp" (9 chars)
unsafe fn ocpp_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-ocpp".as_ptr() as *const u8, 9) as *const u8
}

#[inline]
unsafe fn ocfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FOcppConnect = unsafe extern "C" fn(*const u8, *const u8, u32, *const u8, *const u8, *mut u32) -> i32;
type FOcppDisconnect = unsafe extern "C" fn(*const u8, u32) -> i32;
type FOcppStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FOcppCall = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, *const u8, u32) -> i32;
type FOcppResult = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, u32) -> i32;
type FOcppCallError = unsafe extern "C" fn(*const u8, u32, *const u8, *const u8, *const u8, *const u8, u32) -> i32;
type FOcppReceive = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;

/// `__tplc_rt_FB_Ocpp` — synchronous op dispatch over the tplc-ocpp
/// OCPP-J client. OP: 1=connect (HOST/PORT/PATH/OCPPID -> HANDLE),
/// 2=disconnect, 3=status (-> STATE/RXTOT/TXTOT/LASTERR), 4=call
/// (MSGID/ACTION/PAYLOAD), 5=result (MSGID/PAYLOAD), 6=callerror
/// (MSGID/CODE/DESC/PAYLOAD), 7=receive (-> RXACTION/RXPAYLOAD/
/// RXMSGTYPE or NOMSG). STATUS carries the service return code,
/// ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Ocpp(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(OC_OP));
    wi32(this.add(OC_STATUS), 0);
    wu32(this.add(OC_ERRID), 0);
    let svc = ocpp_service(ctx);
    if svc.is_null() {
        wi32(this.add(OC_STATUS), -2);
        wu32(this.add(OC_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(OC_STATUS), rc);
        wu32(this.add(OC_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut host = [0u8; 64];
            let mut path = [0u8; 64];
            let mut oid = [0u8; 64];
            uacstr(this, OC_HOST_LEN, OC_HOST, 63, host.as_mut_ptr());
            uacstr(this, OC_PATH_LEN, OC_PATH, 63, path.as_mut_ptr());
            uacstr(this, OC_OCPPID_LEN, OC_OCPPID, 63, oid.as_mut_ptr());
            let f: FOcppConnect = core::mem::transmute(ocfn(svc, OC_SVC_CONNECT));
            let mut h: u32 = 0;
            let rc = f(svc, host.as_ptr(), ru32(this.add(OC_PORT)),
                       path.as_ptr(), oid.as_ptr(), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(OC_HANDLE), h);
            }
        }
        2 => {
            let f: FOcppDisconnect = core::mem::transmute(ocfn(svc, OC_SVC_DISCONNECT));
            set_rc(f(svc, ru32(this.add(OC_HANDLE))));
        }
        3 => {
            // TplcOcppStatus (48B): state@0 rx_total@4 tx_total@8
            // last_error@12
            let mut st = [0u8; 48];
            let f: FOcppStatus = core::mem::transmute(ocfn(svc, OC_SVC_STATUS));
            let rc = f(svc, ru32(this.add(OC_HANDLE)), st.as_mut_ptr());
            wi32(this.add(OC_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(OC_STATE), rd(0));
                wu32(this.add(OC_RXTOT), rd(4) as u32);
                wu32(this.add(OC_TXTOT), rd(8) as u32);
                wi32(this.add(OC_LASTERR), rd(12));
            } else {
                wu32(this.add(OC_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let mut mid = [0u8; 64];
            let mut act = [0u8; 64];
            uacstr(this, OC_MSGID_LEN, OC_MSGID, 63, mid.as_mut_ptr());
            uacstr(this, OC_ACT_LEN, OC_ACT, 63, act.as_mut_ptr());
            let n = (*this.add(OC_PAYLOAD_LEN) as usize).min(255);
            let f: FOcppCall = core::mem::transmute(ocfn(svc, OC_SVC_CALL));
            let rc = f(svc, ru32(this.add(OC_HANDLE)),
                       mid.as_ptr(), act.as_ptr(),
                       this.add(OC_PAYLOAD), n as u32);
            set_rc(rc);
        }
        5 => {
            let mut mid = [0u8; 64];
            uacstr(this, OC_MSGID_LEN, OC_MSGID, 63, mid.as_mut_ptr());
            let n = (*this.add(OC_PAYLOAD_LEN) as usize).min(255);
            let f: FOcppResult = core::mem::transmute(ocfn(svc, OC_SVC_RESULT));
            let rc = f(svc, ru32(this.add(OC_HANDLE)),
                       mid.as_ptr(), this.add(OC_PAYLOAD), n as u32);
            set_rc(rc);
        }
        6 => {
            let mut mid = [0u8; 64];
            let mut code = [0u8; 64];
            let mut desc = [0u8; 128];
            uacstr(this, OC_MSGID_LEN, OC_MSGID, 63, mid.as_mut_ptr());
            uacstr(this, OC_CODE_LEN, OC_CODE, 63, code.as_mut_ptr());
            uacstr(this, OC_DESC_LEN, OC_DESC, 127, desc.as_mut_ptr());
            let n = (*this.add(OC_PAYLOAD_LEN) as usize).min(255);
            let f: FOcppCallError = core::mem::transmute(ocfn(svc, OC_SVC_CALLERROR));
            let rc = f(svc, ru32(this.add(OC_HANDLE)),
                       mid.as_ptr(), code.as_ptr(), desc.as_ptr(),
                       this.add(OC_PAYLOAD), n as u32);
            set_rc(rc);
        }
        7 => {
            // TplcOcppRx: action_cap@0 payload_cap@4 action_len@8
            // payload_len@12 msg_type@16 reserved@20 action@24 ptr
            // payload@32 ptr
            let mut rx = [0u8; 40];
            let mut wu = |off: usize, v: u32| {
                let b = v.to_le_bytes();
                core::ptr::copy_nonoverlapping(b.as_ptr(), rx.as_mut_ptr().add(off), 4);
            };
            wu(0, 63);
            wu(4, 255);
            let ap = this.add(OC_RXACTION) as usize;
            let pp = this.add(OC_RXPAYLOAD) as usize;
            core::ptr::copy_nonoverlapping(
                ap.to_le_bytes().as_ptr(), rx.as_mut_ptr().add(24), 8);
            core::ptr::copy_nonoverlapping(
                pp.to_le_bytes().as_ptr(), rx.as_mut_ptr().add(32), 8);
            let f: FOcppReceive = core::mem::transmute(ocfn(svc, OC_SVC_RECEIVE));
            let rc = f(svc, ru32(this.add(OC_HANDLE)), rx.as_mut_ptr());
            wi32(this.add(OC_STATUS), rc);
            wu32(this.add(OC_ERRID), (-rc).max(0) as u32);
            if rc == 0 {
                let rd = |off: usize| -> u32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(rx.as_ptr().add(off), b.as_mut_ptr(), 4);
                    u32::from_le_bytes(b)
                };
                let al = rd(8).min(63) as usize;
                *this.add(OC_RXACTION_LEN) = al as u8;
                *this.add(OC_RXACTION + al) = 0;
                let pl = rd(12).min(255) as usize;
                *this.add(OC_RXPAYLOAD_LEN) = pl as u8;
                wu32(this.add(OC_RXMSGTYPE), rd(16));
            }
        }
        _ => {
            wi32(this.add(OC_STATUS), -1);
            wu32(this.add(OC_ERRID), 1);
        }
    }
    0
}

// ---------------------------------------------------------------------------
// FB_Aes70 — OCA/AES70 audio-control client over tplc-aes70 (OSCP TCP).
// ---------------------------------------------------------------------------

// FB_Aes70 member offsets (see builtin_fb_layout("FB_Aes70")). Strings
// are len-prefixed u8len+data.
const A7_OP: usize = 0;
const A7_HANDLE: usize = 4;
const A7_HOST_LEN: usize = 8;
const A7_HOST: usize = 9;
const A7_PORT: usize = 72;
const A7_TARGET: usize = 76;
const A7_METH: usize = 80;
const A7_PARAMS_LEN: usize = 84;
const A7_PARAMS: usize = 85;
const A7_HANDLE2: usize = 340;
const A7_STATUSCODE: usize = 344;
const A7_RXPAYLOAD_LEN: usize = 348;
const A7_RXPAYLOAD: usize = 349;
const A7_STATE: usize = 604;
const A7_RXTOT: usize = 608;
const A7_TXTOT: usize = 612;
const A7_LASTERR: usize = 616;
const A7_STATUS: usize = 620;
const A7_ERRID: usize = 624;

// tplc-aes70 service vtable offsets (TplcAes70Service):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
const A7_SVC_CONNECT: usize = 16;
const A7_SVC_DISCONNECT: usize = 24;
const A7_SVC_STATUS: usize = 32;
const A7_SVC_CALL: usize = 40;
const A7_SVC_KEEPALIVE: usize = 48;
const A7_SVC_RECEIVE: usize = 56;

/// service lookup for "tplc-aes70" (10 chars)
unsafe fn aes70_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-aes70".as_ptr() as *const u8, 10) as *const u8
}

#[inline]
unsafe fn a7fn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

type FAes70Connect = unsafe extern "C" fn(*const u8, *const u8, u32, *mut u32) -> i32;
type FAes70Disconnect = unsafe extern "C" fn(*const u8, u32) -> i32;
type FAes70Status = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
type FAes70Call = unsafe extern "C" fn(*const u8, u32, u32, u32, *const u8, u32, *mut u32) -> i32;
type FAes70Keepalive = unsafe extern "C" fn(*const u8, u32) -> i32;
type FAes70Receive = unsafe extern "C" fn(*const u8, u32, *mut u32, *mut u32, *mut u8, u32) -> i32;

/// `__tplc_rt_FB_Aes70` — synchronous op dispatch over the tplc-aes70
/// OCA client. OP: 1=connect (HOST/PORT -> HANDLE), 2=disconnect,
/// 3=status (-> STATE/RXTOT/TXTOT/LASTERR), 4=call (TARGET/METHOD/
/// PARAMS -> HANDLE2), 5=keepalive, 6=receive (-> HANDLE2/STATUSCODE/
/// RXPAYLOAD or NOMSG). STATUS carries the service return code,
/// ERRID its positive form.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Aes70(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(A7_OP));
    wi32(this.add(A7_STATUS), 0);
    wu32(this.add(A7_ERRID), 0);
    let svc = aes70_service(ctx);
    if svc.is_null() {
        wi32(this.add(A7_STATUS), -2);
        wu32(this.add(A7_ERRID), 99);
        return 0;
    }
    let set_rc = |rc: i32| {
        wi32(this.add(A7_STATUS), rc);
        wu32(this.add(A7_ERRID), (-rc).max(0) as u32);
    };
    match op {
        1 => {
            let mut buf = [0u8; 64];
            uacstr(this, A7_HOST_LEN, A7_HOST, 63, buf.as_mut_ptr());
            let f: FAes70Connect = core::mem::transmute(a7fn(svc, A7_SVC_CONNECT));
            let mut h: u32 = 0;
            let rc = f(svc, buf.as_ptr(), ru32(this.add(A7_PORT)), &mut h);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(A7_HANDLE), h);
            }
        }
        2 => {
            let f: FAes70Disconnect = core::mem::transmute(a7fn(svc, A7_SVC_DISCONNECT));
            set_rc(f(svc, ru32(this.add(A7_HANDLE))));
        }
        3 => {
            // TplcAes70Status (48B): state@0 rx_total@4 tx_total@8
            // last_error@12
            let mut st = [0u8; 48];
            let f: FAes70Status = core::mem::transmute(a7fn(svc, A7_SVC_STATUS));
            let rc = f(svc, ru32(this.add(A7_HANDLE)), st.as_mut_ptr());
            wi32(this.add(A7_STATUS), rc);
            if rc == 0 {
                let rd = |off: usize| -> i32 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(st.as_ptr().add(off), b.as_mut_ptr(), 4);
                    i32::from_le_bytes(b)
                };
                wi32(this.add(A7_STATE), rd(0));
                wu32(this.add(A7_RXTOT), rd(4) as u32);
                wu32(this.add(A7_TXTOT), rd(8) as u32);
                wi32(this.add(A7_LASTERR), rd(12));
            } else {
                wu32(this.add(A7_ERRID), (-rc).max(0) as u32);
            }
        }
        4 => {
            let n = (*this.add(A7_PARAMS_LEN) as usize).min(255);
            let f: FAes70Call = core::mem::transmute(a7fn(svc, A7_SVC_CALL));
            let mut h2: u32 = 0;
            let rc = f(svc, ru32(this.add(A7_HANDLE)),
                       ru32(this.add(A7_TARGET)), ru32(this.add(A7_METH)),
                       this.add(A7_PARAMS), n as u32, &mut h2);
            set_rc(rc);
            if rc == 0 {
                wu32(this.add(A7_HANDLE2), h2);
            }
        }
        5 => {
            let f: FAes70Keepalive = core::mem::transmute(a7fn(svc, A7_SVC_KEEPALIVE));
            set_rc(f(svc, ru32(this.add(A7_HANDLE))));
        }
        6 => {
            let mut out = [0u8; 255];
            let mut h2: u32 = 0;
            let mut sc: u32 = 0;
            let f: FAes70Receive = core::mem::transmute(a7fn(svc, A7_SVC_RECEIVE));
            let rc = f(svc, ru32(this.add(A7_HANDLE)),
                       &mut h2, &mut sc, out.as_mut_ptr(), 255);
            wi32(this.add(A7_STATUS), rc);
            wu32(this.add(A7_ERRID), (-rc).max(0) as u32);
            if rc >= 0 {
                wu32(this.add(A7_HANDLE2), h2);
                wu32(this.add(A7_STATUSCODE), sc);
                let n = (rc as usize).min(255);
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(A7_RXPAYLOAD), n);
                *this.add(A7_RXPAYLOAD_LEN) = n as u8;
            }
        }
        _ => {
            wi32(this.add(A7_STATUS), -1);
            wu32(this.add(A7_ERRID), 1);
        }
    }
    0
}
