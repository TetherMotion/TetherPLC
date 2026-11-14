// iolink.st — IO-Link master data-link access for TetherPLC.
// Wraps the tplc-iolink host service: the IEC 61131-9 / SDCI wire
// protocol (wakeup, MC/CKT/CKS frames, A.1.6 checksum, page-channel
// identification, type-2_V cyclic process data, ISDU on-request
// data) over per-port byte transports configured via TPLC_IOLINK_IO
// (comma-separated tcp://host:port endpoints; port index selects
// the endpoint). Real IO-Link devices need 24 V signaling — deploy
// behind a PHY-equipped master adapter; a UART/PHY transport is the
// hardware path.
//
// FB_IoLinkMaster is the FBD/IDE-facing block: EXEC rising edge runs
// one synchronous wire round-trip (do not call every scan). DONE
// pulses on completion; STATUS/ERRID/CKSFLAGS carry the service
// result and the device CKS flag bits (0x80 event pending,
// 0x40 PD-in invalid).
//
//   tplcc build --library --name IoLink \
//       libraries/iolink/iolink.st -o iolink.tplc

// ========================= FB_IoLinkMaster =========================
// One-shot IO-Link port operation. OP selects the operation:
//   1 = open port (connect transport + wakeup burst)
//   2 = close port
//   3 = service state (STATUS = 1 when ports are configured)
//   4 = identify (PORT -> IDENT/DLEN: DPP1 record, 16 octets —
//       [5]=PD-in desc, [6]=PD-out desc, [7..8]=vendor LE,
//       [9..10]=device LE, [11]=revision)
//   5 = cyclic process-data exchange (PORT/MSEQ/PDOUT[0..DLEN)
//       -> PDIN/DLEN/CKSFLAGS; MSEQ selects the M-sequence type,
//       6 = type 2_V is the general case)
//   6 = ISDU/on-request read (PORT/ADDR -> OD/DLEN, single octet;
//       legacy profile — prefer OP 9)
//   7 = ISDU/on-request write (PORT/ADDR/OD[0..DLEN); legacy)
//   8 = MasterCommand (PORT/ADDR=command: 16#5A fallback->SIO,
//       16#95 MasterIdent, 16#96 DeviceIdent, 16#97 DeviceStartup,
//       16#98 PDOutOperate, 16#99 DeviceOperate, 16#9A
//       DevicePreoperate)
//   9 = ISDU read, spec-segmented (PORT/MSEQ=index low16,
//       ADDR=subindex -> ISDU/DLEN_OUT, ISDUERR=device ErrorType)
//   10 = ISDU write, spec-segmented (PORT/MSEQ=index, ADDR=subindex,
//       ISDU[0..DLEN) payload -> ISDUERR)
//   11 = diagnosis event-memory read (PORT -> DIAG[0..18], DLEN=octet
//       count). Reads the Table-58 snapshot over the diagnosis
//       channel on the negotiated type-2_V profile and confirms the
//       readout (Table 59 T8). Requires OP 4 ident first (the
//       negotiated PD sizes frame the exchange). DIAG[0] =
//       StatusCode (bit7 set, bits0-5 active-slot mask); each active
//       slot is 3 octets at 1+3i: {EventQualifier, codeHi, codeLo}.
//       EventQualifier: bits7-6 MODE (16#C0 appears, 16#40
//       single-shot, 16#80 disappears), bits5-4 type (16#10
//       notification, 16#20 warning, 16#30 error), bit2 instance.
//   12 = AL negotiation (PORT, MSEQ=PD-out request, ADDR=PD-in
//       request, 0 = device maximum -> ALPDO/ALPDI). Performs the
//       DPP1 identification implicitly when needed, clamps requests
//       to the device maximum, and stores the result: subsequent
//       OP 5/11 exchanges frame themselves from the negotiated PD
//       lengths instead of caller-supplied ones.
// ISDU indices follow Table B.8: 16#0010 VendorName, 16#0012
// ProductName, 16#0015 SerialNumber, 16#0024 DeviceStatus,
// 16#0028/16#0029 last PDin/PDout, vendor range 16#0040+.
// STATUS mirrors the tplc-iolink return code (0 ok, -5 = device
// checksum mismatch, -6 = malformed reply, -7 = device error —
// ISDUERR then carries the ErrorType, e.g. 16#8011 index not
// available); ERRID is its magnitude.

FUNCTION_BLOCK FB_IoLinkMaster
VAR_INPUT
    EXEC : BOOL;
    OP : UDINT := 1;
    PORT : UDINT := 0;
    MSEQ : UDINT := 6;
    ADDR : UDINT := 0;
    DLEN : UDINT := 0;            // write: payload len; read: cap
    PDOUT : ARRAY[0..31] OF BYTE;
    OD : ARRAY[0..7] OF BYTE;
END_VAR
VAR_IN_OUT
    ISDU : ARRAY[0..239] OF BYTE;   // OP10 payload in / OP9 response out
END_VAR
VAR_OUTPUT
    DONE : BOOL;
    STATUS : DINT;
    ERRID : UDINT;
    CKSFLAGS : UDINT;
    DLEN_OUT : UDINT;
    ISDUERR : UDINT;                // device ErrorType on STATUS=-7
    IDENT : ARRAY[0..15] OF BYTE;
    PDIN : ARRAY[0..39] OF BYTE;
    ODDATA : ARRAY[0..7] OF BYTE;
    DIAG : ARRAY[0..18] OF BYTE;    // OP11 event memory snapshot
    ALPDO : UDINT;                  // OP12 negotiated PD-out octets
    ALPDI : UDINT;                  // OP12 negotiated PD-in octets
END_VAR
VAR
    raw : FB_IoLink;
    prevExec : BOOL;
    i : UDINT;
END_VAR
    DONE := FALSE;
    STATUS := 0;
    ERRID := 0;
    CKSFLAGS := 0;
    DLEN_OUT := 0;
    ISDUERR := 0;
    IF EXEC AND NOT prevExec THEN
        raw.OP := OP;
        raw.PORT := PORT;
        raw.MSEQ := MSEQ;
        raw.ADDR := ADDR;
        raw.DLEN := DLEN;
        FOR i := 0 TO 31 DO
            raw.PDOUT[i] := PDOUT[i];
        END_FOR;
        FOR i := 0 TO 7 DO
            raw.OD[i] := OD[i];
        END_FOR;
        FOR i := 0 TO 239 DO
            raw.ISDU[i] := ISDU[i];
        END_FOR;
        raw();
        STATUS := raw.STATUS;
        ERRID := raw.ERRID;
        CKSFLAGS := raw.CKSFLAGS;
        DLEN_OUT := raw.DLEN;
        ISDUERR := raw.ISDUERR;
        FOR i := 0 TO 15 DO
            IDENT[i] := raw.IDENT[i];
        END_FOR;
        FOR i := 0 TO 39 DO
            PDIN[i] := raw.PDIN[i];
        END_FOR;
        FOR i := 0 TO 7 DO
            ODDATA[i] := raw.OD[i];
        END_FOR;
        FOR i := 0 TO 239 DO
            ISDU[i] := raw.ISDU[i];
        END_FOR;
        FOR i := 0 TO 18 DO
            DIAG[i] := raw.DIAG[i];
        END_FOR;
        ALPDO := raw.ALPDO;
        ALPDI := raw.ALPDI;
        DONE := TRUE;
    END_IF;
    prevExec := EXEC;
END_FUNCTION_BLOCK
