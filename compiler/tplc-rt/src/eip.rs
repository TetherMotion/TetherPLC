//! tplc-rt builtin FB: eip.
use crate::*;

// ---------------------------------------------------------------------------
// FB_Eip — EtherNet/IP explicit-messaging client over tplc-eip.
// ---------------------------------------------------------------------------

// FB_Eip member offsets (see builtin_fb_layout("FB_Eip")). Strings are
// len-prefixed: HOST u8len@8+data@9 (cap63), DATA u8len@92+data@93
// (cap255).
pub(crate) const EIP_OP: usize = 0;
pub(crate) const EIP_HANDLE: usize = 4;
pub(crate) const EIP_HOST_LEN: usize = 8;
pub(crate) const EIP_HOST: usize = 9;
pub(crate) const EIP_PORT: usize = 72;
pub(crate) const EIP_CLS: usize = 76;
pub(crate) const EIP_INST: usize = 80;
pub(crate) const EIP_ATTR: usize = 84;
pub(crate) const EIP_DATALEN: usize = 88;
pub(crate) const EIP_DATA_LEN: usize = 92;
pub(crate) const EIP_DATA: usize = 93;
pub(crate) const EIP_STATE: usize = 348;
pub(crate) const EIP_SESSION: usize = 352;
pub(crate) const EIP_LASTCIP: usize = 356;
pub(crate) const EIP_STATUS: usize = 360;
pub(crate) const EIP_ERRID: usize = 364;

// tplc-eip service vtable offsets (TplcEipService):
// struct_size@0 u32, abi@4 u32, host@8 ptr(8B), then fn ptrs.
pub(crate) const EIP_SVC_OPEN: usize = 16;
pub(crate) const EIP_SVC_CLOSE: usize = 24;
pub(crate) const EIP_SVC_STATUS: usize = 32;
pub(crate) const EIP_SVC_LISTID: usize = 40;
pub(crate) const EIP_SVC_GET_ATTR: usize = 48;
pub(crate) const EIP_SVC_SET_ATTR: usize = 56;

/// service lookup for "tplc-eip" (8 chars)
pub(crate) unsafe fn eip_service(ctx: *const u8) -> *const u8 {
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
pub(crate) unsafe fn eipfn(svc: *const u8, off: usize) -> usize {
    (svc.add(off) as *const usize).read()
}

pub(crate) type FEipOpen = unsafe extern "C" fn(*const u8, *const u8, u32, *mut u32) -> i32;
pub(crate) type FEipClose = unsafe extern "C" fn(*const u8, u32) -> i32;
pub(crate) type FEipStatus = unsafe extern "C" fn(*const u8, u32, *mut u8) -> i32;
pub(crate) type FEipListId = unsafe extern "C" fn(*const u8, *const u8, u32, *mut u8, u32) -> i32;
pub(crate) type FEipGetAttr = unsafe extern "C" fn(*const u8, u32, u32, u32, u32, *mut u8, u32) -> i32;
pub(crate) type FEipSetAttr = unsafe extern "C" fn(*const u8, u32, u32, u32, u32, *const u8, u32) -> i32;

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
