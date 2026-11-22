//! builtin_fb_layout arms — timers domain.
use super::*;

pub(super) fn arm_ton(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            push(&mut off, "IN", Type::Bool);
            push(&mut off, "PT", Type::Time);
            push(&mut off, "Q", Type::Bool);
            push(&mut off, "ET", Type::Time);
            push(&mut off, "__in_last", Type::Bool);
            push(&mut off, "__start", Type::Time);
            push(&mut off, "__run", Type::Bool);
    off
}

pub(super) fn arm_r_trig(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            push(&mut off, "CLK", Type::Bool);
            push(&mut off, "Q", Type::Bool);
            push(&mut off, "__clk_last", Type::Bool);
    off
}

pub(super) fn arm_ctu(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            push(&mut off, "CU", Type::Bool);
            push(&mut off, "CD", Type::Bool);
            push(&mut off, "R", Type::Bool);
            push(&mut off, "LD", Type::Bool);
            push(
                &mut off,
                "PV",
                Type::Int {
                    signed: true,
                    bits: 32,
                },
            );
            push(&mut off, "QU", Type::Bool);
            push(&mut off, "QD", Type::Bool);
            push(
                &mut off,
                "CV",
                Type::Int {
                    signed: true,
                    bits: 32,
                },
            );
            push(&mut off, "__cu_last", Type::Bool);
            push(&mut off, "__cd_last", Type::Bool);
    off
}

pub(super) fn arm_rs(off: u64, m: &mut Members) -> u64 {
    let mut push = |o: &mut u64, n: &str, t: Type| {
        let a = t.align();
        *o = (*o + a - 1) & !(a - 1);
        m.insert(n.to_string(), (*o, t.clone()));
        *o += t.size();
    };
    let mut off = off;
            push(&mut off, "S", Type::Bool);
            push(&mut off, "R1", Type::Bool);
            push(&mut off, "S1", Type::Bool);
            push(&mut off, "R", Type::Bool);
            push(&mut off, "Q1", Type::Bool);
    off
}
