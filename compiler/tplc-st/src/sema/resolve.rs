//! resolve — split from sema.rs.
use super::*;

impl<'e> Checker<'e> {
    pub(crate) fn err(&mut self, code: &'static str, msg: impl Into<String>, p: &Prov) {
        self.diags.push(Diag::error(code, msg, p.clone()));
    }

    /// closest declared name within edit-distance 2 (case-insensitive)
    pub(crate) fn did_you_mean(&self, name: &str) -> Option<String> {
        let cand = |n: &str| -> Option<usize> {
            let d = levenshtein(&name.to_ascii_lowercase(), &n.to_ascii_lowercase());
            (d <= 2).then_some(d)
        };
        self.scope
            .vars
            .keys()
            .chain(self.scope.gvls.keys())
            .filter_map(|n| cand(n).map(|d| (d, n.clone())))
            .min_by_key(|(d, _)| *d)
            .map(|(_, n)| n)
    }

    /// Type of a declared var name (case-insensitive).
    pub(crate) fn var_ty(&self, name: &str) -> Option<Type> {
        self.scope
            .vars
            .get(&name.to_ascii_uppercase())
            .map(|v| v.ty.clone())
    }

    /// Resolve a Ref chain → its type; canonicalizes Var case in place.
    pub(crate) fn ref_ty(&mut self, r: &mut Ref, prov: &Prov) -> Option<Type> {
        match r {
            Ref::Var(name) => {
                if let Some(decl) = self.scope.vars.get(&name.to_ascii_uppercase()) {
                    *name = decl.name.clone(); // canonical case
                    self.used.insert(name.to_ascii_uppercase());
                    Some(decl.ty.clone())
                } else if self.scope.gvls.contains_key(&name.to_ascii_uppercase()) {
                    // bare GVL name — resolves as struct-like pseudo-var;
                    // implicit global access is a lint hit
                    self.diags.push(Diag::warning(
                        wcodes::EXTERNAL,
                        format!("implicit global access `{name}` — declare VAR_EXTERNAL"),
                        prov.clone(),
                    ));
                    Some(Type::Struct {
                        name: name.clone(),
                        fields: self.scope.gvls[&name.to_ascii_uppercase()]
                            .values()
                            .map(|v| (v.name.clone(), v.ty.clone()))
                            .collect(),
                    })
                } else if self.env.enum_ordinal(name).is_some() {
                    // enum element `GREEN` — expr() rewrites to ordinal
                    Some(Type::Int {
                        signed: true,
                        bits: 32,
                    })
                } else {
                    let mut d = Diag::error(
                        codes::UNKNOWN_NAME,
                        format!("undeclared name {name}"),
                        prov.clone(),
                    );
                    if let Some(sug) = self.did_you_mean(name) {
                        d = d.with_help(format!("did you mean `{sug}`?"));
                    }
                    self.diags.push(d);
                    None
                }
            }
            Ref::Field(base, name) => {
                let bt = self.ref_ty(base, prov)?;
                match &bt {
                    Type::Fb { name: fbname } => {
                        if let Some(pou) = self.env.pous.get(&fbname.to_ascii_uppercase()) {
                            if let Some(v) =
                                pou.iface.iter().find(|v| v.name.eq_ignore_ascii_case(name))
                            {
                                *name = v.name.clone();
                                return Some(v.ty.clone());
                            }
                        }
                        // builtin FB member
                        if let Some(fbl) = builtin_fb_layout(fbname) {
                            if let Some((_, t)) = fbl.members.get(name.as_str()) {
                                return Some(t.clone());
                            }
                        }
                        self.err(
                            codes::UNKNOWN_NAME,
                            format!("{fbname} has no member {name}"),
                            prov,
                        );
                        None
                    }
                    Type::Struct { fields, .. } => fields
                        .iter()
                        .find(|(n, _)| n.eq_ignore_ascii_case(name))
                        .map(|(n, t)| {
                            *name = n.clone();
                            t.clone()
                        })
                        .or_else(|| {
                            self.err(codes::UNKNOWN_NAME, format!("no field {name}"), prov);
                            None
                        }),
                    _ => {
                        self.err(
                            codes::UNKNOWN_NAME,
                            format!("field access `.{name}` on non-aggregate"),
                            prov,
                        );
                        None
                    }
                }
            }
            Ref::Deref(inner) => match self.ref_ty(inner, prov)? {
                Type::Ref { to } => Some(*to),
                other => {
                    self.err(
                        codes::TYPE,
                        format!("deref of non-reference {other:?}"),
                        prov,
                    );
                    None
                }
            },
            Ref::Index(base, idx) => {
                let bt = self.ref_ty(base, prov)?;
                self.expr(idx);
                if let Type::Array { elem, lo, hi } = &bt {
                    let (lo, hi) = (*lo, *hi);
                    let _ = (lo, hi);
                    Some(*elem.clone())
                } else {
                    self.err(codes::UNKNOWN_NAME, "index on non-array", prov);
                    None
                }
            }
        }
    }

}
