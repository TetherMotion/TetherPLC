// tc3_ecsync.st — Tc3_EtherCATExtSync compatibility surface.
//
// TwinCAT's EtherCAT external-sync library exposes FBs that request
// and report the distributed-clock (DC) synchronization state of the
// EtherCAT master.  TetherPLC bridges those semantics onto the
// tplc-ecsync host service, which reads/writes the ec.dc.* params
// exposed by a Tether IO endpoint (tether_io_motion_server in sim
// mode, or a real master bridging DCManager/EtherCATDC stats under
// the same names).
//
// FB_EcSync (builtin, layout FB_EcSync):
//   OP      1 = poll status, 2 = set sync enable
//   ENABLE  sync enable value for OP 2 (0/1)
//   outputs CONNECTED SYNCEN PARAMSOK LASTERR STATUS ERRID DIFFNS
//           REFNS
// STATUS is the service return code (0 ok, negative error), ERRID
// its positive magnitude.  When the service is inactive (no
// TPLC_EC_IO/TPLC_MOTION_IO endpoint, or the endpoint does not
// expose ec.dc.* params) STATUS = -2.
//
// Mapping notes: the endpoint exposes a single simulated/real master
// link, so sNetId selects nothing beyond "a master is targeted" — a
// nonempty net id enables sync, an empty one disables it.  The
// documented 10-cycle internal/external diff check is reduced to the
// per-call |time_diff_ns| <= nSyncWindow comparison the service
// reports.
//
// SPDX-License-Identifier: Apache-2.0 OR MPL-2.0

// ---- FB_EcExtSyncExtTimes: enable/disable DC sync -----------------
// Nonempty sNetId enables ec.dc.sync_enabled and polls until the
// link reports it; bTS1Done then stays set while enabled.  Empty
// sNetId disables sync.  bError/nErrorId latch the last nonzero
// service status.

FUNCTION_BLOCK FB_EcExtSyncExtTimes
VAR_INPUT
    sNetId : STRING[22];   // T_AMSNetId
END_VAR
VAR_OUTPUT
    bTS1Done : BOOL;
    bError   : BOOL;
    nErrorId : UDINT;
END_VAR
VAR
    ch    : FB_EcSync;
    prev  : BOOL;   // previous "target present" level
    armed : BOOL;
END_VAR
    bError := FALSE;
    nErrorId := 0;

    ch.OP := 1;
    ch();
    IF ch.STATUS < 0 THEN
        bError := TRUE;
        nErrorId := ch.ERRID;
    END_IF;

    IF LEN(sNetId) > 0 AND NOT prev THEN
        ch.OP := 2;
        ch.ENABLE := 1;
        ch();
        IF ch.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := ch.ERRID;
        ELSE
            armed := TRUE;
        END_IF;
        ch.OP := 1;
    ELSIF LEN(sNetId) = 0 AND prev THEN
        ch.OP := 2;
        ch.ENABLE := 0;
        ch();
        IF ch.STATUS < 0 THEN
            bError := TRUE;
            nErrorId := ch.ERRID;
        END_IF;
        ch.OP := 1;
        bTS1Done := FALSE;
        armed := FALSE;
    END_IF;

    bTS1Done := armed AND ch.CONNECTED <> 0 AND ch.SYNCEN <> 0;
    prev := LEN(sNetId) > 0;
END_FUNCTION_BLOCK

// ---- FB_EcExtSyncIsSynchronized: windowed sync check ---------------
// Polls the status op once per call.  bNotConnected is set when the
// service is unavailable or the master link is down; bSynchronized
// is set only when connected, sync enabled, and |time_diff_ns| is
// within nSyncWindow nanoseconds.

FUNCTION_BLOCK FB_EcExtSyncIsSynchronized
VAR_INPUT
    nSyncWindow : UDINT;   // nanoseconds
END_VAR
VAR_OUTPUT
    bSynchronized : BOOL;
    bNotConnected : BOOL;
    bError         : BOOL;
    nErrorId       : UDINT;
END_VAR
VAR
    ch : FB_EcSync;
    d  : LREAL;
    w  : LREAL;
END_VAR
    bError := FALSE;
    nErrorId := 0;

    ch.OP := 1;
    ch();
    IF ch.STATUS < 0 THEN
        bError := TRUE;
        nErrorId := ch.ERRID;
    END_IF;

    bSynchronized := FALSE;
    bNotConnected := FALSE;
    IF ch.STATUS < 0 OR ch.CONNECTED = 0 THEN
        bNotConnected := TRUE;
    ELSIF ch.SYNCEN <> 0 THEN
        d := ch.DIFFNS;
        IF d < 0.0 THEN d := -d; END_IF;
        w := nSyncWindow;
        IF d <= w THEN
            bSynchronized := TRUE;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK
