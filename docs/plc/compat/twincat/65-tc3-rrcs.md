# Tc3_RRCS

- **Manual:** `Riedel_Communications_RRCS_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Industry · **Tier:** T2 · **API entries:** 5
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Riedel broadcast router control: TCP/serial transport to Riedel devices.

---

#### Programming › Function blocks

##### `FB_RRCScom` — §5.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eCommState` | `E_CommState` | The state of the function block. |
| `nStatusCode` | `UINT` | Servers deliver the status code as a response to every |

**Description:** This function block is required for communication with the RRCS server

##### `FB_GetState` — §5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eRRCS_ErrorCode` | `E_RRCS_ErrorCodes [} 19]` | Error code received from RRCS_Servers. |
| `bBusy` | `BOOL` | Is TRUE as long as the function block is busy with the |

**Description:** This function block queries the state of the RRCS server.

##### `FB_GetAlive` — §5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eRRCS_ErrorCode` | `E_RRCS_ErrorCodes [} 19]` | Error code received from RRCS_Servers. |
| `bBusy` | `BOOL` | Is TRUE as long as the function block is busy with the |

**Description:** To ensure that there is a correct connection between TwinCAT and the RRCS server, the function block FB_GetAlive can be used.

##### `FB_IsConnectedToArtist` — §5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eRRCS_ErrorCode` | `E_RRCS_ErrorCodes [} 19]` | Error code received from RRCS_Servers. |
| `bBusy` | `BOOL` | Is TRUE as long as the function block is busy with the |
| `eRRCS_ErrorCode` | `E_RRCS_ErrorCodes [} 19]` | Error code received from RRCS_Servers. |
| `bBusy` | `BOOL` | Is TRUE as long as the function block is busy with the |

**Description:** This function block is used to query whether the RRCS server is communicating with an Artist device.

##### `FB_SetLogicSourceState` — §5.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AR_OUTPUT` | `` | — |

**Description:** This function block offers the option of changing the state of a 'Logic Source'. This is done using the unique ObjectID.
