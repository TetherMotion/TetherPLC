// tc2_mqtt_queue_case.st — MQTT message-queue e2e over the python
// fake broker: connect, subscribe tplc/q/#, pump three published
// messages into FB_IotMqttMessageQueue, dequeue them FIFO, reset the
// queue, publish+dequeue a fourth, disconnect.
// NOTE: separate FB_IotMqttPublish instances per message — the FBs
// are edge-triggered and same-scan re-arming never produces an edge.
PROGRAM tc2_mqtt_queue_case
VAR
    con : FB_IotMqttClient;
    dis : FB_IotMqttDisconnect;
    sub : FB_IotMqttSubscribe;
    pub1 : FB_IotMqttPublish;
    pub2 : FB_IotMqttPublish;
    pub3 : FB_IotMqttPublish;
    pub4 : FB_IotMqttPublish;
    q : FB_IotMqttMessageQueue;
    st : INT;
    waitN : INT;
    dqPend : INT;
    dqN : INT;
    mask AT %QD0 : UDINT;
    hdl : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        waitN := 0;
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
            mask := mask + 1;              // bit0: connected
            sub.Execute := TRUE;
            sub.ConnectionHdl := hdl;
            sub.TopicFilter := 'tplc/q/#';
            sub.QoS := 0;
            q.Pump := TRUE;
            q.ConnectionHdl := hdl;
            st := 2;
        END_IF;
        IF con.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 2 THEN
        IF sub.Done THEN
            sub.Execute := FALSE;
            mask := mask + 2;              // bit1: subscribed
            pub1.Execute := TRUE;
            pub1.ConnectionHdl := hdl;
            pub1.Topic := 'tplc/q/one';
            pub1.Payload := 'm1';
            pub1.QoS := 0;
            st := 3;
        END_IF;
        IF sub.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 3 THEN
        IF pub1.Done THEN
            pub1.Execute := FALSE;
            pub2.Execute := TRUE;
            pub2.ConnectionHdl := hdl;
            pub2.Topic := 'tplc/q/two';
            pub2.Payload := 'm2';
            pub2.QoS := 0;
            st := 4;
        END_IF;
        IF pub1.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 4 THEN
        IF pub2.Done THEN
            pub2.Execute := FALSE;
            pub3.Execute := TRUE;
            pub3.ConnectionHdl := hdl;
            pub3.Topic := 'tplc/q/three';
            pub3.Payload := 'm3';
            pub3.QoS := 0;
            st := 5;
        END_IF;
        IF pub2.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    IF st = 5 THEN
        IF pub3.Done THEN
            pub3.Execute := FALSE;
            waitN := 0;
            st := 6;
        END_IF;
        IF pub3.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // wait for the pump to collect all three messages
    IF st = 6 THEN
        waitN := waitN + 1;
        IF q.Queued >= 3 THEN
            mask := mask + 4;              // bit2: queue filled
            dqN := 0;
            dqPend := 0;
            st := 7;
        ELSIF waitN > 300 THEN
            mask := 16#DEAD; st := 99;
        END_IF;
        IF q.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // dequeue x3 with a one-scan request pulse; outputs are checked
    // the scan after each request
    IF st = 7 THEN
        IF dqPend = 0 THEN
            q.DequeueReq := TRUE;
            dqPend := 1;
        ELSE
            q.DequeueReq := FALSE;
            IF q.HasMessage THEN
                dqN := dqN + 1;
                IF dqN = 1 AND q.Topic = 'tplc/q/one'
                   AND q.Payload = 'm1' THEN
                    mask := mask + 8;       // bit3: FIFO m1
                END_IF;
                IF dqN = 2 AND q.Topic = 'tplc/q/two'
                   AND q.Payload = 'm2' THEN
                    mask := mask + 16;      // bit4: FIFO m2
                END_IF;
                IF dqN = 3 AND q.Topic = 'tplc/q/three'
                   AND q.Payload = 'm3' THEN
                    mask := mask + 32;      // bit5: FIFO m3
                END_IF;
            END_IF;
            dqPend := 0;
            IF dqN >= 3 THEN st := 8; END_IF;
        END_IF;
        IF q.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    // queue must now be empty; reset and verify
    IF st = 8 THEN
        IF q.Queued = 0 THEN
            mask := mask + 64;             // bit6: drained
            q.ResetReq := TRUE;
            st := 9;
        ELSE
            waitN := waitN + 1;
            IF waitN > 300 THEN mask := 16#DEAD; st := 99; END_IF;
        END_IF;
    END_IF;

    IF st = 9 THEN
        q.ResetReq := FALSE;
        pub4.Execute := TRUE;
        pub4.ConnectionHdl := hdl;
        pub4.Topic := 'tplc/q/four';
        pub4.Payload := 'm4';
        pub4.QoS := 0;
        waitN := 0;
        st := 10;
    END_IF;

    // pump refills; wait for the fourth message then dequeue it
    IF st = 10 THEN
        waitN := waitN + 1;
        IF q.Queued >= 1 THEN
            dqPend := 0;
            st := 11;
        ELSIF waitN > 300 THEN
            mask := 16#DEAD; st := 99;
        END_IF;
    END_IF;

    IF st = 11 THEN
        IF dqPend = 0 THEN
            q.DequeueReq := TRUE;
            dqPend := 1;
        ELSE
            q.DequeueReq := FALSE;
            IF q.HasMessage AND q.Topic = 'tplc/q/four'
               AND q.Payload = 'm4' THEN
                mask := mask + 128;       // bit7: post-reset msg
            END_IF;
            dqPend := 0;
            st := 12;
        END_IF;
    END_IF;

    IF st = 12 THEN
        dis.Execute := TRUE;
        dis.ConnectionHdl := hdl;
        st := 13;
    END_IF;

    IF st = 13 THEN
        IF dis.Done THEN
            dis.Execute := FALSE;
            mask := mask + 256;            // bit8: disconnected
            st := 99;
        END_IF;
        IF dis.Error THEN mask := 16#DEAD; st := 99; END_IF;
    END_IF;

    con(); dis(); sub(); pub1(); pub2(); pub3(); pub4(); q();
END_PROGRAM
