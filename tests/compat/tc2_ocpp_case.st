// tc2_ocpp_case.st — OCPP-J client e2e over the python CSMS fake.
// WS upgrade, OCPP call -> echoed result, disconnect.
PROGRAM tc2_ocpp_case
VAR
    con : FB_OcppConnect;
    dis : FB_OcppDisconnect;
    cal : FB_OcppCall;
    rx  : FB_OcppReceive;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        con.Execute := TRUE;
        con.Host := '127.0.0.1';
        con.Port := 11890;
        con.Path := '/ocpp';
        con.ChargePointId := 'CP001';
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;         // bit0: ws session up
            cal.Execute := TRUE;
            cal.ConnectionHdl := hdl;
            cal.MessageId := 'boot-1';
            cal.ActName := 'BootNotification';
            cal.Payload := '{"chargePointVendor":"Tether","chargePointModel":"TPLC"}';
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF cal.Done THEN
            mask := mask + 2;         // bit1: call sent
            cal.Execute := FALSE;
            rx.Execute := TRUE;
            rx.ConnectionHdl := hdl;
            st := 3;
        END_IF;
        IF cal.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // poll for the CSMS fake's echoed result
    IF st = 3 THEN
        waitN := waitN + 1;
        IF rx.Message THEN
            IF rx.MessageType = 3 THEN
                mask := mask + 4;     // bit2: result landed
            END_IF;
            rx.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 4;
        END_IF;
        IF rx.Error THEN mask := 16#DEAD; st := 99; END_IF;
        IF waitN > 40 THEN
            rx.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 4;
        END_IF;
    END_IF;

    IF st = 4 THEN
        IF dis.Done THEN
            mask := mask + 8;         // bit3: released
            st := 5;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con(); dis(); cal(); rx();
END_PROGRAM
