// mqtt_events.st — FB_MqttEventLog: ring-buffer journal of MQTT
// messages as executable logic.  Feed it from FB_MqttRx (Push <-
// HasMsg, TopicIn <- Topic, PayloadIn <- Payload) or any source;
// entries are readable as ordinary members (log.LOG[i].Topic) for
// trending, alarming, or export.
//
// Push is edge-detected internally: hold it high, one entry lands.
// Tick is seconds accumulated from CycleTime (0 -> 0.01 s steps).
// Ordering: newest entry at index Head, oldest at (Head+1) MOD 32;
// scan forward from oldest for chronological order.

// ST_MqttEvent — one journal entry
TYPE ST_MqttEvent : STRUCT
    Tick : LREAL;          // seconds since first push
    Topic : STRING[127];
    Payload : STRING[255];
END_STRUCT
END_TYPE

FUNCTION_BLOCK FB_MqttEventLog
VAR_INPUT
    Execute : BOOL;       // call every scan
    CycleTime : LREAL;    // s; <=0 -> 0.01
    Push : BOOL;          // rising edge = store TopicIn/PayloadIn
    TopicIn : STRING[127];
    PayloadIn : STRING[255];
END_VAR
VAR_OUTPUT
    Count : UDINT;        // entries stored (<= 32)
    Full : BOOL;          // ring wrapped at least once
    Head : UDINT;         // index of newest entry
END_VAR
VAR
    LOG : ARRAY[0..31] OF ST_MqttEvent;
    w : UDINT;            // next write index
    n : UDINT;
    tick : LREAL;
    pPrev : BOOL;
END_VAR
    IF NOT Execute THEN
        pPrev := FALSE;
        RETURN;
    END_IF;
    IF CycleTime > 0.0 THEN tick := tick + CycleTime;
    ELSE tick := tick + 0.01; END_IF;
    IF Push AND NOT pPrev THEN
        LOG[w].Tick := tick;
        LOG[w].Topic := TopicIn;
        LOG[w].Payload := PayloadIn;
        w := (w + 1) MOD 32;
        IF n < 32 THEN n := n + 1; ELSE Full := TRUE; END_IF;
        Head := (w + 31) MOD 32;
    END_IF;
    pPrev := Push;
    Count := n;
END_FUNCTION_BLOCK
