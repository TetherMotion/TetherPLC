// tc2_dmx_case.st — DMX512 output e2e over a PTY fake.  Open,
// set channels, send a frame, status, close.
PROGRAM tc2_dmx_case
VAR
    opn : FB_DmxOpen;
    cls : FB_DmxClose;
    set : FB_DmxSetChannel;
    snd : FB_DmxSend;
    sta : FB_DmxStatus;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        opn.Execute := TRUE;
        opn.Path := '/tmp/dmx_pty';
        opn.Baud := 250000;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF opn.Done THEN
            hdl := opn.PortHdl;
            opn.Execute := FALSE;
            mask := mask + 1;         // bit0: serial open
            set.Execute := TRUE;
            set.PortHdl := hdl;
            set.Channel := 1;
            set.Value := 200;
            st := 2;
        END_IF;
        IF opn.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF set.Done THEN
            mask := mask + 2;         // bit1: channel written
            set.Execute := FALSE;
            snd.Execute := TRUE;
            snd.PortHdl := hdl;
            st := 3;
        END_IF;
        IF set.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 3 THEN
        IF snd.Done THEN
            mask := mask + 4;         // bit2: frame emitted
            snd.Execute := FALSE;
            sta.Execute := TRUE;
            sta.PortHdl := hdl;
            st := 4;
        END_IF;
        IF snd.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 4 THEN
        IF sta.Done THEN
            IF sta.Frames > 0 THEN
                mask := mask + 8;     // bit3: frame counted
            END_IF;
            sta.Execute := FALSE;
            cls.Execute := TRUE;
            cls.PortHdl := hdl;
            st := 5;
        END_IF;
        IF sta.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 5 THEN
        IF cls.Done THEN
            mask := mask + 16;        // bit4: closed
            st := 6;
        END_IF;
        IF cls.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    opn(); cls(); set(); snd(); sta();
END_PROGRAM
