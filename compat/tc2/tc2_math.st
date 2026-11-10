// Tc2_Math compatibility functions (TwinCAT 3 PLC Library Tc2_Math).
// Pure LREAL math; sources from docs/plc/plan/21-twincat-libraries.md §7.
// Limitation: TRUNC narrows through DINT, so |x| >= 2^31 loses precision
// (same bound as Beckhoff's 32-bit TRUNC-based implementations).

FUNCTION F_TruncL : LREAL
VAR_INPUT
    lr_in : LREAL;
END_VAR
    F_TruncL := DINT_TO_LREAL(TRUNC(lr_in));
END_FUNCTION

FUNCTION CEIL : LREAL
VAR_INPUT
    lr_in : LREAL;
END_VAR
VAR
    t : LREAL;
END_VAR
    t := F_TruncL(lr_in);
    IF lr_in > t THEN
        CEIL := t + 1.0;
    ELSE
        CEIL := t;
    END_IF;
END_FUNCTION

FUNCTION FLOOR : LREAL
VAR_INPUT
    lr_in : LREAL;
END_VAR
VAR
    t : LREAL;
END_VAR
    t := F_TruncL(lr_in);
    IF lr_in < t THEN
        FLOOR := t - 1.0;
    ELSE
        FLOOR := t;
    END_IF;
END_FUNCTION

FUNCTION FRAC : LREAL
VAR_INPUT
    lr_in : LREAL;
END_VAR
    FRAC := lr_in - F_TruncL(lr_in);
END_FUNCTION

FUNCTION LTRUNC : LREAL
VAR_INPUT
    lr_in : LREAL;
END_VAR
    LTRUNC := F_TruncL(lr_in);
END_FUNCTION

FUNCTION LMOD : LREAL
VAR_INPUT
    lr_Value : LREAL;
    lr_Arg   : LREAL;
END_VAR
    IF lr_Arg = 0.0 THEN
        LMOD := 0.0;
    ELSE
        LMOD := lr_Value - F_TruncL(lr_Value / lr_Arg) * lr_Arg;
    END_IF;
END_FUNCTION

FUNCTION MODABS : LREAL
VAR_INPUT
    lr_val : LREAL;
    lr_mod : LREAL;
END_VAR
VAR
    am : LREAL;
END_VAR
    am := ABS(lr_mod);
    IF am = 0.0 THEN
        MODABS := 0.0;
    ELSE
        MODABS := lr_val - FLOOR(lr_val / am) * am;
    END_IF;
END_FUNCTION

FUNCTION MODTURNS : DINT
VAR_INPUT
    lr_Value : LREAL;
    lr_Arg   : LREAL;
END_VAR
    IF lr_Arg = 0.0 THEN
        MODTURNS := 0;
    ELSE
        MODTURNS := TRUNC(lr_Value / lr_Arg);
    END_IF;
END_FUNCTION

FUNCTION F_GetVersionTcMath : UINT
VAR_INPUT
    nVersionElement : INT;
END_VAR
    F_GetVersionTcMath := 16#0100;
END_FUNCTION
