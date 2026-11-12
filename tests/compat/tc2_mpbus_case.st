// tc2_mpbus_case.st — wired MP-Bus e2e over a PTY slave fake
// (node 0x05).  Open, damper setpoint write -> ack, actual-pos
// read -> 0x80 (~50%), CMV read -> 0x42, close.
PROGRAM tc2_mpbus_case
VAR
    opn : FB_MpbusOpen;
    cls : FB_MpbusClose;
    dmp : MP_DamperLinearActuator;
    cmv : MP_CMV;
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
        opn.Path := '/tmp/mpb_pty';
        st := 1;
    ELSIF st = 1 THEN
        IF opn.Done THEN
            hdl := opn.PortHdl;
            opn.Execute := FALSE;
            mask := mask OR 1;        // bit0: serial open
            dmp.Execute := TRUE;
            dmp.PortHdl := hdl;
            dmp.DestAddr := 16#05;
            dmp.WriteSp := TRUE;
            dmp.Setpoint := 50.0;
            st := 2;
        END_IF;
        IF opn.Error THEN diag := 16#D0; st := 99; END_IF;
    ELSIF st = 2 THEN
        waitN := waitN + 1;
        IF dmp.Done THEN
            mask := mask OR 2;        // bit1: SP write ack
            st := 3;
        END_IF;
        IF dmp.Error THEN diag := 16#D1; st := 99; END_IF;
        IF waitN > 80 THEN diag := 16#D2; st := 99; END_IF;
    ELSIF st = 3 THEN
        // switch to actual-position read
        dmp.Execute := FALSE;
        waitN := 0;
        st := 4;
    ELSIF st = 4 THEN
        dmp.WriteSp := FALSE;
        dmp.Execute := TRUE;
        waitN := waitN + 1;
        IF dmp.Done THEN
            IF dmp.ActualPos > 45.0 AND dmp.ActualPos < 55.0 THEN
                mask := mask OR 4;    // bit2: actual pos ~50%
            END_IF;
            st := 5;
        END_IF;
        IF dmp.Error THEN diag := 16#D3; st := 99; END_IF;
        IF waitN > 80 THEN diag := 16#D4; st := 99; END_IF;
    ELSIF st = 5 THEN
        dmp.Execute := FALSE;
        cmv.Execute := TRUE;
        cmv.PortHdl := hdl;
        cmv.DestAddr := 16#05;
        cmv.Channel := 1;
        waitN := 0;
        st := 6;
    ELSIF st = 6 THEN
        waitN := waitN + 1;
        IF cmv.Done THEN
            IF cmv.Value = 16#42 THEN
                mask := mask OR 8;    // bit3: CMV value
            END_IF;
            cls.Execute := TRUE;
            cls.PortHdl := hdl;
            st := 7;
        END_IF;
        IF cmv.Error THEN diag := 16#D5; st := 99; END_IF;
        IF waitN > 80 THEN diag := 16#D6; st := 99; END_IF;
    ELSIF st = 7 THEN
        cmv.Execute := FALSE;
        IF cls.Done THEN
            mask := mask OR 16;       // bit4: close ok
            st := 8;
        END_IF;
        IF cls.Error THEN diag := 16#D7; st := 99; END_IF;
    END_IF;

    opn(); cls(); dmp(); cmv();
END_PROGRAM
