// o06_seq.st — FB_AtcSeq drum sequencer, ct=0.1.
//   step1 TIME=0.5 NEXT=2 EV=1 ; step2 TIME=0 (wait StepDone)
//   NEXT=3 EV=2 ; step3 TIME=0.5 NEXT=0 EV=4
//   n=1 RUN -> step1; StepTime hits 0.5 at n=6 -> step2
//   n=8 StepDone pulse -> step3; hits 0.5 at n=13 -> NEXT=0
//   n=14 Done, Busy off.
// Sticky: b0 step1 active, b1 step2 reached, b2 step3 via StepDone,
// b3 Done+not Busy, b4 EV of step3 seen.
PROGRAM O06
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    sq : FB_AtcSeq;
    run, sdone : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    // segment table written member-wise (aggregates can't bind)
    IF n = 1 THEN
        sq.STEPS[1].TIME := 0.5; sq.STEPS[1].NEXT := 2;
        sq.STEPS[1].EV := 1;
        sq.STEPS[2].TIME := 0.0; sq.STEPS[2].NEXT := 3;
        sq.STEPS[2].EV := 2;
        sq.STEPS[3].TIME := 0.5; sq.STEPS[3].NEXT := 0;
        sq.STEPS[3].EV := 4;
    END_IF;
    IF n = 1 THEN run := TRUE; END_IF;
    IF n = 8 THEN sdone := TRUE; END_IF;
    IF n = 9 THEN sdone := FALSE; END_IF;
    sq(NUM_STEPS := 3, RUN := run, HOLD := FALSE,
       RESET := FALSE, StepDone := sdone, CycleTime := 0.1);
    IF n = 3 AND sq.StepIdx = 1 AND sq.Busy THEN ok0 := TRUE; END_IF;
    IF n = 7 AND sq.StepIdx = 2 THEN ok1 := TRUE; END_IF;
    IF n = 9 AND sq.StepIdx = 3 THEN ok2 := TRUE; END_IF;
    IF n >= 14 AND sq.Done AND NOT sq.Busy THEN ok3 := TRUE; END_IF;
    IF n = 10 AND sq.EV = 4 THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
