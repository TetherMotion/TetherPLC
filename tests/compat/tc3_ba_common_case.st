// tc3_ba_common_case.st — pure-ST e2e for tc3_ba_common.
PROGRAM tc3_ba_common_case
VAR
    rft : FB_BA_RFTrig;
    cov : FB_BA_ATrigCOV;
    rlm : FB_BA_RampLmt;
    sw2 : FB_BA_Swi2P;
    swh : FB_BA_SwiHys2P;
    pid : FB_BA_PIDCtrl;
    seq : FB_BA_SeqCtrl;
    arr : FB_BA_DynamicArray;
    rem : UDINT;
    okw : BOOL;
    st : INT;
    mask AT %QD0 : UDINT;
    zz : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    mask := zz;
    IF st = 0 THEN
        zz := 0;
        pid.bEnable := TRUE;
        pid.fActual := 18.0; pid.fSetpoint := 22.0;
        pid.fKp := 2.0; pid.fTi := 0.0;
        pid.fOutMin := 0.0; pid.fOutMax := 100.0;
        pid.CycleTime := 0.1;
        rem := F_BA_RemSecsTp(10, 40);
        okw := F_BA_IsWeekdayValid(5);
        IF rem = 30 AND okw THEN zz := zz OR 1; END_IF;
        // COV trigger: first call reports
        cov.bEnable := TRUE;
        cov.fIn := 10.0;
        cov.fDelta := 1.0;
        st := 1;
    ELSIF st = 1 THEN
        IF pid.fOut > 5.0 THEN zz := zz OR 128; END_IF;
        IF cov.bTrig THEN zz := zz OR 2; END_IF;
        cov.fIn := 10.5;           // within delta: no trig
        st := 2;
    ELSIF st = 2 THEN
        IF NOT cov.bTrig THEN zz := zz OR 4; END_IF;
        cov.fIn := 12.5;           // delta 2.5: trig
        st := 3;
    ELSIF st = 3 THEN
        IF cov.bTrig THEN zz := zz OR 8; END_IF;
        // ramp limiter
        rlm.bEnable := TRUE;
        rlm.fIn := 50.0; rlm.fSlewUp := 10.0; rlm.fSlewDown := 10.0;
        rlm.fMin := 0.0; rlm.fMax := 40.0;
        rlm.CycleTime := 0.1;
        st := 4;
    ELSIF st = 4 THEN
        IF rlm.fOut = 40.0 THEN zz := zz OR 16; st := 5; END_IF;
        IF rlm.fOut > 40.0 THEN diag := 16#E4; st := 99; END_IF;
    ELSIF st = 5 THEN
        // switches
        sw2.bEnable := TRUE;
        sw2.fIn := 6.0; sw2.fOn := 5.0; sw2.fOff := 1.0;
        swh.bEnable := TRUE;
        swh.fIn := 18.0; swh.fSp := 21.0; swh.fHys := 2.0;
        swh.bInvert := FALSE;      // heating: on below SP-hys
        st := 6;
    ELSIF st = 6 THEN
        IF sw2.bOut THEN zz := zz OR 32; END_IF;
        IF swh.bOut THEN zz := zz OR 64; END_IF;
        // PID
        // seq: demand 60 across 3 stages -> s1 100, s2 80, s3 0
        seq.bEnable := TRUE;
        seq.fDemand := 60.0; seq.nStages := 3;
        st := 7;
    ELSIF st = 7 THEN
        st := 20;
    ELSIF st = 20 THEN
        IF seq.fStage1 > 99.0 AND seq.fStage2 > 75.0 AND
           seq.fStage2 < 85.0 AND seq.fStage3 < 1.0 THEN
            zz := zz OR 256;
        END_IF;
        // dynamic array: set/get/remove
        arr.bEnable := TRUE;
        arr.Op := 1;               // set
        arr.Key := 7;
        arr.Value := 42.0;
        st := 8;
    ELSIF st = 8 THEN
        IF arr.Done THEN zz := zz OR 512; END_IF;
        arr.Op := 2;               // get
        arr.Key := 7;
        st := 9;
    ELSIF st = 9 THEN
        IF arr.Found AND arr.OutValue = 42.0 THEN zz := zz OR 1024; END_IF;
        arr.Op := 3;               // remove
        arr.Key := 7;
        st := 10;
    ELSIF st = 10 THEN
        IF arr.Done THEN zz := zz OR 2048; END_IF;
        arr.Op := 4;               // find (should miss)
        arr.Key := 7;
        st := 11;
    ELSIF st = 11 THEN
        IF NOT arr.Found THEN zz := zz OR 4096; END_IF;
        arr.Op := 0;
        // edge trigger
        rft.bIn := TRUE;
        st := 12;
    ELSIF st = 12 THEN
        IF rft.bRise THEN zz := zz OR 8192; END_IF;
        rft.bIn := FALSE;
        st := 13;
    ELSIF st = 13 THEN
        IF rft.bFall THEN zz := zz OR 16384; END_IF;
        st := 14;
    END_IF;

    rft(); cov(); rlm(); sw2(); swh(); pid(); seq(); arr();
END_PROGRAM
