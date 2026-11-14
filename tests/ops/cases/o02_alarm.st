// o02_alarm.st — FB_AtcAlarm: Cond edge latches Unacked (survives
// Cond clear), Ack clears it, Shelve suppresses Active for
// ShelveTime s, FirstOut respects HigherPriActive.
//   n=1 Cond up   -> Active+Unacked, FirstOut (no higher pri)
//   n=2 Cond down -> Unacked stays, Active off
//   n=3 Ack       -> Unacked off
//   n=4 Cond up   -> edge again (ack still held -> net unacked off)
//   n=5 Shelve 1s -> Active off, Shelved on
//   n=6..13 shelf window: alm2 (same cond, no shelve, higher-pri =
//        alm.Active=FALSE) -> FirstOut TRUE while alm suppressed
//   n=15+ shelf expires -> alm Active back -> alm2 FirstOut off
// Sticky: b0 unacked latch, b1 ack clears, b2 shelve works,
// b3 shelf expiry restores, b4 firstout suppression.
PROGRAM O02
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2, ok3, ok4 : BOOL;
    alm : FB_AtcAlarm;
    alm2 : FB_AtcAlarm;
    cond, ack, shelve : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    IF n = 1 THEN cond := TRUE; END_IF;
    IF n = 2 THEN cond := FALSE; END_IF;
    IF n = 3 THEN ack := TRUE; END_IF;
    IF n = 4 THEN cond := TRUE; END_IF;
    IF n = 5 THEN shelve := TRUE; ack := FALSE; END_IF;
    IF n = 6 THEN shelve := FALSE; END_IF;
    alm(Cond := cond, Ack := ack, Shelve := shelve,
        ShelveTime := 1.0, HigherPriActive := FALSE,
        CycleTime := 0.1);
    alm2(Cond := cond, Ack := FALSE, Shelve := FALSE,
         ShelveTime := 1.0, HigherPriActive := alm.Active,
         CycleTime := 0.1);
    // b0: unacked latched at n=2 even though cond cleared
    IF n = 2 AND alm.Unacked AND NOT alm.Active THEN ok0 := TRUE; END_IF;
    // b1: ack at n=3 cleared unacked
    IF n = 3 AND NOT alm.Unacked THEN ok1 := TRUE; END_IF;
    // b2: shelved at n=6 — cond up but Active suppressed
    IF n = 6 AND alm.Shelved AND NOT alm.Active AND alm.ShelfET > 0.5
       THEN ok2 := TRUE; END_IF;
    // b3: shelf expired -> Active again while cond still up
    IF n >= 15 AND alm.Active AND NOT alm.Shelved THEN ok3 := TRUE; END_IF;
    // b4: during shelf window alm2 is the first-out (alm suppressed)
    IF n = 8 AND alm2.FirstOut AND NOT alm.Active THEN ok4 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
    IF ok3 THEN flags := flags + 8; END_IF;
    IF ok4 THEN flags := flags + 16; END_IF;
END_PROGRAM
