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
  tc2_sms_sms.st      Tc2_SMS + Tc2_SMTP: FB_SendSMS GSM AT-command
                      state machine over FB_Serial, FB_SmtpV3 /
                      FB_SmtpV3_Full over the builtin FB_SmtpSend
                      (tplc-smtp service), FB_Smtp / FB_SmtpFull
                      aliases, FB_SmtpAttach (unsupported stub),
                      FB_GetTcPlcSMS_Version
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
  tc2_mc2.st         Tc2_MC2 PLCopen motion wrappers over the builtin
                      FB_MotionAxis (tplc-motion service): MC_Power,
                      MC_Reset, MC_SetPosition (abs+rel),
                      MC_ReadActualPosition/Velocity, MC_ReadAxisError,
                      MC_ReadStatus, plus the command FBs
                      MC_MoveAbsolute/Relative/Velocity, MC_Stop,
                      MC_Halt, MC_Home, MC_Jog. AXIS_REF is a UDINT
                      axis handle; positions are int32 user units
                      (TwinCAT LREAL is a documented gap). The command
                      FBs drive the DS402 controlword (NewSetPoint for
                      PP/HM, QuickStop for MC_Stop) + mode/target
                      objects; per-command Velocity/Acceleration/
                      Deceleration are signature-compatible but the
                      axis applies its configured dynamics. Gearing/
                      phasing/superposition land in a later tranche.
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
  tc3_pjlink.st       Tc3_PJLink over the builtin FB_Pjlink
                      (tplc-pjlink service): JBMIA PJLink
                      projector client over TCP 4352 — generic
                      FB_PJLink command wrapper plus
                      FB_PJLink_Power / FB_PJLink_Input /
                      FB_PJLink_Mute method-shaped wrappers;
                      MD5(nonce+password) auth handled by the
                      service when the projector greets
                      `PJLINK 1`
  tc3_s7comm.st      Tc3_S7Comm + Tc2_S5S7Com over the builtins
                      FB_S7Conn / FB_S7Io (tplc-s7 service):
                      Siemens S7comm client over RFC1006
                      (TPKT + COTP + S7comm, TCP 102) —
                      FB_S7_Connect session wrapper plus
                      FB_S7_Read / FB_S7_Write /
                      FB_S7_ReadBit / FB_S7_WriteBit
                      transaction wrappers; covers the
                      FB_S7CommConnection/AddReadVar/
                      AddReadBit/AddReadByteArray concepts and
                      the FB_NLinkOpen/Close/Fetch/Write set
  tc3_ftp.st         Tc3_FTP + Tc2_FTP over the builtins
                      FB_FtpConn / FB_FtpCmd / FB_FtpXfer
                      (tplc-ftp service): FTP client over
                      RFC 959 with PASV data connections —
                      FB_FTP_Connect session wrapper,
                      FB_FTP_Cmd generic control wrapper,
                      FB_FTP_Download / FB_FTP_Upload /
                      FB_FTP_List transfer wrappers, and
                      MkDir/RemoveDir/DeleteFile/FileExists/
                      DirExists/Rename method wrappers;
                      covers FB_FtpConnection/FB_FtpExplorer/
                      FB_FtpTransfer and the FB_FTP_* set
  tc3_iec870.st      Tc2_IEC60870_5_10x over the builtins
                      FB_Iec870Conn / FB_Iec870Tx / FB_Iec870Rx
                      (tplc-iec870 service): IEC 60870-5-104
                      client on TCP 2404; FB_IEC870_5_104Master
                      session wrapper (STARTDT/STOPDT/TESTFR,
                      I/S/U APCI), FB_IEC870_SendAsdu /
                      FB_IEC870_RecvAsdu raw-ASDU wrappers, plus
                      pure helpers: COT/TYPEID strings, CP56Time2a
                      encode/decode, CP24/CP32 truncation, IOA
                      composition, SPI/DPI/VTI bit helpers,
                      GetIec870_Version
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
  tc3_pjlink_case.st   autonomous PJLink program (7 packed
                      checks: POWR query, Power/Input/Mute
                      wrappers, MD5 auth accept, auth reject,
                      ERR1 command error)
  tc3_pjlink.sh        ctest tc3_pjlink: library + consumer +
                      250-step sim replay expecting mask 0x7f,
                      python PJLink peer (pjlink_test_peer.py)
  pjlink_test_peer.py  minimal PJLink (JBMIA) responder on TCP
                      4352 used by tc3_pjlink.sh: connection-
                      count greetings (PJLINK 0 / PJLINK 1
                      nonce), digest verify, ERRA, ERR1
  tc3_s7comm_case.st   autonomous S7comm program (9 packed
                      checks: connect handshake, DB read,
                      write+readback, MK bit read/write,
                      unregistered-area s7err, refused connect,
                      session close)
  tc3_s7comm.sh        ctest tc3_s7comm: library + consumer +
                      300-step sim replay expecting mask 0x1ff,
                      python S7comm peer (s7_test_peer.py)
  s7_test_peer.py      minimal Siemens S7comm (RFC1006)
                      responder on TCP 1102 used by
                      tc3_s7comm.sh: TPKT + COTP CR/CC/DT,
                      setup communication, Read/Write Var;
                      DB1 + MK memory areas, spec bit semantics
  tc3_ftp_case.st      autonomous FTP program (12 packed
                      checks: login, SIZE, MKD, RETR, LIST,
                      STOR, SIZE verify, file/dir exists,
                      RNFR/RNTO rename, refused connect,
                      session close)
  tc3_ftp.sh           ctest tc3_ftp: library + consumer +
                      500-step sim replay expecting mask 0xfff,
                      python FTP peer (ftp_test_peer.py)
  ftp_test_peer.py     minimal FTP (RFC 959, PASV) responder
                      on TCP 2121 used by tc3_ftp.sh:
                      USER/PASS login, MKD/RMD/DELE/SIZE/
                      RNFR/RNTO, RETR/STOR/LIST/NLST over
                      passive data connections
  tc2_sms_sms_case.st  autonomous serial+SMTP program (9 packed
                      checks: pty open, AT SMS, SMTP plain,
                      SMTP AUTH LOGIN, RCPT 550 reject,
                      attach-unsupported, version, refused
                      connect, serial close)
  tc2_sms_sms.sh       ctest tc2_sms_sms: both libraries +
                      consumer + 700-step sim replay expecting
                      mask 0x1ff; pty GSM modem peer
                      (gsm_modem_peer.py) and RFC 5321 peer
                      (smtp_test_peer.py) on TCP 2525
  gsm_modem_peer.py    minimal GSM AT modem on a pty pair used
                      by tc2_sms_sms.sh: AT / AT+CMGF=1 /
                      AT+CMGS text-mode send with Ctrl-Z
  smtp_test_peer.py    minimal SMTP (RFC 5321) responder on
                      TCP 2525 used by tc2_sms_sms.sh:
                      EHLO+AUTH LOGIN, MAIL/RCPT/DATA, 550 on
                      "reject" recipients, MSG log lines
  tc3_iec870_case.st   autonomous IEC 60870-5-104 program (12
                      packed checks: conn ready, interrogation
                      sent, M_SP+M_ME recv, clock sync sent+recv,
                      single command sent+recv, refused connect,
                      helpers, STOPDT close)
  tc3_iec870.sh        ctest tc3_iec870: library + consumer +
                      400-step sim replay expecting mask 0xfff,
                      python IEC 60870-5-104 slave peer
                      (iec870_test_peer.py) on TCP 2480
  iec870_test_peer.py  minimal IEC 60870-5-104 slave on TCP 2480
                      used by tc3_iec870.sh: STARTDT/STOPDT U-frame
                      handshake, I/S APCI sequencing, C_IC_NA_1 ->
                      M_SP_NA_1+M_ME_NA_1, C_CS_NA_1 echo,
                      C_SC_NA_1 act-con
  tc2_motion_case.st   deterministic CiA402 axis program (12
                      packed checks: create, mode/target/controlword
                      writes, operation-enabled, PP move target-
                      reached, velocity mode, bad-object/bad-mode/
                      bad-handle errors, destroy, destroyed-handle)
  tc2_motion.sh        ctest tc2_motion: consumer + 500-step sim
                      replay expecting mask 0xfff, in-process
                      tplc-motion service (no external peer)
  tc2_mc2_case.st      PLCopen axis-management program (10 packed
                      checks: axis create, MC_Power enable, MC_Set-
                      Position abs+rel readback, MC_Read* validity,
                      MC_Reset, MC_Power disable, disabled state)
  tc2_mc2.sh           ctest tc2_mc2: builds compat/tc2/tc2_mc2.st
                      as a library, consumes via --lib, 300-step sim
                      replay expecting mask 0x3ff, in-process
                      tplc-motion service (no external peer)
  tc2_motion_io.sh     ctest tc2_motion_io: same tc2_motion_case.st
                      replayed over the Tether IO protocol against
                      tether_io_motion_server (SimulatedCiA402Drive,
                      --clock sim) via TPLC_MOTION_IO; skips when the
                      Tether server binary is unavailable
  tc2_mc2_motion_case.st PLCopen motion-command program (13 packed
                      checks: axis create, MC_Power enable,
                      MC_MoveAbsolute/Relative + position readback,
                      MC_MoveVelocity InVelocity, MC_Stop quick-stop,
                      MC_Halt, MC_Home, MC_Jog, MC_Power disable)
  tc2_mc2_motion.sh    ctest tc2_mc2_motion: builds tc2_mc2.st as a
                      library, consumes via --lib, 1501-step sim
                      replay expecting mask 0x1fff, in-process
                      tplc-motion service (no external peer)
  tc2_mc2_motion_io.sh ctest tc2_mc2_motion_io: same
                      tc2_mc2_motion_case.st replayed over the Tether
                      IO protocol against tether_io_motion_server
                      (--clock sim) via TPLC_MOTION_IO; skips when the
                      Tether server binary is unavailable
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
| C4 | Connectivity wave 1 | `tplc-modbus` done (Tc2_ModbusSrv 20/20 TCP+UDP + Tc2_ModbusRTU V2-master subset, pty e2e); `tplc-sacn` done (Tc3_sACN source/receiver + pure-IEC helpers, multicast e2e); `tplc-bacnet` done (BACnet/IP Who-Is/ReadProperty/WriteProperty + wrappers, UDP e2e); `tplc-pjlink` done (PJLink TCP client + auth + wrappers, TCP e2e); `tplc-s7` done (S7comm RFC1006 client + session/io FBs + wrappers, TCP e2e); `tplc-ftp` done (FTP PASV client + conn/cmd/xfer FBs + wrappers, TCP e2e); `tplc-smtp` + GSM AT SMS done (RFC 5321 client + AUTH LOGIN, AT state machine over tplc-serial, pty/TCP e2e); `tplc-iec870` done (IEC 60870-5-104 client + conn/tx/rx FBs + pure helpers, TCP e2e). C4 complete |
| C5 | Motion core | `tplc-motion` backend done (simulated CiA402/DS402 axis pool + `FB_MotionAxis` builtin); Tether-IO-backed backend done (`MotionIoService` maps object ops to `mc.<slave>.<index>` params on `tether_io_motion_server`/`SimulatedCiA402Drive`, deterministic via `mc.time` + `--clock sim`, in-process fallback retained); `compat/tc2/tc2_mc2.st` axis-management + command wrappers done (MC_Power/MC_Reset/MC_SetPosition abs+rel/MC_Read* plus MC_MoveAbsolute/Relative/Velocity/MC_Stop/MC_Halt/MC_Home/MC_Jog, e2e on both backends); gearing/phasing/superposition pending |

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

### C4 coverage notes (tplc-pjlink / Tc3_PJLink)

- **Architecture**: builtin `FB_Pjlink` bridges to the host
  `tplc-pjlink` service (`include/tplc/tplc_pjlink.h`) through
  `TplcServices.get_service`. One transaction per EXEC rising
  edge; the service owns a nonblocking TCP socket per
  transaction (connect-per-command, `TCP_NODELAY`), drives a
  Connect → Greet → Send → Recv state machine, and enforces the
  deadline via the injected `steadyNs` clock. Completed
  transactions close the socket and drop the map entry on the
  terminal `pjl_poll`, matching the modbus/bacnet model.
- **Protocol**: JBMIA PJLink over TCP 4352 — ASCII command
  lines `%<class><CMD4> <param>\r` (bare CR terminator, no `$`),
  responses `%<class><CMD4>=<value>\r`. The class digit selects
  the PJLink class (standard commands use `1`); the command body
  is a fixed 4-char field (`POWR`, `INPT`, `AVMT`, `ERST`,
  `INFO`, `NAME`, `INFM`, `LAMP`, ...). `?` as the parameter
  queries. PJLink-level command errors arrive as `ERR1`–`ERR4`
  *values* (`%1POWR=ERR3`) with transport `STATUS = 1`;
  `PJLINK ERRA` maps to `STATUS = 5`.
- **Authentication**: on connect a PJLink projector may greet
  `PJLINK 0\r` (no auth) or `PJLINK 1 <8-hex-nonce>\r`
  (password required). For the latter the service prepends
  `MD5(nonce_ascii + password)` — 32 lowercase hex chars, no
  separator — to the command line, using a compact in-tree MD5
  verified against the spec's worked example. Projectors that
  send no greeting within 400 ms are treated as no-auth
  (legacy behavior). The `PASSWORD` FB input therefore only
  needs the plaintext password; the digest is per-connection
  and never cached.
- **Wrappers**: `compat/tc3/tc3_pjlink.st` provides the generic
  `FB_PJLink` command FB (nCls/sCmd/sParam/sPassword in,
  sValue/nRespCls/nRespCmd/nValueLen/bAuthed out) plus
  method-shaped wrappers `FB_PJLink_Power` (POWR on/off/query),
  `FB_PJLink_Input` (INPT set/query), and `FB_PJLink_Mute`
  (AVMT mute on/off/query) covering the most-used TwinCAT
  `FB_PJLink` methods in poll-model form.
- **Verified**: `tc3_pjlink` ctest builds the library and a
  consumer program, then replays 250 free-running sim steps
  against `tests/compat/pjlink_test_peer.py` — a python PJLink
  responder on TCP 4352 whose greeting is chosen by connection
  count. Seven packed checks (`0x7f`): unauthenticated POWR
  query with response-header echo, Power/Input/Mute wrapper
  transactions, MD5 auth accept (`bAuthed`), wrong-password
  `PJLINK ERRA` → `STATUS = 5`, and `ERR1` command-error
  surfacing.
- **Honest gaps**:
  - TwinCAT's `FB_PJLink` is an auto-managed object: inputs
    `bKeepAliveSocket`, `tStatusUpdateTime`, `sServerNetID`,
    `eTraceLevel`, and outputs `eStep`/`ipResultMessage`/
    `nErrID`/`sErrBlock`/`stProjStatus` have no poll-model
    equivalent — `I_TcMessage`, `T_AmsNetId` routing,
    `TcEventSeverity`/EventLogger, and `HRESULT` method returns
    are documented incompatibilities.
  - No persistent session or keep-alive: every command is a
    fresh TCP connection (PJLink's safe pattern), so per-command
    connect/greeting overhead applies and `tStatusUpdateTime`
    cyclic polling is the caller's job.
  - Class 1 commands only in practice (the digit is passed
    through, but only class-1 semantics are exercised);
    Class 2 expanded commands are not separately validated.
  - IPv4 dotted-quad only; no DNS, no IPv6.
  - The remaining TwinCAT method surface (freeze, increase/
    decrease, lamp info, freeze status) is reachable through
    the generic `FB_PJLink` command FB but has no dedicated
    wrapper yet.

### C4 coverage notes (tplc-s7 / Tc3_S7Comm + Tc2_S5S7Com)

- **Architecture**: two builtins bridge to the host `tplc-s7`
  service (`include/tplc/tplc_s7.h`) through
  `TplcServices.get_service`. `FB_S7Conn` owns the persistent
  session: EXEC rising edge runs TCP connect → COTP CR/CC →
  S7 Setup Communication, driven through `s7_conn_poll` while
  BUSY; CLOSE rising edge tears the session down and clears
  CONN_HANDLE. `FB_S7Io` runs one Read Var / Write Var
  transaction per EXEC edge on a live CONN_HANDLE (MODE 0/1),
  driven through `s7_poll`; one outstanding io per connection
  (E_BUSY otherwise). Unlike PJLink's connect-per-command the
  S7 session is persistent, matching TwinCAT's connection
  object model.
- **Protocol**: Siemens S7comm over RFC1006 ISO-on-TCP, TCP
  port 102 (PORT 0). Wire stack verified against the
  python-snap7 reference server: TPKT `03 00 <len16>`, COTP
  CR/CC/DT (`06 E0`/`06 D0`/`02 F0 80`), S7 Job header 10B
  (`32 01`), Ack/Ack_Data header 12B with error class/code.
  Read Var func `0x04`, Write Var func `0x05`, Setup Comm
  func `0xF0` run automatically on connect. The COTP called
  TSAP folds rack/slot as `0x0100 | (rack<<5) | slot`.
- **Addressing**: S7-Any single-item specs — area codes
  `0x81` PE, `0x82` PA, `0x83` MK, `0x84` DB, `0x1C` CT,
  `0x1D` TM; `BYTE_OFF` is a 24-bit byte offset sent as
  `offset*8` bits; `BIT_ACCESS`+`BIT_NO` select single-bit
  access (`byte*8+bit`). Byte-array access uses the S7WordLen
  BYTE transport code so `LEN` is a plain byte count. Payloads
  cap at 240 bytes per transaction.
- **Wrappers**: `compat/tc3/tc3_s7comm.st` provides
  `FB_S7_Connect` (covers `FB_S7CommConnection` and the
  `FB_NLinkOpen`/`FB_NLinkClose` pair), `FB_S7_Read` /
  `FB_S7_Write` (cover `AddReadVar`/`AddReadByteArray`/
  `AddWriteVar`/`AddWriteByteArray` and `FB_NLinkFetch`/
  `FB_NLinkWrite`), and `FB_S7_ReadBit`/`FB_S7_WriteBit`
  (cover `AddReadBit` and bit writes). Bit reads return one
  byte right-justified in `pData[0]`/`nBitValue`; bit writes
  take `pData[0]&1`.
- **Verified**: `tc3_s7comm` ctest builds the library and a
  consumer program, then replays 300 free-running sim steps
  against `tests/compat/s7_test_peer.py` — a python S7comm
  responder on TCP 1102. Nine packed checks (`0x1ff`): connect
  handshake, DB1 read with pattern check, write+readback,
  MK bit read/write, unregistered-area item rc `0x0A` →
  `STATUS = 5`, refused connect → conn `STATUS = 2`, session
  close. The service codec was additionally verified
  standalone against the python-snap7 reference `Server`
  (read/write/bit round-trips byte-exact).
- **Honest gaps**:
  - TwinCAT's `FB_S7CommConnection` request-queue model
    (`AddRequest`, `FB_S7CommCyclicRequest`, `I_S7CommRequest`
    objects, `PVOID` buffers, `HRESULT` returns, AMS `T_AmsNetId`
    routing) has no direct equivalent — requests are explicit
    poll-model FBs, one outstanding transaction per connection.
  - No cyclic/triggered request scheduling; the caller polls.
  - Single-item requests only; no multi-item Read/Write Var,
    no Read String with S7 string header, no area DB-symbolic
    addressing, no PUT/GET, GET_PDU, SZL, or H-CPU functions.
  - S5 protocol (3964/PKW) used by `FB_NLink*` on S5 targets is
    not implemented — `tplc-s7` speaks S7comm only; S5 links
    need a separate service.
  - Rack/slot map only into the COTP called TSAP; no ISO-on-TCP
    port negotiation beyond the fixed/default port, no DNS,
    no IPv6.

### C4 coverage notes (tplc-ftp / Tc3_FTP + Tc2_FTP)

- **Transport**: `tplc-ftp` is an RFC 959 FTP *client* over TCP
  (default port 21) with **PASV passive data connections
  only**. The control channel is line-based (`USER`/`PASS`
  login, greeting 220, multi-line reply handling); each data
  transfer runs `TYPE I` -> `PASV` (227 tuple parse) -> data
  connect -> `RETR`/`STOR`/`LIST` -> 150 interim -> stream ->
  226 final. One outstanding control command or transfer per
  connection.
- **Builtins**: `FB_FtpConn` (connect/login/close session,
  STATUS 0=pending 1=ready 2=error), `FB_FtpCmd` (one control
  command, RESP_CODE + TEXT reply), `FB_FtpXfer` (OP 0=RETR
  download / 1=STOR upload / 2=LIST; downloads stream 512-byte
  chunks per scan into BUF with CHUNK/TOTAL, uploads feed
  UP_DATA[0..UP_LEN) automatically; STATUS adds 6=more-data).
- **Wrappers**: `compat/tc3/tc3_ftp.st` provides
  `FB_FTP_Connect` (covers `FB_FtpConnection.Connect/Disconnect`
  and `FB_FTP_Open`/`OpenEX`/`Close`/`CloseAll`/`Info`),
  `FB_FTP_Cmd` (generic control command), `FB_FTP_Download` /
  `FB_FTP_Upload` / `FB_FTP_List` (cover
  `FB_FtpTransfer.Download/Upload` and
  `FB_FTP_FileDownload(Ex)`/`FileUpload(Ex)`/`FileList(ListEx)`),
  and `FB_FTP_MkDir`/`RemoveDir`/`DeleteFile`/`FileExists`/
  `DirExists`/`Rename` (cover the `FB_FtpExplorer` methods and
  the `FB_FTP_Dir*`/`File*` set). Rename is RNFR+RNTO sequenced
  inside the wrapper.
- **Verified**: `tc3_ftp` ctest builds the library and a
  consumer program, then replays 500 free-running sim steps
  against `tests/compat/ftp_test_peer.py` — a python RFC 959
  responder on TCP 2121. Twelve packed checks (`0xfff`):
  login, SIZE (213/"68"), MKD (257), RETR (68B content check),
  LIST (non-empty), STOR (226) + SIZE verify, FileExists
  (213), DirExists (LIST ok), RNFR/RNTO rename (250),
  refused connect (STATUS 2), session close. The service was
  additionally verified standalone by a C++ harness covering
  the same exchange plus a 550 error path.
- **Honest gaps**:
  - PASV data connections only — no PORT/active mode, no EPSV,
    no FTPS/TLS, no SFTP/SSH (TwinCAT FTP *and* SFTP profiles;
    SFTP is a separate SSH protocol and is not implemented).
  - No FTP profile objects (`AddProfile`/profile persistence),
    no `I_TcMessage`/EventLogger/`HRESULT` surfaces, no AMS
    routing — credentials are plain per-connection STRINGs.
  - `FB_FTP_HostResolve` (DNS) is not implemented — hosts must
    be dotted-quad IPv4; no DNS or IPv6 anywhere in the service.
  - No REST resume, MLSD/MLST, SITE-specific extensions, or
    FXP; `DirExists` is implemented as a LIST on the path
    (listable = exists) rather than CWD probing.
  - Uploads are capped at 512 bytes per transfer (UP_DATA);
    downloads stream unlimited data in 512-byte chunks.
  - Rename/move relies on RNFR+RNTO — semantics for
    cross-directory moves depend on the server.

### C4 coverage notes (tplc-smtp + GSM AT SMS / Tc2_SMS + Tc2_SMTP)

- **SMTP transport**: `tplc-smtp` is a direct RFC 5321 SMTP
  *client* over TCP (default port 25), TwinCAT's `FB_Smtp*`
  blocks proxy through a TwinCAT SMTP server over ADS, which has
  no equivalent here, so the compat layer talks SMTP directly.
  Transaction: connect → 220 greeting → EHLO → optional
  `AUTH LOGIN` (334/334/235 challenge, base64 credentials) →
  MAIL FROM → RCPT TO → DATA → dot-stuffed headers+body →
  "." terminator → 250 → QUIT → 221. Plain text bodies only,
  one recipient, no TLS/attachments/receipts/MIME.
- **Builtins**: `FB_SmtpSend` (HOST/PORT/USER/PASS/FROM/RCPT/
  SUBJECT/TIMEOUT/BODY[255], EXEC edge, STATUS 0=pending
  1=ok 2=smtp-fail 3=timeout 4=error 5=badresp, RESP_CODE holds
  the transaction result code, the DATA-accept 250, not the
  QUIT 221). The internal name avoids colliding with the
  TwinCAT-compat `FB_Smtp` alias.
- **Wrappers**: `compat/tc2/tc2_sms_sms.st` provides `FB_SmtpV3`
  (unauthenticated) and `FB_SmtpV3_Full` (optional AUTH LOGIN),
  `FB_Smtp`/`FB_SmtpFull` obsolete aliases, `FB_SmtpAttach`
  (always fails with ERRID 90, attachments unsupported), and
  `FB_GetTcPlcSMS_Version` (`TetherPLC-TcSMS 1.0`).
- **SMS**: `FB_SendSMS` is a pure-ST GSM AT-command state
  machine over the existing `FB_Serial` wrappers (tplc-serial):
  `AT` → `AT+CMGF=1` → `AT+CMGS="<number>"` → wait `>` prompt →
  text + Ctrl-Z → wait `+CMGS`/`OK`. Text mode only, single
  GSM-7 segment (160 chars), no PDU mode/UCS2/concatenation/
  status reports. nStatus: 0=ok 1=timeout 2=modem/protocol
  3=serial 4=generic.
- **Serial EXEC contract**: `FB_Serial` arms on a 0->1 EXEC
  edge and `__last_exec` tracks the previous scan's EXEC. A
  consumer that pulses EXEC must call the FB every scan so
  EXEC sees a low scan between transactions; gating calls by
  phase freezes EXEC high after a fast write and swallows the
  next kick (phantom DONE). `FB_SendSMS` therefore drives both
  serial FBs unconditionally with `bExecute` gated instead.
- **Verified**: `tc2_sms_sms` ctest builds both libraries and
  a consumer, then replays 700 free-running sim steps against
  `tests/compat/gsm_modem_peer.py` (pty AT modem) and
  `tests/compat/smtp_test_peer.py` (RFC 5321 on TCP 2525).
  Nine packed checks (`0x1ff`): serial open, SMS send (peer
  logged `SMS to=+4915123456`), SMTP plain + AUTH LOGIN (both
  MSG lines), RCPT 550 reject, attach ERRID 90, version
  string, refused connect, serial close.
- **Honest gaps**: no TLS/SSL, attachments, delivery receipts,
  Cc/Bcc, multi-recipient envelopes, or MIME headers beyond
  From/To/Subject/Date; no TwinCAT SMTP-server ADS proxy; no
  DNS (dotted-quad IPv4 only); SMS PDU/UCS2/concatenated/status
  modes unimplemented; modem must support text mode AT.

### C4 coverage notes (tplc-iec870 / Tc2_IEC60870_5_10x)

- **Scope**: IEC 60870-5-104 telecontrol client over TCP
  (default port 2404), the TCP variant of the IEC 60870-5-10x
  family. TwinCAT's high-level FBs are driven by AO databases,
  AMS objects, `PVOID` pointers to arrays, interfaces, HRESULTs,
  and generated server objects, none of which the ST frontend or
  runtime can express, so this tranche implements the wire
  protocol plus the pure helper functions and documents the rest
  as gaps (below).
- **APCI framing**: `0x68` start octet + 1-byte length (counts
  bytes after the length octet) + 4-octet control field.
  I-format carries 15-bit send/recv sequence numbers shifted
  left one; S-format is the fixed 6-byte ack (`0x01` first
  control byte); U-format is the fixed 6-byte control frame
  (STARTDT ACT/CON `0x07`/`0x0B`, STOPDT `0x13`/`0x23`, TESTFR
  `0x43`/`0x83`). Handshake: TCP connect, STARTDT ACT, wait
  STARTDT CON, then active data transfer. Close sends STOPDT
  ACT and closes without awaiting confirm. TESTFR keepalive at
  the configured interval.
- **ASDU**: `type_id`, `vsq` (bit7 sequence-addressing, low 7
  bits object count), COT (low byte cause, upper byte originator
  when COT size is 2), common address, then raw information-
  object payload. CA/COT widths configurable 1 or 2 octets
  (default 2). The service does not implement per-type
  information-object codecs; user ST supplies and interprets
  the raw payload bytes.
- **Builtins**: `FB_Iec870Conn` (session: sHost/nPort/nCA_Size/
  nCOT_Size/bExecute/bClose/tTimeout, nConnHandle/nStatus/
  nSubState/nErrId), `FB_Iec870Tx` (one ASDU per EXEC edge, one
  outstanding transmit per connection, k-window aware),
  `FB_Iec870Rx` (consume-on-read, one queued ASDU per DONE,
  bounded queue with drop accounting). All service ops are
  nonblocking and driven by the injected host clock, preserving
  deterministic sim replay.
- **Wrappers**: `compat/tc3/tc3_iec870.st` provides
  `FB_IEC870_5_104Master` session wrapper,
  `FB_IEC870_SendAsdu` / `FB_IEC870_RecvAsdu` raw-ASDU wrappers,
  COT and TYPEID string helpers for IEC 101/102/103,
  `SYSTEMTIME_TO_CP56Time2a` / `CP56Time2a_TO_SYSTEMTIME`
  FUNCTION_BLOCKs (multi-output time conversion), CP56 to
  CP24/CP32 truncation, IOA composition, `F_iecGetSPI`/
  `F_iecGetDPI`/`F_iecIncVTI` bit helpers, and
  `GetIec870_Version` (`16#0100`).
- **CP56Time2a**: 7 octets; octets 0-1 are the little-endian
  millisecond count within the minute (`second*1000 + ms`,
  max 59999), octet 2 packs minute + SUC/IV, octet 3 hour +
  summer-time, octet 4 day-of-week + day-of-month, octet 5
  month, octet 6 year offset from 1900 with IV2.
- **Verified**: `tc3_iec870` ctest builds the library and a
  consumer, then replays 400 free-running sim steps against
  `tests/compat/iec870_test_peer.py`, a python IEC
  60870-5-104 slave on TCP 2480. Twelve packed checks
  (`0xfff`): connection ready, C_IC_NA_1 interrogation sent,
  M_SP_NA_1 + M_ME_NA_1 received, C_CS_NA_1 clock sync sent
  and echoed, C_SC_NA_1 single command sent and act-con
  received, refused-port connection error, CP56Time2a
  roundtrip + string/IOA/bit/version helpers, and STOPDT
  close.
- **Honest gaps**: no IEC 60870-5-101 serial, 5-102 HDLC, or
  5-103 relay-protection transports (104 only); no slave/server
  mode; no per-type information-object codecs (raw payload
  passthrough); no AO database, AMS NetID routing,
  PVOID/pointer-to-array/interface/HRESULT object model,
  redundancy groups, file transfer, or TLS; dotted-quad IPv4
  only, no DNS; k-window and sequence-number edge cases are
  handled at the protocol layer but not exhaustively fuzzed
  against every ASDU type.

### C5 coverage notes (tplc-motion / FB_MotionAxis)

- **Backend**: `include/tplc/tplc_motion.h` defines
  `TplcMotionService` (ABI v1, service name `"tplc-motion"`): a
  bounded pool of simulated CiA402/DS402-shaped axes. Each axis
  exposes the DS402 object subset the MC layer needs
  (Controlword 0x6040, Statusword 0x6041, ModesOfOperation
  0x6060/Display 0x6061, TargetPosition 0x607A, TargetVelocity
  0x60FF, TargetTorque 0x6071, PositionActualValue 0x6064,
  VelocityActualValue 0x606C, TorqueActualValue 0x6077,
  FollowingErrorWindow 0x6065, plus error code 0x603F and a sim
  home-offset 0x607C) through `motion_axis_create` /
  `motion_axis_destroy` / `motion_axis_poll` /
  `motion_axis_info` / `motion_obj_read` / `motion_obj_write`.
- **State machine**: `runtime/host/src/MotionService.cpp` runs
  the CiA402 controlword-command model: the low nibble
  (EnableVoltage/QuickStop/SwitchOn/EnableOperation) selects the
  target state and bit 7 FaultReset clears a latched fault,
  producing the standard statusword patterns (SwitchOnDisabled
  0x40, ReadyToSwitchOn 0x21, SwitchedOn 0x23, OperationEnabled
  0x27, QuickStopActive 0x07, Fault 0x08). Operating modes
  mirror CiA402Defs: ProfilePosition=1, Velocity=2,
  ProfileVelocity=3, ProfileTorque=4, Homing=6,
  CyclicSyncPosition=8, CyclicSyncVelocity=9,
  CyclicSyncTorque=10.
- **Dynamics**: `motion_axis_poll` (and every obj read/write)
  advances all axes by the elapsed `HostServices::steadyNs`
  delta, so `--sim` steps replay deterministically. Profile
  modes integrate position/velocity/torque toward the target
  under configurable velocity/acceleration limits;
  ProfilePosition honors the NewSetPoint edge and reports
  TargetReached (statusword bit 10) inside the follow window;
  cyclic-sync modes apply the target directly each cycle.
- **Builtin**: `FB_MotionAxis` is a synchronous op-dispatch
  bridge (`compiler/tplc-mir/src/layout.rs` +
  `compiler/tplc-rt/src/lib.rs`): set `OP` plus the config /
  `AXIS` / `INDEX` / `VALUE` fields and call; `STATUS` carries
  the service return code (for `OP=6` info it carries the
  CiA402 state). `OP` 1=create (returns `AXIS`), 2=destroy,
  3=poll, 4=read object (`INDEX`->`VALUE`), 5=write object,
  6=info (full snapshot into `MODE`/`FAULT`/`TARGET_REACHED`/
  `BUSY`/`POS_ACTUAL`/`VEL_ACTUAL`). All calls are immediate;
  the compat layer gates create/destroy timing, so no EXEC edge
  is needed.
- **Verified**: `tc2_motion` ctest builds
  `tests/compat/tc2_motion_case.st` and replays 500 free-running
  sim steps against the in-process `tplc-motion` service. Twelve
  packed checks (`0xfff`): axis create, mode/target/controlword
  writes, OperationEnabled reached, PP move target-reached
  within the follow window, velocity mode settle, bad-object /
  bad-mode / bad-handle error paths, destroy, and destroyed-
  handle rejection.
- **Tether IO backend**: `runtime/host/src/MotionIoService.cpp`
  implements the same `TplcMotionService` vtable over the Tether
  IO protocol when `TPLC_MOTION_IO` names a reachable endpoint
  (`tcp://host:port` or `host:port`); `HostServices` prefers it
  over the in-process `MotionService`, which remains the offline
  fallback. Object ops become `getParam`/`setParam` on the
  `mc.<slave>.<index>` params that `tether_io_motion_server`
  (Tether `examples/tether_io_motion_server.cpp`) exposes for its
  `SimulatedCiA402Drive` fleet (`include/tether/io/
  SimulatedCiA402Drive.hpp`, an `IMachineSdoAccess` backend running
  the CiA402 state machine and profile dynamics). Axis handles map
  to free slave indices; the service's `svc_` table is the first
  member so the rt bridge's `&svc_` pointer recovers the object.
  The server runs `--clock sim` (default) so a writable U64
  `mc.time` param carries the PLC `--sim` timestamp (the service
  writes host steady ns / 1000 before each op, deduplicated) and
  replay stays deterministic; `--clock realtime` uses the wall
  clock for live PLCs. Wire `writeFn` is void, so the client
  pre-validates mode-of-operation and clamps the follow window to
  match the local backend, and `motion_axis_create` validates but
  does not propagate axis dynamics config to the remote drive.
- **Verified**: `tc2_motion_io` ctest replays the same
  `tc2_motion_case.st` (mask `0xfff`) through `tether-plc-sim`
  against a live `tether_io_motion_server --clock sim`, exercising
  create/mode/target/controlword writes, OperationEnabled, PP
  target-reached, velocity mode, error paths and destroy over the
  wire; `tc2_mc2` also passes against the remote backend.
- **MC2 axis-management wrappers**: `compat/tc2/tc2_mc2.st`
  provides `MC_Power` (level-driven controlword 0x6040: 0x000F
  enable / 0x0000 disable, `Status` from OperationEnabled state
  0x27), `MC_Reset` (edge-triggered 0x008F fault-reset pulse that
  releases to 0x000F the next scan and leaves the controlword to
  `MC_Power` otherwise), `MC_SetPosition` (edge-triggered
  PositionActualValue 0x6064 write, absolute or relative via
  `Mode`), `MC_ReadActualPosition`/`MC_ReadActualVelocity`/
  `MC_ReadAxisError` (cyclic object reads with `Valid`), and
  `MC_ReadStatus` (DS402 state to PLCopen operating-state map).
  `AXIS_REF` is a UDINT axis handle and positions are int32 user
  units, both documented simplifications. `MC_SetPosition` and
  `MC_Reset` write only on their execute edge / reset window so
  they do not fight `MC_Power` or re-apply a stale `Position`
  input.
- **Verified**: `tc2_mc2` ctest builds `tc2_mc2.st` as a library,
  consumes it via `--lib` into `tests/compat/tc2_mc2_case.st`,
  and replays 300 free-running sim steps. Ten packed checks
  (`0x3ff`): axis create, `MC_Power` enable, `MC_SetPosition`
  absolute and relative readback, `MC_Read*` validity,
  `MC_Reset`, `MC_Power` disable, and disabled operating state.
- **Honest gaps**: the MC2 command blocks
  (`MC_MoveAbsolute`/`MC_MoveVelocity`/`MC_Stop`/`MC_Halt`/
  `MC_Home`/`MC_Jog`), the full PLCopen state machine, and
  gearing/phasing/superposition/touch-probe/parameter FBs land in
  later tranches. `AXIS_REF` is a UDINT handle rather than the
  TwinCAT structure, and positions/velocities are int32 user units
  rather than LREAL engineering units (no `LREAL_TO_DINT`
  conversion exists yet). The model is a deterministic simulation,
  not hardware: no EtherCAT/CiA-402-over-ADS transport, no real
  following-error dynamics, and torque-mode gain is a fixed sim
  constant. A production backend can map the same object dictionary
  onto Tether's DS402/EtherCAT drives (see DECISIONS D-IMPL-52).

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
