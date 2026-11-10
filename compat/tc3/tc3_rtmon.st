// Tc3_RealtimeMonitoring compatibility (TwinCAT 3 PLC Library
// Tc3_RealtimeMonitoring). Beckhoff-shaped wrappers over the builtin
// FB_RtMon, which drives the host "tplc-rtmon" service: a bounded
// ring of timestamped marks (steady-clock ns; sim time under
// --sim), appended from the scan path.
//
// Differences from TwinCAT, documented in docs/plc/compat/README.md:
//   - TwinCAT exposes methods on the FBs (fb.LogMark(...)); this
//     dialect has no METHOD, so each method is a separate
//     FUNCTION_BLOCK gated by bExecute: one rising edge = one mark.
//   - nContextId default TwinCAT_SystemInfoVarList._AppInfo.AdsPort
//     is unavailable; pass the context explicitly.
//   - bLogCallingTask / TcMarkOption.RefToCaller only records the
//     flag bit in the stored markctrl; no task reference is resolved.
//   - FB_RTMon_LogTrigger stores a plain mark; the realtime-monitor
//     server-side recording trigger is not modeled.
//   - hrErrorCode carries 0 or a negative service code, not a real
//     COM HRESULT.
//   - TcMarkOption has no VAR_GLOBAL CONSTANT in this dialect; the
//     UDINT masks are documented inline at each call site:
//       Start            16#E0000000   Stop             16#C0000000
//       SequenceStart    16#A0000000   SequenceStop     16#80000000
//       IntervalStart    16#60000000   IntervalStop     16#40000000
//       RefToCaller      16#08000000   (OR with a type)
//   - ST_RTMon_MarkDef cannot cross the --lib boundary (types are
//     file-scoped in this dialect), so LogMarkEx takes the three
//     MarkDef fields (nContextId/nGroupId/nEventId) flattened.
// Sources: docs/plc/plan/21-twincat-libraries.md §7 (TE1010).

// ---- FB_RTMon_LogMarkBase family (explicit context + markctrl) ----

FUNCTION_BLOCK FB_RTMon_LogMarkBase
VAR_INPUT
    bExecute : BOOL;
    nContextId : UINT;
    nGroupId : UINT;
    nEventId : UINT;
    nMarkCtrl : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    hrErrorCode : DINT;
END_VAR
VAR
    rtmon : FB_RtMon;
END_VAR
    rtmon.OP := 1;
    rtmon.CONTEXT := nContextId;
    rtmon.GROUP := nGroupId;
    rtmon.EVENT := nEventId;
    rtmon.MARKCTRL := nMarkCtrl;
    rtmon.EXEC := bExecute;
    rtmon();
    bBusy := rtmon.BUSY;
    bDone := rtmon.DONE;
    bError := rtmon.ERRID <> 0;
    hrErrorCode := rtmon.RESULT - UDINT_TO_DINT(rtmon.ERRID);
END_FUNCTION_BLOCK

// FB_RTMon_LogMarkBase.LogMarkEx — stMark flattened to its three
// UINT fields (see header note on ST_RTMon_MarkDef).
FUNCTION_BLOCK FB_RTMon_LogMarkBaseEx
VAR_INPUT
    bExecute : BOOL;
    nContextId : UINT;
    nGroupId : UINT;
    nEventId : UINT;
    nMarkCtrl : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    hrErrorCode : DINT;
END_VAR
VAR
    rtmon : FB_RtMon;
END_VAR
    rtmon.OP := 1;
    rtmon.CONTEXT := nContextId;
    rtmon.GROUP := nGroupId;
    rtmon.EVENT := nEventId;
    rtmon.MARKCTRL := nMarkCtrl;
    rtmon.EXEC := bExecute;
    rtmon();
    bBusy := rtmon.BUSY;
    bDone := rtmon.DONE;
    bError := rtmon.ERRID <> 0;
    hrErrorCode := rtmon.RESULT - UDINT_TO_DINT(rtmon.ERRID);
END_FUNCTION_BLOCK

// ---- FB_RTMon_LogMark family (context on the call, fixed type) ----
// Each FB embeds the markctrl for its TwinCAT method; bLogCallingTask
// ORs in TcMarkOption.RefToCaller (16#08000000).

FUNCTION_BLOCK FB_RTMon_LogMark
VAR_INPUT
    bExecute : BOOL;
    nContextId : UINT;
    bLogCallingTask : BOOL := TRUE;
    nGroupId : UINT;
    nEventId : UINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    hrErrorCode : DINT;
END_VAR
VAR
    rtmon : FB_RtMon;
    mctrl : UDINT;
END_VAR
    mctrl := 16#00000000;
    IF bLogCallingTask THEN
        mctrl := mctrl + 16#08000000;
    END_IF;
    rtmon.OP := 1;
    rtmon.CONTEXT := nContextId;
    rtmon.GROUP := nGroupId;
    rtmon.EVENT := nEventId;
    rtmon.MARKCTRL := mctrl;
    rtmon.EXEC := bExecute;
    rtmon();
    bBusy := rtmon.BUSY;
    bDone := rtmon.DONE;
    bError := rtmon.ERRID <> 0;
    hrErrorCode := rtmon.RESULT - UDINT_TO_DINT(rtmon.ERRID);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_RTMon_LogIntervalStart
VAR_INPUT
    bExecute : BOOL;
    nContextId : UINT;
    bLogCallingTask : BOOL := TRUE;
    nGroupId : UINT;
    nEventId : UINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    hrErrorCode : DINT;
END_VAR
VAR
    rtmon : FB_RtMon;
    mctrl : UDINT;
END_VAR
    mctrl := 16#60000000;
    IF bLogCallingTask THEN
        mctrl := mctrl + 16#08000000;
    END_IF;
    rtmon.OP := 1;
    rtmon.CONTEXT := nContextId;
    rtmon.GROUP := nGroupId;
    rtmon.EVENT := nEventId;
    rtmon.MARKCTRL := mctrl;
    rtmon.EXEC := bExecute;
    rtmon();
    bBusy := rtmon.BUSY;
    bDone := rtmon.DONE;
    bError := rtmon.ERRID <> 0;
    hrErrorCode := rtmon.RESULT - UDINT_TO_DINT(rtmon.ERRID);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_RTMon_LogIntervalStop
VAR_INPUT
    bExecute : BOOL;
    nContextId : UINT;
    bLogCallingTask : BOOL := TRUE;
    nGroupId : UINT;
    nEventId : UINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    hrErrorCode : DINT;
END_VAR
VAR
    rtmon : FB_RtMon;
    mctrl : UDINT;
END_VAR
    mctrl := 16#40000000;
    IF bLogCallingTask THEN
        mctrl := mctrl + 16#08000000;
    END_IF;
    rtmon.OP := 1;
    rtmon.CONTEXT := nContextId;
    rtmon.GROUP := nGroupId;
    rtmon.EVENT := nEventId;
    rtmon.MARKCTRL := mctrl;
    rtmon.EXEC := bExecute;
    rtmon();
    bBusy := rtmon.BUSY;
    bDone := rtmon.DONE;
    bError := rtmon.ERRID <> 0;
    hrErrorCode := rtmon.RESULT - UDINT_TO_DINT(rtmon.ERRID);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_RTMon_LogSequenceStart
VAR_INPUT
    bExecute : BOOL;
    nContextId : UINT;
    bLogCallingTask : BOOL := TRUE;
    nGroupId : UINT;
    nEventId : UINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    hrErrorCode : DINT;
END_VAR
VAR
    rtmon : FB_RtMon;
    mctrl : UDINT;
END_VAR
    mctrl := 16#A0000000;
    IF bLogCallingTask THEN
        mctrl := mctrl + 16#08000000;
    END_IF;
    rtmon.OP := 1;
    rtmon.CONTEXT := nContextId;
    rtmon.GROUP := nGroupId;
    rtmon.EVENT := nEventId;
    rtmon.MARKCTRL := mctrl;
    rtmon.EXEC := bExecute;
    rtmon();
    bBusy := rtmon.BUSY;
    bDone := rtmon.DONE;
    bError := rtmon.ERRID <> 0;
    hrErrorCode := rtmon.RESULT - UDINT_TO_DINT(rtmon.ERRID);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_RTMon_LogSequenceStop
VAR_INPUT
    bExecute : BOOL;
    nContextId : UINT;
    bLogCallingTask : BOOL := TRUE;
    nGroupId : UINT;
    nEventId : UINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    hrErrorCode : DINT;
END_VAR
VAR
    rtmon : FB_RtMon;
    mctrl : UDINT;
END_VAR
    mctrl := 16#80000000;
    IF bLogCallingTask THEN
        mctrl := mctrl + 16#08000000;
    END_IF;
    rtmon.OP := 1;
    rtmon.CONTEXT := nContextId;
    rtmon.GROUP := nGroupId;
    rtmon.EVENT := nEventId;
    rtmon.MARKCTRL := mctrl;
    rtmon.EXEC := bExecute;
    rtmon();
    bBusy := rtmon.BUSY;
    bDone := rtmon.DONE;
    bError := rtmon.ERRID <> 0;
    hrErrorCode := rtmon.RESULT - UDINT_TO_DINT(rtmon.ERRID);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_RTMon_LogStart
VAR_INPUT
    bExecute : BOOL;
    nContextId : UINT;
    bLogCallingTask : BOOL := TRUE;
    nGroupId : UINT;
    nEventId : UINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    hrErrorCode : DINT;
END_VAR
VAR
    rtmon : FB_RtMon;
    mctrl : UDINT;
END_VAR
    mctrl := 16#E0000000;
    IF bLogCallingTask THEN
        mctrl := mctrl + 16#08000000;
    END_IF;
    rtmon.OP := 1;
    rtmon.CONTEXT := nContextId;
    rtmon.GROUP := nGroupId;
    rtmon.EVENT := nEventId;
    rtmon.MARKCTRL := mctrl;
    rtmon.EXEC := bExecute;
    rtmon();
    bBusy := rtmon.BUSY;
    bDone := rtmon.DONE;
    bError := rtmon.ERRID <> 0;
    hrErrorCode := rtmon.RESULT - UDINT_TO_DINT(rtmon.ERRID);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_RTMon_LogStop
VAR_INPUT
    bExecute : BOOL;
    nContextId : UINT;
    bLogCallingTask : BOOL := TRUE;
    nGroupId : UINT;
    nEventId : UINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    hrErrorCode : DINT;
END_VAR
VAR
    rtmon : FB_RtMon;
    mctrl : UDINT;
END_VAR
    mctrl := 16#C0000000;
    IF bLogCallingTask THEN
        mctrl := mctrl + 16#08000000;
    END_IF;
    rtmon.OP := 1;
    rtmon.CONTEXT := nContextId;
    rtmon.GROUP := nGroupId;
    rtmon.EVENT := nEventId;
    rtmon.MARKCTRL := mctrl;
    rtmon.EXEC := bExecute;
    rtmon();
    bBusy := rtmon.BUSY;
    bDone := rtmon.DONE;
    bError := rtmon.ERRID <> 0;
    hrErrorCode := rtmon.RESULT - UDINT_TO_DINT(rtmon.ERRID);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_RTMon_LogTrigger
VAR_INPUT
    bExecute : BOOL;
    nContextId : UINT;
    bLogCallingTask : BOOL := TRUE;
    nGroupId : UINT;
    nEventId : UINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    hrErrorCode : DINT;
END_VAR
VAR
    rtmon : FB_RtMon;
    mctrl : UDINT;
END_VAR
    // TwinCAT trigger marks arm server-side recording; with no
    // trigger bit defined we store a plain mark (markctrl 0)
    mctrl := 16#00000000;
    IF bLogCallingTask THEN
        mctrl := mctrl + 16#08000000;
    END_IF;
    rtmon.OP := 1;
    rtmon.CONTEXT := nContextId;
    rtmon.GROUP := nGroupId;
    rtmon.EVENT := nEventId;
    rtmon.MARKCTRL := mctrl;
    rtmon.EXEC := bExecute;
    rtmon();
    bBusy := rtmon.BUSY;
    bDone := rtmon.DONE;
    bError := rtmon.ERRID <> 0;
    hrErrorCode := rtmon.RESULT - UDINT_TO_DINT(rtmon.ERRID);
END_FUNCTION_BLOCK
