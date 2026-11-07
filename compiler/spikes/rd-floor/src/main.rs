//! P0.5 spike S6 — C-fallback floor-cost check (doc 13 §7).
//! Hand-rolled recursive-descent ST lexer + parser for a deliberately
//! small subset: FB/program declarations, VAR blocks, assignments,
//! FB-instance calls, and operator-precedence expressions. The point
//! is NOT completeness — it is measuring what the "own parser" floor
//! costs, to size option C against the 350-line B-path lowering.
//!
//! Subset deliberately excludes: VAR blocks beyond the 4 basic kinds,
//! types beyond scalar builtins, initializers, AT syntax, IF/CASE/loops,
//! arrays/structs/enums, user types, access modifiers — i.e. ~85% of
//! the real dialect surface (doc 05 grammar, ~150 EBNF productions).

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Ident(String),
    Int(i64),
    Real(f64),
    Str(String),
    Kw(Kw),
    Sym(&'static str),
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Kw {
    FunctionBlock, Program, Var, VarInput, VarOutput, EndVar,
    EndFunctionBlock, EndProgram, If, Then, EndIf, Assign, Semi, Colon,
    Of, True, False,
}

fn lex(src: &str) -> Result<Vec<(Tok, usize)>, String> {
    let b = src.as_bytes();
    let mut i = 0;
    let mut out = Vec::new();
    while i < b.len() {
        let c = b[i] as char;
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c == '(' && b.get(i + 1) == Some(&b'*') {
            // (* comment *)
            i += 2;
            while i + 1 < b.len() && !(b[i] == b'*' && b[i + 1] == b')') {
                i += 1;
            }
            i += 2;
            continue;
        }
        if c == '/' && b.get(i + 1) == Some(&b'/') {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let s = i;
            while i < b.len() && ((b[i] as char).is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            let w = &src[s..i];
            let kw = match w.to_ascii_uppercase().as_str() {
                "FUNCTION_BLOCK" => Some(Kw::FunctionBlock),
                "PROGRAM" => Some(Kw::Program),
                "VAR" => Some(Kw::Var),
                "VAR_INPUT" => Some(Kw::VarInput),
                "VAR_OUTPUT" => Some(Kw::VarOutput),
                "END_VAR" => Some(Kw::EndVar),
                "END_FUNCTION_BLOCK" => Some(Kw::EndFunctionBlock),
                "END_PROGRAM" => Some(Kw::EndProgram),
                "IF" => Some(Kw::If),
                "THEN" => Some(Kw::Then),
                "END_IF" => Some(Kw::EndIf),
                "TRUE" => Some(Kw::True),
                "FALSE" => Some(Kw::False),
                _ => None,
            };
            out.push((
                kw.map(Tok::Kw).unwrap_or_else(|| Tok::Ident(w.to_string())),
                s,
            ));
            continue;
        }
        if c.is_ascii_digit() {
            let s = i;
            while i < b.len() && ((b[i] as char).is_ascii_digit() || b[i] == b'_') {
                i += 1;
            }
            let mut is_real = false;
            if i < b.len() && b[i] == b'.' && b.get(i + 1).map(|c| c.is_ascii_digit()).unwrap_or(false) {
                is_real = true;
                i += 1;
                while i < b.len() && b[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let t = &src[s..i].replace('_', "");
            out.push((
                if is_real {
                    Tok::Real(t.parse().map_err(|_| "bad real")?)
                } else {
                    Tok::Int(t.parse().map_err(|_| "bad int")?)
                },
                s,
            ));
            continue;
        }
        if c == '\'' {
            let s = i;
            i += 1;
            while i < b.len() && b[i] != b'\'' {
                i += 1;
            }
            let t = src[s + 1..i].replace("$'", "'");
            i += 1;
            out.push((Tok::Str(t), s));
            continue;
        }
        // multi-char ops first
        let rest = &src[i..];
        let sym = [":=", "=>", "<=", ">=", "<>", ".."]
            .iter()
            .find(|s| rest.starts_with(**s));
        if let Some(s) = sym {
            out.push((Tok::Sym(s), i));
            i += s.len();
            continue;
        }
        let singles = ";:+-*/()=<>,.";
        if singles.contains(c) {
            out.push((Tok::Sym(Box::leak(c.to_string().into_boxed_str())), i));
            i += 1;
            continue;
        }
        return Err(format!("lex error at {i}: {c:?}"));
    }
    out.push((Tok::Eof, src.len()));
    Ok(out)
}

// ---- tiny AST ----
#[derive(Debug)]
enum Expr {
    Lit(String),
    Var(String),
    Bin(&'static str, Box<Expr>, Box<Expr>),
    Un(&'static str, Box<Expr>),
}
#[derive(Debug)]
enum Stmt {
    Assign(String, Expr),
    Call(String, Vec<(String, Expr)>),
}
#[derive(Debug)]
struct Pou {
    kind: &'static str,
    name: String,
    vars: Vec<(String, String, &'static str)>, // name, type, block
    body: Vec<Stmt>,
}

struct P<'a> {
    t: &'a [(Tok, usize)],
    i: usize,
}
impl<'a> P<'a> {
    fn peek(&self) -> &Tok {
        &self.t[self.i].0
    }
    fn next(&mut self) -> Tok {
        let t = self.t[self.i].0.clone();
        self.i += 1;
        t
    }
    fn at_sym(&self, s: &str) -> bool {
        matches!(self.peek(), Tok::Sym(x) if *x == s)
    }
    fn eat_sym(&mut self, s: &str) -> Result<(), String> {
        if self.at_sym(s) {
            self.i += 1;
            Ok(())
        } else {
            Err(format!("expected {s:?} got {:?} @{}", self.peek(), self.t[self.i].1))
        }
    }
    fn at_kw(&self, k: Kw) -> bool {
        matches!(self.peek(), Tok::Kw(x) if *x == k)
    }
    fn ident(&mut self) -> Result<String, String> {
        match self.next() {
            Tok::Ident(s) => Ok(s),
            t => Err(format!("expected ident got {t:?}")),
        }
    }

    // expr := or_expr
    fn expr(&mut self) -> Result<Expr, String> {
        self.or_expr()
    }
    fn or_expr(&mut self) -> Result<Expr, String> {
        let mut l = self.xor_expr()?;
        while matches!(self.peek(), Tok::Ident(w) if w == "OR") {
            self.next();
            let r = self.xor_expr()?;
            l = Expr::Bin("OR", Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn xor_expr(&mut self) -> Result<Expr, String> {
        let mut l = self.and_expr()?;
        while matches!(self.peek(), Tok::Ident(w) if w == "XOR") {
            self.next();
            let r = self.and_expr()?;
            l = Expr::Bin("XOR", Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn and_expr(&mut self) -> Result<Expr, String> {
        let mut l = self.cmp_expr()?;
        while matches!(self.peek(), Tok::Ident(w) if w == "AND") || self.at_sym("&") {
            self.next();
            let r = self.cmp_expr()?;
            l = Expr::Bin("AND", Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn cmp_expr(&mut self) -> Result<Expr, String> {
        let mut l = self.add_expr()?;
        loop {
            let op = match self.peek() {
                Tok::Sym("=") => "=",
                Tok::Sym("<>") => "<>",
                Tok::Sym("<") => "<",
                Tok::Sym("<=") => "<=",
                Tok::Sym(">") => ">",
                Tok::Sym(">=") => ">=",
                _ => break,
            };
            self.next();
            let r = self.add_expr()?;
            l = Expr::Bin(op, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn add_expr(&mut self) -> Result<Expr, String> {
        let mut l = self.mul_expr()?;
        loop {
            let op = match self.peek() {
                Tok::Sym("+") => "+",
                Tok::Sym("-") => "-",
                Tok::Ident(w) if w == "MOD" => "MOD",
                _ => break,
            };
            if op == "MOD" || self.at_sym(op) {
                self.next();
                let r = self.mul_expr()?;
                l = Expr::Bin(op, Box::new(l), Box::new(r));
            } else {
                break;
            }
        }
        Ok(l)
    }
    fn mul_expr(&mut self) -> Result<Expr, String> {
        let mut l = self.unary()?;
        loop {
            let op = match self.peek() {
                Tok::Sym("*") => "*",
                Tok::Sym("/") => "/",
                _ => break,
            };
            self.next();
            let r = self.unary()?;
            l = Expr::Bin(op, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Tok::Sym("-") => {
                self.next();
                Ok(Expr::Un("-", Box::new(self.unary()?)))
            }
            Tok::Ident(w) if w == "NOT" => {
                self.next();
                Ok(Expr::Un("NOT", Box::new(self.unary()?)))
            }
            _ => self.primary(),
        }
    }
    fn primary(&mut self) -> Result<Expr, String> {
        match self.next() {
            Tok::Int(v) => Ok(Expr::Lit(v.to_string())),
            Tok::Real(v) => Ok(Expr::Lit(v.to_string())),
            Tok::Str(v) => Ok(Expr::Lit(format!("'{v}'"))),
            Tok::Kw(Kw::True) => Ok(Expr::Lit("TRUE".into())),
            Tok::Kw(Kw::False) => Ok(Expr::Lit("FALSE".into())),
            Tok::Sym("(") => {
                let e = self.expr()?;
                self.eat_sym(")")?;
                Ok(e)
            }
            Tok::Ident(w) => {
                // qualified ref a.b.c
                let mut name = w;
                while self.at_sym(".") {
                    self.next();
                    name.push('.');
                    name.push_str(&self.ident()?);
                }
                Ok(Expr::Var(name))
            }
            t => Err(format!("primary: unexpected {t:?}")),
        }
    }

    // stmt := ident (':=' expr | '(' args ')') ';'
    fn stmt(&mut self) -> Result<Option<Stmt>, String> {
        match self.peek() {
            Tok::Ident(_) => {
                let name = self.ident()?;
                // qualified lhs
                let mut name = name;
                while self.at_sym(".") {
                    self.next();
                    name.push('.');
                    name.push_str(&self.ident()?);
                }
                if self.at_sym("(") {
                    // FB call: name(params)
                    self.next();
                    let mut args = Vec::new();
                    while !self.at_sym(")") {
                        let an = self.ident()?;
                        self.eat_sym(":=")?;
                        let ae = self.expr()?;
                        args.push((an, ae));
                        if self.at_sym(",") {
                            self.next();
                        }
                    }
                    self.eat_sym(")")?;
                    self.eat_sym(";")?;
                    Ok(Some(Stmt::Call(name, args)))
                } else {
                    self.eat_sym(":=")?;
                    let e = self.expr()?;
                    self.eat_sym(";")?;
                    Ok(Some(Stmt::Assign(name, e)))
                }
            }
            Tok::Kw(Kw::EndFunctionBlock) | Tok::Kw(Kw::EndProgram) | Tok::Eof => Ok(None),
            t => Err(format!("stmt: unexpected {t:?}")),
        }
    }

    fn pou(&mut self) -> Result<Option<Pou>, String> {
        let kind = if self.at_kw(Kw::FunctionBlock) {
            self.next();
            "FUNCTION_BLOCK"
        } else if self.at_kw(Kw::Program) {
            self.next();
            "PROGRAM"
        } else {
            return Ok(None);
        };
        let name = self.ident()?;
        let mut vars = Vec::new();
        loop {
            let block = if self.at_kw(Kw::VarInput) {
                self.next();
                Some("VAR_INPUT")
            } else if self.at_kw(Kw::VarOutput) {
                self.next();
                Some("VAR_OUTPUT")
            } else if self.at_kw(Kw::Var) {
                self.next();
                Some("VAR")
            } else {
                None
            };
            let Some(blk) = block else { break };
            while !self.at_kw(Kw::EndVar) {
                let vn = self.ident()?;
                self.eat_sym(":")?;
                let tn = self.ident()?;
                self.eat_sym(";")?;
                vars.push((vn, tn, blk));
            }
            self.next(); // END_VAR
        }
        let mut body = Vec::new();
        while let Some(s) = self.stmt()? {
            body.push(s);
        }
        // END_*
        self.next();
        Ok(Some(Pou { kind, name, vars, body }))
    }
}

const SUBSET: &str = r#"
FUNCTION_BLOCK FB_Ramp
VAR_INPUT speed : INT; END_VAR
VAR_OUTPUT out : DINT; END_VAR
VAR acc : DINT; END_VAR
    acc := acc + speed * 2 - (3 + 1);
    out := acc;
END_FUNCTION_BLOCK

PROGRAM MAIN
VAR
    r1 : FB_Ramp;
    result : DINT;
    flag : BOOL;
END_VAR
    r1(speed := 4 + 1);
    result := r1.out + 10 * 2;
    flag := result > 0 AND NOT FALSE OR (1 < 2);
END_PROGRAM
"#;

fn main() {
    let toks = lex(SUBSET).expect("lex");
    println!("=== S6: RD-parser floor check (C fallback cost) ===");
    println!("tokens: {}", toks.len());
    let mut p = P { t: &toks, i: 0 };
    let mut npou = 0;
    let mut nstmt = 0;
    let mut nvar = 0;
    loop {
        match p.pou() {
            Ok(Some(pou)) => {
                npou += 1;
                nstmt += pou.body.len();
                nvar += pou.vars.len();
            }
            Ok(None) => break,
            Err(e) => {
                println!("PARSE ERROR: {e}");
                break;
            }
        }
    }
    println!("parsed: {npou} POUs, {nvar} vars, {nstmt} stmts");
    let loc = include_str!("main.rs").lines().count();
    println!("parser+lexer LOC: {loc} (this file)");
    println!("\nS6 verdict: subset floor = ~{loc} LOC for ~15% of dialect surface.");
    println!("Full-ST RD parser estimate: ~3-5k LOC + sema/typeck (weeks),");
    println!("vs B-path lowering of ~350 LOC observed in S5 — C floor >> B.");
}
