PROGRAM Demo
VAR
    d AT %ID0 : DINT;
    e AT %ID4 : DINT;
    out AT %QD0 : DINT;
    out2 AT %QD4 : DINT;
END_VAR
out := 100 / d;
out2 := 103 MOD e;
END_PROGRAM
