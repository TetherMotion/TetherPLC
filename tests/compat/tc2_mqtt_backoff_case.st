// tc2_mqtt_backoff_case.st — exponential-backoff reconnect e2e.
// Connect + subscribe + echo, arm FB_IotMqttBackoff, then the io
// script kills the broker (connection drops), restarts it, and the
// host watchdog must reconnect with a doubling delay.  The case
// waits for the drop, waits for reconnect, resubscribes (subs are
// not restored), publishes a second message and receives its echo.
// mask bits 0..9 -> expect 0x3ff.
PROGRAM tc2_mqtt_backoff_case
VAR
    con  : FB_IotMqttClient;
    sub  : FB_IotMqttSubscribe;
    pub  : FB_IotMqttPublish;
    rx   : FB_IotMqttReceive;
    sts  : FB_IotMqttStatus;
    bo   : FB_IotMqttBackoff;
    sub2 : FB_IotMqttSubscribe;
    pub2 : FB_IotMqttPublish;
    rx2  : FB_IotMqttReceive;
    dis  : FB_IotMqttDisconnect;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        con.Execute := TRUE;
        con.HostName := '127.0.0.1';
        con.Port := 11885;
        con.CleanSession := TRUE;
        con.KeepAlive := 60;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF con.Done THEN
            hdl := con.ConnectionHdl;
            con.Execute := FALSE;
            mask := mask + 1;
            sub.Execute := TRUE;
            sub.ConnectionHdl := hdl;
            sub.TopicFilter := 'tplc/bo/#';
            sub.QoS := 0;
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF sub.Done THEN
            sub.Execute := FALSE;
            mask := mask + 2;
            pub.Execute := TRUE;
            pub.ConnectionHdl := hdl;
            pub.Topic := 'tplc/bo/a';
            pub.Payload := 'one';
            pub.QoS := 0;
            pub.RetainOn := FALSE;
            st := 3;
        END_IF;
        IF sub.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 3 THEN
        IF pub.Done THEN
            pub.Execute := FALSE;
            mask := mask + 4;
            // rx/sts start only after SUBACK+PUBLISH settled — a
            // concurrent MQTTClient_receive can steal the SUBACK
            // while subscribe waits for it
            rx.Execute := TRUE;
            rx.ConnectionHdl := hdl;
            sts.Execute := TRUE;
            sts.ConnectionHdl := hdl;
            waitN := 0;
            st := 4;
        END_IF;
        IF pub.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // first echo proves the baseline session works
    IF st = 4 THEN
        waitN := waitN + 1;
        IF rx.Message AND rx.Payload = 'one' THEN
            mask := mask + 8;
            bo.Execute := TRUE;
            bo.ConnectionHdl := hdl;
            bo.MinMs := 50;
            bo.MaxMs := 500;
            bo.Enable := TRUE;
            st := 5;
        END_IF;
        IF waitN > 600 THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // arm the watchdog, then wait for the broker kill to surface
    IF st = 5 THEN
        IF bo.Done THEN
            bo.Execute := FALSE;
            mask := mask + 16;
            waitN := 0;
            st := 6;
        END_IF;
        IF bo.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // broker is dead: rx keeps polling so Paho observes the EOF
    IF st = 6 THEN
        waitN := waitN + 1;
        IF NOT sts.Connected THEN
            mask := mask + 32;
            waitN := 0;
            st := 7;
        END_IF;
        IF waitN > 800 THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // broker restarted: watchdog reconnects (delay doubling)
    IF st = 7 THEN
        waitN := waitN + 1;
        IF sts.Connected THEN
            mask := mask + 64;
            sts.Execute := FALSE;
            rx.Execute := FALSE;
            sub2.Execute := TRUE;
            sub2.ConnectionHdl := hdl;
            sub2.TopicFilter := 'tplc/bo/#';
            sub2.QoS := 0;
            st := 8;
        END_IF;
        IF waitN > 1200 THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 8 THEN
        IF sub2.Done THEN
            sub2.Execute := FALSE;
            mask := mask + 128;
            pub2.Execute := TRUE;
            pub2.ConnectionHdl := hdl;
            pub2.Topic := 'tplc/bo/b';
            pub2.Payload := 'two';
            pub2.QoS := 0;
            pub2.RetainOn := FALSE;
            rx2.Execute := TRUE;
            rx2.ConnectionHdl := hdl;
            waitN := 0;
            st := 9;
        END_IF;
        IF sub2.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 9 THEN
        waitN := waitN + 1;
        IF rx2.Message AND rx2.Payload = 'two' THEN
            mask := mask + 256;
            rx2.Execute := FALSE;
            dis.Execute := TRUE;
            dis.ConnectionHdl := hdl;
            st := 10;
        END_IF;
        IF waitN > 600 THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 10 THEN
        IF dis.Done THEN
            mask := mask + 512;
            st := 11;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con(); sub(); pub(); rx(); sts(); bo();
    sub2(); pub2(); rx2(); dis();
END_PROGRAM
