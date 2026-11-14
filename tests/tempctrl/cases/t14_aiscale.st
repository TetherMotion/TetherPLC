// t14_aiscale.st — FB_AtcAiScale / FB_AtcAoScale: raw int16 channel
// counts <-> engineering units, incl. PT1 filtering and int16
// saturation. Sticky check bits compose into flags %QB0 at the end
// of every scan (idempotent): b0 ai linear exact, b1 ai filtered
// converging, b2 ao roundtrip, b3 ao saturates high, b4 ao
// saturates low -> expect 0x1f.
PROGRAM T14
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    ai : FB_AtcAiScale;
    aoRT : FB_AtcAoScale;
    aoHi : FB_AtcAoScale;
    aoLo : FB_AtcAoScale;
    aiF : FB_AtcAiScale;
    raw : INT;               // analog input channel counts (EL3104-style)
    rawF : INT;              // second channel for the filter test
    counts : INT;            // analog output channel counts (EL4104-style)
    ct : LREAL;
    n : DINT;
END_VAR
    ct := 0.1;
    n := n + 1;

    // ---- AI: EU = Raw*Scale + Offset, no filter
    IF n = 1 THEN raw := 16384; END_IF;
    ai(Execute := TRUE, Raw := raw, Scale := 0.5, Offset := 10.0,
       FiltT := 0.0, CycleTime := ct, Reset := FALSE);
    // 16384 * 0.5 + 10 = 8202
    IF ai.Done AND ai.Out > 8201.9 AND ai.Out < 8202.1 THEN
        ok0 := TRUE;
    END_IF;

    // ---- AI filtered: FiltT=1 s, ct=0.1 -> alpha = 0.1/1.1
    // step rawF 0 -> 1100 counts at n=20; rawf approaches 1100
    IF n = 20 THEN rawF := 1100; END_IF;
    aiF(Execute := TRUE, Raw := rawF, Scale := 1.0, Offset := 0.0,
        FiltT := 1.0, CycleTime := ct, Reset := FALSE);
    // after ~40 filtered steps rawf ~= 1100*(1-(1-alpha)^40) ~ 1080
    IF n >= 40 AND aiF.RawF > 1000.0 AND aiF.RawF < 1100.0 THEN
        ok1 := TRUE;
    END_IF;

    // ---- AO: counts = (In - Offset)/Scale -> int16
    aoRT(Execute := TRUE, In := 8202.0, Scale := 0.5, Offset := 10.0,
         CycleTime := ct, Reset := FALSE);
    IF aoRT.Done AND aoRT.Counts = 16384 AND NOT aoRT.OutBad THEN
        ok2 := TRUE;
    END_IF;

    // ---- AO saturation: way over range -> rail + OutBad
    aoHi(Execute := TRUE, In := 40000.0, Scale := 0.5, Offset := 10.0,
         CycleTime := ct, Reset := FALSE);
    IF aoHi.Done AND aoHi.Counts >= 32767 AND aoHi.OutBad THEN
        ok3 := TRUE;
    END_IF;
    aoLo(Execute := TRUE, In := -40000.0, Scale := 0.5, Offset := 10.0,
         CycleTime := ct, Reset := FALSE);
    IF aoLo.Done AND aoLo.Counts <= -32768 AND aoLo.OutBad THEN
        ok4 := TRUE;
    END_IF;

    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;

    // settle: feed AO counts back to image for observability
    counts := aoRT.Counts;
END_PROGRAM
