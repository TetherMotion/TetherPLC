# Tc3_RealtimeMonitoring

- **Manual:** `TE1010_TC3_Realtime_Monitor_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Engineering · **Tier:** T1 · **API entries:** 12
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Runtime introspection (cycle times, task stats) extends the existing debug plane.

---

#### Function blocks

##### `FB_RTMon_LogMark` — §12.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `set with each mark` | `` | — |
| `nContextId` | `UINT` | Context ID for which this marker is to be set. The initial |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Boolean value that indicates whether the function block is |

**Description:** The FB_RTMon_LogMark is an extended function block that enables the setting of "simple" (time) markers.

#### Function blocks › FB_RTMon_LogMark

##### `LogIntervalStart` — §12.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGroupId` | `UINT` | Marker ID (marker group) for which the marker is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LogIntervalStart` | `HRESULT` | Returns the error code in the event of an error. |

**Description:** The method creates a marker with an interval start for the transferred Marker ID.

##### `LogIntervalStop` — §12.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGroupId` | `UINT` | Marker ID (marker group) for which the marker is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LogIntervalStop` | `HRESULT` | Returns the error code in the event of an error. |

**Description:** The method creates a marker with an interval stop for the transferred Marker ID.

##### `LogMark` — §12.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGroupId` | `UINT` | Marker ID (marker group) for which the marker is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LogMark` | `HRESULT` | Returns the error code in the event of an error. |

**Description:** The method creates a marker for the marker ID that was transferred. Optionally, the event ID can be used to distinguish between different user events or to display additional data (formatted as UINT) in the TwinCAT 3 Realtime Monitor display.

##### `LogSequenceStart` — §12.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGroupId` | `UINT` | Marker ID (marker group) for which the marker is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LogSequenceSta HRESULT` | `Returns the error code in the event of an error. rt` | — |

**Description:** The method creates a marker with a sequence start for the marker ID that was transferred.

##### `LogSequenceStop` — §12.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGroupId` | `UINT` | Marker ID (marker group) for which the marker is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LogSequenceSto HRESULT` | `Returns the error code in the event of an error. p` | — |

**Description:** The method creates a marker with a sequence stop for the marker ID that was transferred.

##### `LogStart` — §12.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGroupId` | `UINT` | Marker ID (marker group) for which the marker is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LogStart` | `HRESULT` | Returns the error code in the event of an error. |

**Description:** The method creates a marker with a sequence and interval start for the transferred marker ID. Thus, this marker represents the time of a process at which it is immediately active / started.

##### `LogStop` — §12.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGroupId` | `UINT` | Marker ID (marker group) for which the marker is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LogStop` | `HRESULT` | Returns the error code in the event of an error. |

**Description:** The method creates a marker with a sequence and interval stop for the passed marker ID. Thus, this marker represents the point in time of a process at which it is terminated directly.

##### `LogTrigger` — §12.1.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGroupId` | `UINT` | Marker ID (marker group) for which the marker is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LogTrigger` | `HRESULT` | Returns the error code in the event of an error. |

**Description:** The method creates a marker with a trigger for the transferred marker ID. The real-time monitor server can be configured to start recording based on this trigger.

#### Function blocks

##### `FB_RTMon_LogMarkBase` — §12.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `hrErrorCode` | `HRESULT` | Outputs the error code that has occurred. |

**Description:** The FB_RTMon_LogMarkBase is a generic function block that enables the setting of (time) marks.

#### Function blocks › FB_RTMon_LogMarkBase

##### `LogMark` — §12.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `60 PLC API` | `` | — |
| `nContextId` | `UINT` | Defines the context ID under which the marker is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LogMark` | `HRESULT` | Returns the error code in the event of an error. |

**Description:** The method creates a marker for the transferred marker group ID. The marker type is transferred using the parameter nMarkCtrl (see TcMarkOption [} 62]).

##### `LogMarkEx` — §12.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stMark` | `ST_RTMon_MarkDef [} 61]` | Transfer parameter for a defined marker that is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LogMarkEx` | `HRESULT` | Returns the error code in the event of an error. |

**Description:** The method creates a marker. The marker is defined using the data type ST_RTMon_MarkDef [} 61]. The marker type is transferred using the parameter nMarkCtrl (see TcMarkOption [} 62]).
