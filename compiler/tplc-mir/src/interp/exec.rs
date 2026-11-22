//! exec — split from interp.rs.
use super::*;

impl<'a> Rt<'a> {
    // ---- statements ----
    pub fn exec(&mut self, ctx: &Ctx, s: &Stmt) -> Flow {
        let f = self.exec_inner(ctx, s);
        match self.fault {
            Some(code) => Flow::Abort(code),
            None => f,
        }
    }

    pub(crate) fn exec_inner(&mut self, ctx: &Ctx, s: &Stmt) -> Flow {
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

    pub(crate) fn abort_check(&self) -> bool {
        self.abort_flag.map(|f| f()).unwrap_or(false)
    }

    /// FB-instance invocation. Builtins run spec semantics on the
    /// instance sub-object; user FBs recurse into their POU body.
    pub(crate) fn fb_call(&mut self, ctx: &Ctx, inst: &Ref, args: &[(String, Expr)]) -> Flow {
        let (off, ty) = self.resolve(ctx, inst);
        let fbname = match &ty {
            Type::Fb { name } => name.clone(),
            _ => panic!("call on non-FB ref"),
        };
        let fbl = self.member_of(&fbname);
        // VAR_IN_OUT params of user FBs alias the bound lvalue — record
        // the operand's (offset, type) and remap the name in the callee
        // ctx below rather than copying through a member.
        let inouts: std::collections::HashMap<String, String> = self
            .module
            .pous
            .iter()
            .find(|p| p.name == fbname && p.kind == PouKind::FunctionBlock)
            .map(|p| {
                p.iface
                    .iter()
                    .filter(|v| v.kind == VarKind::InOut)
                    .map(|v| (v.name.to_ascii_uppercase(), v.name.clone()))
                    .collect()
            })
            .unwrap_or_default();
        let mut io_binds: Vec<(String, u64, Type)> = Vec::new();
        // bind named args to instance fields (evaluate in caller ctx)
        for (pname, pe) in args {
            if let Some(decl) = inouts.get(&pname.to_ascii_uppercase()) {
                let ExprKind::Ref(r) = &pe.kind else {
                    panic!("inout {pname} bound to non-reference")
                };
                let (o, t) = self.resolve(ctx, r);
                io_binds.push((decl.clone(), o, t));
                continue;
            }
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
                        // inouts alias the caller's storage — unbound
                        // names fault on access like an unresolved var
                        for n in inouts.values() {
                            sub.remove(n.as_str());
                        }
                        for (n, o, t) in io_binds {
                            sub.insert(n, (o, t));
                        }
                        // RETURN/EXIT inside an FB body end only that
                        // invocation — the caller's scan continues.
                        // Abort (runtime fault) still propagates.
                        return match self.exec_all(&sub, stmts) {
                            Flow::Return(_) | Flow::Exit => Flow::Normal,
                            f => f,
                        };
                    }
                }
                Flow::Normal
            }
        }
    }

}
