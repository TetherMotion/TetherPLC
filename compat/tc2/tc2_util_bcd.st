// Tc2_Utilities BCD conversion FBs (TwinCAT 3 PLC Library Tc2_Utilities).
// Beckhoff semantics: START rising edge begins conversion, BUSY marks the
// active scan, ERR/ERRID report invalid BCD nibbles (>9). Conversion is
// single-scan here (no async engine), so BUSY pulses for one cycle after
// START and ERR latches until the next START.
// Sources: docs/plc/plan/21-twincat-libraries.md §7.

FUNCTION_BLOCK DEC_TO_BCD
VAR_INPUT
    START : BOOL;
    DIN   : BYTE;
END_VAR
VAR_OUTPUT
    BUSY  : BOOL;
    ERR   : BOOL;
    ERRID : UDINT;
    BOUT  : BYTE;
END_VAR
VAR
    prev    : BOOL;
    pending : BOOL;
    d       : INT;
    hi      : INT;
    lo      : INT;
END_VAR
    BUSY := FALSE;
    IF pending THEN
        pending := FALSE;
        BUSY := TRUE;
        d := DIN;
        hi := d / 10;
        lo := d - hi * 10;
        BOUT := 0;
        ERR := FALSE;
        ERRID := 0;
        IF hi > 9 OR lo > 9 THEN
            ERR := TRUE;
            ERRID := 16#8002;
        ELSE
            BOUT := (hi * 16 + lo);
        END_IF;
    END_IF;
    IF START AND NOT prev THEN
        pending := TRUE;
    END_IF;
    prev := START;
END_FUNCTION_BLOCK

FUNCTION_BLOCK BCD_TO_DEC
VAR_INPUT
    START : BOOL;
    BIN   : BYTE;
END_VAR
VAR_OUTPUT
    BUSY  : BOOL;
    ERR   : BOOL;
    ERRID : UDINT;
    DOUT  : BYTE;
END_VAR
VAR
    prev    : BOOL;
    pending : BOOL;
    b       : INT;
    hi      : INT;
    lo      : INT;
END_VAR
    BUSY := FALSE;
    IF pending THEN
        pending := FALSE;
        BUSY := TRUE;
        b := BIN;
        hi := b / 16;
        lo := b - hi * 16;
        DOUT := 0;
        ERR := FALSE;
        ERRID := 0;
        IF hi > 9 OR lo > 9 THEN
            ERR := TRUE;
            ERRID := 16#8002;
        ELSE
            DOUT := (hi * 10 + lo);
        END_IF;
    END_IF;
    IF START AND NOT prev THEN
        pending := TRUE;
    END_IF;
    prev := START;
END_FUNCTION_BLOCK
