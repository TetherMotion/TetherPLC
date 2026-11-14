// tc3_mc3camtbl_case.st — Tc3_Mc3Camming cam-table e2e over the
// tplc-cam service: FB_CamTable object creation + WritePt/Config
// edges, MC_CamTableSelect, MC_CamIn CSP coupling (slave tracks
// master+5000 through an identity cam table), MC_CamScaling offset
// update, MC_ReadCamInfo and MC_CamRemove. Mask 0xff (8 checks).
PROGRAM Tc3Mc3CamTblCase
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    prevSt : INT;
    pulse  : BOOL;
    started : BOOL;
    wrN    : INT;
    axM    : UDINT;
    axS    : UDINT;
    camH   : UDINT;
    crM    : FB_MotionAxis;
    crS    : FB_MotionAxis;
    wrM    : FB_MotionAxis;
    wrS    : FB_MotionAxis;
    pwrM   : MC_Power;
    pwrS   : MC_Power;
    tbl    : FB_CamTable;
    camI   : MC_CamIn;
    camS   : MC_CamScaling;
    camSel : MC_CamTableSelect;
    camR   : MC_CamRemove;
    camInfo: MC_ReadCamInfo;
    dSel   : BOOL;
    dScl   : BOOL;
    dRem   : BOOL;
    dInfo1 : BOOL;
    dInfo2 : BOOL;
    iFound : UDINT;
    iCam   : UDINT;
    iMaster: UDINT;
    jFound : UDINT;
    gapSeen : BOOL;
    selEx  : BOOL;
    camInEx: BOOL;
    camScEx: BOOL;
    camRemEx: BOOL;
    infoEx : BOOL;
END_VAR

pulse := FALSE;
IF NOT started THEN
    pulse := TRUE; started := TRUE;
ELSIF st <> prevSt THEN
    pulse := TRUE;
END_IF;
prevSt := st;

// ---- level inputs, computed before the FB calls ----
tbl.WritePt := FALSE; tbl.ClearTable := FALSE; tbl.Config := FALSE;
IF st = 3 THEN
    IF wrN = 0 THEN
        tbl.PtIndex := 0; tbl.PtMaster := 0.0; tbl.PtSlave := 0.0;
        tbl.WritePt := TRUE;
    ELSIF wrN = 2 THEN
        tbl.PtIndex := 1; tbl.PtMaster := 100000.0;
        tbl.PtSlave := 100000.0; tbl.WritePt := TRUE;
    ELSIF wrN = 3 THEN
        tbl.Config := TRUE; tbl.TableCount := 2;
        tbl.Periodic := FALSE; tbl.Continuity := 0;
    END_IF;
END_IF

selEx := st = 5;
camInEx := st = 6;
camScEx := st = 7;
camRemEx := st = 9;
infoEx := (st = 7) OR (st = 9);

camI(Execute := camInEx, Master := axM, Slave := axS,
     CamObject := camH, SlaveOffset := 5000.0);
camS(Execute := camScEx, Slave := axS, CamObject := camH,
     SlaveOffset := 8000.0);
camSel(Execute := selEx, CamRef := camH, Periodic := FALSE,
       ContinuityRequirement := 0, CamObjectIn := camH);
camR(Execute := camRemEx, Slave := axS, CamObject := camH);
camInfo(Execute := infoEx, Slave := axS);
tbl();

IF camSel.Done THEN dSel := TRUE; END_IF;
IF camS.Done THEN dScl := TRUE; END_IF;
IF camR.Done THEN dRem := TRUE; END_IF;
IF camInfo.Done THEN
    IF st = 7 THEN
        dInfo1 := TRUE;
        iFound := camInfo.NumberOfCouplingsFound;
        iCam := camInfo.CamHandle;
        iMaster := camInfo.MasterAxis;
    ELSIF st = 9 THEN
        dInfo2 := TRUE;
        jFound := camInfo.NumberOfCouplingsFound;
    END_IF;
END_IF;

IF st = 0 THEN
    crM.UNITS_PER_REV := 1000; crM.MAX_VEL := 100000;
    crM.MAX_ACCEL := 200000; crM.MAX_DECEL := 200000;
    crM.POS_MIN := -100000; crM.POS_MAX := 100000;
    crM.HOME_OFFSET := 0; crM.FOLLOW_WINDOW := 3; crM.FLAGS := 1;
    crM.OP := 1; crM();
    IF crM.AXIS <> 0 THEN axM := crM.AXIS; END_IF;
    crS.UNITS_PER_REV := 1000; crS.MAX_VEL := 100000;
    crS.MAX_ACCEL := 200000; crS.MAX_DECEL := 200000;
    crS.POS_MIN := -100000; crS.POS_MAX := 100000;
    crS.HOME_OFFSET := 0; crS.FOLLOW_WINDOW := 3; crS.FLAGS := 1;
    crS.OP := 1; crS();
    IF crS.AXIS <> 0 THEN axS := crS.AXIS; END_IF;
    IF axM <> 0 AND axS <> 0 THEN mask := mask + 1; END_IF; // bit0
    st := 1;
ELSIF st = 1 THEN
    pwrM(Enable := TRUE, Enable_Positive := TRUE,
         Enable_Negative := TRUE, Axis := axM);
    pwrS(Enable := TRUE, Enable_Positive := TRUE,
         Enable_Negative := TRUE, Axis := axS);
    IF pwrM.Status AND pwrS.Status THEN
        mask := mask + 2; st := 2;                     // bit1
    END_IF;
ELSIF st = 2 THEN
    // master PV move; slave into CSP mode for 0x607A streaming
    wrM.AXIS := axM; wrM.OP := 5; wrM.INDEX := 16#6060;
    wrM.VALUE := 3; wrM();
    wrM.OP := 5; wrM.INDEX := 16#60FF; wrM.VALUE := 5000; wrM();
    wrS.AXIS := axS; wrS.OP := 5; wrS.INDEX := 16#6060;
    wrS.VALUE := 8; wrS();
    wrN := 0;
    st := 3;
ELSIF st = 3 THEN
    // cam table point writes + config (wrN sequencer, ~5 scans)
    wrN := wrN + 1;
    IF wrN >= 5 THEN st := 4; END_IF;
ELSIF st = 4 THEN
    IF tbl.Handle <> 0 THEN
        camH := tbl.Handle;
        mask := mask + 4; st := 5;                     // bit2
    END_IF;
ELSIF st = 5 THEN
    IF dSel THEN
        mask := mask + 8; st := 6;                     // bit3
    END_IF;
ELSIF st = 6 THEN
    // MC_CamIn bound on entry; slave must converge to mpos+5000
    IF camI.InSync THEN
        mask := mask + 16; st := 7;                    // bit4
    END_IF;
ELSIF st = 7 THEN
    // scaling update + cam info read
    IF dScl AND dInfo1 THEN
        IF iFound = 1 AND iCam = camH AND iMaster = axM THEN
            mask := mask + 32;                        // bit5
        END_IF;
        st := 8;
    END_IF;
ELSIF st = 8 THEN
    // wait for the +3000 target jump to break sync, then re-converge
    IF NOT camI.InSync THEN gapSeen := TRUE; END_IF;
    IF gapSeen AND camI.InSync THEN
        mask := mask + 64; st := 9;                    // bit6
    END_IF;
ELSIF st = 9 THEN
    // cam remove + second info read must show no coupling
    IF dRem AND dInfo2 THEN
        IF jFound = 0 THEN
            mask := mask + 128;                       // bit7
        END_IF;
        st := 10;
    END_IF;
ELSIF st = 10 THEN
    // hold; expect written at final step
END_IF;
END_PROGRAM
