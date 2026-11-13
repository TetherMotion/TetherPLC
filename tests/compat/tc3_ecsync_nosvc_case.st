// tc3_ecsync_nosvc_case.st — negative e2e: no TPLC_EC_IO endpoint,
// so tplc-ecsync is inactive and both FBs must report the link as
// not connected with a service error.
PROGRAM tc3_ecsync_nosvc_case
VAR
    ext : FB_EcExtSyncExtTimes;
    iso : FB_EcExtSyncIsSynchronized;
    st : INT;
    mask AT %QD0 : UDINT;
    netid : STRING[22];
END_VAR
    IF st = 0 THEN
        mask := 0;
        netid := '';
        iso.nSyncWindow := 4000000000;
        st := 1;
    ELSIF st = 1 THEN
        iso();
        IF iso.bNotConnected AND iso.bError AND NOT iso.bSynchronized THEN
            mask := mask + 1;
        END_IF;
        netid := '1.2.3.4:5.6';
        st := 2;
    ELSIF st = 2 THEN
        IF ext.bError AND NOT ext.bTS1Done THEN
            mask := mask + 2;
        END_IF;
        st := 3;
    ELSIF st = 3 THEN
        // hold
    END_IF
    ext.sNetId := netid;
    ext();
    iso();
END_PROGRAM
