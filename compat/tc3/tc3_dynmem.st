// Tc3_DynamicMemory -> FB_DynMem_Buffer / FB_DynMem_Manager.
// Beckhoff's originals hand out PVOIDs into a shared heap; we
// model the same behaviour over the tplc-dynmem bounded arena —
// blocks are integer handles (never raw pointers), accessed via
// FB_DynMem_Access byte I/O.  Documented differences:
//   - No POINTER semantics: a "buffer" is a handle; data crosses
//     one byte at a time through FB_DynMem_Access.
//   - METHOD calls are expressed as edge-triggered command
//     inputs (this compiler has no METHOD support): nCmd
//     1=Allocate 2=Reallocate 3=Deallocate.
//   - Arena is fixed (64 blocks, 64 KiB max block, 1 MiB total)
//     — deterministic worst case, no host-heap interaction.
//   - nErrId carries tplc-dynmem service codes.

// ---- FB_DynMem_Buffer: owned buffer handle ----
FUNCTION_BLOCK FB_DynMem_Buffer
VAR_INPUT
    nCmd : UDINT;               // 0 none, 1 alloc, 2 realloc, 3 dealloc
    nNewSize : UDINT;
    bPreserve : BOOL;
    bReset : BOOL;
END_VAR
VAR_OUTPUT
    hBuffer : UDINT;            // 0 = not allocated
    nSize : UDINT;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    dm : FB_DynMem;
    prev : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 1 THEN
            dm.OP := 1; dm.SIZE := nNewSize;
            dm.RESET := BOOL_TO_UDINT(bReset);
        ELSIF nCmd = 2 THEN
            IF hBuffer = 0 THEN
                bError := TRUE; nErrId := 2;
            ELSE
                dm.OP := 3; dm.ID := hBuffer; dm.SIZE := nNewSize;
                dm.PRESERVE := BOOL_TO_UDINT(bPreserve);
                dm.RESET := BOOL_TO_UDINT(bReset);
            END_IF;
        ELSIF nCmd = 3 THEN
            dm.OP := 2; dm.ID := hBuffer;
        END_IF;
        IF NOT bError THEN
            dm();
            IF dm.STATUS = 0 THEN
                bDone := TRUE;
                IF nCmd = 1 THEN
                    hBuffer := dm.ID; nSize := nNewSize;
                ELSIF nCmd = 2 THEN
                    nSize := nNewSize;
                ELSE
                    hBuffer := 0; nSize := 0;
                END_IF;
            ELSE
                bError := TRUE; nErrId := dm.ERRID;
            END_IF;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_DynMem_Access: byte I/O into a block handle ----
FUNCTION_BLOCK FB_DynMem_Access
VAR_INPUT
    nCmd : UDINT;               // 0 none, 4 read, 5 write
    nHandle : UDINT;
    nOffset : UDINT;
    nByte : BYTE;
END_VAR
VAR_OUTPUT
    nOut : BYTE;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    dm : FB_DynMem;
    prev : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 4 OR nCmd = 5 THEN
            dm.OP := nCmd; dm.ID := nHandle; dm.OFFSET := nOffset;
            dm.BYTE := BYTE_TO_UDINT(nByte);
            dm();
            IF dm.STATUS = 0 THEN
                bDone := TRUE;
                IF nCmd = 4 THEN nOut := UDINT_TO_BYTE(dm.BYTE); END_IF;
            ELSE
                bError := TRUE; nErrId := dm.ERRID;
            END_IF;
        END_IF;
    END_IF;
    prev := nCmd;
END_FUNCTION_BLOCK

// ---- FB_DynMem_Manager: alloc/free + consumption counters ----
FUNCTION_BLOCK FB_DynMem_Manager
VAR_INPUT
    nCmd : UDINT;               // 0 none, 1 alloc, 3 dealloc
    nReqSize : UDINT;
    bReset : BOOL;
    nId : UDINT;                // dealloc target
END_VAR
VAR_OUTPUT
    hNew : UDINT;               // handle returned by alloc
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nLiveBlocks : UDINT;
    nLiveBytes : UDINT;
    nTotalAllocs : UDINT;
    nTotalFrees : UDINT;
    nArenaBytes : UDINT;
    nArenaUsed : UDINT;
END_VAR
VAR
    dm : FB_DynMem;
    prev : UDINT;
END_VAR
    bDone := FALSE; bError := FALSE; nErrId := 0;
    IF nCmd <> 0 AND prev = 0 THEN
        IF nCmd = 1 THEN
            dm.OP := 1; dm.SIZE := nReqSize;
            dm.RESET := BOOL_TO_UDINT(bReset);
            dm();
            IF dm.STATUS = 0 THEN
                bDone := TRUE; hNew := dm.ID;
            ELSE
                bError := TRUE; nErrId := dm.ERRID;
            END_IF;
        ELSIF nCmd = 3 THEN
            dm.OP := 2; dm.ID := nId;
            dm();
            IF dm.STATUS = 0 THEN
                bDone := TRUE;
            ELSE
                bError := TRUE; nErrId := dm.ERRID;
            END_IF;
        END_IF;
    END_IF;
    prev := nCmd;
    // refresh counters every invocation
    dm.OP := 6;
    dm();
    IF dm.STATUS = 0 THEN
        nLiveBlocks := dm.LIVEBLKS;
        nLiveBytes := dm.LIVEBYTES;
        nTotalAllocs := dm.TOTALALLOCS;
        nTotalFrees := dm.TOTALFREES;
        nArenaBytes := dm.ARENABYTES;
        nArenaUsed := dm.ARENAUSED;
    END_IF;
END_FUNCTION_BLOCK
