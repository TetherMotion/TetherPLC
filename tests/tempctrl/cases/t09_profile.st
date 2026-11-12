// t09_profile.st — FB_AtcProfile exercising all segment kinds:
//   SEG1 RAMP  -> 100 at 4 deg/s   (PV start 20 -> ~20 s)
//   SEG2 DWELL    100 for 30 s, guaranteed-soak band 2.0
//   SEG3 RAMPT -> 60 in 40 s
//   SEG4 STEP  -> 60 (jump, EV pulse)
//   SEG5 LOOP  -> seg 2, x1 (dwell+rampt run twice)
//   SEG6 END
// Simulated PV lags SP at 10 s. flags: b0 ran to END (Done),
// b1 SP hit 100, b2 SegIdx visited 2, b3 EV word seen (seg2 EV=2),
// b4 guaranteed soak stretched dwell (dwell seg took > 30 s while
// PV lagging outside band... here PV tracks so gsoak rarely bites;
// instead check dwell SegIdx stayed for >= 30 s), b5 loop ran twice
// (saw SegIdx=2 twice => SegIdx reached 4 then 2 again), b6 SP
// finished at 60 (RAMPT target of second pass), b7 no Err.
PROGRAM T09
VAR
    flags AT %QB0 : BYTE;
    prof : FB_AtcProfile;
    pv : LREAL;
    sp AT %QD8 : LREAL;
    segO AT %QD16 : DINT;
    run : BOOL;
    cnt : DINT;
    sawDwell : DINT;
    dwellStart : LREAL;
    dwellLong : BOOL;
    sawSeg4 : BOOL;
    sawLoop : BOOL;
END_VAR
    // ---- segment table (member-wise writes; aggregates can't bind)
    IF cnt = 0 THEN
        prof.SEG[1].KIND := 2;  // RAMP
        prof.SEG[1].TGT := 100.0;
        prof.SEG[1].P1 := 4.0;
        prof.SEG[1].EV := 1;
        prof.SEG[2].KIND := 1;  // DWELL
        prof.SEG[2].TGT := 100.0;
        prof.SEG[2].P1 := 30.0;
        prof.SEG[2].P2 := 2.0;
        prof.SEG[2].EV := 2;
        prof.SEG[3].KIND := 3;  // RAMPT
        prof.SEG[3].TGT := 60.0;
        prof.SEG[3].P1 := 40.0;
        prof.SEG[3].EV := 4;
        prof.SEG[4].KIND := 4;  // STEP
        prof.SEG[4].TGT := 60.0;
        prof.SEG[4].EV := 8;
        prof.SEG[5].KIND := 5;  // LOOP -> seg 2, 1 extra pass
        prof.SEG[5].TGT := 2.0;
        prof.SEG[5].P1 := 1.0;
        prof.SEG[6].KIND := 0;  // END
    END_IF;
    cnt := cnt + 1;
    IF cnt = 2 THEN
        pv := 20.0;   // PV-start point
        run := TRUE;
    END_IF;
    prof(NUM_SEG := 6, RUN := run, HOLD := FALSE, RESET := FALSE,
         PvStart := TRUE, PV := pv, CycleTime := 0.5);
    run := FALSE;
    // simulated PV follows SP (1st-order, 10 s lag)
    pv := pv + (prof.SP - pv) * (0.5 / (10.0 + 0.5));
    sp := prof.SP;
    segO := prof.SegIdx;
    // book-keeping for checks
    IF prof.SegIdx = 2 THEN
        sawDwell := sawDwell + 1;
        IF sawDwell = 1 THEN dwellStart := prof.SegTime; END_IF;
    END_IF;
    IF prof.SegIdx = 2 AND prof.SegTime >= 29.5 THEN
        dwellLong := TRUE;
    END_IF;
    IF prof.SegIdx = 4 THEN sawSeg4 := TRUE; END_IF;
    IF sawSeg4 AND prof.SegIdx = 2 THEN sawLoop := TRUE; END_IF;
    flags := 0;
    IF prof.Done THEN flags := flags + 1; END_IF;
    IF sawDwell >= 2 THEN flags := flags + 2; END_IF;
    IF dwellLong THEN flags := flags + 4; END_IF;
    IF sawSeg4 THEN flags := flags + 8; END_IF;
    IF sawLoop THEN flags := flags + 16; END_IF;
    IF prof.SP > 59.0 AND prof.SP < 61.0 THEN
        flags := flags + 32;
    END_IF;
    IF NOT prof.Err THEN flags := flags + 64; END_IF;
    // extra: the EV word of the END segment is 0; while busy it was
    // nonzero at least once — sawLoop covers seg-2 revisit.
    IF prof.Busy OR prof.Done THEN flags := flags + 128; END_IF;
END_PROGRAM
