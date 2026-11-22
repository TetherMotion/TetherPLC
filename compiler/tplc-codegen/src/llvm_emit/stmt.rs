//! stmt — split from llvm_emit.rs.
use super::*;

impl<'ctx> Gen<'ctx> {
    /// store a literal init value at blob+off
    pub(crate) fn lit_store(&mut self, blob: PointerValue<'ctx>, off: u64, ty: &Type, init: &Expr) {
        let v = self.expr(&Ctx::new(), init); // Lit → const, ctx unused
        let v = self.coerce(v, &init.ty, ty);
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
    pub(crate) fn abort_check(&mut self) {
        let f = self.b.get_insert_block().unwrap().get_parent().unwrap();
        let cont = self.ctx.append_basic_block(f, "ck.cont");
        let dlck = self.ctx.append_basic_block(f, "ck.dl");
        let af = {
            let p = self.off_ptr(self.ctx_ptr, self.abi.ctx_abort);
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
            .ctx_load(self.abi.ctx_deadline, self.ctx.i64_type().into(), "dl")
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

    /// svc->steady_ns(svc->host) — TplcServices/TplcCtx offsets come
    /// from AbiOffsets (32/64-bit safe).
    pub(crate) fn steady_ns(&mut self) -> IntValue<'ctx> {
        let i8p = self.ctx.ptr_type(inkwell::AddressSpace::default());
        let svc = self
            .ctx_load(self.abi.ctx_svc, i8p.into(), "svc")
            .into_pointer_value();
        let fpp = unsafe {
            self.b
                .build_gep(
                    self.ctx.i8_type(),
                    svc,
                    &[self.ctx.i64_type().const_int(self.abi.svc_steady_ns, false)],
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
                    &[self.ctx.i64_type().const_int(self.abi.svc_host, false)],
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
    pub(crate) fn term_br(&mut self, target: BasicBlock<'ctx>) {
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

    pub(crate) fn stmt(&mut self, ctx: &Ctx, s: &Stmt) {
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
                if matches!(rhs.ty, Type::Array { .. }) && matches!(rhs.kind, ExprKind::Ref(_)) {
                    let ExprKind::Ref(r) = &rhs.kind else {
                        unreachable!()
                    };
                    let (sp, _) = self.resolve(ctx, r);
                    let (dp, dty) = self.resolve(ctx, lhs);
                    self.b
                        .build_memcpy(
                            dp,
                            1,
                            sp,
                            1,
                            self.ctx
                                .i64_type()
                                .const_int(rhs.ty.size().min(dty.size()), false),
                        )
                        .unwrap();
                } else {
                    let v = self.expr(ctx, rhs);
                    self.write(ctx, lhs, v, &rhs.ty);
                }
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
                self.write(ctx, &vr, sv, &start.ty);
                let vt = self.read(ctx, &vr).into_int_value().get_type();
                // step/end are arbitrary int widths — normalize to the
                // loop variable's type before compare/add
                let step_e = step
                    .as_ref()
                    .map(|e| self.expr(ctx, e).into_int_value())
                    .unwrap_or(vt.const_int(1, true));
                let end_v = self.expr(ctx, end).into_int_value();
                let step_e = self.b.build_int_cast(step_e, vt, "fs").unwrap();
                let end_v = self.b.build_int_cast(end_v, vt, "fe").unwrap();
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
            StmtKind::Return(e) => {
                if let Some(e) = e {
                    if let Some(fname) = self.cur_fn.clone() {
                        // FUNCTION: result lives in the member named
                        // after the pou (same convention as the
                        // function-call site)
                        let (rp, rty) =
                            self.resolve(ctx, &Ref::Var(fname));
                        let v = self.expr(ctx, e);
                        let v = self.coerce(v, &e.ty, &rty);
                        self.b.build_store(rp, v).unwrap();
                    }
                }
                self.b
                    .build_unconditional_branch(
                        self.retok_bb.unwrap_or_else(|| panic!("return outside fn")),
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

    pub(crate) fn stmts(&mut self, ctx: &Ctx, ss: &[Stmt]) {
        for s in ss {
            self.stmt(ctx, s);
        }
    }

}
