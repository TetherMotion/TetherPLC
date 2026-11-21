// mqtt_ws_case.st — FB_Mqtt WebSocket e2e against the python fake
// broker (--ws).  Two sequential connects:
//   conn1: ws://  (plain WebSocket, port 11887)
//   conn2: wss:// (WebSocket over TLS, self-signed, port 11888)
// Both carry username/password/client_id.  mask %QD0 bits
// (expect 0x0f):
//   b0 ws  CONNECTED    b2 wss CONNECTED
//   b1 ws  publish ok   b3 wss publish ok
PROGRAM mqtt_ws_case
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
        // ---- conn1: ws:// plain WebSocket ----
        m.OP := 1;
        m.HOST := 'localhost';
        m.PORT := 11887;
        m.USERNAME := 'wsuser';
        m.PASSWORD := 'wspass';
        m.CLIENTID := 'ws-e2e-1';
        m.CONNECTMS := 5000;
        m.TRANSPORT := 3;             // ws
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
        m.TOPIC := 'ws/one';
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
        // ---- conn2: wss:// WebSocket over TLS (self-signed) ----
        m.OP := 1;
        m.HOST := 'localhost';
        m.PORT := 11888;
        m.USERNAME := 'wsuser';
        m.PASSWORD := 'wspass';
        m.CLIENTID := 'ws-e2e-2';
        m.CONNECTMS := 5000;
        m.TRANSPORT := 4;             // wss
        m.USETLS := 1;
        m.TLSINSECURE := 1;
        m.TLSCAFILE := '';
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
        m.TOPIC := 'ws/two';
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
