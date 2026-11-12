PROGRAM Tc2NciGcodeCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    ldt    : FB_ItpLoadText;
    ldf    : FB_ItpLoadProg;
    ss     : FB_ItpStartStop;
    rst    : FB_ItpReset;
    sts    : FB_ItpStatus;
    ldtEx  : BOOL;
    ldfEx  : BOOL;
    stStart : BOOL;
    stStop : BOOL;
    rsEx   : BOOL;
    waitN  : INT;
    fwA    : FB_ItpFiFoWrite;
    fwB    : FB_ItpFiFoWrite;
    cnf    : FB_ItpConfirmHsk;
    rpw    : FB_ItpWriteRParam;
    rpr    : FB_ItpReadRParam;
    zs     : FB_ItpZeroShift;
    fwAEx  : BOOL;
    fwBEx  : BOOL;
    cnfEx  : BOOL;
    rpwEx  : BOOL;
    rprEx  : BOOL;
    zsEx   : BOOL;
    rpDone : BOOL;
    m8Seen : BOOL;
END_VAR

// The 7-block program lives on the server host (the shell script
// writes /tmp/tplc_nci_case.g before the sim runs); inline text is
// exercised with a tiny two-block program.
IF st = 0 THEN
    mask := 0;
    st := 1;
END_IF;

IF st <> prevSt THEN waitN := 0; END_IF;
prevSt := st;
waitN := waitN + 1;

ldtEx   := (st = 1) AND (waitN = 2);
ldfEx   := (st = 4) AND (waitN = 2);
stStart := ((st = 6) OR (st = 14) OR (st = 19)) AND (waitN = 2);
stStop  := (st = 10) AND (waitN = 2);
rsEx    := (st = 3) OR (st = 11) OR (st = 16) OR (st = 21);
rsEx    := rsEx AND (waitN = 2);
fwAEx   := (st = 13) AND (waitN = 2);
fwBEx   := (st = 13) AND (waitN = 3);
rpwEx   := (st = 18) AND (waitN = 2);
rprEx   := (st = 18) AND (waitN = 4);
zsEx    := (st = 18) AND (waitN = 6);
cnfEx   := (st = 19) AND (sts.MFunc = 8);

sts(ChanId := 0);
ldt(Execute := ldtEx, ChanId := 0,
    ProgText := 'G1 X1 F600$NM30$N');
ldf(Execute := ldfEx, ChanId := 0,
    PathName := '/tmp/tplc_nci_case.g');
ss(Start := stStart, Stop := stStop, ChanId := 0);
rst(Execute := rsEx, ChanId := 0);
fwA(Execute := fwAEx, ChanId := 0, Block := 'G1 X2 F600');
fwB(Execute := fwBEx, ChanId := 0, Block := 'G1 X0 F600');
cnf(Execute := cnfEx, ChanId := 0);
rpw(Execute := rpwEx, ChanId := 0, Index := 100, Value := 42.5);
rpr(Execute := rprEx, ChanId := 0, Index := 100);
zs(Execute := zsEx, ChanId := 0, OffX := 5.0, OffY := 0.0, OffZ := 0.0);

IF st = 1 THEN
    // inline text upload accepted
    IF ldt.Done THEN
        mask := mask + 1;                    // bit0: text upload ok
        st := 2;
    ELSIF ldt.Error THEN
        st := 90;
    END_IF;
ELSIF st = 2 THEN
    // compiled: READY (state 1) with 2 blocks
    IF sts.Valid AND (sts.State = 1) THEN
        mask := mask + 2;                    // bit1: ready
        IF sts.Total = 2 THEN
            mask := mask + 4;                // bit2: 2 blocks
        END_IF;
        st := 3;
    ELSIF sts.Valid AND (sts.State = 5) THEN
        st := 91;
    END_IF;
ELSIF st = 3 THEN
    // reset back to IDLE before the file load
    IF rst.Done THEN
        mask := mask + 8;                    // bit3: reset ok
        st := 4;
    ELSIF rst.Error THEN
        st := 92;
    END_IF;
ELSIF st = 4 THEN
    // file load accepted
    IF ldf.Done THEN
        mask := mask + 16;                   // bit4: file load ok
        st := 5;
    ELSIF ldf.Error THEN
        st := 93;
    END_IF;
ELSIF st = 5 THEN
    // file compiled: READY with 7 blocks
    IF sts.Valid AND (sts.State = 1) THEN
        mask := mask + 32;                   // bit5: ready
        IF sts.Total = 7 THEN
            mask := mask + 64;               // bit6: 7 blocks
        END_IF;
        st := 6;
    ELSIF sts.Valid AND (sts.State = 5) THEN
        st := 94;
    END_IF;
ELSIF st = 6 THEN
    IF ss.Done THEN
        mask := mask + 128;                  // bit7: run accepted
        st := 7;
    ELSIF ss.Error THEN
        st := 95;
    END_IF;
ELSIF st = 7 THEN
    // running: mid-program block reached or raced to finish
    IF sts.Valid AND (sts.State = 2) AND (sts.Line >= 3) THEN
        mask := mask + 256;                  // bit8: mid-program
        st := 8;
    ELSIF sts.Valid AND (sts.State = 4) THEN
        st := 8;
    END_IF;
ELSIF st = 8 THEN
    IF sts.Valid AND (sts.State = 4) THEN
        mask := mask + 512;                  // bit9: finished
        st := 9;
    ELSIF sts.Valid AND (sts.State = 5) THEN
        st := 96;
    END_IF;
ELSIF st = 9 THEN
    // planned duration ~4.1s at the given feeds
    IF sts.DurationMs >= 3500 AND sts.DurationMs <= 5000 THEN
        mask := mask + 1024;                 // bit10: duration sane
    END_IF;
    st := 10;
ELSIF st = 10 THEN
    // permissive stop after finish
    IF ss.Done THEN
        mask := mask + 2048;                 // bit11: stop ok
        st := 11;
    ELSIF ss.Error THEN
        st := 97;
    END_IF;
ELSIF st = 11 THEN
    IF rst.Done THEN
        mask := mask + 4096;                 // bit12: reset ok
        st := 12;
    ELSIF rst.Error THEN
        st := 98;
    END_IF;
ELSIF st = 12 THEN
    // reset returns the channel to IDLE with no program
    IF sts.Valid AND (sts.State = 0) AND (sts.Total = 0) THEN
        mask := mask + 8192;                 // bit13: idle cleared
        st := 13;
    END_IF;
ELSIF st = 13 THEN
    // two FIFO blocks queued (Done pulses are per-scan; the queue
    // depth is the real check)
    IF sts.Valid AND sts.FifoLen = 2 THEN
        mask := mask + 16384;                // bit14: fifo queued
        st := 14;
    END_IF;
ELSIF st = 14 THEN
    // run the FIFO-only program from idle (stStart pulses above)
    IF ss.Done THEN
        st := 15;
    END_IF;
ELSIF st = 15 THEN
    // both FIFO entries consumed, channel finished
    IF sts.Valid AND (sts.State = 4) AND (sts.FifoSeq = 2) THEN
        mask := mask + 32768;                // bit15: fifo chained
        st := 16;
    END_IF;
ELSIF st = 16 THEN
    IF rst.Done THEN
        mask := mask + 65536;                // bit16: reset ok
        st := 17;
    END_IF;
ELSIF st = 17 THEN
    // M-func program: M8 pauses the channel with pending code
    ldt(Execute := TRUE, ChanId := 0,
        ProgText := 'G21 G90$NG1 X5 F600$NM8$NG1 X10$NM30$N');
    IF ldt.Done THEN
        st := 18;
    END_IF;
ELSIF st = 18 THEN
    // R-param roundtrip + zero-shift write in the same wait window
    IF rpw.Done THEN mask := mask + 131072; END_IF;   // bit17: rp write
    IF rpr.Done AND (rpr.Value > 42.0) AND (rpr.Value < 43.0) THEN
        mask := mask + 262144;                        // bit18: rp read
        rpDone := TRUE;
    END_IF;
    IF zs.Done THEN mask := mask + 524288; END_IF;    // bit19: zshift
    IF (mask AND 917504) = 917504 THEN
        st := 19;
    END_IF;
ELSIF st = 19 THEN
    // run the M-func program; it pauses on M8 (stStart pulses above)
    IF sts.Valid AND (sts.MFunc = 8) AND NOT m8Seen THEN
        mask := mask + 1048576;                // bit20: M8 pending
        m8Seen := TRUE;
    END_IF;
    IF sts.Valid AND (sts.State = 4) AND
       (sts.MFunc = 0) THEN
        mask := mask + 2097152;                // bit21: resumed+done
        st := 20;
    END_IF;
ELSIF st = 20 THEN
    st := 21;
ELSIF st = 21 THEN
    IF rst.Done THEN
        mask := mask + 4194304;                // bit22: reset ok
        st := 22;
    END_IF;
END_IF;

END_PROGRAM
