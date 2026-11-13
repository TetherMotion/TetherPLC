// tc3_ecdiag.st — Tc3_EtherCATDiag compatibility surface.
//
// TwinCAT's EtherCAT diagnostics library reads the CoE 0x10F3
// "Diagnosis History" object from EtherCAT slaves.  TetherPLC bridges
// this onto the tplc-ecdiag host service, which reads the
// ec.coediag.* params exposed by a Tether IO endpoint
// (tether_io_motion_server in sim mode; a real master would bridge
// per-slave SDO reads of 0x10F3 under the same names).
//
// FB_EcDiagRead (builtin, layout FB_EcDiagRead):
//   OP      1 = read history, 2 = acknowledge (ACKVAL = newest acked)
//   SLAVE   slave port address
//   outputs COUNT NEWEST ACKED NEWMSGS INFOFLAGS STATUS ERRID plus
//           16-slot parallel entry arrays MSGTYPE ACKST TSMS CODE
//           CODETYPE TEXTID MODULE CHANNEL MSGTEXT
// STATUS is the service return code (0 ok, negative error), ERRID
// its positive magnitude.  When the service is inactive STATUS = -2.
//
// Mapping notes (documented deviations from the TwinCAT surface):
//   - ST_CoE_DiagHistory_Entry is a TwinCAT STRUCT; this dialect has
//     no user STRUCTs, so entries are exposed as parallel member
//     arrays aE* on FB_CoE_DiagHistory_Read (slot i = entry i).
//   - stTimeStamp TIMESTRUCT is flattened to aETimestampMs (ms since
//     the slave timestamp epoch reported by the device).
//   - sMsgText carries a generated "TID=/CODE=" string; real message
//     texts come from the slave ESI/XML which is not available.
//   - tReadInterval is approximated by nReadIntervalScans (scan
//     count, not wall time) since the scan period is not visible to
//     the FB.
//   - FB_CoE_DiagHistory_ReadAxis (AXIS_REF variant) is not provided:
//     it needs Tc2_MC2 axis objects; use the address variant.
//   - diag_code_type is approximated (emergency flag -> Emergency,
//     nonzero code -> DeviceRelated); module/profile typing needs
//     device profile data.
//
// SPDX-License-Identifier: Apache-2.0 OR MPL-2.0

// ---- FB_CoE_DiagHistory_Read -------------------------------------
// bEnable: rising edge reads once; while held, re-reads every
// nReadIntervalScans calls.  bAck (rising edge) acknowledges all
// messages up to NEWEST, then triggers a fresh read.

FUNCTION_BLOCK FB_CoE_DiagHistory_Read
VAR_INPUT
    sNetId : STRING[22];          // T_AmsNetId (accepted, unused)
    nSlaveAddr : UINT;            // slave port address
    bEnable : BOOL;
    bAck : BOOL;
    tTimeout : TIME := T#5S;      // accepted; IO is internally bounded
    tReadInterval : TIME := T#10S;// accepted; see nReadIntervalScans
    nReadIntervalScans : UDINT := 1000;
    // ST_CoE_DiagHistory_Options (flattened)
    nLangId : DINT := 1033;
    eBufferMode : USINT := 0;
    bEnableEmergSending : BOOL := TRUE;
    bFilterOutInfoMsgs : BOOL := FALSE;
    bFilterOutWarnMsgs : BOOL := FALSE;
    bFilterOutErrMsgs : BOOL := FALSE;
    bDisableAutoRecoverAfterConnectionErr : BOOL := FALSE;
    bExcludeTextIdFromMsgText : BOOL := FALSE;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bValid : BOOL;
    bError : BOOL;
    nErrorId : UDINT;
    // ST_CoE_DiagHistory_Info (flattened)
    bNewMsgsAvail : BOOL;
    bEmergSendingEnabled : BOOL;
    bInfoMsgsDisabled : BOOL;
    bWarnMsgsDisabled : BOOL;
    bErrMsgsDisabled : BOOL;
    bOldMsgsOverwritten : BOOL;
    bNewMsgsDiscarded : BOOL;
    // object header mirrors (sticky after each read)
    oNewest : UDINT;
    oAcked : UDINT;
    oNewMsgs : UDINT;
    oCount : UDINT;
END_VAR
VAR
    raw : FB_EcDiagRead;
    scanN : UDINT;
    prevEn : BOOL;
    prevAck : BOOL;
    haveResult : BOOL;
    lastOk : BOOL;
    i : UDINT;
    j : UDINT;
    keep : BOOL;
    // flattened entry arrays (public-readable member data)
    aEMsgType : ARRAY[0..15] OF UDINT;
    aEAckStatus : ARRAY[0..15] OF UDINT;
    aETimestampMs : ARRAY[0..15] OF UDINT;
    aEDiagCode : ARRAY[0..15] OF UDINT;
    aEDiagCodeType : ARRAY[0..15] OF UDINT;
    aETextId : ARRAY[0..15] OF UDINT;
    aEModuleNo : ARRAY[0..15] OF UDINT;
    aEChannelNo : ARRAY[0..15] OF UDINT;
    aEMsgText : ARRAY[0..15] OF STRING[63];
END_VAR
    bBusy := FALSE;
    bValid := haveResult AND lastOk;
    bError := haveResult AND NOT lastOk;
    nErrorId := 0;

    // acknowledge request: write newest as acked, then re-read below
    IF bAck AND NOT prevAck THEN
        raw.OP := 2;
        raw.SLAVE := nSlaveAddr;
        raw.ACKVAL := oNewest;
        raw();
        IF raw.STATUS < 0 THEN
            haveResult := TRUE;
            lastOk := FALSE;
            nErrorId := raw.ERRID;
        ELSE
            haveResult := TRUE;
            lastOk := TRUE;
        END_IF;
        scanN := 0;
    END_IF;

    // read trigger: rising edge, ack edge, or level-held interval
    IF (bEnable AND NOT prevEn) OR
       (bEnable AND scanN >= nReadIntervalScans) OR
       (bAck AND NOT prevAck) THEN
        raw.OP := 1;
        raw.SLAVE := nSlaveAddr;
        raw();
        bBusy := TRUE;   // service call is synchronous; pulses this scan
        haveResult := TRUE;
        lastOk := raw.STATUS >= 0;
        IF NOT lastOk THEN
            nErrorId := raw.ERRID;
        END_IF;
        scanN := 0;
    ELSIF bEnable THEN
        scanN := scanN + 1;
    END_IF;

    // mirror the object header (sticky across scans)
    oNewest := raw.NEWEST;
    oAcked := raw.ACKED;
    oNewMsgs := raw.NEWMSGS;
    oCount := raw.COUNT;

    // flatten raw entry arrays into the public aE* arrays, applying
    // the option filters (info/warn/error suppression)
    j := 0;
    FOR i := 0 TO 15 DO
        keep := i < raw.COUNT;
        IF keep AND bFilterOutInfoMsgs AND raw.MSGTYPE[i] = 1 THEN
            keep := FALSE;
        END_IF;
        IF keep AND bFilterOutWarnMsgs AND raw.MSGTYPE[i] = 2 THEN
            keep := FALSE;
        END_IF;
        IF keep AND bFilterOutErrMsgs AND raw.MSGTYPE[i] = 3 THEN
            keep := FALSE;
        END_IF;
        IF keep THEN
            aEMsgType[j] := raw.MSGTYPE[i];
            aEAckStatus[j] := raw.ACKST[i];
            aETimestampMs[j] := raw.TSMS[i];
            aEDiagCode[j] := raw.CODE[i];
            aEDiagCodeType[j] := raw.CODETYPE[i];
            aETextId[j] := raw.TEXTID[i];
            aEModuleNo[j] := raw.MODULE[i];
            aEChannelNo[j] := raw.CHANNEL[i];
            aEMsgText[j] := raw.MSGTEXT[i];
            j := j + 1;
        END_IF;
    END_FOR;

    // stInfo echo of the option inputs plus object state
    bNewMsgsAvail := oNewMsgs <> 0;
    bEmergSendingEnabled := bEnableEmergSending;
    bInfoMsgsDisabled := bFilterOutInfoMsgs;
    bWarnMsgsDisabled := bFilterOutWarnMsgs;
    bErrMsgsDisabled := bFilterOutErrMsgs;
    bOldMsgsOverwritten := FALSE;   // overwrite-mode tracking unsupported
    bNewMsgsDiscarded := FALSE;     // ack-mode discard tracking unsupported

    prevEn := bEnable;
    prevAck := bAck;
END_FUNCTION_BLOCK
