// tc2_mqtt_case.st — MQTT client e2e over the python fake broker.
// Connect, subscribe, publish, receive the echo, disconnect.
PROGRAM tc2_mqtt_case
VAR
    con : FB_IotMqttClient;
    dis : FB_IotMqttDisconnect;
    sub : FB_IotMqttSubscribe;
    pub : FB_IotMqttPublish;
    rx  : FB_IotMqttReceive;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        con.Execute := TRUE;
        con.HostName := '127.0.0.1';
        con.Port := 11883;
        con.CleanSession := TRUE;
        con.KeepAlive := 60;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;         // bit0: connected
            sub.Execute := TRUE;
            sub.ConnectionHdl := hdl;
            sub.TopicFilter := 'tplc/test';
            sub.QoS := 0;
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF sub.Done THEN
            mask := mask + 2;         // bit1: subscribed
            sub.Execute := FALSE;
            pub.Execute := TRUE;
            pub.ConnectionHdl := hdl;
            pub.Topic := 'tplc/test';
            pub.Payload := 'hello';
            pub.QoS := 0;
            pub.RetainOn := FALSE;
            st := 3;
        END_IF;
        IF sub.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 3 THEN
        IF pub.Done THEN
            mask := mask + 4;         // bit2: published
            pub.Execute := FALSE;
            rx.Execute := TRUE;
            rx.ConnectionHdl := hdl;
            st := 4;
        END_IF;
        IF pub.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // poll for the broker's echo of our publish
    IF st = 4 THEN
        waitN := waitN + 1;
        IF rx.Message THEN
            IF rx.Payload = 'hello' AND rx.Topic = 'tplc/test' THEN
                mask := mask + 8;     // bit3: echoed publish landed
            END_IF;
            rx.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 5;
        END_IF;
        IF rx.Error THEN mask := 16#DEAD; st := 99; END_IF;
        IF waitN > 40 THEN
            rx.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 5;
        END_IF;
    END_IF;

    IF st = 5 THEN
        IF dis.Done THEN
            mask := mask + 16;        // bit4: disconnected
            st := 6;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con(); dis(); sub(); pub(); rx();
END_PROGRAM
