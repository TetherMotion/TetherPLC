// ha_bridge_case.st — FB_HaBridge e2e over the python fake broker.
// Configures 6 channels (sensor/switch/number/button/text/event),
// connects, publishes discovery+state, receives injected commands.
// Broker --inject fires HA->PLC commands 0.3-1.1 s after each
// topic's subscription appears (subscription-gated, sim-speed safe).
// mask %QD0 bits (all must be 1 -> 0xff):
//   b0 Connected seen          b4 text cmd RxTxt='reflow'
//   b1 helper unit tests pass  b5 event channel RxTxt='PING'
//   b2 switch cmd ON seen      b6 button Event seen
//   b3 number cmd RxVal=72.5   b7 TxCnt>=10 (discovery+state+avail)
PROGRAM ha_bridge_case
VAR
    br : FB_HaBridge;
    st : INT;
    mask AT %QD0 : UDINT;
    l2, l3, l4, l5, l6, l7 : BOOL;
END_VAR
    // ---- one-shot string helper checks ----
    IF st = 0 THEN
        mask := 0;
        IF F_HaTopicMatch('tether/+', 'tether/plc1')
           AND F_HaTopicMatch('a/#', 'a/b/c')
           AND F_HaTopicMatch('#', 'anything/at/all')
           AND NOT F_HaTopicMatch('a/+', 'a/b/c')
           AND F_HaTopicMatch('x/y/z', 'x/y/z')
           AND NOT F_HaTopicMatch('x/y', 'x/y/z')
           AND F_HaStrReal('72.5') > 72.4
           AND F_HaStrReal('72.5') < 72.6
           AND F_HaRealStr(23.5) = '23.500' THEN
            mask := mask + 2;                    // b1 all-or-nothing
        END_IF;
        // ---- bridge config ----
        br.Execute := TRUE;
        br.CycleTime := 0.01;
        br.Dev := 'plc1';
        br.Host := '127.0.0.1';
        br.Port := 11883;
        br.User := 'plcuser';
        br.Pass := 'testpass';
        br.ClientId := 'ha-bridge-e2e';
        br.ConnectMs := 3000;
        br.KeepAlive := 60;
        br.Clean := TRUE;
        br.BackoffMin := 500;
        br.BackoffMax := 2000;
        br.HbS := 0;
        // ch1 sensor (PLC->HA)
        br.CHAN[1].En := TRUE;  br.CHAN[1].Kind := 0;
        br.CHAN[1].ObjId := 'boiler_temp';
        br.CHAN[1].Name := 'Boiler temp';
        br.CHAN[1].Unit := 'degC';
        br.CHAN[1].DevCls := 'temperature';
        br.CHAN[1].MinV := 0.0; br.CHAN[1].MaxV := 150.0;
        br.CHAN[1].Value := 23.5;
        // ch2 switch (both)
        br.CHAN[2].En := TRUE;  br.CHAN[2].Kind := 2;
        br.CHAN[2].ObjId := 'pump';
        br.CHAN[2].Name := 'Circulation pump';
        br.CHAN[2].BVal := TRUE;
        // ch3 number (both, AutoWr)
        br.CHAN[3].En := TRUE;  br.CHAN[3].Kind := 3;
        br.CHAN[3].ObjId := 'sp';
        br.CHAN[3].Name := 'Setpoint';
        br.CHAN[3].MinV := 0.0; br.CHAN[3].MaxV := 100.0;
        br.CHAN[3].StepV := 0.5; br.CHAN[3].AutoWr := TRUE;
        br.CHAN[3].Value := 50.0;
        // ch4 button (HA->PLC)
        br.CHAN[4].En := TRUE;  br.CHAN[4].Kind := 4;
        br.CHAN[4].ObjId := 'jog';
        br.CHAN[4].Name := 'Jog';
        // ch5 text (both, AutoWr)
        br.CHAN[5].En := TRUE;  br.CHAN[5].Kind := 5;
        br.CHAN[5].ObjId := 'mode';
        br.CHAN[5].Name := 'Mode';
        br.CHAN[5].AutoWr := TRUE;
        br.CHAN[5].Txt := 'auto';
        // ch6 event (HA->PLC, absolute topic via leading '/')
        br.CHAN[6].En := TRUE;  br.CHAN[6].Kind := 6;
        br.CHAN[6].ObjId := '/ha/button/press';
        br.CHAN[6].Name := 'HA button';
        st := 1;
    END_IF;

    br();

    IF st = 1 THEN
        IF br.Connected THEN
            mask := mask + 1;                 // b0
            st := 2;
        END_IF;
        IF st < 90 AND br.Err THEN st := 90; END_IF;
    END_IF;

    IF st = 2 THEN
        // wait for discovery + subscriptions + first publishes
        IF br.TxCnt >= 10 THEN
            st := 3;
        END_IF;
        IF st < 90 AND br.Err THEN st := 90; END_IF;
    END_IF;

    IF st >= 3 AND st < 90 THEN
        // latch command observations once (received fields persist,
        // Event is a 1-scan pulse — both need edge guards)
        IF br.CHAN[2].Cmd = 1 AND NOT l2 THEN
            mask := mask + 4; l2 := TRUE;                    // b2
        END_IF;
        IF br.CHAN[3].RxVal > 72.4 AND br.CHAN[3].RxVal < 72.6
           AND br.CHAN[3].MsgCnt > 0 AND NOT l3 THEN
            mask := mask + 8; l3 := TRUE;                    // b3
        END_IF;
        IF br.CHAN[5].RxTxt = 'reflow' AND NOT l4 THEN
            mask := mask + 16; l4 := TRUE;                   // b4
        END_IF;
        IF br.CHAN[6].RxTxt = 'PING' AND NOT l5 THEN
            mask := mask + 32; l5 := TRUE;                   // b5
        END_IF;
        IF br.CHAN[4].Event AND NOT l6 THEN
            mask := mask + 64; l6 := TRUE;                   // b6
        END_IF;
        IF br.TxCnt >= 10 AND NOT l7 THEN
            mask := mask + 128; l7 := TRUE;                  // b7
        END_IF;
    END_IF;

    IF st >= 90 THEN
        mask := 16#DEAD;
    END_IF;
END_PROGRAM
