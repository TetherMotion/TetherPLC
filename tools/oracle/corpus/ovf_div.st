PROGRAM OvfDiv
VAR
    a AT %IL0 : LINT;
    b AT %IL8 : LINT;
    out AT %QL0 : LINT;
    out2 AT %QL8 : LINT;
END_VAR
out := a / b;
out2 := a + b;
END_PROGRAM
