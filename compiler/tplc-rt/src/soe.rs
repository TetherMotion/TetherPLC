//! tplc-rt builtin FB: soe.
use crate::*;

// FB_Soe — EtherCAT SoE mailbox over tplc-soe (Tether IO ec.soe.*
// params).
// ---------------------------------------------------------------------------

// FB_Soe member offsets (see builtin_fb_layout("FB_Soe")).
pub(crate) const SOE_OP: usize = 0;
pub(crate) const SOE_SLAVE: usize = 4;
pub(crate) const SOE_VALUE: usize = 8;
pub(crate) const SOE_STATUS: usize = 12;
pub(crate) const SOE_ERRID: usize = 16;
pub(crate) const SOE_STATUSW: usize = 20;
pub(crate) const SOE_CMDSTATE: usize = 24;
pub(crate) const SOE_PASSWD: usize = 28;
pub(crate) const SOE_DIAGNUM: usize = 32;
pub(crate) const SOE_CLASSX: usize = 36;
pub(crate) const SOE_DIAGCOUNT: usize = 40;
pub(crate) const SOE_DIAGLIST: usize = 44; // u32[8]
pub(crate) const SOE_AMPTEMP: usize = 80;  // f64
pub(crate) const SOE_MTEMPT: usize = 88;
pub(crate) const SOE_DCVOLT: usize = 96;
pub(crate) const SOE_DCCURR: usize = 104;
pub(crate) const SOE_AXMAINS: usize = 112;
pub(crate) const SOE_AXCTRL: usize = 120;
pub(crate) const SOE_AXPARK: usize = 124;
pub(crate) const SOE_AXFWREV: usize = 128;
pub(crate) const SOE_AXFWBUILD: usize = 132;
pub(crate) const SOE_CMDCTRL: usize = 136;
pub(crate) const SOE_DIAGMSG: usize = 140; // Str{63}: u8 len + data

// tplc-soe service vtable offsets (TplcSoeService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs:
// soe_read@16, write_password@24, write_cmd_ctrl@32,
// exec_command@40, reset@48, write_ax_ctrl@56, write_ax_park@64.
pub(crate) const SOE_SVC_READ: usize = 16;
pub(crate) const SOE_SVC_WR_PW: usize = 24;
pub(crate) const SOE_SVC_WR_CTRL: usize = 32;
pub(crate) const SOE_SVC_EXEC: usize = 40;
pub(crate) const SOE_SVC_RESET: usize = 48;
pub(crate) const SOE_SVC_WR_AXCTRL: usize = 56;
pub(crate) const SOE_SVC_WR_AXPARK: usize = 64;

// TplcSoeState decode offsets (pinned by static_assert in
// SoeService.cpp): status@0 u16, cmd_state@2 u16, password@4 u32,
// diag_num@8 u16, classx@10 u16, diag_count@12 u16, diag_list@14
// u16[8], amp_temp@32 f32, motor_temp@36, dc_volt@40, dc_curr@44,
// ax_mains@48, ax_ctrl@52 u16, ax_park@54, ax_fw_rev@56,
// ax_fw_build@58, cmd_ctrl@60 u16, diagmsg@62 char[64].

/// service lookup for "tplc-soe" (8 chars)
pub(crate) unsafe fn soe_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-soe".as_ptr() as *const u8, 8) as *const u8
}

/// `__tplc_rt_FB_Soe` — OP 1=read state, OP 2..7=mailbox writes.
/// # Safety
/// `this` must point at a valid instance block of the documented
/// layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Soe(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(SOE_OP));
    wi32(this.add(SOE_STATUS), 0);
    wu32(this.add(SOE_ERRID), 0);
    let svc = soe_service(ctx);
    if svc.is_null() {
        wi32(this.add(SOE_STATUS), -2);
        wu32(this.add(SOE_ERRID), 99);
        return 0;
    }
    let slave = ru32(this.add(SOE_SLAVE));
    let set_rc = |rc: i32| {
        wi32(this.add(SOE_STATUS), rc);
        wu32(this.add(SOE_ERRID), (-rc).max(0) as u32);
    };
    let wr_fn = |off: usize| -> unsafe extern "C" fn(*const u8, u32, u32) -> i32 {
        core::mem::transmute((svc.add(off) as *const usize).read())
    };
    match op {
        1 => {
            // TplcSoeState (128 bytes) on stack
            let mut st = [0u8; 128];
            let f: unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32 =
                core::mem::transmute(
                    (svc.add(SOE_SVC_READ) as *const usize).read(),
                );
            let rc = f(svc, slave, st.as_mut_ptr());
            set_rc(rc);
            if rc == 0 {
                let p = st.as_ptr();
                let rd16 = |o: usize| -> u32 {
                    ru16(p.add(o)) as u32
                };
                let rd32 = |o: usize| -> u32 { ru32(p.add(o)) };
                let rdf32 = |o: usize| -> f64 {
                    let mut b = [0u8; 4];
                    core::ptr::copy_nonoverlapping(p.add(o), b.as_mut_ptr(), 4);
                    f32::from_le_bytes(b) as f64
                };
                wu32(this.add(SOE_STATUSW), rd16(0));
                wu32(this.add(SOE_CMDSTATE), rd16(2));
                wu32(this.add(SOE_PASSWD), rd32(4));
                wu32(this.add(SOE_DIAGNUM), rd16(8));
                wu32(this.add(SOE_CLASSX), rd16(10));
                wu32(this.add(SOE_DIAGCOUNT), rd16(12));
                let cnt = rd16(12).min(8) as usize;
                for i in 0..cnt {
                    wu32(this.add(SOE_DIAGLIST + i * 4), rd16(14 + i * 2));
                }
                let wrf = |off: usize, v: f64| {
                    core::ptr::write_unaligned(
                        this.add(off) as *mut f64,
                        v,
                    );
                };
                wrf(SOE_AMPTEMP, rdf32(32));
                wrf(SOE_MTEMPT, rdf32(36));
                wrf(SOE_DCVOLT, rdf32(40));
                wrf(SOE_DCCURR, rdf32(44));
                wrf(SOE_AXMAINS, rdf32(48));
                wu32(this.add(SOE_AXCTRL), rd16(52));
                wu32(this.add(SOE_AXPARK), rd16(54));
                wu32(this.add(SOE_AXFWREV), rd16(56));
                wu32(this.add(SOE_AXFWBUILD), rd16(58));
                wu32(this.add(SOE_CMDCTRL), rd16(60));
                // diagmsg char[64] -> Str blob: u8 len + bytes
                let mt = this.add(SOE_DIAGMSG);
                let mut len = 0usize;
                while len < 63 && *p.add(62 + len) != 0 {
                    len += 1;
                }
                *mt = len as u8;
                core::ptr::copy_nonoverlapping(p.add(62), mt.add(1), len);
            }
        }
        2..=7 => {
            let off = match op {
                2 => SOE_SVC_WR_PW,
                3 => SOE_SVC_WR_CTRL,
                4 => SOE_SVC_EXEC,
                5 => SOE_SVC_RESET,
                6 => SOE_SVC_WR_AXCTRL,
                _ => SOE_SVC_WR_AXPARK,
            };
            let v = if op == 5 { 0 } else { ru32(this.add(SOE_VALUE)) };
            set_rc(wr_fn(off)(svc, slave, v));
        }
        _ => {
            wi32(this.add(SOE_STATUS), -1);
            wu32(this.add(SOE_ERRID), 1);
        }
    }
    0
}
