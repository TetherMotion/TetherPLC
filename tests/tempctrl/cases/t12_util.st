// t12_util.st — signal helpers: FB_AtcPt1, FB_AtcRateLim,
// FB_AtcPwm, FB_AtcHeatCool.
// flags: b0 Pt1 near input after 10*timeconst scans,
//        b1 RateLim respected (|dOut| <= Rate*Ts + eps),
//        b2 Pwm duty ~50% over a period (count on-scans),
//        b3 HeatCool splits (U=-40 -> Cool=40-DeadZone),
//        b4 all Done.
PROGRAM T12
VAR
    flags AT %QB0 : BYTE;
    f1 : FB_AtcPt1;
    rl : FB_AtcRateLim;
    pw : FB_AtcPwm;
    hc : FB_AtcHeatCool;
    cnt : DINT;
    onCnt : DINT;
    pv : LREAL;
    rlPrev : LREAL;
    rlOk : BOOL;
    rlIn : LREAL;
END_VAR
    cnt := cnt + 1;
    // Pt1: step 0->50 at scan 1, Tf=4s
    f1(Execute := TRUE, In := 50.0, Tf := 4.0,
       CycleTime := 0.5, Reset := FALSE);
    pv := f1.Out;
    // RateLim: square wave input +-20, Rate=10/s -> |dOut| <= 5/scan
    IF cnt >= 100 THEN rlIn := -20.0; ELSE rlIn := 20.0; END_IF;
    rl(Execute := TRUE, In := rlIn, Rate := 10.0,
       CycleTime := 0.5, Reset := FALSE);
    IF cnt > 1 THEN
        IF rl.Out - rlPrev > 5.001 OR rl.Out - rlPrev < -5.001 THEN
            rlOk := TRUE;
        END_IF;
    END_IF;
    rlPrev := rl.Out;
    // Pwm: Duty=50, Period=4s (8 scans) — count on-scans over 8 scans
    pw(Execute := TRUE, Duty := 50.0, Period := 4.0, MinOn := 0.0,
       CycleTime := 0.5, Reset := FALSE);
    IF cnt > 190 AND cnt <= 198 THEN
        IF pw.Q THEN onCnt := onCnt + 1; END_IF;
    END_IF;
    // HeatCool: U=-40, dz=2 -> Cool=38*1=38
    hc(Execute := TRUE, U := -40.0, DeadZone := 2.0, CoolGain := 1.0);
    flags := 0;
    IF pv > 48.0 AND pv <= 50.0 THEN flags := flags + 1; END_IF;
    IF NOT rlOk THEN flags := flags + 2; END_IF;
    IF onCnt >= 3 AND onCnt <= 5 THEN flags := flags + 4; END_IF;
    IF hc.Cool > 37.9 AND hc.Cool < 38.1 AND hc.Heat = 0.0 THEN
        flags := flags + 8;
    END_IF;
    IF f1.Done AND rl.Done AND pw.Done AND hc.Done THEN
        flags := flags + 16;
    END_IF;
END_PROGRAM
