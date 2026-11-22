//! tplc-rt builtin FB: xml srv.
use crate::*;

// ---- FB_XmlSrv builtin — tplc-xmlsrv bridge ----
// Member offsets (see builtin_fb_layout("FB_XmlSrv")).
pub(crate) const XS_OP: usize = 0;
pub(crate) const XS_VALLEN: usize = 4;
pub(crate) const XS_STATUS: usize = 8;
pub(crate) const XS_ERRID: usize = 12;
pub(crate) const XS_PATH_LEN: usize = 16;
pub(crate) const XS_PATH: usize = 17;
pub(crate) const XS_XPATH_LEN: usize = 144;
pub(crate) const XS_XPATH: usize = 145;
pub(crate) const XS_VAL_LEN: usize = 272;
pub(crate) const XS_VAL: usize = 273;

// TplcXmlsrvService vtable: size@0 abi@4 host@8
pub(crate) const XS_SVC_READ: usize = 16;
pub(crate) const XS_SVC_WRITE: usize = 24;

pub(crate) unsafe fn xmlsrv_service(ctx: *const u8) -> *const u8 {
    let svcs = (ctx.add(CTX_SVC) as *const *const u8).read();
    if svcs.is_null() {
        return core::ptr::null();
    }
    let host = (svcs.add(SVC_HOST) as *const *mut u8).read();
    let gs = core::mem::transmute::<usize, unsafe extern "C" fn(*mut u8, *const u8, u32) -> *mut u8>(
        (svcs.add(SVC_GET_SERVICE) as *const usize).read(),
    );
    gs(host, c"tplc-xmlsrv".as_ptr() as *const u8, 11) as *const u8
}

pub(crate) type FXsRead = unsafe extern "C" fn(*const u8, *const u8, *const u8, *mut u8, u32, *mut u32) -> i32;
pub(crate) type FXsWrite = unsafe extern "C" fn(*const u8, *const u8, *const u8, *const u8, u32) -> i32;

/// `__tplc_rt_FB_XmlSrv` — synchronous op dispatch over the
/// tplc-xmlsrv file-backed XML store.  OP: 1=read
/// (PATH/XPATH -> VAL), 2=write (PATH/XPATH/VAL).
/// # Safety
/// `this` must point at a valid instance block of the
/// documented layout; `ctx` at a live `TplcCtx`.
#[no_mangle]
pub unsafe extern "C" fn __tplc_rt_FB_XmlSrv(this: *mut u8, ctx: *mut u8) -> i32 {
    let op = ru32(this.add(XS_OP));
    wi32(this.add(XS_STATUS), 0);
    wu32(this.add(XS_ERRID), 0);
    let svc = xmlsrv_service(ctx);
    if svc.is_null() {
        wi32(this.add(XS_STATUS), -2);
        wu32(this.add(XS_ERRID), 99);
        return 0;
    }
    let mut path = [0u8; 128];
    let mut xp = [0u8; 128];
    uacstr(this, XS_PATH_LEN, XS_PATH, 127, path.as_mut_ptr());
    uacstr(this, XS_XPATH_LEN, XS_XPATH, 127, xp.as_mut_ptr());
    let rc: i32 = match op {
        1 => {
            let mut out = [0u8; 256];
            let mut n: u32 = 0;
            let f: FXsRead = core::mem::transmute(mbusfn(svc, XS_SVC_READ));
            let rc = f(svc, path.as_ptr(), xp.as_ptr(), out.as_mut_ptr(),
                       255, &mut n);
            if rc == 0 {
                core::ptr::copy_nonoverlapping(out.as_ptr(), this.add(XS_VAL), n as usize);
                *this.add(XS_VAL_LEN) = n as u8;
                wu32(this.add(XS_VALLEN), n);
            }
            rc
        }
        2 => {
            let mut val = [0u8; 256];
            let n = uacstr(this, XS_VAL_LEN, XS_VAL, 255, val.as_mut_ptr());
            let f: FXsWrite = core::mem::transmute(mbusfn(svc, XS_SVC_WRITE));
            f(svc, path.as_ptr(), xp.as_ptr(), val.as_ptr(), n)
        }
        _ => -1,
    };
    wi32(this.add(XS_STATUS), rc);
    wu32(this.add(XS_ERRID), (-rc).max(0) as u32);
    0
}
