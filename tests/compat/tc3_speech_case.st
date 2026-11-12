// tc3_speech_case.st — Tc3_Speech e2e: honest engine-absent
// handshake — TTS/SR commands run Busy for a bounded window then
// fail with ErrorId=1 (engine-unavailable); parameter validation
// fires immediately (ErrorId=2/3).
PROGRAM tc3_speech_case
VAR
    mask AT %QD0 : UDINT;
    tts : FB_TextToSpeech;
    sr : FB_SpeechRecognition;
    st : INT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        tts.Speak := FALSE;
        sr.Listen := FALSE;
        tts.Utterance := 'hello axis';
        tts.ConfigurationId := 1;
        tts.LanguageId := 0;
        sr.ConfigurationId := 1;
        st := 1;
    ELSIF st = 1 THEN
        tts.Speak := TRUE;         // valid command -> handshake
        st := 2;
    ELSIF st = 2 THEN
        IF tts.Busy THEN mask := mask OR 1; END_IF;
        IF tts.Error AND tts.ErrorId = 1 THEN
            mask := mask OR 2;      // engine-unavailable reported
            st := 3;
        END_IF;
    ELSIF st = 3 THEN
        tts.Speak := FALSE;
        st := 4;
    ELSIF st = 4 THEN
        tts.Utterance := '';       // empty -> param error
        tts.Speak := TRUE;
        st := 5;
    ELSIF st = 5 THEN
        IF tts.Error AND tts.ErrorId = 2 THEN
            mask := mask OR 4;      // bad-utterance validated
            st := 6;
        END_IF;
    ELSIF st = 6 THEN
        tts.Speak := FALSE;
        sr.Listen := TRUE;         // valid recognition command
        st := 7;
    ELSIF st = 7 THEN
        IF sr.Error AND sr.ErrorId = 1 AND NOT sr.ResultValid THEN
            mask := mask OR 8;      // SR engine-absent, no fake text
            st := 8;
        END_IF;
    ELSIF st = 8 THEN
        sr.Listen := FALSE;        // let the FB re-arm one scan
        st := 9;
    ELSIF st = 9 THEN
        sr.ConfigurationId := 0;   // missing config -> param error
        sr.Listen := TRUE;
        st := 10;
    ELSIF st = 10 THEN
        IF sr.Error AND sr.ErrorId = 3 THEN
            mask := mask OR 16;
            st := 11;
        END_IF;
    END_IF;

    tts();
    sr();
END_PROGRAM
