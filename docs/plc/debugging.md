# Debug & observability plane (as-built)

The live debug surface on `ctl.sock` — symbol resolution, read/
write/force, boundary-consistent watch, task suspend/step,
breakpoints with trigger expressions, flight-recorder trace, and
retain inspection. Every mutating op is role-gated and journaled
(authority model in `management.md`/plc-runtime.md). All ops are
also proxied over tether_io by `tplc-io` (tether-io-attach.md).

## Op surface

| op | code | payload → reply |
|---|---|---|
| `DBG_RESOLVE` | 0x0501 | `{u16 nlen,name}` → region/offset/len/type |
| `DBG_READ` | 0x0502 | `{u64 off,u32 len}` → bytes (boundary-sampled) |
| `DBG_WRITE` | 0x0503 | `{u64 off,u32 len,bytes}` → ∅ |
| `DBG_STATE` | 0x0504 | ∅ → blob/retain extents + stats |
| `DBG_FIELDS` | 0x0518 | ∅ → symmap field table (symbol→blob map) |
| `DBG_FORCE` | 0x0505 | `{u64 off,u32 len,bytes}` → `{u32 handle}` |
| `DBG_FORCE_LIST` | 0x0506 | ∅ → `{u32 n, entries}` |
| `DBG_FORCE_CLEAR` | 0x0507 | `{u32 handle}` / `0xFFFFFFFF` = all |
| `TASK_SUSPEND/RESUME` | 0x0402/3 | `{u32 task_idx}` → ∅ |
| `TASK_STEP` | 0x0404 | `{u32 task_idx,u32 n}` → ∅ |
| `DBG_WATCH_ADD/GET/REMOVE/STREAM/LIST` | 0x0510–0x0514 | named/resolved items; GET samples at one boundary |
| `DBG_BREAK` | 0x0508 | `{task,off,len,cmp,val}` → `{u32 handle}` |
| `DBG_BREAK_EXPR` | 0x0515 | `{task, expr-string[, capture]}` → `{u32 handle}` |
| `DBG_BREAK_LIST/CLEAR` | 0x0509/0A | list; `{u32 handle}` / `0xFF..` = all |
| `DBG_CAPTURE/_EXPR` | 0x050F/16 | one-shot multi-channel boundary capture |
| `TRACE_CONF` | 0x050B | `{u32 n, u32 pad, {u64 off,u32 len,u32 pad}*}` |
| `TRACE_STATUS/READ/CLEAR` | 0x050C–E | flight-recorder ring status/read/clear |
| `DBG_STATE_DIFF` | 0x0517 | ∅ → last swap's migrate-rule walk |
| `RETAIN_STATUS/FLUSH/CLEAR/DUMP` | 0x0601–4 | retain store ops |

Error codes: `DBG_RESOLVE` -410, `DBG_STALE` -411 (reserved, not yet
emitted), `DBG_OVERLOAD` -412, `DBG_RATE` -413, `FORCE_DENIED` -414,
`DBG_ACCESS` -415, `DBG_STATE` -416, `TRACE_CONFIG` -417;
`CTL_AUTHZ` -401 for role denials.

## Semantics

- **Boundary consistency.** Reads, watch samples, and captures are
  taken at task boundaries; a 64-bit field written by a 4 kHz task
  is never observed torn (`debug_stress` hammers this).
- **Forces** are overlay writes applied per cycle, handle-scoped,
  overlapping ranges rejected (-415 on overlap; -414 when the
  resolver denies the range, e.g. safety-bound channels). Forces
  are dropped on program stop — a hidden force surviving a stop is
  a hazard.
- **Suspend/step** is per-task: suspended tasks skip release;
  `step n` runs n releases then re-suspends.
- **Breakpoints**: value-compared or expression triggers evaluated
  post-force at the boundary; a hit suspends the task and emits an
  event.
- **Trace** is a fixed-capacity ring sampled per cycle on the
  configured channels — the flight recorder for pre-fault history.

## Post-swap resolution

Subscription items and expression triggers keep their *source text*;
on a program-set change they re-resolve through the new symmap.
Renamed/aliased vars continue; vanished vars report `stale` (the
`DBG_STALE` code is reserved for this path — currently surfaced via
watch entries, not yet emitted as an error).

## Trigger-expression language (`dbg.expr.v1`)

Bounded stack machine compiled at op time, evaluated at task
boundaries. No calls, no loops, no allocation, max 64 ops.

```
expr    = or
or      = and { '||' and }
and     = cmp { '&&' cmp }
cmp     = add [ ('=='|'!='|'<'|'<='|'>'|'>=') add ]
add     = mul { ('+'|'-') mul }
mul     = unary { ('*'|'/'|'%') unary }
unary   = [ '!' | '-' ] unary | primary
primary = path | literal | '(' expr ')' | 'changed' '(' path ')'
        | 'rising' '(' path ')' | 'falling' '(' path ')'
path    = qualified symmap path (e.g. "PROG.Ton.Q", "GVL.x[3].v")
literal = int | float | 'true' | 'false'
```

- Paths resolve at config time via the same resolver as
  `DBG_RESOLVE`; stale after swap → trigger deactivates (re-resolve
  from source text on next epoch).
- `changed(p)`: value differs from the previous boundary sample;
  `rising`/`falling`: BOOL edge in consecutive samples. The
  evaluator keeps one previous value per path.
- Numeric widening only; BOOL is not numeric; comparisons need
  matching families — compile-checked, errors name the term.

## Determinism under debug load

Debug ops run on the ctl thread between scan ticks at task
boundaries; a ctl flood sheds (ops queue, tasks never slip past the
bound). `debug_stress` gates cadence ≥ 50% nominal and
`max_late` < 5 ms under a 6-way flood; `tests/perf/evidence.sh`
records cadence vs DESIGN-GOALS §6.
