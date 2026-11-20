// t18_trend.st — FB_AtcTrend ring-buffer capture.  Depth=64,
// Value=cycle index, Reset on cycle 1 (records start cycle 2),
// trigger at cycle 20 -> ring pos 18, PostN=4.  72 writes total:
// Wr=70, Full wraps at write 64.
// flags: b0 Done, b1 Full, b2 TrigIdx=18, b3 Buf[TrigIdx]=20
// (trigger sample, not yet overwritten), b4 Count>=40, b5 Wr=72 mod 64 = 8.
PROGRAM T18
VAR
    flags AT %QB0 : BYTE;
    tr : FB_AtcTrend;
    cyc : DINT;
END_VAR
    cyc := cyc + 1;
    tr(Execute := TRUE, CycleTime := 0.1,
       Value := DINT_TO_LREAL(cyc), Depth := 64, PostN := 4,
       Arm := TRUE, Trig := cyc = 20, Reset := cyc = 1);
    flags := 0;
    IF tr.Done THEN flags := flags + 1; END_IF;
    IF tr.Full THEN flags := flags + 2; END_IF;
    IF tr.TrigIdx = 18 THEN flags := flags + 4; END_IF;
    IF tr.TrigIdx >= 0 THEN
        IF tr.Buf[tr.TrigIdx] = 20.0 THEN flags := flags + 8; END_IF;
    END_IF;
    IF tr.Count >= 40 THEN flags := flags + 16; END_IF;
    IF tr.Wr = 8 THEN flags := flags + 32; END_IF;
END_PROGRAM
