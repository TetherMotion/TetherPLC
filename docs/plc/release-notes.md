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
  kernel-gate violations. Seventh tranche adds
  `compat/tc2/tc2_modbusrtu.st` (Tc2_ModbusRTU V2-master FBs) over
  the same `tplc-modbus` service — service ABI v3 adds
  `mb_begin_rtu`/`TplcModbusRtuReq`: the serial device opens per
  transaction with termios settings from the request, frames are
  `unit|func|data|crc16` (CRC-16/Modbus), responses are
  unit/CRC/func-validated and parsed by the shared PDU parser,
  and variable-length functions complete on the T3.5 inter-frame
  gap; builtin `FB_ModbusRtu` mirrors `FB_Modbus` with serial-line
  inputs. Verified by a six-transaction RTU exchange against a
  python RTU slave on a pty pair (ctest `tc2_modbusrtu`, mask
  `0x3f`). One transport bug was caught by the e2e: a 0-byte
  `read()` under `VMIN=0`/`VTIME=0` means "no data this scan", not
  EOF — treating it as hangup desynced every transaction. Eighth
  tranche adds `compat/tc3/tc3_sacn.st` (Tc3_sACN) over the new
  `tplc-sacn` service — a `TplcSacnService` vtable, an E1.31-2018
  codec (root/framing/DMP layers verified byte-identical against
  two independent reference implementations), nonblocking UDP
  multicast sockets for source TX and receiver RX on the
  universe-derived `239.255.x.y` group, sACN priority filtering
  (lower value wins; worse-priority packets counted and dropped),
  and builtins `FB_SacnSource`/`FB_SacnReceiver`; the library also
  provides the three pure-IEC helpers `F_HighLowArr_TO_UINT`,
  `F_UNIVERSE_TO_MulticastAdr`, `FB_UINT_TO_HighLowArr`. Verified
  by a real multicast exchange: a python sACN peer waits for the
  DUT's own SRC1 stream, then injects priority-10 HIPRI (accepted,
  preempts) and priority-200 LOWPR (dropped) packets — four packed
  checks (`0x0f`, ctest `tc3_sacn`). The tranche fixed three
  latent codegen bugs the e2e exposed: narrow-typed `:=` literals
  stored at literal width (a `BYTE`/`WORD` store clobbered up to
  three neighboring state bytes — now coerced to the destination
  type in `write`, `lit_store`, FOR-init, and call-arg binding);
  aggregate array actuals bound through `read()` copied only
  element 0 (now `llvm.memcpy` of the bounded aggregate size in
  FB/FUNCTION arg binding and whole-array assignment); and the
  receiver's priority comparison was inverted (it rejected
  better-priority packets and accepted worse ones). Ninth
  tranche adds `compat/tc3/tc3_bacnet.st` (Tc3_BACnetRev14) over
  the new `tplc-bacnet` service — a `TplcBacnetService` vtable,
  a BACnet/IP (ASHRAE 135) BVLC+NPDU+APDU codec over UDP 47808
  (verified against bacpypes3 encodings), Who-Is/I-Am discovery
  with device-instance range and an 8-responder table,
  confirmed ReadProperty (Complex-ACK value decode) and
  WriteProperty (Simple-ACK), raw application-tag value payloads,
  injected-clock timeouts, and builtin `FB_Bacnet` with poll-model
  wrappers `FB_BACnet_WhoIs`/`FB_BACnet_ReadProperty`/
  `FB_BACnet_WriteProperty`. Verified by a real UDP exchange
  against a python BACnet/IP peer — six packed checks (`0x3f`,
  ctest `tc3_bacnet`): broadcast and directed Who-Is, REAL and
  characterString reads, write ACK, and BACnet Error handling.
  TwinCAT's BACnet FBs are device-config-generated, so the compat
  surface is the protocol layer; COV/events/segmentation/router
  support remain documented gaps. Tenth tranche adds
  `compat/tc3/tc3_pjlink.st` (Tc3_PJLink) over the new
  `tplc-pjlink` service — a `TplcPjlinkService` vtable, a JBMIA
  PJLink projector client over TCP 4352 (`%<class><CMD4>
  <param>\r` commands, `%<class><CMD4>=<value>\r` responses),
  `PJLINK 0`/`PJLINK 1 <nonce>` greeting handling with
  `MD5(nonce+password)` digest auth (compact in-tree MD5
  verified against the spec's worked example), `PJLINK ERRA`
  auth-failure and `ERR1`–`ERR4` command-error surfacing,
  injected-clock timeouts, and builtin `FB_Pjlink` with
  poll-model wrappers `FB_PJLink`/`FB_PJLink_Power`/
  `FB_PJLink_Input`/`FB_PJLink_Mute`. Verified by a real TCP
  exchange against a python PJLink peer — seven packed checks
  (`0x7f`, ctest `tc3_pjlink`): unauthenticated POWR query,
  wrapper transactions, MD5 auth accept/reject, and command
  error handling. TwinCAT's auto-managed `FB_PJLink` object
  (keep-alive, status polling, AMS NetID, I_TcMessage,
  EventLogger) is a documented incompatibility; the compat
  surface is the protocol layer with connect-per-command.
  Eleventh tranche adds `compat/tc3/tc3_s7comm.st`
  (Tc3_S7Comm + Tc2_S5S7Com) over the new `tplc-s7` service —
  a `TplcS7Service` vtable with separate connection and io
  handle spaces, a Siemens S7comm client over RFC1006
  ISO-on-TCP (TPKT + COTP CR/CC/DT + S7comm, TCP 102) with a
  persistent COTP session, Read Var / Write Var jobs
  (func 0x04/0x05), automatic Setup Communication on connect,
  S7-Any single-item addressing across PE/PA/MK/DB/CT/TM with
  byte and bit access, injected-clock timeouts, and builtins
  `FB_S7Conn`/`FB_S7Io` with poll-model wrappers
  `FB_S7_Connect`/`FB_S7_Read`/`FB_S7_Write`/
  `FB_S7_ReadBit`/`FB_S7_WriteBit`. The codec was verified
  byte-exact against the python-snap7 reference server; the
  `tc3_s7comm` ctest replays a 300-step free-running sim
  against `s7_test_peer.py` — nine packed checks (`0x1ff`):
  connect, DB read, write+readback, MK bit read/write,
  unregistered-area s7err, refused connect, session close.
  TwinCAT's request-queue model (`AddRequest`, cyclic
  requests, `I_S7CommRequest`, `PVOID`/`HRESULT`, AMS
  routing) is adapted to explicit poll-model FBs, and the S5
  (3964/PKW) protocol behind `FB_NLink*` on S5 targets is a
  documented gap.
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
