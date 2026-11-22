//! FB/FN/var/device declaration lowering — split out of lower.rs.
use super::*;

impl<'a> Cx<'a> {
    /// FB block → instance var decl + sig (FBD006/012/013 checks).
    pub(super) fn fb_decl(&mut self, b: &Block) {
        let tref = b.type_ref.to_ascii_uppercase();
        if tref == self.pou.name.to_ascii_uppercase() {
            self.err(
                codes::RECURSION,
                format!("{} instantiates itself", self.pou.name),
                b.id,
            );
            return;
        }
        let sig = builtin_sig(&tref)
            .or_else(|| self.pous.get(&tref).map(|p| pou_sig(p, self.fb_names)))
            .or_else(|| {
                self.ext_pous
                    .get(&tref)
                    .filter(|p| p.kind == ir::PouKind::FunctionBlock)
                    .map(|p| mir_sig(p))
            });
        let Some(sig) = sig else {
            self.err(
                codes::UNKNOWN,
                format!("unknown FB type {}", b.type_ref),
                b.id,
            );
            return;
        };
        let inst = match &b.instance {
            Some(n) => n.clone(),
            None => format!("__fb_{}", b.id),
        };
        if self.var_by_name.contains_key(&inst.to_ascii_uppercase()) {
            self.err(
                codes::INST_REUSE,
                format!("instance name {inst} reused"),
                b.id,
            );
            return;
        }
        self.sigs.insert(b.id, sig);
        self.inst_by_block.insert(b.id, inst.clone());
        self.declare(VarDecl {
            name: inst,
            kind: VarKind::Local,
            // original case — layout resolves Type::Fb verbatim
            ty: Type::Fb {
                name: b.type_ref.clone(),
            },
            init: None,
            retain: false,
            at: None,
            probe: false,
            prov: self.prov(b.id, 0),
        });
    }

    /// FUNCTION block → sig only (no instance state).
    pub(super) fn fn_sig(&mut self, b: &Block) {
        let tref = b.type_ref.to_ascii_uppercase();
        if let Some(p) = self
            .pous
            .get(&tref)
            .filter(|p| p.kind == PouKind::Function)
        {
            self.sigs.insert(b.id, pou_sig(p, self.fb_names));
            return;
        }
        if let Some(p) = self
            .ext_pous
            .get(&tref)
            .filter(|p| p.kind == ir::PouKind::Function)
        {
            self.sigs.insert(b.id, mir_sig(p));
            return;
        }
        self.err(
            codes::UNKNOWN,
            format!("unknown FUNCTION {}", b.type_ref),
            b.id,
        );
    }

    /// Variable block → located shadow or iface binding (FBD008).
    pub(super) fn var_decl(&mut self, b: &Block) {
        let at_p = b.at.as_deref().and_then(|s| {
            let a = parse_at(s);
            if a.is_none() {
                self.err(codes::BAD_AT, format!("malformed AT {s}"), b.id);
            }
            a
        });
        match (&b.name, at_p) {
            (Some(n), atp) => {
                let un = n.to_ascii_uppercase();
                if let Some(&idx) = self.var_by_name.get(&un) {
                    if let Some(a) = &atp {
                        let v = &mut self.vars[idx];
                        match &v.at {
                            Some(prev) if prev != a => self.err(
                                codes::BAD_AT,
                                format!("{n} bound to conflicting AT"),
                                b.id,
                            ),
                            None => {
                                v.at = Some(a.clone());
                                self.io_bindings.push(IoBind {
                                    name: format!("{}.{}", self.pou.name, v.name),
                                    at: a.clone(),
                                    ty: v.ty.clone(),
                                });
                            }
                            _ => {}
                        }
                    }
                    self.var_by_block.insert(b.id, idx);
                    return;
                }
                if self.gvl_member(n).is_some() && atp.is_none() {
                    return; // global — resolved in the expr path
                }
                // undeclared name is only a declaration when `at` present
                let Some(a) = atp else {
                    // PLCopen constant taps (Beremiz <inVariable>
                    // <expression>1</expression>) carry a literal,
                    // not a name — src_expr resolves them, no storage
                    if parse_lit(n).is_ok() {
                        return;
                    }
                    self.err(codes::UNDECLARED, format!("undeclared name {n}"), b.id);
                    return;
                };
                let idx = self.declare(VarDecl {
                    name: n.clone(),
                    kind: VarKind::Local,
                    ty: ty_for_width(a.width),
                    init: None,
                    retain: false,
                    at: Some(a.clone()),
                    probe: false,
                    prov: self.prov(b.id, 0),
                });
                self.io_bindings.push(IoBind {
                    name: format!("{}.{}", self.pou.name, self.vars[idx].name),
                    at: a,
                    ty: self.vars[idx].ty.clone(),
                });
                self.var_by_block.insert(b.id, idx);
            }
            (None, Some(a)) => {
                let key = format!("{:?}{}x{}", a.region, a.byte, a.bit.unwrap_or(255));
                let idx = match self.at_vars.get(&key) {
                    Some(&i) => i,
                    None => {
                        let name = format!(
                            "__io_{}_{}{}",
                            region_ch(a.region),
                            a.byte,
                            a.bit.map(|b| format!("_{b}")).unwrap_or_default()
                        );
                        let i = self.declare(VarDecl {
                            name: name.clone(),
                            kind: VarKind::Local,
                            ty: ty_for_width(a.width),
                            init: None,
                            retain: false,
                            at: Some(a.clone()),
                            probe: false,
                            prov: self.prov(b.id, 0),
                        });
                        self.io_bindings.push(IoBind {
                            name: format!("{}.{}", self.pou.name, name),
                            at: a,
                            ty: self.vars[i].ty.clone(),
                        });
                        self.at_vars.insert(key, i);
                        i
                    }
                };
                self.var_by_block.insert(b.id, idx);
            }
            (None, None) => self.err(
                codes::UNDECLARED,
                "variable block with neither name nor at",
                b.id,
            ),
        }
    }

    /// Device block → located channel vars, one per pin (FBD device
    /// blocks). Each channel pin must carry an `at` binding; the pin
    /// aliases a located var shared through `at_vars` so overlapping
    /// addresses across blocks see the same channel. The pin→var
    /// binding lands in `dev_vars`; the signature lands in `sigs` so
    /// wire/param/type machinery treats the block like any other.
    pub(super) fn device_decl(&mut self, b: &Block) {
        self.sigs.insert(b.id, device_sig(b, self.fb_names));
        for v in &b.pins {
            if !matches!(
                v.kind,
                IfaceKind::VarInput | IfaceKind::VarOutput | IfaceKind::VarInOut
            ) {
                continue;
            }
            let pname = v.name.to_ascii_uppercase();
            let Some(at_s) = v.at.as_deref() else {
                self.err(
                    codes::BAD_AT,
                    format!("device pin {} has no AT binding", v.name),
                    b.id,
                );
                continue;
            };
            let Some(a) = parse_at(at_s) else {
                self.err(codes::BAD_AT, format!("malformed AT {at_s}"), b.id);
                continue;
            };
            let ty = match parse_type(&v.ty, self.fb_names) {
                Ok(t) => t,
                Err(e) => {
                    self.err(codes::PARSE, format!("device pin {}: {e}", v.name), b.id);
                    Type::Bool
                }
            };
            // share storage with any var already bound to this address
            let key = format!("{:?}{}x{}", a.region, a.byte, a.bit.unwrap_or(255));
            let idx = match self.at_vars.get(&key) {
                Some(&i) => i,
                None => {
                    let ident: String = v
                        .name
                        .chars()
                        .map(|c| {
                            if c.is_ascii_alphanumeric() || c == '_' {
                                c
                            } else {
                                '_'
                            }
                        })
                        .collect();
                    let name = format!("__dev{}_{}", b.id, ident);
                    let i = self.declare(VarDecl {
                        name: name.clone(),
                        kind: VarKind::Local,
                        ty: ty.clone(),
                        init: None,
                        retain: false,
                        at: Some(a.clone()),
                        probe: false,
                        prov: self.prov(b.id, 0),
                    });
                    self.io_bindings.push(IoBind {
                        name: format!("{}.{}", self.pou.name, name),
                        at: a,
                        ty,
                    });
                    self.at_vars.insert(key, i);
                    i
                }
            };
            self.dev_vars.insert((b.id, pname), idx);
        }
    }

    /// `(GVL, member)` if `n` names a global (`GVL.x` or bare member).
    pub(super) fn gvl_member(&self, n: &str) -> Option<(String, String)> {
        for g in &self.project.global_vars {
            if g.name.eq_ignore_ascii_case(n) {
                return Some(("GVL".into(), g.name.clone()));
            }
        }
        if let Some((g, m)) = n.split_once('.') {
            if g.eq_ignore_ascii_case("GVL")
                && self
                    .project
                    .global_vars
                    .iter()
                    .any(|v| v.name.eq_ignore_ascii_case(m))
            {
                return Some(("GVL".into(), m.to_string()));
            }
        }
        None
    }

    // ---- network lowering ----


}
