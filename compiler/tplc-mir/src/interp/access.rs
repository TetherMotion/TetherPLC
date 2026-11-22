//! access — split from interp.rs.
use super::*;

impl<'a> Rt<'a> {
    pub fn new(blob: &'a mut [u8], layout: &'a Layout, module: &'a Module, now_ns: u64) -> Self {
        Rt {
            blob,
            layout,
            module,
            now_ns,
            cycle: 0,
            abort_flag: None,
            scratch: Vec::new(),
            fault: None,
        }
    }

    pub(crate) fn region(&self, off: u64) -> (&[u8], u64) {
        if off >= SCRATCH {
            (&self.scratch, off - SCRATCH)
        } else {
            (self.blob, off)
        }
    }

    pub(crate) fn region_mut(&mut self, off: u64) -> (&mut [u8], u64) {
        if off >= SCRATCH {
            (&mut self.scratch, off - SCRATCH)
        } else {
            (self.blob, off)
        }
    }

    pub(crate) fn rd(&self, off: u64, ty: &Type) -> Value {
        let (mem, o) = self.region(off);
        let b = &mem[o as usize..];
        match ty {
            Type::Bool => Value::Bool(b[0] != 0),
            Type::Int { signed: true, bits } => Value::I(match bits {
                8 => i8::from_le_bytes(b[..1].try_into().unwrap()) as i64,
                16 => i16::from_le_bytes(b[..2].try_into().unwrap()) as i64,
                32 => i32::from_le_bytes(b[..4].try_into().unwrap()) as i64,
                _ => i64::from_le_bytes(b[..8].try_into().unwrap()),
            }),
            Type::Int {
                signed: false,
                bits,
            } => Value::U(match bits {
                8 => u8::from_le_bytes(b[..1].try_into().unwrap()) as u64,
                16 => u16::from_le_bytes(b[..2].try_into().unwrap()) as u64,
                32 => u32::from_le_bytes(b[..4].try_into().unwrap()) as u64,
                _ => u64::from_le_bytes(b[..8].try_into().unwrap()),
            }),
            Type::Real { bits } => Value::F(if *bits == 32 {
                f32::from_le_bytes(b[..4].try_into().unwrap()) as f64
            } else {
                f64::from_le_bytes(b[..8].try_into().unwrap())
            }),
            Type::Time => Value::Time(i64::from_le_bytes(b[..8].try_into().unwrap())),
            Type::Str { cap } => {
                let len = (b[0] as usize).min(*cap as usize);
                Value::Str(b[1..1 + len].to_vec())
            }
            Type::Ref { .. } => Value::Ref(u64::from_le_bytes(b[..8].try_into().unwrap())),
            Type::WStr { cap } => {
                let len = (u16::from_le_bytes([b[0], b[1]]) as usize).min(*cap as usize);
                let mut v = Vec::with_capacity(len);
                for i in 0..len {
                    let o = 2 + i * 2;
                    v.push(u16::from_le_bytes([b[o], b[o + 1]]));
                }
                Value::WStr(v)
            }
            Type::Enum { .. } => Value::I(i32::from_le_bytes(b[..4].try_into().unwrap()) as i64),
            _ => Value::I(0), // aggregates accessed fieldwise
        }
    }

    pub(crate) fn wr(&mut self, off: u64, ty: &Type, v: &Value) {
        let (mem, o) = self.region_mut(off);
        let b = &mut mem[o as usize..];
        match (ty, v) {
            (Type::Bool, Value::Bool(x)) => b[0] = *x as u8,
            (Type::Int { signed: true, bits }, Value::I(x)) => match bits {
                8 => b[..1].copy_from_slice(&(*x as i8).to_le_bytes()),
                16 => b[..2].copy_from_slice(&(*x as i16).to_le_bytes()),
                32 => b[..4].copy_from_slice(&(*x as i32).to_le_bytes()),
                _ => b[..8].copy_from_slice(&x.to_le_bytes()),
            },
            (
                Type::Int {
                    signed: false,
                    bits,
                },
                Value::U(x),
            ) => match bits {
                8 => b[..1].copy_from_slice(&(*x as u8).to_le_bytes()),
                16 => b[..2].copy_from_slice(&(*x as u16).to_le_bytes()),
                32 => b[..4].copy_from_slice(&(*x as u32).to_le_bytes()),
                _ => b[..8].copy_from_slice(&x.to_le_bytes()),
            },
            (Type::Int { bits, .. }, Value::I(x)) => {
                // coercion convenience: signed writes into unsigned fields etc.
                let ux = *x as u64;
                match bits {
                    8 => b[..1].copy_from_slice(&(ux as u8).to_le_bytes()),
                    16 => b[..2].copy_from_slice(&(ux as u16).to_le_bytes()),
                    32 => b[..4].copy_from_slice(&(ux as u32).to_le_bytes()),
                    _ => b[..8].copy_from_slice(&ux.to_le_bytes()),
                }
            }
            (Type::Int { bits, .. }, Value::U(x)) => {
                let ix = *x as i64;
                match bits {
                    8 => b[..1].copy_from_slice(&(ix as i8).to_le_bytes()),
                    16 => b[..2].copy_from_slice(&(ix as i16).to_le_bytes()),
                    32 => b[..4].copy_from_slice(&(ix as i32).to_le_bytes()),
                    _ => b[..8].copy_from_slice(&ix.to_le_bytes()),
                }
            }
            (Type::Real { bits }, Value::F(x)) => {
                if *bits == 32 {
                    b[..4].copy_from_slice(&(*x as f32).to_le_bytes())
                } else {
                    b[..8].copy_from_slice(&x.to_le_bytes())
                }
            }
            (Type::Real { bits }, Value::I(x)) => {
                if *bits == 32 {
                    b[..4].copy_from_slice(&(*x as f32).to_le_bytes())
                } else {
                    b[..8].copy_from_slice(&(*x as f64).to_le_bytes())
                }
            }
            (Type::Time, Value::Time(x)) | (Type::Time, Value::I(x)) => {
                b[..8].copy_from_slice(&x.to_le_bytes())
            }
            (Type::Ref { .. }, Value::Ref(o)) => {
                b[..8].copy_from_slice(&o.to_le_bytes());
            }
            (Type::Str { cap }, Value::Str(s)) => {
                let n = (*cap as usize).min(s.len());
                b[0] = n as u8;
                b[1..1 + n].copy_from_slice(&s[..n]);
            }
            (Type::WStr { cap }, Value::WStr(s)) => {
                let n = (*cap as usize).min(s.len());
                b[..2].copy_from_slice(&(n as u16).to_le_bytes());
                for i in 0..n {
                    let o = 2 + i * 2;
                    b[o..o + 2].copy_from_slice(&s[i].to_le_bytes());
                }
                for i in n..*cap as usize {
                    let o = 2 + i * 2;
                    b[o..o + 2].copy_from_slice(&0u16.to_le_bytes());
                }
            }
            (Type::Enum { .. }, Value::I(x)) => b[..4].copy_from_slice(&(*x as i32).to_le_bytes()),
            _ => {}
        }
    }

    /// Apply declaration initializers (`x : DINT := 42`) to the blob —
    /// the compiled path runs these in `tplc_init`; the interpreter
    /// must do the same or L1 diverges from L2 on any initialized var.
    pub fn run_inits(&mut self, ctx: &Ctx) {
        for p in &self.module.pous {
            for v in &p.iface {
                let Some(init) = &v.init else { continue };
                let Some((off, ty)) = ctx.get(v.name.as_str()).cloned() else {
                    continue;
                };
                let val = self.eval(ctx, init);
                self.wr(off, &ty, &val);
            }
        }
    }

    /// Read a top-level field by qualified name (host-side probe).
    pub fn read_field(&self, qname: &str) -> Value {
        let f = self
            .layout
            .fields
            .get(qname)
            .unwrap_or_else(|| panic!("no field {qname}"));
        self.rd(f.offset, &f.ty)
    }

    pub fn write_field(&mut self, qname: &str, v: &Value) {
        let (off, ty) = {
            let f = self
                .layout
                .fields
                .get(qname)
                .unwrap_or_else(|| panic!("no field {qname}"));
            (f.offset, f.ty.clone())
        };
        self.wr(off, &ty, v);
    }

    // ---- ref resolution → (offset, type) ----
    pub(crate) fn member_of(&self, fbname: &str) -> FbLayout {
        self.layout
            .fb_types
            .get(fbname)
            .cloned()
            .or_else(|| builtin_fb_layout(fbname))
            .unwrap_or_else(|| panic!("no FB layout {fbname}"))
    }

    pub(crate) fn resolve(&mut self, ctx: &Ctx, r: &Ref) -> (u64, Type) {
        match r {
            Ref::Deref(inner) => {
                let (slot, ty) = self.resolve(ctx, inner);
                let to = match &ty {
                    Type::Ref { to } => (**to).clone(),
                    _ => Type::Bool,
                };
                let off = match self.rd(
                    slot,
                    &Type::Ref {
                        to: Box::new(to.clone()),
                    },
                ) {
                    Value::Ref(o) => o,
                    _ => u64::MAX,
                };
                if off == u64::MAX {
                    self.fault = Some(TPLC_TASK_E_PANIC);
                } else if off
                    .checked_add(to.size())
                    .map_or(true, |end| end > self.blob.len() as u64)
                {
                    self.fault = Some(TPLC_TASK_E_BOUNDS);
                }
                (if off == u64::MAX { 0 } else { off }, to)
            }
            Ref::Var(name) => ctx
                .get(name.as_str())
                .map(|(o, t)| (*o, t.clone()))
                .unwrap_or_else(|| panic!("unresolved var {name}")),
            Ref::Field(inner, name) => {
                let (off, ty) = self.resolve(ctx, inner);
                match ty {
                    Type::Fb { name: fbname } => {
                        let fbl = self.member_of(&fbname);
                        let (moff, mty) = fbl
                            .members
                            .get(name.as_str())
                            .unwrap_or_else(|| panic!("{fbname} has no member {name}"));
                        (off.wrapping_add(*moff), mty.clone())
                    }
                    Type::Struct { fields, .. } => {
                        let mut o = 0u64;
                        for (n, t) in &fields {
                            o = (o + t.align() - 1) & !(t.align() - 1);
                            if n == name {
                                return (off.wrapping_add(o), t.clone());
                            }
                            o += t.size();
                        }
                        panic!("no field {name} in struct")
                    }
                    _ => panic!("field access on non-aggregate"),
                }
            }
            Ref::Index(inner, idx_e) => {
                let (off, ty) = self.resolve(ctx, inner);
                if let Type::Array { elem, lo, .. } = ty {
                    let idx = self.eval(ctx, idx_e).as_i64() - lo;
                    // wrapping: matches release-mode / C unsigned wrap —
                    // debug builds must not panic on OOB-style indices
                    (
                        off.wrapping_add(elem.size().wrapping_mul(idx as u64)),
                        *elem,
                    )
                } else {
                    panic!("index on non-array")
                }
            }
        }
    }

    pub fn read_ref(&mut self, ctx: &Ctx, r: &Ref) -> Value {
        let (off, ty) = self.resolve(ctx, r);
        self.rd(off, &ty)
    }

    pub(crate) fn write_ref(&mut self, ctx: &Ctx, r: &Ref, v: &Value) {
        let (off, ty) = self.resolve(ctx, r);
        self.wr(off, &ty, v);
    }

}
