// tc2_enocean.st — Tc2_EnOcean surface (TwinCAT_3_PLC_Lib_Tc2_
// EnOcean_EN): EnOcean wireless over an opt-in serial ESP3
// transceiver (57600 8N1, e.g. TCM 310/USB 300).  Wrappers
// follow the M-Bus model: open a serial path, poll/send ESP3
// radio telegrams through the builtin FB_EnOcean service
// bridge.  The port is opened only when the surface is used —
// without a transceiver attached, open fails E_NOSERIAL.
//
// Differences (docs/plc/compat/README.md):
//   - TwinCAT models the transceiver via FB_EnOceanReceive with
//     a stEnOceanReceivedData struct; here the receive surface
//     is flattened: PTYPE/RORG/SenderId + RXPAYLOAD bytes.
//   - Sensor-module evaluators (FB_EnOceanPTM100 etc.) decode
//     on top of the raw payload — FB_EnOceanPtm100 is included
//     as a pure-ST decoder for the PTM rocker-switch profile.
//   - No SmartAck/teach-in, no remote-management, no security
//     layer — documented engine gaps.

// ---- FB_EnOceanOpen: open the serial ESP3 port ----
FUNCTION_BLOCK FB_EnOceanOpen
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
    ch : FB_EnOcean;
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

// ---- FB_EnOceanClose: release the serial port ----
FUNCTION_BLOCK FB_EnOceanClose
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
    ch : FB_EnOcean;
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

// ---- FB_EnOceanSendRadio: emit a RADIO_ERP1 telegram ----
FUNCTION_BLOCK FB_EnOceanSendRadio
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
    Org : UDINT;                // RORG
    SenderId : UDINT;           // own 32-bit sender id
    DestId : UDINT;             // 16#FFFFFFFF broadcast
    Data : STRING;              // payload bytes
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Error : BOOL;
    ErrorID : UDINT;
END_VAR
VAR
    ch : FB_EnOcean;
    exPrev : BOOL;
END_VAR
    Done := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute AND NOT exPrev THEN
        ch.OP := 4;
        ch.HANDLE := PortHdl;
        ch.ORG := Org;
        ch.SENDER := SenderId;
        ch.DEST := DestId;
        ch.DATA := Data;
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

// ---- FB_EnOceanReceive: poll for an ESP3 telegram ----
FUNCTION_BLOCK FB_EnOceanReceive
VAR_INPUT
    Execute : BOOL;
    PortHdl : UDINT;
END_VAR
VAR_OUTPUT
    Done : BOOL;
    Telegram : BOOL;            // a packet was decoded this call
    Error : BOOL;
    ErrorID : UDINT;
    PacketType : UDINT;         // 1 radio, 2 response, ...
    Org : UDINT;                // RORG (radio packets)
    SenderId : UDINT;           // sender id (radio packets)
    Payload : STRING[255];
    Data0 : UDINT;              // first payload byte
END_VAR
VAR
    ch : FB_EnOcean;
END_VAR
    Done := FALSE;
    Telegram := FALSE;
    Error := FALSE;
    ErrorID := 0;
    IF Execute THEN
        ch.OP := 5;
        ch.HANDLE := PortHdl;
        ch();
        IF ch.STATUS = 16#100 THEN
            Done := TRUE;              // NOMSG: still waiting
        ELSIF ch.STATUS >= 0 THEN
            Done := TRUE;
            Telegram := TRUE;
            PacketType := ch.PTYPE;
            Org := ch.RORG;
            SenderId := ch.RSENDER;
            Payload := ch.RXPAYLOAD;
            Data0 := ch.RXB0;
        ELSE
            Error := TRUE;
            ErrorID := ch.ERRID;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK

// ---- FB_EnOceanPtm100: decode a PTM rocker-switch payload ----
// PTM 2xx (RPS, RORG F6): pressed => payload byte0 bits3-0 =
// action (0 AI, 1 A0, 2 BI, 3 B0), bit4 = energy bow,
// released => 0x00 with T21=1 NU=0 in status.  Pure-ST decode.
FUNCTION_BLOCK FB_EnOceanPtm100
VAR_INPUT
    Execute : BOOL;
    Org : UDINT;                // expected F6 (RPS)
    B0 : UDINT;                 // payload byte 0
END_VAR
VAR_OUTPUT
    Done : BOOL;
    IsPtm : BOOL;
    Pressed : BOOL;
    Act : UDINT;             // 0-3: AI/A0/BI/B0
    EnergyBow : BOOL;
END_VAR
    Done := FALSE;
    IsPtm := FALSE;
    Pressed := FALSE;
    EnergyBow := FALSE;
    Act := 0;
    IF NOT Execute THEN RETURN; END_IF;
    Done := TRUE;
    IF Org = 16#F6 THEN
        IsPtm := TRUE;
        IF B0 <> 0 THEN
            Pressed := TRUE;
            Act := B0 AND 16#0F;
            IF Act > 3 THEN Act := 0; END_IF;
            EnergyBow := (B0 AND 16#10) <> 0;
        END_IF;
    END_IF;
END_FUNCTION_BLOCK
