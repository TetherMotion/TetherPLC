// mqtt_journal_case.st — ST-side event journal e2e: FB_MqttLink +
// FB_MqttSub + FB_MqttRx feed FB_MqttEventLog; the fake broker injects
// three messages on jrn/# after the subscription lands.
// mask %QD0 bits (expect 0x3f):
//   b0 link Connected     b3 LOG[0..2] topics+payloads in order
//   b1 sub Done seen      b4 journal Head entry = newest
//   b2 journal Count>=3   b5 rx MsgCnt>=3
PROGRAM mqtt_journal_case
VAR
    lk : FB_MqttLink;
    sub : FB_MqttSub;
    rx : FB_MqttRx;
    lg : FB_MqttEventLog;
    st : INT;
    mask AT %QD0 : UDINT;
    gotCnt : BOOL;
    gotRx : BOOL;
    gotTopics : BOOL;
    gotHead : BOOL;
    i : UDINT;
    nScan : UDINT;
    sawA : BOOL;
    sawB : BOOL;
    sawC : BOOL;
END_VAR
    lk.Execute := TRUE;
    lk.Host := 'localhost';
    lk.Port := 11883;
    lk.ClientId := 'journal-e2e';
    lk.ConnectMs := 5000;
    lk.BackoffMin := 500;
    lk.BackoffMax := 2000;

    sub.Hdl := lk.Hdl;
    sub.Filter := 'jrn/#';
    sub.Qos := 0;
    sub.Execute := (st = 1);

    rx.Hdl := lk.Hdl;
    lg.Execute := TRUE;
    lg.CycleTime := 0.01;
    lg.Push := rx.HasMsg;
    lg.TopicIn := rx.Topic;
    lg.PayloadIn := rx.Payload;

    lk();
    sub();
    rx();
    lg();

    IF st = 0 THEN
        mask := 0;
        IF lk.Connected THEN
            mask := mask + 1;
            st := 1;
        END_IF;
        IF st < 90 AND lk.Err THEN st := 90; END_IF;
    END_IF;
    IF st = 1 THEN
        IF sub.Done THEN
            mask := mask + 2;
            st := 2;
        END_IF;
        IF st < 90 AND lk.Err THEN st := 90; END_IF;
    END_IF;
    IF st >= 2 AND st < 90 THEN
        // journal content — injects repeat cyclically so arrival
        // order interleaves; require all three pairs present in the
        // ring (any order) rather than a fixed first-3 sequence
        IF lg.Count >= 3 AND NOT gotCnt THEN
            mask := mask + 4;
            gotCnt := TRUE;
        END_IF;
        IF lg.Count >= 3 AND NOT gotTopics THEN
            sawA := FALSE; sawB := FALSE; sawC := FALSE;
            nScan := lg.Count;
            IF nScan > 16 THEN nScan := 16; END_IF;
            FOR i := 0 TO 15 DO
                IF i < nScan THEN
                    IF lg.LOG[i].Topic = 'jrn/a'
                       AND lg.LOG[i].Payload = 'MSG1' THEN sawA := TRUE; END_IF;
                    IF lg.LOG[i].Topic = 'jrn/b'
                       AND lg.LOG[i].Payload = 'MSG2' THEN sawB := TRUE; END_IF;
                    IF lg.LOG[i].Topic = 'jrn/c'
                       AND lg.LOG[i].Payload = 'MSG3' THEN sawC := TRUE; END_IF;
                END_IF;
            END_FOR;
            IF sawA AND sawB AND sawC THEN
                mask := mask + 8;
                gotTopics := TRUE;
            END_IF;
        END_IF;
        IF lg.Count >= 1 AND NOT gotHead THEN
            // Head points at the newest entry — payload must be one
            // of the injected values and Tick must be > 0
            IF (lg.LOG[lg.Head].Payload = 'MSG1'
                OR lg.LOG[lg.Head].Payload = 'MSG2'
                OR lg.LOG[lg.Head].Payload = 'MSG3')
               AND lg.LOG[lg.Head].Tick > 0.0 THEN
                mask := mask + 16;
                gotHead := TRUE;
            END_IF;
        END_IF;
        IF rx.MsgCnt >= 3 AND NOT gotRx THEN
            mask := mask + 32;
            gotRx := TRUE;
        END_IF;
        // all bits latched → done (expect 0x3f)
        IF mask >= 63 THEN st := 90; END_IF;
    END_IF;
END_PROGRAM
