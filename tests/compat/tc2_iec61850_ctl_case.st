// tc2_iec61850_ctl_case.st — IEC 61850 controllable-object e2e over
// the fake IED's SWI1 BSC (SBO normal + cancel): bare operate is
// rejected, select -> operate(true) -> stVal verifies, select ->
// cancel -> stVal unchanged, disconnect.
PROGRAM tc2_iec61850_ctl_case
VAR
    con : FB_Iec61850Connect;
    dis : FB_Iec61850Disconnect;
    ctl : FB_Iec61850Control;
    cpol : FB_Iec61850ControlPoll;
    rd  : FB_Iec61850Read;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
    cref : STRING[127];
    sval : STRING[255];
END_VAR
    IF st = 0 THEN
        mask := 0;
        cref := 'tetherIEDtetherTestLD/LLN0.SWI1';
        con.Execute := TRUE;
        con.RemoteHost := '127.0.0.1';
        con.RemotePort := 11020;
        st := 1;
    ELSIF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;
            // bare operate without select must be rejected by the
            // SBO server (async CommandTermination- expected)
            ctl.Execute := TRUE;
            ctl.ConnectionHdl := hdl;
            ctl.ObjectRef := cref;
            ctl.Mode := 3;              // operate
            ctl.CtlValue := 'true';
            st := 20;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 20 THEN
        IF ctl.Done THEN
            ctl.Execute := FALSE;
            mask := mask + 2;           // command accepted for tx
            waitN := 0;
            st := 21;
        END_IF;
        IF ctl.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 21 THEN
        cpol.Execute := TRUE;
        cpol.ConnectionHdl := hdl;
        st := 22;
    ELSIF st = 22 THEN
        IF cpol.Done THEN
            cpol.Execute := FALSE;
            // expect a pending operate termination with success=0
            IF cpol.Pending AND NOT cpol.Success
               AND cpol.EvType = 1 THEN
                mask := mask + 4;
                ctl.Execute := TRUE;
                ctl.ConnectionHdl := hdl;
                ctl.ObjectRef := cref;
                ctl.Mode := 1;          // select
                ctl.CtlValue := '';
                st := 23;
            ELSIF waitN > 100 THEN
                mask := 16#DEAD; st := 99;
            ELSE
                waitN := waitN + 1;
                st := 21;
            END_IF;
        END_IF;
        IF cpol.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 23 THEN
        IF ctl.Done THEN
            ctl.Execute := FALSE;
            mask := mask + 8;
            waitN := 0;
            st := 24;
        END_IF;
        IF ctl.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 24 THEN
        cpol.Execute := TRUE;
        cpol.ConnectionHdl := hdl;
        st := 25;
    ELSIF st = 25 THEN
        IF cpol.Done THEN
            cpol.Execute := FALSE;
            // select termination: type 0, success
            IF cpol.Pending AND cpol.Success AND cpol.EvType = 0 THEN
                mask := mask + 16;
                ctl.Execute := TRUE;
                ctl.ConnectionHdl := hdl;
                ctl.ObjectRef := cref;
                ctl.Mode := 3;          // operate
                ctl.CtlValue := 'true';
                st := 26;
            ELSIF waitN > 100 THEN
                mask := 16#DEAD; st := 99;
            ELSE
                waitN := waitN + 1;
                st := 24;
            END_IF;
        END_IF;
        IF cpol.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 26 THEN
        IF ctl.Done THEN
            ctl.Execute := FALSE;
            mask := mask + 32;
            waitN := 0;
            st := 27;
        END_IF;
        IF ctl.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 27 THEN
        cpol.Execute := TRUE;
        cpol.ConnectionHdl := hdl;
        st := 28;
    ELSIF st = 28 THEN
        IF cpol.Done THEN
            cpol.Execute := FALSE;
            // operate termination: type 1, success
            IF cpol.Pending AND cpol.Success AND cpol.EvType = 1 THEN
                mask := mask + 64;
                rd.Execute := TRUE;
                rd.ConnectionHdl := hdl;
                rd.Domain := 'tetherIEDtetherTestLD';
                rd.Item := 'LLN0.SWI1.valWTr.posVal#ST';
                st := 29;
            ELSIF waitN > 100 THEN
                mask := 16#DEAD; st := 99;
            ELSE
                waitN := waitN + 1;
                st := 27;
            END_IF;
        END_IF;
        IF cpol.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 29 THEN
        IF rd.Done THEN
            rd.Execute := FALSE;
            sval := rd.Value;
            // boolean stVal reads back as "1"
            IF FIND(sval, '1') > 0 THEN
                mask := mask + 128;
                // select again, then cancel: stVal must stay true
                ctl.Execute := TRUE;
                ctl.ConnectionHdl := hdl;
                ctl.ObjectRef := cref;
                ctl.Mode := 1;          // select
                ctl.CtlValue := '';
                st := 30;
            ELSE
                mask := 16#DEAD; st := 99;
            END_IF;
        END_IF;
        IF rd.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 30 THEN
        IF ctl.Done THEN
            ctl.Execute := FALSE;
            mask := mask + 256;
            waitN := 0;
            st := 31;
        END_IF;
        IF ctl.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 31 THEN
        cpol.Execute := TRUE;
        cpol.ConnectionHdl := hdl;
        st := 32;
    ELSIF st = 32 THEN
        IF cpol.Done THEN
            cpol.Execute := FALSE;
            IF cpol.Pending AND cpol.Success AND cpol.EvType = 0 THEN
                ctl.Execute := TRUE;
                ctl.ConnectionHdl := hdl;
                ctl.ObjectRef := cref;
                ctl.Mode := 4;          // cancel
                ctl.CtlValue := '';
                st := 33;
            ELSIF waitN > 100 THEN
                mask := 16#DEAD; st := 99;
            ELSE
                waitN := waitN + 1;
                st := 31;
            END_IF;
        END_IF;
        IF cpol.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 33 THEN
        IF ctl.Done THEN
            ctl.Execute := FALSE;
            mask := mask + 512;
            waitN := 0;
            st := 34;
        END_IF;
        IF ctl.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 34 THEN
        cpol.Execute := TRUE;
        cpol.ConnectionHdl := hdl;
        st := 35;
    ELSIF st = 35 THEN
        IF cpol.Done THEN
            cpol.Execute := FALSE;
            // cancel termination: type 2, success
            IF cpol.Pending AND cpol.Success AND cpol.EvType = 2 THEN
                mask := mask + 1024;
                rd.Execute := TRUE;
                rd.ConnectionHdl := hdl;
                rd.Domain := 'tetherIEDtetherTestLD';
                rd.Item := 'LLN0.SWI1.valWTr.posVal#ST';
                st := 36;
            ELSIF waitN > 100 THEN
                mask := 16#DEAD; st := 99;
            ELSE
                waitN := waitN + 1;
                st := 34;
            END_IF;
        END_IF;
        IF cpol.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 36 THEN
        IF rd.Done THEN
            rd.Execute := FALSE;
            sval := rd.Value;
            // cancel leaves stVal at the operated value (true)
            IF FIND(sval, '1') > 0 THEN
                mask := mask + 2048;
                dis.Execute := TRUE;
                dis.ConnectionHdl := hdl;
                st := 7;
            ELSE
                mask := 16#DEAD; st := 99;
            END_IF;
        END_IF;
        IF rd.Error THEN mask := 16#DEAD; st := 99; END_IF;
    ELSIF st = 7 THEN
        IF dis.Done THEN
            dis.Execute := FALSE;
            mask := mask + 4096;
            st := 99;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; END_IF;
    ELSIF st = 99 THEN
        // terminal
    END_IF
    con(); dis(); ctl(); cpol(); rd();
END_PROGRAM
