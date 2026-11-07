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
    fn c32(&self, v: u64) -> IntValue<'ctx> {
        self.ctx.i64_type().const_int(v, false)
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
        } else {
            v
        };
        self.b.build_store(p, v).unwrap();
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
                    Lit::Str(_) => self.ctx.i8_type().const_zero().into(),
                }
            }
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
            ExprKind::Call { fun, args } => self.fn_call(ctx, fun, args),
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
            (Type::Int { signed: s, .. }, Type::Int { bits: _, .. }) => {
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
    fn fn_call(&mut self, ctx: &Ctx, fun: &str, args: &[(String, Expr)]) -> BasicValueEnum<'ctx> {
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
            let l = self
                .b
                .build_load(self.ctx.i32_type(), p, "af")
                .unwrap();
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
            .build_int_compare(IntPredicate::NE, dl, self.ctx.i64_type().const_zero(), "arm")
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

    fn stmt(&mut self, ctx: &Ctx, s: &Stmt) {
        match &s.kind {
            StmtKind::Assign { lhs, rhs } => {
                let v = self.expr(ctx, rhs);
                self.write(ctx, lhs, v);
            }
            StmtKind::If { branches, else_ } => {
                let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
                let end_bb = self.ctx.append_basic_block(f, "if.end");
                let mut next_bb = end_bb;
                for (cond, body) in branches.iter().rev() {
                    let then_bb = self.ctx.append_basic_block(f, "if.then");
                    let cv = self.expr(ctx, cond).into_int_value();
                    self.b
                        .build_conditional_branch(cv, then_bb, next_bb)
                        .unwrap();
                    self.b.position_at_end(then_bb);
                    for st in body {
                        self.stmt(ctx, st);
                    }
                    self.b.build_unconditional_branch(end_bb).unwrap();
                    next_bb = then_bb;
                    self.b.position_at_end(next_bb);
                }
                // else block
                for st in else_ {
                    self.stmt(ctx, st);
                }
                self.b.build_unconditional_branch(end_bb).unwrap();
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
                self.b.build_unconditional_branch(cond_bb).unwrap();
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
                self.b.build_unconditional_branch(cond_bb).unwrap();
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
                let cond_bb = self.ctx.append_basic_block(f, "f.cond");
                let body_bb = self.ctx.append_basic_block(f, "f.body");
                let step_bb = self.ctx.append_basic_block(f, "f.step");
                let end_bb = self.ctx.append_basic_block(f, "f.end");
                let step_e = step
                    .as_ref()
                    .map(|e| self.expr(ctx, e).into_int_value())
                    .unwrap_or(self.ctx.i64_type().const_int(1, true));
                let end_v = self.expr(ctx, end).into_int_value();
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
                self.b.build_unconditional_branch(step_bb).unwrap();
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
                    self.b.build_unconditional_branch(end_bb).unwrap();
                    self.b.position_at_end(next_bb);
                }
                for st in else_ {
                    self.stmt(ctx, st);
                }
                self.b.build_unconditional_branch(end_bb).unwrap();
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
    fn emit_descriptor(
        &mut self,
        _program: &Pou,
        io_binds: &[IoBind],
        program_name: &str,
        uuid: [u8; 16],
        layout_hash: [u8; 32],
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
                lctx.i64_type().const_int(10_000_000, false).into(), // 10ms
                lctx.i64_type().const_zero().into(),                 // wcet_hint
                lctx.i32_type().const_zero().into(),                 // entry 0
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
                let (off, ty) = scope.get(b.name.split('.').last().unwrap_or(&b.name))?;
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
                lctx.i32_type().const_zero().into(),        // flags
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
                    &[lctx.i32_type().const_zero().into(), idx],
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
        self.b.position_at_end(entry);
        self.abort_check();
        if let Body::Stmts(ss) = &pou.body {
            self.stmts(scope, ss);
        }
        self.b
            .build_return(Some(&self.ctx.i32_type().const_zero()))
            .unwrap();
    }
}

/// Emit the module: abi_version, init, shutdown, task_run + per-POU fns.
pub fn emit_program<'ctx>(
    lctx: &'ctx Context,
    mir: &'static Module,
    layout: &'static Layout,
    program: &'static Pou,
    io_binds: &[IoBind],
    program_name: &str,
    uuid: [u8; 16],
    layout_hash: [u8; 32],
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
            blob, 1, lctx.i8_type().const_zero(),
            lctx.i64_type().const_int(layout.total_size, false),
        ).unwrap();
    } else {
        g.b.build_memset(
            blob, 1, lctx.i8_type().const_zero(),
            lctx.i64_type().const_int(roff, false),
        ).unwrap();
        if roff + rsize < layout.total_size {
            let tail = unsafe {
                g.b.build_gep(lctx.i8_type(), blob, &[lctx.i64_type().const_int(roff + rsize, false)], "tb")
            }.unwrap();
            g.b.build_memset(
                tail, 1, lctx.i8_type().const_zero(),
                lctx.i64_type().const_int(layout.total_size - roff - rsize, false),
            ).unwrap();
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
                g.b.build_gep(lctx.i8_type(), blob, &[lctx.i64_type().const_int(roff, false)], "rb")
            }.unwrap();
            g.b.build_memset(
                rseg, 1, lctx.i8_type().const_zero(),
                lctx.i64_type().const_int(rsize, false),
            ).unwrap();
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

    g.emit_descriptor(program, io_binds, program_name, uuid, layout_hash);
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
