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
                      MC_ReadStatus (8-state PLCopen machine), plus the
                      command FBs MC_MoveAbsolute/Relative/Modulo/
                      Velocity, MC_Stop, MC_Halt, MC_Home, MC_Jog,
                      MC_SetOverride, and the axis-parameter FBs
                      MC_Read/WriteParameter + MC_Read/WriteBool-
                      Parameter (ParameterNumber = DS402 object index).
                      AXIS_REF is a UDINT axis handle; positions are
                      int32 user units (TwinCAT LREAL is a documented
                      gap). Command FBs drive the DS402 controlword
                      (NewSetPoint PP/HM, QuickStop MC_Stop) + mode/
                      target objects; per-command dynamics inputs are
                      signature-compatible but the axis applies its
                      configured limits. Gearing/phasing/camming/
                      touch-probe land in a later tranche.
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
  tc2_s5s7com.st     Tc2_S5S7Com S5 side over the builtins
                      FB_S5Conn / FB_S5Io (tplc-s5 service):
                      Siemens S5 client via RK512 job protocol
                      over TCP with 3964 STX/BCC/ETX framing —
                      FB_NLinkOpen session wrapper plus
                      FB_NLinkFetch / FB_NLinkWrite transaction
                      wrappers on the DB area
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
  tc2_s5s7com_case.st  autonomous S5 program (6 packed checks:
                      connect, DB fetch, write+readback,
                      unknown-DB s5err, refused connect,
                      session close)
  tc2_s5s7com.sh       ctest tc2_s5s7com: library + consumer +
                      300-step sim replay expecting mask 0x3f,
                      python S5 peer (s5_test_peer.py)
  s5_test_peer.py      minimal Siemens S5 RK512/3964 responder
                      on TCP 1103 used by tc2_s5s7com.sh:
                      STX/BCC/ETX framing, FETCH/SEND job
                      header; DB1 + DB2 memory areas
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
  tc2_mc2_modulo_case.st Tc2_MC2 tranche-4 program (12 packed checks:
                      axis create, MC_Power enable, MC_MoveModulo
                      shortest + positive-direction with readback,
                      MC_Write/ReadParameter follow-window roundtrip,
                      MC_Read/WriteBoolParameter, MC_MoveVelocity +
                      MC_SetOverride halving + PLCopen state checks,
                      MC_Halt, MC_Power disable)
  tc2_mc2_modulo.sh    ctest tc2_mc2_modulo: builds tc2_mc2.st as a
                      library, consumes via --lib, 401-step sim replay
                      expecting mask 0xfff, in-process tplc-motion
  tc2_mc2_modulo_io.sh ctest tc2_mc2_modulo_io: same
                      tc2_mc2_modulo_case.st replayed over the Tether
                      IO protocol against tether_io_motion_server
                      (--clock sim) via TPLC_MOTION_IO; skips when the
                      Tether server binary is unavailable
  tc2_mc2_gear_case.st Tc2_MC2 tranche-5 program (9 packed checks:
                      two axes created, both powered, master velocity,
                      MC_TouchProbe latch in window, MC_GearIn 2:1
                      coupling, MC_GearOut slave stop, MC_Halt master
                      stop, both powered off)
  tc2_mc2_gear.sh    ctest tc2_mc2_gear: builds tc2_mc2.st as a
                      library, consumes via --lib, 501-step sim replay
                      expecting mask 0x1ff, in-process tplc-motion
  tc2_mc2_gear_io.sh ctest tc2_mc2_gear_io: same
                      tc2_mc2_gear_case.st replayed over the Tether IO
                      protocol against tether_io_motion_server
                      (--clock sim, --axes 4) via TPLC_MOTION_IO; skips
                      when the Tether server binary is unavailable
  tc2_nci_case.st    Tc2_NCI program (8 packed checks: 3-axis group,
                      powered, FB_NciLine move to (1000,600,300),
                      endpoints hit, path length, return move, origin,
                      group E-stop)
  tc2_nci.sh         ctest tc2_nci: builds tc2_mc2.st + tc2_nci.st as
                      libraries, consumes via --lib, 501-step sim
                      replay expecting mask 0xff, in-process tplc-motion
  tc2_nci_io.sh      ctest tc2_nci_io: same tc2_nci_case.st replayed
                      over the Tether IO protocol against
                      tether_io_motion_server (--clock sim, --axes 4)
                      via TPLC_MOTION_IO; skips when the Tether server
                      binary is unavailable
  tc2_nci_gcode_case.st
                     Tc2_NCI G-code channel program (14 packed
                      checks: text upload, file load, READY +
                      block counts, run, mid-program line, finish,
                      duration, stop, reset, IDLE)
  tc2_nci_gcode_io.sh
                     ctest tc2_nci_gcode_io: drives the server's
                      nc.* channel over Tether IO (G-code
                      interpreted by Tether's interpreter +
                      motion planner, CSP streaming to the fleet)
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
| C5 | Motion core | `tplc-motion` backend done (simulated CiA402/DS402 axis pool + `FB_MotionAxis` builtin); Tether-IO-backed backend done (`MotionIoService` maps object ops to `mc.<slave>.<index>` params on `tether_io_motion_server`/`SimulatedCiA402Drive`, deterministic via `mc.time` + `--clock sim`, in-process fallback retained); `compat/tc2/tc2_mc2.st` axis-management + command wrappers done (MC_Power/MC_Reset/MC_SetPosition abs+rel/MC_Read* plus MC_MoveAbsolute/Relative/Modulo/Velocity/MC_Stop/MC_Halt/MC_Home/MC_Jog/MC_SetOverride/MC_Read|WriteParameter(+Bool), an 8-state PLCopen operating state, and coupling/probe blocks (MC_TouchProbe cycle-level latch, MC_GearIn velocity-ratio coupling, MC_GearOut; e2e on both backends); exact hardware-latch touch-probe, `MC_AbortTrigger`, phase-locked gearing/phasing pending; `compat/tc3/tc3_nccam.st` single-table cam coupling done (e2e `tc3_nccam{,_io}`) |
| C6 | NC interpolation | `compat/tc2/tc2_nci.st` interpolation core done (`FB_NciLine` coordinated 3-axis linear CSP move, `F_NciSqrt`, `FB_NciEStop`, e2e on both backends); **Tether-owned G-code channel done**: `tether_io_motion_server` compiles program text/files with `GCode::PlanningSegmentBuilder`, builds a `MotionPlanner::MotionPlan` (corner blending, jerk-limited profile, linear/arc/dwell), and streams CSP `0x607A` setpoints to slaves 0-2 on the `mc.time` clock — PLC side is the `tplc-nci` service + `FB_NciChan` builtin + `FB_ItpLoadText`/`ItpLoadProg`/`ItpStartStop`/`ItpReset`/`ItpEStop`/`ItpStatus`/`ItpFiFoWrite`/`ItpConfirmHsk`/`ItpWriteRParam`/`ItpReadRParam`/`ItpZeroShift` wrappers (e2e `tc2_nci_gcode_io` mask `0x7fffff`: FIFO queue + run-from-idle + chained blocks, R-param roundtrip, zero-shift, M8 pause→confirm→resume); cyclic-channel offsets pending |
| C7 | Connectivity wave 2 | `tplc-opcua` **done** (OPC UA client via libopen62541 — opt-in like other special features, auto-detected, absent → `getService` nullptr): `FB_UaClient` builtin + `FB_UA_Connect`/`FB_UA_Disconnect`/`FB_UA_ConnectGetStatus`/`FB_UA_Read`/`FB_UA_Write`/`FB_UA_GetNamespaceIndex` wrappers in `compat/tc2/tc2_opcua.st` (TwinCAT Tc2_OPCUA/Tc3_PlcOpenOpcUa shape); the endpoint is Tether's `tether_io_motion_server --opcua` (open62541 server exposing the motion Registry as `ns=2;s=<name>` nodes); e2e `tc2_opcua_io` — connect/session/read `nc.state`/write+read `nc.scale`/ns-index/disconnect; node-handle registration, subscriptions, methods, history pending. `tplc-eip` **done** (EtherNet/IP explicit-messaging client — RegisterSession/ListIdentity/SendRRData + CIP Get/Set_Attribute_Single, nonblocking poll-bounded IO): `FB_Eip` builtin + `FB_EipConnect`/`Disconnect`/`Status`/`ListIdentity`/`GetAttribute`/`SetAttribute` wrappers in `compat/tc2/tc2_eip.st`; e2e `tc2_eip_io` vs `eip_adapter_fake.py` — register/identity/attr r-w/disconnect `0x7f`; cyclic class-1 I/O, ForwardOpen, CIP services beyond Get/Set pending. `tplc-iec61850` **done** (IEC 61850 MMS client via libiec61850 — opt-in, auto-downloaded + built via FetchContent, absent → `getService` nullptr): `FB_Iec61850` builtin + `FB_Iec61850Connect`/`Disconnect`/`Status`/`Read`/`Write`/`GetNameList` wrappers in `compat/tc2/tc2_iec61850.st` (domain+item addressing, `#FC` suffix selects functional constraint); e2e `tc2_iec61850_io` vs `iec61850_ied_fake` (real libiec61850 IedServer) — associate/status/read/write-error/namelist/disconnect `0x7f`; reporting/RCB, GOOSE/SMV, control sequences pending. `tplc-mqtt` **done** (MQTT 3.1.1 client via libpaho-mqtt3c — opt-in, FetchContent fallback, absent → `getService` nullptr): `FB_Mqtt` builtin + `FB_IotMqttClient`/`Disconnect`/`Publish`/`Subscribe`/`Unsubscribe`/`Receive` wrappers in `compat/tc2/tc2_mqtt.st`; e2e `tc2_mqtt_io` vs `mqtt_broker_fake.py` — connect/subscribe/publish/echo/disconnect `0x1f`; MQTT5 properties, wildcard detail, QoS2, retained, TLS pending. `tplc-ocpp` **done** (OCPP-J charge-point client — WebSocket RFC6455 + OCPP JSON-RPC framing, always available): `FB_Ocpp` builtin + `FB_OcppConnect`/`Disconnect`/`Call`/`Result`/`CallError`/`Receive` wrappers in `compat/tc2/tc2_ocpp.st`; e2e `tc2_ocpp_io` vs `ocpp_csms_fake.py` — WS upgrade/call/result/disconnect `0xf`; OCPP 2.x, security profiles, schema validation pending. `tplc-aes70` **done** (AES70/OCA audio-control client over OSCP TCP — OCP1 + OcaMessage PDU codec, always available): `FB_Aes70` builtin + `FB_Aes70Connect`/`Disconnect`/`Call`/`Keepalive`/`Receive` wrappers in `compat/tc2/tc2_aes70.st`; e2e `tc2_aes70_io` vs `aes70_device_fake.py` — connect/keepalive/call/response/disconnect `0x1f`; notification subscriptions, full OCA catalog, property shorthands pending. `tplc-dmx` **done** (DMX512 output over an opt-in serial port — TIOCSBRK break + MAB + start code + 512 ch at 250 kbaud): `FB_Dmx` builtin + `FB_DmxOpen`/`Close`/`SetChannel`/`SetChannels`/`Send`/`Status` wrappers in `compat/tc2/tc2_dmx.st`; e2e `tc2_dmx_io` vs `dmx_rx_fake.py` (PTY fake) — open/set/send/status/close `0x1f`; RDM + receive pending. DALI/EIB/LON/MBus/EnOcean follow the same opt-in-serial pattern |

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
  - S5 links are covered by the separate `tplc-s5` service
    (RK512/3964, DB-area FETCH/SEND subset) — see the
    `Tc2_S5S7Com` notes below; serial 3964(R) links and the
    non-DB RK512 areas remain unimplemented.
  - Rack/slot map only into the COTP called TSAP; no ISO-on-TCP
    port negotiation beyond the fixed/default port, no DNS,
    no IPv6.

### C4 coverage notes (tplc-s5 / Tc2_S5S7Com S5 side)

- **Transport**: `tplc-s5` is an RK512 job-protocol client over
  TCP (default port 102, `TPLC_S5_PORT`) carrying 3964
  STX/BCC/ETX frames. The serial 3964(R) DLE link-acquisition
  handshake is not used; over TCP the frames are sent
  byte-streamed. BCC is the XOR of every message byte plus the
  trailing ETX; ETX bytes inside payloads are disambiguated by
  BCC verification rather than DLE stuffing.
- **Protocol subset**: RK512 command telegram 10-byte header
  (id `00 00`, order type `'E'` FETCH / `'A'` SEND, area `'D'`
  DB, DB number, word offset, word length big-endian, flags
  `0xFF`) and a 4-byte response header (id/type echo, status
  byte). DB-area word access only: DB 0..255, word offset
  0..255 words (byte offsets 0..510), at most 255 words per
  transaction, one outstanding io per connection. SEND payloads
  are transferred most-significant-byte first and FETCH
  responses return wire-order bytes; `BYTE_OFF`/`LEN` in the
  FBs are byte units converted to words by the service. Other
  RK512 areas (I/O, memory, flags, timers, counters, DX),
  continuation telegrams, and job priorities are not
  implemented.
- **Builtins**: `FB_S5Conn` (EXEC opens the TCP session, no
  handshake; STATUS 0=pending 1=ready 2=error, CLOSE edge
  tears down) and `FB_S5Io` (MODE 0=FETCH 1=SEND on a live
  CONN_HANDLE; STATUS 0=pending 1=ok 2=timeout 3=error
  4=badresp 5=s5err).
- **Wrappers**: `compat/tc2/tc2_s5s7com.st` provides
  `FB_NLinkOpen` (covers `FB_NLinkOpen`/`FB_NLinkClose` and the
  S7-side `FB_NLink*` connect pair), `FB_NLinkFetch` /
  `FB_NLinkWrite` (DB-area only; `POINTER TO BYTE` buffers are
  `ARRAY[0..509] OF BYTE`, `nArea` accepts only the DB value),
  and `F_GetVersionTcS5S7Comm`.
- **Verified**: `tc2_s5s7com` ctest builds the library and a
  consumer program, then replays 300 free-running sim steps
  against `tests/compat/s5_test_peer.py` — a python RK512/3964
  responder on TCP 1103. Six packed checks (`0x3f`): connect,
  DB1 fetch with pattern check, write+readback, unknown-DB
  status byte → `STATUS = 5`, refused connect → conn
  `STATUS = 2`, session close.

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
- **Coupling and probe wrappers**: `MC_TouchProbe` captures
  `PositionActualValue` on the `TriggerInput` rising edge with
  optional `WindowOnly`/`FirstPosition`/`LastPosition` gating, and
  rearms on each `Execute` rising edge. `MC_GearIn` couples a slave
  to a master by velocity ratio: on `Execute` it enables the slave
  and selects ProfileVelocity, then each scan writes slave
  `TargetVelocity` 0x60FF = master `VelocityActualValue` 0x606C *
  `RatioNumerator` / `RatioDenominator` (zero denominator -> Error).
  `MC_GearOut` writes slave 0x60FF = 0 on its edge and reports
  `Done` at rest. `MC_AbortTrigger` is intentionally absent because
  the model has no shared per-axis probe state (see DECISIONS
  D-IMPL-55).
- **Verified**: `tc2_mc2_gear` and `tc2_mc2_gear_io` ctests replay
  `tc2_mc2_gear_case.st` (mask `0x1ff`) against the in-process and
  Tether IO backends. Nine checks: two axes created, both powered,
  master velocity, `MC_TouchProbe` latch inside one scan of the
  trigger, `MC_GearIn` 2:1 slave velocity, `MC_GearOut` slave stop,
  `MC_Halt` master stop, both powered off.
- **Camming wrappers**: `compat/tc3/tc3_nccam.st` provides
  `FB_McCamCoupling` (`TF5050_TC3_NC_Camming` shape): a cam table
  stored as public `MasterPosT`/`SlavePosT` LREAL arrays (the
  `MC_CAM_REF` flattening, up to 64 points) with `WritePt` edge
  writes and `ClearTable`, plus `Execute` coupling that polls both
  `FB_MotionAxis` handles each scan, reads master
  `PositionActualValue`, interpolates the table (linear, clamped
  ends, optional periodic wrap), and streams the scaled slave
  position into target `0x607A` — the slave must be powered in CSP
  mode. `Invert` swaps master/slave roles, `CamSlavePos` reports
  the last interpolated value. The remaining TF5050 APIs
  (multi-plate `MC_CamAdd`/`Exchange`/`Remove` blending,
  `MC_CamTableSelect`/`MC_CamInfo` table management, online-change
  modes, motion-function read/write beyond the point edge,
  slave-dynamics and characteristics reads) are not implemented.
- **Verified**: `tc3_nccam` and `tc3_nccam_io` ctests replay
  `tc3_nccam_case.st` (mask `0x3f`) against the in-process and
  Tether IO backends. Six checks: two axes created, 3-point table
  loaded, master velocity move with slave tracking 2x through the
  table, periodic wrap past the last master point, table clear,
  `WritePt` edge write.
- **Honest gaps**: `MC_TouchProbe` is a cycle-level software latch —
  TwinCAT's hardware touch-probe latches sub-cycle against an encoder
  event, but neither the in-process axis nor `SimulatedCiA402Drive`
  exposes a hardware latch object, so capture latency is one scan and
  trigger edges are scan-synchronous. `MC_GearIn` is velocity-ratio
  coupling, not exact position-phase lock — TwinCAT gearing locks
  phase via cyclic setpoint coupling, which needs a dedicated
  coupling object or CSP setpoint streaming. `MC_AbortTrigger` needs
  shared probe state; single-table cam coupling lives in
  `compat/tc3/tc3_nccam.st`, phasing/superposition and multi-plate
  cam blending land in later tranches. `AXIS_REF` is a UDINT handle rather than the
  TwinCAT structure, and positions/velocities are int32 user units
  rather than LREAL engineering units (no `LREAL_TO_DINT`
  conversion exists yet). The model is a deterministic simulation,
  not hardware: no EtherCAT/CiA-402-over-ADS transport, no real
  following-error dynamics, and torque-mode gain is a fixed sim
  constant. A production backend can map the same object dictionary
  onto Tether's DS402/EtherCAT drives (see DECISIONS D-IMPL-52).

### C6 coverage notes (Tc2_NCI channel interpolation)

- **Model**: TwinCAT's NC-I runs a G-code/FIFO interpreter in the NC
  kernel that emits per-axis position setpoints each NC cycle to axes
  in CyclicSyncPosition (CSP, mode 8). `compat/tc2/tc2_nci.st`
  collapses one in-flight geometric block into each FB: `FB_NciLine`
  parameterizes a straight line across a 3-axis group (X/Y/Z axis
  handles), advances the path parameter `s` by `Velocity*CycleUs/1e6`
  per scan, and streams per-axis targets (0x607A) to CSP axes — the
  same way the NC kernel drives CSP axes. There is no backend group
  registry, so a "3D group" is just the three axis handles passed to
  the block (TwinCAT's `CfgBuild3DGroup` maps to passing handles).
- **Time base**: the FB has no host clock, so `CycleUs` supplies the
  scan period; path `Velocity` is user-units/second. Under `--sim`,
  `CycleUs` = the step period makes the path advance a fixed amount
  per step, and CSP snaps each axis to the streamed target so
  commanded and actual positions match — deterministic on both
  backends.
- **Helpers**: `F_NciSqrt` is a fixed-iteration Newton square root
  (no SQRT builtin) used to compute the Euclidean path length.
  `FB_NciEStop` writes DisableVoltage to all three group axes.
- **Verified**: `tc2_nci` and `tc2_nci_io` ctests replay
  `tc2_nci_case.st` (mask `0xff`) on in-process and Tether IO
  backends — power, a coordinated move to (1000,600,300) with every
  axis hitting its coordinate, path length 1204, a return move, and a
  group E-stop. Implementing it surfaced and fixed a real
  `tether_io_motion_server` bug: the per-object `readFn`/`writeFn`
  captured `slave` by reference (`[&, idx, sz]` drops it under the
  `&` default), so every param operated on the last-configured slave
  — invisible for single-axis use, breaking multi-axis groups.

#### Tether-owned G-code channel (`nc.*`)

- **Model**: G-code interpretation is owned by Tether, not the PLC.
  `tether_io_motion_server` exposes an `nc.*` channel: `nc.prog`
  (text) or `nc.prog_path` (server-side file) compiles the program
  with `GCode::PlanningSegmentBuilder`; `nc.cmd` runs commands
  (run/step/pause/resume/stop/reset/verify); on RUN the channel
  builds a `MotionPlanner::MotionPlan` — corner blending plus the
  planner's velocity profile over linear, arc and dwell segments —
  then streams scaled CSP `0x607A` setpoints to slaves 0..2 on the
  `mc.time` drive clock (deterministic under `--clock sim`).
- **Status params**: `nc.state` (Idle/Ready/Running/Paused/
  Finished/Error/Stopped), `nc.line` (executing source line),
  `nc.total` (parsed blocks), `nc.errcode`, `nc.progress`/
  `nc.duration` (plan seconds), `nc.scale`, `nc.feedovr`,
  `nc.fifolen`, `nc.fifoseq`, `nc.mfunc`, `nc.lastx/y/z`.
- **FIFO**: `nc.fifo` accepts one G-code block per write (trial-
  compiled; bad entries report the error immediately). Queued
  blocks chain after the active plan, each re-anchored at the last
  commanded position (`nc.lastx/y/z`) — a FIFO-only program also
  runs from Idle. `nc.fifolen`/`nc.fifoseq` report queue depth and
  entries consumed.
- **M-functions**: blocks carrying an M-code outside the program-
  control set (M0/M1/M2/M30 are interpreter-owned) pause the
  channel with `nc.mfunc` set to the pending code; a write to
  `nc.confirm` clears it and resumes.
- **R-params / zero-shift**: `nc.ridx`+`nc.rval` read/write
  numbered `#`-registers (mapped to the interpreter's
  `VariableSystem` via injected `#n = v` prefix lines), `nc.zsx/
  y/z` inject a `G92` prefix. Both apply on the next compile and
  are stripped from the visible block/line numbering.
- **PLC side**: `tplc-nci` service (`include/tplc/tplc_nci.h`) is
  served by `NciIoService` only when `TPLC_MOTION_IO` names an
  endpoint exposing `nc.*` — there is intentionally no in-process
  interpreter, so `getService("tplc-nci")` returns null without a
  Tether endpoint. `FB_NciChan` is the op-multiplexed builtin
  (1=load text, 2=load file, 3=command, 4=status, 5=FIFO append,
  6=confirm, 7/8=R-param write/read, 9=zero-shift) and
  `compat/tc2/tc2_nci.st` wraps it in TwinCAT `Itp*` shape:
  `FB_ItpLoadText`, `FB_ItpLoadProg`, `FB_ItpStartStop`,
  `FB_ItpReset`, `FB_ItpEStop`, `FB_ItpStatus`, `FB_ItpFiFoWrite`,
  `FB_ItpConfirmHsk`, `FB_ItpWriteRParam`, `FB_ItpReadRParam`,
  `FB_ItpZeroShift`.
- **Verified**: `tc2_nci_gcode_io` replays `tc2_nci_gcode_case.st`
  (mask `0x7fffff`) over Tether IO — inline-text upload (2 blocks),
  server-side file load (7 blocks), run, mid-program line tracking,
  finish, duration sanity, permissive stop, reset to IDLE, FIFO
  queue/run-from-idle/chained execution, R-parameter write+read
  roundtrip, zero-shift write, and the M8 pause → confirm →
  resume handshake. Step semantics verified by direct probe (one
  source block per step, then pause).
- **Honest gaps**: no cyclic-channel offset tables; `FB_NciLine`
  remains the interpreter-free single-block path. The in-process
  backend has no channel (by design). (DECISIONS D-IMPL-56,
  D-IMPL-57, D-IMPL-58).

### C7 coverage notes

- **Model**: the PLC is the OPC UA *client*. `tplc-opcua` wraps
  libopen62541 — present only when the dev package was linked at
  build time (auto-detected like the other optional deps, or
  `-DTPLC_ENABLE_OPCUA=OFF`); without it `getService("tplc-opcua")`
  returns nullptr and every `FB_UA_*` reports Error + `ErrorID 99`.
  The endpoint for the simulated path is Tether's
  `tether_io_motion_server --opcua` listener — an open62541 server
  folding the whole motion Registry into `ns=2;s=<name>` Variable
  nodes with DataSource read/write callbacks.
- **Surface**: `FB_UaClient` is the op-multiplexed builtin
  (1=connect, 2=disconnect, 3=status, 4=read, 5=write,
  6=namespace-index); `compat/tc2/tc2_opcua.st` wraps it in the
  TwinCAT `UA_*` shape — `FB_UA_Connect`, `FB_UA_Disconnect`,
  `FB_UA_ConnectGetStatus`, `FB_UA_Read`, `FB_UA_Write`,
  `FB_UA_GetNamespaceIndex`. Node ids are passed as text
  (`ns=2;s=name`, `i=85`, `ns=1;i=1001`, bare `name` → `ns=2;s=`).
- **Value model**: numeric kinds widen into `Value : LREAL`
  (signed/unsigned 64-bit nodes lose precision above 2^53 — an
  honest LREAL limit); strings land in `StrValue : STRING[255]`;
  `ValueType` carries the `TPLC_UA_TYPE_*` code
  (1=BOOL 2=I64 3=U64 4=F64 5=STRING).
- **Verified**: `tc2_opcua_io` replays `tc2_opcua_case.st` over a
  real open62541 session — connect, session-open status, read
  `nc.state`, write+read `nc.scale`, namespace-index lookup,
  disconnect.
- **Honest gaps**: no browse surface in the wrappers (the server
  supports it — open62541 does it natively), no node-handle
  registration (`UA_NodeGetHandle`/`ReleaseHandle` — node ids are
  resolved per call instead of once up front), no subscriptions,
  method calls, or history; the client surface is the read/write/
  status core TwinCAT apps use most.
  (DECISIONS D-IMPL-59).

### EtherCAT surfaces (`tplc-ecsync` / `tplc-ecdiag`)

- **Model**: both services are Tether-IO bridges — the PLC reads
  `ec.*` params on `TPLC_EC_IO` (falling back to `TPLC_MOTION_IO`)
  served by `tether_io_motion_server --clock sim`; a real master
  bridges the same param names.
- **`tplc-ecsync`** (`compat/tc3/tc3_ecsync.st`): `FB_EcSync`
  builtin + `FB_EcExtSyncExtTimes`/`FB_EcExtSyncIsSynchronized`
  wrappers over the `ec.dc.*` params — e2e `tc3_ecsync_io`.
- **`tplc-ecdiag`** (`compat/tc3/tc3_ecdiag.st`): `FB_EcDiagRead`
  builtin (OP 1=read CoE 0x10F3 diag history, OP 2=ack) +
  `FB_CoE_DiagHistory_Read` wrapper exposing the entry set as
  flattened parallel `aE*` member arrays (no user STRUCTs in this
  dialect; `sMsgText` is a generated `TID=/CODE=` fallback since
  ESI/XML texts are unavailable; `tReadInterval` maps to
  `nReadIntervalScans`).  The wire blob is the 0x10F3 header plus
  22-byte ETG.1020 records; `ec.coediag.slave` selects the port
  address, `ec.coediag.ack` writes newest-acked.  e2e
  `tc3_ecdiag_io` covers read/field decode/ack/per-slave counts.
  `FB_CoE_DiagHistory_ReadAxis` needs Tc2_MC2 `AXIS_REF` objects and
  is not provided.
- **`tplc-soe`** (`compat/tc2/tc2_drive.st`): `FB_Soe` builtin
  (OP 1=read state, OP 2..7=mailbox writes) + the TwinCAT-shaped
  `FB_SoE*`/`FB_SoEAX5000*`/`FB_CoEDrive*`/`FB_SoEDrive*` wrappers
  over the `ec.soe.*` params — `ec.soe.slave` selects the drive,
  `ec.soe.state` is the 128-byte LE snapshot (S-0-0135 status word,
  command state, password, diagnosis number/list/class-X, amplifier
  and motor temperatures, DC bus voltage/current, AX5000 mains/
  ctrl/park/firmware fields, S-0-0095 message), `ec.soe.cmd` is the
  16-byte mailbox (op@0, u32 payload@4: password, command control,
  command exec with bit15 forcing a simulated error, reset, AX5000
  ctrl/park).  The sim drives command state (idle/running 100 ms/
  done/error) on `mc.time`; a real SoE master bridges the same
  params.  e2e `tc2_drive_io` (mask `0x3fff`, 14 checks) covers
  state/diag/password/command/reset/AX5000 plus the
  SimplePlcMotion enable/velocity blocks on a DS402 axis.  Not
  covered: `ST_DriveRef` naming, the `F_ConvWordToST*` STRUCT
  conversions, real mailbox timing/profiles, firmware flashing.

### EtherNet/IP client (`tplc-eip`)

- **Model**: the PLC is the scanner-side explicit-messaging client.
  `tplc-eip` talks the EtherNet/IP encapsulation layer on TCP 44818 —
  `RegisterSession`/`UnRegisterSession`, `SendRRData` carrying
  unconnected CIP `Get_Attribute_Single`/`Set_Attribute_Single`, and
  `ListIdentity` discovery. Cyclic implicit (class-1) I/O is a
  separate surface and deliberately out of scope for this tranche.
- **Codec**: nonblocking `connect`+`poll`-bounded IO throughout —
  every transaction is bounded well under the PLC scan-stall
  watchdog, so a stuck adapter fails the call instead of stalling
  the task. CIP object addressing is class/instance/attribute u16;
  the codec emits the 8-bit or 16-bit path segment automatically.
- **Surface**: `FB_Eip` is the op-multiplexed builtin (1=open+
  RegisterSession, 2=close+unregister, 3=status, 4=ListIdentity,
  5=Get_Attribute_Single, 6=Set_Attribute_Single);
  `compat/tc2/tc2_eip.st` wraps it in the TwinCAT `Tc3_EtherNetIP`/
  TF6281 shape — `FB_EipConnect`, `FB_EipDisconnect`, `FB_EipStatus`,
  `FB_EipListIdentity`, `FB_EipGetAttribute`, `FB_EipSetAttribute`.
  Attribute payloads travel as bounded `STRING[255]` bytes (TwinCAT
  uses PVOID buffers — the string is the practical carrier here);
  `LastCipStatus` exposes the last CIP general status.
- **Verified**: `tc2_eip_io` replays `tc2_eip_case.st` against
  `tests/compat/eip_adapter_fake.py` (a wire-level adapter fake
  answering encapsulation + CIP Get/Set_Attribute_Single) —
  RegisterSession, ListIdentity, attribute write+read roundtrip,
  unregister — mask `0x7f`.
- **Honest gaps**: no cyclic implicit I/O (class-1 connections),
  no connection manager / ForwardOpen, no CIP object services
  beyond Get/Set_Attribute_Single, no multicast/UDP discovery
  (ListIdentity is TCP unicast). (DECISIONS D-IMPL-60).

### IEC 61850 client (`tplc-iec61850`)

  `tplc-iec61850` is an IEC 61850 MMS client — the PLC is the
  control-center side over the full OSI stack via **libiec61850**
  (`IedConnection_*`). Opt-in like the other special features:
  `-DTPLC_ENABLE_IEC61850` (default ON) auto-downloads and builds
  libiec61850 via CMake FetchContent; when the fetch/build fails or
  the option is off the service compiles inert and
  `getService("tplc-iec61850")` returns nullptr.

  `FB_Iec61850` is the runtime builtin (synchronous op dispatch like
  `FB_Eip`): connect+associate (HOST/PORT → HANDLE), close+release,
  status (→ STATE/INVOKE/LASTERR), read (DOMAIN/ITEM → VAL),
  write (DOMAIN/ITEM + VAL), and GetNameList (DOMAIN → VAL, one
  logical node per line).  `FB_Iec61850Connect`/`Disconnect`/
  `Status`/`Read`/`Write`/`GetNameList` in
  `compat/tc2/tc2_iec61850.st` are the TwinCAT-shaped wrappers
  (Tc3_IEC61850/TF6101).  Objects are addressed by domain
  (logical device) + item (VariableAccess name); values travel as
  bounded `STRING[255]` (numerics as their ASCII repr, structured
  values as their MmsValue string).  An `#FC` suffix on the item
  selects the functional constraint (`ST`, `MX`, `CO`, `SP`, `SV`,
  `CF`, `DC`, `SG`, `SE`, `SR`, `OR`, `BL`, `EX`, `US`, `MS`,
  `RP`, `BR`, `LG`, `GO`, `ALL`; default `ST`).

  Every blocking call is bounded below the PLC scan-stall watchdog
  (`IedConnection_setConnectTimeout`/`setRequestTimeout` ≈800 ms)
  so a stuck peer errors out instead of stalling the task.

  e2e `tc2_iec61850_io` runs the surface against
  `iec61850_ied_fake` (a real libiec61850 `IedServer` with a
  `tetherIEDtetherTestLD`/`LLN0` CDC model — Health.stVal ST,
  Meas.instMag.f MX, SetPt.setMag.f SP, Name.val): associate,
  status, read, the write path's error reporting on a read-only
  attribute, name-list, disconnect — `0x7f`.  RCB/reporting,
  data-set, and SBO control tranches are covered by the
  `tc2_iec61850_{rcb,brcb,ctl,ds}_io` ctests; APC controls accept
  scalar ctlVal text which libiec61850 wraps into the
  AnalogueValue setpoint structure (user-defined structured
  ctlVal member lists unsupported).  GOOSE/SMV needs
  CAP_NET_RAW and remains out of scope.

### MQTT client (`tplc-mqtt`)

  `tplc-mqtt` is an MQTT 3.1.1 client — the PLC is the client over
  the broker wire protocol via **libpaho-mqtt3c** (Eclipse Paho
  synchronous `MQTTClient`). Opt-in like the other special features:
  `-DTPLC_ENABLE_MQTT` (default ON) prefers a system libpaho-mqtt3c
  and falls back to a CMake FetchContent build of paho.mqtt.c; when
  absent or off the service compiles inert and
  `getService("tplc-mqtt")` returns nullptr.

  `FB_Mqtt` is the runtime builtin (synchronous op dispatch like
  `FB_Eip`): connect (HOST/PORT/CLEAN/KEEPALIVE → HANDLE),
  disconnect, status (→ STATE/LASTMSGID/LASTERR), publish
  (TOPIC/PAYLOAD/QOS/RETAIN), subscribe (TOPIC/QOS), unsubscribe
  (TOPIC), receive (nonblocking poll → RXTOPIC/RXPAYLOAD/RXQOS/
  RXMSGID/RXPLEN, or NOMSG), and isConnected.  `FB_IotMqttClient`/
  `Disconnect`/`Publish`/`Subscribe`/`Unsubscribe`/`Receive` in
  `compat/tc2/tc2_mqtt.st` are the TwinCAT-shaped wrappers
  (Tc3_IoT/TF6701).  Topics are bounded `STRING[127]`, payloads
  `STRING[255]`; `Retain`/`RETAIN` are reserved keywords so the
  wrapper surfaces them as `RetainOn`/`FLG_RETAIN`.

  e2e `tc2_mqtt_io` runs the surface against
  `mqtt_broker_fake.py` (a real CONNECT/CONNACK/SUBACK/
  PUBLISH-echo peer): connect, subscribe, publish, the broker's
  echo landing in the receive poll, disconnect — `0x1f`.
  MQTT5 properties, wildcard-matching detail, QoS2 two-phase,
  retained messages, and TLS are honest gaps for a later tranche.

### OCPP-J client (`tplc-ocpp`)

  `tplc-ocpp` is an OCPP-J (Open Charge Point Protocol over
  WebSocket + JSON-RPC) charge-point client — the PLC is the
  charge-point side.  Always available (no external lib — TCP +
  RFC6455 WebSocket + OCPP JSON-RPC array framing, hand-rolled
  like `tplc-eip`): `FB_Ocpp` runs connect (HOST/PORT/PATH/OCPPID →
  HANDLE, WS upgrade + ocpp1.6 subprotocol), disconnect, status,
  call ([2,msgId,action,payload]), result ([3,msgId,payload]),
  callerror ([4,msgId,code,desc,details]), and a nonblocking receive
  poll (→ action/payload/msg-type or NOMSG).  `FB_OcppConnect`/
  `Disconnect`/`Call`/`Result`/`CallError`/`Receive` in
  `compat/tc2/tc2_ocpp.st` are the TwinCAT-shaped wrappers
  (Tc3_OCPP/TF6741-ish).  Payloads are raw JSON text bounded
  `STRING[255]` — the caller composes/parses them.  Every socket op
  is poll-bounded under the scan-stall watchdog.

  e2e `tc2_ocpp_io` runs the surface against `ocpp_csms_fake.py` (a
  real WS-upgrade + OCPP-J echo peer): connect, a BootNotification
  call, the echoed result landing in the receive poll, disconnect —
  `0xf`.  OCPP 2.x, security profiles, per-action schema validation,
  and the call-result matching a TwinCAT application would compose
  are honest gaps for a later tranche.

### AES70/OCA client (`tplc-aes70`)

  `tplc-aes70` is an AES70 (Open Control Architecture / OCA) audio-
  control client — the PLC is the OCA controller over OSCP TCP on
  the OCA control port (default 65000).  Always available (no
  external lib — OCP1 sync header + OcaMessage PDU codec,
  hand-rolled like `tplc-eip`): `FB_Aes70` runs connect, disconnect,
  status, a method call on an OCA object (targetONo + methodID +
  parameter bytes → result handle), a keepalive heartbeat, and a
  nonblocking receive poll (→ handle/status-code/payload or NOMSG).
  `FB_Aes70Connect`/`Disconnect`/`Call`/`Keepalive`/`Receive` in
  `compat/tc2/tc2_aes70.st` are the TwinCAT-shaped wrappers
  (Tc3_AES70/TF6301-ish).  Parameters and response payloads are raw
  bytes bounded `STRING[255]` — the caller composes/parses the
  OCA-serialized parameter set.  `METHOD` is a reserved keyword
  surfaced as `METH`.  Every socket op is poll-bounded under the
  scan-stall watchdog.

  e2e `tc2_aes70_io` runs the surface against
  `aes70_device_fake.py` (a real OCP1 frame peer echoing OcaCommands
  as OcaResponses): connect, keepalive, a method call, the echoed
  response landing in the receive poll, disconnect — `0x1f`.
  Notification subscriptions, the full OCA class catalog, and
  property/get-set shorthands are honest gaps for a later tranche.

### Building buses — serial-opt-in surfaces

  The building-bus surfaces layer per-bus framing and device
  management on top of an opt-in serial port — each service opens
  its UART only when its surface is used, like the IP services own
  their sockets.  `tplc-dmx` is the first: a DMX512 output service
  over a serial port at 250 kbaud 8N2 — `FB_Dmx` runs open (PATH →
  HANDLE), close, status, channel set/set-run on the 512-slot
  image, and a bounded send emitting one complete frame (TIOCSBRK
  break + mark-after-break + start code + 512 bytes).
  `FB_DmxOpen`/`Close`/`SetChannel`/`SetChannels`/`Send`/`Status`
  in `compat/tc2/tc2_dmx.st` are the TwinCAT-shaped wrappers
  (Tc3_DMX/TF6501-ish).  DMX-over-IP is already covered by
  `tplc-sacn`; `tplc-dmx` is the serial-media path.

  e2e `tc2_dmx_io` runs the surface against `dmx_rx_fake.py` (a PTY
  fake reading the emitted bytes): open, channel set, a frame sent,
  status counting the frame, close — `0x1f`.  RDM (bidirectional
  DMX) and receive are honest gaps; the PLC-side image is a snapshot
  the scan emits at its own cadence (real DMX runs ~44 Hz).

  DALI, EIB/KNX, LON, M-Bus, EnOcean follow the same
  opt-in-serial pattern pending their own services.



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
