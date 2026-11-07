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

#![no_std]

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

/* ---------------- timers (shared block) ---------------- */

/// TON: IN↑ captures start; while IN, ET=min(now-start,PT), Q=(ET>=PT).
#[no_mangle]
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
pub unsafe extern "C" fn __tplc_rt_R_TRIG(this: *mut u8, _ctx: *mut u8) -> i32 {
    let clk = rb(this);
    let last = rb(this.add(2));
    wb(this.add(1), clk && !last);
    wb(this.add(2), clk);
    0
}

#[no_mangle]
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
pub unsafe extern "C" fn __tplc_rt_CTU(this: *mut u8, _ctx: *mut u8) -> i32 {
    ctud_step(this);
    0
}
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_CTD(this: *mut u8, _ctx: *mut u8) -> i32 {
    ctud_step(this);
    0
}
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_CTUD(this: *mut u8, _ctx: *mut u8) -> i32 {
    ctud_step(this);
    0
}

/* ---------------- bistables ---------------- */

/// SR: set-dominant (S1 wins over R).
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_SR(this: *mut u8, _ctx: *mut u8) -> i32 {
    let s1 = rb(this.add(2));
    let r = rb(this.add(3));
    let q1 = if s1 { true } else if r { false } else { rb(this.add(4)) };
    wb(this.add(4), q1);
    0
}

/// RS: reset-dominant (R1 wins over S).
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_RS(this: *mut u8, _ctx: *mut u8) -> i32 {
    let s = rb(this);
    let r1 = rb(this.add(1));
    let q1 = if r1 { false } else if s { true } else { rb(this.add(4)) };
    wb(this.add(4), q1);
    0
}
