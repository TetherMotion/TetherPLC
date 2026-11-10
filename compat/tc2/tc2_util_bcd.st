// Tc2_Utilities BCD conversion FBs (TwinCAT 3 PLC Library Tc2_Utilities).
// Beckhoff semantics: START rising edge begins conversion, BUSY marks the
// active scan, ERR/ERRID report invalid BCD nibbles (>9). Conversion is
// single-scan here (no async engine), so BUSY pulses for one cycle after
// START and ERR latches until the next START.
// Outputs are driven unconditionally every call: conditional output writes
// can lose state on the LLVM path (see docs/plc/compat/README.md).
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
    r_busy  : BOOL;
    r_err   : BOOL;
    r_errid : UDINT;
    r_bout  : BYTE;
END_VAR
    r_busy := FALSE;
    IF pending THEN
        pending := FALSE;
        r_busy := TRUE;
        d := DIN;
        hi := d / 10;
        lo := d - hi * 10;
        r_bout := 0;
        r_err := FALSE;
        r_errid := 0;
        IF hi > 9 OR lo > 9 THEN
            r_err := TRUE;
            r_errid := 16#8002;
        ELSE
            r_bout := (hi * 16 + lo);
        END_IF;
    END_IF;
    IF START AND NOT prev THEN
        pending := TRUE;
    END_IF;
    prev := START;
    BUSY := r_busy;
    ERR := r_err;
    ERRID := r_errid;
    BOUT := r_bout;
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
    r_busy  : BOOL;
    r_err   : BOOL;
    r_errid : UDINT;
    r_dout  : BYTE;
END_VAR
    r_busy := FALSE;
    IF pending THEN
        pending := FALSE;
        r_busy := TRUE;
        b := BIN;
        hi := b / 16;
        lo := b - hi * 16;
        r_dout := 0;
        r_err := FALSE;
        r_errid := 0;
        IF hi > 9 OR lo > 9 THEN
            r_err := TRUE;
            r_errid := 16#8002;
        ELSE
            r_dout := (hi * 10 + lo);
        END_IF;
    END_IF;
    IF START AND NOT prev THEN
        pending := TRUE;
    END_IF;
    prev := START;
    BUSY := r_busy;
    ERR := r_err;
    ERRID := r_errid;
    DOUT := r_dout;
END_FUNCTION_BLOCK
