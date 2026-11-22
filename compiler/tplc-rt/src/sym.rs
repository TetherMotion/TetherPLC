//! tplc-rt builtin FB: sym.
use crate::*;

// ---- FB_Sym builtin — tplc-sym by-name variable access ----
// Member offsets (see builtin_fb_layout("FB_Sym")).
pub(crate) const SY_OP: usize = 0;
pub(crate) const SY_TYPE: usize = 4;
pub(crate) const SY_SIZE: usize = 8;
pub(crate) const SY_VALLEN: usize = 12;
pub(crate) const SY_FOUND: usize = 16;
pub(crate) const SY_STATUS: usize = 20;
pub(crate) const SY_ERRID: usize = 24;
pub(crate) const SY_IB: usize = 28;
pub(crate) const SY_FB: usize = 32;
pub(crate) const SY_BB: usize = 40;
pub(crate) const SY_NAME_LEN: usize = 42;   /* Str member offset = len byte */
pub(crate) const SY_NAME: usize = 43;
pub(crate) const SY_VAL_LEN: usize = 106;
pub(crate) const SY_VAL: usize = 107;

// TplcSymService vtable: size@0 abi@4 host@8
pub(crate) const SY_SVC_FIND: usize = 32;
pub(crate) const SY_SVC_READ: usize = 40;
pub(crate) const SY_SVC_WRITE: usize = 48;

pub(crate) unsafe fn sym_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-sym".as_ptr() as *const u8, 8) as *const u8
}

pub(crate) type FSyFind = unsafe extern "C" fn(*const u8, *const u8, *mut u8) -> i32;
pub(crate) type FSyRead = unsafe extern "C" fn(*const u8, *const u8, *mut u8, u32, *mut u32) -> i32;
pub(crate) type FSyWrite = unsafe extern "C" fn(*const u8, *const u8, *const u8, u32) -> i32;

/// Read a variable's raw bytes into `out`, returning (rc, len, type, size).
pub(crate) unsafe fn sym_fetch(
    svc: *const u8,
    name: *const u8,
    out: &mut [u8; 256],
) -> (i32, u32, u32, u32) {
    let f: FSyRead = core::mem::transmute(mbusfn(svc, SY_SVC_READ));
    let mut n: u32 = 0;
    let rc = f(svc, name, out.as_mut_ptr(), 255, &mut n);
    let mut ty: u32 = 0;
    let mut sz: u32 = 0;
    let ff: FSyFind = core::mem::transmute(mbusfn(svc, SY_SVC_FIND));
    let mut desc = [0u8; 48];
    if ff(svc, name, desc.as_mut_ptr()) == 0 {
        ty = desc[40] as u32;
        sz = u32::from_le_bytes([desc[36], desc[37], desc[38], desc[39]]);
    }
    (rc, n, ty, sz)
}

/// Write `len` raw bytes into a variable by name.
pub(crate) unsafe fn sym_store(svc: *const u8, name: *const u8, bytes: *const u8,
                    len: usize) -> i32 {
    let f: FSyWrite = core::mem::transmute(mbusfn(svc, SY_SVC_WRITE));
    f(svc, name, bytes, len as u32)
}

/// Encode an integer value for a variable of `size` bytes (LE).
pub(crate) fn int_bytes(v: i64, size: u32, out: *mut u8) -> usize {
    let n = match size {
        1 | 2 | 4 | 8 => size as usize,
        _ => 4,
    };
    let mut i = 0;
    while i < n {
        unsafe { *out.add(i) = ((v >> (8 * i)) & 0xff) as u8; }
        i += 1;
    }
    n
}

/// `__tplc_rt_FB_Sym` — by-name variable access through the
/// tplc-sym service.  OP: 1=read bytes, 2=write bytes,
/// 3=write int, 4=write real, 5=write bool, 6=read int,
/// 7=read real, 8=read bool, 9=find.
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_Sym(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(SY_OP));
    wi32(this.add(SY_STATUS), 0);
    wu32(this.add(SY_ERRID), 0);
    *this.add(SY_FOUND) = 0;
    let svc = sym_service(ctx);
    if svc.is_null() {
        wi32(this.add(SY_STATUS), -2);
        wu32(this.add(SY_ERRID), 99);
        return 0;
    }
    let mut name = [0u8; 64];
    uacstr(this, SY_NAME_LEN, SY_NAME, 63, name.as_mut_ptr());
    let mut raw = [0u8; 256];
    let rc: i32 = match op {
        1 => {
            let (rc, n, ty, sz) = sym_fetch(svc, name.as_ptr(), &mut raw);
            if rc == 0 {
                core::ptr::copy_nonoverlapping(raw.as_ptr(), this.add(SY_VAL), n as usize);
                *this.add(SY_VAL_LEN) = n as u8;
                wu32(this.add(SY_VALLEN), n);
                wu32(this.add(SY_TYPE), ty);
                wu32(this.add(SY_SIZE), sz);
                *this.add(SY_FOUND) = 1;
            }
            rc
        }
        2 => {
            let mut val = [0u8; 256];
            let n = uacstr(this, SY_VAL_LEN, SY_VAL, 255, val.as_mut_ptr()) as usize;
            sym_store(svc, name.as_ptr(), val.as_ptr(), n)
        }
        3 => {
            let mut b = [0u8; 8];
            let mut sz: u32 = 4;
            let ff: FSyFind = core::mem::transmute(mbusfn(svc, SY_SVC_FIND));
            let mut desc = [0u8; 48];
            if ff(svc, name.as_ptr(), desc.as_mut_ptr()) == 0 {
                sz = u32::from_le_bytes([desc[36], desc[37], desc[38], desc[39]]);
            }
            let n = int_bytes(ri32(this.add(SY_IB)) as i64, sz, b.as_mut_ptr());
            sym_store(svc, name.as_ptr(), b.as_ptr(), n)
        }
        4 => {
            let mut sz: u32 = 8;
            let mut ty: u32 = 10;
            let ff: FSyFind = core::mem::transmute(mbusfn(svc, SY_SVC_FIND));
            let mut desc = [0u8; 48];
            if ff(svc, name.as_ptr(), desc.as_mut_ptr()) == 0 {
                sz = u32::from_le_bytes([desc[36], desc[37], desc[38], desc[39]]);
                ty = desc[40] as u32;
            }
            let v = f64::from_bits(ru64(this.add(SY_FB)));
            let mut b = [0u8; 8];
            let n = if sz == 4 || ty == 9 {
                core::ptr::write_unaligned(
                    b.as_mut_ptr() as *mut u32, (v as f32).to_bits());
                4
            } else {
                core::ptr::write_unaligned(
                    b.as_mut_ptr() as *mut u64, v.to_bits());
                8
            };
            sym_store(svc, name.as_ptr(), b.as_ptr(), n)
        }
        5 => {
            let b = [*this.add(SY_BB)];
            sym_store(svc, name.as_ptr(), b.as_ptr(), 1)
        }
        6 => {
            let (rc, n, _ty, _sz) = sym_fetch(svc, name.as_ptr(), &mut raw);
            if rc == 0 {
                let p = raw.as_ptr();
                let v: i32 = if n == 1 {
                    core::ptr::read_unaligned(p) as i8 as i32
                } else if n == 2 {
                    core::ptr::read_unaligned(p as *const i16) as i32
                } else if n >= 4 {
                    core::ptr::read_unaligned(p as *const i32)
                } else {
                    0
                };
                wi32(this.add(SY_IB), v);
                *this.add(SY_FOUND) = 1;
            }
            rc
        }
        7 => {
            let (rc, n, ty, _sz) = sym_fetch(svc, name.as_ptr(), &mut raw);
            if rc == 0 {
                let p = raw.as_ptr();
                let v = if n == 4 || ty == 9 {
                    core::ptr::read_unaligned(p as *const f32) as f64
                } else if n >= 8 {
                    core::ptr::read_unaligned(p as *const f64)
                } else {
                    0.0
                };
                wu64(this.add(SY_FB), v.to_bits());
                *this.add(SY_FOUND) = 1;
            }
            rc
        }
        8 => {
            let (rc, n, _ty, _sz) = sym_fetch(svc, name.as_ptr(), &mut raw);
            if rc == 0 {
                let b = if n > 0 && *raw.as_ptr() != 0 { 1 } else { 0 };
                *this.add(SY_BB) = b;
                *this.add(SY_FOUND) = 1;
            }
            rc
        }
        9 => {
            let ff: FSyFind = core::mem::transmute(mbusfn(svc, SY_SVC_FIND));
            let mut desc = [0u8; 48];
            let rc = ff(svc, name.as_ptr(), desc.as_mut_ptr());
            if rc == 0 {
                *this.add(SY_FOUND) = 1;
                wu32(this.add(SY_TYPE), desc[40] as u32);
                wu32(this.add(SY_SIZE), u32::from_le_bytes([
                    desc[36], desc[37], desc[38], desc[39],
                ]));
            }
            rc
        }
        _ => -1,
    };
    wi32(this.add(SY_STATUS), rc);
    wu32(this.add(SY_ERRID), (-rc).max(0) as u32);
    0
}
