// tc2_mqtt_case.st — MQTT client e2e over the python fake broker.
// Connect, publish retained, subscribe wildcard, receive the retained
// delivery (Retained=TRUE), publish live, receive the echo
// (Retained=FALSE), disconnect.  Also exercises F_MqttTopicMatch.
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
        IF F_MqttTopicMatch('tplc/#', 'tplc/a/b')
           AND F_MqttTopicMatch('tplc/+/x', 'tplc/a/x')
           AND NOT F_MqttTopicMatch('tplc/+/x', 'tplc/a/y')
           AND F_MqttTopicMatch('a/b', 'a/b')
           AND NOT F_MqttTopicMatch('a/b', 'a/b/c') THEN
            mask := mask + 64;    // bit6: topic matcher
        END_IF;
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
            pub.Execute := TRUE;
            pub.ConnectionHdl := hdl;
            pub.Topic := 'tplc/keep';
            pub.Payload := 'saved';
            pub.QoS := 0;
            pub.RetainOn := TRUE;
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF pub.Done THEN
            mask := mask + 2;         // bit1: retained publish sent
            pub.Execute := FALSE;
            sub.Execute := TRUE;
            sub.ConnectionHdl := hdl;
            sub.TopicFilter := 'tplc/#';
            sub.QoS := 0;
            st := 3;
        END_IF;
        IF pub.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 3 THEN
        IF sub.Done THEN
            mask := mask + 4;         // bit2: subscribed
            sub.Execute := FALSE;
            rx.Execute := TRUE;
            rx.ConnectionHdl := hdl;
            st := 4;
        END_IF;
        IF sub.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // first message must be the broker's retained delivery
    IF st = 4 THEN
        waitN := waitN + 1;
        IF rx.Message THEN
            IF rx.Topic = 'tplc/keep' AND rx.Payload = 'saved'
               AND rx.Retained THEN
                mask := mask + 8;     // bit3: retained msg landed
            END_IF;
            // keep rx.Execute TRUE so st5 keeps polling for the echo
            pub.Execute := TRUE;
            pub.ConnectionHdl := hdl;
            pub.Topic := 'tplc/test';
            pub.Payload := 'hello';
            pub.QoS := 0;
            pub.RetainOn := FALSE;
            st := 5;
        END_IF;
        IF rx.Error THEN mask := 16#DEAD; st := 99; END_IF;
        IF waitN > 400 THEN
            rx.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 6;
        END_IF;
    END_IF;

    // second message is the live echo of our publish
    IF st = 5 THEN
        waitN := waitN + 1;
        IF rx.Message THEN
            IF rx.Topic = 'tplc/test' AND rx.Payload = 'hello'
               AND NOT rx.Retained THEN
                mask := mask + 16;    // bit4: live echo, not retained
                rx.Execute := FALSE;
                dis.Execute := TRUE;
                dis.ConnectionHdl := hdl;
                st := 6;
            END_IF;
            // non-matching messages (e.g. a repeated retained
            // delivery) are consumed and polling continues
        END_IF;
        IF rx.Error THEN mask := 16#DEAD; st := 99; END_IF;
        IF waitN > 400 THEN
            rx.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 6;
        END_IF;
    END_IF;

    IF st = 6 THEN
        IF dis.Done THEN
            mask := mask + 32;        // bit5: disconnected
            st := 7;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con(); dis(); sub(); pub(); rx();
END_PROGRAM
