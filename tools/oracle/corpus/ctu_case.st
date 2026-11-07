PROGRAM CtuCase
VAR
    pulse AT %IX0.0 : BOOL;
    reset AT %IX0.1 : BOOL;
    done AT %QX0.0 : BOOL;
    ctu1 : CTU;
    last_cv : INT;
END_VAR
ctu1(CU := pulse, R := reset, PV := 3);
done := ctu1.QU;
last_cv := ctu1.CV;
END_PROGRAM
