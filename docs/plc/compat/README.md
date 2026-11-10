# TwinCAT compatibility layer

ST-source compatibility implementations of Beckhoff TwinCAT 3 PLC
libraries, packaged as `kind="library"` `.tplc` artifacts (plan 21,
C0/C1). Each library mirrors the documented API surface closely enough
for TwinCAT ST sources to compile and run unchanged where the
underlying semantics allow.

Per-library API references (parameters, descriptions, recursive
catalog) live in `twincat/` — generated from
`docs/plc/plan/21-twincat-libraries.md` by
`tools/twincat-docs/split_plan.py`.

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
