// tc3_plctest_case.st — e2e for tc3_plctest (TF1140 surface).
// Every command gets its own scan and a following clear scan, so
// the FB's edge detectors (prev = 0) see a fresh request.
PROGRAM tc3_plctest_case
VAR
    tc : FB_TestCase;
    prep : FB_TestPrepBase;
    suite : FB_TestSuite;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        tc.nCmd := 5;                        // Reset
        st := 1;
    ELSIF st = 1 THEN
        tc.nCmd := 0;
        st := 2;
    // ---- assertions: set / clear pairs ----
    ELSIF st = 2 THEN
        tc.nAssertCmd := 1; tc.bAssertCond := TRUE;
        st := 3;
    ELSIF st = 3 THEN
        tc.nAssertCmd := 0;
        st := 4;
    ELSIF st = 4 THEN
        tc.nAssertCmd := 2; tc.fActual := 5.0; tc.fExpected := 5.0;
        st := 5;
    ELSIF st = 5 THEN
        tc.nAssertCmd := 0;
        st := 6;
    ELSIF st = 6 THEN
        tc.nAssertCmd := 3;
        tc.fActual := 10.05; tc.fExpected := 10.0; tc.fTolerance := 0.1;
        st := 7;
    ELSIF st = 7 THEN
        tc.nAssertCmd := 0;
        st := 8;
    ELSIF st = 8 THEN
        tc.nAssertCmd := 4;
        tc.fActual := 100.5; tc.fExpected := 100.0; tc.fTolerance := 0.01;
        st := 9;
    ELSIF st = 9 THEN
        tc.nAssertCmd := 0;
        st := 10;
    ELSIF st = 10 THEN
        tc.nAssertCmd := 5; tc.fActual := 7.0; tc.fExpected := 3.0;
        st := 11;
    ELSIF st = 11 THEN
        tc.nAssertCmd := 0;
        IF tc.nAssertions = 5 AND tc.nFailures = 0 AND tc.bSucceeded THEN
            mask := mask OR 1;
        END_IF;
        st := 12;
    ELSIF st = 12 THEN
        tc.nAssertCmd := 2; tc.fActual := 1.0; tc.fExpected := 2.0;
        st := 13;
    ELSIF st = 13 THEN
        tc.nAssertCmd := 0;
        IF tc.nAssertions = 6 AND tc.nFailures = 1
           AND NOT tc.bSucceeded THEN
            mask := mask OR 2;
        END_IF;
        st := 14;
    // ---- step control ----
    ELSIF st = 14 THEN
        tc.nCmd := 1;                        // IncStep
        st := 15;
    ELSIF st = 15 THEN
        tc.nCmd := 0;
        IF tc.nStep = 1 THEN mask := mask OR 4; END_IF;
        st := 16;
    ELSIF st = 16 THEN
        tc.nCmd := 2; tc.nStepArg := 10;     // GoToStep 10
        st := 17;
    ELSIF st = 17 THEN
        tc.nCmd := 0;
        IF tc.nStep = 10 THEN mask := mask OR 8; END_IF;
        st := 18;
    ELSIF st = 18 THEN
        tc.nCmd := 3; tc.bStepCond := FALSE; // IncStepIf false -> stay
        st := 19;
    ELSIF st = 19 THEN
        tc.nCmd := 0;
        IF tc.nStep = 10 THEN mask := mask OR 16; END_IF;
        st := 20;
    ELSIF st = 20 THEN
        tc.nCmd := 3; tc.bStepCond := TRUE;  // IncStepIf true -> 11
        st := 21;
    ELSIF st = 21 THEN
        tc.nCmd := 0;
        IF tc.nStep = 11 THEN mask := mask OR 32; END_IF;
        st := 22;
    // ---- prep block ----
    ELSIF st = 22 THEN
        prep.nAssertCmd := 1; prep.bAssertCond := TRUE;
        st := 23;
    ELSIF st = 23 THEN
        prep.nAssertCmd := 0;
        IF prep.nAssertions = 1 AND prep.bSucceeded THEN
            mask := mask OR 64;
        END_IF;
        st := 24;
    // ---- suite aggregation ----
    ELSIF st = 24 THEN
        suite.bCasePassed := TRUE; suite.nCmd := 1;
        st := 25;
    ELSIF st = 25 THEN
        suite.nCmd := 0;
        st := 26;
    ELSIF st = 26 THEN
        suite.bCasePassed := FALSE; suite.nCmd := 1;
        st := 27;
    ELSIF st = 27 THEN
        suite.nCmd := 0;
        IF suite.nTests = 2 AND suite.nPassed = 1
           AND suite.nFailed = 1 AND NOT suite.bAllPassed THEN
            mask := mask OR 128;
        END_IF;
        st := 28;
    END_IF;
    tc(); prep(); suite();
END_PROGRAM
