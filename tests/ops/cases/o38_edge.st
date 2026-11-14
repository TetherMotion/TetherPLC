// o38_edge.st — FB_AtcEdge: I high n=6..15, pulse n=26, disabled
// pulse n=32 (Enable off n=30..35), I high n=38..42. Edges: rise n=6,
// fall n=16, rise+fall n=26/27, suppressed n=32, rise n=38.
// flags %QB0: expect 0x1f.
PROGRAM O38
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    ed : FB_AtcEdge;
    n : DINT;
    i : BOOL;
    sawQ, sawQN, sawQ2 : BOOL;
END_VAR
    n := n + 1;
    i := (n >= 6 AND n <= 15) OR (n = 26) OR (n = 32)
        OR (n >= 38 AND n <= 42);
    ed(I := i, Enable := NOT (n >= 30 AND n <= 35));
    IF ed.Q THEN sawQ := TRUE; END_IF;
    IF ed.QN THEN sawQN := TRUE; END_IF;
    IF n >= 38 AND ed.Q THEN sawQ2 := TRUE; END_IF;
    IF n >= 8 AND n <= 14 AND NOT ed.Q AND NOT ed.QN THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 20 AND n <= 25 AND NOT ed.Q AND NOT ed.QN THEN
        ok1 := TRUE;
    END_IF;
    IF sawQ THEN ok2 := TRUE; END_IF;
    IF sawQN THEN ok3 := TRUE; END_IF;
    IF sawQ2 THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
