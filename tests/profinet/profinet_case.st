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
        pn.OP := 2;
        pn.PORT := 0;
        st := 8;
    ELSIF st = 8 THEN
        pn();
        ok8 := pn.STATUS = 0;
        IF ok8 THEN mask := mask + 128; END_IF;
        // invalid op -> rejection
        pn.OP := 99;
        pn.PORT := 0;
        st := 9;
    ELSIF st = 9 THEN
        pn();
        ok9 := pn.STATUS = -1 AND pn.ERRID = 1;
        IF ok9 THEN mask := mask + 256; END_IF;
        pn.OP := 0;
        st := 10;
    END_IF
END_PROGRAM
