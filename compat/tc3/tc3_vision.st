// tc3_vision.st — Tc3_Vision surface (TF7000-TF7810): GigE Vision
// camera control, image acquisition, and vision-function job
// dispatch.  The real library is ~2,500 APIs behind the TwinCAT
// Vision engine (GenICam/GenTL producer + GPU/CPU algorithms) —
// TetherPLC ships no vision engine, so these representative
// blocks are honest engine-absent implementations: commands are
// validated, Busy runs a bounded handshake, then Error with a
// documented ErrorId.  No image data, no algorithm results, and
// no camera register traffic is ever faked.
//
// Differences from the TwinCAT library
// (docs/plc/compat/README.md):
//   - Only the control/transport spine is surfaced — the ~2,400
//     algorithm APIs (FB_VN_* measurement/matching/filtering)
//     need the engine; they are catalogued in
//     plan/21-twincat-libraries.md.
//   - ITcVnImage buffers are represented by an image-handle +
//     width/height/format status outputs; no pixel memory is
//     allocated without an engine.
//   - METHOD-style calls (OpenCamera, trigger params) are
//     flattened to Execute-style inputs per the other compat
//     surfaces.
//   - ErrorId: 0 none, 1 engine-unavailable, 2 bad-address,
//     3 bad-job, 4 aborted, 5 not-open.

// ---- FB_VN_GevCamera: GigE Vision control channel ----
FUNCTION_BLOCK FB_VN_GevCamera
VAR_INPUT
    Open : BOOL;                // rising edge opens the channel
    Close : BOOL;
    IpAddress : STRING;         // dotted-quad camera address
    Port : UINT;                // GVCP port (default 3956)
END_VAR
VAR_OUTPUT
    Opened : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
END_VAR
VAR
    phase : INT;                 // 0 closed, 1 handshake, 2 done
    tmo : UDINT;
END_VAR
    IF phase = 0 THEN
        Busy := FALSE;
        IF Open THEN
            Busy := TRUE;
            Error := FALSE;
            ErrorId := 0;
            Opened := FALSE;
            tmo := 0;
            IF IpAddress = '' OR Port = 0 THEN
                Error := TRUE;
                ErrorId := 2;        // bad-address
                Busy := FALSE;
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
            ErrorId := 1;            // engine-unavailable
            phase := 2;
        END_IF;
    ELSE
        Busy := FALSE;
        IF NOT Open THEN
            phase := 0;
        END_IF;
    END_IF;
    IF Close THEN
        Opened := FALSE;
        phase := 0;
        Busy := FALSE;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_VN_AcquireImage: trigger + frame grab ----
FUNCTION_BLOCK FB_VN_AcquireImage
VAR_INPUT
    Trigger : BOOL;             // rising edge grabs one frame
    CameraOpen : BOOL;          // from FB_VN_GevCamera.Opened
    TimeoutMs : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
    ImageReady : BOOL;          // a frame buffer was filled —
                                // never TRUE without an engine
    ImageHandle : UDINT;
    ImageWidth : UDINT;
    ImageHeight : UDINT;
END_VAR
VAR
    phase : INT;
    tmo : UDINT;
END_VAR
    ImageReady := FALSE;
    ImageHandle := 0;
    ImageWidth := 0;
    ImageHeight := 0;
    IF phase = 0 THEN
        Busy := FALSE;
        IF Trigger THEN
            Busy := TRUE;
            Error := FALSE;
            ErrorId := 0;
            tmo := 0;
            IF NOT CameraOpen THEN
                Error := TRUE;
                ErrorId := 5;        // not-open
                Busy := FALSE;
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
            ErrorId := 1;            // engine-unavailable
            phase := 2;
        END_IF;
    ELSE
        Busy := FALSE;
        IF NOT Trigger THEN
            phase := 0;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_VN_JobControl: generic vision-function job ----
FUNCTION_BLOCK FB_VN_JobControl
VAR_INPUT
    Execute : BOOL;
    Abort : BOOL;
    JobType : UDINT;            // engine job family id (>0)
    TimeoutMs : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
    Done : BOOL;
    ResultValid : BOOL;          // never TRUE without an engine
END_VAR
VAR
    phase : INT;
    tmo : UDINT;
END_VAR
    Done := FALSE;
    ResultValid := FALSE;
    IF phase = 0 THEN
        Busy := FALSE;
        IF Execute THEN
            Busy := TRUE;
            Error := FALSE;
            ErrorId := 0;
            tmo := 0;
            IF JobType = 0 THEN
                Error := TRUE;
                ErrorId := 3;        // bad-job
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
            ErrorId := 1;            // engine-unavailable
            phase := 2;
        END_IF;
    ELSE
        Busy := FALSE;
        IF NOT Execute THEN
            phase := 0;
        END_IF;
    END_IF;
    IF Abort AND phase = 1 THEN
        Busy := FALSE;
        Error := TRUE;
        ErrorId := 4;
        phase := 2;
    END_IF;
    Done := (phase = 2);
END_FUNCTION_BLOCK
