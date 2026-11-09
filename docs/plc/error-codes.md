# Error-code registry

Every `TPLC_E_*` code is stable API: never renumbered once shipped;
new codes append within their owner's range. The authoritative
definitions live in the headers/sources listed below — this document
is the index and the human-readable layer (there is no `tplcc
explain`; diagnostics render their message strings directly).

Coverage status (which codes are emitted and test-exercised):
`error-code-coverage.md` — regenerate with
`tools/error_code_report.py --gaps`.

## Numeric codes (wire / ABI domain)

| Range | Owner | Prefix | Meaning |
|---|---|---|---|
| `-1..-4` | `include/tplc/tplc_abi.h` | `TPLC_E_INIT_*` | `tplc_init` failures: bad layout, invalid descriptor, bad initial value, resource exhaustion |
| `-16..-23` | `tplc_abi.h` | `TPLC_TASK_E_*` | task-cycle faults raised by *generated code*: fault, panic (`-17`, null `REF` deref), bounds (`-18`, `REF` offset past the blob), div-zero, overflow, range, aborted, overrun |
| `-32..-35` | `tplc_abi.h` | `TPLC_E_MIGRATE_*` | migrate failures: layout mismatch, denied rule, invalid descriptor, blob out of range |
| `-64/-65` | `tplc_abi.h` | `TPLC_E_ABI_*` | ABI version mismatch / extension unmet |
| `-100..-107` | `include/tplc/tplc_image.h` | `TPLC_E_IMG_*` | SHM image: magic, version, size, seq exhaustion, epoch, range, I/O, state |
| `-200..-215` | `compiler/tplc-pkg/src/error.rs` + `runtime/pkg/` | `TPLC_E_PKG_*`, `TPLC_E_SIG_*` | package/signature validation: magic, version, TOC, header/content hash, unknown section, limit, sig required/invalid/untrusted, hidden state, symmap range, target/ABI mismatch, requires unmet, section format |
| `-300..-304` | `runtime/host/include/tplc/host/Status.hpp` | `TPLC_E_HOST_*` | module loading: dlopen, missing symbol, descriptor validation, range, lifecycle state |
| `-400..-404` | `include/tplc/tplc_ctl.h` | `TPLC_E_CTL_*` | ctl plumbing: wrong state, authz denied (`-401`), unknown op, protocol version, malformed payload |
| `-410..-417` | `tplc_ctl.h` | `TPLC_E_DBG_*`, `TPLC_E_FORCE_DENIED`, `TPLC_E_TRACE_CONFIG` | debug plane: resolve, stale, overload, rate, force denied (`-414`), access (`-415`), state, trace config |
| `-500..-503` | `tplc_ctl.h` | `TPLC_E_MGMT_*` | management surface: illegal transition (`-501`), unknown object (`-500`), object-level authz (`-502`), object schema (`-503`) |
| `-600..-609` | `runtime/bridge/include/tplc/bridge/MapRunner.hpp` | `TPLC_E_MAP_*` | io-map validation: owner, dir, range, type, PDO, safety, AT, bind, safe, epoch — see note below |
| `-999..-1999` | `tplc_abi.h` | module-defined | reserved for generated/module codes |

> **Range note.** `TPLC_E_MAP_*` lives at `-600..-609`, disjoint from
> wire-side `TPLC_E_MGMT_*` (`-500..-503`): MAP codes are returned
> inside map-validation reports (`MAP_GET`/deploy-map checks) and
> never appear as a ctl wire error. (They originally aliased the
> `-500` block; split for clarity.)

## String codes (compiler / diagnostic domain)

Compiler diagnostics carry string codes, not numbers:

| Owner | Codes |
|---|---|
| `compiler/tplc-st/` | `TPLC_E_ST_{PARSE,TYPE,UNKNOWN_NAME,UNKNOWN_TYPE,DUP_NAME,BAD_AT,UNSUPPORTED}` |
| `compiler/fbd-model`, `fbd-lower`, `plcopen-io` | `FBD001..FBD014` validation rules; `TPLC_E_FBD_{PARSE,BAD_AT,UNSUPPORTED}`; `TPLC_E_MODEL_SCHEMA` |
| `compiler/tplcc/` driver | `TPLC_E_DRIVER`; sim compare: `TPLC_E_SIM_DIVERGENCE` |

## Events and result enums (not error codes)

Non-`_E_` forms are events, states or result enums — they must not
be treated as failures:

| Family | Owner | Members |
|---|---|---|
| `TPLC_MIGRATE_*` | `tplc_abi.h` | `OK`, `OK_DEGRADED`, `DENY`, `ERR_DESC`, `ERR_BLOB_RANGE` — migrate result enum |
| `TPLC_IMG_*` states | `tplc_image.h` | `MASTER_{INIT,RUN,DEGRADED,STOPPING}`, `PLC_{OPEN,RUN,FAULT,STOPPED,SAFE}` — peer state values |
| `TPLC_IMG_*` alarms | `tplc_image.h` | `PLC_LOST` (1), `MASTER_LOST` (2) — loss-of-peer event codes |
| `TPLC_EV_*` | `tplc_ctl.h` | `PROG_TRANSITION`, `FAULT`, `DBG_BREAK`, `DBG_WATCH` — journal/notify event op codes |

## Rules

1. A code may be *raised* anywhere but is *defined* only in its
   owner's range.
2. UI-facing messages come from the `Display`/message strings next
   to each definition and from this registry — never hard-coded per
   call site.
3. Codes are append-only; removing a shipped code's meaning is an
   ABI break.
