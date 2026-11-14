// o05_sensorcheck.st — FB_AtcSensorCheck: Min=0 Max=100 MaxRate=50
// EU/s StuckBand=0.5 StuckTime=1.0 ct=0.1.
//   n=1  In=50 init (no rate/frozen yet)
//   n=2.. In=50 frozen -> Reason=3 Bad latches ~n=6 (0.5 s)
//   n=12 In=150 -> range Reason=1
//   n=13 In=50 -> in range again; Bad stays latched (Reason=1 latched)
//   n=14 Reset -> Bad/Reason clear
//   n=15 In=80 jump +30 in 0.1 s -> rate 300>50 Reason=2
//   n=16 In=80.4 (within band) frozen resumes
// Sticky: b0 stuck reason3, b1 range reason1, b2 bad latched,
// b3 rate reason2, b4 reset clears.
PROGRAM O05
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    sc : FB_AtcSensorCheck;
    inp : LREAL;
    rst : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    inp := 50.0;
    IF n = 1 THEN inp := 50.0; END_IF;
    IF n = 12 THEN inp := 150.0; END_IF;
    IF n = 13 THEN inp := 50.0; END_IF;
    rst := n = 14 OR n = 16;
    IF n = 15 THEN inp := 80.0; END_IF;
    IF n >= 16 THEN inp := 80.4; END_IF;
    // n=15 rate violation latches Bad; n=16 resets to check recovery
    sc(In := inp, Min := 0.0, Max := 100.0, MaxRate := 50.0,
       StuckBand := 0.5, StuckTime := 0.5, Reset := rst,
       CycleTime := 0.1);
    IF n >= 7 AND n <= 11 AND sc.Reason = 3 AND sc.Bad THEN ok0 := TRUE; END_IF;
    IF n = 12 AND sc.Reason = 1 THEN ok1 := TRUE; END_IF;
    IF n = 13 AND sc.Bad AND NOT sc.Ok THEN ok2 := TRUE; END_IF;
    IF n = 15 AND sc.Reason = 2 THEN ok3 := TRUE; END_IF;
    IF n = 16 AND NOT sc.Bad AND sc.Ok AND sc.Reason = 0
       THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
