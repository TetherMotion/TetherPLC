// mqtt_blocks_case.st — easy FBD blocks e2e: FB_MqttLink (wss://)
// + FB_MqttSub + FB_MqttTx + FB_MqttRx loopback through the fake
// broker's --ws --tls mode.  mask %QD0 bits (expect 0x1f):
//   b0 link Connected     b3 rx HasMsg fields match
//   b1 sub Done seen      b4 rx MsgCnt>=1
//   b2 tx Done seen
PROGRAM mqtt_blocks_case
VAR
    lk : FB_MqttLink;
    tx : FB_MqttTx;
    sub : FB_MqttSub;
    rx : FB_MqttRx;
    st : INT;
    mask AT %QD0 : UDINT;
    gotMsg : BOOL;
    gotCnt : BOOL;
END_VAR
    // link config — wss:// self-signed + credentials
    lk.Execute := TRUE;
    lk.Host := 'localhost';
    lk.Port := 11888;
    lk.User := 'wsuser';
    lk.Pass := 'wspass';
    lk.ClientId := 'blocks-e2e';
    lk.ConnectMs := 5000;
    lk.Transport := 4;            // wss
    lk.UseTls := TRUE;
    lk.TlsInsecure := TRUE;
    lk.BackoffMin := 500;
    lk.BackoffMax := 2000;
    // sub/tx are level-driven: Execute high only during their step
    sub.Hdl := lk.Hdl;
    sub.Filter := 'fbd/test/#';
    sub.Qos := 0;
    sub.Execute := (st = 1);
    tx.Hdl := lk.Hdl;
    tx.Topic := 'fbd/test/1';
    tx.Payload := 'PING';
    tx.Qos := 0;
    tx.Retained := FALSE;
    tx.Execute := (st = 2);

    lk();
    sub();
    tx();
    rx.Hdl := lk.Hdl;
    rx();

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
    IF st = 2 THEN
        IF tx.Done THEN
            mask := mask + 4;
            st := 3;
        END_IF;
        IF st < 90 AND (lk.Err OR tx.Err) THEN st := 90; END_IF;
    END_IF;
    IF st >= 3 AND st < 90 THEN
        // rx fields persist — edge-guard so bits latch once
        IF rx.MsgCnt > 0 AND rx.Topic = 'fbd/test/1'
           AND rx.Payload = 'PING' AND NOT gotMsg THEN
            mask := mask + 8;
            gotMsg := TRUE;
        END_IF;
        IF rx.MsgCnt >= 1 AND NOT gotCnt THEN
            mask := mask + 16;
            gotCnt := TRUE;
        END_IF;
        IF mask >= 31 THEN
            st := 6;
        END_IF;
        IF st < 90 AND rx.Err THEN st := 90; END_IF;
    END_IF;
    IF st >= 90 THEN
        mask := 16#DEAD;
    END_IF;
END_PROGRAM
