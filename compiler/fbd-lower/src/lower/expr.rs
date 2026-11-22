//! pin expr resolution + op typing/coercion — split out of lower.rs.
use super::*;

impl<'a> Cx<'a> {
    pub(super) fn fix_probe_ty(&mut self, name: &str, ty: &Type) {
        if let Some(&idx) = self.var_by_name.get(&name.to_ascii_uppercase()) {
            self.vars[idx].ty = ty.clone();
        }
    }


    /// Map a dst `PortSel` on `b` to an input index.
    pub(super) fn in_index(&self, b: &Block, sel: &PortSel) -> u32 {
        match sel {
            PortSel::Index(i) => *i,
            PortSel::Name(n) => {
                let un = n.to_ascii_uppercase();
                if un == "EN" {
                    return u32::MAX;
                }
                match b.kind {
                    BlockKind::Fb | BlockKind::Function | BlockKind::Device => self
                        .sigs
                        .get(&b.id)
                        .and_then(|s| {
                            s.ins
                                .iter()
                                .position(|(pn, _)| pn.eq_ignore_ascii_case(&un))
                                .map(|i| i as u32)
                        })
                        .unwrap_or(u32::MAX),
                    BlockKind::Operator => op_kind(&b.type_ref)
                        .and_then(|op| op_in_index(op, &un))
                        .unwrap_or(u32::MAX),
                    _ => 0,
                }
            }
        }
    }

    /// Name of input pin `want` on `b` (for param lookup).
    pub(super) fn in_name(&self, b: &Block, want: u32) -> Option<String> {
        match b.kind {
            BlockKind::Fb | BlockKind::Function | BlockKind::Device => self
                .sigs
                .get(&b.id)
                .and_then(|s| s.ins.get(want as usize))
                .map(|(n, _)| n.clone()),
            BlockKind::Operator => op_kind(&b.type_ref).map(|op| op_in_name(op, want)),
            _ => None,
        }
    }

    /// Declared type of input pin `want` on `b`.
    pub(super) fn in_ty(&self, b: &Block, want: u32) -> Option<Type> {
        self.sigs
            .get(&b.id)
            .and_then(|s| s.ins.get(want as usize))
            .map(|(_, t)| t.clone())
    }

    /// Expr for a source port (`wire.src`).
    pub(super) fn src_expr(
        &mut self,
        pref: &PortRef,
        net: &Network,
        blocks: &BTreeMap<NodeId, &Block>,
    ) -> Option<Expr> {
        let b = blocks.get(&pref.block).copied()?;
        match b.kind {
            BlockKind::Variable => {
                // ladder wire topology: contacts/coils carry gated
                // flow, not the raw variable — lower_ladder_wired
                // populates ld_gated in topo order before consumers
                // resolve. Unwired blocks fall back to the raw ref.
                if net.kind == NetworkKind::Ladder {
                    if let Some(g) = self.ld_gated.get(&b.id) {
                        return Some(g.clone());
                    }
                }
                if let Some(r) = self.var_ref(b) {
                    let ty = self.ref_ty(&r);
                    return Some(Expr {
                        kind: ExprKind::Ref(r),
                        ty,
                        prov: self.prov(b.id, 0),
                    });
                }
                // constant tap — a PLCopen variable element whose
                // expression is a literal (e.g. Beremiz inVariable
                // "1") sources its value directly, no storage
                if let Some(n) = &b.name {
                    if let Ok(l) = parse_lit(n) {
                        let ty = lit_ty(&l);
                        return Some(Expr {
                            kind: ExprKind::Lit(l),
                            ty,
                            prov: self.prov(b.id, 0),
                        });
                    }
                }
                None
            }
            BlockKind::Fb => {
                let sig = self.sigs.get(&b.id).cloned().unwrap_or_default();
                let i = match &pref.port {
                    PortSel::Index(i) => *i,
                    PortSel::Name(n) => sig
                        .outs
                        .iter()
                        .position(|(pn, _)| pn.eq_ignore_ascii_case(n))
                        .map(|i| i as u32)
                        .unwrap_or(u32::MAX),
                };
                if i as usize >= sig.outs.len() {
                    // past-the-end index or ENO → the eno flag var
                    let eno = self.eno_var(b.id);
                    return Some(Expr {
                        kind: ExprKind::Ref(Ref::Var(eno)),
                        ty: Type::Bool,
                        prov: self.prov(b.id, i as u16),
                    });
                }
                let inst = self.inst_by_block.get(&b.id)?.clone();
                let (pn, pty) = sig.outs[i as usize].clone();
                Some(Expr {
                    kind: ExprKind::Ref(Ref::Field(Box::new(Ref::Var(inst)), pn)),
                    ty: pty,
                    prov: self.prov(b.id, i as u16),
                })
            }
            BlockKind::Device => {
                let sig = self.sigs.get(&b.id).cloned().unwrap_or_default();
                let i = match &pref.port {
                    PortSel::Index(i) => *i,
                    PortSel::Name(n) => sig
                        .outs
                        .iter()
                        .position(|(pn, _)| pn.eq_ignore_ascii_case(n))
                        .map(|i| i as u32)
                        .unwrap_or(u32::MAX),
                };
                let (pn, pty) = sig.outs.get(i as usize)?.clone();
                let &vi = self.dev_vars.get(&(b.id, pn.to_ascii_uppercase()))?;
                Some(Expr {
                    kind: ExprKind::Ref(Ref::Var(self.vars[vi].name.clone())),
                    ty: pty,
                    prov: self.prov(b.id, i as u16),
                })
            }
            BlockKind::Function => {
                if !self.expanding.insert(b.id) {
                    self.err(codes::ALG_CYCLE, format!("cycle at node {}", b.id), b.id);
                    return None;
                }
                let sig = self.sigs.get(&b.id).cloned().unwrap_or_default();
                let mut args = Vec::new();
                for (i, (pn, _)) in sig.ins.iter().enumerate() {
                    match self.dst_input(net, b, i as u32, blocks) {
                        Some(e) => args.push((pn.clone(), e)),
                        None => self.err(
                            codes::UNCONNECTED,
                            format!("{} input {pn} unconnected", b.type_ref),
                            b.id,
                        ),
                    }
                }
                self.expanding.remove(&b.id);
                // out pin 0 of a FUNCTION sig is the return value
                let ret = sig
                    .outs
                    .first()
                    .map(|(_, t)| t.clone())
                    .unwrap_or(Type::Bool);
                Some(Expr {
                    kind: ExprKind::Call {
                        fun: b.type_ref.clone(),
                        args,
                    },
                    ty: ret,
                    prov: self.prov(b.id, 0),
                })
            }
            BlockKind::Operator => self.op_expr(net, b, blocks),
            BlockKind::JumpTarget => None,
            BlockKind::St => {
                self.err(
                    codes::UNSUPPORTED,
                    "st block not expanded — use expand_st_blocks (tplcc mixed mode)",
                    b.id,
                );
                None
            }
        }
    }

    /// Operator block → expr (pure fold or materialized temp ref).
    pub(super) fn op_expr(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
    ) -> Option<Expr> {
        if !self.expanding.insert(b.id) {
            self.err(codes::ALG_CYCLE, format!("cycle at node {}", b.id), b.id);
            return None;
        }
        let tref = b.type_ref.clone();
        let p = self.prov(b.id, 0);
        let r = (|| {
            if let Some(to) = cast_target(&tref) {
                let e = self.dst_input(net, b, 0, blocks)?;
                let ty = parse_type(to, self.fb_names).ok()?;
                return Some(Expr {
                    kind: ExprKind::Conv {
                        to: ty.clone(),
                        e: Box::new(e),
                    },
                    ty,
                    prov: p.clone(),
                });
            }
            let op = op_kind(&tref)?;
            match op {
                Op::Sel | Op::Min | Op::Max | Op::Limit | Op::Abs | Op::Mux => {
                    // temp materialized by emit_op
                    Some(Expr {
                        kind: ExprKind::Ref(Ref::Var(format!("__op_{}", b.id))),
                        ty: Type::Bool,
                        prov: p.clone(),
                    })
                }
                Op::Move => self.dst_input(net, b, 0, blocks),
                Op::Not => {
                    let e = self.dst_input(net, b, 0, blocks)?;
                    if !matches!(e.ty, Type::Bool) {
                        self.err(
                            codes::ANY_UNRESOLVED,
                            format!("{} input 0 needs BOOL", b.type_ref),
                            b.id,
                        );
                    }
                    Some(Expr {
                        kind: ExprKind::Un(UnOp::Not, Box::new(e)),
                        ty: Type::Bool,
                        prov: p.clone(),
                    })
                }
                Op::Fold(binop) | Op::Bin(binop) => {
                    let ins = self.op_inputs(net, b, blocks);
                    let mut exprs = Vec::new();
                    for i in &ins {
                        match self.dst_input(net, b, *i, blocks) {
                            Some(e) => exprs.push(e),
                            None => self.err(
                                codes::UNCONNECTED,
                                format!("{} input {i} unconnected", b.type_ref),
                                b.id,
                            ),
                        }
                    }
                    if let Some(2) = op_arity(op) {
                        if exprs.len() != 2 {
                            self.err(
                                codes::UNCONNECTED,
                                format!("{} needs 2 inputs, has {}", b.type_ref, exprs.len()),
                                b.id,
                            );
                        }
                    }
                    self.check_op_class(op, b, &exprs);
                    let mut it = exprs.into_iter();
                    let mut acc = it.next()?;
                    for e in it {
                        let ty = unify_num(&acc.ty, &e.ty);
                        acc = Expr {
                            kind: ExprKind::Bin(binop, Box::new(acc), Box::new(e)),
                            ty,
                            prov: p.clone(),
                        };
                    }
                    Some(acc)
                }
            }
        })();
        self.expanding.remove(&b.id);
        r
    }

    /// Operator input exprs + result type for If-expansion ops.
    /// `exprs[i]` is pin index i (positional — gaps are FBD002 errors
    /// filled with a literal-0 placeholder so lowering can continue).
    pub(super) fn typed_inputs(
        &mut self,
        net: &Network,
        b: &Block,
        blocks: &BTreeMap<NodeId, &Block>,
        op: Op,
        ins: &BTreeSet<u32>,
    ) -> Option<(Vec<Expr>, Type)> {
        // required pin count: fixed-arity ops or the highest wired pin
        let need = match op_arity(op) {
            Some(n) => n,
            None => ins.iter().max().copied().unwrap_or(0) + 1,
        };
        let mut exprs: Vec<Expr> = Vec::new();
        for i in 0..need {
            match self.dst_input(net, b, i, blocks) {
                Some(e) => exprs.push(e),
                None => {
                    self.err(
                        codes::UNCONNECTED,
                        format!("{} input {i} unconnected", b.type_ref),
                        b.id,
                    );
                    exprs.push(Expr {
                        kind: ExprKind::Lit(Lit::Int(0)),
                        ty: Type::Bool,
                        prov: self.prov(b.id, i as u16),
                    });
                }
            }
        }
        if exprs.is_empty() {
            return None;
        }
        self.check_op_class(op, b, &exprs);
        let rty = match op {
            Op::Sel | Op::Mux => exprs
                .iter()
                .skip(1)
                .map(|e| e.ty.clone())
                .reduce(|a, t| unify_num(&a, &t))
                .unwrap_or(Type::Bool),
            Op::Min | Op::Max => exprs
                .iter()
                .map(|e| e.ty.clone())
                .reduce(|a, t| unify_num(&a, &t))
                .unwrap_or(Type::Bool),
            Op::Limit => exprs[1].ty.clone(),
            Op::Abs => exprs[0].ty.clone(),
            _ => Type::Bool,
        };
        Some((exprs, rty))
    }

    /// FBD009 — generic `ANY_*` pins must resolve to a type in the
    /// operator's class: BOOL ops take BOOL, arith ops take
    /// Int/Real/Time, shifts take Int, SEL.G is BOOL, MUX.K is Int.
    pub(super) fn check_op_class(&mut self, op: Op, b: &Block, exprs: &[Expr]) {
        // literals classify by payload — param literals carry a Bool
        // placeholder ty until coerced at the pin
        let num = |e: &Expr| match &e.kind {
            ExprKind::Lit(Lit::Int(_) | Lit::UInt(_) | Lit::Real(_) | Lit::TimeNs(_)) => true,
            ExprKind::Lit(_) => false,
            _ => matches!(e.ty, Type::Int { .. } | Type::Real { .. } | Type::Time),
        };
        let int = |e: &Expr| match &e.kind {
            ExprKind::Lit(Lit::Int(_) | Lit::UInt(_)) => true,
            ExprKind::Lit(_) => false,
            _ => matches!(e.ty, Type::Int { .. }),
        };
        let boolean = |e: &Expr| match &e.kind {
            ExprKind::Lit(Lit::Bool(_)) => true,
            ExprKind::Lit(_) => false,
            _ => matches!(e.ty, Type::Bool),
        };
        let mut bad = |cond: bool, pin: usize, want: &str| {
            if cond {
                self.err(
                    codes::ANY_UNRESOLVED,
                    format!("{} input {pin} needs {want}", b.type_ref),
                    b.id,
                );
            }
        };
        match op {
            Op::Fold(BinOp::And | BinOp::Or | BinOp::Xor) | Op::Not => {
                for (i, e) in exprs.iter().enumerate() {
                    bad(!boolean(e), i, "BOOL");
                }
            }
            Op::Bin(BinOp::Shl | BinOp::Shr | BinOp::Rol | BinOp::Ror) => {
                for (i, e) in exprs.iter().enumerate() {
                    bad(!int(e), i, "ANY_INT");
                }
            }
            Op::Sel => {
                for (i, e) in exprs.iter().enumerate() {
                    if i == 0 {
                        bad(!boolean(e), i, "BOOL");
                    } else {
                        bad(!num(e), i, "ANY_NUM");
                    }
                }
            }
            Op::Mux => {
                for (i, e) in exprs.iter().enumerate() {
                    if i == 0 {
                        bad(!int(e), i, "ANY_INT");
                    } else {
                        bad(!num(e), i, "ANY_NUM");
                    }
                }
            }
            // ANY operand ops: EQ/NE accept every same-class pair
            Op::Move | Op::Bin(BinOp::Eq | BinOp::Ne) => {}
            _ => {
                for (i, e) in exprs.iter().enumerate() {
                    bad(!num(e), i, "ANY_NUM");
                }
            }
        }
    }

    /// `eno` internal var for an fb block (lazily declared).
    pub(super) fn eno_var(&mut self, blk: NodeId) -> String {
        let name = format!("__eno_{blk}");
        if !self.var_by_name.contains_key(&name.to_ascii_uppercase()) {
            self.declare(VarDecl {
                name: name.clone(),
                kind: VarKind::Local,
                ty: Type::Bool,
                init: None,
                retain: false,
                at: None,
                probe: false,
                prov: Prov::Generated,
            });
        }
        name
    }

    /// `Ref` for a variable block's storage.
    pub(super) fn var_ref(&mut self, b: &Block) -> Option<Ref> {
        if let Some(&idx) = self.var_by_block.get(&b.id) {
            return Some(Ref::Var(self.vars[idx].name.clone()));
        }
        let n = b.name.as_ref()?;
        if let Some(&idx) = self.var_by_name.get(&n.to_ascii_uppercase()) {
            return Some(Ref::Var(self.vars[idx].name.clone()));
        }
        self.gvl_member(n)
            .map(|(g, m)| Ref::Var(format!("{g}.{m}")))
    }

    pub(super) fn ref_ty(&self, r: &Ref) -> Type {
        match r {
            Ref::Var(n) => {
                if let Some(&i) = self.var_by_name.get(&n.to_ascii_uppercase()) {
                    return self.vars[i].ty.clone();
                }
                // GVL-qualified ref ("GVL.x") — type from the project
                // global list
                if let Some((g, m)) = n.split_once('.') {
                    if g.eq_ignore_ascii_case("gvl") {
                        for gv in &self.project.global_vars {
                            if gv.name.eq_ignore_ascii_case(m) {
                                return parse_type(&gv.ty, self.fb_names)
                                    .unwrap_or(Type::Bool);
                            }
                        }
                    }
                }
                Type::Bool
            }
            Ref::Field(inner, m) => {
                if let Ref::Var(g) = inner.as_ref() {
                    if g.eq_ignore_ascii_case("GVL") {
                        for gv in &self.project.global_vars {
                            if gv.name.eq_ignore_ascii_case(m) {
                                return parse_type(&gv.ty, self.fb_names).unwrap_or(Type::Bool);
                            }
                        }
                    }
                }
                Type::Bool
            }
            Ref::Index(inner, _) => self.ref_ty(inner),
            Ref::Deref(inner) => match self.ref_ty(inner) {
                Type::Ref { to } => *to,
                _ => Type::Bool,
            },
        }
    }

    /// FBD001 check: src expr → dst pin type must be coercible.
    pub(super) fn check_coerce(&mut self, e: &Expr, want: &Type, node: u64) {
        let ok = compatible(&e.ty, want) || convertible(&e.ty, want) || literal_fits(e, want);
        if !ok {
            self.diags.push(
                Diag::error(
                    codes::TYPE_MISMATCH,
                    format!("expected {want:?}, found {:?}", e.ty),
                    self.prov(node, 0),
                )
                .with_help("no implicit conversion — insert a cast block"),
            );
        }
    }
}
