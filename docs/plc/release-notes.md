# TetherPLC release notes

## v1 (candidate)

First functional slice of the PLC subsystem: ST + FBD compile to
native ELF modules, deterministic scan on a sidecar runtime, online
change with state preservation, a full debug plane over ctl.sock,
live attach over tether_io, and the safety/security envelope.

### Feature surface

- **Compile**: `tplcc build prog.st -o prog.tplc` — ST and FBD lower
  into shared `tplc-mir`, LLVM emits host-arch `.so`; `.tplc`
  packages carry sectioned metadata (IO map, migrate descriptors,
  symbols, toolchain provenance).
- **Runtime**: `tether-plc --image IMG --package PKG --ctl SOCK` —
  cyclic tasks to 4 kHz measured, SHM process image, per-instance
  journal, optional `--authz`, `--hang-timeout-ms`, `--retain`.
- **Online change**: `stop→prepare→start` or hot `swap` with
  layout-checked state migration, predecessor lineage, quarantine
  release, `rollback`, `state-diff` rule reporting.
- **Debug**: resolve/read/write/force, boundary-consistent watch,
  suspend/resume/step, breakpoints + trigger expressions, flight-
  recorder trace, retain inspect — all role-gated and journaled.
- **Live attach**: `tplc-io --ctl SOCK --port 43770` serves the ctl
  surface as a real tether_io TCP catalog (38 functions, live
  status/blob signals, schema-epoch bridging on swap).
- **Security**: artifact signing + trust policy, post-link syscall
  scanner, in-process seccomp sandbox, SPDX license gate, durable
  audit journal.

### Measured (perf-evidence.md)

1002 runs/s at 1 kHz, 3974 at 4 kHz on a loaded desktop; swap adds
no measurable scan pause; watchdog stall→fault ≈ hang-timeout +
≤50 ms, unrecovered `_exit` at ~2x for supervisor respawn.

### Known limits

- **Not a safety layer.** Safety functions belong in certified
  components; the runtime enforces read-only/non-forceable safety
  channels as defense-in-depth, not as the boundary itself.
- **No cgroup/namespace/FS isolation** — in-process seccomp+rlimits
  only; the deploy unit must supply the envelope
  (field-trial.md has a reference unit).
- **tether_io attach is loopback-only.** TCP has no transport
  authZ; the provider's own ctl role covers all its clients. No
  per-remote-user identity until a WS-side gateway exists.
- **Debug plane gaps**: flight-recorder induced-fault capture is
  not asserted by tests; error-code coverage is 74/75 test-hit
  (see error-code-coverage.md).
- **Arithmetic fault policy is opt-in**: `tplcc --checked-arith`
  makes integer div/mod by zero a `TPLC_TASK_E_DIVZERO` (-19)
  task fault, int add/sub/mul/neg overflow and `INT_MIN / -1` a
  `TPLC_TASK_E_OVERFLOW` (-20) fault, and out-of-range conversions
  a `TPLC_TASK_E_RANGE` (-21) fault; the default keeps saturating,
  wrapping, and truncating. The `tplc_init` contract codes
  `INIT_LAYOUT`/`INIT_INVALID`/`INIT_VALUE` (-1..-3) remain
  declared-but-unemitted reservations.
- **`debug_stress` lateness flake**: under a 6-way ctl-flood on a
  loaded host, the 5 ms lateness bound occasionally trips at
  ~18 ms. Cadence holds; the bound assumes an unloaded host.
- **MCU target** (doc 18) is out of v1 scope.
