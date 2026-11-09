PROGRAM RngCase
VAR
    d AT %ID0 : DINT;
    r AT %IL8 : LREAL;
    u AT %ID16 : UDINT;
    out AT %QD0 : DINT;
    out2 AT %QD4 : DINT;
    out3 AT %QD8 : DINT;
    out4 AT %QD12 : DINT;
END_VAR
out := DINT_TO_INT(d);
out2 := LREAL_TO_DINT(r);
out3 := DINT_TO_UINT(d);
out4 := UDINT_TO_DINT(u);
END_PROGRAM
