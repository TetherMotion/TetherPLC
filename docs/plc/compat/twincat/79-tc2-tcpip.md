# Tc2_TcpIp

- **Manual:** `TF6310_TC3_TCP_IP_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 34
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** TCP/UDP sockets, DNS, multicast: needs a socket service in the runtime (none today). sACN and most connectivity libs build on this.

---

#### PLC API › Function blocks

##### `FB_SocketConnect` — §5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/ |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active |

**Description:** Using the function block FB_SocketConnect, a local client can establish a new TCP/IP connection to a remote server via the TwinCAT TCP/IP Connection Server. If successful, a new socket is opened, and the associated connection handle is returned at the hSocket output. The connection handle is required by the function blocks FB_SocketSend [} 28] and FB_SocketReceive [} 30], for example, in order to exchange data with a remote server. If a connection is no longer required, it can be closed with the function block FB_SocketClose [} 24]. Several clients can establish a connection with the remote server at the same time. For each new client, a new socket is opened and a new connection handle is returned. The TwinCAT TCP/ IP Connection Server automatically assigns a new IP port number for each client.

##### `FB_SocketClose` — §5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/ |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** The function block FB_SocketClose can be used to close an open TCP/IP or UDP socket.

##### `FB_SocketCloseAll` — §5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/ |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** If TwinCAT is restarted or stopped, the TwinCAT TCP/IP Connection Server is also stopped. Any open sockets (TCP/IP and UDP connection handles) are closed automatically. The PLC program is reset after a "PLC reset", a "Rebuild all..." or a new "Download", and the information about already opened sockets (connection handles) is no longer available in the PLC. Any open connections can then no longer be closed properly.

##### `FB_SocketListen` — §5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId String containing the network address of the TwinCAT TCP/` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** Using the function block FB_SocketListen, a new listener socket can be opened via the TwinCAT TCP/ IP Connection Server. Via a listener socket, the TwinCAT TCP/IP Connection Server can 'listen' for incoming connection requests from remote clients. If successful, the associated connection handle is returned at the hListner output. This handle is required by the function block FB_SocketAccept [} 27]. If a listener socket is no longer required, it can be closed with the function block FB_SocketClose [} 24]. The listener sockets on an individual computer must have unique IP port numbers.

##### `FB_SocketAccept` — §5.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId String containing the network address of the TwinCAT TCP/` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bAccepted` | `BOOL` | This output is set if a new connection to a remote client was established. |
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** The remote client connection requests arriving at the TwinCAT TCP/IP Connection Server have to be acknowledged (accepted). The function block FB_SocketAccept accepts the incoming remote client connection requests, opens a new remote client socket and returns the associated connection handle. The connection handle is required by the function blocks FB_SocketSend [} 28] and FB_SocketReceive [} 30], for example, in order to exchange data with a remote client. All incoming connection requests first have to be accepted. If a connection is no longer required or undesirable, it can be closed with the function block FB_SocketClose [} 24].

##### `FB_SocketSend` — §5.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/ |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active |

**Description:** Using the function block FB_SocketSend, data can be sent to a remote client or remote server via the TwinCAT TCP/IP Connection Server. A remote client connection will first have to be established via the function block FB_SocketAccept [} 27], or a remote server connection via the function block FB_SocketConnect [} 23].

##### `FB_SocketReceive` — §5.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/ |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** Using the function block FB_SocketReceive, data from a remote client or remote server can be received via the TwinCAT TCP/IP Connection Server. A remote client connection will first have to be established via the function block FB_SocketAccept [} 27], and a remote server connection via the function block FB_SocketConnect [} 23]. The data can be received or sent in fragmented form (i.e. in several packets) within a TCP/IP network. It is therefore possible that not all data may be received with a single call of the FB_SocketReceive instance. For this reason, the instance has to be called cyclically (polling) within the PLC task, until all required data have been received. During this process, an rising edge is generated at the bExecute input, e.g. every 100 ms. If successful, the data received last are copied into the receive buffer. The nRecBytes output returns the number of the last successfully received data bytes. If no new data could be read during the last call, the function block returns no error and nRecBytes == zero.

##### `FB_SocketUdpCreate` — §5.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/ |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** The function block FB_SocketUdpCreate can be used to open a client/server socket for the User Datagram Protocol (UDP). If successful, a new socket is opened, and the associated socket handle is returned at the hSocket output. The handle is required by the function blocks FB_SocketUdpSendTo [} 32] and FB_SocketUdpReceiveFrom [} 34], for example, in order to exchange data with a remote device. If a UDP socket is no longer required, it can be closed with the function block FB_SocketClose [} 24]. The port address nLocalHost is internally reserved by the TCP/IP Connection Server for the UDP protocol (a "bind" is carried out). Several network adapters may exist in a PC. The input parameter sLocalHost determines the network adapter to be used. If the sLocalHost input variable is ignored (empty string), the TCP/IP Connection Server uses the default network adapter. This is usually the first network adapter from the list of the network adapters in the Control Panel.

##### `FB_SocketUdpSendTo` — §5.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId String containing the network address of the TwinCAT TCP/` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains |

**Description:** The function block FB_SocketUdpSendTo can be used to send UDP data to a remote device via the TwinCAT TCP/IP Connection Server. The UDP socket must first be opened with the function block FB_SocketUdpCreate [} 31].

##### `FB_SocketUdpReceiveFrom` — §5.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId String containing the network address of the TwinCAT TCP/` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** Using the function block FB_SocketUdpReceiveFrom, data from an open UDP socket can be received via the TwinCAT TCP/IP Connection Server. The UDP socket must first be opened with the function block FB_SocketUdpCreate [} 31]. The instance of the FB_SocketUdpReceive function block has to be called cyclically (polling) within the PLC task. During this process, an rising edge is generated at the bExecute

##### `FB_SocketUdpAddMulticastAddress` — §5.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId String containing the network address of the TwinCAT TCP/` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** Binds the server to a multicast IP address so that multicast packages can be received. This function block expects an already established UDP socket connection, which can be established via the function block FB_SocketUdpCreate [} 31].

##### `FB_SocketUdpDropMulticastAddress` — §5.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetI String containing the network address of the TwinCAT TCP/` | — |
| `d` | `IP Connection Server. For the local computer (default) an empty string may` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** Removes the binding to a multicast IP address that was previously set up via the function block FB_SocketUdpAddMulticastAddress [} 36].

##### `FB_TlsSocketConnect` — §5.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/IP Connection Server. For the local |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hSocket` | `T_HSOCKET` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active |

**Description:** The FB_TlsSocketConnect function block enables a client to establish a new TCP/IP connection to a remote server via the TwinCAT TCP/IP Connection Server, secured via TLS. If successful, a new socket is opened, and the associated connection handle is returned at the hSocket output. The connection handle is required by the function blocks FB_SocketSend [} 28] and FB_SocketReceive [} 30], for example, in order to exchange data with a remote server. If a connection is no longer required, it can be closed with the function block FB_SocketClose [} 24]. Several clients can establish a connection with the remote server at the same time.

##### `FB_TlsSocketListen` — §5.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hListener` | `T_HSOCKET` | Socket handle, which was created via the function block |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hListener` | `T_HSOCKET` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** The function block FB_TLsSocketListen can be used to open a new listener socket secured via TLS via the TwinCAT TCP/IP Connection Server. Via a listener socket, the TwinCAT TCP/IP Connection Server can 'listen' for incoming connection requests from remote clients. The socket handle created with the function block FB_TlsSocketCreate [} 41] can then be used by the function block FB_SocketAccept [} 27] to accept an incoming client request. If a listener socket is no longer required, it can be closed with the function block FB_SocketClose [} 24]. The listener sockets on an individual computer must have unique IP port numbers. Programming samples for using this function block can be found in our samples.

##### `FB_TlsSocketCreate` — §5.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/ |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** The function block FB_TlsSocketCreate can be used to create a new socket via the TwinCAT TCP/IP Connection Server, either for a server (bListener:=true) or client application (bListener:=false). Via a listener socket, the TwinCAT TCP/IP Connection Server can 'listen' for incoming connection requests from remote clients. If successful, the associated connection handle (hSocket) is returned at the hListner output. This handle is required by the function block FB_TlsSocketListen [} 40], and subsequently FB_SocketAccept [} 27]. If a listener socket is no longer required, it can be closed with the function block FB_SocketClose [} 24]. After the execution of the function block FB_TlsSocketCreate TLS parameters can be set to secure the communication connection. This is done using the function blocks FB_TlsSocketAddCa [} 42], FB_TlsSocketAddCrl [} 43], FB_TlsSocketSetCert [} 44] and FB_TlsSocketSetPsk [} 45]. Programming samples for this can be found in our samples.

##### `FB_TlsSocketAddCa` — §5.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/ |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** The FB_TlsSocketAddCa function block is used to configure the path to a CA certificate for an existing socket handle. The certificate file must be in PEM format. Programming samples for using this function block can be found in our samples.

##### `FB_TlsSocketAddCrl` — §5.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/ |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** The function block FB_TlsSocketAddCrl is used to specify the path to a CRL file for an existing socket handle. The CRL must be in PEM format. Programming samples for using this function block can be found in our samples.

##### `FB_TlsSocketSetCert` — §5.1.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/IP Connection Server. For the local |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** The function block FB_TlsSocketSetCert can be used to configure a client/server certificate that is to be used for a specific socket handle. The certificates must be in PEM format. Programming samples for using this function block can be found in our samples.

##### `FB_TlsSocketSetPsk` — §5.1.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT TCP/IP Connection Server. For the local |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is active if the function block is activated. It remains active until |

**Description:** The function block FB_TlsSocketSetPsk can be used to configure a pre-shared secret for an existing socket handle. Programming samples for using this function block can be found in our samples.

#### PLC API › Function blocks › Helper

##### `FB_ClientServerConnection` — §5.1.20.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetID` | `T_AmsNetI String containing the AMS network address of the TwinCAT TCP/` | — |
| `D` | `IP Connection Server. For the local computer (default) an empty string may be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE, as long as the function block is active. |
| `bError` | `BOOL` | Becomes TRUE if an error code occurs. |
| `nErrID` | `UDINT` | If an bError output is set, this parameter returns the TwinCAT TCP/ |

**Description:** The function block FB_ClientServerConnection can be used to manage (establish or remove) a client connection. FB_ClientServerConnection simplifies the implementation of a client application by encapsulating the functionality of the two function blocks FB_SocketConnect [} 23] and FB_SocketClose [} 24]

##### `FB_ServerClientConnection` — §5.1.20.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `E_SocketAcceptM Defines whether all or only certain connections [} 56] are to be accepted. ode` | — |
| `sRemot T_IPv4Addr` | `IP address (Ipv4) in string form (e.g. '172.33.5.1') of the remote client whose` | — |
| `eHost` | `connection is to be accepted. For a client on the local computer an empty` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hServer` | `T_HSERVER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE, as long as the function block is active. |
| `bError` | `BOOL` | Becomes TRUE if an error code occurs. |
| `nErrId` | `UDINT` | If an bError output is set, this parameter returns the TwinCAT TCP/ |

**Description:** The function block FB_ServerClientConnection can be used to manage (establish or remove) a server connection. FB_ServerClientConnection simplifies the implementation of a server application by encapsulating the functionality of the three function blocks FB_SocketListen [} 26], FB_SocketAccept [} 27] and FB_SocketClose [} 24] internally. The integrated debugging output of the connection status facilitates troubleshooting in the event of configuration or communication errors. In addition, a minimum server application only requires an instance of the function block FB_SocketSend [} 28] and/or an instance of the function block FB_SocketReceive [} 30].

##### `FB_ConnectionlessSocket` — §5.1.20.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetID` | `T_AmsNetI String containing the AMS network address of the TwinCAT TCP/` | — |
| `D` | `IP Connection Server. For the local computer (default) an empty string may be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE, as long as the function block is active. |
| `bError` | `BOOL` | Becomes TRUE if an error code occurs. |
| `nErrID` | `UDINT` | If an bError output is set, this parameter returns the TwinCAT TCP/ |

**Description:** A UDP socket can be managed (opened/generated and closed) with the function block FB_ConnectionlessSocket. FB_ConnectionlessSocket simplifies the implementation of a UDP application by encapsulating the functionality of the two function blocks FB_SocketUdpCreate [} 31] and FB_SocketClose [} 24] already internally. The integrated debugging output of the socket status facilitates troubleshooting in the event of configuration or communication errors. In addition, a minimum UDP application only requires an instance of the function block SocketUdpSendTod [} 32] and/or an instance of the function block FB_SocketUdpReceiveFrom [} 34].

#### PLC API › Functions

##### `F_CreateServerHnd` (FUNCTION) → `BOOL` — §5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetID` | `T_AmsNetID String containing the AMS network address of the TwinCAT TCP/` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hServer` | `T_HSERVER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_CreateServerHnd` | `BOOL` | Returns TRUE if everything is OK, FALSE if there is an incorrect |

**Description:** The function F_CreateServerHnd is used to initialize/set the internal parameters of a server handle variable hServer. The server handle is then transferred to the instances of the function block FB_ServerClientConnection [} 48] via VAR_IN_OUT. An instance of the FB_ServerClientConnection function block can be used to manage (establish or remove) a sever connection in a straightforward manner. The same server handle can be transferred to several instances of the function block FB_ServerClientConnection, in order to enable the server to establish several concurrent connections.

##### `HSOCKET_TO_STRING` (FUNCTION) → `STRING` — §5.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hSocket` | `T_HSOCKET` | The connection handle [} 61] to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HSOCKET STRING` | `Contains the STRING representation of the connection handle. _TO_STRI NG` | — |

**Description:** The function converts the connection handle of type T_HSOCKET to a string (e.g. for debug outputs).

##### `HSOCKET_TO_STRINGEX` (FUNCTION) → `STRING` — §5.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hSocket` | `T_HSOCKET The connection handle [} 61] to be converted.` | — |
| `bLocal` | `BOOL` | TRUE: Include the local address, FALSE: Exclude the local address. |
| `bRemote` | `BOOL` | TRUE: Include the remote address, FALSE: Exclude the remote address. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HSOCKET_TO_ST STRING` | `Contains the hex-based STRING representation of the connection` | — |
| `RINGEX` | `handle.` | — |

**Description:** The function converts the connection handle of type T_HSOCKET to a string (e.g. for debug outputs).

##### `SOCKETADDR_TO_STRING` (FUNCTION) → `STRING` — §5.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stSockeAddr` | `ST_SockAddr` | The variable to be converted. See ST_SockAddr [} 59] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SOCKETADDR_TO STRING` | `Contains the STRING representation of the socket address. _STRING` | — |

**Description:** The function converts a variable of type ST_SockAddr to a string (e.g. for debug outputs).

#### Samples › TCP › Sample01: "Echo" client/server (basic function blocks)

##### `Overview` (FUNCTION) → `DINT` — §6.1.1.1

**Return type:** `DINT`

**Description:** The following example shows an implementation of an "echo" client/server. The client sends a test string to the server at certain intervals (e.g. every second). The remote server then immediately resends the same string to the client.

#### Samples › TCP › Sample01: "Echo" client/server (basic function blocks) › PLC Client

##### `FB_LocalClient` (FUNCTION_BLOCK) — §6.1.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRemoteHost` | `STRING(15) := '127.0.0.1';(* IP adress of remote server *)` | — |
| `nRemotePort` | `UDINT := 0` | — |
| `sToServer` | `T_MaxString:= 'TEST'` | — |
| `bEnable` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bConnected` | `BOOL` | — |
| `hSocket` | `T_HSOCKET` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrId` | `UDINT` | — |
| `sFromServer` | `T_MaxString` | — |

**Description:** If the bEnable input is set, the system will keep trying to establish the connection to the remote server once the CLIENT_RECONNECT_TIME has elapsed. The remote server is identified via the sRemoteHost IP address and the nRemotePort IP port address. The data exchange with the server was encapsulated in a separate function block FB_ClientDataExcha [} 72]. The data exchange is cyclic, always after PLCPRJ_SEND_CYCLE_TIME has expired. The sToServer string variable is sent to the server and the string returned by the server is available at the sFormServer output. Another implementation, in which the remote server is addressed as required is also possible. In the event of an error, the existing connection is closed, and a new connection is established.

##### `FB_ClientDataExcha` (FUNCTION_BLOCK) — §6.1.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hSocket` | `T_HSOCKET` | — |
| `sToServer` | `T_MaxString` | — |
| `bExecute` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrId` | `UDINT` | — |
| `sFromServer` | `T_MaxString` | — |

**Description:** In the event of an rising edge at the bExecute input, a null-terminated string is sent to the remote server, and a string returned by the remote server is read. The function block will try reading the data until null termination was detected in the string received. If the PLCPRJ_RECEIVE_TIMEOUT time-out period is exceeded or if an error occurs, reception is aborted. Data are attempted to be read again after a certain delay time, if no new data could be read during the last read attempt. This reduces the system load.

#### Samples › TCP › Sample01: "Echo" client/server (basic function blocks) › PLC Server

##### `FB_LocalServer` (FUNCTION_BLOCK) — §6.1.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sLocalHost` | `STRING(15) := '127.0.0.1';(* own IP address! *)` | — |
| `nLocalPort` | `UDINT := 0` | — |
| `bEnable` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bListening` | `BOOL` | — |
| `hListener` | `T_HSOCKET` | — |
| `nAcceptedClients` | `UDINT` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrId` | `UDINT` | — |

**Description:** The server must first be allocated a unique sLocalHost IP address and an nLocaPort IP port number. If the bEnable input is set, the local server will repeatedly try to open the listener socket once the SERVER_RECONNECT_TIME has elapsed. The listener socket can usually be opened at the first attempt, if the TwinCAT TCP/IP Connection Server is located on the local PC. The functionality of a remote client was encapsulated in the function block FB_RemoteClient [} 76]. The remote client instances are activated once the listener socket was opened successfully. Each instance of the FB_RemoteClient corresponds to a remote client, with which the local server can communicate simultaneously. The maximum number of remote clients communicating with the server can be modified via the value of the MAX_CLIENT_CONNECTIONS constant. In the event of an error, all remote client connections are closed first and then the listener socket. The nAcceptedClients output provides information about the current number of connected clients.

##### `FB_RemoteClient` (FUNCTION_BLOCK) — §6.1.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hListener` | `T_HSOCKET` | — |
| `bEnable` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bAccepted` | `BOOL` | — |
| `hSocket` | `T_HSOCKET` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sFromClient` | `T_MaxString` | — |

**Description:** If the bEnable input is set, an attempt is made to accept the connection request of a remote client, once the SERVER_ACCEPT_POOLING_TIME has elapsed. The data exchange with the remote client was encapsulated in a separate function block FB_ServerDataExcha [} 78]. Once the connection was established successfully, the instance is enabled via the FB_ServerDataExcha function block. If an error occurs, the accepted connection is closed and a new one is established.

##### `FB_ServerDataExcha` (FUNCTION_BLOCK) — §6.1.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hSocket` | `T_HSOCKET` | — |
| `bExecute` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sFromClient` | `T_MaxString` | — |

**Description:** In the event of an rising edge at the bExecute input, a zero-terminated string is read by the remote client and returned to the remote client, if zero termination was detected. The function block will try reading the data until zero termination was detected in the string received. Reception is aborted in the event of an error, and if no new data were received within the PLCPRJ_RECEIVE_TIMEOUT timeout time. Data are attempted to be read again after a certain delay time, if no new data could be read during the last read attempt. This reduces the system load.

#### Samples › UDP › Sample01: Peer-to-peer communication

##### `Overview` (FUNCTION_BLOCK) — §6.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `new` | `ST_FifoEntry` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | — |
| `old` | `ST_FifoEntry` | — |

**Description:** The following example demonstrates the implementation of a simple Peer-to-Peer application in the PLC and consists of two PLC projects (PeerA and PeerB) plus a .NET application which also acts as a separate peer. All peer applications send a test string to a remote peer and at the same time receive strings from a remote peer. The received strings are displayed in a message box on the monitor of the target computer. Feel free to use and customize this sample to your needs.

##### `PLC devices A and B` (FUNCTION_BLOCK) — §6.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sLocalHost` | `STRING(15)` | — |
| `nLocalPort` | `UDINT` | — |
| `bEnable` | `BOOL` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sendFifo` | `FB_Fifo` | — |
| `receiveFifo` | `FB_Fifo` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bCreated` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrId` | `UDINT` | — |

**Description:** The required functionality was encapsulated in the function block FB_PeerToPeer. Each of the communication partners uses an instance of the FB_PeerToPeer function block. The block is activated through a rising edge at the bEnable input. A new UDP socket is opened, and data exchange commences. The socket address is specified via the variables sLocalHost and nLocalPort. A falling edge stops the data exchange and closes the socket. The data to be sent are transferred to the block through a reference (VAR_IN_OUT) via the variable sendFifo. The data received are stored in the variable receiveFifo.
