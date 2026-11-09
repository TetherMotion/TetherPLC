PROGRAM OvfCase
VAR
    a AT %ID0 : DINT;
    b AT %ID4 : DINT;
    s AT %IW8 : INT;
    n AT %IW10 : INT;
    out AT %QD0 : DINT;
    out2 AT %QW4 : INT;
    out3 AT %QW6 : INT;
    out4 AT %QD8 : DINT;
END_VAR
out := a + b;
out2 := s * 1000;
out3 := -n;
out4 := a - b;
END_PROGRAM
