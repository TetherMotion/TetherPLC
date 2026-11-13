// tc2_mqtt5_case.st — MQTT 5.0 client e2e over the python fake
// broker.  Connect via FB_IotMqtt5Client with connect properties,
// subscribe, publish, receive the echo, unsubscribe, disconnect.
PROGRAM tc2_mqtt5_case
VAR
    con : FB_IotMqtt5Client;
    dis : FB_IotMqttDisconnect;
    sub : FB_IotMqttSubscribe;
    unsub : FB_IotMqttUnsubscribe;
    pub : FB_IotMqttPublish;
    rx  : FB_IotMqttReceive;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    con(); dis(); sub(); unsub(); pub(); rx();
    IF st = 0 THEN
        mask := 0;
        con.Execute := TRUE;
        con.HostName := '127.0.0.1';
        con.Port := 11883;
        con.CleanSession := TRUE;
        con.KeepAlive := 60;
        con.SessionExpiryS := 30;
        con.ReceiveMaximum := 10;
        con.TopicAliasMaximum := 4;
        con.RequestResponseInformation := TRUE;
        con.RequestProblemInformation := TRUE;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;         // bit0: v5 connected
            sub.Execute := TRUE;
            sub.ConnectionHdl := hdl;
            sub.TopicFilter := 'tplc/v5/#';
            sub.QoS := 0;
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF sub.Done THEN
            sub.Execute := FALSE;
            mask := mask + 2;         // bit1: subscribed (v5 SUBACK)
            pub.Execute := TRUE;
            pub.ConnectionHdl := hdl;
            pub.Topic := 'tplc/v5/one';
            pub.Payload := 'm5';
            pub.QoS := 0;
            st := 3;
        END_IF;
        IF sub.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 3 THEN
        IF pub.Done THEN
            pub.Execute := FALSE;
            mask := mask + 4;         // bit2: published (v5 PUBLISH)
            waitN := 0;
            st := 4;
        END_IF;
        IF pub.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 4 THEN
        rx.Execute := TRUE;
        rx.ConnectionHdl := hdl;
        IF rx.Done AND rx.Message THEN
            rx.Execute := FALSE;
            IF rx.Topic = 'tplc/v5/one' AND rx.Payload = 'm5' THEN
                mask := mask + 8;     // bit3: echo received (v5)
                unsub.Execute := TRUE;
                unsub.ConnectionHdl := hdl;
                unsub.TopicFilter := 'tplc/v5/#';
                st := 5;
            END_IF;
        END_IF;
        IF rx.Error THEN mask := 16#DEAD; st := 99; END_IF;
        waitN := waitN + 1;
        IF waitN > 300 THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 5 THEN
        IF unsub.Done THEN
            unsub.Execute := FALSE;
            mask := mask + 16;        // bit4: unsubscribed
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 6;
        END_IF;
        IF unsub.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 6 THEN
        IF dis.Done THEN
            mask := mask + 32;        // bit5: disconnected
            st := 7;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 7 THEN
        st := 8;
    END_IF;
END_PROGRAM
