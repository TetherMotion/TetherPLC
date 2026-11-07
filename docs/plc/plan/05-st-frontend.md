# 05 — ST Frontend: Dialect Specification & `tplc-mir` Requirements

> **Status:** work-item document; deleted when `docs/plc/st-dialect.md`
> (the language reference) exists and the chosen frontend (per 13)
> implements the conformance tables herein.
>
> **Satisfies:** G9 (IEC subset honesty), G10 (SFC hooks), D6 (v1=ST+FBD).
> **Depends on:** 13 (parser strategy; this spec is strategy-neutral —
> it defines WHAT is accepted, not who parses it).
> **Informs:** 07 (lowering), 11 (editor language services), 14 (oracle).

---

## 1. Position

This chapter specifies the **TetherPLC ST dialect**: the set of IEC
61131-3 structured-text constructs the compiler accepts, plus documented
extensions and deliberate deviations. It is the contract every user sees,
the input to `tplc-mir`, and the yardstick the matiec oracle measures
against (doc 14). It is deliberately a *subset with intent*: everything
accepted must lower to our static, allocation-free, migratable execution
model — constructs that violate that model are rejected loudly rather
than emulated badly.

Conformance posture: **IEC 61131-3 ST subset** — we do not claim edition-3
full conformance; the tables in §3–§9 are the conformance statement.

## 2. Compilation unit and POU model

### 2.1 Source organization

- One or more `.st` text files (UTF-8, LF; BOM tolerated). A *project*
  aggregates files via the PLCopen XML project or explicit file list;
  there is no textual include directive (file lists only — keeps module
  boundaries explicit and the oracle comparisons clean).
- Declared entities: `PROGRAM`, `FUNCTION_BLOCK`, `FUNCTION`,
  `TYPE` declarations, and `VAR_GLOBAL` blocks (GVLs). All are POU kinds
  or libraries thereof.

### 2.2 POUs

| kind | v1 semantics |
|---|---|
| `PROGRAM` | instantiated once per task binding; owns VARs incl. located vars; may call FBs/functions; cannot be called by other POUs |
| `FUNCTION_BLOCK` | stateful; instantiated by name (blob-resident); called with named/positional args; may declare VAR/INPUT/OUTPUT/IN_OUT/TEMP/CONSTANT |
| `FUNCTION` | stateless, single return (`FUNCTION <name> : <type>`); assignable to name; EN/ENO forms |

Instantiation: `VAR myTon : TON; END_VAR` inside PROGRAM/FB — static
instance graph only (see §10 prohibitions).

### 2.3 Task binding (program instantiation)

Programs bind to tasks **in configuration, not source** — matching IEC's
CONFIGURATION/RESOURCE model but expressed in our project file
(`program "main_prog" task MAIN` style entries), because textual
CONFIGURATION syntax adds a third language for no user benefit. The
compiler resolves bindings → `TplcTaskDesc` table (doc 02).

## 3. Data types

### 3.1 Elementary

| family | types | notes |
|---|---|---|
| Boolean | `BOOL` | 1 byte storage in blob (bit-packing is a codegen decision, not semantic) |
| Signed int | `SINT INT DINT LINT` | two's complement |
| Unsigned | `USINT UINT UDINT ULINT` `BYTE WORD DWORD LWORD` | BIT-family = unsigned + bitwise ops |
| Float | `REAL LREAL` | IEEE-754 binary32/64, strict semantics |
| Time | `TIME` (ns i64) `LTIME` (v1.5) `DATE` `TIME_OF_DAY/TOD` `DATE_AND_TIME/DT` | canonical ns/epoch reps |
| String | `STRING(n)` | fixed capacity ≤255, length-prefixed, blob-resident; `WSTRING` v1.5 |
| Char | `CHAR`/`WCHAR` (v1.5) | inside STRING handling |

### 3.2 Derived

| construct | v1 |
|---|---|
| `TYPE ... STRUCT ... END_STRUCT` | yes; nested; member initializers |
| `TYPE ... : ARRAY[a..b (,c..d)] OF T` | yes; static bounds, ≤4 dims (indexing lower/upper arbitrary per IEC) |
| `TYPE E : (a, b, c) END_TYPE` | yes; explicit values allowed |
| subrange `TYPE R : INT(0..100)` | yes; range-checked on assignment (see §11) |
| `TYPE ALIAS : INT` | yes |
| `POINTER TO` | **no (v1)** — §10 |
| `REFERENCE TO` | **no (v1)** — §10 (inout covers by-ref needs) |
| unions | no (not IEC anyway) |

### 3.3 Literals

All IEC forms: `2#1010`, `8#17`, `16#FF`, typed (`INT#-5`, `DINT#16#1`),
underscore separators `1_000`, exponents `1.5e3`, `T#1h30m`, `T#250ms`,
`D#2026-11-07`, `TOD#12:34:56.789`, `DT#...`, strings `'...'` with `$$`
escapes, `TRUE/FALSE`.

## 4. VAR declarations

```
VAR | VAR_INPUT | VAR_OUTPUT | VAR_IN_OUT | VAR_GLOBAL | VAR_EXTERNAL
VAR_TEMP | VAR_CONSTANT | VAR_RETAIN | VAR_STAT(rsvd)
  name (,name)* : type (:= init)?  (;)*
END_VAR
```

Semantics notes:

- `VAR` inside PROGRAM/FB = instance state (blob); `VAR_TEMP`/`VAR` inside
  FUNCTION = frame temporaries (stack — allowed! temporaries are not
  state; the *blob* holds only cross-invocation state — important
  distinction for codegen, doc 07).
- `VAR_INPUT/OUTPUT` = FB/function interface; copied in/out at call.
- `VAR_IN_OUT` = **call-time binding** to caller's lvalue — implemented
  as an offset-typed reference parameter valid for the call duration;
  never stored in the blob (keeps G4's no-pointers invariant; an inout
  chain through calls is statically resolved or rejected FBD014-style).
- `VAR_EXTERNAL` (FB referencing a GVL var) supported — resolves to the
  GVL's blob field; **diagnosed** (lint `st-external-warn`: implicit
  global coupling hurts testability; off by default for imports).
- `VAR_RETAIN` marks fields for the retain region (doc 02/04).
- `VAR_CONSTANT` anywhere → rodata, never blob.
- Located variables: `x AT %IX0.0 : BOOL` and wildcard `AT %I* : INT`
  (resolved by io map, doc 09); AT allowed on PROGRAM VARs and GVL vars
  only. Fixed-address forms validated against ownership table; two ATs
  overlapping = error.
- Initialization: compile-time constant exprs; cold-init defaults →
  symmap defaults blob (doc 01 §6.4).

### 4.1 Attributes (extension, IEC attribute syntax)

```
{attribute 'retain'}            // per-var retain (alternative to VAR_RETAIN blocks)
{attribute 'probe'}             // expose as probe point
{attribute 'noprofile'}         // exclude from timing instrumentation
{attribute 'task' 'MAIN'}       // bind GVL access?? — NO: reserved, see doc09
{tether 'deprecated' 'use X'}   // diagnostics hint
```

Vendor attributes live in `tether` namespace in source; standard
`{attribute '...'}` keys per IEC are honored where listed.

## 5. Expressions

Operators (precedence IEC Table; parentheses override):

| group | ops |
|---|---|
| arithmetic | `+ - * / MOD` `**`(EXPT) unary `-`/`+` `NOT`(bool+bitwise) |
| comparison | `= <> < <= > >=` (→BOOL) |
| bool logic | `AND OR XOR` `AND_THEN`/`OR_ELSE` (v1: `AND/OR` evaluate per IEC — we define **short-circuit for AND/OR/XOR?** IEC leaves it impl-defined; we define full evaluation EXCEPT `AND_THEN/OR_ELSE` which short-circuit explicitly — deterministic + no surprise) |
| bit ops | via BOOL/bitstring ops + stdlib shifts (`SHL/SHR/ROL/ROR`) |
| string | `+` concat? — v1: concat via `CONCAT()` stdlib only (keeps `+` overload matrix small) |

Implicit conversions: IEC hierarchy `SINT→INT→DINT→LINT`,
`USINT→UINT→…`, `INT→REAL→LREAL`, `BOOL` never implicit, `BYTE`↔int
never implicit (explicit cast), signed↔unsigned never implicit,
`TIME`/`DATE`/numeric never implicit. Explicit: `INT(x)`/`TO_*` stdlib.
Strictness is deliberate — implicit conversion bugs are a top PLC-field
defect class; the oracle suite (doc 14) pins behavior.

Division: `/` on ints = truncating IEC semantics, **div-zero → task
fault** `TPLC_TASK_E_DIVZERO` (not UB, not trap — deterministic fault,
doc 02). Float `/` = IEEE (Inf/NaN propagate). `MOD` sign semantics
follow IEC (dividend-signed).

Overflow: defined **wrapping** (two's complement) on all int ops —
deviation vs IEC-undefined; plus compile flag `checks.overflow` →
faulting variant (§11 checked mode). Float: no ffast-math, IEEE strict.

## 6. Statements

```
assignment      lvalue := expr ;
IF/ELSIF/ELSE/END_IF
CASE expr OF v: stmts; v1,v2: ...; ELSE stmts; END_CASE  // int/enum selectors
FOR v := a TO b (BY c) DO stmts END_FOR
WHILE cond DO stmts END_WHILE
REPEAT stmts UNTIL cond END_REPEAT
EXIT ;  CONTINUE ;   RETURN ;
FB-instance(args) ;  // incl. named arg assoc  x:=, =>
function call as expression
label:  and GOTO?? — NO: ST has no labels (SFC handles sequencing)
```

Rules:

- `FOR`: bounds evaluated once at entry (IEC); loop var declared int
  type; **static bound required?** — runtime bounds allowed (FOR loops
  with dynamic `TO` are legal; WCET enforcement is the checkpoint
  mechanism's job, doc 04) but `FOR` is checkpoint-instrumented like all
  loops.
- `CASE` selector type: INT-family/enum/CHAR; `ELSE` optional.
- `EXIT`/`CONTINUE` innermost loop only (labeled loops rejected — not
  IEC).
- Empty statements / stray `;` tolerated with warning.

## 7. Calls

```
fb(in1:=e1, in2:=e2);          // named
fb(e1, e2);                    // positional
y := fb.out1;                  // post-call access to outputs (IEC)
y := func(e1, e2);             // function → return value
y := func();  y := ENO-form?   // EN/ENO: stdlib fns carry them; user FUNCTIONs: always EN/ENO (we add implicitly — deviation: IEC makes them optional; uniform is simpler and predictable)
```

Named-argument evaluation order: argument expressions evaluated in
**source order** into call-bound temps before the call (IEC order is
undefined; deterministic requires pinning). Nested FB calls in args are
evaluated inside-out. Recursion (direct or via mutual calls) is a
compile error §10.

## 8. Scoping and namespaces

- Name scope: GVL names + POU-local + FB member access via `inst.member`.
- Shadowing rules: local > POU-interface > GVL; member access never
  ambiguous; shadowing a GVL name warns (`st-shadow`).
- No textual modules/namespaces in v1: GVLs are named (`GVL_IO.x`),
  POUs occupy one flat project namespace (IEC-consistent).
- Reserved words: full IEC keyword list + `TPLC_`-prefixed internals.

## 9. Direct addresses and access paths

```
%I/%Q/%M + X|B|W|D|L + addr(.bit)
  AT %IX1.3 : BOOL;   // image byte 1 bit 3
  AT %QD4   : DWORD;
  AT %MB100 : BYTE;   // marker (runtime memory)
```

- Syntax full IEC `%` notation in AT clauses. Bare `%IX0.0` *expressions*
  inline in code: **not supported** — located access is always through
  declared AT-bound names (auditable IO surface; mapping is config-
  owned, doc 09). Inline `%` reads are the classic way IO coupling
  hides; we forbid them and diagnostics say why.
- `%M` addressing: allowed via AT (marker region); also plain VARs serve
  the same role — `%M` is for imported-code compatibility.
- Wildcard AT (`%I*`) = resolved at link-to-image stage; missing owner →
  deploy error not compile error (the program is image-agnostic until
  mapped — important property: same package deploys to different
  stations via different maps).

## 10. Prohibitions (v1, compile errors with reasons)

| construct | reason |
|---|---|
| `POINTER TO`/`ADR()`/`^` deref | stored pointers break relocate/migrate (G4); REF-style uses covered by IN_OUT |
| `REFERENCE TO`/`REF=` | same; revisit v1.5 with offset-repr design |
| recursion / cyclic call graph | static stack + WCET model required |
| function values / indirect calls | static call graph required |
| dynamic arrays, `NEW`/`DELETE`, heap | allocation-free scan path (G2) |
| `VAR_ACCESS`/`VAR_CONFIG` | access paths via symmap/debug API instead |
| inline `__asm`, external C calls | auditability; named services cover host ops |
| textual `INCLUDE` | project-level file lists only |
| `GOTO`/labels in ST | not IEC-ST; structured only |
| `VAR_STAT` | reserved; semantics deferred |
| threads/locks/wait primitives | single-thread cooperative model (doc 02) |
| nondeterministic builtins (`RAND` unless seeded API) | determinism G7 — `RAND` absent v1; seeded PRNG FB may ship in tplc-rt |

## 11. Checked mode (`checks` compile profile)

Per-POU or global `{tether 'checks' 'on'}` / compile flag:

- array bounds, subrange assignment, div-zero (always on — see §5),
  string overflow, enum validity, **int overflow** (opt-in), REAL→INT
  range on cast.
- Violations = `TPLC_TASK_E_BOUNDS/DIVZERO/OVERFLOW/RANGE` task faults
  (doc 02), never silent UB. Default profile: IEC-mandated checks on,
  overflow off (wrap), per §5.

## 12. Diagnostics requirements

- Every rejection: `{code, span, message, [help], [notes]}` —
  rustc-quality rendering (multi-line snippets, labeled spans); FBD-side
  provenance renders node refs when mir came from FBD.
- Required classes: parse errors (recovery to decl boundaries), name
  errors (did-you-mean), type errors (expected/found + conversion hint),
  prohibition errors (the §10 reason shown verbatim), AT/mapping
  mismatches, call-shape errors, sema warnings (lint set:
  `unused-var, shadow, unreachable-case, dead-code, implicit-narrow,
  external-warn, no-jumps`).
- Codes: `TPLC_E_ST_*` numbering stable across versions (diagnostic
  codes are API).
- Max-quality bar: the eval in doc 13 must reproduce this rendering for
  whichever parser ships — a frontend that can't carry spans+notes to
  this standard fails acceptance regardless of option chosen.

## 13. Mapping to `tplc-mir`

Per-POU translation (normative summary; doc 07 owns codegen):

- `PROGRAM`/`FB` instance → blob sub-struct type + entry/call fns;
  member/VAR fields → struct fields (retain-flagged into retain region).
- `FUNCTION` → pure fn (may inline).
- Statements/exprs → mir nodes with spans; loops marked for checkpoint
  insertion; calls → `FBCall {instance_offset, arg_bindings}`.
- AT-bound vars → `io_binding` entries + shadow fields (never direct
  image access in code).
- `VAR_IN_OUT` → mir `InOutBinding` (offset reference, call-scoped).

## 14. Conformance tables (to be maintained in as-built doc)

Format per feature: `syntax | status: ok|ext|dev|rej | oracle-test id`.
Initial corpus: the IEC annex examples + Beremiz demo corpus +
OpenPLC test programs — each compiled through our frontend and the
matiec oracle (doc 14) with IO-trace equivalence on the runnable subset.

## 15. Iteration-2 hooks

- `ACTION name ... END_ACTION` syntax reserved (SFC attaches POUs).
- `STEP/TRANSITION` keywords reserved (not parsed v1 — clear error).
- `INTERFACE`/`METHOD` reserved for later OOP evaluation (NOT promised —
  our stance: composition via FBs suffices for scan logic; evaluate with
  user demand data, not speculatively).
- `WSTRING`, `LTIME`, `REF` in the deferred list with conditions noted.
- LD-in-ST? — no; LD is a view (doc 06), not a text form.

## 16. Error codes (owned by this chapter)

`TPLC_E_ST_LEX, _ST_PARSE, _ST_NAME, _ST_TYPE, _ST_PROHIBITED,
_ST_ATMAP, _ST_CALLSHAPE, _ST_RECURSION, _ST_LINT`

## 17. Acceptance criteria

1. Dialect reference doc generated from this spec; every §3–§10 row has
   a positive + negative conformance test (≥400 cases).
2. Parser (whichever strategy): 100% of the corpus either compiles
   correctly or fails with the *assigned* diagnostic code — no crashes,
   no wrong codes, fuzzed inputs bounded.
3. Oracle equivalence: runnable IEC subset produces identical IO traces
   vs matiec reference on the shared vector suite (doc 14).
4. Prohibited constructs each have a dedicated test asserting the
   reasoned error (not a generic reject).
5. `tplc-mir` output for the reference corpus is stable (golden tests).
6. Strict-conversion matrix: the full IEC implicit-conversion table is
   tested pairwise.

---

## Appendix A — ST grammar (EBNF, normative)

Notation: `=` production, `|` alternation, `[x]` optional, `{x}`
repetition, `'x'` terminal, `(* *)` comment. Terminals are
case-insensitive keywords unless quoted otherwise. Whitespace and IEC
comments `(* ... *)` (non-nesting v1; nested-block comments rejected
with `_ST_LEX` — documented deviation; IEC allows them, most files
don't need them) are skipped between tokens. `//` line comments are a
`{tether}` extension: accepted, flagged by lint `st-ext-comment`.

### A.1 Compilation unit

```
library      = { declaration | global_var_block } ;
declaration  = pou_decl | type_decl | config_decl ;
config_decl  = (* reserved — task binding is project-side, §2.3 *)
```

### A.2 POU declarations

```
pou_decl     = program_decl | fb_decl | function_decl ;

program_decl = 'PROGRAM' ident
                 { var_block }
                 statement_list
               'END_PROGRAM' ;

fb_decl      = 'FUNCTION_BLOCK' ident
                 { var_block }
                 statement_list
               'END_FUNCTION_BLOCK' ;

function_decl= 'FUNCTION' ident ':' type_spec
                 { var_block }
                 statement_list
               'END_FUNCTION' ;
```

Inside a `FUNCTION`, the return value is assigned to the function name
(IEC rule): `myFn := expr;` — sema rewrites to a return-value
assignment; multiple assignments keep last (IEC).

### A.3 Type declarations

```
type_decl    = 'TYPE' type_entry { type_entry } 'END_TYPE' ;
type_entry   = ident ':' type_spec ';' ;
type_spec    = elementary_type
             | array_spec
             | struct_spec
             | enum_spec
             | subrange_spec
             | ident ;               (* alias to declared type *)

array_spec   = 'ARRAY' '[' subrange { ',' subrange } ']'
               'OF' type_spec ;
subrange     = signed_int '..' signed_int ;

struct_spec  = 'STRUCT' { member_decl } 'END_STRUCT' ;
member_decl  = ident ':' type_spec [ init ] ';' ;

enum_spec    = '(' enum_item { ',' enum_item } ')' ;
enum_item    = ident [ ':=' signed_int ] ;

subrange_spec= elementary_int_type '(' signed_int '..' signed_int ')' ;

elementary_int_type = 'SINT'|'INT'|'DINT'|'LINT'
                    |'USINT'|'UINT'|'UDINT'|'ULINT'
                    |'BYTE'|'WORD'|'DWORD'|'LWORD' ;
elementary_type = elementary_int_type | 'BOOL'
                | 'REAL' | 'LREAL'
                | 'TIME' | 'DATE' | 'TIME_OF_DAY' | 'TOD'
                | 'DATE_AND_TIME' | 'DT'
                | 'STRING' [ '(' unsigned_int ')' ]
                | 'CHAR' ;
```

### A.4 Variable blocks

```
global_var_block = 'VAR_GLOBAL' [ 'RETAIN' | 'CONSTANT' ]
                     { var_decl } 'END_VAR' ;

var_block    = var_kind { var_decl } 'END_VAR' ;
var_kind     = 'VAR' [ 'RETAIN' | 'CONSTANT' ]
             | 'VAR_INPUT' | 'VAR_OUTPUT' | 'VAR_IN_OUT'
             | 'VAR_TEMP' | 'VAR_EXTERNAL'
             | 'VAR_RETAIN' | 'VAR_CONSTANT' ;

var_decl     = ident_list ':' type_spec [ init ]
               [ at_clause ] ';' ;
ident_list   = ident { ',' ident } ;
init         = ':=' expression ;   (* must be const-evaluable *)

at_clause    = 'AT' direct_address ;
direct_address = '%' region size_opt addr ;
region       = 'I' | 'Q' | 'M' ;
size_opt     = 'X' | 'B' | 'W' | 'D' | 'L' | '*' ;
addr         = unsigned_int [ '.' unsigned_int ] | '*' ;
```

Rules on `AT` (§4/§9): fixed numeric form binds to the image map;
`%I*`/`%Q*`/`%M*` wildcard binds at deploy; `AT` forbidden inside
FUNCTION and VAR_TEMP. `%M` allowed for imported-code compat.

### A.5 Statements

```
statement_list = { statement ';' } ;
statement  = assign_stmt
           | call_stmt
           | if_stmt | case_stmt
           | for_stmt | while_stmt | repeat_stmt
           | exit_stmt | continue_stmt | return_stmt
           | (* empty *) ;

assign_stmt  = lvalue ':=' expression ;
lvalue       = [ ident '.' ] var_path ;
var_path     = ident { '[' expr_list ']' | '.' ident } ;
expr_list    = expression { ',' expression } ;

call_stmt    = fb_call | std_call ;
fb_call      = var_path '(' [ arg_list ] ')' ;
std_call     = ident '(' [ arg_list ] ')' ;   (* resolved to function *)
arg_list     = arg { ',' arg } ;
arg          = expression                        (* positional *)
             | ident ':=' expression             (* named input *)
             | ident '=>' var_path ;             (* output capture *)

if_stmt      = 'IF' expression 'THEN' statement_list
               { 'ELSIF' expression 'THEN' statement_list }
               [ 'ELSE' statement_list ]
               'END_IF' ;

case_stmt    = 'CASE' expression 'OF'
                 { case_sel ':' statement_list }
               [ 'ELSE' statement_list ]
               'END_CASE' ;
case_sel     = case_val { ',' case_val } ;
case_val     = signed_int | ident ;   (* enum member or literal *)

for_stmt     = 'FOR' ident ':=' expression 'TO' expression
               [ 'BY' expression ] 'DO'
               statement_list 'END_FOR' ;

while_stmt   = 'WHILE' expression 'DO' statement_list 'END_WHILE' ;
repeat_stmt  = 'REPEAT' statement_list
               'UNTIL' expression 'END_REPEAT' ;

exit_stmt    = 'EXIT' ;
continue_stmt= 'CONTINUE' ;
return_stmt  = 'RETURN' ;
```

### A.6 Expressions

```
expression   = or_expr ;
or_expr      = xor_expr { 'OR' xor_expr | 'OR_ELSE' xor_expr } ;
xor_expr     = and_expr { 'XOR' and_expr } ;
and_expr     = cmp_expr { 'AND' cmp_expr | 'AND_THEN' cmp_expr } ;
cmp_expr     = add_expr [ cmp_op add_expr ] ;   (* non-assoc *)
cmp_op       = '=' | '<>' | '<' | '<=' | '>' | '>=' ;
add_expr     = mul_expr { ('+'|'-') mul_expr } ;
mul_expr     = unary { ('*'|'/'|'MOD') unary } ;
unary        = [ '-' | '+' | 'NOT' ] unary | power ;
power        = primary [ '**' unary ] ;          (* right-assoc *)
primary      = literal
             | var_path
             | '(' expression ')'
             | fb_result_ref
             | func_call ;

fb_result_ref= var_path '.' ident ;   (* post-call output access;
                                         merged into var_path in AST *)
func_call    = ident '(' [ arg_list ] ')' ;

literal      = numeric | bool_lit | time_lit | char_lit | string_lit ;
bool_lit     = 'TRUE' | 'FALSE' ;
numeric      = [ type_name '#' ] unsigned_num
             | [ type_name '#' ] based_num
             | signed_num ;
based_num    = ('2'|'8'|'16') '#' digit_seq ;
unsigned_num = digit_seq [ '.' digit_seq ] [ exp ] ;
exp          = ('E'|'e') ['+'|'-'] digit_seq ;
signed_num   = ['+'|'-'] numeric ;
digit_seq    = digit { ['_'] digit } ;
time_lit     = ('T'|'TIME') '#' time_spec
             | ('D'|'DATE') '#' date_spec
             | ('TOD'|'TIME_OF_DAY') '#' tod_spec
             | ('DT'|'DATE_AND_TIME') '#' dt_spec ;
time_spec    = [ '-' ] time_comp { time_comp } ;
time_comp    = unsigned_num ('d'|'h'|'m'|'s'|'ms'|'us'|'ns') ;
string_lit   = "'" { char_seq | '$' esc } "'" ;
char_lit     = 'CHAR' '#' "'" char "'" ;
esc          = '$' | "'" | 'L' | 'N' | 'P' | 'R' | 'T'
             | hex hex ;
```

### A.7 Reserved words (v1)

```
ACTION AND AND_THEN ARRAY AT BOOL BY BYTE CASE CHAR CONFIGURATION
CONSTANT CONTINUE D DATE DATE_AND_TIME DINT DO DT DWORD ELSE ELSIF
EN ENO END_ACTION END_CASE END_FOR END_FUNCTION END_FUNCTION_BLOCK
END_IF END_PROGRAM END_REPEAT END_STRUCT END_TYPE END_VAR END_WHILE
EXIT FALSE FOR FUNCTION FUNCTION_BLOCK IF IN INT L LINT LREAL LTIME
LWORD MOD NOT OF OR OR_ELSE PROGRAM REAL REPEAT RESOURCE RETAIN
RETURN ROL ROR SHL SHR SINT STEP STRING STRUCT TASK THEN TIME
TIME_OF_DAY TO TOD TRANSITION TRUE TYPE UDINT UINT ULINT UNTIL USINT
VAR VAR_ACCESS VAR_CONFIG VAR_EXTERNAL VAR_GLOBAL VAR_IN_OUT
VAR_INPUT VAR_OUTPUT VAR_RETAIN VAR_STAT VAR_TEMP W WHILE WORD
WSTRING WCHAR XOR
```

Reserved-but-not-yet-meaningful (parse → `_ST_RESERVED` with message):
`ACTION END_ACTION STEP TRANSITION CONFIGURATION RESOURCE TASK
VAR_ACCESS VAR_CONFIG VAR_STAT WSTRING WCHAR LTIME`.

## Appendix B — Implicit conversion matrix (normative)

Rows = source, columns = destination. `✓` implicit allowed,
`C` explicit cast required, `—` never (compile error). This matrix is
the conformance target; the oracle suite tests it pairwise.

| from\to | BOOL | SINT | INT | DINT | LINT | USINT | UINT | UDINT | ULINT | BYTE | WORD | DWORD | LWORD | REAL | LREAL | TIME | STRING |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| BOOL  | ✓ | — | — | — | — | — | — | — | — | C | C | C | C | — | — | — | C |
| SINT  | — | ✓ | ✓ | ✓ | ✓ | C | C | C | C | — | — | — | — | ✓ | ✓ | — | C |
| INT   | — | C | ✓ | ✓ | ✓ | C | C | C | C | — | — | — | — | ✓ | ✓ | — | C |
| DINT  | — | C | C | ✓ | ✓ | C | C | C | C | — | — | — | — | ✓ | ✓ | — | C |
| LINT  | — | C | C | C | ✓ | C | C | C | C | — | — | — | — | C | ✓ | — | C |
| USINT | — | C | C | C | C | ✓ | ✓ | ✓ | ✓ | — | — | — | — | ✓ | ✓ | — | C |
| UINT  | — | C | C | C | C | C | ✓ | ✓ | ✓ | — | — | — | — | ✓ | ✓ | — | C |
| UDINT | — | C | C | C | C | C | C | ✓ | ✓ | — | — | — | — | C | ✓ | — | C |
| ULINT | — | C | C | C | C | C | C | C | ✓ | — | — | — | — | C | ✓ | — | C |
| BYTE  | C | — | — | — | — | — | — | — | — | ✓ | ✓ | ✓ | ✓ | — | — | — | C |
| WORD  | C | — | — | — | — | — | — | — | — | C | ✓ | ✓ | ✓ | — | — | — | C |
| DWORD | C | — | — | — | — | — | — | — | — | C | C | ✓ | ✓ | — | — | — | C |
| LWORD | C | — | — | — | — | — | — | — | — | C | C | C | ✓ | — | — | — | C |
| REAL  | — | C | C | C | C | C | C | C | C | — | — | — | — | ✓ | ✓ | — | C |
| LREAL | — | C | C | C | C | C | C | C | C | — | — | — | — | C | ✓ | — | C |
| TIME  | — | — | — | — | — | — | — | — | — | — | — | — | — | — | — | ✓ | C |
| STRING| C | C | C | C | C | C | C | C | C | — | — | — | — | C | C | C | ✓ |

- Widening = value-preserving direction; all `C` = `TO_*`/`X_TO_Y()`
  stdlib calls (checked-mode range applies on narrowing).
- `CHAR`/`WCHAR` behave as `STRING(1)`-adjacent for conversions.
- `DATE`/`TOD`/`DT` convert only `↔ STRING` and `↔ TIME-family` via
  stdlib; arithmetic via stdlib `ADD`/`SUB` overloads, never `+`.
- Subrange types convert as their base type + bounds check (§11).
