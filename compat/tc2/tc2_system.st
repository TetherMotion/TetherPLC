// Tc2_System ADS function blocks (TwinCAT 3 PLC Library Tc2_System).
// Beckhoff-shaped wrappers over the builtin FB_Ads, which drives
// the host "tplc-ads" service (AMS/TCP ADS client, nonblocking,
// deadline-bounded).
// Differences from TwinCAT, documented in docs/plc/compat/README.md:
//   - TwinCAT connects implicitly per (NETID, PORT); this port needs
//     an explicit FB_AdsConnect because the transport is TCP.
//   - sNetId is accepted as sHost: only IPv4 literals are supported
//     (the first four NetID octets map to the host address).
//   - POINTER TO BYTE payloads are STRING[256] buffers.
//   - nAdsError carries the ADS-layer error code; nErrId carries
//     transport/service errors.
//   - Indication FBs (ADSREADIND/ADSWRITEIND/...RES) and combined
//     ADSRDWRT are not provided yet.
// Sources: docs/plc/plan/21-twincat-libraries.md §7 (TF6000).

FUNCTION_BLOCK FB_AdsConnect
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    sHost : STRING[15];
    nPort : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    hConn : UDINT;
END_VAR
VAR
    ads : FB_Ads;
END_VAR
    ads.OP := 1;
    ads.HOST := sHost;
    ads.PORT := nPort;
    ads.TIMEOUT := tTimeout;
    ads.EXEC := bExecute;
    ads();
    bBusy := ads.BUSY;
    bDone := ads.DONE;
    nErrId := ads.ERRID;
    hConn := ads.HANDLE;
    bError := ads.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_AdsRead
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hConn : UDINT;
    nIdxGrp : UDINT;
    nIdxOff : UDINT;
    nLen : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nAdsError : DINT;
    pData : STRING[256];
    nBytes : UDINT;
END_VAR
VAR
    ads : FB_Ads;
END_VAR
    ads.OP := 2;
    ads.HANDLE := hConn;
    ads.IDXGRP := nIdxGrp;
    ads.IDXOFF := nIdxOff;
    ads.DLEN := nLen;
    ads.TIMEOUT := tTimeout;
    ads.EXEC := bExecute;
    ads();
    bBusy := ads.BUSY;
    bDone := ads.DONE;
    nErrId := ads.ERRID;
    nAdsError := ads.RESULT;
    pData := ads.DATA;
    nBytes := ads.DLEN;
    bError := (ads.ERRID <> 0) OR (ads.RESULT <> 0);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_AdsWrite
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hConn : UDINT;
    nIdxGrp : UDINT;
    nIdxOff : UDINT;
    pData : STRING[256];
    nLen : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nAdsError : DINT;
END_VAR
VAR
    ads : FB_Ads;
END_VAR
    ads.OP := 3;
    ads.HANDLE := hConn;
    ads.IDXGRP := nIdxGrp;
    ads.IDXOFF := nIdxOff;
    ads.DATA := pData;
    ads.DLEN := nLen;
    ads.TIMEOUT := tTimeout;
    ads.EXEC := bExecute;
    ads();
    bBusy := ads.BUSY;
    bDone := ads.DONE;
    nErrId := ads.ERRID;
    nAdsError := ads.RESULT;
    bError := (ads.ERRID <> 0) OR (ads.RESULT <> 0);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_AdsReadState
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hConn : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nAdsError : DINT;
    nAdsState : DINT;
    nDevState : DINT;
END_VAR
VAR
    ads : FB_Ads;
END_VAR
    ads.OP := 4;
    ads.HANDLE := hConn;
    ads.TIMEOUT := tTimeout;
    ads.EXEC := bExecute;
    ads();
    bBusy := ads.BUSY;
    bDone := ads.DONE;
    nErrId := ads.ERRID;
    nAdsError := ads.RESULT;
    nAdsState := ads.STATE;
    nDevState := ads.DEVSTATE;
    bError := (ads.ERRID <> 0) OR (ads.RESULT <> 0);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_AdsWriteControl
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hConn : UDINT;
    nAdsState : DINT;
    nDevState : DINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nAdsError : DINT;
END_VAR
VAR
    ads : FB_Ads;
END_VAR
    ads.OP := 5;
    ads.HANDLE := hConn;
    ads.STATE := nAdsState;
    ads.DEVSTATE := nDevState;
    ads.TIMEOUT := tTimeout;
    ads.EXEC := bExecute;
    ads();
    bBusy := ads.BUSY;
    bDone := ads.DONE;
    nErrId := ads.ERRID;
    nAdsError := ads.RESULT;
    bError := (ads.ERRID <> 0) OR (ads.RESULT <> 0);
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_AdsClose
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hConn : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    ads : FB_Ads;
END_VAR
    ads.OP := 6;
    ads.HANDLE := hConn;
    ads.TIMEOUT := tTimeout;
    ads.EXEC := bExecute;
    ads();
    bBusy := ads.BUSY;
    bDone := ads.DONE;
    nErrId := ads.ERRID;
    bError := ads.ERRID <> 0;
END_FUNCTION_BLOCK
