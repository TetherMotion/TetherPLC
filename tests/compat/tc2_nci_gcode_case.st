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
stStart := (st = 6) AND (waitN = 2);
stStop  := (st = 10) AND (waitN = 2);
rsEx    := (st = 3) OR (st = 11);
rsEx    := rsEx AND (waitN = 2);

sts(ChanId := 0);
ldt(Execute := ldtEx, ChanId := 0,
    ProgText := 'G1 X1 F600$NM30$N');
ldf(Execute := ldfEx, ChanId := 0,
    PathName := '/tmp/tplc_nci_case.g');
ss(Start := stStart, Stop := stStop, ChanId := 0);
rst(Execute := rsEx, ChanId := 0);

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
END_IF;

END_PROGRAM
