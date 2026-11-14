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

## 3. `tplc-ctl-proxy` — HTTP → UDS bridge

Browsers can't reach `AF_UNIX`. The proxy is a small single-threaded
HTTP/1.1 server (`tools/tplc_ctl_proxy.cpp`, no deps) exposing:

| route | method | what it does |
|---|---|---|
| `/ctl` | POST | one verbatim ctl frame round-trip (auto-issues the session `HELLO` per UDS connection) |
| `/compile` | POST | binary src frame → `tplcc check|build --json`; wraps output as `{compile, exit, package?b64}` |
| `/events` | GET | SSE stream of ctl `F_EVENT` frames (see `web-ide.md` §events) |
| `/blocks` | GET | merged `tplcc blocks --json` catalog over every `--libdir` (cached; `?refresh=1` forces) |
| `/libsrc` | GET | `?lib=<name|index>&file=<name>` → raw ST source (direct `.st` or extracted from a `.tplc`) |
| `/health` | GET | `ok` iff the ctl UDS connects |
| any | OPTIONS | CORS preflight (`Access-Control-Allow-Origin: *`) |

The src frame is `[u16 count][u16 nameLen,name,u32 len,content]×N` LE;
names are basename-sanitized, ≤64 files × 4 MB. The frame carries every
source of a mixed project (`project.fbd.json` + all `.st`), which is why
the IDE's build & deploy path works for FBD+ST modules. Build responses
include the `.tplc` as base64. `exit` mirrors the compiler's verdict:
the proxy ignores `SIGCHLD` (auto-reap for `/events` children), so
`pclose()` may report `ECHILD`; a `"ok":true` in the wrapped JSON is
treated as exit 0 and still attaches the package
(`tests/proxy/e2e.sh` guards this).

Flags: `--socket` (ctl UDS, default `/tmp/tplc-image.ctl`), `--port`,
`--bind`, `--tplcc` (compiler path for the popen path), `--tplccd`
(daemon UDS, default `/tmp/tplccd.sock`), `--libdir <dir>` (repeatable;
each dir's `.st` files and `.tplc` packages feed `/blocks` and
`/libsrc` — point it at the installed `libraries/` tree, e.g.
`--libdir libraries/tempctrl --libdir libraries/compat/tc2`).

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

## 6. Debian package (`apt install`)

`scripts/make-deb.sh` stages a self-contained `.deb` — stripped
binaries, library sources, a prebuilt `atc_tempctrl.tplc`, and a
`tplc-proxy` wrapper with the installed libdirs:

```bash
cmake --build build -j8
(cd compiler && cargo build --release -p tplcc -p tplc-pkg -j8)
scripts/make-deb.sh                      # → build/deb/tetherplc_0.1.0-1_amd64.deb
sudo apt install ./build/deb/tetherplc_*.deb
```

The package installs to `/usr/bin` (`tplcc`, `tplc-pkg`, `tether-plc`,
`tether-plc-sim`, `tplc-io`, `tplc-ctl`, `tplc-ctl-proxy`, `tplc-image`,
`tplc-io-*`, `tplc-proxy`) and `/usr/share/tetherplc`
(`libraries/tempctrl`, `compat/{tc2,tc3}`, `lib/atc_tempctrl.tplc`).
`Depends:` covers only runtime libs (libc6, libstdc++6, libssl3, zlib1g,
libffi8, libzstd1) — LLVM is statically linked into `tplcc`.

After install:

```bash
tplcc build my_program.st \
    --lib /usr/share/tetherplc/lib/atc_tempctrl.tplc -o my_program.tplc
tplcc build --library --name Atc_TempCtrl \
    /usr/share/tetherplc/libraries/tempctrl/atc_tempctrl.st \
    -o ~/atc_tempctrl.tplc        # rebuild the library package locally
tplc-proxy --port 8080            # IDE backend with libdirs preconfigured
```

To serve the deb from an apt repository, drop it into a `reprepro` (or
`apt-ftparchive`) pool as `tetherplc_<ver>_amd64.deb` — the control file
already carries `Package/Version/Architecture/Depends`.
