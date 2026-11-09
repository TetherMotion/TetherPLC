PROGRAM Demo
VAR
    x   : DINT := 42;
    r   : REFERENCE TO DINT;
    out AT %QD0 : DINT;
    ok  AT %QD4 : DINT;
END_VAR
r REF= x;
out := r;
ok := 0;
IF ISVALIDREF(r) THEN
    ok := ok + 1;
END_IF
IF r^ = 42 THEN
    ok := ok + 2;
END_IF
IF r = 42 THEN
    ok := ok + 4;
END_IF
END_PROGRAM
