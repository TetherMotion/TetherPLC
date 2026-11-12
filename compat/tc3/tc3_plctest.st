// Tc3_PlcTestFramework (TF1140) -> PLC-side test framework.
// Beckhoff's library exposes a METHOD-based assertion API on
// FB_TestCaseBase/FB_TestPrepBase plus a test client that drives
// the run.  This port keeps the assertion/step semantics and
// replaces the METHOD calls with edge-triggered command inputs
// (this compiler has no METHOD support):
//   FB_TestCase   nAssertCmd 1=Assert 2=AssertEqual
//                 3=AssertAbsEqual 4=AssertRelEqual
//                 5=AssertStrictLarger
//                 nCmd 1=IncStep 2=GoToStep 3=IncStepIf 4=Cleanup
//                 5=Reset
//   FB_TestSuite  nCmd 1=AddResult 2=Reset
// Documented differences:
//   - No test client / no .tcuitest files: results are PLC values
//     (counters + bSucceeded) that the host or a CI script reads
//     from the process image.
//   - Assertions are counted, not reported with stack traces;
//     nFailures is the failure count and bSucceeded the verdict.
//   - AssertMsg carries no message text (documented reduction).

FUNCTION_BLOCK FB_TestCase
VAR_INPUT
    // assertion request
    nAssertCmd : UDINT;         // see header
    bAssertCond : BOOL;         // Assert / AssertMsg
    fActual : LREAL;
    fExpected : LREAL;
    fTolerance : LREAL;         // abs tolerance / rel fraction
    // step control
    nCmd : UDINT;
    nStepArg : UDINT;
    bStepCond : BOOL;           // IncStepIf
END_VAR
VAR_OUTPUT
    nAssertions : UDINT;
    nFailures : UDINT;
    nStep : UDINT;
    bSucceeded : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    prevAssert : UDINT;
    prevCmd : UDINT;
    fDiff : LREAL;
    fRel : LREAL;
    ok : BOOL;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;

    // ---- assertions ----
    IF nAssertCmd <> 0 AND prevAssert = 0 THEN
        ok := FALSE;
        IF nAssertCmd = 1 THEN
            ok := bAssertCond;
        ELSIF nAssertCmd = 2 THEN
            ok := fActual = fExpected;
        ELSIF nAssertCmd = 3 THEN
            fDiff := fActual - fExpected;
            IF fDiff < 0.0 THEN fDiff := -fDiff; END_IF;
            ok := fDiff <= fTolerance;
        ELSIF nAssertCmd = 4 THEN
            fRel := 0.0;
            IF fExpected <> 0.0 THEN
                fRel := (fActual - fExpected) / fExpected;
                IF fRel < 0.0 THEN fRel := -fRel; END_IF;
            END_IF;
            ok := fRel <= fTolerance;
        ELSIF nAssertCmd = 5 THEN
            ok := fActual > fExpected;
        END_IF;
        nAssertions := nAssertions + 1;
        IF NOT ok THEN nFailures := nFailures + 1; END_IF;
        bDone := TRUE;
    END_IF;
    prevAssert := nAssertCmd;

    // ---- step control ----
    IF nCmd <> 0 AND prevCmd = 0 THEN
        IF nCmd = 1 THEN
            nStep := nStep + 1;
            bDone := TRUE;
        ELSIF nCmd = 2 THEN
            nStep := nStepArg;
            bDone := TRUE;
        ELSIF nCmd = 3 THEN
            IF bStepCond THEN nStep := nStep + 1; END_IF;
            bDone := TRUE;
        ELSIF nCmd = 4 THEN
            bDone := TRUE;          // Cleanup: nothing owned here
        ELSIF nCmd = 5 THEN
            nStep := 0; nAssertions := 0; nFailures := 0;
            bDone := TRUE;
        ELSE
            bError := TRUE; nErrId := 1;
        END_IF;
    END_IF;
    prevCmd := nCmd;

    bSucceeded := nFailures = 0;
END_FUNCTION_BLOCK

// ---- FB_TestPrepBase: same surface, used for fixtures ----
// The upstream Prep block differs only in the run phase; here it is
// the same implementation with a distinct type so suites can tell
// preparation assertions from test assertions.
FUNCTION_BLOCK FB_TestPrepBase
VAR_INPUT
    nAssertCmd : UDINT;
    bAssertCond : BOOL;
    fActual : LREAL;
    fExpected : LREAL;
    fTolerance : LREAL;
    nCmd : UDINT;
    nStepArg : UDINT;
    bStepCond : BOOL;
END_VAR
VAR_OUTPUT
    nAssertions : UDINT;
    nFailures : UDINT;
    nStep : UDINT;
    bSucceeded : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    tc : FB_TestCase;
END_VAR
    tc.nAssertCmd := nAssertCmd;
    tc.bAssertCond := bAssertCond;
    tc.fActual := fActual;
    tc.fExpected := fExpected;
    tc.fTolerance := fTolerance;
    tc.nCmd := nCmd;
    tc.nStepArg := nStepArg;
    tc.bStepCond := bStepCond;
    tc();
    nAssertions := tc.nAssertions;
    nFailures := tc.nFailures;
    nStep := tc.nStep;
    bSucceeded := tc.bSucceeded;
    bDone := tc.bDone;
    bError := tc.bError;
    nErrId := tc.nErrId;
END_FUNCTION_BLOCK

// ---- FB_TestCaseBasePrep: prep-phase variant of the case ----
FUNCTION_BLOCK FB_TestCaseBasePrep
VAR_INPUT
    nAssertCmd : UDINT;
    bAssertCond : BOOL;
    fActual : LREAL;
    fExpected : LREAL;
    fTolerance : LREAL;
    nCmd : UDINT;
    nStepArg : UDINT;
    bStepCond : BOOL;
END_VAR
VAR_OUTPUT
    nAssertions : UDINT;
    nFailures : UDINT;
    nStep : UDINT;
    bSucceeded : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    tc : FB_TestCase;
END_VAR
    tc.nAssertCmd := nAssertCmd;
    tc.bAssertCond := bAssertCond;
    tc.fActual := fActual;
    tc.fExpected := fExpected;
    tc.fTolerance := fTolerance;
    tc.nCmd := nCmd;
    tc.nStepArg := nStepArg;
    tc.bStepCond := bStepCond;
    tc();
    nAssertions := tc.nAssertions;
    nFailures := tc.nFailures;
    nStep := tc.nStep;
    bSucceeded := tc.bSucceeded;
    bDone := tc.bDone;
    bError := tc.bError;
    nErrId := tc.nErrId;
END_FUNCTION_BLOCK

// ---- FB_TestSuite: aggregate case results ----
FUNCTION_BLOCK FB_TestSuite
VAR_INPUT
    nCmd : UDINT;               // 1=AddResult 2=Reset
    bCasePassed : BOOL;
END_VAR
VAR_OUTPUT
    nTests : UDINT;
    nPassed : UDINT;
    nFailed : UDINT;
    bAllPassed : BOOL;
    bDone : BOOL;
END_VAR
VAR
    prev : UDINT;
END_VAR
    bDone := FALSE;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 1 THEN
            nTests := nTests + 1;
            IF bCasePassed THEN
                nPassed := nPassed + 1;
            ELSE
                nFailed := nFailed + 1;
            END_IF;
            bDone := TRUE;
        ELSIF nCmd = 2 THEN
            nTests := 0; nPassed := 0; nFailed := 0;
            bDone := TRUE;
        END_IF;
    END_IF;
    prev := nCmd;
    bAllPassed := nFailed = 0 AND nTests > 0;
END_FUNCTION_BLOCK
