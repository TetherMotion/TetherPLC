// tc2_spa_case.st — SPA e2e: peak detect, envelope, dft-bin,
// cross-correlation — pure-ST signal analysis.
PROGRAM tc2_spa_case
VAR
    pk : FB_PeakDetect;
    env : FB_Envelope;
    dft : FB_DftBin;
    xc : FB_XCorrQuality;
    st : INT;
    n : INT;
    mask AT %QD0 : UDINT;
    sig : LREAL;
END_VAR
    IF st = 0 THEN
        mask := 0;
        pk.Execute := TRUE;
        env.Execute := TRUE;
        dft.Execute := TRUE;
        xc.Execute := TRUE;
        pk.N := 8;
        env.AttackTn := 0.05; env.ReleaseTn := 0.5; env.CycleTime := 0.1;
        dft.N := 8; dft.Bin := 1; dft.CycleTime := 0.1;
        xc.N := 8;
        sig := 0.0;
        st := 1;
    END_IF;

    // drive a stepped signal
    IF st = 1 THEN
        n := n + 1;
        IF n < 4 THEN
            sig := 1.0;
        ELSIF n < 8 THEN
            sig := 4.0;   // peak
        ELSE
            sig := 2.0;
        END_IF;
        pk.Input := sig;
        env.Input := sig;
        dft.Input := sig;
        xc.Input := sig;
        xc.Reference := 2.0;
        IF n > 12 THEN
            IF pk.Peak >= 4.0 THEN mask := mask + 1; END_IF;   // saw peak
            IF pk.Valley <= 2.0 THEN mask := mask + 2; END_IF; // saw valley
            IF env.Output > 0.5 THEN mask := mask + 4; END_IF;  // envelope
            IF dft.Magnitude > 0.5 THEN mask := mask + 8; END_IF;
            st := 2;
        END_IF;
    END_IF;
    // cross-correlation of identical signals ~ +1
    IF st = 2 THEN
        n := n + 1;
        xc.Input := 2.0;
        xc.Reference := 2.0;
        IF n > 20 THEN
            IF xc.Quality > 0.9 THEN mask := mask + 16; END_IF; // ~+1
            st := 3;
        END_IF;
    END_IF;

    pk(); env(); dft(); xc();
END_PROGRAM
