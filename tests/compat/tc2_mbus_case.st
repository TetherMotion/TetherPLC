// tc2_mbus_case.st — wired M-Bus master e2e over a PTY slave fake.
// Open, SND_NKE -> E5, REQ_UD2 -> data frame, close.
PROGRAM tc2_mbus_case
VAR
    opn : FB_MbusOpen;
    cls : FB_MbusClose;
    nke : FB_MbusShort;
    req : FB_MbusShort;
    rx  : FB_MbusReceive;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
    hdl : UDINT;
END_VAR
    diag := 0;
    IF st = 0 THEN
        mask := 0;
        opn.Execute := TRUE;
        opn.Path := '/tmp/mbus_pty';
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF opn.Done THEN
            hdl := opn.PortHdl;
            opn.Execute := FALSE;
            mask := mask + 1;         // bit0: serial open
            nke.Execute := TRUE;
            nke.PortHdl := hdl;
            nke.Control := 16#40;     // SND_NKE
            nke.Address := 1;
            st := 2;
        END_IF;
        IF opn.Error THEN diag := 1; mask := 16#DE00 + 1; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF nke.Done THEN
            mask := mask + 2;         // bit1: NKE sent
            nke.Execute := FALSE;
            rx.Execute := TRUE;
            rx.PortHdl := hdl;
            st := 3;
        END_IF;
        IF nke.Error THEN diag := 2; mask := 16#DE00 + 2; st := 99; END_IF;
    END_IF;

    // poll for the slave's E5 ack
    IF st = 3 THEN
        waitN := waitN + 1;
        IF rx.Message THEN
            IF rx.Kind = 16#E5 THEN
                mask := mask + 4;     // bit2: E5 ack
            END_IF;
            rx.Execute := FALSE;
            req.Execute := TRUE;
            req.PortHdl := hdl;
            req.Control := 16#5B;     // REQ_UD2
            req.Address := 1;
            st := 4;
        END_IF;
        IF rx.Error THEN diag := 3; mask := 16#DE00 + 3; st := 99; END_IF;
        IF waitN > 40 THEN
            rx.Execute := FALSE;
            req.Execute := TRUE;
            req.PortHdl := hdl;
            req.Control := 16#5B;
            req.Address := 1;
            st := 4;
        END_IF;
    END_IF;

    IF st = 4 THEN
        IF req.Done THEN
            mask := mask + 8;         // bit3: REQ_UD2 sent
            req.Execute := FALSE;
            rx.Execute := TRUE;
            rx.PortHdl := hdl;
            waitN := 0;
            st := 5;
        END_IF;
        IF req.Error THEN diag := 4; mask := 16#DE00 + 4; st := 99; END_IF;
    END_IF;

    // poll for the slave's 68.. data frame
    IF st = 5 THEN
        waitN := waitN + 1;
        IF rx.Message THEN
            IF rx.Kind = 16#68 THEN
                mask := mask + 16;    // bit4: data frame
            END_IF;
            rx.Execute := FALSE;
            cls.Execute := TRUE;
            cls.PortHdl := hdl;
            st := 6;
        END_IF;
        IF rx.Error THEN diag := 5; mask := 16#DE00 + 5; st := 99; END_IF;
        IF waitN > 40 THEN
            rx.Execute := FALSE;
            cls.Execute := TRUE;
            cls.PortHdl := hdl;
            st := 6;
        END_IF;
    END_IF;

    IF st = 6 THEN
        IF cls.Done THEN
            mask := mask + 32;        // bit5: closed
            st := 7;
        END_IF;
        IF cls.Error THEN diag := 6; mask := 16#DE00 + 6; st := 99; END_IF;
    END_IF;

    opn(); cls(); nke(); req(); rx();
END_PROGRAM
