// Tc2_RFID (TF6600) -> FB_RFIDReader over the tplc-serial service.
// TwinCAT's library is a single transparent transport block: the
// application puts a vendor command into TxBuffer and reads the
// reader's answer from RxBuffer.  This port keeps that shape —
// open the reader port, write the command, poll for the reply —
// with STRING buffers instead of ComBuffer/POINTER TO BYTE.
// Documented differences:
//   - No ComBuffer objects: TxBuffer/RxBuffer are STRING[255]
//     payloads plus explicit lengths (this compiler has no
//     POINTER TO BYTE in ST).
//   - The port is opened on the first execute and kept until
//     bClose is set; TwinCAT's reader handle is the same idea.
//   - Vendor framing (Balluff BIS / Turck / SICK command sets) is
//     application-level: the transport is byte-transparent.

FUNCTION_BLOCK FB_RFIDReader
VAR_INPUT
    bExecute : BOOL;            // start a transceive cycle
    bClose : BOOL;              // release the reader port
    sDevice : STRING[63];       // e.g. '/dev/ttyUSB0'
    nBaud : UDINT;
    nParity : UDINT;            // 0 none, 1 even, 2 odd
    nDataBits : UDINT;
    nStopBits : UDINT;
    sTxBuffer : STRING[255];    // vendor command
    nTxLen : UDINT;
    tTimeout : TIME;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    sRxBuffer : STRING[255];    // reader reply
    nRxLen : UDINT;
    hReader : UDINT;
END_VAR
VAR
    ser : FB_Serial;
    prevExec : BOOL;
    prevClose : BOOL;
END_VAR
    bBusy := FALSE; bError := FALSE; nErrId := 0;
    // close request
    IF bClose AND NOT prevClose AND hReader <> 0 THEN
        ser.OP := 4; ser.HANDLE := hReader; ser.TIMEOUT := tTimeout;
        ser.EXEC := TRUE;
        ser();
        ser.EXEC := FALSE;
        ser();                    // release the EXEC latch
        hReader := 0;
        bDone := TRUE;
    END_IF;
    prevClose := bClose;
    // transceive cycle: open (if needed), send, receive
    IF bExecute AND NOT prevExec THEN
        bBusy := TRUE;
        IF hReader = 0 THEN
            ser.OP := 1; ser.DEV := sDevice; ser.BAUD := nBaud;
            ser.PARITY := nParity; ser.DBITS := nDataBits;
            ser.SBITS := nStopBits; ser.TIMEOUT := tTimeout;
            ser.EXEC := TRUE;
            ser();
            ser.EXEC := FALSE;
            ser();                // release the EXEC latch
            IF ser.ERRID <> 0 THEN
                bError := TRUE; nErrId := ser.ERRID;
            ELSE
                hReader := ser.HANDLE;
            END_IF;
        END_IF;
        IF NOT bError AND hReader <> 0 THEN
            IF nTxLen > 0 THEN
                ser.OP := 2; ser.HANDLE := hReader;
                ser.DATA := sTxBuffer; ser.DLEN := nTxLen;
                ser.TIMEOUT := tTimeout;
                ser.EXEC := TRUE;
                ser();
                ser.EXEC := FALSE;
                ser();            // release the EXEC latch
                IF ser.ERRID <> 0 THEN
                    bError := TRUE; nErrId := ser.ERRID;
                END_IF;
            END_IF;
        END_IF;
        IF NOT bError AND hReader <> 0 THEN
            ser.OP := 3; ser.HANDLE := hReader;
            ser.TIMEOUT := tTimeout;
            ser.EXEC := TRUE;
            ser();
            ser.EXEC := FALSE;
            ser();                // release the EXEC latch
            IF ser.ERRID <> 0 THEN
                bError := TRUE; nErrId := ser.ERRID;
            ELSE
                sRxBuffer := ser.DATA;
                nRxLen := ser.DLEN;
                bDone := TRUE;
            END_IF;
        END_IF;
        bBusy := FALSE;
    END_IF;
    prevExec := bExecute;
END_FUNCTION_BLOCK
