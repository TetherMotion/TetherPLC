// tc2_mqtt5_req_case.st — MQTT5 request/response e2e: the fake
// broker runs in --responder mode and answers publishes on
// 'tplc/v5/req' that carry ResponseTopic + CorrelationData.
PROGRAM tc2_mqtt5_req_case
VAR
    con : FB_IotMqtt5Client;
    dis : FB_IotMqttDisconnect;
    req : FB_IotMqtt5RequestResponse;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    con(); dis(); req();
    IF st = 0 THEN
        mask := 0;
        con.Execute := TRUE;
        con.HostName := '127.0.0.1';
        con.Port := 11884;
        con.CleanSession := TRUE;
        con.KeepAlive := 60;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;         // bit0: v5 connected
            req.Execute := TRUE;
            req.ConnectionHdl := hdl;
            req.RequestTopic := 'tplc/v5/req';
            req.RequestPayload := 'ping';
            req.QoS := 0;
            req.ResponseTopic := 'tplc/v5/resp/in';
            req.CorrelationData := 'rc1';
            req.TimeoutScans := 400;
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF req.Done THEN
            req.Execute := FALSE;
            IF req.ResponsePayload = 'pong'
               AND req.ResponseTopicOut = 'tplc/v5/resp/in' THEN
                mask := mask + 2;     // bit1: correlated response
                dis.Execute := TRUE;
                dis.ConnectionHdl := hdl;
                st := 3;
            ELSE
                mask := 16#DEAD; st := 99;
            END_IF;
        END_IF;
        IF req.Error THEN mask := 16#DEAD; st := 99; END_IF;
        waitN := waitN + 1;
        IF waitN > 600 THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 3 THEN
        IF dis.Done THEN
            mask := mask + 4;         // bit2: disconnected
            st := 4;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 4 THEN
        st := 5;
    END_IF;
END_PROGRAM
