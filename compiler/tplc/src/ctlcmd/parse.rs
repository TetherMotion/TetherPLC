//! parse — split from ctlcmd.rs.
use super::*;

/* ---------------- arg helpers ---------------- */

pub(crate) fn parse_u64(s: &str) -> u64 {
    // 0x… → hex (strtoul base-0); otherwise decimal
    if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(h, 16).unwrap_or(0)
    } else {
        s.parse().unwrap_or(0)
    }
}
pub(crate) fn parse_u32(s: &str) -> u32 {
    parse_u64(s) as u32
}

/// 32-hex → raw 16B uuid; otherwise u16 len + bytes (path|uuid form).
pub(crate) fn uuid_or_name(s: &str) -> Result<Vec<u8>, String> {
    if s.len() == 32 {
        let mut u = [0u8; 16];
        let mut ok = true;
        for i in 0..16 {
            match u8::from_str_radix(&s[i * 2..i * 2 + 2], 16) {
                Ok(v) => u[i] = v,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            return Ok(u.to_vec());
        }
        return Err("bad uuid".into());
    }
    let mut w = Wr::new();
    w.u16(s.len() as u16).bytes(s.as_bytes());
    Ok(w.b)
}

/// shadow selector tail {u16 slen, sel}: 16B uuid or name bytes.
pub(crate) fn append_sel(pre: &mut Vec<u8>, sel: Option<&str>) {
    let Some(sel) = sel else { return };
    if sel.is_empty() {
        return;
    }
    let b = if sel.len() == 32 {
        let mut u = vec![0u8; 16];
        let mut ok = true;
        for i in 0..16 {
            match u8::from_str_radix(&sel[i * 2..i * 2 + 2], 16) {
                Ok(v) => u[i] = v,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            u
        } else {
            sel.as_bytes().to_vec()
        }
    } else {
        sel.as_bytes().to_vec()
    };
    if b.len() > 256 {
        return;
    }
    let mut w = Wr::new();
    w.u16(b.len() as u16).bytes(&b);
    pre.extend_from_slice(&w.b);
}

pub(crate) fn parse_hex(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err("bad hex".into());
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let b = s.as_bytes();
    for i in 0..s.len() / 2 {
        let hi = (b[i * 2] as char).to_digit(16);
        let lo = (b[i * 2 + 1] as char).to_digit(16);
        match (hi, lo) {
            (Some(h), Some(l)) => out.push(((h << 4) | l) as u8),
            _ => return Err("bad hex".into()),
        }
    }
    Ok(out)
}

pub(crate) fn cstr_name(b: &[u8], max: usize) -> String {
    let n = b.iter().position(|&c| c == 0).unwrap_or(b.len().min(max));
    String::from_utf8_lossy(&b[..n.min(max)]).into_owned()
}

/* ---------------- reply printers (port of tplc_ctl.c) ------------- */
