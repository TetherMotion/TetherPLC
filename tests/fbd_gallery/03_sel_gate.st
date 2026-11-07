PROGRAM P03_SEL_GATE
VAR
    g AT %IX0.0 : BOOL;
    a AT %IB1 : BYTE;
    b AT %IB2 : BYTE;
    out AT %QB0 : BYTE;
END_VAR
IF g THEN
    out := b;
ELSE
    out := a;
END_IF;
END_PROGRAM
