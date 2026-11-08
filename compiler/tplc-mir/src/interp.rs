//! Tree-walking MIR interpreter over a byte blob. Proves that the MIR +
//! layout pair is executable: refs resolve to `base + offset`, the same
//! addressing model generated code will use (doc 07 §3.4). Function
//! calls get call-scoped scratch memory (locals are transient, not in
//! the state blob — same split codegen will make). Eval/oracle-side
//! tool, not a shipping runtime.

use crate::ir::*;
use crate::layout::{builtin_fb_layout, layout_members, FbLayout, Layout};
use crate::ty::{builtin_fn, BuiltinFn};
use std::collections::BTreeMap;

/// Offsets at or above this base index into `scratch` instead of `blob`
/// — marks call-scoped storage in the ctx maps.
const SCRATCH: u64 = 1 << 48;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    I(i64),
    U(u64),
    F(f64),
    Time(i64), // ns
    Str(Vec<u8>),
    /// wide string: UTF-16 code units
    WStr(Vec<u16>),
}

impl Value {
    pub fn truthy(&self) -> bool {
        matches!(self, Value::Bool(true))
    }
    pub fn as_i64(&self) -> i64 {
        match self {
            Value::I(v) => *v,
            Value::U(v) => *v as i64,
            Value::F(v) => *v as i64,
            Value::Time(v) => *v,
            Value::Bool(v) => *v as i64,
            Value::Str(_) | Value::WStr(_) => 0,
        }
    }
    pub fn as_f64(&self) -> f64 {
        match self {
            Value::I(v) => *v as f64,
            Value::U(v) => *v as f64,
            Value::F(v) => *v,
            Value::Time(v) => *v as f64,
            _ => 0.0,
        }
    }
}

#[derive(Debug)]
pub enum Flow {
    Normal,
    /// IEC EXIT — breaks the innermost loop
    Exit,
    Return(Value),
    Abort(i32),
}

/// member name → (offset, type). Offsets ≥ SCRATCH index `scratch`.
pub type Ctx = BTreeMap<String, (u64, Type)>;

/// Build the program-level ctx: top-level var name → (blob offset, ty).
pub fn program_ctx(layout: &Layout) -> Ctx {
    layout
        .fields
        .iter()
        .filter(|(k, _)| !k.contains('.'))
        .map(|(k, f)| (k.clone(), (f.offset, f.ty.clone())))
        .collect()
}

/// Build the ctx for an FB body: member → (member offset, ty), resolved
/// relative to the instance base.
pub fn fb_ctx(fbl: &FbLayout) -> Ctx {
    fbl.members
        .iter()
        .map(|(k, (o, t))| (k.clone(), (*o, t.clone())))
        .collect()
}

pub struct Rt<'a> {
    pub blob: &'a mut [u8],
    pub layout: &'a Layout,
    pub module: &'a Module,
    pub now_ns: u64,
    pub cycle: u64,
    pub abort_flag: Option<&'a dyn Fn() -> bool>,
    scratch: Vec<u8>,
}

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
        }
    }

    fn region(&self, off: u64) -> (&[u8], u64) {
        if off >= SCRATCH {
            (&self.scratch, off - SCRATCH)
        } else {
            (self.blob, off)
        }
    }
    fn region_mut(&mut self, off: u64) -> (&mut [u8], u64) {
        if off >= SCRATCH {
            (&mut self.scratch, off - SCRATCH)
        } else {
            (self.blob, off)
        }
    }

    fn rd(&self, off: u64, ty: &Type) -> Value {
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
            Type::WStr { cap } => {
                let len = (u16::from_le_bytes([b[0], b[1]]) as usize)
                    .min(*cap as usize);
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

    fn wr(&mut self, off: u64, ty: &Type, v: &Value) {
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
    fn member_of(&self, fbname: &str) -> FbLayout {
        self.layout
            .fb_types
            .get(fbname)
            .cloned()
            .or_else(|| builtin_fb_layout(fbname))
            .unwrap_or_else(|| panic!("no FB layout {fbname}"))
    }

    fn resolve(&mut self, ctx: &Ctx, r: &Ref) -> (u64, Type) {
        match r {
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
                        (off + *moff, mty.clone())
                    }
                    Type::Struct { fields, .. } => {
                        let mut o = 0u64;
                        for (n, t) in &fields {
                            o = (o + t.align() - 1) & !(t.align() - 1);
                            if n == name {
                                return (off + o, t.clone());
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
                    (off + elem.size() * idx as u64, *elem)
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
    fn write_ref(&mut self, ctx: &Ctx, r: &Ref, v: &Value) {
        let (off, ty) = self.resolve(ctx, r);
        self.wr(off, &ty, v);
    }

    // ---- expressions ----
    pub fn eval(&mut self, ctx: &Ctx, e: &Expr) -> Value {
        match &e.kind {
            ExprKind::Lit(l) => match l {
                Lit::Bool(v) => Value::Bool(*v),
                Lit::Int(v) => Value::I(*v),
                Lit::UInt(v) => Value::U(*v),
                Lit::Real(v) => Value::F(*v),
                Lit::TimeNs(v) => Value::Time(*v),
                Lit::Str(s) => Value::Str(s.as_bytes().to_vec()),
                Lit::WStr(s) => Value::WStr(s.encode_utf16().collect()),
                Lit::EnumVal { val, .. } => Value::I(val.parse().unwrap_or(0)),
            },
            ExprKind::Ref(r) => self.read_ref(ctx, r),
            ExprKind::Un(op, a) => {
                let v = self.eval(ctx, a);
                match op {
                    UnOp::Neg => match v {
                        Value::F(x) => Value::F(-x),
                        x => Value::I(-x.as_i64()),
                    },
                    UnOp::Not => Value::Bool(!v.truthy()),
                }
            }
            ExprKind::Bin(op, a, b) => {
                use BinOp::*;
                match op {
                    AndThen => {
                        if !self.eval(ctx, a).truthy() {
                            return Value::Bool(false);
                        }
                        return Value::Bool(self.eval(ctx, b).truthy());
                    }
                    OrElse => {
                        if self.eval(ctx, a).truthy() {
                            return Value::Bool(true);
                        }
                        return Value::Bool(self.eval(ctx, b).truthy());
                    }
                    _ => {}
                }
                let l = self.eval(ctx, a);
                let r = self.eval(ctx, b);
                match op {
                    Add => arith(l, r, |x, y| x.wrapping_add(y), |x, y| x + y),
                    Sub => arith(l, r, |x, y| x.wrapping_sub(y), |x, y| x - y),
                    Mul => arith(l, r, |x, y| x.wrapping_mul(y), |x, y| x * y),
                    Div => {
                        if r.as_i64() == 0 {
                            Value::I(0) // div-zero fault model is P1; v0: saturate
                        } else {
                            arith(l, r, |x, y| x / y, |x, y| x / y)
                        }
                    }
                    Mod => arith(l, r, |x, y| x.wrapping_rem(y), |x, y| x % y),
                    Eq => Value::Bool(cmp(&l, &r) == 0),
                    Ne => Value::Bool(cmp(&l, &r) != 0),
                    Lt => Value::Bool(cmp(&l, &r) < 0),
                    Le => Value::Bool(cmp(&l, &r) <= 0),
                    Gt => Value::Bool(cmp(&l, &r) > 0),
                    Ge => Value::Bool(cmp(&l, &r) >= 0),
                    And => Value::Bool(l.truthy() & r.truthy()),
                    Or => Value::Bool(l.truthy() | r.truthy()),
                    Xor => Value::Bool(l.truthy() ^ r.truthy()),
                    Shl => Value::I(l.as_i64() << (r.as_i64() & 63)),
                    Shr => Value::I(l.as_i64() >> (r.as_i64() & 63)),
                    Rol | Ror => Value::I(l.as_i64()),
                    AndThen | OrElse => unreachable!(),
                }
            }
            ExprKind::Conv { to, e } => {
                let v = self.eval(ctx, e);
                match to {
                    Type::Bool => Value::Bool(v.truthy()),
                    Type::Int { signed: true, .. } => Value::I(v.as_i64()),
                    Type::Int { signed: false, .. } => Value::U(v.as_i64() as u64),
                    Type::Real { .. } => Value::F(v.as_f64()),
                    Type::Time => Value::Time(v.as_i64()),
                    _ => v,
                }
            }
            ExprKind::Call { fun, args } => self.fn_call(ctx, fun, args),
        }
    }

    /// FUNCTION invocation: inputs bind into scratch, body runs with
    /// call-scoped ctx, return value read from the name-var.
    /// LEN/CONCAT over narrow and wide strings (IEC string functions)
    fn builtin_call(&mut self, caller: &Ctx, b: BuiltinFn,
                    args: &[(String, Expr)]) -> Value {
        match b {
            BuiltinFn::Len => {
                let v = args.first()
                    .map(|(_, e)| self.eval(caller, e))
                    .unwrap_or(Value::I(0));
                let n = match v {
                    Value::Str(s) => s.len(),
                    Value::WStr(s) => s.len(),
                    _ => 0,
                };
                Value::I(n as i64)
            }
            BuiltinFn::Concat => {
                let a = args.first().map(|(_, e)| self.eval(caller, e));
                let b2 = args.get(1).map(|(_, e)| self.eval(caller, e));
                match (a, b2) {
                    (Some(Value::Str(x)), Some(Value::Str(y))) => {
                        Value::Str([x, y].concat())
                    }
                    (Some(Value::WStr(x)), Some(Value::WStr(y))) => {
                        Value::WStr([x, y].concat())
                    }
                    _ => Value::Str(Vec::new()),
                }
            }
        }
    }

    fn fn_call(&mut self, caller: &Ctx, fun: &str, args: &[(String, Expr)]) -> Value {
        if let Some(b) = builtin_fn(fun) {
            return self.builtin_call(caller, b, args);
        }
        let pou = self
            .module
            .pous
            .iter()
            .find(|p| p.name == fun && p.kind == PouKind::Function)
            .unwrap_or_else(|| panic!("no FUNCTION {fun}"));
        // evaluate args in caller ctx first
        let argv: Vec<Value> = args.iter().map(|(_, e)| self.eval(caller, e)).collect();
        // lay out fn vars (inputs + locals + implicit return var)
        let fbl = layout_members(&pou.iface, self.layout.fb_types.clone());
        let base = SCRATCH + self.scratch.len() as u64;
        self.scratch
            .resize(self.scratch.len() + fbl.size as usize, 0);
        let mut ctx = fb_ctx(&fbl);
        for v in ctx.values_mut() {
            v.0 += base;
        }
        // positional + named binding
        let inputs: Vec<&VarDecl> = pou
            .iface
            .iter()
            .filter(|v| matches!(v.kind, VarKind::Input | VarKind::InOut))
            .collect();
        for (i, (pname, _)) in args.iter().enumerate() {
            let name = if pname.is_empty() {
                inputs.get(i).map(|v| v.name.clone())
            } else {
                Some(pname.clone())
            };
            if let Some(n) = name {
                if let Some((o, t)) = ctx.get(&n).cloned() {
                    self.wr(o, &t, &argv[i]);
                }
            }
        }
        if let Body::Stmts(stmts) = &pou.body {
            self.exec_all(&ctx, stmts);
        }
        ctx.get(&pou.name)
            .map(|(o, t)| {
                let (o, t) = (*o, t.clone());
                self.rd(o, &t)
            })
            .unwrap_or(Value::I(0))
    }

    // ---- statements ----
    pub fn exec(&mut self, ctx: &Ctx, s: &Stmt) -> Flow {
        use StmtKind::*;
        match &s.kind {
            Assign { lhs, rhs } => {
                let v = self.eval(ctx, rhs);
                self.write_ref(ctx, lhs, &v);
                Flow::Normal
            }
            If { branches, else_ } => {
                for (cond, stmts) in branches {
                    if self.eval(ctx, cond).truthy() {
                        return self.exec_all(ctx, stmts);
                    }
                }
                self.exec_all(ctx, else_)
            }
            While { cond, body } => {
                let mut guard = 0u64;
                while self.eval(ctx, cond).truthy() {
                    if self.abort_check() {
                        return Flow::Abort(-1);
                    }
                    match self.exec_all(ctx, body) {
                        Flow::Normal => {}
                        Flow::Exit => break,
                        f => return f,
                    }
                    guard += 1;
                    if guard > 10_000_000 {
                        return Flow::Abort(-2);
                    }
                }
                Flow::Normal
            }
            Repeat { cond, body } => {
                let mut guard = 0u64;
                loop {
                    if self.abort_check() {
                        return Flow::Abort(-1);
                    }
                    match self.exec_all(ctx, body) {
                        Flow::Normal => {}
                        Flow::Exit => break,
                        f => return f,
                    }
                    if self.eval(ctx, cond).truthy() {
                        break;
                    }
                    guard += 1;
                    if guard > 10_000_000 {
                        return Flow::Abort(-2);
                    }
                }
                Flow::Normal
            }
            For {
                var,
                start,
                end,
                step,
                body,
            } => {
                let s0 = self.eval(ctx, start).as_i64();
                let e0 = self.eval(ctx, end).as_i64();
                let st = step
                    .as_ref()
                    .map(|s| self.eval(ctx, s).as_i64())
                    .unwrap_or(1);
                if st == 0 {
                    return Flow::Abort(-3);
                }
                let r = Ref::Var(var.clone());
                let mut i = s0;
                while if st > 0 { i <= e0 } else { i >= e0 } {
                    self.write_ref(ctx, &r, &Value::I(i));
                    match self.exec_all(ctx, body) {
                        Flow::Normal => {}
                        Flow::Exit => break,
                        f => return f,
                    }
                    if self.abort_check() {
                        return Flow::Abort(-1);
                    }
                    i += st;
                }
                Flow::Normal
            }
            Case { sel, arms, else_ } => {
                let sv = self.eval(ctx, sel);
                for (sels, stmts) in arms {
                    for cs in sels {
                        let hit = match cs {
                            CaseSel::Lit(e) => cmp(&sv, &self.eval(ctx, e)) == 0,
                            CaseSel::Range(a, b) => {
                                let lo = self.eval(ctx, a);
                                let hi = self.eval(ctx, b);
                                cmp(&sv, &lo) >= 0 && cmp(&sv, &hi) <= 0
                            }
                        };
                        if hit {
                            return self.exec_all(ctx, stmts);
                        }
                    }
                }
                self.exec_all(ctx, else_)
            }
            Call { inst, args } => self.fb_call(ctx, inst, args),
            Return(e) => Flow::Return(e.as_ref().map(|x| self.eval(ctx, x)).unwrap_or(Value::I(0))),
            Exit => Flow::Exit,
            Continue => Flow::Normal,
        }
    }

    pub fn exec_all(&mut self, ctx: &Ctx, stmts: &[Stmt]) -> Flow {
        for s in stmts {
            match self.exec(ctx, s) {
                Flow::Normal => {}
                f => return f,
            }
        }
        Flow::Normal
    }

    fn abort_check(&self) -> bool {
        self.abort_flag.map(|f| f()).unwrap_or(false)
    }

    /// FB-instance invocation. Builtins run spec semantics on the
    /// instance sub-object; user FBs recurse into their POU body.
    fn fb_call(&mut self, ctx: &Ctx, inst: &Ref, args: &[(String, Expr)]) -> Flow {
        let (off, ty) = self.resolve(ctx, inst);
        let fbname = match &ty {
            Type::Fb { name } => name.clone(),
            _ => panic!("call on non-FB ref"),
        };
        let fbl = self.member_of(&fbname);
        // bind named args to instance fields (evaluate in caller ctx)
        for (pname, pe) in args {
            if let Some((moff, mty)) = fbl.members.get(pname.as_str()) {
                let v = self.eval(ctx, pe);
                let (o, t) = (off + *moff, mty.clone());
                self.wr(o, &t, &v);
            }
        }
        match fbname.as_str() {
            "TON" => {
                let m = &fbl.members;
                let get = |n: &str| (off + m[n].0) as usize;
                let in_v = self.blob[get("IN")] != 0;
                let pt =
                    i64::from_le_bytes(self.blob[get("PT")..get("PT") + 8].try_into().unwrap());
                let last = self.blob[get("__in_last")] != 0;
                let now = self.now_ns as i64;
                if in_v && !last {
                    self.blob[get("__start")..get("__start") + 8]
                        .copy_from_slice(&now.to_le_bytes());
                }
                let start = i64::from_le_bytes(
                    self.blob[get("__start")..get("__start") + 8]
                        .try_into()
                        .unwrap(),
                );
                let (q, et) = if in_v {
                    let el = now - start;
                    (el >= pt, el.min(pt))
                } else {
                    (false, 0)
                };
                self.blob[get("Q")] = q as u8;
                self.blob[get("ET")..get("ET") + 8].copy_from_slice(&et.to_le_bytes());
                self.blob[get("__in_last")] = in_v as u8;
                Flow::Normal
            }
            "TOF" => {
                // IN → Q=1 immediately, ET=0; IN↓ starts off-timer
                let m = &fbl.members;
                let get = |n: &str| (off + m[n].0) as usize;
                let inn = self.blob[get("IN")] != 0;
                let pt =
                    i64::from_le_bytes(self.blob[get("PT")..get("PT") + 8].try_into().unwrap());
                let last = self.blob[get("__in_last")] != 0;
                let now = self.now_ns as i64;
                if inn {
                    self.blob[get("Q")] = 1;
                    self.blob[get("ET")..get("ET") + 8].copy_from_slice(&0i64.to_le_bytes());
                    self.blob[get("__run")] = 0;
                } else {
                    if last {
                        self.blob[get("__start")..get("__start") + 8]
                            .copy_from_slice(&now.to_le_bytes());
                        self.blob[get("__run")] = 1;
                    }
                    if self.blob[get("__run")] != 0 {
                        let start = i64::from_le_bytes(
                            self.blob[get("__start")..get("__start") + 8]
                                .try_into()
                                .unwrap(),
                        );
                        let et = (now - start).min(pt);
                        self.blob[get("ET")..get("ET") + 8].copy_from_slice(&et.to_le_bytes());
                        if et >= pt {
                            self.blob[get("Q")] = 0;
                            self.blob[get("__run")] = 0;
                        }
                    }
                }
                self.blob[get("__in_last")] = inn as u8;
                Flow::Normal
            }
            "TP" => {
                // IN↑ starts a non-retriggerable PT pulse
                let m = &fbl.members;
                let get = |n: &str| (off + m[n].0) as usize;
                let inn = self.blob[get("IN")] != 0;
                let pt =
                    i64::from_le_bytes(self.blob[get("PT")..get("PT") + 8].try_into().unwrap());
                let last = self.blob[get("__in_last")] != 0;
                let now = self.now_ns as i64;
                if inn && !last && self.blob[get("__run")] == 0 {
                    self.blob[get("__run")] = 1;
                    self.blob[get("__start")..get("__start") + 8]
                        .copy_from_slice(&now.to_le_bytes());
                    self.blob[get("Q")] = 1;
                }
                if self.blob[get("__run")] != 0 {
                    let start = i64::from_le_bytes(
                        self.blob[get("__start")..get("__start") + 8]
                            .try_into()
                            .unwrap(),
                    );
                    let et = (now - start).min(pt);
                    self.blob[get("ET")..get("ET") + 8].copy_from_slice(&et.to_le_bytes());
                    if et >= pt {
                        self.blob[get("Q")] = 0;
                        self.blob[get("__run")] = 0;
                    }
                }
                self.blob[get("__in_last")] = inn as u8;
                Flow::Normal
            }
            "R_TRIG" | "F_TRIG" => {
                let m = &fbl.members;
                let get = |n: &str| (off + m[n].0) as usize;
                let clk = self.blob[get("CLK")] != 0;
                let last = self.blob[get("__clk_last")] != 0;
                self.blob[get("Q")] = if fbname == "R_TRIG" {
                    clk && !last
                } else {
                    !clk && last
                } as u8;
                self.blob[get("__clk_last")] = clk as u8;
                Flow::Normal
            }
            "CTU" | "CTD" | "CTUD" => {
                let m = &fbl.members;
                let get = |n: &str| (off + m[n].0) as usize;
                let cu = self.blob[get("CU")] != 0;
                let cd = self.blob[get("CD")] != 0;
                let r = self.blob[get("R")] != 0;
                let ld = self.blob[get("LD")] != 0;
                let cu_last = self.blob[get("__cu_last")] != 0;
                let cd_last = self.blob[get("__cd_last")] != 0;
                let pv =
                    i32::from_le_bytes(self.blob[get("PV")..get("PV") + 4].try_into().unwrap());
                let mut cv =
                    i32::from_le_bytes(self.blob[get("CV")..get("CV") + 4].try_into().unwrap());
                if ld {
                    cv = pv;
                }
                if r {
                    cv = 0;
                }
                if !r && !ld {
                    if cu && !cu_last {
                        cv = cv.saturating_add(1);
                    }
                    if cd && !cd_last {
                        cv = cv.saturating_sub(1);
                    }
                }
                self.blob[get("CV")..get("CV") + 4].copy_from_slice(&cv.to_le_bytes());
                self.blob[get("QU")] = (cv >= pv) as u8;
                self.blob[get("QD")] = (cv <= 0) as u8;
                self.blob[get("__cu_last")] = cu as u8;
                self.blob[get("__cd_last")] = cd as u8;
                Flow::Normal
            }
            "SR" | "RS" => {
                // SR set-dominant (S1 wins), RS reset-dominant (R1 wins)
                let m = &fbl.members;
                let get = |n: &str| (off + m[n].0) as usize;
                let s = self.blob[get("S")] != 0;
                let r1 = self.blob[get("R1")] != 0;
                let s1 = self.blob[get("S1")] != 0;
                let r = self.blob[get("R")] != 0;
                let q1 = if fbname == "SR" {
                    if s1 {
                        true
                    } else if r {
                        false
                    } else {
                        self.blob[get("Q1")] != 0
                    }
                } else if r1 {
                    false
                } else if s {
                    true
                } else {
                    self.blob[get("Q1")] != 0
                };
                self.blob[get("Q1")] = q1 as u8;
                Flow::Normal
            }
            _ => {
                if let Some(pou) = self
                    .module
                    .pous
                    .iter()
                    .find(|p| p.name == fbname && p.kind == PouKind::FunctionBlock)
                {
                    if let Body::Stmts(stmts) = &pou.body {
                        let mut sub = fb_ctx(&fbl);
                        for v in sub.values_mut() {
                            v.0 += off;
                        }
                        return self.exec_all(&sub, stmts);
                    }
                }
                Flow::Normal
            }
        }
    }
}

fn arith(l: Value, r: Value, fi: fn(i64, i64) -> i64, ff: fn(f64, f64) -> f64) -> Value {
    match (&l, &r) {
        (Value::F(_), _) | (_, Value::F(_)) => Value::F(ff(l.as_f64(), r.as_f64())),
        (Value::Time(_), _) | (_, Value::Time(_)) => Value::Time(fi(l.as_i64(), r.as_i64())),
        (Value::U(_), Value::U(_)) => Value::U(fi(l.as_i64(), r.as_i64()) as u64),
        _ => Value::I(fi(l.as_i64(), r.as_i64())),
    }
}

fn cmp(l: &Value, r: &Value) -> i8 {
    if matches!(l, Value::F(_)) || matches!(r, Value::F(_)) {
        let (x, y) = (l.as_f64(), r.as_f64());
        return if x < y {
            -1
        } else if x > y {
            1
        } else {
            0
        };
    }
    match (l, r) {
        (Value::Bool(a), Value::Bool(b)) => (*a as i8) - (*b as i8),
        (Value::WStr(a), Value::WStr(b)) => match a.cmp(b) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        },
        (Value::Str(a), Value::Str(b)) => {
            // lexicographic over bytes (IEC defines =/<> only; the
            // ordering is what a memcmp-style engine would produce)
            match a.cmp(b) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            }
        }
        _ => {
            let (a, b) = (l.as_i64(), r.as_i64());
            if a < b {
                -1
            } else if a > b {
                1
            } else {
                0
            }
        }
    }
}
