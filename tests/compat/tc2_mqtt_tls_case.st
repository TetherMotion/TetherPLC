// tc2_mqtt_tls_case.st — MQTT over TLS e2e against the python fake
// broker's --tls mode.  conn1 uses UseTls+TlsInsecure (skip verify,
// self-signed broker); conn2 verifies against TlsCaFile (the
// broker's self-signed cert doubles as its own CA).  Each
// connection: CONNECT -> SUBSCRIBE -> PUBLISH -> receive echo ->
// DISCONNECT.  mask bits 0..10 -> expect 0x7ff.
PROGRAM tc2_mqtt_tls_case
VAR
    con1 : FB_IotMqttClient;
    sub1 : FB_IotMqttSubscribe;
    pub1 : FB_IotMqttPublish;
    rx1  : FB_IotMqttReceive;
    dis1 : FB_IotMqttDisconnect;
    con2 : FB_IotMqttClient;
    sub2 : FB_IotMqttSubscribe;
    pub2 : FB_IotMqttPublish;
    rx2  : FB_IotMqttReceive;
    dis2 : FB_IotMqttDisconnect;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    hdl1 : UDINT;
    hdl2 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        con1.Execute := TRUE;
        con1.HostName := '127.0.0.1';
        con1.Port := 11884;
        con1.CleanSession := TRUE;
        con1.KeepAlive := 60;
        con1.UseTls := TRUE;
        con1.TlsInsecure := TRUE;
        st := 1;
    END_IF;

    IF st = 1 THEN
        IF con1.Done THEN
            hdl1 := con1.ConnectionHdl;
            con1.Execute := FALSE;
            mask := mask + 1;
            sub1.Execute := TRUE;
            sub1.ConnectionHdl := hdl1;
            sub1.TopicFilter := 'tplc/tls/#';
            sub1.QoS := 0;
            st := 2;
        END_IF;
        IF con1.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF sub1.Done THEN
            sub1.Execute := FALSE;
            mask := mask + 2;
            pub1.Execute := TRUE;
            pub1.ConnectionHdl := hdl1;
            pub1.Topic := 'tplc/tls/a';
            pub1.Payload := 'secure1';
            pub1.QoS := 0;
            pub1.RetainOn := FALSE;
            st := 3;
        END_IF;
        IF sub1.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 3 THEN
        IF pub1.Done THEN
            pub1.Execute := FALSE;
            mask := mask + 4;
            rx1.Execute := TRUE;
            rx1.ConnectionHdl := hdl1;
            waitN := 0;
            st := 4;
        END_IF;
        IF pub1.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 4 THEN
        waitN := waitN + 1;
        IF rx1.Message THEN
            IF rx1.Topic = 'tplc/tls/a' AND rx1.Payload = 'secure1' THEN
                mask := mask + 8;
                rx1.Execute := FALSE;
                dis1.Execute := TRUE;
                dis1.ConnectionHdl := hdl1;
                st := 5;
            END_IF;
        END_IF;
        IF rx1.Error THEN mask := 16#DEAD; st := 99; END_IF;
        IF waitN > 400 THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 5 THEN
        IF dis1.Done THEN
            dis1.Execute := FALSE;
            mask := mask + 16;
            con2.Execute := TRUE;
            con2.HostName := '127.0.0.1';
            con2.Port := 11884;
            con2.CleanSession := TRUE;
            con2.KeepAlive := 60;
            con2.UseTls := TRUE;
            con2.TlsInsecure := FALSE;
            con2.TlsCaFile := '/tmp/tplc_mqtt_tls/cert.pem';
            st := 6;
        END_IF;
        IF dis1.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 6 THEN
        IF con2.Done THEN
            hdl2 := con2.ConnectionHdl;
            con2.Execute := FALSE;
            mask := mask + 32;
            sub2.Execute := TRUE;
            sub2.ConnectionHdl := hdl2;
            sub2.TopicFilter := 'tplc/tls/#';
            sub2.QoS := 0;
            st := 7;
        END_IF;
        IF con2.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 7 THEN
        IF sub2.Done THEN
            sub2.Execute := FALSE;
            mask := mask + 64;
            pub2.Execute := TRUE;
            pub2.ConnectionHdl := hdl2;
            pub2.Topic := 'tplc/tls/b';
            pub2.Payload := 'secure2';
            pub2.QoS := 0;
            pub2.RetainOn := FALSE;
            st := 8;
        END_IF;
        IF sub2.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 8 THEN
        IF pub2.Done THEN
            pub2.Execute := FALSE;
            mask := mask + 128;
            rx2.Execute := TRUE;
            rx2.ConnectionHdl := hdl2;
            waitN := 0;
            st := 9;
        END_IF;
        IF pub2.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 9 THEN
        waitN := waitN + 1;
        IF rx2.Message THEN
            IF rx2.Topic = 'tplc/tls/b' AND rx2.Payload = 'secure2' THEN
                mask := mask + 256;
                rx2.Execute := FALSE;
                dis2.Execute := TRUE;
                dis2.ConnectionHdl := hdl2;
                st := 10;
            END_IF;
        END_IF;
        IF rx2.Error THEN mask := 16#DEAD; st := 99; END_IF;
        IF waitN > 400 THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 10 THEN
        IF dis2.Done THEN
            mask := mask + 512;
            st := 11;
        END_IF;
        IF dis2.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con1(); sub1(); pub1(); rx1(); dis1();
    con2(); sub2(); pub2(); rx2(); dis2();
END_PROGRAM
