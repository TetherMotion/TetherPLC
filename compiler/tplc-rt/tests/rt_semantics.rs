//! tplc-rt semantics tests — drives the extern fns against byte blocks,
//! per doc 07 App C waveforms.

use tplc_rt::*;

#[repr(C, align(8))]
struct Ctx([u8; 104]);
impl Ctx {
    fn at(now: i64) -> Self {
        let mut c = Ctx([0; 104]);
        c.0[48..56].copy_from_slice(&now.to_le_bytes());
        c
    }
}
unsafe fn now_ctx(t: i64) -> *const u8 {
    Box::leak(Box::new(Ctx::at(t))).0.as_ptr()
}

#[test]
fn ton_waveform() {
    let mut b = [0u8; 56];
    b[8..16].copy_from_slice(&500i64.to_le_bytes()); // PT
    unsafe {
        let ctx = now_ctx(0);
        __tplc_rt_TON(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 0);
        b[0] = 1; // IN rises at t=0
        __tplc_rt_TON(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 0);
        let ctx = now_ctx(499);
        __tplc_rt_TON(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 0);
        let ctx = now_ctx(500);
        __tplc_rt_TON(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 1, "Q at PT");
        assert_eq!(i64::from_le_bytes(b[24..32].try_into().unwrap()), 500);
        // ET clamps at PT
        let ctx = now_ctx(900);
        __tplc_rt_TON(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(i64::from_le_bytes(b[24..32].try_into().unwrap()), 500);
        // IN falls → reset
        b[0] = 0;
        __tplc_rt_TON(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 0);
        assert_eq!(i64::from_le_bytes(b[24..32].try_into().unwrap()), 0);
    }
}

#[test]
fn tof_waveform() {
    let mut b = [0u8; 56];
    b[8..16].copy_from_slice(&500i64.to_le_bytes());
    unsafe {
        let ctx = now_ctx(0);
        b[0] = 1;
        __tplc_rt_TOF(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 1, "Q high while IN");
        b[0] = 0; // falling edge at t=0
        __tplc_rt_TOF(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 1, "Q holds during off-delay");
        let ctx = now_ctx(499);
        __tplc_rt_TOF(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 1);
        let ctx = now_ctx(500);
        __tplc_rt_TOF(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 0, "Q drops at PT");
    }
}

#[test]
fn tp_pulse() {
    let mut b = [0u8; 56];
    b[8..16].copy_from_slice(&300i64.to_le_bytes());
    unsafe {
        let ctx = now_ctx(0);
        b[0] = 1;
        __tplc_rt_TP(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 1);
        b[0] = 0; // IN can drop; pulse continues
        let ctx = now_ctx(100);
        __tplc_rt_TP(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 1);
        // non-retriggerable: IN back up mid-pulse does not extend
        b[0] = 1;
        let ctx = now_ctx(200);
        __tplc_rt_TP(b.as_mut_ptr(), ctx as *mut u8);
        b[0] = 0;
        let ctx = now_ctx(300);
        __tplc_rt_TP(b.as_mut_ptr(), ctx as *mut u8);
        assert_eq!(b[16], 0, "pulse ends at PT");
    }
}

#[test]
fn edge_and_counters() {
    let mut b = [0u8; 8];
    unsafe {
        let ctx = now_ctx(0) as *mut u8;
        b[0] = 1;
        __tplc_rt_R_TRIG(b.as_mut_ptr(), ctx);
        assert_eq!(b[1], 1, "rising edge");
        __tplc_rt_R_TRIG(b.as_mut_ptr(), ctx);
        assert_eq!(b[1], 0, "one cycle only");
        b[0] = 0;
        __tplc_rt_F_TRIG(b.as_mut_ptr(), ctx);
        assert_eq!(b[1], 1, "falling edge");
    }
    let mut c = [0u8; 24];
    c[4..8].copy_from_slice(&3i32.to_le_bytes()); // PV=3
    unsafe {
        let ctx = now_ctx(0) as *mut u8;
        for _ in 0..3 {
            c[0] = 1;
            __tplc_rt_CTU(c.as_mut_ptr(), ctx);
            c[0] = 0;
            __tplc_rt_CTU(c.as_mut_ptr(), ctx);
        }
        assert_eq!(i32::from_le_bytes(c[12..16].try_into().unwrap()), 3);
        assert_eq!(c[8], 1, "QU at CV>=PV");
        c[2] = 1; // R
        __tplc_rt_CTU(c.as_mut_ptr(), ctx);
        assert_eq!(i32::from_le_bytes(c[12..16].try_into().unwrap()), 0);
        assert_eq!(c[8], 0);
        // CTD
        c[2] = 0;
        c[3] = 1; // LD
        __tplc_rt_CTD(c.as_mut_ptr(), ctx);
        assert_eq!(i32::from_le_bytes(c[12..16].try_into().unwrap()), 3);
        c[3] = 0;
        for _ in 0..3 {
            c[1] = 1;
            __tplc_rt_CTD(c.as_mut_ptr(), ctx);
            c[1] = 0;
            __tplc_rt_CTD(c.as_mut_ptr(), ctx);
        }
        assert_eq!(i32::from_le_bytes(c[12..16].try_into().unwrap()), 0);
        assert_eq!(c[9], 1, "QD at CV<=0");
    }
}

#[test]
fn bistables() {
    let mut b = [0u8; 8];
    unsafe {
        let ctx = now_ctx(0) as *mut u8;
        // SR: set-dominant
        b[2] = 1; b[3] = 1; // S1=1, R=1 → set wins
        __tplc_rt_SR(b.as_mut_ptr(), ctx);
        assert_eq!(b[4], 1);
        b[2] = 0;
        __tplc_rt_SR(b.as_mut_ptr(), ctx);
        assert_eq!(b[4], 0, "R clears");
        // RS: reset-dominant
        b[0] = 1; b[1] = 1; // S=1, R1=1 → reset wins
        __tplc_rt_RS(b.as_mut_ptr(), ctx);
        assert_eq!(b[4], 0);
        b[1] = 0;
        __tplc_rt_RS(b.as_mut_ptr(), ctx);
        assert_eq!(b[4], 1, "S sets once R1 clears");
    }
}
