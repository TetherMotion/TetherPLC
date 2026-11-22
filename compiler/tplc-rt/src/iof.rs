//! tplc-rt builtin FB: iof.
use crate::*;

// ---- FB_Iof builtin — I/O device/box enumeration ----
// Member offsets (see builtin_fb_layout("FB_Iof")).
pub(crate) const IO_OP: usize = 0;
pub(crate) const IO_IDX: usize = 4;
pub(crate) const IO_COUNT: usize = 8;
pub(crate) const IO_REGION: usize = 12;
pub(crate) const IO_ADDR: usize = 16;
pub(crate) const IO_UNIT: usize = 20;
pub(crate) const IO_FOUND: usize = 24;
pub(crate) const IO_STATUS: usize = 28;
pub(crate) const IO_ERRID: usize = 32;
pub(crate) const IO_NAME_LEN: usize = 36;
pub(crate) const IO_NAME: usize = 37;
pub(crate) const IO_VAL_LEN: usize = 100;
pub(crate) const IO_VAL: usize = 101;

// TplcSymService vtable: count@16 at@24 find@32
pub(crate) const IOF_SVC_COUNT: usize = 16;
pub(crate) const IOF_SVC_AT: usize = 24;
pub(crate) const IOF_SVC_FIND: usize = 32;

pub(crate) type FIofCount = unsafe extern "C" fn(*const u8) -> u32;
pub(crate) type FIofAt = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;

/// byte `k` of a TplcVarDesc buffer
#[inline]
pub(crate) unsafe fn vd8(d: *const u8, k: usize) -> u8 {
    *d.add(k)
}

/// u32 field at byte `k` of a TplcVarDesc buffer
#[inline]
pub(crate) unsafe fn vd32(d: *const u8, k: usize) -> u32 {
    u32::from_le_bytes([
        *d.add(k),
        *d.add(k + 1),
        *d.add(k + 2),
        *d.add(k + 3),
    ])
}

/// `__tplc_rt_FB_Iof` — I/O device/box enumeration over the
/// package variable table (Tc2_IoFunctions general FBs).
/// OP: 1=box count, 2=box by index, 3=find by name,
/// 4=device count, 5=device name.
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Iof(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(IO_OP));
    wi32(this.add(IO_STATUS), 0);
    wu32(this.add(IO_ERRID), 0);
    *this.add(IO_FOUND) = 0;
    let svc = sym_service(ctx);
    if svc.is_null() {
        wi32(this.add(IO_STATUS), -2);
        wu32(this.add(IO_ERRID), 99);
        return 0;
    }
    let fcount: FIofCount = core::mem::transmute(mbusfn(svc, IOF_SVC_COUNT));
    let fat: FIofAt = core::mem::transmute(mbusfn(svc, IOF_SVC_AT));
    let rc: i32 = match op {
        1 | 4 => {
            let total = fcount(svc);
            let mut boxes: u32 = 0;
            let mut i: u32 = 0;
            let mut d = [0u8; 48];
            while i < total {
                if fat(svc, i, d.as_mut_ptr()) == 0 && vd8(d.as_ptr(), 41) != 0 {
                    boxes += 1;
                }
                i += 1;
            }
            wu32(this.add(IO_COUNT), if op == 1 { boxes } else if boxes > 0 { 1 } else { 0 });
            0
        }
        2 => {
            let total = fcount(svc);
            let want = ru32(this.add(IO_IDX));
            let mut seen: u32 = 0;
            let mut i: u32 = 0;
            let mut d = [0u8; 48];
            while i < total {
                if fat(svc, i, d.as_mut_ptr()) == 0 && vd8(d.as_ptr(), 41) != 0 {
                    if seen == want {
                        let p = d.as_ptr();
                        let mut n: usize = 0;
                        while n < 63 && vd8(p, n) != 0 {
                            n += 1;
                        }
                        core::ptr::copy_nonoverlapping(p, this.add(IO_NAME), n);
                        *this.add(IO_NAME_LEN) = n as u8;
                        wu32(this.add(IO_REGION), vd8(p, 42) as u32);
                        wu32(this.add(IO_UNIT), vd8(p, 43) as u32);
                        wu32(this.add(IO_ADDR), vd32(p, 44));
                        *this.add(IO_FOUND) = 1;
                        break;
                    }
                    seen += 1;
                }
                i += 1;
            }
            0
        }
        3 => {
            let mut name = [0u8; 64];
            uacstr(this, IO_NAME_LEN, IO_NAME, 63, name.as_mut_ptr());
            let mut d = [0u8; 48];
            let ff: FSyFind = core::mem::transmute(mbusfn(svc, IOF_SVC_FIND));
            let rc = ff(svc, name.as_ptr(), d.as_mut_ptr());
            if rc == 0 {
                let p = d.as_ptr();
                *this.add(IO_FOUND) = 1;
                wu32(this.add(IO_REGION), vd8(p, 42) as u32);
                wu32(this.add(IO_UNIT), vd8(p, 43) as u32);
                wu32(this.add(IO_ADDR), vd32(p, 44));
            }
            rc
        }
        5 => {
            let nm = b"Local";
            core::ptr::copy_nonoverlapping(nm.as_ptr(), this.add(IO_VAL), nm.len());
            *this.add(IO_VAL_LEN) = nm.len() as u8;
            0
        }
        _ => -1,
    };
    wi32(this.add(IO_STATUS), rc);
    wu32(this.add(IO_ERRID), (-rc).max(0) as u32);
    0
}
