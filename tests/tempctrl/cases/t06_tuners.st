// t06_tuners.st — solvers vs reference values.
// Model from FB_AtcModel2s for K=1 Tau=20 TauH=5 Theta=0 Ts=0.5:
//   a1=e^(-0.025)=0.9753099 a2=e^(-0.1)=0.9048374
//   A=[[.9753,.0247],[0,.9048]] B=[0,0.09516]
// FB_AtcLqrTune Q=diag(1,0.1,0.5) R=1 ->
//   ref (scipy DARE, -Ts aug): K1=6.1884 K2=1.0892 Ki=0.6712
// FB_AtcKalmanTune Q=diag(.01,.01) R=1 -> L1=0.0777 L2=0.00609
// flags: b0/b4 done+conv, b1..3 K in range, b5..6 L in range,
//        b7 model outputs in range
PROGRAM T06
VAR
    flags AT %QB0 : BYTE;
    m : FB_AtcModel2s;
    t : FB_AtcLqrTune;
    kf : FB_AtcKalmanTune;
    k1 AT %QD8 : LREAL;
    k2 AT %QD16 : LREAL;
    ki AT %QD24 : LREAL;
    l1 AT %QD32 : LREAL;
END_VAR
    m(Execute := TRUE, K := 1.0, Tau := 20.0, Theta := 0.0,
      TauH := 5.0, Ts := 0.5);
    t(Execute := TRUE, A11 := m.A11, A12 := m.A12,
      A21 := m.A21, A22 := m.A22, B1 := m.B1, B2 := m.B2,
      Q1 := 1.0, Q2 := 0.1, Qi := 0.5, R := 1.0, Ts := 0.5,
      MaxIter := 0, Tol := 0.0);
    kf(Execute := TRUE, A11 := m.A11, A12 := m.A12,
       A21 := m.A21, A22 := m.A22,
       Q1 := 0.01, Q2 := 0.01, R := 1.0,
       MaxIter := 0, Tol := 0.0);
    k1 := t.K1;
    k2 := t.K2;
    ki := t.Ki;
    l1 := kf.L1;
    flags := 0;
    IF m.Done THEN flags := flags + 1; END_IF;
    IF t.K1 > 6.0 AND t.K1 < 6.4 THEN flags := flags + 2; END_IF;
    IF t.K2 > 1.0 AND t.K2 < 1.2 THEN flags := flags + 4; END_IF;
    IF t.Ki > 0.6 AND t.Ki < 0.75 THEN flags := flags + 8; END_IF;
    IF t.Done AND t.Conv THEN flags := flags + 16; END_IF;
    IF kf.Done AND kf.Conv THEN flags := flags + 32; END_IF;
    IF kf.L1 > 0.07 AND kf.L1 < 0.09 THEN flags := flags + 64; END_IF;
    IF kf.L2 > 0.004 AND kf.L2 < 0.008 THEN flags := flags + 128; END_IF;
END_PROGRAM
