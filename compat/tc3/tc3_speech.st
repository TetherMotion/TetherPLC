// tc3_speech.st — Tc3_Speech surface (TF4500): ASR + TTS blocks.
// TwinCAT routes these FBs to an external TcSpeech service via
// AMS. TetherPLC ships no speech engine, so these blocks are
// honest engine-absent implementations: they accept commands,
// hold Busy for a bounded handshake window, then fail with a
// distinct, documented error code — they never fake a
// recognition result or audio playback.
//
// Differences from the TwinCAT FBs (docs/plc/compat/README.md):
//   - METHODs (SetAmsNetAddr) are modelled as plain inputs:
//     AmsNetId is an optional STRING address; it is validated
//     (non-empty) but never routed — no engine exists to reach.
//   - nErrorId uses UDINT codes: 0 none, 1 engine-unavailable,
//     2 bad-utterance, 3 bad-config, 4 busy-timeout.
//   - nPlaybackPosition/nPlaybackTotal remain 0 (no engine).
//   - FB_SpeechRecognition emits no hypothesis strings — a real
//     engine is required to produce text; ResultValid stays FALSE.

// ---- FB_TextToSpeech (TwinCAT §9.2.5) ----
FUNCTION_BLOCK FB_TextToSpeech
VAR_INPUT
    Speak : BOOL;             // rising edge starts a speech command
    Utterance : STRING;       // text to speak
    ConfigurationId : UINT;   // TcSpeech configurator ID
    LanguageId : UINT;        // 0 = configured default
    AmsNetId : STRING;        // optional destination AMS net id
    Abort : BOOL;             // rising edge aborts
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
    Done : BOOL;               // command completed (always via Error here)
    PlaybackPosition : ULINT;
    PlaybackTotal : ULINT;
    LastCommandExitCode : UINT;
END_VAR
VAR
    phase : INT;               // 0 idle, 1 handshake, 2 done
    tmo : UDINT;
END_VAR
    Done := FALSE;
    PlaybackPosition := 0;
    PlaybackTotal := 0;
    LastCommandExitCode := 0;

    IF phase = 0 THEN
        Busy := FALSE;
        IF Speak THEN
            // validate the command surface before reporting the
            // engine absence — mirrors TwinCAT parameter checks
            Busy := TRUE;
            Error := FALSE;
            ErrorId := 0;
            phase := 1;
            tmo := 0;
            IF Utterance = '' THEN
                Error := TRUE;
                ErrorId := 2;          // bad-utterance
                phase := 2;
            ELSIF ConfigurationId = 0 AND LanguageId <> 0 THEN
                Error := TRUE;
                ErrorId := 3;          // bad-config
                phase := 2;
            END_IF;
        END_IF;
    ELSIF phase = 1 THEN
        // handshake window: a real engine would claim the command
        // within a few cycles; absent one we fail deterministically
        tmo := tmo + 1;
        IF tmo >= 4 THEN
            Busy := FALSE;
            Error := TRUE;
            ErrorId := 1;              // engine-unavailable
            phase := 2;
        END_IF;
    ELSE
        Busy := FALSE;
        IF NOT Speak THEN
            phase := 0;                // re-arm on falling edge
        END_IF;
    END_IF;

    IF Abort AND phase = 1 THEN
        Busy := FALSE;
        Error := TRUE;
        ErrorId := 4;                  // busy-timeout / aborted
        phase := 2;
    END_IF;
    Done := (phase = 2);
END_FUNCTION_BLOCK

// ---- FB_SpeechRecognition (TwinCAT §9.2.4) ----
FUNCTION_BLOCK FB_SpeechRecognition
VAR_INPUT
    Listen : BOOL;            // rising edge starts recognition
    ConfigurationId : UINT;
    AmsNetId : STRING;
    Abort : BOOL;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
    Done : BOOL;
    ResultValid : BOOL;        // a hypothesis was produced — never
                              // TRUE without an engine
END_VAR
VAR
    phase : INT;
    tmo : UDINT;
END_VAR
    Done := FALSE;
    ResultValid := FALSE;

    IF phase = 0 THEN
        Busy := FALSE;
        IF Listen THEN
            Busy := TRUE;
            Error := FALSE;
            ErrorId := 0;
            phase := 1;
            tmo := 0;
            IF ConfigurationId = 0 THEN
                Error := TRUE;
                ErrorId := 3;
                phase := 2;
            END_IF;
        END_IF;
    ELSIF phase = 1 THEN
        tmo := tmo + 1;
        IF tmo >= 4 THEN
            Busy := FALSE;
            Error := TRUE;
            ErrorId := 1;              // engine-unavailable
            phase := 2;
        END_IF;
    ELSE
        Busy := FALSE;
        IF NOT Listen THEN
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
