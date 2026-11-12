// tc2_opcua_case.st — OPC UA client e2e over the open62541 endpoint.
// Connects to the tether_io_motion_server --opcua listener, reads
// nc.state (U8), writes nc.scale=500.0 (F64), reads it back, then
// disconnects. The final mask lands in out 0.
PROGRAM tc2_opcua_case
VAR
    con : FB_UA_Connect;
    dis : FB_UA_Disconnect;
    sta : FB_UA_ConnectGetStatus;
    rd  : FB_UA_Read;
    wr  : FB_UA_Write;
    nsi : FB_UA_GetNamespaceIndex;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR

    // ---- state machine ----
    IF st = 0 THEN
        mask := 0;
        con.Execute := TRUE;
        con.ServerUrl := 'opc.tcp://[::1]:14199';
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;         // bit0: connected
            st := 2;
            waitN := 0;
        END_IF;
        IF con.Error THEN
            mask := 16#DEAD;
            st := 99;
        END_IF;
    END_IF;

    IF st = 2 THEN
        waitN := waitN + 1;
        IF waitN >= 10 THEN
            sta.Execute := TRUE;
            sta.ConnectionHdl := hdl;
            st := 3;
        END_IF;
    END_IF;

    IF st = 3 THEN
        IF sta.Done THEN
            IF sta.ConnectionState = 1 THEN   // TPLC_UA_CONN_OPEN
                mask := mask + 2;             // bit1: session open
                sta.Execute := FALSE;
                rd.Execute := TRUE;
                rd.ConnectionHdl := hdl;
                rd.NodeId := 'ns=2;s=nc.state';
                st := 4;
            END_IF;
        END_IF;
        IF sta.Error THEN
            mask := 16#DEAD;
            st := 99;
        END_IF;
    END_IF;

    IF st = 4 THEN
        IF rd.Done THEN
            IF rd.ValueType = 3 THEN          // U8 maps to U64 family
                mask := mask + 4;             // bit2: nc.state read
            END_IF;
            rd.Execute := FALSE;
            wr.Execute := TRUE;
            wr.ConnectionHdl := hdl;
            wr.NodeId := 'ns=2;s=nc.scale';
            wr.Value := 500.0;
            wr.ValueType := 4;                // F64
            st := 5;
        END_IF;
        IF rd.Error THEN
            mask := 16#DEAD;
            st := 99;
        END_IF;
    END_IF;

    IF st = 5 THEN
        IF wr.Done THEN
            mask := mask + 8;                 // bit3: nc.scale written
            wr.Execute := FALSE;
            rd.Execute := TRUE;
            rd.NodeId := 'ns=2;s=nc.scale';
            st := 6;
        END_IF;
        IF wr.Error THEN
            mask := 16#DEAD;
            st := 99;
        END_IF;
    END_IF;

    IF st = 6 THEN
        IF rd.Done THEN
            IF rd.Value > 499.0 AND rd.Value < 501.0 THEN
                mask := mask + 16;            // bit4: nc.scale==500
            END_IF;
            rd.Execute := FALSE;
            nsi.Execute := TRUE;
            nsi.ConnectionHdl := hdl;
            nsi.NamespaceUri := 'http://tether.io/params';
            st := 7;
        END_IF;
        IF rd.Error THEN
            mask := 16#DEAD;
            st := 99;
        END_IF;
    END_IF;

    IF st = 7 THEN
        IF nsi.Done THEN
            IF nsi.NamespaceIndex = 2 THEN
                mask := mask + 32;            // bit5: ns index 2
            END_IF;
            nsi.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 8;
        END_IF;
        IF nsi.Error THEN
            mask := 16#DEAD;
            st := 99;
        END_IF;
    END_IF;

    IF st = 8 THEN
        IF dis.Done THEN
            mask := mask + 64;                // bit6: disconnected
            st := 9;
        END_IF;
        IF dis.Error THEN
            mask := 16#DEAD;
            st := 99;
        END_IF;
    END_IF;

    con(); dis(); sta(); rd(); wr(); nsi();
END_PROGRAM
