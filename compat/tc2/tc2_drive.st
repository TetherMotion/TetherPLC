// tc2_drive.st — Tc2_Drive compatibility surface.
//
// TwinCAT's drive library talks to EtherCAT drives through two
// channels: CoE (CiA-402 objects, covered by compat/tc2/tc2_mc2.st
// over FB_MotionAxis) and SoE (Sercos-over-EtherCAT mailbox services,
// S-0-xxxx / P-0-xxxx).  This file implements the SoE side over the
// builtin FB_Soe (tplc-soe service), which bridges the ec.soe.*
// params exposed by a Tether IO endpoint (tether_io_motion_server
// sim contract; a real master would bridge the drive's SoE mailbox
// services under the same names).
//
// FB_Soe (builtin, layout FB_Soe):
//   OP      1 = read state, 2 = write password, 3 = write command
//           control, 4 = execute command, 5 = reset,
//           6 = write AX5000 motor ctrl word, 7 = write AX5000 park
//   SLAVE   drive port address (0..15 in the sim)
//   VALUE   u32 payload for the write ops
//   outputs STATUS (service rc, 0 ok / negative error), ERRID, plus
//           the full state snapshot: STATUSW CMDSTATE PASSWD DIAGNUM
//           CLASSX DIAGCOUNT DIAGLIST[0..7] AMPTEMP MTEMPT DCVOLT
//           DCCURR AXMAINS AXCTRL AXPARK AXFWREV AXFWBUILD CMDCTRL
//           DIAGMSG (STRING[63]).
//
// stDriveRef mapping: TwinCAT's ST_DriveRef is a drive-manager
// reference structure; this dialect has no REFERENCE TO, so the SoE
// FBs take the drive's port address as a UDINT `Drive` input.  The
// SimplePlcMotion FBs take a FB_MotionAxis UDINT handle instead
// (same AXIS_REF flattening as tc2_mc2.st).
//
// Mapping notes (documented deviations):
//   - SoE command execution is simulated: cmd_state reads 1
//     (running) for ~100 ms of drive clock after exec, then 2
//     (done); a command id with bit15 set latches error state 3 and
//     a fault bit in the status word until reset.
//   - FB_SoEAX5000FirmwareUpdate reports the simulated firmware
//     revision/build; it does not flash firmware (no firmware pool).
//   - The F_ConvWordToST* conversion functions return TwinCAT
//     STRUCTs (ST_AX5000_C1D etc.) which this dialect cannot model;
//     they are not provided.  The raw status/class-1-diagnosis words
//     are available on the read FBs' outputs.
//   - Positions/values are the backend's native units; LREAL
//     engineering-unit scaling is the caller's responsibility.
//
// SPDX-License-Identifier: Apache-2.0 OR MPL-2.0

// ---- shared read helper: one FB_Soe state read per call ----------
// (each public read FB owns a private FB_Soe instance)

// ---- FB_SoEReadCommandState --------------------------------------
// Reads the SoE command state (0 idle, 1 running, 2 done, 3 error).
FUNCTION_BLOCK FB_SoEReadCommandState
VAR_INPUT
    Drive : UDINT;                 // SoE port address
    Execute : BOOL;
    stDriveRef : UDINT;            // alias accepted (= Drive)
END_VAR
VAR_OUTPUT
    CommandState : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            CommandState := raw.CMDSTATE;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// ---- FB_SoEReadStatus — S-0-0135 drive status word ----------------
// Generic status read used by the AX5000/IndraDrive conversion
// functions and for fault monitoring.
FUNCTION_BLOCK FB_SoEReadStatus
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    StatusWord : UDINT;
    CommandState : UDINT;
    Password : UDINT;
    CmdCtrl : UDINT;
    AxCtrl : UDINT;
    AxPark : UDINT;
    AxFwRev : UDINT;
    AxFwBuild : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            StatusWord := raw.STATUSW;
            CommandState := raw.CMDSTATE;
            Password := raw.PASSWD;
            CmdCtrl := raw.CMDCTRL;
            AxCtrl := raw.AXCTRL;
            AxPark := raw.AXPARK;
            AxFwRev := raw.AXFWREV;
            AxFwBuild := raw.AXFWBUILD;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// ---- FB_SoEReadDiagMessage ---------------------------------------
// Reads the Sercos diagnosis message string (S-0-0095).
FUNCTION_BLOCK FB_SoEReadDiagMessage
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    DiagMessage : STRING[63];
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            DiagMessage := raw.DIAGMSG;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// ---- FB_SoEReadDiagNumber ----------------------------------------
FUNCTION_BLOCK FB_SoEReadDiagNumber
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    DiagNumber : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            DiagNumber := raw.DIAGNUM;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// ---- FB_SoEReadDiagNumberList ------------------------------------
// DiagList is a public member array; DiagCount bounds the valid
// entries (TwinCAT returns a list structure; flattened here).
FUNCTION_BLOCK FB_SoEReadDiagNumberList
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    DiagCount : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
    i : UDINT;
    DiagList : ARRAY[0..7] OF UDINT;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            DiagCount := raw.DIAGCOUNT;
            FOR i := 0 TO 7 DO
                DiagList[i] := raw.DIAGLIST[i];
            END_FOR;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// ---- FB_SoEReadClassXDiag ----------------------------------------
FUNCTION_BLOCK FB_SoEReadClassXDiag
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    ClassXDiag : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            ClassXDiag := raw.CLASSX;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// ---- current-value readers (S-0-0112/0113/0380/0381 family) -------
// Each reads the matching f32 field from the state snapshot.

FUNCTION_BLOCK FB_SoEReadAmplifierTemperature
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    Value : LREAL;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            Value := raw.AMPTEMP;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SoEReadMotorTemperature
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    Value : LREAL;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            Value := raw.MTEMPT;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SoEReadDcBusVoltage
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    Value : LREAL;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            Value := raw.DCVOLT;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SoEReadDcBusCurrent
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    Value : LREAL;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            Value := raw.DCCURR;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// ---- write FBs: Execute rising edge -> one mailbox op -------------

// FB_SoEReset — S-0-0099 drive reset (no NC reset).
FUNCTION_BLOCK FB_SoEReset
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 5;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// FB_SoEWritePassword — S-0-0267 drive password.
FUNCTION_BLOCK FB_SoEWritePassword
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    Password : UDINT;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 2;
        raw.SLAVE := Drive;
        raw.VALUE := UDINT_TO_DINT(Password);
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// FB_SoEExecuteCommand — execute an SoE command id.
FUNCTION_BLOCK FB_SoEExecuteCommand
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    CommandId : UDINT;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 4;
        raw.SLAVE := Drive;
        raw.VALUE := UDINT_TO_DINT(CommandId);
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// FB_SoEWriteCommandControl — command control word.
FUNCTION_BLOCK FB_SoEWriteCommandControl
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    CommandControl : UDINT;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 3;
        raw.SLAVE := Drive;
        raw.VALUE := UDINT_TO_DINT(CommandControl);
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// ---- AX5000 SoE ----------------------------------------------------

// FB_SoEAX5000ReadActMainVoltage — P-0-0200 peak mains voltage.
FUNCTION_BLOCK FB_SoEAX5000ReadActMainVoltage
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    Voltage : LREAL;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            Voltage := raw.AXMAINS;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// FB_SoEAX5000SetMotorCtrlWord — motor control word write.
// bForceUnlock is accepted for signature compatibility (the sim has
// no unlock interlock).
FUNCTION_BLOCK FB_SoEAX5000SetMotorCtrlWord
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    MotorCtrlWord : UDINT;
    bForceUnlock : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 6;
        raw.SLAVE := Drive;
        raw.VALUE := UDINT_TO_DINT(MotorCtrlWord);
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// FB_SoEAX5000FirmwareUpdate — reports the simulated firmware
// revision/build; no firmware pool or flashing is simulated.
FUNCTION_BLOCK FB_SoEAX5000FirmwareUpdate
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    FirmwareRevision : UDINT;
    FirmwareBuild : UDINT;
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 1;
        raw.SLAVE := Drive;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            FirmwareRevision := raw.AXFWREV;
            FirmwareBuild := raw.AXFWBUILD;
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// FB_SoEAX5000ParkAxis — park/unpark the AX5000 axis.
FUNCTION_BLOCK FB_SoEAX5000ParkAxis
VAR_INPUT
    Drive : UDINT;
    Execute : BOOL;
    Park : BOOL;
    stDriveRef : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    raw : FB_Soe;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        raw.OP := 7;
        raw.SLAVE := Drive;
        IF Park THEN
            raw.VALUE := 1;
        ELSE
            raw.VALUE := 0;
        END_IF;
        raw();
        IF raw.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := raw.ERRID;
        ELSE
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// ---- SimplePlcMotion: CoE/SoE drive enable + velocity -------------
// TwinCAT's SimplePlcMotion blocks are drive-manager level commands;
// over the simulated DS402 axis they map to the standard
// controlword/mode/target-velocity objects via FB_MotionAxis.
// `Drive` here is a FB_MotionAxis handle (UDINT), not a SoE port.

// FB_CoEDriveEnable — EtherCAT (CoE) drive enable.
FUNCTION_BLOCK FB_CoEDriveEnable
VAR_INPUT
    Execute : BOOL;
    Enable : BOOL;
    stDriveRef : UDINT;
    Drive : UDINT;                 // FB_MotionAxis handle
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        ax.AXIS := Drive;
        ax.OP := 5;
        ax.INDEX := 16#6040;
        IF Enable THEN
            ax.VALUE := 16#000F;
        ELSE
            ax.VALUE := 0;
        END_IF;
        ax();
        IF ax.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := ax.ERRID;
        ELSE
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// FB_SoEDriveEnable — Sercos (SoE) drive enable. Same DS402 object
// surface; the EtherCAT-vs-Sercos distinction is transport-level and
// not modeled by the simulated backend.
FUNCTION_BLOCK FB_SoEDriveEnable
VAR_INPUT
    Execute : BOOL;
    Enable : BOOL;
    stDriveRef : UDINT;
    Drive : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
    prevEx : BOOL;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute AND NOT prevEx THEN
        ax.AXIS := Drive;
        ax.OP := 5;
        ax.INDEX := 16#6040;
        IF Enable THEN
            ax.VALUE := 16#000F;
        ELSE
            ax.VALUE := 0;
        END_IF;
        ax();
        IF ax.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := ax.ERRID;
        ELSE
            bDone := TRUE;
        END_IF;
    END_IF;
    prevEx := Execute;
END_FUNCTION_BLOCK

// FB_CoEDriveMoveVelocity — profile-velocity move while Execute is
// held (level-driven: mode + target velocity rewritten each scan).
FUNCTION_BLOCK FB_CoEDriveMoveVelocity
VAR_INPUT
    Execute : BOOL;
    Velocity : LREAL;             // backend int units
    stDriveRef : UDINT;
    Drive : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute THEN
        ax.AXIS := Drive;
        ax.OP := 3;                // advance dynamics once per scan
        ax();
        ax.OP := 5;
        ax.INDEX := 16#6060;
        ax.VALUE := 3;             // ProfileVelocity
        ax();
        ax.INDEX := 16#60FF;
        ax.VALUE := LREAL_TO_DINT(Velocity);
        ax();
        IF ax.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := ax.ERRID;
        ELSE
            bDone := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// FB_SoEDriveMoveVelocity — Sercos variant; same DS402 surface.
FUNCTION_BLOCK FB_SoEDriveMoveVelocity
VAR_INPUT
    Execute : BOOL;
    Velocity : LREAL;
    stDriveRef : UDINT;
    Drive : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ax : FB_MotionAxis;
END_VAR
    bBusy := FALSE;
    bDone := FALSE;
    bError := FALSE;
    nErrorId := 0;
    IF Execute THEN
        ax.AXIS := Drive;
        ax.OP := 3;
        ax();
        ax.OP := 5;
        ax.INDEX := 16#6060;
        ax.VALUE := 3;
        ax();
        ax.INDEX := 16#60FF;
        ax.VALUE := LREAL_TO_DINT(Velocity);
        ax();
        IF ax.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := ax.ERRID;
        ELSE
            bDone := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- version ------------------------------------------------------

// F_GetVersionTcDrive — library version placeholder.
FUNCTION F_GetVersionTcDrive : UDINT
F_GetVersionTcDrive := 16#0100;
END_FUNCTION
