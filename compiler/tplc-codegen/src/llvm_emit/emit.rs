//! emit — module/POU/migrate emission entry points (split from llvm_emit.rs).
use super::*;

impl<'ctx> Gen<'ctx> {
    /// descriptor globals + tplc_get_descriptor + tplc_task_run
    /// dispatcher (task_fns table + bounds check + indirect call).
    /// `tasks` is the resolved task table (module.tasks, or the
    /// synthesized single MAIN task) — one TplcTaskDesc + one
    /// __tplc_task_<i> entry fn each.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn emit_descriptor(
        &mut self,
        _program: &Pou,
        io_binds: &[IoBind],
        program_name: &str,
        uuid: [u8; 16],
        layout_hash: [u8; 32],
        has_migrate: bool,
        tasks: &[TaskDecl],
    ) {
        let lctx = self.ctx;
        let i8p = self.i8p();

        // -- TplcTaskDesc[N]: cyclic tasks enabled-at-start
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
        let task_consts: Vec<_> = tasks
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let mut name_b = [0u8; 32];
                let nb = t.name.as_bytes();
                name_b[..nb.len().min(32)].copy_from_slice(&nb[..nb.len().min(32)]);
                lctx.const_struct(
                    &[
                        lctx.const_string(&name_b, false).into(),
                        lctx.i8_type()
                            .const_int(t.kind as u64, false)
                            .into(), // TplcTaskKind
                        lctx.i8_type().const_int(3, false).into(), // ENABLED|WATCHDOGGED
                        lctx.i8_type().const_int(t.priority as u64, false).into(),
                        lctx.i8_type().const_zero().into(),
                        lctx.i64_type().const_int(t.period_ns, false).into(),
                        lctx.i64_type().const_int(t.wcet_hint_ns, false).into(),
                        lctx.i32_type().const_int(i as u64, false).into(),
                        lctx.i32_type().const_zero().into(),
                    ],
                    false,
                )
            })
            .collect();
        let tasks_arr = task_ty.const_array(&task_consts);
        let g_tasks = self
            .module
            .add_global(tasks_arr.get_type(), None, "__tplc_tasks");
        g_tasks.set_initializer(&tasks_arr);
        g_tasks.set_constant(true);
        g_tasks.set_linkage(Linkage::Internal);

        // -- task_fns[N] = { __tplc_task_<i> }
        let tfns: Vec<_> = (0..tasks.len())
            .map(|i| {
                self.module
                    .get_function(&format!("__tplc_task_{i}"))
                    .unwrap()
                    .as_global_value()
                    .as_pointer_value()
            })
            .collect();
        let fns_arr = i8p.const_array(&tfns);
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
        let entries: Vec<_> = io_binds
            .iter()
            .filter_map(|b| {
                // "PROG.var" or bare "var" — resolve inside the
                // owning program's scope (multi-program fields are
                // qualified PROG.var)
                let sc = match b.name.split_once('.') {
                    Some((p, _)) => pou_scope(self.layout, p),
                    None => program_ctx(self.layout),
                };
                let (off, ty) = sc.get(b.name.split('.').next_back().unwrap_or(&b.name))?;
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
                // abi_ext: TPLC_EXT_EVENT_TASKS (1<<1) when the module
                // exports event-task triggers
                lctx.i32_type()
                    .const_int(
                        if tasks.iter().any(|t| t.kind == TaskKind::Event) {
                            2
                        } else {
                            0
                        },
                        false,
                    )
                    .into(),
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
                lctx.i32_type()
                    .const_int(tasks.len() as u64, false)
                    .into(), // task_count
                lctx.i32_type().const_zero().into(),
                g_tasks.as_pointer_value().into(),
                g_fns.as_pointer_value().into(),
                lctx.i32_type()
                    .const_int(tasks.len() as u64, false)
                    .into(), // task_fn_count
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

        // -- variable table export (tplc_sym.h): by-name access
        let var_ty = lctx.struct_type(
            &[
                lctx.i8_type().array_type(32).into(), // name[32]
                lctx.i32_type().into(),               // offset
                lctx.i32_type().into(),               // size
                lctx.i8_type().into(),                // type_code
                lctx.i8_type().into(),                // is_at
                lctx.i8_type().into(),                // region
                lctx.i8_type().into(),                // unit
                lctx.i32_type().into(),               // _pad
            ],
            false,
        );
        // every layout field is top-level storage with an absolute blob
        // offset (bare names single-program, PROG.name / GVL.name qualified
        // otherwise) — program_ctx would drop the qualified ones and leave
        // multi-program modules with an empty table
        let vscope: Ctx = self
            .layout
            .fields
            .iter()
            .map(|(k, f)| (k.clone(), (f.offset, f.ty.clone())))
            .collect();
        let ventries: Vec<_> = vscope
            .iter()
            .filter_map(|(name, (off, ty))| {
                let mut nb = [0u8; 32];
                let bytes = name.as_bytes();
                if bytes.len() >= 32 {
                    return None;
                }
                nb[..bytes.len()].copy_from_slice(bytes);
                let (is_at, region, tc, unit, img) = match io_binds
                    .iter()
                    .find(|b| {
                        // binds are "POU.var"; single-program fields are
                        // bare "var", qualified fields match in full
                        b.name.eq_ignore_ascii_case(name.as_str())
                            || (!name.contains('.')
                                && b.name
                                    .rsplit('.')
                                    .next()
                                    .map(|s| s.eq_ignore_ascii_case(name.as_str()))
                                    == Some(true))
                    })
                {
                    Some(b) => {
                        let (r, t, u) = io_codes(&b.at, ty);
                        (1u64, r, t, u, b.at.byte)
                    }
                    None => (0u64, 0u8, io_type_code(ty), 0u8, 0u64),
                };
                Some(self.ctx.const_struct(
                    &[
                        lctx.const_string(&nb, false).into(),
                        lctx.i32_type().const_int(*off, false).into(),
                        lctx.i32_type().const_int(ty.size(), false).into(),
                        lctx.i8_type().const_int(tc as u64, false).into(),
                        lctx.i8_type().const_int(is_at, false).into(),
                        lctx.i8_type().const_int(region as u64, false).into(),
                        lctx.i8_type().const_int(unit as u64, false).into(),
                        lctx.i32_type().const_int(img, false).into(),
                    ],
                    false,
                ))
            })
            .collect();
        let var_arr_ty = var_ty.array_type(ventries.len().max(1) as u32);
        let g_vars = self.module.add_global(var_arr_ty, None, "__tplc_vars");
        if ventries.is_empty() {
            g_vars.set_initializer(&var_arr_ty.const_zero());
        } else {
            g_vars.set_initializer(&var_ty.const_array(&ventries));
        }
        g_vars.set_constant(true);
        g_vars.set_linkage(Linkage::Internal);

        // tplc_get_vars(uint32_t* count) -> const TplcVarDesc*
        let ft = i8p.fn_type(&[i8p.into()], false);
        let fv = self.module.add_function(
            "tplc_get_vars",
            ft,
            Some(Linkage::External),
        );
        let vb = lctx.append_basic_block(fv, "e");
        self.b.position_at_end(vb);
        let cp = fv.get_nth_param(0).unwrap().into_pointer_value();
        self.b
            .build_store(
                cp,
                lctx.i32_type()
                    .const_int(ventries.len() as u64, false),
            )
            .unwrap();
        self.b
            .build_return(Some(&g_vars.as_pointer_value()))
            .unwrap();

        // -- TplcTaskEvent[N] + tplc_get_task_events (event tasks only)
        let ev_tasks: Vec<&TaskDecl> = tasks
            .iter()
            .filter(|t| t.kind == TaskKind::Event)
            .collect();
        if !ev_tasks.is_empty() {
            let ev_ty = lctx.struct_type(
                &[
                    lctx.i8_type().array_type(32).into(),  // task name
                    lctx.i8_type().into(),                 // trig
                    lctx.i8_type().array_type(7).into(),   // _pad
                    lctx.i8_type().array_type(48).into(),  // signal
                ],
                false,
            );
            let ev_consts: Vec<_> = ev_tasks
                .iter()
                .map(|t| {
                    let (tt, sig) = t
                        .trig
                        .clone()
                        .unwrap_or((TaskTrig::Manual, String::new()));
                    let mut tn = [0u8; 32];
                    let tb = t.name.as_bytes();
                    tn[..tb.len().min(32)].copy_from_slice(&tb[..tb.len().min(32)]);
                    let mut sn = [0u8; 48];
                    let sb = sig.as_bytes();
                    sn[..sb.len().min(48)].copy_from_slice(&sb[..sb.len().min(48)]);
                    lctx.const_struct(
                        &[
                            lctx.const_string(&tn, false).into(),
                            lctx.i8_type().const_int(tt as u64, false).into(),
                            lctx.const_string(&[0u8; 7], false).into(),
                            lctx.const_string(&sn, false).into(),
                        ],
                        false,
                    )
                })
                .collect();
            let ev_arr = ev_ty.const_array(&ev_consts);
            let g_ev = self
                .module
                .add_global(ev_arr.get_type(), None, "__tplc_task_events");
            g_ev.set_initializer(&ev_arr);
            g_ev.set_constant(true);
            g_ev.set_linkage(Linkage::Internal);

            // tplc_get_task_events(uint32_t* count) -> const TplcTaskEvent*
            let ft = i8p.fn_type(&[i8p.into()], false);
            let fe = self.module.add_function(
                "tplc_get_task_events",
                ft,
                Some(Linkage::External),
            );
            let eb = lctx.append_basic_block(fe, "e");
            self.b.position_at_end(eb);
            let ecp = fe.get_nth_param(0).unwrap().into_pointer_value();
            self.b
                .build_store(
                    ecp,
                    lctx.i32_type()
                        .const_int(ev_consts.len() as u64, false),
                )
                .unwrap();
            self.b
                .build_return(Some(&g_ev.as_pointer_value()))
                .unwrap();
        }

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
        let idx_p = self.off_ptr(tctx, self.abi.tctx_index);
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
    pub(crate) fn emit_pou(&mut self, pou: &Pou, scope: &Ctx) {
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
        let rk = self.ctx.append_basic_block(f, "ret.ok");
        self.retok_bb = Some(rk);
        self.b.position_at_end(rk);
        self.b
            .build_return(Some(&self.ctx.i32_type().const_zero()))
            .unwrap();
        self.cur_fn = if pou.kind == PouKind::Function {
            Some(pou.name.clone())
        } else {
            None
        };
        self.cur_inouts = pou
            .iface
            .iter()
            .filter(|v| v.kind == VarKind::InOut)
            .map(|v| v.name.clone())
            .collect();
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
        /* checked-arith div/mod by zero (TPLC_TASK_E_DIVZERO) */
        let db = self.ctx.append_basic_block(f, "div.zero");
        self.divzero_bb = Some(db);
        self.b.position_at_end(db);
        self.b
            .build_return(Some(
                &self
                    .ctx
                    .i32_type()
                    .const_int((TPLC_TASK_E_DIVZERO as i64) as u64, true),
            ))
            .unwrap();
        /* checked-arith overflow (TPLC_TASK_E_OVERFLOW) and
        conversion range (TPLC_TASK_E_RANGE) */
        let ov = self.ctx.append_basic_block(f, "arith.ovf");
        self.overflow_bb = Some(ov);
        self.b.position_at_end(ov);
        self.b
            .build_return(Some(
                &self
                    .ctx
                    .i32_type()
                    .const_int((TPLC_TASK_E_OVERFLOW as i64) as u64, true),
            ))
            .unwrap();
        let rg = self.ctx.append_basic_block(f, "conv.rng");
        self.range_bb = Some(rg);
        self.b.position_at_end(rg);
        self.b
            .build_return(Some(
                &self
                    .ctx
                    .i32_type()
                    .const_int((TPLC_TASK_E_RANGE as i64) as u64, true),
            ))
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

    /// `i32 tplc_migrate(i8* mctx)` — baked rule table: verify the
    /// predecessor hash, then straight-line copy/transform ops of the
    /// old blob into the new program's state_blob (doc 08 §2: the
    /// snapshot is the source, defaults already ran via tplc_init).
    pub(crate) fn emit_migrate(&mut self, mig: &MigrateEmit) {
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
            let v = ld(
                self,
                mctx,
                self.abi.mctx_old_layout_hash + i * 8,
                i64t.into(),
                "hw",
            )
            .into_int_value();
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
        let new_ctx =
            ld(self, mctx, self.abi.mctx_new_prog, i8p.into(), "nctx").into_pointer_value();
        let new_blob =
            ld(self, new_ctx, self.abi.ctx_state_blob, i8p.into(), "nblob").into_pointer_value();
        let old_blob =
            ld(self, mctx, self.abi.mctx_old_blob, i8p.into(), "oblob").into_pointer_value();
        let old_size = ld(
            self,
            mctx,
            self.abi.mctx_old_blob_size,
            i64t.into(),
            "osize",
        )
        .into_int_value();
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
                out_ptr(self, mctx, self.abi.mctx_out_status),
                i32t.const_int(2, false),
            )
            .unwrap(); // TPLC_MIGRATE_DENY
        self.b
            .build_store(
                out_ptr(self, mctx, self.abi.mctx_out_first_err),
                i32t.const_int(1, false),
            )
            .unwrap(); // SWAP_RACE
        self.b
            .build_return(Some(&i32t.const_int((u32::MAX - 32) as u64, true)))
            .unwrap(); // -33

        self.b.position_at_end(brange_bb);
        self.b
            .build_store(
                out_ptr(self, mctx, self.abi.mctx_out_status),
                i32t.const_int(4, false),
            )
            .unwrap(); // ERR_BLOB_RANGE
        self.b
            .build_return(Some(&i32t.const_int((u32::MAX - 34) as u64, true)))
            .unwrap(); // -35

        self.b.position_at_end(done_bb);
        self.b
            .build_store(
                out_ptr(self, mctx, self.abi.mctx_out_status),
                i32t.const_int(mig.degraded as u64, false),
            )
            .unwrap();
        self.b
            .build_store(
                out_ptr(self, mctx, self.abi.mctx_out_first_err),
                i32t.const_zero(),
            )
            .unwrap();
        self.b.build_return(Some(&i32t.const_zero())).unwrap();
    }

}
