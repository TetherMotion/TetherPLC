// tc2_iec61850_case.st — IEC 61850 client e2e over the fake IED.
// Connect+associate, read, write, GetNameList, disconnect.
PROGRAM tc2_iec61850_case
VAR
    con : FB_Iec61850Connect;
    dis : FB_Iec61850Disconnect;
    sta : FB_Iec61850Status;
    rd  : FB_Iec61850Read;
    wr  : FB_Iec61850Write;
    gnl : FB_Iec61850GetNameList;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        con.Execute := TRUE;
        con.RemoteHost := '127.0.0.1';
        con.RemotePort := 11020;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;         // bit0: associated
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        waitN := waitN + 1;
        IF waitN >= 4 THEN
            sta.Execute := TRUE;
            sta.ConnectionHdl := hdl;
            st := 3;
        END_IF;
    END_IF;

    IF st = 3 THEN
        IF sta.Done THEN
            IF sta.ConnectionState = 1 THEN
                mask := mask + 2;             // bit1: association open
                sta.Execute := FALSE;
                rd.Execute := TRUE;
                rd.ConnectionHdl := hdl;
                rd.Domain := 'tetherIEDtetherTestLD';
                rd.Item := 'LLN0.Name.val';
                st := 4;
            END_IF;
        END_IF;
        IF sta.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 4 THEN
        IF rd.Done THEN
            IF rd.Value = 'TetherIED' THEN
                mask := mask + 4;             // bit2: read name
            END_IF;
            rd.Execute := FALSE;
            rd.ConnectionHdl := hdl;
            rd.Domain := 'tetherIEDtetherTestLD';
            rd.Item := 'LLN0.Health.stVal';
            st := 5;
        END_IF;
        IF rd.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 5 THEN
        IF rd.Done THEN
            mask := mask + 8;                 // bit3: read stVal
            rd.Execute := FALSE;
            wr.Execute := TRUE;
            wr.ConnectionHdl := hdl;
            wr.Domain := 'tetherIEDtetherTestLD';
            wr.Item := 'LLN0.SetPt.setMag.f#SP';
            wr.Value := '9';
            st := 6;
        END_IF;
        IF rd.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 6 THEN
        IF wr.Done THEN
            mask := mask + 16;                // bit4: write stVal
            wr.Execute := FALSE;
            gnl.Execute := TRUE;
            gnl.ConnectionHdl := hdl;
            gnl.Domain := 'tetherIEDtetherTestLD';
            st := 7;
        END_IF;
        IF wr.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 7 THEN
        IF gnl.Done THEN
            mask := mask + 32;                // bit5: namelist
            gnl.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 8;
        END_IF;
        IF gnl.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 8 THEN
        IF dis.Done THEN
            mask := mask + 64;                // bit6: released
            st := 9;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con(); dis(); sta(); rd(); wr(); gnl();
END_PROGRAM
