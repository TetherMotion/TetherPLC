// tc3_ecsync_case.st — Tc3_EtherCATExtSync e2e against
// tether_io_motion_server (TPLC_EC_IO): enable sync -> TS1Done,
// windowed IsSynchronized checks, disable, re-enable.
// ELSIF chain prevents same-scan state cascade; each state does one
// thing per scan.
PROGRAM tc3_ecsync_case
VAR
    ext : FB_EcExtSyncExtTimes;
    iso : FB_EcExtSyncIsSynchronized;
    st : INT;
    waitN : INT;
    mask AT %QD0 : UDINT;
    netid : STRING[22];
    sBig : BOOL;
    ncBig : BOOL;
    sNarrow : BOOL;
    ncNarrow : BOOL;
    sOff : BOOL;
    ncOff : BOOL;
END_VAR
    IF st = 0 THEN
        mask := 0;
        netid := '';
        waitN := 0;
        st := 1;
    ELSIF st = 1 THEN
        netid := '1.2.3.4:5.6';
        st := 2;
    ELSIF st = 2 THEN
        IF ext.bTS1Done THEN
            IF NOT ext.bError THEN mask := mask + 1; END_IF;
            st := 3;
        ELSIF waitN > 60 THEN
            st := 90;   // timeout -> fail state
        ELSE
            waitN := waitN + 1;
        END_IF;
    ELSIF st = 3 THEN
        iso.nSyncWindow := 4000000000;
        iso();
        sBig := iso.bSynchronized;
        ncBig := iso.bNotConnected;
        iso.nSyncWindow := 500;
        iso();
        sNarrow := iso.bSynchronized;
        ncNarrow := iso.bNotConnected;
        IF sBig AND NOT ncBig AND NOT ncNarrow AND NOT sNarrow
           AND NOT iso.bError THEN
            mask := mask + 2;
        END_IF;
        netid := '';
        st := 4;
    ELSIF st = 4 THEN
        // sync disabled: neither synchronized nor not-connected
        iso.nSyncWindow := 4000000000;
        iso();
        sOff := iso.bSynchronized;
        ncOff := iso.bNotConnected;
        IF NOT sOff AND NOT ncOff AND NOT iso.bError THEN
            mask := mask + 4;
        END_IF;
        netid := '1.2.3.4:5.6';
        waitN := 0;
        st := 5;
    ELSIF st = 5 THEN
        IF ext.bTS1Done THEN
            iso.nSyncWindow := 4000000000;
            iso();
            IF iso.bSynchronized AND NOT iso.bNotConnected THEN
                mask := mask + 8;
            END_IF;
            st := 6;
        ELSIF waitN > 60 THEN
            st := 90;
        ELSE
            waitN := waitN + 1;
        END_IF;
    ELSIF st = 6 THEN
        st := 90;
    ELSIF st = 90 THEN
        // done; mask holds results
    END_IF
    ext.sNetId := netid;
    ext();
    iso();
END_PROGRAM
