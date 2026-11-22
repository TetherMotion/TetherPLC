# TetherPLC

**[HIGHLY EXPERIMENTAL]** A soft-PLC that compiles IEC 61131-3 logic —
Structured Text, FBD and LD — to native code and runs it on a real-time
runtime with a cyclic task engine, process image, live debug channel,
and hot-swap package management.

The compiler (`tplcc`, Rust + LLVM 21) lowers ST/FBD/LD to a Tether
package; the runtime (`tether-plc`, C++20) executes it against a shared
memory process image with deterministic scan tasks. Everything is
scriptable: compile, deploy, monitor and debug from the CLI, CI tests,
or the browser IDE ([TetherWebUI](https://github.com/TetherMotion/TetherWebUI)).

- **Concepts & architecture:** [`docs/plc/architecture.md`](docs/plc/architecture.md)
- **Design goals:** [`docs/DESIGN-GOALS.md`](docs/DESIGN-GOALS.md)
- **Per-topic as-built specs:** [`docs/plc/`](docs/plc/) (doc map in architecture.md §7)
- **v1 acceptance status:** [`docs/plc/acceptance-matrix.md`](docs/plc/acceptance-matrix.md)
- **Temperature-control quickstart:** [`docs/plc/quickstart-tempcontrol.md`](docs/plc/quickstart-tempcontrol.md)
- **Home Assistant demo:** [`examples/ha_demo.st`](examples/ha_demo.st)

---

## What you can build

| Area | What's included |
|---|---|
| Languages | ST, FBD, LD — mixed in one project, callable across languages |
| Control | PID, cascade, Smith predictor, **LQR/LQG + DARE + Kalman**, relay/step autotuners, tuning rules |
| Temperature | Profile engine, soldering/reflow curves, scaling, batch recipes |
| Fieldbuses | EtherCAT (EK/EL terminals), PROFINET, CANopen, EtherNet/IP, Modbus, BACnet, IEC 60870-5-104, IEC 61850 |
| IT | MQTT 3.1.1/5 (tcp/ssl/**ws/wss**, credentials, TLS, last-will), Home Assistant bridge + entity blocks |
| Other | OPC UA, SMTP, FTP, S7, ADS, SACN, DMX, serial, IO-Link, Genibus, MP-Bus |
| Runtime | Multiple PROGRAMs, task bindings, retain, shadow/online-change, eventlog, historian, trace rings |
| Security | Signed packages, sandboxing (seccomp/rlimits), role-based ctl auth, audit log |

---

## Repository layout

```
compiler/            Rust workspace — tplc (unified entrypoint: dev stack,
                     ctl client, HTTP/WebUI backend), tplcc (ST/FBD/LD →
                     LLVM), tplc-pkg, tplc-mir/tplc-st/tplc-codegen/
                     tplc-rt, fbd-model, plcopen-io, fbd-lower
runtime/
  image/             tplc_image — shared-memory process image (C)
  host/              runtime services + CtlServer (alarms, historian, MQTT, …)
  tether-plc/        the runtime executable (main.cpp)
  tether-plc-sim/    deterministic step replay for tests
  tether-plc-bridge/ process image ↔ file/IO transport bridge
  tplc-io/           tether_io server fronting the ctl channel
  ecpump/            EtherCAT master pump (PDO ↔ simdir files)
  pkg/               package loader (manifest, sections, signature check)
libraries/           IEC libraries compiled with tplcc --library:
                     tempctrl/  atc_tempctrl.st, atc_ops.st  (PID, LQR/LQG,
                                autotune, profiles, recipes, alarms)
                     mqtt/      mqtt_blocks.st, mqtt_events.st
                     homeassistant/  ha_bridge.st, ha_entity.st
                     canopen/ iolink/ profinet/
compat/              tc2/tc3 compatibility wrappers (.st)
examples/            ha_demo.st (interactive Home Assistant demo)
TetherWebUI/         git submodule — browser IDE; `tplc dev`/`serve`
                     auto-builds its dist/ (override: --webui/TPLC_WEBUI)
tests/               unit, compat e2e scripts, vector suites (see below)
docs/                architecture, per-topic specs, design goals
```

---

## Prerequisites

- **C++20/23 compiler** — GCC ≥ 13 recommended (the runtime uses
  `<format>`/`<print>`/`<expected>` shims when the stdlib lacks them).
- **CMake** ≥ 3.16, **pkg-config** not required.
- **Rust toolchain** (stable, edition 2021) for `tplcc`.
- **LLVM 21** development files for `tplcc` (`inkwell`/`llvm21-1`).
  On Ubuntu: `apt install llvm-21-dev libclang-21-dev` or point
  `LLVM_SYS_211_PREFIX` at an LLVM 21 install.
- **TetherWebUI submodule** (`TetherWebUI/`): the browser IDE — init it
  with `git submodule update --init` (or clone `--recursive`). On first
  `tplc dev`/`tplc serve` its `dist/` is built automatically
  (`npm install` + `npm run build` when missing or stale;
  `TPLC_NO_WEBUI_BUILD=1` opts out). `--webui <dir>` / `TPLC_WEBUI`
  point at any built dist; a sibling `../TetherWebUI` checkout is the
  fallback when the submodule isn't initialized.
- **Optional — sibling Tether checkout** at `../Tether` (or set
  `-DTETHER_DIR=`): required for `tplc-io`, `tplc-io-cli`,
  `tplc-io-probe` and `tplc-ec-pump` (EtherCAT fleet pump, tether_io
  transport). Without it those targets are skipped cleanly.
- **Optional libraries** (all auto-fetched or gracefully disabled):
  libpaho-mqtt3c(s) for MQTT (**ON by default** — FetchContent builds
  paho.mqtt.c v1.3.14 with TLS if not installed), libsqlite3, libsodium,
  libseccomp, open62541 (OPC UA), libiec61850.

---

## Build from source

### 1. Compiler (Rust) — one command builds everything

The Cargo workspace lives at the repo root; crates are under
`compiler/`. `cargo build` also drives the CMake build for the runtime
tools `tplc` orchestrates (set `TPLC_SKIP_NATIVE_BUILD=1` to skip that):

```bash
cargo build --release -p tplcc -p tplc-pkg -p tplc -j8
# → compiler/target/release/{tplcc,tplc}  (target dir unchanged)
# → build/{tether-plc,tether-plc-sim,tether-plc-bridge,tplc-image,tplc-io*,tplc-ec-pump}
```

`tplcc` needs LLVM 21 visible to `llvm-sys`. If it isn't on the default
path:

```bash
export LLVM_SYS_211_PREFIX=/path/to/llvm-21   # dir containing bin/llvm-config
```

### 2. Runtime (CMake)

```bash
cmake -B build -DTPLC_ENABLE_MQTT=ON      # MQTT on by default
cmake --build build -j8                   # keep -j ≤ 8 on small machines
```

Useful options:

| Flag | Default | Effect |
|---|---|---|
| `TPLC_ENABLE_MQTT` | ON | MQTT client service (Paho; FetchContent fallback builds it) |
| `TPLC_ENABLE_OPCUA` | ON | OPC UA server service (needs open62541) |
| `TPLC_ENABLE_IEC61850` | ON | IEC 61850 service (FetchContent fallback) |
| `TETHER_DIR` | `../Tether` | Tether checkout → enables `tplc-io*`, `tplc-ec-pump` |
| `TPLC_SQLITE3_LIBRARY`, `TPLC_SODIUM_LIBSODIUM`, `TPLC_SECCOMP_LIBSECCOMP` | auto | point at non-standard installs |

Binaries land in `build/`: `tether-plc`, `tether-plc-sim`,
`tether-plc-bridge`, `tplc-image`,
`tplc-io`, `tplc-io-cli`, `tplc-io-probe`, `tplc-ec-pump`
(`tplc-ctl` there is a shim forwarding to `tplc ctl`). The Rust
`tplc` entrypoint builds via `cargo build -p tplc`.

### 3. Debian package (optional)

```bash
cmake --build build --target package   # or cpack -G DEB
# → build/deb/tetherplc_<ver>_amd64.deb  →  sudo apt install ./…deb
```

---

## Quickstart — first program in 5 minutes

The normal workflow is **project-based**: `tplc` keeps each project in
its own directory (`.tplc/project.json` manifest + sources + an
integrated git repo — no database) and auto-detects it from any
subdirectory.

### 1 — Build `tplc`

```bash
cargo build --release -p tplc          # → compiler/target/release/tplc
```

### 2 — Create a project

```bash
./compiler/target/release/tplc new oven
cd oven
# → .tplc/project.json   project manifest (sources, libs, image sizes…)
# → src/main.st          starter ST program
# → .gitignore           build/, .tplc/cache/, *.o, *.so
# → .git/                repo with one "scaffold" commit
```

`tplc new` also works as `tplc init` inside an existing directory.
Multiple projects = multiple directories; `tplc projects` lists the
recent ones.

### 3 — Write the program

Edit `src/main.st` (ST shown; FBD/LD projects are edited in the browser
IDE and stored in the manifest's `ui` member):

```st
// src/main.st — blink a %Q output at 1 Hz using the task clock
PROGRAM Oven
VAR
    tick AT %QX0.0 : BOOL;
    t    : TIME;
END_VAR
t := t + T#50ms;
IF t >= T#1s THEN
    t := T#0ms;
    tick := NOT tick;
END_IF;
END_PROGRAM
```

`AT %QX0.0` places the var in the process image (see
[`docs/plc/process-image.md`](docs/plc/process-image.md)). Add more
`.st` files under `src/` and list them in the manifest's `sources`
(`tplc project set sources=src/main.st,src/extra.st`), or just pass
files on the command line — explicit sources override the manifest.

### 4 — Run the dev stack

```bash
./compiler/target/release/tplc dev
# inside the project dir the manifest sources are picked up
automatically; compile → image → tether-plc → tplc-io → webserver:
#   http://127.0.0.1:8089  WebUI (TetherWebUI/ submodule, auto-built)
#   tplc ctl status        inspect the running PLC
#   tplc down              shut the stack down
```

`tplc dev` reuses a live runtime if one is already up, and names the
session/image/ctl socket after the project (`oven`). From a subdirectory
it still finds the project; `--project /path` targets another one.

### 5 — Version it

```bash
tplc git status            # "M src/main.st" / "?? new.st" / …
tplc git diff              # unified diff vs HEAD
tplc git commit wip        # stage everything + commit (manual commits)
tplc git log               # history
tplc git branch            # current branch
```

Or use the IDE's **Project** pane — manifest editor, file tree, git
status/diff/commit/log and a recent-projects picker, all over the same
backend.

### Other composites

| Command | What it does |
|---|---|
| `tplc run [src…]` | compile → image → realtime `tether-plc`; Ctrl-C stops |
| `tplc sim [src…] --steps f` | deterministic step replay (CI/tests) |
| `tplc serve` | HTTP + `/ws` backend + WebUI static files only |
| `tplc ctl <op>` | native ctl client (`tplc ctl` lists ops) |
| `tplc doctor` | tool resolution check |
| `tplc cc …` | `tplcc` verbatim (incl. `tplc cc serve` compile daemon) |
| `tplc image\|bridge\|io\|pump` | pass-throughs to the runtime tools |

## Under the hood — the manual pipeline

Everything in this section is what `tplc dev`/`run`/`sim` orchestrate for
you — useful when scripting CI or driving the runtime directly.

```bash
TPLCC=compiler/target/release/tplcc

# single program → signed-capable .tplc package (default task 10 ms)
$TPLCC build src/main.st -o app.tplc

# slower scan, e.g. 100 ms for temperature loops
$TPLCC build src/main.st --task-period-ns 100000000 -o app.tplc

# with libraries (compiled once as --library, linked with --lib)
$TPLCC build --library --name Atc_TempCtrl libraries/tempctrl/atc_tempctrl.st -o /tmp/atc.tplc
$TPLCC build myapp.st --lib /tmp/atc.tplc -o myapp.tplc

$TPLCC info app.tplc     # package manifest/sections
$TPLCC blocks            # FBD block catalog (--json for tooling)
```

### Run — deterministic sim (tests/CI)

`tether-plc-sim` replays a **steps file** — one `step` per scan with
logical time, optional `%I` stimuli and `%Q` expectations:

```text
# oven.steps — 31 scans @10 ms logical
step 0  time=0          in 0=00
step 1  time=10000000   in 0=00  expect 0=00
step 30 time=300000000  in 0=00  expect 0=01
```

```bash
./build/tplc-image create /oven 64 64
./build/tether-plc-sim --image /oven --package oven.tplc \
    --steps oven.steps --ctl /tmp/oven.ctl
# → "31 steps all matched"; --dump-q FILE logs every step's %Q image,
#   --scan runs the module scan, --settle-ms/--hold-ms tune timing
# --plc PATH overrides the runtime (default ./build/tether-plc)
```

This is how all `tests/compat` e2e scripts drive the PLC headlessly.

### Run — realtime

```bash
./build/tplc-image create /oven 64 64
./build/tether-plc --image /oven --package oven.tplc --ctl /tmp/oven.ctl
# runs forever; Ctrl-C or `tplc ctl /tmp/oven.ctl rt-shutdown` stops
```

`--package` extracts the module and runs it; `--module x.so` runs a
pre-extracted module directly. Process image is POSIX shm —
`tplc-image dump /oven` peeks at `%I`/`%Q`/`%M` live.

### Control it — `tplc ctl`

```bash
tplc ctl /tmp/oven.ctl status          # runtime state
tplc ctl /tmp/oven.ctl prog            # program status
tplc ctl /tmp/oven.ctl dbg-state       # symbol → image offsets
tplc ctl /tmp/oven.ctl watch 0:1       # live var watches
tplc ctl /tmp/oven.ctl map-get         # deploy map in package
tplc ctl /tmp/oven.ctl events 60       # eventlog tail
tplc ctl /tmp/oven.ctl deploy new.tplc # package swap
# … break/force/trace/retain/shadow/od — full list: `tplc ctl` with no args
```

Full op list: [`include/tplc/tplc_ctl.h`](include/tplc/tplc_ctl.h) and
[`docs/plc/debugging.md`](docs/plc/debugging.md). Roles: observer /
engineer / security — set at `HELLO`.

### Web backend — `tplc serve` (WebSocket)

```bash
tplc serve --ctl /tmp/oven.ctl --port 8089 \
    --tplcc compiler/target/release/tplcc --tplc-io 127.0.0.1:43770
# GET /ws            one WebSocket: binary ctl frames + JSON meta-ops
#                    (compile/blocks/libsrc/pkgmap), F_EVENT pushes
# GET /tether-io     WebSocket → tplc-io provider (live values)
# GET /health        liveness
# GET /*             static WebUI (TetherWebUI/ submodule, auto-built)
```

Point TetherWebUI's dev proxy at this and the browser IDE gets
compile + live monitor + deploy.

---

## Projects & workspaces — reference

Quickstart steps 2 and 5 cover the basics; this is the full reference.
Each project is a self-contained directory — no database, just files
plus an integrated git repository (one repo per project, `git2`-backed,
manual commits; only the scaffold commit is automatic).

```bash
tplc project            # show manifest (pretty JSON); `tplc project` with
                        #   no subcommand inside a project prints it
tplc project path       # print the discovered project root
tplc project set K=V …  # edit manifest fields without a text editor
                        #   (name description sources libs taskPeriodNs
                        #    imageIn imageOut deployMap)
```

**Discovery rules** — `tplc run|sim|dev|serve` (and the `/ws`
`project_*` ops) look for `.tplc/project.json` starting at the cwd and
walking up; `--project /path` (CLI) or `project_open` (WS) targets a
specific directory. When a manifest is found, its `sources` are
compiled into `<project>/build/app.tplc` and the session/image/ctl
socket are named after the project; explicit source arguments override
the manifest.

**Git** — all commits are manual (`tplc git commit <msg>` stages
everything first). `tplc git diff [path]` diffs worktree vs HEAD;
`tplc git log [n]` defaults to 20 entries.

`.tplc/project.json` fields: `name`, `description`, `version`,
`sources` (ST files, relative to the project root), `libs` (library
dirs), `taskPeriodNs`, `imageIn`/`imageOut` (process-image bytes),
`deployMap`, plus an opaque `ui` member where the WebUI persists its
FBD/ST/curve/tune working document — unknown keys round-trip
untouched, so newer schemas stay forward-compatible.

**Browser IDE** — the same operations are exposed over the `/ws` meta
channel: `project_get/save/open`, `project_files`,
`project_file_get/put/delete`, `projects` (recent roots), and
`git_status/diff/commit/log/branch`. The IDE's **Project** pane shows
the manifest editor, file tree, git status/diff/commit UI, log, and a
recent-projects picker; the IDE document itself is stored in
`manifest.ui`. When no backend project is open the IDE falls back to
browser `localStorage` (`tether.plc.project.v1`).

---

## I/O and fieldbuses

### Process image + deploy map

PLC logic only sees `%I`/`%Q`/`%M`. The **deploy map** (bridge text
format, section `0x31` of the package) binds image bytes to transport
channels — `key=` files for the sim transport, `pdo=` endpoints for the
live EtherCAT bus. Generate/validate:

```bash
./build/tether-plc-bridge --gen-map app.tplc --bus simdir/.bus -o map.txt
./build/tether-plc-bridge --validate map.txt --bus simdir/.bus
./build/tether-plc-bridge --map-hash map.txt   # epoch stamp
```

The WebUI's Deploy pane does the same graphically (channel/binding
grid, overlap/AT/PDO diagnostics). Spec:
[`docs/plc/io-mapping.md`](docs/plc/io-mapping.md).

### EtherCAT (EK1100 couplers + EL terminals)

`tplc-ec-pump` is the master side: it brings the bus up (discovery →
PREOP → SAFEOP → OP), then pumps each terminal's PDO image to files in
`--simdir` that `tether-plc-bridge` maps into the process image.

```text
# cell.fleet — one node per line, chain order; EK1100 opens a coupler
EK1100
EL1014        # 4 DI  → pdo=el1014.c1
EL2004        # 4 DO  → pdo=el2004.c1
EL3104        # 4 AI  → pdo=el3104.c1
EK1100
EL1014        # second coupler segment → el1014.c2
```

```bash
# emulated bus (Tether's beckhoff_slave_chain) — no hardware needed
../Tether/build/bin/beckhoff_slave_chain --port 47820 --io-port 43771 &
./build/tplc-ec-pump --peer 127.0.0.1 --port 47820 \
    --simdir /tmp/tplc_sim --fleet cell.fleet --io-port 43770 --hz 500 &
./build/tether-plc-bridge --image /cell --map map.txt \
    --simdir /tmp/tplc_sim --plc ./build/tether-plc \
    --image /cell --package app.tplc --ctl /tmp/plc.ctl &
```

Real hardware: `tplc-ec-pump --iface eth0` (CAP_NET_RAW) against a NIC
on the EtherCAT segment; `--fleet` accepts the same file. End-to-end
test: `tests/ec/fleet_e2e.sh`.

### MQTT / Home Assistant

`FB_Mqtt*` and `FB_HaBridge`/`FB_HaEntity` in `libraries/` drive the
`tplc-mqtt` host service — all options are pins/inputs: host, port,
credentials, client-id, `TRANSPORT` (0 auto, 1 tcp, 2 ssl, 3 ws,
4 **wss**), TLS CA/cert/key, MQTT version 3/5, last-will, backoff.

```bash
# quick offline check — fake broker + sim replay
TPLCC=compiler/target/release/tplcc \
    bash tests/compat/mqtt_blocks_io.sh        # ws/wss + credentials
TPLCC=compiler/target/release/tplcc \
    bash tests/homeassistant/ha_entity_io.sh   # HA discovery + cmd
```

Against a real broker see [`examples/ha_demo.st`](examples/ha_demo.st)
(header comment has the full field list) — Mosquitto on :1883 or any
TLS/WSS endpoint (HiveMQ/EMQX Cloud, AWS IoT).

Inbound MQTT messages are journaled twice: the service keeps a
64-entry/connection ring (visible via ctl `MQTT_STATUS`/`MQTT_JOURNAL`
and the IDE **Events** pane), and `FB_MqttEventLog`
(`libraries/mqtt/mqtt_events.st`) keeps a logic-side ring you can
trend or alarm on.

---

## Libraries

Compile once, link into any program:

```bash
$TPLCC build --library --name Atc_TempCtrl libraries/tempctrl/atc_tempctrl.st -o atc.tplc
$TPLCC build --library --name HaBridge     libraries/homeassistant/ha_bridge.st -o ha.tplc
$TPLCC build --library --name MqttBlocks   libraries/mqtt/mqtt_blocks.st -o mqtt.tplc
$TPLCC build --library --name MqttEvents   libraries/mqtt/mqtt_events.st -o events.tplc
```

| Library | Highlights |
|---|---|
| `tempctrl` | `FB_AtcPid`, cascade, Smith, `FB_AtcLqr`/`FB_AtcLqg` (DARE+Kalman), `FB_AtcAutoTune*` (relay/step), `FB_AtcProfile` (segments incl. reflow/solder curves), `FB_AtcBatch`, `FB_AtcPwm`, scalers, alarms |
| `mqtt` | `FB_MqttLink/Tx/Rx/Sub/Unsub` (pin-driven FBD blocks), `FB_MqttEventLog` journal |
| `homeassistant` | `FB_HaBridge` (16 channels), `FB_HaEntity` (single entity, 50+ pins) |
| `canopen`, `iolink`, `profinet` | fieldbus function blocks |
| `compat/tc2`, `compat/tc3` | TwinCAT-style wrapper FBs |

`$TPLCC blocks --json` lists every FB/FUNCTION with pin signatures —
this is what the WebUI palette and the `blocks` meta-op serve.

---

## Testing

```bash
# unit + conformance (C/C++ and Rust)
cargo test -j8               # default workspace members (LLVM-free set)
ctest --test-dir build --output-on-failure

# compat e2e — each script spins fake brokers / sim replay
TPLCC=compiler/target/release/tplcc bash tests/compat/mqtt_journal_io.sh
TPLCC=compiler/target/release/tplcc bash tests/compat/mqtt_journal_ctl.sh
TPLCC=compiler/target/release/tplcc bash tests/homeassistant/ha_bridge_io.sh
TPLCC=compiler/target/release/tplcc bash tests/tempctrl/run.sh   # vector suite

# EtherCAT fleet e2e (needs ../Tether built)
TPLCC=compiler/target/release/tplcc bash tests/ec/fleet_e2e.sh
```

Layout: `tests/compat` (broker/IOLink e2e), `tests/tempctrl` (control
vectors), `tests/ec` (EtherCAT), `tests/runtime|debug|scan|fault`
(realtime/ctl/sandbox), `tests/image|pkg|abi` (format conformance),
`tests/mixed|fbd_gallery` (FBD↔ST), `tests/sim` (replay).

---

## Documentation map

| Doc | Contents |
|---|---|
| [architecture.md](docs/plc/architecture.md) | system overview, doc map (§7) |
| [quickstart-tempcontrol.md](docs/plc/quickstart-tempcontrol.md) | EL1014→profile→EL2004 worked example |
| [process-image.md](docs/plc/process-image.md) | `%I/%Q/%M`, AT addressing |
| [io-mapping.md](docs/plc/io-mapping.md) | deploy map format, pdo=/key= channels |
| [plc-runtime.md](docs/plc/plc-runtime.md) | tasks, scans, retain, services |
| [debugging.md](docs/plc/debugging.md) | ctl protocol, break/force/trace/watch |
| [management.md](docs/plc/management.md) | packages, deploy, rollback, signatures |
| [atc-tempctrl.md](docs/plc/atc-tempctrl.md) | temperature-control library reference |
| [online-change.md](docs/plc/online-change.md) | shadow/promotion/rollback |
| [security.md](docs/plc/security.md) | sandbox, signing, ctl auth |
| [acceptance-matrix.md](docs/plc/acceptance-matrix.md) | v1 feature status |

## Status

Experimental. The core path — compile ST/FBD/LD, run realtime or sim,
deploy maps, MQTT/HA, EtherCAT fleet — is e2e-tested; exotic services
compile in but vary in maturity (see the acceptance matrix).
