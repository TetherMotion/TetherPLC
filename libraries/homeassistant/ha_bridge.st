// ha_bridge.st — Home Assistant integration over MQTT.
//
// FB_HaBridge is a single-block HA device bridge: one MQTT
// connection (tplc-mqtt / FB_Mqtt, with the service's built-in
// exponential-backoff auto-reconnect via OP10) hosting up to 16
// channels.  Each channel is one HA entity; configure them
// member-wise before Execute (same pattern as prof.SEG[i]):
//
//   br.CHAN[1].Kind  := 0;                    // sensor
//   br.CHAN[1].ObjId := 'boiler_temp';
//   br.CHAN[1].Name  := 'Boiler temperature';
//   br.CHAN[1].Unit  := 'degC';
//   br.CHAN[1].DevCls:= 'temperature';
//   br.CHAN[1].En    := TRUE;
//   ...
//   br.CHAN[2].Kind  := 2;                    // switch (bidirectional)
//   br.CHAN[2].ObjId := 'circulation_pump';
//   ...
//
// Works identically from ST, FBD and LD — all three compile to the
// same FB call + member writes.
//
// Channel kinds (CHAN[i].Kind):
//   0 sensor        PLC->HA   Value (LREAL) published to state topic
//   1 binary_sensor PLC->HA   BVal (BOOL) published ON/OFF
//   2 switch        both      BVal published; cmd topic -> Cmd
//                             (1=ON 2=OFF 3=TOGGLE), RxVal, HasMsg,
//                             Event pulse; AutoWr latches BVal
//   3 number        both      Value published; cmd -> RxVal (clamped
//                             MinV..MaxV), AutoWr latches Value
//   4 button        HA->PLC   cmd topic -> Event pulse (momentary)
//   5 text          both      Txt published; cmd -> RxTxt, AutoWr
//   6 event         HA->PLC   subscribes ObjId as topic (leading '/'
//                             = absolute, else tether/<Dev>/<ObjId>);
//                             any message -> Event pulse + RxTxt
//
// Topics:  state  tether/<Dev>/<ObjId>/state
//          cmd    tether/<Dev>/<ObjId>/set
//          avail  tether/<Dev>/avail   (retained ON/OFF)
// HA MQTT discovery configs are published retained to
// homeassistant/<kind>/<Dev>/<ObjId>/config on every connect.
//
// Inputs (once per bridge): Dev (node/device id, also the HA
// device ids/name), Host, Port (0->1883 or 8883 for TLS),
// User/Pass/ClientId broker credentials ('' = anonymous/auto),
// ConnectMs (0->5000), MqttVer (3|5), Transport (0=auto tcp/ssl,
// 1=tcp, 2=ssl, 3=ws, 4=wss — WebSocket for cloud brokers such as
// AWS IoT / HiveMQ / EMQX; Host may carry a '/path' suffix for the
// WS upgrade path, default /mqtt), KeepAlive (0->60 s),
// Clean, TLS fields (UseTls/TlsInsecure/TlsCaFile/TlsCertFile/
// TlsKeyFile — ssl:// transport, Paho SSLOptions), last-will
// fields, BackoffMin/Max (ms, 0->service defaults 1000/30000),
// HbS heartbeat seconds (0=publish-on-change only).
//
// Outputs: Connected, Hdl, State (0 down/1 up), TxCnt/RxCnt,
// Err/ErrId.  NOTE: secure transport is MQTT over TLS (ssl://);
// wss:// is MQTT over WebSocket+TLS (Transport=4, Paho wss://).

// ---- string helpers ----

// LREAL -> "123.456" (3 decimals, no exponent)
FUNCTION F_HaRealStr : STRING[24]
VAR_INPUT
    f : LREAL;
END_VAR
VAR
    neg : BOOL;
    ip : DINT;
    frac : DINT;
    d1 : DINT;
    d2 : DINT;
    d3 : DINT;
    digs : STRING[10];
    s : STRING[24];
END_VAR
    neg := f < 0.0;
    IF neg THEN f := -f; END_IF;
    ip := LREAL_TO_DINT(f);
    frac := LREAL_TO_DINT((f - DINT_TO_LREAL(ip)) * 1000.0 + 0.5);
    IF frac > 999 THEN
        ip := ip + 1;
        frac := 0;
    END_IF;
    digs := '0123456789';
    // integer part
    s := '';
    IF ip = 0 THEN
        s := '0';
    ELSE
        WHILE ip > 0 DO
            s := CONCAT(MID(digs, 1, (ip MOD 10) + 1), s);
            ip := ip / 10;
        END_WHILE;
    END_IF;
    // fraction: exactly 3 digits
    d1 := frac / 100;
    d2 := (frac MOD 100) / 10;
    d3 := frac MOD 10;
    s := CONCAT(s, '.');
    s := CONCAT(s, MID(digs, 1, d1 + 1));
    s := CONCAT(s, MID(digs, 1, d2 + 1));
    s := CONCAT(s, MID(digs, 1, d3 + 1));
    IF neg THEN s := CONCAT('-', s); END_IF;
    F_HaRealStr := s;
END_FUNCTION

// STRING -> LREAL: optional sign, int + up to 6 frac digits,
// stops at first non-numeric; invalid -> 0.0
FUNCTION F_HaStrReal : LREAL
VAR_INPUT
    s : STRING[63];
END_VAR
VAR
    i : DINT;
    n : DINT;
    neg : BOOL;
    dot : BOOL;
    ip : LREAL;
    fp : LREAL;
    sc : LREAL;
    dig : DINT;
    c : STRING[1];
END_VAR
    F_HaStrReal := 0.0;
    n := LEN(s);
    i := 1;
    neg := FALSE;
    dot := FALSE;
    ip := 0.0;
    fp := 0.0;
    sc := 0.1;
    IF n >= 1 AND MID(s, 1, 1) = '-' THEN
        neg := TRUE;
        i := 2;
    END_IF;
    WHILE i <= n DO
        c := MID(s, 1, i);
        IF c = '.' AND NOT dot THEN
            dot := TRUE;
        ELSE
            dig := FIND('0123456789', c) - 1;
            IF dig < 0 THEN
                i := n + 1;
            ELSIF NOT dot THEN
                ip := ip * 10.0 + DINT_TO_LREAL(dig);
            ELSE
                fp := fp + DINT_TO_LREAL(dig) * sc;
                sc := sc * 0.1;
            END_IF;
        END_IF;
        i := i + 1;
    END_WHILE;
    F_HaStrReal := ip + fp;
    IF neg THEN F_HaStrReal := -F_HaStrReal; END_IF;
END_FUNCTION

// MQTT topic filter match: exact, '+' single level, '#' suffix.
// 'a/#' matches 'a/b', 'a/b/c'; '#' alone matches everything.
// ('a/#' does NOT match bare 'a' — edge case, documented.)
FUNCTION F_HaTopicMatch : BOOL
VAR_INPUT
    flt : STRING[127];
    topic : STRING[127];
END_VAR
VAR
    fi : DINT;
    ti : DINT;
    fn : DINT;
    tn : DINT;
    fc : STRING[1];
    tc : STRING[1];
    done : BOOL;
END_VAR
    F_HaTopicMatch := FALSE;
    fn := LEN(flt);
    tn := LEN(topic);
    fi := 1;
    ti := 1;
    done := FALSE;
    WHILE NOT done DO
        IF fi > fn THEN
            // filter exhausted: match iff topic exhausted too
            F_HaTopicMatch := ti > tn;
            done := TRUE;
        ELSIF ti > tn THEN
            done := TRUE;   // topic exhausted first -> no match
        ELSE
            fc := MID(flt, 1, fi);
            tc := MID(topic, 1, ti);
            IF fc = '#' THEN
                F_HaTopicMatch := TRUE;   // '#' matches the rest
                done := TRUE;
            ELSIF fc = '+' THEN
                // consume one topic level (up to next '/')
                WHILE ti <= tn AND MID(topic, 1, ti) <> '/' DO
                    ti := ti + 1;
                END_WHILE;
                fi := fi + 1;   // ti stays on '/' or past end
            ELSIF fc = tc THEN
                fi := fi + 1;
                ti := ti + 1;
            ELSE
                done := TRUE;
            END_IF;
        END_IF;
    END_WHILE;
END_FUNCTION

// ---- channel struct ----

// ST_HaChan — one HA entity channel inside FB_HaBridge.
// Config fields (user writes): En, Kind, ObjId, Name, Unit, DevCls,
// MinV, MaxV, StepV, Value, BVal, Txt, AutoWr.
// Runtime fields (bridge writes): HasMsg, MsgCnt, RxVal, RxTxt,
// Cmd, Event.
TYPE ST_HaChan : STRUCT
    En : BOOL;
    Kind : DINT;         // 0 sensor 1 binary 2 switch 3 number
                         // 4 button 5 text 6 event
    ObjId : STRING[31];
    Name : STRING[31];
    Unit : STRING[15];
    DevCls : STRING[31];
    MinV : LREAL;
    MaxV : LREAL;
    StepV : LREAL;
    AutoWr : BOOL;       // latch command value into published value
    Value : LREAL;       // published analog value (kinds 0,3)
    BVal : BOOL;         // published bool state (kinds 1,2)
    Txt : STRING[63];    // published text payload (kind 5)
    HasMsg : BOOL;       // runtime: message this scan
    MsgCnt : UDINT;      // runtime: messages received
    RxVal : LREAL;       // runtime: parsed numeric command
    RxTxt : STRING[63];  // runtime: received text/payload
    Cmd : DINT;          // runtime: switch cmd 1 ON 2 OFF 3 TOGGLE
    Event : BOOL;        // runtime: event pulse this scan
END_STRUCT
END_TYPE

// ======================= FB_HaBridge ==============================
FUNCTION_BLOCK FB_HaBridge
VAR_INPUT
    Execute : BOOL;
    CycleTime : LREAL;
    Dev : STRING[31];
    Host : STRING[63];
    Port : UDINT;
    User : STRING[63];        // broker username ('' = anonymous)
    Pass : STRING[127];       // broker password
    ClientId : STRING[63];    // '' = service auto id
    ConnectMs : UDINT;        // connect timeout ms; 0 -> 5000
    MqttVer : UDINT;          // 3 = MQTT 3.1.1, 5 = MQTT 5
    Transport : UDINT;       // 0=auto 1=tcp 2=ssl 3=ws 4=wss
    KeepAlive : UDINT;
    Clean : BOOL;
    UseTls : BOOL;
    TlsInsecure : BOOL;
    TlsCaFile : STRING[63];
    TlsCertFile : STRING[63];
    TlsKeyFile : STRING[63];
    WillTopic : STRING[127];
    WillMsg : STRING[127];
    WillQoS : UDINT;
    WillRetain : BOOL;
    BackoffMin : UDINT;   // ms; 0 -> service default
    BackoffMax : UDINT;   // ms; 0 -> service default
    HbS : UDINT;          // heartbeat seconds; 0 = change-only
END_VAR
VAR_OUTPUT
    Connected : BOOL;
    Err : BOOL;
    ErrId : UDINT;
    Hdl : UDINT;
    State : DINT;         // 0 down, 1 up
    TxCnt : UDINT;
    RxCnt : UDINT;
END_VAR
VAR
    CHAN : ARRAY[1..16] OF ST_HaChan;
    ch : FB_Mqtt;
    conPrev : BOOL;
    connUp : BOOL;
    upPrev : BOOL;
    hbT : LREAL;
    i : DINT;
    rxN : DINT;
    got : BOOL;
    forcePub : BOOL;
    topic : STRING[127];
    pay : STRING[255];
    j : STRING[255];
    statT : STRING[127];
    cmdT : STRING[127];
    evtT : STRING[127];
    cfgT : STRING[127];
    avT : STRING[127];
    objT : STRING[31];
    devT : STRING[31];
    nmT : STRING[31];
    untT : STRING[15];
    clsT : STRING[31];
    rs : STRING[24];
    subOk : ARRAY[1..16] OF BOOL;
    discOk : ARRAY[1..16] OF BOOL;
    lastInit : ARRAY[1..16] OF BOOL;
    lastV : ARRAY[1..16] OF LREAL;
    lastB : ARRAY[1..16] OF BOOL;
    lastT : ARRAY[1..16] OF STRING[63];
END_VAR
    Err := FALSE;
    ErrId := 0;
    // runtime pulses are 1-scan: clear every scan
    FOR i := 1 TO 16 DO
        CHAN[i].HasMsg := FALSE;
        CHAN[i].Event := FALSE;
    END_FOR;
    // ---- connection lifecycle (edge on Execute) ----
    IF Execute AND NOT conPrev THEN
        ch.OP := 1;
        ch.HOST := Host;
        IF Port > 0 THEN ch.PORT := Port; ELSE ch.PORT := 1883; END_IF;
        IF Clean THEN ch.CLEAN := 1; ELSE ch.CLEAN := 0; END_IF;
        IF KeepAlive > 0 THEN ch.KEEPALIVE := KeepAlive;
        ELSE ch.KEEPALIVE := 60; END_IF;
        IF LEN(WillTopic) > 0 THEN
            ch.WILLTOPIC := WillTopic;
            ch.WILLPAYLOAD := WillMsg;
            ch.WILLQOS := WillQoS;
            IF WillRetain THEN ch.WILLRETAIN := 1;
            ELSE ch.WILLRETAIN := 0; END_IF;
        END_IF;
        ch.USERNAME := User;
        ch.PASSWORD := Pass;
        ch.CLIENTID := ClientId;
        ch.CONNECTMS := ConnectMs;
        ch.MQTTVER := MqttVer;
        ch.TRANSPORT := Transport;
        IF UseTls THEN ch.USETLS := 1; ELSE ch.USETLS := 0; END_IF;
        IF TlsInsecure THEN ch.TLSINSECURE := 1;
        ELSE ch.TLSINSECURE := 0; END_IF;
        ch.TLSCAFILE := TlsCaFile;
        ch.TLSCERTFILE := TlsCertFile;
        ch.TLSKEYFILE := TlsKeyFile;
        ch();
        IF ch.STATUS = 0 THEN
            Hdl := ch.HANDLE;
            connUp := TRUE;
            // service-side exponential-backoff auto-reconnect
            ch.OP := 10;
            ch.HANDLE := Hdl;
            ch.BACKOFFMIN := BackoffMin;
            ch.BACKOFFMAX := BackoffMax;
            ch.BACKOFFEN := 1;
            ch();
        ELSE
            Err := TRUE;
            ErrId := ch.ERRID;
        END_IF;
    END_IF;
    IF NOT Execute AND conPrev THEN
        // graceful disconnect: avail OFF (retained) then DISCONNECT
        IF connUp THEN
            pay := 'OFF';
            avT := CONCAT('tether/', Dev);
            avT := CONCAT(avT, '/avail');
            ch.OP := 4;
            ch.HANDLE := Hdl;
            ch.TOPIC := avT;
            ch.PAYLOAD := pay;
            ch.QOS := 0;
            ch.FLG_RETAIN := 1;
            ch();
        END_IF;
        ch.OP := 2;
        ch.HANDLE := Hdl;
        ch();
        connUp := FALSE;
        Hdl := 0;
        Connected := FALSE;
        State := 0;
    END_IF;
    conPrev := Execute;
    IF NOT connUp THEN
        Connected := FALSE;
        State := 0;
    ELSE
        // ---- liveness poll (service auto-reconnects on drop) ----
        ch.OP := 8;
        ch.HANDLE := Hdl;
        ch();
        Connected := ch.CONNECTED <> 0;
        IF Connected THEN State := 1; ELSE State := 0; END_IF;
        IF Connected AND NOT upPrev THEN
            // (re)subscribe everything + republish discovery/avail
            FOR i := 1 TO 16 DO
                subOk[i] := FALSE;
                discOk[i] := FALSE;
            END_FOR;
            pay := 'ON';
            avT := CONCAT('tether/', Dev);
            avT := CONCAT(avT, '/avail');
            ch.OP := 4;
            ch.HANDLE := Hdl;
            ch.TOPIC := avT;
            ch.PAYLOAD := pay;
            ch.QOS := 0;
            ch.FLG_RETAIN := 1;
            ch();
            TxCnt := TxCnt + 1;
        END_IF;
        upPrev := Connected;
    END_IF;
    IF NOT (Execute AND connUp AND Connected) THEN
        RETURN;   // pulses already cleared above
    END_IF;
    // shared topic prefixes
    devT := Dev;
    avT := CONCAT('tether/', devT);
    avT := CONCAT(avT, '/avail');
    // ---- heartbeat ----
    hbT := hbT + CycleTime;
    forcePub := FALSE;
    IF HbS > 0 AND hbT >= DINT_TO_LREAL(HbS) THEN
        forcePub := TRUE;
        hbT := 0.0;
    END_IF;
    // ---- per-channel: topics, discovery, subscribe, publish ----
    FOR i := 1 TO 16 DO
        IF CHAN[i].En AND LEN(CHAN[i].ObjId) > 0 THEN
            objT := CHAN[i].ObjId;
            statT := CONCAT('tether/', devT);
            statT := CONCAT(statT, '/');
            statT := CONCAT(statT, objT);
            statT := CONCAT(statT, '/state');
            cmdT := CONCAT('tether/', devT);
            cmdT := CONCAT(cmdT, '/');
            cmdT := CONCAT(cmdT, objT);
            cmdT := CONCAT(cmdT, '/set');
            // event kind: leading '/' in ObjId = absolute topic
            evtT := '';
            IF CHAN[i].Kind = 6 THEN
                IF MID(objT, 1, 1) = '/' THEN
                    evtT := MID(objT, LEN(objT) - 1, 2);
                ELSE
                    evtT := CONCAT('tether/', devT);
                    evtT := CONCAT(evtT, '/');
                    evtT := CONCAT(evtT, objT);
                END_IF;
            END_IF;
            // subscribe command/event topics once per connect
            IF (CHAN[i].Kind >= 2 AND CHAN[i].Kind <= 6)
               AND NOT subOk[i] THEN
                topic := cmdT;
                IF CHAN[i].Kind = 6 THEN topic := evtT; END_IF;
                IF LEN(topic) > 0 THEN
                    ch.OP := 5;
                    ch.HANDLE := Hdl;
                    ch.TOPIC := topic;
                    ch.QOS := 0;
                    ch();
                    IF ch.STATUS = 0 THEN
                        subOk[i] := TRUE;
                    END_IF;
                END_IF;
            END_IF;
            // discovery config (retained) once per connect
            IF CHAN[i].Kind <= 5 AND NOT discOk[i] THEN
                nmT := CHAN[i].Name;
                untT := CHAN[i].Unit;
                clsT := CHAN[i].DevCls;
                j := '{"name":"';
                j := CONCAT(j, nmT);
                j := CONCAT(j, '"');
                CASE CHAN[i].Kind OF
                    0, 1, 2, 3, 5:
                        j := CONCAT(j, ',"stat_t":"');
                        j := CONCAT(j, statT);
                        j := CONCAT(j, '"');
                    ELSE
                END_CASE;
                IF CHAN[i].Kind = 2 OR CHAN[i].Kind = 3
                   OR CHAN[i].Kind = 4 OR CHAN[i].Kind = 5 THEN
                    j := CONCAT(j, ',"cmd_t":"');
                    j := CONCAT(j, cmdT);
                    j := CONCAT(j, '"');
                END_IF;
                IF CHAN[i].Kind = 0 OR CHAN[i].Kind = 3 THEN
                    IF LEN(untT) > 0 THEN
                        j := CONCAT(j, ',"unit_of_measurement":"');
                        j := CONCAT(j, untT);
                        j := CONCAT(j, '"');
                    END_IF;
                    IF LEN(clsT) > 0 THEN
                        j := CONCAT(j, ',"dev_cla":"');
                        j := CONCAT(j, clsT);
                        j := CONCAT(j, '"');
                    END_IF;
                    IF CHAN[i].MaxV > CHAN[i].MinV THEN
                        rs := F_HaRealStr(CHAN[i].MinV);
                        j := CONCAT(j, ',"min":');
                        j := CONCAT(j, rs);
                        rs := F_HaRealStr(CHAN[i].MaxV);
                        j := CONCAT(j, ',"max":');
                        j := CONCAT(j, rs);
                    END_IF;
                    IF CHAN[i].Kind = 3 AND CHAN[i].StepV > 0.0 THEN
                        rs := F_HaRealStr(CHAN[i].StepV);
                        j := CONCAT(j, ',"step":');
                        j := CONCAT(j, rs);
                    END_IF;
                END_IF;
                j := CONCAT(j, ',"avty_t":"');
                j := CONCAT(j, avT);
                j := CONCAT(j, '","dev":{"ids":["');
                j := CONCAT(j, devT);
                j := CONCAT(j, '"],"name":"');
                j := CONCAT(j, devT);
                j := CONCAT(j, '"},"uniq_id":"');
                j := CONCAT(j, devT);
                j := CONCAT(j, '-');
                j := CONCAT(j, objT);
                j := CONCAT(j, '"}');
                ch.OP := 4;
                ch.HANDLE := Hdl;
                // topic: homeassistant/<comp>/<dev>/<obj>/config
                topic := '';
                CASE CHAN[i].Kind OF
                    0: topic := 'sensor';
                    1: topic := 'binary_sensor';
                    2: topic := 'switch';
                    3: topic := 'number';
                    4: topic := 'button';
                    5: topic := 'text';
                END_CASE;
                cfgT := CONCAT('homeassistant/', topic);
                cfgT := CONCAT(cfgT, '/');
                cfgT := CONCAT(cfgT, devT);
                cfgT := CONCAT(cfgT, '/');
                cfgT := CONCAT(cfgT, objT);
                cfgT := CONCAT(cfgT, '/config');
                ch.TOPIC := cfgT;
                ch.PAYLOAD := j;
                ch.QOS := 0;
                ch.FLG_RETAIN := 1;
                ch();
                IF ch.STATUS = 0 THEN
                    discOk[i] := TRUE;
                    TxCnt := TxCnt + 1;
                END_IF;
            END_IF;
            // publish state on change or heartbeat (kinds 0,1,2,3,5)
            IF CHAN[i].Kind <= 5 AND CHAN[i].Kind <> 4 THEN
                got := FALSE;
                pay := '';
                CASE CHAN[i].Kind OF
                    0, 3:
                        pay := F_HaRealStr(CHAN[i].Value);
                        IF forcePub OR NOT lastInit[i]
                           OR CHAN[i].Value <> lastV[i] THEN
                            got := TRUE;
                        END_IF;
                        lastV[i] := CHAN[i].Value;
                    1, 2:
                        IF CHAN[i].BVal THEN pay := 'ON';
                        ELSE pay := 'OFF'; END_IF;
                        IF forcePub OR NOT lastInit[i]
                           OR CHAN[i].BVal <> lastB[i] THEN
                            got := TRUE;
                        END_IF;
                        lastB[i] := CHAN[i].BVal;
                    5:
                        pay := CHAN[i].Txt;
                        IF forcePub OR NOT lastInit[i]
                           OR CHAN[i].Txt <> lastT[i] THEN
                            got := TRUE;
                        END_IF;
                        lastT[i] := CHAN[i].Txt;
                END_CASE;
                IF got THEN
                    ch.OP := 4;
                    ch.HANDLE := Hdl;
                    ch.TOPIC := statT;
                    ch.PAYLOAD := pay;
                    ch.QOS := 0;
                    ch.FLG_RETAIN := 0;
                    ch();
                    IF ch.STATUS = 0 THEN
                        lastInit[i] := TRUE;
                        TxCnt := TxCnt + 1;
                    END_IF;
                END_IF;
            END_IF;
        END_IF;
    END_FOR;
    // ---- receive: up to 8 messages per scan, dispatch by topic ----
    rxN := 0;
    WHILE rxN < 8 DO
        ch.OP := 7;
        ch.HANDLE := Hdl;
        ch();
        IF ch.STATUS <> 0 THEN
            rxN := 8;
        ELSE
            topic := ch.RXTOPIC;
            pay := ch.RXPAYLOAD;
            got := FALSE;
            FOR i := 1 TO 16 DO
                IF CHAN[i].En AND CHAN[i].Kind >= 2
                   AND CHAN[i].Kind <= 6 THEN
                    // rebuild this channel's subscribe topic
                    objT := CHAN[i].ObjId;
                    cmdT := CONCAT('tether/', devT);
                    cmdT := CONCAT(cmdT, '/');
                    cmdT := CONCAT(cmdT, objT);
                    cmdT := CONCAT(cmdT, '/set');
                    evtT := '';
                    IF CHAN[i].Kind = 6 THEN
                        IF MID(objT, 1, 1) = '/' THEN
                            evtT := MID(objT, LEN(objT) - 1, 2);
                        ELSE
                            evtT := CONCAT('tether/', devT);
                            evtT := CONCAT(evtT, '/');
                            evtT := CONCAT(evtT, objT);
                        END_IF;
                    END_IF;
                    IF (CHAN[i].Kind < 6
                        AND F_HaTopicMatch(cmdT, topic))
                       OR (CHAN[i].Kind = 6
                        AND F_HaTopicMatch(evtT, topic)) THEN
                        got := TRUE;
                        CHAN[i].MsgCnt := CHAN[i].MsgCnt + 1;
                        CHAN[i].RxTxt := LEFT(pay, 63);
                        CHAN[i].Event := TRUE;
                        CHAN[i].HasMsg := TRUE;
                        CHAN[i].Cmd := 0;
                        CASE CHAN[i].Kind OF
                            2: // switch
                                IF LEFT(pay, 2) = 'ON'
                                   OR LEFT(pay, 1) = '1' THEN
                                    CHAN[i].Cmd := 1;
                                    CHAN[i].RxVal := 1.0;
                                ELSIF LEFT(pay, 3) = 'OFF'
                                      OR LEFT(pay, 1) = '0' THEN
                                    CHAN[i].Cmd := 2;
                                    CHAN[i].RxVal := 0.0;
                                ELSIF LEFT(pay, 6) = 'TOGGLE' THEN
                                    CHAN[i].Cmd := 3;
                                    CHAN[i].RxVal := -1.0;
                                END_IF;
                                IF CHAN[i].AutoWr AND CHAN[i].Cmd > 0 THEN
                                    IF CHAN[i].Cmd = 1 THEN
                                        CHAN[i].BVal := TRUE;
                                    ELSIF CHAN[i].Cmd = 2 THEN
                                        CHAN[i].BVal := FALSE;
                                    ELSE
                                        CHAN[i].BVal := NOT CHAN[i].BVal;
                                    END_IF;
                                END_IF;
                            3: // number
                                CHAN[i].RxVal := F_HaStrReal(pay);
                                IF CHAN[i].RxVal < CHAN[i].MinV THEN
                                    CHAN[i].RxVal := CHAN[i].MinV;
                                END_IF;
                                IF CHAN[i].RxVal > CHAN[i].MaxV
                                   AND CHAN[i].MaxV > CHAN[i].MinV THEN
                                    CHAN[i].RxVal := CHAN[i].MaxV;
                                END_IF;
                                IF CHAN[i].AutoWr THEN
                                    CHAN[i].Value := CHAN[i].RxVal;
                                END_IF;
                            5: // text
                                IF CHAN[i].AutoWr THEN
                                    CHAN[i].Txt := LEFT(pay, 63);
                                END_IF;
                        END_CASE;
                    END_IF;
                END_IF;
            END_FOR;
            IF got THEN
                RxCnt := RxCnt + 1;
            END_IF;
            rxN := rxN + 1;
        END_IF;
    END_WHILE;
END_FUNCTION_BLOCK
