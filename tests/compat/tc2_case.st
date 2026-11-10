PROGRAM Tc2Case
VAR
    start_in AT %IX0.0 : BOOL;
    mask      AT %QD0   : DINT;
    tmask     AT %QD4   : DINT;
    d2b       : DEC_TO_BCD;
    b2d       : BCD_TO_DEC;
    d2b_err   : DEC_TO_BCD;
    lton1     : LTON;
    ltof1     : LTOF;
    ltp1      : LTP;
    enm       : FB_EnumStringNumbers;
    enm2      : FB_EnumStringNumbers;
    enm3      : FB_EnumStringNumbers;
    scan_n    : UDINT;
    eCmd      : INT;
END_VAR

d2b(START := start_in, DIN := 42);
b2d(START := start_in, BIN := 16#26);
d2b_err(START := start_in, DIN := 200);
lton1(IN := start_in, PT := T#15ms);
ltof1(IN := start_in, PT := T#20ms);
ltp1(IN := start_in, PT := T#25ms);
IF scan_n = 0 THEN
    eCmd := 0;
ELSE
    eCmd := 1;
END_IF
enm(sSearch := 'abc12.5xyz-7', eCmd := eCmd, eType := 0);
enm2(sSearch := 'abc12.5xyz', eCmd := 0, eType := 0);
enm3(sSearch := '-7x', eCmd := 0, eType := 2);
scan_n := scan_n + 1;

mask := 0;
IF CEIL(2.3) = 3.0 THEN
    mask := mask + 1;
END_IF
IF FLOOR(-2.3) = -3.0 THEN
    mask := mask + 2;
END_IF
IF FRAC(3.25) = 0.25 THEN
    mask := mask + 4;
END_IF
IF LTRUNC(-3.7) = -3.0 THEN
    mask := mask + 8;
END_IF
IF LMOD(7.5, 2.0) = 1.5 THEN
    mask := mask + 16;
END_IF
IF MODABS(-7.5, 2.0) = 0.5 THEN
    mask := mask + 32;
END_IF
IF MODTURNS(7.5, 2.0) = 3 THEN
    mask := mask + 64;
END_IF
IF MODTURNS(-7.5, 2.0) = -3 THEN
    mask := mask + 128;
END_IF
IF WCONCAT("ab", "cd") = "abcd" THEN
    mask := mask + 256;
END_IF
IF WLEFT("hello", 2) = "he" THEN
    mask := mask + 512;
END_IF
IF WRIGHT("hello", 2) = "lo" THEN
    mask := mask + 1024;
END_IF
IF WMID("hello", 3, 2) = "ell" THEN
    mask := mask + 2048;
END_IF
IF WFIND("hello", "ll") = 3 THEN
    mask := mask + 4096;
END_IF
IF WLEN("hello") = 5 THEN
    mask := mask + 8192;
END_IF
IF WINSERT("ac", "b", 2) = "abc" THEN
    mask := mask + 16384;
END_IF
IF WDELETE("abc", 1, 2) = "ac" THEN
    mask := mask + 32768;
END_IF
IF WREPLACE("abc", "x", 1, 2) = "axc" THEN
    mask := mask + 65536;
END_IF
IF d2b.BOUT = 16#42 THEN
    mask := mask + 131072;
END_IF
IF b2d.DOUT = 26 THEN
    mask := mask + 262144;
END_IF
IF d2b_err.ERR THEN
    mask := mask + 524288;
END_IF
IF d2b_err.ERRID <> 0 THEN
    mask := mask + 1048576;
END_IF

tmask := 0;
IF lton1.Q = FALSE THEN
    tmask := tmask + 1;
END_IF
IF ltof1.Q = TRUE THEN
    tmask := tmask + 2;
END_IF
IF ltp1.Q = FALSE THEN
    tmask := tmask + 4;
END_IF
IF enm2.sNumber = '12.5' THEN
    tmask := tmask + 8;
END_IF
IF enm3.sNumber = '-7' THEN
    tmask := tmask + 16;
END_IF
IF enm.bEOS = TRUE THEN
    tmask := tmask + 32;
END_IF
IF enm.nPos = 13 THEN
    tmask := tmask + 64;
END_IF
IF enm2.nPos = 8 THEN
    tmask := tmask + 128;
END_IF
END_PROGRAM
