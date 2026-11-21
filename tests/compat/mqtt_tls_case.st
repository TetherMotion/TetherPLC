// mqtt_tls_case.st — FB_Mqtt TLS e2e against the python fake broker
// (--tls).  Two sequential connects on one handle:
//   conn1: ssl:// + tls_insecure (self-signed accepted) + credentials
//   conn2: ssl:// + CA-file verification (TlsCaFile) + credentials
// mask %QD0 bits (expect 0x0f):
//   b0 conn1 CONNECTED    b2 conn2 CONNECTED
//   b1 conn1 publish ok   b3 conn2 publish ok
PROGRAM mqtt_tls_case
VAR
    m : FB_Mqtt;
    st : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
    caf : STRING[255];
END_VAR
    caf := '/tmp/tether_mqtt_tls/ca.pem';
    IF st = 0 THEN
        mask := 0;
        // ---- conn1: TLS, insecure (self-signed), creds, MQTT5 ----
        m.OP := 1;
        m.HOST := 'localhost';
        m.PORT := 11886;
        m.USERNAME := 'tlsuser';
        m.PASSWORD := 'tlspass';
        m.CLIENTID := 'tls-e2e-1';
        m.CONNECTMS := 3000;
        m.MQTTVER := 5;
        m.USETLS := 1;
        m.TLSINSECURE := 1;
        m.TLSCAFILE := '';
        m();
        IF m.STATUS = 0 THEN
            hdl := m.HANDLE;
            st := 1;
        ELSE
            st := 90;
        END_IF;
    END_IF;

    IF st = 1 THEN
        m.OP := 8;
        m.HANDLE := hdl;
        m();
        IF m.CONNECTED <> 0 THEN
            mask := mask + 1;
            st := 2;
        END_IF;
        IF st < 90 AND m.STATUS < 0 THEN st := 90; END_IF;
    END_IF;

    IF st = 2 THEN
        m.OP := 4;
        m.HANDLE := hdl;
        m.TOPIC := 'tls/one';
        m.PAYLOAD := 'A';
        m.QOS := 0;
        m.FLG_RETAIN := 0;
        m();
        IF m.STATUS = 0 THEN mask := mask + 2; END_IF;
        st := 3;
    END_IF;

    IF st = 3 THEN
        m.OP := 2;
        m.HANDLE := hdl;
        m();
        // ---- conn2: TLS with CA verification ----
        m.OP := 1;
        m.HOST := 'localhost';
        m.PORT := 11886;
        m.USERNAME := 'tlsuser';
        m.PASSWORD := 'tlspass';
        m.CLIENTID := 'tls-e2e-2';
        m.CONNECTMS := 3000;
        m.MQTTVER := 5;
        m.USETLS := 1;
        m.TLSINSECURE := 0;
        m.TLSCAFILE := caf;
        m();
        IF m.STATUS = 0 THEN
            hdl := m.HANDLE;
            st := 4;
        ELSE
            st := 90;
        END_IF;
    END_IF;

    IF st = 4 THEN
        m.OP := 8;
        m.HANDLE := hdl;
        m();
        IF m.CONNECTED <> 0 THEN
            mask := mask + 4;
            st := 5;
        END_IF;
        IF st < 90 AND m.STATUS < 0 THEN st := 90; END_IF;
    END_IF;

    IF st = 5 THEN
        m.OP := 4;
        m.HANDLE := hdl;
        m.TOPIC := 'tls/two';
        m.PAYLOAD := 'B';
        m.QOS := 0;
        m.FLG_RETAIN := 0;
        m();
        IF m.STATUS = 0 THEN mask := mask + 8; END_IF;
        st := 6;
    END_IF;

    IF st >= 90 THEN
        mask := 16#DEAD;
    END_IF;
END_PROGRAM
