// o23_endpoint.st — FB_AtcEndpoint: Execute at n=1, SettleTime=0.5,
// SampleTime=1.0, MaxRise=10/s, ct=0.1. Pressure ramps 50/s during
// the sample window -> RiseRate ~500 -> Result=2 (wet). Checks valve
// isolation, testing flag, wet classification, valve reopen, rate.
// Sticky bits -> flags %QB0: expect 0x1f.
PROGRAM O23
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    ep : FB_AtcEndpoint;
    p : LREAL;
    n : DINT;
    ex : BOOL;
END_VAR
    n := n + 1;
    ex := (n = 1);
    // flat 100 during settle; +50/s ramp once sampling starts (n>5)
    IF n <= 5 THEN p := 100.0;
    ELSE p := 100.0 + DINT_TO_LREAL(n - 5) * 50.0; END_IF;
    ep(Execute := ex, Pressure := p,
       SettleTime := 0.5, SampleTime := 1.0, MaxRise := 10.0,
       CycleTime := 0.1, Reset := FALSE);
    // valve isolated during settle+sample (st 1,2 -> n=2..15)
    IF n >= 3 AND n <= 14 AND NOT ep.ValveCmd THEN ok0 := TRUE; END_IF;
    IF n = 10 AND ep.Testing THEN ok1 := TRUE; END_IF;
    IF n >= 16 AND ep.Result = 2 THEN ok2 := TRUE; END_IF;
    IF n >= 16 AND ep.ValveCmd AND NOT ep.Testing THEN ok3 := TRUE; END_IF;
    IF n >= 16 AND ep.RiseRate > 100.0 THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
