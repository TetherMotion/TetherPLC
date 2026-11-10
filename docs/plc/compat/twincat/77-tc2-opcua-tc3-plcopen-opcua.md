# Tc2_OpcUa+Tc3_PLCopen_OpcUa

- **Manual:** `TF6100_TC3_OPC_UA_Server_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 8
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** OPC UA server: needs a full OPC UA stack (address space, services, security policies) — multi-month effort; PLCopen companion-model FBs are T0 once the server exists.

---

#### Technical introduction › Data Access › Enabling symbols

##### `PLC` (FUNCTION_BLOCK) — §4.11.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ni1` | `INT` | — |
| `ni2` | `INT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `no1` | `INT` | — |
| `no2` | `INT` | — |

**Description:** This section describes how to configure the namespace of the TwinCAT OPC UA Server in order to gain access to the symbols of a PLC project. This requires the following steps to be carried out:

#### Technical introduction › Data Access

##### `Structures` (FUNCTION_BLOCK) — §4.11.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input1` | `INT` | — |
| `Input2` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output1` | `LREAL` | — |

**Description:** The TwinCAT OPC UA Server enables the use of so-called StructuredTypes for structures from the TwinCAT 3 PLC. StructuredTypes allow structures to be read or written in a data-consistent manner and the type description of the structure to be made available to the client.

##### `Properties` (PROPERTY) → `BOOL` — §4.11.9

**Return type:** `BOOL`

**Description:** The display of PLC properties in the server namespace has been supported since TwinCAT 3.1 Build 4024. All you need to do is set two special PLC attributes on the Property so that the Property is imported into the namespace and represented there as a UA Property. Sample:

#### Technical introduction › Method calls

##### `Job methods` (FUNCTION_BLOCK) — §4.14.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `http` | `//www.opcfoundation.org/UA/schemas/StatusCode.csv` | — |
| `http` | `//www.opcfoundation.org/UA/schemas/StatusCode.csv` | — |
| `a` | `INT; // some description` | — |
| `b` | `LREAL; // some description` | — |
| `c` | `BOOL; // some description` | — |
| `d` | `ST_Complex_1; // some description` | — |
| `hdl` | `UDINT; // handle, can be used for concurrent calls` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hdl` | `UDINT; // handle, can be used for concurrent calls` | — |
| `bBusy` | `BOOL; // required` | — |
| `a` | `INT; // some description` | — |
| `b` | `LREAL; // some description` | — |
| `c` | `BOOL; // some description` | — |
| `d` | `ST_Complex_1; // some description` | — |

**Description:** The concept of Job methods has a fundamental difference compared to regular method calls: the OPC UA methods are no longer mapped 1:1 to a PLC method, but instead to a function block with a specific signature. This also allows method calls to be realized that take longer than one cycle from the perspective of a PLC application.

##### `RPC methods` (METHOD) → `INT` — §4.14.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `INT` | — |
| `b` | `INT` | — |

**Description:** Method calls are a fundamental part of the OPC UA specification. With the introduction of these functionalities into the PLC world, TwinCAT 3 offers the possibility of efficiently executing RPC calls in the IEC61131 world and thus reduces the classic handshake patterns for communication between devices.

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
| `[} 148]` | `ServerInfo is selected at the eOpcUAServerOption input.` | — |
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
| `s` | `[} 149]` | — |
| `bDone` | `BOOL` | TRUE when processing of the function block is complete. |
| `bBusy` | `BOOL` | TRUE as long as processing of the function block is in |

**Description:** The function block enables the current status (Running, NotResponding) of a TwinCAT OPC UA Server to be read. It should be noted at this point that this function block deals with the ADS interface of the OPC UA Server. If the OPC UA Server is restarted or shut down, the ADS interface of the server remains accessible. The ADS interface can only be closed by terminating the server process.

#### PLC API › Tc3_OpcUa › Function blocks › Server

##### `FB_OpcUaServer_SetServiceLevel` (FUNCTION) — §5.2.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tAmsNetId` | `T_AmsNetId` | AmsNetId of the system on which the TwinCAT OPC UA Server |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Is TRUE as long as the function block is busy with processing. |
| `bError` | `BOOL` | Becomes TRUE when an error situation occurs. |
| `nErrorId` | `UDINT` | ADS error code. |

**Description:** The TwinCAT OPC UA Server currently sets the ServiceLevel statically to the value 255. This function block enables the ServiceLevel to be set via an ADS interface in the server. A distinction is made between use with and without a watchdog.
