// tc2_mqtt5_case.st — MQTT 5.0 client e2e over the python fake
// broker.  Connect via FB_IotMqtt5Client with connect properties,
// subscribe, publish, receive the echo, unsubscribe, disconnect.
PROGRAM tc2_mqtt5_case
VAR
    con : FB_IotMqtt5Client;
    dis : FB_IotMqttDisconnect;
    sub : FB_IotMqtt5Subscribe;
    unsub : FB_IotMqttUnsubscribe;
    pub : FB_IotMqttPublish;
    pub5 : FB_IotMqtt5Publish;
    rx  : FB_IotMqttReceive;
    rx5 : FB_IotMqtt5Receive;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    con(); dis(); sub(); unsub(); pub(); pub5(); rx(); rx5();
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
            sub.NoLocal := FALSE;
            sub.RetainAsPublished := FALSE;
            sub.RetainHandling := 0;
            sub.SubscriptionId := 7;
            sub.UserProperties := 'sk=sv';
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
                pub5.Execute := TRUE;
                pub5.ConnectionHdl := hdl;
                pub5.Topic := 'tplc/v5/props';
                pub5.Payload := 'withprops';
                pub5.QoS := 0;
                pub5.PayloadFormatUtf8 := TRUE;
                pub5.MessageExpiryS := 30;
                pub5.ContentType := 'application/json';
                pub5.ResponseTopic := 'tplc/v5/reply';
                pub5.CorrelationData := 'corr-1';
                pub5.UserProperties := 'k1=v1;k2=v2';
                st := 55;
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

    IF st = 55 THEN
        IF pub5.Done THEN
            pub5.Execute := FALSE;
            mask := mask + 64;        // bit6: v5 publish w/ props
            waitN := 0;
            st := 56;
        END_IF;
        IF pub5.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 56 THEN
        // pump the echo so the broker forwards the message; the
        // props themselves are verified broker-side via the log
        rx5.Execute := TRUE;
        rx5.ConnectionHdl := hdl;
        IF rx5.Done AND rx5.Message THEN
            rx5.Execute := FALSE;
            IF rx5.Topic = 'tplc/v5/props'
               AND rx5.PayloadFormatUtf8
               AND rx5.MessageExpiryS = 30
               AND rx5.ContentType = 'application/json'
               AND rx5.ResponseTopic = 'tplc/v5/reply'
               AND rx5.CorrelationData = 'corr-1'
               AND rx5.CorrelationLen = 6
               AND rx5.UserProperties = 'k1=v1;k2=v2' THEN
                mask := mask + 128; // bit7: props publish echoed
                unsub.Execute := TRUE;
                unsub.ConnectionHdl := hdl;
                unsub.TopicFilter := 'tplc/v5/#';
                st := 5;
            END_IF;
        END_IF;
        waitN := waitN + 1;
        IF waitN > 300 THEN mask := 16#DEAD; st := 99; END_IF;
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
