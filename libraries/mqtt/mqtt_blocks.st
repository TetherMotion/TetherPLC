// mqtt_blocks.st — easy-to-wire FBD blocks over the raw FB_Mqtt
// primitive.  One FB_MqttLink owns a broker connection (all connect
// options as pins, incl. credentials + ws/wss transport); FB_MqttTx,
// FB_MqttRx, FB_MqttSub and FB_MqttUnsub share it through the Hdl
// output — wire Hdl into each op block's Hdl input.
//
// FB_MqttLink semantics: Execute is a RUN LEVEL — rising edge issues
// the (blocking) connect, falling edge disconnects.  While Execute
// stays high the service-side exponential-backoff watchdog
// (BackoffMin/Max) auto-reconnects drops.  Err is level: TRUE after
// a failed connect attempt until the next attempt or Execute low.
//
// For MQTT5 connect/publish/subscribe property pins use the raw
// FB_Mqtt builtin directly (M5* members) — these blocks cover the
// common IEC-style surface.

// ======================= FB_MqttLink ==============================
FUNCTION_BLOCK FB_MqttLink
VAR_INPUT
    Execute : BOOL;         // run level: rise=connect, fall=disconnect
    Host : STRING[63];      // ws/wss may append '/path' (def /mqtt)
    Port : UDINT;           // 0 -> scheme default
    User : STRING[63];
    Pass : STRING[127];
    ClientId : STRING[63];  // '' = service auto id
    ConnectMs : UDINT;      // 0 -> 5000
    MqttVer : UDINT;        // 3 = 3.1.1, 5 = MQTT5
    Transport : UDINT;      // 0 auto,1 tcp,2 ssl,3 ws,4 wss
    KeepAlive : UDINT;      // 0 -> 60 s
    Clean : BOOL;
    UseTls : BOOL;
    TlsInsecure : BOOL;
    TlsCaFile : STRING[63];
    TlsCertFile : STRING[63];
    TlsKeyFile : STRING[63];
    WillTopic : STRING[127];
    WillMsg : STRING[255];
    WillQoS : UDINT;
    WillRetain : BOOL;
    BackoffMin : UDINT;     // ms; 0 -> service default 1000
    BackoffMax : UDINT;     // ms; 0 -> service default 30000
END_VAR
VAR_OUTPUT
    Connected : BOOL;
    Err : BOOL;
    ErrId : UDINT;
    Hdl : UDINT;
    State : DINT;           // 0 down, 1 up
END_VAR
VAR
    ch : FB_Mqtt;
    exPrev : BOOL;
    connUp : BOOL;
END_VAR
    ErrId := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 1;
        ch.HOST := Host;
        ch.PORT := Port;
        ch.USERNAME := User;
        ch.PASSWORD := Pass;
        ch.CLIENTID := ClientId;
        ch.CONNECTMS := ConnectMs;
        ch.MQTTVER := MqttVer;
        ch.TRANSPORT := Transport;
        IF KeepAlive > 0 THEN ch.KEEPALIVE := KeepAlive;
        ELSE ch.KEEPALIVE := 60; END_IF;
        IF Clean THEN ch.CLEAN := 1; ELSE ch.CLEAN := 0; END_IF;
        IF LEN(WillTopic) > 0 THEN
            ch.WILLTOPIC := WillTopic;
            ch.WILLPAYLOAD := WillMsg;
            ch.WILLQOS := WillQoS;
            IF WillRetain THEN ch.WILLRETAIN := 1;
            ELSE ch.WILLRETAIN := 0; END_IF;
        END_IF;
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
            Err := FALSE;
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
            connUp := FALSE;
            Hdl := 0;
        END_IF;
    END_IF;
    IF NOT Execute AND exPrev THEN
        IF connUp THEN
            ch.OP := 2;
            ch.HANDLE := Hdl;
            ch();
        END_IF;
        connUp := FALSE;
        Hdl := 0;
        Connected := FALSE;
        State := 0;
        Err := FALSE;
    END_IF;
    exPrev := Execute;
    IF connUp THEN
        // liveness poll; service watchdog reconnects on drop
        ch.OP := 8;
        ch.HANDLE := Hdl;
        ch();
        Connected := ch.CONNECTED <> 0;
        IF Connected THEN State := 1; ELSE State := 0; END_IF;
    ELSE
        Connected := FALSE;
        State := 0;
    END_IF;
END_FUNCTION_BLOCK

// ======================= FB_MqttTx ===============================
// Publish one message per Execute rising edge.  Done/Err are
// 1-scan pulses; STATUS carries the service return code.
FUNCTION_BLOCK FB_MqttTx
VAR_INPUT
    Execute : BOOL;
    Hdl : UDINT;            // from FB_MqttLink.Hdl
    Topic : STRING[127];
    Payload : STRING[255];
    Qos : UDINT;            // 0..2
    Retained : BOOL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Err : BOOL;
    ErrId : UDINT;
END_VAR
VAR
    ch : FB_Mqtt;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Err := FALSE;
    ErrId := 0;
    IF Execute AND NOT exPrev AND Hdl > 0 THEN
        ch.OP := 4;
        ch.HANDLE := Hdl;
        ch.TOPIC := Topic;
        ch.PAYLOAD := Payload;
        ch.QOS := Qos;
        IF Retained THEN ch.FLG_RETAIN := 1;
        ELSE ch.FLG_RETAIN := 0; END_IF;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Err := TRUE;
            ErrId := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ======================= FB_MqttRx ===============================
// Polls the broker once per scan; HasMsg is a 1-scan pulse carrying
// the message fields.  Wire HasMsg to whatever should react.
FUNCTION_BLOCK FB_MqttRx
VAR_INPUT
    Hdl : UDINT;            // from FB_MqttLink.Hdl; 0 = disabled
END_VAR
VAR_OUTPUT
    HasMsg : BOOL;
    Topic : STRING[127];
    Payload : STRING[255];
    Qos : UDINT;
    Retained : UDINT;
    MsgId : UDINT;
    MsgCnt : UDINT;
    Err : BOOL;
    ErrId : UDINT;
END_VAR
VAR
    ch : FB_Mqtt;
END_VAR
    HasMsg := FALSE;
    Err := FALSE;
    ErrId := 0;
    IF Hdl > 0 THEN
        ch.OP := 7;
        ch.HANDLE := Hdl;
        ch();
        IF ch.STATUS = 0 THEN
            HasMsg := TRUE;
            Topic := ch.RXTOPIC;
            Payload := ch.RXPAYLOAD;
            Qos := ch.RXQOS;
            Retained := ch.RXRETAIN;
            MsgId := ch.RXMSGID;
            MsgCnt := MsgCnt + 1;
        ELSIF ch.STATUS < 0 THEN
            Err := TRUE;
            ErrId := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ======================= FB_MqttSub ===============================
// Subscribe to a topic filter once per Execute rising edge.
FUNCTION_BLOCK FB_MqttSub
VAR_INPUT
    Execute : BOOL;
    Hdl : UDINT;
    Filter : STRING[127];   // supports '+' and '#' wildcards
    Qos : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Err : BOOL;
    ErrId : UDINT;
END_VAR
VAR
    ch : FB_Mqtt;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Err := FALSE;
    ErrId := 0;
    IF Execute AND NOT exPrev AND Hdl > 0 THEN
        ch.OP := 5;
        ch.HANDLE := Hdl;
        ch.TOPIC := Filter;
        ch.QOS := Qos;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Err := TRUE;
            ErrId := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ======================= FB_MqttUnsub =============================
// Unsubscribe from a topic filter once per Execute rising edge.
FUNCTION_BLOCK FB_MqttUnsub
VAR_INPUT
    Execute : BOOL;
    Hdl : UDINT;
    Filter : STRING[127];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Err : BOOL;
    ErrId : UDINT;
END_VAR
VAR
    ch : FB_Mqtt;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Err := FALSE;
    ErrId := 0;
    IF Execute AND NOT exPrev AND Hdl > 0 THEN
        ch.OP := 6;
        ch.HANDLE := Hdl;
        ch.TOPIC := Filter;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Err := TRUE;
            ErrId := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK
