// tc3_analytics_case.st — Tc3_Analytics e2e: counter+rate,
// windowed stats, ring logger, and honest engine-absent store.
PROGRAM tc3_analytics_case
VAR
    mask AT %QD0 : UDINT;
    cnt_ : FB_AnalyticsCounter;
    sts_ : FB_AnalyticsStats;
    lg_ : FB_AnalyticsLogger;
    sto_ : FB_AnalyticsStore;
    st : INT;
END_VAR
    // counter: 4 event edges in the first window, then rate>0
    IF st = 0 THEN
        mask := 0;
        cnt_.Enable := TRUE;
        cnt_.WindowTime := 0.05;
        cnt_.CycleTime := 0.01;
        cnt_.Event := FALSE;
        st := 1;
    ELSIF st = 1 THEN cnt_.Event := TRUE;  st := 2;
    ELSIF st = 2 THEN cnt_.Event := FALSE; st := 3;
    ELSIF st = 3 THEN cnt_.Event := TRUE;  st := 4;
    ELSIF st = 4 THEN cnt_.Event := FALSE; st := 5;
    ELSIF st = 5 THEN cnt_.Event := TRUE;  st := 6;
    ELSIF st = 6 THEN cnt_.Event := FALSE; st := 7;
    ELSIF st = 7 THEN cnt_.Event := TRUE;  st := 8;
    ELSIF st = 8 THEN
        cnt_.Event := FALSE;
        IF cnt_.Total = 4 THEN
            mask := mask OR 1;
            st := 9;
        END_IF;
    ELSIF st = 9 THEN
        IF cnt_.Rate > 0.0 THEN
            mask := mask OR 2;      // a window closed with events
            cnt_.Enable := FALSE;
            st := 10;
        END_IF;
    // stats: samples 2, 4, 9 -> count 3, min 2, max 9, mean 5
    ELSIF st = 10 THEN sts_.Execute := TRUE; sts_.Sample := 2.0; st := 11;
    ELSIF st = 11 THEN sts_.Sample := 4.0; st := 12;
    ELSIF st = 12 THEN sts_.Sample := 9.0; st := 13;
    ELSIF st = 13 THEN
        sts_.Execute := FALSE;
        IF sts_.Count = 3 AND sts_.Min = 2.0 AND sts_.Max = 9.0
           AND sts_.Mean > 4.9 AND sts_.Mean < 5.1 THEN
            mask := mask OR 4;
            st := 14;
        END_IF;
    // logger: 9 records -> wrap once, slot0 holds record 9
    ELSIF st = 14 THEN lg_.Record := TRUE;  lg_.Value := 10.0; lg_.Timestamp := 1.0; st := 15;
    ELSIF st = 15 THEN lg_.Record := FALSE; st := 16;
    ELSIF st = 16 THEN lg_.Record := TRUE;  lg_.Value := 20.0; lg_.Timestamp := 2.0; st := 17;
    ELSIF st = 17 THEN lg_.Record := FALSE; st := 18;
    ELSIF st = 18 THEN lg_.Record := TRUE;  lg_.Value := 30.0; lg_.Timestamp := 3.0; st := 19;
    ELSIF st = 19 THEN lg_.Record := FALSE; st := 20;
    ELSIF st = 20 THEN lg_.Record := TRUE;  lg_.Value := 40.0; lg_.Timestamp := 4.0; st := 21;
    ELSIF st = 21 THEN lg_.Record := FALSE; st := 22;
    ELSIF st = 22 THEN lg_.Record := TRUE;  lg_.Value := 50.0; lg_.Timestamp := 5.0; st := 23;
    ELSIF st = 23 THEN lg_.Record := FALSE; st := 24;
    ELSIF st = 24 THEN lg_.Record := TRUE;  lg_.Value := 60.0; lg_.Timestamp := 6.0; st := 25;
    ELSIF st = 25 THEN lg_.Record := FALSE; st := 26;
    ELSIF st = 26 THEN lg_.Record := TRUE;  lg_.Value := 70.0; lg_.Timestamp := 7.0; st := 27;
    ELSIF st = 27 THEN lg_.Record := FALSE; st := 28;
    ELSIF st = 28 THEN lg_.Record := TRUE;  lg_.Value := 80.0; lg_.Timestamp := 8.0; st := 29;
    ELSIF st = 29 THEN lg_.Record := FALSE; st := 30;
    ELSIF st = 30 THEN lg_.Record := TRUE;  lg_.Value := 90.0; lg_.Timestamp := 9.0; st := 31;
    ELSIF st = 31 THEN
        lg_.Record := FALSE;
        IF lg_.Count = 9 AND lg_.Overflow AND lg_.Slot0 = 90.0 AND lg_.LastTs = 9.0 THEN
            mask := mask OR 8;
            st := 32;
        END_IF;
    // store: provider configured -> honest engine-absent
    ELSIF st = 32 THEN
        sto_.ProviderPath := 'influx://db';
        sto_.RecordCount := 9;
        sto_.Execute := TRUE;
        st := 33;
    ELSIF st = 33 THEN
        IF sto_.Error AND sto_.ErrorId = 1 THEN
            mask := mask OR 16;
            st := 34;
        END_IF;
    ELSIF st = 34 THEN
        sto_.Execute := FALSE;
        st := 35;
    ELSIF st = 35 THEN
        sto_.ProviderPath := '';    // bad provider -> param error
        sto_.Execute := TRUE;
        st := 36;
    ELSIF st = 36 THEN
        IF sto_.Error AND sto_.ErrorId = 2 THEN
            mask := mask OR 32;
            st := 37;
        END_IF;
    END_IF;

    cnt_();
    sts_();
    lg_();
    sto_();
END_PROGRAM
