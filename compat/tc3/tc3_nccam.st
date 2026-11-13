// tc3_nccam.st — TF5050 Tc3_NC_Camming compatibility surface.
//
// Electronic camming couples a slave axis to a master axis through
// a cam table (master position -> slave position).  TwinCAT models
// this as MC_CAM_REF table objects shared between the motion-
// function FBs (MC_WriteMotionFunction*) and the coupling FBs
// (MC_CamTableSelect/MC_CamIn/MC_CamOut/MC_CamAdd/...).  This
// dialect has no REFERENCE TO, so the table lives inside the
// coupling FB as public member arrays MasterPosT/SlavePosT
// (LREAL, sorted ascending by master position, up to 64 points) —
// the documented flattening of MC_CAM_REF.  Points can also be
// written through the Pt* edge inputs (the
// MC_WriteMotionFunctionPoint shape).
//
// FB_McCamAxis (builtin FB_MotionAxis) supplies the axis handles:
// MasterAxis/SlaveAxis are the UDINT handles returned by
// FB_MotionAxis OP=1 create — the same AXIS_REF flattening used by
// compat/tc2/tc2_mc2.st.
//
// Semantics: while Execute is TRUE the FB polls both axes, reads
// the master position actual (0x6064), applies
//   mt = masterActual * MasterScaling + MasterOffset
// looks the cam table up (linear interpolation, clamped ends,
// optional periodic wrap), applies
//   st = slaveTablePos * SlaveScaling + SlaveOffset
// and writes the slave position target (0x607A) every scan.  The
// slave axis must be powered and in CSP mode (the case/program
// sets 0x6060=8 once; the FB does not change modes).  Invert=TRUE
// swaps the coupling direction (MC_CamIn vs MC_CamOut roles).
//
// Mapping notes (documented deviations):
//   - MC_CamAdd/Exchange/Remove multi-plate blending, online
//     change modes, cam-table file I/O, slave-dynamics and
//     characteristics reads are not implemented; single-table
//     linear coupling only.
//   - LREAL engineering units map onto the backend's int32 user
//     units at the axis boundary (same documented gap as tc2_mc2).
//
// SPDX-License-Identifier: Apache-2.0 OR MPL-2.0

FUNCTION_BLOCK FB_McCamCoupling
VAR_INPUT
    Execute : BOOL;              // coupling active while TRUE
    MasterAxis : UDINT;          // FB_MotionAxis handle
    SlaveAxis : UDINT;           // FB_MotionAxis handle
    Periodic : BOOL;             // wrap master position
    Invert : BOOL;               // swap master/slave roles
    CamTableID : UDINT;          // echoed on CamTableID output
    MasterOffset : LREAL := 0.0;
    SlaveOffset : LREAL := 0.0;
    MasterScaling : LREAL := 1.0;
    SlaveScaling : LREAL := 1.0;
    // MC_WriteMotionFunctionPoint shape (edge writes one point)
    WritePt : BOOL;
    PtIndex : UDINT;             // 0..63
    PtMaster : LREAL;
    PtSlave : LREAL;
    ClearTable : BOOL;           // edge: TableCount := 0
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Busy : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    InSync : BOOL;
    CamTableIDOut : UDINT;
    CamSlavePos : LREAL;         // last interpolated slave pos
    TableCountOut : UDINT;
END_VAR
VAR
    axM : FB_MotionAxis;
    axS : FB_MotionAxis;
    prevExec : BOOL;
    prevWr : BOOL;
    prevClr : BOOL;
    i : UDINT;
    mt : LREAL;
    st : LREAL;
    frac : LREAL;
    range : LREAL;
    wraps : UDINT;
    masterH : UDINT;
    slaveH : UDINT;
    mActual : DINT;
    seg : UDINT;
    found : BOOL;
    // public table storage (MC_CAM_REF flattening)
    MasterPosT : ARRAY[0..63] OF LREAL;
    SlavePosT : ARRAY[0..63] OF LREAL;
    TableCount : UDINT;
END_VAR
    Busy := FALSE;
    Error := FALSE;
    ErrorID := 0;
    CamTableIDOut := CamTableID;
    TableCountOut := TableCount;

    // table maintenance edges
    IF ClearTable AND NOT prevClr THEN
        TableCount := 0;
    END_IF;
    IF WritePt AND NOT prevWr AND PtIndex < 64 THEN
        MasterPosT[PtIndex] := PtMaster;
        SlavePosT[PtIndex] := PtSlave;
        IF PtIndex >= TableCount THEN
            TableCount := PtIndex + 1;
        END_IF;
    END_IF;
    prevWr := WritePt;
    prevClr := ClearTable;

    masterH := MasterAxis;
    slaveH := SlaveAxis;
    IF Invert THEN
        masterH := SlaveAxis;
        slaveH := MasterAxis;
    END_IF;

    IF Execute THEN
        // advance dynamics once per scan on both axes
        axM.AXIS := masterH; axM.OP := 3; axM();
        axS.AXIS := slaveH;   axS.OP := 3; axS();

        IF axM.STATUS < 0 OR axS.STATUS < 0 THEN
            Error := TRUE;
            ErrorID := 1;
        ELSIF TableCount < 2 THEN
            // need at least two points for interpolation
            InSync := FALSE;
        ELSE
            // master position actual -> scaled table coordinate
            axM.OP := 6; axM();
            mActual := axM.POS_ACTUAL;
            mt := DINT_TO_LREAL(mActual) * MasterScaling + MasterOffset;

            // locate segment (table sorted ascending by master pos)
            seg := 0;
            found := FALSE;
            FOR i := 0 TO TableCount - 2 DO
                IF mt <= MasterPosT[i + 1] THEN
                    seg := i;
                    found := TRUE;
                END_IF;
            END_FOR;
            IF mt < MasterPosT[0] THEN
                st := SlavePosT[0];
            ELSIF mt > MasterPosT[TableCount - 1] THEN
                IF Periodic AND
                   MasterPosT[TableCount - 1] > MasterPosT[0] THEN
                    range := MasterPosT[TableCount - 1] - MasterPosT[0];
                    wraps := 0;
                    WHILE mt > MasterPosT[TableCount - 1] AND wraps < 1024 DO
                        mt := mt - range;
                        wraps := wraps + 1;
                    END_WHILE;
                    // re-locate after wrap
                    FOR i := 0 TO TableCount - 2 DO
                        IF mt <= MasterPosT[i + 1] THEN
                            seg := i;
                            found := TRUE;
                        END_IF;
                    END_FOR;
                    IF found AND MasterPosT[seg + 1] > MasterPosT[seg] THEN
                        frac := (mt - MasterPosT[seg]) /
                                (MasterPosT[seg + 1] - MasterPosT[seg]);
                        st := SlavePosT[seg] +
                              frac * (SlavePosT[seg + 1] - SlavePosT[seg]);
                    ELSE
                        st := SlavePosT[TableCount - 1];
                    END_IF;
                ELSE
                    st := SlavePosT[TableCount - 1];
                END_IF;
            ELSIF NOT found THEN
                st := SlavePosT[TableCount - 1];
            ELSE
                IF MasterPosT[seg + 1] > MasterPosT[seg] THEN
                    frac := (mt - MasterPosT[seg]) /
                            (MasterPosT[seg + 1] - MasterPosT[seg]);
                    st := SlavePosT[seg] +
                          frac * (SlavePosT[seg + 1] - SlavePosT[seg]);
                ELSE
                    st := SlavePosT[seg];
                END_IF;
            END_IF;

            // slave target in backend units
            st := st * SlaveScaling + SlaveOffset;
            CamSlavePos := st;
            axS.OP := 5;
            axS.INDEX := 16#607A;
            axS.VALUE := LREAL_TO_DINT(st);
            axS();
            IF axS.STATUS < 0 THEN
                Error := TRUE;
                ErrorID := 2;
            END_IF;
            InSync := NOT Error;
        END_IF;
        Done := Execute;
    ELSE
        InSync := FALSE;
        Done := FALSE;
        CamSlavePos := 0.0;
    END_IF;
    prevExec := Execute;
END_FUNCTION_BLOCK
