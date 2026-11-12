// tc2_mpbus.st — Tc2_MPBus surface (TwinCAT_3_PLC_Lib_Tc2_
// MPBus_EN): Belimo MP-Bus master over an opt-in serial adapter
// (9600 8N1, e.g. a USB MP-Bus gateway presenting a tty — the
// KL6771 terminal's process image is not modeled).  Wire
// format: [DA][CMD][LEN][data][CHK8-xor] request/response
// frames; the transact op sends and captures the reply.
//
// Differences (docs/plc/compat/README.md):
//   - The KL6771 bus-terminal process image is replaced by a
//     serial MP-Bus adapter — the function blocks take a PortHdl
//     from FB_MpbusOpen instead of terminal IO arrays.
//   - The full Belimo device-profile command matrix (Energy
//     Valve, CMV, addressing wizards) is simplified to raw
//     command ids; profile semantics are a documented gap.

// ---- FB_MpbusOpen: open the serial MP-Bus adapter ----
FUNCTION_BLOCK FB_MpbusOpen
VAR_INPUT
    Execute : BOOL;
    Path : STRING;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    PortHdl : UDINT;
END_VAR
VAR
    ch : FB_Mpbus;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 1;
        ch.PATH := Path;
        ch();
        IF ch.STATUS = 0 THEN
            Done := TRUE;
            PortHdl := ch.HANDLE;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_MpbusClose: release the port ----
FUNCTION_BLOCK FB_MpbusClose
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
    ch : FB_Mpbus;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
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
END_FUNCTION_BLOCK

// ---- FB_MPBusCommand: raw transact to a node ----
// Polls the reply across scans: Done + Reply fields when a
// frame arrives, Error on wire failure.
FUNCTION_BLOCK FB_MPBusCommand
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    DestAddr : BYTE;            // MP node address
    Command : BYTE;
    Data : STRING[250];
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Reply : BOOL;               // a reply frame arrived
    Error : BOOL;
    ErrorID : UDINT;
    RespAddr : UDINT;
    RespCmd : UDINT;
    RespLen : UDINT;
    RespB0 : UDINT;
END_VAR
VAR
    ch : FB_Mpbus;
    exPrev : BOOL;
    pending : BOOL;
END_VAR
    Done := FALSE;
    Reply := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF NOT Execute THEN
        exPrev := FALSE;
        pending := FALSE;
        RETURN;
    END_IF;
    IF NOT exPrev THEN
        exPrev := TRUE;
        pending := TRUE;
        ch.OP := 4;
        ch.HANDLE := PortHdl;
        ch.DA := DestAddr;
        ch.CMD := Command;
        ch.APDU := Data;
        ch();
        // transact polls once itself; report if already there
        IF ch.STATUS = 16#100 THEN
            // no reply yet — keep polling below
        ELSIF ch.STATUS >= 0 THEN
            pending := FALSE;
            Done := TRUE;
            Reply := TRUE;
            RespAddr := ch.RDA;
            RespCmd := ch.RCMD;
            RespLen := ch.STATUS;
            RespB0 := ch.RXB0;
        ELSE
            pending := FALSE;
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    ELSIF pending THEN
        ch.OP := 5;
        ch.HANDLE := PortHdl;
        ch();
        IF ch.STATUS = 16#100 THEN
            // still waiting
        ELSIF ch.STATUS >= 0 THEN
            pending := FALSE;
            Done := TRUE;
            Reply := TRUE;
            RespAddr := ch.RDA;
            RespCmd := ch.RCMD;
            RespLen := ch.STATUS;
            RespB0 := ch.RXB0;
        ELSE
            pending := FALSE;
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- MP_DamperLinearActuator: setpoint write + actual read ----
// CMD 0x01 = write setpoint (pos 0..255 raw), CMD 0x02 = read
// actual position.  Simplified documented profile mapping.
FUNCTION_BLOCK MP_DamperLinearActuator
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    DestAddr : BYTE;
    Setpoint : LREAL;           // 0..100 %
    WriteSp : BOOL;             // TRUE: write SP, FALSE: read actual
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    ActualPos : LREAL;          // 0..100 % (from read)
END_VAR
VAR
    cmd : FB_MPBusCommand;
    exPrev : BOOL;
    armed : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF NOT Execute THEN exPrev := FALSE; armed := FALSE; RETURN; END_IF;
    IF NOT exPrev THEN
        exPrev := TRUE;
        armed := FALSE;
        cmd.PortHdl := PortHdl;
        cmd.DestAddr := DestAddr;
        IF WriteSp THEN
            cmd.Command := 1;
            cmd.Data := 'x';        // 1-byte raw position payload
        ELSE
            cmd.Command := 2;
            cmd.Data := '';
        END_IF;
    END_IF;
    // give the inner command a clean FALSE->TRUE edge: the
    // FALSE is presented this scan, the TRUE next scan
    IF NOT armed THEN
        cmd.Execute := FALSE;
        armed := TRUE;
    ELSE
        cmd.Execute := TRUE;
    END_IF;
    cmd();
    IF cmd.Reply THEN
        Done := TRUE;
        IF cmd.RespLen > 0 THEN
            ActualPos := UDINT_TO_LREAL(cmd.RespB0) * 100.0 / 255.0;
        END_IF;
    END_IF;
    IF cmd.Error THEN Error := TRUE; ErrorID := cmd.ErrorID; END_IF;
END_FUNCTION_BLOCK

// ---- MP_CMV: read a measured value from a CMV node ----
// CMD 0x10 = read measured value (channel in data byte).
FUNCTION_BLOCK MP_CMV
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    DestAddr : BYTE;
    Channel : BYTE;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
    Value : UDINT;              // raw measured value byte
END_VAR
VAR
    cmd : FB_MPBusCommand;
    exPrev : BOOL;
    armed : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF NOT Execute THEN exPrev := FALSE; armed := FALSE; RETURN; END_IF;
    IF NOT exPrev THEN
        exPrev := TRUE;
        armed := FALSE;
        cmd.PortHdl := PortHdl;
        cmd.DestAddr := DestAddr;
        cmd.Command := 16#10;
        cmd.Data := 'x';
    END_IF;
    IF NOT armed THEN
        cmd.Execute := FALSE;
        armed := TRUE;
    ELSE
        cmd.Execute := TRUE;
    END_IF;
    cmd();
    IF cmd.Reply THEN
        Done := TRUE;
        Value := cmd.RespB0;
    END_IF;
    IF cmd.Error THEN Error := TRUE; ErrorID := cmd.ErrorID; END_IF;
END_FUNCTION_BLOCK
