PROGRAM Tc3RtmonCase
VAR
    mask      AT %QD0   : DINT;
    st        : INT;
    scan_n    : UDINT;
    mkBase    : FB_RTMon_LogMarkBase;
    mkEx      : FB_RTMon_LogMarkBaseEx;
    mStart    : FB_RTMon_LogStart;
    mPlain    : FB_RTMon_LogMark;
    cc        : FB_RtMon;
    c0        : FB_RtMon;
    g0        : FB_RtMon;
    g3        : FB_RtMon;
    g4        : FB_RtMon;
END_VAR

// autonomous realtime-monitor sequence against the tplc-rtmon ring:
// mark x4 (base, ex, simple start, simple plain) -> count=4 ->
// get[0] -> get[3] -> get[4] out-of-range -> clear -> count=0.
mkBase(bExecute := st = 0, nContextId := 1, nGroupId := 10,
       nEventId := 20, nMarkCtrl := 16#E8000000);
mkEx(bExecute := st = 1, nContextId := 2, nGroupId := 30,
     nEventId := 40, nMarkCtrl := 16#60000000);
mStart(bExecute := st = 2, nContextId := 3, bLogCallingTask := TRUE,
       nGroupId := 50, nEventId := 60);
mPlain(bExecute := st = 3, nContextId := 4, bLogCallingTask := FALSE,
       nGroupId := 70, nEventId := 80);

cc.OP := 2;
cc.EXEC := (st = 4) OR (st = 9);
cc();
c0.OP := 4;
c0.EXEC := st = 8;
c0();
g0.OP := 3;
g0.AUX := 0;
g0.EXEC := st = 5;
g0();
g3.OP := 3;
g3.AUX := 3;
g3.EXEC := st = 6;
g3();
g4.OP := 3;
g4.AUX := 4;
g4.EXEC := st = 7;
g4();

IF st = 0 AND mkBase.bDone THEN
    IF mkBase.bError = FALSE AND mkBase.hrErrorCode = 0 THEN
        mask := mask + 1;
    END_IF;
    st := 1;
ELSIF st = 1 AND mkEx.bDone THEN
    IF mkEx.bError = FALSE AND mkEx.hrErrorCode = 0 THEN
        mask := mask + 2;
    END_IF;
    st := 2;
ELSIF st = 2 AND mStart.bDone THEN
    IF mStart.bError = FALSE AND mStart.hrErrorCode = 0 THEN
        mask := mask + 4;
    END_IF;
    st := 3;
ELSIF st = 3 AND mPlain.bDone THEN
    IF mPlain.bError = FALSE AND mPlain.hrErrorCode = 0 THEN
        mask := mask + 8;
    END_IF;
    st := 4;
ELSIF st = 4 AND cc.DONE THEN
    IF cc.RESULT = 4 AND cc.ERRID = 0 THEN
        mask := mask + 16;
    END_IF;
    st := 5;
ELSIF st = 5 AND g0.DONE THEN
    IF g0.ERRID = 0 AND g0.CONTEXT = 1 AND g0.GROUP = 10 AND
       g0.EVENT = 20 AND g0.MARKCTRL = 16#E8000000 THEN
        mask := mask + 32;
    END_IF;
    st := 6;
ELSIF st = 6 AND g3.DONE THEN
    IF g3.ERRID = 0 AND g3.CONTEXT = 4 AND g3.GROUP = 70 AND
       g3.EVENT = 80 AND g3.MARKCTRL = 0 THEN
        mask := mask + 64;
    END_IF;
    st := 7;
ELSIF st = 7 AND g4.DONE THEN
    IF g4.ERRID = 3 THEN
        mask := mask + 128;
    END_IF;
    st := 8;
ELSIF st = 8 AND c0.DONE THEN
    IF c0.ERRID = 0 THEN
        mask := mask + 256;
    END_IF;
    st := 9;
ELSIF st = 9 AND cc.DONE THEN
    IF cc.RESULT = 0 AND cc.ERRID = 0 THEN
        mask := mask + 512;
    END_IF;
    st := 10;
END_IF;

scan_n := scan_n + 1;
END_PROGRAM
