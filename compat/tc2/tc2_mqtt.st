// tc2_mqtt.st — TwinCAT MQTT/IoT client surface (Tc3_IoT/TF6701
// shape) over the tplc-mqtt service + FB_Mqtt builtin.
//
// The PLC is the client over MQTT 3.1.1 via libpaho-mqtt3c:
// FB_IotMqttClient runs CONNECT/CONNACK; FB_IotMqttPublish carries
// PUBLISH/PUBACK; FB_IotMqttSubscribe/Unsubscribe run
// SUBSCRIBE/SUBACK and UNSUBSCRIBE/UNSUBACK; FB_IotMqttReceive polls
// for a pending publish each scan (Retained reports the broker's
// retain flag on delivery); Disconnect releases.  F_MqttTopicMatch
// implements the MQTT 3.1.1 '+'/'#' filter semantics client-side.
// MQTT5 properties, TLS, last-will, and QoS2 two-phase handshake
// detail are out of scope for this tranche.
//
// Topics are bounded STRING[127], payloads STRING[255].  STATUS
// carries the service return code, ErrorID its positive form;
// receive returns TPLC_MQTT_E_NOMSG when nothing is pending.

// ---- FB_IotMqttClient: CONNECT/CONNACK ----
FUNCTION_BLOCK FB_IotMqttClient
VAR_INPUT
    Execute : BOOL;
    HostName : STRING[63];
    Port : UDINT;
    CleanSession : BOOL;
    KeepAlive : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    ConnectionHdl : UDINT;
END_VAR
VAR
    ch : FB_Mqtt;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 1;
        ch.HOST := HostName;
        ch.PORT := Port;
        IF CleanSession THEN ch.CLEAN := 1; ELSE ch.CLEAN := 0; END_IF;
        ch.KEEPALIVE := KeepAlive;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            ConnectionHdl := ch.HANDLE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_IotMqttDisconnect: DISCONNECT + release ----
FUNCTION_BLOCK FB_IotMqttDisconnect
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Mqtt;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 2;
        ch.HANDLE := ConnectionHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
    Busy := FALSE;
END_FUNCTION_BLOCK

// ---- FB_IotMqttPublish: PUBLISH/PUBACK ----
FUNCTION_BLOCK FB_IotMqttPublish
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    Topic : STRING[127];
    Payload : STRING[255];
    QoS : UDINT;
    RetainOn : BOOL;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Mqtt;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 4;
        ch.HANDLE := ConnectionHdl;
        ch.TOPIC := Topic;
        ch.PAYLOAD := Payload;
        ch.QOS := QoS;
        IF RetainOn THEN ch.FLG_RETAIN := 1; ELSE ch.FLG_RETAIN := 0; END_IF;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_IotMqttSubscribe: SUBSCRIBE/SUBACK ----
FUNCTION_BLOCK FB_IotMqttSubscribe
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    TopicFilter : STRING[127];
    QoS : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Mqtt;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 5;
        ch.HANDLE := ConnectionHdl;
        ch.TOPIC := TopicFilter;
        ch.QOS := QoS;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_IotMqttUnsubscribe: UNSUBSCRIBE/UNSUBACK ----
FUNCTION_BLOCK FB_IotMqttUnsubscribe
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    TopicFilter : STRING[127];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Mqtt;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 6;
        ch.HANDLE := ConnectionHdl;
        ch.TOPIC := TopicFilter;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_IotMqttReceive: poll for a pending publish ----
FUNCTION_BLOCK FB_IotMqttReceive
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Message : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Topic : STRING[127];
    Payload : STRING[255];
    PayloadLen : UDINT;
    QoS : UDINT;
    MessageId : UDINT;
    Retained : BOOL;
END_VAR
VAR
    ch : FB_Mqtt;
END_VAR
    Done := FALSE;
    Message := FALSE;
    Error := FALSE;
    ErrorID := 0;
    Retained := FALSE;
    IF Execute THEN
        ch.OP := 7;
        ch.HANDLE := ConnectionHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            Message := TRUE;
            Topic := ch.RXTOPIC;
            Payload := ch.RXPAYLOAD;
            PayloadLen := ch.RXPLEN;
            QoS := ch.RXQOS;
            MessageId := ch.RXMSGID;
            Retained := ch.RXRETAIN <> 0;
        ELSIF ch.STATUS = 1 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- F_MqttTopicMatch: MQTT 3.1.1 topic-filter match ----
// '+' matches exactly one topic level, '#' matches the remaining
// levels (must be the last filter level).  Topic names and filters
// are bounded STRING[255]; matching is case-sensitive per spec.
// NOTE: no early RETURN (ST FUNCTION return semantics) — a match
// flag drives a single exit.
FUNCTION F_MqttTopicMatch : BOOL
VAR_INPUT
    Filter : STRING[255];
    Topic : STRING[255];
END_VAR
VAR
    f : STRING[255];
    t : STRING[255];
    fs : UDINT;
    ft : UDINT;
    fseg : STRING[255];
    tseg : STRING[255];
    fend : BOOL;
    match : BOOL;
END_VAR
    f := Filter;
    t := Topic;
    fend := FALSE;
    match := TRUE;
    WHILE NOT fend AND match DO
        IF LEN(f) = 0 THEN
            fend := TRUE;
        ELSE
            fs := FIND(f, '/');
            IF fs = 0 THEN
                fseg := f;
                f := '';
            ELSE
                fseg := LEFT(f, fs - 1);
                f := MID(f, LEN(f) - fs, fs + 1);
            END_IF;
            // '#' terminates the filter and matches the rest
            IF LEN(fseg) = 1 AND FIND(fseg, '#') = 1 THEN
                fend := TRUE;
                t := '';
            ELSE
                // consume the matching topic level
                IF LEN(t) = 0 THEN
                    tseg := '';
                ELSE
                    ft := FIND(t, '/');
                    IF ft = 0 THEN
                        tseg := t;
                        t := '';
                    ELSE
                        tseg := LEFT(t, ft - 1);
                        t := MID(t, LEN(t) - ft, ft + 1);
                    END_IF;
                END_IF;
                // '+' matches any single level
                IF LEN(fseg) = 1 AND FIND(fseg, '+') = 1 THEN
                    // matched; continue to next filter level
                ELSIF LEN(fseg) = 0 AND LEN(tseg) = 0 THEN
                    // empty==empty
                ELSIF LEN(fseg) > 0 AND LEN(tseg) > 0
                      AND LEN(fseg) = LEN(tseg) AND FIND(fseg, tseg) = 1 THEN
                    // literal level match
                ELSE
                    match := FALSE;
                END_IF;
            END_IF;
        END_IF;
    END_WHILE;
    // filter done: match only if topic is also exhausted
    F_MqttTopicMatch := match AND LEN(t) = 0;
END_FUNCTION
