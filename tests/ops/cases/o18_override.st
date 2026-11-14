// o18_override.st — FB_AtcOverride manual/auto + release track.
// AutoIn=50 ManIn=80 TrackIn=60; ManEn n=2..4.
// n=5 release scan -> Out=TrackIn=60 Tracking; n>=6 -> AutoIn=50.
// Sticky: b0 manual, b1 release-sync, b2 auto passthrough.
PROGRAM O18
VAR
    flags AT %QB0 : BYTE;
    ok0, ok1, ok2 : BOOL;
    ov : FB_AtcOverride;
    men : BOOL;
    n : DINT;
END_VAR
    n := n + 1;
    men := n >= 2 AND n <= 4;
    ov(AutoIn := 50.0, ManIn := 80.0, ManEn := men,
       TrackIn := 60.0);
    IF n = 3 AND ov.Out = 80.0 THEN ok0 := TRUE; END_IF;
    IF n = 5 AND ov.Tracking AND ov.Out = 60.0 THEN ok1 := TRUE; END_IF;
    IF n = 7 AND NOT ov.Tracking AND ov.Out = 50.0
       THEN ok2 := TRUE; END_IF;
    flags := 0;
    IF ok0 THEN flags := flags + 1; END_IF;
    IF ok1 THEN flags := flags + 2; END_IF;
    IF ok2 THEN flags := flags + 4; END_IF;
END_PROGRAM
