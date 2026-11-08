PROGRAM Demo
VAR
    w  : WSTRING[16];
    w2 : WSTRING[16];
    eq1 AT %QX0.0 : BOOL;
    eq2 AT %QX0.1 : BOOL;
    eq3 AT %QX0.2 : BOOL;
END_VAR
w := "abc";
w2 := "abd";
eq1 := (w = w2);
eq2 := (w = "abc");
eq3 := (w <> w2);
END_PROGRAM
