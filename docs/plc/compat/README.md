# TwinCAT compatibility layer

ST-source compatibility implementations of Beckhoff TwinCAT 3 PLC
libraries, packaged as `kind="library"` `.tplc` artifacts (plan 21,
C0/C1). Each library mirrors the documented API surface closely enough
for TwinCAT ST sources to compile and run unchanged where the
underlying semantics allow.

Per-library API references (parameters, descriptions, recursive
catalog) for the libraries still awaiting implementation live in
`twincat/` — generated from `docs/plc/plan/21-twincat-libraries.md`
by `tools/twincat-docs/split_plan.py`.

## Layout

```
compat/tc2/           TwinCAT 2/3-era library sources
  tc2_math.st         Tc2_Math: CEIL FLOOR FRAC LTRUNC LMOD MODABS
                      MODTURNS F_GetVersionTcMath F_TruncL (helper)
  tc2_util_bcd.st     Tc2_Utilities subset: DEC_TO_BCD, BCD_TO_DEC
  tc2_util_enum.st    Tc2_Utilities: FB_EnumStringNumbers + F_IsDigit
  tc2_standard_wide.st Tc2_Standard W* wrappers (WCONCAT WDELETE
                      WFIND WINSERT WLEFT WLEN WMID WREPLACE WRIGHT)
  tc2_standard_lt.st  Tc2_Standard L* timers: LTON LTOF LTP wrapping
                      the TON/TOF/TP builtins (LTIME == TIME repr)
  tc2_tcpip.st        Tc2_TcpIp wrappers over the builtin FB_Socket
                      (tplc-net service): FB_SocketConnect/Listen/
                      Accept/Send/Receive/Close + UdpCreate/
                      UdpSendTo/UdpReceiveFrom
  tc2_serialcom.st    Tc2_SerialCom wrappers over the builtin
                      FB_Serial (tplc-serial service): FB_SerialOpen/
                      Send/Receive/Close/Flush/Rs485
  tc2_system.st       Tc2_System ADS wrappers over the builtin
                      FB_Ads (tplc-ads service): FB_AdsConnect/
                      Read/Write/ReadState/WriteControl/Close
  tc2_modbussrv.st    Tc2_ModbusSrv wrappers over the builtin
                      FB_Modbus (tplc-modbus service): the ten
                      TCP client FBs FB_MBReadCoils/ReadInputs/
                      ReadRegs/ReadInputRegs/WriteSingleCoil/
                      WriteSingleReg/WriteCoils/WriteRegs/
                      ReadWriteRegs + FB_MBDiagnose, plus the ten
                      FB_MBUdp* UDP variants
  tc2_modbusrtu.st   Tc2_ModbusRTU wrappers over the builtin
                      FB_ModbusRtu (tplc-modbus RTU transport):
                      the ten master FBs FB_MBRtuReadCoils/
                      ReadInputs/ReadRegs/ReadInputRegs/
                      WriteSingleCoil/WriteSingleReg/WriteCoils/
                      WriteRegs/ReadWriteRegs + FB_MBRtuDiagnose
compat/tc3/
  tc3_rtmon.st        Tc3_RealtimeMonitoring wrappers over the
                      builtin FB_RtMon (tplc-rtmon service):
                      FB_RTMon_LogMarkBase/Ex + the eight
                      FB_RTMon_Log* simple-mark FBs
  tc3_sacn.st         Tc3_sACN over the builtin FB_SacnSource +
                      FB_SacnReceiver (tplc-sacn service): E1.31
                      data/discovery/sync codec, multicast TX/RX,
                      priority filtering; plus the three pure-IEC
                      helpers F_HighLowArr_TO_UINT,
                      F_UNIVERSE_TO_MulticastAdr,
                      FB_UINT_TO_HighLowArr
  tc3_bacnet.st       Tc3_BACnetRev14 over the builtin FB_Bacnet
                      (tplc-bacnet service): BACnet/IP Who-Is /
                      ReadProperty / WriteProperty wrappers
                      (FB_BACnet_WhoIs, FB_BACnet_ReadProperty,
                      FB_BACnet_WriteProperty); TwinCAT's BACnet
                      FBs are device-config-generated, so these
                      poll-model wrappers are the compat surface
tests/compat/
  tc2_case.st         consumer program (28 packed checks)
  tc2.sh              ctest tc2_compat: build libs, --lib consume,
                      6-step sim replay expecting masks 0x1FFFFF/0xFF
  tc2_tcpip_case.st   autonomous loopback client/server program
                      (7 packed checks)
  tc2_tcpip.sh        ctest tc2_tcpip: library + consumer + 37-step
                      sim replay expecting mask 0x7f
  tc2_serial_case.st  pty loopback program (8 packed checks, device
                      path substituted by the script)
  tc2_serial.sh       ctest tc2_serial: library + consumer + 37-step
                      sim replay expecting mask 0xff, python pty
                      echo peer
  tc2_system_ads_case.st  autonomous ADS exchange program (8 packed
                      checks: connect/read/write/read-state/
                      write-control/close + refused-port error leg)
  tc2_system_ads.sh   ctest tc2_system_ads: library + consumer +
                      37-step sim replay expecting mask 0xff, python
                      ADS/AMS-TCP peer (ads_test_server.py)
  ads_test_server.py  minimal Beckhoff-wire ADS responder used by
                      tc2_system_ads.sh
  tc3_rtmon_case.st   autonomous mark sequence program (10 packed
                      checks: 4 marks, count, 2 indexed reads,
                      out-of-range, clear, recount)
  tc3_rtmon.sh        ctest tc3_rtmon: library + consumer + 14-step
                      sim replay expecting mask 0x3ff, in-process
                      tplc-rtmon service (no external peer)
  tc2_modbussrv_case.st  autonomous Modbus TCP sequence program
                      (6 packed checks: read regs, write single
                      reg, write regs, read coils, exception read,
                      read/write regs)
  tc2_modbussrv_udp_case.st  autonomous Modbus UDP sequence
                      program (5 packed checks on the datagram
                      transport)
  tc2_modbussrv.sh    ctest tc2_modbussrv: library + both consumers
                      + TCP (mask 0x3f) and UDP (mask 0x1f) sim
                      replays, python Modbus TCP/UDP peer
                      (modbus_test_server.py)
  modbus_test_server.py  minimal Modbus TCP/UDP responder used by
                      tc2_modbussrv.sh
  tc2_modbusrtu_case.st  autonomous Modbus RTU sequence program
                      (6 packed checks: read regs, write single
                      reg, write regs, read coils, exception read,
                      read/write regs)
  tc2_modbusrtu.sh   ctest tc2_modbusrtu: library + consumer +
                      38-step sim replay expecting mask 0x3f,
                      python Modbus RTU slave on a pty pair
                      (modbus_rtu_test_server.py)
  modbus_rtu_test_server.py  minimal Modbus RTU slave used by
                      tc2_modbusrtu.sh
  tc3_sacn_case.st    autonomous sACN program (4 packed checks:
                      own-source receive, HIPRI preemption, drop
                      accounting, helper conversions)
  tc3_sacn.sh         ctest tc3_sacn: library + consumer + 600-step
                      sim replay expecting mask 0x0f, python sACN
                      peer (sacn_test_peer.py)
  sacn_test_peer.py   E1.31 data-packet peer used by tc3_sacn.sh:
                      waits for the DUT's own SRC1 stream, then
                      injects priority-10 HIPRI and priority-200
                      LOWPR packets on a wall-clock schedule
  tc3_bacnet_case.st  autonomous BACnet/IP program (6 packed
                      checks: broadcast Who-Is/I-Am with range,
                      ReadProperty REAL, WriteProperty Simple-ACK,
                      object-name charstring, Error on unknown
                      object, directed unicast Who-Is)
  tc3_bacnet.sh       ctest tc3_bacnet: library + consumer +
                      350-step sim replay expecting mask 0x3f,
                      python BACnet/IP peer (bacnet_test_peer.py)
  bacnet_test_peer.py  minimal BACnet/IP (ASHRAE 135) responder
                      on UDP 47808 used by tc3_bacnet.sh: I-Am,
                      Complex-ACK (real/unsigned/charstring),
                      Simple-ACK, Error
```

## Building and consuming

```sh
tplcc build --library --name Tc2_Math compat/tc2/tc2_math.st \
    -o Tc2_Math.tplc
tplcc build my_program.st \
    --lib Tc2_Math.tplc --lib Tc2_Utilities_BCD.tplc \
    -o my_program.tplc
```

`--lib` extracts the package's `ST_SOURCES` into the compilation
unit, so library POUs resolve as ordinary user functions/FBs.
Binary (precompiled object) consumption is not wired yet; the
library object keeps relocatable `__tplc_pou_*` symbols for it.

## Status vs. plan 21 phases

| Phase | Scope | Status |
|-------|-------|--------|
| C0 | Library package format | done (`f912616`) |
| C1 | T0 core | tranches 1+2 done: Tc2_Math (8/8), Tc2_Standard W* (9/9) + L* timers (3/3), Tc2_Utilities BCD pair + FB_EnumStringNumbers |
| C2 | T1 services I | `tplc-net` done (FB_Socket + POSIX sockets + Tc2_TcpIp, loopback e2e); `tplc-serial` done (FB_Serial + termios + Tc2_SerialCom, pty e2e); `tplc-ads` done (FB_Ads + AMS/TCP ADS client + Tc2_System ADS FBs, wire-protocol e2e) |
| C3 | T1 services II | `tplc-rtmon` done (FB_RtMon + Tc3_RealtimeMonitoring, in-process e2e); Tc3_DynamicMemory blocked (needs PVOID/REFERENCE TO PVOID); Tc3_Module blocked (TwinCAT COM object model); Tc3_JsonXml blocked (METHOD/interface/POINTER/REFERENCE/HRESULT pervade all 8 FBs); Tc3_EventLogger blocked (I_Tc* interfaces, methods, HRESULT, GUID type) |
| C4 | Connectivity wave 1 | `tplc-modbus` done (Tc2_ModbusSrv 20/20 TCP+UDP + Tc2_ModbusRTU V2-master subset, pty e2e); `tplc-sacn` done (Tc3_sACN source/receiver + pure-IEC helpers, multicast e2e); `tplc-bacnet` done (BACnet/IP Who-Is/ReadProperty/WriteProperty + wrappers, UDP e2e); PJLink/S7/FTP/SMS/IEC 60870 remain |

### C1 coverage notes

- **Tc2_Standard**: complete — the 16 IEC narrow-string functions,
  9 timer/counter FBs, 9 `W*` wrappers, and 3 `L*` LTIME timers are
  all covered (builtins + wrappers).
- **Tc2_Math**: all 7 math functions implemented in pure ST.
  `TRUNC` narrows through `DINT`, so magnitudes `>= 2^31` lose
  precision (documented in the source header). `LMOD` uses
  truncation semantics (fmod-style); `MODABS` uses floor-modulo so
  the result is always in `[0, |m|)` — both flagged for verification
  against Beckhoff behaviour on edge cases.
- **Tc2_Utilities**: 325 APIs, mostly T2 (file, registry, license,
  AMS router, timezone, `NT_*`). Implemented: BCD pair,
  `FB_EnumStringNumbers` (enum ordinals passed as INT; number-format
  consume sets approximated per `E_NumGroupTypes`). Blocked by
  missing language features, not effort: `FB_CalcHashValue` and
  `FB_MemBuffer*`/`FB_MemRingBuffer*` need `PVOID`/pointers;
  `FB_FormatString` needs the `T_Arg` union type.
- **Tc2_IoFunctions**: re-tiered T0 -> T3 in the plan — the catalog
  is dominated by EtherCAT device FBs (KL terminals, ASI, DPV1,
  IO-Link, SCIT, TcTouch, AX2000), not process-image helpers.

### C2 coverage notes (tplc-net / Tc2_TcpIp)

- **Architecture**: one builtin `FB_Socket` (op-multiplexed, members
  `OP/HOST/PORT/TIMEOUT/DATA/DLEN/HANDLE/AUX/RESULT/ERRID/EXEC/BUSY/
  DONE`) bridges to the host `tplc-net` service through
  `TplcServices.get_service`. All service ops are nonblocking POSIX
  sockets; long ops (connect/accept/send/recv) are polled per scan
  until `DONE` or the `TIMEOUT` deadline. `ERRID` carries the
  positive service code (1=param, 2=no service, 3=socket, 4=connect,
  5=timeout, 6=closed, 7=again, 8=state).
- **Wrappers**: `compat/tc2/tc2_tcpip.st` exposes the Beckhoff names
  with their documented in/out roles. `sSrvNetId` is dropped (no AMS
  routing yet); `POINTER TO BYTE` payloads are `STRING[256]`;
  `hSocket`/`hListener` are `UDINT` service handles.
- **Verified**: `tc2_tcpip` ctest runs a real loopback exchange —
  listen, connect, accept, 4-byte send, receive, three closes —
  through `tether-plc` + `tether-plc-sim`, 7 packed checks.
- **Honest gaps**:
  - IPv4 dotted-quad only; no DNS (a resolver would block the scan).
  - UDP ops exist in the service and `FB_Socket` but only TCP is
    exercised by the e2e; multicast add/drop is not implemented.
  - `FB_SocketCloseAll` and the higher-level `FB_Client`/`FB_Server`/
    `FB_Helper` connection wrappers are not provided.
  - Socket handles do not survive program reload: the state blob is
    rebuilt but host fds persist until process exit (documented
    limitation, no handle re-validation yet).
  - `tTimeout <= 0` means no deadline; Beckhoff requires a timeout.

### C2 coverage notes (tplc-serial / Tc2_SerialCom)

- **Architecture**: same shape as `tplc-net` — builtin `FB_Serial`
  (op-multiplexed, `OP/DEV/BAUD/PARITY/DBITS/SBITS/FLAGS/AUX/TIMEOUT/
  DATA/DLEN/HANDLE/RESULT/ERRID/EXEC/BUSY/DONE`) bridges to the host
  `tplc-serial` service through `TplcServices.get_service`. Service
  ops are single nonblocking termios reads/writes; `serial_open`
  configures the port once (baud, databits, parity, stopbits, raw
  mode, optional RS485 flag) before returning.
- **Wrappers**: `compat/tc2/tc2_serialcom.st` exposes
  `FB_SerialOpen/Send/Receive/Close/Flush/Rs485`. TwinCAT's
  `ComBuffer`/`SerialLineControl` model keeps per-port state in a
  runtime-owned buffer object; this port uses explicit `UDINT`
  handles returned by `FB_SerialOpen`. `POINTER TO BYTE` payloads
  are `STRING[256]`; protocol FBs (3964R, RK512, KL6xxx) are not
  provided.
- **Verified**: `tc2_serial` ctest runs a real pty-pair loopback —
  open, 4-byte send, receive (python echo peer), flush, rs485 flag,
  close, plus a bad-path open asserting `ERRID=3` — through
  `tether-plc` + `tether-plc-sim`, 8 packed checks.
- **Honest gaps**:
  - POSIX termios devices only (`/dev/ttyS*`, `/dev/ttyUSB*`,
    `/dev/pts/*`); no microcontroller UART backend yet.
  - RS485 is a best-effort `TIOCSRS485` ioctl at open plus a runtime
    direction stub; board-specific DE GPIO is not wired.
  - `ComReset` maps to `FB_SerialFlush` (tcflush), not a hardware
    reset; `SerialLineControl`/`SerialLineControlADS` background
    FBs and the 3964R/RK512 protocol FBs are not implemented.
  - Baud rates limited to the termios `speed_t` set; exotic rates
    (e.g. 230400+ on some drivers) may be rejected with `ERRID=1`.

### C2 coverage notes (tplc-ads / Tc2_System)

- **Architecture**: same shape as `tplc-net`/`tplc-serial` — builtin
  `FB_Ads` (op-multiplexed, `OP/HOST/PORT/IDXGRP/IDXOFF/STATE/DEVSTATE/
  AUX/TIMEOUT/DATA/DLEN/HANDLE/RESULT/ERRID/EXEC/BUSY/DONE`) bridges to
  the host `tplc-ads` service through `TplcServices.get_service`.
  `OP` selects open(1)/read(2)/write(3)/read-state(4)/write-control(5)/
  close(6). The service speaks the Beckhoff AMS/TCP wire format
  (6-byte TCP header, 32-byte AMS header, ADS payload) over a
  nonblocking socket; connect completes lazily inside the first
  request, and each request/response exchange is polled per scan
  until `DONE` or the `TIMEOUT` deadline. One outstanding request per
  connection; polling the same op continues, a different op gets
  `ERRID=7` (state). `RESULT` carries the ADS-layer error code,
  `ERRID` the transport/service code.
- **Wrappers**: `compat/tc2/tc2_system.st` exposes
  `FB_AdsConnect/Read/Write/ReadState/WriteControl/Close`. TwinCAT
  connects implicitly per (NETID, PORT); this port needs the explicit
  connect FB because the transport is TCP. `sNetId` maps to `sHost`
  (IPv4 literal only — the first four NetID octets become the host
  address and `.1.1` the target AMS port suffix). `POINTER TO BYTE`
  payloads are `STRING[256]`; indication FBs (`ADSREADIND`,
  `ADSWRITEIND`, `...RES`) and combined `ADSRDWRT` are not provided.
- **Verified**: `tc2_system_ads` ctest runs a real AMS/TCP exchange —
  connect, read (`'ABCDEFGH'`), write, read-state (RUN), write-control,
  close, plus a refused-port connect+read asserting `ERRID=3` —
  through `tether-plc` + `tether-plc-sim` against
  `tests/compat/ads_test_server.py`, a python ADS responder speaking
  the Beckhoff wire format. 8 packed checks.
- **Honest gaps**:
  - Client side only; no AMS router, no ADS server, no routing
    between NetIDs. The source AMS NetID/port are fixed placeholders
    (`127.0.0.1.1.1`/0); a real deployment needs a configured local
    NetID.
  - Target NetID is derived as `host + ".1.1"` (the common Beckhoff
    device convention); arbitrary target NetIDs are not selectable.
  - Commands implemented: Read, Write, Read State, Write Control.
    Read-Write (0x09), Add/Delete Device Notification, and the
    broadcast/system commands are not implemented.
  - Write Control sends additional-data length 0; the
    `ADSWriteControl` extended control-data path is not wired.
  - `nAdsError` surfaces the ADS result code but TwinCAT's full
    `ADS_ERROR` enumeration is not mapped.

### C3 coverage notes (tplc-rtmon / Tc3_RealtimeMonitoring)

- **Architecture**: builtin `FB_RtMon` (op-multiplexed,
  `OP/CONTEXT/GROUP/EVENT/MARKCTRL/AUX/RESULT/ERRID/EXEC/BUSY/DONE`)
  bridges to the host `tplc-rtmon` service. `OP` selects
  mark(1)/count(2)/get(3)/clear(4). Unlike the socket services the
  ops are synchronous O(1) ring operations, so `DONE` is set on the
  same scan as the `EXEC` rising edge; there is no poll state.
- **Service**: `RtmonService` keeps a 4096-entry ring of
  `TplcRtmonMark { ts_ns, context, group, event, markctrl }`;
  oldest record is overwritten when full. Timestamps come from the
  injected clock, which is `HostServices::steadyNs`, so `--sim`
  steps produce deterministic sim-time marks. `rtmon_count` returns
  the retained count (the valid `rtmon_get` index domain);
  `rtmon_get(0)` is the oldest retained record and fails with
  `TPLC_RTMON_E_RANGE` past the end.
- **Wrappers**: `compat/tc3/tc3_rtmon.st` exposes
  `FB_RTMon_LogMarkBase` (explicit context + `nMarkCtrl`) and
  `FB_RTMon_LogMarkBaseEx` (`LogMarkEx` with the `ST_RTMon_MarkDef`
  fields flattened — struct types cannot cross the `--lib` boundary,
  they are file-scoped), plus the eight simple-mark FBs
  (`FB_RTMon_LogMark`, `_LogIntervalStart/Stop`,
  `_LogSequenceStart/Stop`, `_LogStart`, `_LogStop`, `_LogTrigger`)
  which embed the `TcMarkOption` mask for their method and OR in
  `RefToCaller` (`16#08000000`) when `bLogCallingTask` is set.
- **Verified**: `tc3_rtmon` ctest replays a 10-state sequence —
  base/ex/simple marks, count=4, `get[0]`/`get[3]` field checks,
  `get[4]` out-of-range (`ERRID=3`), clear, recount=0 — through
  `tether-plc` + `tether-plc-sim`. 10 packed checks (`0x3ff`).
- **Honest gaps**:
  - TwinCAT FB methods (`fb.LogMark(...)`) are not expressible; each
    method is a separate `bExecute`-gated FB (one rising edge = one
    mark). `nContextId`'s `AdsPort` default and task references are
    unavailable; `RefToCaller` only records the flag bit.
  - `LogTrigger` stores a plain mark; server-side recording triggers
    are not modeled. `hrErrorCode` carries 0 or a negative service
    code, not a real COM HRESULT.
  - Marks are consumed in-process only; there is no realtime-monitor
    server, trace export, or marker-group naming.
- **Blocked siblings**: `Tc3_DynamicMemory` needs `PVOID` /
  `REFERENCE TO PVOID` (no pointer type in the language — same blocker
  as the Tc2_Utilities heap/buffer FBs). `Tc3_Module` is the TwinCAT
  COM-like object model (`ITcUnknown`, `Fw_ObjMgr_*`, interface
  pointers) — not expressible without an object/interface runtime.
- **Tc3_JsonXml blocked**: all 8 FBs are unusable in the current
  dialect — `FB_JsonDomParser`/`FB_JsonDynDomParser` are ~150
  `METHOD`s over opaque `SJsonValue` handles with `REFERENCE TO BOOL`
  exec params; `FB_JsonSaxReader` needs the `ITcJsonSaxHandler`
  interface; `FB_JsonSaxWriter`/`FB_JsonSaxPrettyWriter` take
  `POINTER TO BYTE`; `FB_JsonReadWriteDataType` needs symbol-address
  introspection; `FB_XmlDomParser` shares the DOM/method model;
  `FB_JwtEncode` is an `ITcJsonSaxHandler` implementation. A faithful
  port needs METHOD syntax, interface types, and pointers — a
  JSON "subset" would be a new API, not TwinCAT compatibility.
- **Tc3_EventLogger blocked**: the core is an object model —
  `FB_TcEventBase`/`FB_TcEvent`/`FB_TcAlarm`/`FB_TcMessage` expose
  `I_TcEventBase`/`I_TcArguments`/`I_TcMessage`/`I_TcSourceInfo`
  interface properties and METHODs returning `HRESULT`; the remote
  listener needs ADS device notifications (not implemented in
  `tplc-ads`). The only plain functions, the four
  `*TcEventEntry*`/`AdsErr` converters, lack documented mapping
  semantics in the manual (which GUID/event-ID/severity each ADS
  error produces is unspecified) and `TcEventEntry` needs a `GUID`
  type the dialect does not have.

### C4 coverage notes (tplc-modbus / Tc2_ModbusSrv + Tc2_ModbusRTU)

- **Architecture**: same shape as the other service bridges — builtin
  `FB_Modbus` (`HOST/PORT/UNIT/TIMEOUT/FUNC/DATA/RLEN/DLEN/HANDLE/
  RESULT/EXC/ERRID/EXEC/BUSY/DONE`, 320-byte instance) bridges to the
  host `tplc-modbus` service through `TplcServices.get_service`.
  `FUNC` is the raw Modbus function code; `DATA` is an
  `ARRAY[0..255] OF BYTE` carrying request PDU bytes in (`RLEN`) and
  response PDU bytes out (`DLEN`). One transaction per EXEC edge;
  the service owns the nonblocking connect/send/receive state and
  frees the handle on terminal results. `RESULT` is 0=ok / 1=Modbus
  exception (`EXC` = exception code); `ERRID` carries the transport
  code.
- **Service**: `ModbusService` implements Modbus TCP — MBAP header
  (transaction id, protocol 0, length, unit) + PDU, all multi-byte
  fields big-endian on the wire. Connect-per-transaction like the
  TwinCAT FBs; partial sends/receives are driven per poll; MBAP
  length is used to bound the response; transaction id, protocol id,
  unit id, and function code are validated; exception responses
  (func|0x80) surface via `status`/`exc`. Its clock is injected by
  `HostServices` as `steadyNs`.
- **Wrappers**: `compat/tc2/tc2_modbussrv.st` exposes all twenty
  catalog FBs — the ten TCP client FBs (`FB_MBReadCoils` func 1,
  `FB_MBReadInputs` 2, `FB_MBReadRegs` 3, `FB_MBReadInputRegs` 4,
  `FB_MBWriteSingleCoil` 5, `FB_MBWriteSingleReg` 6,
  `FB_MBWriteCoils` 15, `FB_MBWriteRegs` 16, `FB_MBReadWriteRegs`
  23, `FB_MBDiagnose` 8) and the ten `FB_MBUdp*` UDP variants with
  identical signatures — with the documented
  `bExecute`/`bBusy`/`bDone`/`bError`/`nErrId` outputs. Modbus
  exception responses map to `nErrId = 16#8000 + exception code`.
  Coil/register payloads are raw big-endian bytes; bit packing for
  coil arrays stays the caller's job, as in TwinCAT.
- **UDP transport**: service ABI v2 appends `mb_begin_udp`; the
  request ADU goes in one datagram, the response is the first
  datagram from the request peer whose MBAP length is covered.
  `FB_Modbus.UDP` selects the transport per transaction; handles
  share the `mb_poll`/`mb_abort` namespace.
- **RTU transport**: service ABI v3 appends `mb_begin_rtu` and
  `TplcModbusRtuReq` (device path, baud, parity, data/stop bits,
  unit id, timeout, function, request bytes). The serial device is
  opened per transaction with termios raw settings matching the
  request; the RTU frame is `unit | func | data | crc16lo |
  crc16hi` (CRC-16/Modbus, transmitted low byte first). Responses
  are validated on unit id, CRC, and function code, then the PDU
  feeds the same `finishPdu` parser as TCP/UDP. Expected response
  length is derived for fixed-size functions (echo + byte-count
  functions); functions without a derivable length complete on the
  T3.5 inter-frame gap measured from baud rate. Nonblocking
  `read`/`write`; a 0-byte read under `VMIN=0`/`VTIME=0` means "no
  data this scan", not EOF.
- **RTU bridge**: builtin `FB_ModbusRtu` (384-byte instance,
  `DEV/BAUD/PARITY/DBITS/SBITS/UNIT/TIMEOUT/FUNC/DATA/RLEN/DLEN/
  HANDLE/RESULT/EXC/ERRID/EXEC/BUSY/DONE`) mirrors `FB_Modbus` with
  serial-line inputs replacing `HOST`/`PORT`. Handles share the
  same `mb_poll`/`mb_abort` namespace.
- **RTU wrappers**: `compat/tc2/tc2_modbusrtu.st` exposes the ten
  `ModbusRtuMasterV2_*` actions as standalone FBs
  (`FB_MBRtuReadCoils` func 1, `FB_MBRtuReadInputs` 2,
  `FB_MBRtuReadRegs` 3, `FB_MBRtuReadInputRegs` 4,
  `FB_MBRtuWriteSingleCoil` 5, `FB_MBRtuWriteSingleReg` 6,
  `FB_MBRtuWriteCoils` 15, `FB_MBRtuWriteRegs` 16,
  `FB_MBRtuReadWriteRegs` 23, `FB_MBRtuDiagnose` 8) with serial
  line settings as explicit inputs (`sDevice`, `nBaud`, `nParity`,
  `nDataBits`, `nStopBits`) instead of TwinCAT System Manager COM
  bindings. `POINTER TO BYTE` payloads are bounded
  `ARRAY[0..255] OF BYTE` buffers.
- **Verified**: `tc2_modbussrv` ctest replays a six-transaction
  TCP sequence (read regs, write single reg, write regs, read
  coils, exception read at addr>=99, read/write regs) plus a
  five-transaction UDP sequence through `tether-plc` +
  `tether-plc-sim` against `tests/compat/modbus_test_server.py`,
  a python Modbus TCP/UDP responder. 6 + 5 packed checks
  (`0x3f` / `0x1f`). `tc2_modbusrtu` ctest replays the same
  six-transaction sequence over RTU through `tether-plc` +
  `tether-plc-sim` against `tests/compat/modbus_rtu_test_server.py`,
  a python Modbus RTU slave on a pty pair, expecting mask `0x3f`.
- **Honest gaps**:
  - IPv4 dotted-quad only; no DNS, no IPv6.
  - `Tc2_ModbusRTU` is partially covered: the V2 master action set
    is implemented, but `ModbusRtuSlave_*` (needs RTU listen mode),
    `*_Generic` ComBuffer variants (needs the TwinCAT ComBuffer
    object model), the KL6x*B bus-terminal variants (need EtherCAT
    terminal integration), and the obsolete non-V2 master FBs are
    not provided.
  - `POINTER TO BYTE` payloads are bounded `ARRAY[0..255] OF BYTE`
    buffers; `cbLength`-style capacity args are advisory.
  - Slave/server mode (listening Modbus TCP/UDP/RTU) is not
    provided.
  - UDP encapsulation assumes the same MBAP+PDU ADU per datagram;
    the TwinCAT manual does not specify a different framing.

### C4 coverage notes (tplc-sacn / Tc3_sACN)

- **Architecture**: builtin `FB_SacnSource` /
  `FB_SacnReceiver` bridge to the host `tplc-sacn` service
  (`include/tplc/tplc_sacn.h`) through `TplcServices.get_service`.
  The service owns two nonblocking UDP sockets: a sender bound to
  port 5568 and a receiver joined to the per-universe multicast
  group `239.255.x.y` (universe-derived, not configurable — the
  E1.31 group allocation is fixed). The source is ENABLE-edge
  lifecycle like `FB_Modbus`; the receiver is always-on once
  enabled.
- **Protocol**: E1.31-2018 wire codec verified byte-identical
  against two independent reference implementations — root layer
  (preamble `ASC-E1.17`, flags+length, vector 0x0004 data / 0x0001
  discovery under the extended framing vector 0x0002, sender CID),
  framing layer (vector 0x0002, source name, priority, sequence,
  universe, DMP vector), and DMP layer (static field descriptor
  `A1 00 00 00 01`, property count including the start-code byte,
  start code, DMX payload). Sync (vector 0x0001) and discovery
  (vector 0x0001 under root vector 0x0001) packets are built by the
  service but not yet exposed to PLC inputs.
- **Priority semantics**: sACN priority is inverted relative to
  intuition — lower value wins. The receiver replaces its DMX
  buffer only when an incoming packet's priority is equal or
  better than the held frame; worse-priority packets are counted
  in `nDroppedPackets` and their payload is discarded.
- **Wrappers**: `compat/tc3/tc3_sacn.st` adapts TwinCAT's
  callback-driven `I_sAcn*` surface into poll-driven FBs
  (`FB_sACN_Source`/`FB_sACN_Receiver`) plus the three pure-IEC
  catalog helpers (`F_HighLowArr_TO_UINT`,
  `F_UNIVERSE_TO_MulticastAdr`, `FB_UINT_TO_HighLowArr`). The
  `I_sAcnCommunicator`/`I_sAcnSource`/`I_sAcnReceiver` interface
  methods and the five interface-callback FBs remain unimplemented
  (same blocker class as Tc3_EventLogger).
- **Verified**: `tc3_sacn` ctest builds the library and a
  consumer program, then replays 600 free-running sim steps
  against `tests/compat/sacn_test_peer.py` — a python E1.31 peer
  that joins the group, waits for the DUT's own `SRC1` stream,
  then injects priority-10 `HIPRI` and priority-200 `LOWPR`
  packets. Four packed checks (`0x0f`): own-source receive,
  HIPRI preemption, LOWPR drop accounting, helper conversions.
- **Honest gaps**:
  - Universe range 1..63999 maps to multicast groups per E1.31;
    universes > 63999 are rejected.
  - `nDmxLength` excludes the sACN start-code byte (service strips
    it); the wire always carries `start code + payload`.
  - Discovery/sync transmission is service-internal; no PLC-level
    inputs expose `F_SYNC`/`F_DISCOVERY` addresses or the
    `sACN_E131_*` personality fields.
  - The `I_sAcn*` COM-style interface model is not implemented;
    TwinCAT code written against `I_sAcnCommunicator` callbacks
    needs porting to the poll model.
  - IPv4 multicast only; no IPv6, no unicast sACN (E1.31 is
    multicast-only for data).

### C4 coverage notes (tplc-bacnet / Tc3_BACnetRev14)

- **Architecture**: builtin `FB_Bacnet` bridges to the host
  `tplc-bacnet` service (`include/tplc/tplc_bacnet.h`) through
  `TplcServices.get_service`. One transaction per EXEC rising
  edge; the service owns a nonblocking UDP socket per transaction
  (ephemeral port, `SO_BROADCAST` for limited broadcast), sends
  the request in `bac_begin`, and `bac_poll` drains datagrams,
  matches APDU invoke IDs, and enforces the deadline via the
  injected `steadyNs` clock. Completed transactions are dropped
  on the terminal `bac_poll`, matching the modbus model.
- **Protocol**: BACnet/IP (ASHRAE 135) BVLC (`0x81`, unicast
  `0x0A` / broadcast `0x0B`, u16 length) + NPDU (version 1,
  expecting-reply for confirmed) + APDU codec verified against
  `bacpypes3` encodings: Who-Is (unconfirmed svc 8, optional
  device-instance range) / I-Am collection into an 8-responder
  table (object ID, responder IPv4, vendor, max-APDU,
  segmentation), ReadProperty (confirmed svc 12 → Complex-ACK
  object/property echo + first application-tagged value),
  WriteProperty (confirmed svc 15 → Simple-ACK, optional array
  index and write priority). Error/Reject/Abort APDUs surface as
  `STATUS = 3`; malformed or wrong-invoke datagrams as
  `STATUS = 4`; deadline expiry as `STATUS = 2` (Who-Is instead
  reports `STATUS = 1` with whatever responders arrived).
- **Values**: property values cross the ABI as raw BACnet
  application-tag payload bytes — `nValueType`/`VAL_TYPE`
  selects the tag (1 boolean, 2 unsigned, 3 integer, 4 real,
  5 double, 6 octetString, 7 characterString, 9 enumerated,
  12 objectIdentifier) and `pValue`/`VAL` holds the tag payload
  in wire byte order. The service wraps/unwraps tag framing but
  does not interpret payloads.
- **Wrappers**: `compat/tc3/tc3_bacnet.st` provides
  `FB_BACnet_WhoIs`, `FB_BACnet_ReadProperty`, and
  `FB_BACnet_WriteProperty` in the same poll-driven shape as the
  other compat libraries. TwinCAT's actual BACnet integration is
  device-configuration-driven (the System Manager generates FBs
  per configured BACnet device), so there is no static TwinCAT FB
  API to mirror — the catalog's three entries are documentation
  examples, not callable functions.
- **Verified**: `tc3_bacnet` ctest builds the library and a
  consumer program, then replays 350 free-running sim steps
  against `tests/compat/bacnet_test_peer.py` — a python
  BACnet/IP responder on UDP 47808. Six packed checks (`0x3f`):
  broadcast Who-Is/I-Am with instance range, ReadProperty REAL
  Complex-ACK, WriteProperty Simple-ACK, object-name
  characterString, BACnet Error on unknown object, and directed
  unicast Who-Is.
- **Honest gaps**:
  - No COV subscriptions/notifications, no event/alarm
    services, no segmentation (max-APDU 1476 assumed), no
    Who-Has/I-Have, no router (BBMD/FD) support.
  - ReadProperty returns only the first application-tagged
    value; multi-value properties and array-element reads beyond
    a single optional index are not exposed.
  - Peer addresses are dotted-quad IPv4 strings only; no DNS,
    no IPv6.
  - The TwinCAT device-configuration model (XML device imports,
    generated per-device FBs, alarm routing to the TwinCAT
    event system) is not reproduced; TwinCAT BACnet projects
    need porting to the explicit Who-Is/Read/Write wrappers.

## Known limitations

- Source inclusion, not binary linking: library globals must come
  from the consumer via `VAR_EXTERNAL`; libraries cannot own
  `VAR_GLOBAL` (rejected by `tplcc build --library`).
- `F_GetVersionTcMath` returns a placeholder `16#0100`; Beckhoff's
  exact version-element encoding is not yet replicated.
- `BCD_TO_DEC`/`DEC_TO_BCD` are single-scan FBs: `BUSY` pulses for
  one scan after `START` and `ERR` latches until the next `START`,
  matching the documented handshake shape but not any asynchronous
  timing Beckhoff guarantees.
- **Known toolchain issue**: on the LLVM path, FUNCTION_BLOCK
  outputs written only inside conditional branches can lose
  retention between scans (observed: `ERR`/`ERRID` partially
  cleared several scans after a conditional write; the MIR
  interpreter is unaffected). Compat FBs work around it by
  computing into instance vars and assigning every output
  unconditionally each call — see `tc2_util_bcd.st`. Root cause
  not yet isolated; tracked as a compiler bug.
