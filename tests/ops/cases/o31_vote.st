// o31_vote.st — FB_AtcVote: Band=2.0. Clean triple n=1..10 ->
// median 51. V3=80 n=11..20 -> spread 30 -> Bad, median still 51.
// UseDefault+DefV=45 n=21..25 -> Vout=45 while Bad. Clean again
// n=26..30 -> Bad clears (level, not latched). flags %QB0: 0x1f.
PROGRAM O31
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    vt : FB_AtcVote;
    n : DINT;
    v3 : LREAL;
    ud : BOOL;
END_VAR
    n := n + 1;
    v3 := 52.0;
    IF n >= 11 AND n <= 25 THEN v3 := 80.0; END_IF;
    ud := (n >= 21 AND n <= 25);
    vt(V1 := 50.0, V2 := 51.0, V3 := v3, Band := 2.0,
       UseDefault := ud, DefV := 45.0);
    IF n >= 5 AND n <= 10 AND vt.Vout = 51.0 AND NOT vt.Bad THEN
        ok0 := TRUE;
    END_IF;
    IF n >= 13 AND n <= 19 AND vt.Vout = 51.0 AND vt.Bad
        AND vt.Spread > 25.0 THEN
        ok1 := TRUE;
    END_IF;
    IF n >= 22 AND n <= 25 AND vt.Bad AND vt.Vout = 45.0 THEN
        ok2 := TRUE;
    END_IF;
    IF n >= 27 AND n <= 30 AND vt.Vout = 51.0 AND NOT vt.Bad THEN
        ok3 := TRUE;
    END_IF;
    IF n >= 13 AND n <= 19 AND vt.Spread >= 29.0 AND vt.Spread <= 31.0
    THEN
        ok4 := TRUE;
    END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
