// profinet_case.st — tplc-profinet e2e against
// fake_profinet_device.py (TPLC_PROFINET_IO=127.0.0.1:PORT):
// exercises FB_Profinet open, service state, DCP Identify (all
// device properties), DCP Get name, DCP Set+Get alias, cyclic IO
// record exchange, close, and invalid-op rejection.  One FB call
// per scan (each call is a synchronous UDP round-trip).
PROGRAM profinet_case
VAR
    pn : FB_Profinet;
    st : INT;
    mask AT %QD0 : UDINT;
    ok1 : BOOL;
    ok2 : BOOL;
    ok3 : BOOL;
    ok4 : BOOL;
    ok5 : BOOL;
    ok6 : BOOL;
    ok7 : BOOL;
    ok8 : BOOL;
    ok9 : BOOL;
    ok10 : BOOL;
    ok11 : BOOL;
    ok12 : BOOL;
    ok13 : BOOL;
    ok14 : BOOL;
    ok15 : BOOL;
    ok16 : BOOL;
    ok17 : BOOL;
END_VAR
    IF st = 0 THEN
        mask := 0;
        pn.OP := 0;
        pn.PORT := 0;
        pn.XID := 16#12345;
        pn.OPTION := 0;
        pn.SUB := 0;
        pn.DLEN := 0;
        st := 1;
    ELSIF st = 1 THEN
        // OP 1: open port 0 (UDP socket + peer)
        pn.OP := 1;
        pn.PORT := 0;
        pn();
        ok1 := pn.STATUS = 0;
        IF ok1 THEN mask := mask + 1; END_IF;
        pn.OP := 3;
        pn.PORT := 0;
        st := 2;
    ELSIF st = 2 THEN
        pn();
        ok2 := pn.STATUS = 1;
        IF ok2 THEN mask := mask + 2; END_IF;
        // OP 4: DCP Identify (all-selector)
        pn.OP := 4;
        pn.PORT := 0;
        pn.XID := 16#111;
        st := 3;
    ELSIF st = 3 THEN
        pn();
        ok3 := pn.STATUS = 0 AND pn.VENDOR = 16#002A AND
               pn.DEVID = 16#BEEF AND pn.ROLE = 1 AND
               pn.DEVINST = 7 AND pn.OEM = 16#CAFEBABE AND
               pn.NAME = 'tplc-dev-1';
        IF ok3 THEN mask := mask + 4; END_IF;
        // OP 5: DCP Get name-of-station (opt 2 / sub 2)
        pn.OP := 5;
        pn.PORT := 0;
        pn.XID := 16#222;
        pn.OPTION := 2;
        pn.SUB := 2;
        st := 4;
    ELSIF st = 4 THEN
        pn();
        ok4 := pn.STATUS = 0 AND pn.DLEN = 10 AND
               pn.OD[0] = 116 AND pn.OD[9] = 49;
        IF ok4 THEN mask := mask + 8; END_IF;
        // OP 6: DCP Set alias (opt 2 / sub 6) <- "AB"
        pn.OP := 6;
        pn.PORT := 0;
        pn.XID := 16#333;
        pn.OPTION := 2;
        pn.SUB := 6;
        pn.OD[0] := 65;
        pn.OD[1] := 66;
        pn.DLEN := 2;
        st := 5;
    ELSIF st = 5 THEN
        pn();
        ok5 := pn.STATUS = 0;
        IF ok5 THEN mask := mask + 16; END_IF;
        // read it back
        pn.OP := 5;
        pn.PORT := 0;
        pn.XID := 16#444;
        pn.OPTION := 2;
        pn.SUB := 6;
        pn.DLEN := 0;
        st := 6;
    ELSIF st = 6 THEN
        pn();
        ok6 := pn.STATUS = 0 AND pn.DLEN = 2 AND
               pn.OD[0] = 65 AND pn.OD[1] = 66;
        IF ok6 THEN mask := mask + 32; END_IF;
        // OP 7: cyclic IO record exchange
        pn.OP := 7;
        pn.PORT := 0;
        pn.IOOUT[0] := 1;
        pn.IOOUT[1] := 2;
        pn.IOOUT[2] := 3;
        pn.IOOUT[3] := 4;
        pn.DLEN := 4;
        st := 7;
    ELSIF st = 7 THEN
        pn();
        // device replies each byte + 1
        ok7 := pn.STATUS = 0 AND pn.DLEN = 4 AND
               pn.IOIN[0] = 2 AND pn.IOIN[1] = 3 AND
               pn.IOIN[2] = 4 AND pn.IOIN[3] = 5;
        IF ok7 THEN mask := mask + 64; END_IF;
        // OP 8: IOCR cyclic exchange (frame_id via XID=0 -> default
        // 0x8000); IOOUT/IOIN carry the 'PN' record
        pn.OP := 8;
        pn.PORT := 0;
        pn.XID := 0;
        pn.IOOUT[0] := 10;
        pn.IOOUT[1] := 20;
        pn.IOOUT[2] := 30;
        pn.DLEN := 3;
        st := 8;
    ELSIF st = 8 THEN
        pn();
        // device replies bytes +1, cycle counter = 1, status 0x35
        ok8 := pn.STATUS = 0 AND pn.DLEN = 3 AND
               pn.IOIN[0] = 11 AND pn.IOIN[1] = 21 AND
               pn.IOIN[2] = 31 AND pn.PNCYC = 1 AND
               pn.PNSTAT = 16#35;
        IF ok8 THEN mask := mask + 128; END_IF;
        // second IOCR exchange: device cycle counter must advance
        pn.OP := 8;
        pn.PORT := 0;
        pn.XID := 0;
        pn.IOOUT[0] := 7;
        pn.DLEN := 1;
        st := 9;
    ELSIF st = 9 THEN
        pn();
        ok9 := pn.STATUS = 0 AND pn.DLEN = 1 AND
               pn.IOIN[0] = 8 AND pn.PNCYC = 2 AND
               pn.PNSTAT = 16#35;
        IF ok9 THEN mask := mask + 256; END_IF;
        // OP 13: AR state query (idle -> 0)
        pn.OP := 13;
        pn.PORT := 0;
        st := 10;
    ELSIF st = 10 THEN
        pn();
        ok10 := pn.STATUS = 0 AND pn.ARSTATE = 0 AND pn.ARKEY = 0;
        IF ok10 THEN mask := mask + 512; END_IF;
        // OP 9: PNIO-CM AR Connect — ARBlockReq block lifted from a
        // real CPU->device capture; UUIDs are that session's.
        pn.OP := 9;
        pn.PORT := 0;
        pn.DLEN := 95;
        pn.DEVOBJ[0] := 16#DE; pn.DEVOBJ[1] := 16#A0; pn.DEVOBJ[2] := 16#00;
        pn.DEVOBJ[3] := 16#00; pn.DEVOBJ[4] := 16#6C; pn.DEVOBJ[5] := 16#97;
        pn.DEVOBJ[6] := 16#11; pn.DEVOBJ[7] := 16#D1; pn.DEVOBJ[8] := 16#82;
        pn.DEVOBJ[9] := 16#71; pn.DEVOBJ[10] := 16#00; pn.DEVOBJ[11] := 16#01;
        pn.DEVOBJ[12] := 16#00; pn.DEVOBJ[13] := 16#02; pn.DEVOBJ[14] := 16#04;
        pn.DEVOBJ[15] := 16#93;
        pn.ACTUUID[0] := 16#AE; pn.ACTUUID[1] := 16#A1; pn.ACTUUID[2] := 16#AC;
        pn.ACTUUID[3] := 16#D2; pn.ACTUUID[4] := 16#32; pn.ACTUUID[5] := 16#00;
        pn.ACTUUID[6] := 16#10; pn.ACTUUID[7] := 16#10; pn.ACTUUID[8] := 16#B0;
        pn.ACTUUID[9] := 16#58; pn.ACTUUID[10] := 16#EC; pn.ACTUUID[11] := 16#1C;
        pn.ACTUUID[12] := 16#5D; pn.ACTUUID[13] := 16#61; pn.ACTUUID[14] := 16#E7;
        pn.ACTUUID[15] := 16#3F;
        pn.ARUUID[0] := 16#E5; pn.ARUUID[1] := 16#E1; pn.ARUUID[2] := 16#AE;
        pn.ARUUID[3] := 16#CC; pn.ARUUID[4] := 16#B1; pn.ARUUID[5] := 16#33;
        pn.ARUUID[6] := 16#4B; pn.ARUUID[7] := 16#4D; pn.ARUUID[8] := 16#B1;
        pn.ARUUID[9] := 16#87; pn.ARUUID[10] := 16#CC; pn.ARUUID[11] := 16#68;
        pn.ARUUID[12] := 16#B0; pn.ARUUID[13] := 16#21; pn.ARUUID[14] := 16#1E;
        pn.ARUUID[15] := 16#D2;
        pn.ARBLK[0] := 16#01;
        pn.ARBLK[1] := 16#01;
        pn.ARBLK[2] := 16#00;
        pn.ARBLK[3] := 16#5B;
        pn.ARBLK[4] := 16#01;
        pn.ARBLK[5] := 16#00;
        pn.ARBLK[6] := 16#00;
        pn.ARBLK[7] := 16#01;
        pn.ARBLK[8] := 16#E5;
        pn.ARBLK[9] := 16#E1;
        pn.ARBLK[10] := 16#AE;
        pn.ARBLK[11] := 16#CC;
        pn.ARBLK[12] := 16#B1;
        pn.ARBLK[13] := 16#33;
        pn.ARBLK[14] := 16#4B;
        pn.ARBLK[15] := 16#4D;
        pn.ARBLK[16] := 16#B1;
        pn.ARBLK[17] := 16#87;
        pn.ARBLK[18] := 16#CC;
        pn.ARBLK[19] := 16#68;
        pn.ARBLK[20] := 16#B0;
        pn.ARBLK[21] := 16#21;
        pn.ARBLK[22] := 16#1E;
        pn.ARBLK[23] := 16#D2;
        pn.ARBLK[24] := 16#00;
        pn.ARBLK[25] := 16#02;
        pn.ARBLK[26] := 16#EC;
        pn.ARBLK[27] := 16#1C;
        pn.ARBLK[28] := 16#5D;
        pn.ARBLK[29] := 16#61;
        pn.ARBLK[30] := 16#E7;
        pn.ARBLK[31] := 16#3F;
        pn.ARBLK[32] := 16#DE;
        pn.ARBLK[33] := 16#A0;
        pn.ARBLK[34] := 16#00;
        pn.ARBLK[35] := 16#00;
        pn.ARBLK[36] := 16#6C;
        pn.ARBLK[37] := 16#97;
        pn.ARBLK[38] := 16#11;
        pn.ARBLK[39] := 16#D1;
        pn.ARBLK[40] := 16#82;
        pn.ARBLK[41] := 16#71;
        pn.ARBLK[42] := 16#10;
        pn.ARBLK[43] := 16#64;
        pn.ARBLK[44] := 16#01;
        pn.ARBLK[45] := 16#0E;
        pn.ARBLK[46] := 16#00;
        pn.ARBLK[47] := 16#2A;
        pn.ARBLK[48] := 16#40;
        pn.ARBLK[49] := 16#00;
        pn.ARBLK[50] := 16#00;
        pn.ARBLK[51] := 16#11;
        pn.ARBLK[52] := 16#00;
        pn.ARBLK[53] := 16#C8;
        pn.ARBLK[54] := 16#88;
        pn.ARBLK[55] := 16#92;
        pn.ARBLK[56] := 16#00;
        pn.ARBLK[57] := 16#25;
        pn.ARBLK[58] := 16#70;
        pn.ARBLK[59] := 16#6C;
        pn.ARBLK[60] := 16#63;
        pn.ARBLK[61] := 16#78;
        pn.ARBLK[62] := 16#62;
        pn.ARBLK[63] := 16#62;
        pn.ARBLK[64] := 16#65;
        pn.ARBLK[65] := 16#6E;
        pn.ARBLK[66] := 16#63;
        pn.ARBLK[67] := 16#68;
        pn.ARBLK[68] := 16#2E;
        pn.ARBLK[69] := 16#70;
        pn.ARBLK[70] := 16#72;
        pn.ARBLK[71] := 16#6F;
        pn.ARBLK[72] := 16#66;
        pn.ARBLK[73] := 16#69;
        pn.ARBLK[74] := 16#6E;
        pn.ARBLK[75] := 16#65;
        pn.ARBLK[76] := 16#74;
        pn.ARBLK[77] := 16#78;
        pn.ARBLK[78] := 16#61;
        pn.ARBLK[79] := 16#69;
        pn.ARBLK[80] := 16#6E;
        pn.ARBLK[81] := 16#74;
        pn.ARBLK[82] := 16#65;
        pn.ARBLK[83] := 16#72;
        pn.ARBLK[84] := 16#66;
        pn.ARBLK[85] := 16#61;
        pn.ARBLK[86] := 16#63;
        pn.ARBLK[87] := 16#65;
        pn.ARBLK[88] := 16#78;
        pn.ARBLK[89] := 16#62;
        pn.ARBLK[90] := 16#32;
        pn.ARBLK[91] := 16#35;
        pn.ARBLK[92] := 16#66;
        pn.ARBLK[93] := 16#62;
        pn.ARBLK[94] := 16#64;
        st := 11;
    ELSIF st = 11 THEN
        pn();
        ok11 := pn.STATUS = 0 AND pn.ARPSTAT = 0 AND pn.DLEN > 0;
        IF ok11 THEN mask := mask + 1024; END_IF;
        pn.OP := 13;
        pn.PORT := 0;
        st := 12;
    ELSIF st = 12 THEN
        pn();
        ok12 := pn.STATUS = 0 AND pn.ARSTATE = 1;
        IF ok12 THEN mask := mask + 2048; END_IF;
        // OP 11: PNIO-CM Control — golden PrmEnd block
        pn.OP := 11;
        pn.PORT := 0;
        pn.DLEN := 32;
        pn.ARBLK[0] := 16#01;
        pn.ARBLK[1] := 16#10;
        pn.ARBLK[2] := 16#00;
        pn.ARBLK[3] := 16#1C;
        pn.ARBLK[4] := 16#01;
        pn.ARBLK[5] := 16#00;
        pn.ARBLK[6] := 16#00;
        pn.ARBLK[7] := 16#00;
        pn.ARBLK[8] := 16#E5;
        pn.ARBLK[9] := 16#E1;
        pn.ARBLK[10] := 16#AE;
        pn.ARBLK[11] := 16#CC;
        pn.ARBLK[12] := 16#B1;
        pn.ARBLK[13] := 16#33;
        pn.ARBLK[14] := 16#4B;
        pn.ARBLK[15] := 16#4D;
        pn.ARBLK[16] := 16#B1;
        pn.ARBLK[17] := 16#87;
        pn.ARBLK[18] := 16#CC;
        pn.ARBLK[19] := 16#68;
        pn.ARBLK[20] := 16#B0;
        pn.ARBLK[21] := 16#21;
        pn.ARBLK[22] := 16#1E;
        pn.ARBLK[23] := 16#D2;
        pn.ARBLK[24] := 16#00;
        pn.ARBLK[25] := 16#02;
        pn.ARBLK[26] := 16#00;
        pn.ARBLK[27] := 16#00;
        pn.ARBLK[28] := 16#00;
        pn.ARBLK[29] := 16#01;
        pn.ARBLK[30] := 16#00;
        pn.ARBLK[31] := 16#00;
        st := 13;
    ELSIF st = 13 THEN
        pn();
        ok13 := pn.STATUS = 0 AND pn.ARPSTAT = 0;
        IF ok13 THEN mask := mask + 4096; END_IF;
        // OP 12: CM service — consumes device ApplicationReady
        pn.OP := 12;
        pn.PORT := 0;
        pn.XID := 200;
        st := 14;
    ELSIF st = 14 THEN
        pn();
        // ARSTATE=3 (data exchange) is only reached when service()
        // consumed the device ApplicationReady; STATUS stays 0.
        ok14 := pn.STATUS = 0 AND pn.ARSTATE = 3;
        IF ok14 THEN mask := mask + 8192; END_IF;
        // OP 14: PNIO-CM record Read — golden IODReadReq for I&M0
        // (ar/target UUIDs patched to this session's)
        pn.OP := 14;
        pn.PORT := 0;
        pn.DLEN := 64;
        pn.ARBLK[0] := 16#00;
        pn.ARBLK[1] := 16#09;
        pn.ARBLK[2] := 16#00;
        pn.ARBLK[3] := 16#3C;
        pn.ARBLK[4] := 16#01;
        pn.ARBLK[5] := 16#00;
        pn.ARBLK[6] := 16#00;
        pn.ARBLK[7] := 16#06;
        pn.ARBLK[8] := 16#E5;
        pn.ARBLK[9] := 16#E1;
        pn.ARBLK[10] := 16#AE;
        pn.ARBLK[11] := 16#CC;
        pn.ARBLK[12] := 16#B1;
        pn.ARBLK[13] := 16#33;
        pn.ARBLK[14] := 16#4B;
        pn.ARBLK[15] := 16#4D;
        pn.ARBLK[16] := 16#B1;
        pn.ARBLK[17] := 16#87;
        pn.ARBLK[18] := 16#CC;
        pn.ARBLK[19] := 16#68;
        pn.ARBLK[20] := 16#B0;
        pn.ARBLK[21] := 16#21;
        pn.ARBLK[22] := 16#1E;
        pn.ARBLK[23] := 16#D2;
        pn.ARBLK[24] := 16#00;
        pn.ARBLK[25] := 16#00;
        pn.ARBLK[26] := 16#00;
        pn.ARBLK[27] := 16#00;
        pn.ARBLK[28] := 16#00;
        pn.ARBLK[29] := 16#00;
        pn.ARBLK[30] := 16#00;
        pn.ARBLK[31] := 16#01;
        pn.ARBLK[32] := 16#00;
        pn.ARBLK[33] := 16#00;
        pn.ARBLK[34] := 16#AF;
        pn.ARBLK[35] := 16#F0;
        pn.ARBLK[36] := 16#00;
        pn.ARBLK[37] := 16#00;
        pn.ARBLK[38] := 16#80;
        pn.ARBLK[39] := 16#00;
        pn.ARBLK[40] := 16#E5;
        pn.ARBLK[41] := 16#E1;
        pn.ARBLK[42] := 16#AE;
        pn.ARBLK[43] := 16#CC;
        pn.ARBLK[44] := 16#B1;
        pn.ARBLK[45] := 16#33;
        pn.ARBLK[46] := 16#4B;
        pn.ARBLK[47] := 16#4D;
        pn.ARBLK[48] := 16#B1;
        pn.ARBLK[49] := 16#87;
        pn.ARBLK[50] := 16#CC;
        pn.ARBLK[51] := 16#68;
        pn.ARBLK[52] := 16#B0;
        pn.ARBLK[53] := 16#21;
        pn.ARBLK[54] := 16#1E;
        pn.ARBLK[55] := 16#D2;
        pn.ARBLK[56] := 16#00;
        pn.ARBLK[57] := 16#00;
        pn.ARBLK[58] := 16#00;
        pn.ARBLK[59] := 16#00;
        pn.ARBLK[60] := 16#00;
        pn.ARBLK[61] := 16#00;
        pn.ARBLK[62] := 16#00;
        pn.ARBLK[63] := 16#00;
        st := 15;
    ELSIF st = 15 THEN
        pn();
        // res: IODReadResHeader 0x8009 then I&M0 record 0x0020;
        // vendor 0x0493 at record body offset 70
        ok15 := pn.STATUS = 0 AND pn.ARPSTAT = 0 AND
                pn.ARBLK[0] = 16#80 AND pn.ARBLK[1] = 16#09 AND
                pn.ARBLK[64] = 16#00 AND pn.ARBLK[65] = 16#20 AND
                pn.ARBLK[70] = 16#04 AND pn.ARBLK[71] = 16#93;
        IF ok15 THEN mask := mask + 16384; END_IF;
        pn.OP := 2;
        pn.PORT := 0;
        st := 16;
    ELSIF st = 16 THEN
        pn();
        ok16 := pn.STATUS = 0;
        IF ok16 THEN mask := mask + 32768; END_IF;
        // invalid op -> rejection
        pn.OP := 99;
        pn.PORT := 0;
        st := 17;
    ELSIF st = 17 THEN
        pn();
        ok17 := pn.STATUS = -1 AND pn.ERRID = 1;
        IF ok17 THEN mask := mask + 65536; END_IF;
        pn.OP := 0;
        st := 18;
    END_IF
END_PROGRAM
