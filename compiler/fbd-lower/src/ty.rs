//! IEC type-name, literal, and AT-address parsers for `fbd.v1`
//! string fields (`IfaceVar.ty`, `Block.params`, `Block.at`).

use std::collections::BTreeSet;
use tplc_mir::ir::{At, AtRegion, Lit, Type};

/// `INT`, `ARRAY[0..9] OF BOOL`, `TON` → `Type`. `fb_names` holds
/// user-declared FB POU names; builtin names resolve without listing.
pub fn parse_type(s: &str, fb_names: &BTreeSet<String>) -> Result<Type, String> {
    let t = s.trim();
    let up = t.to_ascii_uppercase();
    if let Some(inner) = up.strip_prefix("ARRAY") {
        let inner = inner.trim_start();
        let Some(bopen) = inner.find('[') else {
            return Err(format!("malformed ARRAY type {t}"));
        };
        let bclose = inner
            .find(']')
            .ok_or_else(|| format!("malformed ARRAY type {t}"))?;
        let bounds = &inner[bopen + 1..bclose];
        let (lo, hi) = bounds
            .split_once("..")
            .ok_or_else(|| format!("ARRAY bounds need lo..hi in {t}"))?;
        let lo: i64 = lo.trim().parse().map_err(|_| format!("bad bound {lo}"))?;
        let hi: i64 = hi.trim().parse().map_err(|_| format!("bad bound {hi}"))?;
        let rest = inner[bclose + 1..].trim();
        let elem = rest
            .strip_prefix("OF")
            .or_else(|| rest.strip_prefix("of"))
            .ok_or_else(|| format!("ARRAY needs OF in {t}"))?;
        return Ok(Type::Array {
            elem: Box::new(parse_type(elem.trim(), fb_names)?),
            lo,
            hi,
        });
    }
    if let Some(cap) = up
        .strip_prefix("STRING")
        .or_else(|| up.strip_prefix("WSTRING"))
        .and_then(|r| parse_cap(r.trim()))
    {
        return Ok(Type::Str { cap });
    }
    match up.as_str() {
        "BOOL" => Ok(Type::Bool),
        "SINT" => Ok(Type::Int {
            signed: true,
            bits: 8,
        }),
        "INT" => Ok(Type::Int {
            signed: true,
            bits: 16,
        }),
        "DINT" => Ok(Type::Int {
            signed: true,
            bits: 32,
        }),
        "LINT" => Ok(Type::Int {
            signed: true,
            bits: 64,
        }),
        "USINT" | "BYTE" => Ok(Type::Int {
            signed: false,
            bits: 8,
        }),
        "UINT" | "WORD" => Ok(Type::Int {
            signed: false,
            bits: 16,
        }),
        "UDINT" | "DWORD" => Ok(Type::Int {
            signed: false,
            bits: 32,
        }),
        "ULINT" | "LWORD" => Ok(Type::Int {
            signed: false,
            bits: 64,
        }),
        "REAL" => Ok(Type::Real { bits: 32 }),
        "LREAL" => Ok(Type::Real { bits: 64 }),
        "TIME" | "DATE" | "TOD" | "TIME_OF_DAY" | "DT" | "DATE_AND_TIME" => Ok(Type::Time),
        _ => {
            let name = t.to_string();
            let is_fb = fb_names.contains(&name.to_ascii_uppercase())
                || tplc_mir::layout::builtin_fb_layout(&name.to_ascii_uppercase()).is_some();
            if is_fb {
                Ok(Type::Fb { name })
            } else {
                Err(format!("unknown type {t}"))
            }
        }
    }
}

/// `STRING[80]` / `STRING(80)` / bare `STRING` (default 80).
fn parse_cap(r: &str) -> Option<u16> {
    if r.is_empty() {
        return Some(80);
    }
    let inner = r
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .or_else(|| r.strip_prefix('(').and_then(|s| s.strip_suffix(')')))?;
    inner.trim().parse().ok()
}

/// `%IX0.0` / `%QB3` / `%MD16` → `At`. Width codes X B W D L.
pub fn parse_at(s: &str) -> Option<At> {
    let s = s.trim();
    let s = s.strip_prefix('%')?;
    let region = match s.chars().next()? {
        'I' => AtRegion::I,
        'Q' => AtRegion::Q,
        'M' => AtRegion::M,
        _ => return None,
    };
    let s = &s[1..];
    let width = s.chars().next()?;
    if !matches!(width, 'X' | 'B' | 'W' | 'D' | 'L') {
        return None;
    }
    let s = &s[1..];
    let (byte, bit) = match s.split_once('.') {
        Some((b, bit)) => (b.parse().ok()?, Some(bit.parse().ok()?)),
        None => (s.parse().ok()?, None),
    };
    Some(At {
        region,
        byte,
        bit,
        width,
    })
}

/// Declared type implied by an AT width code (for located vars with no
/// declared type): X→BOOL, B→BYTE, W→WORD, D→DWORD, L→LWORD.
pub fn ty_for_width(width: char) -> Type {
    match width {
        'X' => Type::Bool,
        'B' => Type::Int {
            signed: false,
            bits: 8,
        },
        'W' => Type::Int {
            signed: false,
            bits: 16,
        },
        'D' => Type::Int {
            signed: false,
            bits: 32,
        },
        _ => Type::Int {
            signed: false,
            bits: 64,
        },
    }
}

/// `T#200ms`, `42`, `3.14`, `TRUE`, `'text'`, `16#FF`, `DINT#5` → `Lit`.
/// Typed prefixes (`INT#`) are annotations — the payload parses as the
/// base form, sema coerces to the declared type.
pub fn parse_lit(s: &str) -> Result<Lit, String> {
    let t = s.trim();
    if t.is_empty() {
        return Err("empty literal".into());
    }
    let (neg, t) = match t.strip_prefix('-') {
        Some(r) => (true, r.trim()),
        None => (false, t),
    };
    if let Some((pre, rest)) = t.split_once('#') {
        let pu = pre.to_ascii_uppercase();
        if pu == "T" || pu == "TIME" {
            let ns = parse_duration(rest)?;
            return Ok(Lit::TimeNs(if neg { -ns } else { ns }));
        }
        match pu.as_str() {
            "2" | "8" | "16" => {
                let radix = pu.parse::<u32>().unwrap();
                let clean: String = rest.chars().filter(|c| *c != '_').collect();
                let v = u64::from_str_radix(&clean, radix)
                    .map_err(|_| format!("bad based literal {t}"))?;
                return Ok(Lit::UInt(v));
            }
            _ => {
                // typed literal prefix: `INT#5`, `DINT#-2` — parse payload
                return parse_lit(rest).map(|l| match (l, neg) {
                    (Lit::Int(v), true) => Lit::Int(-v),
                    (Lit::UInt(v), true) => Lit::Int(-(v as i64)),
                    (other, _) => other,
                });
            }
        }
    }
    if t.eq_ignore_ascii_case("TRUE") {
        return Ok(Lit::Bool(true));
    }
    if t.eq_ignore_ascii_case("FALSE") {
        return Ok(Lit::Bool(false));
    }
    if let Some(inner) = t.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')) {
        return Ok(Lit::Str(inner.replace("''", "'")));
    }
    let clean: String = t.chars().filter(|c| *c != '_').collect();
    if clean.contains('.') || clean.contains('e') || clean.contains('E') {
        let v: f64 = clean.parse().map_err(|_| format!("bad real literal {t}"))?;
        return Ok(Lit::Real(if neg { -v } else { v }));
    }
    let v: i64 = clean.parse().map_err(|_| format!("bad literal {t}"))?;
    Ok(Lit::Int(if neg { -v } else { v }))
}

/// `200ms`, `1s`, `1.5s`, `1h30m`, `2m` → nanoseconds.
fn parse_duration(s: &str) -> Result<i64, String> {
    const UNITS: &[(&str, i64)] = &[
        ("ns", 1),
        ("us", 1_000),
        ("ms", 1_000_000),
        ("s", 1_000_000_000),
        ("m", 60_000_000_000),
        ("h", 3_600_000_000_000),
        ("d", 86_400_000_000_000),
    ];
    let mut rest = s;
    let mut total = 0i64;
    while !rest.is_empty() {
        let nend = rest
            .find(|c: char| c.is_ascii_alphabetic())
            .ok_or_else(|| format!("bad duration {s}"))?;
        let num: f64 = rest[..nend]
            .parse()
            .map_err(|_| format!("bad duration {s}"))?;
        rest = &rest[nend..];
        let unit = UNITS
            .iter()
            .filter(|(u, _)| rest.starts_with(u))
            .max_by_key(|(u, _)| u.len())
            .ok_or_else(|| format!("bad duration unit in {s}"))?;
        rest = &rest[unit.0.len()..];
        total += (num * unit.1 as f64) as i64;
    }
    Ok(total)
}
