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
  Twelfth tranche adds `compat/tc3/tc3_ftp.st`
  (Tc3_FTP + Tc2_FTP) over the new `tplc-ftp` service — a
  `TplcFtpService` vtable with connection, command, and
  transfer handle spaces, an RFC 959 FTP client over TCP 21
  with PASV passive data connections (line-based control
  channel with multi-line reply handling, TYPE I → PASV 227
  parse → data connect → RETR/STOR/LIST → 150 → stream →
  226), injected-clock timeouts, and builtins
  `FB_FtpConn`/`FB_FtpCmd`/`FB_FtpXfer` with poll-model
  wrappers `FB_FTP_Connect`/`FB_FTP_Cmd`/`FB_FTP_Download`/
  `FB_FTP_Upload`/`FB_FTP_List` plus MkDir/RemoveDir/
  DeleteFile/FileExists/DirExists/Rename method wrappers
  (RNFR+RNTO sequenced internally). The `tc3_ftp` ctest
  replays a 500-step free-running sim against
  `ftp_test_peer.py` — twelve packed checks (`0xfff`):
  login, SIZE, MKD, RETR content, LIST, STOR + verify,
  file/dir exists, rename, refused connect, close. TwinCAT's
  profile objects, `I_TcMessage`/EventLogger/`HRESULT`
  surfaces, and `FB_FTP_HostResolve` DNS are documented gaps;
  FTPS/TLS and SFTP/SSH are not implemented.
  Thirteenth tranche adds `compat/tc2/tc2_sms_sms.st`
  (Tc2_SMS + Tc2_SMTP) over the new `tplc-smtp` service and
  the existing `tplc-serial` service. `tplc-smtp` is an RFC
  5321 SMTP client over TCP with a single-handle transaction
  (connect → 220 → EHLO → optional AUTH LOGIN 334/334/235 →
  MAIL → RCPT → DATA dot-stuffed → 250 → QUIT), exposed as
  builtin `FB_SmtpSend` (STATUS 0..5, RESP_CODE keeps the
  DATA-accept 250 rather than the QUIT 221). `FB_SendSMS` is
  a pure-ST GSM AT-command state machine over the serial
  wrappers (AT → AT+CMGF=1 → AT+CMGS → ">" → text+Ctrl-Z →
  +CMGS/OK), with wrappers `FB_SmtpV3`/`FB_SmtpV3_Full`,
  obsolete `FB_Smtp`/`FB_SmtpFull` aliases, `FB_SmtpAttach`
  (ERRID 90 stub), and `FB_GetTcPlcSMS_Version`. The
  `tc2_sms_sms` ctest replays a 700-step free-running sim
  against `gsm_modem_peer.py` (pty AT modem) and
  `smtp_test_peer.py` (RFC 5321 on TCP 2525) — nine packed
  checks (`0x1ff`). Debugging surfaced and fixed a real
  string-codegen bug: `CONCAT`/`str_fit`/`fb_call` did all
  length math in the string element type, so capacities over
  255 truncated to zero and produced empty strings; math now
  runs in i32 with a final clamp. It also documented the
  serial EXEC contract: pulsed EXEC requires the FB be
  called every scan or `__last_exec` stays high and swallows
  the next edge. TLS, attachments, Cc/Bcc, multi-recipient,
  and the TwinCAT SMTP-server ADS proxy are documented gaps;
  SMS PDU/UCS2/concatenation/status modes are unimplemented.
  Fourteenth tranche adds `compat/tc3/tc3_iec870.st`
  (Tc2_IEC60870_5_10x) over the new `tplc-iec870` service, an
  IEC 60870-5-104 telecontrol client on TCP 2404. The service
  implements APCI I/S/U framing (`0x68`+len+4-octet control,
  15-bit sequence numbers shifted left one), the
  STARTDT/STOPDT/TESTFR handshake and TESTFR keepalive, and
  ASDU header build/parse with configurable 1-or-2-octet CA/COT
  widths and raw information-object payload passthrough (no
  per-type codecs). Builtin `FB_Iec870Conn` (session),
  `FB_Iec870Tx` (one ASDU per EXEC edge, k-window aware), and
  `FB_Iec870Rx` (consume-on-read, bounded queue) bridge to the
  host service; all ops are nonblocking on the injected host
  clock. The compat layer adds `FB_IEC870_5_104Master`/
  `FB_IEC870_SendAsdu`/`FB_IEC870_RecvAsdu` wrappers plus pure
  helpers: COT/TYPEID strings for IEC 101/102/103,
  `SYSTEMTIME_TO_CP56Time2a` and `CP56Time2a_TO_SYSTEMTIME`
  FUNCTION_BLOCKs, CP56 to CP24/CP32 truncation, IOA
  composition, SPI/DPI/VTI bit helpers, and
  `GetIec870_Version`. The `tc3_iec870` ctest replays a
  400-step free-running sim against `iec870_test_peer.py`, a
  python IEC 60870-5-104 slave on TCP 2480; twelve packed
  checks (`0xfff`) cover connect/STARTDT, interrogation,
  M_SP+M_ME receive, clock sync echo, single-command act-con,
  refused-port error, helpers, and STOPDT close. TwinCAT's
  AO-database/AMS-object/PVOID/interface/HRESULT model and the
  5-101 serial, 5-102 HDLC, and 5-103 relay-protection
  transports are documented gaps; this tranche is the 104 wire
  protocol plus pure helpers only.
- **Fifteenth tranche (C5 motion backend)**: adds the
  `tplc-motion` service (`include/tplc/tplc_motion.h`, host
  impl `runtime/host/src/MotionService.cpp`), a bounded pool of
  deterministic simulated CiA402/DS402-shaped axes for the
  PLCopen MC2 function blocks. Each axis exposes the DS402
  object subset the MC layer needs (Controlword 0x6040,
  Statusword 0x6041, ModesOfOperation 0x6060/Display 0x6061,
  TargetPosition 0x607A, TargetVelocity 0x60FF, TargetTorque
  0x6071, Position/Velocity/TorqueActualValue, FollowingError
  Window, error code) through `motion_axis_create` /
  `motion_axis_destroy` / `motion_axis_poll` /
  `motion_axis_info` / `motion_obj_read` / `motion_obj_write`.
  The axis runs the CiA402 controlword-command state machine
  (EnableVoltage/QuickStop/SwitchOn/EnableOperation nibble +
  FaultReset) producing the standard statusword patterns, and
  integrates profile-position/velocity/torque and cyclic-sync
  modes toward their targets under configurable limits on the
  injected `HostServices::steadyNs` clock, so `--sim` steps
  replay exactly. Builtin `FB_MotionAxis` is a synchronous
  op-dispatch bridge (OP 1=create, 2=destroy, 3=poll, 4=read,
  5=write, 6=info) with no EXEC edge; the compat layer gates
  create/destroy timing. The `tc2_motion` ctest replays a
  500-step sim driving create, a PP-position move to
  target-reached, velocity mode, bad-object/bad-mode/bad-handle
  error paths, and destroy; twelve packed checks (`0xfff`).
  This tranche is the axis backend and builtin only; a
  production backend can map the same object dictionary onto
  Tether's DS402/EtherCAT drives (DECISIONS D-IMPL-52).
- **Sixteenth tranche (C5 MC2 axis-management wrappers)**: adds
  `compat/tc2/tc2_mc2.st`, the first PLCopen MC2 function blocks
  over `FB_MotionAxis`: `MC_Power` (level-driven controlword
  enable, `Status` from OperationEnabled), `MC_Reset`
  (edge-triggered fault-reset pulse that releases the next scan
  and leaves the controlword to `MC_Power` otherwise),
  `MC_SetPosition` (edge-triggered absolute/relative
  PositionActualValue write), `MC_ReadActualPosition` /
  `MC_ReadActualVelocity` / `MC_ReadAxisError` (cyclic object
  reads), and `MC_ReadStatus` (DS402 state to PLCopen
  operating-state map). `AXIS_REF` is a UDINT axis handle and
  positions are int32 user units, both documented
  simplifications. `MC_SetPosition` writes only on its execute
  edge so it neither fights `MC_Power` nor re-applies a stale
  `Position` input. The `tc2_mc2` ctest builds the library,
  consumes it via `--lib`, and replays a 300-step sim; ten
  packed checks (`0x3ff`). The motion-command blocks
  (`MC_Move*`/`MC_Stop`/`MC_Halt`/`MC_Home`/`MC_Jog`), the full
  PLCopen state machine, and gearing/phasing/superposition land
  in later tranches.
- **Seventeenth tranche (C5 Tether-IO motion backend)**: adds
  `MotionIoService`, a `TplcMotionService` implementation that
  maps the DS402 object dictionary onto the Tether IO protocol.
  `HostServices` selects it when `TPLC_MOTION_IO` names a
  reachable `tether_io_motion_server` endpoint and otherwise
  falls back to the in-process `MotionService`. Object reads and
  writes become `getParam`/`setParam` on the `mc.<slave>.<index>`
  params the server exposes for its `SimulatedCiA402Drive` fleet
  (Tether `include/tether/io/SimulatedCiA402Drive.hpp`, an
  `IMachineSdoAccess` backend running the CiA402 state machine
  and PP/HM/VL/PV/TQ/CSP/CSV/CST dynamics). `tether_io_motion_server
  --clock sim` advances the drives on a writable U64 `mc.time`
  param; the service writes the host steady-clock timestamp
  before each op so `tether-plc-sim --sim` replay stays
  deterministic, and `--clock realtime` serves live PLCs. The
  `svc_` table is the first member so the rt bridge's `&svc_`
  pointer recovers the object (a mis-ordering segfaulted).
  `FB_MotionAxis` and the MC2 wrappers are unchanged; the new
  `tc2_motion_io` ctest replays `tc2_motion_case.st` (mask
  `0xfff`) over the wire, and `tc2_mc2` passes on both backends.
  `machine.sdo.*` stays the commissioning surface; cyclic motion
  uses params because SDO writes are rate-limited. Axis dynamics
  config is validated but not propagated to the remote drive, and
  wire write errors cannot propagate so the client pre-validates
  mode-of-operation (DECISIONS D-IMPL-53).
- **Eighteenth tranche (C5 MC2 command FBs)**: adds the PLCopen
  motion-command blocks to `compat/tc2/tc2_mc2.st` —
  `MC_MoveAbsolute`/`MC_MoveRelative` (ProfilePosition +
  NewSetPoint), `MC_MoveVelocity`/`MC_Halt`/`MC_Jog` (ProfileVelocity),
  `MC_Stop` (QuickStop controlword), and `MC_Home` (Homing mode).
  `MC_Power` is now edge-triggered so the command FBs can layer
  controlword command bits without being overwritten every scan;
  each command writes the controlword only on its `Execute` edge
  (plus the single NSP release scan) so a later `MC_Stop`/`MC_Power`
  transition can override a running command and `MC_Power` disable
  is not fought. The FBs expose the standard PLCopen outputs
  (`Done`/`Busy`/`InVelocity`/`CommandAborted`/`Error`/`ErrorID`) and
  drive both the in-process `MotionService` and the Tether-IO
  `MotionIoService` unchanged. Positions/velocities stay int32 user
  units (TwinCAT LREAL is a documented gap) and per-command
  dynamics inputs are signature-compatible only — the axis applies
  its configured limits. New `tc2_mc2_motion` (in-process) and
  `tc2_mc2_motion_io` (Tether IO) ctests replay
  `tc2_mc2_motion_case.st` (mask `0x1fff`): power, abs+rel move with
  position readback, velocity InVelocity, quick-stop lock, halt,
  homing, jog, and power-off — identical results on both backends.
  Gearing/phasing/superposition remain for a later tranche.
- **Nineteenth tranche (C5 MC2 modulo/override/params + PLCopen
  state)**: extends `compat/tc2/tc2_mc2.st` with `MC_MoveModulo`
  (shortest / positive / negative direction modulo positioning over
  `ModuloFactor`), `MC_SetOverride` (velocity-override percent that
  scales the `0x60FF` target for velocity modes and restores the
  latched base on release), the axis-parameter accessors
  `MC_ReadParameter`/`MC_WriteParameter`/`MC_ReadBoolParameter`/
  `MC_WriteBoolParameter` (ParameterNumber is the DS402 object index,
  giving generic object access through the PLCopen signature), and a
  fuller PLCopen operating-state machine in `MC_ReadStatus`
  (0=Disabled, 1=Standstill, 2=Homing, 3=DiscreteMotion,
  4=ContinuousMotion, 6=Stopping, 7=ErrorStop; 5=SynchronizedMotion
  reserved for gearing/camming). New `tc2_mc2_modulo` (in-process)
  and `tc2_mc2_modulo_io` (Tether IO) ctests replay
  `tc2_mc2_modulo_case.st` (mask `0xfff`) on both backends. Gearing,
  phasing, camming and hardware touch-probe latch need multi-axis
  coupling / latch objects the simulated backend does not expose and
  remain documented gaps (DECISIONS D-IMPL-54).
- **Twentieth tranche (C5 MC2 coupling + touch probe)**: extends
  `compat/tc2/tc2_mc2.st` with the two-axis and latch blocks as honest
  software approximations over the existing object ops — no new ABI.
  `MC_TouchProbe` arms/rearms on `Execute` and captures
  `PositionActualValue` 0x6064 on the `TriggerInput` rising edge with
  optional `WindowOnly`/`FirstPosition`/`LastPosition` gating; it is a
  cycle-level software latch, not TwinCAT's sub-cycle hardware
  encoder latch (the simulated drive exposes no latch object, so
  capture latency is one scan). `MC_GearIn` couples a slave to a
  master by velocity ratio — enable slave + ProfileVelocity on
  `Execute`, then each scan write slave `TargetVelocity` 0x60FF =
  master `VelocityActualValue` 0x606C * `RatioNumerator` /
  `RatioDenominator` (zero denominator -> Error). `MC_GearOut` writes
  slave 0x60FF = 0 on its edge and reports `Done` at rest. This is
  velocity-ratio coupling, not exact phase lock — true TwinCAT
  gearing needs cyclic setpoint coupling. `MC_AbortTrigger` is
  intentionally omitted (no shared per-axis probe state; see
  DECISIONS D-IMPL-55). New `tc2_mc2_gear` (in-process) and
  `tc2_mc2_gear_io` (Tether IO) ctests replay
  `tc2_mc2_gear_case.st` (mask `0x1ff`) on both backends — two axes,
  both powered, master velocity, latch in window, 2:1 coupling,
  gear-out, halt, power-off. Exact hardware-latch touch-probe,
  `MC_AbortTrigger`, and phase-locked gearing/phasing/camming remain
  documented gaps needing Tether backend latch + coupling objects.
- **Twenty-first tranche (C6 NC-I channel interpolation)**: adds
  `compat/tc2/tc2_nci.st`, the NC interpolation core. TwinCAT's NC-I
  runs a G-code/FIFO interpreter in the NC kernel emitting per-axis
  CSP setpoints each cycle; the compat model collapses one geometric
  block into `FB_NciLine`, which parameterizes a straight line over a
  3-axis group, advances the path parameter by `Velocity*CycleUs`, and
  streams per-axis targets (0x607A) to axes in CSP mode 8 — the same
  mechanism the NC kernel uses. `F_NciSqrt` (Newton sqrt, no SQRT
  builtin) computes the Euclidean path length; `FB_NciEStop` cuts
  power to the group. The caller supplies `CycleUs` as the time base.
  Verified `tc2_nci` + `tc2_nci_io` (mask `0xff`) on both backends.
  Implementation surfaced and fixed a real `tether_io_motion_server`
  bug: per-object `readFn`/`writeFn` captured `slave` by reference
  (`[&, idx, sz]` lets it fall under the `&` default), so every param
  hit the last-configured slave — invisible for single-axis, breaking
  multi-axis groups. G-code interpreter, FIFO queue, circular/helical/
  spline, look-ahead, M-functions, R-params remain gaps needing a real
  NC channel object in the backend (D-IMPL-56).
- **Twenty-second tranche (C6 Tether-owned G-code channel)**: adds
  the `nc.*` interpreter channel. `tether_io_motion_server` compiles
  `nc.prog` text or `nc.prog_path` files with
  `GCode::PlanningSegmentBuilder`, time-parameterizes with
  `MotionPlanner::MotionPlanBuilder` (corner blending, velocity
  profile, linear/arc/dwell segments, zero-length moves skipped with
  source-line mapping preserved), and streams scaled CSP `0x607A`
  setpoints to slaves 0..2 on the `mc.time` clock. Commands
  run/step/pause/resume/stop/reset/verify with InterpreterState
  states; `nc.state`/`line`/`total`/`errcode`/`progress`/`duration`/
  `scale`/`feedovr` report channel status. PLC side: `tplc-nci`
  vtable + `NciIoService` (active only with a Tether endpoint; no
  in-process interpreter by design), `FB_NciChan` builtin, and
  `FB_ItpLoadText`/`ItpLoadProg`/`ItpStartStop`/`ItpReset`/`ItpEStop`/
  `ItpStatus` wrappers in `tc2_nci.st`. Verified `tc2_nci_gcode_io`
  (mask `0x3fff`) — text upload, file load, run, line tracking,
  finish, duration, stop, reset; plus direct-probe step/pause/
  resume. Remaining gaps: FIFO queue, M-function handshake,
  R-params/zero-shift, cyclic-channel offsets (D-IMPL-57).
- **NCI FIFO + M-func + R-params (tranche 23)**: the `nc.*` channel
  grew a FIFO block queue (`nc.fifo` append, trial-compiled, chained
  continuation plans anchored at the last commanded position, runs
  FIFO-only from Idle; `nc.fifolen`/`nc.fifoseq` status), an
  M-function handshake (channel pauses on M-codes outside the
  program-control set with `nc.mfunc` pending; `nc.confirm` resumes),
  R-parameters (`nc.ridx`/`nc.rval` injected as `#n = v` prefix
  lines into `VariableSystem`), and zero-shift (`nc.zsx/y/z` as a
  `G92` prefix); prefixes are stripped from visible block/line
  numbering. `PlanningSegment.blockIndex` now maps segments to real
  source blocks via `mseg.lineNumber`, fixing `nc.line` when one
  block emits several segments. PLC side: `tplc-nci` gains
  `nci_fifo`/`nci_confirm`/`nci_rparam`/`nci_zshift` (status struct
  carves `mfunc`/`fifolen`/`fifoseq` out of `reserved`, 64-byte ABI
  unchanged), `FB_NciChan` ops 5-9, and `FB_ItpFiFoWrite`/
  `FB_ItpConfirmHsk`/`FB_ItpWriteRParam`/`FB_ItpReadRParam`/
  `FB_ItpZeroShift` wrappers; `FB_ItpStatus` exposes
  `MFunc`/`FifoLen`/`FifoSeq`. `tc2_nci_gcode_io` now verifies the
  full surface at mask `0x7fffff` (D-IMPL-58). Cyclic-channel
  offset tables remain a gap.
- **OPC UA client (C7 tranche 1, tranche 24)**: `tplc-opcua` is the
  PLC-side OPC UA client via **libopen62541** (opt-in like the other
  special features — `-DTPLC_ENABLE_OPCUA`, auto-detected, inert +
  `getService` nullptr when `libopen62541-dev` is absent). `FB_UaClient`
  ops 1-6 (connect/disconnect/status/read/write/ns-index) +
  `FB_UA_Connect`/`FB_UA_Disconnect`/`FB_UA_ConnectGetStatus`/`FB_UA_Read`/
  `FB_UA_Write`/`FB_UA_GetNamespaceIndex` wrappers in
  `compat/tc2/tc2_opcua.st` (TwinCAT `Tc3_PlcOpenOpcUa` shape; node ids
  as text `ns=2;s=name`/`i=85`, numerics widen into `LREAL`, strings
  into `STRING[255]`). The endpoint for the sim path is Tether's
  `tether_io_motion_server --opcua` — an open62541 server folding the
  motion Registry into `ns=2;s=<name>` nodes. `tc2_opcua_io` verifies
  connect/session/read/write/ns-index/disconnect end-to-end
  (D-IMPL-59). Browse/node-handle/subscription/method wrappers remain
  gaps; EtherNet/IP, IEC 61850, MQTT/IoT, OCPP, AES70, building buses
  are the rest of wave 2.
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
- **EtherNet/IP client (C7 tranche 2, tranche 25)**: `tplc-eip` is a
  scanner-side explicit-messaging client — RegisterSession/
  UnRegisterSession, ListIdentity discovery, and SendRRData carrying
  unconnected CIP Get/Set_Attribute_Single on TCP 44818, all on
  nonblocking connect+poll-bounded IO (fails fast, never stalls the
  scan). `FB_Eip` ops 1-6 (open/close/status/listid/get/set) +
  `FB_EipConnect`/`FB_EipDisconnect`/`FB_EipStatus`/`FB_EipListIdentity`/
  `FB_EipGetAttribute`/`FB_EipSetAttribute` wrappers in
  `compat/tc2/tc2_eip.st` (TwinCAT Tc3_EtherNetIP/TF6281 shape; CIP
  class/instance/attribute u16 with auto 8/16-bit segments; attribute
  payloads bounded STRING[255]). `tc2_eip_io` verifies
  register/identity/attr r-w/disconnect at `0x7f` against
  `eip_adapter_fake.py` (D-IMPL-60). Cyclic class-1 I/O, ForwardOpen,
  other CIP services, and UDP multicast discovery are gaps; IEC 61850,
  MQTT/IoT, OCPP, AES70, building buses are the rest of wave 2.

### C7 tranche 3 — IEC 61850 MMS client (`tplc-iec61850`)

  `tplc-iec61850` is an IEC 61850 MMS client — the PLC is the
  control-center side over the full OSI stack via **libiec61850**
  (`IedConnection_*`), opt-in like the other special features:
  `-DTPLC_ENABLE_IEC61850` (default ON) auto-downloads and builds
  libiec61850 via CMake FetchContent; absent or off → the service
  compiles inert and `getService("tplc-iec61850")` returns nullptr.
  `FB_Iec61850` is the runtime builtin (connect+associate,
  close+release, status, read, write, GetNameList) and
  `FB_Iec61850Connect`/`Disconnect`/`Status`/`Read`/`Write`/
  `GetNameList` in `compat/tc2/tc2_iec61850.st` are the
  TwinCAT-shaped wrappers.  Objects are addressed by domain
  (logical device) + item (VariableAccess name) with an `#FC`
  suffix for the functional constraint; values travel as bounded
  `STRING[255]`.  Blocking calls are bounded
  (`setConnectTimeout`/`setRequestTimeout` ≈800 ms) under the
  scan-stall watchdog.  `tc2_iec61850_io` verifies
  associate/status/read/write-error/namelist/disconnect at `0x7f`
  against `iec61850_ied_fake` (a real libiec61850 `IedServer` with a
  `tetherIEDtetherTestLD`/`LLN0` CDC model).  Reporting/RCB,
  GOOSE/SMV, control sequences, and the full data model are gaps.

### C7 tranche 4 — MQTT/IoT client (`tplc-mqtt`)

  `tplc-mqtt` is an MQTT 3.1.1 client — the PLC is the client over
  the broker wire via **libpaho-mqtt3c** (Eclipse Paho synchronous
  `MQTTClient`), opt-in like the other special features:
  `-DTPLC_ENABLE_MQTT` (default ON) prefers a system libpaho-mqtt3c
  and falls back to a CMake FetchContent build of paho.mqtt.c;
  absent or off → `getService` nullptr.  `FB_Mqtt` is the runtime
  builtin (connect/disconnect/status/publish/subscribe/unsubscribe/
  receive/isConnected) and `FB_IotMqttClient`/`Disconnect`/`Publish`/
  `Subscribe`/`Unsubscribe`/`Receive` in `compat/tc2/tc2_mqtt.st` are
  the TwinCAT-shaped wrappers (Tc3_IoT/TF6701).  Topics bounded
  `STRING[127]`, payloads `STRING[255]`; `Retain`/`RETAIN` are
  reserved keywords surfaced as `RetainOn`/`FLG_RETAIN`.  Receive is a
  bounded nonblocking poll (NOMSG when empty) so the scan can't
  stall.  `tc2_mqtt_io` verifies connect/subscribe/publish/echo/
  disconnect at `0x1f` against `mqtt_broker_fake.py`.  MQTT5
  properties, wildcard detail, QoS2, retained messages, and TLS are
  gaps.

### C7 tranche 5 — OCPP-J client (`tplc-ocpp`)

  `tplc-ocpp` is an OCPP-J (Open Charge Point Protocol over
  WebSocket + JSON-RPC) charge-point client — the PLC is the
  charge-point side, always available (TCP + RFC6455 WebSocket +
  OCPP JSON-RPC array framing, hand-rolled like tplc-eip).
  `FB_Ocpp` is the runtime builtin (connect/disconnect/status/
  call/result/callerror/receive) and `FB_OcppConnect`/`Disconnect`/
  `Call`/`Result`/`CallError`/`Receive` in `compat/tc2/tc2_ocpp.st`
  are the TwinCAT-shaped wrappers (Tc3_OCPP/TF6741-ish).  Payloads
  are raw JSON text bounded `STRING[255]`; `ACTION` is a reserved
  keyword surfaced as `ACT`/`ActName`.  Every socket op is
  poll-bounded under the scan-stall watchdog.  `tc2_ocpp_io`
  verifies WS-upgrade/call/result/disconnect at `0xf` against
  `ocpp_csms_fake.py`.  OCPP 2.x, security profiles, and schema
  validation are gaps.
