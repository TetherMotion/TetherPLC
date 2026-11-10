// Tc2_Standard L* timer FBs (TwinCAT 3 PLC Library Tc2_Standard).
// LTON/LTOF/LTP are the LTIME variants of TON/TOF/TP; LTIME and TIME
// share the same 64-bit representation here, so each wrapper forwards
// to the corresponding builtin timer instance.
// Sources: docs/plc/plan/21-twincat-libraries.md §7.

FUNCTION_BLOCK LTON
VAR_INPUT
    IN : BOOL;
    PT : LTIME;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    ET : LTIME;
END_VAR
VAR
    t : TON;
END_VAR
    t(IN := IN, PT := PT);
    Q := t.Q;
    ET := t.ET;
END_FUNCTION_BLOCK

FUNCTION_BLOCK LTOF
VAR_INPUT
    IN : BOOL;
    PT : LTIME;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    ET : LTIME;
END_VAR
VAR
    t : TOF;
END_VAR
    t(IN := IN, PT := PT);
    Q := t.Q;
    ET := t.ET;
END_FUNCTION_BLOCK

FUNCTION_BLOCK LTP
VAR_INPUT
    IN : BOOL;
    PT : LTIME;
END_VAR
VAR_OUTPUT
    Q : BOOL;
    ET : LTIME;
END_VAR
VAR
    t : TP;
END_VAR
    t(IN := IN, PT := PT);
    Q := t.Q;
    ET := t.ET;
END_FUNCTION_BLOCK
