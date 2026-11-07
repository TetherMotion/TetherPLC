PROGRAM P04_LIMIT_CLAMP
VAR
    x AT %IB1 : BYTE;
    out AT %QB0 : BYTE;
END_VAR
IF x < 10 THEN
    out := 10;
ELSIF x > 200 THEN
    out := 200;
ELSE
    out := x;
END_IF;
END_PROGRAM
