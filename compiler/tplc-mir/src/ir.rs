//! `tplc-mir` IR — the typed, provenance-complete middle layer (doc 07 §2).
//! ST and FBD frontends both produce this; everything below it
//! (layout → codegen → symmap/migrate) is unconditionally ours.

/// Provenance: where a node came from. FBD nodes arrive as ids, ST as
/// byte spans, generated code marks itself (dual provenance, G8).
#[derive(Debug, Clone, PartialEq)]
pub enum Prov {
    St { file: u32, offset: u32, len: u32 },
    Fbd { sheet: u32, node: u32, pin: u16 },
    Generated,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Bool,
    /// signed ints: SINT=8 INT=16 DINT=32 LINT=64; unsigned same widths
    Int {
        signed: bool,
        bits: u8,
    },
    Real {
        bits: u8,
    }, // 32 | 64
    /// TIME family stored as i64 ns (TIME/DATE/TOD/DT all time-valued)
    Time,
    /// STRING(cap): blob = u8 len + u8 data[cap], cap+1 rounded to 2
    Str {
        cap: u16,
    },
    /// WSTRING(cap): blob = u16 len + u16 data[cap] (UTF-16 code
    /// units), 2 + 2*cap rounded to 2
    WStr {
        cap: u16,
    },
    /// REFERENCE TO T: blob = u64 byte offset into the same blob
    /// (u64::MAX = null). Pointer-free by construction — the offset
    /// survives relocation but not a layout change (see migrate).
    Ref {
        to: Box<Type>,
    },
    Array {
        elem: Box<Type>,
        lo: i64,
        hi: i64,
    },
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Enum {
        name: String,
        values: Vec<String>,
    },
    /// FB instance type reference (by qualified FB name)
    Fb {
        name: String,
    },
}

impl Type {
    /// byte size in the state blob (layout §3.3)
    pub fn size(&self) -> u64 {
        match self {
            Type::Bool => 1,
            Type::Int { bits, .. } => (*bits as u64) / 8,
            Type::Real { bits } => (*bits as u64) / 8,
            Type::Time => 8,
            Type::Str { cap } => align2(1 + *cap as u64),
            Type::WStr { cap } => align2(2 + 2 * *cap as u64),
            Type::Ref { .. } => 8,
            Type::Array { elem, lo, hi } => elem.size() * (*hi - *lo + 1).max(0) as u64,
            Type::Struct { fields, .. } => {
                let mut sz = 0u64;
                for (_, t) in fields {
                    sz = align(sz, t.align()) + t.size();
                }
                align(sz, self.align())
            }
            Type::Enum { .. } => 4,
            Type::Fb { .. } => 0, // resolved by layout pass (sub-object)
        }
    }
    pub fn align(&self) -> u64 {
        match self {
            Type::Bool => 1,
            Type::Int { bits, .. } | Type::Real { bits } => ((*bits as u64) / 8).min(8),
            Type::Time => 8,
            Type::Str { .. } | Type::WStr { .. } => 2,
            Type::Ref { .. } => 8,
            Type::Array { elem, .. } => elem.align(),
            Type::Struct { fields, .. } => fields.iter().map(|(_, t)| t.align()).max().unwrap_or(1),
            Type::Enum { .. } => 4,
            Type::Fb { .. } => 8,
        }
    }
}

fn align(v: u64, a: u64) -> u64 {
    (v + a - 1) & !(a - 1)
}
fn align2(v: u64) -> u64 {
    (v + 1) & !1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarKind {
    Input,
    Output,
    InOut,
    Local,
    Global,
    Constant,
    External,
}

/// An `AT`-bound variable ↔ process-image binding (doc 01 §5).
#[derive(Debug, Clone)]
pub struct IoBind {
    /// qualified name `POU.var`
    pub name: String,
    pub at: At,
    pub ty: Type,
}

/// An `AT %IX0.0` style image address (doc 05 §4: only via declared names).
#[derive(Debug, Clone, PartialEq)]
pub struct At {
    pub region: AtRegion,
    pub byte: u64,
    pub bit: Option<u8>,
    /// declared width code: X B W D L
    pub width: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtRegion {
    I,
    Q,
    M,
}

#[derive(Debug, Clone)]
pub struct VarDecl {
    pub name: String,
    pub kind: VarKind,
    pub ty: Type,
    pub init: Option<Expr>,
    pub retain: bool,
    pub at: Option<At>,
    pub probe: bool,
    pub prov: Prov,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PouKind {
    Program,
    FunctionBlock,
    Function,
}

#[derive(Debug, Clone)]
pub struct Pou {
    pub kind: PouKind,
    pub name: String,
    pub iface: Vec<VarDecl>,
    pub body: Body,
    /// function return type (Function only)
    pub ret_ty: Option<Type>,
    pub prov: Prov,
}

#[derive(Debug, Clone)]
pub enum Body {
    Stmts(Vec<Stmt>),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub prov: Prov,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Assign {
        lhs: Ref,
        rhs: Expr,
    },
    If {
        branches: Vec<(Expr, Vec<Stmt>)>,
        else_: Vec<Stmt>,
    },
    Case {
        sel: Expr,
        arms: Vec<(Vec<CaseSel>, Vec<Stmt>)>,
        else_: Vec<Stmt>,
    },
    For {
        var: String,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
        body: Vec<Stmt>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    Repeat {
        cond: Expr,
        body: Vec<Stmt>,
    },
    /// FB-instance invocation with named bindings (`ton1(IN := ..)`)
    Call {
        inst: Ref,
        args: Vec<(String, Expr)>,
    },
    Return(Option<Expr>),
    Exit,
    Continue,
}

#[derive(Debug, Clone)]
pub enum CaseSel {
    Lit(Expr),
    Range(Expr, Expr),
}

/// Lvalue/reference path — shared by reads and writes.
#[derive(Debug, Clone)]
pub enum Ref {
    Var(String),
    /// field of an FB instance or struct (`ton1.Q`)
    Field(Box<Ref>, String),
    /// array element (`arr[i]`)
    Index(Box<Ref>, Box<Expr>),
    /// dereference a REF (`r^`, or auto-deref on read)
    Deref(Box<Ref>),
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub ty: Type,
    pub prov: Prov,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Lit(Lit),
    Ref(Ref),
    Bin(BinOp, Box<Expr>, Box<Expr>),
    Un(UnOp, Box<Expr>),
    /// function call (FUNCTION POUs)
    Call {
        fun: String,
        args: Vec<(String, Expr)>,
    },
    /// REF(x) / `&x`: the blob offset of an lvalue (type Ref)
    RefTo {
        r: Ref,
    },
    /// explicit conversion inserted by sema — codegen never guesses
    Conv {
        to: Type,
        e: Box<Expr>,
    },
}

/// task return codes (mirror of tplc_abi.h TPLC_TASK_E_*)
pub const TPLC_TASK_E_FAULT: i32 = -2;
pub const TPLC_TASK_E_PANIC: i32 = -17;
pub const TPLC_TASK_E_BOUNDS: i32 = -18;
pub const TPLC_TASK_E_DIVZERO: i32 = -19;
pub const TPLC_TASK_E_ABORTED: i32 = -22;
pub const TPLC_TASK_E_OVERRUN: i32 = -23;

#[derive(Debug, Clone)]
pub enum Lit {
    Bool(bool),
    Int(i64),
    UInt(u64),
    Real(f64),
    TimeNs(i64),
    Str(String),
    /// wide string literal — encoded to UTF-16 code units at
    /// interp/codegen (never stored as narrow bytes)
    WStr(String),
    EnumVal { ty: String, val: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Xor,
    AndThen,
    OrElse,
    Shl,
    Shr,
    Rol,
    Ror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub struct TaskDecl {
    pub name: String,
    /// cyclic period; 0 = event-driven (reserved)
    pub period_ns: u64,
    pub wcet_hint_ns: u64,
    pub priority: u8,
    /// program instances bound to this task
    pub programs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Gvl {
    pub name: String,
    pub vars: Vec<VarDecl>,
    pub retain: bool,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub pous: Vec<Pou>,
    pub gvls: Vec<Gvl>,
    pub tasks: Vec<TaskDecl>,
}
