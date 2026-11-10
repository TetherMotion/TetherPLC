PROGRAM Tc2Case
VAR
    start_in AT %IX0.0 : BOOL;
    mask      AT %QD0   : DINT;
    d2b       : DEC_TO_BCD;
    b2d       : BCD_TO_DEC;
    d2b_err   : DEC_TO_BCD;
END_VAR

d2b(START := start_in, DIN := 42);
b2d(START := start_in, BIN := 16#26);
d2b_err(START := start_in, DIN := 200);

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
END_PROGRAM
