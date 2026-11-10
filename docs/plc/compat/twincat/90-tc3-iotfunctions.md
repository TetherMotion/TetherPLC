# Tc3_iotFunctions

- **Manual:** `TF6710_TC3_IoT_Functions_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 11
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** IoT functions (telemetry, cloud connectors): socket service + TLS.

---

#### PLC API › Function blocks

##### `FB_IotFunctions_Connector` (FUNCTION_BLOCK) — §6.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Designation` | `Type` | Description |
| `sAmsNetId` | `STRING` | Target Data Agent Instance AmsNetId [127.0.0.1.1.1] |
| `nDefaultRequestTimeout UDINT` | `Default timeout value in milliseconds [10000]` | — |
| `nDefaultMessageRetryInter UDINT` | `Default message retry interval in milliseconds (0 disables` | — |
| `val` | `retries) [0]` | — |
| `nDefaultMessageCumulati UDINT` | `Default cumulative timeout in milliseconds (if message` | — |
| `veTimeout` | `data is successfully received, the timeout of the relevant request is extended by this value) [0]` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Designation` | `Type` | Description |
| `hrInitializationErrorCode` | `HRESULT` | HRESULT of driver instantiation and configuration |
| `hrLastMessageErrorCode` | `HRESULT` | HRESULT of latest reported error occurred in any |

**Description:** The function block enables communication with a local/remote TC3 IoT Data Agent installation. The Execute method of the function block must be called cyclically in order to ensure the background communication with the TC3 IoT Data Agent to facilitate the reception of messages. All connection parameters and global settings are available as input parameters.

#### PLC API › Function blocks › FB_IotFunctions_Connector

##### `Execute` (METHOD) → `HRESULT` — §6.1.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `HRESULT` | Indicates whether the call was successful. |

**Description:** This method allows background communication with the TC3 IoT Data Agent. It must be called cyclically.

#### PLC API › Function blocks

##### `FB_IotFunctions_Message` (FUNCTION_BLOCK) — §6.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Designation` | `Type` | Description |
| `nChannelId` | `UINT` | Channel ID of the corresponding IotFunctions channel in |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Designation` | `Type` | Description |
| `hResult` | `HRESULT` | Contains the last hresult (is updated when bBusy or bError are accessed or Read/Write is called) |
| `pStMessageDetails` | `POINTER TO` | Pointer to structure containing detailed information about ST_IotFunctionsMess the underlying message age [} 44] |
| `pStRequestDetails` | `POINTER TO` | Pointer to structure containing detailed information about |

**Description:** The function block provides read/write operations for messages. All message parameters as well as global settings exist as input parameters.

#### PLC API › Function blocks › FB_IotFunctions_Message

##### `Read` (METHOD) → `HRESULT` — §6.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPayload` | `PVOID` | Pointer to the target memory. |
| `nPayload` | `UDINT` | Size of the target memory. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `HRESULT` | Indicates whether the call was successful. |

**Description:** This method reads a message from the channel.

##### `Acknowledge` (METHOD) → `BOOL` — §6.1.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Acknowledge BOOL` | `Indicates whether the call was successful.` | — |

**Description:** The method acknowledges the error/success status and releases the associated message objects. Call this method after status acknowledgement if no new read/write process is started in this cycle to prevent multiple evaluation of the last status.

##### `Write` (METHOD) → `BOOL` — §6.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPayload` | `PVOID` | Address of the target memory. |
| `nPayloadSize` | `UDINT` | Size of the target memory. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Write` | `BOOL PLC API` | Indicates whether the call was successful. |

**Description:** This method writes a message to the channel.

#### PLC API › Function blocks

##### `FB_IotFunctions_Request` (FUNCTION_BLOCK) — §6.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Designation` | `Type` | Description |
| `iotFunctionsDriverOTCID` | `OTCID` | OTCID of the target IotFunctions driver instance [defaults to the first created object of the FB_IotFunctions_Connector] |
| `nRequestTimeout` | `UDINT` | Timeout value in milliseconds |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Designation` | `Type` | Description |
| `hResult` | `HRESULT` | Contains the hresult of the last message (is updated when bBusy or bError is accessed or Create/Execute/ EnqueueRead/EnqueueWrite is called) |
| `pStRequestDetails` | `POINTER TO` | Pointer to structure containing details information about ST_IotFunctionsRequ the underlying request est [} 45] |
| `bInitialized` | `BOOL` | Indicates whether interface search and initialization was successful |
| `fbTcResultEvent` | `FB_TcIotFunctionsR Instance of a helper function block that contains detailed` | — |
| `esultEvent` | `information about the send or receive process.` | — |
| `Designation` | `Description Create [} 42]` | Creates a new request |
| `EnqueueRead [} 42]` | `Adds an instance of FB_IotFunctions_Message [} 38] for read operation` | — |
| `EnqueueWrite [} 43]` | `Adds an instance of FB_IotFunctions_Message [} 38] for write operations Execute [} 43]` | Executes the request |
| `Acknowledge [} 40]` | `Acknowledges the error/success status and releases the associated message` | — |

**Description:** The function block enables the synchronization of multiple messages (see Synchronizing message operations [} 19]).

#### PLC API › Function blocks › FB_IotFunctions_Request

##### `Create` (METHOD) → `BOOL` — §6.1.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Create` | `BOOL` | Indicates whether the call was successful. |

**Description:** This method creates a new request.

##### `EnqueueRead` (METHOD) → `BOOL` — §6.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pMessageFB` | `POINTER TO` | Pointer to an object of type FB_IotFunctions_Message. FB_IotFunctions_Mes sage |
| `pPayload` | `POINTER TO PVOID Address of the content.` | — |
| `nPayloadSize` | `UDINT` | Size of the content. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EnqueueRead` | `BOOL` | Indicates whether the call was successful. |

**Description:** The method adds an instance of FB_IotFunctions_Message [} 38] for read operations.

##### `EnqueueWrite` (METHOD) → `BOOL` — §6.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pMessageFB` | `POINTER TO` | Pointer to an object of type FB_IotFunctions_Message. FB_IotFunctions_Mes sage |
| `pPayload` | `POINTER TO PVOID Address of the content.` | — |
| `nPayloadSize` | `UDINT` | Size of the content. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EnqueueWrite` | `BOOL` | Indicates whether the call was successful. |

**Description:** Adds an instance of FB_IotFunctions_Message [} 38] for write operations.

##### `Execute` (METHOD) → `BOOL` — §6.1.3.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Indicates whether the call was successful. |

**Description:** Syntax
