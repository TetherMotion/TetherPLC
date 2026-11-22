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


mod std_fb;
mod socket;
mod serial;
mod ads;
mod rtmon;
mod modbus;
mod modbus_rtu;
mod sacn;
mod bacnet;
mod pjlink;
mod s7;
mod s5;
mod ftp;
mod smtp;
mod iec870;
mod motion_axis;
mod nci_chan;
mod fifo;
mod kin;
mod mcgroup;
mod cam;
mod path;
mod ua_client;
mod eip;
mod iec61850;
mod mqtt;
mod ec_sync;
mod ec_diag_read;
mod can_open;
mod io_link;
mod profinet;
mod soe;
mod ocpp;
mod aes70;
mod dmx;
mod mbus;
mod en_ocean;
mod genibus;
mod mpbus;
mod dyn_mem;
mod event_log;
mod db;
mod xml_srv;
mod sym;
mod iof;

#[allow(unused_imports)]
pub use std_fb::*;
#[allow(unused_imports)]
pub use socket::*;
#[allow(unused_imports)]
pub use serial::*;
#[allow(unused_imports)]
pub use ads::*;
#[allow(unused_imports)]
pub use rtmon::*;
#[allow(unused_imports)]
pub use modbus::*;
#[allow(unused_imports)]
pub use modbus_rtu::*;
#[allow(unused_imports)]
pub use sacn::*;
#[allow(unused_imports)]
pub use bacnet::*;
#[allow(unused_imports)]
pub use pjlink::*;
#[allow(unused_imports)]
pub use s7::*;
#[allow(unused_imports)]
pub use s5::*;
#[allow(unused_imports)]
pub use ftp::*;
#[allow(unused_imports)]
pub use smtp::*;
#[allow(unused_imports)]
pub use iec870::*;
#[allow(unused_imports)]
pub use motion_axis::*;
#[allow(unused_imports)]
pub use nci_chan::*;
#[allow(unused_imports)]
pub use fifo::*;
#[allow(unused_imports)]
pub use kin::*;
#[allow(unused_imports)]
pub use mcgroup::*;
#[allow(unused_imports)]
pub use cam::*;
#[allow(unused_imports)]
pub use path::*;
#[allow(unused_imports)]
pub use ua_client::*;
#[allow(unused_imports)]
pub use eip::*;
#[allow(unused_imports)]
pub use iec61850::*;
#[allow(unused_imports)]
pub use mqtt::*;
#[allow(unused_imports)]
pub use ec_sync::*;
#[allow(unused_imports)]
pub use ec_diag_read::*;
#[allow(unused_imports)]
pub use can_open::*;
#[allow(unused_imports)]
pub use io_link::*;
#[allow(unused_imports)]
pub use profinet::*;
#[allow(unused_imports)]
pub use soe::*;
#[allow(unused_imports)]
pub use ocpp::*;
#[allow(unused_imports)]
pub use aes70::*;
#[allow(unused_imports)]
pub use dmx::*;
#[allow(unused_imports)]
pub use mbus::*;
#[allow(unused_imports)]
pub use en_ocean::*;
#[allow(unused_imports)]
pub use genibus::*;
#[allow(unused_imports)]
pub use mpbus::*;
#[allow(unused_imports)]
pub use dyn_mem::*;
#[allow(unused_imports)]
pub use event_log::*;
#[allow(unused_imports)]
pub use db::*;
#[allow(unused_imports)]
pub use xml_srv::*;
#[allow(unused_imports)]
pub use sym::*;
#[allow(unused_imports)]
pub use iof::*;
