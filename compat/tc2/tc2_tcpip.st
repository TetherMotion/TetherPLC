// Tc2_TcpIp socket FBs (TwinCAT 3 PLC Library Tc2_TcpIp).
// Beckhoff-shaped wrappers over the builtin FB_Socket, which drives
// the host "tplc-net" service (TCP/UDP, nonblocking, deadline-bounded).
// Differences from TwinCAT, documented in docs/plc/compat/README.md:
//   - sSrvNetId is not taken: sockets are local to this runtime; remote
//     AMS routing is not implemented yet.
//   - POINTER TO BYTE payloads are STRING[256] buffers.
//   - hSocket/hListener are UDINT service handles, not opaque objects.
//   - tTimeout <= 0 means no deadline.
// Sources: docs/plc/plan/21-twincat-libraries.md §7 (TF6310).

FUNCTION_BLOCK FB_SocketConnect
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
    hSocket : UDINT;
END_VAR
VAR
    sock : FB_Socket;
END_VAR
    sock.OP := 1;
    sock.HOST := sHost;
    sock.PORT := nPort;
    sock.TIMEOUT := tTimeout;
    sock.EXEC := bExecute;
    sock();
    bBusy := sock.BUSY;
    bDone := sock.DONE;
    nErrId := sock.ERRID;
    hSocket := sock.HANDLE;
    bError := sock.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SocketListen
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
    hListener : UDINT;
END_VAR
VAR
    sock : FB_Socket;
END_VAR
    sock.OP := 2;
    sock.HOST := sHost;
    sock.PORT := nPort;
    sock.TIMEOUT := tTimeout;
    sock.EXEC := bExecute;
    sock();
    bBusy := sock.BUSY;
    bDone := sock.DONE;
    nErrId := sock.ERRID;
    hListener := sock.HANDLE;
    bError := sock.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SocketAccept
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hListener : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bAccepted : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    hSocket : UDINT;
END_VAR
VAR
    sock : FB_Socket;
END_VAR
    sock.OP := 3;
    sock.AUX := hListener;
    sock.TIMEOUT := tTimeout;
    sock.EXEC := bExecute;
    sock();
    bBusy := sock.BUSY;
    bDone := sock.DONE;
    bAccepted := sock.DONE AND sock.ERRID = 0;
    nErrId := sock.ERRID;
    hSocket := sock.HANDLE;
    bError := sock.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SocketSend
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hSocket : UDINT;
    pData : STRING[256];
    nLen : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nSent : DINT;
END_VAR
VAR
    sock : FB_Socket;
END_VAR
    sock.OP := 4;
    sock.HANDLE := hSocket;
    sock.DATA := pData;
    sock.DLEN := nLen;
    sock.TIMEOUT := tTimeout;
    sock.EXEC := bExecute;
    sock();
    bBusy := sock.BUSY;
    bDone := sock.DONE;
    nErrId := sock.ERRID;
    nSent := sock.RESULT;
    bError := sock.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SocketReceive
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hSocket : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    pData : STRING[256];
    nRecBytes : UDINT;
END_VAR
VAR
    sock : FB_Socket;
END_VAR
    sock.OP := 5;
    sock.HANDLE := hSocket;
    sock.TIMEOUT := tTimeout;
    sock.EXEC := bExecute;
    sock();
    bBusy := sock.BUSY;
    bDone := sock.DONE;
    nErrId := sock.ERRID;
    pData := sock.DATA;
    nRecBytes := sock.DLEN;
    bError := sock.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SocketClose
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hSocket : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
END_VAR
VAR
    sock : FB_Socket;
END_VAR
    sock.OP := 6;
    sock.HANDLE := hSocket;
    sock.TIMEOUT := tTimeout;
    sock.EXEC := bExecute;
    sock();
    bBusy := sock.BUSY;
    bDone := sock.DONE;
    nErrId := sock.ERRID;
    bError := sock.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SocketUdpCreate
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
    hSocket : UDINT;
END_VAR
VAR
    sock : FB_Socket;
END_VAR
    sock.OP := 7;
    sock.HOST := sHost;
    sock.PORT := nPort;
    sock.TIMEOUT := tTimeout;
    sock.EXEC := bExecute;
    sock();
    bBusy := sock.BUSY;
    bDone := sock.DONE;
    nErrId := sock.ERRID;
    hSocket := sock.HANDLE;
    bError := sock.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SocketUdpSendTo
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hSocket : UDINT;
    sHost : STRING[15];
    nPort : UDINT;
    pData : STRING[256];
    nLen : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    nSent : DINT;
END_VAR
VAR
    sock : FB_Socket;
END_VAR
    sock.OP := 8;
    sock.HANDLE := hSocket;
    sock.HOST := sHost;
    sock.PORT := nPort;
    sock.DATA := pData;
    sock.DLEN := nLen;
    sock.TIMEOUT := tTimeout;
    sock.EXEC := bExecute;
    sock();
    bBusy := sock.BUSY;
    bDone := sock.DONE;
    nErrId := sock.ERRID;
    nSent := sock.RESULT;
    bError := sock.ERRID <> 0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_SocketUdpReceiveFrom
VAR_INPUT
    bExecute : BOOL;
    tTimeout : TIME;
    hSocket : UDINT;
END_VAR
VAR_OUTPUT
    bBusy : BOOL;
    bDone : BOOL;
    bError : BOOL;
    nErrId : UDINT;
    sHost : STRING[15];
    nPort : UDINT;
    pData : STRING[256];
    nRecBytes : UDINT;
END_VAR
VAR
    sock : FB_Socket;
END_VAR
    sock.OP := 9;
    sock.HANDLE := hSocket;
    sock.TIMEOUT := tTimeout;
    sock.EXEC := bExecute;
    sock();
    bBusy := sock.BUSY;
    bDone := sock.DONE;
    nErrId := sock.ERRID;
    sHost := sock.HOST;
    nPort := sock.AUX;
    pData := sock.DATA;
    nRecBytes := sock.DLEN;
    bError := sock.ERRID <> 0;
END_FUNCTION_BLOCK
