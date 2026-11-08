//! ruSTy AST → `tplc-mir` lowering (P1-02).
//!
//! Boundary per DEC-2026-01: consumes ruSTy's RAW AST (no pre_process —
//! its `__PI_*` desugar and builtin injection stay out). Their AST is
//! an implementation detail; `tplc-mir` is the contract.
//!
//! Output: `FrontendOutput` — Module + io_bindings (AT vars → image
//! binding descriptors) + diagnostics. Aggregate initializers are
//! lowered best-effort; unsupported constructs are diagnostics, never
//! panics (unlike the spike).

use plc_ast::ast::{
    AstNode, AstStatement, DataType, DataTypeDeclaration, HardwareAccessType, Operator, PouType,
    ReferenceAccess, ReferenceExpr, VariableBlockType,
};
use plc_ast::control_statements::AstControlStatement;
use plc_ast::literals::AstLiteral;
use plc_ast::provider::IdProvider;
use plc_source::source_location::SourceLocationFactory;
use plc_source::SourceCode;
use std::collections::BTreeMap;
use tplc_mir as mir;

use crate::diag::{codes, Diag, Severity};
use crate::{FrontendOutput, IoBind};

struct Ctx {
    file: u32,
    user_types: BTreeMap<String, mir::Type>,
    enum_ordinals: BTreeMap<String, BTreeMap<String, u32>>,
    io_bindings: Vec<IoBind>,
    diags: Vec<Diag>,
    scope: String, // current POU name for qualified paths
}

impl Ctx {
    fn prov(&self, n: &AstNode) -> mir::Prov {
        match n.location.to_range() {
            Some(r) => mir::Prov::St {
                file: self.file,
                offset: r.start as u32,
                len: (r.end - r.start) as u32,
            },
            None => mir::Prov::None,
        }
    }
    fn err(&mut self, code: &'static str, msg: impl Into<String>, n: &AstNode) {
        self.diags.push(Diag::error(code, msg, self.prov(n)));
    }
    fn err_at(&mut self, code: &'static str, msg: impl Into<String>, p: &mir::Prov) {
        self.diags.push(Diag::error(code, msg, p.clone()));
    }
    fn sloc(&self, l: &plc_source::source_location::SourceLocation) -> mir::Prov {
        match l.to_range() {
            Some(r) => mir::Prov::St {
                file: self.file,
                offset: r.start as u32,
                len: (r.end - r.start) as u32,
            },
            None => mir::Prov::None,
        }
    }

    // ---- types ----
    fn ty_of(&mut self, dtd: &DataTypeDeclaration, at: &mir::Prov) -> mir::Type {
        match dtd {
            DataTypeDeclaration::Reference {
                referenced_type, ..
            }
            | DataTypeDeclaration::Aggregate {
                referenced_type, ..
            } => self.named_ty(referenced_type),
            DataTypeDeclaration::Definition { data_type, .. } => self.inline_ty(data_type, at),
        }
    }

    fn named_ty(&mut self, name: &str) -> mir::Type {
        match name.to_ascii_uppercase().as_str() {
            "BOOL" => mir::Type::Bool,
            "SINT" => mir::Type::Int {
                signed: true,
                bits: 8,
            },
            "INT" => mir::Type::Int {
                signed: true,
                bits: 16,
            },
            "DINT" => mir::Type::Int {
                signed: true,
                bits: 32,
            },
            "LINT" => mir::Type::Int {
                signed: true,
                bits: 64,
            },
            "USINT" | "BYTE" => mir::Type::Int {
                signed: false,
                bits: 8,
            },
            "UINT" | "WORD" => mir::Type::Int {
                signed: false,
                bits: 16,
            },
            "UDINT" | "DWORD" => mir::Type::Int {
                signed: false,
                bits: 32,
            },
            "ULINT" | "LWORD" => mir::Type::Int {
                signed: false,
                bits: 64,
            },
            "REAL" => mir::Type::Real { bits: 32 },
            "LREAL" => mir::Type::Real { bits: 64 },
            "TIME" | "LTIME" | "DATE" | "TOD" | "DT" => mir::Type::Time,
            "STRING" => mir::Type::Str { cap: 80 },
            "WSTRING" => mir::Type::WStr { cap: 160 },
            _ => {
                if let Some(t) = self.user_types.get(name) {
                    return t.clone();
                }
                // FB-type reference — original case preserved (IEC is
                // case-insensitive; canonicalize in sema, P1-03)
                mir::Type::Fb {
                    name: name.to_string(),
                }
            }
        }
    }

    fn inline_ty(&mut self, dt: &DataType, at: &mir::Prov) -> mir::Type {
        match dt {
            DataType::ArrayType {
                bounds,
                referenced_type,
                ..
            } => {
                let (lo, hi) = match &bounds.stmt {
                    AstStatement::RangeStatement(r) => (lit_int(&r.start), lit_int(&r.end)),
                    _ => {
                        self.err_at(codes::UNSUPPORTED, "non-range array bound", at);
                        (0, 0)
                    }
                };
                mir::Type::Array {
                    elem: Box::new(self.ty_of(referenced_type, at)),
                    lo,
                    hi,
                }
            }
            DataType::StringType { is_wide, size, .. } => {
                let cap = size.as_ref().map(|s| lit_int(s) as u16)
                    .unwrap_or(if *is_wide { 160 } else { 80 });
                if *is_wide {
                    mir::Type::WStr { cap }
                } else {
                    mir::Type::Str { cap }
                }
            }
            DataType::SubRangeType {
                referenced_type, ..
            } => {
                // bounds enforced by sema (P1-03); storage = base type
                self.named_ty(referenced_type)
            }
            // ruSTy encodes `x AT %QX0.0 : T` as an auto-deref alias
            // pointer. We want shadow field + io_binding — unwrap.
            DataType::PointerType {
                referenced_type,
                auto_deref: Some(plc_ast::ast::AutoDerefType::Alias),
                ..
            } => self.ty_of(referenced_type, at),
            _ => {
                self.err_at(
                    codes::UNSUPPORTED,
                    format!("unsupported inline type {dt:?}"),
                    at,
                );
                mir::Type::Bool
            }
        }
    }

    // ---- references ----
    fn ref_of(&mut self, n: &AstNode) -> mir::Ref {
        match &n.stmt {
            AstStatement::Identifier(name) => mir::Ref::Var(name.clone()),
            AstStatement::ReferenceExpr(ReferenceExpr { access, base }) => match access {
                ReferenceAccess::Member(inner) | ReferenceAccess::Global(inner) => {
                    let name = inner.get_flat_reference_name().unwrap_or("?").to_string();
                    match base {
                        Some(b) => mir::Ref::Field(Box::new(self.ref_of(b)), name),
                        None => mir::Ref::Var(name),
                    }
                }
                ReferenceAccess::Index(idx) => {
                    let e = self.expr(idx);
                    match base {
                        Some(b) => mir::Ref::Index(Box::new(self.ref_of(b)), Box::new(e)),
                        None => {
                            self.err(codes::UNSUPPORTED, "index without base", n);
                            mir::Ref::Var("?".into())
                        }
                    }
                }
                _ => {
                    self.err(codes::UNSUPPORTED, "unsupported ref access", n);
                    mir::Ref::Var("?".into())
                }
            },
            _ => {
                self.err(codes::UNSUPPORTED, "non-reference lvalue", n);
                mir::Ref::Var("?".into())
            }
        }
    }

    // ---- expressions ----
    fn expr(&mut self, n: &AstNode) -> mir::Expr {
        let kind = match &n.stmt {
            AstStatement::Literal(l) => match l {
                AstLiteral::Integer(v) => mir::ExprKind::Lit(mir::Lit::Int(*v as i64)),
                AstLiteral::Real(s) => mir::ExprKind::Lit(mir::Lit::Real(s.parse().unwrap_or(0.0))),
                AstLiteral::Bool(b) => mir::ExprKind::Lit(mir::Lit::Bool(*b)),
                AstLiteral::Time(t) => {
                    let ns = t.nanos as i64;
                    mir::ExprKind::Lit(mir::Lit::TimeNs(if t.negative { -ns } else { ns }))
                }
                AstLiteral::String(s) if s.is_wide => {
                    mir::ExprKind::Lit(mir::Lit::WStr(s.value.clone()))
                }
                AstLiteral::String(s) => mir::ExprKind::Lit(mir::Lit::Str(s.value.clone())),
                _ => {
                    self.err(codes::UNSUPPORTED, "literal kind", n);
                    mir::ExprKind::Lit(mir::Lit::Int(0))
                }
            },
            AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Cast(inner),
                ..
            }) => {
                // typed literal `INT#1` — cast is a type annotation;
                // explicit Conv nodes are inserted by sema (P1-03)
                return self.expr(inner);
            }
            AstStatement::Identifier(_) | AstStatement::ReferenceExpr(_) => {
                mir::ExprKind::Ref(self.ref_of(n))
            }
            AstStatement::BinaryExpression(b) => {
                let l = self.expr(&b.left);
                let r = self.expr(&b.right);
                mir::ExprKind::Bin(binop(b.operator), Box::new(l), Box::new(r))
            }
            AstStatement::UnaryExpression(u) => {
                let v = self.expr(&u.value);
                let op = match u.operator {
                    Operator::Minus => mir::UnOp::Neg,
                    _ => mir::UnOp::Not,
                };
                mir::ExprKind::Un(op, Box::new(v))
            }
            AstStatement::ParenExpression(p) => return self.expr(p),
            AstStatement::CallStatement(c) => {
                let fun = c
                    .operator
                    .get_flat_reference_name()
                    .unwrap_or("?")
                    .to_string();
                mir::ExprKind::Call {
                    fun,
                    args: self.call_args(c),
                }
            }
            AstStatement::ExpressionList(v) if v.len() == 1 => return self.expr(&v[0]),
            AstStatement::MultipliedStatement(m) => return self.expr(&m.element),
            AstStatement::DirectAccess(_) | AstStatement::HardwareAccess(_) => {
                self.err(
                    codes::UNSUPPORTED,
                    "direct %I/%Q access in expressions — declare an AT-bound var",
                    n,
                );
                mir::ExprKind::Lit(mir::Lit::Int(0))
            }
            _ => {
                self.err(codes::UNSUPPORTED, "expression kind", n);
                mir::ExprKind::Lit(mir::Lit::Int(0))
            }
        };
        mir::Expr {
            kind,
            ty: mir::Type::Bool,
            prov: self.prov(n),
        }
    }

    fn call_args(&mut self, c: &plc_ast::ast::CallStatement) -> Vec<(String, mir::Expr)> {
        let mut out = Vec::new();
        if let Some(params) = &c.parameters {
            let list = match &params.stmt {
                AstStatement::ExpressionList(v) => v.iter().collect::<Vec<_>>(),
                _ => vec![params.as_ref()],
            };
            for p in list {
                match &p.stmt {
                    AstStatement::Assignment(a) | AstStatement::OutputAssignment(a) => {
                        let name = a.left.get_flat_reference_name().unwrap_or("").to_string();
                        out.push((name, self.expr(&a.right)));
                    }
                    _ => out.push((String::new(), self.expr(p))),
                }
            }
        }
        out
    }

    // ---- statements ----
    fn stmt(&mut self, n: &AstNode) -> Option<mir::Stmt> {
        let kind = match &n.stmt {
            AstStatement::EmptyStatement(_) => return None,
            AstStatement::Assignment(a) => mir::StmtKind::Assign {
                lhs: self.ref_of(&a.left),
                rhs: self.expr(&a.right),
            },
            AstStatement::CallStatement(c) => mir::StmtKind::Call {
                inst: self.ref_of(&c.operator),
                args: self.call_args(c),
            },
            AstStatement::ControlStatement(c) => self.control(c),
            AstStatement::ReturnStatement(_) => mir::StmtKind::Return(None),
            AstStatement::ExitStatement(_) => mir::StmtKind::Exit,
            AstStatement::ContinueStatement(_) => mir::StmtKind::Continue,
            _ => {
                self.err(codes::UNSUPPORTED, "statement kind", n);
                return None;
            }
        };
        Some(mir::Stmt {
            kind,
            prov: self.prov(n),
        })
    }

    fn stmts(&mut self, ns: &[AstNode]) -> Vec<mir::Stmt> {
        ns.iter().filter_map(|n| self.stmt(n)).collect()
    }

    fn control(&mut self, c: &AstControlStatement) -> mir::StmtKind {
        match c {
            AstControlStatement::If(i) => mir::StmtKind::If {
                branches: i
                    .blocks
                    .iter()
                    .map(|b| (self.expr(&b.condition), self.stmts(&b.body)))
                    .collect(),
                else_: self.stmts(&i.else_block),
            },
            AstControlStatement::ForLoop(f) => mir::StmtKind::For {
                var: f
                    .counter
                    .get_flat_reference_name()
                    .unwrap_or("i")
                    .to_string(),
                start: self.expr(&f.start),
                end: self.expr(&f.end),
                step: f.by_step.as_deref().map(|s| self.expr(s)),
                body: self.stmts(&f.body),
            },
            AstControlStatement::WhileLoop(w) => mir::StmtKind::While {
                cond: self.expr(&w.condition),
                body: self.stmts(&w.body),
            },
            AstControlStatement::RepeatLoop(w) => mir::StmtKind::Repeat {
                cond: self.expr(&w.condition),
                body: self.stmts(&w.body),
            },
            AstControlStatement::Case(c) => mir::StmtKind::Case {
                sel: self.expr(&c.selector),
                arms: c
                    .case_blocks
                    .iter()
                    .map(|b| (self.case_sels(&b.condition), self.stmts(&b.body)))
                    .collect(),
                else_: self.stmts(&c.else_block),
            },
        }
    }

    fn case_sels(&mut self, n: &AstNode) -> Vec<mir::CaseSel> {
        match &n.stmt {
            AstStatement::CaseCondition(c) => self.case_sels(c),
            AstStatement::RangeStatement(r) => {
                vec![mir::CaseSel::Range(self.expr(&r.start), self.expr(&r.end))]
            }
            AstStatement::ExpressionList(v) => v.iter().flat_map(|e| self.case_sels(e)).collect(),
            _ => vec![mir::CaseSel::Lit(self.expr(n))],
        }
    }

    // ---- declarations ----
    fn var_of(
        &mut self,
        v: &plc_ast::ast::Variable,
        blk: &plc_ast::ast::VariableBlock,
    ) -> mir::VarDecl {
        let kind = match blk.kind {
            VariableBlockType::Input(_) => mir::VarKind::Input,
            VariableBlockType::Output => mir::VarKind::Output,
            VariableBlockType::InOut => mir::VarKind::InOut,
            VariableBlockType::Global => mir::VarKind::Global,
            VariableBlockType::External => mir::VarKind::External,
            _ if blk.constant => mir::VarKind::Constant,
            _ => mir::VarKind::Local,
        };
        let at = v.address.as_ref().and_then(|a| self.at_of(a));
        let vprov = self.sloc(&v.location);
        let ty = self.ty_of(&v.data_type_declaration, &vprov);
        let init = v
            .initializer
            .as_ref()
            .and_then(|i| self.init_of(i))
            .map(|mut e| {
                // literals lower untyped (Bool default); the declared
                // type is authoritative for the stored constant width
                if matches!(e.kind, mir::ExprKind::Lit(_)) {
                    e.ty = ty.clone();
                }
                e
            });
        if let Some(at) = &at {
            self.io_bindings.push(IoBind {
                name: format!("{}.{}", self.scope, v.name),
                at: at.clone(),
                ty: ty.clone(),
            });
        }
        mir::VarDecl {
            name: v.name.clone(),
            kind,
            ty,
            init,
            retain: blk.retain,
            at,
            probe: false,
            prov: mir::Prov::None,
        }
    }

    fn init_of(&mut self, n: &AstNode) -> Option<mir::Expr> {
        match &n.stmt {
            AstStatement::Literal(_)
            | AstStatement::UnaryExpression(_)
            | AstStatement::Identifier(_)
            | AstStatement::ReferenceExpr(_)
            | AstStatement::MultipliedStatement(_)
            | AstStatement::ExpressionList(_)
            | AstStatement::ParenExpression(_) => Some(self.expr(n)),
            _ => {
                self.err(codes::UNSUPPORTED, "initializer form", n);
                None
            }
        }
    }

    fn at_of(&mut self, n: &AstNode) -> Option<mir::At> {
        if let AstStatement::HardwareAccess(h) = &n.stmt {
            let region = match h.direction {
                HardwareAccessType::Input => mir::AtRegion::I,
                HardwareAccessType::Output => mir::AtRegion::Q,
                _ => mir::AtRegion::M,
            };
            let width = match h.access {
                plc_ast::ast::DirectAccessType::Bit => 'X',
                plc_ast::ast::DirectAccessType::Byte => 'B',
                plc_ast::ast::DirectAccessType::Word => 'W',
                plc_ast::ast::DirectAccessType::DWord => 'D',
                plc_ast::ast::DirectAccessType::LWord => 'L',
                _ => 'X',
            };
            let byte = h.address.first().map(lit_int).unwrap_or(0) as u64;
            let bit = h.address.get(1).map(|b| lit_int(b) as u8);
            return Some(mir::At {
                region,
                byte,
                bit,
                width,
            });
        }
        self.err(codes::BAD_AT, "malformed AT address", n);
        None
    }
}

fn lit_int(n: &AstNode) -> i64 {
    match &n.stmt {
        AstStatement::Literal(AstLiteral::Integer(v)) => *v as i64,
        AstStatement::ParenExpression(p) => lit_int(p),
        AstStatement::CaseCondition(c) => lit_int(c),
        _ => 0,
    }
}

fn binop(o: Operator) -> mir::BinOp {
    use mir::BinOp::*;
    match o {
        Operator::Plus => Add,
        Operator::Minus => Sub,
        Operator::Multiplication => Mul,
        Operator::Division => Div,
        Operator::Modulo => Mod,
        Operator::Equal => Eq,
        Operator::NotEqual => Ne,
        Operator::Less => Lt,
        Operator::LessOrEqual => Le,
        Operator::Greater => Gt,
        Operator::GreaterOrEqual => Ge,
        Operator::And => And,
        Operator::Or => Or,
        Operator::Xor => Xor,
        Operator::AndThen => AndThen,
        Operator::OrElse => OrElse,
        _ => Add,
    }
}

/// Compile ST sources through the ruSTy parser into `tplc-mir`.
/// `sources`: (filename, source) pairs — file id = index.
pub fn compile(sources: &[(&str, &str)]) -> FrontendOutput {
    let mut diags = Vec::new();
    let mut module = mir::Module {
        pous: vec![],
        gvls: vec![],
        tasks: vec![],
    };
    let mut io_bindings = Vec::new();

    for (file_id, (fname, src)) in sources.iter().enumerate() {
        let source = SourceCode::from(*src);
        let factory = SourceLocationFactory::for_source(&source);
        // the upstream parser panics on some malformed inputs (e.g. a
        // truncated `%QX0.` location); a panic must surface as a
        // diagnostic, never crash the frontend
        let parsed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            plc_compiler::parser::parse(
                plc_compiler::lexer::lex_with_ids(&source.source, IdProvider::default(), factory),
                plc_ast::ast::LinkageType::Internal,
                Box::leak(fname.to_string().into_boxed_str()),
            )
        }));
        let (unit, pdiags) = match parsed {
            Ok(x) => x,
            Err(_) => {
                diags.push(Diag {
                    code: codes::PARSE,
                    severity: Severity::Error,
                    message: "parser rejected the input (internal panic)".to_string(),
                    help: None,
                    notes: Vec::new(),
                    span: mir::Prov::St {
                        file: file_id as u32,
                        offset: 0,
                        len: 0,
                    },
                });
                continue;
            }
        };
        for d in &pdiags {
            diags.push(Diag {
                code: codes::PARSE,
                severity: Severity::Error,
                message: d.get_message().to_string(),
                help: None,
                notes: Vec::new(),
                span: mir::Prov::St {
                    file: file_id as u32,
                    offset: 0,
                    len: 0,
                },
            });
        }

        let mut ctx = Ctx {
            file: file_id as u32,
            user_types: BTreeMap::new(),
            enum_ordinals: BTreeMap::new(),
            io_bindings: Vec::new(),
            diags: Vec::new(),
            scope: String::new(),
        };

        // user types
        for ut in &unit.user_types {
            match &ut.data_type {
                DataType::StructType { name, variables } => {
                    let fields = variables
                        .iter()
                        .map(|v| {
                            (
                                v.name.clone(),
                                ctx.ty_of(&v.data_type_declaration, &ctx.sloc(&v.location)),
                            )
                        })
                        .collect();
                    let n = name.clone().unwrap_or_default();
                    ctx.user_types
                        .insert(n.clone(), mir::Type::Struct { name: n, fields });
                }
                DataType::EnumType { name, elements, .. } => {
                    let names = plc_ast::ast::get_enum_element_names(elements);
                    let n = name.clone().unwrap_or_default();
                    let ords: BTreeMap<String, u32> = names
                        .iter()
                        .enumerate()
                        .map(|(i, s)| (s.clone(), i as u32))
                        .collect();
                    ctx.enum_ordinals.insert(n.clone(), ords);
                    ctx.user_types.insert(
                        n.clone(),
                        mir::Type::Enum {
                            name: n,
                            values: names,
                        },
                    );
                }
                _ => ctx.err_at(
                    codes::UNSUPPORTED,
                    "user type form",
                    &ctx.sloc(&ut.location),
                ),
            }
        }

        // global variable blocks → GVLs
        for gvb in &unit.global_vars {
            let gname = "GLOBAL".to_string(); // unnamed blocks share the default GVL
            let vars: Vec<mir::VarDecl> = gvb
                .variables
                .iter()
                .map(|v| {
                    ctx.scope = gname.clone();
                    ctx.var_of(v, gvb)
                })
                .collect();
            if let Some(g) = module.gvls.iter_mut().find(|g| g.name == gname) {
                g.vars.extend(vars);
            } else {
                module.gvls.push(mir::Gvl {
                    name: gname,
                    vars,
                    retain: gvb.retain,
                });
            }
        }

        // POUs
        for pou in &unit.pous {
            let kind = match pou.kind {
                PouType::Program => mir::PouKind::Program,
                PouType::FunctionBlock => mir::PouKind::FunctionBlock,
                PouType::Function => mir::PouKind::Function,
                _ => {
                    diags.push(Diag::error(
                        codes::UNSUPPORTED,
                        format!("POU kind {:?} out of dialect", pou.kind),
                        mir::Prov::None,
                    ));
                    continue;
                }
            };
            ctx.scope = pou.name.clone();
            let mut iface = Vec::new();
            for blk in &pou.variable_blocks {
                for v in &blk.variables {
                    iface.push(ctx.var_of(v, blk));
                }
            }
            if kind == mir::PouKind::Function {
                if let Some(rt) = &pou.return_type {
                    let ty = ctx.ty_of(rt, &ctx.sloc(&pou.name_location));
                    iface.push(mir::VarDecl {
                        name: pou.name.clone(),
                        kind: mir::VarKind::Local,
                        ty,
                        init: None,
                        retain: false,
                        at: None,
                        probe: false,
                        prov: mir::Prov::Generated,
                    });
                }
            }
            let body = unit
                .implementations
                .iter()
                .find(|i| i.name == pou.name)
                .map(|i| mir::Body::Stmts(ctx.stmts(&i.statements)))
                .unwrap_or(mir::Body::Stmts(vec![]));
            let ret_ty = pou
                .return_type
                .as_ref()
                .map(|t| ctx.ty_of(t, &ctx.sloc(&pou.name_location)));
            module.pous.push(mir::Pou {
                kind,
                name: pou.name.clone(),
                iface,
                body,
                ret_ty,
                prov: mir::Prov::None,
            });
        }

        diags.extend(ctx.diags);
        io_bindings.extend(ctx.io_bindings);
    }

    FrontendOutput {
        module,
        io_bindings,
        diags,
    }
}
