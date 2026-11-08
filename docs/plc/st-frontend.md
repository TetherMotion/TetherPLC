# ST frontend — as-built

Status: as-built for `compiler/tplc-st`.

## Shape

- `tplc-st` = ruSTy AST adapter (`rusty_lower.rs`, behind the `rusty`
  cargo feature) + MIR semantic pass (`sema.rs`, always built, pure
  `tplc-mir` — FBD lowering lands here too) + product diagnostics
  (`diag.rs`).
- `compile(files) -> FrontendOutput { module, io_bindings, diags }`;
  `sema::check(&mut module) -> Vec<Diag>` is a separate step so other
  frontends can run the same pass.
- Unnamed `VAR_GLOBAL` blocks merge into the `GLOBAL` GVL; access is
  `GLOBAL.x` or via `VAR_EXTERNAL` declarations.

## Sema rules (st-frontend.md §5–§9)

- Name resolution canonicalizes case in place (`x`/`X` same var).
- Enum elements resolve to ordinals when no var shadows the name.
- Untyped literals (`1`, `1.5`) have no declared width: they adopt the
  other operand's type in arithmetic and assign to any numeric/time/
  enum target (folded by codegen). Declared `DINT` values do NOT get
  this treatment — `INT := DINT` inserts a `Conv` and lints
  `implicit-narrow`.
- Legal implicit conversions become `ExprKind::Conv` nodes; codegen
  never guesses. Incompatible pairs error `TPLC_E_ST_TYPE` with
  `expected …, found …`.
- Conditions (`IF`/`WHILE`/`REPEAT`) must be `BOOL` — strict, no
  permissive `Int→Bool` here.
- `STRING`: comparisons are `=`/`<>` only — an ordered compare (or a
  STRING↔non-STRING compare) errors `TPLC_E_ST_TYPE`. Literals assign
  to any target cap (zero-padded; longer literals truncate with the
  usual `implicit-narrow` lint).
- `WSTRING`: a distinct UTF-16 type — blob layout is `u16 len` +
  `u16[cap]` code units (surrogate pairs preserved), wide literals
  (`"…"`) carry `is_wide`, and narrow↔wide assignment is a type error
  rather than a silent re-encode. Same `=`/`<>`-only comparison rule.
- `FUNCTION` inputs without defaults are required; missing bindings
  error `TPLC_E_ST_TYPE`. FB-call args are checked against the FB
  iface (or builtin member layout); all FB inputs optional.

## Diagnostics (st-frontend.md §12)

Every diagnostic: `{code, severity, message, [help], [notes], span}`.
`Diag::render(file, src)` produces rustc-style output: `file:line:col`
header, source line, caret span, `= help`/`= note` lines. `Prov::Fbd`
renders `fbd sheet S node N pin P`.

Error codes (`diag::codes`, stable API):

| code | fires on |
|---|---|
| `TPLC_E_ST_PARSE` | ruSTy parse diagnostics (adapter boundary) |
| `TPLC_E_ST_UNSUPPORTED` | unsupported-but-parsed constructs |
| `TPLC_E_ST_UNKNOWN_TYPE` | unresolvable type names |
| `TPLC_E_ST_UNKNOWN_NAME` | undeclared names/members (with levenshtein did-you-mean) |
| `TPLC_E_ST_TYPE` | type mismatch, missing required args, non-Bool condition |
| `TPLC_E_ST_BAD_AT` | malformed/width-mismatched `AT` addresses |
| `TPLC_E_ST_DUP_NAME` | duplicate declarations |

Warning codes (`diag::wcodes`, also stable API):

| code | fires on |
|---|---|
| `TPLC_W_ST_UNUSED_VAR` | `VAR` never resolved (AT-bound exempt) |
| `TPLC_W_ST_SHADOW` | local name shadows a GVL name |
| `TPLC_W_ST_UNREACHABLE_CASE` | case selector overlaps an earlier arm |
| `TPLC_W_ST_DEAD_CODE` | statement after `RETURN`/`EXIT`/`CONTINUE` |
| `TPLC_W_ST_IMPLICIT_NARROW` | implicit conversion loses range/precision |
| `TPLC_W_ST_EXTERNAL` | `VAR_EXTERNAL` decl / implicit GVL access |
| `TPLC_W_ST_NO_JUMPS` | `EXIT`/`CONTINUE` loop jumps |

## Not yet implemented

- Parse-error recovery to declaration boundaries is whatever ruSTy
  gives us; no extra recovery layer.
- §10 prohibition table (pointers, recursion, …) — rejected today only
  as `UNSUPPORTED` when ruSTy parses them; no dedicated reason strings.
- Builtin scalar functions (`ABS`/`MIN`/`SEL`, string ops) — sema
  knows FUNCTION POUs only.
- `VAR_EXTERNAL` semantics beyond the lint: the decl links to GVL vars
  by name but is not yet a checked binding.
- Multi-line span rendering (a span crossing lines renders only the
  first line today).
