# Tc3_AnalyticsCommunication

- **Manual:** `TF3500_TC3_Analytics_Logger_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Measurement · **Tier:** T4 · **API entries:** 20
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Analytics transport needs the analytics engine plus MQTT/HTTP egress.

---

#### PLC › Analytics Communication Library › PLC API › Function blocks

##### `FB_ALYC_MqttStream` (FUNCTION_BLOCK) — §7.1.1.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nObjectID` | `OTCID` | TcCom-Object ID of the referenced stream. This can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialized` | `BOOL` | TRUE if the function block is initialized and can be used. |

**Description:** This function block represents an Analytics MQTT stream. The connection to a stream can be established using the ObjectId as an input variable. The requirement is an existing stream, e.g. created in the System Manager under the Data Logger node in the project tree. The stream can be controlled using the Start/Stop methods and reconfigured using Reconfigure. A structure of the type ST_ALYC_MqttStreamConfig containing the new configuration parameters is transferred to the method for this purpose. During reconfiguration, starting from the OP state, the TcCom states SAFEOP, PREOP, SAFEOP, OP are run through in this order. Since all states below SAFEOP no longer run in real-time mode, but the remaining TwinCAT runtime does, the reconfiguration must take place asynchronously to the task cycle, whereby the Reconfig method should be called cyclically as long as the OP state is not reached again. The properties bConnected, bStarted etc. provide information about the current state of the stream. Errors can be recorded via the bError output and the corresponding ipResultMessage.

##### `FB_ALYC_FileStream` (FUNCTION_BLOCK) — §7.1.1.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nObjectID` | `OTCID` | TcCom-Object ID of the referenced stream. This can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialized` | `BOOL` | TRUE if the function block is initialized and can be used. |

**Description:** This function block represents an Analytics Stream in file mode. The connection to a stream can be established using the ObjectId as an input variable. The requirement is an existing stream that is in file mode, e.g. created in the System Manager under the Data Logger node in the project tree. The symbols that are to be logged must also be configured via the System Manager.

##### `FB_ALYC_MqttStreamHelper` (FUNCTION_BLOCK) — §7.1.1.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nObjectID` | `OTCID` | TcCom-Object ID of the referenced StreamHelper. This |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialized` | `BOOL` | TRUE if the function block is initialized and can be used. |

**Description:** This function block represents an Analytics Stream Helper in MQTT mode. The connection to an existing Stream Helper can be established via the ObjectId as an input variable. This must be configured in MQTT mode. The stream can be controlled using the Start/Stop methods and reconfigured using Reconfigure. For this purpose, a structure of the type ST_ALYC_MqttStreamHelperConfig is transferred to the method, which contains the new configuration parameters. During reconfiguration, starting from the OP state, the TcCom states SAFEOP, PREOP, SAFEOP, OP are run through in this order. Since all states below SAFEOP no longer run in real-time mode, but the remaining TwinCAT runtime does, the reconfiguration must take place asynchronously to the task cycle, whereby the Reconfigure method should be called cyclically as long as the OP state is not reached again. The properties bConnected, bStarted etc. provide information about the current state of the Stream Helper. Errors can be recorded via the bError output and the corresponding ipResultMessage.

##### `FB_ALYC_FileStreamHelper` (FUNCTION_BLOCK) — §7.1.1.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nObjectID` | `OTCID` | TcCom-Object ID of the referenced StreamHelper. This |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialized` | `BOOL` | TRUE if the function block is initialized and can be used. |

**Description:** This function block represents an Analytics Stream Helper in file mode. The connection to an existing Stream Helper can be established via the ObjectId as an input variable. This must be configured in file mode. The stream can be controlled using the Start/Stop methods and reconfigured using Reconfigure. For this purpose, a structure of the type ST_ALYC_FileStreamHelperConfig is transferred to the method, which contains the new configuration parameters. During reconfiguration, starting from the OP state, the TcCom states SAFEOP, PREOP, SAFEOP, OP are run through in this order. Since all states below SAFEOP no longer run in real-time mode, but the remaining TwinCAT runtime does, the reconfiguration must take place asynchronously to the task cycle, whereby the Reconfigure method should be called cyclically as long as the OP state is not reached again. The properties bStarted etc. provide information about the current state of the Stream Helper. Errors can be recorded via the bError output and the corresponding ipResultMessage.

#### PLC › Analytics Communication Library › PLC API › Function blocks › IoT symbol

##### `FB_ALYC_IotSymbol_BOOL` (FUNCTION_BLOCK) — §7.1.1.3.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_BYTE` (FUNCTION_BLOCK) — §7.1.1.3.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_DINT` (FUNCTION_BLOCK) — §7.1.1.3.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_DWORD` (FUNCTION_BLOCK) — §7.1.1.3.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_INT` (FUNCTION_BLOCK) — §7.1.1.3.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_LINT` (FUNCTION_BLOCK) — §7.1.1.3.1.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_LREAL` (FUNCTION_BLOCK) — §7.1.1.3.1.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_LWORD` (FUNCTION_BLOCK) — §7.1.1.3.1.5.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_REAL` (FUNCTION_BLOCK) — §7.1.1.3.1.5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_SINT` (FUNCTION_BLOCK) — §7.1.1.3.1.5.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_STRING` (FUNCTION_BLOCK) — §7.1.1.3.1.5.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_UDINT` (FUNCTION_BLOCK) — §7.1.1.3.1.5.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_UINT` (FUNCTION_BLOCK) — §7.1.1.3.1.5.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_ULINT` (FUNCTION_BLOCK) — §7.1.1.3.1.5.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_USINT` (FUNCTION_BLOCK) — §7.1.1.3.1.5.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax

##### `FB_ALYC_IotSymbol_WORD` (FUNCTION_BLOCK) — §7.1.1.3.1.5.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALYC_IotSymbol_Co Structure for the configuration of the FB. nfig` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | EventLogger |
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated. |
| `bConfigured` | `BOOL` | TRUE if the FB is successfully configured. |
| `bSymbolHandlerAssi BOOL` | `TRUE if the symbol handler has been assigned. gned` | — |
| `bVariableFound` | `BOOL` | TRUE if a variable was found in the stream. |
| `sSymbolPath` | `STRING(255)` | Theam/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of the publishing system. |
| `nMaxNumElements UDINT` | `Maximum number of saved symbols influenced by` | — |

**Description:** Syntax
