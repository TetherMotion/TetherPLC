//! block/FB/device emission — split out of lower.rs.
use super::*;

impl<'a> Cx<'a> {
    /// Emit one block's statements in topo position.
    pub(super) fn emit_block(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        stmts: &mut Vec<Stmt>,
    ) {
        match b.kind {
            BlockKind::Variable => self.emit_var_block(net, b, blocks, stmts),
            BlockKind::Device => self.emit_device(net, b, blocks, stmts),
            BlockKind::Fb => self.emit_fb(net, b, blocks, stmts),
            BlockKind::Operator => self.emit_op(net, b, blocks, stmts),
            BlockKind::Function | BlockKind::JumpTarget => {}
            BlockKind::St => self.err(
                codes::UNSUPPORTED,
                "st block not expanded — use expand_st_blocks (tplcc mixed mode)",
                b.id,
            ),
        }
    }

    /// variable out/inout → Assign to the resolved storage.
    pub(super) fn emit_var_block(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        stmts: &mut Vec<Stmt>,
    ) {
        if matches!(b.role, Some(VarRole::In)) {
            return;
        }
        let Some(lhs) = self.var_ref(b) else {
            return;
        };
        let Some(e) = self.dst_input(net, b, 0, blocks) else {
            self.warn(codes::ORPHAN, "out variable has no input", b.id);
            return;
        };
        let lt = self.ref_ty(&lhs);
        self.check_coerce(&e, &lt, b.id);
        stmts.push(Stmt {
            kind: StmtKind::Assign { lhs, rhs: e },
            prov: self.prov(b.id, 0),
        });
    }

    /// device block → Assign per driven input channel. Output
    /// channels need no code — consumers read the located var
    /// directly through `src_expr`.
    pub(super) fn emit_device(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        stmts: &mut Vec<Stmt>,
    ) {
        let sig = self.sigs.get(&b.id).cloned().unwrap_or_default();
        for (i, (pname, pty)) in sig.ins.iter().enumerate() {
            let Some(e) = self.dst_input(net, b, i as u32, blocks) else {
                continue; // undriven channel holds its image value
            };
            let Some(&vi) = self.dev_vars.get(&(b.id, pname.to_ascii_uppercase())) else {
                continue;
            };
            let lhs = Ref::Var(self.vars[vi].name.clone());
            self.check_coerce(&e, &pty, b.id);
            stmts.push(Stmt {
                kind: StmtKind::Assign { lhs, rhs: e },
                prov: self.prov(b.id, i as u16),
            });
        }
    }

    /// fb block → Call stmt (EN-gated when EN is bound).
    pub(super) fn emit_fb(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        stmts: &mut Vec<Stmt>,
    ) {
        let Some(inst) = self.inst_by_block.get(&b.id).cloned() else {
            return;
        };
        let sig = self.sigs.get(&b.id).cloned().unwrap_or_default();
        let mut args: Vec<(String, Expr)> = Vec::new();
        for (i, (pname, pty)) in sig.ins.iter().enumerate() {
            match self.dst_input(net, b, i as u32, blocks) {
                Some(e) => {
                    if sig.inouts.contains(&(i as u32)) {
                        match &e.kind {
                            // storage ref — legal inout binding
                            ExprKind::Ref(_) => {}
                            ExprKind::Lit(_) => self.err(
                                codes::INOUT_STORAGE,
                                format!("inout pin {pname} bound to literal"),
                                b.id,
                            ),
                            _ => self.err(
                                codes::INOUT_EXPR,
                                format!("inout pin {pname} fed by expression"),
                                b.id,
                            ),
                        }
                    }
                    self.check_coerce(&e, pty, b.id);
                    args.push((pname.clone(), e));
                }
                None => {
                    if let Some(p) = b.params.get(pname) {
                        match parse_lit(p) {
                            Ok(l) => args.push((
                                pname.clone(),
                                Expr {
                                    kind: ExprKind::Lit(l),
                                    ty: pty.clone(),
                                    prov: self.prov(b.id, i as u16),
                                },
                            )),
                            Err(e) => self.err(codes::PARSE, format!("param {pname}: {e}"), b.id),
                        }
                    }
                    // unbound fb inputs hold their blob value (default)
                }
            }
        }
        // EN: named pin or the index just past the last in-pin
        let en = self
            .dst_input(net, b, sig.ins.len() as u32, blocks)
            .or_else(|| self.dst_input(net, b, u32::MAX, blocks));
        let mut body = vec![Stmt {
            kind: StmtKind::Call {
                inst: Ref::Var(inst.clone()),
                args,
            },
            prov: self.prov(b.id, 0),
        }];
        // ENO var holds EN result for wired consumers
        if self.eno_used(net, b, &sig) {
            let eno = self.eno_var(b.id);
            let p = self.prov(b.id, 0);
            let set_eno = |v: bool| Stmt {
                kind: StmtKind::Assign {
                    lhs: Ref::Var(eno.clone()),
                    rhs: Expr {
                        kind: ExprKind::Lit(Lit::Bool(v)),
                        ty: Type::Bool,
                        prov: p.clone(),
                    },
                },
                prov: p.clone(),
            };
            match en {
                Some(cond) => stmts.push(Stmt {
                    kind: StmtKind::If {
                        branches: vec![(cond, {
                            let mut v = body;
                            v.push(set_eno(true));
                            v
                        })],
                        else_: vec![set_eno(false)],
                    },
                    prov: self.prov(b.id, 0),
                }),
                None => {
                    stmts.append(&mut body);
                    stmts.push(set_eno(true));
                }
            }
            return;
        }
        match en {
            Some(cond) => stmts.push(Stmt {
                kind: StmtKind::If {
                    branches: vec![(cond, body)],
                    else_: vec![],
                },
                prov: self.prov(b.id, 0),
            }),
            None => stmts.append(&mut body),
        }
    }

    /// Does any wire/assoc read this fb's ENO (index ≥ outs.len()
    /// or a pin named ENO)?
    pub(super) fn eno_used(&self, net: &Network, b: &Block, sig: &Sig) -> bool {
        net.wires.iter().any(|w| {
            w.src.block == b.id
                && match &w.src.port {
                    PortSel::Index(i) => *i as usize >= sig.outs.len(),
                    PortSel::Name(n) => n.eq_ignore_ascii_case("ENO"),
                }
        })
    }

    /// If-expansion operators emit their temp computation here; pure
    /// ops fold at consumers and emit nothing.
    pub(super) fn emit_op(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        stmts: &mut Vec<Stmt>,
    ) {
        let Some(op) = op_kind(&b.type_ref) else {
            if cast_target(&b.type_ref).is_none() {
                self.err(
                    codes::UNKNOWN,
                    format!("unknown operator {}", b.type_ref),
                    b.id,
                );
            }
            return;
        };
        if !matches!(
            op,
            Op::Sel | Op::Min | Op::Max | Op::Limit | Op::Abs | Op::Mux
        ) {
            return;
        }
        let ins = self.op_inputs(net, b, blocks);
        let Some((exprs, rty)) = self.typed_inputs(net, b, blocks, op, &ins) else {
            return;
        };
        let tname = format!("__op_{}", b.id);
        self.declare(VarDecl {
            name: tname.clone(),
            kind: VarKind::Local,
            ty: rty,
            init: None,
            retain: false,
            at: None,
            probe: false,
            prov: self.prov(b.id, 0),
        });
        let p = self.prov(b.id, 0);
        let lhs = Ref::Var(tname);
        let assign = |e: Expr| Stmt {
            kind: StmtKind::Assign {
                lhs: lhs.clone(),
                rhs: e,
            },
            prov: p.clone(),
        };
        let lit0 = || Expr {
            kind: ExprKind::Lit(Lit::Int(0)),
            ty: Type::Int {
                signed: true,
                bits: 32,
            },
            prov: p.clone(),
        };
        let e = |i: usize| exprs.get(i).cloned().unwrap_or_else(lit0);
        let cmp = |op: BinOp, a: Expr, b2: Expr| Expr {
            kind: ExprKind::Bin(op, Box::new(a), Box::new(b2)),
            ty: Type::Bool,
            prov: p.clone(),
        };
        match op {
            Op::Sel => stmts.push(Stmt {
                kind: StmtKind::If {
                    branches: vec![(e(0), vec![assign(e(2))])],
                    else_: vec![assign(e(1))],
                },
                prov: p,
            }),
            Op::Min | Op::Max => {
                let cmpop = if op == Op::Min { BinOp::Lt } else { BinOp::Gt };
                stmts.push(assign(e(0)));
                for (i, _) in exprs.iter().enumerate().skip(1) {
                    stmts.push(Stmt {
                        kind: StmtKind::If {
                            branches: vec![(
                                cmp(
                                    cmpop,
                                    e(i),
                                    Expr {
                                        kind: ExprKind::Ref(lhs.clone()),
                                        ty: exprs[i].ty.clone(),
                                        prov: p.clone(),
                                    },
                                ),
                                vec![assign(e(i))],
                            )],
                            else_: vec![],
                        },
                        prov: p.clone(),
                    });
                }
            }
            Op::Limit => stmts.push(Stmt {
                kind: StmtKind::If {
                    branches: vec![
                        (cmp(BinOp::Lt, e(1), e(0)), vec![assign(e(0))]),
                        (cmp(BinOp::Gt, e(1), e(2)), vec![assign(e(2))]),
                    ],
                    else_: vec![assign(e(1))],
                },
                prov: p,
            }),
            Op::Abs => stmts.push(Stmt {
                kind: StmtKind::If {
                    branches: vec![(
                        cmp(BinOp::Lt, e(0), lit0()),
                        vec![assign(Expr {
                            kind: ExprKind::Un(UnOp::Neg, Box::new(e(0))),
                            ty: exprs.first().map(|x| x.ty.clone()).unwrap_or(Type::Bool),
                            prov: p.clone(),
                        })],
                    )],
                    else_: vec![assign(e(0))],
                },
                prov: p,
            }),
            Op::Mux => {
                let mut branches = Vec::new();
                for i in 1..exprs.len() {
                    branches.push((
                        cmp(
                            BinOp::Eq,
                            e(0),
                            Expr {
                                kind: ExprKind::Lit(Lit::Int((i - 1) as i64)),
                                ty: Type::Int {
                                    signed: true,
                                    bits: 32,
                                },
                                prov: p.clone(),
                            },
                        ),
                        vec![assign(e(i))],
                    ));
                }
                stmts.push(Stmt {
                    kind: StmtKind::If {
                        branches,
                        // K out of range → first input (documented v1 rule)
                        else_: vec![assign(e(1))],
                    },
                    prov: p,
                });
            }
            _ => {}
        }
    }

    /// Ordered input indexes present on an operator block.
    pub(super) fn op_inputs(
        &mut self,
        net: &Network,
        b: &Block,
        _blocks: &BTreeMap<NodeId, &Block>,
    ) -> BTreeSet<u32> {
        let mut set = BTreeSet::new();
        for w in &net.wires {
            for d in &w.dsts {
                if d.block == b.id {
                    set.insert(self.in_index(b, &d.port));
                }
            }
        }
        for a in &net.assocs {
            if a.dst.block == b.id {
                set.insert(self.in_index(b, &a.dst.port));
            }
        }
        if let Some(op) = op_kind(&b.type_ref) {
            for pname in b.params.keys() {
                if let Some(i) = op_in_index(op, &pname.to_ascii_uppercase()) {
                    set.insert(i);
                }
            }
        }
        set
    }

    /// `dst_input`: the expr feeding `b`'s input index `want` — wire,
    /// assoc, or param literal. Reports FBD002/003.
    pub(super) fn dst_input(
        &mut self,
        net: &Network,
        b: &Block,
        want: u32,
        blocks: &BTreeMap<NodeId, &Block>,
    ) -> Option<Expr> {
        let mut srcs: Vec<Expr> = Vec::new();
        for w in &net.wires {
            for d in &w.dsts {
                if d.block != b.id || self.in_index(b, &d.port) != want {
                    continue;
                }
                let Some(mut e) = self.src_expr(&w.src, net, blocks) else {
                    continue;
                };
                // probed wires read the probe field, not the fold
                if let Some(pn) = self.probes.get(&w.id) {
                    e = Expr {
                        kind: ExprKind::Ref(Ref::Var(pn.clone())),
                        ty: e.ty.clone(),
                        prov: self.prov(w.id, 0),
                    };
                }
                if b.negated.contains(&want) {
                    e = Expr {
                        kind: ExprKind::Un(UnOp::Not, Box::new(e)),
                        ty: Type::Bool,
                        prov: self.prov(d.block, want as u16),
                    };
                }
                srcs.push(e);
            }
        }
        for a in &net.assocs {
            if a.dst.block != b.id || self.in_index(b, &a.dst.port) != want {
                continue;
            }
            let un = a.src_name.to_ascii_uppercase();
            let r = if let Some(&idx) = self.var_by_name.get(&un) {
                Ref::Var(self.vars[idx].name.clone())
            } else if let Some((g, m)) = self.gvl_member(&a.src_name) {
                Ref::Var(format!("{g}.{m}"))
            } else {
                self.err(
                    codes::UNDECLARED,
                    format!("assoc source {}", a.src_name),
                    b.id,
                );
                continue;
            };
            let ty = self.ref_ty(&r);
            srcs.push(Expr {
                kind: ExprKind::Ref(r),
                ty,
                prov: self.prov(a.dst.block, want as u16),
            });
        }
        if srcs.len() > 1 {
            self.err(
                codes::FAN_IN,
                format!("input {want} has {} sources", srcs.len()),
                b.id,
            );
        }
        if let Some(e) = srcs.into_iter().next() {
            return Some(e);
        }
        let pname = self.in_name(b, want);
        if let Some(p) = pname.as_ref().and_then(|n| b.params.get(n)) {
            let want_ty = self.in_ty(b, want);
            return match parse_lit(p) {
                Ok(l) => Some(Expr {
                    kind: ExprKind::Lit(l.clone()),
                    ty: want_ty.unwrap_or_else(|| lit_ty(&l)),
                    prov: self.prov(b.id, want as u16),
                }),
                Err(e) => {
                    self.err(codes::PARSE, format!("param: {e}"), b.id);
                    None
                }
            };
        }
        None
    }


}
