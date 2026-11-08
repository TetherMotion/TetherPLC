PROGRAM CtdCase
VAR
    dn   AT %IX0.0 : BOOL;
    load AT %IX0.1 : BOOL;
    done AT %QX0.0 : BOOL;
    ctd1 : CTD;
END_VAR
ctd1(CD := dn, LD := load, PV := 3);
done := ctd1.QD;
END_PROGRAM
