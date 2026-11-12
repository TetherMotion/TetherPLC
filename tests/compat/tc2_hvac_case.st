// tc2_hvac_case.st — pure-ST e2e for tc2_hvac.
PROGRAM tc2_hvac_case
VAR
    ain : FB_HVACAnalogInput;
    aout : FB_HVACAnalogOutput;
    tp2 : FB_HVAC2PointActuator;
    tp3 : FB_HVAC3PointActuator;
    pump : FB_HVACCirculationPump;
    mot : FB_HVACMotor3Speed;
    mux : FB_HVACMux8;
    red : FB_HVACRedundancyCtrl;
    c2p : FB_HVAC2PointCtrl;
    stp : FB_HVACI_CtrlStep;
    pid : FB_HVACPIDCtrl;
    hcv : FB_HVACHeatingCurve;
    dmp : FB_HVACOutsideTempDamped;
    alm : FB_HVACAlarm;
    abp : FB_HVACAntiBlockingPump;
    blk : FB_HVACBlink;
    bar : FB_BARSetpointRoom;
    dhy : FB_BARDelayedHysteresis;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        // analog input: 8000 of 0..16000 -> 0..50 = 25
        ain.Raw := 8000.0; ain.RawMin := 0.0; ain.RawMax := 16000.0;
        ain.EngMin := 0.0; ain.EngMax := 50.0;
        st := 1;
    ELSIF st = 1 THEN
        IF ain.Value > 24.9 AND ain.Value < 25.1 THEN mask := mask OR 1; END_IF;
        // 2-point ctrl: heating, actual below SP-hyst -> on
        c2p.bEnable := TRUE;
        c2p.fActual := 18.0; c2p.fSetpoint := 21.0; c2p.fHysteresis := 1.0;
        st := 2;
    ELSIF st = 2 THEN
        IF c2p.bOut THEN mask := mask OR 2; END_IF;
        // step ctrl: error 6 of 8 range, 4 stages -> 3 stages
        stp.bEnable := TRUE;
        stp.fActual := 14.0; stp.fSetpoint := 20.0;
        stp.fStepRange := 8.0; stp.nStages := 4;
        st := 3;
    ELSIF st = 3 THEN
        IF stp.bStage1 AND stp.bStage2 AND stp.bStage3 AND NOT stp.bStage4 THEN
            mask := mask OR 4;
        END_IF;
        // PID: error positive -> output above min
        pid.bEnable := TRUE;
        pid.fActual := 18.0; pid.fSetpoint := 22.0;
        pid.fKp := 2.0; pid.fTi := 0.0; pid.fTd := 0.0;
        pid.fOutMin := 0.0; pid.fOutMax := 100.0;
        pid.CycleTime := 0.1;
        st := 4;
    ELSIF st = 4 THEN
        IF pid.fOut > 5.0 THEN mask := mask OR 8; END_IF;
        // heating curve: outside -10, design -15, room 20
        // flow = 30 + (80-30) * (20-(-10)) / (20-(-15)) = 30+50*30/35 ≈ 72.86
        hcv.bEnable := TRUE;
        hcv.fOutsideTemp := -10.0; hcv.fOutsideMin := -15.0;
        hcv.fRoomTemp := 20.0; hcv.fSpMin := 30.0; hcv.fSpMax := 80.0;
        st := 5;
    ELSIF st = 5 THEN
        IF hcv.fFlowSp > 70.0 AND hcv.fFlowSp < 76.0 THEN mask := mask OR 16; END_IF;
        // motor3 speed 2 -> out2 only
        mot.bEnable := TRUE; mot.Speed := 2;
        mux.Sel := 3; mux.In3 := 42.0;
        st := 6;
    ELSIF st = 6 THEN
        IF mot.bOut2 AND NOT mot.bOut1 AND NOT mot.bOut3 THEN mask := mask OR 32; END_IF;
        IF mux.Out = 42.0 THEN mask := mask OR 64; END_IF;
        // redundancy: need 2, fault1 -> out2+out3
        red.bEnable := TRUE; red.Required := 2;
        red.bFault1 := TRUE;
        st := 7;
    ELSIF st = 7 THEN
        IF red.bOut2 AND red.bOut3 AND NOT red.bOut1 THEN mask := mask OR 128; END_IF;
        // alarm: value over limit 1s delay
        alm.bEnable := TRUE;
        alm.fValue := 10.0; alm.fLimit := 5.0; alm.fHysteresis := 1.0;
        alm.tDelay := 0.5; alm.CycleTime := 0.1;
        waitN := 0;
        st := 8;
    ELSIF st = 8 THEN
        waitN := waitN + 1;
        IF alm.bAlarm THEN mask := mask OR 256; st := 9; END_IF;
        IF waitN > 40 THEN diag := 16#E8; st := 99; END_IF;
    ELSIF st = 9 THEN
        // anti-blocking: idle pump exercises after tIdle
        abp.bEnable := TRUE; abp.bDemand := FALSE;
        abp.tIdle := 0.5; abp.tRunTime := 0.4; abp.CycleTime := 0.1;
        waitN := 0;
        st := 10;
    ELSIF st = 10 THEN
        waitN := waitN + 1;
        IF abp.bExercise THEN mask := mask OR 512; st := 11; END_IF;
        IF waitN > 40 THEN diag := 16#E9; st := 99; END_IF;
    ELSIF st = 11 THEN
        // blinker toggles
        blk.bEnable := TRUE; blk.tOn := 0.2; blk.tOff := 0.2;
        blk.CycleTime := 0.1;
        waitN := 0;
        st := 12;
    ELSIF st = 12 THEN
        waitN := waitN + 1;
        IF blk.bOut THEN mask := mask OR 1024; st := 13; END_IF;
        IF waitN > 40 THEN diag := 16#EA; st := 99; END_IF;
    ELSIF st = 13 THEN
        // bar setpoint: base 21 + shift 5 clamped to +3
        bar.bEnable := TRUE;
        bar.fBaseSp := 21.0; bar.fShift := 5.0; bar.fShiftMax := 3.0;
        // delayed hysteresis
        dhy.bEnable := TRUE;
        dhy.fValue := 10.0; dhy.fOn := 5.0; dhy.fOff := 2.0;
        dhy.tOnDelay := 0.0; dhy.tOffDelay := 0.0;
        dhy.CycleTime := 0.1;
        st := 14;
    ELSIF st = 14 THEN
        IF bar.fSetpoint = 24.0 THEN mask := mask OR 2048; END_IF;
        waitN := waitN + 1;
        IF dhy.bOut THEN mask := mask OR 4096; st := 15; END_IF;
        IF waitN > 40 THEN diag := 16#EB; st := 99; END_IF;
    ELSIF st = 15 THEN
        // 3-point actuator drives open
        tp3.bEnable := TRUE; tp3.bOpen := TRUE;
        tp3.tRun := 1.0; tp3.CycleTime := 0.1;
        waitN := 0;
        st := 16;
    ELSIF st = 16 THEN
        waitN := waitN + 1;
        IF tp3.Position > 50.0 THEN mask := mask OR 8192; st := 17; END_IF;
        IF waitN > 60 THEN diag := 16#EC; st := 99; END_IF;
    ELSIF st = 17 THEN
        // circulation pump: lead1 faulted -> pump2 runs
        pump.bEnable := TRUE; pump.bDemand := TRUE;
        pump.bFault1 := TRUE;
        st := 18;
    ELSIF st = 18 THEN
        IF pump.bPump2 AND NOT pump.bPump1 THEN mask := mask OR 16384; END_IF;
        // 2-point actuator: open command, endswitch ok
        tp2.bEnable := TRUE; tp2.bIn := TRUE;
        tp2.bLimitSwitchOpen := FALSE; tp2.bLimitSwitchClose := TRUE;
        tp2.tRun := 1.0; tp2.CycleTime := 0.1;
        waitN := 0;
        st := 19;
    ELSIF st = 19 THEN
        waitN := waitN + 1;
        IF tp2.Position > 50.0 THEN mask := mask OR 32768; st := 20; END_IF;
        IF waitN > 60 THEN diag := 16#ED; st := 99; END_IF;
    END_IF;

    ain(); aout(); tp2(); tp3(); pump(); mot(); mux(); red();
    c2p(); stp(); pid(); hcv(); dmp(); alm(); abp(); blk();
    bar(); dhy();
END_PROGRAM
