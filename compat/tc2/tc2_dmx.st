// tc2_dmx.st — DMX512 output surface (TwinCAT Tc3_DMX/TF6501-ish
// shape) over the tplc-dmx service + FB_Dmx builtin.
//
// The PLC is the DMX master over an opt-in serial port (the surface
// only touches the UART when FB_DmxOpen is called): FB_DmxOpen opens
// the port at 250 kbaud 8N2; FB_DmxSetChannel / FB_DmxSetChannels
// write the 512-slot channel image; FB_DmxSend emits one complete
// DMX512 frame (break + mark-after-break + start code + 512 bytes);
// FB_DmxStatus reports frame count / state; FB_DmxClose releases.
//
// The PLC-side image is a snapshot — the PLC calls Send at its own
// cadence; a real DMX stream runs ~44 Hz, which a cyclic scan
// approximates.  The serial port is opt-in (no UART touched until
// Open).  RDM (bidirectional DMX) and receive are out of scope.

// ---- FB_DmxOpen: open the serial port ----
FUNCTION_BLOCK FB_DmxOpen
VAR_INPUT
    Execute : BOOL;
    Path : STRING[127];
    Baud : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    PortHdl : UDINT;
END_VAR
VAR
    ch : FB_Dmx;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 1;
        ch.PATH := Path;
        ch.BAUD := Baud;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            PortHdl := ch.HANDLE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_DmxClose: release the serial port ----
FUNCTION_BLOCK FB_DmxClose
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Dmx;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 2;
        ch.HANDLE := PortHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_DmxSetChannel: write one channel of the image ----
FUNCTION_BLOCK FB_DmxSetChannel
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    Channel : UDINT;
    Value : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Dmx;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 4;
        ch.HANDLE := PortHdl;
        ch.CHANNEL := Channel;
        ch.VALUE := Value;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_DmxSetChannels: write a run of channels ----
FUNCTION_BLOCK FB_DmxSetChannels
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    Start : UDINT;
    Count : UDINT;
    Values : STRING[255];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Dmx;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 5;
        ch.HANDLE := PortHdl;
        ch.START := Start;
        ch.COUNT := Count;
        ch.VALUES := Values;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
    exPrev := Execute;
END_FUNCTION_BLOCK

// ---- FB_DmxSend: emit one complete DMX512 frame ----
FUNCTION_BLOCK FB_DmxSend
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_Dmx;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 6;
        ch.HANDLE := PortHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_DmxStatus: frame count + state ----
FUNCTION_BLOCK FB_DmxStatus
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    State : DINT;
    Frames : UDINT;
    LastErr : DINT;
END_VAR
VAR
    ch : FB_Dmx;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 3;
        ch.HANDLE := PortHdl;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            State := ch.STATE;
            Frames := ch.FRAMES;
            LastErr := ch.LASTERR;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK
