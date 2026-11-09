PROGRAM Demo
VAR
    s  : STRING[16];
    t  : STRING[16];
    ok AT %QD0 : DINT;
    n1 AT %QD4 : DINT;
END_VAR
s := 'abcdef';
t := 'XY';
ok := 0;
IF LEFT(s, 3) = 'abc' THEN
    ok := ok + 1;
END_IF
IF RIGHT(s, 3) = 'def' THEN
    ok := ok + 2;
END_IF
IF MID(s, 2, 3) = 'cd' THEN
    ok := ok + 4;
END_IF
IF INSERT(s, t, 4) = 'abcXYdef' THEN
    ok := ok + 8;
END_IF
IF DELETE(s, 2, 3) = 'abef' THEN
    ok := ok + 16;
END_IF
IF REPLACE(s, t, 2, 3) = 'abXYef' THEN
    ok := ok + 32;
END_IF
n1 := FIND(s, 'cd');
END_PROGRAM
