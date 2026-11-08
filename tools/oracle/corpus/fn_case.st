PROGRAM Demo
VAR
    s     : STRING[16];
    t     : STRING[16];
    c     : STRING[32];
    w     : WSTRING[16];
    wc    : WSTRING[32];
    n1 AT %QD0    : DINT;
    flags AT %QD4 : DINT;
END_VAR
s := 'abc';
t := 'xyz';
c := CONCAT(s, t);
w := "aä";
wc := CONCAT(w, w);
n1 := LEN(c);
flags := 0;
IF c = 'abcxyz' THEN
    flags := flags + 1;
END_IF
IF LEN(wc) = 4 THEN
    flags := flags + 2;
END_IF
IF wc = "aäaä" THEN
    flags := flags + 4;
END_IF
END_PROGRAM
