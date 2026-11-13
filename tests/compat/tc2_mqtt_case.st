// tc2_mqtt_case.st — MQTT client e2e over the python fake broker.
// Connect, publish retained, subscribe wildcard, receive the retained
// delivery (Retained=TRUE), publish live, receive the echo
// (Retained=FALSE), disconnect.  Then a second pair: a will-bearing
// client connects, a subscriber watches its will topic, the client
// aborts ungracefully (no DISCONNECT), and the broker's will delivery
// lands.  Also exercises F_MqttTopicMatch.
PROGRAM tc2_mqtt_case
VAR
    con : FB_IotMqttClient;
    dis : FB_IotMqttDisconnect;
    sub : FB_IotMqttSubscribe;
    pub : FB_IotMqttPublish;
    rx  : FB_IotMqttReceive;
    conW : FB_IotMqttClient;
    con2 : FB_IotMqttClient;
    sub2 : FB_IotMqttSubscribe;
    pub2 : FB_IotMqttPublish;
    rx2  : FB_IotMqttReceive;
    ab   : FB_IotMqttAbort;
    dis2 : FB_IotMqttDisconnect;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
    hdlW : UDINT;
    hdl2 : UDINT;
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
            conW.Execute := TRUE;
            conW.HostName := '127.0.0.1';
            conW.Port := 11883;
            conW.CleanSession := TRUE;
            conW.KeepAlive := 60;
            conW.WillTopic := 'tplc/will';
            conW.WillPayload := 'gone';
            conW.WillQoS := 0;
            conW.WillRetain := FALSE;
            st := 7;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // will client connected -> bring up the watcher connection
    IF st = 7 THEN
        IF conW.Done THEN
            hdlW := conW.ConnectionHdl;
            conW.Execute := FALSE;
            mask := mask + 128;       // bit7: will client connected
            con2.Execute := TRUE;
            con2.HostName := '127.0.0.1';
            con2.Port := 11883;
            con2.CleanSession := TRUE;
            con2.KeepAlive := 60;
            st := 8;
        END_IF;
        IF conW.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 8 THEN
        IF con2.Done THEN
            hdl2 := con2.ConnectionHdl;
            con2.Execute := FALSE;
            sub2.Execute := TRUE;
            sub2.ConnectionHdl := hdl2;
            sub2.TopicFilter := 'tplc/will';
            sub2.QoS := 0;
            st := 9;
        END_IF;
        IF con2.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 9 THEN
        IF sub2.Done THEN
            sub2.Execute := FALSE;
            mask := mask + 256;       // bit8: will topic subscribed
            // sanity: conn2 rx path works before the abort — publish
            // a ping to its own subscription (echo arrives as 'ping',
            // ignored by the will check in st11)
            pub2.Execute := TRUE;
            pub2.ConnectionHdl := hdl2;
            pub2.Topic := 'tplc/will';
            pub2.Payload := 'ping';
            pub2.QoS := 0;
            pub2.RetainOn := FALSE;
            rx2.Execute := TRUE;
            rx2.ConnectionHdl := hdl2;
            waitN := 0;
            st := 10;
        END_IF;
        IF sub2.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // wait for the self-ping echo, then abort the will client
    IF st = 10 THEN
        waitN := waitN + 1;
        IF rx2.Message AND rx2.Payload = 'ping' THEN
            mask := mask + 512;       // bit9: conn2 rx path verified
            ab.Execute := TRUE;
            ab.ConnectionHdl := hdlW;
            st := 13;
        END_IF;
        IF rx2.Error THEN mask := 16#DEAD; st := 99; END_IF;
        IF pub2.Error THEN mask := 16#DEAD; st := 99; END_IF;
        IF waitN > 400 THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // ungraceful drop: no DISCONNECT, broker must fire the will
    IF st = 13 THEN
        IF ab.Done THEN
            ab.Execute := FALSE;
            waitN := 0;
            st := 11;
        END_IF;
        IF ab.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 11 THEN
        waitN := waitN + 1;
        IF rx2.Message THEN
            IF rx2.Topic = 'tplc/will' AND rx2.Payload = 'gone' THEN
                mask := mask + 1024;  // bit10: will delivered
                rx2.Execute := FALSE;
                dis2.Execute := TRUE;
                dis2.ConnectionHdl := hdl2;
                st := 12;
            END_IF;
        END_IF;
        IF rx2.Error THEN mask := 16#DEAD; st := 99; END_IF;
        IF waitN > 400 THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 12 THEN
        IF dis2.Done THEN
            mask := mask + 2048;      // bit11: watcher disconnected
            st := 14;
        END_IF;
        IF dis2.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con(); dis(); sub(); pub(); rx();
    conW(); con2(); sub2(); pub2(); rx2(); ab(); dis2();
END_PROGRAM
