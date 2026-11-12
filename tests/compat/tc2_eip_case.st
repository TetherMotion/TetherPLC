// tc2_eip_case.st — EtherNet/IP client e2e over the fake adapter.
// ListIdentity, RegisterSession, Get/Set_Attribute_Single, close.
PROGRAM tc2_eip_case
VAR
    con : FB_EipConnect;
    dis : FB_EipDisconnect;
    sta : FB_EipStatus;
    lid : FB_EipListIdentity;
    ga  : FB_EipGetAttribute;
    sa  : FB_EipSetAttribute;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        con.Execute := TRUE;
        con.RemoteHost := '127.0.0.1';
        con.RemotePort := 14481;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF con.Done THEN
            hdl := con.SessionHdl;
            con.Execute := FALSE;
            mask := mask + 1;         // bit0: registered
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        waitN := waitN + 1;
        IF waitN >= 4 THEN
            sta.Execute := TRUE;
            sta.SessionHdl := hdl;
            st := 3;
        END_IF;
    END_IF;

    IF st = 3 THEN
        IF sta.Done THEN
            IF sta.SessionState = 1 AND sta.SessionId <> 0 THEN
                mask := mask + 2;             // bit1: session open
                sta.Execute := FALSE;
                lid.Execute := TRUE;
                lid.RemoteHost := '127.0.0.1';
                lid.RemotePort := 14481;
                st := 4;
            END_IF;
        END_IF;
        IF sta.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 4 THEN
        IF lid.Done THEN
            mask := mask + 4;                 // bit2: identity read
            lid.Execute := FALSE;
            ga.Execute := TRUE;
            ga.SessionHdl := hdl;
            ga.ClassId := 4;
            ga.InstanceId := 100;
            ga.AttributeId := 3;
            st := 5;
        END_IF;
        IF lid.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 5 THEN
        IF ga.Done THEN
            IF ga.ValueLen = 4 THEN
                mask := mask + 8;             // bit3: attr read (4B)
            END_IF;
            ga.Execute := FALSE;
            sa.Execute := TRUE;
            sa.SessionHdl := hdl;
            sa.ClassId := 4;
            sa.InstanceId := 100;
            sa.AttributeId := 3;
            sa.Value := 'ABCD';
            st := 6;
        END_IF;
        IF ga.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 6 THEN
        IF sa.Done THEN
            mask := mask + 16;                // bit4: attr written
            sa.Execute := FALSE;
            ga.Execute := TRUE;
            st := 7;
        END_IF;
        IF sa.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 7 THEN
        IF ga.Done THEN
            IF ga.Value = 'ABCD' THEN
                mask := mask + 32;            // bit5: readback ABCD
            END_IF;
            ga.Execute := FALSE;
            dis.Execute := TRUE;
            dis.SessionHdl := hdl;
            st := 8;
        END_IF;
        IF ga.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 8 THEN
        IF dis.Done THEN
            mask := mask + 64;                // bit6: disconnected
            st := 9;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con(); dis(); sta(); lid(); ga(); sa();
END_PROGRAM
