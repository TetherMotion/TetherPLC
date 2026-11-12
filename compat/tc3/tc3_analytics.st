// tc3_analytics.st — Tc3_Analytics surface (TF3510/TF3500/TF3520):
// measurement analytics.  TwinCAT's full library is ~670 APIs
// behind a licensed analytics engine; the local measurement-side
// primitives (event counting, windowed statistics, sample
// logging) are pure-ST here and fully functional, while the
// engine-backed surfaces (storage provider, MQTT/HTTP egress
// transport) report an honest engine-absent error.
//
// Differences from the TwinCAT library
// (docs/plc/compat/README.md):
//   - TwinCAT's class/method/object model is flattened into
//     FB_* blocks with Execute-style inputs, matching the other
//     TetherPLC compat surfaces.
//   - The ring logger exposes the newest N samples via indexed
//     outputs Slot0..Slot7 plus Count; a real circular-buffer
//     API needs dynamic memory we do not model.
//   - Storage/provider and transport FBs accept the command,
//     run a bounded handshake, then fail ErrorId=1
//     (engine-unavailable): no database or MQTT egress is faked.

// ---- FB_AnalyticsCounter: edge counter + rate per window ----
FUNCTION_BLOCK FB_AnalyticsCounter
VAR_INPUT
    Enable : BOOL;
    Event : BOOL;               // count rising edges
    WindowTime : LREAL;         // seconds per rate window
    CycleTime : LREAL;          // task cycle seconds
    Reset : BOOL;
END_VAR
VAR_OUTPUT
    Total : UDINT;
    WindowCount : UDINT;
    Rate : LREAL;               // events per second (last window)
    WindowDone : BOOL;
END_VAR
VAR
    prev : BOOL;
    acc : LREAL;
END_VAR
    WindowDone := FALSE;
    IF Reset THEN
        Total := 0;
        WindowCount := 0;
        Rate := 0.0;
        acc := 0.0;
        prev := FALSE;
    END_IF;
    IF NOT Enable THEN
        prev := Event;
        RETURN;
    END_IF;
    IF Event AND NOT prev THEN
        Total := Total + 1;
        WindowCount := WindowCount + 1;
    END_IF;
    prev := Event;
    acc := acc + CycleTime;
    IF WindowTime > 0.0 AND acc >= WindowTime THEN
        Rate := UDINT_TO_LREAL(WindowCount) / acc;
        WindowCount := 0;
        acc := 0.0;
        WindowDone := TRUE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_AnalyticsStats: running + windowed min/max/mean ----
FUNCTION_BLOCK FB_AnalyticsStats
VAR_INPUT
    Execute : BOOL;             // sample the input when TRUE
    Sample : LREAL;
    Clear : BOOL;
END_VAR
VAR_OUTPUT
    Count : UDINT;
    Min : LREAL;
    Max : LREAL;
    Mean : LREAL;
    Valid : BOOL;
END_VAR
VAR
    sum : LREAL;
END_VAR
    Valid := FALSE;
    IF Clear THEN
        Count := 0;
        Min := 0.0;
        Max := 0.0;
        Mean := 0.0;
        sum := 0.0;
    END_IF;
    IF NOT Execute THEN RETURN; END_IF;
    IF Count = 0 THEN
        Min := Sample;
        Max := Sample;
    ELSE
        IF Sample < Min THEN Min := Sample; END_IF;
        IF Sample > Max THEN Max := Sample; END_IF;
    END_IF;
    Count := Count + 1;
    sum := sum + Sample;
    Mean := sum / UDINT_TO_LREAL(Count);
    Valid := TRUE;
END_FUNCTION_BLOCK

// ---- FB_AnalyticsLogger: 8-deep ring of timestamped samples ----
FUNCTION_BLOCK FB_AnalyticsLogger
VAR_INPUT
    Record : BOOL;              // rising edge stores a sample
    Value : LREAL;
    Timestamp : LREAL;          // caller-supplied time tag
    Clear : BOOL;
END_VAR
VAR_OUTPUT
    Count : UDINT;              // total records stored
    Slot0 : LREAL; Slot1 : LREAL; Slot2 : LREAL; Slot3 : LREAL;
    Slot4 : LREAL; Slot5 : LREAL; Slot6 : LREAL; Slot7 : LREAL;
    LastTs : LREAL;
    Overflow : BOOL;            // wrapped at least once
END_VAR
VAR
    head : UDINT;
    prev : BOOL;
END_VAR
    IF Clear THEN
        Count := 0;
        head := 0;
        LastTs := 0.0;
        Overflow := FALSE;
        Slot0 := 0.0; Slot1 := 0.0; Slot2 := 0.0; Slot3 := 0.0;
        Slot4 := 0.0; Slot5 := 0.0; Slot6 := 0.0; Slot7 := 0.0;
    END_IF;
    IF Record AND NOT prev THEN
        IF head = 0 THEN Slot0 := Value; END_IF;
        IF head = 1 THEN Slot1 := Value; END_IF;
        IF head = 2 THEN Slot2 := Value; END_IF;
        IF head = 3 THEN Slot3 := Value; END_IF;
        IF head = 4 THEN Slot4 := Value; END_IF;
        IF head = 5 THEN Slot5 := Value; END_IF;
        IF head = 6 THEN Slot6 := Value; END_IF;
        IF head = 7 THEN Slot7 := Value; END_IF;
        head := head + 1;
        IF head >= 8 THEN
            head := 0;
            Overflow := TRUE;
        END_IF;
        Count := Count + 1;
        LastTs := Timestamp;
    END_IF;
    prev := Record;
END_FUNCTION_BLOCK

// ---- FB_AnalyticsStore: engine-backed storage provider ----
// Honest engine-absent block: accepts a flush/store command,
// holds Busy for a bounded handshake, then fails with
// ErrorId=1 — no database write is ever claimed.
FUNCTION_BLOCK FB_AnalyticsStore
VAR_INPUT
    Execute : BOOL;
    Abort : BOOL;
    ProviderPath : STRING;      // e.g. database/Influx endpoint
    RecordCount : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;            // 0 none, 1 engine-unavailable,
                                // 2 bad-provider, 3 aborted
    Done : BOOL;
END_VAR
VAR
    phase : INT;
    tmo : UDINT;
END_VAR
    Done := FALSE;
    IF phase = 0 THEN
        Busy := FALSE;
        IF Execute THEN
            Busy := TRUE;
            Error := FALSE;
            ErrorId := 0;
            tmo := 0;
            IF ProviderPath = '' THEN
                Error := TRUE;
                ErrorId := 2;
                phase := 2;
            ELSE
                phase := 1;
            END_IF;
        END_IF;
    ELSIF phase = 1 THEN
        tmo := tmo + 1;
        IF tmo >= 4 THEN
            Busy := FALSE;
            Error := TRUE;
            ErrorId := 1;       // engine-unavailable
            phase := 2;
        END_IF;
    ELSE
        Busy := FALSE;
        IF NOT Execute THEN phase := 0; END_IF;
    END_IF;
    IF Abort AND phase = 1 THEN
        Busy := FALSE;
        Error := TRUE;
        ErrorId := 3;
        phase := 2;
    END_IF;
    Done := (phase = 2);
END_FUNCTION_BLOCK
