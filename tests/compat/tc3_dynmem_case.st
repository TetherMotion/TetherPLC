// tc3_dynmem_case.st — e2e for tc3_dynmem (tplc-dynmem arena).
// Convention: a state that sets nCmd executes at that scan's
// FB call; the next state checks *Done and clears nCmd so the
// following command sees a fresh edge.
PROGRAM tc3_dynmem_case
VAR
    buf : FB_DynMem_Buffer;
    acc : FB_DynMem_Access;
    mgr : FB_DynMem_Manager;
    st : INT;
    mask AT %QD0 : UDINT;
    diag AT %QD4 : UDINT;
END_VAR
    diag := st;
    IF st = 0 THEN
        mask := 0;
        buf.bReset := TRUE;
        buf.nNewSize := 16; buf.nCmd := 1;   // alloc 16, zeroed
        st := 1;
    ELSIF st = 1 THEN
        IF buf.bDone AND buf.hBuffer <> 0 THEN mask := mask OR 1; END_IF;
        buf.nCmd := 0;
        acc.nHandle := buf.hBuffer; acc.nOffset := 3;
        acc.nByte := 16#A5;
        st := 2;
    ELSIF st = 2 THEN
        acc.nCmd := 5;                       // write 0xA5@3
        st := 3;
    ELSIF st = 3 THEN
        IF acc.bDone THEN mask := mask OR 2; END_IF;
        acc.nCmd := 0;
        st := 4;
    ELSIF st = 4 THEN
        acc.nCmd := 4;                       // read it back
        st := 5;
    ELSIF st = 5 THEN
        IF acc.bDone AND acc.nOut = 16#A5 THEN mask := mask OR 4; END_IF;
        acc.nCmd := 0;
        buf.bPreserve := TRUE; buf.bReset := FALSE;
        buf.nNewSize := 32;
        st := 6;
    ELSIF st = 6 THEN
        buf.nCmd := 2;                       // realloc grow, preserve
        st := 7;
    ELSIF st = 7 THEN
        IF buf.bDone AND buf.nSize = 32 THEN mask := mask OR 8; END_IF;
        buf.nCmd := 0;
        st := 8;
    ELSIF st = 8 THEN
        acc.nHandle := buf.hBuffer; acc.nOffset := 3;
        acc.nCmd := 4;                       // preserved byte?
        st := 9;
    ELSIF st = 9 THEN
        IF acc.bDone AND acc.nOut = 16#A5 THEN mask := mask OR 16; END_IF;
        acc.nCmd := 0;
        acc.nOffset := 64;
        st := 10;
    ELSIF st = 10 THEN
        acc.nCmd := 4;                       // out-of-range -> error
        st := 11;
    ELSIF st = 11 THEN
        IF acc.bError THEN mask := mask OR 32; END_IF;
        acc.nCmd := 0;
        st := 12;
    ELSIF st = 12 THEN
        buf.nCmd := 3;                       // deallocate
        st := 13;
    ELSIF st = 13 THEN
        IF buf.bDone AND buf.hBuffer = 0 THEN mask := mask OR 64; END_IF;
        buf.nCmd := 0;
        mgr.nReqSize := 8;
        st := 14;
    ELSIF st = 14 THEN
        mgr.nCmd := 1;                       // manager alloc
        st := 15;
    ELSIF st = 15 THEN
        IF mgr.bDone AND mgr.hNew <> 0
           AND mgr.nLiveBlocks = 1 AND mgr.nLiveBytes = 8 THEN
            mask := mask OR 128;
        END_IF;
        mgr.nCmd := 0;
        mgr.nId := mgr.hNew;
        st := 16;
    ELSIF st = 16 THEN
        mgr.nCmd := 3;                       // manager dealloc
        st := 17;
    ELSIF st = 17 THEN
        IF mgr.bDone AND mgr.nLiveBlocks = 0
           AND mgr.nTotalFrees = 2 THEN
            mask := mask OR 256;
        END_IF;
        mgr.nCmd := 0;
        st := 18;
    END_IF;
    buf(); acc(); mgr();
END_PROGRAM
