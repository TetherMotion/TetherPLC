//! IEC operand names ↔ positional pin indices for operator blocks.
//!
//! TC6 `formalParameter` carries the operand *name*, not a position.
//! Beremiz/IEC conventions differ per operator family: arithmetic and
//! compare ops are 1-based (`IN1`, `IN2`, …), `SEL` is `G` plus
//! 0-based `IN0`/`IN1`, `MUX` is `K` plus `IN0`…, `LIMIT` is
//! `MN`/`IN`/`MX`, unary ops and casts take `IN`. The model's
//! `PortSel::Index` is positional over *all* in-pins, so this module
//! translates at the XML boundary — the model never sees operand
//! names.

/// Operator family for pin-name purposes (mirrors fbd-lower's `Op`
/// classification — kept string-based so this crate stays independent).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Fam {
    /// AND OR XOR ADD MUL SUB DIV MOD GT GE LT LE EQ NE SHL SHR ROL ROR MIN MAX
    Arith,
    Sel,
    Mux,
    Limit,
    /// NOT MOVE ABS and `X_TO_Y` casts — single `IN` pin
    Unary,
}

fn family(type_ref: &str) -> Option<Fam> {
    Some(match type_ref.to_ascii_uppercase().as_str() {
        "AND" | "OR" | "XOR" | "ADD" | "MUL" | "SUB" | "DIV" | "MOD" | "GT" | "GE" | "LT"
        | "LE" | "EQ" | "NE" | "SHL" | "SHR" | "ROL" | "ROR" | "MIN" | "MAX" => Fam::Arith,
        "SEL" => Fam::Sel,
        "MUX" => Fam::Mux,
        "LIMIT" => Fam::Limit,
        "NOT" | "MOVE" | "ABS" => Fam::Unary,
        t if t.contains("_TO_") => Fam::Unary,
        _ => return None,
    })
}

/// `formalParameter` name → positional in-index for an operator dst.
/// `None` = not a recognized operand name (caller falls back to a
/// name port, which the lowerer reports as unconnected).
pub fn dst_index(type_ref: &str, formal: &str) -> Option<u32> {
    let n = formal.to_ascii_uppercase();
    match family(type_ref)? {
        Fam::Arith => {
            // IEC arith operands are 1-based; tolerate a stray IN0
            // (our older positional exports) as pin 0.
            let k: u32 = n.strip_prefix("IN")?.parse().ok()?;
            Some(k.saturating_sub(1))
        }
        Fam::Sel => match n.as_str() {
            "G" => Some(0),
            _ => n.strip_prefix("IN")?.parse::<u32>().ok().map(|k| k + 1),
        },
        Fam::Mux => match n.as_str() {
            "K" => Some(0),
            _ => n.strip_prefix("IN")?.parse::<u32>().ok().map(|k| k + 1),
        },
        Fam::Limit => match n.as_str() {
            "MN" => Some(0),
            "IN" => Some(1),
            "MX" => Some(2),
            _ => None,
        },
        Fam::Unary => match n.as_str() {
            "IN" | "IN0" => Some(0),
            _ => None,
        },
    }
}

/// Positional in-index → IEC `formalParameter` name for export.
/// `None` for out-of-range pins or non-operator type_refs.
pub fn in_name(type_ref: &str, index: u32) -> Option<String> {
    Some(match family(type_ref)? {
        Fam::Arith => format!("IN{}", index + 1),
        Fam::Sel => {
            if index == 0 {
                "G".into()
            } else {
                format!("IN{}", index - 1)
            }
        }
        Fam::Mux => {
            if index == 0 {
                "K".into()
            } else {
                format!("IN{}", index - 1)
            }
        }
        Fam::Limit => ["MN", "IN", "MX"].get(index as usize)?.to_string(),
        Fam::Unary => {
            if index == 0 {
                "IN".into()
            } else {
                return None;
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beremiz_operand_names() {
        // CounterFBD: ADD IN1/IN2, SEL G/IN0/IN1
        assert_eq!(dst_index("ADD", "IN1"), Some(0));
        assert_eq!(dst_index("ADD", "IN2"), Some(1));
        assert_eq!(dst_index("SEL", "G"), Some(0));
        assert_eq!(dst_index("SEL", "IN0"), Some(1));
        assert_eq!(dst_index("SEL", "IN1"), Some(2));
        assert_eq!(dst_index("MUX", "K"), Some(0));
        assert_eq!(dst_index("MUX", "IN0"), Some(1));
        assert_eq!(dst_index("LIMIT", "MN"), Some(0));
        assert_eq!(dst_index("LIMIT", "MX"), Some(2));
        assert_eq!(dst_index("NOT", "IN"), Some(0));
        assert_eq!(dst_index("REAL_TO_DINT", "IN"), Some(0));
        assert_eq!(dst_index("ADD", "bogus"), None);
        assert_eq!(dst_index("TON", "IN"), None);
    }

    #[test]
    fn export_names_roundtrip() {
        for (tr, pins) in [
            ("ADD", vec!["IN1", "IN2"]),
            ("SEL", vec!["G", "IN0", "IN1"]),
            ("MUX", vec!["K", "IN0", "IN1"]),
            ("LIMIT", vec!["MN", "IN", "MX"]),
            ("NOT", vec!["IN"]),
            ("INT_TO_REAL", vec!["IN"]),
        ] {
            for (i, want) in pins.iter().enumerate() {
                assert_eq!(in_name(tr, i as u32).as_deref(), Some(*want), "{tr}[{i}]");
                assert_eq!(dst_index(tr, want), Some(i as u32), "{tr} {want}");
            }
        }
    }
}
