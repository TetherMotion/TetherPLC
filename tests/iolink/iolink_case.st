// iolink_case.st — tplc-iolink e2e against fake_iolink_device.py
// (TPLC_IOLINK_IO=tcp://127.0.0.1:PORT): exercises FB_IoLink open,
// service state, DPP1 identification via the page channel, cyclic
// type-2_V process-data exchange, legacy single-octet ISDU
// write/read, MasterCommand (DeviceOperate), spec-segmented ISDU
// write/read over index 0x0100 (20-octet payload -> ExtLength
// segmentation), a negative ISDU response, port close, and
// invalid-op rejection.  One FB call per scan (each call is a
// synchronous wire round-trip).
PROGRAM iolink_case
VAR
    il : FB_IoLink;
    st : INT;
    k : DINT;
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
END_VAR
    IF st = 0 THEN
        mask := 0;
        il.OP := 0;
        il.PORT := 0;
        il.MSEQ := 0;
        il.ADDR := 0;
        il.DLEN := 0;
        st := 1;
    ELSIF st = 1 THEN
        // OP 1: open port 0 (connect + wakeup burst)
        il.OP := 1;
        il.PORT := 0;
        il();
        ok1 := il.STATUS = 0;
        IF ok1 THEN mask := mask + 1; END_IF;
        // OP 3: service state — ports configured
        il.OP := 3;
        il.PORT := 0;
        st := 2;
    ELSIF st = 2 THEN
        // executes OP 3 armed in st=1
        il();
        ok2 := il.STATUS = 1;
        IF ok2 THEN mask := mask + 2; END_IF;
        // OP 4: ident — 16 page reads -> DPP1 record
        il.OP := 4;
        il.PORT := 0;
        st := 3;
    ELSIF st = 3 THEN
        il();
        ok3 := il.STATUS = 0 AND il.DLEN = 16 AND
               il.IDENT[5] = 16#84 AND il.IDENT[6] = 16#84 AND
               il.IDENT[7] = 16#34 AND il.IDENT[8] = 16#12 AND
               il.IDENT[9] = 16#78 AND il.IDENT[10] = 16#56;
        IF ok3 THEN mask := mask + 4; END_IF;
        // arm cyclic PD exchange: 4 octets out, M-seq type 2_V (6)
        il.OP := 5;
        il.PORT := 0;
        il.MSEQ := 6;
        il.PDOUT[0] := 16#10;
        il.PDOUT[1] := 16#20;
        il.PDOUT[2] := 16#30;
        il.PDOUT[3] := 16#40;
        il.DLEN := 4;
        st := 4;
    ELSIF st = 4 THEN
        il();
        // device PD-in: [0x42, pdo0+1, 0x99, exchange-count]
        ok4 := il.STATUS = 0 AND il.DLEN = 4 AND
               il.PDIN[0] = 16#42 AND il.PDIN[1] = 16#11 AND
               il.PDIN[2] = 16#99 AND il.CKSFLAGS = 0;
        IF ok4 THEN mask := mask + 8; END_IF;
        // arm ISDU write: addr 5 <- 0x77
        il.OP := 7;
        il.PORT := 0;
        il.ADDR := 5;
        il.OD[0] := 16#77;
        il.DLEN := 1;
        st := 5;
    ELSIF st = 5 THEN
        il();
        ok5 := il.STATUS = 0;
        IF ok5 THEN mask := mask + 16; END_IF;
        // arm ISDU read addr 5
        il.OP := 6;
        il.PORT := 0;
        il.ADDR := 5;
        il.DLEN := 0;
        st := 6;
    ELSIF st = 6 THEN
        il();
        ok6 := il.STATUS = 0 AND il.DLEN = 1 AND il.OD[0] = 16#77;
        IF ok6 THEN mask := mask + 32; END_IF;
        // arm MasterCommand DeviceOperate (0x99)
        il.OP := 8;
        il.PORT := 0;
        il.ADDR := 16#99;
        st := 7;
    ELSIF st = 7 THEN
        il();
        ok7 := il.STATUS = 0;
        IF ok7 THEN mask := mask + 64; END_IF;
        // arm segmented ISDU write: index 0x0100 (16-bit form),
        // subindex 0, 20-octet payload -> ExtLength segmentation
        il.OP := 10;
        il.PORT := 0;
        il.MSEQ := 16#0100;
        il.ADDR := 0;
        FOR k := 0 TO 19 DO
            il.ISDU[k] := k + 1;
        END_FOR;
        il.DLEN := 20;
        st := 8;
    ELSIF st = 8 THEN
        il();
        ok8 := il.STATUS = 0 AND il.ISDUERR = 0;
        IF ok8 THEN mask := mask + 128; END_IF;
        // arm segmented ISDU read of the same index
        il.OP := 9;
        il.PORT := 0;
        il.MSEQ := 16#0100;
        il.ADDR := 0;
        il.DLEN := 0;
        st := 9;
    ELSIF st = 9 THEN
        il();
        ok9 := il.STATUS = 0 AND il.DLEN = 20 AND
               il.ISDU[0] = 1 AND il.ISDU[19] = 20;
        IF ok9 THEN mask := mask + 256; END_IF;
        // arm ISDU read of an unknown index -> negative response
        il.OP := 9;
        il.PORT := 0;
        il.MSEQ := 16#0200;
        il.ADDR := 0;
        il.DLEN := 0;
        st := 10;
    ELSIF st = 10 THEN
        il();
        // ErrorType 0x8011 = ErrorCode 0x80 (APP_DEV) |
        // AdditionalCode 0x11 (index not available)
        ok10 := il.STATUS = -7 AND il.ISDUERR = 16#8011;
        IF ok10 THEN mask := mask + 512; END_IF;
        // close the port
        il.OP := 2;
        il.PORT := 0;
        st := 11;
    ELSIF st = 11 THEN
        il();
        ok11 := il.STATUS = 0;
        IF ok11 THEN mask := mask + 1024; END_IF;
        // invalid op -> rejection
        il.OP := 99;
        il.PORT := 0;
        st := 12;
    ELSIF st = 12 THEN
        il();
        ok12 := il.STATUS = -1 AND il.ERRID = 1;
        IF ok12 THEN mask := mask + 2048; END_IF;
        il.OP := 0;
        st := 13;
    END_IF
END_PROGRAM
