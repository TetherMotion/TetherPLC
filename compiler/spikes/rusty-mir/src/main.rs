//! P0.5 spike S5 — one non-trivial POU end-to-end through the B path:
//! ST source → ruSTy parse → tplc-mir lowering → blob layout →
//! interpreter execution over the state blob. Measures: how much of our
//! dialect lowers from ruSTy's AST without their codegen, and what the
//! per-construct lowering effort looks like (the B-cost data point).
//!
//! The corpus is the same one used in rusty-parse (S1–S3) so coverage
//! numbers are comparable.

use plc_ast::ast::{
    AstNode, AstStatement, DataType, DataTypeDeclaration, HardwareAccessType,
    LinkageType, Operator, PouType, ReferenceAccess, ReferenceExpr, VariableBlockType,
};
use plc_ast::control_statements::AstControlStatement;
use plc_ast::literals::AstLiteral;
use plc_ast::provider::IdProvider;
use plc_source::source_location::SourceLocationFactory;
use plc_source::SourceCode;
use std::collections::BTreeMap;
use tplc_mir as mir;

const CORPUS: &str = r#"
TYPE E_Color : (RED, GREEN, BLUE) := GREEN; END_TYPE
TYPE S_Point : STRUCT x : REAL; y : REAL; END_STRUCT END_TYPE

FUNCTION_BLOCK FB_TonWrap
VAR_INPUT  IN : BOOL; PT : TIME; END_VAR
VAR_OUTPUT Q  : BOOL; ET : TIME; END_VAR
VAR        tmr : TON; cnt : INT := 0; END_VAR
    tmr(IN := IN, PT := PT);
    Q := tmr.Q;
    ET := tmr.ET;
    IF IN THEN cnt := cnt + 1; ELSE cnt := 0; END_IF;
END_FUNCTION_BLOCK

FUNCTION F_Add : INT
VAR_INPUT a : INT; b : INT; END_VAR
    F_Add := a + b * 2;
END_FUNCTION

PROGRAM MAIN
VAR
    ton1     : FB_TonWrap;
    running  AT %QX0.0 : BOOL;
    sensor   AT %IW0.0 : INT;
    arr      : ARRAY[0..9] OF DINT;
    ticks    : UDINT;
    count    : INT;
END_VAR
VAR RETAIN
    r        : DINT := 42;
END_VAR
    ton1(IN := sensor > 0, PT := T#500ms);
    running := ton1.Q;
    ticks := ticks + 1;
    arr[0] := F_Add(INT#1, INT#2);
    CASE sensor OF
        0: ticks := 0;
        1..3: ticks := ticks * 2;
    ELSE
        ticks := ticks;
    END_CASE;
    FOR count := 0 TO 9 BY 2 DO arr[count] := arr[count] + 1; END_FOR;
    WHILE ticks > 100 DO ticks := ticks - 1; END_WHILE;
    REPEAT ticks := ticks + 1; UNTIL ticks >= 200 END_REPEAT;
END_PROGRAM
"#;

#[derive(Default)]
struct Lower {
    user_types: BTreeMap<String, mir::Type>,
    stats: BTreeMap<&'static str, usize>,
    gaps: Vec<String>,
}

impl Lower {
    fn note(&mut self, k: &'static str) {
        *self.stats.entry(k).or_default() += 1;
    }
    fn gap(&mut self, s: impl std::fmt::Display) {
        self.gaps.push(s.to_string());
    }

    fn prov(&self, n: &AstNode) -> mir::Prov {
        match n.location.to_range() {
            Some(r) => mir::Prov::St {
                file: 0,
                offset: r.start as u32,
                len: (r.end - r.start) as u32,
            },
            None => mir::Prov::None,
        }
    }

    // ---- types ----
    fn ty_of(&mut self, dtd: &DataTypeDeclaration) -> mir::Type {
        match dtd {
            DataTypeDeclaration::Reference { referenced_type, .. } => {
                self.named_ty(referenced_type)
            }
            DataTypeDeclaration::Definition { data_type, .. } => self.inline_ty(data_type),
            DataTypeDeclaration::Aggregate { referenced_type, .. } => {
                self.named_ty(referenced_type)
            }
        }
    }

    fn named_ty(&mut self, name: &str) -> mir::Type {
        self.note("named-type");
        match name.to_ascii_uppercase().as_str() {
            "BOOL" => mir::Type::Bool,
            "SINT" => mir::Type::Int { signed: true, bits: 8 },
            "INT" => mir::Type::Int { signed: true, bits: 16 },
            "DINT" => mir::Type::Int { signed: true, bits: 32 },
            "LINT" => mir::Type::Int { signed: true, bits: 64 },
            "USINT" | "BYTE" => mir::Type::Int { signed: false, bits: 8 },
            "UINT" | "WORD" => mir::Type::Int { signed: false, bits: 16 },
            "UDINT" | "DWORD" => mir::Type::Int { signed: false, bits: 32 },
            "ULINT" | "LWORD" => mir::Type::Int { signed: false, bits: 64 },
            "REAL" => mir::Type::Real { bits: 32 },
            "LREAL" => mir::Type::Real { bits: 64 },
            "TIME" | "LTIME" => mir::Type::Time,
            _ => {
                if let Some(t) = self.user_types.get(name) {
                    return t.clone();
                }
                // FB type reference — preserve original case so lookups
                // against module POU names match (IEC is case-insensitive
                // at the source level; canonicalize later in sema).
                mir::Type::Fb { name: name.to_string() }
            }
        }
    }

    fn inline_ty(&mut self, dt: &DataType) -> mir::Type {
        match dt {
            DataType::ArrayType { bounds, referenced_type, .. } => {
                self.note("inline-array-type");
                let (lo, hi) = match &bounds.stmt {
                    AstStatement::RangeStatement(r) => {
                        (lit_int(&r.start), lit_int(&r.end))
                    }
                    _ => (0, 0),
                };
                mir::Type::Array {
                    elem: Box::new(self.ty_of(referenced_type)),
                    lo,
                    hi,
                }
            }
            DataType::StringType { size, .. } => {
                self.note("inline-string-type");
                let cap = size
                    .as_ref()
                    .map(|s| lit_int(s) as u16)
                    .unwrap_or(80);
                mir::Type::Str { cap }
            }
            DataType::SubRangeType { referenced_type, .. } => self.named_ty(referenced_type),
            // ruSTy encodes `x AT %QX0.0 : T` as an auto-deref alias
            // pointer. Our model wants a shadow field + io_binding
            // instead — unwrap the alias to its base type.
            DataType::PointerType {
                referenced_type,
                auto_deref: Some(plc_ast::ast::AutoDerefType::Alias),
                ..
            } => {
                self.note("at-alias-unwrap");
                self.ty_of(referenced_type)
            }
            _ => {
                self.gap(format!("inline DataType {dt:?}"));
                mir::Type::Bool
            }
        }
    }

    // ---- references ----
    fn ref_of(&mut self, n: &AstNode) -> mir::Ref {
        match &n.stmt {
            AstStatement::Identifier(name) => {
                self.note("ref-var");
                mir::Ref::Var(name.clone())
            }
            AstStatement::ReferenceExpr(ReferenceExpr { access, base }) => match access {
                ReferenceAccess::Member(inner) | ReferenceAccess::Global(inner) => {
                    self.note("ref-field");
                    let name = inner
                        .get_flat_reference_name()
                        .unwrap_or("?")
                        .to_string();
                    match base {
                        Some(b) => mir::Ref::Field(Box::new(self.ref_of(b)), name),
                        None => mir::Ref::Var(name),
                    }
                }
                ReferenceAccess::Index(idx) => {
                    self.note("ref-index");
                    let e = self.expr(idx);
                    mir::Ref::Index(Box::new(self.ref_of(base.as_ref().unwrap())), Box::new(e))
                }
                other => {
                    self.gap(format!("ref-access {other:?}"));
                    mir::Ref::Var("?".into())
                }
            },
            _ => {
                self.gap(format!("non-ref lhs {:?}", kind(n)));
                mir::Ref::Var("?".into())
            }
        }
    }

    // ---- expressions ----
    fn expr(&mut self, n: &AstNode) -> mir::Expr {
        let kind = match &n.stmt {
            AstStatement::Literal(l) => match l {
                AstLiteral::Integer(v) => {
                    self.note("lit-int");
                    mir::ExprKind::Lit(if *v >= 0 {
                        mir::Lit::Int(*v as i64)
                    } else {
                        mir::Lit::Int(*v as i64)
                    })
                }
                AstLiteral::Real(s) => {
                    self.note("lit-real");
                    mir::ExprKind::Lit(mir::Lit::Real(s.parse().unwrap_or(0.0)))
                }
                AstLiteral::Bool(b) => {
                    self.note("lit-bool");
                    mir::ExprKind::Lit(mir::Lit::Bool(*b))
                }
                AstLiteral::Time(t) => {
                    self.note("lit-time");
                    let ns = t.nanos as i64;
                    mir::ExprKind::Lit(mir::Lit::TimeNs(if t.negative { -ns } else { ns }))
                }
                AstLiteral::String(s) => {
                    self.note("lit-str");
                    mir::ExprKind::Lit(mir::Lit::Str(s.value.clone()))
                }
                _ => {
                    self.gap("literal variant");
                    mir::ExprKind::Lit(mir::Lit::Int(0))
                }
            },
            AstStatement::ReferenceExpr(ReferenceExpr {
                access: ReferenceAccess::Cast(inner), ..
            }) => {
                // typed literal `INT#1` — the cast is a type annotation;
                // our sema will emit explicit Conv nodes, spike unwraps
                self.note("typed-literal");
                return self.expr(inner);
            }
            AstStatement::Identifier(_) | AstStatement::ReferenceExpr(_) => {
                mir::ExprKind::Ref(self.ref_of(n))
            }
            AstStatement::BinaryExpression(b) => {
                self.note("binop");
                let l = self.expr(&b.left);
                let r = self.expr(&b.right);
                mir::ExprKind::Bin(binop(b.operator), Box::new(l), Box::new(r))
            }
            AstStatement::UnaryExpression(u) => {
                self.note("unop");
                let v = self.expr(&u.value);
                match u.operator {
                    Operator::Minus => {
                        mir::ExprKind::Un(mir::UnOp::Neg, Box::new(v))
                    }
                    _ => mir::ExprKind::Un(mir::UnOp::Not, Box::new(v)),
                }
            }
            AstStatement::ParenExpression(p) => return self.expr(p),
            AstStatement::CallStatement(c) => {
                self.note("call-expr");
                let fun = c
                    .operator
                    .get_flat_reference_name()
                    .unwrap_or("?")
                    .to_string();
                let args = self.call_args(c);
                mir::ExprKind::Call { fun, args }
            }
            AstStatement::ExpressionList(v) if v.len() == 1 => return self.expr(&v[0]),
            AstStatement::MultipliedStatement(m) => return self.expr(&m.element),
            AstStatement::DefaultValue(_) => {
                self.gap("default-value");
                mir::ExprKind::Lit(mir::Lit::Int(0))
            }
            other => {
                self.gap(format!("expr kind {}", kind(n)));
                let _ = other;
                mir::ExprKind::Lit(mir::Lit::Int(0))
            }
        };
        mir::Expr { kind, ty: mir::Type::Bool, prov: self.prov(n) }
    }

    fn call_args(&mut self, c: &plc_ast::ast::CallStatement) -> Vec<(String, mir::Expr)> {
        let mut out = Vec::new();
        if let Some(params) = &c.parameters {
            let list = match &params.stmt {
                AstStatement::ExpressionList(v) => v.clone(),
                _ => vec![(**params).clone()],
            };
            for p in list {
                match &p.stmt {
                    AstStatement::Assignment(a) => {
                        let name = a
                            .left
                            .get_flat_reference_name()
                            .unwrap_or("")
                            .to_string();
                        out.push((name, self.expr(&a.right)));
                    }
                    AstStatement::OutputAssignment(a) => {
                        let name = a
                            .left
                            .get_flat_reference_name()
                            .unwrap_or("")
                            .to_string();
                        out.push((name, self.expr(&a.right)));
                    }
                    _ => out.push((String::new(), self.expr(&p))),
                }
            }
        }
        out
    }

    // ---- statements ----
    fn stmt(&mut self, n: &AstNode) -> Option<mir::Stmt> {
        let kind = match &n.stmt {
            AstStatement::EmptyStatement(_) => return None,
            AstStatement::Assignment(a) => {
                self.note("assign");
                mir::StmtKind::Assign {
                    lhs: self.ref_of(&a.left),
                    rhs: self.expr(&a.right),
                }
            }
            AstStatement::CallStatement(c) => {
                self.note("fb-call");
                let inst = self.ref_of(&c.operator);
                mir::StmtKind::Call { inst, args: self.call_args(c) }
            }
            AstStatement::ControlStatement(c) => self.control(c),
            AstStatement::ReturnStatement(_) => {
                self.note("return");
                mir::StmtKind::Return(None)
            }
            AstStatement::ExitStatement(_) => {
                self.note("exit");
                mir::StmtKind::Exit
            }
            AstStatement::ContinueStatement(_) => {
                self.note("continue");
                mir::StmtKind::Continue
            }
            other => {
                self.gap(format!("stmt kind {}", kind(n)));
                let _ = other;
                return None;
            }
        };
        Some(mir::Stmt { kind, prov: self.prov(n) })
    }

    fn stmts(&mut self, ns: &[AstNode]) -> Vec<mir::Stmt> {
        ns.iter().filter_map(|n| self.stmt(n)).collect()
    }

    fn control(&mut self, c: &AstControlStatement) -> mir::StmtKind {
        match c {
            AstControlStatement::If(i) => {
                self.note("if");
                let branches = i
                    .blocks
                    .iter()
                    .map(|b| (self.expr(&b.condition), self.stmts(&b.body)))
                    .collect();
                mir::StmtKind::If {
                    branches,
                    else_: self.stmts(&i.else_block),
                }
            }
            AstControlStatement::ForLoop(f) => {
                self.note("for");
                let var = f
                    .counter
                    .get_flat_reference_name()
                    .unwrap_or("i")
                    .to_string();
                mir::StmtKind::For {
                    var,
                    start: self.expr(&f.start),
                    end: self.expr(&f.end),
                    step: f.by_step.as_deref().map(|s| self.expr(s)),
                    body: self.stmts(&f.body),
                }
            }
            AstControlStatement::WhileLoop(w) => {
                self.note("while");
                mir::StmtKind::While {
                    cond: self.expr(&w.condition),
                    body: self.stmts(&w.body),
                }
            }
            AstControlStatement::RepeatLoop(w) => {
                self.note("repeat");
                mir::StmtKind::Repeat {
                    cond: self.expr(&w.condition),
                    body: self.stmts(&w.body),
                }
            }
            AstControlStatement::Case(c) => {
                self.note("case");
                let arms = c
                    .case_blocks
                    .iter()
                    .map(|b| (self.case_sels(&b.condition), self.stmts(&b.body)))
                    .collect();
                mir::StmtKind::Case {
                    sel: self.expr(&c.selector),
                    arms,
                    else_: self.stmts(&c.else_block),
                }
            }
        }
    }

    fn case_sels(&mut self, n: &AstNode) -> Vec<mir::CaseSel> {
        match &n.stmt {
            AstStatement::CaseCondition(c) => self.case_sels(c),
            AstStatement::RangeStatement(r) => {
                self.note("case-range");
                vec![mir::CaseSel::Range(self.expr(&r.start), self.expr(&r.end))]
            }
            AstStatement::ExpressionList(v) => {
                v.iter().flat_map(|e| self.case_sels(e)).collect()
            }
            _ => {
                self.note("case-lit");
                vec![mir::CaseSel::Lit(self.expr(n))]
            }
        }
    }

    // ---- declarations ----
    fn var_of(&mut self, v: &plc_ast::ast::Variable, blk: &VariableBlockType, retain: bool, constant: bool) -> mir::VarDecl {
        self.note("vardecl");
        let kind = match blk {
            VariableBlockType::Input(_) => mir::VarKind::Input,
            VariableBlockType::Output => mir::VarKind::Output,
            VariableBlockType::InOut => mir::VarKind::InOut,
            VariableBlockType::Global => mir::VarKind::Global,
            _ if constant => mir::VarKind::Constant,
            _ => mir::VarKind::Local,
        };
        let at = v.address.as_ref().and_then(|a| self.at_of(a));
        mir::VarDecl {
            name: v.name.clone(),
            kind,
            ty: self.ty_of(&v.data_type_declaration),
            init: v.initializer.as_ref().and_then(|i| self.init_of(i)),
            retain,
            at,
            probe: false,
            prov: mir::Prov::None,
        }
    }

    fn init_of(&mut self, n: &AstNode) -> Option<mir::Expr> {
        match &n.stmt {
            AstStatement::Literal(_) | AstStatement::UnaryExpression(_)
            | AstStatement::Identifier(_) | AstStatement::ReferenceExpr(_) => {
                Some(self.expr(n))
            }
            AstStatement::MultipliedStatement(_) | AstStatement::ExpressionList(_)
            | AstStatement::ParenExpression(_) => Some(self.expr(n)),
            _ => {
                self.gap(format!("init kind {}", kind(n)));
                None
            }
        }
    }

    fn at_of(&mut self, n: &AstNode) -> Option<mir::At> {
        self.note("at-binding");
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
            // address nodes: byte idx then bit idx (literals)
            let byte = h.address.first().map(lit_int).unwrap_or(0) as u64;
            let bit = h.address.get(1).map(|b| lit_int(b) as u8);
            return Some(mir::At { region, byte, bit, width });
        }
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

fn kind(n: &AstNode) -> &'static str {
    match &n.stmt {
        AstStatement::EmptyStatement(_) => "empty",
        AstStatement::DefaultValue(_) => "default-value",
        AstStatement::Literal(_) => "literal",
        AstStatement::MultipliedStatement(_) => "multiplied",
        AstStatement::ReferenceExpr(_) => "ref",
        AstStatement::Identifier(_) => "ident",
        AstStatement::BinaryExpression(_) => "binexpr",
        AstStatement::UnaryExpression(_) => "unexpr",
        AstStatement::ExpressionList(_) => "expr-list",
        AstStatement::ParenExpression(_) => "paren",
        AstStatement::RangeStatement(_) => "range",
        AstStatement::Assignment(_) => "assign",
        AstStatement::OutputAssignment(_) => "out-assign",
        AstStatement::CallStatement(_) => "call",
        AstStatement::ControlStatement(_) => "control",
        AstStatement::CaseCondition(_) => "case-cond",
        _ => "other",
    }
}

fn main() {
    let source = SourceCode::from(CORPUS);
    let idp = IdProvider::default();
    let factory = SourceLocationFactory::for_source(&source);
    let (unit, diags) = plc_compiler::parser::parse(
        plc_compiler::lexer::lex_with_ids(&source.source, idp, factory),
        LinkageType::Internal,
        "<corpus>",
    );
    println!("=== S5: ST → tplc-mir → blob → execute ===");
    println!("parse diagnostics: {}", diags.len());
    // NOTE: we lower the RAW AST — no pre_process(). Their AT-desugar
    // (PI globals), builtin injection etc. stay out of our pipeline.

    let mut lw = Lower::default();

    // user types first (struct/enum tables)
    for ut in &unit.user_types {
        match &ut.data_type {
            DataType::StructType { name, variables } => {
                let fields = variables
                    .iter()
                    .map(|v| (v.name.clone(), lw.ty_of(&v.data_type_declaration)))
                    .collect();
                let n = name.clone().unwrap_or_default();
                lw.user_types.insert(n.clone(), mir::Type::Struct { name: n, fields });
            }
            DataType::EnumType { name, elements, .. } => {
                let names = elements
                    .get_flat_reference_name()
                    .map(|s| vec![s.to_string()])
                    .unwrap_or_else(|| {
                        plc_ast::ast::get_enum_element_names(elements)
                    });
                let n = name.clone().unwrap_or_default();
                lw.user_types.insert(
                    n.clone(),
                    mir::Type::Enum { name: n, values: names },
                );
            }
            _ => lw.gap(format!("user-type {:?}", ut.data_type.get_name())),
        }
    }

    // POUs
    let mut module = mir::Module { pous: vec![], gvls: vec![], tasks: vec![], checked_arith: false };
    for pou in &unit.pous {
        let kind = match pou.kind {
            PouType::Program => mir::PouKind::Program,
            PouType::FunctionBlock => mir::PouKind::FunctionBlock,
            PouType::Function => mir::PouKind::Function,
            _ => {
                lw.gap(format!("pou kind {:?}", pou.kind));
                continue;
            }
        };
        let mut iface = Vec::new();
        for blk in &pou.variable_blocks {
            let constant = blk.constant;
            let retain = blk.retain;
            for v in &blk.variables {
                iface.push(lw.var_of(v, &blk.kind, retain, constant));
            }
        }
        // implicit function return var: FUNCTIONs assign to their own name
        if kind == mir::PouKind::Function {
            if let Some(rt) = &pou.return_type {
                iface.push(mir::VarDecl {
                    name: pou.name.clone(),
                    kind: mir::VarKind::Local,
                    ty: lw.ty_of(rt),
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
            .map(|i| mir::Body::Stmts(lw.stmts(&i.statements)))
            .unwrap_or(mir::Body::Stmts(vec![]));
        module.pous.push(mir::Pou {
            kind,
            name: pou.name.clone(),
            iface,
            body,
            ret_ty: pou.return_type.as_ref().map(|t| lw.ty_of(t)),
            prov: mir::Prov::None,
        });
    }

    println!("\nlowering stats:");
    for (k, n) in &lw.stats {
        println!("  {k:<16} {n}");
    }
    println!("lowering gaps ({}):", lw.gaps.len());
    for g in &lw.gaps {
        println!("  gap: {g}");
    }

    // ---- layout + execute ----
    let program = module
        .pous
        .iter()
        .find(|p| p.name == "MAIN")
        .expect("MAIN");
    let layout = mir::layout_program(&module, program);
    println!("\nlayout: blob {} B, align {} B, retain {} B @ {}",
             layout.total_size, layout.align, layout.retain_size, layout.retain_offset);
    println!("fields:");
    for (name, f) in &layout.fields {
        println!("  {name:<16} off {:>3} size {:>2} retain={}",
                 f.offset, f.size, f.retain);
    }

    let mut blob = vec![0u8; layout.total_size as usize];
    // apply scalar initializers (aggregate inits are a noted gap)
    {
        let mut rt = mir::Rt::new(&mut blob, &layout, &module, 0);
        let ctx = mir::interp::program_ctx(&layout);
        for v in &program.iface {
            if let Some(init) = &v.init {
                if layout.fields.contains_key(&v.name) {
                    let val = rt.eval(&ctx, init);
                    rt.write_field(&v.name, &val);
                }
            }
        }
    }

    // simulate cycles: sensor>0 for first 600ms then 0
    let period_ns = 10_000_000u64; // 10 ms
    let mut rt = mir::Rt::new(&mut blob, &layout, &module, 0);
    let ctx = mir::interp::program_ctx(&layout);
    let mut checks = 0u32;
    let mut fails = 0u32;
    macro_rules! check {
        ($cond:expr, $msg:expr) => {
            checks += 1;
            if !($cond) { fails += 1; println!("  FAIL {}", $msg); }
        };
    }

    let ton_q = mir::Ref::Field(
        Box::new(mir::Ref::Var("ton1".into())),
        "Q".to_string(),
    );
    rt.write_field("sensor", &mir::Value::I(5));
    // 60 cycles × 10ms = 600 ms with IN true → TON fires at 500ms
    for c in 0..60 {
        rt.now_ns = c * period_ns;
        if let mir::Body::Stmts(stmts) = &program.body {
            rt.exec_all(&ctx, stmts);
        }
        if c == 10 {
            check!(rt.read_ref(&ctx, &ton_q) == mir::Value::Bool(false), "TON off at 100ms");
        }
    }
    check!(rt.read_ref(&ctx, &ton_q) == mir::Value::Bool(true), "TON on after 500ms");
    check!(rt.read_field("running") == mir::Value::Bool(true), "running := ton1.Q");
    check!(rt.read_field("ticks") == mir::Value::U(200), "ticks reached 200 (REPEAT)");
    // arr[0] := F_Add(1,2)=5 each cycle, then FOR adds 1 → 6
    check!(rt.read_ref(&ctx, &mir::Ref::Index(
        Box::new(mir::Ref::Var("arr".into())),
        Box::new(mir::Expr {
            kind: mir::ExprKind::Lit(mir::Lit::Int(0)),
            ty: mir::Type::Bool,
            prov: mir::Prov::None,
        }))) == mir::Value::I(6), "arr[0] = F_Add(1,2)+1 = 6");

    println!("\nexecution results:");
    for name in ["running", "ticks", "r", "count", "sensor"] {
        if layout.fields.contains_key(name) {
            println!("  {name:<10} = {:?}", rt.read_field(name));
        }
    }
    println!("  ton1.Q       = {:?}", rt.read_ref(&ctx, &ton_q));
    println!("  ton1.ET      = {:?}", rt.read_ref(&ctx, &mir::Ref::Field(
        Box::new(mir::Ref::Var("ton1".into())), "ET".to_string())));
    println!("  ton1.cnt     = {:?}", rt.read_ref(&ctx, &mir::Ref::Field(
        Box::new(mir::Ref::Var("ton1".into())), "cnt".to_string())));
    println!("  arr[0]       = {:?}", rt.read_ref(&ctx, &mir::Ref::Index(
        Box::new(mir::Ref::Var("arr".into())), Box::new(mir::Expr {
            kind: mir::ExprKind::Lit(mir::Lit::Int(0)), ty: mir::Type::Bool, prov: mir::Prov::None }))));

    println!("\nS5 checks: {checks} run, {fails} failed");
    if fails == 0 {
        println!("S5 verdict: non-trivial POU ran end-to-end through B path — PASS");
    } else {
        println!("S5 verdict: {fails} behavioral mismatches — INVESTIGATE");
    }
}
