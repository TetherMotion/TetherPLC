// profinet.st — PROFINET DCP discovery/configuration + cyclic IO
// record exchange for TetherPLC.  Wraps the tplc-profinet host
// service: DCP PDUs (12-byte header + option blocks, FrameID
// 0xFEFD/0xFEFE/0xFEFF, services Get/Set/Identify) over per-port
// UDP transports configured via TPLC_PROFINET_IO (comma-separated
// host:port or udp://host:port endpoints; port index selects the
// endpoint).  On real hardware a Layer-2 Ethertype-0x8892 transport
// is the deployment path; UDP is the tunnel/test transport.
//
// FB_ProfinetMaster is the FBD/IDE-facing block: EXEC rising edge
// runs one synchronous round-trip (do not call every scan). DONE
// pulses on completion; STATUS/ERRID carry the service result.
//
//   tplcc build --library --name Profinet \
//       libraries/profinet/profinet.st -o profinet.tplc

// ========================= FB_ProfinetMaster =========================
// One-shot PROFINET port operation. OP selects the operation:
//   1 = open port (UDP socket + peer)
//   2 = close port
//   3 = service state (STATUS = 1 when ports are configured)
//   4 = DCP Identify (PORT/XID -> NAME/VENDOR/DEVID/ROLE/DEVINST/OEM)
//   5 = DCP Get  (PORT/XID/OPTION/SUB -> OD/DLEN)
//   6 = DCP Set  (PORT/XID/OPTION/SUB/OD[0..DLEN))
//   7 = cyclic IO record exchange (IOOUT[0..DLEN) -> IOIN/DLEN)
// DCP option/suboption constants: OPT_IP=1 (SUB_MAC=1, SUB_IPPARM=2,
// SUB_FULLIP=3), OPT_PROP=2 (SUB_VENDOR=1, SUB_NAME=2, SUB_DEVID=3,
// SUB_ROLE=4, SUB_OPTIONS=5, SUB_ALIAS=6, SUB_INSTANCE=7,
// SUB_OEM=8), OPT_DHCP=3, OPT_CTRL=5.
// STATUS mirrors the tplc-profinet return code (0 ok, -5 malformed
// reply, -7 device block error); ERRID is its magnitude.

FUNCTION_BLOCK FB_ProfinetMaster
VAR_INPUT
    EXEC : BOOL;
    OP : UDINT := 1;
    PORT : UDINT := 0;
    XID : UDINT := 16#12345;
    OPTION : UDINT := 0;
    SUB : UDINT := 0;
    DLEN : UDINT := 0;            // write: payload len; read: cap
    OD : ARRAY[0..63] OF BYTE;
    IOOUT : ARRAY[0..63] OF BYTE;
END_VAR
VAR_OUTPUT
    DONE : BOOL;
    STATUS : DINT;
    ERRID : UDINT;
    DLEN_OUT : UDINT;
    NAME : STRING[31];
    VENDOR : UDINT;
    DEVID : UDINT;
    ROLE : UDINT;
    DEVINST : UDINT;
    OEM : UDINT;
    ODDATA : ARRAY[0..63] OF BYTE;
    IOIN : ARRAY[0..63] OF BYTE;
END_VAR
VAR
    raw : FB_Profinet;
    prevExec : BOOL;
    i : UDINT;
END_VAR
    DONE := FALSE;
    STATUS := 0;
    ERRID := 0;
    DLEN_OUT := 0;
    IF EXEC AND NOT prevExec THEN
        raw.OP := OP;
        raw.PORT := PORT;
        raw.XID := XID;
        raw.OPTION := OPTION;
        raw.SUB := SUB;
        raw.DLEN := DLEN;
        FOR i := 0 TO 63 DO
            raw.OD[i] := OD[i];
            raw.IOOUT[i] := IOOUT[i];
        END_FOR;
        raw();
        STATUS := raw.STATUS;
        ERRID := raw.ERRID;
        DLEN_OUT := raw.DLEN;
        NAME := raw.NAME;
        VENDOR := raw.VENDOR;
        DEVID := raw.DEVID;
        ROLE := raw.ROLE;
        DEVINST := raw.DEVINST;
        OEM := raw.OEM;
        FOR i := 0 TO 63 DO
            ODDATA[i] := raw.OD[i];
            IOIN[i] := raw.IOIN[i];
        END_FOR;
        DONE := TRUE;
    END_IF;
    prevExec := EXEC;
END_FUNCTION_BLOCK
