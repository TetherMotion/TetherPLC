PROGRAM Tc2NciFifoCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    cur    : INT;
    pulse  : BOOL;
    ax     : FB_MotionAxis;
    axM    : UDINT;
    axS    : UDINT;
    pwrM   : MC_Power;
    pwrS   : MC_Power;
    integ1 : FiFoGroupIntegrate;
    integ2 : FiFoGroupIntegrate;
    dim    : FiFoGetDimension;
    wr     : FiFoWrite;
    owr    : FiFoOverwrite;
    ovr    : FiFoSetChannelOverride;
    start  : FiFoStart;
    dis    : FiFoGroupDisintegrate;
    mon    : FB_Fifo;
    dim2   : FiFoGetDimension;
    rpM    : MC_ReadActualPosition;
    rpS    : MC_ReadActualPosition;
    data   : ARRAY[0..255] OF LREAL;
    k      : UDINT;
    enSig  : BOOL;
    finishedSeen : BOOL;
    seqOk  : BOOL;
    seqOk2 : BOOL;
    posOk1 : BOOL;
    posOk2 : BOOL;
    startEx : BOOL;
END_VAR

// Stage machine: one stage per scan via the cur snapshot.
cur := st;
IF cur <> prevSt THEN pulse := TRUE; ELSE pulse := FALSE; END_IF;
prevSt := cur;

// Channel monitor: status+poll every scan. In-process backends
// consume one queued row per poll; the IO backend consumes rows on
// the mc.time drive clock (poll is a no-op there).
mon.OP := 6;
mon.CHAN := 0;
mon();

enSig := (st >= 1) AND (st <= 40);
// FiFoStart is invoked every scan so its Execute edge tracking stays
// live across the two run stages.
startEx := pulse AND ((cur = 6) OR (cur = 10));
start(bExecute := startEx, iChannelId := 0, tTimeout := T#1S);
pwrM(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
    Axis := axM);
pwrS(Enable := enSig, Enable_Positive := TRUE, Enable_Negative := TRUE,
    Axis := axS);
rpM(Enable := TRUE, Axis := axM);
rpS(Enable := TRUE, Axis := axS);

IF cur = 0 THEN
    ax.UNITS_PER_REV := 1000;
    ax.MAX_VEL := 2000;
    ax.MAX_ACCEL := 8000;
    ax.MAX_DECEL := 8000;
    ax.POS_MIN := -200000;
    ax.POS_MAX := 200000;
    ax.HOME_OFFSET := 0;
    ax.FOLLOW_WINDOW := 3;
    ax.FLAGS := 1;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axM := ax.AXIS; END_IF;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axS := ax.AXIS; END_IF;
    IF axM <> 0 AND axS <> 0 THEN
        mask := mask + 1;                       // bit0: two axes
        st := 1;
    END_IF;
ELSIF cur = 1 THEN
    IF pwrM.Status AND pwrS.Status THEN
        mask := mask + 2;                       // bit1: powered
        st := 2;
    END_IF;
ELSIF cur = 2 THEN
    integ1(bExecute := pulse, iChannelId := 0, iAxisId := axM,
           iGroupPosition := 1, tTimeout := T#1S);
    integ2(bExecute := pulse, iChannelId := 0, iAxisId := axS,
           iGroupPosition := 2, tTimeout := T#1S);
    IF pulse AND NOT integ1.bError AND NOT integ2.bError THEN
        mask := mask + 4;                       // bit2: integrated
        st := 3;
    END_IF;
ELSIF cur = 3 THEN
    dim(bExecute := pulse, iChannelId := 0, tTimeout := T#1S);
    IF dim.bError = FALSE THEN
        IF dim.iAxes = 2 AND dim.iMaxRows >= 100 THEN
            mask := mask + 8;                   // bit3: dimension
            st := 4;
        END_IF;
    END_IF;
ELSIF cur = 4 THEN
    // two rows x two axes: [100,200] then [300,400]
    data[0] := 100.0; data[1] := 200.0;
    data[8] := 300.0; data[9] := 400.0;
    wr(bExecute := pulse, iChannelId := 0,
       AdrDataArray := data, iRowsToWrite := 2, tTimeout := T#1S);
    IF pulse AND NOT wr.bError THEN
        mask := mask + 16;                      // bit4: write
        st := 5;
    END_IF;
ELSIF cur = 5 THEN
    ovr(bExecute := pulse, iChannelId := 0, iOverride := 150,
        tTimeout := T#1S);
    IF pulse AND NOT ovr.bError THEN
        mask := mask + 32;                      // bit5: override
        st := 6;
    END_IF;
ELSIF cur = 6 THEN
    IF pulse AND NOT start.bError THEN
        mask := mask + 64;                      // bit6: start
        st := 7;
    END_IF;
ELSIF cur = 7 THEN
    // wait for the channel to drain (state 2 = finished)
    IF mon.STATE = 2 THEN
        IF mon.SEQ = 2 THEN
            seqOk := TRUE;
        END_IF;
        finishedSeen := TRUE;
        st := 8;
    END_IF;
ELSIF cur = 8 THEN
    // positions should have snapped/tracked to the last row
    IF rpM.Valid AND rpS.Valid THEN
        IF DINT_TO_LREAL(rpM.Position) >= 295.0 AND
           DINT_TO_LREAL(rpS.Position) >= 395.0 THEN
            posOk1 := TRUE;
        END_IF;
    END_IF;
    IF posOk1 THEN
        mask := mask + 128;                     // bit7: row positions
        st := 9;
    END_IF;
ELSIF cur = 9 THEN
    // overwrite the queue with a single row [500,600] and rerun
    FOR k := 0 TO 255 DO data[k] := 0.0; END_FOR;
    data[0] := 500.0; data[1] := 600.0;
    owr(bExecute := pulse, iChannelId := 0,
        AdrDataArray := data, iRowsToWrite := 1, tTimeout := T#1S);
    IF pulse AND NOT owr.bError THEN
        mask := mask + 256;                     // bit8: overwrite
        st := 10;
    END_IF;
ELSIF cur = 10 THEN
    IF pulse AND NOT start.bError THEN
        st := 11;
    END_IF;
ELSIF cur = 11 THEN
    IF mon.STATE = 2 THEN
        IF mon.SEQ = 1 THEN seqOk2 := TRUE; END_IF;
        st := 12;
    END_IF;
ELSIF cur = 12 THEN
    IF rpM.Valid AND rpS.Valid THEN
        IF DINT_TO_LREAL(rpM.Position) >= 495.0 AND
           DINT_TO_LREAL(rpS.Position) >= 595.0 THEN
            posOk2 := TRUE;
        END_IF;
    END_IF;
    IF posOk2 THEN
        mask := mask + 512;                     // bit9: overwrite pos
        st := 13;
    END_IF;
ELSIF cur = 13 THEN
    dis(bExecute := pulse, iChannelId := 0, tTimeout := T#1S);
    IF pulse AND NOT dis.bError THEN
        dim2(bExecute := pulse, iChannelId := 0, tTimeout := T#1S);
        IF dim2.iAxes = 0 THEN
            mask := mask + 1024;                // bit10: disintegrate
            st := 14;
        END_IF;
    END_IF;
ELSIF cur = 14 THEN
    // final: fold run flags into spare bits (one-shot stage)
    IF seqOk AND seqOk2 THEN mask := mask + 2048; END_IF; // bit11
    IF posOk1 THEN mask := mask + 4096; END_IF;           // bit12
    IF posOk2 THEN mask := mask + 8192; END_IF;           // bit13
    st := 15;
ELSIF cur = 15 THEN
    // hold; expect written at final step
END_IF;
END_PROGRAM
