# Toolchain — `tplc`, `tplcc`, and the Package Pipeline

`tplc` is the unified user-facing entrypoint (Rust, LLVM-free): it
fronts the compiler, runtime, simulation, I/O service and web backend
as one application — see §0. `tplcc` is the compiler driver, the
`.tplc` artifact pipeline, and the HTTP bridge (now implemented inside
`tplc serve`) that lets a browser reach the ctl channel and the
compiler.

## 0. `tplc` — the unified entrypoint

`compiler/tplc` builds a single `tplc` binary (deps: `sha2`,
`serde_json`, `libc` only — no LLVM). It replaces the manual
multi-binary orchestration (`tplc-image create` → `tplcc build` →
`tether-plc` → `tplc-io` → `tplc serve`) with managed workflows:

| command | what it does |
|---|---|
| `tplc dev [srcs…\|pkg.tplc]` | full local stack: compile-if-needed → shm image → `tether-plc` → `tplc-io` → HTTP server + WebUI static files. Reuses a live runtime when the ctl socket already answers |
| `tplc run [srcs…\|pkg]` | compile-if-needed → image → `tether-plc` in foreground; SIGINT/SIGTERM stops the child |
| `tplc sim [srcs…\|pkg] --steps F` | deterministic replay via `tether-plc-sim` (`--shadow`, `--dump-q`, `--settle-ms`, `--hold-ms`, `--scan`) |
| `tplc serve` | the HTTP/WebSocket backend (§3) + optional WebUI static root; can also spawn `tplcc serve` and `tplc-io` |
| `tplc ctl [--socket S\|--session N] <op>` | native UDS ctl client — full ctl op surface, no subprocess |
| `tplc down [name]` / `tplc sessions` | session lifecycle: `RT_SHUTDOWN` + child cleanup, list live stacks |
| `tplc doctor` | resolve every sibling tool (bindir → exe-adjacent → repo layout → PATH) |
| `tplc cc <args>` | `tplcc` passthrough (incl. `serve` daemon, `sign`, `pkg`) |
| `tplc image\|bridge\|io\|pump <args>` | runtime tool passthrough |
| `tplc check\|build\|info\|…` | `tplcc` subcommand passthrough |

Sessions are recorded under `${TPLC_STATE_DIR:-${XDG_RUNTIME_DIR:-/tmp}/tplc}/<name>.json`
(name, pids, ctl socket, image, ports) so `tplc ctl --session`,
`tplc down` and `tplc serve` find a running stack without memorized
paths; `--name` (default `main`) keys everything. Tool resolution
order: `--bindir`/`TPLC_BINDIR` → exe-adjacent → repo layout
(`build/`, `compiler/target/`) → `PATH`.

`tplc dev`/`run` pass extra args after `--` verbatim to `tether-plc`,
so supervisor mode (`--supervise --respawn …`), retain policies,
signature/trust options and `--stopped`/`--sim` all remain available —
the runtime's own sandbox (rlimits/seccomp) and `tplc-mcgroup`
coordinated-motion service apply unchanged.

## 1. `tplcc` — the compiler driver

One binary, five subcommands:

| command | role |
|---|---|
| `tplcc check <src>…` | parse + sema only; human diags to stderr |
| `tplcc build <src>… -o p.tplc` | full pipeline → `.tplc` package |
| `tplcc info <p.tplc>` | dump package header + section table |
| `tplcc serve <uds>` | persistent check service (see §4) |
| `tplcc vectors` | conformance-vector runner (oracle harness) |
| `tplcc sign keygen --dir D --name N` | ed25519 keypair → `N.key` + `<keyid>.pub` |
| `tplcc sign sign <in.tplc> --key K [-o out]` | attach/replace signature section |
| `tplcc sign verify <pkg> [--trust-dir D]` | verify sig (+ trust-store lookup) |

`check` and `build` take `--json`: instead of the human renderer they
emit one JSON line `{ok, errors, diags:[…]}` on stdout (build adds
`"package"` on success). `build` also takes `--task-period-ns <ns>` to
override the cyclic MAIN task period (default 10 ms) and
`--checked-arith` to make integer div/mod by zero a
`TPLC_TASK_E_DIVZERO` (-19) task fault, integer add/sub/mul/neg
that leaves the type's range or signed `INT_MIN / -1` a
`TPLC_TASK_E_OVERFLOW` (-20) fault, and out-of-range conversions
(narrowing int, real→int) a `TPLC_TASK_E_RANGE` (-21) fault,
instead of the default saturating/wrapping/truncating; the choice
is baked into the generated code and recorded as
`"checked_arith":true` in the package manifest.
`build` also takes `--target <triple>` for foreign-target objects
and `--pic-blob` (foreign targets only) to store a TPLB loadable
image instead of the raw ET_REL — see `mcu-path.md`.
`build --library` packages a FUNCTION/FUNCTION_BLOCK collection
with no PROGRAM, VAR_GLOBAL, or TASK declarations into a
`"kind":"library"` package: manifest + bitcode + the relocatable
object (POU `__tplc_pou_*` symbols stay linkable) + ST sources +
toolchain, but no program-state sections. `build --lib <pkg.tplc>`
(pull the package's ST_SOURCES section into the compilation, for
consuming library packages) and `--name <n>` (library manifest
name; default is the output file stem) round out the compat-library
flow — see `plan/21-twincat-libraries.md` §8 C0 and
`compat/README.md` for the compat layer status;
`compat/twincat/INDEX.md` indexes the per-library catalog.
St spans carry
`file/offset/len/line/col`;
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

## 3. `tplc serve` — `/ws` backend + static WebUI

Browsers can't reach `AF_UNIX`, and the whole UI channel is one
WebSocket: `GET /ws` upgrades to a per-client ctl UDS connection
(`HELLO` + `SET_NOTIFY` all-events once at connect). Two message kinds:

- **binary** — a verbatim ctl frame `[u16 type][u16 flags][u32 len]
  [u64 req][payload]`; replies correlate by `req`, unsolicited
  `F_EVENT` frames arrive as pushes (this replaces the old `/ctl`
  POST round-trips and the `/events` SSE stream).
- **text** — a JSON meta-op, answered by a JSON text message:

  | op | fields | `data` |
  |---|---|---|
  | `compile` | `build:bool`, `files:[{name,content}]` | `{compile, exit, package?b64}` |
  | `blocks` | `refresh?:bool` | merged `tplcc blocks --json` over every `--libdir` (cached) |
  | `libsrc` | `lib:<name\|index>`, `file:<name>` | `{text}` — direct `.st` or extracted from a `.tplc` |
  | `pkgmap` | `package:<b64>`, `map:<text>` | `{package:<b64>}` — `tplcc pkg add-section` injects the deploy map |

  Request `{id, op, …}` → reply `{id, ok:true, data}` or
  `{id, ok:false, error}`.

Remaining HTTP surface: `/health` (liveness; body `ok`/`down` for ctl
reachability), `/tether-io` (WebSocket → `tplc-io` TCP bridge,
SLIP-framed), `OPTIONS` CORS preflight, and static files from
`--webui` (SPA fallback to `index.html`). The old `/ctl`, `/compile`,
`/events`, `/blocks`, `/libsrc`, `/pkgmap` routes are gone.

The ctl backend is best-effort: meta-ops work without a runtime, and
ctl frames sent while no runtime is attached get a synthesized
`F_ERROR` reply ("ctl backend down") instead of hanging.

Flags: `--ctl` (ctl UDS, default: live session else
`/tmp/tplc-image.ctl`), `--port`, `--bind`, `--tplcc` (compiler path),
`--tplccd` (daemon UDS; `""` disables), `--libdir <dir>` (repeatable;
each dir's `.st` files and `.tplc` packages feed `blocks`/`libsrc` —
point it at the installed `libraries/` tree), `--webui DIR` /
`--no-webui`, `--tplc-io host:port` + `--io`/`--io-port`/`--packages`/
`--pkg` to spawn `tplc-io`.

`tests/serve/e2e.sh` exercises the whole surface with a minimal
Python WebSocket client (compile envelope, catalog, libsrc traversal
rejection, degraded ctl error, driver-failure JSON).

## 4. `tplcc serve` — the compile daemon

`compile` with `build:true` needs files on disk (package emit), so it
always spawns `tplcc build`. But `check` — the hot path for IDE
diagnostics — does not: `tplcc serve` accepts the same binary src
frame on a Unix socket, runs `compile_srcs` + `sema::check`
in-process, and replies with the identical `{compile, exit}` envelope.
`tplc serve` forwards check requests to the daemon when configured
(`--tplccd`) and falls back to spawning `tplcc check` otherwise, so a
missing daemon is invisible to clients. This removes the
process-spawn + dynamic-link cost per debounced keystroke; incremental
AST caching is a deliberate non-goal for v1.

## 5. End-to-end deploy flow

```
IDE sources ──/ws "compile" meta-op──► {compile,package:b64}
                                 │
            PlcCtl.deploy: SHA-256 → DEPLOY_BEGIN → CHUNK×N → COMMIT
                                 │
              tether-plc stores pkg → PROG_PREPARE (dry-run MIGD)
                                 │
              PROG_SWAP (quiesce→migrate→swap) or PROG_START
                                 │
              EV_PROG_TRANSITION ──/ws binary frame──► IDE status refresh
```

## 6. Debian package (`apt install`)

`scripts/make-deb.sh` stages a self-contained `.deb` — stripped
binaries, library sources, a prebuilt `atc_tempctrl.tplc`, and a
`tplc-ide` wrapper (`tplc serve` with the installed libdirs):

```bash
cmake --build build -j8
cargo build --release -p tplcc -p tplc-pkg -p tplc -j8   # from repo root
scripts/make-deb.sh                      # → build/deb/tetherplc_0.1.0-1_amd64.deb
sudo apt install ./build/deb/tetherplc_*.deb
```

The package installs to `/usr/bin` (`tplc`, `tplcc`, `tplc-pkg`,
`tether-plc`, `tether-plc-sim`, `tplc-io`, `tplc-image`,
`tplc-io-*`, `tether-plc-bridge`, `tplc-ide`) and
`/usr/share/tetherplc` (`libraries/tempctrl`, `compat/{tc2,tc3}`,
`lib/atc_tempctrl.tplc`).
`Depends:` covers only runtime libs (libc6, libstdc++6, libssl3, zlib1g,
libffi8, libzstd1) — LLVM is statically linked into `tplcc`.

After install:

```bash
tplcc build my_program.st \
    --lib /usr/share/tetherplc/lib/atc_tempctrl.tplc -o my_program.tplc
tplcc build --library --name Atc_TempCtrl \
    /usr/share/tetherplc/libraries/tempctrl/atc_tempctrl.st \
    -o ~/atc_tempctrl.tplc        # rebuild the library package locally
tplc serve --port 8080            # IDE backend with libdirs preconfigured
```

To serve the deb from an apt repository, drop it into a `reprepro` (or
`apt-ftparchive`) pool as `tetherplc_<ver>_amd64.deb` — the control file
already carries `Package/Version/Architecture/Depends`.
