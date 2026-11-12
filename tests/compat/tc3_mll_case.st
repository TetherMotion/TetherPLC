// tc3_mll_case.st — Tc3_MLL e2e: simulation-mode linear
// inference, engine-absent error path, and input validation.
PROGRAM tc3_mll_case
VAR
    mask AT %QD0 : UDINT;
    pred : FB_MllPrediction;
    st : INT;
END_VAR
    IF st = 0 THEN
        mask := 0;
        pred.Execute := FALSE;
        // y = 2*x0 + 3*x1 - 1 ; x=(4,5) -> 22
        pred.InputCount := 2;
        pred.X0 := 4.0; pred.X1 := 5.0;
        pred.W0 := 2.0; pred.W1 := 3.0;
        pred.Bias := -1.0;
        pred.FmtIn := 1; pred.FmtOut := 1;   // f32/f32 preferred
        st := 1;
    ELSIF st = 1 THEN
        pred.UseSimulation := TRUE;
        pred.Execute := TRUE;
        st := 2;
    ELSIF st = 2 THEN
        IF pred.YValid AND pred.Y > 21.9 AND pred.Y < 22.1 THEN
            mask := mask OR 1;               // sim inference = 22
            st := 3;
        END_IF;
    ELSIF st = 3 THEN
        pred.Execute := FALSE;
        st := 4;
    ELSIF st = 4 THEN
        IF pred.FmtPreferred THEN
            mask := mask OR 2;               // f32/f32 preferred
            st := 5;
        END_IF;
    ELSIF st = 5 THEN
        pred.UseSimulation := FALSE;
        pred.ModelPath := 'model.onnx';
        pred.Execute := TRUE;                // engine mode, no engine
        st := 6;
    ELSIF st = 6 THEN
        IF pred.Error AND pred.ErrorId = 1 AND NOT pred.YValid THEN
            mask := mask OR 4;               // honest engine-absent
            st := 7;
        END_IF;
    ELSIF st = 7 THEN
        pred.Execute := FALSE;
        st := 8;
    ELSIF st = 8 THEN
        pred.InputCount := 9;                // out of rank range
        pred.Execute := TRUE;
        st := 9;
    ELSIF st = 9 THEN
        IF pred.Error AND pred.ErrorId = 2 THEN
            mask := mask OR 8;
            st := 10;
        END_IF;
    ELSIF st = 10 THEN
        pred.Execute := FALSE;
        st := 11;
    ELSIF st = 11 THEN
        pred.InputCount := 2;
        pred.UseSimulation := FALSE;
        pred.ModelPath := '';                // empty path, engine mode
        pred.Execute := TRUE;
        st := 12;
    ELSIF st = 12 THEN
        IF pred.Error AND pred.ErrorId = 3 THEN
            mask := mask OR 16;
            st := 13;
        END_IF;
    END_IF;

    pred();
END_PROGRAM
