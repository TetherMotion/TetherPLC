# Toolchain — `tplcc`, the Package Pipeline, and the Ctl Proxy

The compiler driver, the `.tplc` artifact pipeline, and the HTTP bridge
that lets a browser reach the ctl channel and the compiler.

## 1. `tplcc` — the compiler driver

One binary, five subcommands:

| command | role |
|---|---|
| `tplcc check <src>…` | parse + sema only; human diags to stderr |
| `tplcc build <src>… -o p.tplc` | full pipeline → `.tplc` package |
| `tplcc info <p.tplc>` | dump package header + section table |
| `tplcc serve <uds>` | persistent check service (see §4) |
| `tplcc vectors` | conformance-vector runner (oracle harness) |

`check` and `build` take `--json`: instead of the human renderer they
emit one JSON line `{ok, errors, diags:[…]}` on stdout (build adds
`"package"` on success). St spans carry `file/offset/len/line/col`;
Fbd spans carry `sheet/node/pin`; non-frontend failures (LLVM verify,
link, change-deny) are normalized into a uniform `TPLC_E_DRIVER` diag
so the envelope stays parseable. This is the contract the web IDE and
the proxy consume.

### Frontend routing

`compile_srcs` picks a frontend by *file name*, not flag: a lone
`.fbd.json` source goes through `fbd-model` → `fbd-lower` → MIR;
anything else goes through `tplc-st` (parse → sema → MIR). Both end at
the same `tplc-mir::ir::Module`, so codegen/ABI/package don't care
which language the user wrote. A mixed-language project is a frontend
error surfaced as a normal diag.

### Pipeline (build)

```
ST / fbd.json ──► MIR module + diags ──► desc (program descriptor)
   │                                          │
   │                                    layout (state blob offsets)
   │                                          │
   │   --change-against old.tplc:  tplc_migrate (rules + deny verdict)
   │                                          │
   ▼                                          ▼
 emit_program → LLVM module → verify → emit_bitcode (.bc)
                                   → emit_object → link → .so
                                                   │
                            PackageWriter: HDR + sections → p.tplc
```

Sections (see `artifact-format.md`): `DESC` (program descriptor blob),
`LAYD` (layout descriptor), `SYMMAP` (name→offset), `MIGD` (only with
`--change-against`), `BC` (bitcode — the canonical, target-independent
form), `SO` (the native ELF shared object — the deployable form),
`META` (uuid = SHA-256 of sources, hashes, flags).

## 2. Why `.tplc` carries both bitcode and native code

The `SO` is what `tether-plc` `dlopen`s for zero-cost native scans.
The `BC` is what makes the artifact retargetable: `tether-plc-sim`
loads it for host-side deterministic simulation, and the future MCU
path re-emits object code for the target triple from the same bitcode —
the package is the interchange, not the ELF.

## 3. `tplc-ctl-proxy` — HTTP → UDS bridge

Browsers can't reach `AF_UNIX`. The proxy is a small single-threaded
HTTP/1.1 server (`tools/tplc_ctl_proxy.cpp`, no deps) exposing:

| route | method | what it does |
|---|---|---|
| `/ctl` | POST | one verbatim ctl frame round-trip (auto-issues the session `HELLO` per UDS connection) |
| `/compile` | POST | binary src frame → `tplcc check|build --json`; wraps output as `{compile, exit, package?b64}` |
| `/events` | GET | SSE stream of ctl `F_EVENT` frames (see `web-ide.md` §events) |
| `/health` | GET | `ok` iff the ctl UDS connects |
| any | OPTIONS | CORS preflight (`Access-Control-Allow-Origin: *`) |

The src frame is `[u16 count][u16 nameLen,name,u32 len,content]×N` LE;
names are basename-sanitized, ≤64 files × 4 MB. Build responses include
the `.tplc` as base64.

Flags: `--socket` (ctl UDS, default `/tmp/tplc-image.ctl`), `--port`,
`--bind`, `--tplcc` (compiler path for the popen path), `--tplccd`
(daemon UDS, default `/tmp/tplccd.sock`).

## 4. `tplcc serve` — the compile daemon

`/compile?build` needs files on disk (package emit), so it always goes
through `popen`. But `check` — the hot path for IDE diagnostics — does
not: `tplcc serve` accepts the same binary src frame on a Unix socket,
runs `compile_srcs` + `sema::check` in-process, and replies with the
identical `{compile, exit}` envelope. The proxy forwards check bodies
to the daemon verbatim and falls back to popen when the daemon is
absent, so a missing `tplccd` is invisible to clients. This removes the
process-spawn + dynamic-link cost per debounced keystroke; incremental
AST caching is a deliberate non-goal for v1.

## 5. End-to-end deploy flow

```
IDE sources ──/compile?build──► {compile,package:b64}
                                 │
            PlcCtl.deploy: SHA-256 → DEPLOY_BEGIN → CHUNK×N → COMMIT
                                 │
              tether-plc stores pkg → PROG_PREPARE (dry-run MIGD)
                                 │
              PROG_SWAP (quiesce→migrate→swap) or PROG_START
                                 │
              EV_PROG_TRANSITION ──/events──► IDE status refresh
```
