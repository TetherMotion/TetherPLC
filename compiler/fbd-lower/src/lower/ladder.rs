//! network + LD/ladder lowering — split out of lower.rs.
use super::*;

impl<'a> Cx<'a> {
    /// First network pass: fb instance decls, sig resolution, located
    /// variable shadows, probe fields, exec_order collisions.
    pub(super) fn precollect(&mut self, net: &Network) {
        let mut orders: BTreeMap<u32, NodeId> = BTreeMap::new();
        for b in &net.blocks {
            if b.exec_order != 0 {
                if let Some(other) = orders.insert(b.exec_order, b.id) {
                    self.err(
                        codes::ORDER_COLLISION,
                        format!("exec_order {} on nodes {other} and {}", b.exec_order, b.id),
                        b.id,
                    );
                }
            }
            match b.kind {
                BlockKind::Fb => self.fb_decl(b),
                BlockKind::Function => self.fn_sig(b),
                BlockKind::Variable => self.var_decl(b),
                BlockKind::Device => self.device_decl(b),
                _ => {}
            }
        }
        for w in &net.wires {
            if w.probe {
                let name = format!("__probe_{}", w.id);
                self.probes.insert(w.id, name.clone());
                self.declare(VarDecl {
                    name,
                    kind: VarKind::Local,
                    ty: Type::Bool, // refined at store emission
                    init: None,
                    retain: false,
                    at: None,
                    probe: true,
                    prov: self.prov(w.id, 0),
                });
            }
        }
    }


    pub(super) fn lower_network(&mut self, net: &Network, stmts: &mut Vec<Stmt>) {
        if net.kind == NetworkKind::Ladder {
            self.lower_ladder(net, stmts);
            for r in &net.returns {
                stmts.push(Stmt {
                    kind: StmtKind::Return(None),
                    prov: self.prov(r.id, 0),
                });
            }
            return;
        }
        if net.blocks.is_empty() {
            return;
        }
        let order = match topo_order(net) {
            Ok(o) => o,
            Err(stuck) => {
                self.err(
                    codes::ALG_CYCLE,
                    format!("algebraic loop through nodes {stuck:?}"),
                    stuck[0],
                );
                return;
            }
        };
        let blocks: BTreeMap<NodeId, &Block> = net.blocks.iter().map(|b| (b.id, b)).collect();
        let mut effectful = 0usize;

        // jump checks (FBD011); jumps have no mir target today
        let labels: BTreeSet<&str> = net.labels.iter().map(|l| l.name.as_str()).collect();
        for j in &net.jumps {
            if labels.contains(j.target.as_str()) {
                self.err(
                    codes::UNSUPPORTED,
                    format!("jump to {} — goto lowering not implemented", j.target),
                    j.id,
                );
            } else {
                self.err(
                    codes::BAD_JUMP,
                    format!("jump to nonexistent label {}", j.target),
                    j.id,
                );
            }
        }

        for id in &order {
            let b = blocks[id];
            let pre = stmts.len();
            self.emit_block(net, b, &blocks, stmts);
            if stmts.len() > pre {
                effectful += 1;
            }
            // probe stores for wires sourced at this block (the value
            // just produced — matches "store after production", §4.3)
            for w in &net.wires {
                if w.src.block == *id && self.probes.contains_key(&w.id) {
                    if let Some(e) = self.src_expr(&w.src, net, &blocks) {
                        let pname = self.probes[&w.id].clone();
                        self.fix_probe_ty(&pname, &e.ty.clone());
                        stmts.push(Stmt {
                            kind: StmtKind::Assign {
                                lhs: Ref::Var(pname),
                                rhs: e,
                            },
                            prov: self.prov(w.id, 0),
                        });
                    }
                }
            }
        }

        // unconditional returns end the POU after this network's work
        for r in &net.returns {
            stmts.push(Stmt {
                kind: StmtKind::Return(None),
                prov: self.prov(r.id, 0),
            });
            effectful += 1;
        }
        if effectful == 0 {
            self.warn(codes::ORPHAN, "network produces no effects", 0);
        }
    }

    /// Ladder network → rung stmt sequence. Rungs are reconstructed
    /// positionally: blocks clustered by `pos.y` (one rung per
    /// horizontal band), ordered left→right by `pos.x`. Contacts
    /// (variable role `in`, NC when `negated`) AND into the rung
    /// flow; coils (role `out`) latch the flow — every coil on a rung
    /// sees the same power, matching parallel-output semantics.
    /// Wire topology drives rung semantics when present: bands linked
    /// by wires union into one rung evaluated as a signal graph —
    /// variable blocks are flow gates, fb/function/operator/device
    /// blocks lower through the FBD machinery. Wire-free bands keep
    /// positional series semantics for backwards compatibility.
    pub(super) fn lower_ladder(&mut self, net: &Network, stmts: &mut Vec<Stmt>) {
        self.ld_gated.clear();
        // rung bands: sort by y, split where the gap exceeds half the
        // smallest inter-band spacing (adaptive to editor/import scale)
        let mut ys: Vec<u32> = net.blocks.iter().map(|b| b.pos.y).collect();
        ys.sort_unstable();
        ys.dedup();
        let min_gap = ys
            .windows(2)
            .map(|w| w[1] - w[0])
            .filter(|&g| g > 0)
            .min()
            .unwrap_or(0);
        let tol = if min_gap > 0 {
            (min_gap / 2).clamp(4, 64)
        } else {
            24
        };
        let mut bands: Vec<Vec<&Block>> = Vec::new();
        let mut sorted: Vec<&Block> = net.blocks.iter().collect();
        sorted.sort_by_key(|b| (b.pos.y, b.pos.x));
        for b in sorted {
            match bands.last_mut() {
                Some(band) if b.pos.y.abs_diff(band[0].pos.y) <= tol => band.push(b),
                _ => bands.push(vec![b]),
            }
        }
        // wires merge bands: a parallel branch drawn on its own row
        // still belongs to the same rung as the contacts it ties to
        let band_of = |id: NodeId, bands: &[Vec<&Block>]| -> Option<usize> {
            bands.iter().position(|band| band.iter().any(|b| b.id == id))
        };
        let mut parent: Vec<usize> = (0..bands.len()).collect();
        fn find(p: &mut Vec<usize>, i: usize) -> usize {
            if p[i] != i {
                p[i] = find(p, p[i]);
            }
            p[i]
        }
        for w in &net.wires {
            let mut first: Option<usize> = None;
            for id in std::iter::once(w.src.block)
                .chain(w.dsts.iter().map(|d| d.block))
            {
                if let Some(bi) = band_of(id, &bands) {
                    match first {
                        None => first = Some(bi),
                        Some(f) => {
                            let (a, b) = (find(&mut parent, f), find(&mut parent, bi));
                            if a != b {
                                parent[b] = a;
                            }
                        }
                    }
                }
            }
        }
        let mut merged: BTreeMap<usize, Vec<&Block>> = BTreeMap::new();
        for (i, band) in bands.into_iter().enumerate() {
            merged.entry(find(&mut parent, i)).or_default().extend(band);
        }
        for mut band in merged.into_values() {
            band.sort_by_key(|b| b.pos.x);
            // a band participates in wire topology when any wire has
            // both endpoints inside it; otherwise positional series
            if self.band_has_wires(net, &band) {
                self.lower_ladder_wired(net, &band, stmts);
            } else {
                self.lower_ladder_positional(&band, stmts);
            }
        }
    }

    /// Positional rung: contacts AND left→right, coils latch the flow.
    pub(super) fn lower_ladder_positional(&mut self, band: &[&Block], stmts: &mut Vec<Stmt>) {
        {
            let mut flow = Expr {
                kind: ExprKind::Lit(Lit::Bool(true)),
                ty: Type::Bool,
                prov: self.prov(band[0].id, 0),
            };
            for &b in band {
                match (b.kind, b.role) {
                    (BlockKind::Variable, _) if is_data_var(b) => {
                        self.err(
                            codes::UNSUPPORTED,
                            format!(
                                "ladder data tap {} needs wire topology — \
                                 positional rungs carry power flow only",
                                b.id
                            ),
                            b.id,
                        );
                    }
                    (BlockKind::Variable, Some(VarRole::In)) => {
                        // NO/NC contact: flow AND (var [NOT])
                        let Some(r) = self.var_ref(b) else { continue };
                        let mut e = Expr {
                            kind: ExprKind::Ref(r),
                            ty: Type::Bool,
                            prov: self.prov(b.id, 0),
                        };
                        if !b.negated.is_empty() {
                            e = Expr {
                                kind: ExprKind::Un(UnOp::Not, Box::new(e)),
                                ty: Type::Bool,
                                prov: self.prov(b.id, 0),
                            };
                        }
                        flow = Expr {
                            kind: ExprKind::Bin(BinOp::And, Box::new(flow), Box::new(e)),
                            ty: Type::Bool,
                            prov: self.prov(b.id, 0),
                        };
                    }
                    (BlockKind::Variable, Some(VarRole::Out)) => {
                        // coil: latch rung flow (negated → NOT)
                        let Some(lhs) = self.var_ref(b) else { continue };
                        let rhs = if b.negated.is_empty() {
                            flow.clone()
                        } else {
                            Expr {
                                kind: ExprKind::Un(UnOp::Not, Box::new(flow.clone())),
                                ty: Type::Bool,
                                prov: self.prov(b.id, 0),
                            }
                        };
                        stmts.push(Stmt {
                            kind: StmtKind::Assign { lhs, rhs },
                            prov: self.prov(b.id, 0),
                        });
                    }
                    (BlockKind::Variable, _) => {
                        self.err(
                            codes::UNSUPPORTED,
                            format!("ladder element {} has unsupported role", b.id),
                            b.id,
                        );
                    }
                    (kind, _) => {
                        self.err(
                            codes::UNSUPPORTED,
                            format!(
                                "ladder element {kind:?} needs wire topology — \
                                 only contacts/coils lower positionally"
                            ),
                            b.id,
                        );
                    }
                }
            }
        }
    }

    /// True when any wire has an endpoint inside `band` — such bands
    /// lower by wire topology instead of positional x-order.
    pub(super) fn band_has_wires(&self, net: &Network, band: &[&Block]) -> bool {
        let ids: HashSet<NodeId> = band.iter().map(|b| b.id).collect();
        net.wires.iter().any(|w| {
            ids.contains(&w.src.block)
                || w.dsts.iter().any(|d| ids.contains(&d.block))
        })
    }

    /// Wired rung evaluation. Variable blocks (contacts/coils/
    /// in-outs) are power-flow gates: contacts publish
    /// `var [NOT] AND flow-in` (rail-powered when unwired), coils and
    /// in-outs latch their OR-merged flow-in. Fb/function/operator/
    /// device blocks are dataflow elements whose pins resolve through
    /// the same `dst_input`/`src_expr` machinery as FBD — a contact's
    /// gated signal is what flows onto data wires. Everything is
    /// evaluated in wire-graph topo order so producers emit before
    /// consumers; wire cycles are diagnosed.
    pub(super) fn lower_ladder_wired(
        &mut self,
        net: &Network,
        band: &[&Block],
        stmts: &mut Vec<Stmt>,
    ) {
        let ids: HashMap<NodeId, &Block> =
            band.iter().map(|b| (b.id, *b)).collect();
        let blocks_map: BTreeMap<NodeId, &Block> =
            band.iter().map(|b| (b.id, *b)).collect();
        // wires touching this band; cross-rung endpoints are an error.
        // `incoming` tracks flow dsts (variable blocks) with full src
        // PortRefs — data-pin dsts resolve through dst_input instead.
        let mut incoming: HashMap<NodeId, Vec<PortRef>> = HashMap::new();
        let mut wired: HashSet<NodeId> = HashSet::new();
        for w in &net.wires {
            let touches = ids.contains_key(&w.src.block)
                || w.dsts.iter().any(|d| ids.contains_key(&d.block));
            if !touches {
                continue;
            }
            if !ids.contains_key(&w.src.block)
                || w.dsts.iter().any(|d| !ids.contains_key(&d.block))
            {
                self.err(
                    codes::UNSUPPORTED,
                    format!("ladder wire {} crosses rung boundaries", w.id),
                    w.src.block,
                );
                continue;
            }
            let src = ids[&w.src.block];
            if matches!(src.kind, BlockKind::St | BlockKind::JumpTarget) {
                self.err(
                    codes::UNSUPPORTED,
                    format!("ladder wire source block {} cannot source", w.src.block),
                    w.src.block,
                );
                continue;
            }
            for d in &w.dsts {
                let dst = ids[&d.block];
                if matches!(dst.kind, BlockKind::St | BlockKind::JumpTarget) {
                    self.err(
                        codes::UNSUPPORTED,
                        format!(
                            "ladder wire destination block {} cannot sink",
                            d.block
                        ),
                        d.block,
                    );
                    continue;
                }
                // power-flow dsts merge into `incoming`; data taps
                // (flow=data) resolve their pins via dst_input like
                // any FBD variable block
                if dst.kind == BlockKind::Variable && !is_data_var(dst) {
                    incoming.entry(d.block).or_default().push(w.src.clone());
                }
                wired.insert(w.src.block);
                wired.insert(d.block);
            }
        }
        // unwired power elements in a wired rung: warn — they
        // contribute nothing to the flow graph. Dataflow blocks
        // (incl. data taps) still emit — params/assocs may drive them.
        for b in band {
            if b.kind == BlockKind::Variable
                && !is_data_var(b)
                && !wired.contains(&b.id)
            {
                self.warn(
                    codes::ORPHAN,
                    format!("ladder element {} not in wire graph", b.id),
                    b.id,
                );
            }
        }
        // unwired dataflow blocks emit unconditionally — their inputs
        // may come from params/assocs rather than wires
        for b in band {
            if (b.kind != BlockKind::Variable || is_data_var(b))
                && !wired.contains(&b.id)
            {
                self.emit_block(net, b, &blocks_map, stmts);
            }
        }
        // topo-order every wired block: producers before consumers.
        // deps count distinct src blocks per dst; Kahn's algorithm.
        let mut deps: HashMap<NodeId, usize> = HashMap::new();
        let mut consumers: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut edge_seen: HashSet<(NodeId, NodeId)> = HashSet::new();
        for w in &net.wires {
            for d in &w.dsts {
                if wired.contains(&w.src.block)
                    && wired.contains(&d.block)
                    && w.src.block != d.block
                    && edge_seen.insert((w.src.block, d.block))
                {
                    *deps.entry(d.block).or_insert(0) += 1;
                    consumers.entry(w.src.block).or_default().push(d.block);
                }
            }
        }
        let mut queue: VecDeque<NodeId> = deps
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(&id, _)| id)
            .chain(
                wired
                    .iter()
                    .filter(|id| !deps.contains_key(*id))
                    .copied(),
            )
            .collect();
        let mut seen: HashSet<NodeId> = HashSet::new();
        while let Some(id) = queue.pop_front() {
            if !seen.insert(id) {
                continue;
            }
            self.emit_ladder_node(net, id, &ids, &blocks_map, &incoming, stmts);
            if let Some(cs) = consumers.get(&id) {
                for &c in cs {
                    if let Some(d) = deps.get_mut(&c) {
                        *d -= 1;
                        if *d == 0 {
                            queue.push_back(c);
                        }
                    }
                }
            }
        }
        // cycle recovery: data taps are registers — a store's read-port
        // edge that closes a cycle sees the pre-store value (same rule
        // as FBD var stores / FB feedback). Drop those edges and re-run
        // Kahn over the stuck subgraph; power-flow cycles still error.
        let stuck: BTreeSet<NodeId> = wired
            .iter()
            .filter(|id| !seen.contains(*id))
            .copied()
            .collect();
        if !stuck.is_empty() {
            self.lower_ladder_feedback(
                net, &stuck, &ids, &blocks_map, &incoming, stmts, &mut seen,
            );
        }
        // cycle check: wired blocks still holding deps never evaluated
        for (&id, &d) in &deps {
            if d > 0 && !seen.contains(&id) {
                self.err(
                    codes::UNSUPPORTED,
                    format!("ladder wire cycle at block {id}"),
                    id,
                );
            }
        }
    }

    /// Emit one wired ladder node: dataflow elements through
    /// `emit_block`, power elements through `lower_ld_var`, then any
    /// probe stores sourced at this block.
    pub(super) fn emit_ladder_node(
        &mut self,
        net: &Network,
        id: NodeId,
        ids: &HashMap<NodeId, &Block>,
        blocks_map: &BTreeMap<NodeId, &Block>,
        incoming: &HashMap<NodeId, Vec<PortRef>>,
        stmts: &mut Vec<Stmt>,
    ) {
        let b = ids[&id];
        if b.kind != BlockKind::Variable || is_data_var(b) {
            // dataflow element — emit_block resolves its pins
            // through dst_input/src_expr (gated contacts and
            // data taps both included)
            self.emit_block(net, b, blocks_map, stmts);
        } else {
            self.lower_ld_var(net, b, incoming, ids, stmts);
        }
        // probe stores for wires sourced at this block
        for w in &net.wires {
            if w.src.block == id && self.probes.contains_key(&w.id) {
                if let Some(e) = self.src_expr(&w.src, net, blocks_map) {
                    let pname = self.probes[&w.id].clone();
                    self.fix_probe_ty(&pname, &e.ty.clone());
                    stmts.push(Stmt {
                        kind: StmtKind::Assign {
                            lhs: Ref::Var(pname),
                            rhs: e,
                        },
                        prov: self.prov(w.id, 0),
                    });
                }
            }
        }
    }

    /// Second Kahn pass over the stuck subgraph: drop store read-edges
    /// that close a cycle (data-var stores are registers — cycle
    /// members read the pre-store value), emit what releases, and
    /// report whatever remains as a genuine wire cycle.
    pub(super) fn lower_ladder_feedback(
        &mut self,
        net: &Network,
        stuck: &BTreeSet<NodeId>,
        ids: &HashMap<NodeId, &Block>,
        blocks_map: &BTreeMap<NodeId, &Block>,
        incoming: &HashMap<NodeId, Vec<PortRef>>,
        stmts: &mut Vec<Stmt>,
        seen: &mut HashSet<NodeId>,
    ) {
        // full adjacency within the stuck subgraph — reachability
        // decides whether a store's outgoing edge actually closes a
        // cycle (downstream edges must keep ordering: they read the
        // NEW value)
        let mut adj: HashMap<NodeId, BTreeSet<NodeId>> = HashMap::new();
        for w in &net.wires {
            for d in &w.dsts {
                if stuck.contains(&w.src.block) && stuck.contains(&d.block) {
                    adj.entry(w.src.block).or_default().insert(d.block);
                }
            }
        }
        let reaches = |from: NodeId, to: NodeId| -> bool {
            let mut seen: HashSet<NodeId> = HashSet::new();
            let mut stack = vec![from];
            while let Some(n) = stack.pop() {
                if n == to {
                    return true;
                }
                if seen.insert(n) {
                    if let Some(ns) = adj.get(&n) {
                        stack.extend(ns.iter().copied());
                    }
                }
            }
            false
        };
        let is_store = |id: NodeId| -> bool {
            let b = ids[&id];
            is_data_var(b) && !matches!(b.role, Some(VarRole::In))
        };
        let mut deps: HashMap<NodeId, usize> = HashMap::new();
        let mut consumers: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
        let mut edge_seen: HashSet<(NodeId, NodeId)> = HashSet::new();
        for w in &net.wires {
            for d in &w.dsts {
                if !stuck.contains(&w.src.block)
                    || !stuck.contains(&d.block)
                    || w.src.block == d.block
                    || !edge_seen.insert((w.src.block, d.block))
                {
                    continue;
                }
                if is_store(w.src.block) && reaches(d.block, w.src.block) {
                    continue; // register read-port — sees pre-store value
                }
                *deps.entry(d.block).or_insert(0) += 1;
                consumers.entry(w.src.block).or_default().push(d.block);
            }
        }
        // seed with every stuck node holding no stuck-internal deps —
        // nodes absent from `deps` entirely (all their cycle-closing
        // inputs were store read-edges) must still emit
        let mut queue: VecDeque<NodeId> = stuck
            .iter()
            .filter(|id| deps.get(*id).copied().unwrap_or(0) == 0)
            .copied()
            .collect();
        while let Some(id) = queue.pop_front() {
            if !seen.insert(id) {
                continue;
            }
            self.emit_ladder_node(net, id, ids, blocks_map, incoming, stmts);
            if let Some(cs) = consumers.get(&id) {
                for &c in cs {
                    if let Some(d) = deps.get_mut(&c) {
                        *d -= 1;
                        if *d == 0 {
                            queue.push_back(c);
                        }
                    }
                }
            }
        }
        for (&id, &d) in &deps {
            if d > 0 && !seen.contains(&id) {
                self.err(
                    codes::UNSUPPORTED,
                    format!("ladder wire cycle at block {id}"),
                    id,
                );
            }
        }
    }

    /// One variable block in a wired rung: merged incoming flow
    /// (OR, FALSE when none) gates the variable value. Contacts
    /// publish `var [NOT] AND flow` to ld_gated; coils/in-outs latch
    /// the merged flow (in-outs also publish it downstream).
    pub(super) fn lower_ld_var(
        &mut self,
        net: &Network,
        b: &Block,
        incoming: &HashMap<NodeId, Vec<PortRef>>,
        ids: &HashMap<NodeId, &Block>,
        stmts: &mut Vec<Stmt>,
    ) {
        let blocks_map: BTreeMap<NodeId, &Block> =
            ids.iter().map(|(&k, &v)| (k, v)).collect();
        // merged flow-in: OR of source signals, FALSE when unwired
        let mut fin = Expr {
            kind: ExprKind::Lit(Lit::Bool(false)),
            ty: Type::Bool,
            prov: self.prov(b.id, 0),
        };
        let mut any = false;
        if let Some(srcs) = incoming.get(&b.id) {
            for p in srcs {
                if let Some(e) = self.src_expr(p, net, &blocks_map) {
                    fin = Expr {
                        kind: ExprKind::Bin(BinOp::Or, Box::new(fin), Box::new(e)),
                        ty: Type::Bool,
                        prov: self.prov(b.id, 0),
                    };
                    any = true;
                }
            }
        }
        match b.role {
            Some(VarRole::In) => {
                // contact: rail-powered (no inputs) → flow TRUE
                if !any {
                    fin = Expr {
                        kind: ExprKind::Lit(Lit::Bool(true)),
                        ty: Type::Bool,
                        prov: self.prov(b.id, 0),
                    };
                }
                let Some(r) = self.var_ref(b) else { return };
                let mut val = Expr {
                    kind: ExprKind::Ref(r),
                    ty: Type::Bool,
                    prov: self.prov(b.id, 0),
                };
                if !b.negated.is_empty() {
                    val = Expr {
                        kind: ExprKind::Un(UnOp::Not, Box::new(val)),
                        ty: Type::Bool,
                        prov: self.prov(b.id, 0),
                    };
                }
                let gated = Expr {
                    kind: ExprKind::Bin(BinOp::And, Box::new(val), Box::new(fin)),
                    ty: Type::Bool,
                    prov: self.prov(b.id, 0),
                };
                self.ld_gated.insert(b.id, gated);
            }
            Some(VarRole::Out) | Some(VarRole::InOut) => {
                if !any {
                    self.warn(
                        codes::ORPHAN,
                        format!("ladder coil {} has no incoming flow", b.id),
                        b.id,
                    );
                }
                if !b.negated.is_empty() {
                    fin = Expr {
                        kind: ExprKind::Un(UnOp::Not, Box::new(fin)),
                        ty: Type::Bool,
                        prov: self.prov(b.id, 0),
                    };
                }
                if let Some(lhs) = self.var_ref(b) {
                    stmts.push(Stmt {
                        kind: StmtKind::Assign { lhs, rhs: fin.clone() },
                        prov: self.prov(b.id, 0),
                    });
                }
                // downstream readers see the written value
                self.ld_gated.insert(b.id, fin);
            }
            _ => {}
        }
    }


}
