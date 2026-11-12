// tc2_enocean_case.st — wired EnOcean ESP3 e2e over a PTY
// transceiver fake.  Open port, send a RADIO telegram (fake
// ACKs with RESPONSE + emits an inbound PTM radio), poll the
// receive, decode the PTM action, close.
PROGRAM tc2_enocean_case
VAR
    opn : FB_EnOceanOpen;
    cls : FB_EnOceanClose;
    snd : FB_EnOceanSendRadio;
    rx  : FB_EnOceanReceive;
    ptm : FB_EnOceanPtm100;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    m2 : UDINT;
    diag AT %QD4 : UDINT;
    hdl : UDINT;
END_VAR
    diag := st;
    mask := m2;
    IF st = 0 THEN
        m2 := 0;
        opn.Execute := TRUE;
        opn.Path := '/tmp/eno_pty';
        st := 1;

    ELSIF st = 1 THEN
        IF opn.Done THEN
            hdl := opn.PortHdl;
            opn.Execute := FALSE;
            m2 := m2 OR 1;        // bit0: serial open
            snd.Execute := TRUE;
            snd.PortHdl := hdl;
            snd.Org := 16#F6;         // RPS
            snd.SenderId := 16#AABBCCDD;
            snd.DestId := 16#FFFFFFFF;
            snd.Data := 'x';
            st := 2;
        END_IF;
        IF opn.Error THEN diag := 1; m2 := 16#DE00 + 1; st := 99; END_IF;

    ELSIF st = 2 THEN
        IF snd.Done THEN
            m2 := m2 OR 2;        // bit1: radio frame sent
            snd.Execute := FALSE;
            rx.Execute := TRUE;
            rx.PortHdl := hdl;
            st := 3;
        END_IF;
        IF snd.Error THEN diag := 2; m2 := 16#DE00 + 2; st := 99; END_IF;

    // poll for the inbound radio telegram (fake emits after it
    // sees our transmit; RESPONSE packet may arrive first and
    // must be skipped, PTYPE=2)
    ELSIF st = 3 THEN
        waitN := waitN + 1;
        IF rx.Telegram THEN
            IF rx.PacketType = 1 AND rx.Org = 16#F6 AND rx.SenderId = 16#00112233 THEN
                m2 := m2 OR 4;    // bit2: radio decoded
                ptm.Execute := TRUE;
                ptm.Org := rx.Org;
                ptm.B0 := rx.Data0;
                st := 4;
            END_IF;
        END_IF;
        IF rx.Error THEN diag := 3; m2 := 16#DE00 + 3; st := 99; END_IF;
        IF waitN > 80 THEN diag := 4; m2 := 16#DE00 + 4; st := 99; END_IF;

    ELSIF st = 4 THEN
        rx.Execute := FALSE;
        IF ptm.Done AND ptm.IsPtm AND ptm.Pressed AND ptm.Act = 2 THEN
            m2 := m2 OR 8;        // bit3: PTM decode
            cls.Execute := TRUE;
            cls.PortHdl := hdl;
            st := 5;
        END_IF;

    ELSIF st = 5 THEN
        IF cls.Done THEN
            m2 := m2 OR 16;       // bit4: close ok
            st := 6;
        END_IF;
        IF cls.Error THEN diag := 5; m2 := 16#DE00 + 5; st := 99; END_IF;
    END_IF;

    opn(); cls(); snd(); rx(); ptm();
END_PROGRAM
