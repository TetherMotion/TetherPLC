// t01_math.st — math function conformance: exp/ln/sqrt/pow against
// known values; flags bit i set when the check passes.
PROGRAM T01
VAR
    flags AT %QB0 : BYTE;
    e1 AT %QD8 : LREAL;
    l1 AT %QD16 : LREAL;
    s1 AT %QD24 : LREAL;
    p1 AT %QD32 : LREAL;
    l2 AT %QD40 : LREAL;
    s2 AT %QD48 : LREAL;
END_VAR
    e1 := F_AtcExp(1.0);          // e = 2.718281828...
    l1 := F_AtcLn(F_AtcExp(3.7)); // = 3.7
    s1 := F_AtcSqrt(2.0);         // 1.41421356...
    p1 := F_AtcPow(2.0, 10.0);    // 1024
    l2 := F_AtcLn(100.0);         // 4.605170186
    s2 := F_AtcSqrt(0.001);       // 0.031622776
    flags := 0;
    IF e1 > 2.718281 AND e1 < 2.718282 THEN flags := flags + 1; END_IF;
    IF l1 > 3.6999 AND l1 < 3.7001 THEN flags := flags + 2; END_IF;
    IF s1 > 1.414213 AND s1 < 1.414214 THEN flags := flags + 4; END_IF;
    IF p1 > 1023.999 AND p1 < 1024.001 THEN flags := flags + 8; END_IF;
    IF l2 > 4.605169 AND l2 < 4.605171 THEN flags := flags + 16; END_IF;
    IF s2 > 0.031622 AND s2 < 0.031624 THEN flags := flags + 32; END_IF;
END_PROGRAM
