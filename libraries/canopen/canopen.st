// canopen.st — CANopen/CoE object-dictionary access for TetherPLC.
// Wraps the tplc-canopen host service (Tether IO machine.sdo.* —
// CiA 301 SDO upload/download by explicit index; EtherCAT CoE carries
// the CANopen object dictionary, the same protocol CANopen uses on
// CAN). Requires TPLC_CANOPEN_IO (or TPLC_EC_IO / TPLC_MOTION_IO)
// pointing at a tether_io server exposing machine.sdo.* — e.g.
// tether_io_ec_server --session-role technician.
//
// FB_CoSdo is the FBD/IDE-facing block: EXEC rising edge runs one
// synchronous SDO op (a Tether IO round-trip — do not call every
// scan). DONE pulses on completion; STATUS/ERRID/ABORT carry the
// service result and the slave's CoE abort code.
//
//   tplcc build --library --name CanOpen \
//       libraries/canopen/canopen.st -o canopen.tplc

// ============================ FB_CoSdo ============================
// One-shot CiA 301 SDO client. OP selects the operation:
//   1 = upload  (SLAVE/INDEX/SUB -> RDATA/RLEN, cap = DLEN or 256)
//   2 = download(SLAVE/INDEX/SUB/WDATA[0..DLEN) -> slave, VERIFY
//                requests the server-side post-write readback check)
//   3 = service state (STATUS = 0 when the IO endpoint is up)
// STATUS mirrors the tplc-canopen return code (0 ok, -4 = CoE abort
// with ABORT set, -3 = no endpoint, ...); ERRID is its magnitude.

FUNCTION_BLOCK FB_CoSdo
VAR_INPUT
    EXEC : BOOL;
    OP : UDINT := 1;
    SLAVE : UDINT := 0;
    INDEX : UDINT := 0;
    SUB : UDINT := 0;
    VERIFY : UDINT := 0;
    DLEN : UDINT := 0;            // write: payload len; read: cap (0=256)
    WDATA : ARRAY[0..255] OF BYTE;
END_VAR
VAR_OUTPUT
    DONE : BOOL;
    STATUS : DINT;
    ERRID : UDINT;
    ABORT : UDINT;
    RLEN : UDINT;
    RDATA : ARRAY[0..255] OF BYTE;
END_VAR
VAR
    raw : FB_CanOpen;
    prevExec : BOOL;
    i : UDINT;
END_VAR
    DONE := FALSE;
    STATUS := 0;
    ERRID := 0;
    ABORT := 0;
    RLEN := 0;
    IF EXEC AND NOT prevExec THEN
        raw.OP := OP;
        raw.SLAVE := SLAVE;
        raw.INDEX := INDEX;
        raw.SUB := SUB;
        raw.VERIFY := VERIFY;
        raw.DLEN := DLEN;
        IF OP = 2 THEN
            FOR i := 0 TO 255 DO
                raw.DATA[i] := WDATA[i];
            END_FOR;
        END_IF;
        raw();
        STATUS := raw.STATUS;
        ERRID := raw.ERRID;
        ABORT := raw.ABORT;
        IF OP = 1 THEN
            RLEN := raw.DLEN;
            FOR i := 0 TO 255 DO
                RDATA[i] := raw.DATA[i];
            END_FOR;
        END_IF;
        DONE := TRUE;
    END_IF;
    prevExec := EXEC;
END_FUNCTION_BLOCK
