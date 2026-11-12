// tc2_genibus_case.st — wired GENIbus e2e over a PTY slave
// fake (unit 0x20).  Open, ping -> ack, GET -> APDU reply,
// close.
PROGRAM tc2_genibus_case
VAR
    opn : FB_GenibusOpen;
    cls : FB_GenibusClose;
    png : FB_GENIbusPing;
    get : FB_GENIbusGet;
    rx  : FB_GENIbusReceive;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
    hdl : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        opn.Execute := TRUE;
        opn.Path := '/tmp/gen_pty';
        st := 1;
    ELSIF st = 1 THEN
        IF opn.Done THEN
            hdl := opn.PortHdl;
            opn.Execute := FALSE;
            mask := mask OR 1;        // bit0: serial open
            png.Execute := TRUE;
            png.PortHdl := hdl;
            png.DestAddr := 16#20;
            png.SrcAddr := 0;
            st := 2;
        END_IF;
        IF opn.Error THEN diag := 16#D0; st := 99; END_IF;
    ELSIF st = 2 THEN
        IF png.Done THEN
            mask := mask OR 2;        // bit1: ping sent
            png.Execute := FALSE;
            rx.Execute := TRUE;
            rx.PortHdl := hdl;
            st := 3;
        END_IF;
        IF png.Error THEN diag := 16#D1; st := 99; END_IF;
    ELSIF st = 3 THEN
        waitN := waitN + 1;
        IF rx.Telegram THEN
            IF rx.SrcAddr = 16#20 THEN
                mask := mask OR 4;    // bit2: slave ack
                rx.Execute := FALSE;
                get.Execute := TRUE;
                get.PortHdl := hdl;
                get.DestAddr := 16#20;
                get.Item := 16#42;
                st := 4;
            END_IF;
        END_IF;
        IF rx.Error THEN diag := 16#D2; st := 99; END_IF;
        IF waitN > 80 THEN diag := 16#D3; st := 99; END_IF;
    ELSIF st = 4 THEN
        waitN := 0;
        IF get.Done THEN
            mask := mask OR 8;        // bit3: GET sent
            get.Execute := FALSE;
            rx.Execute := TRUE;
            st := 5;
        END_IF;
        IF get.Error THEN diag := 16#D4; st := 99; END_IF;
    ELSIF st = 5 THEN
        waitN := waitN + 1;
        IF rx.Telegram THEN
            IF rx.SrcAddr = 16#20 AND rx.ApduLen > 0 THEN
                mask := mask OR 16;   // bit4: APDU reply
                rx.Execute := FALSE;
                cls.Execute := TRUE;
                cls.PortHdl := hdl;
                st := 6;
            END_IF;
        END_IF;
        IF rx.Error THEN diag := 16#D5; st := 99; END_IF;
        IF waitN > 80 THEN diag := 16#D6; st := 99; END_IF;
    ELSIF st = 6 THEN
        IF cls.Done THEN
            mask := mask OR 32;       // bit5: close ok
            st := 7;
        END_IF;
        IF cls.Error THEN diag := 16#D7; st := 99; END_IF;
    END_IF;

    opn(); cls(); png(); get(); rx();
END_PROGRAM
