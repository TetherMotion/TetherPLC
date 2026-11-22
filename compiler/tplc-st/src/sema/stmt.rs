//! stmt — split from sema.rs.
use super::*;

impl<'e> Checker<'e> {
    /// Statement list — tracks dead code after unconditional
    /// Return/Exit/Continue within the list.
    pub(crate) fn stmts(&mut self, list: &mut [Stmt]) {
        let mut dead = false;
        for s in list {
            if dead {
                self.diags.push(Diag::warning(
                    wcodes::DEAD_CODE,
                    "unreachable statement",
                    s.prov.clone(),
                ));
                // keep checking — more diags inside are still useful
            }
            self.stmt(s);
            dead |= matches!(
                s.kind,
                StmtKind::Return(_) | StmtKind::Exit | StmtKind::Continue
            );
        }
    }

    pub(crate) fn stmt(&mut self, s: &mut Stmt) {
        match &mut s.kind {
            StmtKind::Assign { lhs, rhs } => {
                let lt = self.ref_ty(lhs, &s.prov);
                let rt = self.expr(rhs);
                if let Some(lt) = lt {
                    self.coerce(rhs, &rt, &lt);
                }
            }
            StmtKind::If { branches, else_ } => {
                for (c, b) in branches {
                    self.cond(c);
                    self.stmts(b);
                }
                self.stmts(else_);
            }
            StmtKind::Case { sel, arms, else_ } => {
                self.expr(sel);
                let mut covered: Vec<(i64, i64)> = Vec::new();
                for (sels, b) in arms {
                    for cs in sels.iter_mut() {
                        match cs {
                            CaseSel::Lit(e) => {
                                self.expr(e);
                                if let Some(v) = const_int(e) {
                                    self.case_cover(&mut covered, v, v, &e.prov);
                                }
                            }
                            CaseSel::Range(a, b) => {
                                self.expr(a);
                                self.expr(b);
                                if let (Some(lo), Some(hi)) = (const_int(a), const_int(b)) {
                                    self.case_cover(&mut covered, lo, hi, &a.prov);
                                }
                            }
                        }
                    }
                    self.stmts(b);
                }
                self.stmts(else_);
            }
            StmtKind::For {
                var,
                start,
                end,
                step,
                body,
            } => {
                if self.var_ty(var).is_none() {
                    self.err(
                        codes::UNKNOWN_NAME,
                        format!("FOR counter {var} undeclared"),
                        &s.prov,
                    );
                }
                self.expr(start);
                self.expr(end);
                if let Some(st) = step {
                    self.expr(st);
                }
                self.stmts(body);
            }
            StmtKind::While { cond, body } | StmtKind::Repeat { cond, body } => {
                self.cond(cond);
                self.stmts(body);
            }
            StmtKind::Call { inst, args } => {
                let it = self.ref_ty(inst, &s.prov);
                let fbname = match &it {
                    Some(Type::Fb { name }) => Some(name.clone()),
                    _ => None,
                };
                if let Some(fbname) = fbname {
                    // arg names against FB iface / builtin members
                    let members: Vec<(String, Type)> = self
                        .env
                        .pous
                        .get(&fbname.to_ascii_uppercase())
                        .map(|p| {
                            p.iface
                                .iter()
                                .map(|v| (v.name.clone(), v.ty.clone()))
                                .collect()
                        })
                        .or_else(|| {
                            builtin_fb_layout(&fbname).map(|f| {
                                f.members
                                    .iter()
                                    .filter(|(n, _)| !n.starts_with("__"))
                                    .map(|(n, (_, t))| (n.clone(), t.clone()))
                                    .collect()
                            })
                        })
                        .unwrap_or_default();
                    for (pname, pe) in args.iter_mut() {
                        let rt = self.expr(pe);
                        if let Some((cn, ct)) =
                            members.iter().find(|(n, _)| n.eq_ignore_ascii_case(pname))
                        {
                            let (cn, ct) = (cn.clone(), ct.clone());
                            *pname = cn;
                            self.coerce(pe, &rt, &ct);
                        } else {
                            self.err(
                                codes::UNKNOWN_NAME,
                                format!("{fbname} has no input {pname}"),
                                &s.prov,
                            );
                        }
                    }
                } else {
                    self.err(codes::UNKNOWN_NAME, "call on non-FB-instance ref", &s.prov);
                }
            }
            StmtKind::Return(e) => {
                if let Some(x) = e {
                    self.expr(x);
                }
            }
            StmtKind::Exit | StmtKind::Continue => {
                self.diags.push(Diag::warning(
                    wcodes::NO_JUMPS,
                    "loop jump — prefer structured conditions",
                    s.prov.clone(),
                ));
            }
        }
    }

    /// Condition position: must be BOOL (IEC strict — no Int→Bool here).
    pub(crate) fn cond(&mut self, e: &mut Expr) {
        let t = self.expr(e);
        if t != Type::Bool {
            self.diags.push(Diag::error(
                codes::TYPE,
                format!("expected Bool condition, found {t:?}"),
                e.prov.clone(),
            ));
        }
    }

    /// Record a case-selector interval; overlap → unreachable-case lint.
    pub(crate) fn case_cover(&mut self, covered: &mut Vec<(i64, i64)>, lo: i64, hi: i64, prov: &Prov) {
        if covered.iter().any(|&(l, h)| lo <= h && l <= hi) {
            self.diags.push(Diag::warning(
                wcodes::UNREACHABLE_CASE,
                format!("case selector {lo}..{hi} overlaps an earlier arm"),
                prov.clone(),
            ));
        }
        covered.push((lo, hi));
    }

}
