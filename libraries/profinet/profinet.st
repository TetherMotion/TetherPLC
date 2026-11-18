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
//   8 = IOCR cyclic exchange (XID=frame_id, 0=keep default 0x8000;
//       IOOUT[0..DLEN) -> IOIN/DLEN, PNCYC=device cycle counter,
//       PNSTAT=DataStatus byte e.g. 16#35)
//   9 = PNIO-CM AR Connect (DEVOBJ/ACTUUID/ARUUID u8[16] +
//       ARBLK[0..DLEN) connect blocks -> ARBLK res blocks/DLEN,
//       ARSTATE/ARKEY/ARPSTAT filled)
//   10 = PNIO-CM Write   (ARBLK blocks -> ARBLK res, ARPSTAT)
//   11 = PNIO-CM Control (ARBLK blocks -> ARBLK res, ARPSTAT;
//       PrmEnd/AppReady/Release IODControl blocks)
//   12 = PNIO-CM service (XID=timeout_ms; ARSTATE=3 data exchange
//       once the device ApplicationReady is consumed)
//   13 = AR state query (ARSTATE/ARKEY/ARUUID out)
//   14 = PNIO-CM record Read (ARBLK IODReadReq -> ARBLK
//       IODReadRes+record, ARPSTAT)
// AR block payloads are encoded with the PnioCm codec
// (tplc/host/PnioCm.hpp); ARSTATE: 0 idle 1 connected 2 prmEndSent
// 3 dataExchange 4 released. The CM session rides UDP/34964 to the
// port endpoint's host.
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
    DEVOBJ : ARRAY[0..15] OF BYTE;   // OP9: device object UUID
    ACTUUID : ARRAY[0..15] OF BYTE;  // OP9: activity UUID
    ARUUID : ARRAY[0..15] OF BYTE;   // OP9 in / OP13 out
    ARBLK : ARRAY[0..511] OF BYTE;   // OP9-11,14: blocks in/out
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
    PNCYC : UDINT;
    PNSTAT : UDINT;
    ARSTATE : UDINT;
    ARKEY : UDINT;
    ARPSTAT : UDINT;
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
        FOR i := 0 TO 15 DO
            raw.DEVOBJ[i] := DEVOBJ[i];
            raw.ACTUUID[i] := ACTUUID[i];
            raw.ARUUID[i] := ARUUID[i];
        END_FOR;
        FOR i := 0 TO 511 DO
            raw.ARBLK[i] := ARBLK[i];
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
        PNCYC := raw.PNCYC;
        PNSTAT := raw.PNSTAT;
        FOR i := 0 TO 15 DO
            DEVOBJ[i] := raw.DEVOBJ[i];
            ACTUUID[i] := raw.ACTUUID[i];
            ARUUID[i] := raw.ARUUID[i];
        END_FOR;
        FOR i := 0 TO 511 DO
            ARBLK[i] := raw.ARBLK[i];
        END_FOR;
        ARSTATE := raw.ARSTATE;
        ARKEY := raw.ARKEY;
        ARPSTAT := raw.ARPSTAT;
        DONE := TRUE;
    END_IF;
    prevExec := EXEC;
END_FUNCTION_BLOCK
