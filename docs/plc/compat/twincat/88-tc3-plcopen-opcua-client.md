# Tc3_PLCopen_OpcUa-client

- **Manual:** `TF6100_TC3_OPC_UA_Client_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 18
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** OPC UA client: same OPC UA stack requirement, client role.

---

#### PLC API › Tc2_OpcUa › Function blocks

##### `FB_OpcUAServer` (FUNCTION_BLOCK) — §5.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | AmsNetId of the system on which the TwinCAT OPC |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stOpcUAServerInfo` | `ST_OpcUAServerInfo` | Contains status information from the server when |
| `[} 62]` | `ServerInfo is selected at the eOpcUAServerOption input.` | — |
| `bBusy` | `BOOL` | TRUE as long as processing of the function block is in |

**Description:** The function block enables status information to be read out and a TwinCAT OPC UA Server to be restarted.

##### `FB_OpcUAServerGetStatus` (FUNCTION_BLOCK) — §5.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | AmsNetId of the system on which the TwinCAT OPC |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eOPCUAServerStatu E_OpcUAServerStatus` | `Contains status information about the server.` | — |
| `s` | `[} 63]` | — |
| `bDone` | `BOOL` | TRUE when processing of the function block is complete. |
| `bBusy` | `BOOL` | TRUE as long as processing of the function block is in |

**Description:** The function block enables the current status (Running, NotResponding) of a TwinCAT OPC UA Server to be read. It should be noted at this point that this function block deals with the ADS interface of the OPC UA Server. If the OPC UA Server is restarted or shut down, the ADS interface of the server remains accessible. The ADS interface can only be closed by terminating the server process.

#### PLC API › Tc3_PLCopen_OpcUa › Function blocks

##### `UA_Browse` — §5.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReferenceDescriptions` | `POINTER TO ST_UAReferenceDescriptions` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for |

**Description:** This function block allows browsing through the namespace of a server. Starting from a start node, its references are read and returned accordingly.

##### `UA_Connect` — §5.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConnectionHdl` | `DWORD` | OPC UA connection handle. |
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the |

**Description:** This function block establishes an OPC UA Remote connection to another OPC UA Server, which is specified via ServerUrl and SessionConnectInfo. The function block returns a connection handle that can be used for other function blocks, such as UA_Read.

##### `UA_ConnectGetStatus` — §5.2.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle of an existing communication link. |
| `GetServiceLevel` | `BOOL` | Reads out the ServiceLevel of the OPC UA Server. |
| `Timeout` | `TIME` | Time until the function is aborted. DEFAULT_ADS_TIMEOUT is a global |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Switches to TRUE if the function block was executed |

**Description:** This function block checks the connection status of an existing connection to another OPC UA Server. The connection is referenced via the respective connection handle. The status is then returned as E_UAConnectionStatus [} 69]. The connection status is determined based on the internal session info or the OPC UA heartbeat, no additional communication (read or similar) is performed.

##### `UA_Disconnect` — §5.2.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle previously output by the function block UA_Connect. |
| `Timeout` | `TIME` | Time until the function is aborted. DEFAULT_ADS_TIMEOUT is a global |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the |

**Description:** This function block closes an OPC UA Remote connection to another OPC UA Server. The connection is specified via its connection handle.

##### `UA_GetNamespaceIndex` — §5.2.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle previously output by the function block UA_Connect. |
| `NamespaceUri` | `STRING` | Namespace URI to be resolved. For the TwinCAT OPC UA Server, this is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NamespaceIndex` | `UINT` | Namespace Index of the given namespace URI. This can be used in other function blocks, e.g. UA_NodeGetHandle or |

**Description:** This function block collects the namespace index for a namespace URI. The namespace index is required for identifying symbols, for example, if the function blocks UA_Read [} 96]or UA_Write [} 99]are used.

##### `UA_HistoryUpdate` — §5.2.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle previously output by the function block UA_Connect. |
| `NodeHdl` | `DWORD` | Node handle that was previously output by the function block |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DataValues` | `ARRAY[*] OF UAHADataValue` | — |
| `ValueErrorIDs` | `ARRAY[*] OF DWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the |

**Description:** This function block sends historical data via OPC UA to a server that supports the OPC UA HistoryUpdate function, e.g. the TwinCAT OPC UA Server. With one call you can transfer a large number of values including time stamps to the server for a node handle. The server ensures that the values transmitted are saved in a data memory and are available via Historical Access.

##### `UA_MethodCall` — §5.2.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pOutputArgInfo` | `POINTER TO ST_UAMethodArgInfo` | Points to the buffer address where output parameter information is stored as array |
| `cbRead_R` | `UDINT` | Counts all the bytes received. |
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the |

**Description:** This function block calls a method on a remote UA Server. The method is determined by a connection and a method handle. The former can be queried by UA_Connect [} 82], the latter by UA_MethodGetHandle [} 91].

##### `UA_MethodGetHandle` — §5.2.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle previously output by the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MethodHdl` | `DWORD` | Returns a method handle that can be used to call a method via UA_MethodCall |

**Description:** This function block collects a handle for a UA method, which can then be used to call a method using UA_MethodCall [} 89].

##### `UA_MethodReleaseHandle` — §5.2.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle previously output by the function block UA_Connect. |
| `MethodHdl` | `DWORD` | Method handle previously output by the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the duration |

**Description:** This function block releases the specified method handle.

##### `UA_NodeGetHandle` — §5.2.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle previously output by the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeHdl` | `DWORD` | Node handle that can be used for other function blocks, such as UA_Read or |

**Description:** This function block queries a node handle for a given symbol in the UA namespace. The symbol is specified by a connection handle and its node ID.

##### `UA_NodeGetHandleList` — §5.2.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle previously output by the function block UA_Connect. |
| `NodeIDCount` | `UINT` | Number of nodes for which a node handle is required. |
| `NodeIDs` | `ARRAY` | Array of NodeIDs created with struct ST_UANodeID [} 77]. |
| `Timeout` | `TIME` | Time until the function is aborted. DEFAULT_ADS_TIMEOUT is a global |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeHdls` | `ARRAY` | Array of requested node handles. |
| `NodeErrorIDs` | `ARRAY` | Array of error IDs if no node handles are available. |
| `cbData_R` | `UDINT` | Size of the data read. |
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the |

**Description:** This function block queries node handles for nodes in the UA namespace.

##### `UA_NodeReleaseHandle` — §5.2.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle previously output by the function block UA_Connect. |
| `NodeHdl` | `DWORD` | Node handle to be released. |
| `Timeout` | `TIME` | Time until the function is aborted. DEFAULT_ADS_TIMEOUT is a global |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the |

**Description:** This function block releases a node handle.

##### `UA_NodeReleaseHandleList` — §5.2.3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle previously output by the function block UA_Connect. |
| `NodeHdlCount` | `UINT` | Number of node handles. |
| `NodeHdls` | `ARRAY` | Array of node handles to be released. |
| `Timeout` | `TIME` | Time until the function is aborted. DEFAULT_ADS_TIMEOUT is a global |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeErrorIDs` | `ARRAY` | Array of error IDs if a node handle could not be released. |
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the |

**Description:** This function block releases several node handles.

##### `UA_Read` — §5.2.3.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `Connection` | `DWORD` | Connection handle previously output by the function block |
| `Hdl` | `UA_Connect.` | — |
| `NodeHdl` | `DWORD` | Node handle that was previously output by the function |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the |

**Description:** This function block reads values from a given node and connection handle.

##### `UA_ReadList` — §5.2.3.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ConnectionHdl` | `DWORD` | Connection handle previously output by the function block UA_Connect |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the |

**Description:** This function block reads values from several given node and connection handles.

##### `UA_Write` — §5.2.3.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Switches to TRUE if the function block was executed successfully. |
| `Busy` | `BOOL` | TRUE until the function block has executed a command, at the most for the |

**Description:** This function block writes values to a given node and connection handle.
