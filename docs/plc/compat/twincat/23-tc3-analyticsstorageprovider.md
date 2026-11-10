# Tc3_AnalyticsStorageProvider

- **Manual:** `TF3520_TC3_Analytics_Storage_Provider_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Measurement · **Tier:** T4 · **API entries:** 50
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Needs SQL/NoSQL storage driver.

---

#### ERROR_SYMBOL_NOT_SELECTED The symbol is not selected. › PLC API › Function blocks › Topic Architecture › Commands

##### `T_ALY_SPCancel_Cmd` (FUNCTION_BLOCK) — §7.1.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eCancelType` | `E_CancelType [} 168]` | — |
| `arrParameter` | `ARRAY [0..99] OF T_MaxString Methods` | — |
| `Reset` | `Reset all values in the payload FB.` | — |
| `Init_JsonValue` | `Inherited from` | Initialization of the FB with JSON |
| `T_ALY_JsonPayload` | `object.` | — |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** Syntax

##### `T_ALY_SPCleanupCtrl_Cmd` (FUNCTION_BLOCK) — §7.1.1.1.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eAction` | `E_CleanupControlAction [} 168]` | Action to be performed |
| `sStorage` | `GUID` | Individual GUID of the Storage to be used |
| `sRecordDataGuid` | `GUID` | Unique GUID of the Live Recording to be used |
| `sCleanupKey` | `GUID` | GUID “Key” used to identify the |

**Description:** Syntax

##### `T_ALY_SPGetHistorical_Cmd` (FUNCTION_BLOCK) — §7.1.1.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRecordingID` | `LINT` | Specific ID of the recording to be used |
| `sSubBroker` | `GUID` | Individual GUID of the MessageBroker via which the |

**Description:** Syntax

##### `T_ALY_SPReadStreamRecords_Cmd` (FUNCTION_BLOCK) — §7.1.1.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRecordingID` | `LINT` | Specific ID of the recording to be used |
| `sStreamTopic` | `STRING(255)` | Topic name of the recorded live |

**Description:** Syntax

##### `T_ALY_SPRecordData_Cmd` (FUNCTION_BLOCK) — §7.1.1.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRecordDataKey` | `GUID` | Individual GUID to identify the recording |
| `sStorage` | `GUID` | Individual GUID of the Storage to be used |
| `sSubBroker` | `GUID` | Individual GUID of the message broker to be used |
| `sPipeline` | `GUID` | Optional – GUID of the RuleEngine pipeline |
| `sAlias` | `T_MaxString` | Alias name for the recording. |
| `sRecordName` | `T_MaxString` | Name for this record. |
| `eRecording` | `E_RecordMode [} 171]` | Start or Stop the recording. |
| `sRecorder` | `GUID` | Individual GUID of the |

**Description:** Syntax

##### `T_ALY_SPReloadHistoricalStreams_Cmd` (FUNCTION_BLOCK) — §7.1.1.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eReloadType` | `E_ReloadType [} 172]` | Update mode selection |
| `arrParameter` | `ARRAY [0..9] OF ARRAY [0..1]` | Additional parameters OF T_MaxString Methods |
| `Reset` | `Reset all values in the payload FB.` | — |
| `Init_JsonValue` | `Inherited from` | Initialization of the FB with JSON |
| `T_ALY_JsonPayload` | `object.` | — |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** Syntax

##### `T_ALY_SPRuleEnginePipeline_Cmd` (FUNCTION_BLOCK) — §7.1.1.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRuleEnginePipeline` | `GUID` | Individual GUID of the RuleEngine |

**Description:** Syntax

##### `T_ALY_SPSetGetHistoricalDataState_Cmd` (FUNCTION_BLOCK) — §7.1.1.1.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResultTopic` | `T_MaxString` | Topic name of the result stream |

**Description:** Syntax

##### `T_ALY_SPStorageCtrl_Cmd` (FUNCTION_BLOCK) — §7.1.1.1.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eCtrlMode` | `E_ControlMode [} 170]` | Start, stop, etc. of the |

**Description:** Syntax

#### ERROR_SYMBOL_NOT_SELECTED The symbol is not selected. › PLC API › Function blocks › Topic Architecture › Descriptions

##### `T_ALY_HistoricalStream_Desc` (FUNCTION_BLOCK) — §7.1.1.1.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sSource` | `STRING(255)` | Data source name |
| `sStreamTopic` | `STRING(255)` | Topic name of the recorded stream |
| `sStreamAlias` | `STRING(255)` | Alias name of the stream |
| `sStreamSystemID` | `GUID` | SystemID GUID of the stream |
| `sLayout` | `GUID` | Layout GUID of the recording |
| `nCycleTime` | `UDINT` | Cycle time of the recording |
| `nDataSize` | `UDINT` | Data size of an entry of the recording |
| `arrRecords` | `ARRAY [0..cMaxRecordCount] OF` | Timestamp of the various T_RecordTimestamps [} 150] recordings |
| `sStorage` | `GUID` | Individual GUID of the Storage |
| `nHistStreamID` | `LINT` | Specific ID of the historical stream |
| `sSystemAlias` | `STRING(255)` | The system alias of the recorded |

**Description:** Syntax

##### `T_ALY_SPCleanup_Desc` (FUNCTION_BLOCK) — §7.1.1.1.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sCleanupKey` | `GUID` | GUID “Key” for identifying the |

**Description:** Syntax

##### `T_ALY_SPInstance_Desc` (FUNCTION_BLOCK) — §7.1.1.1.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `dtTimestamp` | `DATE_AND_TIME` | Start time of the Storage Provider Service |
| `bOnline` | `BOOL` | Indicates whether the service is |

**Description:** Syntax

##### `T_ALY_SPRecordData_Desc` (FUNCTION_BLOCK) — §7.1.1.1.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sRecordDataGuid` | `GUID` | GUID "Key" to identify the recording |
| `nStartTimestamp` | `LINT` | Start time of the recording. |
| `eStatus` | `E_RecordingState [} 171]` | Recording state |
| `nRecordID` | `LINT` | Recording ID |
| `stRecord` | `T_ALY_SPRecordData_Cmd [} 135]` | Associated recording command Methods |
| `Reset` | `Reset all values in the payload FB.` | — |
| `Init_JsonValue` | `Inherited from` | Initialization of the FB with JSON |
| `T_ALY_JsonPayload` | `object.` | — |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** API | PLC API

##### `T_ALY_SPRecording_Desc` (FUNCTION_BLOCK) — §7.1.1.1.2.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aRecordings` | `ARRAY [0..99] OF` | List of all current recordings T_ALY_SPRecordData_Desc [} 143] Methods |
| `Reset` | `Reset all values in the payload FB.` | — |
| `Init_JsonValue` | `Inherited from` | Initialization of the FB with JSON |
| `T_ALY_JsonPayload` | `object.` | — |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** Syntax

#### ERROR_SYMBOL_NOT_SELECTED The symbol is not selected. › PLC API › Function blocks › Topic Architecture › Info

##### `T_ALY_ReadStreamRecord_Info` (FUNCTION_BLOCK) — §7.1.1.1.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nRecordCountAll` | `UDINT` | Number of all existing records |
| `nRecordCount` | `UDINT` | Number of records read out |
| `arrRecords` | `ARRAY [0..cMaxRecordCount] OF` | Timestamp of the records read |
| `T_RecordTimestamps [} 150]` | `out Methods` | — |
| `Reset` | `Reset all values in the payload FB.` | — |
| `Init_JsonValue` | `Inherited from` | Initialization of the FB with JSON |
| `T_ALY_JsonPayload` | `object.` | — |

**Description:** Syntax

#### ERROR_SYMBOL_NOT_SELECTED The symbol is not selected. › PLC API › Function blocks › Topic Architecture › SubTypes

##### `T_ALY_SPDataStorageInfo` (FUNCTION_BLOCK) — §7.1.1.1.4.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stStorage` | `T_ALY_SPStorageInfo [} 147] Detailed Storage information` | — |
| `eStatus` | `E_StorageState [} 173]` | Status of the Storage |
| `sStatusMessage` | `STRING(255)` | Storage status message Methods |
| `Reset` | `Reset all values in the payload FB.` | — |
| `Init_JsonValue` | `Inherited from` | Initialization of the FB with JSON |
| `T_ALY_JsonPayload` | `object.` | — |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** Syntax

##### `T_ALY_SPInstanceInfo` (FUNCTION_BLOCK) — §7.1.1.1.4.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sProviderGuid` | `GUID` | Individual GUID of a Storage Provider instance |
| `sServiceType` | `STRING` | Service type |
| `sDataStoreType` | `STRING(255)` | Storage type |
| `sComment` | `STRING(255)` | Comment on the Storage Provider instance |
| `sDefaultStorageGuid` | `GUID` | Storage GUID of the Standard Storage |
| `arrDataStorages` | `ARRAY [0..49] OF` | List of configured Storages T_ALY_SPDataStorageInfo [} 146] Methods |
| `Reset` | `Reset all values in the payload FB.` | — |
| `Init_JsonValue` | `Inherited from` | Initialization of the FB with JSON |
| `T_ALY_JsonPayload` | `object.` | — |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** Syntax

##### `T_ALY_SPStorageInfo` (FUNCTION_BLOCK) — §7.1.1.1.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sStorageGuid` | `GUID` | Individual GUID of a Storage |
| `sStorageName` | `STRING` | Name of the Storage |
| `eDataStorageType` | `E_DataStorageType [} 170]` | Storage type |
| `sComment` | `STRING(255)` | Comment on Storage Methods |
| `Reset` | `Reset all values in the payload FB.` | — |
| `Init_JsonValue` | `Inherited from` | Initialization of the FB with JSON |
| `T_ALY_JsonPayload` | `object.` | — |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** Syntax

##### `T_ALY_SPSubBrokerInfo` (FUNCTION_BLOCK) — §7.1.1.1.4.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sAlias` | `STRING(255)` | Alias name of the broker configuration |
| `sBrokerGuid` | `GUID` | Individual GUID of the broker configuration |
| `sBrokerHost` | `STRING(255)` | Broker Host Name |
| `nBrokerPort` | `DINT` | Broker port |
| `bSecure` | `BOOL` | TRUE if communication is established via |

**Description:** Syntax

##### `T_ALY_Symbol` (FUNCTION_BLOCK) — §7.1.1.1.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sName` | `T_MaxString` | Name of the symbol |
| `sBaseType` | `T_MaxString` | DataType of the symbol |
| `nBitOffset` | `UDINT` | BitOffset of the symbol |
| `nBitSize` | `UDINT` | BitSize of the symbol Methods |
| `Reset` | `Reset all values in the payload FB.` | — |
| `Init_JsonValue` | `Inherited from` | Initialization of the FB with JSON |
| `T_ALY_JsonPayload` | `object.` | — |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** Syntax

##### `T_RecordTimestamps` (FUNCTION_BLOCK) — §7.1.1.1.4.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nRecordID` | `DINT` | Recording number |
| `sAlias` | `STRING(255)` | Alias name of the recording |
| `nStartTimestamp` | `LINT` | Start timestamp of the recording |
| `nEndTimestamp` | `LINT` | End timestamp of the recording Methods |
| `Reset` | `Reset all values in the payload FB.` | — |
| `Init_JsonValue` | `Inherited from` | Initialization of the FB with JSON |
| `T_ALY_JsonPayload` | `object.` | — |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** Syntax

#### ERROR_SYMBOL_NOT_SELECTED The symbol is not selected. › PLC API › Function blocks

##### `FB_ALY_StorageProvider` (FUNCTION_BLOCK) — §7.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_SP_Config [} 166]` | Structure for the configuration of the FB. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block is |

**Description:** The FB_ALY_StorageProvider is a client FB for communication with a Storage Provider instance. The FB provides methods to trigger historical data or start/stop recordings.

#### ERROR_SYMBOL_NOT_SELECTED The symbol is not selected. › PLC API › Function blocks › FB_ALY_StorageProvider

##### `Call` (METHOD) → `BOOL` — §7.1.1.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | — |

**Description:** Syntax

##### `Cancel` (METHOD) → `BOOL` — §7.1.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to cancel |
| `T_ALY_SPCancel_Cmd [} 130]` | `operations of the TwinCAT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Cancel` | `BOOL` | Is TRUE if done |

**Description:** Syntax

##### `CleanupControlling` (METHOD) → `BOOL` — §7.1.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command for the Cleanup |
| `T_ALY_SPCleanupCtrl_Cmd [} 131]` | `Services` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CleanupControlling` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `DeleteRecording` (METHOD) → `BOOL` — §7.1.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSourceTopic` | `T_MaxString` | Topic name of the recorded live stream |
| `sLayout` | `GUID` | Layout GUID of the recording |
| `sSourceSystemID` | `GUID` | SystemID-GUID of the recording |
| `sStorageGuid` | `GUID` | Individual GUID of the Storage to be used |
| `sAlias` | `T_MaxString` | Alias name of the recording |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DeleteRecording` | `BOOL` | Is TRUE when completed |

**Description:** API | PLC API

##### `DeleteRecordingByID` (METHOD) → `BOOL` — §7.1.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRecordingID` | `LINT` | Specific ID of the recording to be used |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DeleteRecordingByID` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `DeleteRecords` (METHOD) → `BOOL` — §7.1.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aRecordIDs` | `ARRAY [*] OF LINT` | Specific IDs of the records to be deleted |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DeleteRecords` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `GetCleanupInfoByKey` (METHOD) → `BOOL` — §7.1.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sCleanupKey` | `GUID` | Search criterion “sCleanupKey” |
| `sStorageGuid` | `GUID` | Search criterion “Storage” |
| `tTimeout` | `TIME` | Duration until the procedure is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetCleanupInfoByKey` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `GetHistoricalData` (METHOD) → `BOOL` — §7.1.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to get historical |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetHistoricalData` | `BOOL` | Is TRUE if done |

**Description:** API | PLC API

##### `GetInstanceInfo` (METHOD) → `BOOL` — §7.1.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeout` | `TIME` | Duration until the procedure is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetInstanceInfo` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `GetRecordingInfoByAlias` (METHOD) → `BOOL` — §7.1.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRecordingAlias` | `STRING(255)` | Search criterion "Alias" |
| `tTimeout` | `TIME` | Duration until the procedure is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetRecordingInfoByAlias` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `GetRecordingInfoByKey` (METHOD) → `BOOL` — §7.1.1.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRecordDataKey` | `GUID` | Search criterion "RecordDataKey" |
| `tTimeout` | `TIME` | Duration until the procedure is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetRecordingInfoByKey` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `ReadHistoricalStreams` (METHOD) → `BOOL` — §7.1.1.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tSearchDuration` | `TIME` | Time period in which to wait for |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReadHistoricalStreams` | `BOOL` | Is TRUE when completed |
| `nStreamCount` | `INT` | Number of streams read out |

**Description:** Syntax

##### `ReadStreamRecords` (METHOD) → `BOOL` — §7.1.1.2.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to get recordings |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReadStreamRecords` | `BOOL` | Is TRUE when completed |
| `nRecordCount` | `DINT` | Number of records read out |

**Description:** Syntax

##### `ReadSubBroker` (METHOD) → `BOOL` — §7.1.1.2.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tSearchDuration` | `TIME` | Duration until the search is completed |
| `aSubBrokerInfo` | `POINTER TO` | Address to an array in which the |
| `T_ALY_SPSubBrokerInfo [} 148]` | `broker information found is stored.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReadSubBroker` | `BOOL` | Is TRUE when completed. |
| `nBrokerCount` | `INT` | Number of brokers found. |

**Description:** Syntax

##### `ResetCommunication` (METHOD) → `BOOL` — §7.1.1.2.15

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ResetCommunication` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `RestartPipelineRule` (METHOD) → `BOOL` — §7.1.1.2.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to start the |
| `T_ALY_SPRuleEnginePipeline_Cmd` | `recording a live stream. [} 137]` | — |
| `nRuleId` | `DINT` | ID of the rule to be restarted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RestartPipelineRule` | `BOOL` | Is TRUE when completed. |

**Description:** API | PLC API

##### `SendCommand` (METHOD) → `BOOL` — §7.1.1.2.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `I_ALY_SPCommand` | JSON command to interact with the TwinCAT Analytics Storage |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendCommand` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `SetHistoricalDataState` (METHOD) → `BOOL` — §7.1.1.2.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to set parameters |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetHistoricalDataState` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `StartPipeline` (METHOD) → `BOOL` — §7.1.1.2.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to start the |
| `T_ALY_SPRuleEnginePipeline_Cmd` | `recording a live stream. [} 137]` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartPipeline` | `BOOL` | Is TRUE when completed. |

**Description:** Syntax

##### `StartRecord` (METHOD) → `BOOL` — §7.1.1.2.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to start the |
| `T_ALY_SPRecordData_Cmd [} 135]` | `recording a live stream.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartRecord` | `BOOL` | Is TRUE if done |

**Description:** Syntax

##### `StartRecordEx` (METHOD) → `BOOL` — §7.1.1.2.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to start the |
| `T_ALY_SPRecordData_Cmd [} 135]` | `recording a live stream.` | — |
| `sRecordDataKey` | `GUID` | Guid "Key" to identify the recording |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartRecordEx` | `BOOL` | Is TRUE when completed |

**Description:** In contrast to the StartRecord [} 161] method, a RecordDataKey can be specified here. This key makes it easier to find the recording you have started in order to check the status of the recording. The GetRecordingInfoByKey [} 157] method can be used to retrieve the recording information.

##### `StopPipeline` (METHOD) → `BOOL` — §7.1.1.2.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to stop recording |
| `T_ALY_SPRuleEnginePipeline_Cmd` | `of a live stream. [} 137]` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StopPipeline` | `BOOL` | Is TRUE when completed. |

**Description:** Syntax

##### `StopRecord` (METHOD) → `BOOL` — §7.1.1.2.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to stop recording |
| `T_ALY_SPRecordData_Cmd [} 135]` | `of a live stream.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StopRecord` | `BOOL` | Is TRUE when completed. |

**Description:** Syntax

##### `StorageControlling` (METHOD) → `BOOL` — §7.1.1.2.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `REFERENCE TO` | JSON command to control the T_ALY_SPStorageCtrl_Cmd [} 139] storages. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StorageControlling` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

#### ERROR_SYMBOL_NOT_SELECTED The symbol is not selected. › PLC API › Function blocks

##### `FB_ALY_ActiveRecordings` (FUNCTION_BLOCK) — §7.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_SP_Config [} 166]` | Structure for the configuration of the FB. Inputs/outputs |
| `arrRecordings` | `ARRAY [*] OF` | List of active recordings found ST_RecordingStatus [} 167] |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrRecordings` | `ARRAY [*] OF ST_RecordingStatus` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE when an error situation occurs. |
| `ipResultMessage` | `I_TcMessage` | Message interface of the TwinCAT 3 EventLogger, |

**Description:** The FB_ALY_ActiveRecordings is a client FB for monitoring the active recordings of a Storage Provider instance. The FB provides methods to start and stop monitoring. The active recordings found are output at arrRecordings with the corresponding status information.

#### ERROR_SYMBOL_NOT_SELECTED The symbol is not selected. › PLC API › Function blocks › FB_ALY_ActiveRecordings

##### `Call` (METHOD) → `BOOL` — §7.1.1.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | — |

**Description:** Syntax

##### `StartMonitoring` (METHOD) → `BOOL` — §7.1.1.3.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartMonitoring` | `BOOL` | Is TRUE when completed |

**Description:** Syntax

##### `StopMonitoring` (METHOD) → `BOOL` — §7.1.1.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StopMonitoring` | `BOOL` | Is TRUE when completed. |

**Description:** Syntax
