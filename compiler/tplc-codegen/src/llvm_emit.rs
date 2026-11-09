//! `tplc-mir` → LLVM IR lowering (doc 07 §4).
//!
//! Memory model: every access is `i8* + const-offset` into ctx fields
//! and the state blob — identical addressing to the interpreter
//! (interp.rs), so oracle/sim parity is structural, not aspirational.
//!
//! Ctx layout (tplc_abi.h): state_blob@8, now_ns@48, abort_flag@88,
//! budget_deadline_ns@96. TplcTaskCtx: prog@0, task_index@8.
//!
//! Generated pou fns: `__tplc_pou_<NAME>(u8* ctx, u8* this) -> i32`
//! rc<0 = abort/fault propagation. Program `this` = state_blob base;
//! FB `this` = instance sub-object; FUNCTION `this` = caller alloca.

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module as LlvmModule};
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, IntValue, PointerValue};
use inkwell::{FloatPredicate, IntPredicate};
use std::collections::BTreeMap;
use tplc_mir::interp::{fb_ctx, program_ctx, Ctx};
use tplc_mir::ir::AtRegion;
use tplc_mir::ir::*;
use tplc_mir::layout::{builtin_fb_layout, Layout};
use tplc_mir::migrate::{MigOp, MigrateEmit};

const CTX_STATE_BLOB: u64 = 8;
const CTX_SVC: u64 = 72;
const CTX_ABORT: u64 = 88;
const CTX_DEADLINE: u64 = 96;
const TCTX_PROG: u64 = 0;

/// Type → TplcIoType code (tplc_abi.h §enum TplcIoType); unit = bit idx.
fn io_codes(at: &At, ty: &Type) -> (u8, u8, u8) {
    let region = match at.region {
        AtRegion::I => 0,
        AtRegion::Q => 1,
        AtRegion::M => 2,
    };
    let tc = match ty {
        Type::Bool => 0,
        Type::Int {
            signed: false,
            bits: 8,
        } => 1,
        Type::Int {
            signed: true,
            bits: 8,
        } => 2,
        Type::Int {
            signed: false,
            bits: 16,
        } => 3,
        Type::Int {
            signed: true,
            bits: 16,
        } => 4,
        Type::Int {
            signed: false,
            bits: 32,
        } => 5,
        Type::Int {
            signed: true,
            bits: 32,
        } => 6,
        Type::Int {
            signed: false,
            bits: 64,
        } => 7,
        Type::Int {
            signed: true,
            bits: 64,
        } => 8,
        Type::Real { bits: 32 } => 9,
        Type::Real { bits: 64 } => 10,
        _ => 15,
    };
    (region, tc, at.bit.unwrap_or(0))
}

/// highest byte touched in an image region (descriptor image_*_bytes)
fn img_bytes(binds: &[IoBind], r: AtRegion) -> u64 {
    binds
        .iter()
        .filter(|b| b.at.region == r)
        .map(|b| b.at.byte + b.ty.size().max(1))
        .max()
        .unwrap_or(0)
}

pub struct Gen<'ctx> {
    pub ctx: &'ctx Context,
    pub module: LlvmModule<'ctx>,
    pub b: Builder<'ctx>,
    layout: &'static Layout,
    mir: &'static Module,
    abort_bb: Option<BasicBlock<'ctx>>,
    overrun_bb: Option<BasicBlock<'ctx>>,
    /// null-reference deref → -17, out-of-range deref → -18
    panic_bb: Option<BasicBlock<'ctx>>,
    bounds_bb: Option<BasicBlock<'ctx>>,
    loop_stack: Vec<(BasicBlock<'ctx>, BasicBlock<'ctx>)>, // (cond, exit)
    ctx_ptr: PointerValue<'ctx>,
    this_ptr: PointerValue<'ctx>,
}

fn llty<'c>(ctx: &'c Context, t: &Type) -> BasicTypeEnum<'c> {
    match t {
        Type::Bool => ctx.bool_type().into(),
        Type::Int { signed: _, bits } => ctx
            .custom_width_int_type(std::num::NonZeroU32::new(*bits as u32).unwrap())
            .unwrap()
            .into(),
        Type::Real { bits: 32 } => ctx.f32_type().into(),
        Type::Real { .. } => ctx.f64_type().into(),
        Type::Time | Type::Enum { .. } => ctx.i64_type().into(),
        Type::Str { cap } => ctx.i8_type().array_type(1 + *cap as u32).into(),
        Type::WStr { cap } => ctx.i16_type().array_type(1 + *cap as u32).into(),
        Type::Ref { .. } => ctx.i64_type().into(),
        _ => ctx.i8_type().into(), // aggregates addressed, not valued
    }
}

/// integer-ish values used for bool storage
fn is_bool(t: &Type) -> bool {
    matches!(t, Type::Bool)
}

impl<'ctx> Gen<'ctx> {
    fn i8p(&self) -> inkwell::types::PointerType<'ctx> {
        self.ctx.ptr_type(inkwell::AddressSpace::default())
    }
    /// gep on an i8* base by constant offset
    fn off_ptr(&self, base: PointerValue<'ctx>, off: u64) -> PointerValue<'ctx> {
        unsafe {
            self.b
                .build_in_bounds_gep(
                    self.ctx.i8_type(),
                    base,
                    &[self.ctx.i64_type().const_int(off, false)],
                    "p",
                )
                .unwrap()
        }
    }

    /// ctx field load: i8* ctx + off → typed load
    fn ctx_load(&self, off: u64, ty: BasicTypeEnum<'ctx>, n: &str) -> BasicValueEnum<'ctx> {
        let p = self.off_ptr(self.ctx_ptr, off);
        self.b.build_load(ty, p, n).unwrap()
    }

    fn blob_ptr(&self) -> PointerValue<'ctx> {
        self.this_ptr
    }

    // ---- ref resolution → (ptr, Type) ----
    fn member_of(&self, fbname: &str) -> tplc_mir::layout::FbLayout {
        self.layout
            .fb_types
            .get(fbname)
            .cloned()
            .or_else(|| builtin_fb_layout(fbname))
            .unwrap_or_else(|| panic!("no FB layout {fbname}"))
    }

    fn resolve(&mut self, ctx: &Ctx, r: &Ref) -> (PointerValue<'ctx>, Type) {
        match r {
            /* r^: the slot holds a blob offset; null → -17, out of
               range → -18. The pointee type comes from the slot's
               declared Ref type. */
            Ref::Deref(inner) => {
                let (slot, ty) = self.resolve(ctx, inner);
                let to = match &ty {
                    Type::Ref { to } => (**to).clone(),
                    _ => Type::Bool,
                };
                let i64t = self.ctx.i64_type();
                let off = self.b.build_load(i64t, slot, "ro").unwrap().into_int_value();
                let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                let ok_bb = self.ctx.append_basic_block(f, "ref.ok");
                let null = self
                    .b
                    .build_int_compare(
                        IntPredicate::EQ,
                        off,
                        i64t.const_int(u64::MAX, false),
                        "rn",
                    )
                    .unwrap();
                let cur = self.b.get_insert_block().unwrap();
                self.b
                    .build_conditional_branch(null, self.panic_bb.unwrap(), ok_bb)
                    .unwrap();
                self.b.position_at_end(ok_bb);
                let blob_sz = i64t.const_int(self.layout.total_size, false);
                let end = self
                    .b
                    .build_int_add(off, i64t.const_int(to.size(), false), "re")
                    .unwrap();
                let over = self
                    .b
                    .build_int_compare(IntPredicate::UGT, end, blob_sz, "rb")
                    .unwrap();
                let ok2 = self.ctx.append_basic_block(f, "ref.ok2");
                self.b
                    .build_conditional_branch(over, self.bounds_bb.unwrap(), ok2)
                    .unwrap();
                self.b.position_at_end(ok2);
                let _ = cur;
                let base8 = self
                    .b
                    .build_pointer_cast(self.blob_ptr(), self.i8p(), "b8")
                    .unwrap();
                let p = unsafe {
                    self.b
                        .build_gep(self.ctx.i8_type(), base8, &[off], "rd")
                        .unwrap()
                };
                (p, to)
            }
            Ref::Var(name) => {
                let (o, t) = ctx
                    .get(name.as_str())
                    .cloned()
                    .unwrap_or_else(|| panic!("unresolved var {name}"));
                (self.off_ptr(self.blob_ptr(), o), t)
            }
            Ref::Field(inner, name) => {
                let (p, ty) = self.resolve(ctx, inner);
                match &ty {
                    Type::Fb { name: fb } => {
                        let fbl = self.member_of(fb);
                        let (moff, mty) = fbl.members[name.as_str()].clone();
                        (self.off_ptr(p, moff), mty)
                    }
                    Type::Struct { fields, .. } => {
                        let mut o = 0u64;
                        for (n, t) in fields {
                            o = (o + t.align() - 1) & !(t.align() - 1);
                            if n == name {
                                return (self.off_ptr(p, o), t.clone());
                            }
                            o += t.size();
                        }
                        panic!("no field {name}");
                    }
                    _ => panic!("field on non-aggregate"),
                }
            }
            Ref::Index(inner, idx) => {
                let (p, ty) = self.resolve(ctx, inner);
                if let Type::Array { elem, lo, .. } = ty {
                    let iv = self.expr(ctx, idx);
                    let iv = self
                        .b
                        .build_int_sub(
                            iv.into_int_value(),
                            self.ctx.i64_type().const_int(lo as u64, true),
                            "idx",
                        )
                        .unwrap();
                    let esz = self.ctx.i64_type().const_int(elem.size(), false);
                    let bo = self.b.build_int_mul(iv, esz, "bo").unwrap();
                    unsafe {
                        let q = self
                            .b
                            .build_gep(self.ctx.i8_type(), p, &[bo], "ai")
                            .unwrap();
                        (q, *elem)
                    }
                } else {
                    panic!("index on non-array")
                }
            }
        }
    }

    fn read(&mut self, ctx: &Ctx, r: &Ref) -> BasicValueEnum<'ctx> {
        let (p, ty) = self.resolve(ctx, r);
        if is_bool(&ty) {
            let v = self.b.build_load(self.ctx.i8_type(), p, "lb").unwrap();
            self.b
                .build_int_compare(
                    IntPredicate::NE,
                    v.into_int_value(),
                    self.ctx.i8_type().const_zero(),
                    "lb1",
                )
                .unwrap()
                .into()
        } else {
            self.b.build_load(llty(self.ctx, &ty), p, "l").unwrap()
        }
    }

    fn write(&mut self, ctx: &Ctx, r: &Ref, v: BasicValueEnum<'ctx>) {
        let (p, ty) = self.resolve(ctx, r);
        let v = if is_bool(&ty) {
            self.b
                .build_int_z_extend(v.into_int_value(), self.ctx.i8_type(), "z")
                .unwrap()
                .into()
        } else if let Type::Str { cap } = &ty {
            // source array may differ in width (literal cap vs target
            // cap): rebuild to the target width, zero-padding/truncating
            self.str_fit(v, 1 + *cap as u32)
        } else if let Type::WStr { cap } = &ty {
            self.str_fit(v, 1 + *cap as u32)
        } else {
            v
        };
        self.b.build_store(p, v).unwrap();
    }

    /// coerce an array value to `want` elements, zero-padding or
    /// truncating (STRING store semantics: len byte + data, rest 0)
    fn str_fit(&mut self, v: BasicValueEnum<'ctx>, want: u32) -> BasicValueEnum<'ctx> {
        let av = match v {
            BasicValueEnum::ArrayValue(a) => a,
            other => return other,
        };
        let have = av.get_type().len();
        if have == want {
            return av.into();
        }
        let elem = av.get_type().get_element_type().into_int_type();
        let zeros: Vec<inkwell::values::IntValue<'ctx>> =
            vec![elem.const_zero(); want as usize];
        let mut acc = elem.const_array(&zeros);
        for i in 0..want.min(have) {
            let el = self.b.build_extract_value(av, i, "se").unwrap();
            acc = self
                .b
                .build_insert_value(acc, el, i, "si")
                .unwrap()
                .into_array_value();
        }
        if have > want {
            /* truncating store: the len element must not claim data
               that no longer fits (a len > cap is a malformed string) */
            let len = self
                .b
                .build_extract_value(acc, 0, "tl")
                .unwrap()
                .into_int_value();
            let max = elem.const_int((want - 1) as u64, false);
            let over = self
                .b
                .build_int_compare(IntPredicate::UGT, len, max, "tov")
                .unwrap();
            let clamped = self
                .b
                .build_select(over, max, len, "tc")
                .unwrap()
                .into_int_value();
            acc = self
                .b
                .build_insert_value(acc, clamped, 0, "tsi")
                .unwrap()
                .into_array_value();
        }
        acc.into()
    }

    /// blob offset of an lvalue as an i64 value (REF(x) / `&x`):
    /// constant for vars/fields, runtime for array indices and for
    /// re-binding through an existing reference
    fn ref_offset(&mut self, ctx: &Ctx, r: &Ref) -> inkwell::values::IntValue<'ctx> {
        let i64t = self.ctx.i64_type();
        match r {
            Ref::Var(name) => {
                let (o, _) = ctx
                    .get(name.as_str())
                    .cloned()
                    .unwrap_or_else(|| panic!("unresolved var {name}"));
                i64t.const_int(o, false)
            }
            Ref::Field(inner, name) => {
                let (_, ty) = self.resolve(ctx, inner);
                let moff = match &ty {
                    Type::Fb { name: fb } => {
                        let fbl = tplc_mir::layout::builtin_fb_layout(fb)
                            .or_else(|| {
                                self.layout.fb_types.get(fb).cloned()
                            })
                            .unwrap_or_else(|| panic!("no FB layout {fb}"));
                        fbl.members
                            .get(name.as_str())
                            .map(|(o, _)| *o)
                            .unwrap_or_else(|| panic!("{fb} has no member {name}"))
                    }
                    Type::Struct { fields, .. } => {
                        let mut o = 0u64;
                        let mut found = None;
                        for (n, t) in fields {
                            o = (o + t.align() - 1) & !(t.align() - 1);
                            if n == name {
                                found = Some(o);
                                break;
                            }
                            o += t.size();
                        }
                        found.unwrap_or_else(|| panic!("struct has no field {name}"))
                    }
                    _ => panic!("field on non-aggregate"),
                };
                let base = self.ref_offset(ctx, inner);
                self.b
                    .build_int_add(base, i64t.const_int(moff, false), "fo")
                    .unwrap()
            }
            Ref::Index(inner, idx) => {
                let (_, ty) = self.resolve(ctx, inner);
                let (lo, elem) = match &ty {
                    Type::Array { elem, lo, .. } => (*lo, (**elem).clone()),
                    _ => panic!("index on non-array"),
                };
                let iv = self.expr(ctx, idx).into_int_value();
                let iv = self
                    .b
                    .build_int_sub(iv, i64t.const_int(lo as u64, true), "ri")
                    .unwrap();
                let esz = self.b
                    .build_int_mul(iv, i64t.const_int(elem.size(), false), "re")
                    .unwrap();
                let base = self.ref_offset(ctx, inner);
                self.b.build_int_add(base, esz, "ra").unwrap()
            }
            Ref::Deref(inner) => {
                let (slot, _) = self.resolve(ctx, inner);
                self.b.build_load(i64t, slot, "rl").unwrap().into_int_value()
            }
        }
    }

    // ---- expressions ----
    fn expr(&mut self, ctx: &Ctx, e: &Expr) -> BasicValueEnum<'ctx> {
        match &e.kind {
            // literals materialize at the expr's declared type — sema
            // already fixed the type, codegen never re-guesses
            ExprKind::Lit(l) => {
                let lt = llty(self.ctx, &e.ty);
                match l {
                    Lit::Bool(v) => self.ctx.bool_type().const_int(*v as u64, false).into(),
                    Lit::Int(v) => lt.into_int_type().const_int(*v as u64, true).into(),
                    Lit::UInt(v) => lt.into_int_type().const_int(*v, false).into(),
                    Lit::Real(v) => lt.into_float_type().const_float(*v).into(),
                    Lit::TimeNs(v) => lt.into_int_type().const_int(*v as u64, true).into(),
                    Lit::EnumVal { .. } => lt.into_int_type().const_zero().into(),
                    Lit::Str(s) => {
                        // STRING literal → [1+cap x i8] {len, bytes, 0…}
                        // (a bare i8 zero would only clear the len byte)
                        let cap = match &e.ty {
                            Type::Str { cap } => *cap as usize,
                            _ => s.len(),
                        };
                        let i8t = self.ctx.i8_type();
                        let n = s.len().min(cap);
                        let mut els: Vec<inkwell::values::IntValue<'ctx>> =
                            Vec::with_capacity(1 + cap);
                        els.push(i8t.const_int(n as u64, false));
                        for b in s.as_bytes().iter().take(n) {
                            els.push(i8t.const_int(*b as u64, false));
                        }
                        while els.len() < 1 + cap {
                            els.push(i8t.const_zero());
                        }
                        i8t.const_array(&els).into()
                    }
                    Lit::WStr(s) => {
                        // WSTRING literal → [1+cap x i16] {len, units, 0…}
                        let cap = match &e.ty {
                            Type::WStr { cap } => *cap as usize,
                            _ => s.encode_utf16().count(),
                        };
                        let i16t = self.ctx.i16_type();
                        let units: Vec<u16> = s.encode_utf16().collect();
                        let n = units.len().min(cap);
                        let mut els: Vec<inkwell::values::IntValue<'ctx>> =
                            Vec::with_capacity(1 + cap);
                        els.push(i16t.const_int(n as u64, false));
                        for u in units.iter().take(n) {
                            els.push(i16t.const_int(*u as u64, false));
                        }
                        while els.len() < 1 + cap {
                            els.push(i16t.const_zero());
                        }
                        i16t.const_array(&els).into()
                    }
                }
            }
            ExprKind::RefTo { r } => self.ref_offset(ctx, r).into(),
            ExprKind::Ref(r) => self.read(ctx, r),
            ExprKind::Un(op, a) => {
                let v = self.expr(ctx, a);
                match op {
                    UnOp::Neg => {
                        if v.is_float_value() {
                            self.b
                                .build_float_neg(v.into_float_value(), "n")
                                .unwrap()
                                .into()
                        } else {
                            self.b
                                .build_int_neg(v.into_int_value(), "n")
                                .unwrap()
                                .into()
                        }
                    }
                    UnOp::Not => self.b.build_not(v.into_int_value(), "not").unwrap().into(),
                }
            }
            ExprKind::Bin(op, a, b) => self.bin(ctx, *op, a, b, &e.ty),
            ExprKind::Conv { to, e } => {
                let v = self.expr(ctx, e);
                self.conv(v, &e.ty, to)
            }
            ExprKind::Call { fun, args } => self.fn_call(ctx, fun, args, &e.ty),
        }
    }

    fn conv(&mut self, v: BasicValueEnum<'ctx>, from: &Type, to: &Type) -> BasicValueEnum<'ctx> {
        match (from, to) {
            (Type::Bool, Type::Int { .. }) => self
                .b
                .build_int_z_extend(v.into_int_value(), llty(self.ctx, to).into_int_type(), "c")
                .unwrap()
                .into(),
            (Type::Int { .. }, Type::Bool) => self
                .b
                .build_int_compare(
                    IntPredicate::NE,
                    v.into_int_value(),
                    llty(self.ctx, from).into_int_type().const_zero(),
                    "c",
                )
                .unwrap()
                .into(),
            (Type::Int { signed: s, .. }, Type::Real { .. }) => {
                if *s {
                    self.b
                        .build_signed_int_to_float(
                            v.into_int_value(),
                            llty(self.ctx, to).into_float_type(),
                            "c",
                        )
                        .unwrap()
                        .into()
                } else {
                    self.b
                        .build_unsigned_int_to_float(
                            v.into_int_value(),
                            llty(self.ctx, to).into_float_type(),
                            "c",
                        )
                        .unwrap()
                        .into()
                }
            }
            (Type::Real { .. }, Type::Int { .. }) => self
                .b
                .build_float_to_signed_int(
                    v.into_float_value(),
                    llty(self.ctx, to).into_int_type(),
                    "c",
                )
                .unwrap()
                .into(),
            (Type::Real { bits: fb }, Type::Real { bits: tb }) if fb != tb => {
                let dt = llty(self.ctx, to).into_float_type();
                if tb > fb {
                    self.b
                        .build_float_ext(v.into_float_value(), dt, "c")
                        .unwrap()
                        .into()
                } else {
                    self.b
                        .build_float_trunc(v.into_float_value(), dt, "c")
                        .unwrap()
                        .into()
                }
            }
            (Type::Int { signed: s, .. }, Type::Int { .. }) => {
                let dt = llty(self.ctx, to).into_int_type();
                let iv = v.into_int_value();
                let sw = iv.get_type().get_bit_width();
                let dw = dt.get_bit_width();
                if dw > sw {
                    if *s {
                        self.b.build_int_s_extend(iv, dt, "c").unwrap().into()
                    } else {
                        self.b.build_int_z_extend(iv, dt, "c").unwrap().into()
                    }
                } else if dw < sw {
                    self.b.build_int_truncate(iv, dt, "c").unwrap().into()
                } else {
                    v
                }
            }
            (Type::Real { .. }, Type::Real { bits }) => {
                let dt = llty(self.ctx, to).into_float_type();
                if *bits == 32 {
                    self.b
                        .build_float_trunc(v.into_float_value(), dt, "c")
                        .unwrap()
                        .into()
                } else {
                    self.b
                        .build_float_ext(v.into_float_value(), dt, "c")
                        .unwrap()
                        .into()
                }
            }
            _ => v,
        }
    }

    /// numeric coercion for operand-width plumbing (sema owns semantic
    /// conversions; LLVM just needs equal-width operands)
    fn coerce(&mut self, v: BasicValueEnum<'ctx>, from: &Type, to: &Type) -> BasicValueEnum<'ctx> {
        if v.is_int_value() && matches!(to, Type::Int { .. } | Type::Time | Type::Enum { .. }) {
            let dt = llty(self.ctx, to).into_int_type();
            let iv = v.into_int_value();
            let (sw, dw) = (iv.get_type().get_bit_width(), dt.get_bit_width());
            if dw > sw {
                let sext = matches!(from, Type::Int { signed: true, .. });
                return if sext {
                    self.b.build_int_s_extend(iv, dt, "w").unwrap().into()
                } else {
                    self.b.build_int_z_extend(iv, dt, "w").unwrap().into()
                };
            }
            if dw < sw {
                return self.b.build_int_truncate(iv, dt, "w").unwrap().into();
            }
        }
        if v.is_float_value() && matches!(to, Type::Real { .. }) {
            let dt = llty(self.ctx, to).into_float_type();
            let fv = v.into_float_value();
            if fv.get_type() != dt {
                return self.b.build_float_ext(fv, dt, "w").unwrap().into();
            }
        }
        v
    }

    /// common numeric type for comparisons (wider wins; signed wins ties)
    fn cmp_ty(a: &Type, b: &Type) -> Type {
        let bits = |t: &Type| match t {
            Type::Int { bits, .. } => *bits,
            Type::Real { bits } => *bits,
            Type::Bool => 1,
            _ => 64,
        };
        if matches!(a, Type::Real { .. }) || matches!(b, Type::Real { .. }) {
            return Type::Real {
                bits: bits(a).max(bits(b)).max(32),
            };
        }
        let (ba, bb) = (bits(a), bits(b));
        let w = ba.max(bb);
        let signed = matches!(a, Type::Int { signed: true, .. })
            || matches!(b, Type::Int { signed: true, .. })
            || matches!(a, Type::Time)
            || matches!(b, Type::Time);
        if matches!(a, Type::Time) || matches!(b, Type::Time) {
            return Type::Time;
        }
        Type::Int { signed, bits: w }
    }

    fn bin(
        &mut self,
        ctx: &Ctx,
        op: BinOp,
        a: &Expr,
        b: &Expr,
        e_ty: &Type,
    ) -> BasicValueEnum<'ctx> {
        use BinOp::*;
        match op {
            AndThen | OrElse => {
                // short-circuit via blocks
                let l_bb = self.b.get_insert_block().unwrap();
                let f = l_bb.get_parent().unwrap();
                let l = self.expr(ctx, a).into_int_value();
                let l_bb = self.b.get_insert_block().unwrap();
                let rhs_bb = self.ctx.append_basic_block(f, "sc.rhs");
                let done_bb = self.ctx.append_basic_block(f, "sc.done");
                if op == AndThen {
                    self.b.build_conditional_branch(l, rhs_bb, done_bb).unwrap();
                } else {
                    self.b.build_conditional_branch(l, done_bb, rhs_bb).unwrap();
                }
                self.b.position_at_end(rhs_bb);
                let r = self.expr(ctx, b).into_int_value();
                self.b.build_unconditional_branch(done_bb).unwrap();
                self.b.position_at_end(done_bb);
                let phi = self.b.build_phi(self.ctx.bool_type(), "sc").unwrap();
                phi.add_incoming(&[(&l, l_bb), (&r, rhs_bb)]);
                return phi.as_basic_value();
            }
            _ => {}
        }
        // STRING =/<> : length byte then code-unit compare within the
        // shorter string (sema rejects ordered compares). Unrolled —
        // caps are small and this keeps the scan path libc-free.
        if matches!(op, Eq | Ne)
            && (matches!(a.ty, Type::Str { .. } | Type::WStr { .. })
                || matches!(b.ty, Type::Str { .. } | Type::WStr { .. }))
        {
            let av = self.expr(ctx, a).into_array_value();
            let bv = self.expr(ctx, b).into_array_value();
            let cap = av.get_type().len().min(bv.get_type().len()) as usize;
            let elem = av.get_type().get_element_type().into_int_type();
            let la = self.b.build_extract_value(av, 0, "la").unwrap().into_int_value();
            let lb = self.b.build_extract_value(bv, 0, "lb").unwrap().into_int_value();
            let mut neq = self
                .b
                .build_int_compare(IntPredicate::NE, la, lb, "lneq")
                .unwrap();
            for i in 1..cap {
                let ca = self.b.build_extract_value(av, i as u32, "ca").unwrap().into_int_value();
                let cb = self.b.build_extract_value(bv, i as u32, "cb").unwrap().into_int_value();
                let d = self.b.build_int_compare(IntPredicate::NE, ca, cb, "cd").unwrap();
                let idx = elem.const_int((i - 1) as u64, false);
                let ina = self.b.build_int_compare(IntPredicate::ULT, idx, la, "ia").unwrap();
                let inb = self.b.build_int_compare(IntPredicate::ULT, idx, lb, "ib").unwrap();
                let act = self.b.build_and(ina, inb, "act").unwrap();
                let m = self.b.build_and(d, act, "m").unwrap();
                neq = self.b.build_or(neq, m, "no").unwrap();
            }
            return if op == Eq {
                self.b.build_not(neq, "seq").unwrap().into()
            } else {
                neq.into()
            };
        }
        let target = if matches!(op, Eq | Ne | Lt | Le | Gt | Ge) {
            Self::cmp_ty(&a.ty, &b.ty)
        } else {
            e_ty.clone()
        };
        let lv = self.expr(ctx, a);
        let l = self.coerce(lv, &a.ty, &target);
        let rv = self.expr(ctx, b);
        let r = self.coerce(rv, &b.ty, &target);
        let isf = l.is_float_value();
        if isf {
            let (l, r) = (l.into_float_value(), r.into_float_value());
            return match op {
                Add => self.b.build_float_add(l, r, "f").unwrap().into(),
                Sub => self.b.build_float_sub(l, r, "f").unwrap().into(),
                Mul => self.b.build_float_mul(l, r, "f").unwrap().into(),
                Div => self.b.build_float_div(l, r, "f").unwrap().into(),
                Mod => self.b.build_float_rem(l, r, "f").unwrap().into(),
                Eq => self.fcmp(FloatPredicate::OEQ, l, r),
                Ne => self.fcmp(FloatPredicate::ONE, l, r),
                Lt => self.fcmp(FloatPredicate::OLT, l, r),
                Le => self.fcmp(FloatPredicate::OLE, l, r),
                Gt => self.fcmp(FloatPredicate::OGT, l, r),
                Ge => self.fcmp(FloatPredicate::OGE, l, r),
                _ => self.b.build_float_add(l, r, "f").unwrap().into(),
            };
        }
        let (l, r) = (l.into_int_value(), r.into_int_value());
        match op {
            Add => self.b.build_int_add(l, r, "i").unwrap().into(),
            Sub => self.b.build_int_sub(l, r, "i").unwrap().into(),
            Mul => self.b.build_int_mul(l, r, "i").unwrap().into(),
            Div | Mod => {
                // div-zero guard → 0 (deterministic fault model is P2)
                let zero = r.get_type().const_zero();
                let dz = self
                    .b
                    .build_int_compare(IntPredicate::EQ, r, zero, "dz")
                    .unwrap();
                let q = if op == Div {
                    self.b.build_int_signed_div(l, r, "d").unwrap()
                } else {
                    self.b.build_int_signed_rem(l, r, "m").unwrap()
                };
                self.b
                    .build_select(dz, zero, q, "sel")
                    .unwrap()
                    .into_int_value()
                    .into()
            }
            Eq => self.icmp(IntPredicate::EQ, l, r),
            Ne => self.icmp(IntPredicate::NE, l, r),
            Lt => self.icmp(IntPredicate::SLT, l, r),
            Le => self.icmp(IntPredicate::SLE, l, r),
            Gt => self.icmp(IntPredicate::SGT, l, r),
            Ge => self.icmp(IntPredicate::SGE, l, r),
            And => self.b.build_and(l, r, "a").unwrap().into(),
            Or => self.b.build_or(l, r, "o").unwrap().into(),
            Xor => self.b.build_xor(l, r, "x").unwrap().into(),
            Shl => self.b.build_left_shift(l, r, "s").unwrap().into(),
            Shr => self.b.build_right_shift(l, r, true, "s").unwrap().into(),
            Rol | Ror => l.into(),
            AndThen | OrElse => unreachable!(),
        }
    }

    fn icmp(
        &mut self,
        p: IntPredicate,
        l: IntValue<'ctx>,
        r: IntValue<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        self.b.build_int_compare(p, l, r, "c").unwrap().into()
    }
    fn fcmp(
        &mut self,
        p: FloatPredicate,
        l: inkwell::values::FloatValue<'ctx>,
        r: inkwell::values::FloatValue<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        self.b.build_float_compare(p, l, r, "c").unwrap().into()
    }

    // ---- calls ----
    fn pou_fn(&mut self, name: &str) -> FunctionValue<'ctx> {
        let fname = format!("__tplc_pou_{name}");
        if let Some(f) = self.module.get_function(&fname) {
            return f;
        }
        let i8p = self.i8p();
        let ft = self
            .ctx
            .i32_type()
            .fn_type(&[i8p.into(), i8p.into()], false);
        self.module
            .add_function(&fname, ft, Some(Linkage::External))
    }

    fn rt_fn(&mut self, name: &str) -> FunctionValue<'ctx> {
        let fname = format!("__tplc_rt_{name}");
        if let Some(f) = self.module.get_function(&fname) {
            return f;
        }
        let i8p = self.i8p();
        let ft = self
            .ctx
            .i32_type()
            .fn_type(&[i8p.into(), i8p.into()], false);
        self.module
            .add_function(&fname, ft, Some(Linkage::External))
    }

    /// check rc after call → propagate nonzero to abort_bb
    fn rc_check(&mut self, cs: inkwell::values::CallSiteValue<'ctx>) {
        let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
        let cont = self.ctx.append_basic_block(f, "rc.cont");
        let rc = cs.try_as_basic_value().expect_basic("rc").into_int_value();
        let bad = self
            .b
            .build_int_compare(IntPredicate::NE, rc, self.ctx.i32_type().const_zero(), "rc")
            .unwrap();
        self.b
            .build_conditional_branch(bad, self.abort_bb.unwrap(), cont)
            .unwrap();
        self.b.position_at_end(cont);
    }

    fn fb_call(&mut self, ctx: &Ctx, inst: &Ref, args: &[(String, Expr)]) {
        let (instp, ty) = self.resolve(ctx, inst);
        let fbname = match &ty {
            Type::Fb { name } => name.clone(),
            _ => panic!("call on non-FB"),
        };
        let fbl = self.member_of(&fbname);
        // bind args into instance fields
        for (pname, pe) in args {
            if let Some((moff, mty)) = fbl.members.get(pname.as_str()).cloned() {
                let v = self.expr(ctx, pe);
                let p = self.off_ptr(instp, moff);
                let v = if is_bool(&mty) {
                    self.b
                        .build_int_z_extend(v.into_int_value(), self.ctx.i8_type(), "z")
                        .unwrap()
                        .into()
                } else {
                    v
                };
                self.b.build_store(p, v).unwrap();
            }
        }
        // rt builtins take (this, ctx); generated pou fns take (ctx, this)
        let (callee, a0, a1) = if builtin_fb_layout(&fbname).is_some() {
            (self.rt_fn(&fbname), instp, self.ctx_ptr)
        } else {
            (self.pou_fn(&fbname), self.ctx_ptr, instp)
        };
        let cs = self
            .b
            .build_call(callee, &[a0.into(), a1.into()], "c")
            .unwrap();
        self.rc_check(cs);
    }

    /// FUNCTION call: caller-side scratch alloca, bind inputs, invoke,
    /// read the name-var back.
    /// byte-indexed element pointer over a typed array pointer
    fn elem_ptr(&mut self, base: PointerValue<'ctx>,
                idx: inkwell::values::IntValue<'ctx>, esz: u64) -> PointerValue<'ctx> {
        let i64t = self.ctx.i64_type();
        let idx64 = if idx.get_type().get_bit_width() < 64 {
            self.b.build_int_z_extend(idx, i64t, "i64").unwrap()
        } else {
            idx
        };
        let bo = self
            .b
            .build_int_mul(idx64, i64t.const_int(esz, false), "bo")
            .unwrap();
        let base8 = self.b.build_pointer_cast(base, self.i8p(), "b8").unwrap();
        unsafe {
            self.b
                .build_gep(self.ctx.i8_type(), base8, &[bo], "ep")
                .unwrap()
        }
    }

    /// extend/truncate an int value to i32 (position/length maths)
    fn as_i32(&mut self, v: inkwell::values::IntValue<'ctx>)
        -> inkwell::values::IntValue<'ctx> {
        let w = v.get_type().get_bit_width();
        let i32t = self.ctx.i32_type();
        if w < 32 {
            self.b.build_int_z_extend(v, i32t, "e32").unwrap()
        } else if w > 32 {
            self.b.build_int_truncate(v, i32t, "t32").unwrap()
        } else {
            v
        }
    }

    /// clamp `v` into [lo, hi] (all i32)
    fn clamp32(&mut self, v: inkwell::values::IntValue<'ctx>,
               lo: i64, hi: inkwell::values::IntValue<'ctx>)
        -> inkwell::values::IntValue<'ctx> {
        let i32t = self.ctx.i32_type();
        let lov = i32t.const_int(lo as u64, true);
        let below = self.b.build_int_compare(IntPredicate::SLT, v, lov, "cb").unwrap();
        let v = self.b.build_select(below, lov, v, "cl").unwrap().into_int_value();
        let above = self.b.build_int_compare(IntPredicate::SGT, v, hi, "ca").unwrap();
        self.b.build_select(above, hi, v, "ch").unwrap().into_int_value()
    }

    /// one runtime-bound unit copy: dst[dst_idx..] <- src[src_idx..]
    fn copy_units(&mut self, dst: PointerValue<'ctx>,
                  dst_idx: inkwell::values::IntValue<'ctx>,
                  src: PointerValue<'ctx>,
                  src_idx: inkwell::values::IntValue<'ctx>,
                  count: inkwell::values::IntValue<'ctx>,
                  elem: inkwell::types::IntType<'ctx>, esz: u64,
                  name: &str) {
        let i32t = self.ctx.i32_type();
        let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
        let pre = self.b.get_insert_block().unwrap();
        let cond_bb = self.ctx.append_basic_block(f, "cp.cond");
        let body_bb = self.ctx.append_basic_block(f, "cp.body");
        let end_bb = self.ctx.append_basic_block(f, "cp.end");
        self.b.build_unconditional_branch(cond_bb).unwrap();
        self.b.position_at_end(cond_bb);
        let i = self.b.build_phi(i32t, "i").unwrap();
        i.add_incoming(&[(&i32t.const_zero(), pre)]);
        let iv = i.as_basic_value().into_int_value();
        let done = self.b.build_int_compare(IntPredicate::SGE, iv, count, "cd").unwrap();
        self.b.build_conditional_branch(done, end_bb, body_bb).unwrap();
        self.b.position_at_end(body_bb);
        let di = self.b.build_int_add(dst_idx, iv, "di").unwrap();
        let si = self.b.build_int_add(src_idx, iv, "si").unwrap();
        let dp = self.elem_ptr(dst, di, esz);
        let sp = self.elem_ptr(src, si, esz);
        let v = self.b.build_load(elem, sp, name).unwrap();
        self.b.build_store(dp, v).unwrap();
        let next = self.b.build_int_add(iv, i32t.const_int(1, false), "in").unwrap();
        i.add_incoming(&[(&next, body_bb)]);
        self.b.build_unconditional_branch(cond_bb).unwrap();
        self.b.position_at_end(end_bb);
    }

    /// LEFT/RIGHT/MID/INSERT/DELETE/REPLACE/FIND on narrow and wide
    /// strings. Positions are 1-based (IEC), clamped; L<=0 is empty.
    fn str_slice_call(&mut self, ctx: &Ctx, b: tplc_mir::ty::BuiltinFn,
                      args: &[(String, Expr)], ret_ty: &Type)
        -> BasicValueEnum<'ctx> {
        use tplc_mir::ty::BuiltinFn;
        let i32t = self.ctx.i32_type();
        let av = self.expr(ctx, &args[0].1).into_array_value();
        let aty = av.get_type();
        let elem = aty.get_element_type().into_int_type();
        let esz = (elem.get_bit_width() / 8) as u64;
        let pa = self.b.build_alloca(aty, "sa").unwrap();
        self.b.build_store(pa, av).unwrap();
        let la = self.b.build_load(elem, pa, "la").unwrap().into_int_value();
        let la32 = self.as_i32(la);
        let zero = i32t.const_zero();
        let one = i32t.const_int(1, false);

        let arg_i32 = |g: &mut Self, i: usize| {
            let v = g.expr(ctx, &args[i].1).into_int_value();
            g.as_i32(v)
        };

        if b == BuiltinFn::Find {
            let bv = self.expr(ctx, &args[1].1).into_array_value();
            let bty = bv.get_type();
            let pb = self.b.build_alloca(bty, "sb").unwrap();
            self.b.build_store(pb, bv).unwrap();
            let lb = self.b.build_load(elem, pb, "lb").unwrap().into_int_value();
            let lb32 = self.as_i32(lb);
            /* result starts at 0 (not found); outer scan i in 1..=la-lb+1,
               inner compare of lb units with an early exit             */
            let found = self.b.build_alloca(i32t, "fd").unwrap();
            self.b.build_store(found, zero).unwrap();
            let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
            let ocond = self.ctx.append_basic_block(f, "fd.oc");
            let obody = self.ctx.append_basic_block(f, "fd.ob");
            let oend = self.ctx.append_basic_block(f, "fd.oe");
            let pre = self.b.get_insert_block().unwrap();
            self.b.build_unconditional_branch(ocond).unwrap();
            self.b.position_at_end(ocond);
            let i = self.b.build_phi(i32t, "i").unwrap();
            i.add_incoming(&[(&one, pre)]);
            let iv = i.as_basic_value().into_int_value();
            let last = self.b.build_int_sub(la32, lb32, "lm").unwrap();
            let last = self.b.build_int_add(last, one, "lmp").unwrap();
            let past = self.b.build_int_compare(IntPredicate::SGT, iv, last, "fp").unwrap();
            let empty = self.b.build_int_compare(IntPredicate::SLE, lb32, zero, "fe").unwrap();
            let stop = self.b.build_or(past, empty, "fs").unwrap();
            self.b.build_conditional_branch(stop, oend, obody).unwrap();
            self.b.position_at_end(obody);
            let jcond = self.ctx.append_basic_block(f, "fd.jc");
            let jbody = self.ctx.append_basic_block(f, "fd.jb");
            let jnext = self.ctx.append_basic_block(f, "fd.jn");
            let ok = self.b.build_alloca(self.ctx.bool_type(), "ok").unwrap();
            self.b.build_store(ok, self.ctx.bool_type().const_all_ones()).unwrap();
            self.b.build_unconditional_branch(jcond).unwrap();
            self.b.position_at_end(jcond);
            let j = self.b.build_phi(i32t, "j").unwrap();
            j.add_incoming(&[(&one, obody)]);
            let jv = j.as_basic_value().into_int_value();
            let jdone = self.b.build_int_compare(IntPredicate::SGT, jv, lb32, "jd").unwrap();
            let cur_ok = self.b.build_load(self.ctx.bool_type(), ok, "ol").unwrap().into_int_value();
            let quit = self.b.build_or(jdone, self.b.build_not(cur_ok, "on").unwrap(), "q").unwrap();
            self.b.build_conditional_branch(quit, jnext, jbody).unwrap();
            self.b.position_at_end(jbody);
            let si = self.b.build_int_add(iv, jv, "si").unwrap();
            let si = self.b.build_int_sub(si, one, "si1").unwrap();
            let sp = self.elem_ptr(pa, si, esz);
            let dp = self.elem_ptr(pb, jv, esz);
            let va = self.b.build_load(elem, sp, "va").unwrap().into_int_value();
            let vb = self.b.build_load(elem, dp, "vb").unwrap().into_int_value();
            let eq = self.b.build_int_compare(IntPredicate::EQ, va, vb, "eq").unwrap();
            self.b.build_store(ok, eq).unwrap();
            let jn = self.b.build_int_add(jv, one, "jn").unwrap();
            j.add_incoming(&[(&jn, jbody)]);
            self.b.build_unconditional_branch(jcond).unwrap();
            self.b.position_at_end(jnext);
            let fin = self.b.build_load(self.ctx.bool_type(), ok, "fl").unwrap().into_int_value();
            let hit = self.ctx.append_basic_block(f, "fd.hit");
            let miss = self.ctx.append_basic_block(f, "fd.miss");
            self.b.build_conditional_branch(fin, hit, miss).unwrap();
            self.b.position_at_end(hit);
            self.b.build_store(found, iv).unwrap();
            self.b.build_unconditional_branch(oend).unwrap();
            self.b.position_at_end(miss);
            let inext = self.b.build_int_add(iv, one, "inx").unwrap();
            i.add_incoming(&[(&inext, miss)]);
            self.b.build_unconditional_branch(ocond).unwrap();
            self.b.position_at_end(oend);
            let res = self.b.build_load(i32t, found, "fr").unwrap().into_int_value();
            let want = llty(self.ctx, ret_ty).into_int_type();
            return if want.get_bit_width() < 32 {
                self.b.build_int_truncate(res, want, "frt").unwrap().into()
            } else {
                res.into()
            };
        }

        let cap = match ret_ty {
            Type::Str { cap } | Type::WStr { cap } => *cap,
            _ => 0,
        } as u32;
        let want = 1 + cap;
        let pr = self.b.build_alloca(elem.array_type(want), "sr").unwrap();
        let p0 = self.b.build_pointer_cast(pr, self.i8p(), "p0").unwrap();
        self.b.build_store(p0, elem.const_zero()).unwrap();
        let cap32 = i32t.const_int(cap as u64, false);

        let total: inkwell::values::IntValue<'ctx> = match b {
            BuiltinFn::Left => {
                let l = arg_i32(self, 1);
                let n = self.clamp32(l, 0, la32);
                self.copy_units(pr, one, pa, one, n, elem, esz, "u");
                n
            }
            BuiltinFn::Right => {
                let l = arg_i32(self, 1);
                let n = self.clamp32(l, 0, la32);
                let si = self.b.build_int_sub(la32, n, "rs").unwrap();
                let si = self.b.build_int_add(si, one, "rs1").unwrap();
                self.copy_units(pr, one, pa, si, n, elem, esz, "u");
                n
            }
            BuiltinFn::Mid => {
                let l = arg_i32(self, 1);
                let p = arg_i32(self, 2);
                let start0 = self.clamp32(
                    self.b.build_int_sub(p, one, "m0").unwrap(), 0, la32);
                let avail = self.b.build_int_sub(la32, start0, "ma").unwrap();
                let n = self.clamp32(l, 0, avail);
                let si = self.b.build_int_add(start0, one, "ms").unwrap();
                self.copy_units(pr, one, pa, si, n, elem, esz, "u");
                n
            }
            BuiltinFn::Insert | BuiltinFn::Replace => {
                let bv = self.expr(ctx, &args[1].1).into_array_value();
                let bty = bv.get_type();
                let pb = self.b.build_alloca(bty, "sb").unwrap();
                self.b.build_store(pb, bv).unwrap();
                let lb = self.b.build_load(elem, pb, "lb").unwrap().into_int_value();
                let lb32 = self.as_i32(lb);
                let (d, p_arg) = if b == BuiltinFn::Insert {
                    (zero, 2usize)
                } else {
                    (arg_i32(self, 2), 3usize)
                };
                let p = arg_i32(self, p_arg);
                let last = self.b.build_int_add(la32, one, "lp").unwrap();
                let p32 = self.clamp32(p, 1, last);
                let start0 = self.b.build_int_sub(p32, one, "s0").unwrap();
                let avail = self.b.build_int_sub(la32, start0, "av").unwrap();
                let dcl = self.clamp32(d, 0, avail);
                let tail = self.b.build_int_sub(avail, dcl, "tl").unwrap();
                self.copy_units(pr, one, pa, one, start0, elem, esz, "p1");
                self.copy_units(pr, p32, pb, one, lb32, elem, esz, "p2");
                let si = self.b.build_int_add(p32, dcl, "ps").unwrap();
                let di = self.b.build_int_add(start0, lb32, "pd").unwrap();
                let di = self.b.build_int_add(di, one, "pd1").unwrap();
                self.copy_units(pr, di, pa, si, tail, elem, esz, "p3");
                let t = self.b.build_int_add(start0, lb32, "t1").unwrap();
                self.b.build_int_add(t, tail, "t2").unwrap()
            }
            BuiltinFn::Delete => {
                let l = arg_i32(self, 1);
                let p = arg_i32(self, 2);
                let last = self.b.build_int_add(la32, one, "lp").unwrap();
                let p32 = self.clamp32(p, 1, last);
                let start0 = self.b.build_int_sub(p32, one, "s0").unwrap();
                let avail = self.b.build_int_sub(la32, start0, "av").unwrap();
                let dcl = self.clamp32(l, 0, avail);
                let tail = self.b.build_int_sub(avail, dcl, "tl").unwrap();
                self.copy_units(pr, one, pa, one, start0, elem, esz, "p1");
                let si = self.b.build_int_add(p32, dcl, "ps").unwrap();
                let di = self.b.build_int_add(start0, one, "pd").unwrap();
                self.copy_units(pr, di, pa, si, tail, elem, esz, "p2");
                self.b.build_int_add(start0, tail, "t").unwrap()
            }
            _ => unreachable!(),
        };
        /* result length: clamped to the target cap */
        let over = self.b.build_int_compare(IntPredicate::SGT, total, cap32, "ov").unwrap();
        let tcl = self.b.build_select(over, cap32, total, "tc").unwrap().into_int_value();
        let tel = if elem.get_bit_width() < 32 {
            self.b.build_int_truncate(tcl, elem, "te").unwrap()
        } else {
            tcl
        };
        self.b.build_store(p0, tel).unwrap();
        self.b
            .build_load(elem.array_type(want), pr, "sl")
            .unwrap()
            .into()
    }

    /// LEN/CONCAT over narrow and wide strings (IEC string functions)
    fn builtin_call(&mut self, ctx: &Ctx, b: tplc_mir::ty::BuiltinFn,
                    args: &[(String, Expr)], ret_ty: &Type) -> BasicValueEnum<'ctx> {
        use tplc_mir::ty::BuiltinFn;
        match b {
            BuiltinFn::Left | BuiltinFn::Right | BuiltinFn::Mid
            | BuiltinFn::Find | BuiltinFn::Insert | BuiltinFn::Delete
            | BuiltinFn::Replace => self.str_slice_call(ctx, b, args, ret_ty),
            BuiltinFn::Len => {
                let av = self.expr(ctx, &args[0].1).into_array_value();
                let len = self
                    .b
                    .build_extract_value(av, 0, "len")
                    .unwrap()
                    .into_int_value();
                let want = llty(self.ctx, ret_ty).into_int_type();
                let (lb, wb) = (len.get_type().get_bit_width(), want.get_bit_width());
                if lb < wb {
                    self.b.build_int_z_extend(len, want, "lenz").unwrap().into()
                } else if lb > wb {
                    self.b.build_int_truncate(len, want, "lent").unwrap().into()
                } else {
                    len.into()
                }
            }
            BuiltinFn::Abs => {
                let v = self.expr(ctx, &args[0].1);
                if v.is_float_value() {
                    let f = v.into_float_value();
                    let neg = self.b.build_float_neg(f, "an").unwrap();
                    let lt = self
                        .b
                        .build_float_compare(
                            inkwell::FloatPredicate::OLT,
                            f,
                            self.ctx.f64_type().const_zero(),
                            "ac",
                        )
                        .unwrap();
                    // compare in the value's own width
                    let lt = if f.get_type() == self.ctx.f32_type() {
                        self.b
                            .build_float_compare(
                                inkwell::FloatPredicate::OLT,
                                f,
                                self.ctx.f32_type().const_zero(),
                                "ac",
                            )
                            .unwrap()
                    } else {
                        lt
                    };
                    self.b.build_select(lt, neg, f, "abs").unwrap()
                } else {
                    let i = v.into_int_value();
                    let neg = self.b.build_int_neg(i, "an").unwrap();
                    let lt = self
                        .b
                        .build_int_compare(
                            IntPredicate::SLT,
                            i,
                            i.get_type().const_zero(),
                            "ac",
                        )
                        .unwrap();
                    self.b.build_select(lt, neg, i, "abs").unwrap()
                }
            }
            BuiltinFn::Min | BuiltinFn::Max => {
                let a = self.expr(ctx, &args[0].1);
                let b2 = self.expr(ctx, &args[1].1);
                let is_min = b == BuiltinFn::Min;
                let pick = if a.is_float_value() {
                    let (x, y) = (a.into_float_value(), b2.into_float_value());
                    let p = if is_min {
                        inkwell::FloatPredicate::OLE
                    } else {
                        inkwell::FloatPredicate::OGE
                    };
                    self.b.build_float_compare(p, x, y, "cmp").unwrap()
                } else {
                    let (x, y) = (a.into_int_value(), b2.into_int_value());
                    let signed = matches!(ret_ty, Type::Int { signed: true, .. });
                    let p = match (is_min, signed) {
                        (true, true) => IntPredicate::SLE,
                        (true, false) => IntPredicate::ULE,
                        (false, true) => IntPredicate::SGE,
                        (false, false) => IntPredicate::UGE,
                    };
                    self.b.build_int_compare(p, x, y, "cmp").unwrap()
                };
                self.b.build_select(pick, a, b2, "mm").unwrap()
            }
            BuiltinFn::Sel => {
                let g = self.expr(ctx, &args[0].1);
                let a = self.expr(ctx, &args[1].1);
                let b2 = self.expr(ctx, &args[2].1);
                self.b.build_select(g.into_int_value(), b2, a, "sel").unwrap()
            }
            BuiltinFn::Limit => {
                let mn = self.expr(ctx, &args[0].1);
                let v = self.expr(ctx, &args[1].1);
                let mx = self.expr(ctx, &args[2].1);
                if v.is_float_value() {
                    let (lo, x, hi) = (mn.into_float_value(), v.into_float_value(),
                                       mx.into_float_value());
                    let below = self.b
                        .build_float_compare(inkwell::FloatPredicate::OLT, x, lo, "lb")
                        .unwrap();
                    let above = self.b
                        .build_float_compare(inkwell::FloatPredicate::OGT, x, hi, "la")
                        .unwrap();
                    let lo_sel = self.b
                        .build_select(below, lo, x, "ll")
                        .unwrap()
                        .into_float_value();
                    self.b.build_select(above, hi, lo_sel, "lim").unwrap()
                } else {
                    let (lo, x, hi) = (mn.into_int_value(), v.into_int_value(),
                                       mx.into_int_value());
                    let signed = matches!(ret_ty, Type::Int { signed: true, .. });
                    let (bl, ab) = if signed {
                        (IntPredicate::SLT, IntPredicate::SGT)
                    } else {
                        (IntPredicate::ULT, IntPredicate::UGT)
                    };
                    let below = self.b.build_int_compare(bl, x, lo, "lb").unwrap();
                    let above = self.b.build_int_compare(ab, x, hi, "la").unwrap();
                    let lo_sel = self.b
                        .build_select(below, lo, x, "ll")
                        .unwrap()
                        .into_int_value();
                    self.b.build_select(above, hi, lo_sel, "lim").unwrap()
                }
            }
            BuiltinFn::IsValidRef => {
                let v = self.expr(ctx, &args[0].1).into_int_value();
                let null = self
                    .b
                    .build_int_compare(
                        IntPredicate::EQ,
                        v,
                        v.get_type().const_int(u64::MAX, false),
                        "iv",
                    )
                    .unwrap();
                self.b.build_not(null, "ivn").unwrap().into()
            }
            BuiltinFn::Trunc => {
                /* sema rewrites TRUNC into a real→int Conv; a direct
                   MIR use still lands here (fptosi truncates) */
                let v = self.expr(ctx, &args[0].1);
                let want = llty(self.ctx, ret_ty).into_int_type();
                self.b
                    .build_float_to_signed_int(v.into_float_value(), want, "tr")
                    .unwrap()
                    .into()
            }
            BuiltinFn::Round => {
                /* round half away from zero: fptosi(x ± 0.5) */
                let f = self.expr(ctx, &args[0].1).into_float_value();
                let f64t = self.ctx.f64_type();
                let v = if f.get_type() == self.ctx.f32_type() {
                    self.b
                        .build_float_ext(f, f64t, "rw")
                        .unwrap()
                } else {
                    f
                };
                let pos = self
                    .b
                    .build_float_compare(
                        inkwell::FloatPredicate::OGE,
                        v,
                        f64t.const_zero(),
                        "rp",
                    )
                    .unwrap();
                let half = self
                    .b
                    .build_select(
                        pos,
                        f64t.const_float(0.5),
                        f64t.const_float(-0.5),
                        "rh",
                    )
                    .unwrap()
                    .into_float_value();
                let adj = self.b.build_float_add(v, half, "ra").unwrap();
                let want = llty(self.ctx, ret_ty).into_int_type();
                self.b
                    .build_float_to_signed_int(adj, want, "rr")
                    .unwrap()
                    .into()
            }
            BuiltinFn::Mux => {
                let k = self.expr(ctx, &args[0].1).into_int_value();
                let mut r = self.expr(ctx, &args[1].1);
                for i in 1..args.len() - 1 {
                    let cand = self.expr(ctx, &args[i + 1].1);
                    let cond = self
                        .b
                        .build_int_compare(
                            IntPredicate::EQ,
                            k,
                            k.get_type().const_int(i as u64, false),
                            "mc",
                        )
                        .unwrap();
                    r = self.b.build_select(cond, cand, r, "ms").unwrap();
                }
                r
            }
            BuiltinFn::Concat => {
                let av = self.expr(ctx, &args[0].1).into_array_value();
                let bv = self.expr(ctx, &args[1].1).into_array_value();
                let elem = av.get_type().get_element_type().into_int_type();
                let esz = (elem.get_bit_width() / 8) as u64;
                let cap = match ret_ty {
                    Type::Str { cap } | Type::WStr { cap } => *cap,
                    _ => 0,
                } as u32;
                let want = 1 + cap;
                let aty = av.get_type();
                let bty = bv.get_type();
                let pa = self.b.build_alloca(aty, "pa").unwrap();
                self.b.build_store(pa, av).unwrap();
                let pb = self.b.build_alloca(bty, "pb").unwrap();
                self.b.build_store(pb, bv).unwrap();
                let pr = self.b.build_alloca(elem.array_type(want), "pr").unwrap();

                /* a's whole array (len byte + data) lands first — fixed
                   size — then b's units shift in after a's live length */
                let bytes = aty.len() as u64 * esz;
                let dst8 = self.b.build_pointer_cast(pr, self.i8p(), "d8").unwrap();
                let src8 = self.b.build_pointer_cast(pa, self.i8p(), "s8").unwrap();
                self.b
                    .build_memcpy(dst8, 1, src8, 1,
                                  self.ctx.i64_type().const_int(bytes, false))
                    .unwrap();

                let la = self.b.build_load(elem, pa, "la").unwrap().into_int_value();
                let lb = self.b.build_load(elem, pb, "lb").unwrap().into_int_value();
                let sum = self.b.build_int_add(la, lb, "sum").unwrap();
                let capv = elem.const_int(cap as u64, false);
                let over = self
                    .b
                    .build_int_compare(IntPredicate::UGT, sum, capv, "ov")
                    .unwrap();
                let total = self
                    .b
                    .build_select(over, capv, sum, "tot")
                    .unwrap()
                    .into_int_value();

                /* copy b[1..=lb] to pr[la+1 ..] */
                let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                let pre = self.b.get_insert_block().unwrap();
                let cond_bb = self.ctx.append_basic_block(f, "cc.cond");
                let body_bb = self.ctx.append_basic_block(f, "cc.body");
                let end_bb = self.ctx.append_basic_block(f, "cc.end");
                self.b.build_unconditional_branch(cond_bb).unwrap();
                self.b.position_at_end(cond_bb);
                let j = self.b.build_phi(elem, "j").unwrap();
                j.add_incoming(&[(&elem.const_int(1, false), pre)]);
                let jv = j.as_basic_value().into_int_value();
                let jc = self
                    .b
                    .build_int_compare(IntPredicate::ULE, jv, lb, "jc")
                    .unwrap();
                self.b.build_conditional_branch(jc, body_bb, end_bb).unwrap();
                self.b.position_at_end(body_bb);
                let di = self.b.build_int_add(la, jv, "di").unwrap();
                let dp = self.elem_ptr(pr, di, esz);
                let sp = self.elem_ptr(pb, jv, esz);
                let v = self.b.build_load(elem, sp, "v").unwrap();
                self.b.build_store(dp, v).unwrap();
                let jn = self
                    .b
                    .build_int_add(jv, elem.const_int(1, false), "jn")
                    .unwrap();
                j.add_incoming(&[(&jn, body_bb)]);
                self.b.build_unconditional_branch(cond_bb).unwrap();
                self.b.position_at_end(end_bb);

                let p0 = self.b.build_pointer_cast(pr, self.i8p(), "p0").unwrap();
                self.b.build_store(p0, total).unwrap();
                self.b
                    .build_load(elem.array_type(want), pr, "cat")
                    .unwrap()
                    .into()
            }
        }
    }

    fn fn_call(&mut self, ctx: &Ctx, fun: &str, args: &[(String, Expr)],
               ret_ty: &Type) -> BasicValueEnum<'ctx> {
        if let Some(b) = tplc_mir::ty::builtin_fn(fun) {
            return self.builtin_call(ctx, b, args, ret_ty);
        }
        let pou = self
            .mir
            .pous
            .iter()
            .find(|p| p.name == fun && p.kind == PouKind::Function)
            .unwrap_or_else(|| panic!("no FUNCTION {fun}"));
        let fbl = tplc_mir::layout::layout_members(&pou.iface, BTreeMap::new());
        let scratch = self
            .b
            .build_alloca(self.ctx.i8_type().array_type(fbl.size as u32), "fs")
            .unwrap();
        let inputs: Vec<&VarDecl> = pou
            .iface
            .iter()
            .filter(|v| matches!(v.kind, VarKind::Input | VarKind::InOut))
            .collect();
        for (i, (pname, pe)) in args.iter().enumerate() {
            let name = if pname.is_empty() {
                inputs.get(i).map(|v| v.name.as_str()).unwrap_or("")
            } else {
                pname.as_str()
            };
            if let Some((moff, _mty)) = fbl.members.get(name).cloned() {
                let v = self.expr(ctx, pe);
                let p = self.off_ptr(scratch, moff);
                self.b.build_store(p, v).unwrap();
            }
        }
        let callee = self.pou_fn(fun);
        let cs = self
            .b
            .build_call(callee, &[self.ctx_ptr.into(), scratch.into()], "fc")
            .unwrap();
        self.rc_check(cs);
        // return var = member named pou.name
        let (roff, rty) = fbl.members[&pou.name].clone();
        let rp = self.off_ptr(scratch, roff);
        self.b.build_load(llty(self.ctx, &rty), rp, "ret").unwrap()
    }

    /// store a literal init value at blob+off
    fn lit_store(&mut self, blob: PointerValue<'ctx>, off: u64, ty: &Type, init: &Expr) {
        let v = self.expr(&Ctx::new(), init); // Lit → const, ctx unused
        let p = self.off_ptr(blob, off);
        let v = if is_bool(ty) {
            self.b
                .build_int_z_extend(v.into_int_value(), self.ctx.i8_type(), "z")
                .unwrap()
                .into()
        } else {
            v
        };
        self.b.build_store(p, v).unwrap();
    }

    // ---- statements ----
    /// __ckpt (doc 04 §2.2): abort_flag → TPLC_TASK_E_ABORTED;
    /// armed budget_deadline → svc->steady_ns() compare →
    /// TPLC_TASK_E_OVERRUN. deadline==0 = unbounded (no clock call).
    fn abort_check(&mut self) {
        let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
        let cont = self.ctx.append_basic_block(f, "ck.cont");
        let dlck = self.ctx.append_basic_block(f, "ck.dl");
        let af = {
            let p = self.off_ptr(self.ctx_ptr, CTX_ABORT);
            let l = self.b.build_load(self.ctx.i32_type(), p, "af").unwrap();
            l.as_instruction_value()
                .unwrap()
                .set_volatile(true)
                .unwrap();
            l.into_int_value()
        };
        let bad = self
            .b
            .build_int_compare(IntPredicate::NE, af, self.ctx.i32_type().const_zero(), "ck")
            .unwrap();
        self.b
            .build_conditional_branch(bad, self.abort_bb.unwrap(), dlck)
            .unwrap();
        // deadline armed? then read the host clock once
        self.b.position_at_end(dlck);
        let dl = self
            .ctx_load(CTX_DEADLINE, self.ctx.i64_type().into(), "dl")
            .into_int_value();
        let armed = self
            .b
            .build_int_compare(
                IntPredicate::NE,
                dl,
                self.ctx.i64_type().const_zero(),
                "arm",
            )
            .unwrap();
        let clock_bb = self.ctx.append_basic_block(f, "ck.clk");
        self.b
            .build_conditional_branch(armed, clock_bb, cont)
            .unwrap();
        self.b.position_at_end(clock_bb);
        let now = self.steady_ns();
        let over = self
            .b
            .build_int_compare(IntPredicate::UGT, now, dl, "ov")
            .unwrap();
        self.b
            .build_conditional_branch(over, self.overrun_bb.unwrap(), cont)
            .unwrap();
        self.b.position_at_end(cont);
    }

    /// svc->steady_ns(svc->host) — TplcServices offsets: steady_ns@24,
    /// host cookie@56; TplcCtx.svc@72.
    fn steady_ns(&mut self) -> IntValue<'ctx> {
        let i8p = self.ctx.ptr_type(inkwell::AddressSpace::default());
        let svc = self
            .ctx_load(CTX_SVC, i8p.into(), "svc")
            .into_pointer_value();
        let fpp = unsafe {
            self.b
                .build_gep(
                    self.ctx.i8_type(),
                    svc,
                    &[self.ctx.i64_type().const_int(24, false)],
                    "sfp",
                )
                .unwrap()
        };
        let fp = self
            .b
            .build_load(i8p, fpp, "fp")
            .unwrap()
            .into_pointer_value();
        let hp = unsafe {
            self.b
                .build_gep(
                    self.ctx.i8_type(),
                    svc,
                    &[self.ctx.i64_type().const_int(56, false)],
                    "hp",
                )
                .unwrap()
        };
        let host = self.b.build_load(i8p, hp, "h").unwrap();
        let ft = self.ctx.i64_type().fn_type(&[i8p.into()], false);
        let call = self
            .b
            .build_indirect_call(ft, fp, &[host.into()], "now")
            .unwrap();
        call.try_as_basic_value()
            .expect_basic("steady_ns returns i64")
            .into_int_value()
    }

    /// close the current block with `br target` unless a previous
    /// statement already terminated it (Return/Exit/Continue)
    fn term_br(&mut self, target: BasicBlock<'ctx>) {
        if self
            .b
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            self.b.build_unconditional_branch(target).unwrap();
        }
    }

    fn stmt(&mut self, ctx: &Ctx, s: &Stmt) {
        // a previous Return/Exit/Continue left the block terminated —
        // resume in a fresh dead block so trailing statements stay
        // well-formed IR (unreachable, eliminated later)
        if self
            .b
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_some()
        {
            let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
            let dead = self.ctx.append_basic_block(f, "dead");
            self.b.position_at_end(dead);
        }
        match &s.kind {
            StmtKind::Assign { lhs, rhs } => {
                let v = self.expr(ctx, rhs);
                self.write(ctx, lhs, v);
            }
            StmtKind::If { branches, else_ } => {
                let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                let end_bb = self.ctx.append_basic_block(f, "if.end");
                for (cond, body) in branches.iter() {
                    let then_bb = self.ctx.append_basic_block(f, "if.then");
                    let next_bb = self.ctx.append_basic_block(f, "if.next");
                    let cv = self.expr(ctx, cond).into_int_value();
                    self.b
                        .build_conditional_branch(cv, then_bb, next_bb)
                        .unwrap();
                    self.b.position_at_end(then_bb);
                    for st in body {
                        self.stmt(ctx, st);
                    }
                    self.term_br(end_bb);
                    self.b.position_at_end(next_bb);
                }
                // else block
                for st in else_ {
                    self.stmt(ctx, st);
                }
                self.term_br(end_bb);
                self.b.position_at_end(end_bb);
            }
            StmtKind::While { cond, body } => {
                let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                let cond_bb = self.ctx.append_basic_block(f, "w.cond");
                let body_bb = self.ctx.append_basic_block(f, "w.body");
                let end_bb = self.ctx.append_basic_block(f, "w.end");
                self.b.build_unconditional_branch(cond_bb).unwrap();
                self.b.position_at_end(cond_bb);
                self.abort_check();
                let cv = self.expr(ctx, cond).into_int_value();
                self.b
                    .build_conditional_branch(cv, body_bb, end_bb)
                    .unwrap();
                self.b.position_at_end(body_bb);
                self.loop_stack.push((cond_bb, end_bb));
                for st in body {
                    self.stmt(ctx, st);
                }
                self.loop_stack.pop();
                self.term_br(cond_bb);
                self.b.position_at_end(end_bb);
            }
            StmtKind::Repeat { cond, body } => {
                let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                let body_bb = self.ctx.append_basic_block(f, "r.body");
                let cond_bb = self.ctx.append_basic_block(f, "r.cond");
                let end_bb = self.ctx.append_basic_block(f, "r.end");
                self.b.build_unconditional_branch(body_bb).unwrap();
                self.b.position_at_end(body_bb);
                self.loop_stack.push((cond_bb, end_bb));
                for st in body {
                    self.stmt(ctx, st);
                }
                self.loop_stack.pop();
                self.term_br(cond_bb);
                self.b.position_at_end(cond_bb);
                self.abort_check();
                let cv = self.expr(ctx, cond).into_int_value();
                self.b
                    .build_conditional_branch(cv, end_bb, body_bb)
                    .unwrap();
                self.b.position_at_end(end_bb);
            }
            StmtKind::For {
                var,
                start,
                end,
                step,
                body,
            } => {
                let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                let vr = Ref::Var(var.clone());
                let sv = self.expr(ctx, start);
                self.write(ctx, &vr, sv);
                let vt = self.read(ctx, &vr).into_int_value().get_type();
                // step/end are arbitrary int widths — normalize to the
                // loop variable's type before compare/add
                let step_e = step
                    .as_ref()
                    .map(|e| self.expr(ctx, e).into_int_value())
                    .unwrap_or(vt.const_int(1, true));
                let end_v = self.expr(ctx, end).into_int_value();
                let step_e = self
                    .b
                    .build_int_cast(step_e, vt, "fs")
                    .unwrap();
                let end_v = self
                    .b
                    .build_int_cast(end_v, vt, "fe")
                    .unwrap();
                let cond_bb = self.ctx.append_basic_block(f, "f.cond");
                let body_bb = self.ctx.append_basic_block(f, "f.body");
                let step_bb = self.ctx.append_basic_block(f, "f.step");
                let end_bb = self.ctx.append_basic_block(f, "f.end");
                self.b.build_unconditional_branch(cond_bb).unwrap();
                self.b.position_at_end(cond_bb);
                self.abort_check();
                let cur = self.read(ctx, &vr).into_int_value();
                // v1: positive step → i <= end
                let cmp = self
                    .b
                    .build_int_compare(IntPredicate::SLE, cur, end_v, "fc")
                    .unwrap();
                self.b
                    .build_conditional_branch(cmp, body_bb, end_bb)
                    .unwrap();
                self.b.position_at_end(body_bb);
                self.loop_stack.push((step_bb, end_bb));
                for st in body {
                    self.stmt(ctx, st);
                }
                self.loop_stack.pop();
                self.term_br(step_bb);
                self.b.position_at_end(step_bb);
                let cur = self.read(ctx, &vr).into_int_value();
                let nxt = self.b.build_int_add(cur, step_e, "fn").unwrap();
                let (p, _) = self.resolve(ctx, &vr);
                self.b.build_store(p, nxt).unwrap();
                self.b.build_unconditional_branch(cond_bb).unwrap();
                self.b.position_at_end(end_bb);
            }
            StmtKind::Case { sel, arms, else_ } => {
                // desugar to if-chain (v1)
                let sv = self.expr(ctx, sel);
                let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                let end_bb = self.ctx.append_basic_block(f, "c.end");
                for (sels, body) in arms {
                    let then_bb = self.ctx.append_basic_block(f, "c.arm");
                    let next_bb = self.ctx.append_basic_block(f, "c.next");
                    let mut acc: Option<IntValue> = None;
                    for cs in sels {
                        let hit = match cs {
                            CaseSel::Lit(e) => {
                                let lv = self.expr(ctx, e);
                                self.b
                                    .build_int_compare(
                                        IntPredicate::EQ,
                                        sv.into_int_value(),
                                        lv.into_int_value(),
                                        "cm",
                                    )
                                    .unwrap()
                            }
                            CaseSel::Range(a, b) => {
                                let lo = self.expr(ctx, a).into_int_value();
                                let hi = self.expr(ctx, b).into_int_value();
                                let s = sv.into_int_value();
                                let ge = self
                                    .b
                                    .build_int_compare(IntPredicate::SGE, s, lo, "g")
                                    .unwrap();
                                let le = self
                                    .b
                                    .build_int_compare(IntPredicate::SLE, s, hi, "l")
                                    .unwrap();
                                self.b.build_and(ge, le, "r").unwrap()
                            }
                        };
                        acc = Some(match acc {
                            None => hit,
                            Some(a) => self.b.build_or(a, hit, "co").unwrap(),
                        });
                    }
                    self.b
                        .build_conditional_branch(acc.unwrap(), then_bb, next_bb)
                        .unwrap();
                    self.b.position_at_end(then_bb);
                    for st in body {
                        self.stmt(ctx, st);
                    }
                    self.term_br(end_bb);
                    self.b.position_at_end(next_bb);
                }
                for st in else_ {
                    self.stmt(ctx, st);
                }
                self.term_br(end_bb);
                self.b.position_at_end(end_bb);
            }
            StmtKind::Call { inst, args } => self.fb_call(ctx, inst, args),
            StmtKind::Return(_) => {
                self.b
                    .build_unconditional_branch(
                        self.abort_bb.unwrap_or_else(|| panic!("return outside fn")),
                    )
                    .unwrap();
            }
            StmtKind::Exit => {
                let (_, end) = *self.loop_stack.last().expect("EXIT outside loop");
                self.b.build_unconditional_branch(end).unwrap();
            }
            StmtKind::Continue => {
                let (cond, _) = *self.loop_stack.last().expect("CONTINUE outside loop");
                self.b.build_unconditional_branch(cond).unwrap();
            }
        }
    }

    fn stmts(&mut self, ctx: &Ctx, ss: &[Stmt]) {
        for s in ss {
            self.stmt(ctx, s);
        }
    }

    /// descriptor globals + tplc_get_descriptor + tplc_task_run
    /// dispatcher (task_fns table + bounds check + indirect call).
    #[allow(clippy::too_many_arguments)]
    fn emit_descriptor(
        &mut self,
        _program: &Pou,
        io_binds: &[IoBind],
        program_name: &str,
        uuid: [u8; 16],
        layout_hash: [u8; 32],
        has_migrate: bool,
        task_period_ns: u64,
    ) {
        let lctx = self.ctx;
        let i8p = self.i8p();

        // -- TplcTaskDesc[1]: MAIN cyclic enabled-at-start
        let mut name_b = [0u8; 32];
        name_b[..4].copy_from_slice(b"MAIN");
        let name_init = lctx.const_string(&name_b, false);
        let task_ty = lctx.struct_type(
            &[
                lctx.i8_type().array_type(32).into(), // name[32]
                lctx.i8_type().into(),                // kind
                lctx.i8_type().into(),                // flags
                lctx.i8_type().into(),                // priority
                lctx.i8_type().into(),                // _pad0
                lctx.i64_type().into(),               // period_ns
                lctx.i64_type().into(),               // wcet_hint_ns
                lctx.i32_type().into(),               // entry_index
                lctx.i32_type().into(),               // task_local_size
            ],
            false,
        );
        let task0 = lctx.const_struct(
            &[
                name_init.into(),
                lctx.i8_type().const_int(0, false).into(), // CYCLIC
                lctx.i8_type().const_int(3, false).into(), // ENABLED|WATCHDOGGED
                lctx.i8_type().const_int(10, false).into(),
                lctx.i8_type().const_zero().into(),
                lctx.i64_type().const_int(task_period_ns, false).into(),
                lctx.i64_type().const_zero().into(), // wcet_hint
                lctx.i32_type().const_zero().into(), // entry 0
                lctx.i32_type().const_zero().into(),
            ],
            false,
        );
        let tasks_arr = task_ty.const_array(&[task0]);
        let g_tasks = self
            .module
            .add_global(tasks_arr.get_type(), None, "__tplc_tasks");
        g_tasks.set_initializer(&tasks_arr);
        g_tasks.set_constant(true);
        g_tasks.set_linkage(Linkage::Internal);

        // -- task_fns[1] = { __tplc_task_0 }
        let tfn = self.module.get_function("__tplc_task_0").unwrap();
        let fns_arr = i8p.const_array(&[tfn.as_global_value().as_pointer_value()]);
        let g_fns = self
            .module
            .add_global(fns_arr.get_type(), None, "__tplc_task_fns");
        g_fns.set_initializer(&fns_arr);
        g_fns.set_constant(true);
        g_fns.set_linkage(Linkage::Internal);

        // -- io bindings
        let io_ty = lctx.struct_type(
            &[
                lctx.i32_type().into(), // blob_offset
                lctx.i8_type().into(),  // region
                lctx.i8_type().into(),  // type_code
                lctx.i8_type().into(),  // flags
                lctx.i8_type().into(),  // unit
                lctx.i32_type().into(), // image_offset
                lctx.i32_type().into(), // size
                lctx.i32_type().into(), // _pad
            ],
            false,
        );
        let scope = program_ctx(self.layout);
        let entries: Vec<_> = io_binds
            .iter()
            .filter_map(|b| {
                let (off, ty) = scope.get(b.name.split('.').next_back().unwrap_or(&b.name))?;
                let (region, tc, unit) = io_codes(&b.at, ty);
                Some(self.ctx.const_struct(
                    &[
                        lctx.i32_type().const_int(*off, false).into(),
                        lctx.i8_type().const_int(region as u64, false).into(),
                        lctx.i8_type().const_int(tc as u64, false).into(),
                        lctx.i8_type().const_int((tc == 0) as u64, false).into(), // forceable (bool)
                        lctx.i8_type().const_int(unit as u64, false).into(),
                        lctx.i32_type().const_int(b.at.byte, false).into(),
                        lctx.i32_type().const_int(ty.size(), false).into(),
                        lctx.i32_type().const_zero().into(),
                    ],
                    false,
                ))
            })
            .collect();
        let io_arr = io_ty.const_array(&entries);
        let g_io = self.module.add_global(io_arr.get_type(), None, "__tplc_io");
        g_io.set_initializer(&io_arr);
        g_io.set_constant(true);
        g_io.set_linkage(Linkage::Internal);

        // -- TplcProgramDescriptor
        let mut pname = [0u8; 64];
        let nb = program_name.as_bytes();
        pname[..nb.len().min(64)].copy_from_slice(&nb[..nb.len().min(64)]);
        let desc_ty = lctx.struct_type(
            &[
                lctx.i32_type().into(),               // struct_size
                lctx.i32_type().into(),               // abi_version
                lctx.i32_type().into(),               // abi_ext
                lctx.i32_type().into(),               // flags
                lctx.i8_type().array_type(16).into(), // program_uuid
                lctx.i8_type().array_type(32).into(), // layout_hash
                lctx.i8_type().array_type(64).into(), // name
                lctx.i64_type().into(),               // state_blob_size
                lctx.i64_type().into(),               // retain_size
                lctx.i64_type().into(),               // retain_offset
                lctx.i64_type().into(),               // task_scratch_size
                lctx.i32_type().into(),               // image_in_bytes
                lctx.i32_type().into(),               // image_out_bytes
                lctx.i32_type().into(),               // marker_bytes
                lctx.i32_type().into(),               // _pad0
                lctx.i32_type().into(),               // task_count
                lctx.i32_type().into(),               // _pad1
                i8p.into(),                           // tasks
                i8p.into(),                           // task_fns
                lctx.i32_type().into(),               // task_fn_count
                lctx.i32_type().into(),               // _pad2
                i8p.into(),                           // io_bindings
                lctx.i32_type().into(),               // io_binding_count
                lctx.i32_type().into(),               // _pad3
            ],
            false,
        );
        let desc = lctx.const_struct(
            &[
                lctx.i32_type().const_int(224, false).into(),
                lctx.i32_type().const_int(1, false).into(), // ABI v1
                lctx.i32_type().const_zero().into(),        // abi_ext
                lctx.i32_type().const_int(has_migrate as u64, false).into(), // flags bit0: has_migrate
                lctx.const_string(&uuid, false).into(),
                lctx.const_string(&layout_hash, false).into(),
                lctx.const_string(&pname, false).into(),
                lctx.i64_type()
                    .const_int(self.layout.total_size, false)
                    .into(),
                lctx.i64_type()
                    .const_int(self.layout.retain_size, false)
                    .into(),
                lctx.i64_type()
                    .const_int(self.layout.retain_offset, false)
                    .into(),
                lctx.i64_type().const_zero().into(),
                lctx.i32_type()
                    .const_int(img_bytes(io_binds, AtRegion::I), false)
                    .into(),
                lctx.i32_type()
                    .const_int(img_bytes(io_binds, AtRegion::Q), false)
                    .into(),
                lctx.i32_type()
                    .const_int(img_bytes(io_binds, AtRegion::M), false)
                    .into(),
                lctx.i32_type().const_zero().into(),
                lctx.i32_type().const_int(1, false).into(), // task_count
                lctx.i32_type().const_zero().into(),
                g_tasks.as_pointer_value().into(),
                g_fns.as_pointer_value().into(),
                lctx.i32_type().const_int(1, false).into(), // task_fn_count
                lctx.i32_type().const_zero().into(),
                g_io.as_pointer_value().into(),
                lctx.i32_type()
                    .const_int(entries.len() as u64, false)
                    .into(),
                lctx.i32_type().const_zero().into(),
            ],
            false,
        );
        let g_desc = self.module.add_global(desc_ty, None, "__tplc_desc");
        g_desc.set_initializer(&desc);
        g_desc.set_constant(true);
        g_desc.set_linkage(Linkage::Internal);

        // tplc_get_descriptor() -> ptr
        let ft = i8p.fn_type(&[], false);
        let f = self
            .module
            .add_function("tplc_get_descriptor", ft, Some(Linkage::External));
        let bb = lctx.append_basic_block(f, "e");
        self.b.position_at_end(bb);
        self.b
            .build_return(Some(&g_desc.as_pointer_value()))
            .unwrap();

        // tplc_task_run(tctx): bounds-check task_index, indirect-call
        // task_fns[i]
        let ft = lctx.i32_type().fn_type(&[i8p.into()], false);
        let f = self
            .module
            .add_function("tplc_task_run", ft, Some(Linkage::External));
        let e_bb = lctx.append_basic_block(f, "e");
        let ok_bb = lctx.append_basic_block(f, "ok");
        let bad_bb = lctx.append_basic_block(f, "bad");
        self.b.position_at_end(e_bb);
        let tctx = f.get_nth_param(0).unwrap().into_pointer_value();
        let idx_p = self.off_ptr(tctx, 8);
        let idx = self
            .b
            .build_load(lctx.i32_type(), idx_p, "idx")
            .unwrap()
            .into_int_value();
        let inb = self
            .b
            .build_int_compare(
                IntPredicate::ULT,
                idx,
                lctx.i32_type().const_int(1, false),
                "inb",
            )
            .unwrap();
        self.b.build_conditional_branch(inb, ok_bb, bad_bb).unwrap();
        self.b.position_at_end(bad_bb);
        self.b
            .build_return(Some(&lctx.i32_type().const_int((-2i64) as u64, true)))
            .unwrap();
        self.b.position_at_end(ok_bb);
        let fnpp = unsafe {
            self.b
                .build_in_bounds_gep(
                    fns_arr.get_type(),
                    g_fns.as_pointer_value(),
                    &[lctx.i32_type().const_zero(), idx],
                    "fp",
                )
                .unwrap()
        };
        let fptr = self
            .b
            .build_load(i8p, fnpp, "f")
            .unwrap()
            .into_pointer_value();
        let cs = self
            .b
            .build_indirect_call(ft, fptr, &[tctx.into()], "call")
            .unwrap();
        let rc = cs.try_as_basic_value().expect_basic("rc");
        self.b.build_return(Some(&rc)).unwrap();
    }

    /// one generated POU fn: (ctx, this) → i32 rc
    fn emit_pou(&mut self, pou: &Pou, scope: &Ctx) {
        let f = self.pou_fn(&pou.name);
        let entry = self.ctx.append_basic_block(f, "entry");
        self.b.position_at_end(entry);
        self.ctx_ptr = f.get_nth_param(0).unwrap().into_pointer_value();
        self.this_ptr = f.get_nth_param(1).unwrap().into_pointer_value();
        // abort exit blocks: -22 aborted, -23 WCET overrun
        let ab = self.ctx.append_basic_block(f, "abort");
        self.abort_bb = Some(ab);
        self.b.position_at_end(ab);
        self.b
            .build_return(Some(&self.ctx.i32_type().const_int((-22i64) as u64, true)))
            .unwrap();
        let ob = self.ctx.append_basic_block(f, "overrun");
        self.overrun_bb = Some(ob);
        self.b.position_at_end(ob);
        self.b
            .build_return(Some(&self.ctx.i32_type().const_int((-23i64) as u64, true)))
            .unwrap();
        /* reference faults (TPLC_TASK_E_PANIC / _BOUNDS) */
        let pb = self.ctx.append_basic_block(f, "ref.panic");
        self.panic_bb = Some(pb);
        self.b.position_at_end(pb);
        self.b
            .build_return(Some(&self.ctx.i32_type().const_int((-17i64) as u64, true)))
            .unwrap();
        let bb = self.ctx.append_basic_block(f, "ref.bounds");
        self.bounds_bb = Some(bb);
        self.b.position_at_end(bb);
        self.b
            .build_return(Some(&self.ctx.i32_type().const_int((-18i64) as u64, true)))
            .unwrap();
        self.b.position_at_end(entry);
        self.abort_check();
        if let Body::Stmts(ss) = &pou.body {
            self.stmts(scope, ss);
        }
        if self
            .b
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            self.b
                .build_return(Some(&self.ctx.i32_type().const_zero()))
                .unwrap();
        }
    }
}

/// Emit the module: abi_version, init, shutdown, task_run + per-POU fns.
/// Field offsets in TplcMigrateCtx (doc 02 ABI).
const MCTX_NEW_PROG: u64 = 0;
const MCTX_OLD_BLOB: u64 = 8;
const MCTX_OLD_BLOB_SIZE: u64 = 16;
const MCTX_OLD_LAYOUT_HASH: u64 = 24;
const MCTX_OUT_STATUS: u64 = 80;
const MCTX_OUT_FIRST_ERR: u64 = 84;

impl<'ctx> Gen<'ctx> {
    /// `i32 tplc_migrate(i8* mctx)` — baked rule table: verify the
    /// predecessor hash, then straight-line copy/transform ops of the
    /// old blob into the new program's state_blob (doc 08 §2: the
    /// snapshot is the source, defaults already ran via tplc_init).
    fn emit_migrate(&mut self, mig: &MigrateEmit) {
        let lctx = self.ctx;
        let i8p = self.i8p();
        let i32t = lctx.i32_type();
        let i64t = lctx.i64_type();
        let ft = i32t.fn_type(&[i8p.into()], false);
        let f = self
            .module
            .add_function("tplc_migrate", ft, Some(Linkage::External));
        let ld = |g: &Self,
                  base: inkwell::values::PointerValue<'ctx>,
                  off: u64,
                  ty: inkwell::types::BasicTypeEnum<'ctx>,
                  n: &str| {
            let p = unsafe {
                g.b.build_gep(lctx.i8_type(), base, &[i64t.const_int(off, false)], n)
                    .unwrap()
            };
            g.b.build_load(ty, p, n).unwrap()
        };
        let entry = lctx.append_basic_block(f, "e");
        let deny_bb = lctx.append_basic_block(f, "deny");
        let brange_bb = lctx.append_basic_block(f, "brange");
        let ok_bb = lctx.append_basic_block(f, "ok");
        let done_bb = lctx.append_basic_block(f, "done");
        self.b.position_at_end(entry);

        let mctx = f.get_nth_param(0).unwrap().into_pointer_value();
        // predecessor hash check: 4 × u64 compare, bail to deny on
        // first mismatch (SWAP_RACE — wrong-layout migrate is unsafe)
        for i in 0..4u64 {
            let word = u64::from_le_bytes(
                mig.predecessor_hash[i as usize * 8..i as usize * 8 + 8]
                    .try_into()
                    .unwrap(),
            );
            let v =
                ld(self, mctx, MCTX_OLD_LAYOUT_HASH + i * 8, i64t.into(), "hw").into_int_value();
            let eq = self
                .b
                .build_int_compare(
                    inkwell::IntPredicate::EQ,
                    v,
                    i64t.const_int(word, false),
                    "eq",
                )
                .unwrap();
            let ne = self.b.build_not(eq, "mm").unwrap();
            let cont = lctx.append_basic_block(f, "hcont");
            self.b.build_conditional_branch(ne, deny_bb, cont).unwrap();
            self.b.position_at_end(cont);
        }
        self.b.build_unconditional_branch(ok_bb).unwrap();

        // ok: per-op byte copies / widens
        self.b.position_at_end(ok_bb);
        let new_ctx = ld(self, mctx, MCTX_NEW_PROG, i8p.into(), "nctx").into_pointer_value();
        let new_blob = ld(self, new_ctx, CTX_STATE_BLOB, i8p.into(), "nblob").into_pointer_value();
        let old_blob = ld(self, mctx, MCTX_OLD_BLOB, i8p.into(), "oblob").into_pointer_value();
        let old_size = ld(self, mctx, MCTX_OLD_BLOB_SIZE, i64t.into(), "osize").into_int_value();
        for op in &mig.ops {
            let (oo, no, need) = match op {
                MigOp::Copy {
                    old_off,
                    new_off,
                    size,
                } => (*old_off, *new_off, *size),
                MigOp::Widen {
                    old_off, new_off, ..
                } => (*old_off, *new_off, 8),
                MigOp::CopyMin {
                    old_off,
                    new_off,
                    old_size,
                    ..
                } => (*old_off, *new_off, *old_size),
            };
            // bound: old_off + need <= old_size
            let end = self
                .b
                .build_int_add(
                    i64t.const_int(oo, false),
                    i64t.const_int(need, false),
                    "end",
                )
                .unwrap();
            let inb = self
                .b
                .build_int_compare(inkwell::IntPredicate::ULE, end, old_size, "inb")
                .unwrap();
            let next = lctx.append_basic_block(f, "op");
            self.b
                .build_conditional_branch(inb, next, brange_bb)
                .unwrap();
            self.b.position_at_end(next);
            let src = unsafe {
                self.b
                    .build_gep(
                        lctx.i8_type(),
                        old_blob,
                        &[i64t.const_int(oo, false)],
                        "src",
                    )
                    .unwrap()
            };
            let dst = unsafe {
                self.b
                    .build_gep(
                        lctx.i8_type(),
                        new_blob,
                        &[i64t.const_int(no, false)],
                        "dst",
                    )
                    .unwrap()
            };
            match op {
                MigOp::Copy { size, .. } => {
                    self.b
                        .build_memcpy(dst, 1, src, 1, i64t.const_int(*size, false))
                        .unwrap();
                }
                MigOp::Widen {
                    old_tag, new_tag, ..
                } => {
                    let ot = prim_ty(lctx, *old_tag);
                    let nt = prim_ty(lctx, *new_tag);
                    let v = self.b.build_load(ot, src, "v").unwrap();
                    let w = widen_value(&self.b, v, ot, nt, *old_tag, *new_tag);
                    self.b.build_store(dst, w).unwrap();
                }
                MigOp::CopyMin {
                    old_size, new_size, ..
                } => {
                    let n = (*old_size).min(*new_size);
                    self.b
                        .build_memcpy(dst, 1, src, 1, i64t.const_int(n, false))
                        .unwrap();
                }
            }
        }
        self.b.build_unconditional_branch(done_bb).unwrap();

        self.b.position_at_end(deny_bb);
        self.b
            .build_store(
                out_ptr(self, mctx, MCTX_OUT_STATUS),
                i32t.const_int(2, false),
            )
            .unwrap(); // TPLC_MIGRATE_DENY
        self.b
            .build_store(
                out_ptr(self, mctx, MCTX_OUT_FIRST_ERR),
                i32t.const_int(1, false),
            )
            .unwrap(); // SWAP_RACE
        self.b
            .build_return(Some(&i32t.const_int((u32::MAX - 32) as u64, true)))
            .unwrap(); // -33

        self.b.position_at_end(brange_bb);
        self.b
            .build_store(
                out_ptr(self, mctx, MCTX_OUT_STATUS),
                i32t.const_int(4, false),
            )
            .unwrap(); // ERR_BLOB_RANGE
        self.b
            .build_return(Some(&i32t.const_int((u32::MAX - 34) as u64, true)))
            .unwrap(); // -35

        self.b.position_at_end(done_bb);
        self.b
            .build_store(
                out_ptr(self, mctx, MCTX_OUT_STATUS),
                i32t.const_int(mig.degraded as u64, false),
            )
            .unwrap();
        self.b
            .build_store(out_ptr(self, mctx, MCTX_OUT_FIRST_ERR), i32t.const_zero())
            .unwrap();
        self.b.build_return(Some(&i32t.const_zero())).unwrap();
    }
}

/// TYPC tag → LLVM primitive type for migrate widening.
fn prim_ty<'ctx>(
    lctx: &'ctx inkwell::context::Context,
    tag: u8,
) -> inkwell::types::BasicTypeEnum<'ctx> {
    match tag {
        1 | 5 => lctx.i8_type().into(),
        2 | 6 => lctx.i16_type().into(),
        3 | 7 | 11 => lctx.i32_type().into(),
        4 | 8 => lctx.i64_type().into(),
        9 => lctx.f32_type().into(),
        10 => lctx.f64_type().into(),
        _ => lctx.i64_type().into(),
    }
}

fn widen_value<'ctx>(
    b: &inkwell::builder::Builder<'ctx>,
    v: inkwell::values::BasicValueEnum<'ctx>,
    ot: inkwell::types::BasicTypeEnum<'ctx>,
    nt: inkwell::types::BasicTypeEnum<'ctx>,
    old_tag: u8,
    new_tag: u8,
) -> inkwell::values::BasicValueEnum<'ctx> {
    let _ = (ot, nt);
    if old_tag == 9 && new_tag == 10 {
        return b
            .build_float_ext(v.into_float_value(), nt.into_float_type(), "f")
            .unwrap()
            .into();
    }
    let src = v.into_int_value();
    let dst_ty = nt.into_int_type();
    if (5..=8).contains(&old_tag) {
        b.build_int_z_extend(src, dst_ty, "z").unwrap().into()
    } else {
        b.build_int_s_extend(src, dst_ty, "s").unwrap().into()
    }
}

fn out_ptr<'ctx>(
    g: &Gen<'ctx>,
    mctx: inkwell::values::PointerValue<'ctx>,
    off: u64,
) -> inkwell::values::PointerValue<'ctx> {
    unsafe {
        g.b.build_gep(
            g.ctx.i8_type(),
            mctx,
            &[g.ctx.i64_type().const_int(off, false)],
            "op",
        )
        .unwrap()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn emit_program<'ctx>(
    lctx: &'ctx Context,
    mir: &'static Module,
    layout: &'static Layout,
    program: &'static Pou,
    io_binds: &[IoBind],
    program_name: &str,
    uuid: [u8; 16],
    layout_hash: [u8; 32],
    migrate: Option<&MigrateEmit>,
    task_period_ns: u64,
) -> LlvmModule<'ctx> {
    let module = lctx.create_module("tplc_program");
    let mut g = Gen {
        ctx: lctx,
        module,
        b: lctx.create_builder(),
        layout,
        mir,
        abort_bb: None,
        overrun_bb: None,
        panic_bb: None,
        bounds_bb: None,
        loop_stack: Vec::new(),
        ctx_ptr: lctx.ptr_type(inkwell::AddressSpace::default()).const_zero(),
        this_ptr: lctx.ptr_type(inkwell::AddressSpace::default()).const_zero(),
    };

    let i8p = g.i8p();
    // tplc_abi_version
    let ft = lctx.i32_type().fn_type(&[], false);
    let f = g
        .module
        .add_function("tplc_abi_version", ft, Some(Linkage::External));
    let bb = lctx.append_basic_block(f, "e");
    g.b.position_at_end(bb);
    g.b.build_return(Some(&lctx.i32_type().const_int(1, false)))
        .unwrap();

    // tplc_init(ctx): memset blob + zero-init (literal inits: P1-07)
    let ft = lctx.i32_type().fn_type(&[i8p.into()], false);
    let f = g
        .module
        .add_function("tplc_init", ft, Some(Linkage::External));
    let bb = lctx.append_basic_block(f, "e");
    g.b.position_at_end(bb);
    g.ctx_ptr = f.get_nth_param(0).unwrap().into_pointer_value();
    let blob = g
        .ctx_load(CTX_STATE_BLOB, i8p.into(), "blob")
        .into_pointer_value();
    // zero the blob in two segments around the retain region; the
    // retain segment itself is zeroed only on a cold start — a warm
    // restore (CTXF_RETAIN_RESTORED = ctx_flags bit3) keeps host bytes
    let (roff, rsize) = (layout.retain_offset, layout.retain_size);
    if rsize == 0 {
        g.b.build_memset(
            blob,
            1,
            lctx.i8_type().const_zero(),
            lctx.i64_type().const_int(layout.total_size, false),
        )
        .unwrap();
    } else {
        g.b.build_memset(
            blob,
            1,
            lctx.i8_type().const_zero(),
            lctx.i64_type().const_int(roff, false),
        )
        .unwrap();
        if roff + rsize < layout.total_size {
            let tail = unsafe {
                g.b.build_gep(
                    lctx.i8_type(),
                    blob,
                    &[lctx.i64_type().const_int(roff + rsize, false)],
                    "tb",
                )
            }
            .unwrap();
            g.b.build_memset(
                tail,
                1,
                lctx.i8_type().const_zero(),
                lctx.i64_type()
                    .const_int(layout.total_size - roff - rsize, false),
            )
            .unwrap();
        }
    }
    // literal default inits; retain zeroing + retain literal inits are
    // skipped when the host signals a warm restore
    let scope0 = program_ctx(layout);
    g.this_ptr = blob;
    let cf = g.ctx_load(4, lctx.i32_type().into(), "cf").into_int_value();
    let restored =
        g.b.build_and(cf, lctx.i32_type().const_int(8, false), "rr")
            .unwrap();
    let retain_bb = lctx.append_basic_block(f, "i.ret");
    let done_bb = lctx.append_basic_block(f, "i.done");
    let plain: Vec<&VarDecl> = program
        .iface
        .iter()
        .filter(|v| !v.retain && matches!(v.init.as_ref().map(|i| &i.kind), Some(ExprKind::Lit(_))))
        .collect();
    let retains: Vec<&VarDecl> = program
        .iface
        .iter()
        .filter(|v| v.retain && matches!(v.init.as_ref().map(|i| &i.kind), Some(ExprKind::Lit(_))))
        .collect();
    for v in &plain {
        if let Some((off, ty)) = scope0.get(v.name.as_str()).cloned() {
            g.lit_store(blob, off, &ty, v.init.as_ref().unwrap());
        }
    }
    if retains.is_empty() && rsize == 0 {
        g.b.build_unconditional_branch(done_bb).unwrap();
        g.b.position_at_end(retain_bb);
        g.b.build_unconditional_branch(done_bb).unwrap();
    } else {
        let nz =
            g.b.build_int_compare(
                IntPredicate::EQ,
                restored,
                lctx.i32_type().const_zero(),
                "z",
            )
            .unwrap();
        g.b.build_conditional_branch(nz, retain_bb, done_bb)
            .unwrap();
        g.b.position_at_end(retain_bb);
        if rsize > 0 {
            let rseg = unsafe {
                g.b.build_gep(
                    lctx.i8_type(),
                    blob,
                    &[lctx.i64_type().const_int(roff, false)],
                    "rb",
                )
            }
            .unwrap();
            g.b.build_memset(
                rseg,
                1,
                lctx.i8_type().const_zero(),
                lctx.i64_type().const_int(rsize, false),
            )
            .unwrap();
        }
        for v in &retains {
            if let Some((off, ty)) = scope0.get(v.name.as_str()).cloned() {
                g.lit_store(blob, off, &ty, v.init.as_ref().unwrap());
            }
        }
        g.b.build_unconditional_branch(done_bb).unwrap();
    }
    g.b.position_at_end(done_bb);
    g.b.build_return(Some(&lctx.i32_type().const_zero()))
        .unwrap();

    // tplc_shutdown(ctx)
    let ft = lctx.void_type().fn_type(&[i8p.into()], false);
    let f = g
        .module
        .add_function("tplc_shutdown", ft, Some(Linkage::External));
    let bb = lctx.append_basic_block(f, "e");
    g.b.position_at_end(bb);
    g.b.build_return(None).unwrap();

    // per-POU fns (program first so its fn exists for task_run)
    let scope = program_ctx(layout);
    for pou in &mir.pous {
        let sc: Ctx = match pou.kind {
            PouKind::Program => scope.clone(),
            PouKind::FunctionBlock => {
                let fbl = layout
                    .fb_types
                    .get(&pou.name)
                    .cloned()
                    .or_else(|| builtin_fb_layout(&pou.name))
                    .unwrap();
                fb_ctx(&fbl)
            }
            PouKind::Function => {
                let fbl = tplc_mir::layout::layout_members(&pou.iface, BTreeMap::new());
                fb_ctx(&fbl)
            }
        };
        g.emit_pou(pou, &sc);
    }

    // __tplc_task_0(tctx) -> i32 : ctx=tctx->prog; run program pou
    let ft = lctx.i32_type().fn_type(&[i8p.into()], false);
    let f = g
        .module
        .add_function("__tplc_task_0", ft, Some(Linkage::Internal));
    let bb = lctx.append_basic_block(f, "e");
    g.b.position_at_end(bb);
    let tctx = f.get_nth_param(0).unwrap().into_pointer_value();
    g.ctx_ptr = tctx;
    let prog = g
        .ctx_load(TCTX_PROG, i8p.into(), "prog")
        .into_pointer_value();
    g.ctx_ptr = prog;
    let blob = g
        .ctx_load(CTX_STATE_BLOB, i8p.into(), "blob")
        .into_pointer_value();
    let pf = g.pou_fn(&program.name);
    let cs =
        g.b.build_call(pf, &[prog.into(), blob.into()], "run")
            .unwrap();
    let rc = cs.try_as_basic_value().expect_basic("rc");
    g.b.build_return(Some(&rc)).unwrap();

    if let Some(m) = migrate {
        g.emit_migrate(m);
    }
    g.emit_descriptor(
        program,
        io_binds,
        program_name,
        uuid,
        layout_hash,
        migrate.is_some(),
        task_period_ns,
    );
    g.module
}

/* ------------------------------------------------------------------ */
/* Object / bitcode emission (P1-07)                                   */
/* ------------------------------------------------------------------ */

use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::OptimizationLevel;
use std::path::Path;

/// Create a TargetMachine for `triple` ("x86_64-unknown-linux-gnu",
/// "aarch64-unknown-linux-gnu", "armv6-unknown-linux-gnueabihf").
pub fn target_machine(triple: &str) -> Result<TargetMachine, String> {
    let _ = triple;
    Target::initialize_native(&InitializationConfig::default()).map_err(|e| e.to_string())?;
    let triple_t = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple_t).map_err(|e| e.to_string())?;
    target
        .create_target_machine(
            &triple_t,
            "generic",
            "",
            OptimizationLevel::Default,
            RelocMode::PIC,
            CodeModel::Default,
        )
        .ok_or_else(|| "no target machine".to_string())
}

/// Emit an ELF object file for the native target.
pub fn emit_object(module: &LlvmModule<'_>, path: &Path) -> Result<(), String> {
    let tm = target_machine("")?;
    tm.write_to_file(module, FileType::Object, path)
        .map_err(|e| e.to_string())
}

/// Emit LLVM bitcode (the canonical .tplc retained form, doc 01 §4).
pub fn emit_bitcode(module: &LlvmModule<'_>, path: &Path) -> Result<(), String> {
    module.write_bitcode_to_path(path);
    Ok(())
}
