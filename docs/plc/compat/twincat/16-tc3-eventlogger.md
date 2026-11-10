# Tc3_EventLogger

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_EventLogger_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T1 · **API entries:** 146
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Event service exists in TetherPLC (raise/query); needs severity/category model parity.

---

#### Functions and function blocks › Asynchronous text requests › FB_AsyncStrResult

##### `GetString` (METHOD) → `BOOL` — §3.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `REFERENCE TO STRING` | Buffer variable for the requested text |
| `nResult` | `UDINT` | Buffer size in bytes Functions and function blocks |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetString` | `BOOL` | Returns TRUE if the text could be assigned. |

**Description:** As soon as bBusy is FALSE and provided no error has occurred (bError = FALSE), the requested text can be fetched with this method.

#### Functions and function blocks › Asynchronous text requests › FB_RequestCauseRemedy

##### `Clear` (METHOD) — §3.1.2.1

**Description:** This method clears the last result queried.

##### `Get` (METHOD) → `HRESULT` — §3.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `UDINT` | Index in the list of cause/remedy elements to be queried. |
| `fbCauseRemedy Reference to` | `Reference to a FB_TcCauseRemedy in which the texts FB_TcCauseRemedy [} 29]` | are stored. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Get` | `HRESULT` | S_OK on success |

**Description:** As soon as bBusy is FALSE and if no error has occurred (bError = FALSE), this method can be used to get a result from the queried list.

##### `Request` (METHOD) → `HRESULT` — §3.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | Specifies the event class. |
| `nEventId` | `UDINT` | ID of the event |
| `nLangId` | `DINT …` | Specifies the language ID: English (en-US) = 1033 German (de-DE) = 1031 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Request` | `HRESULT` | Returns possible error information. |

**Description:** Calling this method triggers the asynchronous request.

##### `RequestRemote` (METHOD) → `HRESULT` — §3.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipRemoteLogger I_TcRemoteEventLogger` | `Instance of I_TcRemoteEventLogger, which describes` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestRemote` | `HRESULT` | Returns possible error information. |

**Description:** Calling this method triggers the asynchronous request on a remote target system.

#### Functions and function blocks › Asynchronous text requests › FB_RequestEventClassDetails

##### `Clear` (METHOD) — §3.1.3.1

**Description:** This method clears the last result queried.

##### `Get` (METHOD) → `HRESULT` — §3.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `UDINT` | Index in the list of cause/remedy elements to be queried. |
| `fbDetail` | `Reference to FB_TcDetail` | Reference to an FB_TcDetail in which the details are [} 31] stored. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Get` | `HRESULT` | Returns S_OK if successful. |

**Description:** As soon as bBusy is FALSE and if no error has occurred (bError = FALSE), this method can be used to get a result from the queried list.

##### `Request` (METHOD) → `HRESULT` — §3.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | Specifies the event class. |
| `nLangId` | `DINT …` | Specifies the language ID: English (en-US) = 1033 German (de-DE) = 1031 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Request` | `HRESULT` | Returns possible error information. |

**Description:** Calling this method triggers the asynchronous request.

##### `RequestRemote` (METHOD) → `HRESULT` — §3.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipRemoteLogger I_TcRemoteEventLogger` | `Instance of I_TcRemoteEventLogger, which describes the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestRemote` | `HRESULT` | Returns possible error information. |

**Description:** Calling this method triggers the asynchronous request on a remote target system.

#### Functions and function blocks › Asynchronous text requests › FB_RequestEventClassName

##### `Clear` (METHOD) — §3.1.4.1

**Description:** This method clears the last result queried.

##### `GetString` (METHOD) → `BOOL` — §3.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `REFERENCE TO STRING` | Buffer variable for the requested text |
| `nResult` | `UDINT` | Buffer size in bytes |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetString` | `BOOL` | Returns TRUE if the text could be assigned. |

**Description:** As soon as bBusy is FALSE and provided no error has occurred (bError = FALSE), the requested text can be fetched with this method.

##### `Request` (METHOD) → `HRESULT` — §3.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | GUID of the event class. |
| `nLangId` | `DINT` | Specifies the language ID English (en-US) = 1033 German (de-DE) = 1031 Functions and function blocks |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Request` | `HRESULT` | Returns possible error information. |

**Description:** Calling this method triggers the asynchronous text request.

##### `RequestRemote` (METHOD) → `HRESULT` — §3.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipRemoteLogger I_TcRemoteEventLogger` | `Instance of I_TcRemoteEventLogger, which describes the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestRemote` | `HRESULT` | Returns possible error information. |

**Description:** Calling this method triggers the asynchronous request on a remote target system.

#### Functions and function blocks › Asynchronous text requests › FB_RequestEventDetails

##### `Clear` (METHOD) — §3.1.5.1

**Description:** This method clears the last result queried.

##### `Get` (METHOD) → `HRESULT` — §3.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `UDINT` | Index in the list of causes/remedy elements to be queried. |
| `fbDetail` | `Reference to FB_TcDetail` | Reference to an FB_TcDetail in which the details are [} 31] stored. Functions and function blocks |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Get` | `HRESULT` | S_OK on success |

**Description:** As soon as bBusy is FALSE and if no error has occurred (bError = FALSE), this method can be used to get a result from the queried list.

##### `Request` (METHOD) → `HRESULT` — §3.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | Specifies the event class. |
| `nEventId` | `UDINT` | ID of the event |
| `nLangId` | `DINT …` | Specifies the language ID: English (en-US) = 1033 German (de-DE) = 1031 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Request` | `HRESULT` | Returns possible error information. |

**Description:** Calling this method triggers the asynchronous request.

##### `RequestRemote` (METHOD) → `HRESULT` — §3.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipRemoteLogger I_TcRemoteEventLogger` | `Instance of I_TcRemoteEventLogger, which describes` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestRemote` | `HRESULT` | Returns possible error information. |

**Description:** 22

#### Functions and function blocks › Asynchronous text requests › FB_RequestEventText

##### `GetString` (METHOD) → `BOOL` — §3.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `REFERENCE TO STRING` | Buffer variable for the requested text |
| `nResult` | `UDINT` | Buffer size in bytes |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetString` | `BOOL` | Returns TRUE if the text could be assigned. |

**Description:** As soon as bBusy is FALSE and provided no error has occurred (bError = FALSE), the requested text can be fetched with this method.

##### `Request` (METHOD) → `BOOL` — §3.1.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | Specifies the event class. |
| `nEventId` | `UDINT` | ID of the event. |
| `nLangId` | `DINT` | Specifies the language ID: English (en-US) = 1033 German (de-DE) = 1031 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Request` | `HRESULT` | Returns possible error information. |

**Description:** 24

##### `RequestRemote` (METHOD) → `HRESULT` — §3.1.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipRemoteLogger` | `I_TcRemoteEventLogger` | Instance of I_TcRemoteEventLogger, which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestRe HRESULT mote Also see about this` | `` | — |

**Description:** Calling this method triggers the asynchronous request on a remote target system.

##### `Clear` (METHOD) — §3.1.6.4

**Description:** This method clears the last result queried.

#### Functions and function blocks › Asynchronous text requests › FB_RequestTranslation

##### `GetString` (METHOD) → `BOOL` — §3.1.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `REFERENCE TO STRING` | Buffer variable for the requested text |
| `nResult` | `UDINT` | Buffer size in bytes |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetString` | `BOOL` | Returns TRUE if the text could be assigned. |

**Description:** As soon as bBusy is FALSE and provided no error has occurred (bError = FALSE), the requested text can be fetched with this method.

##### `Request` (METHOD) → `BOOL` — §3.1.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | Specifies the event class to be used as the basis for |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `text` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Request` | `HRESULT` | Returns possible error information. |

**Description:** Functions and function blocks

##### `RequestRemote` (METHOD) → `HRESULT` — §3.1.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipRemoteLogger` | `I_TcRemoteEventLogger` | Instance of I_TcRemoteEventLogger, which |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `text` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestRe HRESULT` | `Returns possible error information.` | — |
| `mote` | `S_OK if the query is successful.` | — |

**Description:** Calling this method triggers the asynchronous request on a remote target system.

##### `Clear` (METHOD) — §3.1.7.4

**Description:** This method clears the last result queried.

#### Functions and function blocks › Asynchronous text requests › FB_TcCauseRemedy

##### `GetCause` (METHOD) → `BOOL` — §3.1.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `REFERENCE TO STRING` | Reference to a string in which the result is to be stored. |
| `nResult` | `UDINT` | Length of the sResult |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetCause` | `BOOL` | Returns the success whether a cause was provided. |

**Description:** This method returns the cause.

##### `GetId` (METHOD) → `BOOL` — §3.1.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `REFERENCE TO STRING` | Reference to a string in which the result is to be stored. |
| `nResult` | `UDINT` | Length of the sResult |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetId` | `BOOL 30` | Returns the success whether an ID was provided. Functions and function blocks |

**Description:** This method returns the index of the cause/remedy.

##### `GetRemedy` (METHOD) → `BOOL` — §3.1.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `REFERENCE TO STRING` | Reference to a string in which the result is to be stored. |
| `nResult` | `UDINT` | Length of the sResult |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetRemedy` | `BOOL` | Returns the success of whether a remedy has been |

**Description:** This method returns the remedy.

##### `Release` (METHOD) → `HRESULT` — §3.1.8.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Release` | `HRESULT` | Returns whether the memory has been cleaned. |

**Description:** This method clears the internal memory of the function block.

#### Functions and function blocks › Asynchronous text requests › FB_TcDetail

##### `Release` (METHOD) → `HRESULT` — §3.1.9.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Release` | `HRESULT` | Returns whether the memory has been cleaned. |

**Description:** This method clears the internal memory of the function block.

##### `GetComment` (METHOD) → `BOOL` — §3.1.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `REFERENCE TO STRING` | Reference to a string in which the result is to be stored. |
| `nResult` | `UDINT` | Length of the sResult |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetCommend` | `BOOL 32` | Returns the success whether a comment was provided. Functions and function blocks |

**Description:** This method returns the comment.

##### `GetName` (METHOD) → `BOOL` — §3.1.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `REFERENCE TO STRING` | Reference to a string in which the result is to be stored. |
| `nResult` | `UDINT` | Length of the sResult |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetName` | `BOOL` | Returns the success whether a name was provided. |

**Description:** This method returns the name.

##### `GetText` (METHOD) → `BOOL` — §3.1.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `REFERENCE TO STRING` | Reference to a string in which the result is to be stored. |
| `nResult` | `UDINT` | Length of the sResult |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetText` | `BOOL` | Returns the success whether a text was provided. Functions and function blocks |

**Description:** This method returns the text.

#### Functions and function blocks › Asynchronous text requests

##### `F_GetEventClassName` (FUNCTION) → `HRESULT` — §3.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLangId` | `DINT …` | Specifies the language ID English (en-US) = 1033 German (de-DE) = 1031 |
| `fbEventBase` | `REFERENCE TO` | Specification of an event/alarm/message object. FB_TcEventBase [} 71] |
| `/` | `Inputs/outputs` | — |
| `fbResult` | `FB_AsyncStrResult [} 11]` | Specification of a function block instance in order to track an |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fbResult` | `FB_AsyncStrResult` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GetEventClassName` | `HRESULT` | Returns possible error information. |

**Description:** The function triggers the asynchronous request for the name of an event class.

##### `F_GetEventText` (FUNCTION) → `HRESULT` — §3.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLangId` | `DINT …` | Specifies the language ID English (en-US) = 1033 German (de-DE) = 1031 |
| `fbEventBase` | `REFERENCE TO` | Specification of an event/alarm/message object. FB_TcEventBase [} 71] |
| `/` | `Inputs/outputs` | — |
| `fbResult` | `FB_AsyncStrResult [} 11]` | Specification of a function block instance in order to track an |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fbResult` | `FB_AsyncStrResult` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GetEventText` | `HRESULT` | Returns possible error information. |

**Description:** The function triggers the asynchronous request for an event text.

#### Functions and function blocks › EventEntry conversion

##### `AdsErr_TO_TcEventEntry` (FUNCTION) → `BOOL` — §3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eErrorId` | `E_AdsErr` | Error code to be converted. |
| `stEventEntry` | `REFERENCE TO TcEventEntry [} 112]` | Outputs the resulting event definition. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AdsErr_TO_TcEventEntry` | `BOOL` | Returns TRUE if the conversion was |

**Description:** This function converts a standard ADS error into a TcEventEntry.

##### `HRESULTAdsErr_TO_TcEventEntry` (FUNCTION) → `BOOL` — §3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hr` | `E_HRESULTAdsErr` | Error code to be converted. |
| `stEventEntry` | `REFERENCE TO TcEventEntry [} 112]` | Outputs the resulting event definition. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULTAdsErr_TO_TcEventEntry` | `BOOL` | Returns TRUE if the conversion was carried out |

**Description:** This function converts a standard ADS error (HRESULT) into a TcEventEntry.

##### `TcEventEntry_TO_AdsErr` (FUNCTION) → `BOOL` — §3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry [} 112]` | Event definition to be converted. |
| `eErrorId` | `REFERENCE TO E_AdsErr` | Outputs the resulting error code. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcEventEntry_TO_AdsErr` | `BOOL` | Returns TRUE if the conversion was carried out successfully and |

**Description:** This function converts a TcEventEntry into a standard ADS error.

##### `TcEventEntry_TO_HRESULTAdsErr` (FUNCTION) → `BOOL` — §3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry [} 112]` | Event definition to be converted. |
| `hr` | `REFERENCE TO` | Outputs the resulting error code. E_HRESULTAdsErr Functions and function blocks |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcEventEntry_TO_HRESULTAdsErr` | `BOOL` | Returns TRUE if the conversion was carried out successfully and FALSE if the event class is |

**Description:** This function converts a TcEventEntry into a standard ADS error (HRESULT).

#### Functions and function blocks › Filter

##### `FB_TcClearLoggedEventsSettings` (FUNCTION_BLOCK) — §3.3.1

**Description:** This function block provides the functionality to specify which events are to be removed from the cache.

#### Functions and function blocks › Filter › FB_TcClearLoggedEventsSettings

##### `AddFilter` — §3.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipEventFilter` | `I_TcEventFilter` | Instance of the filter to be used Return values |
| `AddFilter` | `HRESULT` | Returns S_OK if successful. |

**Description:** Method for adding a filter.

##### `SetLimit` — §3.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eType` | `TcEventLimitType` | Determines the reference whether the limit applies to the first or last |

**Description:** Indicates the number of events to be cleared. The limit is applied after sorting and filtering.

##### `SetSorting` — §3.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eField` | `TcEventField` | Property to be used for sorting. |
| `eOrder` | `TcEventSortOrder` | Defines the sort order. Return values |
| `SetSorting` | `HRESULT` | Returns S_OK if successful. |

**Description:** Sets the sort order for the query.

#### Functions and function blocks › Filter

##### `FB_TcEventCsvExportSettings` (FUNCTION_BLOCK) — §3.3.2

**Description:** Provides the functionality to specify the csv export.

##### `FB_TcEventFilter` (FUNCTION_BLOCK) — §3.3.3

**Description:** Provides the functionality to specify an event filter.

#### Functions and function blocks › Filter › FB_TcEventFilter

##### `FilterExpression` — §3.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipExpression` | `I_TcEventFilterBase 42` | Link point Functions and function blocks Return values |
| `FilterExpression` | `I_TcLogicalExpression` | Link point |

**Description:** The method specifies a subordinate filter definition.

#### Functions and function blocks › RemoteEventLogger

##### `FB_RemoteListenerBase` (FUNCTION_BLOCK) — §3.4.1

**Description:** The function block serves as the basic implementation of an event listener of a remote system. New messages and state changes of alarms can be recognized through the overwriting of the event-driven methods.

#### Functions and function blocks › RemoteEventLogger › FB_RemoteListenerBase

##### `Execute` (METHOD) → `HRESULT` — §3.4.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `HRESULT` | Returns S_OK if the method call was successful, |

**Description:** The method must be called cyclically so that the event queue can be processed.

##### `OnAlarmCleared` (METHOD) → `HRESULT` — §3.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipContext` | `I_TcListenerContext` | Context of the different EventLoggers |
| `fbEvent` | `REFERENCE TO` | Reference to the alarm that has occurred. This reference |
| `FB_TcEvent` | `must not be copied, e.g. by assignment.` | — |

**Description:** This method is called if the state of an alarm changes from Raised to Clear.

##### `OnAlarmConfirmed` (METHOD) → `HRESULT` — §3.4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipContext` | `I_TcListenerContext` | Context of the different EventLoggers |
| `fbEvent` | `REFERENCE TO` | Reference to the alarm that has occurred. This reference |
| `FB_TcEvent` | `must not be copied, e.g. by assignment.` | — |

**Description:** This method is called when an alarm has been confirmed.

##### `OnAlarmDisposed` (METHOD) → `HRESULT` — §3.4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipContext` | `I_TcListenerContext` | Context of the different EventLoggers |
| `fbEvent` | `REFERENCE TO` | Reference to the alarm that has occurred. This reference |
| `FB_TcEvent` | `must not be copied, e.g. by assignment.` | — |

**Description:** This method is called when an alarm instance has been released again.

##### `OnAlarmRaised` (METHOD) → `HRESULT` — §3.4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipContext` | `I_TcListenerContext` | Context of the different EventLoggers |
| `fbEvent` | `REFERENCE TO` | Reference to the alarm that has occurred. This reference |
| `FB_TcEvent` | `must not be copied, e.g. by assignment.` | — |

**Description:** This method is called if the state of an alarm changes from Clear to Raised.

##### `OnConnectionStateChanged` (METHOD) → `HRESULT` — §3.4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipContext` | `I_TcListenerContext` | Context of the different EventLoggers |
| `eReason` | `TcRemoteConnectionChange The type of change in connection status. Reason` | — |

**Description:** 46

##### `OnDatabaseChanged` (METHOD) → `HRESULT` — §3.4.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipContext` | `I_TcListenerContext` | Context of the different EventLoggers |
| `eReason` | `TcDatabaseChangeReason` | One of the values of the ENUM: Undefined, CleardByCommand, MaximumSizeReached, Deleted |

**Description:** This method is called if the database has changed, e.g. if a ClearLoggedEvents() has been called.

##### `OnEventLoggerError` (METHOD) → `HRESULT` — §3.4.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipContext` | `I_TcListenerContext` | Context of the different EventLoggers |
| `hr` | `HRESULT` | The error code that has occurred. |

**Description:** This method is called if an ADS error occurs during communication.

##### `OnMessageSent` (METHOD) → `HRESULT` — §3.4.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipContext` | `I_TcListenerContext` | Context of the different EventLoggers |
| `fbEvent` | `REFERENCE TO` | Reference to the alarm that has occurred. This reference |
| `FB_TcEvent` | `must not be copied, e.g. by assignment.` | — |

**Description:** This method is called when a message has been sent.

##### `Subscribe` (METHOD) → `HRESULT` — §3.4.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipRemoteLogger I_TcRemoteEventLogger` | `Reference to the EventLogger for which a notification is to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Subscribe` | `HRESULT` | Returns S_OK if the method call was successful. Returns ADS_E_EXISTS if the listener is already |

**Description:** The method subscribes notifications.

##### `Unsubscribe` (METHOD) → `HRESULT` — §3.4.1.11

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Unsubscribe` | `HRESULT` | Returns S_OK if the method call was successful. Returns ADS_E_NOTFOUND if the listener was not |

**Description:** The listener is unsubcribed with this method.

#### Functions and function blocks › RemoteEventLogger

##### `FB_TcRemoteEventLogger` (FUNCTION_BLOCK) — §3.4.2

**Description:** This function block represents the TwinCAT 3 EventLogger for a remote system.

#### Functions and function blocks › RemoteEventLogger › FB_TcRemoteEventLogger

##### `ClearAlarms` (METHOD) → `HRESULT` — §3.4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | Set to 0 to obtain the current time automatically. Initial: 0 |
| `bResetConfirmati BOOL` | `If TRUE and the confirmation status is` | — |
| `on` | `WaitForConfirmation, the confirmation status is set to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ClearAlarms` | `HRESULT` | Returns S_OK if the method call was successful, |

**Description:** Method for clearing active alarms. Returns S_OK if successful.

##### `ClearLoggedEvents` (METHOD) → `HRESULT` — §3.4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipClearSettings` | `I_TcClearLoggedEventsSettin Optional` | — |
| `gs` | `(otherwise the entire cache is emptied)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ClearLoggedEve HRESULT` | `Returns S_PENDING as long as the request has not yet` | — |
| `nts` | `been completed. Returns S_OK if the method call was` | — |

**Description:** Async method for clearing logged events. Returns TRUE if the asynchronous request is no longer assigned.

##### `ConfirmAlarms` (METHOD) → `HRESULT` — §3.4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | Set to 0 to obtain the current time automatically. Initial: 0 |
| `ipFilter` | `I_TcEventFilter` | Specify which alarms are to be confirmed, otherwise all |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfirmAlarms` | `HRESULT` | Returns S_OK if the method call was successful, |

**Description:** The method confirms alarms.

##### `Connect` (METHOD) → `HRESULT` — §3.4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | AmsNetId of the remote system |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Connect` | `HRESULT` | Returns S_OK if the method call was successful, |

**Description:** The method connects to the remote system.

##### `Disconnect` (METHOD) → `HRESULT` — §3.4.2.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Disconnect` | `HRESULT` | Returns S_OK if the method call was successful, |

**Description:** The method terminates the connection with the remote system.

##### `SendMessage` (METHOD) → `HRESULT` — §3.4.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | GUID of the event class |
| `nEventId` | `UDINT` | ID of the event |
| `eSeverity` | `TcEventSeverity` | Severity of the event |
| `ipSourceInfo` | `I_TcSourceInfo` | Interface pointer to the source information. |
| `eventClass` | `GUID` | GUID of the event class |
| `nEventId` | `UDINT` | ID of the event. |
| `eSeverity` | `TcEventSeverity` | Severity of the event |
| `ipSourceInfo` | `I_TcSourceInfo` | Interface pointer to the source information. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendMessage` | `HRESULT` | Returns S_OK if the method call was successful, |
| `SendMessage2` | `HRESULT` | Returns S_OK if the method call was successful, |

**Description:** This method sends a message.

##### `SendMessageEx` (METHOD) → `HRESULT` — §3.4.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry` | Event definition |
| `ipSourceInfo` | `I_TcSourceInfo` | Interface pointer to the source information. |
| `stEventEntry` | `TcEventEntry` | Event definition |
| `ipSourceInfo` | `I_TcSourceInfo` | Interface pointer to the source information. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendMessageEx HRESULT` | `Returns S_OK if the method call was successful,` | — |
| `SendMessageEx HRESULT` | `Returns S_OK if the method call was successful,` | — |

**Description:** This method sends a message.

#### (top level)

##### `otherwise an HRESULT as the error code.` (FUNCTION_BLOCK) — §2

**Description:** The function block serves as the basic implementation of an event listener.

#### Functions and function blocks

##### `Execute` (METHOD) → `HRESULT` — §3.5.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `HRESULT` | Returns S_OK if the method call was successful, otherwise an |

**Description:** This method must be called cyclically so that the event queue can be processed.

##### `OnAlarmCleared` (METHOD) → `HRESULT` — §3.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbEvent` | `REFERENCE TO FB_TcEvent [} 69]` | Reference to the alarm that has occurred. This |

**Description:** This method is called if the state of an alarm changes from Raised to Clear.

##### `OnAlarmConfirmed` (METHOD) → `HRESULT` — §3.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbEvent` | `REFERENCE TO FB_TcEvent [} 69]` | Reference to the alarm that has occurred. This |

**Description:** This method is called when an alarm has been confirmed.

##### `OnAlarmDisposed` (METHOD) → `HRESULT` — §3.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbEvent` | `REFERENCE TO FB_TcEvent [} 69]` | Reference to the alarm that has occurred. This |

**Description:** This method is called when an alarm instance has been released again.

##### `OnAlarmRaised` (METHOD) → `HRESULT` — §3.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbEvent` | `REFERENCE TO FB_TcEvent [} 69]` | Reference to the alarm that has occurred. This |

**Description:** This method is called if the state of an alarm changes from Clear to Raised.

##### `OnMessageSent` (METHOD) → `HRESULT` — §3.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbEvent` | `REFERENCE TO FB_TcEvent [} 69]` | Reference to the event that has occurred. This |

**Description:** This method is called when a message has been sent.

##### `Subscribe` (METHOD) → `HRESULT` — §3.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipMessageFilterConfig` | `POINTER TO ITcEventFilterConfig` | Pointer to ITcEventFilterConfig if |
| `ipEventFilter` | `I_TcEventFilterBase` | Pointer to an instance of FB_TcEventFilter [} 41], if a filter is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Subscribe` | `HRESULT` | Returns S_OK if the method call was successful. |
| `Subscribe2` | `HRESULT` | Delivers if successful S_OK. |

**Description:** The listener is subscribed for messages with this method.

##### `Unsubscribe` (METHOD) → `HRESULT` — §3.5.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Unsubscribe` | `HRESULT` | Returns S_OK if the method call was successful. |

**Description:** The listener is unsubcribed with this method.

##### `FB_TcAlarm` (FUNCTION_BLOCK) — §3.6

**Description:** This function block represents an alarm of the TwinCAT 3 EventLogger.

#### Functions and function blocks › FB_TcAlarm

##### `Clear` (METHOD) → `HRESULT` — §3.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | 0: Current time stamp is used |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Clear` | `HRESULT` | Returns S_OK if the method call was successful. |

**Description:** This method sets the alarm state to Not Raised.

##### `Confirm` (METHOD) → `HRESULT` — §3.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | 0: Current timestamp is used. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Confirm` | `HRESULT` | Returns S_OK if the method call was successful. |

**Description:** This method sets the confirmation state of WaitForConfirmation to Confirmed.

##### `Create` (METHOD) → `HRESULT` — §3.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | GUID of the event class. |
| `nEventId` | `UDINT` | ID of the event. |
| `eSeverity` | `TcEventSeverity [} 112]` | Severity of the event. |
| `bWithConfirmation` | `BOOL` | Defines whether the alarm requires mandatory confirmation. |
| `ipSourceInfo` | `I_TcSourceInfo` | Interface pointer to the source information. |
| `[} 110]` | `This information is optional. An instance of the type` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Create` | `HRESULT` | Returns S_OK if a new alarm was successfully created. |

**Description:** This method creates an alarm instance in the EventLogger.

##### `CreateEx` (METHOD) → `HRESULT` — §3.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry [} 112]` | Event definition. |
| `bWithConfirmation` | `BOOL` | Defines whether the alarm requires mandatory |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateEx` | `HRESULT` | Returns S_OK if a new alarm was successfully created. |

**Description:** This method creates an alarm instance in the EventLogger.

##### `Raise` (METHOD) → `HRESULT` — §3.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | 0: Current timestamp is used. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Raise` | `HRESULT` | Returns S_OK if the method call was successful. |

**Description:** The method sets the alarm state to Raised.

##### `SetJsonAttribute` (METHOD) → `HRESULT` — §3.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonAttribute` | `STRING` | JSON string |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonAttribute` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetJsonAttribute` | `HRESULT` | Returns S_OK if the method call was successful, otherwise an |

**Description:** This method sets the JSON attribute.

#### Functions and function blocks

##### `FB_TcArguments` (FUNCTION_BLOCK) — §3.7

**Description:** Arguments of an event can be defined with this function block. The ITcArguments interface is implemented for this.

#### Functions and function blocks › FB_TcArguments

##### `IsEmpty` (METHOD) → `BOOL` — §3.7.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsEmpty` | `BOOL` | Returns TRUE if no arguments have been added. |

**Description:** This method checks whether arguments have been added.

#### Functions and function blocks

##### `FB_TcEvent` (FUNCTION_BLOCK) — §3.8

**Description:** This function block provides only read methods and read properties for an event.

##### `FB_TcEventBase` (FUNCTION_BLOCK) — §3.9

**Description:** This function block contains the basic implementation.

#### Functions and function blocks › FB_TcEventBase

##### `EqualsTo` (METHOD) → `BOOL` — §3.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipOther` | `I_TcEventBase [} 104] 72` | Event to be compared Functions and function blocks |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsTo` | `BOOL` | Returns TRUE if the events match. |

**Description:** This method carries out a comparison with another event specified at the input.

##### `EqualsToEventClass` (METHOD) → `BOOL` — §3.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `OtherEventClass` | `GUID` | Event class to be compared. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsToEventClass` | `BOOL` | Returns TRUE if the event classes match. |

**Description:** This method carries out a comparison with another event class specified at the input.

##### `EqualsToEventEntry` (METHOD) → `BOOL` — §3.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `OtherEventClass` | `GUID` | Event class of the event to be compared. |
| `nOtherEventID` | `UDINT` | Event ID of the event to be compared. |
| `eOtherSeverity` | `TcEventSeverity [} 112]` | Event severity of the event to be compared. Functions and function blocks |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsToEventEntry` | `BOOL` | Returns TRUE if the events match. |

**Description:** This method carries out a comparison with another event specified at the input.

##### `EqualsToEventEntryEx` (METHOD) → `BOOL` — §3.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stOther` | `TcEventEntry [} 112]` | Event to be compared. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsToEventEntryEx` | `BOOL` | Returns TRUE if the events match. |

**Description:** This method carries out a comparison with another event specified at the input.

##### `GetJsonAttribute` (METHOD) → `HRESULT` — §3.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonAttribute` | `REFERENCE TO STRING` | Reference to a variable of the type String |
| `nJsonAttribute` | `UDINT 74` | Length of the String variable Functions and function blocks |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetJsonAttribute` | `HRESULT` | Returns S_OK if the method call was successful. |

**Description:** This method returns the JSON attribute.

##### `Release` (METHOD) → `HRESULT` — §3.9.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Release` | `HRESULT` | Returns S_OK if the method call was successful, otherwise an |

**Description:** This method releases the instance created by the EventLogger again.

##### `RequestEventClassName` (METHOD) → `BOOL` — §3.9.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLangId` | `DINT …` | Specifies the language ID English (en-US) = 1033 German (de-DE) = 1031 |
| `sResult` | `REFERENCE TO STRING` | Reference to a variable of the type String |
| `nResultSize` | `UDINT` | Size of the String variable in bytes Functions and function blocks |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestEventClassName` | `BOOL` | Returns TRUE as soon as the request has been terminated. |
| `bError` | `BOOL` | Returns FALSE if the method call was successful. Returns TRUE if |

**Description:** This method returns the name of the event class.

##### `RequestEventText` (METHOD) → `BOOL` — §3.9.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLangId` | `DINT …` | Specifies the language ID English (en-US) = 1033 German (de-DE) = 1031 |
| `sResult` | `REFERENCE TO STRING` | Reference to a variable of the type String |
| `nResultSize` | `UDINT` | Size of the String variable in bytes |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestEventText` | `BOOL` | Returns TRUE as soon as the request has been terminated. |
| `bError` | `BOOL` | Returns FALSE if the method call was successful. Returns TRUE if |

**Description:** This method returns the event text.

##### `ipArguments` (PROPERTY) — §3.9.9

##### `ipSourceInfo` (PROPERTY) → `I_TcSourceInfo` — §3.9.10

**Return type:** `I_TcSourceInfo`

#### Functions and function blocks

##### `FB_TcEventLogger` (FUNCTION_BLOCK) — §3.10

**Description:** This function block represents the TwinCAT 3 EventLogger itself.

#### Functions and function blocks › FB_TcEventLogger

##### `ClearAlarms` (METHOD) → `HRESULT` — §3.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | Set to 0 to obtain the current time |

**Description:** Method for clearing active alarms. Returns S_OK if successful.

##### `ClearAllAlarms` (METHOD) → `HRESULT` — §3.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | 0: Current time stamp is used |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ClearAllAlarms` | `HRESULT` | Returns S_OK if the method call was successful, otherwise an HRESULT as the error code |

**Description:** This method calls the Clear() method for all alarms in the alarm state Raised.

##### `ClearLoggedEvents` (METHOD) → `BOOL` — §3.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipClearSettings` | `I_TcClearLoggedEventsSettings` | Optional (otherwise the entire cache is emptied) Return values |
| `ClearLoggedEvents` | `BOOL` | Returns TRUE as soon as the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Returns TRUE if the query is |

**Description:** Async method for clearing logged events. Returns TRUE if the asynchronous request is no longer assigned.

##### `ConfirmAlarms` (METHOD) → `HRESULT` — §3.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | Set to 0 to obtain the current time |

**Description:** This method confirms alarms.

##### `ConfirmAllAlarms` (METHOD) → `HRESULT` — §3.10.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | 0: Current time stamp is used |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfirmAllAlarms` | `HRESULT` | Returns S_OK if the method call was successful, otherwise an |

**Description:** This method calls the Confirm() method for all alarms having the confirmation state WaitForConfirmation.

##### `ExportLoggedEvents` (METHOD) → `BOOL` — §3.10.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipExportSettings` | `I_TcEventExportSettings` | Specify which events are to be exported, otherwise all events will |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sFileName` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE when an error occurs. |
| `hrErrorCode` | `HRESULT` | Outputs the error information if |

**Description:** This method exports logged events asynchronously. Returns TRUE when asynchronous processing is complete.

##### `GetAlarm` (METHOD) → `HRESULT` — §3.10.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | GUID of the event class. |
| `nEventId` | `UDINT` | ID of the event |
| `ipSourceInfo` | `I_TcSourceInfo [} 110]` | Pointer to an ITcSourceInfo interface. |
| `fbAlarm` | `REFERENCE TO FB_TcAlarm [} 61] Pointer to an alarm.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetAlarm` | `HRESULT` | Returns ADS_E_NOTFOUND if no instance was found. |

**Description:** This method returns an interface pointer to an existing instance.

##### `GetAlarmEx` (METHOD) → `HRESULT` — §3.10.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry [} 112]` | Event definition |
| `ipSourceInfo` | `I_TcSourceInfo [} 110]` | Pointer to an ITcSourceInfo interface |
| `fbAlarm` | `REFERENCE TO FB_TcAlarm [} 61]` | Pointer to an alarm |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetAlarmEx` | `HRESULT` | Returns ADS_E_NOTFOUND if no instance was found. |

**Description:** This method returns an interface pointer to an existing instance.

##### `IsAlarmRaised` (METHOD) → `BOOL` — §3.10.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | GUID of the event class. |
| `nEventId` | `UDINT` | ID of the event. |
| `ipSourceInfo` | `I_TcSourceInfo [} 110]` | Pointer to an ITcSourceInfo interface. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsAlarmRaised` | `BOOL` | Returns TRUE if the alarm is in the raised state. |

**Description:** This method queries whether an alarm is in the Raised state.

##### `IsAlarmRaisedEx` (METHOD) → `BOOL` — §3.10.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `UDINT` | Event definition. |
| `ipSourceInfo` | `I_TcSourceInfo [} 110]` | Pointer to an ITcSourceInfo interface. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsAlarmRaisedEx` | `BOOL` | Returns TRUE if the alarm is in the raised state. |

**Description:** Functions and function blocks

##### `SendMessage` (METHOD) → `HRESULT` — §3.10.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | GUID of the event class. |
| `nEventId` | `UDINT` | ID of the event. |
| `eSeverity` | `TcEventSeverity [} 112]` | Severity of the event. |
| `ipSourceInfo` | `I_TcSourceInfo [} 110]` | Interface pointer to the source information. |
| `eventClass` | `GUID` | GUID of the event class |
| `nEventId` | `UDINT` | ID of the event |
| `eSeverity` | `TcEventSeverity` | Severity of the event |
| `ipSourceInfo` | `I_TcSourceInfo` | Interface pointer to the source |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonAttribute` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendMessage` | `HRESULT` | Returns S_OK if the method call was successful, otherwise an |

**Description:** This method sends a message.

##### `SendMessageEx` (METHOD) → `HRESULT` — §3.10.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry [} 112]` | Event definition. |
| `ipSourceInfo` | `I_TcSourceInfo [} 110]` | Interface pointer to the source information. This information is optional. An instance of the type FB_TcSourceInfo [} 92] can be specified here. If nothing (NULL) is transferred, standard source information is |
| `stEventEntry` | `TcEventEntry` | Event definition |
| `ipSourceInfo` | `I_TcSourceInfo` | Interface pointer to the source |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonAttribute` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendMessageEx` | `HRESULT` | Returns S_OK if the method call was successful, otherwise an |

**Description:** This method sends a message.

#### Functions and function blocks

##### `FB_TcMessage` (FUNCTION_BLOCK) — §3.11

**Description:** This function block represents a message from the TwinCAT 3 EventLogger.

#### Functions and function blocks › FB_TcMessage

##### `Create` (METHOD) → `HRESULT` — §3.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eventClass` | `GUID` | GUID of the event class. |
| `nEventId` | `UDINT` | ID of the event. |
| `eSeverity` | `TcEventSeverity [} 112]` | Defines the severity. |
| `ipSourceInfo` | `I_TcSourceInfo` | Interface pointer to the source information. |
| `[} 110]` | `This information is optional. An instance of the type` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Create` | `HRESULT` | Returns S_OK if the method call was successful, otherwise an |

**Description:** This method creates a message instance in the EventLogger.

##### `CreateEx` (METHOD) — §3.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry [} 112]` | Event definition. |
| `ipSourceInfo` | `I_TcSourceInfo [} 110]` | Interface pointer to the source information. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateEx` | `HRESULT` | Returns S_OK if the method call was successful, otherwise an |

**Description:** This method creates a message instance in the EventLogger from an event definition.

##### `SetJsonAttribute` (METHOD) → `HRESULT` — §3.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonAttribute` | `STRING` | JSON string |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonAttribute` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetJsonAttribute` | `HRESULT` | Returns S_OK if the method call was successful, otherwise an |

**Description:** Functions and function blocks

#### Functions and function blocks

##### `FB_TcSourceInfo` (FUNCTION_BLOCK) — §3.12

**Description:** The source information of an event can be defined with this function block.

#### Functions and function blocks › FB_TcSourceInfo

##### `Clear` (METHOD) — §3.12.1

**Description:** This method resets the source information.

##### `ExtendName` (METHOD) → `BOOL` — §3.12.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sExtension` | `STRING(255)` | Text to be appended to the right. Functions and function blocks |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ExtendName` | `BOOL` | Returns TRUE if the concatenation was successful. |

**Description:** This method extends the name.

##### `ResetToDefault` (METHOD) — §3.12.3

**Description:** This method sets the source information to default values.

#### Interfaces › I_TcArguments

##### `AddBlob` (METHOD) → `I_TcArguments` — §4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pData` | `PVOID` | Pointer to the first byte of the binary data. |
| `cbData` | `UINT` | Length of the binary data in bytes. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddBlob` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds binary data as an argument.

##### `AddBool` (METHOD) → `I_TcArguments` — §4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `BOOL` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddBool` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type BOOL.

##### `AddByte` (METHOD) → `I_TcArguments` — §4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `BYTE` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddByte` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type BYTE.

##### `AddDint` (METHOD) → `I_TcArguments` — §4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DINT` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddDINT` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type DINT.

##### `AddDWord` (METHOD) → `I_TcArguments` — §4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DWORD` | Value to be added. Interfaces |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddDWord I_TcArguments [} 95]` | `Returns the I_TcArgument pointer again.` | — |

**Description:** This method adds an argument of the type DWORD.

##### `AddEventReferenceId` (METHOD) → `I_TcArguments` — §4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nEventId` | `UDINT` | ID of the event. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddEventReferenceId` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds a reference to another event as an argument.

##### `AddEventReferenceIdGuid` (METHOD) → `I_TcArguments` — §4.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nEventId` | `UDINT` | ID of the event. |
| `EventClass` | `GUID` | GUID of the event class. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddEventReferenceIdGuid` | `I_TcArguments [} 95] 98` | Returns the I_TcArgument pointer again. Interfaces |

**Description:** This method adds a reference to another event as an argument.

##### `AddInt` (METHOD) → `I_TcArguments` — §4.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `INT` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddInt` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type INT.

##### `AddLInt` (METHOD) → `I_TcArguments` — §4.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `LINT` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddLInt` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type LINT.

##### `AddLReal` (METHOD) → `I_TcArguments` — §4.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `LREAL` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddLReal` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type LREAL.

##### `AddReal` (METHOD) → `I_TcArguments` — §4.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `REAL` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReal` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type REAL.

##### `AddSInt` (METHOD) → `I_TcArguments` — §4.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `SINT` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddSInt` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type SINT.

##### `AddString` (METHOD) → `I_TcArguments` — §4.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `STRING` | Value to be added. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddString` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type STRING.

##### `AddUDint` (METHOD) → `I_TcArguments` — §4.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `UDINT` | Value to be added. Interfaces |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddUDInt` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type UDINT.

##### `AddUInt` (METHOD) → `I_TcArguments` — §4.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `UINT` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddUInt` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type INT.

##### `AddULInt` (METHOD) → `I_TcArguments` — §4.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `ULINT` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddULInt` | `I_TcArguments [} 95] 102` | Returns the I_TcArgument pointer again. Interfaces |

**Description:** This method adds an argument of the type ULINT.

##### `AddUSInt` (METHOD) → `I_TcArguments` — §4.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `USINT` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddUSInt` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type USINT.

##### `AddWord` (METHOD) → `I_TcArguments` — §4.1.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `WORD` | Value to be added. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddWord` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type WORD.

##### `AddWString` (METHOD) → `I_TcArguments` — §4.1.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `WSTRING` | Value to be added. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `WSTRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddWString` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method adds an argument of the type WSTRING.

##### `Clear` (METHOD) → `I_TcArguments` — §4.1.20

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Clear` | `I_TcArguments [} 95]` | Returns the I_TcArgument pointer again. |

**Description:** This method removes all arguments.

#### Interfaces › I_TcEventBase

##### `EqualsTo` (METHOD) → `BOOL` — §4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipOther` | `I_TcEventBase [} 104]` | Event to be compared |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsTo` | `BOOL` | Returns TRUE if the events match. |

**Description:** This method carries out a comparison with another event specified at the input.

##### `EqualsToEventClass` (METHOD) → `BOOL` — §4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `OtherEventClass` | `GUID` | Event class to be compared. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsToEventClass` | `BOOL` | Returns TRUE if the event classes match. |

**Description:** This method carries out a comparison with another event class specified at the input.

##### `EqualsToEventEntry` (METHOD) → `BOOL` — §4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `OtherEventClass` | `GUID` | Event class of the event to be compared. |
| `nOtherEventID` | `UDINT` | Event ID of the event to be compared. |
| `eOtherSeverity` | `TcEventSeverity [} 112]` | Event severity of the event to be compared. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsToEventEntry` | `BOOL` | Returns TRUE if the events match. |

**Description:** This method carries out a comparison with another event specified at the input.

##### `EqualsToEventEntryEx` (METHOD) → `BOOL` — §4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stOther` | `TcEventEntry [} 112]` | Event to be compared. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsToEventEntryEx` | `BOOL` | Returns TRUE if the events match. |

**Description:** This method carries out a comparison with another event specified at the input.

##### `GetJsonAttribute` (METHOD) → `HRESULT` — §4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonAttribute` | `REFERENCE TO STRING` | Reference to a variable of the type String |
| `nJsonAttribute` | `UDINT` | Length of the String variable |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetJsonAttribute` | `HRESULT` | Returns S_OK if the method call was successful. |

**Description:** This method returns the JSON attribute.

##### `RequestEventClassName` (METHOD) → `BOOL` — §4.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLangId` | `DINT …` | Specifies the language ID English (en-US) = 1033 German (de-DE) = 1031 |
| `sResult` | `REFERENCE TO STRING` | Reference to a variable of the type String |
| `nResultSize` | `UDINT` | Size of the String variable in bytes |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestEventClassName` | `BOOL` | Returns TRUE as soon as the request has been terminated. |
| `bError` | `BOOL` | Returns FALSE if the method call was successful. Returns TRUE if |

**Description:** This method returns the name of the event class.

##### `RequestEventText` (METHOD) → `BOOL` — §4.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLangId` | `DINT …` | Specifies the language ID English (en-US) = 1033 German (de-DE) = 1031 |
| `sResult` | `REFERENCE TO STRING` | Reference to a variable of the type String |
| `nResultSize` | `UDINT` | Size of the String variable in bytes |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestEventText` | `BOOL` | Returns TRUE as soon as the request has been terminated. |
| `bError` | `BOOL` | Returns FALSE if the method call was successful. Returns TRUE if |

**Description:** This method returns the event text.

#### Interfaces › I_TcMessage

##### `Send` (METHOD) → `HRESULT` — §4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | 0: Current time stamp is used |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Send` | `FB_ HRESULT` | Returns S_OK if the method call was successful, otherwise an HRESULT as the error code |

**Description:** This method sends the message.

#### Interfaces › I_TcSourceInfo

##### `EqualsTo` (METHOD) → `BOOL` — §4.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipOther` | `I_TcSourceInfo [} 110]` | Items of source information to be compared |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsTo` | `BOOL` | Returns TRUE if the items of source information match. Data types |

**Description:** This method compares an instance with source information with another instance.
