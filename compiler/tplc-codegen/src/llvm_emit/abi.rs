//! ABI/layout + LLVM type helpers — split from llvm_emit.rs.
use super::*;

#[derive(Clone, Copy)]
pub(crate) struct AbiOffsets {
    pub(crate) ctx_state_blob: u64,
    pub(crate) ctx_flags: u64,
    pub(crate) ctx_svc: u64,
    pub(crate) ctx_abort: u64,
    pub(crate) ctx_deadline: u64,
    pub(crate) tctx_prog: u64,
    pub(crate) tctx_index: u64,
    pub(crate) svc_steady_ns: u64,
    pub(crate) svc_host: u64,
    pub(crate) mctx_new_prog: u64,
    pub(crate) mctx_old_blob: u64,
    pub(crate) mctx_old_blob_size: u64,
    pub(crate) mctx_old_layout_hash: u64,
    pub(crate) mctx_out_status: u64,
    pub(crate) mctx_out_first_err: u64,
}

/// C-layout member placement: align offset up, record, advance.
pub(crate) fn place(off: &mut u64, size: u64, align: u64) -> u64 {
    *off = (*off).div_ceil(align) * align;
    let at = *off;
    *off += size;
    at
}

pub(crate) fn abi_offsets(ptr: u64, i64a: u64) -> AbiOffsets {
    // TplcCtx: u32 size, u32 flags, 5 ptrs, 3 i64s, 2 ptrs, u32, u32, i64
    let mut o = 0;
    place(&mut o, 4, 4);
    let ctx_flags = place(&mut o, 4, 4);
    let ctx_state_blob = place(&mut o, ptr, ptr);
    place(&mut o, ptr, ptr);
    place(&mut o, ptr, ptr);
    place(&mut o, ptr, ptr);
    place(&mut o, ptr, ptr);
    place(&mut o, 8, i64a);
    place(&mut o, 8, i64a);
    place(&mut o, 8, i64a);
    let ctx_svc = place(&mut o, ptr, ptr);
    place(&mut o, ptr, ptr);
    let ctx_abort = place(&mut o, 4, 4);
    place(&mut o, 4, 4);
    let ctx_deadline = place(&mut o, 8, i64a);

    // TplcServices: u32, u32, 6 ptrs, ptr
    let mut s = 0;
    place(&mut s, 4, 4);
    place(&mut s, 4, 4);
    place(&mut s, ptr, ptr);
    place(&mut s, ptr, ptr);
    let svc_steady_ns = place(&mut s, ptr, ptr);
    place(&mut s, ptr, ptr);
    place(&mut s, ptr, ptr);
    place(&mut s, ptr, ptr);
    let svc_host = place(&mut s, ptr, ptr);

    // TplcTaskCtx: ptr, u32, u32, ptr, 3×i64
    let mut t = 0;
    let tctx_prog = place(&mut t, ptr, ptr);
    let tctx_index = place(&mut t, 4, 4);
    place(&mut t, 4, 4);
    place(&mut t, ptr, ptr);
    place(&mut t, 8, i64a);
    place(&mut t, 8, i64a);
    place(&mut t, 8, i64a);

    // TplcMigrateCtx: 2 ptrs, i64, u8[32], u32, u32, ptr, i64, 2×u32
    let mut m = 0;
    let mctx_new_prog = place(&mut m, ptr, ptr);
    let mctx_old_blob = place(&mut m, ptr, ptr);
    let mctx_old_blob_size = place(&mut m, 8, i64a);
    let mctx_old_layout_hash = place(&mut m, 32, 1);
    place(&mut m, 4, 4);
    place(&mut m, 4, 4);
    place(&mut m, ptr, ptr);
    place(&mut m, 8, i64a);
    let mctx_out_status = place(&mut m, 4, 4);
    let mctx_out_first_err = place(&mut m, 4, 4);

    AbiOffsets {
        ctx_state_blob,
        ctx_flags,
        ctx_svc,
        ctx_abort,
        ctx_deadline,
        tctx_prog,
        tctx_index,
        svc_steady_ns,
        svc_host,
        mctx_new_prog,
        mctx_old_blob,
        mctx_old_blob_size,
        mctx_old_layout_hash,
        mctx_out_status,
        mctx_out_first_err,
    }
}

/// Pointer size (bytes) and i64 ABI alignment (bytes) from an LLVM
/// data-layout string ("e-m:e-p:32:32-Fi8-i64:64-…"). Unparseable →
/// LP64 defaults, matching the previous hardcoded offsets.
pub(crate) fn layout_geom(dl: &str) -> (u64, u64) {
    let (mut ptr, mut i64a) = (0u64, 0u64);
    for spec in dl.split('-') {
        let rest = if let Some(r) = spec.strip_prefix("p:") {
            Some(r)
        } else {
            spec.strip_prefix("p0:")
        };
        if let Some(r) = rest {
            if let Ok(bits) = r.split(':').next().unwrap_or("").parse::<u64>() {
                ptr = bits / 8;
            }
        } else if let Some(r) = spec.strip_prefix("i64:") {
            if let Ok(bits) = r.split(':').next().unwrap_or("").parse::<u64>() {
                i64a = bits / 8;
            }
        }
    }
    if ptr == 0 || i64a == 0 {
        (8, 8)
    } else {
        (ptr, i64a)
    }
}

pub(crate) fn abi_offsets_for_triple(triple: &str) -> Result<AbiOffsets, String> {
    let tm = target_machine(triple)?;
    let dl = tm.get_target_data().get_data_layout();
    let (ptr, i64a) = layout_geom(dl.as_str().to_str().unwrap_or(""));
    Ok(abi_offsets(ptr, i64a))
}

/// TplcIoType code for a type (shared by io bindings and the
/// by-name variable table).
pub(crate) fn io_type_code(ty: &Type) -> u8 {
    match ty {
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
    }
}

pub(crate) fn io_codes(at: &At, ty: &Type) -> (u8, u8, u8) {
    let region = match at.region {
        AtRegion::I => 0,
        AtRegion::Q => 1,
        AtRegion::M => 2,
    };
    (region, io_type_code(ty), at.bit.unwrap_or(0))
}

/// highest byte touched in an image region (descriptor image_*_bytes)
pub(crate) fn img_bytes(binds: &[IoBind], r: AtRegion) -> u64 {
    binds
        .iter()
        .filter(|b| b.at.region == r)
        .map(|b| b.at.byte + b.ty.size().max(1))
        .max()
        .unwrap_or(0)
}

/// inclusive (lo, hi) of an int type as i128 — checked-arith
/// conversion range tests compare widened values against this
pub(crate) fn int128_range(t: &Type) -> Option<(i128, i128)> {
    match t {
        Type::Int { signed: true, bits } => {
            let b = *bits as u32;
            Some((-(1i128 << (b - 1)), (1i128 << (b - 1)) - 1))
        }
        Type::Int {
            signed: false,
            bits,
        } => Some((0, (1i128 << *bits as u32) - 1)),
        _ => None,
    }
}

/// i128 constant from two's-complement 64-bit limbs (lo, hi)
pub(crate) fn i128_const<'c>(ctx: &'c Context, v: i128) -> IntValue<'c> {
    ctx.i128_type()
        .const_int_arbitrary_precision(&[v as u64, (v >> 64) as u64])
}

pub(crate) fn llty<'c>(ctx: &'c Context, t: &Type) -> BasicTypeEnum<'c> {
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
pub(crate) fn is_bool(t: &Type) -> bool {
    matches!(t, Type::Bool)
}

/// TYPC tag → LLVM primitive type for migrate widening.
pub(crate) fn prim_ty<'ctx>(
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

pub(crate) fn widen_value<'ctx>(
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

pub(crate) fn out_ptr<'ctx>(
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

