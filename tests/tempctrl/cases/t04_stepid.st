// t04_stepid.st — FB_AtcStepIdent on a simulated FOPDT plant:
// K=2 deg/%, Tau=20 s, true delay 7.5 s (15-scan shift at Ts=0.5),
// step U0=10 -> U1=40. Expect K~2, Tau~20, Theta~7.5+lag bias.
// flags: b0 Done, b1 |K-2|<0.15, b2 14<Tau<28, b3 4<Theta<14,
//        b4 Busy low, b5 no Err, b6 PvFinal ~ K*U1 = 80.
PROGRAM T04
VAR
    flags AT %QB0 : BYTE;
    id : FB_AtcStepIdent;
    pv : LREAL;
    dl : ARRAY[0..15] OF LREAL;
    i : DINT;
    u AT %QD8 : LREAL;
    kO AT %QD16 : LREAL;
    tauO AT %QD24 : LREAL;
    thO AT %QD32 : LREAL;
END_VAR
    id(Execute := TRUE, Actual := pv, U0 := 10.0, U1 := 40.0,
       SettleT := 120.0, TolBand := 0.05, SettleN := 8,
       MaxTime := 600.0, StoreEvery := 4, CycleTime := 0.5,
       Reset := FALSE);
    u := id.Out;
    // plant: 7.5 s transport delay + 20 s lag, gain 2
    pv := pv + (2.0 * dl[15] - pv) * (0.5 / (20.0 + 0.5));
    FOR i := 0 TO 14 DO
        dl[15 - i] := dl[14 - i];
    END_FOR;
    dl[0] := u;
    kO := id.K;
    tauO := id.Tau;
    thO := id.Theta;
    flags := 0;
    IF id.Done THEN flags := flags + 1; END_IF;
    IF id.K > 1.85 AND id.K < 2.15 THEN flags := flags + 2; END_IF;
    IF id.Tau > 14.0 AND id.Tau < 28.0 THEN flags := flags + 4; END_IF;
    IF id.Theta > 4.0 AND id.Theta < 14.0 THEN flags := flags + 8; END_IF;
    IF NOT id.Busy THEN flags := flags + 16; END_IF;
    IF NOT id.Err THEN flags := flags + 32; END_IF;
    IF id.PvFinal > 75.0 AND id.PvFinal < 85.0 THEN
        flags := flags + 64;
    END_IF;
END_PROGRAM
