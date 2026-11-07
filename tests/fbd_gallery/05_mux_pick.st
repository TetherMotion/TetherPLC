PROGRAM P05_MUX_PICK
VAR
    k AT %IB1 : BYTE;
    out AT %QB0 : BYTE;
END_VAR
CASE k OF
    0: out := 10;
    1: out := 20;
    2: out := 30;
ELSE
    out := 10;
END_CASE;
END_PROGRAM
