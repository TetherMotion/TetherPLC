# AGENTS.md — Project Guide for AI Agents

## Project Overview

TetherPLC is a soft-PLC that compiles IEC 61131-3 logic — Structured
Text, FBD, and LD — to native code and runs it on a real-time runtime
with a cyclic task engine, a shared-memory process image, a live debug
channel, and hot-swap package management. It is **[HIGHLY
EXPERIMENTAL]**: the acceptance matrix
(`docs/plc/acceptance-matrix.md`) is the honest status source — check
it before claiming a feature works.

Two halves:

- **`compiler/`** — Rust workspace. `tplcc` is the CLI
  (ST/FBD/LD → LLVM IR → `.tplc` package); `tplc-mir` is the middle
  IR; `tplc-st`/`tplc-codegen`/`tplc-rt` are frontend/lowering/
  runtime-contract layers; `tplc-pkg` is the .tplc package library;
  `fbd-model`,
  `fbd-lower`, `plcopen-io` are the graphical-language crates; `tplc`
  is the umbrella binary. Built with **cargo + LLVM 21**.
  `tplc` also owns **project workspaces**: `tplc new <dir>`
  scaffolds a self-contained project (`.tplc/project.json` manifest,
  `src/`, `.gitignore`, initial git commit via `git2` — no database).
  `run`/`sim`/`dev`/`serve` auto-discover the project by walking up to
  `.tplc/project.json` (`--project` overrides) and compile the
  manifest's `sources` into `build/app.tplc`, naming the
  session/image/ctl socket after the project. `tplc git
  status|diff|commit|log|branch` + `tplc projects` expose the
  per-project repo; `tplc serve` mirrors the same ops over `/ws`
  meta (`project_*`, `git_*`, `projects`) for the WebUI Project
  pane. The IDE document round-trips through `manifest.ui`; the
  browser `localStorage` copy is only a fallback when no project is
  open.
- **`runtime/` + `include/tplc/` + `tests/`** — C/C++20. `tether-plc`
  is the realtime runtime executable; `tether-plc-sim` is the
  deterministic step replay used by tests; `tether-plc-bridge` joins
  the process image to file/IO transports; `tplc-ec-pump` is the
  EtherCAT master pump; `tplc-io*` are the Tether IO server/client
  tools; `tplc-image` is the process-image tool. The control channel
  is driven by `tplc` (Rust, `compiler/tplc/`): `tplc ctl` is the
  client CLI, `tplc serve` the `/ws` browser backend, `tplc dev` the
  unified dev stack. Host services (alarms, historian, MQTT, …)
  live under `runtime/host/`; the C ABI surface is `include/tplc/*.h`.

**Sibling dependency:** `tplc-io`/`tplc-ec-pump` need a Tether
checkout — CMake auto-detects `../Tether` (or `-DTETHER_DIR=`). The
EtherCAT stack, fleet catalog (`tether/ethercat/BeckhoffFleet.hpp`),
and `tether_io` protocol all come from there. **Tether is
library-only for new code — never add executables to it**; PLC-facing
tools belong here.

## Build Commands

> **Parallelism:** always `-j8` max (`cmake --build build -j8`,
> `cargo build -j8`). Higher parallelism OOM-kills this machine.

```bash
# Compiler (Rust) — release for real use, debug is fine for tests
cargo build --release -p tplcc -p tplc-pkg -j8   # from repo root
# or the debug binaries tests default to:
cargo build -p tplcc -j8

# Runtime (C/C++) — configure once
cmake -B build
cmake --build build -j8

# Optional features (auto-detected, force with -D)
cmake -B build -DTPLC_ENABLE_MQTT=ON      # libpaho MQTT (FetchContent fallback)
cmake -B build -DTPLC_ENABLE_OPCUA=ON     # open62541
cmake -B build -DTPLC_ENABLE_IEC61850=ON  # libiec61850

# Rebuild just the pump/bridge/etc.
cmake --build build --target tplc-ec-pump -j8
cmake --build build --target tether-plc tether-plc-sim -j8
```

Key outputs in `build/`: `tether-plc`, `tether-plc-sim`,
`tether-plc-bridge`, `tplc-image`, `tplc-io`, `tplc-io-probe`,
`tplc-io-cli`, `tplc-ec-pump`, plus a `tplc-ctl` shim that forwards
to the Rust `tplc ctl` for legacy test invocations.
Compiler binaries: `compiler/target/{release,debug}/tplcc` and `tplc`.

## Test Commands

Tests are ctest-registered (`ctest -N` is the registry of truth) plus
standalone shell e2e scripts. **Exit 77 = SKIP** — tests needing
optional tooling (tplcc, python3, a bus) skip rather than fail.

```bash
cd build && ctest --output-on-failure -j8          # whole suite
ctest -R '<regex>' --output-on-failure             # subset

# Standalone e2e scripts (bash, self-contained, use mktemp dirs):
bash tests/compat/mqtt_journal_io.sh               # example
bash tests/ec/fleet_e2e.sh                         # EtherCAT stack
bash tests/homeassistant/ha_bridge_io.sh
TPLCC=compiler/target/debug/tplcc bash tests/...   # override compiler
```

E2e script conventions: `TPLCC`/`BUILD`/port env vars are
overridable; they `cd` to repo root; they exit 77 when prerequisites
are missing; pass/fail counters print `== N pass, M fail`.

**Stale-binary footgun:** rebuild test targets before trusting
failures — `cmake --build build --target <t> -j8` first.

## Running a Program (verified quickstart)

```bash
TPLCC=compiler/target/debug/tplcc   # or target/release

# 1. compile ST → package (oven.st is the README quickstart example;
#    examples/ha_demo.st needs --lib libraries first — see README)
$TPLCC build oven.st -o /tmp/q.tplc

# 2. create the process image — the name MUST be a POSIX shm name
#    (/name), NOT a filesystem path, and sized for AT vars
./build/tplc-image create /tplc_q 64 64

# 3a. deterministic replay (tests) — --steps takes a STEP FILE:
#     step <n>  time=<ns>  in <i>=<hex>  expect <o>=<hex>
./build/tether-plc-sim --image /tplc_q --package /tmp/q.tplc \
    --steps steps.txt

# 3b. realtime (runs forever)
./build/tether-plc --image /tplc_q --package /tmp/q.tplc \
    --ctl /tmp/tplc_q.ctl

# 4. inspect/control
./compiler/target/debug/tplc ctl /tmp/tplc_q.ctl status
./compiler/target/debug/tplc serve --ctl /tmp/tplc_q.ctl --port 8080   # browser IDE backend
```

## Architecture Essentials

- **Package format:** `.tplc` = compiled program + task bindings +
  service contracts. `tplcc build` (program) and
  `tplcc build --library --name X lib.st` (reusable FB/FN library,
  linked later via `--lib`).
- **Process image:** POSIX shm, `%I`/`%Q`/`%M`/`%MW` areas. `tplc-image
  create <shm-name> <in-bytes> <out-bytes>`; the shm *name* (not a
  path) is the handle — `tplc-image create /tmp/x` fails with -106.
- **Deploy map:** binds image locations to transports — `key=` for
  file/sim channels, `pdo=` for live EtherCAT endpoints. Generated
  and validated client-side in the WebUI (`src/plc/deploymap.ts` in
  TetherWebUI); consumed/validated by `runtime/bridge`.
- **ctl channel:** role-based ops over a socket (`--ctl`). New ops
  get a `TPLC_OP_*` code in the op table; observer-role ops must not
  mutate program state. ABI structs in `include/tplc/` are
  versioned — append fields, never reorder (see `tplc_mqtt.h` ABI v2
  for the pattern: `reserved[]` slots, sizeof-stable additions).
- **Services:** host services (MQTT, historian, alarms, eventlog,
  fieldbuses) are reached through the runtime service vtable; ST
  blocks marshal member layouts into Cfg structs — offsets are
  pinned by layout tests, keep ST member order == C struct order.

## Repo Layout

```
compiler/           Rust crate sources (workspace root is the repo
                    toplevel Cargo.toml; artifacts stay in
                    compiler/target via .cargo/config.toml)
include/tplc/       C ABI headers — one per service/domain
libraries/          ST library sources (built with --library):
                    tempctrl, mqtt, homeassistant, canopen, iolink,
                    profinet, …
compat/             IEC 61131-3 / vendor compat ST wrappers (tc2/tc3)
runtime/
  image/            tplc_image — shm process image
  host/             runtime services + CtlServer
  tether-plc/       realtime executable (main.cpp)
  tether-plc-sim/   deterministic replay
  tether-plc-bridge/, bridge/   process image ↔ transports, deploy-map
  ecpump/           tplc-ec-pump — EtherCAT master pump executable
  tplc-io/          tether_io server/client/cli
tests/              ctest suite + standalone *_io.sh e2e scripts
docs/plc/           as-built specs (architecture.md is the doc map)
examples/           demo .st programs (ha_demo.st etc.)
tools/              tplc-image source, ctl_client.h, oracle adapters
schema/ packaging/ scripts/   package schema, deb packaging, misc
```

## Conventions

- **C ABI discipline:** `include/tplc/*.h` structs are append-only.
  Reuse `reserved[]` padding before growing `sizeof`; bump the ABI
  version constant when semantics change. ST-side layouts in
  `compiler/tplc-mir` must mirror them exactly — there are offset
  tests; run them after touching a header.
- **ST library blocks:** every feature ships as pin-driven FUNCTION_BLOCKs
  so FBD can wire them — no member-array config that FBD can't reach.
  Inputs that should pulse get edge guards inside the FB (see
  `FB_MqttTx`/`FB_HaEntity` — persistent BOOL inputs re-fire every
  scan otherwise).
- **Compat wrappers** (`compat/tc2`, `compat/tc3`) are thin ST
  aliases over the native blocks — keep them that way.
- **E2e tests** live next to their domain (`tests/<area>/`), are
  bash + the Python fake broker/devices under `tests/compat/`, and
  must skip (77) cleanly without prerequisites.
- **Docs:** `docs/plc/architecture.md` §7 is the doc map — new
  subsystems get a spec page there. README is the user-facing
  quickstart; keep commands verified against real binaries.
- Commits: short imperative subject; body explains *why*. Sign-off
  trailer as usual.

## Gotchas (learned the hard way)

- `tether-plc-sim --steps` takes a **file**, not a count; format is
  `step <n>  time=<ns>  in <i>=<hex>  expect <o>=<hex>`.
- Sim replay runs **faster than wall-clock** — one-shot timed injects
  race the run; use periodic/repeated injects or long step counts.
- `tplc-image create` wants a POSIX shm name (`/tplc_x`), not a path.
- Never `pkill -f <pattern>` where the pattern appears in your own
  command line — it kills the shell. Kill by PID.
- Level-held BOOL outputs re-trigger edge logic every scan — guard
  with previous-state members inside the FB.
- An FB does nothing until `Execute := TRUE` — a common silent-test
  bug.
- MQTT: `TRANSPORT` 0–4 = auto/tcp/ssl/ws/wss; service journal
  (ABI v2 ops `MQTT_STATUS`/`MQTT_JOURNAL`) only records messages the
  program actually polls. WebUI Events pane consumes them.
- EtherCAT live discovery (BRD over UDP) **does not work in this
  sandbox** — `tests/ec/fleet_e2e.sh` and `tplc-ec-pump --scan` fail
  with "no slaves discovered" even on unmodified code. Unit-test the
  catalog/parser layer; treat live-bus results as environment-gated.
- `tplc-ec-pump` writes `.fleet`/`.topology`/`.bus` artifacts into
  `--simdir`; `.topology` is the source for the WebUI discovered-
  terminals view (GET /bus endpoint is planned, not yet built).
- WebUI (`../TetherWebUI`) mirrors the terminal catalog in
  `src/plc/deploymap.ts` — keep `pdo=` endpoint names in lockstep
  with `fleetInfosFromText` keys (`el1014.c1`, `.c1.2`, …).

## Docs Map

- `README.md` — build, quickstart, feature tour (verified commands)
- `docs/plc/architecture.md` — concepts + doc map (§7)
- `docs/plc/testing.md` — test taxonomy and gates
- `docs/plc/plc-runtime.md`, `process-image.md`, `io-mapping.md` —
  runtime internals
- `docs/plc/deploymap`-adjacent specs: `io-mapping.md`,
  `tether-io-attach.md`
- `docs/plc/ec`/`profinet.md`/`canopen.md`/`iolink.md` — fieldbuses
- `docs/plc/acceptance-matrix.md` — honest feature status
- `USERSTORIES.md`, `MISSING.md` — roadmap and known gaps
