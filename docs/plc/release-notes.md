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
  symbols, toolchain provenance). `tplcc build --library` packages
  FUNCTION/FUNCTION_BLOCK collections as `"kind":"library"`
  artifacts (linkable POU object + ST sources, no program state);
  `--lib pkg.tplc` consumes them by source inclusion — the
  foundation for TwinCAT compatibility libraries
  (plan/21-twincat-libraries.md). Compat tranche under
  `compat/tc2/`: Tc2_Math (all APIs), Tc2_Standard W* + L* wrappers
  (complete), Tc2_Utilities BCD FBs + FB_EnumStringNumbers;
  consumed and verified end-to-end by `tests/compat/tc2.sh`
  (ctest `tc2_compat`). Second tranche adds `compat/tc2/tc2_tcpip.st`
  (Tc2_TcpIp socket FBs) over the new `tplc-net` runtime service —
  a `TplcNetService` vtable behind `TplcServices.get_service`,
  nonblocking POSIX sockets on the host, and a builtin op-multiplexed
  `FB_Socket` driving it; verified by a real loopback
  listen/connect/accept/send/receive/close exchange
  (ctest `tc2_tcpip`). Third tranche adds
  `compat/tc2/tc2_serialcom.st` (Tc2_SerialCom port FBs) over the
  `tplc-serial` service — a `TplcSerialService` vtable, nonblocking
  POSIX termios on the host, and a builtin `FB_Serial`; verified by
  a real pty-pair echo exchange plus bad-path error check
  (ctest `tc2_serial`). Fourth tranche adds
  `compat/tc2/tc2_system.st` (Tc2_System ADS function blocks) over
  the `tplc-ads` service — a `TplcAdsService` vtable, an AMS/TCP
  ADS client on the host speaking the Beckhoff wire format, and a
  builtin `FB_Ads`; verified by a real ADS exchange (connect, read,
  write, read-state, write-control, close, refused-port error) against
  a python ADS responder (ctest `tc2_system_ads`). Fifth tranche adds
  `compat/tc3/tc3_rtmon.st` (Tc3_RealtimeMonitoring mark FBs) over
  the `tplc-rtmon` service — a `TplcRtmonService` vtable, a
  4096-entry timestamped mark ring on the host clocked by
  `HostServices::steadyNs` (deterministic under `--sim`), and a
  builtin `FB_RtMon`; verified by an in-process mark/count/indexed-
  read/clear sequence (ctest `tc3_rtmon`). Sixth tranche adds
  `compat/tc2/tc2_modbussrv.st` (Tc2_ModbusSrv TCP client FBs) over
  the `tplc-modbus` service — a `TplcModbusService` vtable, a
  nonblocking Modbus TCP client on the host (MBAP/PDU, big-endian
  wire fields, connect-per-transaction, exception responses), and a
  builtin `FB_Modbus` with a 256-byte `DATA` array; verified by a
  real six-transaction TCP exchange plus a five-transaction UDP
  datagram exchange (read/write regs and coils, exception
  response, read/write) against a python Modbus TCP/UDP peer
  (ctest `tc2_modbussrv`). Service ABI v2 adds `mb_begin_udp`;
  `FB_Modbus.UDP` selects the transport. The tranche also fixed
  two latent bugs:
  FB-call argument binding stored literal-width values into
  narrower members (clobbering adjacent inputs), and the ELF
  sandbox x86 length decoder desynced on `0F 8x` jumps and
  `0x66`-prefixed immediates, rejecting valid modules as
  kernel-gate violations.
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
