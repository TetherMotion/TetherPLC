// tc3_mll.st — Tc3_MLL surface (TF38x0): ML inference prediction
// block.  TwinCAT loads an ONNX/TF-lite model into the ML runtime
// and runs async predictions.  TetherPLC ships no inference
// engine, so the default path is honest engine-absent: commands
// are accepted, Busy runs a bounded handshake, then Error with
// ErrorId=1 (engine-unavailable) — predictions are never faked.
//
// A pure-ST simulation mode is provided for logic testing:
// with UseSimulation=TRUE the block runs a deterministic
// single-output linear model  y = SUM(Wi * Xi) + Bias  over the
// caller-supplied weights — no engine required, and the result is
// honest arithmetic, not a canned answer.
//
// Differences from the TwinCAT FB (docs/plc/compat/README.md):
//   - stPredictionParameter is decomposed into plain inputs
//     (ModelPath, InputCount, X0..X7, UseSimulation, W0..W7, Bias);
//     TetherPLC has no struct/method surface in compat ST.
//   - CheckPreferredIODataTypes is exposed as inputs
//     FmtIn/FmtOut + output FmtPreferred (Tc float32/int32 codes:
//     1=f32, 2=i32, 3=i64, 4=f64 — preferred set {1,2}).
//   - ipResultMessage / I_TcMessage is replaced by UDINT ErrorId:
//     0 none, 1 engine-unavailable, 2 bad-input-rank,
//     3 bad-model, 4 sim-dtype-mismatch, 5 aborted.
//   - Predictions expose a scalar Y plus valid flag; the real
//     tensor output surface needs the engine.

// ---- FB_MllPrediction (TwinCAT §6.2.2.1) ----
FUNCTION_BLOCK FB_MllPrediction
VAR_INPUT
    Execute : BOOL;             // rising edge starts a prediction
    Abort : BOOL;
    ModelPath : STRING;         // ONNX model path (engine mode)
    InputCount : UDINT;         // active X lanes, 1..8
    UseSimulation : BOOL;       // TRUE: run the built-in linear sim
    X0 : LREAL; X1 : LREAL; X2 : LREAL; X3 : LREAL;
    X4 : LREAL; X5 : LREAL; X6 : LREAL; X7 : LREAL;
    W0 : LREAL; W1 : LREAL; W2 : LREAL; W3 : LREAL;
    W4 : LREAL; W5 : LREAL; W6 : LREAL; W7 : LREAL;
    Bias : LREAL;
    FmtIn : UDINT;              // CheckPreferredIODataTypes surface
    FmtOut : UDINT;
END_VAR
VAR_OUTPUT
    Busy : BOOL;
    Error : BOOL;
    ErrorId : UDINT;
    Done : BOOL;
    Y : LREAL;                  // prediction (simulation mode)
    YValid : BOOL;
    FmtPreferred : BOOL;
END_VAR
VAR
    phase : INT;                 // 0 idle, 1 handshake, 2 done
    tmo : UDINT;
    acc : LREAL;
    lanes : UDINT;
END_VAR
    Done := FALSE;
    YValid := FALSE;

    // preferred-IO-type check: TCML supports f32/i32 natively
    FmtPreferred := ((FmtIn = 1) OR (FmtIn = 2)) AND
                    ((FmtOut = 1) OR (FmtOut = 2));

    IF phase = 0 THEN
        Busy := FALSE;
        IF Execute THEN
            Busy := TRUE;
            Error := FALSE;
            ErrorId := 0;
            Y := 0.0;
            tmo := 0;
            // parameter validation first (mirrors TwinCAT checks)
            IF InputCount = 0 OR InputCount > 8 THEN
                Error := TRUE;
                ErrorId := 2;        // bad-input-rank
                phase := 2;
            ELSIF NOT UseSimulation AND ModelPath = '' THEN
                Error := TRUE;
                ErrorId := 3;        // bad-model: no path, no engine
                phase := 2;
            ELSIF UseSimulation THEN
                // deterministic linear inference: y = W.x + Bias
                acc := Bias;
                lanes := InputCount;
                IF lanes > 0 THEN acc := acc + W0 * X0; END_IF;
                IF lanes > 1 THEN acc := acc + W1 * X1; END_IF;
                IF lanes > 2 THEN acc := acc + W2 * X2; END_IF;
                IF lanes > 3 THEN acc := acc + W3 * X3; END_IF;
                IF lanes > 4 THEN acc := acc + W4 * X4; END_IF;
                IF lanes > 5 THEN acc := acc + W5 * X5; END_IF;
                IF lanes > 6 THEN acc := acc + W6 * X6; END_IF;
                IF lanes > 7 THEN acc := acc + W7 * X7; END_IF;
                Y := acc;
                YValid := TRUE;
                Busy := FALSE;
                phase := 2;
            ELSE
                phase := 1;          // engine handshake window
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
            phase := 0;              // re-arm on falling edge
        END_IF;
    END_IF;

    IF Abort AND phase = 1 THEN
        Busy := FALSE;
        Error := TRUE;
        ErrorId := 5;
        phase := 2;
    END_IF;
    Done := (phase = 2);
END_FUNCTION_BLOCK
