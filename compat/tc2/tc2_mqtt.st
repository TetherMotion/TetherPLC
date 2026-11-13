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
// MQTT5 properties and QoS2 two-phase handshake detail are out of
// scope for this tranche.  Last-will is supported: set
// WillTopic/WillPayload on FB_IotMqttClient and the broker publishes
// the will when the link drops ungracefully — FB_IotMqttAbort drops
// the TCP connection without DISCONNECT to trigger exactly that.
// TLS is supported via UseTls + TlsCaFile/TlsCertFile/TlsKeyFile
// (PEM paths); TlsInsecure skips server certificate verification
// for self-signed test brokers.
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
    WillTopic : STRING[127];
    WillPayload : STRING[255];
    WillQoS : UDINT;
    WillRetain : BOOL;
    UseTls : BOOL;
    TlsInsecure : BOOL;
    TlsCaFile : STRING[127];
    TlsCertFile : STRING[127];
    TlsKeyFile : STRING[127];
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
        ch.WILLTOPIC := WillTopic;
        ch.WILLPAYLOAD := WillPayload;
        ch.WILLQOS := WillQoS;
        IF WillRetain THEN ch.WILLRETAIN := 1; ELSE ch.WILLRETAIN := 0; END_IF;
        IF UseTls THEN ch.USETLS := 1; ELSE ch.USETLS := 0; END_IF;
        IF TlsInsecure THEN ch.TLSINSECURE := 1; ELSE ch.TLSINSECURE := 0; END_IF;
        ch.TLSCAFILE := TlsCaFile;
        ch.TLSCERTFILE := TlsCertFile;
        ch.TLSKEYFILE := TlsKeyFile;
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

// ---- FB_IotMqttAbort: drop TCP without DISCONNECT (fires will) ----
FUNCTION_BLOCK FB_IotMqttAbort
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
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
        ch.OP := 9;
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

// ---- FB_IotMqttBackoff: Activate/DeactivateExponentialBackoff ----
// TwinCAT's ActivateExponentialBackoff(min,max) makes the client
// auto-reconnect with a doubling delay bounded by max; Deactivate
// stops it.  Enable=TRUE on the Execute edge activates with
// MinMs/MaxMs; Enable=FALSE deactivates.  Reconnects re-issue the
// original CONNECT (host/port/TLS/will); subscriptions are NOT
// restored — resubscribe after the connection returns.
FUNCTION_BLOCK FB_IotMqttBackoff
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    MinMs : UDINT;
    MaxMs : UDINT;
    Enable : BOOL;
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
        ch.OP := 10;
        ch.HANDLE := ConnectionHdl;
        ch.BACKOFFMIN := MinMs;
        ch.BACKOFFMAX := MaxMs;
        IF Enable THEN ch.BACKOFFEN := 1; ELSE ch.BACKOFFEN := 0; END_IF;
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

// ---- FB_IotMqttStatus: session status + broker-message age ----
// Level-triggered: while Execute is TRUE it polls the connection
// status each scan.  TimeSinceLastBrokerMsgMs is the
// GetTimeSinceLastBrokerMessage() equivalent — milliseconds since
// the last inbound broker message (PUBLISH delivery or connect;
// transport-level pings are not surfaced by the sync client).
FUNCTION_BLOCK FB_IotMqttStatus
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
END_VAR
VAR_OUTPUT
    Connected : BOOL;
    State : DINT;
    LastMsgId : UDINT;
    LastQos : UDINT;
    LastError : DINT;
    TimeSinceLastBrokerMsgMs : UDINT;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Mqtt;
END_VAR
    Connected := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 3;
        ch.HANDLE := ConnectionHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Connected := ch.STATE = 2;
            State := ch.STATE;
            LastMsgId := ch.LASTMSGID;
            LastError := ch.LASTERR;
            TimeSinceLastBrokerMsgMs := ch.LASTMSGSINCE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_IotMqttMessage: one queued MQTT message ----
// TwinCAT exposes GetTopic/GetPayload/CompareTopic methods; this
// dialect surfaces the same data as members.
FUNCTION_BLOCK FB_IotMqttMessage
VAR
    TOPIC : STRING[127];
    PAYLOAD : STRING[255];
    PAYLOADLEN : UDINT;
    QOS : UDINT;
    MSGID : UDINT;
    RETAINED : BOOL;
END_VAR
END_FUNCTION_BLOCK

// ---- FB_IotMqttMessageQueue: bounded FIFO of inbound messages ----
// TwinCAT's queue collects messages in the background and Dequeue
// transfers the oldest into an FB_IotMqttMessage.  This dialect has
// no REFERENCE TO inputs, so the popped message appears on the Msg*
// outputs instead.  Pump is level-triggered: each scan it drains up
// to 8 pending messages (bounded, watchdog-safe) into a 16-entry
// ring; DequeueReq/ResetReq are edge-triggered.
FUNCTION_BLOCK FB_IotMqttMessageQueue
VAR_INPUT
    Pump : BOOL;
    ConnectionHdl : UDINT;
    DequeueReq : BOOL;
    ResetReq : BOOL;
END_VAR
VAR_OUTPUT
    HasMessage : BOOL;
    Topic : STRING[127];
    Payload : STRING[255];
    PayloadLen : UDINT;
    QoS : UDINT;
    MessageId : UDINT;
    Retained : BOOL;
    Queued : UDINT;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    head : UDINT;
    count : UDINT;
    qTopic : ARRAY[0..15] OF STRING[127];
    qPayload : ARRAY[0..15] OF STRING[255];
    qPayLen : ARRAY[0..15] OF UDINT;
    qQos : ARRAY[0..15] OF UDINT;
    qId : ARRAY[0..15] OF UDINT;
    qRet : ARRAY[0..15] OF BOOL;
    rx : FB_IotMqttReceive;
    slot : UDINT;
    dqPrev : BOOL;
    rsPrev : BOOL;
END_VAR
    HasMessage := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Pump AND count < 16 THEN
        // one nonblocking receive per scan keeps the pump bounded
        rx.Execute := TRUE;
        rx.ConnectionHdl := ConnectionHdl;
        rx();
        rx.Execute := FALSE;
        IF rx.Message THEN
            slot := head + count;
            IF slot >= 16 THEN slot := slot - 16; END_IF;
            qTopic[slot] := rx.Topic;
            qPayload[slot] := rx.Payload;
            qPayLen[slot] := rx.PayloadLen;
            qQos[slot] := rx.QoS;
            qId[slot] := rx.MessageId;
            qRet[slot] := rx.Retained;
            count := count + 1;
        END_IF;
        IF rx.Error THEN
            Error := TRUE;
            ErrorID := rx.ErrorID;
        END_IF;
    END_IF;
    Queued := count;
    // dequeue: pop the oldest message into the Msg* outputs
    IF DequeueReq AND NOT dqPrev AND count > 0 THEN
        Topic := qTopic[head];
        Payload := qPayload[head];
        PayloadLen := qPayLen[head];
        QoS := qQos[head];
        MessageId := qId[head];
        Retained := qRet[head];
        HasMessage := TRUE;
        head := head + 1;
        IF head >= 16 THEN head := 0; END_IF;
        count := count - 1;
        Queued := count;
    END_IF;
    dqPrev := DequeueReq;
    // reset: drop every queued message
    IF ResetReq AND NOT rsPrev THEN
        head := 0;
        count := 0;
        Queued := 0;
    END_IF;
    rsPrev := ResetReq;
END_FUNCTION_BLOCK

// ---- FB_IotMqtt5Client: MQTT 5.0 CONNECT/CONNACK ----
// Same surface as FB_IotMqttClient plus the MQTT5 connect
// properties.  Publish/subscribe/unsubscribe reuse the existing
// FBs; the service picks the MQTT5 packet forms automatically for
// a v5 connection.
FUNCTION_BLOCK FB_IotMqtt5Client
VAR_INPUT
    Execute : BOOL;
    HostName : STRING[63];
    Port : UDINT;
    CleanSession : BOOL;
    KeepAlive : UDINT;
    WillTopic : STRING[127];
    WillPayload : STRING[255];
    WillQoS : UDINT;
    WillRetain : BOOL;
    UseTls : BOOL;
    TlsInsecure : BOOL;
    TlsCaFile : STRING[127];
    TlsCertFile : STRING[127];
    TlsKeyFile : STRING[127];
    SessionExpiryS : UDINT;
    ReceiveMaximum : UDINT;
    TopicAliasMaximum : UDINT;
    RequestResponseInformation : BOOL;
    RequestProblemInformation : BOOL;
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
        ch.WILLTOPIC := WillTopic;
        ch.WILLPAYLOAD := WillPayload;
        ch.WILLQOS := WillQoS;
        IF WillRetain THEN ch.WILLRETAIN := 1; ELSE ch.WILLRETAIN := 0; END_IF;
        IF UseTls THEN ch.USETLS := 1; ELSE ch.USETLS := 0; END_IF;
        IF TlsInsecure THEN ch.TLSINSECURE := 1; ELSE ch.TLSINSECURE := 0; END_IF;
        ch.TLSCAFILE := TlsCaFile;
        ch.TLSCERTFILE := TlsCertFile;
        ch.TLSKEYFILE := TlsKeyFile;
        ch.MQTTVER := 5;
        ch.M5SESSIONEXP := SessionExpiryS;
        ch.M5RECVMAX := ReceiveMaximum;
        ch.M5TOPICALIAS := TopicAliasMaximum;
        IF RequestResponseInformation THEN ch.M5REQRESP := 1; ELSE ch.M5REQRESP := 0; END_IF;
        IF RequestProblemInformation THEN ch.M5REQPROB := 1; ELSE ch.M5REQPROB := 0; END_IF;
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

// ---- FB_IotMqtt5Publish: MQTT5 PUBLISH with properties ----
// v5 connections only.  Property inputs map to
// TplcMqtt5PubProps; empty inputs leave the property absent.
// UserProperties is a "k1=v1;k2=v2" list.
FUNCTION_BLOCK FB_IotMqtt5Publish
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    Topic : STRING[127];
    Payload : STRING[255];
    QoS : UDINT;
    RetainOn : BOOL;
    PayloadFormatUtf8 : BOOL;
    MessageExpiryS : UDINT;
    ContentType : STRING[127];
    ResponseTopic : STRING[127];
    CorrelationData : STRING[127];
    UserProperties : STRING[255];
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
        ch.OP := 11;
        ch.HANDLE := ConnectionHdl;
        ch.TOPIC := Topic;
        ch.PAYLOAD := Payload;
        ch.QOS := QoS;
        IF RetainOn THEN ch.FLG_RETAIN := 1; ELSE ch.FLG_RETAIN := 0; END_IF;
        IF PayloadFormatUtf8 THEN ch.M5PUBFMT := 1; ELSE ch.M5PUBFMT := 0; END_IF;
        ch.M5PUBEXPIRY := MessageExpiryS;
        ch.M5PUBCTYPE := ContentType;
        ch.M5PUBRTOPIC := ResponseTopic;
        ch.M5PUBCORR := CorrelationData;
        ch.M5PUBCORR_LEN := LEN(CorrelationData);
        ch.M5PUBUPROPS := UserProperties;
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

// ---- FB_IotMqtt5Subscribe: MQTT5 SUBSCRIBE with properties ----
// v5 connections only.  NoLocal suppresses echoes of own
// publishes; RetainHandling: 0=always send retained, 1=new
// subscriptions only, 2=never.  SubscriptionId tags delivered
// messages; UserProperties is a "k1=v1;k2=v2" list.
FUNCTION_BLOCK FB_IotMqtt5Subscribe
VAR_INPUT
    Execute : BOOL;
    ConnectionHdl : UDINT;
    TopicFilter : STRING[127];
    QoS : UDINT;
    NoLocal : BOOL;
    RetainAsPublished : BOOL;
    RetainHandling : UDINT;
    SubscriptionId : UDINT;
    UserProperties : STRING[255];
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
        ch.OP := 12;
        ch.HANDLE := ConnectionHdl;
        ch.TOPIC := TopicFilter;
        ch.QOS := QoS;
        IF NoLocal THEN ch.M5SUBNOLOCAL := 1; ELSE ch.M5SUBNOLOCAL := 0; END_IF;
        IF RetainAsPublished THEN ch.M5SUBRAP := 1; ELSE ch.M5SUBRAP := 0; END_IF;
        ch.M5SUBRH := RetainHandling;
        ch.M5SUBSID := SubscriptionId;
        ch.M5SUBUPROPS := UserProperties;
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

// ---- FB_IotMqtt5Receive: MQTT5 PUBLISH receive with properties ----
// v5 connections only.  Same poll semantics as FB_IotMqttReceive
// plus the PUBLISH property outputs: PayloadFormatUtf8,
// MessageExpiryS, SubscriptionId, ContentType, ResponseTopic,
// CorrelationData (STRING, binary-safe via LEN), UserProperties
// ("k=v;k=v" list).
FUNCTION_BLOCK FB_IotMqtt5Receive
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
    PayloadFormatUtf8 : BOOL;
    MessageExpiryS : UDINT;
    SubscriptionId : UDINT;
    ContentType : STRING[63];
    ResponseTopic : STRING[127];
    CorrelationData : STRING[127];
    CorrelationLen : UDINT;
    UserProperties : STRING[255];
END_VAR
VAR
    ch : FB_Mqtt;
END_VAR
    Done := FALSE;
    Message := FALSE;
    Error := FALSE;
    ErrorID := 0;
    Retained := FALSE;
    PayloadFormatUtf8 := FALSE;
    MessageExpiryS := 0;
    SubscriptionId := 0;
    CorrelationLen := 0;
    IF Execute THEN
        ch.OP := 13;
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
            PayloadFormatUtf8 := ch.M5RXFMT <> 0;
            MessageExpiryS := ch.M5RXEXPIRY;
            SubscriptionId := ch.M5RXSUBID;
            ContentType := ch.M5RXCTYPE;
            ResponseTopic := ch.M5RXRTOPIC;
            CorrelationData := ch.M5RXCORR;
            CorrelationLen := ch.M5RXCORR_LEN;
            UserProperties := ch.M5RXUPROPS;
        ELSIF ch.STATUS = 1 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_IotMqtt5Message: one queued MQTT5 message ----
// TwinCAT's FB_IotMqtt5Message carries the PUBLISH property block
// alongside topic/payload; this dialect surfaces them as members.
FUNCTION_BLOCK FB_IotMqtt5Message
VAR
    TOPIC : STRING[127];
    PAYLOAD : STRING[255];
    PAYLOADLEN : UDINT;
    QOS : UDINT;
    MSGID : UDINT;
    RETAINED : BOOL;
    PAYLOADFORMATUTF8 : BOOL;
    MESSAGEEXPIRYS : UDINT;
    SUBSCRIPTIONID : UDINT;
    CONTENTTYPE : STRING[63];
    RESPONSETOPIC : STRING[127];
    CORRELATIONDATA : STRING[127];
    CORRELATIONLEN : UDINT;
    USERPROPERTIES : STRING[255];
END_VAR
END_FUNCTION_BLOCK

// ---- FB_IotMqtt5MessageQueue: bounded FIFO of inbound v5 msgs ----
// Same pump/dequeue model as FB_IotMqttMessageQueue but drains via
// FB_IotMqtt5Receive so queued entries keep the PUBLISH properties.
FUNCTION_BLOCK FB_IotMqtt5MessageQueue
VAR_INPUT
    Pump : BOOL;
    ConnectionHdl : UDINT;
    DequeueReq : BOOL;
    ResetReq : BOOL;
END_VAR
VAR_OUTPUT
    HasMessage : BOOL;
    Topic : STRING[127];
    Payload : STRING[255];
    PayloadLen : UDINT;
    QoS : UDINT;
    MessageId : UDINT;
    Retained : BOOL;
    PayloadFormatUtf8 : BOOL;
    MessageExpiryS : UDINT;
    SubscriptionId : UDINT;
    ContentType : STRING[63];
    ResponseTopic : STRING[127];
    CorrelationData : STRING[127];
    CorrelationLen : UDINT;
    UserProperties : STRING[255];
    Queued : UDINT;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    head : UDINT;
    count : UDINT;
    qTopic : ARRAY[0..15] OF STRING[127];
    qPayload : ARRAY[0..15] OF STRING[255];
    qPayLen : ARRAY[0..15] OF UDINT;
    qQos : ARRAY[0..15] OF UDINT;
    qId : ARRAY[0..15] OF UDINT;
    qRet : ARRAY[0..15] OF BOOL;
    qFmt : ARRAY[0..15] OF BOOL;
    qExp : ARRAY[0..15] OF UDINT;
    qSub : ARRAY[0..15] OF UDINT;
    qCType : ARRAY[0..15] OF STRING[63];
    qRTopic : ARRAY[0..15] OF STRING[127];
    qCorr : ARRAY[0..15] OF STRING[127];
    qCorrLen : ARRAY[0..15] OF UDINT;
    qUProps : ARRAY[0..15] OF STRING[255];
    rx : FB_IotMqtt5Receive;
    slot : UDINT;
    dqPrev : BOOL;
    rsPrev : BOOL;
END_VAR
    HasMessage := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Pump AND count < 16 THEN
        rx.Execute := TRUE;
        rx.ConnectionHdl := ConnectionHdl;
        rx();
        rx.Execute := FALSE;
        IF rx.Message THEN
            slot := head + count;
            IF slot >= 16 THEN slot := slot - 16; END_IF;
            qTopic[slot] := rx.Topic;
            qPayload[slot] := rx.Payload;
            qPayLen[slot] := rx.PayloadLen;
            qQos[slot] := rx.QoS;
            qId[slot] := rx.MessageId;
            qRet[slot] := rx.Retained;
            qFmt[slot] := rx.PayloadFormatUtf8;
            qExp[slot] := rx.MessageExpiryS;
            qSub[slot] := rx.SubscriptionId;
            qCType[slot] := rx.ContentType;
            qRTopic[slot] := rx.ResponseTopic;
            qCorr[slot] := rx.CorrelationData;
            qCorrLen[slot] := rx.CorrelationLen;
            qUProps[slot] := rx.UserProperties;
            count := count + 1;
        END_IF;
        IF rx.Error THEN
            Error := TRUE;
            ErrorID := rx.ErrorID;
        END_IF;
    END_IF;
    Queued := count;
    IF DequeueReq AND NOT dqPrev AND count > 0 THEN
        Topic := qTopic[head];
        Payload := qPayload[head];
        PayloadLen := qPayLen[head];
        QoS := qQos[head];
        MessageId := qId[head];
        Retained := qRet[head];
        PayloadFormatUtf8 := qFmt[head];
        MessageExpiryS := qExp[head];
        SubscriptionId := qSub[head];
        ContentType := qCType[head];
        ResponseTopic := qRTopic[head];
        CorrelationData := qCorr[head];
        CorrelationLen := qCorrLen[head];
        UserProperties := qUProps[head];
        HasMessage := TRUE;
        head := head + 1;
        IF head >= 16 THEN head := 0; END_IF;
        count := count - 1;
        Queued := count;
    END_IF;
    dqPrev := DequeueReq;
    IF ResetReq AND NOT rsPrev THEN
        head := 0;
        count := 0;
        Queued := 0;
    END_IF;
    rsPrev := ResetReq;
END_FUNCTION_BLOCK
