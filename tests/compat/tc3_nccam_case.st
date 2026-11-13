// tc3_nccam_case.st — Tc3_NC_Camming e2e: cam coupling over the
// tplc-motion backend. Master axis runs a velocity move; the slave
// axis is in CSP mode and tracks master*2 through the cam table.
// Also exercises ClearTable and the WritePt point-write edge.
PROGRAM tc3_nccam_case
VAR
    mask   AT %QD0 : DINT;
    st     : INT;
    ax     : FB_MotionAxis;
    axM    : UDINT;
    axS    : UDINT;
    axC    : FB_MotionAxis;   // raw object access for the case
    cpl    : FB_McCamCoupling;
    wrSt   : INT;
    wrN    : INT;
    mPos   : DINT;
    sPos   : DINT;
END_VAR

// ---- st=0: create the two axes ----
IF st = 0 THEN
    mask := 0;
    wrSt := 0;
    wrN := 0;
    ax.UNITS_PER_REV := 1000;
    ax.MAX_VEL := 4000;
    ax.MAX_ACCEL := 20000;
    ax.MAX_DECEL := 20000;
    ax.POS_MIN := -100000;
    ax.POS_MAX := 100000;
    ax.HOME_OFFSET := 0;
    ax.FOLLOW_WINDOW := 10;
    ax.FLAGS := 1;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axM := ax.AXIS; END_IF;
    ax.OP := 1; ax();
    IF ax.AXIS <> 0 THEN axS := ax.AXIS; END_IF;
    IF axM <> 0 AND axS <> 0 THEN mask := mask + 1; END_IF;
    st := 1;
END_IF;

// ---- st=1: enable both axes (controlword 0x000F) ----
IF st = 1 THEN
    axC.AXIS := axM;
    axC.OP := 5; axC.INDEX := 16#6040; axC.VALUE := 16#000F; axC();
    axC.AXIS := axS;
    axC.OP := 5; axC.INDEX := 16#6040; axC.VALUE := 16#000F; axC();
    st := 2;
END_IF;

// ---- st=2: slave into CSP mode (0x6060=8) ----
IF st = 2 THEN
    axC.AXIS := axS;
    axC.OP := 5; axC.INDEX := 16#6060; axC.VALUE := 8; axC();
    st := 3;
END_IF;

// ---- st=3: load the cam table directly into the member arrays ----
IF st = 3 THEN
    cpl.MasterPosT[0] := 0.0;    cpl.SlavePosT[0] := 0.0;
    cpl.MasterPosT[1] := 1000.0; cpl.SlavePosT[1] := 2000.0;
    cpl.MasterPosT[2] := 2000.0; cpl.SlavePosT[2] := 4000.0;
    cpl.TableCount := 3;
    IF cpl.TableCountOut = 3 THEN
        mask := mask + 2;      // bit1: table loaded (3 pts)
        st := 4;
    END_IF;
END_IF;

// ---- st=4: master velocity move + coupling live ----
IF st = 4 THEN
    // master: PV mode, target velocity 1000 u/s
    axC.AXIS := axM;
    axC.OP := 5; axC.INDEX := 16#6060; axC.VALUE := 3; axC();
    axC.INDEX := 16#60FF; axC.VALUE := 1000; axC();
    cpl.Execute := TRUE;
    cpl.MasterAxis := axM;
    cpl.SlaveAxis := axS;
    cpl.Periodic := FALSE;
    cpl.Invert := FALSE;
    cpl.MasterScaling := 1.0;
    cpl.SlaveScaling := 1.0;
    cpl.MasterOffset := 0.0;
    cpl.SlaveOffset := 0.0;
    st := 5;
END_IF;

// ---- st=5: wait for the master to cross ~1000, verify slave ~= 2x --
IF st = 5 THEN
    axC.AXIS := axM; axC.OP := 4; axC.INDEX := 16#6064; axC();
    mPos := axC.VALUE;
    IF mPos > 1000 AND mPos < 1400 THEN
        axC.AXIS := axS; axC.OP := 4; axC.INDEX := 16#6064; axC();
        sPos := axC.VALUE;
        // slave target = 2*master at write time; allow scan lag
        IF sPos >= 1900 AND sPos <= 2900 THEN
            mask := mask + 4;  // bit2: cam coupling tracks 2x
            st := 6;
        END_IF;
    ELSIF mPos >= 1400 THEN
        st := 90;            // missed window -> fail
    END_IF;
END_IF;

// ---- st=6: enable periodic wrap; master keeps running past 2000 --
IF st = 6 THEN
    cpl.Periodic := TRUE;
    st := 7;
    wrN := 0;
END_IF;
IF st = 7 THEN
    axC.AXIS := axM; axC.OP := 4; axC.INDEX := 16#6064; axC();
    mPos := axC.VALUE;
    IF mPos > 2050 THEN
        axC.AXIS := axS; axC.OP := 4; axC.INDEX := 16#6064; axC();
        sPos := axC.VALUE;
        // wrapped master ~50-200 -> slave ~100-400
        IF sPos >= 0 AND sPos <= 600 THEN
            mask := mask + 8;  // bit3: periodic wrap
            st := 8;
        END_IF;
    ELSIF wrN > 400 THEN
        st := 90;
    ELSE
        wrN := wrN + 1;
    END_IF;
END_IF;

// ---- st=8: stop coupling, clear table ----
IF st = 8 THEN
    cpl.Execute := FALSE;
    cpl.ClearTable := TRUE;
    st := 9;
END_IF;
IF st = 9 THEN
    IF cpl.TableCountOut = 0 THEN
        mask := mask + 16;     // bit4: table cleared
        st := 10;
    END_IF;
END_IF;

// ---- st=10: WritePt edge writes one point ----
IF st = 10 THEN
    cpl.WritePt := FALSE;
    IF wrSt = 0 THEN
        cpl.PtIndex := 4; cpl.PtMaster := 500.0; cpl.PtSlave := 900.0;
        cpl.WritePt := TRUE; wrSt := 1;
    ELSIF wrSt = 1 THEN
        cpl.WritePt := FALSE;
        IF cpl.TableCountOut = 5 AND cpl.SlavePosT[4] = 900.0 THEN
            mask := mask + 32;  // bit5: WritePt edge
            st := 90;
        END_IF;
    END_IF;
END_IF;

IF st = 90 THEN
    // done
END_IF

// coupling inputs must be driven every scan while active
cpl();
END_PROGRAM
