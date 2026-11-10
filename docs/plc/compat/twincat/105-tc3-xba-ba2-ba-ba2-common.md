# Tc3_XBA+BA2+BA+BA2_Common

- **Manual:** `TF8040_TC3_Building_Automation_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T0 · **API entries:** 407
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Cross-protocol BA aggregation types: shared types; transports per protocol.

---

#### Concepts › HMI

##### `BaInterface` (FUNCTION_BLOCK) — §3.8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bToggle` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stFeedback` | `LightFeedback` | — |

**Description:** TcHmiBa controls [} 1107] that support the BaInterface can also be used without the BaSite and TF8040 PLC templates. Functionality and behavior may vary.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › FunctionBlocks › Accessories › EventSubscriber

##### `FB_BA_EventSubscriber` (FUNCTION_BLOCK) — §6.1.2.1.3.1.1.2.1

**Description:** The FB_BA_EventSubscriber function block is the basic block for responding to events. It does not have to be called explicitly.

##### `FB_BA_ProjectEventSubscriber` (FUNCTION_BLOCK) — §6.1.2.1.3.1.1.2.2

**Description:** The FB_BA_ProjectEventSubscriber function block offers the possibility to respond to project-wide events.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › FunctionBlocks › Accessories › EventSubscriber › FB_BA_ProjectEventSubscriber

##### `OnProjectStateChanged` (METHOD) — §6.1.2.1.3.1.1.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eState` | `E_BA_ProjectState [} 116] State to which the project has changed.` | — |

**Description:** The method is called when the project state has changed.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › FunctionBlocks › Accessories › EventSubscriber

##### `FB_BA_LocalObjectEventSubscriber` (FUNCTION_BLOCK) — §6.1.2.1.3.1.1.2.3

**Description:** The FB_BA_LocalObjectEventSubscriber function block provides the ability to respond to events in local objects.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › FunctionBlocks › Accessories › EventSubscriber › FB_BA_LocalObjectEventSubscriber

##### `OnBACnetObjectStateChanged` (METHOD) — §6.1.2.1.3.1.1.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObject` | `I_BA_Object` | Interface to the context object. |
| `fbBACnetObject` | `POINTER TO` | Supplement specific BACnet object of the context object. FB_BACnet_BaseObject |
| `tState` | `TCOM_STATE` | New state of the BACnet object. |

**Description:** The method is called when the initialization state of a BACnet object changes.

##### `OnInitStateChange` (METHOD) — §6.1.2.1.3.1.1.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObject` | `I_BA_Object` | Interface to the context object. [} 262] |
| `eState` | `E_BA_InitState [} 117]` | New state after initialization. |

**Description:** The method is called when the initialization state of an object changes.

##### `OnInitInstanceId` (METHOD) — §6.1.2.1.3.1.1.2.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObject` | `I_BA_Object` | Interface to the context object. [} 262] |

**Description:** The method can be used to assign a specific value to the instance ID of an object.

##### `OnEventChange` (METHOD) — §6.1.2.1.3.1.1.2.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObject` | `I_BA_Object` | Interface to the context object. [} 262] |

**Description:** The method is called when an object generates an event, or when a generated object changes or is deleted.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › FunctionBlocks › Control

##### `FB_BA_LoopSync` (FUNCTION_BLOCK) — §6.1.2.1.3.1.2.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Loop01` | `FB_BA_Loop` | — |
| `Loop02` | `FB_BA_Loop` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bErr` | `BOOL` | The output indicates when an error has occurred during |

**Description:** The FB_BA_LoopSync function block is used for parameter synchronization for two controllers of type FB_BA_Loop [} 220]. This function block can be used, for example, to calibrate master controllers that are used to generate the supply air setpoints for an air conditioning system.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › FunctionBlocks › Events

##### `FB_BA_EventObserver` (FUNCTION_BLOCK) — §6.1.2.1.3.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bAck` | `BOOL` | The signal for acknowledging alarms / events is applied to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bHasEvents` | `BOOL` | Display that events are pending. |
| `bHasUnackEvents` | `BOOL` | Display that unacknowledged events are pending. Properties |
| `Parent` | `I_BA_View` | Get, Set Access to the project structure (base framework) is realized via an |

**Description:** The FB_BA_EventObserver function block is used to evaluate alarms/events in the project structure and acknowledge them.

##### `FB_BA_PlantLock` (FUNCTION_BLOCK) — §6.1.2.1.3.1.3.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPrioLocalHigh` | `BOOL` | The output signals a global event with a high priority. |
| `bPrioMedium` | `BOOL` | The output signals a global event with a medium priority. |
| `bPrioLocalHigh` | `BOOL` | The output signals a local event with a high priority. |
| `bPrioLocalMedium` | `BOOL` | The output signals a local event with a medium priority. |

**Description:** The events of the objects often signal a relevant malfunction, which requires targeted switching operations to be performed on aggregates or plants. In large plants, there are often a large number of events, which then frequently have to be combined into a collective message. To keep the programming effort for generating collective messages as low as possible, a lock priority can be determined at each event-capable object. All four lock priorities are collected at each level of the project structure [} 38] and passed up from the lowest level of the project structure to the top level. If required, they are displayed by the instance of a function block FB_BA_PlantLock and interconnected with other function blocks for the control of plant and aggregate units.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › FunctionBlocks › Reference › Publish and Subscribe › Publisher

##### `FB_BA_RawPublisher` (FUNCTION_BLOCK) — §6.1.2.1.3.1.4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `xData` | `ANY` | Data to be published. |

**Description:** The FB_BA_RawPublisher function block is used to provide any information.

##### `FB_BA_Publisher` (FUNCTION_BLOCK) — §6.1.2.1.3.1.4.1.1.2

**Description:** The FB_BA_Publisher is a function block for distributing data. It does not have to be called explicitly. This description serves solely to explain the inheritance structure of the Publisher function blocks.

##### `FB_BA_BasePublisher` (FUNCTION_BLOCK) — §6.1.2.1.3.1.4.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnPublish` | `BOOL` | — |

**Description:** The FB_BA_BasePublisher function block is the base class of the publisher. It does not have to be called explicitly. This description serves solely to explain the inheritance structure of the Publisher function blocks.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › FunctionBlocks › Reference › Publish and Subscribe › Subscriber

##### `Objects` (FUNCTION_BLOCK) — §6.1.2.1.3.1.4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sTarget` | `T_BA_MedString [} 123]` | AMS NetId of the Publisher. |
| `tReadInterval` | `TIME` | Read interval [s]. |
| `tReadTolerance` | `TIME` | Waiting time until a pending error is output [ms]. |
| `bClearOnReadError BOOL` | `If TRUE, the data is deleted if an error occurs.` | — |
| `sTarget` | `T_BA_MedString [} 123]` | AMS NetId of the Publisher. |
| `tReadInterval` | `TIME` | Read interval [s]. |
| `tReadTolerance` | `TIME` | Waiting time until a pending error is output [ms]. |
| `bClearOnReadError BOOL` | `If TRUE, the data is deleted if an error occurs.` | — |
| `sTarget` | `T_BA_MedString [} 123]` | AMS NetId of the Publisher. |
| `tReadInterval` | `TIME` | Read interval [s]. |
| `tReadTolerance` | `TIME` | Waiting time until a pending error is output [ms]. |
| `bClearOnReadError BOOL` | `If TRUE, the data is deleted if an error occurs.` | — |
| `sTarget` | `T_BA_MedString [} 123]` | AMS NetId of the Publisher. |
| `tReadInterval` | `TIME` | Read interval [s]. |
| `tReadTolerance` | `TIME` | Waiting time until a pending error is output [ms]. |
| `bClearOnReadError BOOL` | `If TRUE, the data is deleted if an error occurs.` | — |
| `sTarget` | `T_BA_MedString [} 123]` | AMS NetId of the Publisher. |
| `tReadInterval` | `TIME` | Read interval [s]. |
| `tReadTolerance` | `TIME` | Waiting time until a pending error is output [ms]. |
| `bClearOnReadError BOOL` | `If TRUE, the data is deleted if an error occurs.` | — |
| `sTarget` | `T_BA_MedString [} 123]` | AMS NetId of the Publisher. |
| `tReadInterval` | `TIME` | Read interval [s]. |
| `tReadTolerance` | `TIME` | Waiting time until a pending error is output [ms]. |
| `bClearOnReadError BOOL` | `If TRUE, the data is deleted if an error occurs.` | — |
| `sTarget` | `T_BA_MedString [} 123]` | AMS NetId of the Publisher. |
| `tReadInterval` | `TIME` | Read interval [s]. |
| `tReadTolerance` | `TIME` | Waiting time until a pending error is output [ms]. |
| `bClearOnReadError BOOL` | `If TRUE, the data is deleted if an error occurs.` | — |
| `sTarget` | `T_BA_MedString [} 123]` | AMS NetId of the Publisher. |
| `tReadInterval` | `TIME` | Read interval [s]. |
| `tReadTolerance` | `TIME` | Waiting time until a pending error is output [ms]. |
| `bClearOnReadError BOOL` | `If TRUE, the data is deleted if an error occurs.` | — |
| `sTarget` | `T_BA_MedString [} 123]` | AMS NetId of the Publisher. |
| `tReadInterval` | `TIME` | Read interval [s]. |
| `tReadTolerance` | `TIME` | Waiting time until a pending error is output [ms]. |
| `bClearOnReadError BOOL` | `If TRUE, the data is deleted if an error occurs.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current analog output value of the object. |
| `fPresentValue` | `REAL` | Current analog output value of the object. |
| `bEvent` | `BOOL` | Indicates an event or an error that has occurred. |
| `bError` | `BOOL` | Displays the current error state of the subscription. |
| `bError` | `BOOL` | Displays the current error state of the subscription. |
| `bError` | `BOOL` | Displays the current error state of the subscription. |
| `bError` | `BOOL` | Displays the current error state of the subscription. |
| `bError` | `BOOL` | Displays the current error state of the subscription. |
| `bError` | `BOOL` | Displays the current error state of the subscription. |
| `bPresentValue` | `BOOL` | Present value of the subscribed binary value. |
| `bPresentValue` | `BOOL` | Present value of the subscribed binary value. |
| `bEvent` | `BOOL` | Indicates an event or an error that has occurred. |
| `fPresentValue` | `REAL` | Current analog output value of the object. |
| `fPredictedValue` | `REAL` | Value that is assumed after the next switching. Properties |
| `Unit` | `E_BA_Unit` | Get Unit. Methods GetData [} 163] Contains the subscribed data. |
| `pData` | `pData` | Pointer to the subscribed data. |
| `nSize` | `DINT` | Size of the subscribed data. |
| `bPresentValue` | `BOOL` | Current binary output value of the object. |
| `bPredictedValue` | `BOOL` | Value that is assumed after the next switching. |
| `ActiveText` | `STRING` | Get Active text |
| `InactiveText` | `STRING` | Get Inactive text Methods |
| `GetData [} 164]` | `Contains the subscribed data.` | — |
| `pData` | `pData` | Pointer to the subscribed data. |
| `nSize` | `DINT` | Size of the subscribed data. |
| `nPresentValue` | `UDINT` | Analog output value. |
| `nPredictedValue` | `UDINT` | Value that is assumed after the next switching. Properties |
| `StateCount` | `UDINT` | Get Number of possible states. |
| `StateText` | `T_BA_StateTextAr Get` | Output of the state text. ray [} 124] Methods GetData [} 165] Contains the subscribed data. |
| `pData` | `pData` | Pointer to the subscribed data. |
| `nSize` | `DINT` | Size of the subscribed data. |
| `bEvent` | `BOOL` | The variable signals an abnormal event of an object. |
| `pData` | `pData` | Pointer to the subscribed data. |
| `nSize` | `DINT` | Size of the subscribed data. |
| `bError` | `BOOL` | Displays the current error state of the subscription. |
| `bError` | `BOOL` | Displays the current error state of the subscription. |
| `bError` | `BOOL` | Displays the current error state of the subscription. |
| `nPresentValue` | `UDINT` | Present value of the subscribed value. |
| `nPresentValue` | `UDINT` | Present value of the subscribed value. |
| `bEvent` | `BOOL` | Indicates an event or an error that has occurred. |

**Description:** This folder offers objects that make it possible to subscribe to published object information either within the controller or across controllers. 6.1. Analog 2.1. 3.1. 4.1. 2.1. 1

##### `FB_BA_RawSubscriber` (FUNCTION_BLOCK) — §6.1.2.1.3.1.4.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `xData` | `ANY` | Read data. |

**Description:** Function block for subscribing to any information.

##### `FB_BA_Subscriber` (FUNCTION_BLOCK) — §6.1.2.1.3.1.4.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sTarget` | `T_BA_MedString` | — |
| `tReadInterval` | `TIME := XBA_Param.tSub_DefReadInterval` | — |
| `tReadTolerance` | `TIME := XBA_Param.tSub_ReadTolerance; // Time to wait before indicating a` | — |
| `bClearOnReadError` | `BOOL := XBA_Param.bSub_ClearOnReadError; // Clear data if an error occurs.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Displays the current error state of the subscription. |

**Description:** The FB_BA_Subscriber is a function block for subscribing to data. It does not have to be called explicitly. This description serves solely to explain the inheritance structure of the publisher function blocks.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › FunctionBlocks › Sequence

##### `FB_BA_SeqCtrl` (FUNCTION_BLOCK) — §6.1.2.1.3.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nMyNum` | `UDINT` | Number of the considered sequence controller. This |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSeqLink` | `ST_BA_SeqLink` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fE` | `REAL` | Displays the control deviation of the sequence controller. |

**Description:** The FB_BA_SeqCtrl function block is a PID sequence controller as part of a sequence.

##### `FB_BA_SequenceLink` (FUNCTION_BLOCK) — §6.1.2.1.3.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Sequence control is activated when the function block is |

**Description:** The FB_BA_SequenceLink function block connects the individual controllers with each other to form a control sequence.

##### `FB_BA_SequenceLinkBase` (FUNCTION_BLOCK) — §6.1.2.1.3.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Sequence control is activated when the function block is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSeqLink` | `ST_BA_SeqLink` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSeqCtrlOperational BOOL` | `The sequence control is ready to operate or is in operation.` | — |
| `nActvSeqCtrl` | `UDINT` | The output variable shows the number of the active |

**Description:** The FB_BA_SequenceLinkBase function block represents the higher-level connection of a sequence control.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Functions › Parameter › Builder › F_BA_CalendarBuilder

##### `Value` (METHOD) → `I_BA_CalendarBuilder` — §6.1.2.1.3.2.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFromYear` | `UINT` | Start year from 1900 to 2155. |
| `eFromMonth` | `E_BA_Month` | Start month. |
| `nFromDay` | `UINT` | Start day of a month from 1 to 31. |
| `nToYear` | `UINT` | End year from 1900 to 2155. |
| `eToMonth` | `E_BA_Month` | End month. |
| `nToDay` | `UINT` | End day of a month from 1 to 31. |
| `nYear` | `UINT` | Year from 1900 to 2155. |
| `eMonth` | `E_BA_Month` | Month. |
| `nDay` | `UINT` | Day of a month from 1 to 31. |
| `eWeekday` | `E_BA_Weekday` | Day of the week. |
| `eWeekOfMonth` | `E_BA_Week` | Week within a month. |
| `eMonth` | `E_BA_Month` | Month. |

**Description:** 6.1. AppendDateRange 2.1. 3.2. 1.1. 1.1. 1

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Functions › Parameter › Builder › F_BA_ExceptionScheduleBuilder

##### `Value` (METHOD) → `I_BA_CalendarBuilder` — §6.1.2.1.3.2.1.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFromYear` | `UINT` | Start year from 1900 to 2155. |
| `eFromMonth` | `E_BA_Month` | Start month. |
| `nFromDay` | `UINT` | Start day of a month from 1 to 31. |
| `nToYear` | `UINT` | End year from 1900 to 2155. |
| `eToMonth` | `E_BA_Month` | End month. |
| `nToDay` | `UINT` | End day of a month from 1 to 31. |
| `nYear` | `UINT` | Year from 1900 to 2155. |
| `eMonth` | `E_BA_Month` | Month. |
| `nDay` | `UINT` | Day of a month from 1 to 31. Programming |
| `eWeekday` | `E_BA_Weekday` | Day of the week. |
| `eWeekOfMonth` | `E_BA_Week` | Week within a month. |
| `eMonth` | `E_BA_Month` | Month. |

**Description:** 6.1. AppendDateRange 2.1. 3.2. 1.1. 2.1. 1

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Functions › Parameter › Builder › F_BA_ParameterRolesBuilder

##### `Value` (METHOD) → `I_BA_ParamRoleBuilder` — §6.1.2.1.3.2.1.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eParam` | `E_BA_Parameter [} 111]` | BACnet property. |
| `eAccess` | `E_BA_Role [} 102]` | Definition of access rights. |
| `eAccess` | `E_BA_Role [} 102]` | Definition of access rights. Programming |
| `eParamFrom` | `E_BA_Parameter [} 111]` | First BACnet Property. |
| `eParamTo` | `E_BA_Parameter [} 111]` | Last BACnet Property. |
| `eAccess` | `E_BA_Role [} 102]` | Definition of access rights. |

**Description:** 6.1. Set 2.1. 3.2. 1.1. 3.1. 1

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Functions › Parameter › Builder › F_BA_PlantOperationBuilder

##### `Value` (METHOD) → `I_BA_PlantOperationBuilder` — §6.1.2.1.3.2.1.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iRef` | `I_BA_Aggregate` | Reference to the corresponding aggregate. |
| `bWaitForProcesses` | `BOOL` | Active processes are taken into account when switching. |
| `bWaitForEvents` | `BOOL` | Events of this (and higher) lock priority are taken into |
| `nOpMode` | `UDINT` | Operation mode. |
| `ePrio` | `E_BA_Priority [} 113]` | Priority. |
| `eIgnore` | `E_BA_AggregateIgnoreFla Defines the switching conditions. gs [} 109]` | — |

**Description:** 6.1. AddAggregate 2.1. 3.2. 1.1. 4.1. 1

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Functions › Parameter › Builder › F_BA_ScheduleCalendarBuilder

##### `Value` (METHOD) → `I_BA_ScheduleCalendarEntryBuilder` — §6.1.2.1.3.2.1.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iCalendar` | `I_BA_Object` | Calendar reference. |

**Description:** 6.1. AppendReference 2.1. 3.2. 1.1. 5.1. 1

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Functions › Parameter › Builder › F_BA_WeeklyScheduleBuilder

##### `Value` (METHOD) → `I_BA_WeeklyEntryBuilder` — §6.1.2.1.3.2.1.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDay` | `E_BA_Weekday` | Selected day of the week. |
| `eDayFrom` | `E_BA_Weekday` | First day of the period. |
| `eDayTo` | `E_BA_Weekday` | Last day of the period. |

**Description:** 6.1. SetAllDays 2.1. 3.2. 1.1. 6.1. 1

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Objects › Local › Analog

##### `FB_BA_AI` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fVal` | `REAL` | Scaled, analog input value. |
| `eRlbty` | `E_BA_Reliability` | Availability or reliability of a value. |

**Description:** The function block represents an analog input.

##### `FB_BA_AI_IO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fResolution` | `REAL := XBA_Param.fInput_DefResolution` | — |
| `fScaleOffset` | `REAL := XBA_Param.fInput_DefScaleOffset` | — |

**Description:** The function block represents the object for an analog input.

##### `FB_BA_AI_IOEx` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fResolution` | `REAL := XBA_Param.fInput_DefResolution` | — |
| `fScaleOffset` | `REAL := XBA_Param.fInput_DefScaleOffset` | — |
| `eSensor` | `E_BA_MeasuringElement := BA_Param.fInput_DefSensor` | — |
| `bConfigurate` | `BOOL := TRUE` | — |

**Description:** The function block represents the object of an analog input.

##### `FB_BA_AI_Raw` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRawState` | `USINT` | Variable for linking the status information of a terminal. |
| `nRawVal` | `INT` | Raw value. Inputs CONSTANT PERSISTENT |
| `fResolution` | `REAL` | Default value for the Resolution parameter of inputs |

**Description:** The function block represents the object of an analog input.

##### `FB_BA_AO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eRlbty` | `E_BA_Reliability` | Availability or reliability of a value. |
| `bRawOvrrd` | `BOOL` | The feedback of a switch contained in a terminal can be |

**Description:** The function block is used to map a control output within the project structure.

##### `FB_BA_AO_IO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fResolution` | `REAL := XBA_Param.fOutput_DefResolution` | — |
| `fScaleOffset` | `REAL := XBA_Param.fOutput_DefScaleOffset` | — |
| `eOverriddenPolarity` | `E_BA_Polarity := E_BA_Polarity.eNormal` | — |

**Description:** The function block represents an analog output object within the base framework. The variables for linking the control output to the terminal are declared within the function block.

##### `FB_BA_AO_Raw` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bRawOverridden` | `BOOL` | To this variable can be connected the feedback of a switch |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nRawVal` | `INT` | The variable is used to link the raw value of an object with |

**Description:** The function block represents an analog output object within the base framework. The variable nRawVal is used to link the control value with the process image of the terminal.

##### `FB_BA_AV` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnSfty` | `BOOL` | Enabling the "Safety" priority. |
| `fValSfty` | `REAL` | Analog value for the "Safety" priority. |
| `bEnCrit` | `BOOL` | Enabling the "Critical" priority. |
| `fValCrit` | `REAL` | Analog value for the "Critical" priority. |
| `bEnManLoc` | `BOOL` | Enabling the "Manual Local" priority. |
| `fValManLoc` | `REAL` | Analog value for the "Manual Local" priority. |
| `bEnPgm` | `BOOL` | Enabling the "Program" priority. |
| `fValPgm` | `REAL` | Analog value for the "Program" priority. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current analog output value of the object. |
| `eActivePrio` | `E_BA_Priority [} 113]` | Active priority. |
| `fDefaultValue` | `REAL` | Value that is assumed if all 16 priorities of the Priority array |

**Description:** The function block represents an analog Value object.

##### `FB_BA_AV_Op` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fValuePgm` | `Real` | Output values of the analog object for the "Program" |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current analog output value of the object. Inputs CONSTANT PERSISTENT |
| `eUnit` | `E_BA_Unit` | Unit of the input or output value of an analog object. |
| `fCOVIncrement` | `REAL` | The variable specifies the minimum change in present |

**Description:** The function block represents an analog Value object. It is used to display or enter an analog value.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Objects › Local › Binary

##### `FB_BA_BI` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bVal` | `BOOL` | Binary value. |
| `eRlbty` | `E_BA_Reliability` | Availability or reliability of a value. |

**Description:** The function block FB_BA_BI represents a binary input object within the base framework. The variables for linking the input to the terminal are available as input variables on the function block.

##### `FB_BA_BI_IO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ePolarity` | `E_BA_Polarity := E_BA_Polarity.eNormal` | — |

**Description:** The function block FB_BA_BI_IO represents a binary input object within the base framework. The variables for linking the input to the terminal are declared within the function block.

##### `FB_BA_BI_Raw` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bRawVal` | `BOOL` | Raw value. |

**Description:** The function block FB_BA_BI_Raw generates a binary input object within the basic framework. The variable bRawVal is used to link the binary input to the terminal.

##### `FB_BA_BO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eRlbty` | `E_BA_Reliability` | Availability or reliability of a value. |
| `bRawOvrrd` | `BOOL` | The feedback of a switch contained in a terminal can be |

**Description:** The function block FB_BA_BO represents a binary output object within the base framework. The variables for linking the control output to the terminal are available as input or output variables on the function block.

##### `FB_BA_BO_IO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ePolarity` | `E_BA_Polarity := E_BA_Polarity.eNormal` | — |
| `eFeedbackPolarity` | `E_BA_Polarity := E_BA_Polarity.eNormal` | — |
| `eOverriddenPolarity` | `E_BA_Polarity := E_BA_Polarity.eNormal` | — |

**Description:** The FB_BA_BO_IO function block represents a binary output object within the base framework. The variables for linking the switching output to the terminal are declared within the function block. The feedback control of the binary output is automatically activated if the bRawValFeedback variable is linked to the process image of a terminal.

##### `FB_BA_BO_Raw` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bRawOvrrd` | `BOOL` | The feedback of a switch contained in a terminal can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bRawVal` | `BOOL` | Value of the binary output for linking to the process image |

**Description:** The function block FB_BA_BO_Raw represents a binary output object within the base framework. The variable bRawVal is used to link the switching command with the process image of the terminal.

##### `FB_BA_BV` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnSfty` | `BOOL` | Enabling the "Safety" priority. |
| `bValSfty` | `BOOL` | Binary value for the "Safety" priority. |
| `bEnCrit` | `BOOL` | Enabling the "Critical" priority. |
| `bValCrit` | `BOOL` | Binary value for the "Critical" priority. |
| `bEnManLoc` | `BOOL` | Enabling the "Manual Local" priority. |
| `bValManLoc` | `BOOL` | Binary value for the "Manual Local" priority. |
| `bEnPgm` | `BOOL` | Enabling the "Program" priority. |
| `bValPgm` | `BOOL` | Binary value for the "Program" priority. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresentValue` | `BOOL` | Current binary output value of the object. |
| `eActivePrio` | `E_BA_Priority [} 113]` | Active priority. |
| `nMinimumOffTime` | `UDINT` | Minimum time [s] in which the Present Value should |

**Description:** The function block FB_BA_BV represents a binary value object.

##### `FB_BA_BV_Op` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bValuePgm` | `BOOL` | Value of a binary object for the "Program" priority. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresentValue` | `BOOL` | Current binary output value of the object. Inputs CONSTANT PERSISTENT |
| `bDefaultValue` | `BOOL` | Value that is assumed if all 16 priorities of the Priority array |

**Description:** The FB_BA_BV_Op function block represents a binary value object. It is used to display or input a binary value.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Objects › Local › Misc

##### `FB_BA_Cal` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aDateList` | `T_BA_CalendarDateList` | — |
| `bWriteDateList` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresentValue` | `BOOL` | Current binary output value of the object. Inputs CONSTANT PERSISTENT |
| `aDateList` | `T_BA_CalendarDateList [} 123]` | Date list. Inputs CONSTANT |
| `bWriteDateList` | `BOOL` | Enable to write to a date list. Programming |

**Description:** The function block FB_BA_Cal represents a calendar within the project structure.

##### `FB_BA_EC` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aPriority` | `ARRAY[E_BA_EventTransition.First .. E_BA_EventTransition.Last] OF UDIN` | — |
| `eEventType` | `E_BA_EventType := E_BA_EventType.eOther` | — |
| `eAlarmMode` | `E_BA_AlarmMode := E_BA_AlarmMode.Invalid` | — |
| `aAcknowledgeRequired` | `T_BA_EventTransitions` | — |

**Description:** The function block FB_BA_EC represents an event class (cf. notification class) within the project structure of TF8040. The detection of an event and the object-internal reporting (intrinsic reporting) is located in the event-enabled objects. The subsequent distribution of the events to the event clients, on the other hand, is not executed in the objects, but in the event class.

##### `FB_BA_Loop` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Activation of the function block. |
| `fSetpoint` | `REAL` | Setpoint. |
| `fCtrlVal` | `REAL` | Feedback of the control value for calculating the control |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current analog output value of the object. Inputs CONSTANT PERSISTENT |
| `eOutputUnit` | `E_BA_Unit` | Output unit. |
| `fCOVIncrement` | `REAL` | Step size of the Present Value that triggers a COV |

**Description:** The function block FB_BA_Loop represents a PID controller within the project structure of TF8040.

##### `FB_BA_SchedA` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fDefaultValue` | `REAL` | — |
| `eUnit` | `E_BA_Unit:= E_BA_Unit.Invalid` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current analog output value of the object. Inputs CONSTANT PERSISTENT |
| `fDefaultValue` | `REAL` | Value that is assumed if all 16 priorities of the Priority array |

**Description:** The function block FB_BA_SchedA represents an analog scheduler within the project structure of TF8040.

##### `FB_BA_SchedB` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bDefaultValue` | `BOOL` | — |
| `sInactiveText` | `T_BA_StateText` | — |
| `sActiveText` | `T_BA_StateText` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresentValue` | `BOOL` | Current binary output value of the object. |
| `bPredictedValue` | `BOOL` | Value that is assumed after the next switching. Inputs CONSTANT PERSISTENT |
| `bDefaultValue` | `BOOL` | Value that is assumed if all 16 priorities of the Priority array |

**Description:** The FB_BA_SchedB function block represents a binary scheduler within the TF8040 project structure.

##### `FB_BA_SchedM` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aStateText` | `T_BA_StateTextArray` | — |
| `nDefaultValue` | `UDINT := 1` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nPresentValue` | `UDINT` | Current value for multi-stage outputs. Inputs CONSTANT PERSISTENT |
| `aStateText` | `T_BA_StateTextArray` | The array is used to declare the state texts of a multi-state [} 124] object. |
| `nDefaultValue` | `UDINT` | Value that is assumed if all 16 priorities of the Priority array |

**Description:** The function block FB_BA_SchedM represents a multi-state scheduler within the project structure of TF8040.

##### `FB_BA_Trend` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bTrigPgm` | `BOOL` | Trigger to start trending. |
| `nRecordCount` | `UDINT` | Number of records. |
| `nTotalRecordCount UDINT` | `Absolute number of records.` | — |
| `nBufferSize` | `UDINT` | Size of the buffer. |
| `aLogBuffer` | `ST_BA_TrendEntry` | Ring buffer for values with timestamp. |
| `bEnable` | `BOOL` | Recording enable. |
| `stStartTime` | `ST_BA_DateTime` | Start time. |
| `stStopTime` | `ST_BA_DateTime` | Stopping time. |
| `bStopOnFull` | `BOOL` | A TRUE stops the recording when the buffer is full. |
| `nLogInterval` | `UDINT` | Interval for saving the parameters. |
| `nNotificationThresho UDINT` | `Limit value at which notifications are triggered. ld` | — |
| `eLoggingType` | `E_BA_LoggingType` | Setting the type of saving. |
| `stReferencedParam ST_BA_ObjectParameter` | `Parameter object. [} 123]` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nRecordCount` | `UDINT` | — |
| `nTotalRecordCount` | `UDINT` | — |

**Description:** The function block FB_BA_Trend represents a trend within the project structure of TF8040.

##### `FB_BA_Project` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stProject` | `ST_BA_ProjectInfo` | — |
| `stOperator` | `ST_BA_ContactInfo` | — |
| `stTechnicalStaff` | `ST_BA_ContactInfo` | — |
| `stEngineer` | `ST_BA_ContactInfo` | — |

**Description:** The function block provides the ability to describe the project in more detail using optional information.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Objects › Local › Multistate

##### `FB_BA_MI` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVal` | `UDINT` | Scaled, analog input value. |
| `eRlbty` | `E_BA_Reliability` | Availability or reliability of a value. |

**Description:** The FB_BA_MI function block represents the multi-state input object with input of a raw value.

##### `FB_BA_MI_IO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDefaultValue` | `UDINT := 1` | — |

**Description:** The FB_BA_MO_IO function block represents the Multi-State-Output object for connection to the local I/O level.

##### `FB_BA_MI_Raw` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRawVal` | `UDINT` | Raw value. |

**Description:** The FB_BA_MI_Raw function block represents the Multi-State_Input object.

##### `FB_BA_MO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eRlbty` | `E_BA_Reliability` | Availability or reliability of a value. |
| `bRawOvrrd` | `BOOL` | The feedback of a switch contained in a terminal can be |

**Description:** The FB_BA_MO function block represents the Multi-State-Output object. It has a priority array and can therefore be commanded (see Commanding objects).

##### `FB_BA_MO_IO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.4.5

**Description:** The FB_BA_MO_IO function block represents the Multi-State_Output object for connection to the local I/O level. It has a priority array and can therefore be commanded (see Commanding objects).

##### `FB_BA_MO_Raw` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnFdbck` | `BOOL` | Variable for starting the monitoring of the feedback at the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nRawVal` | `UDINT` | The variable is used to link the raw value of an object with |

**Description:** The FB_BA_MO_Raw function block represents the Multi-State-Output object with output of the raw value. It has a priority array and can therefore be commanded (see Commanding Objects).

##### `FB_BA_MV` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnSfty` | `BOOL` | Enabling the "Safety" priority. |
| `nValSfty` | `UDINT` | Analog value for the "Safety" priority. |
| `bEnCrit` | `BOOL` | Enabling the "Critical" priority. |
| `nValCrit` | `UDINT` | Analog value for the "Critical" priority. |
| `bEnManLoc` | `BOOL` | Enabling the "Manual Local" priority. |
| `nValManLoc` | `UDINT` | Analog value for the "Manual Local" priority. |
| `bEnPgm` | `BOOL` | Enabling the "Program" priority. |
| `nValPgm` | `UDINT` | Analog value for the "Program" priority. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nPresentValue` | `UDINT` | Current value for multi-stage outputs. |
| `eActivePrio` | `E_BA_Priority [} 113]` | Active priority. Inputs CONSTANT PERSISTENT |
| `nDefaultValue` | `UDINT` | Value that is assumed if all 16 priorities of the Priority array |

**Description:** The FB_BA_MV function block represents the Multi-State-Value object with a priority array and can therefore be commanded (see Commanding Objects).

##### `FB_BA_MV_Op` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nValuePgm` | `UDINT` | Value of a multi-state object for the "Program" priority. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nPresentValue` | `UDINT` | Current value for multi-stage outputs. Programming Inputs CONSTANT PERSISTENT |
| `nDefaultValue` | `UDINT` | Value that is assumed if all 16 priorities of the Priority array |

**Description:** The FB_BA_MV_Op function block represents a Multi-State-Value object for displaying or entering a multi- state value.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Objects › Local › Structurize

##### `FB_BA_View` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eNodeType` | `E_BA_NodeType := E_BA_NodeType.Automatic` | — |
| `bAcknowledgeRm` | `BOOL` | — |

**Description:** The function block FB_BA_View is used to create a folder within the project structure.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Objects › Local › Structurize › FB_BA_View

##### `EventConditionCount` — §6.1.2.1.3.3.1.5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eCondition` | `E_BA_EventCondition` | State of an event. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EventConditionCoun UINT` | `This shows how many times an event has changed state. t` | — |

**Description:** This method is invoked when certain events occur. It counts state changes.

##### `OnObjectEventChange` (METHOD) — §6.1.2.1.3.3.1.5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObject` | `I_BA_EventObject` | Interface to the context object. [} 262] |
| `bNotifyParents` | `BOOL` | Enable for forwarding the event to the next higher-level |

**Description:** Programming

##### `FindObjectItf` — §6.1.2.1.3.3.1.5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObject` | `I_BA_Object` | Interface pointer to the subordinate object. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nResultIndex` | `DINT` | Index of the object found. |
| `FindObjectItf` | `BOOL` | This indicates whether an object has been found. Programming |

**Description:** This method looks up the index of a subordinate object using the interface pointer of the object.

##### `FindObjectSbj` — §6.1.2.1.3.3.1.5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSbjIdentifier` | `REFERENCE TO` | Subject identifier of the object. T_BA_MedString |
| `nSbjIndex` | `UDINT` | Index of the object. |
| `nSbjHash` | `DWORD` | Hash value of the object. |
| `/` | `Inputs/Outputs` | — |
| `nResultIndex` | `DINT` | Result index of the object to be found. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nResultIndex` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindObjectSbj` | `BOOL` | This indicates whether an object exists. |

**Description:** This method looks up the index of an object using the subject identifier.

##### `GetObject` — §6.1.2.1.3.3.1.5.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `DINT` | Index of the object to be found. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `iResult` | `I_BA_Object` | Interface pointer to the subordinate object. |
| `GetObject` | `BOOL` | This indicates whether the object searched for has been |

**Description:** This method looks up an object using its index.

##### `IterateObjects` — §6.1.2.1.3.3.1.5.1.6

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `_iNextObject` | `I_BA_Object` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IterateObjects` | `BOOL` | This indicates whether subordinate objects exist. |

**Description:** This iterates through all subordinate objects [} 32].

##### `QueryObjectSbj` — §6.1.2.1.3.3.1.5.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSubject` | `REFERENCE TO` | Subject Identifier. T_BA_MedString Programming |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `iResult` | `I_BA_Object` | Interface pointer to the subordinate object. |
| `QueryObjectSbj` | `BOOL` | This indicates whether subordinate objects exist. |

**Description:** Determines a subordinate object [} 32] that matches the specified subject identifier. The query works recursively and is not limited to direct child objects.

##### `ResolveObject` — §6.1.2.1.3.3.1.5.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sObjectPath` | `T_MaxString` | Object path to be resolved. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `iResult` | `I_BA_Object` | Interface pointer to the last object read. |
| `ResolveObject` | `BOOL` | This indicates whether objects in relative paths exist. |

**Description:** This resolves an object path that is relative to the current instance.

#### Programming › PLC › Libraries › Tc3_XBA › POUs › Objects › Local › Base

##### `FB_BA_BaseAI` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fResolution` | `REAL := BA_Param.fInput_DefResolution` | — |
| `fScaleOffset` | `REAL := BA_Param.fInput_DefScaleOffset` | — |
| `bEnOutOfService` | `BOOL` | — |
| `eUnit` | `E_BA_Unit := E_BA_Unit.Invalid` | — |
| `fCOVIncrement` | `REAL := BA_Param.fDefCOVIncrement` | — |
| `stLowLimit` | `ST_BA_LimitParam` | — |
| `stHighLimit` | `ST_BA_LimitParam` | — |
| `fLimitDeadband` | `REAL := BA_Param.fDefLimitDeadband` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current analog output value of the object. Programming Inputs CONSTANT PERSISTENT |
| `fResolution [} 129]` | `REAL` | Resolution of an analog signal for scaling a measured |

**Description:** Programming

##### `FB_BA_BaseAO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnSfty` | `BOOL` | Enabling the "Safety" priority. |
| `fValSfty` | `REAL` | Analog value for the "Safety" priority. |
| `bEnCrit` | `BOOL` | Enabling the "Critical" priority. |
| `fValCrit` | `REAL` | Analog value for the "Critical" priority. |
| `bEnManLoc` | `BOOL` | Enabling the "Manual Local" priority. |
| `fValManLoc` | `REAL` | Analog value for the "Manual Local" priority. |
| `bEnPgm` | `BOOL` | Enabling the "Program" priority. |
| `fValPgm` | `REAL` | Analog value for the "Program" priority. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current analog output value of the object. |
| `eActivePrio` | `E_BA_Priority [} 113]` | Active priority. Programming Inputs CONSTANT PERSISTENT |
| `fResolution [} 129]` | `REAL` | Resolution of an analog signal for scaling a measured |

**Description:** The function block FB_BA_BaseAO represents the object of an analog output. It is the base of all analog outputs.

##### `FB_BA_BaseBI` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnOutOfService` | `BOOL` | — |
| `sInactiveText` | `T_BA_StateText` | — |
| `sActiveText` | `T_BA_StateText` | — |
| `bAlarmValue` | `BOOL := TRUE` | — |
| `ePolarity` | `E_BA_Polarity := E_BA_Polarity.eNormal` | — |
| `nStateChangeCount` | `UDINT` | — |
| `nActiveTimeElapsed` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresentValue` | `BOOL` | Current binary output value of the object. Inputs CONSTANT PERSISTENT |
| `bEnOutOfService` | `BOOL` | This variable sets an object out of service. It is thus |

**Description:** The function block FB_BA_BaseBI generates a binary input object. It is the base of all binary input objects.

##### `FB_BA_BaseBO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnSfty` | `BOOL` | Enabling the "Safety" priority. |
| `bValSfty` | `BOOL` | Binary value for the "Safety" priority. |
| `bEnCrit` | `BOOL` | Enabling the "Critical" priority. |
| `bValCrit` | `BOOL` | Binary value for the "Critical" priority. |
| `bEnManLoc` | `BOOL` | Enabling the "Manual Local" priority. |
| `bValManLoc` | `BOOL` | Binary value for the "Manual Local" priority. |
| `bEnPgm` | `BOOL` | Enabling the "Program" priority. |
| `bValPgm` | `BOOL` | Binary value for the "Program" priority. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresentValue` | `BOOL` | Current binary output value of the object. |
| `eActivePrio` | `E_BA_Priority [} 113]` | Active priority. Programming Inputs CONSTANT PERSISTENT |
| `nMinimumOffTime` | `UDINT` | Minimum time [s] in which the Present Value should |

**Description:** The function block FB_BA_BaseBO represents a binary output object. It is the base for all other binary outputs.

##### `FB_BA_BaseMI` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMappingMode` | `E_BA_ByteMappingMode := E_BA_ByteMappingMode.eIndex1N` | — |
| `bEnOutOfService` | `BOOL` | — |
| `aStateText` | `T_BA_StateTextArray` | — |
| `aAlarmValues` | `ARRAY[1 .. BA_Param.nMultistate_StateCount] OF UDINT` | — |
| `aFaultValues` | `ARRAY[1 .. BA_Param.nMultistate_StateCount] OF UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nPresentValue` | `UDINT` | Analog output value. Programming Inputs CONSTANT PERSISTENT |
| `eMappingMode` | `E_BA_ByteMappingMode` | Mode for configuring the terminal link. |
| `bEnOutOfService` | `BOOL` | This variable sets an object out of service. It is thus |

**Description:** The FB_BA_BaseMI function block generates a multi-state input object. It is the base of all multi-state input objects.

##### `FB_BA_BaseMO` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnSfty` | `BOOL` | Enabling the "Safety" priority. |
| `nValSfty` | `UDINT` | Analog value for the "Safety" priority. |
| `nValCrit` | `UDINT` | Analog value for the "Critical" priority. |
| `bEnCrit` | `BOOL` | Enabling the "Critical" priority. |
| `bEnManLoc` | `BOOL` | Enabling the "Manual Local" priority. |
| `nValManLoc` | `UDINT` | Analog value for the "Manual Local" priority. |
| `bEnPgm` | `BOOL` | Enabling the "Program" priority. |
| `nValPgm` | `UDINT` | Analog value for the "Program" priority. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nPresentValue` | `UDINT` | Analog output value. |
| `eActivePrio` | `E_BA_Priority [} 113]` | Active priority. Programming Inputs CONSTANT PERSISTENT |
| `eMappingMode` | `E_BA_ByteMappingMode` | Mode for configuring the terminal link. |
| `eFeedbackMapping E_BA_ByteMappingMode` | `Structure for mapping the feedback inputs. Mode` | — |
| `nDefaultValue` | `UDINT` | Value that is assumed if all 16 priorities of the Priority array |

**Description:** The function block FB_BA_BaseMO represents a multi-level output.

##### `FB_BA_BaseSched` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nPredictTime` | `UDINT` | — |
| `aWeek` | `T_BA_SchedWeek` | — |
| `aCalendar` | `T_BA_SchedCalendar` | — |
| `aException` | `T_BA_SchedExceptionList` | — |
| `bWriteWeekly` | `BOOL` | — |
| `bWriteCalendar` | `BOOL` | — |
| `bWriteException` | `BOOL` | — |

**Description:** The FB_BA_BaseSched function block forms the base for scheduler function blocks. It does not have to be called explicitly. This description serves solely to explain the inheritance structure of the scheduler function blocks.

##### `FB_BA_BaseStateMV` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eStateTextDeterminationMode` | `E_BA_NodeTypeTarget := E_BA_NodeTypeTarget.eFunction` | — |
| `aStateText` | `T_BA_StateTextArray` | — |

**Description:** The FB_BA_BaseStateMV function block enables states to be displayed. Each referenced object is seen as state. It is represented as multi-state object.

##### `FB_BA_BaseStepMV` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sStep` | `STRING` | Current commanded step. |
| `bOn` | `BOOL` | Indicates operation. |

**Description:** The function block FB_BA_BaseStepMV enables the processing of steps. Each referenced object is seen as a step and can be commanded accordingly.

##### `FB_BA_ComEventObject` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eCommissioningState` | `E_BA_CommissioningState := E_BA_CommissioningState.eUnknown` | — |

**Description:** The function block FB_BA_ComEventObject contains the basic properties or functions for recording, displaying, forwarding, acknowledging and resetting the events of an object.

##### `FB_BA_EventObject` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEventDetectionEnable` | `BOOL := TRUE` | — |
| `nEventClassID` | `UDINT` | — |
| `aEventTransitionText` | `T_BA_EventTransitionText := BA_Param.aEventTransitionText` | — |
| `bAcknowledgeRm` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEvent` | `BOOL` | The variable signals an abnormal event of an object. |

**Description:** The FB_BA_EventObject function block extends the basic functionalities of a BACnet object by the properties of the object-internal notification (Intrinsic Reporting). Here, notifications and alarm messages are generated by the BACnet object itself and forwarded to a notification class.

##### `FB_BA_EventObjectEx` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eEnPlantLock` | `E_BA_LockPriority := E_BA_LockPriority.eNoLock` | — |
| `stTimeDelay` | `ST_BA_TimeDelayParam := F_BA_TimeDelay(BA_Param.nDefTimeDelay_ToAbnormal, 0)` | — |

**Description:** The FB_BA_EventObjectEx function block extends the basic functionalities of a BACnet object by the properties of the object-internal notification (Intrinsic Reporting). Here, notifications and alarm messages are generated by the BACnet object itself and forwarded to a notification class.

##### `FB_BA_Object` (FUNCTION_BLOCK) — §6.1.2.1.3.3.1.6.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nInstanceID` | `UDINT` | — |
| `sDeviceType` | `T_BA_SmallString := XBA_Globals.sPlaceholder_Empty` | — |
| `eAssignAsTrendRef` | `E_BA_AssignRefMode := E_BA_AssignRefMode.eInitByProfile` | — |
| `sObjectName` | `T_MaxString := XBA_Globals.sPlaceholder_Empty` | — |
| `sDescription` | `T_MaxString := XBA_Globals.sPlaceholder_Empty` | — |
| `sTag` | `STRING(XBA_Param.nTag_Length)` | — |

**Description:** The FB_BA_Object function block represents the basis for the development of further objects.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Air Conditioning

##### `FB_BA_FrstPrtc` — §6.1.2.2.3.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bFrst` | `BOOL` | Connection for frost events on the air and water side. |
| `nT1` | `UDINT` | Timer for restart delays [s]. Internally limited to a minimum |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOn` | `BOOL` | Frost alarm active. |
| `bHWRst` | `BOOL` | Output of a pulse for acknowledgement of the frost |

**Description:** The function block FB_BA_FrstPrtc is used for frost monitoring of a heating coil in an air conditioning system.

##### `FB_BA_HX` — §6.1.2.2.3.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fT` | `REAL` | Temperature [°C]. |
| `fHumRel` | `REAL` | Relative humidity [%] |
| `fAP` | `REAL` | Hydrostatic air pressure at 1013.25 hPa. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `fHumAbs` | `REAL` | Absolute humidity g water per kg dry air [g/Kg]. |
| `fDewPnt` | `REAL` | Dew point temperature [°C] |
| `fE` | `REAL` | Enthalpy [kJ/kg] |
| `fDHA` | `REAL` | Density of moist air ρ [kg mixture/m³] |
| `fSpecV` | `REAL` | Specific volume [m³/kg] |
| `fTWet` | `REAL` | Wet bulb temperature [°C] |

**Description:** This function block FB_BA_HX is used to calculate the dew point temperature, the specific enthalpy and the absolute humidity. The temperature, the relative humidity and the barometric air pressure are required for the calculation of these parameters. Enthalpy is a measure of the energy of a thermodynamic system.

##### `FB_BA_NgtCol` — §6.1.2.2.3.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable function block. |
| `fTOts` | `REAL` | Outside temperature [°C]. |
| `fTRm` | `REAL` | Room temperature [°C]. |
| `fSpRm` | `REAL` | Room temperature setpoint [°C]. |
| `fTOtsLoLmt` | `REAL` | Lower outside temperature limit [°C]; prevents excessive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Summer night cooling On. |

**Description:** This function block FB_BA_NgtCol is used to cool down rooms that have been heated up during the day with cool outside air at night. The summer night cooling function serves to improve the quality of the air and to save electrical energy. Electrical energy for cooling is saved during the first hours of the next summer day.

##### `FB_BA_RcvMonit` — §6.1.2.2.3.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable function block. |
| `bRst` | `BOOL` | Reset - all determined values are deleted. |
| `fContrVar` | `REAL` | Control value for the heat recovery, i.e. the actual value. |
| `fTOts` | `REAL` | Outside temperature [°C]. |
| `fTExh` | `REAL` | Exhaust air temperature [°C]. |
| `fTAftRcv` | `REAL` | Temperature [°C] after energy recovery. |
| `bSnsRcvTExh` | `BOOL` | Temperature at the measuring point after energy recovery: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bNewVal` | `BOOL` | Output trigger for new value fEffc. |
| `fEffc` | `REAL` | Efficiency. |
| `bLmtRchd` | `BOOL` | Limit value reached. |
| `bStblOp` | `BOOL` | Stable operation. |

**Description:** The function block FB_BA_RcvMonit is used to calculate the efficiency of an energy recovery system. The function block requires the following measured temperature values to calculate the efficiency (the so- called heat recovery coefficient): • Outside air temperature fTOts • Exhaust air temperature fTExh • Air temperature of the energy recovery system in the supply air duct (alternatively: in the extract air duct) fTAftRcv

##### `FB_BA_SpSupVis` — §6.1.2.2.3.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Function block enable. If bEn = FALSE, all output |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPrSpHi` | `REAL` | Output value for the upper setpoint. |
| `fPrSpLo` | `REAL` | Output value for the lower setpoint. |
| `fSpRcv` | `REAL` | Output value for the resulting heat recovery setpoint. |
| `bErr` | `BOOL` | In case of a fault, e.g. if warning stages are active, this |

**Description:** Function block FB_BA_SpSupVis for processing and checking the lower and upper setpoint of a supply air humidity or temperature control.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Domestic Hot Water

##### `FB_BA_DHW2P` — §6.1.2.2.3.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable boiler charging. |
| `fSp` | `REAL` | Service water temperature setpoint [°C]. |
| `fSpHys` | `REAL` | Hysteresis, recommended 1°K to 5°K. |
| `fTMax` | `REAL` | Maximum selection of all tank temperatures [°C]. |
| `fTMin` | `REAL` | Minimum selection of all tank temperatures [°C]. |
| `bKepFul` | `BOOL` | Control temperature selection: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLd` | `BOOL` | Enable charging mode. |
| `fSpOut` | `REAL` | Setpoint forwarding to charging circuit: |

**Description:** This function block FB_BA_DHW2P controls the charging (heating) of a hot water tank via an on-off controller. Tank heating is activated at input bEn. If tank heating is active the output bLd is TRUE. The variable fSp is used to transfer the setpoint for the service water temperature to the function block. A minimum selection is connected to the input fTMin, a maximum selection of all temperature sensors of the hot water tank is connected to the input fTMax. Due to the temperature stratification in the hot water tank, the top sensor is generally the one with the highest temperature and the bottom one with the lowest.

##### `FB_BA_LglPrev` — §6.1.2.2.3.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnLglPrev` | `BOOL` | Enabling of disinfection operation via a timer program. |
| `fTMin` | `REAL` | Minimum tank temperature [°C]. Minimum selection of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLd` | `BOOL` | Anti-legionella mode active. |
| `fSpOut` | `REAL` | Setpoint forwarding to charging circuit: fSp (input) if the function block is activated |

**Description:** This function block FB_BA_LglPrev is used for disinfection of the service water and for killing off Legionella. Disinfection mode is activated at input bEnLglPrev via a timer program. It is advisable to run the disinfection at least once per week (during the night). The temperature should be at least 70 °C. The activation interval at bEnLglPrev must be adequately long. The output bLd activates the storage tank charging. For hot water tanks with several temperature sensors, a min selection of all sensors must be connected to fTMin. If fTMin exceeds the value of fSp, a monitoring timer with a time of nTi_sec [s] is started. If the minimum tank temperature fTMin remains above fSp - fSpHys while the timer is active, the tank was heated adequately. If circulation is active, the output bLd must be linked to enabling of the circulation pump, to ensure that the water pipe within the service water system is included in the disinfection. If the temperature has fallen below fSp -fSpHys during the disinfection process, this process must be restarted and run until the time nTi has fully elapsed. If disinfection is successful, output bLd is reset. If the disinfection process was incomplete during the function block activation (bEnLglPrev), this is indicated with the output bAlm. The output must be reset with bRst.

#### (top level)

##### `Cooling medium is requested. fRmT > fSpCol` — §3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ePipeSys` | `E_BA_PipeSys [} 265]` | Pipe system (2Pipe, 4Pipe) of the plant. |
| `eMedium` | `E_BA_Medium [} 265]` | Selection of the medium for the entire two-pipe network |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnHtg` | `BOOL` | Heating controller enable. |
| `bEnCol` | `BOOL` | Cooling controller enable. |
| `eReqdMedium` | `E_BA_Medium [} 265]` | Requested medium (see Determination of needs). |
| `nRemTiChgOvrDlyH UDINT` | `Countdown [s] for switchover delay from cooling to` | — |
| `tg` | `heating.` | — |
| `nRemTiChgOvrDlyC UDINT` | `Countdown [s] for switchover delay from heating to` | — |
| `ol` | `cooling.` | — |
| `bErr` | `BOOL` | In case of a fault, e.g. if warning stages are active, this |

**Description:** Error handling

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Room Automation › Heating Cooling Functions

##### `FB_BA_RmTAdj` — §6.1.2.2.3.1.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fRmTAdj` | `REAL` | Room temperature offset value. |
| `stSp` | `ST_BA_SpRmT [} 270]` | Input structure for the setpoints. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValCorr` | `BOOL` | Autocorrection for the values was performed, see above. |
| `fPrPrtcHtg` | `REAL` | Resulting "Protection Heating" setpoint. |
| `fPrEcoHtg` | `REAL` | Resulting setpoint "Economy Heating". |
| `fPrPreCmfHtg` | `REAL` | Resulting setpoint "PreComfort Heating". |
| `fPrCmfCol` | `REAL` | Resulting setpoint "Comfort Cooling". |
| `fPrPreCmfCol` | `REAL` | Resulting setpoint "PreComfort Cooling". |
| `fPrEcoCol` | `REAL` | Resulting setpoint Economy Cooling. |
| `fPrPrtcCol` | `REAL` | Resulting setpoint "Protection Cooling". |
| `stPrSp` | `ST_BA_SpRmT [} 270]` | Consolidated output of the resulting values in a structure. |

**Description:** The function block FB_BA_RmTAdj is used for user adjustment of the room temperature setpoint. It shifts the setpoints at the input of a function block depending on an offset rRmTAdj, as shown in the following diagram. At the input fRmTAdj, the value of a resistance potentiometer or a bus-compatible field device, for example, can be used for the setpoint correction.

##### `FB_BA_SpRmT` — §6.1.2.2.3.1.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fPrtcHtg` | `REAL` | Basic setpoint "Protection Heating". |
| `fEcoHtg` | `REAL` | Basic setpoint "Economy Heating". |
| `fPreCmfHtg` | `REAL` | Basic setpoint "PreComfort Heating". |
| `fCmfHtg` | `REAL` | Basic setpoint "Comfort Heating". |
| `fCmfCol` | `REAL` | Basic setpoint "Comfort Cooling". |
| `fPreCmfCol` | `REAL` | Basic setpoint "PreComfort Cooling". |
| `fEcoCo` | `REAL` | Basic setpoint "Economy Cooling". |
| `fPrtcCol` | `REAL` | Basic setpoint "Protection Cooling". |
| `fShiftHtg` | `REAL` | Setpoint value shift "Heating". |
| `fShiftCol` | `REAL` | Setpoint value shift "Cooling". |
| `fSumCpsn` | `REAL` | Value Summer compensation. |
| `fWinCpsn` | `REAL` | Value Winter compensation. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValCorr` | `BOOL` | Autocorrection for the values was performed, see above. |
| `fPrPrtcHtg` | `REAL` | Resulting "Protection Heating" setpoint. |
| `fPrEcoHtg` | `REAL` | Resulting setpoint "Economy Heating". |
| `fPrPreCmfHtg` | `REAL` | Resulting setpoint "PreComfort Heating". |
| `fPrCmfCol` | `REAL` | Resulting setpoint "Comfort Cooling". |
| `fPrPreCmfCol` | `REAL` | Resulting setpoint "PreComfort Cooling". |
| `fPrEcoCol` | `REAL` | Resulting setpoint Economy Cooling. |
| `fPrPrtcCol` | `REAL` | Resulting setpoint "Protection Cooling". |
| `stPrSp` | `ST_BA_SpRmT [} 270]` | Consolidated output of the resulting values in a structure. |

**Description:** The function block FB_BA_SpRmT assigns setpoints for cooling and heating operation to each of the energy levels Protection, Economy, PreComfort and Comfort. The following graphics illustrates the behavior of the function block; the entered values should be regarded as examples:

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Room Automation › Lighting

##### `FB_BA_LightActrAnalog` — §6.1.2.2.3.1.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enabling the function block and activation of the function. |
| `stLightingCmd` | `ST_BA_Lighting [} 270]` | Light control telegram. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `iActlLgtVal` | `INT` | Current light value in positive integer range (0...32767). |
| `fActlLgtVal` | `REAL` | Current light value in percent [%]. |

**Description:** The function block FB_BA_LightActrAnalog is used to control an analog light actuator. The output is in 0...100% and converted accordingly in 0...32767 (positive integer range). The function block is thus suitable for use with the dimmer terminals (e.g. KL2751, KL2761).

##### `FB_BA_LightingEvt` — §6.1.2.2.3.1.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | A TRUE signal on this input activates the function block |
| `stLightingTgm_N` | `ST_BA_Lighting [} 270]` | Telegram inputs. Programming |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLighting` | `ST_BA_Lighting [} 270]` | Light setting telegram. |
| `bActv` | `BOOL` | Active. |
| `stLighting` | `ST_BA_Lighting [} 270]` | Resulting telegram. |
| `nNumActvTgm` | `UINT` | Indicates which input results, e.g. if stLightingTgm_3 is |

**Description:** The function block FB_BA_LightingEvt is used to set the light value and the color temperature at any event. It can be used, for example, to switch the lights during the night watchman tour or for building cleaning.

##### `FB_BA_LightCtrl` — §6.1.2.2.3.1.3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bSwi` | `BOOL` | A short press of the button at the input deactivates the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `REAL` | Light output value, 0...100%. |
| `bAutomaticMode` | `BOOL` | The function block was activated via a rising edge at the |

**Description:** The function block is a universal block for the implementation of various lighting functions.

##### `FB_BA_LightCtrlValue` — §6.1.2.2.3.1.3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | A TRUE signal at this input enables the function block. If |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOnValue` | `REAL` | Determined switch-on value. If the function block is not |

**Description:** This function block is used to determine a switch-on value (0..100%) for constant light regulation so that it starts with a brightness close to the setpoint and the light does not have to be adjusted for a long time.

##### `FB_BA_LightCtrlConst` — §6.1.2.2.3.1.3.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bSwi` | `BOOL` | If the constant light control function block is not yet active, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `REAL` | Light output value, 0...100%. |
| `bControlMode` | `BOOL` | Indicates whether the function block is operating in control |

**Description:** This function block is a constant light regulation with manual override.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Room Automation › Sun Protection

##### `FB_BA_BldPosEntry` — §6.1.2.2.3.1.3.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSunElv1` | `REAL` | Position of the sun at the first interpolation point [°] (0...90). |
| `fPos1` | `REAL` | Blind position (degree of closure) at the first interpolation |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stBldPosTab` | `ST_BA_BldPosTab [} 271]` | Transfer structure of the interpolation points. |
| `bErr` | `BOOL` | In case of a fault, e.g. if warning stages are active, this |

**Description:** The function block FB_BA_BldPosEntry serves for the input of interpolation points for the function block FB_BARSunProtectionEx [} 395], if this should be operated in the height positioning mode with the help of a table (see E_BARPosMode).

##### `FB_BA_CalcSunPos` — §6.1.2.2.3.1.3.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fDegLngd` | `REAL` | Longitude [°]. |
| `fDegLatd` | `REAL` | Latitude [°]. |
| `tUTC` | `TIMESTRUCT` | Current time as Coordinated Universal Time. The function |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fSunAzm` | `REAL` | Sun direction (northern hemisphere: 0° north ... 90° east ... |

**Description:** The function block FB_BA_CalcSunPos is used to calculate the position of the sun by specifying the date, time, longitude and latitude.

##### `FB_BA_CalcSunriseSunset` — §6.1.2.2.3.1.3.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `fDegreeOfLongitude REAL` | `Longitude in degrees. Eastern longitudes are positive,` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `todSunrise` | `TOD` | Sunrise. Output of hour and minute. |
| `todSunset` | `TOD` | Sunset. Output of hour and minute. |
| `bErr` | `BOOL` | Result verification for the entered values. |
| `sErrDescr` | `T_MaxString` | Contains the description of the error that has the highest |

**Description:** The FB_BA_CalcSunriseSunset function block is used to calculate the sunrise and sunset by specifying the longitude, latitude, reference meridian and time.

##### `FB_BA_FcdElemEntry` — §6.1.2.2.3.1.3.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `nColumn` | `UDINT` | Column index of the selected component on the facade. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aFcdElem` | `ARRAY[1..Param.nSunPrt_MaxColumnFcd, 1..Param.nSunPrt_MaxRowFcd] OF ST_BA_FcdElem` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fCnr2X` | `REAL` | X-coordinate determined for corner point 2 of the window |

**Description:** Programming

##### `FB_BA_InRngAzm` — §6.1.2.2.3.1.3.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fAzm` | `REAL` | Current azimuth angle. |
| `fSttRng` | `REAL` | Start of range [°]. |
| `fEndRng` | `REAL` | End of range [°]. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | The facade element is in the sun if the output is TRUE. |
| `bErr` | `BOOL` | In case of a fault, e.g. if warning stages are active, this |

**Description:** The function block FB_BA_InRngAzm checks whether the current azimuth angle (horizontal position of the sun) lies within the limits entered. As can be seen in the overview [} 332], the function block provides an additionally evaluation as to whether the sun protection of a window group should be activated. Therefore, all subsequent considerations apply to one window group.

##### `FB_BA_InRngElv` — §6.1.2.2.3.1.3.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fElv` | `REAL` | Current elevation angle. |
| `fLoLmt` | `REAL` | Lower limit value [°]. |
| `fHiLmt` | `REAL` | Upper limit value [°]. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | The facade element is in the sun if the output is TRUE. |
| `bErr` | `BOOL` | In case of a fault, e.g. if warning stages are active, this |

**Description:** This function block FB_BA_InRngElv checks whether the current elevation angle (vertical position of the sun) lies within the limits entered. As can be seen in the overview [} 332], the function block provides an additionally evaluation as to whether the sun protection of a window group should be activated. Therefore the observations in the remainder of the text always apply to one window group.

##### `FB_BA_RdFcdElemLst` — §6.1.2.2.3.1.3.3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStt` | `BOOL` | A TRUE edge on this input starts the reading process. |
| `sDataFile` | `STRING` | Contains the path and file name for the data file to be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aFcdElem` | `ARRAY[1..BA_Param.nSunPrt_MaxColumnFcd, 1..BA_Param.nSunPrt_MaxRowFcd] OF ST_BA_FcdElem` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is TRUE as long as elements are being read |

**Description:** With the help of the function block FB_BA_RdFcdElemLst, data for facade elements (windows) can be imported from a pre-defined Excel table in csv format into the List of facade elements [} 339]. In addition the imported data are checked for plausibility and errors are written to a log file.

##### `FB_BA_RdShdObjLst` — §6.1.2.2.3.1.3.3.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStt` | `BOOL` | A TRUE edge on this input starts the reading process. |
| `sDataFile` | `STRING` | Contains the path and file name for the data file to be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aShdObj` | `ARRAY[1..BA_Param.nSunPrt_MaxShdObj] OF ST_BA_ShdObj` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is TRUE as long as elements are being read |

**Description:** With the help of the function block FB_BA_RdShdObjLst, data for shading objects can be imported from a pre-defined Excel table in csv format into the list of shading objects [} 339]. In addition the imported data are checked for plausibility and errors are written to a log file.

##### `FB_BA_RolBldActr` — §6.1.2.2.3.1.3.3.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bEn` | `BOOL` | Enable input for the function block. As long as this input is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bUp` | `BOOL` | Roller blind control output up |
| `bDwn` | `BOOL` | Roller blind control output down |
| `fActlPos` | `READ` | Current position in percent. |
| `bRef` | `BOOL` | The roller blind is in referencing mode, i.e. the output bUp |

**Description:** The function block FB_BA_RolBldActr is used to position a roller blind via two outputs: Up and Down. The roller blind can be driven to any desired position with the positioning telegram stCmd. In addition, the positioning telegram stCmd also contains manual commands with which the roller blind can be moved individually to certain positions. These manual commands are controlled by the function block FB_BA_SunBldSwi [} 387].

##### `FB_BA_ShdCorr` — §6.1.2.2.3.1.3.3.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTiActl` | `TIMESTRUCT` | Input of the current time - local time in this case, since this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aFcdElem` | `ARRAY[1..BA_Param.nSunPrt_MaxColumnFcd, 1..BA_Param.nSunPrt_MaxRowFcd] OF ST_BA_FcdElem` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bGrpNotShdd` | `BOOL` | Is TRUE as long as the window group is not calculated as |

**Description:** The function block FB_BA_ShdCorr is used to assess the shading of a group of windows on a facade.

##### `FB_BA_ShdObjEntry` — §6.1.2.2.3.1.3.3.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `nId` | `UDINT` | Index of the selected element. This refers to the selection |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aShdObj` | `ARRAY[1..BA_Param.nSunPrt_MaxShdObj] OF ST_BA_ShdObj` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fP2x` | `REAL` | Determined X-coordinate of point 2 of the shading element |

**Description:** The function block FB_BA_ShdObjEntry serves for the administration of all shading elements in a facade, which are globally saved in a list of shading elements [} 339]. It is intended to facilitate the input of the element information - also with regard to the use of the visualization. A schematic representation of the objects with description of the coordinates is shown in Shading correction: principles and definitions [} 325].

##### `FB_BA_SunBldActr` — §6.1.2.2.3.1.3.3.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable input for the function block. As long as this input is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bUp` | `BOOL` | Control output for blind up. |
| `bDwn` | `BOOL` | Control output for blind down. |
| `fActlPos` | `REAL` | Current position in percent. |
| `fActlAngl` | `REAL` | Current lamella angle [°]. |
| `bRef` | `BOOL` | The blind is in referencing mode, i.e. the output bUp is set |

**Description:** The function block FB_BA_SunBldActr is used to position a blind via two outputs: Up and Down. The blind can be driven to any desired (height) position and lamella angle via the positioning telegram stCmd. In addition, the positioning telegram stCmd also contains manual commands with which the blind can be moved individually to certain positions. These manual commands are controlled by the function block FB_BA_SunBldSwi [} 387].

##### `FB_BA_SunBldEvt` — §6.1.2.2.3.1.3.3.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | A TRUE signal on this input activates the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output structure of the blind positions. |
| `bActv` | `BOOL` | Corresponds to the boolean value bActv in the blind |

**Description:** The function block FB_BA_SunBldEvt is used for position and angle specification at any event. It can be used, for example, in order to drive to a parking position or to drive the blind upward for maintenance.

##### `FB_BA_SunBldIcePrtc` — §6.1.2.2.3.1.3.3.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | A TRUE signal on this input activates the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output structure of the blind positions. |
| `bActv` | `BOOL` | Corresponds to the boolean value bActv in the blind |

**Description:** The function block FB_BA_SunBldIcePrtc deals with direction-independent anti-icing.

##### `FB_BA_SunBldPosDly` — §6.1.2.2.3.1.3.3.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stIn` | `ST_BA_SunBld [} 272]` | Input positioning telegram. |
| `nDly` | `UDINT` | Delay time of the active bit in the positioning telegram [ms]. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stOut` | `ST_BA_SunBld [} 272]` | Output positioning telegram. |
| `nRemTiDly` | `UDINT` | Display output for elapsed delay time [s]. |

**Description:** This function block delays changes in position based on automatic commands.

##### `FB_BA_SunBldPosHMI` — §6.1.2.2.3.1.3.3.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fPos` | `REAL` | Input of the target position. |
| `fAngl` | `REAL` | Input of the lamella angle. |
| `bStart` | `BOOL` | A rising edge at this input outputs an active telegram with |
| `stSunBld_PrioN` | `ST_BA_SunBld [} 272]` | Positioning telegrams available for selection. stSunBld_Prio1 has the highest priority and |
| `stSunBld_PrioN` | `ST_BA_SunBld [} 272]` | Positioning telegrams available for selection. stSunBld_Prio1 has the highest priority and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output telegram, for the position and angle of the lamella. |
| `bActv` | `BOOL` | Corresponds to the boolean value bActv in the blind |
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output telegram, for the position and angle of the lamella. |
| `nActvPrio` | `UDINT` | Active positioning telegram. If none is active, "0" is output. |
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output telegram, for the position and angle of the lamella. |
| `nActvPrio` | `UDINT` | Active positioning telegram. If none is active, "0" is output. |

**Description:** The function block FB_BA_SunBldPosHMI is used for position and angle specification via a user interface. With each rising edge at bStart an active positioning telegram with the target position fPos and the slat angle fAngl is output at stSunBld.

##### `FB_BA_SunBldScn` — §6.1.2.2.3.1.3.3.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | The function block has no function if this input is FALSE. 0 |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aSunBldScn` | `ARRAY[0..BA_Param.nSunPrt_MaxSunBldScn] OF ST_BA_SunBldScn` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output telegram, for the position and angle of the lamella. |
| `bActv` | `BOOL` | Corresponds to the boolean value bActv in the blind |

**Description:** The function block FB_BA_SunBldScn represents an extension of the manual operation FB_BA_SunBldSw [} 387] by a scene memory and a call function. The blind actuator FB_BA_SunBldActr [} 373] or the roller blind actuator FB_BA_RolBldActr [} 364] can thus be controlled in manual operation mode and can also drive directly to previously saved positions (scenes). Up to 21 scenes can be saved.

##### `FB_BA_SunBldSwi` — §6.1.2.2.3.1.3.3.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | The function block has no function if this input is FALSE. 0 |
| `stSunBldTgm_1…` | `ST_BA_SunBld [} 272]` | Telegram inputs. stSunBldTgm_4 |
| `stSunBldTgm_1…` | `ST_BA_SunBld [} 272]` | Telegram inputs. stSunBldTgm_8 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output telegram, for the position and angle of the lamella. |
| `bActv` | `BOOL` | Corresponds to the boolean value bActv in the blind |
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output telegram, for the position and angle of the lamella. |
| `nNumActvTgm` | `UINT` | Indicates which input is valid, e.g. if stSunBldTgm_3 is |
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output telegram, for the position and angle of the lamella. |
| `nNumActvTgm` | `UINT` | Indicates which input is valid, e.g. if stSunBldTgm_3 is |

**Description:** With the help of the function block FB_BA_SunBldSwi the blind actuator FB_BA_SunBldActr [} 373] or the roller blind actuator FB_BA_RolBldActr [} 364] can be controlled in manual operation mode. The connection takes place via the positioning telegram ST_BA_Sunbld [} 272] either directly or with an additional priority control.

##### `FB_BA_SunBldTwiLgtAuto` — §6.1.2.2.3.1.3.3.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | The function block has no function if this input is FALSE. 0 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output telegram, for the position and angle of the lamella. |
| `bActv` | `BOOL` | Corresponds to the boolean value bActv in the blind |

**Description:** The function block FB_BA_SunBldTwiLgtAuto controls the blind when the outdoor brightness has fallen below a limit value.

##### `FB_BA_SunBldWndPrtc` — §6.1.2.2.3.1.3.3.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | The function block has no function if this input is FALSE. 0 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output telegram, for the position and angle of the lamella. |
| `bActv` | `BOOL` | Corresponds to the boolean value bActv in the blind |

**Description:** The function block FB_BA_SunBldWndPrtc deals with the direction-dependent wind protection.

##### `FB_BA_SunPrtc` — §6.1.2.2.3.1.3.3.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | If this input is set to FALSE, the positioning is inactive, i.e. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Output telegram, for the position and angle of the lamella. |
| `bActv` | `BOOL` | Corresponds to the boolean value bActv in the blind |

**Description:** The function block FB_BA_SunPrtc is used for glare protection with the aid of a blind with lamellas.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › System

##### `FB_BA_CnvtTiSt` — §6.1.2.2.3.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nYear` | `WORD` | The year (1970...2106). |
| `nMonth` | `WORD` | The month (1...12). |
| `nDay` | `WORD` | The day of the month (1...31). |
| `nHour` | `WORD` | The hour (0...23). |
| `nMinute` | `WORD` | The minute (0...59). |
| `nSecond` | `WORD` | The second (0...59). |
| `nMilliseconds` | `WORD` | The millisecond (0...999). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bNewData` | `BOOL` | The output is TRUE in the cycle in which the input |

**Description:** The function block FB_BA_CnvtTiSt can be used to combine the individual components of a time structure into a single structure.

##### `FB_BA_ExtTiSt` — §6.1.2.2.3.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTi` | `TIMESTRUCT` | Input time structure. |
| `nYear` | `WORD` | The year (1970...2106). |
| `nMonth` | `WORD` | The month (1...12). |
| `nDayOfWeek` | `WORD` | The day of the week (0 (Sun)...0 (Sat)). |
| `nDay` | `WORD` | The day of the month (1...31). |
| `nHour` | `WORD` | The hour (0...23). |
| `nMinute` | `WORD` | The minute (0...59). |
| `nSecond` | `WORD` | The second (0...59). |
| `nMilliseconds` | `WORD` | The millisecond (0...999). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nYear` | `WORD` | — |
| `nMonth` | `WORD` | — |
| `nDayOfWeek` | `WORD` | — |
| `nDay` | `WORD` | — |
| `nHour` | `WORD` | — |
| `nMinute` | `WORD` | — |
| `nSecond` | `WORD` | — |
| `nMilliseconds` | `WORD` | — |

**Description:** The function block FB_BA_ExtTiSt resolves a time structure into the different components, so that it can be used for time conditions, for example.

##### `FB_BA_GetTime` — §6.1.2.2.3.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enables the function block. If bEn = TRUE, then the RTC |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bRdySysTi` | `BOOL` | The system time was read successfully from the target |

**Description:** The function block FB_BA_GetTime can be used to implement an internal clock (Real Time Clock RTC) in the TwinCAT PLC. When the function block is enabled via bEn, the RTC clock is initialized with the current NT system time. One system cycle of the CPU is used to calculate the current RTC time. The function block must be called once per PLC cycle in order for the current time to be calculated. Internally, an instance of the function blocks NT_GetTime, FB_GetTimeZoneInformation and RTC_EX2 is called in the function block. The time is output at the outputs tSysTi for the read system time and tUtcTi for the Coordinated Universal Time (UTC). This is determined internally from the system time and the time zone. If the system time and/or the time zone was entered incorrectly, the UTC time will also be wrong.

##### `FB_BA_SetTime` — §6.1.2.2.3.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSet` | `BOOL` | Activation of the function block with a rising edge. |
| `tNetId` | `T_AmsNetID` | This parameter can be used to specify the AmsNetID of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | If the function block is activated via a rising edge at bSet, |

**Description:** Programming

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Actuators

##### `FB_BA_Actuator_3Point` — §6.1.2.2.3.1.5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the function block. |
| `bAutoOpen` | `BOOL` | Command to open the actuator. |
| `bAutoClose` | `BOOL` | Command to close the actuator. |
| `nOpMode` | `UDINT` | Select operation mode (0 = Automatic, 1 = Stop (bOpen = |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOpen` | `BOOL` | Open control output. |
| `bClose` | `BOOL` | Close control output. |

**Description:** The function block FB_BA_Actuator_3Point is used to control a 3-point actuator, e.g. a 3-point damper or a 3-point valve.

##### `FB_BA_Anlg3Pnt` — §6.1.2.2.3.1.5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `fIn` | `REAL` | Setpoint for the actuator position [0…100 %]. Internally |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bCls` | `BOOL` | Output for closing the actuator. |
| `bOpn` | `BOOL` | Output for opening the actuator. |
| `fPos` | `REAL` | Current calculated actuator position [0…100 %]. |

**Description:** The function block FB_BA_Anlg3Pnt is intended for control of three-point actuators for valves or dampers. A continuous control signal for positioning an actuator is converted into the binary commands for opening and closing. If the deviation between the position setpoint fIn and the calculated actual position value fPos of the actuator is greater than the set threshold fHys / 2, the function block starts to correct the position by switching the outputs bOpn or bCls, depending on the amount of the control deviation:

##### `FB_BA_AntBlkg` — §6.1.2.2.3.1.5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | bEn is the general enable of the function block. If bEn is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bQ` | `BOOL` | Output for the blocking protection pulse. |
| `nRemOffMin` | `UDINT` | Remaining time [s] before the next pulse is issued in the |

**Description:** This function block FB_BA_AntBlkg prevents blocking of pumps or actuators after prolonged idle periods by issuing a switch-on pulse. Generally, a pulse output only occurs if the function block at bEn is enabled.

##### `FB_BA_Motor1St` — §6.1.2.2.3.1.5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable motor. |
| `bAutoOpen` | `BOOL` | Actuator request in automatic mode (nOpMode = 0). |
| `bDst` | `BOOL` | Input for collecting the possible motor malfunctions. |
| `nOpMode` | `UDINT` | Selection of the operation mode (0 = Automatic, 1 = |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Control output |

**Description:** The function block FB_BA_Motor1St is used to control 1-step motors.

##### `FB_BA_Motor2St` — §6.1.2.2.3.1.5.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable motor |
| `bAutoSt1` | `BOOL` | Request of the actuator to step 1 in automatic mode |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ1` | `BOOL` | Control output step 1 |
| `bQ2` | `BOOL` | Control output step 2 Programming |

**Description:** The function block FB_BA_Motor2St is used to control 2-step motors.

##### `FB_BA_MotorStpCtrl` — §6.1.2.2.3.1.5.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the function block. If bEn is FALSE, then |

**Description:** The function block FB_BA_MotorStpCtrl is used to control multi-stage drives. The program always starts at level 1 and, depending on the requirements of nValue, switches step by step to the next higher level. Switching up and down into the individual steps is influenced by the 3 time specifications nStepUpDly, nStepDwnDly and nChangOverTime.

#### (top level)

##### `Off` — §1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nStepUpDly` | `UDINT := 3000` | — |
| `nStepDwnDly` | `UDINT := 1000` | — |
| `nChangOverTime` | `UDINT := 100` | — |
| `nNumOfStp` | `UDINT := 4` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `nOut` | `UDINT` | Output of the currently valid step from 1 to 9. |
| `bStep0N` | `BOOL` | Output of step N depending on the stepped output signal |

**Description:** nStepDwnDly       : UDINT := 1000; nChangOverTime    : UDINT := 100;

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Actuators

##### `FB_BA_PWM` — §6.1.2.2.3.1.5.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Activation of pulse width modulation. |
| `fIn` | `REAL` | Input signal, internally limited to 0...100%. |
| `nPrd` | `UDINT` | Period time[s]. Internally limited to a minimum value of 0. |
| `nMinSwiTi` | `UDINT` | Minimum switch-on time [s], to avoid too short pulses. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | PWM output. |
| `bLmtSwiTi` | `BOOL` | Information output to indicate that the input signal is so low |

**Description:** The function block FB_BA_PWM calculates from an analog input signal fIn (0...100 %, internally fixed) and the period duration nPrd [s] a switch-on and a switch-off time fActTiOn and fActTiOff [s].

#### (top level)

##### `8 fSwiO fSwiO nDlyOf TRUE TRUE TRUE TRUE TRUE TRUE TRUE TRUE` — §8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bEn` | `BOOL` | General enable of the function block. If bEn is FALSE, all |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ0N` | `BOOL` | Shows state step 0N. The step can only be active when |

##### `Anti-Reset-Windup Controller enabled – bEn See Anti-Reset-Windup.` — §5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enabling the controller |
| `fSetpoint` | `REAL` | Setpoint of the controlled system |
| `fCtrlVal` | `REAL` | Actual value of the controlled system |
| `bEnSync` | `BOOL` | Synchronizes the controller to the value of fValSync. |
| `fValSync` | `REAL` | Synchronization value. The value fValSync is internally limited to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Control value. The value range is restricted by fYMinOutput and |

**Description:** Neutral zone

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Control

##### `FB_BA_PIDControlSync` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.2.3

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PID01` | `FB_BA_PIDControl` | — |
| `PID02` | `FB_BA_PIDControl` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bErr` | `BOOL` | The output indicates when an error has occurred during |

**Description:** The function block FB_BA_PIDControlSync is used for parameter synchronization for 2 controllers of type FB_BA_PIDControl [} 418]. This function block can be used, for example, to calibrate master controllers that are used to generate the supply air setpoints for an air conditioning system.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › General Control Functions

##### `FB_BA_DMUX_XX` — §6.1.2.2.3.1.5.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Activation of the block function |
| `nSel` | `UDINT` | Number of the output fQ01...fQ16, which is to assume the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fQ01...fQ16` | `LREAL` | Value outputs |

**Description:** Demultiplexer function blocks exist for different variable types (BOOL, INT, LREAL, REAL, USINT, UINT, UDINT and DINT) and in different output values (4, 8, 12 and 16), but they all have the same functionality. The function block FB_BA_DMUX_LR16 is described as an example.

##### `FB_BA_MMUX_XX` — §6.1.2.2.3.1.5.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bEn` | `BOOL` | Activation of the block function |
| `nSel` | `UDINT` | Selector. Internally limited to values between 0 and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | TRUE if the selector nSel matches an input selector |

**Description:** The function block FB_BA_MMUX_XX activates an input value on the output, depending on a selector and the corresponding input selector condition.

##### `FB_BA_MUX_XX` — §6.1.2.2.3.1.5.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Activation of the block function |
| `nSel` | `UDINT` | Number of the input, whose value is to be output at fQ. |
| `f01...f16` | `LREAL` | Input values to select from. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fQ` | `LREAL` | Value of the selected input. |

**Description:** Multiplexer function blocks exist for different variable types (BOOL, INT, LREAL, REAL, USINT, UINT, UDINT and DINT) and in different input values (4, 8, 12 and 16), but they all have the same functionality. The function block FB_BA_MUX_LR16 is described as an example.

##### `FB_BA_PrioSwi_XX` — §6.1.2.2.3.1.5.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn01...bEn08` | `BOOL` | Enabling the priority value |
| `fVal01...fVal08` | `LREAL` | Priority value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Output to indicate whether a priority is enabled. |
| `fVal` | `LREAL` | Output of the value of the current (highest) priority that is |

**Description:** The priority switches exist for different variable types (BOOL, INT, LREAL, REAL, USINT, UINT, UDINT and DINT) and in different output values (4, 8, 12 and 16 or 24), but they all have the same functionality. The function block FB_BA_PrioSwi_LR08 is described as an example.

##### `FB_BA_Blink` — §6.1.2.2.3.1.5.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Function block enable |
| `nTiOn` | `UDINT` | Pulse time [ms] |
| `nTiOff` | `UDINT` | Pause time [ms] |
| `bEn` | `BOOL` | Function block enable |
| `nNum` | `UDINT` | Number of aggregates |
| `bChg` | `BOOL` | Force sequence change |
| `bEn01...bEn04` | `BOOL` | Enable aggregate 1...enable aggregate 4. |
| `nActvTi01...nActvTi0 UDINT` | `Operating hours aggregate 1...operating hours aggregate` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Oscillator output |
| `nTiNextSwi` | `UDINT` | Countdown to next change of bQ [s] |

**Description:** The function block FB_BA_Blink is an oscillator with adjustable pulse and pause time, nTiOn and nTiOff [ms]. It is enabled with a TRUE signal at bEn and starts with the pulse phase.

#### (top level)

##### `4.` — §4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming EN_VAR` | `` | — |
| `bEn` | `BOOL` | Function block enable |
| `nNum` | `UDINT` | Number of aggregates |
| `bChg` | `BOOL` | Force sequence change |
| `bEn01...bEn08` | `BOOL` | Enable aggregate 1...enable aggregate 8. |
| `nActvTi01...nActvTi0 UDINT` | `Operating hours aggregate 1...operating hours aggregate` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ01...bQ04` | `BOOL` | Switches aggregate 1...4. |
| `nNextOn` | `UDINT` | Number of the aggregate that is switched on next. |
| `nNextOff` | `UDINT` | Number of the aggregate which will be switched off next. |
| `aFIFO` | `ARRAY OF UDINT` | FIFO buffer as a field. |
| `nNumOfEn` | `UDINT` | Number of devices, depending on the individual enable |

##### `8.` — §8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Function block enable |
| `bEvt01…08` | `BOOL` | Switch-on command for steps 1 to 8. |
| `nDlyOn01…08` | `UDINT` | Start-up delay for output bQ01…08 [s] |
| `nDlyOff01…08` | `UDINT` | Switch-off delay for output bQ01…08 [s] Programming |
| `bEn` | `BOOL` | Function block enable |
| `bEvt01…012` | `BOOL` | Switch-on command for steps 1 to 12. |
| `nDlyOn01…12` | `UDINT` | Start-up delay for output bQ01…12 [s] |
| `nDlyOff01…12` | `UDINT` | Switch-off delay for output bQ01…12 [s] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ01...bQ08` | `BOOL` | Switches aggregate 1...8. |
| `nNextOn` | `UDINT` | Number of the aggregate that is switched on next. |
| `nNextOff` | `UDINT` | Number of the aggregate which will be switched off next. |
| `aFIFO` | `ARRAY OF UDINT` | FIFO buffer as a field. |
| `nNumOfEn` | `UDINT` | Number of devices, depending on the individual enable |
| `bQ01...bQ08` | `BOOL` | Step 1 to 8 On |
| `bUp` | `BOOL` | Control chain is in ascending state. |
| `bDwn` | `BOOL` | Control chain is in descending state. |
| `nActvEvt` | `UDINT` | Active step, display 0...8, "0" means not active step |
| `bQ01...bQ12` | `BOOL` | Step 1 to 12 On |
| `bUp` | `BOOL` | Control chain is in ascending state. |
| `bDwn` | `BOOL` | Control chain is in descending state. |
| `nActvEvt` | `UDINT` | Active step, display 0...12, "0" means not active step |

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › General Control Functions

##### `FB_BA_FIFO04_XX` — §6.1.2.2.3.1.5.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn01…fIn04` | `: REAL;` | — |
| `aFIFO` | `ARRAY OF UDINT` | Contains the assignment table with a maximum of eight |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fVal01…fVal04` | `: REAL;` | — |
| `fVal01…fVal04` | `REAL` | Actuator setpoint, input value linked according to FIFO |

**Description:** The function block FB_BA_FIFO04_XX is used to evaluate the FiFo memory from FB_BA_FIFO04 [} 435]. The inputs are linked according to the FIFO table to the corresponding outputs of the function block FB_BA_FIFO04_BOOL or FB_BA_FIFO04_REAL.

##### `FB_BA_FIFO08_XX` — §6.1.2.2.3.1.5.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn01…fIn08` | `: REAL;` | — |
| `aFIFO` | `ARRAY OF UDINT` | Contains the assignment table with a maximum of eight |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fVal01…fVal08` | `: REAL;` | — |
| `fVal01…fVal08` | `REAL` | Actuator setpoint, input value linked according to FIFO |

**Description:** The function block FB_BA_FIFO08_XX is used to evaluate the FiFo memory from FB_BA_FIFO08 [} 436]. The inputs are linked according to the FIFO table to the corresponding outputs of the function block FB_BA_FIFO08_BOOL or FB_BA_FIFO08_REAL.

##### `LastWriterWins` — §6.1.2.2.3.1.5.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn01…fIn08` | `: REAL;` | — |
| `fIn01…fIn04` | `REAL` | Input values. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fQ` | `REAL` | Output value. |
| `nActInput` | `USINT` | Indicates which input value is currently being output. |

**Description:** The function block switches the input value fInxx at output fQ that changed last.

##### `FB_BA_Toggle` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Function block enable. |
| `bOn` | `BOOL` | Switches the output on. |
| `bOff` | `BOOL` | Switches the output off. |
| `bToggle` | `BOOL` | Negates the current output state. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Control output |

**Description:** The FB_BA_Toggle function block is used to switch an actuator on or off.

#### (top level)

##### `4 fSwiOn04 fSwiOn04 0 nDlyOff0 TRUE TRUE TRUE TRUE` — §4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bEn` | `BOOL` | General enable of the function block. If bEn is FALSE, all |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ01` | `BOOL TRUE = ON; FALSE = OFF` | Display of status step 01 nStp >= 1 |
| `bQ02` | `BOOL TRUE = ON; FALSE = OFF` | Display of status step 02 nStp >= 2 |
| `bQ03` | `BOOL TRUE = ON; FALSE = OFF` | Display of status step 03 nStp >= 3 |
| `bQ04` | `BOOL TRUE = ON; FALSE = OFF` | Display of status step 04 nStp >= 4 |
| `nStp` | `UDINT` | Shows the current step of the step switch |
| `fSwiOn` | `REAL` | Shows the next switch-on point |
| `fSwiOff` | `REAL` | Shows the next switch-off point |
| `nRemTiDlyOn` | `UDINT` | If the switch-on point for switching to the next level is met, |

**Description:** Diagram 03

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Mathematics

##### `FB_BA_EnAvrg0X` — §6.1.2.2.3.1.5.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn01…bEn04` | `: BOOL;` | — |
| `fVal01…fVal04` | `: REAL; Programming` | — |
| `bEn01…bEn04` | `BOOL` | General enable of an average calculation. If bEn0x = |
| `f01…f08` | `: REAL;` | — |
| `bEn` | `BOOL` | Activation of the block function. |
| `f01...f08` | `REAL` | Input values to be used for the calculation. |
| `nNum` | `USINT` | Number of input values to be used for the calculation. |
| `bEn` | `BOOL` | Activation of the block function. |
| `aVal` | `ARRAY OF REAL` | Field with the values to be calculated. |
| `nNumOfElem` | `UDINT` | Number of input values to be used for the calculation. |
| `fX` | `REAL` | Input value of the characteristic curve. |
| `fX0N` | `REAL` | X-value for interpolation point PN. |
| `fY0N` | `REAL` | Y-value for interpolation point PN. |
| `fYMin` | `REAL` | Lower limit of fCalcVal. The input for the lower limit is Programming |
| `fX` | `REAL` | Input value of the characteristic curve. |
| `fX0N` | `REAL` | X-value for interpolation point PN. |
| `fY0N` | `REAL` | Y-value for interpolation point PN. |
| `fYMin` | `REAL` | Lower limit of fCalcVal. The input for the lower limit is |
| `fX` | `REAL` | Input value of the characteristic curve. |
| `fX0N` | `REAL` | X-value for interpolation point PN. |
| `fY0N` | `REAL` | Y-value for interpolation point PN. |
| `fYMin` | `REAL` | Lower limit of fCalcVal. The input for the lower limit is |
| `fX` | `REAL` | Input value of the characteristic curve. |
| `fX0N` | `REAL` | X-value for interpolation point PN. |
| `fY0N` | `REAL` | Y-value for interpolation point PN. |
| `fYMin` | `REAL` | Lower limit of fCalcVal. The input for the lower limit is |
| `fX` | `REAL` | Input value of the characteristic curve. |
| `aX` | `ARRAY OF REAL` | X-value for the interpolation points. |
| `aY` | `ARRAY OF REAL` | Y-value for the interpolation points. |
| `nNumOfElem` | `DINT` | Number of interpolation points. Internally limited to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fAvgVal` | `REAL` | Calculated arithmetic average. |
| `fMinVal` | `REAL` | Smallest input value. |
| `fMaxVal` | `REAL` | Largest input value. |
| `nActvCnt` | `UDINT` | Number of input values considered for averaging. |
| `fMax` | `REAL` | Maximum value of all inputs. |
| `nMaxActv` | `USINT` | Input at which the maximum value is present. |
| `fMin` | `REAL` | Minimum value of all inputs. |
| `nMinActv` | `UDSNT` | Input at which the minimum value is present. |
| `fAvrg` | `REAL` | Average value of all inputs |
| `fSum` | `REAL` | Sum of all inputs |
| `fDiff` | `REAL` | Difference between maximum and minimum value. |
| `fMax` | `REAL` | Maximum value of all inputs. |
| `nMaxActv` | `UDINT` | Input at which the maximum value is present. |
| `fMin` | `REAL` | Minimum value of all inputs. |
| `nMinActv` | `UDINT` | Input at which the minimum value is present. |
| `fAvrg` | `REAL` | Average value of all inputs |
| `fSum` | `REAL` | Sum of all inputs |
| `fDiff` | `REAL` | Difference between maximum and minimum value. |
| `bErr` | `BOOL` | This output is switched to TRUE if the parameters entered |
| `fCalcVal` | `REAL` | Calculated output value of the characteristic curve. |
| `bErr` | `BOOL` | This output is switched to TRUE if the parameters entered |
| `fCalcVal` | `REAL` | Calculated output value of the characteristic curve. |
| `bErr` | `BOOL` | This output is switched to TRUE if the parameters entered |
| `fCalcVal` | `REAL` | Calculated output value of the characteristic curve. |
| `bErr` | `BOOL` | This output is switched to TRUE if the parameters entered |
| `fCalcVal` | `REAL` | Calculated output value of the characteristic curve. |
| `bErr` | `BOOL` | This output is switched to TRUE if the parameters entered |
| `fCalcVal` | `REAL` | Calculated output value of the characteristic curve. |
| `bErr` | `BOOL` | This output is switched to TRUE if the parameters entered |

**Description:** The function block calculates the arithmetic average from the enabled input values. The function block is available for the variants of 2, 4 and 8 input values. The following documentation refers to the FB_BA_EnAvrg04.

##### `FB_BA_TiAvrg` — §6.1.2.2.3.1.5.5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enables the function block. FALSE means that the output |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Ausgänge` | `` | — |
| `fOut` | `: REAL;` | — |
| `fMax` | `: REAL;` | — |
| `fMin` | `: REAL;` | — |
| `fOut` | `REAL` | Calculated average |
| `fMax` | `REAL` | Largest value in the FIFO buffer |
| `fMin` | `REAL` | Smallest value in the FIFO buffer |

**Description:** The function block FB_BA_TiAvrg calculates the average from a sequence of past input values.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Message

##### `FB_BA_Alm` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable function block. |
| `bIn` | `BOOL` | Message input |
| `bRstRqd` | `BOOL` | Alarm memory activation (requires acknowledgement). |
| `bRst` | `BOOL` | Edge-triggered input for resetting the alarm. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Alarm output |

**Description:** The FB_BA_Alm represents an alarm function block with switchable alarm memory and acknowledgment.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Monitoring Functions

##### `FB_BA_FdbCtrlBinary` — §6.1.2.2.3.1.5.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the function block. If bEn is FALSE, the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | bQ is used to signal that the monitoring of the feedback |

**Description:** The function block FB_BA_FdbCtrlBinary is used for feedback monitoring of an actuator by means of digital feedback. Application examples of the function block are, for example, an operation feedback monitoring, a process feedback monitoring or the run monitoring of a drive by means of limit switches.

##### `FB_BA_FixedLimitCtrl` — §6.1.2.2.3.1.5.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the function block. If bEn is FALSE, the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bHighLimit` | `BOOL` | Upper limit value reached. |
| `bLowLimit` | `BOOL` | Lower limit value reached. |
| `nRemTiDelay` | `UDINT` | Remaining time after exceeding a limit value until one of |

**Description:** This function block FB_BA_FixedLimitCtrl is used to monitor a fixed value.

##### `FB_BA_SlidingLimitCtrl` — §6.1.2.2.3.1.5.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ENDVAR` | `` | — |
| `bEn` | `BOOL` | General enable of the function block. If bEn is FALSE, the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bHighLimit` | `BOOL` | Upper limit value reached. |
| `bLowLimit` | `BOOL` | Lower limit value reached. |
| `fHighLimit` | `REAL` | Output of the upper limit value. fHighLimit = fW + (fHys / 2) |
| `fLowLimit` | `REAL` | Output of the lower limit value. fLowLimit = fW - (fHys / 2) |
| `nRemTiDelay` | `UDINT` | Remaining time after exceeding a limit value until one of |

**Description:** The function block FB_BA_SlidingLimitCtrl is used to monitor a sliding setpoint. The input bEn is used for general enabling of the function block.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Objects › Analog

##### `FB_BA_AnalogInput` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.8.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRawValue` | `INT` | Measured raw value. Inputs CONSTANT PERSISTENT |
| `stScale` | `ST_BA_ScaleIntToReal` | The structure contains the interpolation points for linear [} 282] interpolation. |
| `fOffset` | `REAL` | The offset can be used to adjust the scaled output value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Scaled output value. |
| `bHighLimit` | `BOOL` | The upper limit monitoring is active. |
| `bLowLimit` | `BOOL` | The lower limit monitoring is active. |

**Description:** The FB_BA_AnalogInput function block is used to acquire and scale analog input signals.

##### `FB_BA_AnalogOutput` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.8.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fValue` | `REAL` | Setpoint for the analog output. Inputs CONSTANT PERSISTENT |
| `stScale` | `ST_BA_ScaleRealToInt` | The structure contains the interpolation points for linear [} 283] interpolation. |
| `stHighLimit` | `ST_BA_LimitParameter` | Data and command structure for upper limit value [} 281] monitoring. |
| `stLowLimit` | `ST_BA_LimitParameter` | Data and command structure for lower limit value [} 281] monitoring. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current value -> fPresentValue = fValue + fOffset. |
| `bHighLimit` | `BOOL` | The upper limit monitoring is active. |
| `bLowLimit` | `BOOL` | The lower limit monitoring is active. |
| `nRawValue` | `INT` | Scaled value for an analog output terminal. |

**Description:** The function block FB_BA_AnalogOutput is used to control continuous actuators such as valves or dampers.

##### `FB_BA_AnalogValue` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.8.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fValue` | `REAL` | Input value from the process. Inputs CONSTANT PERSISTENT |
| `fOffset` | `REAL` | The offset can be used to adjust the scaled output value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Output value. |
| `bHighLimit` | `BOOL` | The upper limit monitoring is active. |
| `bLowLimit` | `BOOL` | The lower limit monitoring is active. |

**Description:** The function block FB_BA_AnalogValue is used to operate or monitor an analog setpoint.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › PlantControl

##### `FB_BA_Aggregate` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bFdbUp` | `BOOL` | A feedback from the aggregate is applied to the input. It is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAggregate` | `ST_BA_Aggregate` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrPriorities` | `BOOL` | The binary array outputs the enables of the priorities. Only |

**Description:** The aggregate function block represents a receive block of a step sequence controller.

##### `FB_BA_PlantControl` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | A rising edge starts the sequence. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrAggregate` | `ARRAY [1..MAX(1,BA_Param.nMaxAggregate)] OF ST_BA_Aggregate` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nStep` | `UDINT` | The variable indicates which step the sequence is in. |

**Description:** The function block realizes the sequential start-up of a plant.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Schedule › Base

##### `FB_BA_ScheduleBase` — §6.1.2.2.3.1.5.10.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aWeek` | `T_BA_ScheduleWeek` | — |
| `aException` | `T_BA_ScheduleExceptionList` | — |

**Description:** The basic schedule represents the parameterization and evaluation of the derived schedule function blocks FB_BA_ScheduleAnalog [} 499], FB_BA_ScheduleBinary [} 500] and FB_BA_ScheduleMultistate [} 498].

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Schedule

##### `FB_BA_ScheduleMultistate` — §6.1.2.2.3.1.5.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDefaultValue` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nPresentValue` | `UDINT` | Current value It consists essentially of the base class |

**Description:** The function block FB_BA_ScheduleMultistate represents a numerical schedule. It consists essentially of the base class FB_BA_ScheduleBase [} 483].

##### `FB_BA_ScheduleAnalog` — §6.1.2.2.3.1.5.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fDefaultValue` | `REAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current value It consists essentially of the base class |

**Description:** The function block FB_BA_ScheduleAnalog represents an analog schedule. It consists essentially of the base class FB_BA_ScheduleBase [} 483].

##### `FB_BA_ScheduleBinary` — §6.1.2.2.3.1.5.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bDefaultValue` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bPresentValue` | `BOOL` | Current value It consists essentially of the base class |

**Description:** The function block FB_BA_ScheduleBinary represents a binary schedule. It consists essentially of the base class FB_BA_ScheduleBase [} 483].

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Sequence

##### `FB_BA_PIDControlSequence` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nMyNum` | `UDINT` | My number in the sequence. This number may only be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSeqLink` | `ST_BA_SeqLink` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fE` | `REAL` | Displays the control deviation of the sequence controller. |

**Description:** PID sequence controller as part of a sequence.

##### `FB_BA_PIDControlSequenceLink` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Sequence control is activated when the function block is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSeqLink` | `ST_BA_SeqLink` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSeqCtrlOperational BOOL` | `The sequence control is ready to operate or is in operation.` | — |
| `nActvSeqCtrl` | `UDINT` | The output variable shows the number of the active |

**Description:** This function block represents the higher-level connection of a sequence control.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › Signal Processing

##### `FB_BA_ShortLongClick` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.12.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwi` | `BOOL` | Input signal. |
| `nSwiOvrTi` | `UDINT` | Time duration in milliseconds after which the input signal is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bShortClick` | `BOOL` | Signals a short button press for a PLC cycle. |
| `bLongClick` | `BOOL` | Signals a long button press for a PLC cycle. |

**Description:** The function block FB_BA_ShortLongClick is used to differentiate between a short and long button press.

##### `FB_BA_ShortLongPress` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.12.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwi` | `BOOL` | Input signal. |
| `nSwiOvrTi` | `UDINT` | Time duration in milliseconds after which the input signal is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bShortClick` | `BOOL` | Signals a short button press for a PLC cycle. |
| `bLongPress` | `BOOL` | Signals a long button press that is held as long as bSwi is |

**Description:** The function block FB_BA_ShortLongPress is used to differentiate between short and long button presses. In contrast to the function block FB_BA_ShortLongClick, the bLongPress output remains set as long as the bSwi input is TRUE when the long button press is detected.

##### `FB_BA_SingleDoubleClick` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.12.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwi` | `BOOL` | Input signal. |
| `nSwiOvrTi` | `UDINT` | Time duration in milliseconds within which two consecutive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSingleClick` | `BOOL` | Signals a single button press. |
| `bDoubleClick` | `BOOL` | Signals a double button press. |

**Description:** The function block FB_BA_SingleDoubleClick is used to distinguish between a single and double button press.

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal

##### `StepControl` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnUpDown` | `BOOL` | A rising edge means an increasing switch-on sequence of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nStep` | `UDINT` | Display in which step the step sequence control is located. |

**Description:** 6.1.2.2.3.1.5.13.1      FB_BA_StepCtrlAgg16

#### Programming › PLC › Libraries › POUs › FunctionBlocks › Universal › StepControl

##### `FB_BA_StepBinary` (FUNCTION_BLOCK) — §6.1.2.2.3.1.5.13.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stPriorityEn` | `ST_BA_PriorityEn [} 276]` | The command structure includes the plant enable and the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stStep` | `ST_BA_Step` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Binary [} 275]` | The command structure stCmd transmits the enables and |

**Description:** The function block is part of a step sequence control.

##### `FB_BA_StepMdlt` — §6.1.2.2.3.1.5.13.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stPriorityEn` | `ST_BA_PriorityEn [} 276]` | The command structure includes the plant enable and the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Mdlt [} 274]` | The command structure stCmd transmits the enables and |

**Description:** The function block is part of a step sequence control.

#### Programming › PLC › Libraries › POUs › Functions › Conversion

##### `F_BA_CelsiusToFahrenheit` (FUNCTION) → `LREAL` — §6.1.2.2.3.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `LREAL` | Input value in degrees Celsius. |

**Description:** The function F_BA_CelsiusToFahrenheit of return type LREAL is used to convert a temperature value in degrees Celsius to a temperature value in degrees Fahrenheit.

##### `F_BA_CelsiusToKelvin` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value in degrees Celsius. |

**Description:** The function F_BA_CelsiusToKelvin of the return type REAL is used to convert a temperature value in degrees Celsius to a temperature value in Kelvin.

##### `F_BA_CelsiusToReaumur` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value in degrees Celsius. |

**Description:** The function F_BA_CelsiusToReaumur of return type REAL is used to convert a temperature value in degrees Celsius to a temperature value in degrees Réaumur.

##### `F_BA_FahrenheitToCelsius` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value in degrees Fahrenheit. |

**Description:** The function F_BA_FahrenheitToCelsius of return type REAL is used to convert a temperature value in degrees Fahrenheit to a temperature value in degrees Celsius.

##### `F_BA_FahrenheitToKelvin` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value in degrees Fahrenheit. |

**Description:** The function F_BA_FahrenheitToKelvin of the return type REAL is used to convert a temperature value in degrees Fahrenheit to a temperature value in Kelvin.

##### `F_BA_FahrenheitToReaumur` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value in degrees Fahrenheit. |

**Description:** The function F_BA_FahrenheitToReaumur of return type REAL is used to convert a temperature value in degrees Fahrenheit to a temperature value in degrees Réaumur.

##### `F_BA_KelvinToCelsius` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value in Kelvin. |

**Description:** The function F_BA_KelvinToCelsius of return type REAL is used to convert a temperature value in Kelvin to a temperature value in degrees Celsius.

##### `F_BA_KelvinToFahrenheit` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value in Kelvin. |

**Description:** The function F_BA_KelvinToFahrenheit of the return type REAL is used to convert a temperature value in Kelvin to a temperature value in degrees Fahrenheit.

##### `F_BA_KelvinToReaumur` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value in Kelvin. |

**Description:** The function F_BA_KelvinToReaumur of return type REAL is used to convert a temperature value in Kelvin to a temperature variable in degrees Réaumur.

##### `F_BA_ReaumurToCelsius` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Fx` | `REAL` | Input value in degrees Réaumur. |

**Description:** The function F_BA_ReaumurToCelsius of return type REAL is used to convert a temperature value in degrees Réaumur to a temperature value in degrees Celsius.

##### `F_BA_ReaumurToFahrenheit` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Fx` | `REAL` | Input value in degrees Réaumur. |

**Description:** The function F_BA_ReaumurToFahrenheit of the return type REAL is used to convert a temperature value in degrees Réaumur to a temperature value in degrees Fahrenheit.

##### `F_BA_ReaumurToKelvin` (FUNCTION) → `REAL` — §6.1.2.2.3.2.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value in degrees Réaumur. |

**Description:** The function F_BA_ReaumurToKelvin of the return type REAL is used to convert a temperature value in degrees Réaumur to a temperature value in Kelvin.

#### Programming › PLC › Libraries › POUs › Functions › Universal › Priority

##### `F_BA_BinaryCrit` (FUNCTION) → `ST_BA_Binary` — §6.1.2.2.3.2.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnCrit` | `BOOL` | Enable for writing the priority "Critical" of the return type |

**Description:** The function of return type ST_BA_Binary [} 275] enables writing to the priority "Critical" of the return type of the function.

##### `F_BA_BinarySfty` (FUNCTION) → `ST_BA_Binary` — §6.1.2.2.3.2.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnSfty` | `BOOL` | Enable for writing the priority "Safety" of the return type |

**Description:** The function of return type ST_BA_Binary [} 275] enables writing to the priority "Safety" of the return type of the function.

##### `F_BA_BinarySftyCrit` (FUNCTION) → `ST_BA_Binary` — §6.1.2.2.3.2.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnSfty` | `BOOL` | Enable for writing the priority "Safety" of the return type |

**Description:** The function of return type ST_BA_Binary [} 275] enables writing to the priorities "Safety" and "Critical" of the return type of the function.

##### `F_BA_MdltCrit` (FUNCTION) → `ST_BA_Mdlt` — §6.1.2.2.3.2.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnCrit` | `BOOL` | Enable for writing the "Critical" priority of the return type |

**Description:** The function of return type ST_BA_Mdlt [} 274] enables writing to the priority "Critical" of the return type of the function.

##### `F_BA_MdltSfty` (FUNCTION) → `ST_BA_Mdlt` — §6.1.2.2.3.2.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnSfty` | `BOOL` | Enable for writing the priority "Safety" of the return type |

**Description:** The function of return type ST_BA_Mdlt [} 274] enables writing to the priority "Safety" of the return type of the function.

##### `F_BA_MdltSftyCrit` (FUNCTION) → `ST_BA_Mdlt` — §6.1.2.2.3.2.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnSfty` | `BOOL` | Enable for writing the priority "Safety" of the return type |

**Description:** The function of return type ST_BA_Mdlt [} 274] enables writing to the priorities "Safety" and "Critical" of the return type of the function.

#### Programming › PLC › Libraries › POUs › Functions › Universal › Mathematics

##### `F_BA_RoundLREAL` (FUNCTION) → `LREAL` — §6.1.2.2.3.2.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `REAL` | Floating-point number that is to be formatted. |
| `nDecimalPlace` | `REAL` | Decimal place to which to round. The maximum number of |

**Description:** The F_BA_RoundLREAL function rounds the input variable fIn to the decimal place specified by nDecimalPlace. This function can also be used for REAL data types. An instance of the function LREAL_TO_FMTSTR from the Tc2_Utilities.lib library is used internally in the function.

##### `F_BA_Scale` (FUNCTION) → `REAL` — §6.1.2.2.3.2.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value of the characteristic curve. |
| `fY02` | `REAL` | y-value of the second interpolation point. |
| `fY01` | `REAL` | y-value of the first interpolation point. |
| `fX02` | `REAL` | x-value of the second interpolation point. |
| `fX01` | `REAL` | x-value of the first interpolation point. |

**Description:** The F_BA_Scale function represents a linear interpolation with 2 interpolation points and can be used to generate characteristic curves. The characteristic curve is determined by the interpolation points [fX01/fY01] and [fX02/fY02].

##### `F_BA_ScaleLimit` (FUNCTION) → `REAL` — §6.1.2.2.3.2.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value of the characteristic curve. |
| `bLimit` | `BOOL` | A TRUE indicates that the return value F_BA_ScaleLimit is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fSLope` | `REAL` | fSlope is the result of the slope -> (fY02 - fY01) / (fX02 - |

**Description:** The F_BA_ScaleLimit function represents a linear interpolation with 2 interpolation points and can be used to generate characteristic curves. The characteristic curve is determined by the interpolation points [fX01/fY01] and [fX02/fY02].

##### `F_BA_Slope` (FUNCTION) → `REAL` — §6.1.2.2.3.2.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fY02` | `REAL` | y-value of the second interpolation point. |
| `fY01` | `REAL` | y-value of the first interpolation point. |
| `fX02` | `REAL` | x-value of the second interpolation point. |
| `fX01` | `REAL` | x-value of the first interpolation point. |

**Description:** The F_BA_Slope function is used to calculate the slope for a linear equation.

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Air conditioning equipment

##### `FB_BA_FrstPrtc` — §6.1.2.3.2.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bFrst` | `BOOL` | — |
| `udiT1_sec` | `UDINT` | — |
| `udiT2_sec` | `UDINT` | — |
| `udiAlmCnt` | `UDINT` | — |
| `bRst` | `BOOL` | — |
| `bFrst` | `Connection for frost events on the air and water side.` | — |
| `udiT1_sec` | `Timer for restart delays [s]. Internally limited to a minimum value of 0.` | — |
| `udiT2_sec` | `Timer monitoring time [s]. Internally limited to a minimum value of 0.` | — |
| `udiAlmCnt` | `Maximum number of automatic plant restarts without reset. Internally limited to values between` | — |
| `bRst` | `Resetting and acknowledgement of the frost alarm.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOn` | `BOOL` | — |
| `bHWRst` | `BOOL` | — |
| `udiRemTi1_sec` | `UDINT` | — |
| `udiRemTi2_sec` | `UDINT` | — |
| `bAlmLck` | `BOOL` | — |
| `udiStaCnt` | `UDINT` | — |
| `bOn` | `Frost alarm active.` | — |
| `bHWRst` | `Output of a pulse for acknowledgement of the frost protection hardware.` | — |
| `udiRemTi1_sec` | `Time remaining to plant restart after frost alarm.` | — |
| `udiRemTi2_sec` | `Remaining monitoring time.` | — |
| `bAlmLck` | `Alarm lock - stored alarm.` | — |
| `udiStaCnt` | `Status counter – current number of unacknowledged false starts.` | — |

**Description:** The function block is used for frost monitoring of a heating coil in an air conditioning system.

##### `FB_BA_HX` — §6.1.2.3.2.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rT` | `REAL` | — |
| `rHumRel` | `REAL` | — |
| `rAP` | `REAL` | — |
| `rT` | `Temperature [°C].` | — |
| `rHumRel` | `Relative humidity [%].` | — |
| `rAP` | `Hydrostatic air pressure at 1013.25 hPa.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrHumAbs` | `LREAL` | — |
| `lrDewPnt` | `LREAL` | — |
| `lrE` | `LREAL` | — |
| `lrDHA` | `LREAL` | — |
| `lrSpecV` | `LREAL` | — |
| `lrTWet` | `LREAL` | — |
| `lrHumAbs` | `Absolute humidity g water per kg dry air [g/Kg].` | — |
| `lrDewPnt` | `Dew point temperature [°C].` | — |
| `lrE` | `Enthalpy [kJ/kg].` | — |
| `lrDHA` | `Density of moist air ρ [kg mixture/m³].` | — |
| `lrSpecV` | `Specific volume [m³/kg].` | — |
| `lrTWet` | `Wet bulb temperature [°C].` | — |

**Description:** This function block is used to calculate the dew point temperature, the specific enthalpy and the absolute humidity. The temperature, the relative humidity and the barometric pressure are required for calculating these parameters. The enthalpy is a measure for the energy of a thermodynamic system.

##### `FB_BA_NgtCol` — §6.1.2.3.2.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rTOts` | `REAL` | — |
| `rTRm` | `REAL` | — |
| `rSpRm` | `REAL` | — |
| `rTOtsLoLmt` | `REAL` | — |
| `rTOtsHys` | `REAL` | — |
| `rTRmHys` | `REAL` | — |
| `rSwiOnDiffT` | `REAL` | — |
| `rSwiOffDiffT` | `REAL` | — |
| `bEn` | `Enable function block.` | — |
| `rTOts` | `Outside temperature [°C].` | — |
| `rTRm` | `Outside temperature [°C].` | — |
| `rSpRm` | `Room temperature setpoint.` | — |
| `rTOtsLoLmt` | `Lower outside temperature limit [°C]; prevents excessive cooling.` | — |
| `rTOtsHys` | `Hysteresis for minimum outside temperature [K]. This hysteresis, which at the lower end is` | — |
| `rTRmHys` | `Hysteresis for the room temperature [K]. This hysteresis, which at the lower end is internally` | — |
| `rSwiOnDiffT` | `Difference between the room temperature and the outside temperature, from which summer` | — |
| `rSwiOffDiffT` | `Difference between the room temperature and the outside temperature, from which summer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `bQ` | `Summer night cooling on.` | — |

**Description:** With this function block, rooms that were heated up on the day before can be cooled down during the night using cool outside air. The summer night cooling function serves to improve the quality of the air and to save electrical energy. Electrical energy for cooling is saved during the first hours of the next summer day.

##### `FB_BA_RcvMonit` — §6.1.2.3.2.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bRst` | `BOOL` | — |
| `rContrVar` | `REAL` | — |
| `rTOts` | `REAL` | — |
| `rTExh` | `REAL` | — |
| `rTAftRcv` | `REAL` | — |
| `bSnsRcvTExh` | `BOOL` | — |
| `rTIncFan` | `REAL` | — |
| `rLmtEffc` | `REAL` | — |
| `udiLmtVioDly_sec` | `DINT` | — |
| `bEn` | `Function block enable.` | — |
| `bRst` | `Reset - all determined values are deleted.` | — |
| `rContrVar` | `Control value for the heat recovery, i.e. the actual value.` | — |
| `rTOts` | `Outside temperature.` | — |
| `rTExh` | `Exhaust air temperature.` | — |
| `rTAftRcv` | `Temperature after energy recovery.` | — |
| `bSnsRcvTExh` | `Temperature at the measuring point after energy recovery: FALSE -> in inlet air duct` | — |
| `rTIncFan` | `Temperature increase due to fan.` | — |
| `rLmtEffc` | `Limit value efficiency.` | — |
| `udiLmtVioDly_sec` | `Limit violation delay [s]. Internally limited to a minimum value of 0.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bNewVal` | `BOOL` | — |
| `rEffc` | `REAL` | — |
| `bLmtRchd` | `BOOL` | — |
| `bStblOp` | `BOOL` | — |
| `bNewVal` | `Output trigger for new value rEffc.` | — |
| `rEffc` | `Efficiency` | — |
| `bLmtRchd` | `Limit value reached` | — |
| `bStblOp` | `Stable operation.` | — |

**Description:** The function block is used for calculating the efficiency of an energy recovery system.

##### `FB_BA_SpSupvis` — §6.1.2.3.2.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rSpHi` | `REAL` | — |
| `rSpLo` | `REAL` | — |
| `rSpMax` | `REAL` | — |
| `rSpMin` | `REAL` | — |
| `bActnRcv` | `BOOL` | — |
| `bSlcnSpRcv` | `BOOL` | — |
| `bEn` | `function block enable. If bEn = FALSE, all output parameters are 0.0.` | — |
| `rSpHi` | `Upper setpoint input value to be checked.` | — |
| `rSpLo` | `Lower setpoint input value to be checked.` | — |
| `rSpMax` | `Maximum setpoint.` | — |
| `rSpMin` | `Minimum setpoint.` | — |
| `bActnRcv` | `Direction of action of the downstream heat recovery.` | — |
| `bSlcnSpRcv` | `Setpoint selection of the downstream heat recovery system.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rPrSpHi` | `REAL` | — |
| `rPrSpLo` | `REAL` | — |
| `rSpRcv` | `REAL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `rPrSpHi` | `Output value for the upper setpoint.` | — |
| `rPrSpLo` | `Output value for the lower setpoint.` | — |
| `rSpRcv` | `Output value for the resulting heat recovery setpoint.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** Function block for processing and checking the lower and upper setpoint of an inlet air humidity or temperature control.

#### (top level)

##### `Cooling medium is requested rRmT > rSpCol` — §3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPipeSys` | `BOOL` | — |
| `bMedium` | `BOOL` | — |
| `bDewPnt` | `BOOL` | — |
| `rRmT` | `REAL` | — |
| `rSpHtg` | `REAL` | — |
| `rSpCol` | `REAL` | — |
| `rCtrlValHtg` | `REAL` | — |
| `rCtrlValCol` | `REAL` | — |
| `udiChgOvrDel_sec` | `UDINT` | — |
| `bPipeSys` | `In two-pipe system bPipeSys is FALSE, in four-pipe systems it is TRUE.` | — |
| `bMedium` | `Current supply of the whole two-pipe network with cooling or heating medium. If heating medium` | — |
| `bDewPnt` | `Dew-point monitor: If bDewPnt = FALSE, the cooling controller is locked.` | — |
| `rTRm` | `Room temperature.` | — |
| `rSpHtg` | `Heating setpoint.` | — |
| `rSpCol` | `Cooling setpoint.` | — |
| `rCtrlValHtg` | `Current output value of the heating controller. Used internally as switching criterion from` | — |
| `rCtrlValCol` | `Current output value of the cooling controller. Used internally as switching criterion from cooling` | — |
| `udiChgOvrDel_sec` | `Switchover delay [s] from heating to cooling or vice versa. Internally limited to a` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnHtg` | `BOOL` | — |
| `bEnCol` | `BOOL` | — |
| `udiReqdMedium` | `UDINT` | — |
| `udiRemTiChgOvrDlyHtg_sec` | `UDINT` | — |
| `udiRemTiChgOvrDlyCol_sec` | `UDINT` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `bEnHtg` | `Heating controller enable.` | — |
| `bEnCol` | `Cooling controller enable.` | — |

**Description:** Error handling

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Room automation › Heating, cooling

##### `FB_BA_RmTAdj` — §6.1.2.3.2.1.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rRmTAdj` | `REAL` | — |
| `stSp` | `ST_BA_SpRmT` | — |
| `rRmTAdj` | `Room temperature offset value.` | — |
| `stSp` | `Input structure for the setpoints (see ST_BA_SpRmT [} 697]).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValCorr` | `BOOL` | — |
| `rPrPrtcHtg` | `REAL` | — |
| `rPrEcoHtg` | `REAL` | — |
| `rPrPreCmfHtg` | `REAL` | — |
| `rPrCmfHtg` | `REAL` | — |
| `rPrPrtcCol` | `REAL` | — |
| `rPrEcoCol` | `REAL` | — |
| `rPrPreCmfCol` | `REAL` | — |
| `rPrCmfCol` | `REAL` | — |
| `stPrSp` | `ST_BA_SpRmT` | — |
| `bValCorr` | `Autocorrection for the values was performed, see above.` | — |
| `rPrPrtcHtg` | `Resulting Protection Heating setpoint.` | — |
| `rPrEcoHtg` | `Resulting Economy Heating setpoint.` | — |
| `rPrPreCmfHtg` | `Resulting PreComfort Heating setpoint.` | — |
| `rPrCmfHtg` | `Resulting Comfort Heating setpoint.` | — |
| `rPrCmfCol` | `Resulting Comfort Cooling setpoint.` | — |
| `rPrPreCmfCol` | `Resulting PreComfort Cooling setpoint.` | — |
| `rPrEcoCol` | `Resulting Economy Cooling setpoint.` | — |
| `rPrPrtcCol` | `Resulting Protection Cooling setpoint.` | — |
| `stPrSp` | `Consolidated output of the resulting values in a structure (see ST_BA_SpRmT [} 697]).` | — |

**Description:** The function block FB_BA_RmTAdj is used for user adjustment of the room temperature setpoint. It shifts the setpoints at the input of a function block depending on an offset rRmTAdj, as shown in the following diagram. At the rRmTAdj input, the value of a resistance potentiometer or a bus-capable field device can be used for the setpoint correction.

##### `FB_BA_SpRmT` — §6.1.2.3.2.1.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rSumCpsn` | `REAL` | — |
| `rWrWinCpsn` | `REAL` | — |
| `rSumCpsn` | `Summer compensation value` | — |
| `rWinCpsn` | `Winter compensation value` | — |
| `rShiftCol` | `REAL := 0` | — |
| `rShiftHtg` | `REAL := 0` | — |
| `rPrtcCol` | `REAL := 35` | — |
| `rEcoCol` | `REAL := 28` | — |
| `rPreCmfCol` | `REAL := 25` | — |
| `rCmfCol` | `REAL := 23` | — |
| `rCmfHtg` | `REAL := 21` | — |
| `rPreCmfHtg` | `REAL := 18` | — |
| `rEcoHtg` | `REAL := 14` | — |
| `rPrtcHtg` | `REAL := 6` | — |
| `rShiftCol` | `Cooling setpoint value shift.` | — |
| `rShiftHtg` | `Heating setpoint value shift.` | — |
| `rPrtcCol` | `Basic Protection Cooling setpoint.` | — |
| `rEcoCol` | `Basic Economy Cooling setpoint.` | — |
| `rPreCmfCol` | `Basic PreComfort Cooling setpoint.` | — |
| `rCmfCol` | `Basic Comfort Cooling setpoint.` | — |
| `rCmfHtg` | `Basic Comfort Heating setpoint.` | — |
| `rPreCmfHtg` | `Basic PreComfort Heating setpoint.` | — |
| `rEcoHtg` | `Basic Economy Heating setpoint.` | — |
| `rPrtcHtg` | `Basic Protection Heating setpoint.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValCorr` | `BOOL` | — |
| `rPrPrtcHtg` | `REAL` | — |
| `rPrEcoHtg` | `REAL` | — |
| `rPrPreCmfHtg` | `REAL` | — |
| `rPrCmfHtg` | `REAL` | — |
| `rPrCmfCol` | `REAL` | — |
| `rPrPreCmfCol` | `REAL` | — |
| `rPrEcoCol` | `REAL` | — |
| `rPrPrtcCol` | `REAL` | — |
| `stPrSp` | `ST_BA_SpRmT` | — |
| `bValCorr` | `Autocorrection: At least one of the resulting setpoints was adjusted such that the values continue` | — |
| `rPrPrtcHtg` | `Resulting Protection Heating setpoint.` | — |
| `rPrEcoHtg` | `Resulting Economy Heating setpoint.` | — |
| `rPrPreCmfHtg` | `Resulting PreComfort Heating setpoint.` | — |
| `rPrCmfHtg` | `Resulting Comfort Heating setpoint.` | — |
| `rPrCmfCol` | `Resulting Comfort Cooling setpoint.` | — |
| `rPrPreCmfCol` | `Resulting PreComfort Cooling setpoint.` | — |
| `rPrEcoCol` | `Resulting Economy Cooling setpoint.` | — |
| `rPrPrtcCol` | `Resulting Protection Cooling setpoint.` | — |
| `stPrSp` | `Consolidated output of the resulting values in a structure (see ST_BA_SpRmT [} 697]).` | — |

**Description:** The function block FB_BA_SpRmT assigns setpoints for cooling and heating operation to each of the energy levels Protection, Economy, PreComfort and Comfort. The following graphics illustrates the behavior of the function block; the entered values should be regarded as examples:

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Room automation › Lighting

##### `FB_BA_Toggle` — §6.1.2.3.2.1.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `bToggle` | `BOOL` | — |
| `bEn` | `Function block enable.` | — |
| `bOn` | `Switches the output on.` | — |
| `bOff` | `Switches the output off.` | — |
| `bToggle` | `Negates the current output state.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `bQ` | `Control output.` | — |

**Description:** The function block is used to switch an actuator on or off.

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Room automation › Shading

##### `FB_BA_BldPosEntry` — §6.1.2.3.2.1.2.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rSunElv1` | `REAL` | — |
| `rPos1` | `REAL` | — |
| `rSunElv2` | `REAL` | — |
| `rPos2` | `REAL` | — |
| `rSunElv3` | `REAL` | — |
| `rPos3` | `REAL` | — |
| `rSunElv4` | `REAL` | — |
| `rPos4` | `REAL` | — |
| `rSunElv1` | `Sun position at the first interpolation point (0°..90°).` | — |
| `rPos1` | `Blind position (degree of closure) at the first interpolation point (0%..100%).` | — |
| `rSunElv2` | `Sun position at the second interpolation point (0°..90°).` | — |
| `rPos2` | `Blind position (degree of closure) at the second interpolation point (0%..100%).` | — |
| `rSunElv3` | `Sun position at the third interpolation point (0°..90°).` | — |
| `rPos3` | `Blind position (degree of closure) at the third interpolation point (0%..100%).` | — |
| `rSunElv4` | `Sun position at the fourth interpolation point (0°..90°).` | — |
| `rPos4` | `Blind position (degree of closure) at the fourth interpolation point (0%..100%).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stBldPosTab` | `ST_BA_BldPosTab` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `stBldPosTab` | `Transfer structure of the interpolation points, see ST_BA_BldPosTab [} 695].` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** This function block is used for entering interpolation points for the function block FB_BA_SunPrtc [} 618], if this function block is operated in height positioning mode with the aid of a table, see E_BA_PosMod [} 694].

##### `FB_BA_CalcSunPos` — §6.1.2.3.2.1.2.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rDegLngd` | `REAL` | — |
| `rDegLatd` | `REAL` | — |
| `stUTC` | `TIMESTRUCT` | — |
| `rDegLngd` | `Longitude [°].` | — |
| `rDegLatd` | `Latitude [°].` | — |
| `stUTC` | `Input of the current time as coordinated world time (see TIMESTRUCT). The function block` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rSunAzm` | `REAL` | — |
| `rSunElv` | `REAL` | — |
| `rSunAzm` | `Direction of the sun (northern hemisphere: 0° north ... 90° east ... 180° south ... 270° west ... /` | — |
| `rSunElv` | `Height of the sun (0° horizontal - 90° vertical).` | — |

**Description:** Calculation of sun position based on the date, time, longitude and latitude.

##### `FB_BA_FcdElemEntry` — §6.1.2.3.2.1.2.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `udiColumn` | `UDINT` | — |
| `udiRow` | `UDINT` | — |
| `bWrt` | `BOOL` | — |
| `bRd` | `BOOL` | — |
| `udiGrp` | `UDINT` | — |
| `rCnr1X` | `REAL` | — |
| `rCnr1Y` | `REAL` | — |
| `rWdwWdth` | `REAL` | — |
| `rWdwHght` | `REAL` | — |
| `udiColumn` | `Column index of the selected component on the facade. This refers to the selection of a field` | — |
| `udiRow` | `ditto. row index. udiRow and udiColumn must not be zero! This is due to the field definition, which` | — |
| `bRd` | `A positive edge at this input causes the information of the selected element,` | — |
| `bWrt` | `A positive edge writes the entered and calculated values into the selected field element` | — |
| `udiGrp` | `Group membership. Internally limited to a minimum value of 0.` | — |
| `rCnr1X` | `X-coordinate of corner point 1 [m].` | — |
| `rCnr1Y` | `Y-coordinate of corner point 1 [m].` | — |
| `rWdwWdth` | `Window width [m].` | — |
| `rWdwHght` | `Window height [m].` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrFcdElem` | `ARRAY[1..Param.uiMaxColumnFcd, 1..Param.uiMaxRowFcd] OF ST_BA_FcdElem` | — |
| `arrFcdElem` | `List of facade elements (see List of facade elements [} 570]).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rCnr2X` | `REAL` | — |
| `rCnr2Y` | `REAL` | — |
| `rCnr3X` | `REAL` | — |
| `rCnr3Y` | `REAL` | — |
| `rCnr4X` | `REAL` | — |
| `rCnr4Y` | `REAL` | — |
| `bErr` | `BOOL` | — |
| `sErrDesc` | `T_MAXSTRING` | — |
| `rCnr2X` | `Calculated X-coordinate of corner point 2 of the window [m]. See "Note [} 574]" above.` | — |
| `rCnr2Y` | `Calculated Y-coordinate of corner point 2 of the window [m]. See "Note [} 574]" above.` | — |
| `rCnr3X` | `Calculated X-coordinate of corner point 3 of the window [m]. See "Note [} 574]" above.` | — |
| `rCnr3Y` | `Calculated Y-coordinate of corner point 3 of the window [m]. See "Note [} 574]" above.` | — |
| `rCnr4X` | `Calculated X-coordinate of corner point 4 of the window [m]. See "Note [} 574]" above.` | — |
| `rCnr4Y` | `Calculated Y-coordinate of corner point 4 of the window [m]. See "Note [} 574]" above.` | — |
| `bErr` | `Result verification for the entered values.` | — |
| `sErrDesc` | `Contains the error description.` | — |

**Description:** This function block serves the administration of all facade elements (windows) in a facade, which are saved globally in a list of facade elements [} 570]. It is intended to facilitate inputting element information - not least with regard to using the TC3 PLC HMI. A schematic illustration of the objects with description of the coordinates is given in Shading correction: principles and definitions [} 555].

##### `FB_BA_InRngAzm` — §6.1.2.3.2.1.2.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rAzm` | `REAL` | — |
| `rSttRng` | `REAL` | — |
| `rEndRng` | `REAL` | — |
| `rAzm` | `Current azimuth angle.` | — |
| `rSttRng` | `Start of range [°].` | — |
| `rEndRng` | `End of range [°].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `bOut` | `The facade element is in the sun if the output is TRUE.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** This function block checks whether the current azimuth angle (horizontal position of the sun) lies within the limits entered. As can be seen in the overview [} 563], the function block provides an additionally evaluation as to whether the sun shading of a window group should be activated. Therefore the observations in the remainder of the text always apply to one window group.

##### `FB_BA_InRngElv` — §6.1.2.3.2.1.2.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rElv` | `REAL` | — |
| `rLoLmt` | `REAL` | — |
| `rHiLmt` | `REAL` | — |
| `rElv` | `Current elevation angle [°].` | — |
| `rLoLmt` | `Lower limit value [°].` | — |
| `rHiLmt` | `Upper limit value [°].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `bOut` | `The facade element is in the sun` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** This function block checks whether the current angle of elevation (vertical position of the sun) lies within the limits entered. As can be seen in the overview [} 563], the function block provides an additionally evaluation as to whether the sun shading of a window group should be activated. Therefore the observations in the remainder of the text always apply to one window group.

##### `FB_BA_RdFcdElemLst` — §6.1.2.3.2.1.2.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStt` | `BOOL` | — |
| `sDataFile` | `STRING` | — |
| `sLogFile` | `STRING` | — |
| `sNetId` | `T_AmsNetId` | — |
| `bStt` | `A TRUE edge on this input starts the reading process.` | — |
| `sDataFile` | `Contains the path and file name for the data file to be opened. This must have been saved in` | — |
| `sLogFile` | `ditto. Log file for the accumulating errors. This file is overwritten each time the function block is` | — |
| `sNetId` | `A string can be entered here with the AMS Net ID of the TwinCAT computer on which the files are to` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrFcdElem` | `ARRAY[1..Param.uiMaxColumnFcd, 1..Param.uiMaxRowFcd] OF ST_BA_FcdElem` | — |
| `arrFcdElem` | `List of facade elements [} 570].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `udiAmtSetsRd` | `UDINT` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `bErrDataSet` | `BOOL` | — |
| `bBusy` | `This output is TRUE as long as elements are being read from the file.` | — |
| `udiAmtSetsRd` | `Number of data sets read.` | — |
| `bErr` | `This output is switched to TRUE, if a file write or read error has occurred.` | — |
| `sErrDescr` | `Contains the error description.` | — |
| `bErrDataSet` | `This output is set to TRUE, if the read data sets are faulty. Further details are entered in the` | — |

**Description:** With the help of this function block, data for facade elements (windows) can be imported from a pre-defined Excel table in csv format into the list of facade elements [} 570]. In addition the imported data are checked for plausibility and errors are written to a log file.

##### `FB_BA_RdShdObjLst` — §6.1.2.3.2.1.2.3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStt` | `BOOL` | — |
| `sDataFile` | `STRING` | — |
| `sLogFile` | `STRING` | — |
| `sNetId` | `T_AmsNetId` | — |
| `bStt` | `A TRUE edge on this input starts the reading process.` | — |
| `sDataFile` | `Contains the path and file name for the data file to be opened. This must have been saved in` | — |
| `sLogFile` | `ditto. Log file for the accumulating errors. This file is overwritten each time the function block is` | — |
| `sNetId` | `A string can be entered here with the AMS Net ID of the TwinCAT computer on which the files are to` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrShdObj` | `ARRAY[1..Param.uiMaxShdObj] OF ST_BA_ShdObj` | — |
| `arrShdObj` | `List of shading objects [} 570].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `udiAmtSetsRd` | `UDINT` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `bErrDataSet` | `BOOL` | — |
| `bBusy` | `This output is TRUE as long as elements are being read from the file.` | — |
| `udiAmtSetsRd` | `Number of data sets read.` | — |
| `bErr` | `This output is switched to TRUE, if a file write or read error has occurred.` | — |
| `sErrDescr` | `Contains the error description.` | — |
| `bErrDataSet` | `This output is set to TRUE, if the read data sets are faulty. Further details are entered in the` | — |

**Description:** With the help of this function block, data for shading objects can be imported from a pre-defined Excel table in csv format into the list of shading objects [} 570]. In addition the imported data are checked for plausibility and errors are written to a log file.

##### `FB_BA_RolBldActr` — §6.1.2.3.2.1.2.3.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `stSunBld` | `ST_BA_Sunblind` | — |
| `udiTiUp_ms` | `UDINT` | — |
| `udiTiDwn_ms` | `UDINT` | — |
| `bEn` | `Enable input for the function block. As long as this input is TRUE, the actuator function block accepts` | — |
| `stSunBld` | `Positioning telegram, see ST_BA_SunBld [} 698].` | — |
| `udiTiUp_ms` | `Complete time for driving up [ms].` | — |
| `udiTiDwn_ms` | `Complete time for driving down in ms.` | — |
| `bUp` | `BOOL` | — |
| `bDwn` | `BOOL` | — |
| `rActlPos` | `REAL` | — |
| `bRef` | `BOOL` | — |
| `udiRefTi_sec` | `UDINT` | — |
| `bInitRefCompl` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `bUp` | `Control output roller blind up.` | — |
| `bDwn` | `Control output roller blind down.` | — |
| `rActlPos` | `Current position in percent.` | — |
| `bRef` | `The roller blind is referencing, i.e. the output bUp is set for the complete travel-up time + 5 s. Only a` | — |
| `udiRefTi_sec` | `Referencing countdown display [s].` | — |
| `bInitRefCompl` | `Initial referencing process complete.` | — |
| `bBusy` | `A positioning or a referencing procedure is in progress.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** This function block is used to position a roller shutter over two outputs: up and down. The positioning telegram stSunBld [} 698] can be used to move the roller shutter to any position. In addition, the positioning telegram stSunBld [} 698] offers manual commands, which can be used to move the roller shutter to particular positions. These manual commands are controlled by the function block FB_BA_SunBldSwi [} 613].

##### `FB_BA_ShdCorr` — §6.1.2.3.2.1.2.3.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stTiActl` | `TIMESTRUCT` | — |
| `rFcdOrtn` | `REAL` | — |
| `rAzm` | `REAL` | — |
| `rElv` | `REAL` | — |
| `diGrpID` | `DINT` | — |
| `bSouth` | `BOOL` | — |
| `arrShdObj` | `ARRAY[1..Param.uiMaxShdObj] OF ST_BA_ShdObj` | — |
| `stTiActl` | `Input of the current time - local time in this case, since this time takes into account the shaded` | — |
| `rFcdOrtn` | `Facade orientation, see illustration above.` | — |
| `rAzm` | `Direction of the sun at the time of observation [°].` | — |
| `rElv` | `Solar altitude at the time of observation [°].` | — |
| `diGrpId` | `Window group regarded. The group 0 is reserved here for unused window elements, see` | — |
| `bSouth` | `FALSE: Calculations refer to conditions in the northern hemisphere - TRUE: In the southern` | — |
| `arrShdObj` | `List of shading objects [} 570].` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrFcdElem` | `ARRAY[1..Param.uiMaxColumnFcd, 1..Param.uiMaxRowFcd] OF ST_BA_FcdElem` | — |
| `arrFcdElem` | `List of facade elements [} 570].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bGrpNotShdd` | `BOOL` | — |
| `bFcdSunlit` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `bGrpNotShdd` | `Is TRUE as long as the window group is not calculated as shaded.` | — |
| `bFcdSunlit` | `This output is set to TRUE if the sun is shining on the facade. See description above.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** The function block is used to assess the shading of a group of windows on a facade.

##### `FB_BA_ShdObjEntry` — §6.1.2.3.2.1.2.3.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `udiId` | `UDINT` | — |
| `bRd` | `BOOL` | — |
| `bWrt` | `BOOL` | — |
| `rP1x` | `REAL` | — |
| `rP1y` | `REAL` | — |
| `rP1z` | `REAL` | — |
| `rP2y` | `REAL` | — |
| `rP3x` | `REAL` | — |
| `rP3y` | `REAL` | — |
| `rP3z` | `REAL` | — |
| `rMx` | `REAL` | — |
| `rMy` | `REAL` | — |
| `rMz` | `REAL` | — |
| `rRads` | `REAL` | — |
| `udiBegMth` | `UDINT` | — |
| `udiEndMth` | `UDINT` | — |
| `eType` | `E_BA_ShdObjType` | — |
| `udiId` | `Index of the selected element. This refers to the selection of a field element of the array saved in the` | — |
| `bRd` | `The information of the selected element, arrShdObj[udiId], is read into the function block with a positive` | — |
| `bWrt` | `A positive edge writes the values applied to the inputs rP1x to eType and the values determined and` | — |
| `rP1x` | `X-coordinate of point 1 of the shading element (rectangle) [m].` | — |
| `rP1y` | `Y-coordinate of point 1 of the shading element (rectangle) [m].` | — |
| `rP1z` | `Z-coordinate of point 1 of the shading element (rectangle) [m].` | — |
| `rP2y` | `Y-coordinate of point 2 of the shading element (rectangle) [m].` | — |
| `rP3x` | `X-coordinate of point 3 of the shading element (rectangle) [m].` | — |
| `rP3y` | `Y-coordinate of point 3 of the shading element (rectangle) [m].` | — |
| `rP3z` | `Z-coordinate of point 3 of the shading element (rectangle) [m].` | — |
| `rMx` | `X-coordinate of the center of the shading element (ball) [m].` | — |
| `rMy` | `Y-coordinate of the center of the shading element (ball) [m].` | — |
| `rMz` | `Z-coordinate of the center of the shading element (ball) [m].` | — |
| `rRads` | `Radius of the shading element (ball) [m].` | — |
| `udiBegMth` | `Beginning of the shading period (month).` | — |
| `udiEndMth` | `End of the shading period (month).` | — |
| `eType` | `Selected type of element: Rectangle or sphere (see E_BA_ShdObjType [} 694]).` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrShdObj` | `ARRAY[1..Param.uiMaxShdObj] OF ST_BA_ShdObj` | — |
| `arrShdObj` | `List of shading objects [} 570].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rP2x` | `REAL` | — |
| `rP2z` | `REAL` | — |
| `rP4x` | `REAL` | — |
| `rP4y` | `REAL` | — |
| `rP4z` | `REAL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `rP2x` | `Calculated X-coordinate of point 2 of the shading element (rectangle) [m]. See "Note [} 597]" above.` | — |
| `rP2z` | `Calculated Z-coordinate of point 2 of the shading element (rectangle) [m]. See "Note [} 597]" above.` | — |
| `rP4x` | `Calculated X-coordinate of point 4 of the shading element (rectangle) [m]. See "Note [} 597]" above.` | — |
| `rP4y` | `Calculated Y-coordinate of point 4 of the shading element (rectangle) [m]. See "Note [} 597]" above.` | — |
| `rP4z` | `Calculated Z-coordinate of point 4 of the shading element (rectangle) [m]. See "Note [} 597]" above.` | — |
| `bErr` | `Result of the plausibility check for the values entered. For a rectangle, the internal angle is 360° and` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** This function block serves for the administration of all shading elements in a facade, which are globally saved in a list of shading elements [} 570]. It is intended to facilitate the input of the element information - also with regard to the use of the visualization. A schematic representation of the objects with description of the coordinates is shown in Shading correction: Principles and definitions [} 555].

##### `FB_BA_SunBldActr` — §6.1.2.3.2.1.2.3.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `stSunbld` | `ST_BA_SunBld` | — |
| `udiTiUp_ms` | `UDINT` | — |
| `udiTiDwn_ms` | `UDINT` | — |
| `udiTurnTiUp_ms` | `UDINT` | — |
| `udiTurnTiDwn_ms` | `UDINT` | — |
| `udiBckLshTiUp_ms` | `UDINT` | — |
| `udiBckLshTiDwn_ms` | `UDINT` | — |
| `rAnglLmtUp` | `REAL` | — |
| `rAnglLmtDwn` | `REAL` | — |
| `bEn` | `Enable input for the function block. As long as this input is TRUE, the actuator function block accepts` | — |
| `stSunbld` | `Positioning telegram, (see ST_BA_SunBld [} 698]).` | — |
| `udiTiUp_ms` | `Complete time for driving up [ms].` | — |
| `udiTiDwn_ms` | `Complete time for driving down [ms].` | — |
| `udiTurnTiUp_ms` | `Time for turning the louvres in the upward direction [ms].` | — |
| `udiTurnTiDwn_ms` | `Time for turning the louvres in the downward direction [ms].` | — |
| `udiBckLshTiUp_ms` | `Time to traverse the backlash in the upward direction [ms]. This input is internally` | — |
| `udiBckLshTiDwn_ms` | `Time to traverse the backlash in the downward direction [ms]. This input is internally` | — |
| `rAnglLmtUp` | `Highest position of the louvres [°].` | — |
| `rAnglLmtDwn` | `Lowest position of the louvres [°].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bUp` | `BOOL` | — |
| `bDwn` | `BOOL` | — |
| `rActlPos` | `REAL` | — |
| `rActlAngl` | `REAL` | — |
| `bRef` | `BOOL` | — |
| `udiRefTi_sec` | `UDINT` | — |
| `bInitRefCompl` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `sErrDesc` | `T_MAXSTRING` | — |
| `bUp` | `Control output for blind up.` | — |
| `bDwn` | `Control output for blind down.` | — |
| `rActlPos` | `Current position in percent.` | — |
| `rActlAngl` | `Current louvre angle [°].` | — |
| `bRef` | `The blind is referencing, i.e. the output bUp is set for the complete travel-up time + 5s. Only a manual` | — |
| `udiRefTi_sec` | `Referencing countdown display [s].` | — |
| `bInitRefCompl` | `Initial referencing process complete.` | — |
| `bBusy` | `A positioning or a referencing procedure is in progress.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDesc` | `Contains the error description.` | — |

**Description:** This function block is used for positioning of a louvered blind via two outputs: drive up and drive down. The blind can be driven to any desired (height) position and louvre angle via the positioning telegram stSunBld [} 698]. On top of that, the positioning telegram stSunBld [} 698] also contains manual commands with which the blind can be moved individually to certain positions. These manual commands are controlled by the function block FB_BA_SunBldSwi [} 613].

##### `FB_BA_SunBldEvt` — §6.1.2.3.2.1.2.3.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rPos` | `REAL` | — |
| `rAngl` | `REAL` | — |
| `bEn` | `A TRUE signal on this input activates the function block and transfers the entered setpoint values` | — |
| `rPos` | `Height position of the blind [%] in case of activation.` | — |
| `rAngl` | `Louvre angle of the blind [°] in case of activation.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld` | — |
| `bActv` | `BOOL` | — |
| `bActv` | `Corresponds to the boolean value bActv in the blind telegram ST_BA_SunBld [} 698] and is solely` | — |
| `stSunBld` | `Output structure of the blind positions, see ST_BA_SunBld [} 698]` | — |

**Description:** This function block serves to preset the position and angle for any desired event. It can be used, for example, in order to drive to a parking position or to drive the blind upward for maintenance.

##### `FB_BA_SunBldIcePrtc` — §6.1.2.3.2.1.2.3.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rOtsT` | `REAL` | — |
| `bRainSns` | `BOOL` | — |
| `rFrstT` | `REAL` | — |
| `udiDeiceTi_sec` | `UDINT` | — |
| `rPosProt` | `REAL` | — |
| `rAnglProt` | `REAL` | — |
| `bEn` | `The function block has no function if this input is FALSE. In the positioning telegram ST_BA_Sunbld` | — |
| `rOtsT` | `Outside temperature [°C].` | — |
| `bRainSns` | `Input for a rain sensor.` | — |
| `rFrstT` | `Icing up temperature limit value [°] Celsius. This value may not be greater than 0. Otherwise an error` | — |
| `udiDeiceTi_sec` | `Time until the deicing of the blind after icing up [s]. After that the icing up alarm is reset.` | — |
| `rPosProt` | `Height position of the blind [%] in the case of protection.` | — |
| `rAnglProt` | `Louvre angle of the blind [°] in the case of protection.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld` | — |
| `bActv` | `BOOL` | — |
| `bIceAlm` | `BOOL` | — |
| `udiRemTiIceAlm_sec` | `UDINT` | — |
| `stSunBld` | `Output structure of the blind positions, see ST_BA_SunBld [} 698].` | — |
| `bActv` | `Corresponds to the boolean value bActv in the blind telegram ST_BA_SunBld [} 698] and is solely` | — |
| `bIceAlm` | `Indicates the icing up alarm.` | — |
| `udiRemTiIceAlm_sec` | `In the case of impending icing up (bIceAlm=TRUE), this second counter is set to the` | — |

**Description:** The function block FB_BA_SunBldIcePrtc deals with direction-independent anti-freezing.

##### `FB_BA_SunBldPosDly` — §6.1.2.3.2.1.2.3.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stIn` | `ST_BA_Sunblind` | — |
| `udiDly_ms` | `UDINT` | — |
| `stIn` | `Input positioning telegram, see ST_BA_SunBld [} 698].` | — |
| `udiDly_ms` | `Delay time of the active bit in the positioning telegram [ms].` | — |
| `stSunBld_Prio1` | `ST_BA_SunBld` | — |
| `stSunBld_Prio2` | `ST_BA_SunBld` | — |
| `stSunBld_Prio3` | `ST_BA_SunBld` | — |
| `stSunBld_Prio4` | `ST_BA_SunBld` | — |
| `stSunBld_Prio1` | `ST_BA_SunBld` | — |
| `stSunBld_Prio2` | `ST_BA_SunBld` | — |
| `stSunBld_Prio3` | `ST_BA_SunBld` | — |
| `stSunBld_Prio4` | `ST_BA_SunBld` | — |
| `stSunBld_Prio5` | `ST_BA_SunBld` | — |
| `stSunBld_Prio6` | `ST_BA_SunBld` | — |
| `stSunBld_Prio7` | `ST_BA_SunBld` | — |
| `stSunBld_Prio8` | `ST_BA_SunBld` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stOut` | `ST_BA_Sunblind` | — |
| `udiRemTiDly_sec` | `UDINT` | — |
| `stOut` | `Output positioning telegram, see ST_BA_SunBld [} 698].` | — |
| `udiRemTiDly_sec` | `Display output for elapsed delay time [s].` | — |
| `rPos` | `REAL` | — |
| `rAngl` | `REAL` | — |
| `bManUp` | `BOOL` | — |
| `bManDwn` | `BOOL` | — |
| `bManMod` | `BOOL` | — |
| `bActv` | `BOOL` | — |
| `stSunBld` | `ST_BA_SunBld` | — |
| `udiActvPrio` | `UDINT` | — |
| `stSunBld` | `Resulting positioning telegram.` | — |
| `udiActvPrio` | `Active positioning telegram. If none is active, "0" is output.` | — |
| `rPos` | `REAL` | — |
| `rAngl` | `REAL` | — |
| `bManUp` | `BOOL` | — |
| `bManDwn` | `BOOL` | — |
| `bManMod` | `BOOL` | — |
| `bActv` | `BOOL` | — |
| `stSunBld` | `ST_BA_SunBld` | — |
| `udiActvPrio` | `UDINT` | — |
| `stSunBld` | `Resulting positioning telegram.` | — |
| `udiActvPrio` | `Active positioning telegram. If none is active, "0" is output.` | — |

**Description:** This function block delays changes in position based on automatic commands.

##### `FB_BA_SunBldScn` — §6.1.2.3.2.1.2.3.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bUp` | `BOOL` | — |
| `bDwn` | `BOOL` | — |
| `udiSwiOvrTi_ms` | `UDINT` | — |
| `udiSlcdScn` | `UDINT` | — |
| `bClScn` | `BOOL` | — |
| `bSavScn` | `BOOL` | — |
| `rSpPos` | `REAL` | — |
| `rSpAngl` | `REAL` | — |
| `bEn` | `The function block has no function if this input is FALSE. In the positioning telegram ST_BA_Sunbld` | — |
| `bUp` | `Command input for blind up.` | — |
| `bDwn` | `Command input for blind down.` | — |
| `udiSwiOvrTi_ms` | `Time [ms] until the corresponding manual command in the positioning telegram` | — |
| `udiSlcdScn` | `Selected scene which should either be saved (bSavScn) or called (bClScn). Internally limited to` | — |
| `bClScn` | `Call selected scene.` | — |
| `bSavScn` | `Save selected scene.` | — |
| `rSpPos` | `Set position [%] that is to be saved in the selected scene. This must be linked to the actual position` | — |
| `rSpAngl` | `ditto. Louvre angle [°].` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrSunBldScn` | `ARRAY[0..Param.usiMaxSunBldScn] OF ST_BA_SunBldScn` | — |
| `arrSunBldScn` | `Table with the scene entries of the type ST_BA_SunBldScn [} 699].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld` | — |
| `bActv` | `BOOL` | — |
| `rActlScnPos` | `REAL` | — |
| `rActlScnAngl` | `REAL` | — |
| `stSunBld` | `Positioning telegram, see ST_BA_SunBld [} 698].` | — |
| `bActv` | `Corresponds to the boolean value bActv in the blind telegram ST_BA_SunBld [} 698] and is solely` | — |
| `rActlScnPos` | `Indicates the saved relative blind height position [%] for the currently selected scene.` | — |
| `rActlScnAngl` | `ditto. Louvre angle [°].` | — |

**Description:** This function block represents an extension of the manual controller FB_BA_SunBldSwi [} 613] by a scene memory and a call function. The blind control FB_BA_SunBldActr [} 600] or the roller blind control FB_BA_RolBldActr [} 592] can be active in manual mode and also directly target previously stored positions (scenes). Up to 21 scenes can be saved.

##### `FB_BA_SunBldSwi` — §6.1.2.3.2.1.2.3.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bUp` | `BOOL` | — |
| `bDwn` | `BOOL` | — |
| `udiSwiOvrTi_ms` | `UDINT` | — |
| `bEn` | `The function block has no function if this input is FALSE. In the positioning telegram ST_BA_Sunbld` | — |
| `bUp` | `Command input for blind up.` | — |
| `bDwn` | `Command input for blind down.` | — |
| `udiSwiOvrTi_ms` | `Time [ms] until the corresponding manual command in the positioning telegram` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld` | — |
| `bActv` | `BOOL` | — |
| `stSunBld` | `Positioning telegram, see ST_BA_SunBld [} 698].` | — |
| `bActv` | `Corresponds to the boolean value bActv in the blind telegram ST_BA_SunBld [} 698] and is solely` | — |

**Description:** This function block can be used to control the blind FB_BA_SunBldActr [} 600] or roller shutter FB_BA_RolBldActr [} 592] in manual mode. The connection takes place via the positioning telegram ST_BA_Sunbld [} 698] either directly or with an additional priority controller.

##### `FB_BA_SunBldTwiLgtAuto` — §6.1.2.3.2.1.2.3.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rBrtns` | `REAL` | — |
| `rActvVal` | `REAL` | — |
| `rDctvVal` | `REAL` | — |
| `udiActvDly_sec` | `UDINT` | — |
| `udiDctvDly_sec` | `UDINT` | — |
| `rPosTwiLgt` | `REAL` | — |
| `rAnglTwiLgt` | `REAL` | — |
| `bEn` | `The function block has no function if this input is FALSE. In the positioning telegram ST_BA_Sunbld` | — |
| `rBrtns` | `Outdoor brightness [lx].` | — |
| `rActvVal` | `Activation limit value [lx]. The value rActvVal is internally limited to values from 0 to rDctvVal.` | — |
| `rDctvVal` | `Deactivation limit value [lx]. Internally limited to a minimum value of 0.` | — |
| `udiActvDly_sec` | `Activation delay [s]. Internally limited to a minimum value of 0.` | — |
| `udiDctvDly_sec` | `Deactivation delay [s]. Internally limited to a minimum value of 0.` | — |
| `rPosTwiLgt` | `Vertical position of the blind [%] if the automatic twilight function is active. Internally limited to` | — |
| `rAnglTwiLgt` | `Louvre angle of the blind [°] if the automatic twilight function is active.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld` | — |
| `bActv` | `BOOL` | — |
| `udiRemTiActv_sec` | `UDINT` | — |
| `udiRemTiDctv_sec` | `UDINT` | — |
| `stSunBld` | `Output structure of the blind positions, see ST_BA_SunBld [} 698].` | — |
| `bActv` | `Corresponds to the boolean value bActv in the blind telegram ST_BA_SunBld [} 698] and is solely` | — |
| `udiRemTiActv_sec` | `Shows the time remaining [s] after falling below the switching value rActvVal until` | — |
| `udiRemTiDctv_sec` | `Shows the time remaining [s] after exceeding of the switching value rDctvVal until` | — |

**Description:** This function block controls the blind when the outdoor brightness has fallen below a limit value.

##### `FB_BA_SunBldWndPrtc` — §6.1.2.3.2.1.2.3.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rWndSpd` | `REAL` | — |
| `rWndSpdStrmOn` | `REAL` | — |
| `rWndSpdStrmOff` | `REAL` | — |
| `udiDlyStrmOn_sec` | `UDINT` | — |
| `udiDlyStrmOff_sec` | `UDINT` | — |
| `rPosProt` | `REAL` | — |
| `rAnglProt` | `REAL` | — |
| `bEn` | `The function block has no function if this input is FALSE. In the positioning telegram ST_BA_Sunbld` | — |
| `rWndSpd` | `Wind speed. The unit of entry is arbitrary, but it is important that no value is smaller than 0 and` | — |
| `rWndSpdStrmOn` | `Wind speed limit value for the activation of the storm alarm. This value may be not` | — |
| `rWndSpdStrmOff` | `Wind speed limit value for the deactivation of the storm alarm. This value may be not` | — |
| `udiDlyStrmOn_sec` | `Time delay until the storm alarm is triggered [s].` | — |
| `udiDlyStrmOff_sec` | `Time delay until the storm alarm is reset [s].` | — |
| `rPosProt` | `Height position of the blind [%] in the case of protection.` | — |
| `rAnglProt` | `Louvre angle of the blind [°] in the case of protection.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld` | — |
| `bActv` | `BOOL` | — |
| `bStrmAlm` | `BOOL` | — |
| `udiRemTiStrmDetc_sec` | `UDINT` | — |
| `udiRemTiStrmAlm_sec` | `UDINT` | — |
| `stSunBld` | `Output structure of the blind positions, see ST_BA_SunBld [} 698].` | — |
| `bActv` | `Corresponds to the boolean value bActv in the blind telegram ST_BA_SunBld [} 698] and is solely` | — |
| `bStrmAlm` | `Indicates the storm alarm.` | — |
| `udiRemTiStrmDetc_sec` | `In the non-critical case, this second counter continuously shows the alarm delay` | — |
| `udiRemTiStrmAlm_sec` | `As soon as the storm alarm is initiated, this second counter initially constantly` | — |

**Description:** The function block FB_BA_SunBldWndPrtc deals with the direction-dependent wind protection.

##### `FB_BA_SunPrtc` — §6.1.2.3.2.1.2.3.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `stUTC` | `TIMESTRUCT` | — |
| `udiPosIntval_min` | `UDINT` | — |
| `rDegLngd` | `REAL` | — |
| `rDegLatd` | `REAL` | — |
| `rFcdOrtn` | `REAL` | — |
| `rFcdAngl` | `REAL` | — |
| `rLamWdth` | `REAL` | — |
| `rLamDstc` | `REAL` | — |
| `rFixPos` | `REAL` | — |
| `rMaxLgtIndc` | `REAL` | — |
| `rWdwHght` | `REAL` | — |
| `rDstcWdwFr` | `REAL` | — |
| `stBldPosTab` | `ST_BA_BldPosTab` | — |
| `ePosMod` | `E_BA_PosMod` | — |
| `bEn` | `If this input is set to FALSE the positioning is inactive, i.e. the active bit (bActv) is reset in the` | — |
| `stUTC` | `Input of current time as coordinated world time (UTC - Universal Time Coordinated, previously` | — |
| `udiPosIntval_min` | `Positioning interval in minutes - time between two blind position outputs. Valid range: 1` | — |
| `rDegLngd` | `Longitude [°]. Valid range: - 180°...180°.` | — |
| `rDegLatd` | `Latitude [°]. Valid range: - 90°...90°.` | — |
| `rFcdOrtn` | `Facade orientation [°]:` | — |
| `rFcdAngl` | `Facade inclination [°]. See facade inclination [} 568].` | — |
| `rLamWdth` | `Width of the louvres in mm, see Louvre adjustment [} 565].` | — |
| `rLamDstc` | `Louvre spacing in mm, see Louvre adjustment [} 565].` | — |
| `rFixPos` | `Fixed (constant) shutter height [0..100%]. Applies if ePosMod = ePosModFix (see enumerator` | — |
| `rMaxLgtIndc` | `Maximum desired light incidence in mm measured from the outside of the wall (see Height` | — |
| `rWdwHght` | `Window height in mm for the calculation of the shutter height if the mode "maximum desired` | — |
| `rDstcWdwFlr` | `Distance between the floor and the window sill in mm for the calculation of the shutter height` | — |
| `stBldPosTab` | `Table of 6 interpolation points, 4 of which are parameterizable, from which a blind position is` | — |
| `ePosMod` | `Selection of the positioning mode, see enumerator E_BA_PosMod [} 694].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld` | — |
| `bActv` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `sErrorDescr` | `T_MAXSTRING` | — |
| `stSunBld` | `Output structure of the blind positions, see ST_BA_SunBld [} 698]` | — |
| `bActv` | `The function block is in active state, i.e. no error is pending, the function block is enabled, and the` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** The function block is used for glare protection with the aid of a slatted blind.

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Provision of hot water

##### `FB_BA_DHW2P` — §6.1.2.3.2.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rSp` | `REAL` | — |
| `rSpHys` | `REAL` | — |
| `rTMax` | `REAL` | — |
| `rTMin` | `REAL` | — |
| `bKepFul` | `BOOL` | — |
| `bEn` | `Enable boiler charging.` | — |
| `rSp` | `Service water temperature setpoint [°C].` | — |
| `rSpHys` | `Hysteresis, recommended 1°K to 5°K.` | — |
| `rTMax` | `Maximum selection of all tank temperatures [°C].` | — |
| `rTMin` | `Minimum selection of all tank temperatures [°C].` | — |
| `bKepFul` | `Control temperature selection:` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLd` | `BOOL` | — |
| `rSpOut` | `REAL` | — |
| `bLd` | `Enable charging mode.` | — |
| `rSpOut` | `Setpoint transfer to charging circuit:` | — |

**Description:** This function block controls the heating of a hot water tank via an on-off controller. Tank heating is activated at input bEn. If tank heating is active the output bLd is TRUE. The variable rSp is used to transfer the setpoint for the hot water temperature to the function block. At input rTMin a minimum selection of all temperature sensors for the hot water tank is connected, at input rTMax a maximum selection of all temperature sensors. Due to the thermal stratification in the hot water tank, the sensor at the top is generally the one showing the highest temperature, the one at the bottom the lowest.

##### `FB_BA_LglPrev` — §6.1.2.3.2.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnLglPrev` | `BOOL` | — |
| `rTMin` | `REAL` | — |
| `rSp` | `REAL` | — |
| `rSpHys` | `REAL` | — |
| `udiTi_sec` | `UDINT` | — |
| `bRst` | `BOOL` | — |
| `bEnLglPrev` | `Enabling of disinfection operation via a timer program.` | — |
| `rTMin` | `Minimum tank temperature [°C]. Minimum selection of temperature sensors at the top and bottom.` | — |
| `rSp` | `Setpoint for disinfection [°C].` | — |
| `rSpHys` | `Temperature difference [K] lower limit; always calculated absolute.` | — |
| `udiTi_sec` | `Monitoring period [s].` | — |
| `bRst` | `Resetting of the legionella alarm` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLd` | `BOOL` | — |
| `rSpOut` | `REAL` | — |
| `udiRTi` | `UDINT` | — |
| `udiSta` | `UDINT` | — |
| `bLd` | `Anti-legionella mode active.` | — |
| `rSpOut` | `Setpoint transfer to charging circuit:` | — |
| `udiRTi` | `Disinfection mode timer countdown.` | — |
| `udiSta` | `Disinfection program status:` | — |
| `bAlm` | `The temperature setpoint was not reached consistently over via the interval udiTi_sec, so that` | — |

**Description:** This function block is used for disinfection of the service water and for killing off Legionella. Disinfection mode is activated at input bEnLglPrev via a timer program. It is advisable to run the disinfection at least once per week (during the night). The temperature should be at least 70 °C. The activation interval at bEnLglPrev must be adequately long. The output bLd activates tank heating. For hot water tanks with several temperature sensors, a minimum selection feature for all sensors must be connected at rTMin. If rTMin exceeds the value of rSp, a monitoring timer is started with a time of udiTi_sec [s]. If the minimum tank temperature rTMin remains above rSp -rSpHys while the timer is active, the tank was heated adequately. If circulation is active, the output bLd must be linked to enabling of the circulation pump, to ensure that the water pipe within the service water system is included in the disinfection. If the temperature has fallen below rSp-rSpHys during the disinfection process, the process must be restarted and run until the time udiTi_sec has fully elapsed. If the disinfection was successful, the output bLd is reset. If the disinfection process was incomplete during the function block activation (bEnLglPrev), this is indicated with the output bAlm. The output must be reset with bRst.

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › System

##### `FB_BA_CnvtTiSt` — §6.1.2.3.2.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wYear` | `WORD` | — |
| `wMonth` | `WORD` | — |
| `wDay` | `WORD` | — |
| `wHour` | `WORD` | — |
| `wMinute` | `WORD` | — |
| `wSecond` | `WORD` | — |
| `wMilliseconds` | `WORD` | — |
| `wYear` | `The year (1970..2106).` | — |
| `wMonth` | `The month (1..12).` | — |
| `wDay` | `The day of the month (1..31).` | — |
| `wHour` | `The hour (0..23).` | — |
| `wMinute` | `The minutes (0..59).` | — |
| `wMillisecond` | `The milliseconds (0..999).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTi` | `TIMESTRUCT` | — |
| `stTi` | `Output time structure (see TIMESTRUCT)` | — |

**Description:** The function block FB_BA_CnvtTiSt can be used to consolidate the different components of a time structure.

##### `FB_BA_ExtTiSt` — §6.1.2.3.2.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stTi` | `TIMESTRUCT` | — |
| `stTi` | `Input time structure (see TIMESTRUCT)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `wYear` | `WORD` | — |
| `wMonth` | `WORD` | — |
| `wDayOfWeek` | `WORD` | — |
| `wDay` | `WORD` | — |
| `wHour` | `WORD` | — |
| `wMinute` | `WORD` | — |
| `wSecond` | `WORD` | — |
| `wMilliseconds` | `WORD` | — |
| `wYear` | `The year (1970..2106).` | — |
| `wMonth` | `The month (1..12).` | — |
| `wDayOfWeek` | `The day of the week (0(Sun)..0(Sat)).` | — |
| `wDay` | `The day of the month (1..31).` | — |
| `wHour` | `The hour (0..23).` | — |
| `wMinute` | `The minutes (0..59).` | — |
| `wSecond` | `The seconds (0..59).` | — |
| `wMilliseconds` | `The milliseconds (0..999).` | — |

**Description:** The function block FB_BA_ExtTiSt resolves a time structure into the different components, so that it can be used for time conditions, for example.

##### `FB_BA_GetTime` — §6.1.2.3.2.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `sNetId` | `T_AmsNetId` | — |
| `udiUpdRtc_sec` | `UDINT` | — |
| `bUpdRtc` | `BOOL` | — |
| `bEn` | `Enables the function block. If bEn = TRUE, then the RTC clock is initialized with the NT system time.` | — |
| `sNetId` | `This parameter can be used to specify the AmsNetID (see T_AmsNetId) of the TwinCAT computer` | — |
| `udiUpdRtc_sec` | `Time specification [s] with which the RTC clock is regularly synchronized with the NT` | — |
| `bUpdRtc` | `In parallel with the time udiUpdRtc_sec, the RTC clock can be synchronized via a positive edge at` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bRdySysTi` | `BOOL` | — |
| `bRdyTiZoInfo` | `BOOL` | — |
| `bRdyRTC` | `BOOL` | — |
| `udiRemTiUpd_sec` | `UDINT` | — |
| `stSysTi` | `TIMESTRUCT` | — |
| `stUTC` | `TIMESTRUCT` | — |
| `dtSysTi` | `DT` | — |
| `dtUTC` | `DT` | — |
| `udiCurrentTime_ms` | `UDINT` | — |
| `eTiZId` | `E_TimeZoneID` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `bRdySysTi` | `The system time was read successfully from the target system.` | — |
| `bRdyTiZoInfo` | `The additional time information (time zone, time shift relative to UTC and summer/winter` | — |
| `bRdyRTC` | `This output is set if the function block has been initialized at least once. If this output is set, then` | — |
| `udiRemTiUpd_sec` | `Countdown to next synchronization/update of the time information.` | — |
| `stSysTi` | `System time of the read target system (see TIMESTRUCT). The time structure can be resolved` | — |
| `stUTC` | `Coordinated world time (see TIMESTRUCT). This is determined internally from the system time and` | — |
| `udiCurrentTime_ms` | `Current time of day in ms.` | — |
| `eTiZId` | `Enumerator for summer/winter time information (see E_TimeZoneID).` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** With this function block an internal clock (Real Time Clock RTC) can be implemented in the TwinCAT PLC. When the function block is enabled via bEn, the RTC clock is initialized with the current NT system time. One system cycle of the CPU is used to calculate the current RTC time. The function block must be called once per PLC cycle in order for the current time to be calculated. Within the function block, an instance of the function blocks NT_GetTime, FB_GetTimeZoneInformation and RTC_EX2 is called. The time is output at the outputs stSysTi for the read system time and stUtc for the Coordinated Universal Time (UTC). This is determined internally from the system time and the time zone. If the system time and/or the time zone was entered incorrectly, the UTC time will also be wrong.

##### `FB_BA_SetTime` — §6.1.2.3.2.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSet` | `BOOL` | — |
| `sNetId` | `T_AmsNetId` | — |
| `stSysTi` | `TIMESTRUCT` | — |
| `udiTiOut_sec` | `UDINT` | — |
| `bSet` | `Activation of the function block with a rising edge.` | — |
| `sNetId` | `This parameter can be used to specify the AmsNetID of the TwinCAT computer, whose local NT` | — |
| `stSysTi` | `Structure with the new local NT system time (see TIMESTRUCT). If the time is not available as` | — |
| `udiTiOut_sec` | `Indicates the timeout time [s], which must not be exceeded during execution.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `sErrorDescr` | `T_MAXSTRING` | — |
| `bBusy` | `If the function block is activated via a rising edge at bSet, this output is set and remains set until` | — |
| `bErr` | `This output is set to TRUE, if either the system time to be transferred is incorrect or an ADS error` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** The function block FB_BA_SetTime can be used to set the local NT system time and the date for a TwinCAT system (the local NT system time is shown in the taskbar). The system time is specified via the structure stSysTi. Internally, an instance of the function block NT_SetLocalTime from the TcUtilities library is called in the function block.

##### `FB_BA_WrtPersistDat` — §6.1.2.3.2.1.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStt` | `BOOL` | — |
| `udiInitSttDly_sec` | `UDINT` | — |
| `bStt` | `A rising edge at this input starts the function block if it is not in the start-up phase.` | — |
| `udiInitSttDly_sec` | `Start-up phase after a reset or TwinCAT restart. The duration is set in seconds. Once the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `udiRemTiInitSttDly_sec` | `UDINT` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MaxString` | — |
| `bBusy` | `The function block is being executed.` | — |
| `udiRemTiInitSttDly_sec` | `Countdown of the set startup phase.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** When activated, the function block FB_BA_WrtPersistDat first saves the persistent data in the Port_xxx.bootdata file. It is not necessary to explicitly specify the port or runtime system at which the PLC is located; this is determined internally. Once the data has been written, the content of the file Port_xxx.bootdata is copied to the backup file Port_xxx.bootdata-old. Thus both files are always synchronized. In case the original file with the persistent data is not readable, the backup copy, which is then read, contains the same data.

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Universal › Actuators

##### `FB_BA_Actuator_3Point` — §6.1.2.3.2.1.5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bAutoOpen` | `BOOL` | — |
| `bAutoClose` | `BOOL` | — |
| `udiOpMode` | `UDINT` | — |
| `bEn` | `General enable of the function block.` | — |
| `bAutoOpen` | `Command to open the actuator.` | — |
| `bAutoClose` | `Command to close the actuator.` | — |
| `udiOpMode` | `Select operating mode (0 = Automatic, 1 = Stop (bOpen = bClose = FALSE), 2 = Close, 3 =` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOpen` | `BOOL` | — |
| `bClose` | `BOOL` | — |
| `bOpen` | `Open control output.` | — |
| `bClose` | `Close control output.` | — |

**Description:** The function block is used to control a 3-point actuator, e.g. a 3-point flap or a 3-point valve.

##### `FB_BA_Anlg3Pnt` — §6.1.2.3.2.1.5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rIn` | `REAL` | — |
| `rHys` | `REAL` | — |
| `udiTiCls_ms` | `UDINT` | — |
| `udiTiOpn_ms` | `UDINT` | — |
| `bRef` | `BOOL` | — |
| `rRefVal` | `REAL` | — |
| `bCloseInit` | `BOOL` | — |
| `rIn` | `Setpoint for the actuator position [0 - 100%]. Internally limited to values between 0 and 100.` | — |
| `rHys` | `Hysteresis for the actuator position [0 - 100%]. Internally limited to values between 0 and 100.` | — |
| `udiTiCls_ms` | `Run time of the actuator from open to closed [ms]. Internally limited to values between 0 and` | — |
| `udiTiOpn_ms` | `Run time of the actuator from closed to open [ms]. Internally limited to values between 0 and` | — |
| `bRef` | `Edge references the internal position memory of the drive to value of rRefVal [0 - 100%].` | — |
| `rRefVal` | `Value for referencing the actuator with bRef [0 - 100%]. Internally limited to values between 0 and` | — |
| `bCloseInit` | `If this input is TRUE, output bCls is TRUE for the time udiTiOpn_ms` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bCls` | `BOOL` | — |
| `bOpn` | `BOOL` | — |
| `rPos` | `REAL` | — |
| `bCls` | `Output for closing the actuator.` | — |
| `bOpn` | `Output for opening the actuator.` | — |
| `rPos` | `Current calculated actuator position [0 - 100%].` | — |

**Description:** The function block is intended for control of three-point actuators for valves or dampers.

##### `FB_BA_AntBlkg` — §6.1.2.3.2.1.5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bFdb` | `BOOL` | — |
| `bExe` | `BOOL` | — |
| `udiTiOffMin_sec` | `UDINT` | — |
| `udiTiImplLngt_sec` | `UDINT` | — |
| `bEn` | `Enable of the function block.` | — |
| `bFdb` | `Input for connecting the feedback signal of a motor or valve.` | — |
| `bExe` | `Rising edge forces a pulse output.` | — |
| `udiTiOffMin_sec` | `Minimum switch-off time [s]: a pulse is issued once the time udiTiOffMin_sec has elapsed` | — |
| `udiTiImplLngt_sec` | `Length of the anti-blocking protection pulse [s] at bQ.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `udiRTiOffMin_sec` | `UDINT` | — |
| `udiRTiImplLngt_sec` | `UDINT` | — |
| `bQ` | `Pulse output.` | — |
| `udiRTiOffMin_sec` | `Remaining time [s] before the next pulse is issued in the absence of movement.` | — |
| `udiTiImplLngt_sec` | `Remaining residual time [s] of the pulse at bQ.` | — |

**Description:** This function block prevents pumps or actuators from blocking after long periods without movement by outputting a switch-on pulse. The maximum duration of the standstill until a pulse is output is determined by the value of udiTiOffMin_sec. For logging the idle time, the input bFdb must be linked to the operating feedback from the aggregate. The length of the pulse is parameterized with udiTiImplLngt_sec. The input bExe should be used if the anti- blocking protection pulses are to be issued cyclically based on a schedule, rather than depending on the idle times. A rising edge at bExe immediately triggers output of a pulse to bQ. Generally, a pulse output only occurs if the function block at bEn is enabled.

##### `FB_BA_Motor1St` — §6.1.2.3.2.1.5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bAuto` | `BOOL` | — |
| `bDst` | `BOOL` | — |
| `udiOpMode` | `UDINT` | — |
| `bEn` | `Enable motor.` | — |
| `bAuto` | `Request of the actuator in automatic mode (udiOpMode = 0).` | — |
| `bDst` | `Input for collecting the possible motor malfunctions.` | — |
| `udiOpMode` | `Select the operating mode (0 = Automatic, 1 = Manual off, 2 = Manual on).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `bQ` | `Control output.` | — |

**Description:** Function block for controlling a simple single-stage motor.

##### `FB_BA_Motor2St` — §6.1.2.3.2.1.5.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bAutoSt1` | `BOOL` | — |
| `bAutoSt2` | `BOOL` | — |
| `bDst` | `BOOL` | — |
| `udiOpMode` | `UDINT` | — |
| `bEn` | `Enable motor.` | — |
| `bAutoSt1` | `Request of the actuator at stage 1 in automatic mode (udiOpMode= 0).` | — |
| `bAutoSt2` | `Request of the actuator at stage 2 in automatic mode (udiOpMode= 0).` | — |
| `bDst` | `Input for collecting the possible motor malfunctions.` | — |
| `udiOpMode` | `Select the operating mode (0 = Automatic, 1 = Manual off, 2 = Manual stage 1, 3 = Manual` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ1` | `BOOL` | — |
| `bQ2` | `BOOL` | — |
| `bQ1` | `Control output stage 1.` | — |
| `bQ2` | `Control output stage 2.` | — |

**Description:** Function block for controlling a simple two-stage motor.

##### `FB_BA_PWM` — §6.1.2.3.2.1.5.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rIn` | `REAL` | — |
| `udiPrd_sec` | `UDINT` | — |
| `udiMinSwiTi_sec` | `UDINT` | — |
| `bEn` | `Activation of pulse width modulation.` | — |
| `rIn` | `Input signal, internally limited to 0..100%.` | — |
| `udiPrd_sec` | `Period time[s]. Internally limited to a minimum value of 0.` | — |
| `udiMinSwiTi_sec` | `Minimum switch-on time [s], to avoid too short pulses. Internally limited to values` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `bLmtSwiTi` | `BOOL` | — |
| `rActTiOn_sec` | `REAL` | — |
| `rActTiOff_sec` | `REAL` | — |
| `udiRemTiOn_sec` | `UDINT` | — |
| `udiRemTiOff_sec` | `UDINT` | — |
| `bQ` | `PWM output.` | — |
| `bLmtSwiTi` | `Information output to indicate that the input signal is so low that the minimum switch-on time is` | — |
| `rActTiOn_sec` | `Information output: Calculated switch-on time.` | — |
| `rActTiOff_sec` | `Information output: Calculated switch-off time.` | — |
| `udiRemTiOn_sec` | `Switch-on timer countdown.` | — |
| `udiRemTiOff_sec` | `Switch-off timer countdown.` | — |

**Description:** The function block calculates switch-on and switch-off times rrActTiOn_sec and rActTiOff_sec [s] from an analog input signal rIn (0..100%, internally limited) and the period udiPrd_sec [s].

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Universal › Analog inputs/outputs

##### `FB_BA_AI` — §6.1.2.3.2.1.5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `TI_usiState` | `USINT` | — |
| `TI_iDataIn` | `INT` | — |
| `eTerminal` | `E_BA_TERMINAL_KL` | — |
| `eSensor` | `E_BA_SENSOR` | — |
| `bConfigurate` | `BOOL` | — |
| `rX01` | `REAL` | — |
| `rX02` | `REAL` | — |
| `rY01` | `REAL` | — |
| `rY02` | `REAL` | — |
| `rOffset` | `REAL` | — |
| `rSmoothingFactor` | `REAL` | — |
| `udiDecimalPlace` | `UDINT(1..6)` | — |
| `bEnLimitCtrl` | `BOOL` | — |
| `rHighLimit` | `REAL` | — |
| `rLowLimit` | `REAL` | — |
| `udiDelayLimitCtrl_sec` | `UDINT` | — |
| `TI_usiState` | `Linking with the corresponding status byte of the Bus Terminal in the I/O area of the program.` | — |
| `TI_iDataIn` | `Linking with the corresponding raw data (Data In) of the Bus Terminal in the I/O area of the` | — |
| `eTerminal` | `Selection of the respective Bus Terminal (see E_BA_Terminal_KL).` | — |
| `eSensor` | `Selection of the sensor type (see E_BA_Sensor).` | — |
| `bConfigurate` | `A rising edge starts the configuration of the Bus Terminal.` | — |
| `rX01` | `x-value for the interpolation point P1.` | — |
| `rX02` | `x-value for the interpolation point P2.` | — |
| `rY01` | `y-value for the interpolation point P1.` | — |
| `rY02` | `y-value for the interpolation point P2.` | — |
| `rOffset` | `Offset.` | — |
| `rSmoothingFactor` | `attenuation factor. Internally limited to between 1 and 10000.` | — |
| `udiDecimalPlace` | `Specifies the decimal places for the value rVal. Preset to 1.` | — |
| `bEnLimitCtrl` | `Enable limit value monitoring.` | — |
| `rHighLimit` | `Upper limit value.` | — |
| `rLowLimit` | `Lower limit value.` | — |
| `udiDelayLimitCtrl_sec` | `Time delay until limit value monitoring is activated.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TO_usiCtrl` | `USINT` | — |
| `TO_iDataOut` | `INT` | — |
| `rVal` | `REAL` | — |
| `bWireBreak` | `BOOL` | — |
| `bShortCircuit` | `BOOL` | — |
| `bHighLimit` | `BOOL` | — |
| `bLowLimit` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `TO_usiCtrl` | `Linking with the corresponding control byte of the Bus Terminal in the I/O area of the program.` | — |
| `TO_iDataOut` | `Linking with the corresponding raw data (Data Out) of the Bus Terminal in the I/O area of the` | — |
| `rVal` | `Scaled output value.` | — |
| `bWireBreak` | `Broken wire at the sensor.` | — |
| `bShortCircuit` | `Short-circuit at the sensor.` | — |
| `bHighLimit` | `Upper limit value exceeded.` | — |
| `bLowLimit` | `Value below lower limit.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** The function block is used for measured data processing and terminal configuration of all standard K-bus analog input terminals.

##### `FB_BA_AO` — §6.1.2.3.2.1.5.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rX` | `REAL` | — |
| `rX01` | `REAL` | — |
| `rX02` | `REAL` | — |
| `rY01` | `REAL` | — |
| `rY02` | `REAL` | — |
| `udiOpMod` | `UDINT(0..1)` | — |
| `rXManual` | `REAL` | — |
| `rX` | `Input value of the process in automatic mode.` | — |
| `rX01` | `x-value for the interpolation point P1.` | — |
| `rX02` | `x-value for the interpolation point P2.` | — |
| `rY01` | `y-value for the interpolation point P1.` | — |
| `rY02` | `y-value for the interpolation point P2.` | — |
| `udiOpMod` | `Selection of the operating mode (0 = Automatic, 1 = Manual).` | — |
| `rXManual` | `Input value for manual operation.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `TO_iY` | `INT` | — |
| `rY` | `Output signal as floating point number.` | — |
| `TO_iY` | `Output signal as integer value for linking to the output value of the bus terminal in the I/O section of` | — |

**Description:** The function block FB_BA_AO is used for output and scaling of an analog output. The input value at rX is converted into an output value from 0 to 32767 or 0-10 Volt or 4-20 mA by means of a linear equation.

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Universal › General control functions

##### `FB_BA_DMUX_XX` — §6.1.2.3.2.1.5.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `udiSel` | `UDINT` | — |
| `rIn` | `LREAL` | — |
| `bEn` | `Activation of the block function.` | — |
| `udiSel` | `Number of the output (lrQ00...lrQ16), which is to take on the value of input lrIn.` | — |
| `lrIn` | `Value to be output.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrQ00` | `LREAL` | — |
| `lrQ01` | `LREAL` | — |
| `lrQ02` | `LREAL` | — |
| `lrQ03` | `LREAL` | — |
| `lrQ04` | `LREAL` | — |
| `lrQ05` | `LREAL` | — |
| `lrQ06` | `LREAL` | — |
| `lrQ07` | `LREAL` | — |
| `lrQ08` | `LREAL` | — |
| `lrQ09` | `LREAL` | — |
| `lrQ10` | `LREAL` | — |
| `lrQ11` | `LREAL` | — |
| `lrQ12` | `LREAL` | — |
| `lrQ13` | `LREAL` | — |
| `lrQ14` | `LREAL` | — |
| `lrQ15` | `LREAL` | — |
| `lrQ16` | `LREAL` | — |

**Description:** Demultiplexer function blocks exist for different variable types (BOOL, INT, LREAL, REAL, USINT, UINT, UDINT and DINT) and in different output parameters (4, 8, 12 and 16), but they all have the same functionality. The function block FB_BA_DMUX_LR16 is described as an example.

##### `FB_BA_MMUX_XX` — §6.1.2.3.2.1.5.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `udiSel` | `UDINT` | — |
| `udiEn01` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal01` | `REAL` | — |
| `udiEn02` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal02` | `REAL` | — |
| `udiEn03` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal03` | `REAL` | — |
| `udiEn04` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal04` | `REAL` | — |
| `udiEn05` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal05` | `REAL` | — |
| `udiEn06` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal06` | `REAL` | — |
| `udiEn07` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal07` | `REAL` | — |
| `udiEn08` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal08` | `REAL` | — |
| `udiEn09` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal09` | `REAL` | — |
| `udiEn10` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal10` | `REAL` | — |
| `udiEn11` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal11` | `REAL` | — |
| `udiEn12` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal12` | `REAL` | — |
| `udiEn13` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal13` | `REAL` | — |
| `udiEn14` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal14` | `REAL` | — |
| `udiEn15` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal15` | `REAL` | — |
| `udiEn16` | `UDINT := Const.udiNoActvPrio` | — |
| `rVal16` | `REAL` | — |
| `rReplVal` | `REAL` | — |
| `bEn` | `Activation of the block function.` | — |
| `udiSel` | `Selector. Internally limited to values between 0 and 4294967294.` | — |
| `rReplVal` | `Substitute value, if no input selector condition is active` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `rVal` | `REAL` | — |
| `udiActvPrio` | `UDINT` | — |
| `bQ` | `TRUE, if the selector udiSel matches an input selector condition udiEnxx.` | — |
| `rVal` | `Value of the selected input selector condition.` | — |
| `udiActvPrio` | `Indicates which input selector condition is active. If no priority is active, the value of the global` | — |

**Description:** The function block activates an input value on the output, depending on a selector and the corresponding input selector condition.

##### `FB_BA_MUX_XX` — §6.1.2.3.2.1.5.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `udiSel` | `UDINT` | — |
| `lrIn00` | `LREAL` | — |
| `lrIn01` | `LREAL` | — |
| `lrIn02` | `LREAL` | — |
| `lrIn03` | `LREAL` | — |
| `lrIn04` | `LREAL` | — |
| `lrIn05` | `LREAL` | — |
| `lrIn06` | `LREAL` | — |
| `lrIn07` | `LREAL` | — |
| `lrIn08` | `LREAL` | — |
| `lrIn09` | `LREAL` | — |
| `lrIn10` | `LREAL` | — |
| `lrIn11` | `LREAL` | — |
| `lrIn12` | `LREAL` | — |
| `lrIn13` | `LREAL` | — |
| `lrIn14` | `LREAL` | — |
| `lrIn15` | `LREAL` | — |
| `lrIn16` | `LREAL` | — |
| `bEn` | `Activation of the block function.` | — |
| `udiSel` | `Number of the input, whose value is to be output at lrQ.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrQ` | `LREAL` | — |
| `lrQ` | `Value of the selected input.` | — |

**Description:** Multiplexer function blocks exist for different variable types (BOOL, INT, LREAL, REAL, USINT, UINT, UDINT and DINT) and in different input parameters (4, 8, 12 and 16), but they all have the same functionality. The function block FB_BA_MUX_LR16 is described as an example.

##### `FB_BA_PrioSwi_XX` — §6.1.2.3.2.1.5.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn01` | `BOOL` | — |
| `lrVal01` | `LREAL` | — |
| `bEn02` | `BOOL` | — |
| `lrVal02` | `LREAL` | — |
| `bEn03` | `BOOL` | — |
| `lrVal03` | `LREAL` | — |
| `bEn04` | `BOOL` | — |
| `lrVal04` | `LREAL` | — |
| `bEn05` | `BOOL` | — |
| `lrVal05` | `LREAL` | — |
| `bEn06` | `BOOL` | — |
| `lrVal06` | `LREAL` | — |
| `bEn07` | `BOOL` | — |
| `lrVal07` | `LREAL` | — |
| `bEn08` | `BOOL` | — |
| `lrVal08` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `lrVal` | `LREAL` | — |
| `udiActvPrio` | `UDINT` | — |
| `bQ` | `Output to indicate whether a priority is enabled.` | — |
| `lrVal` | `Output of the value of the current (highest) priority that is enabled.` | — |
| `udiActvPrio` | `Current (highest) priority that is enabled.` | — |

**Description:** The priority switches exist for different variable types (BOOL, INT, LREAL, REAL, USINT, UINT, UDINT and DINT) and in different output sizes (4, 8, 12 and 16 or 24), but they all have the same functionality. The function block FB_BA_PrioSwi_LR08 is described as an example.

##### `FB_BA_Blink` — §6.1.2.3.2.1.5.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `udiTiOn_ms` | `UDINT` | — |
| `udiTiOff_ms` | `UDINT` | — |
| `bEn` | `Function block enable.` | — |
| `udiTiOn_ms` | `pulse time [ms].` | — |
| `udiTiOff_ms` | `pause time [ms].` | — |
| `bEn` | `BOOL` | — |
| `udiNum` | `UDINT` | — |
| `bChg` | `BOOL` | — |
| `bEn01` | `BOOL` | — |
| `bEn02` | `BOOL` | — |
| `bEn03` | `BOOL` | — |
| `bEn04` | `BOOL` | — |
| `udiActvTi01_h` | `UDINT` | — |
| `udiActvTi02_h` | `UDINT` | — |
| `udiActvTi03_h` | `UDINT` | — |
| `udiActvTi04_h` | `UDINT` | — |
| `bEn` | `Enables the function block.` | — |
| `udiNum` | `Number of units.` | — |
| `bChg` | `Force sequence change.` | — |
| `bEn` | `BOOL` | — |
| `udiNum` | `UDINT` | — |
| `bChg` | `BOOL` | — |
| `bEn01` | `BOOL` | — |
| `bEn02` | `BOOL` | — |
| `bEn03` | `BOOL` | — |
| `bEn04` | `BOOL` | — |
| `bEn05` | `BOOL` | — |
| `bEn06` | `BOOL` | — |
| `bEn07` | `BOOL` | — |
| `bEn08` | `BOOL` | — |
| `udiActvTi01_h` | `UDINT` | — |
| `udiActvTi02_h` | `UDINT` | — |
| `udiActvTi03_h` | `UDINT` | — |
| `udiActvTi04_h` | `UDINT` | — |
| `udiActvTi05_h` | `UDINT` | — |
| `udiActvTi06_h` | `UDINT` | — |
| `udiActvTi07_h` | `UDINT` | — |
| `udiActvTi08_h` | `UDINT` | — |
| `bEn` | `Enables the function block.` | — |
| `udiNum` | `Number of units.` | — |
| `bChg` | `Force sequence change.` | — |
| `bEn` | `BOOL` | — |
| `bEvt01` | `BOOL` | — |
| `udiDlyOn01_sec` | `UDINT` | — |
| `udiDlyOff01_sec` | `UDINT` | — |
| `bEvt02` | `BOOL` | — |
| `udiDlyOn02_sec` | `UDINT` | — |
| `udiDlyOff02_sec` | `UDINT` | — |
| `bEvt03` | `BOOL` | — |
| `udiDlyOn03_sec` | `UDINT` | — |
| `udiDlyOff03_sec` | `UDINT` | — |
| `bEvt04` | `BOOL` | — |
| `udiDlyOn04_sec` | `UDINT` | — |
| `udiDlyOff04_sec` | `UDINT` | — |
| `bEvt05` | `BOOL` | — |
| `udiDlyOn05_sec` | `UDINT` | — |
| `udiDlyOff05_sec` | `UDINT` | — |
| `bEvt06` | `BOOL` | — |
| `udiDlyOn06_sec` | `UDINT` | — |
| `udiDlyOff06_sec` | `UDINT` | — |
| `bEvt07` | `BOOL` | — |
| `udiDlyOn07_sec` | `UDINT` | — |
| `udiDlyOff07_sec` | `UDINT` | — |
| `bEvt08` | `BOOL` | — |
| `udiDlyOn08_sec` | `UDINT` | — |
| `udiDlyOff08_sec` | `UDINT` | — |
| `bEn` | `Enable function block.` | — |
| `bEn` | `BOOL` | — |
| `bEvt01` | `BOOL` | — |
| `udiDlyOn01_sec` | `UDINT` | — |
| `udiDlyOff01_sec` | `UDINT` | — |
| `bEvt02` | `BOOL` | — |
| `udiDlyOn02_sec` | `UDINT` | — |
| `udiDlyOff02_sec` | `UDINT` | — |
| `bEvt03` | `BOOL` | — |
| `udiDlyOn03_sec` | `UDINT` | — |
| `udiDlyOff03_sec` | `UDINT` | — |
| `bEvt04` | `BOOL` | — |
| `udiDlyOn04_sec` | `UDINT` | — |
| `udiDlyOff04_sec` | `UDINT` | — |
| `bEvt05` | `BOOL` | — |
| `udiDlyOn05_sec` | `UDINT` | — |
| `udiDlyOff05_sec` | `UDINT` | — |
| `bEvt06` | `BOOL` | — |
| `udiDlyOn06_sec` | `UDINT` | — |
| `udiDlyOff06_sec` | `UDINT` | — |
| `bEvt07` | `BOOL` | — |
| `udiDlyOn07_sec` | `UDINT` | — |
| `udiDlyOff07_sec` | `UDINT` | — |
| `bEvt08` | `BOOL` | — |
| `udiDlyOn08_sec` | `UDINT` | — |
| `udiDlyOff08_sec` | `UDINT` | — |
| `bEvt09` | `BOOL` | — |
| `udiDlyOn09_sec` | `UDINT` | — |
| `udiDlyOff09_sec` | `UDINT` | — |
| `bEvt10` | `BOOL` | — |
| `udiDlyOn10_sec` | `UDINT` | — |
| `udiDlyOff10_sec` | `UDINT` | — |
| `bEvt11` | `BOOL` | — |
| `udiDlyOn11_sec` | `UDINT` | — |
| `udiDlyOff11_sec` | `UDINT` | — |
| `bEvt12` | `BOOL` | — |
| `udiDlyOn12_sec` | `UDINT` | — |
| `udiDlyOff12_sec` | `UDINT` | — |
| `bEn` | `Enable function block.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `udiTiNextSwi_sec` | `UDINT` | — |
| `bQ` | `Oscillator output.` | — |
| `udiTiNextSwi_sec` | `Countdown to next change of bQ [s].` | — |
| `bQ01` | `BOOL` | — |
| `bQ02` | `BOOL` | — |
| `bQ03` | `BOOL` | — |
| `bQ04` | `BOOL` | — |
| `udiNextOn` | `UDINT` | — |
| `udiNextOff` | `UDINT` | — |
| `arrFIFO` | `ARRAY [1..4] OF UDINT` | — |
| `udiNumOfEn` | `UDINT` | — |
| `bErr` | `BOOL` | — |
| `sErrDesc` | `STRING` | — |
| `udiNextOn` | `Number of the unit that is switched on next.` | — |
| `udiNextOff` | `Number of the unit that is switched on next.` | — |
| `arrFIFO` | `FIFO buffer as a field.` | — |
| `udiNumOfEn` | `Number of devices, depending on the individual enable states.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDesc` | `Contains the error description.` | — |
| `bQ01` | `BOOL` | — |
| `bQ02` | `BOOL` | — |
| `bQ03` | `BOOL` | — |
| `bQ04` | `BOOL` | — |
| `bQ05` | `BOOL` | — |
| `bQ06` | `BOOL` | — |
| `bQ07` | `BOOL` | — |
| `bQ08` | `BOOL` | — |
| `udiNextOn` | `UDINT` | — |
| `udiNextOff` | `UDINT` | — |
| `arrFIFO` | `ARRAY [1..8] OF UDINT` | — |
| `udiNumOfEn` | `UDINT` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `STRING` | — |
| `udiNextOn` | `Number of the unit that is switched on next.` | — |
| `udiNextOff` | `Number of the unit that is switched on next.` | — |
| `arrFIFO` | `FIFO buffer as a field.` | — |
| `udiNumOfEn` | `Number of devices, depending on the individual enable states.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |
| `bQ01` | `BOOL` | — |
| `bQ02` | `BOOL` | — |
| `bQ03` | `BOOL` | — |
| `bQ04` | `BOOL` | — |
| `bQ05` | `BOOL` | — |
| `bQ06` | `BOOL` | — |
| `bQ07` | `BOOL` | — |
| `bQ08` | `BOOL` | — |
| `bUp` | `BOOL` | — |
| `bDwn` | `BOOL` | — |
| `udiActvEvt` | `UDINT` | — |
| `udiRemTiDlyOn_sec` | `UDINT` | — |
| `udiRemTiDlyOff_sec` | `UDINT` | — |
| `bUp` | `Control chain is in ascending state.` | — |
| `bDwn` | `Control chain is in descending state.` | — |
| `udiActvEvt` | `Active step, display 0..8; "0" represents an active step sequence.` | — |
| `udiRTiDlyOn` | `Time remaining to up-switching to the next step [s].` | — |
| `udiRTiDlyOff` | `Time remaining to down-switching to the previous step [s].` | — |
| `bQ01` | `BOOL` | — |
| `bQ02` | `BOOL` | — |
| `bQ03` | `BOOL` | — |
| `bQ04` | `BOOL` | — |
| `bQ05` | `BOOL` | — |
| `bQ06` | `BOOL` | — |
| `bQ07` | `BOOL` | — |
| `bQ08` | `BOOL` | — |
| `bQ09` | `BOOL` | — |
| `bQ10` | `BOOL` | — |
| `bQ11` | `BOOL` | — |
| `bQ12` | `BOOL` | — |
| `bUp` | `BOOL` | — |
| `bDwn` | `BOOL` | — |
| `udiActvEvt` | `UDINT` | — |
| `udiRemTiDlyOn_sec` | `UDINT` | — |
| `udiRemTiDlyOff_sec` | `UDINT` | — |
| `bUp` | `Control chain is in ascending state.` | — |
| `bDwn` | `Control chain is in descending state.` | — |
| `udiActvEvt` | `Active step, display 0..12; "0" represents an active step sequence.` | — |
| `udiRTiDlyOn` | `Time remaining to up-switching to the next step [s].` | — |
| `udiRTiDlyOff` | `Time remaining to down-switching to the previous step [s].` | — |

**Description:** This function block is an oscillator with adjustable pulse and pause time, udiTiOn_ms and udiTiOff_ms [ms]. It is enabled with a TRUE signal at bEn and starts with the pulse phase.

##### `FB_BA_FIFO04_XX` — §6.1.2.3.2.1.5.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `arrFIFO` | `Array [1..4] OF UDINT` | — |
| `aFIFO` | `Contains the assignment table with a maximum of eight values. The first value indicates where the` | — |

**Description:** This function block is used to evaluate the FiFo memory from FB_BA_FIFO04 [} 657]. The inputs are linked according to the FIFO table to the corresponding outputs of the function block FB_BA_FIFO04_BOOL or FB_BA_FIFO04_REAL.

##### `FB_BA_FIFO08_XX` — §6.1.2.3.2.1.5.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `arrFIFO` | `Array [1..8] OF UDINT` | — |
| `aFIFO` | `Contains the assignment table with a maximum of eight values. The first value indicates where the` | — |

**Description:** This function block is used to evaluate the FiFo memory from FB_BA_FIFO08 [} 659]. The inputs are linked according to the FIFO table to the corresponding outputs of the function block FB_BA_FIFO08_BOOL or FB_BA_FIFO08_REAL.

#### (top level)

##### `4 rSwiOn04 rSwiOn04 0 udiDlyOff TRUE TRUE TRUE TRUE` — §4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rIn` | `REAL` | — |
| `rSwiOn01` | `REAL` | — |
| `rHys01` | `REAL` | — |
| `udiDlyOn01_sec` | `UDINT` | — |
| `udiDlyOff01_sec` | `UDINT` | — |
| `rSwiOn02` | `REAL` | — |
| `rHys02` | `REAL` | — |
| `udiDlyOn02_sec` | `UDINT` | — |
| `udiDlyOff02_sec` | `UDINT` | — |
| `rSwiOn03` | `REAL` | — |
| `rHys03` | `REAL` | — |
| `udiDlyOn03_sec` | `UDINT` | — |
| `udiDlyOff03_sec` | `UDINT` | — |
| `rSwiOn04` | `REAL` | — |
| `rHys04` | `REAL` | — |
| `udiDlyOn04_sec` | `UDINT` | — |
| `udiDlyOff04_sec` | `UDINT` | — |
| `udiNumOfStp` | `UDINT` | — |
| `bActn` | `BOOL` | — |
| `bEn` | `General enable of the function block. If bEn is FALSE, all outputs are set to 0.` | — |
| `rIn` | `Input value, from which the switching state is derived.` | — |
| `rSwiOn01` | `Switch-on point stage 01` | — |
| `rHys01` | `Absolute value hysteresis stage 01` | — |
| `udiDlyOn01_sec` | `Switch-on delay stage 01` | — |
| `udiDlyOff01_sec` | `Switch-off delay stage 01` | — |
| `rSwiOn02` | `Switch-on point stage 02` | — |
| `rHys02` | `Absolute value hysteresis stage 02` | — |
| `udiDlyOn02_sec` | `Switch-on delay stage 02` | — |
| `udiDlyOff02_sec` | `Switch-off delay stage 02` | — |
| `rSwiOn03` | `Switch-on point stage 03` | — |
| `rHys03` | `Absolute value hysteresis stage 03` | — |
| `udiDlyOn03_sec` | `Switch-on delay stage 03` | — |
| `udiDlyOff03_sec` | `Switch-off delay stage 03` | — |
| `rSwiOn04` | `Switch-on point stage 04` | — |
| `rHys04` | `Absolute value hysteresis stage 04` | — |
| `udiDlyOn04_sec` | `Switch-on delay stage 04` | — |
| `udiDlyOff04_sec` | `Switch-off delay stage 04` | — |
| `udiNumOfStp` | `Number of stages that are required.` | — |
| `bActn` | `Input variable used to determine the control direction of the step switch.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ01` | `BOOL` | — |
| `bQ02` | `BOOL` | — |
| `bQ03` | `BOOL` | — |
| `bQ04` | `BOOL` | — |
| `udiStp` | `UDINT` | — |
| `rSwiOn` | `REAL` | — |
| `rSwiOff` | `REAL` | — |
| `udiRemTiDlyOn_sec` | `UDINT` | — |
| `udiRemTiDlyOff_sec` | `UDINT` | — |
| `bQ01` | `Display of status step 01` | — |
| `bQ02` | `Display of status step 02` | — |
| `bQ03` | `Display of status step 03` | — |
| `bQ04` | `Display of status step 04` | — |
| `udiStp` | `Shows the current step of the step switch` | — |
| `rSwiOn` | `Shows the next switch-on point` | — |
| `rSwiOff` | `Shows the next switch-off point` | — |
| `udiRemTiDlyOn_sec` | `If the switch-on point for switching to the next level is met, the progress of the switch-` | — |
| `udiRemTiDlyOff_sec` | `If the switch-off point for switching down to the next level is met, the progress of the` | — |

**Description:** Diagram 03

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Universal › Hysteresis, 2-point control

##### `FB_BA_Swi2P` — §6.1.2.3.2.1.5.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rIn` | `REAL` | — |
| `rOn` | `REAL` | — |
| `rOff` | `REAL` | — |
| `udiDlyOn_sec` | `UDINT` | — |
| `udiDlyOff_sec` | `UDINT` | — |
| `bEn` | `General enable of the function block.` | — |
| `rIn` | `Input value.` | — |
| `rOn` | `Switch-on point.` | — |
| `rOff` | `Switch-off point.` | — |
| `udiDlyOn_sec` | `Switch-on delay.` | — |
| `udiDlyOff_sec` | `Switch-off delay.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `udiRemTiDlyOn_sec` | `UDINT` | — |
| `udiRemTiDlyOff_sec` | `UDINT` | — |
| `bQ` | `Control output.` | — |
| `udiRemTiDlyOn_sec` | `Remaining time of the switch-on delay.` | — |
| `udiRemTiDlyOff_sec` | `Remaining time of the switch-off delay.` | — |

**Description:** The function block FB_BA_Swi2P is a two-point switch with one switch-on point and one switch-off point.

##### `FB_BA_SwiHys2P` — §6.1.2.3.2.1.5.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rIn` | `REAL` | — |
| `rSp` | `REAL` | — |
| `rHys` | `REAL` | — |
| `rHysOffs` | `REAL` | — |
| `udiDlyOn_sec` | `UDINT` | — |
| `udiDlyOff_sec` | `UDINT` | — |
| `bActn` | `BOOL` | — |
| `bEn` | `General enable of the function block.` | — |
| `rIn` | `Input value.` | — |
| `rSp` | `Setpoint input.` | — |
| `rHys` | `Hysteresis.` | — |
| `rHysOffs` | `Hysteresis offset.` | — |
| `udiDlyOn_sec` | `Switch-on delay` | — |
| `udiDlyOff_sec` | `Release delay` | — |
| `bActn` | `Control direction.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `rSwiHi` | `REAL` | — |
| `rSwiLo` | `REAL` | — |
| `udiRemTiDlyOn_sec` | `UDINT` | — |
| `udiRemTiDlyOff_sec` | `UDINT` | — |
| `bQ` | `Output.` | — |
| `rSwiHi` | `Upper switching point.` | — |
| `rSwiLo` | `Lower switching point.` | — |
| `udiRemTiDlyOn_sec` | `Time remaining before switching on.` | — |
| `udiRemTiDlyOff_sec` | `Time remaining before switching off.` | — |

**Description:** The function block FB_BA_SwiHys2P is a two-point switch with adjustable hysteresis and hysteresis offset.

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Universal › Mathematical functions

##### `FB_BA_MultiCalc_XX` — §6.1.2.3.2.1.5.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `r01` | `REAL` | — |
| `r02` | `REAL` | — |
| `r03` | `REAL` | — |
| `r04` | `REAL` | — |
| `r05` | `REAL` | — |
| `r06` | `REAL` | — |
| `r07` | `REAL` | — |
| `r08` | `REAL` | — |
| `udiNum` | `UDINT` | — |
| `bEn` | `Activation of the block function.` | — |
| `udiNum` | `Number of input values to be used for the calculation.` | — |
| `rX` | `REAL` | — |
| `rX01` | `REAL` | — |
| `rX02` | `REAL` | — |
| `rY01` | `REAL` | — |
| `rY02` | `REAL` | — |
| `bLmt` | `BOOL` | — |
| `rX` | `Input value of the characteristic curve.` | — |
| `rX01` | `X-value for interpolation point P1.` | — |
| `rX02` | `X-value for interpolation point P2.` | — |
| `rY01` | `Y-value for interpolation point P1.` | — |
| `rY02` | `Y-value for interpolation point P2.` | — |
| `bLmt` | `Limit for the output value rY.` | — |
| `rX` | `REAL` | — |
| `rX01` | `REAL` | — |
| `rX02` | `REAL` | — |
| `rX03` | `REAL` | — |
| `rX04` | `REAL` | — |
| `rY01` | `REAL` | — |
| `rY02` | `REAL` | — |
| `rY03` | `REAL` | — |
| `rY04` | `REAL` | — |
| `bLmt` | `BOOL` | — |
| `rX` | `Input value of the characteristic curve.` | — |
| `rX01` | `X-value for interpolation point P1.` | — |
| `rX02` | `X-value for interpolation point P2.` | — |
| `rX03` | `X-value for interpolation point P3.` | — |
| `rX04` | `X-value for interpolation point P4.` | — |
| `rY01` | `Y-value for interpolation point P1.` | — |
| `rY02` | `Y-value for interpolation point P2.` | — |
| `rY03` | `Y-value for interpolation point P3.` | — |
| `rY04` | `Y-value for interpolation point P4.` | — |
| `bLmt` | `Limit for the output value rY.` | — |
| `rX` | `REAL` | — |
| `rX01` | `REAL` | — |
| `rX02` | `REAL` | — |
| `rX03` | `REAL` | — |
| `rX04` | `REAL` | — |
| `rX05` | `REAL` | — |
| `rX06` | `REAL` | — |
| `rX07` | `REAL` | — |
| `rY01` | `REAL` | — |
| `rY02` | `REAL` | — |
| `rY03` | `REAL` | — |
| `rY04` | `REAL` | — |
| `rY05` | `REAL` | — |
| `rY06` | `REAL` | — |
| `rY07` | `REAL` | — |
| `bLmt` | `BOOL` | — |
| `rX` | `Input value of the characteristic curve.` | — |
| `rX01` | `X-value for interpolation point P1.` | — |
| `rX02` | `X-value for interpolation point P2.` | — |
| `rX03` | `X-value for interpolation point P3.` | — |
| `rX04` | `X-value for interpolation point P4.` | — |
| `rX05` | `X-value for interpolation point P5.` | — |
| `rX06` | `X-value for interpolation point P6.` | — |
| `rX07` | `X-value for interpolation point P7.` | — |
| `rY01` | `Y-value for interpolation point P1.` | — |
| `rY02` | `Y-value for interpolation point P2.` | — |
| `rY03` | `Y-value for interpolation point P3.` | — |
| `rY04` | `Y-value for interpolation point P4.` | — |
| `rY05` | `Y-value for interpolation point P5.` | — |
| `rY06` | `Y-value for interpolation point P6.` | — |
| `rY07` | `Y-value for interpolation point P7.` | — |
| `bLmt` | `Limit for the output value rY.` | — |
| `rX` | `REAL` | — |
| `arrX` | `ARRAY [1..FB_BA_Chrct32.cBA_NumOfElem] OF REAL` | — |
| `arrY` | `ARRAY [1..FB_BA_Chrct32.cBA_NumOfElem] OF REAL` | — |
| `diNumOfElem` | `DINT(2..32)` | — |
| `bLmt` | `BOOL` | — |
| `rX` | `Input value of the characteristic curve` | — |
| `arrX` | `Field with the X-values for the interpolation points.` | — |
| `arrY` | `Field with the Y-values for the interpolation points.` | — |
| `nNumOfElem` | `Number of interpolation points. Internally limited to values between 2 and 32.` | — |
| `bLmt` | `Limitation of the output value rY.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rMax` | `REAL` | — |
| `udiMaxActv` | `UDINT` | — |
| `rMin` | `REAL` | — |
| `udiMinActv` | `UDINT` | — |
| `rAvrg` | `REAL` | — |
| `rSum` | `REAL` | — |
| `rDiff` | `REAL` | — |
| `rMax` | `Maximum value of all inputs.` | — |
| `udiMaxActv` | `Input at which the maximum value is present.` | — |
| `rMin` | `Minimum value of all inputs.` | — |
| `udiMinActv` | `Input at which the minimum value is present.` | — |
| `rAvrg` | `Mean value of all inputs.` | — |
| `rSum` | `Sum of all inputs.` | — |
| `rDiff` | `Difference between maximum and minimum value.` | — |
| `rY` | `REAL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `rY` | `Calculated output value of the characteristic curve.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |
| `rY` | `REAL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `rY` | `Calculated output value of the characteristic curve.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |
| `rY` | `REAL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `rY` | `Calculated output value of the characteristic curve.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |
| `rY` | `REAL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `rY` | `Calculated output value of the characteristic curve.` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** The multi-calculation function blocks exist for the variable types LREAL and REAL, although they all have the same functionality. The function block FB_BA_R08 is described as an example.

##### `FB_BA_TiAvrg` — §6.1.2.3.2.1.5.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rIn` | `REAL` | — |
| `udiIntval_sec` | `UDINT` | — |
| `udiNumOfElem` | `UDINT` | — |
| `bEn` | `Enables the function block.` | — |
| `rIn` | `Input value for averaging.` | — |
| `udiIntVal_SEC` | `Time interval [s] for writing new values into the FIFO. Internally limited to a value between 1` | — |
| `udiNumOfElem` | `Size of the FIFO buffer. A change resets the previous averaging. Internally limited to a` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rOut` | `REAL` | — |
| `rMax` | `REAL` | — |
| `rMin` | `REAL` | — |
| `rOut` | `Calculated mean value.` | — |
| `rMax` | `Largest value in the FIFO buffer.` | — |
| `rMin` | `Smallest value in the FIFO buffer.` | — |

**Description:** The function block FB_BA_TiAvrg calculates the arithmetic mean value of an analog value that was logged over a certain period. Discrete values are written into a FIFO buffer. udiIntval_sec specifies the time interval [s] over which the values are logged and written into the FIFO. Values are written if the input bEn is TRUE. The variable udiNumOfElem is used to determine the size of the FIFO buffer. It is limited to 1..512. The function block can be used for calculating an hourly mean outside temperature over a day, for example. In this case udiNumOfElem would be 24 and udiIntval_sec would be 3600 seconds. bEn is the general enable of the function block. If bEn = FALSE, the FIFO buffer within the function block is deleted completely, and no data are recorded.

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Universal › Monitoring functions

##### `FB_BA_FixedLimitCtrl` — §6.1.2.3.2.1.5.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rHighLimit` | `REAL := 32` | — |
| `rLowLimit` | `REAL := 16` | — |
| `rIn` | `REAL` | — |
| `udiDelay_sec` | `UDINT` | — |
| `bEn` | `Function block enable.` | — |
| `rHighLimit` | `Default upper limit value, preset to 32.` | — |
| `rLowLimit` | `Default lower limit value, preset to 16.` | — |
| `rIn` | `Input value to be monitored.` | — |
| `udiDelay_sec` | `Output response delay [s]. Internally limited to values between 0 and Const.udiTiSec [} 701].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bHighLimit` | `BOOL` | — |
| `bLowLimit` | `BOOL` | — |
| `udiRemTiDelay_sec` | `UDINT` | — |
| `bHighLimit` | `Upper limit value reached.` | — |
| `bLowLimit` | `Lower limit value reached.` | — |
| `udiRemTiDelay_sec` | `Time remaining after a limit value has been exceeded until either the output` | — |

**Description:** Function block for monitoring a fixed limit value.

##### `FB_BA_FdbCtrlBinary` — §6.1.2.3.2.1.5.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bActuator` | `BOOL` | — |
| `bSwitch` | `BOOL` | — |
| `udiFdbDelay_sec` | `UDINT` | — |
| `udiInterruptionDelay_sec` | `UDINT` | — |
| `bEn` | `Function block enable.` | — |
| `bActuator` | `Feedback of the switching output.` | — |
| `bSwitch` | `Feedback signal from the process.` | — |
| `udiFdbDelay_sec` | `Response delay [s] of the monitoring function when the actuator is started. Internally` | — |
| `udiInterruptionDelay_sec` | `Response delay [s] of the monitoring function when the actuator has already` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `udiRemTiFdbDelay` | `UDINT` | — |
| `udiRemTiInterruptionDelay` | `UDINT` | — |
| `bQ` | `Output an error message if the feedback signal is not present within the parameterized time of` | — |
| `udiRemTiFdbDelay` | `Remaining time [s] until output bErrOpn is set.` | — |
| `udiRemTiInterruptionDelay` | `Remaining time [s] until output bErrSwi is set.` | — |

**Description:** The function block is used for feedback monitoring of an actuator by means of digital feedback. Application examples of the function block are, for example, an operation feedback monitoring, a process feedback monitoring or the run monitoring of a drive by means of limit switches.

##### `FB_BA_SlidingLimitCtrl` — §6.1.2.3.2.1.5.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `rHys` | `REAL` | — |
| `udiDelay_sec` | `UDINT` | — |
| `bEn` | `Function block enable.` | — |
| `rW` | `Setpoint.` | — |
| `rX` | `Actual value.` | — |
| `rHys` | `Hysteresis.` | — |
| `udiDelay_sec` | `Output response delay [s]. Internally limited to values between 0 and Const.udiTiSec [} 701].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bHighLimit` | `BOOL` | — |
| `bLowLimit` | `BOOL` | — |
| `rHighLimit` | `REAL` | — |
| `rLowLimit` | `REAL` | — |
| `udiRemTiDelay_sec` | `UDINT` | — |
| `bHighLimit` | `Upper limit value reached.` | — |
| `bLowLimit` | `Lower limit value reached.` | — |
| `rHighLimit` | `Output of the upper limit value.` | — |
| `rLowLimit` | `Output of the lower limit value.` | — |
| `udiRemTiDelay_sec` | `Time remaining after a limit value has been exceeded until either the output` | — |

**Description:** Function block for monitoring a floating setpoint.

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Universal

##### `Ramps, filters, controllers` — §6.1.2.3.2.1.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rIn` | `REAL` | — |
| `udiDampConst_sec` | `UDINT` | — |
| `bSetActl` | `BOOL` | — |
| `rIn` | `Input signal` | — |
| `udiDampConst_sec` | `Filter time constant [s]. Internally limited to values between 0 and 86400.` | — |
| `bSetActl` | `A rising edge at this input sets the output value rOut to the input value rIn.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rOut` | `REAL` | — |
| `rOut` | `Filtered output signal.` | — |

**Description:** Function blocks

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Universal › Ramps, filters, controllers

##### `FB_BA_RampLmt` — §6.1.2.3.2.1.5.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bEnRamp` | `BOOL` | — |
| `rIn` | `REAL` | — |
| `rHi` | `REAL` | — |
| `rLo` | `REAL` | — |
| `udiTiUp_sec` | `UDINT` | — |
| `udiTiDwn_sec` | `UDINT` | — |
| `bEn` | `Enable function block if FALSE, in which case rOut = 0.0.` | — |
| `bEnRamp` | `Enable ramp limitation if FALSE, in which case rOut = rIn.` | — |
| `rIn` | `Input value of the ramp function` | — |
| `rHi` | `Upper interpolation point for calculating the ramps.` | — |
| `rLo` | `Lower interpolation point for calculating the ramps. rHi must be greater than rLo, otherwise an error is` | — |
| `udiTiUp_sec` | `Rise time [s].` | — |
| `udiTiDwn_sec` | `Fall time [s]` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rOut` | `REAL` | — |
| `rOut` | `Output signal, slope-limited through the ramps` | — |

**Description:** The function block limits the increase or decrease speed of an input signal. An increase of rIn results in the output rOut to be limited to the slope of (rHi-rLo)/udiTiUp. A decrease of rIn results in the output rOut to be limited to the slope of (rHi-rLo)/udiTiUp.

#### Programming › PLC › Libraries › Archive › Tc3_BA › POUs › Universal › Calendar

##### `FB_BA_SchedulerWeeklyXXCh` — §6.1.2.3.2.1.5.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `stSysTi` | `TIMESTRUCT` | — |
| `udiPredictTime_sec` | `UDINT` | — |
| `bEn` | `General function block enable.` | — |
| `stSysTi` | `Structure with the local NT system time (see TIMESTRUCT).` | — |
| `udiPredictTime_sec` | `Precalculated switch-on time. Internally limited to values between 0 and 43200.` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrChannel` | `ARRAY [1..cBA_NumOfChannels] OF ST_BA_SchedulerWeeklyChannel` | — |
| `arrChannel` | `Weekly scheduler; with the single-channel function block, the name of the variable is stChannel` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | — |
| `bQ` | `Switching output` | — |

**Description:** Weekly timer with 1, 7 or 28 timer channels. The function block FB_BA_SchedulerWeekly07Ch is described as an example.

##### `FB_BA_CalenderXXCh` — §6.1.2.3.2.1.5.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `stSysTi` | `TIMESTRUCT` | — |
| `bEn` | `General function block enable.` | — |
| `stSysTi` | `Structure with the local NT system time (see TIMESTRUCT).` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrChannel` | `ARRAY [1..7] OF ST_BA_CalendarChannel` | — |
| `arrChannel` | `Yearly scheduler; with the single-channel function block, the name of the variable is stChannel` | — |

**Description:** Yearly scheduler with 1, 7 or 28 channels.

#### Programming › PLC › Templates › POUs › Trade › AirConditioning

##### `FB_BA_AirConditioning` (FUNCTION_BLOCK) — §6.1.4.3.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AHU01` | `FB_BA_AHU_1st_10` | — |

**Description:** The function block serves as a call template for plants for the Air conditioning systems.

#### Programming › PLC › Templates › POUs › Trade › AirConditioning › Aggregates › Cooler › Temperature

##### `FB_BA_AC_ColTSuACtrl` (FUNCTION_BLOCK) — §6.1.4.3.1.1.2.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable for the frequency controller Ctrl. |
| `fSpTSuA` | `REAL` | Setpoint of the supply air temperature. |
| `fTSuA` | `REAL` | Measured value of the supply air temperature. |
| `bSync` | `BOOL` | Pulse for synchronization of the sequence controller Ctrl. |
| `fSync` | `REAL` | Value for the synchronization of the sequence controller |
| `fSpTSuA` | `REAL` | Setpoint of the supply air temperature. |
| `fTSuA` | `REAL` | Measured value of the supply air temperature. |
| `bSyncTSuACtrl` | `BOOL` | Input for synchronization of the supply air sequence |
| `fSpTSuA` | `REAL` | Setpoint of the supply air temperature. |
| `fTSuA` | `REAL` | Measured value of the supply air temperature. |
| `bSyncTSuACtrl` | `BOOL` | Input for synchronization of the supply air sequence |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSeqLink` | `ST_BA_SeqLink` | — |
| `stTSeqLink` | `ST_BA_SeqLink` | — |
| `stAggregate` | `ST_BA_Aggregate` | — |
| `stTSeqLink` | `ST_BA_SeqLink` | — |
| `stAggregate` | `ST_BA_Aggregate` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fY` | `REAL` | Control value output |
| `bIsActive` | `BOOL` | The sequence controller is the active one in the sequence |
| `bCmd` | `BOOL` | Current switching status of the single-stage pump. |
| `fPresentValue` | `REAL` | Current value of the cooler valve. |
| `bActiveSeqCtrl` | `BOOL` | Indicates that the sequence controller of the cooler is the |
| `fPresentValue` | `REAL` | Current value of the cooler valve. |
| `bActiveSeqCtrl` | `BOOL` | Indicates that the sequence controller of the cooler is the |

**Description:** The template represents the supply air temperature control of a cooler with the sequence controller Ctrl.

#### Programming › PLC › Templates › POUs › Trade › AirConditioning › Aggregates › Damper

##### `FB_BA_AC_Dmp2P` (FUNCTION_BLOCK) — §6.1.4.3.1.1.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bFan` | `BOOL` | The "Fan On" input signal is used within a ventilation and |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAggregate` | `ST_BA_Aggregate` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwiOpn` | `BOOL` | End position "open" of the damper has been reached. |
| `bSwiCls` | `BOOL` | End position "closed" of the damper has been reached. Inputs/outputs |
| `stAggregate` | `ST_BA_Aggregate [} 276]` | Bidirectional aggregate structure via which the enables |

**Description:** The aggregate template represents the control of a two-point damper with integrated monitoring of both end positions.

#### Programming › PLC › Templates › POUs › Trade › AirConditioning › Aggregates › ERC

##### `FB_BA_AC_ErcTSuACtrl` (FUNCTION_BLOCK) — §6.1.4.3.1.1.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable for the frequency controller Ctrl. |
| `fSpTSuA` | `REAL` | Setpoint of the supply air temperature. |
| `fTSuA` | `REAL` | Measured value of the supply air temperature. |
| `bSync` | `BOOL` | Pulse for synchronization of the sequence controller Ctrl. |
| `fSync` | `REAL` | Value for the synchronization of the sequence controller |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSeqLink` | `ST_BA_SeqLink` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fY` | `REAL` | Control value output |
| `eAction` | `E_BA_Action` | The output of the control direction of the supply air |

**Description:** The template represents the supply air temperature control of an energy recovery system with the sequence controller Ctrl.

##### `Plate` (FUNCTION_BLOCK) — §6.1.4.3.1.1.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSpTSuA` | `REAL` | Setpoint of the supply air temperature |
| `fTSuA` | `REAL` | Measured value of the supply air temperature |
| `bSyncTSuACtrl` | `BOOL` | Pulse for synchronization of the sequence controller |
| `fSpTSuA` | `REAL` | Setpoint of the supply air temperature |
| `fTSuA` | `REAL` | Measured value of the supply air temperature |
| `bSyncTSuACtrl` | `BOOL` | Pulse for synchronization of the sequence controller |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTSeqLink` | `ST_BA_SeqLink` | — |
| `stAggregate` | `ST_BA_Aggregate` | — |
| `stTSeqLink` | `ST_BA_SeqLink` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current value of the bypass damper system |
| `eAction` | `E_BA_Action` | The output of the control direction of the supply air |
| `fPresentValue` | `REAL` | Current value of the bypass damper system |
| `eAction` | `E_BA_Action` | The output of the control direction of the supply air |

**Description:** 6.1.4.3.1.1.2.3.2.1       FB_BA_AC_ErcPl_01

##### `Rotation` (FUNCTION_BLOCK) — §6.1.4.3.1.1.2.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSpTSuA` | `REAL` | Setpoint of the supply air temperature |
| `fTSuA` | `REAL` | Measured value of the supply air temperature |
| `bSyncTSuACtrl` | `BOOL` | Pulse for synchronization of the sequence controller |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTSeqLink` | `ST_BA_SeqLink` | — |
| `stAggregate` | `ST_BA_Aggregate` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current value of the bypass damper system |
| `eAction` | `E_BA_Action` | The output of the control direction of the supply air |

**Description:** The template represents the open-loop and closed-loop control of an energy recovery system with a rotary heat exchanger.

#### Programming › PLC › Templates › POUs › Trade › AirConditioning › Aggregates › Fan

##### `FB_BA_AC_FanCtl` (FUNCTION_BLOCK) — §6.1.4.3.1.1.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bDamperOpen` | `BOOL` | The input signal "Damper open" is used within a air |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAggregate` | `ST_BA_Aggregate` | — |

**Description:** The aggregate template represents the control and regulation of a pressure-controlled fan. Typical applications for this template are the supply and exhaust air fans of an air conditioning system with variable volume flow rate.

#### Programming › PLC › Templates › POUs › Trade › AirConditioning › Aggregates › PreHeater

##### `FB_BA_AC_PreHtr` (FUNCTION_BLOCK) — §6.1.4.3.1.1.2.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSpTSuA` | `REAL` | Setpoint of the supply air temperature |
| `fTSuA` | `REAL` | Measured value of the supply air temperature |
| `bSyncTSuACtrl` | `BOOL` | Input for synchronization of the supply air sequence |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTSeqLink` | `ST_BA_SeqLink` | — |
| `stAggregate` | `ST_BA_Aggregate` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bCmd` | `BOOL` | Current switching status of the single-stage pump. |
| `fPresentValue` | `REAL` | Current control value of the heater valve. |
| `bFrost` | `BOOL` | Frost protection active display. The message is triggered |

**Description:** The template represents the open-loop and closed-loop control of a hot water air heater.

##### `FB_BA_AC_PreHtrTSuACtrl` (FUNCTION_BLOCK) — §6.1.4.3.1.1.2.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable the sequence controller Ctrl. |
| `fSpTSuA` | `REAL` | Setpoint of the supply air temperature. |
| `fTSuA` | `REAL` | Measured value of the supply air temperature. |
| `bSync` | `BOOL` | Pulse for synchronization of the sequence controller Ctrl. |
| `fSync` | `REAL` | Value for the synchronization of the sequence controller |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSeqLink` | `ST_BA_SeqLink` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fY` | `REAL` | Control value output control valve. |
| `bIsActive` | `BOOL` | The sequence controller is the active one in the sequence |

**Description:** The template represents the supply air temperature control of a preheater in a ventilation system. The supply air controller Ctrl is the core of the template. It is integrated into a supply air temperature control sequence.

##### `FB_BA_AC_TRtCtrl` (FUNCTION_BLOCK) — §6.1.4.3.1.1.2.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnPreRinse` | `BOOL` | Enabling the pre-rinse process. |
| `fTRt` | `REAL` | Measured value of the return temperature of the hot water |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fY` | `REAL` | Control value output of the return temperature controller |

**Description:** The template is used for open-loop and closed-loop control of the return temperature of a hot water air heater.

#### Programming › PLC › Templates › POUs › Trade › AirConditioning › General

##### `FB_BA_AC_OpMod1St` (FUNCTION_BLOCK) — §6.1.4.3.1.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSmExtt` | `BOOL` | Smoke extraction of the plant requested |
| `bFireDetc` | `BOOL` | Fire alarm message from fire alarm center |
| `bForceVent` | `BOOL` | Forced ventilation request |
| `bEmgOff` | `BOOL` | Emergency stop |
| `bFrst` | `BOOL` | Frost protection program active |
| `bCentSwiOff` | `BOOL` | Central switch-off |
| `bAlmShutOff` | `BOOL` | Collective error message - shut down plant |
| `bNgtCol` | `BOOL` | Request from the summer night cooling program (see FB_BA_AC_SumNgtCol [} 788]) |
| `bColDwnPrtc` | `BOOL` | Request program cooling protection |
| `bOvrHtgPrtc` | `BOOL` | Request program overheating protection |
| `eOpMode` | `E_BA_AC_OpMod01` | Input of the current operation mode eOpMode, see |
| `[} 705]` | `template FB_BA_AC_OpMod1St [} 774].` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrAggregate` | `ARRAY [1..MAX(1,BA_Param.nMaxAggregate)] OF ST_BA_Aggregate` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eOpMode` | `E_BA_AC_OpMod01 [} 705]` | Output of the current operation mode eOpMode Inputs CONSTANT |
| `OpModMan` | `FB_BA_MV_Op [} 235]` | The Multistate-Value object represents an operating |
| `ePltStp` | `E_BA_AC_PlantStep01` | Output of the current plant step of the air conditioning [} 707] system. |
| `bStepCtrlUp` | `BOOL` | Indicates that the order of the step sequence control is in |

**Description:** The template represents the operation mode of an air conditioning system.

##### `CallInit` (METHOD) — §6.1.4.3.1.1.3.2.1

**Description:** The CallInit method is used to call the initialization methods for the parameterization of the control matrix of the PlantControl [} 477] unit.

##### `FB_BA_AC_SeqT` (FUNCTION_BLOCK) — §6.1.4.3.1.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Activation of the supply air temperature sequence control. |
| `eOpMode` | `E_BA_AC_OpMod01` | The plant operation mode is used for the selection of the [} 705] start sequence controller. |
| `fTWth` | `REAL` | Input for the weather temperature. Programming |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTSeqLink` | `ST_BA_SeqLink` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSumLmt` | `BOOL` | The output variable indicates that the ventilation system |

**Description:** The template represents the start and control of the supply air temperature sequence control of an air conditioning (HVAC) system.

##### `FB_BA_AC_SumNgtCol` (FUNCTION_BLOCK) — §6.1.4.3.1.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fTRm` | `REAL` | Input measured value room temperature. |
| `fSpRm` | `REAL` | Input setpoint room temperature. |
| `fTWth` | `REAL` | Input for the weather temperature. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Summer night cooling switched on. Inputs CONSTANT |
| `WthTLoLmt` | `FB_BA_AV_Op [} 206]` | Input of the value for the lower outside temperature. The |

**Description:** The template represents a night cooling function for ventilation systems. It is used to cool down rooms that have been heated up during the day with cool outside air at night. The summer night cooling function serves to improve the quality of the air and to save electrical energy. Electrical energy for cooling is saved during the first hours of the next summer day.

#### (top level)

##### `E_BA_AC_PlantStep01.eOn Room supply air cascade, TCasCtrl` (FUNCTION_BLOCK) — §08

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `TOuA` | `FB_BA_SensorAnalog` | — |
| `TExhA` | `FB_BA_SensorAnalog` | — |
| `TSuA` | `FB_BA_SensorAnalog` | — |
| `TExtA` | `FB_BA_SensorAnalog` | — |
| `TRm` | `FB_BA_SensorAnalog` | — |
| `DiffPrssOuAFilt` | `FB_BA_SensorAnalog` | — |
| `FiltOuAMonit` | `FB_BA_Hys_21` | — |
| `DiffPrssExtAFilt` | `FB_BA_SensorAnalog` | — |
| `FiltExtAMonit` | `FB_BA_Hys_21` | — |
| `OuADmp` | `FB_BA_AC_Dmp2P` | — |
| `SuAPrss` | `FB_BA_SensorAnalog` | — |
| `SuADiffPrss` | `FB_BA_SensorAnalog` | — |
| `SuAFan` | `FB_BA_AC_FanCtl` | — |
| `SuADiffPrssMonit` | `FB_BA_FdbAnalog` | — |
| `ExhADmp` | `FB_BA_AC_Dmp2P` | — |
| `ExtAPrss` | `FB_BA_SensorAnalog` | — |
| `ExtADiffPrss` | `FB_BA_SensorAnalog` | — |
| `ExtAFan` | `FB_BA_AC_FanCtl` | — |
| `ExtADiffPrssMonit` | `FB_BA_FdbAnalog` | — |
| `PreHtr` | `FB_BA_AC_PreHtr` | — |
| `Erc` | `FB_BA_AC_ErcPl_02` | — |
| `Col` | `FB_BA_AC_ColT_02` | — |
| `SpTRm` | `FB_BA_Scale_04_Y2_3` | — |
| `TCasCtrl` | `FB_BA_AC_TCasCtrl_01` | — |
| `SumNgtCol` | `FB_BA_AC_SumNgtCol` | — |
| `OpMod` | `FB_BA_AC_OpMod1St` | — |
| `PlantLock` | `FB_BA_PlantLock` | — |
| `PlantCmd` | `FB_BA_AC_PlantCmd_AHU_1st_10` | — |
| `TSeqLink` | `FB_BA_AC_SeqT` | — |
| `stTSeqLink` | `ST_BA_SeqLink` | — |
| `arrAggregate` | `ARRAY [1..BA_Param.nMaxAggregate] OF ST_BA_Aggregate` | — |

**Description:** If the plant has reached step 08 E_BA_AC_PlantStep01.eOn, the supply air temperature sequence control TSeqLink is enabled when the cascade control TCasCtrl is switched on.

#### Programming › PLC › Templates › POUs › Trade › AirConditioning › Setpoint

##### `Temperature` (FUNCTION_BLOCK) — §6.1.4.3.1.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fTWth` | `REAL` | Current value of the outside temperature. |
| `fSpRmHtg` | `REAL` | Input variable for the setpoint room temperature Heating |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAggregate` | `ST_BA_Aggregate` | — |
| `stAggregate` | `ST_BA_Aggregate` | — |
| `stAggregate` | `ST_BA_Aggregate` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fSpHtg` | `REAL` | Calculated setpoint of the supply air temperature for the |
| `fSpHtg` | `REAL` | Calculated setpoint of the supply air temperature for the |
| `fSpHtg` | `REAL` | Calculated setpoint of the supply air temperature for the |
| `fSpHtg` | `REAL` | Calculated setpoint of the supply air temperature for the |

**Description:** The template is a setpoint program for an air conditioning system.

#### Programming › PLC › Templates › POUs › Trade › AutomationControl

##### `FB_BA_AutomationControl` (FUNCTION_BLOCK) — §6.1.4.3.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Device` | `FB_BA_Device` | — |
| `EvtGroups` | `FB_BA_EvtGroups` | — |
| `Cabinet` | `FB_BA_ControlCabinet` | — |

**Description:** The template contains basic functions and project information relevant for an automation station.

#### Programming › PLC › Templates › POUs › Trade › AutomationControl › ControlCabinet

##### `FB_BA_ControlCabinetBasicExt` (FUNCTION_BLOCK) — §6.1.4.3.1.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `LampFault` | `FB_BA_ActuatorCmd` | — |
| `AckButton` | `FB_BA_CombinationBI_BV` | — |
| `EventObserver` | `FB_BA_EventObserver` | — |

**Description:** The template is used to control a collective error signal lamp and to acknowledge or reset alarms by means of a push button.

##### `FB_BA_ControlCabinetBasic` (FUNCTION_BLOCK) — §6.1.4.3.1.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `LampFault` | `FB_BA_ActuatorCmd` | — |
| `AckButton` | `FB_BA_CombinationBI_BV` | — |
| `RstHw` | `FB_BA_ActuatorCmd` | — |
| `EventObserver` | `FB_BA_EventObserver` | — |

**Description:** The template is used to control a collective error signal lamp, to acknowledge or reset alarms by means of a push button and to reset hardware in the control cabinet by outputting a momentary pulse.

##### `FB_BA_ControlCabinet` (FUNCTION_BLOCK) — §6.1.4.3.1.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Fuse01` | `FB_BA_SensorBinary` | — |
| `Fuse02` | `FB_BA_SensorBinary` | — |
| `Fuse03` | `FB_BA_SensorBinary` | — |

**Description:** Programming

#### Programming › PLC › Templates › POUs › Trade › AutomationControl › Device

##### `FB_BA_Device` (FUNCTION_BLOCK) — §6.1.4.3.1.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Trend` | `FB_BA_TrendLogging` | — |
| `Project` | `FB_BA_ProjectEx` | — |
| `Parameter` | `FB_BA_Parameter` | — |

**Description:** The template includes several basic functions.

#### Programming › PLC › Templates › POUs › Trade › AutomationControl › Device › BACnetSettings

##### `FB_BACnet_RcpList` (FUNCTION_BLOCK) — §6.1.4.3.1.2.3.2.1

**Description:** The sample shows the parameterization of the recipient list of the message/event class objects. This sample must be adapted to the project in question.

##### `FB_BA_Parameter` (FUNCTION_BLOCK) — §6.1.4.3.1.2.3.2.2

**Description:** The function block contains the function block FB_BA_Param in the declaration part of the method FB_init. This is used to parameterize the global variable lists XBA_BACnetParam and XBA_Param [} 129] of the library Tc3_XBA [} 101]. In the implementation part of the method FB_init various parameterizations are listed for this purpose.

#### Programming › PLC › Templates › POUs › Trade › AutomationControl › Device › TimeSettings

##### `FB_BA_BACnetTime_TO_SysTime` (FUNCTION_BLOCK) — §6.1.4.3.1.2.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `GetLocalSysTime_nTiAuto` | `UDINT := 60` | — |
| `SetSysTimeLocal` | `FB_BA_SetTime` | — |
| `GetLocalSysTime` | `FB_BA_GetTime` | — |
| `GetBACnetAdapterTime` | `FB_BACnet_GetAdapterTime` | — |

**Description:** The template reads the BACnet system time at regular intervals and writes it to the local NT system time of the TwinCAT system.

##### `FB_BA_ExternalSysTime` (FUNCTION_BLOCK) — §6.1.4.3.1.2.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sExternalAmsNetId` | `T_AmsNetId := ''` | — |
| `GetExtSysTime_nTiAuto` | `UDINT := 600` | — |
| `GetLocalSysTime_nTiAuto` | `UDINT := 60` | — |
| `GetExtSysTime` | `FB_BA_GetTime` | — |
| `SetSysTimeLocal` | `FB_BA_SetTime` | — |
| `SetBACnetAdapterTime` | `FB_BACnet_SetAdapterTime` | — |
| `GetLocalSysTime` | `FB_BA_GetTime` | — |
| `GetBACnetAdapterTime` | `FB_BACnet_GetAdapterTime` | — |

**Description:** The template reads the NT system time of an external TwinCAT system at regular intervals.

#### Programming › PLC › Templates › POUs › Trade › AutomationControl › Device › Project

##### `FB_BA_ProjectEx` (FUNCTION_BLOCK) — §6.1.4.3.1.2.3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bWrtPrsOnPowerFai BOOL` | `Input variable for linking with a UPS function block l` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nPrsDataWritten` | `UINT` | Counter output for the executed write cycles. |

**Description:** This template is derived from the basic function block FB_BA_Project [} 226].

#### Programming › PLC › Templates › POUs › Trade › AutomationControl › EventClasses

##### `FB_BA_EvtGroups` (FUNCTION_BLOCK) — §6.1.4.3.1.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `LifeSafety` | `FB_BA_EvtCategory` | — |
| `SafetyMsg` | `FB_BA_EvtCategory` | — |
| `AlarmMsg` | `FB_BA_EvtCategory` | — |
| `FaultMsg` | `FB_BA_EvtCategory` | — |
| `MaintenanceMsg01` | `FB_BA_EvtCategory` | — |
| `MaintenanceMsg02` | `FB_BA_EvtCategory` | — |
| `SystemMsg` | `FB_BA_EvtCategory` | — |
| `ManualOp` | `FB_BA_EvtCategory` | — |
| `OtherMsg` | `FB_BA_EvtCategory` | — |

**Description:** The template provides a group of notification classes.

#### Programming › PLC › Templates › POUs › Trade › BuildingAutomation › Communication

##### `FB_BA_AdsComServer_Basic` (FUNCTION_BLOCK) — §6.1.4.3.1.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `WeatherStation` | `FB_BA_RawPublisher` | — |
| `GeneralSettings` | `FB_BA_RawPublisher` | — |
| `BuildingAlarms` | `FB_BA_RawPublisher` | — |

**Description:** The purpose of the template is to read data and parameters assigned to the "General Building Automation" trade from the global variable list Site [} 718] and make them available within the BA network via FB_BA_RawPublisher [} 149].

##### `FB_BA_AdsComClient_Basic` (FUNCTION_BLOCK) — §6.1.4.3.1.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `WeatherStation` | `FB_BA_RawSubscriber` | — |
| `WeatherStationError` | `FB_BA_BV_ErrorState` | — |
| `GeneralSettings` | `FB_BA_RawSubscriber` | — |
| `GeneralSettingsError` | `FB_BA_BV_ErrorState` | — |
| `BuildingAlarms` | `FB_BA_RawSubscriber` | — |
| `BuildingAlarmsError` | `FB_BA_BV_ErrorState` | — |

**Description:** The template reads global data assigned to the "General building automation" trade from another controller, which makes this data available with the counter-block FB_BA_AdsComServer_Basic [} 840]. The information read is copied to the global variable list Site [} 718].

#### Programming › PLC › Templates › POUs › Trade › BuildingAutomation › General

##### `FB_BA_BuildingData` — §6.1.4.3.1.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `BuildingMode` | `FB_BA_BuildingMode` | — |
| `BuildingEnergLvl` | `FB_BA_BuildingEnergyLevel` | — |
| `BuildingSpRmT` | `FB_BA_BuildingSpRmT` | — |
| `BuildingLight` | `FB_BA_BuildingLighting` | — |

**Description:** This template at system level is used to call up sub-templates that provide building-wide data and operation modes.

##### `FB_BA_FloorData` — §6.1.4.3.1.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `FloorLighting` | `FB_BA_FloorLighting` | — |

**Description:** This template at system level is used to call up sub-templates that provide floor-wide data and operation modes.

#### (top level)

##### `Comfort.` (FUNCTION_BLOCK) — §4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `EnergLvlMan` | `FB_BA_MV_Op` | — |
| `EnergLvlPr` | `FB_BA_MV_Op` | — |
| `Sched` | `FB_BA_SchedM` | — |

**Description:** The currently selected energy level is then displayed via the function block EnergLvlPr and made available via Publisher.

##### `Cleaning.` (FUNCTION_BLOCK) — §3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `OpModMan` | `FB_BA_MV_Op` | — |
| `OpModPr` | `FB_BA_MV_Op` | — |
| `Sched` | `FB_BA_SchedM` | — |

**Description:** The currently selected building mode is then displayed via the function block OpModPr and made available via Publisher.

#### Programming › PLC › Templates › POUs › Trade › BuildingAutomation › General › Functions

##### `FB_BA_BuildingSpRmT` (FUNCTION_BLOCK) — §6.1.4.3.1.3.2.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SumCpsn` | `FB_BA_Scale_02` | — |
| `WinCpsn` | `FB_BA_Scale_02` | — |
| `PrtcHtg` | `FB_BA_AV_Op` | — |
| `EcoHtg` | `FB_BA_AV_Op` | — |
| `PreCmfHtg` | `FB_BA_AV_Op` | — |
| `CmfHtg` | `FB_BA_AV_Op` | — |
| `PrtcCol` | `FB_BA_AV_Op` | — |
| `EcoCol` | `FB_BA_AV_Op` | — |
| `PreCmfCol` | `FB_BA_AV_Op` | — |
| `CmfCol` | `FB_BA_AV_Op` | — |
| `ShiftHtg` | `FB_BA_AV_Op` | — |
| `ShiftCol` | `FB_BA_AV_Op` | — |

**Description:** In this template the basic setpoints Protection Cooling ... Comfort Cooling and Protection Heating ... Comfort Heating are applied with correction values and made available as building-wide setpoints in a globally declared structure variable stBuildingSpRmT (see global variable list Site [} 718]).

#### Programming › PLC › Templates › POUs › Trade › BuildingAutomation › WeatherData

##### `FB_BA_WeatherData` (FUNCTION_BLOCK) — §6.1.4.3.1.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `WthParam` | `FB_BA_WeatherParameter` | — |

**Description:** The template is used to prepare and distribute global data that is required throughout the system or building to control and regulate the system automation.

#### Programming › PLC › Templates › POUs › Trade › BuildingAutomation › WeatherData › Parameter

##### `FB_BA_WeatherParameter` (FUNCTION_BLOCK) — §6.1.4.3.1.3.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fTWth` | `REAL` | Current value of the outside temperature. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fSpFrstProtec` | `REAL` | Frost protection setpoint, e.g. for heating circuits in |

**Description:** The FB_BA_WeatherParameter template calculates various values from the measured value of the outside temperature, which are required system-wide for the control and regulation of heating, ventilation and air conditioning systems.

#### Programming › PLC › Templates › POUs › Trade › BuildingAutomation

##### `FB_BA_BuildingAutomationServer` (FUNCTION_BLOCK) — §6.1.4.3.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AdsComServerBasic` | `FB_BA_AdsComServer_Basic` | — |
| `Weatherstation01` | `FB_BA_WeatherStation_Thies` | — |
| `WeatherData` | `FB_BA_WeatherData` | — |

**Description:** Programming

##### `FB_BA_BuildingAutomationClient` (FUNCTION_BLOCK) — §6.1.4.3.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AdsComClientBasic` | `FB_BA_AdsComClient_Basic` | — |
| `FloorData` | `FB_BA_FloorData` | — |

**Description:** Call template trade "Building automation" - client version.

#### Programming › PLC › Templates › POUs › Trade › DoorsGatesWindowsSunProtection

##### `FB_BA_DGWSPServer` (FUNCTION_BLOCK) — §6.1.4.3.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AdsComServerDGWS` | `FB_BA_AdsComServer_DGWS` | — |
| `BuildingSunProtection` | `FB_BA_BuildingSunprotection` | — |
| `arrFacade` | `ARRAY[1..BA2_Param.nMaxNumberOfFacades] OF FB_BA_Facade` | — |

**Description:** Call template trade "Doors, gates, windows, sun protection" - server version.

##### `FB_BA_DGWSPClient` (FUNCTION_BLOCK) — §6.1.4.3.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AdsComClientDGWS` | `FB_BA_AdsComClient_DGWS` | — |

**Description:** Call template trade "Doors, gates, windows, sun protection" - client version.

#### Programming › PLC › Templates › POUs › Trade › DoorsGatesWindowsSunProtection › Communication

##### `FB_BA_AdsComClient_DGWS` (FUNCTION_BLOCK) — §6.1.4.3.1.4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `BuildingSunBlind` | `FB_BA_RawSubscriber` | — |
| `BuildingSunBlindError` | `FB_BA_BV_ErrorState` | — |
| `arrFacade` | `ARRAY[1..BA2_Param.nMaxNumberOfFacades] OF FB_BA_FacadeSubscr` | — |
| `arrFacadeError` | `ARRAY[1..BA2_Param.nMaxNumberOfFacades] OF FB_BA_BV_ErrorState` | — |

**Description:** The template reads global data that is assigned to the "Doors, gates, windows, sun protection" trade from another controller that provides this data with the counter-block FB_BA_AdsComServer_DGWS [} 857]. The information read is copied to the global variable list Site [} 718].

##### `FB_BA_AdsComServer_DGWS` (FUNCTION_BLOCK) — §6.1.4.3.1.4.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `BuildingSunBlind` | `FB_BA_RawPublisher` | — |
| `arrFacadeData` | `ARRAY[1..BA2_Param.nMaxNumberOfFacades] OF FB_BA_RawPublisher` | — |

**Description:** The template has the task of reading data and parameters that are assigned to the "Doors, gates, windows, sun protection" trade from the global variable list Site [} 718] and making them available within the BA network via FB_BA_RawPublisher [} 149].

#### Programming › PLC › Templates › POUs › Trade › HazardDetectionAlarmSystems › BurglarAlarmSystem

##### `FB_BA_BurglarAlarmSystem` (FUNCTION_BLOCK) — §6.1.4.3.1.5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MagneticContact01` | `FB_BA_SensorBinary` | — |

**Description:** This template receives the burglar alarm as an input via a FB_BA_SensorBinary [} 1078] function block and transfers it as Site.stBuildingAlarms.bBurglary to the local variable list Site [} 718].

#### Programming › PLC › Templates › POUs › Trade › HazardDetectionAlarmSystems › FireAlarmSystem

##### `FB_BA_FireAlarmSystem` (FUNCTION_BLOCK) — §6.1.4.3.1.5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `FireDetector01` | `FB_BA_SensorBinary` | — |

**Description:** This template receives the fire alarm as an input via an FB_BA_SensorBinary [} 1078] function block and transfers it as Site.stBuildingAlarms.bFireAlert to the local variable list Site [} 718].

#### Programming › PLC › Templates › POUs › Trade › HazardDetectionAlarmSystems

##### `FB_BA_HazardDetectionAlarmSystems` (FUNCTION_BLOCK) — §6.1.4.3.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `FireAlarmSystem01` | `FB_BA_FireAlarmSystem` | — |
| `BurglarAlarmSystem01` | `FB_BA_BurglarAlarmSystem` | — |

**Description:** This Trade template is a function block for calling up all sub-templates that are used to record alarm and hazard messages in the building.

#### Programming › PLC › Templates › POUs › Trade › HeatingSystem

##### `Distribution` (FUNCTION_BLOCK) — §6.1.4.3.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `TFl` | `FB_BA_SensorAnalog_Raw` | — |
| `TRt` | `FB_BA_SensorAnalog_Raw` | — |
| `Vlv` | `FB_BA_Vlv` | — |
| `Pu` | `FB_BA_Pu1st` | — |
| `Sp` | `FB_BA_H_HtgCir_Sp` | — |
| `OpMod` | `FB_BA_H_OpMod` | — |
| `HtgLmt` | `FB_BA_HtgLmt` | — |
| `TFlCtrl` | `FB_BA_PID` | — |
| `PlantLock` | `FB_BA_PlantLock` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eOpMod` | `E_BA_EnergyLvl [} 710]` | Operation mode of the heating circuit. |
| `bRls` | `BOOL` | The variable indicates that the heating circuit is in |

**Description:** Template FB_BA_H_HtgCir01 is used to program a static heating circuit.

##### `FB_BA_Heating` (FUNCTION_BLOCK) — §6.1.4.3.1.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `HTC01` | `FB_BA_H_HtgCir01` | — |

**Description:** The FB_BA_Heating function block serves as a call template for the heating trade.

#### Programming › PLC › Templates › POUs › Trade › HeatingSystem › General

##### `FB_BA_H_OpMod` (FUNCTION_BLOCK) — §6.1.4.3.1.6.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bDst` | `BOOL` | Heating circuit fault input. |
| `bHtgLmt` | `BOOL` | Heating mode enable input. |
| `bTWthLowLimit` | `BOOL` | The variable indicates that the outside temperature has |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eOpMode` | `E_BA_EnergyLvl [} 710]` | Display of the current operation mode of the heating |

**Description:** The FB_BA_H_OpMod template represents the operation mode and enable of a heating circuit.

##### `FB_BA_HtgLmt` (FUNCTION_BLOCK) — §6.1.4.3.1.6.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `REAL` | The actual value for the controller Ctrl is connected to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Indicates whether the heating limit switch is active. Inputs CONSTANT |
| `SP` | `FB_BA_AV_Op [} 206]` | AV object for entering the heating limit value for night |

**Description:** Heating limit switch for enabling automatic heating operation depending on the input value fIn and the heating limit value Sp.

#### Programming › PLC › Templates › POUs › Trade › HeatingSystem › Setpoint

##### `FB_BA_H_HtgCir_Sp` (FUNCTION_BLOCK) — §6.1.4.3.1.6.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eOpMode` | `E_BA_EnergyLvl [} 710]` | Input of the current operation mode. |
| `fSpFrstProtec` | `REAL` | Frost protection setpoint input for the "Protection" |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fSpFlWT` | `REAL` | Output of the setpoint for the heating circuit. Inputs CONSTANT |
| `NgtOffs` | `FB_BA_AV_Op [} 206]` | AV object for inputting the value for the night setback. |
| `SpFlWT` | `FB_BA_AV_Op [} 206]` | AV object for displaying the setpoint for the heating circuit. |
| `HtgCrv` | `FB_BA_Scale_04 [} 1036]` | Calculation of the heating curve for the heating circuit |

**Description:** Template for providing the setpoint for a heating circuit control.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › Common › DALIFunctionBlocks

##### `FB_BA_BrtnsPrcDALI` (FUNCTION_BLOCK) — §6.1.4.3.1.7.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bInitialize` | `BOOL` | — |
| `nPrdQueryBrtns` | `UDINT` | — |
| `nAdr` | `BYTE` | — |
| `nInstPrc` | `BYTE` | — |
| `nInstBrtns` | `BYTE` | — |
| `nResBrtns` | `BYTE := 10` | — |
| `ipDALICommunication` | `Tc3_DALI.I_DALICommunication` | — |
| `nDeadtime` | `BYTE := 2` | — |
| `nHold` | `BYTE := 90` | — |
| `nReport` | `BYTE := 60` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPrc` | `BOOL` | — |
| `nBrtns` | `UINT` | — |
| `bInitializing` | `BOOL` | — |
| `bErr` | `BOOL` | — |

**Description:** This function block is used to read a combined DALI brightness and occupancy sensor.

##### `FB_BA_LightActrDALI_Base` (FUNCTION_BLOCK) — §6.1.4.3.1.7.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bInitialize` | `BOOL` | — |
| `stLightingCmd` | `ST_BA_Lighting` | — |
| `eAdrType` | `Tc3_DALI.E_DALIAddressType := Tc3_DALI.E_DALIAddressType.Short` | — |
| `nAdr` | `BYTE` | — |
| `nAdrRefDev` | `BYTE` | — |
| `ipDALICommunication` | `I_DALICommunication` | — |
| `nHysLgtVal` | `BYTE` | — |
| `nPrdQueryLgtVal` | `UDINT` | — |
| `nMinLevel` | `BYTE := 126` | — |
| `nMaxLevel` | `BYTE := 254` | — |
| `nPowerOnLevel` | `BYTE := 254` | — |
| `nSystemFailureLevel` | `BYTE := 254` | — |
| `eFadeTime` | `Tc3_DALI.E_DALIFadeTime := Tc3_DALI.E_DALIFadeTime.Disabled` | — |
| `eFadeRate` | `Tc3_DALI.E_DALIFadeRate := Tc3_DALI.E_DALIFadeRate.N358StepsPerSe` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActlLgtVal` | `BYTE` | — |
| `fActlLgtVal` | `REAL` | — |
| `bErr` | `BOOL` | — |

**Description:** This function block is used to control a DALI light actuator, a DALI group or DALI devices with a broadcast, whereby only the light brightness value is changed.

##### `FB_BA_LightActrDALI_TC` (FUNCTION_BLOCK) — §6.1.4.3.1.7.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bInitialize` | `BOOL` | — |
| `stLightingCmd` | `ST_BA_Lighting` | — |
| `eAdrType` | `Tc3_DALI.E_DALIAddressType := Tc3_DALI.E_DALIAddressType.Short` | — |
| `nAdr` | `BYTE` | — |
| `nAdrRefDev` | `BYTE` | — |
| `ipDALICommunication` | `Tc3_DALI.I_DALICommunication` | — |
| `nHysLgtVal` | `BYTE` | — |
| `fHysLgtT` | `REAL` | — |
| `nPrdQueryLgtVal` | `UDINT` | — |
| `nPrdQueryLgtT` | `UDINT` | — |
| `nMinLevel` | `BYTE := 126` | — |
| `nMaxLevel` | `BYTE := 254` | — |
| `nPowerOnLevel` | `BYTE := 254` | — |
| `nSystemFailureLevel` | `BYTE := 254` | — |
| `eFadeTime` | `Tc3_DALI.E_DALIFadeTime := Tc3_DALI.E_DALIFadeTime.Disabled` | — |
| `eFadeRate` | `Tc3_DALI.E_DALIFadeRate := Tc3_DALI.E_DALIFadeRate.N358StepsPerSec` | — |
| `fTemperatureLimitWarmest` | `LREAL := 2700` | — |
| `fTemperatureLimitCoolest` | `LREAL := 6500` | — |
| `fPowerOnTemperature` | `LREAL := 3000` | — |
| `fSystemFailureTemperature` | `LREAL := 3000` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActlLgtVal` | `BYTE` | — |
| `fActlLgtVal` | `REAL` | — |
| `fActlLgtT` | `REAL` | — |
| `bInitializing` | `BOOL` | — |
| `bErr` | `BOOL` | — |

**Description:** The FB_BA_LightActrDALI_TC function block is used to control a DALI light actuator, a DALI group or DALI devices with a broadcast, whereby color temperature control is supported.

##### `FB_BA_Swi4DALI` (FUNCTION_BLOCK) — §6.1.4.3.1.7.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bInitialize` | `BOOL` | — |
| `nAdr` | `BYTE` | — |
| `nInstButton1` | `BYTE` | — |
| `nInstButton2` | `BYTE` | — |
| `nInstButton3` | `BYTE` | — |
| `nInstButton4` | `BYTE` | — |
| `ipDALICommunication` | `Tc3_DALI.I_DALICommunication` | — |
| `nPrdQuery` | `UDINT := 60` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bButton1` | `BOOL` | — |
| `bButton2` | `BOOL` | — |
| `bButton3` | `BOOL` | — |
| `bButton4` | `BOOL` | — |
| `bInitializing` | `BOOL` | — |
| `bErr` | `BOOL` | — |

**Description:** This function block is used to read a DALI 4-fold push button.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › Common › Sensors

##### `FB_BA_Switch4DALI` (FUNCTION_BLOCK) — §6.1.4.3.1.7.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bInitialize` | `BOOL` | — |
| `nAdr` | `BYTE` | — |
| `nInstButton1` | `BYTE := 0` | — |
| `nInstButton2` | `BYTE := 1` | — |
| `nInstButton3` | `BYTE := 2` | — |
| `nInstButton4` | `BYTE := 3` | — |
| `ipDALICommunication` | `Tc3_DALI.I_DALICommunication` | — |
| `SwitchSns` | `FB_BA_Swi4DALI` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bButton1` | `BOOL` | — |
| `bButton2` | `BOOL` | — |
| `bButton3` | `BOOL` | — |
| `bButton4` | `BOOL` | — |
| `bInitializing` | `BOOL` | — |
| `bErr` | `BOOL` | — |

**Description:** The template is used to read a 4-fold switch.

##### `FB_BA_BrightnessPresenceDALI` (FUNCTION_BLOCK) — §6.1.4.3.1.7.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `bInitialize` | `BOOL` | — |
| `bResetPresence` | `BOOL` | — |
| `nPrdQueryBrtns` | `UDINT := 1` | — |
| `nAdr` | `BYTE` | — |
| `nInstancePresence` | `BYTE` | — |
| `nInstanceBrightness` | `BYTE` | — |
| `nResBrtns` | `BYTE` | — |
| `ipDALICommunication` | `Tc3_DALI.I_DALICommunication` | — |
| `BrightnessPresenceSns` | `FB_BA_BrtnsPrcDALI` | — |
| `PresenceDelay` | `FB_BA_PresenceMonitoring` | — |
| `Brightness` | `FB_BA_AI_RAW` | — |
| `Presence` | `FB_BA_BI_RAW` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresence` | `BOOL` | — |
| `bPresenceDelayActive` | `BOOL` | — |
| `bRstSwi` | `BOOL` | — |
| `fBrightness` | `REAL` | — |
| `bInitializing` | `BOOL` | — |
| `bErr` | `BOOL` | — |

**Description:** This template is used to read a combined DALI brightness and occupancy sensor.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › Communication

##### `FB_BA_AdsComClient_Room` (FUNCTION_BLOCK) — §6.1.4.3.1.7.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `BuildingEnergyLevel` | `FB_BA_RawSubscriber` | — |
| `BuildingEnergyLevelError` | `FB_BA_BV_ErrorState` | — |
| `BuildingSpRmT` | `FB_BA_RawSubscriber` | — |
| `BuildingSpRmTError` | `FB_BA_BV_ErrorState` | — |
| `BuildingLighting` | `FB_BA_RawSubscriber` | — |
| `BuildingLightingError` | `FB_BA_BV_ErrorState` | — |

**Description:** The template reads global data that is assigned to the "Room automation" trade from another controller, which makes this data available with the FB_BA_AdsComServer_Room [} 888] counter-block. The information read is copied to the global variable list Site [} 718].

##### `FB_BA_AdsComServer_Room` (FUNCTION_BLOCK) — §6.1.4.3.1.7.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `BuildingEnergyLevel` | `FB_BA_RawPublisher` | — |
| `BuildingSpRmT` | `FB_BA_RawPublisher` | — |
| `BuildingLighting` | `FB_BA_RawPublisher` | — |

**Description:** The template has the task of reading data and parameters that are assigned to the "Room automation" trade from the global variable list Site and making them available within the BA network via FB_BA_RawPublisher [} 149].

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › HeatingCooling

##### `FB_BA_HeatCool_PID` (FUNCTION_BLOCK) — §6.1.4.3.1.7.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPrc` | `BOOL` | Presence detection. |
| `bWdwCon` | `BOOL` | Window contact (open = TRUE). |
| `eMedium` | `E_BA_Medium [} 265]` | Medium present (heating or cooling medium, only |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bCtrlHtgActv` | `BOOL` | Heating controller is active. |
| `fCtrlValHtg` | `REAL` | Control value heating valve. |
| `bCtrlColActv` | `BOOL` | Cooling controller is active. |
| `fCtrlValCol` | `REAL` | Control value cooling valve. |
| `eReqdMedium` | `E_BA_Medium [} 265]` | Requested medium (heating or cooling medium). Programming Inputs CONSTANT PERSISTENT |
| `ePipeSys` | `E_BA_PipeSys [} 265]` | Selection of two or four-pipe system. |
| `nFnctSelChgOvrDly UDINT` | `Switchover delay [s] from heating to cooling or vice versa.` | — |
| `fRmTAdjLimit` | `REAL` | Adjustable range [K] of the setpoint shift fRmTAdj : |

**Description:** This template is used to control a heating-cooling zone.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › Lighting › Functions › Zone

##### `FB_BA_LightGroupConstant` (FUNCTION_BLOCK) — §6.1.4.3.1.7.4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bManualSwitch` | `BOOL` | If the constant light control function block is not yet active, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLighting` | `ST_BA_Lighting [} 270]` | Resulting output program. |
| `bAdjusting` | `BOOL` | The constant light control function block is in regulation |

**Description:** One or more light actuators are combined in a light zone for simultaneous control, depending on local conditions.

##### `FB_BA_LightGroupPresence` (FUNCTION_BLOCK) — §6.1.4.3.1.7.4.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetRoomFunctio BOOL` | `This input switches off the light function block LightSwitch -` | — |
| `n` | `it is then no longer active (output bAutmaticMode/` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLighting` | `ST_BA_Lighting [} 270]` | Resulting output program. Inputs CONSTANT PERSISTENT |
| `nSwiTi` | `UDINT` | Ramp for the switching functions in seconds, related to a |

**Description:** One or more light actuators are combined in a light zone for simultaneous control, depending on local conditions.

##### `FB_BA_LightGroupSwitch` (FUNCTION_BLOCK) — §6.1.4.3.1.7.4.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bManualSwitch` | `BOOL` | Short button presses at this input switch the light on and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLighting` | `ST_BA_Lighting [} 270]` | Resulting output program. Programming Inputs CONSTANT PERSISTENT |
| `nSwiOvrTi` | `UDINT` | Distinction time [ms] between short and long button press. |

**Description:** One or more light actuators are combined in a light zone for simultaneous control, depending on local conditions.

##### `FB_BA_LightGroupSwitchPresence` (FUNCTION_BLOCK) — §6.1.4.3.1.7.4.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bManualSwitch` | `BOOL` | Short button presses at this input switch the light on and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLighting` | `ST_BA_Lighting [} 270]` | Resulting output program. Programming Inputs CONSTANT PERSISTENT |
| `nSwiOvrTi` | `UDINT` | Distinction time [ms] between short and long button press. |

**Description:** One or more light actuators are combined in a light zone for simultaneous control, depending on local conditions.

##### `FB_BA_LightGroupDaylight` (FUNCTION_BLOCK) — §6.1.4.3.1.7.4.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetRoomFunctio BOOL` | `This input switches off the constant light control function` | — |
| `n` | `block. Switch-off takes place via a ramp and dwell time on` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLighting` | `ST_BA_Lighting [} 270]` | Resulting output program. |
| `bAdjusting` | `BOOL` | The constant light control block is in regulation mode or the |

**Description:** This template represents a daylight-dependent light control that automatically switches on and off depending on occupancy. It is intended for corridors and passageways. An upstream, optimizing switch-on function FB_BA_LightCtrlValue [} 314] ensures the specification of a light switch-on value that switches the downstream constant light regulation close to the desired setpoint. As soon as the light is switched on via occupancy, it is automatically readjusted according to the setpoint.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › Lighting › Functions › Building

##### `FB_BA_BuildingLighting` (FUNCTION_BLOCK) — §6.1.4.3.1.7.4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fLgtVal_FireAlert` | `REAL` | — |
| `fLgtT_FireAlert` | `REAL` | — |
| `fLgtVal_Burglary` | `REAL` | — |
| `fLgtT_Burglary` | `REAL` | — |
| `FireAlert` | `FB_BA_LightingEvt := ( ePrio := E_BA_LightingPrio.eFire)` | — |
| `Burglary` | `FB_BA_LightingEvt := ( ePrio := E_BA_LightingPrio.eBurglary)` | — |
| `GlobalReset` | `FB_BA_BV_Op` | — |

**Description:** This template compiles cross-building control telegrams for the light functions.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › Lighting › Functions › Floor

##### `FB_BA_FloorLighting` (FUNCTION_BLOCK) — §6.1.4.3.1.7.4.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AreaLighting` | `ARRAY[1..BA2_Param.nMaxNumberOfAreas] OF FB_BA_AreaLighting` | — |

**Description:** This template is used to implement various light functions at floor level. It has a very simple design and can be expanded as required.

#### (top level)

##### `User-defined 2.` (FUNCTION_BLOCK) — §5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stFloor` | `ST_BA_Lighting [} 270]` | Light control telegram from the higher level "Floor". |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stArea` | `ST_BA_Lighting [} 270]` | Resulting light control telegram from this template, which |

**Description:** The currently selected lighting mode eAreaMode, which is of the same type as the building mode [} 705], is then displayed via the OpModPr function block.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › Lighting › Actuators

##### `FB_BA_LightAnalog` (FUNCTION_BLOCK) — §6.1.4.3.1.7.4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enabling the function block. |
| `stLightingCmd` | `ST_BA_Lighting [} 270]` | Resulting telegram from the higher-level zone (room). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fActualLightValue` | `REAL` | Current light value in percent. |
| `eActualLightPrio` | `BYTE` | Current priority of the command telegram stLightingCmd. |
| `stFeedback` | `ST_BA_LightActuatorFeed Feedback telegram for linking to the controlling room` | — |
| `back [} 710]` | `(zone) application function. In this way, information about` | — |

**Description:** This template is used to control an analog light actuator.

##### `FB_BA_LightDALI_Base` (FUNCTION_BLOCK) — §6.1.4.3.1.7.4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enabling the function block: A TRUE signal activates the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitializing` | `BOOL` | The light actuator or the actuators are in the DALI |

**Description:** This template is used to control a DALI light actuator, a DALI group or DALI devices with a broadcast, whereby only the light brightness value is changed.

##### `FB_BA_LightDALI_TC` (FUNCTION_BLOCK) — §6.1.4.3.1.7.4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enabling the function block: A TRUE signal activates the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitializing` | `BOOL` | The light actuator or the actuators are in the DALI |

**Description:** This template is used to control a DALI light actuator, a DALI group or DALI devices with a broadcast, whereby color temperature control is supported.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation

##### `RoomSamples` (FUNCTION_BLOCK) — §6.1.4.3.1.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stFacade` | `ST_BA_Facade [} 713]` | Facade-specific blind data and telegrams. |
| `stAreaLighting` | `ST_BA_Lighting [} 270]` | Resulting light telegram from the building, the floor and |

**Description:** Possible room templates.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › SunProtection › Functions › Building

##### `FB_BA_BuildingSunprotection` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `FireAlert` | `FB_BA_SunBldEvt := (ePrio:=E_BA_SunBldPrio.eFire)` | — |
| `Burglary` | `FB_BA_SunBldEvt := (ePrio:=E_BA_SunBldPrio.eBurglary)` | — |
| `IceProtection` | `FB_BA_IceProtection` | — |
| `GlobalThAutoRlse` | `FB_BA_Hys_03` | — |
| `GlobalTwiLgtAutoRlse` | `FB_BA_Hys_03` | — |
| `SunPrtcPosIntval` | `FB_BA_AV_Op_SP` | — |
| `GlobalResetManMode` | `FB_BA_BV_Op_Val` | — |

**Description:** This template collects cross-building criteria for the blind functions, which are then further used in the facade instances of the FB_BA_Facade [} 938]. These are in detail: • Resulting telegram building alarms • Protection telegram Fire The FB_BA_FireAlarmSystem [} 859] template supplies the information "Fire alarm" via a globally declared structure variable stBuildingAlarms (see global variable list Site [} 718]). If this alarm is active, the blinds are raised completely via a FB_BA_SunBldEvt [} 377]. • Protection telegram Burglar The FB_BA_BurglarAlarmSystem [} 858] template supplies the information "burglar alarm" via a globally declared structure variable stBuildingAlarms (see global variable list Site [} 718]). If this alarm is active, the blinds are raised completely via a FB_BA_SunBldEvt [} 377]. This makes the burglar less hidden from view from the outside. • Protection telegram Icing An imminent icing is detected by the fact that during a precipitation detection the measured outside temperature is below the frost limit value - this is specified here by the object SpIce and is pre-set to

##### `FB_BA_IceProtection` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SpIce` | `FB_BA_AV_Op` | — |
| `DeiceTi` | `FB_BA_AV_Op` | — |
| `IceAlert` | `FB_BA_BV` | — |
| `SunBldIcePrtc` | `FB_BA_SunBldIcePrtc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Ice protection telegram. |
| `bIcePrtcActv` | `BOOL` | Control output "Ice protection active". |
| `SpIce` | `FB_BA_AV_Op [} 206]` | Input limit value for icing [°C]. |
| `DeiceTi` | `FB_BA_AV_Op [} 206]` | Enter de-icing time [s]. |
| `IceAlert` | `FB_BA_BV [} 214]` | Display object Status ice protection. |
| `SunBldIcePrtc` | `FB_BA_SunBldIcePrtc [} 378]` | Ice protection for the lamellas. Variables |
| `tonInitialWait` | `TON` | Initial function block enable delay. |

**Description:** Sub-template ice protection.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › SunProtection › Functions › Facade

##### `FB_BA_Facade` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fRefRmT` | `REAL` | Room temperature of the reference room for the facade- |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stFacade` | `ST_BA_Facade [} 713]` | Output structure of the collected facade data. |
| `fFcdOrtn` | `REAL` | Facade orientation northern hemisphere: north=0°, |

**Description:** This template compiles the sun protection telegrams that are valid for an entire facade.

##### `FB_BA_FacadeSubscr` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stData` | `ST_BA_Facade [} 713]` | Read facade telegram, overwritten in case of error. Variables |
| `_fbHighPrio` | `FB_BA_SunBldEvt [} 377]` | Telegram block which activates a high-priority |
| `pData` | `pData` | Pointer to the subscribed data. |
| `nSize` | `DINT` | Size of the subscribed data. |

**Description:** This template is used to receive the facade data of the type ST_BA_Facade [} 713] and is typically used in the floor controllers.

##### `FB_BA_Facade_Information` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stFacadeResult` | `ST_BA_SunBld [} 272]` | Resulting telegram from the higher-level facade template |

**Description:** Sub-template facade information.

##### `FB_BA_Facade_Maintenance` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MntnEn` | `FB_BA_BV_Op` | — |
| `MntnPos` | `FB_BA_AV_Op` | — |
| `MntnAngl` | `FB_BA_AV_Op` | — |
| `MntnEvent` | `FB_BA_SunBldEvt := (ePrio:=E_BA_SunBldPrio.eMaintenance)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Maintenance telegram. |
| `bMntnActv` | `BOOL` | Control output "Maintenance active". Inputs CONSTANT |
| `MntnEn` | `FB_BA_BV_Op [} 216]` | Function block for activating the maintenance telegram. |
| `MntnPos` | `FB_BA_AV_Op [} 206]` | Function block for entering the position. |
| `MntnAngl` | `FB_BA_AV_Op [} 206]` | Function block for entering the angle. |
| `MntnEvent` | `FB_BA_SunBldEvt [} 377]` | Function block for telegram generation. |

**Description:** Sub-template Maintenance position.

##### `FB_BA_Facade_ParkPosition` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `FcdParkEn` | `FB_BA_BV_Op` | — |
| `FcdParkPos` | `FB_BA_AV_Op` | — |
| `FcdParkAngl` | `FB_BA_AV_Op` | — |
| `FcdParkPosEvent` | `FB_BA_SunBldEvt := (ePrio:=E_BA_SunBldPrio.eParkPosition)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Parking position telegram. |
| `bParkPosEvent` | `BOOL` | Control output "Parking position enabled". Programming Inputs CONSTANT |
| `FcdParkEn` | `FB_BA_BV_Op [} 216]` | Function block for activating the park position. |
| `FcdParkPos` | `FB_BA_AV_Op [} 206]` | Function block for entering the position. |
| `FcdParkAngl` | `FB_BA_AV_Op [} 206]` | Function block for entering the angle. |
| `FcdParkPosEvent` | `FB_BA_SunBldEvt [} 377]` | Function block for telegram generation. |

**Description:** Sub-template Park position.

##### `FB_BA_Facade_SunProtection` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bFcdInSun` | `BOOL` | The facade is located in the sun area in relation to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Parking position telegram. |
| `bSunPrtcActv` | `BOOL` | Control output "Sun protection active". |
| `SunPrtcValOn00deg FB_BA_AV_Op [} 206]` | `Brightness thresholds for sun protection On [lx] for` | — |
| `…` | `elevations of 0°, 15°, 30°, 45°, 60°, 75° and 90°. SunPrtcValOn90deg` | — |
| `SunPrtcValOff00deg FB_BA_AV_Op [} 206]` | `Brightness thresholds for sun protection Off [lx] for` | — |
| `…` | `elevations of 0°, 15°, 30°, 45°, 60°, 75° and 90°. SunPrtcValOff90deg` | — |
| `SunPrtcValOnCalc FB_BA_AV_Op [} 206]` | `Calculated brightness threshold value for sun protection` | — |

**Description:** Sub-template Automatic sun protection.

##### `FB_BA_Facade_ThermoAutomatic` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bFcdInSun` | `BOOL` | The facade is located in the sun area in relation to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Thermal automatic telegram. |
| `bThAutoEvent` | `BOOL` | Control output "facade-wide thermal automatic active". Inputs CONSTANT |
| `FcdThAutoEn` | `FB_BA_BV_Op [} 216]` | Facade thermal automatic enable. |
| `FcdThAutoColPos` | `FB_BA_AV_Op [} 206]` | Facade thermal automatic cooling position Position [%]. |
| `FcdThAutoColAngl` | `FB_BA_AV_Op [} 206]` | Facade thermal automatic cooling position Angle [°]. |
| `FcdThAutoEvent` | `FB_BA_SunBldEvt [} 377]` | Telegram block for facade thermal automatic. Variables |
| `FcdThAutoHtgCol` | `FB_BA_Swi2P` | Hysteresis block for heating/cooling changeover for facade |

**Description:** Sub-template facade-wide thermal automatic.

##### `FB_BA_Facade_TwilightAutomatic` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `FcdTwiLgtAutoEn` | `FB_BA_BV_Op` | — |
| `FcdTwiLgtAutoEvent` | `FB_BA_SunBldEvt :=(ePrio:=E_BA_SunBldPrio.eFacadeTwilightAutomatic)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Twilight automatic telegram. |
| `bTwiLgtAutoEvent` | `BOOL` | Control output "facade-wide twilight automatic active". |
| `FcdTwiLgtAutoEn` | `FB_BA_BV_Op [} 216]` | Function block for activating the twilight automatic |

**Description:** Sub-template facade-wide twilight automatic.

##### `FB_BA_Facade_WindProtection` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aWndSpdLmtStormOn` | `ARRAY[1..32] OF REAL` | — |
| `aWndSpdLmtStormOff` | `ARRAY[1..32] OF REAL` | — |
| `DlyStrmOn` | `FB_BA_AV_Op` | — |
| `DlyStrmOff` | `FB_BA_AV_Op` | — |
| `WndPrtcState` | `FB_BA_BV_Op` | — |
| `SunBldWndPrtc` | `FB_BA_SunBldWndPrtc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBld` | `ST_BA_SunBld [} 272]` | Storm protection telegram. |
| `bWndPrtc` | `BOOL` | Control output "Storm protection active". |
| `fWndSpdStormOn` | `REAL` | Calculated value limit value storm detection. |
| `fWndSpdStormOff` | `REAL` | Calculated value limit value Storm abated. Inputs CONSTANT PERSISTENT |
| `aWndSpdLmtStorm ARRAY[1..32] OF REAL` | `aWndSpdLmtStormOn[01] … aWndSpdLmtStormOn[13]:` | — |
| `On` | `These values define the interpolation curve for the` | — |

**Description:** Sub-template Storm protection.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › SunProtection › Functions › Zone

##### `FB_BA_RollerblindZone` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fRoomTempSen` | `REAL` | Room temperature sensor [°C]. |
| `bPrcDetc` | `BOOL` | Presence detection. |
| `bSwiUp` | `BOOL` | Key function Blind "up". |
| `bSwiDwn` | `BOOL` | Key function Blind "down". |
| `bRstManFnct` | `BOOL` | Deletion input for the blind manual flag. |
| `stBuildingSpRmT` | `ST_BA_SpRmT [} 270]` | Structure of room setpoints (Protection Cooling...Comfort |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBldOut` | `ST_BA_Sunbld [} 272]` | Resulting zone telegram. Programming Inputs CONSTANT PERSISTENT |
| `bThAutoSlcn` | `BOOL` | Thermal automatic enable. |
| `fThAutoColPos` | `REAL` | Thermal automatic cooling position [%]. |
| `bSunPrtcSlcn` | `BOOL` | Shading automatic enable. |
| `bTwiLgtAutoSlcn` | `BOOL` | Twilight automatic enable. |
| `fTwiLgtAutoPos` | `REAL` | Twilight automatic cooling position [%]. |
| `nSwiOverTi` | `UDINT` | Push button function of the zone: Switchover time [ms] |

**Description:** In a blind zone, one or more blinds are combined for simultaneous control, depending on the local conditions. The template bundles the predefined data of the facades and decides on the basis of a zone presence detection and local selection and deselection which functionality is active.

##### `FB_BA_SunblindZone` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fRoomtempSen` | `REAL` | Room temperature sensor [°C]. |
| `bPrcDetc` | `BOOL` | Presence detection. |
| `bSwiUp` | `BOOL` | Local push button "up". |
| `bSwiDwn` | `BOOL` | Local push button "down". |
| `bRstManFnct` | `BOOL` | Input for resetting all internal manual functions, both those |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunBldOut` | `ST_BA_SunBld [} 272]` | resulting zone telegram. Programming Inputs CONSTANT PERSISTENT |
| `bThAutoSlcn` | `BOOL` | Thermal automatic enable. |
| `fThAutoColPos` | `REAL` | Thermal automatic cooling position [%]. |
| `fThAutoColAngl` | `REAL` | Thermal automatic cooling angle [°]. |
| `bSunPrtcSlcn` | `BOOL` | Shading automatic enable. |
| `bTwiLgtAutoSlcn` | `BOOL` | Twilight automatic enable. |
| `fTwiLgtAutoPos` | `REAL` | Twilight automatic position [%]. |
| `fTwiLgtAutoAngl` | `REAL` | Twilight automatic angle [°]. |
| `nSwiOverTi` | `UDINT` | Changeover time [ms] of the manual switch SunBldSwi for |

**Description:** In a blind zone, one or more blinds are combined for simultaneous control. The template FB_BA_SunblindZone bundles the predefined data of the facades and decides on the basis of a zone presence detection and local selection and deselection which functionality is active.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation › SunProtection › Actuators

##### `FB_BA_RolBld` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable the function block function. |
| `stSunBldCmd` | `ST_BA_SunBld [} 272]` | Resulting telegram from the higher-level zone (room). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReferencing` | `BOOL` | Blind is being referenced. |
| `nReferenceTimer` | `UDINT` | Elapsed referencing time [s]. |
| `bErr` | `BOOL` | The internal function block FB_BA_RolBldActr is |

**Description:** This template is used to control a blind actuator without slat adjustment.

##### `FB_BA_SunBld` (FUNCTION_BLOCK) — §6.1.4.3.1.7.6.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable the function block function. |
| `stSunBldCmd` | `ST_BA_SunBld [} 272]` | Resulting telegram from the higher-level zone (room). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReferencing` | `BOOL` | Blind is being referenced. |
| `nReferenceTimer` | `UDINT` | Elapsed referencing time [s]. |
| `bErr` | `BOOL` | The internal function block FB_BA_SunBldActr is |

**Description:** This template is used to control a blind actuator with slat adjustment.

#### Programming › PLC › Templates › POUs › Trade › RoomAutomation

##### `FB_BA_RoomAutomationClient` (FUNCTION_BLOCK) — §6.1.4.3.1.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AdsComClientRoom` | `FB_BA_AdsComClient_Room` | — |
| `SampleRoom` | `FB_BA_RoomSample1` | — |

**Description:** Call template trade "Room automation" - client version.

##### `FB_BA_RoomAutomationServer` (FUNCTION_BLOCK) — §6.1.4.3.1.7.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AdsComServerRoom` | `FB_BA_AdsComServer_Room` | — |
| `BuildingData` | `FB_BA_BuildingData` | — |

**Description:** Call template trade "Room automation" - server version.

#### Programming › PLC › Templates › POUs › Universal › Aggregates › Analog

##### `FB_BA_ActuatorAnalog` (FUNCTION_BLOCK) — §6.1.4.3.2.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Analog [} 274]` | The enables and switching values are transmitted to the template via the command structure stCmd . The |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fMdlt` | `REAL` | Current value of the analog output object. |
| `bDst` | `BOOL` | The variable is an evaluation of the lock priorities "Safety" |

**Description:** Programming

##### `FB_BA_ActuatorAnalogTLog` (FUNCTION_BLOCK) — §6.1.4.3.2.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Analog [} 274]` | The enables and switching values are transmitted to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fMdlt` | `REAL` | Current value of the analog output object. |
| `bDst` | `BOOL` | The variable is an evaluation of the lock priorities "Safety" |

**Description:** The FB_BA_ActuatorAnalogTLog template is used to control analog aggregates. It essentially consists of an AO object for controlling an aggregate and the function block PlantLock, which collects all safety-relevant faults.

##### `FB_BA_ActuatorAV` (FUNCTION_BLOCK) — §6.1.4.3.2.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Analog [} 274]` | The enables and switching values are transmitted to the template via the command structure stCmd . The |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current value of the analog value object. |
| `bDst` | `BOOL` | The variable is an evaluation of the lock priorities "Safety" |

**Description:** The template is used to control analog aggregates. It essentially consists of an AV object for controlling an aggregate and the PlantLock function block, which collects all safety-relevant faults.

#### Programming › PLC › Templates › POUs › Universal › Aggregates › Binary

##### `FB_BA_ActuatorCmd` (FUNCTION_BLOCK) — §6.1.4.3.2.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Binary [} 275]` | The enables and switching values are transmitted to the template via the command structure stCmd . The |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bCmd` | `BOOL` | Current value of the binary output object. |
| `bDst` | `BOOL` | The variable is an evaluation of the lock priorities "Safety" |

**Description:** The template is used to control binary aggregates. It essentially consists of a BO object for controlling an aggregate and the function block PlantLock, which collects all safety-relevant faults.

##### `FB_BA_ActuatorCmdTLog` (FUNCTION_BLOCK) — §6.1.4.3.2.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Cmd` | `FB_BA_BO_Raw` | — |
| `TLogCmd` | `FB_BA_Trend` | — |
| `PlantLock` | `FB_BA_PlantLock` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Binary [} 275]` | The enables and switching values of the priorities are |

**Description:** The FB_BA_ActuatorCmdTLog template is used to control binary aggregates. It essentially consists of a BO object for controlling an aggregate and the function block PlantLock, which collects all safety-relevant faults.

#### Programming › PLC › Templates › POUs › Universal › Aggregates › Modulation

##### `FB_BA_ActuatorMdlt` (FUNCTION_BLOCK) — §6.1.4.3.2.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Mdlt [} 274]` | The enables and modulation commands are transmitted to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fMdlt` | `REAL` | Current value of the analog output object. |
| `bDst` | `BOOL` | The variable is an evaluation of the lock priorities "Safety" |

**Description:** The template is used to control modulating aggregates. It essentially consists of an AO object for controlling an aggregate, the priority switch PrioSwiVal for determining the modulation command, the multiplexer MuxMdlt for determining the control value and the function block PlantLock, which collects all safety-related faults.

#### Programming › PLC › Templates › POUs › Universal › Aggregates › Multistate

##### `FB_BA_ActuatorMO` (FUNCTION_BLOCK) — §6.1.4.3.2.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Multistate [} 273]` | The enables and switching values are transmitted to the template via the command structure stCmd . The |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nCmd` | `UDINT` | Current switch value of the multistate output object. |
| `bDst` | `BOOL` | The variable is an evaluation of the lock priorities "Safety" |

**Description:** The template is used to control multi-stage aggregates. It essentially consists of an MO object for controlling an aggregate and the function block PlantLock, which collects all safety-relevant faults.

##### `FB_BA_ActuatorMultistate` (FUNCTION_BLOCK) — §6.1.4.3.2.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Multistate [} 273]` | The enables and switching values are transmitted to the template via the command structure stCmd . The |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nCmd` | `UDINT` | Current switch value of the multistate output object. |
| `bDst` | `BOOL` | The variable is an evaluation of the lock priorities "Safety" |

**Description:** The template is used to control multi-stage aggregates. It essentially consists of an MO object for controlling an aggregate and the function block PlantLock, which collects all safety-relevant faults.

#### Programming › PLC › Templates › POUs › Universal › Control › Analog3Point

##### `FB_BA_Analog3Pnt` (FUNCTION_BLOCK) — §6.1.4.3.2.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fValPgm` | `REAL` | Control value for the position of the actuator. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOpn` | `BOOL` | Output for opening the actuator. |
| `bCls` | `BOOL` | Output for closing the actuator. |
| `HysPos` | `FB_BA_AV_Op [} 206]` | AV object for entering the hysteresis value to start the |

**Description:** The template converts a continuous control signal for the positioning of a 3-point actuator into the binary switching commands Open/Close.

#### Programming › PLC › Templates › POUs › Universal › Control › AntiBlocking

##### `FB_BA_AntiBlocking` (FUNCTION_BLOCK) — §6.1.4.3.2.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bFdb` | `BOOL` | Input for connecting the feedback signal of a motor or |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Output of the anti-lock protection pulse. Inputs CONSTANT |
| `IdlePrd` | `FB_BA_AV_Op [} 206]` | AV object for input of the maximum pump standstill |

**Description:** The template prevents blocking of pumps or actuators after prolonged idle periods by issuing a switch-on pulse.

#### Programming › PLC › Templates › POUs › Universal › Control › ContinuousSteps

##### `FB_BA_Cont4Steps` (FUNCTION_BLOCK) — §6.1.4.3.2.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template. If bEn is FALSE, all |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ01` | `BOOL` | Shows status level 01 |
| `bQ02` | `BOOL` | Shows status level 02 |
| `bQ03` | `BOOL` | Shows status level 03 |
| `bQ04` | `BOOL` | Shows status level 04 Inputs CONSTANT |
| `SwitchOn01` | `FB_BA_AV_Op [} 206]` | AV object for entering the switch-on point step 01. |
| `SwitchOn02` | `FB_BA_AV_Op [} 206]` | AV object for entering the switch-on point step 02. |
| `SwitchOn03` | `FB_BA_AV_Op [} 206]` | AV object for entering the switch-on point step 03. |
| `SwitchOn04` | `FB_BA_AV_Op [} 206]` | AV object for entering the switch-on point step 04. |
| `Hys` | `FB_BA_AV_Op [} 206]` | AV object for entering the hysteresis for the switch-on |

**Description:** The template determines the resulting control steps of a 4-level aggregate, depending on the continuous input signal fIn.

#### Programming › PLC › Templates › POUs › Universal › Control › EventClasses

##### `FB_BA_EvtCategory` (FUNCTION_BLOCK) — §6.1.4.3.2.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `EC` | `FB_BA_EC` | — |

**Description:** The template contains a Notification Class Object.

#### Programming › PLC › Templates › POUs › Universal › Control

##### `Filter` (FUNCTION_BLOCK) — §6.1.4.3.2.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `REAL` | Input signal. |
| `bSync` | `BOOL` | A rising edge at this input switches the output value fOut to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `REAL` | Attenuated output signal. Inputs CONSTANT |
| `DampingConstant` | `FB_BA_AV_Op [} 206]` | AV object for entering filter time constant. Variables |
| `PT1` | `FB_BA_FltrPT1` | Notification Class Object. Programming |

**Description:** First order filter.

##### `Hysteresis` (FUNCTION_BLOCK) — §6.1.4.3.2.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the function block. |
| `fIn` | `REAL` | Actual value |
| `bEn` | `BOOL` | General enable of the function block. |
| `fIn` | `REAL` | Actual value |
| `fOn` | `REAL` | Dynamic switch-on point |
| `fOff` | `REAL` | Dynamic switch-off point |
| `bEn` | `BOOL` | General enable of the function block. |
| `fIn` | `REAL` | Process value |
| `bEn` | `BOOL` | General enable of the function block. |
| `fIn` | `REAL` | Actual value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Output of the current state of the hysteresis function. |
| `bDst` | `BOOL` | Display of a fault or the BV object is active. |
| `bQ` | `BOOL` | Output of the current state of the hysteresis function. |
| `bDst` | `BOOL` | Display of a fault or the BV object is active. |
| `bPresentValue` | `BOOL` | Output of the current state of the hysteresis function. |
| `bEvent` | `BOOL` | Display of a fault or the BV object is active. bEvent is only active if the property |
| `bDst` | `BOOL` | Display of a fault or the BV object is active. |

**Description:** The template represents a hysteresis function with fixed switching points.

#### Programming › PLC › Templates › POUs › Universal › Control › Monitoring

##### `FB_BA_FdbAnalogExt` (FUNCTION_BLOCK) — §6.1.4.3.2.2.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template. |
| `bCommand` | `BOOL` | The switching actuator output of the aggregate to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDst` | `BOOL` | Binary object for displaying the fault. Inputs CONSTANT |
| `SwitchOnValue` | `FB_BA_AV_Op [} 206]` | Analog value object for entering the switch-on value for |

**Description:** The template is used to monitor analog values with dynamic switch values, e.g. for differential pressure monitoring of a fan.

##### `FB_BA_FdbAnalog` (FUNCTION_BLOCK) — §6.1.4.3.2.2.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template. |
| `bCommand` | `BOOL` | The switching actuator output of the aggregate to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDst` | `BOOL` | Binary object for displaying the fault. Inputs CONSTANT |
| `SwitchOnValue` | `FB_BA_AV_Op [} 206]` | Analog value object for entering the switch-on value for |

**Description:** The template is used to monitor analog values with fixed switch values, such as differential pressure monitoring of a fan.

##### `FB_BA_FdbBinary` (FUNCTION_BLOCK) — §6.1.4.3.2.2.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template. |
| `bCommand` | `BOOL` | The switching actuator output of the aggregate to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDst` | `BOOL` | Binary object for displaying the fault. Inputs CONSTANT |
| `DelayFeedback` | `FB_BA_AV_Op [} 206]` | Analog value object for entering the time delay of the |

**Description:** The template is used to monitor binary feedback signals such as the end positions of dampers or valves. However, it can also be used for differential pressure monitoring by means of a differential pressure monitor.

##### `FB_BA_FixedLimitExt` (FUNCTION_BLOCK) — §6.1.4.3.2.2.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template. |
| `fIn` | `REAL` | The analog value to be monitored is connected to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bHighLimit` | `BOOL` | The output indicates that the upper limit value of the |

**Description:** The template represents a limit switch with dynamic limit values.

##### `FB_BA_FixedLimit` (FUNCTION_BLOCK) — §6.1.4.3.2.2.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template. |
| `fIn` | `REAL` | The analog value to be monitored is connected to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bHighLimit` | `BOOL` | The output indicates that the upper limit value of the |

**Description:** The template represents a limit switch with fixed limits.

##### `FB_BA_PresenceMonitoring` (FUNCTION_BLOCK) — §6.1.4.3.2.2.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresence` | `BOOL` | The presence signal input is passed on to the output |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bRstSwi` | `BOOL` | Reset output for manual overrides. |
| `bPresenceState` | `BOOL` | Switch-off delayed presence state. |
| `nCountDownPresen UDINT` | `Remaining time of the delay timer in seconds. ce Programming Variables` | — |
| `tofPrcDetc` | `TOF` | Switch-off delay presence. |
| `rtRstDelayTimer` | `R_TRIG` | Trigger for the reset of the timer. |
| `rtRstManMod` | `R_TRIG` | Trigger for the reset of the manual override. |
| `ftPrc` | `F_TRIG` | Trigger for the reset of the manual override, but controlled |

**Description:** Universal presence monitoring template with reset inputs for delay timer and manual function.

##### `FB_BA_SlidingLimit` (FUNCTION_BLOCK) — §6.1.4.3.2.2.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template. |
| `fActuator` | `REAL` | The base value of the tolerance range is connected to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bHighLimit` | `BOOL` | The output indicates that the upper limit value of the |

**Description:** The template represents a sliding limit value monitoring.

#### Programming › PLC › Templates › POUs › Universal › Control › Object

##### `FB_BA_AV_Op_SP` (FUNCTION_BLOCK) — §6.1.4.3.2.2.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SP` | `FB_BA_AV_Op` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Entered value. Inputs CONSTANT |
| `SP` | `FB_BA_AV_Op [} 206]` | Analog value object for entering the setpoint. |

**Description:** The template FB_BA_AV_Op_SP is used to enter a setpoint within a template at system level.

##### `FB_BA_BV_ErrorState` (FUNCTION_BLOCK) — §6.1.4.3.2.2.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bValPgm` | `BOOL` | — |
| `ErrorState` | `FB_BA_BV` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBVEvent` | `BOOL` | Binary state of the message. |

**Description:** The template is an error state message within a template at system level.

##### `FB_BA_BV_Op_Val` (FUNCTION_BLOCK) — §6.1.4.3.2.2.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Val` | `FB_BA_BV_Op` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Binary state of the switch. Inputs CONSTANT |
| `Val` | `FB_BA_BV_Op [} 216]` | Binary value object to represent the switch. |

**Description:** The template FB_BA_BV_OP_Val is a switch within a template at system level.

#### Programming › PLC › Templates › POUs › Universal › Control › OperatingMode

##### `FB_BA_OpMod1St` (FUNCTION_BLOCK) — §6.1.4.3.2.2.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bValPgm` | `BOOL` | A plant can be switched on or off via the input in the |
| `bValPgm` | `BOOL` | A plant can be switched on or off via the input in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eOpMod` | `E_BA_OnOff [} 710]` | Displays the state of the currently valid plant operating |
| `eOpMod` | `E_BA_OnOff [} 710]` | Displays the state of the currently valid plant operating |

**Description:** The template maps a single-stage plant selector switch with the operating modes Auto, Manual Off and

#### Programming › PLC › Templates › POUs › Universal › Control › PID

##### `FB_BA_PID_Sp` (FUNCTION_BLOCK) — §6.1.4.3.2.2.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template |
| `fX` | `REAL` | Actual value |
| `bSync` | `BOOL` | A rising edge at this input triggers synchronization of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fY` | `REAL` | Control value output |
| `fMax` | `REAL` | Maximum controller value |
| `fMin` | `REAL` | Minimum controller value Inputs CONSTANT |
| `Sp` | `FB_BA_AV_Op [} 206]` | Input of the setpoint |
| `Ctrl` | `FB_BA_Loop [} 220]` | PID controller |

**Description:** The template is a universal PID controller.

##### `FB_BA_PID` (FUNCTION_BLOCK) — §6.1.4.3.2.2.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template |
| `fW` | `REAL` | Setpoint |
| `fX` | `REAL` | Actual value |
| `bSync` | `BOOL` | A rising edge at this input triggers synchronization of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fY` | `REAL` | Control value output |
| `fMax` | `REAL` | Maximum controller value |
| `fMin` | `REAL` | Minimum controller value Inputs CONSTANT |
| `Ctrl` | `FB_BA_Loop [} 220]` | PID controller |

**Description:** The template is a universal PID controller.

#### Programming › PLC › Templates › POUs › Universal › Control › Ramp

##### `FB_BA_RampLimitDecrease` (FUNCTION_BLOCK) — §6.1.4.3.2.2.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template. |
| `bTrig` | `BOOL` | The ramp limitation is activated by a rising edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fY` | `REAL` | Output of the ramp limiting value. Inputs CONSTANT |
| `TiDown` | `FB_BA_AV_Op [} 206]` | AV object for entering the fall time of the ramp limitation. |
| `Y` | `FB_BA_AV_Op [} 206]` | AV object for displaying the ramp limitation value. Programming Variables |
| `RampFnct` | `FB_BA_RampLmt` | The function block for the output of a ramp limitation is the |

**Description:** The template implements a falling ramp limitation from 100 to 0 and remains at this value.

##### `FB_BA_RampLimitIncrease` (FUNCTION_BLOCK) — §6.1.4.3.2.2.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the template. |
| `bTrig` | `BOOL` | The ramp limitation is activated by a rising edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fY` | `REAL` | Output of the ramp limiting value. Inputs CONSTANT |
| `TiUp` | `FB_BA_AV_Op [} 206]` | AV object for entering the time for the ramp limitation to |

**Description:** The template implements a rising ramp limitation from 0 to 100 and remains at this value.

#### Programming › PLC › Templates › POUs › Universal › Control

##### `Scale` (FUNCTION_BLOCK) — §6.1.4.3.2.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fX` | `REAL` | Input value of the characteristic curve. |
| `fX` | `REAL` | Input value of the characteristic curve. |
| `fX` | `REAL` | Input value of the characteristic curve. |
| `fTWth` | `REAL` | Current value of the outside temperature. |
| `fX` | `REAL` | Input value of the characteristic curve. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fY` | `REAL` | Calculated output value of the characteristic curve. |
| `X1` | `FB_BA_AV_Op [} 206]` | The AV object is used to specify the value for interpolation |
| `fY` | `REAL` | Calculated output value of the characteristic curve. Inputs CONSTANT |
| `X1` | `FB_BA_AV_Op [} 206]` | The AV object is used to specify the value for interpolation |
| `fY` | `REAL` | Calculated output value of the characteristic curve. Inputs CONSTANT |
| `X1` | `FB_BA_AV_Op [} 206]` | The AV object is used to specify the value for interpolation |
| `fY` | `REAL` | Calculated setpoint for the room temperature. Inputs CONSTANT |
| `X1` | `FB_BA_AV_Op [} 206]` | The AV object is used to specify the value for interpolation |
| `fY` | `REAL` | Calculated output value of the characteristic curve. Programming Inputs CONSTANT |
| `X1` | `FB_BA_AV_Op [} 206]` | The AV object is used to specify the value for interpolation |

**Description:** Templates for scaling analog values.

#### Programming › PLC › Templates › POUs › Universal › Control › TimeDelay

##### `FB_BA_DlyOff` (FUNCTION_BLOCK) — §6.1.4.3.2.2.13.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bVal` | `BOOL` | A falling edge activates the switch-off delay. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | State of the switch-off delay. |
| `nRT` | `UDINT` | Remaining time of the switch-off delay. After this time has |

**Description:** The template implements a switch-off delay and can be used for the overrun control of pumps.

##### `FB_BA_DlyOn` (FUNCTION_BLOCK) — §6.1.4.3.2.2.13.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bVal` | `BOOL` | A rising edge activates the start-up delay. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | State of the start-up delay. |
| `nRT` | `UDINT` | Remaining time of the start-up delay. After this time has |

**Description:** The template implements a start-up delay.

#### Programming › PLC › Templates › POUs › Universal › Control › Trend

##### `FB_BA_TrendLogging` (FUNCTION_BLOCK) — §6.1.4.3.2.2.14.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Trends` | `ARRAY[1..TREND_COUNT] OF FB_BA_Trend` | — |

**Description:** The template logs the values from a data source and records them in the Trendlog memory. The connection to the Trendlog objects (data sources, e.g. FB_BA_BO_IO [} 211], FB_BA_AO_IO [} 202]) is made via references.

#### Programming › PLC › Templates › POUs › Universal › Control › VolumeFlowDetermination

##### `FB_BA_V_dP_kFactor` (FUNCTION_BLOCK) — §6.1.4.3.2.2.15.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `f_dP` | `REAL` | Detection of the differential pressure [Pa]. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fV` | `REAL` | Output of the calculated volume flow [m³/h]. |

**Description:** The template is used to determine the volume flow by means of differential pressure and k-factor.

#### Programming › PLC › Templates › POUs › Universal › Damper › 2P

##### `FB_BA_Dmp2P` (FUNCTION_BLOCK) — §6.1.4.3.2.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SwiOpn` | `FB_BA_BI_Raw` | — |
| `SwiCls` | `FB_BA_BI_Raw` | — |
| `MonitOpen` | `FB_BA_FdbBinary` | — |
| `MonitClose` | `FB_BA_FdbBinary` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwiOpn` | `BOOL` | End position "Open" has been reached. |
| `bSwiCls` | `BOOL` | End position "Closed" has been reached. Inputs CONSTANT |
| `SwiOpn` | `FB_BA_BI_Raw [} 209]` | Binary input object is used to process the limit switch |

**Description:** The template is for the control of a two-point damper with integrated monitoring of both end positions.

#### Programming › PLC › Templates › POUs › Universal › Motor › 1st

##### `FB_BA_Mot1st` (FUNCTION_BLOCK) — §6.1.4.3.2.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Dst` | `FB_BA_BI_Raw` | — |

**Description:** The template is used for the control of a single-step motor with fault signal, e.g. a fan.

#### Programming › PLC › Templates › POUs › Universal › Motor › 2st

##### `FB_BA_Mot2st` (FUNCTION_BLOCK) — §6.1.4.3.2.4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DstStp01` | `FB_BA_BI_Raw` | — |
| `DstStp02` | `FB_BA_BI_Raw` | — |
| `FdbStp01` | `FB_BA_BI_Raw` | — |
| `FdbStp02` | `FB_BA_BI_Raw` | — |
| `MoniFdbStp01` | `FB_BA_FdbBinary` | — |
| `MoniFdbStp02` | `FB_BA_FdbBinary` | — |
| `StpCtrl` | `FB_BA_MotorStpCtrl` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nPresentValue` | `UDINT` | Current control step of the motor. |
| `bQ01` | `BOOL` | Variable for controlling step 1 of the motor. This variable |

**Description:** The template is used for the control of a two-step motor with fault signal, e.g. a fan.

#### Programming › PLC › Templates › POUs › Universal › Motor › 3st

##### `FB_BA_Mot3st` (FUNCTION_BLOCK) — §6.1.4.3.2.4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DstStp01` | `FB_BA_BI_Raw` | — |
| `DstStp02` | `FB_BA_BI_Raw` | — |
| `DstStp03` | `FB_BA_BI_Raw` | — |
| `FdbStp01` | `FB_BA_BI_Raw` | — |
| `FdbStp02` | `FB_BA_BI_Raw` | — |
| `FdbStp03` | `FB_BA_BI_Raw` | — |
| `MoniFdbStp01` | `FB_BA_FdbBinary` | — |
| `MoniFdbStp02` | `FB_BA_FdbBinary` | — |
| `MoniFdbStp03` | `FB_BA_FdbBinary` | — |
| `StpCtrl` | `FB_BA_MotorStpCtrl` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nPresentValue` | `UDINT` | Current control step of the motor. |
| `bQ01` | `BOOL` | Variable for controlling step 1 of the motor. This variable |

**Description:** The template is used for the control of a three-step motor with fault signal, e.g. a fan.

#### Programming › PLC › Templates › POUs › Universal › Motor › Control

##### `FB_BA_MotCtl` (FUNCTION_BLOCK) — §6.1.4.3.2.4.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `REAL` | The actual value for the controller Ctrl is connected to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bCmd` | `BOOL` | Output of the switch value. |
| `fMdlt` | `REAL` | Current value of the analog output object. |
| `bDst` | `BOOL` | The variable is an evaluation of the lock priorities "Safety" |

**Description:** The template is used to control and regulate a speed-controlled motor.

##### `FB_BA_MotCtlExt` (FUNCTION_BLOCK) — §6.1.4.3.2.4.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ThOvrld` | `FB_BA_BI_Raw` | — |
| `MntnSwi` | `FB_BA_BI_Raw` | — |

**Description:** The template is used to control and regulate a speed-controlled motor. It consists essentially of the base class FB_BA_MotCtl [} 1055].

#### Programming › PLC › Templates › POUs › Universal › Motor › Modulation

##### `FB_BA_MotMdlt` (FUNCTION_BLOCK) — §6.1.4.3.2.4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCmd` | `ST_BA_Mdlt [} 274]` | The enables and modulation commands are transmitted to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bCmd` | `BOOL` | Output of the switch value. |
| `fMdlt` | `REAL` | Current value of the analog output object. |
| `bDst` | `BOOL` | The variable is an evaluation of the lock priorities "Safety" |

**Description:** The template is used to control a speed-controlled motor.

##### `FB_BA_MotMdltExt` (FUNCTION_BLOCK) — §6.1.4.3.2.4.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Sp` | `FB_BA_AV_Op` | — |

**Description:** The template is used to control a speed-controlled motor. It consists essentially of the base class FB_BA_MotMdlt [} 1060].

#### Programming › PLC › Templates › POUs › Universal › Pump › 1st

##### `FB_BA_Pu1st` (FUNCTION_BLOCK) — §6.1.4.3.2.5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bAblkLock` | `BOOL` | A TRUE at this input variable interrupts the anti-blocking |

**Description:** The template is used to control a single-stage pump with binary inputs and outputs. It mainly consists of the base class FB_BA_Mot1st [} 1050], the switch-off delay DlyOff [} 1043] and the anti-blocking protection function Ablk [} 990].

##### `FB_BA_Pu1stExt` (FUNCTION_BLOCK) — §6.1.4.3.2.5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ThOvrld` | `FB_BA_BI_Raw` | — |
| `MntnSwi` | `FB_BA_BI_Raw` | — |

**Description:** The template is used to control a single-stage pump with binary inputs and outputs. It consists essentially of the base class FB_BA_Pu1st [} 1064].

#### Programming › PLC › Templates › POUs › Universal › Pump › Control

##### `FB_BA_PuCtl` (FUNCTION_BLOCK) — §6.1.4.3.2.5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bAblkLock` | `BOOL` | A TRUE at this input variable interrupts the anti-blocking |

**Description:** The template is used to control and regulate a speed-controlled pump with analog and binary inputs and outputs. It mainly consists of the base classFB_BA_MotCtl [} 1055], the switch-off delay DlyOff [} 1043] and the anti-blocking protection function Ablk [} 990]. The template FlowMonit stands for the monitoring of the flow monitor FlowSwi.

#### Programming › PLC › Templates › POUs › Universal › Pump › Modulation

##### `FB_BA_PuMdlt` (FUNCTION_BLOCK) — §6.1.4.3.2.5.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bAblkLock` | `BOOL` | A TRUE at this input variable interrupts the anti-blocking |

**Description:** The template is used to control a speed-controlled pump with analog and binary inputs and outputs. It mainly consists of the base classFB_BA_MotMdlt [} 1060], the switch-off delay DlyOff [} 1043] and the anti-blocking protection function Ablk [} 990]. The template FlowMonit stands for the monitoring of the flow monitor FlowSwi.

##### `FB_BA_PuMdltExt` (FUNCTION_BLOCK) — §6.1.4.3.2.5.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bAblkLock` | `BOOL` | A TRUE at this input variable interrupts the anti-blocking |

**Description:** The template is used to control a speed-controlled pump with analog and binary inputs and outputs. It mainly consists of the base class FB_BA_MotMdltExt [} 1063], the switch-off delay DlyOff [} 1043] and the anti- blocking protection function Ablk [} 990]. The template FlowMonit stands for the monitoring of the flow monitor FlowSwi.

#### Programming › PLC › Templates › POUs › Universal › Sensor

##### `FB_BA_CombinationBI_BV` (FUNCTION_BLOCK) — §6.1.4.3.2.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `FB_BA_BI_Raw` | — |
| `Value` | `FB_BA_BV_Op` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBI_PrVal` | `BOOL` | Current value of the binary input object Input. |
| `bTrigBI_PrVal` | `BOOL` | Current value of the output Qr of the function block |

**Description:** The template is a combination of a BI and BV object.

##### `FB_BA_MeasuredValueCardinal` (FUNCTION_BLOCK) — §6.1.4.3.2.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fVal_N` | `REAL` | Analog input value for "North". |
| `fVal_S` | `REAL` | Analog input value for "South". |
| `fVal_E` | `REAL` | Analog input value for "East". |
| `fVal_W` | `REAL` | Analog input value for "West". Inputs CONSTANT |
| `MV_N` | `FB_BA_AI [} 196]` | Analog input object for the "North" value. |
| `MV_S` | `FB_BA_AI [} 196]` | Analog input object for the "South" value. |
| `MV_E` | `FB_BA_AI [} 196]` | Analog input object for the "East" value. |
| `MV_W` | `FB_BA_AI [} 196]` | Analog input object for the "West" value. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValueN` | `REAL` | Output value for "North". |
| `fPresentValueS` | `REAL` | Output value for "South". |
| `fPresentValueE` | `REAL` | Output value for "East". |
| `fPresentValueW` | `REAL` | Output value for "West". |

**Description:** This template represents at the output, in a hierarchical representation, a collection of four values, which are assigned to the cardinal points. This template can be used to display the external brightnesses for all four cardinal points on one display layer, as shown in the following graphic.

##### `FB_BA_SensorAnalog` (FUNCTION_BLOCK) — §6.1.4.3.2.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MV` | `FB_BA_AI_Raw` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current value of the analog input object MV for displaying |

**Description:** The function block MV, within the template, records an analog input value from an I/O bus terminal and converts it into a real process value.

##### `FB_BA_SensorAnalogTLog` (FUNCTION_BLOCK) — §6.1.4.3.2.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MV` | `FB_BA_AI_Raw` | — |
| `TLogMV` | `FB_BA_Trend` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPresentValue` | `REAL` | Current value of the analog input object MV for displaying |

**Description:** The FB_BA_SensorAnalogTLog function block logs an analog input value from an I/O bus terminal and converts it into a real process value.

##### `FB_BA_SensorBinary` (FUNCTION_BLOCK) — §6.1.4.3.2.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `FB_BA_BI_Raw` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresentValue` | `BOOL` | Current value of the binary input object Input to display a |

**Description:** The function block Input logs a binary input value from an I/O bus terminal and outputs it as a boolean process value.

##### `FB_BA_Button2Way` (FUNCTION_BLOCK) — §6.1.4.3.2.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Swi1` | `FB_BA_BI_Raw` | — |
| `Swi2` | `FB_BA_BI_Raw` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresentValue1` | `BOOL` | Output: state switch 1. |
| `bPresentValue2` | `BOOL` | Output: state switch 2. Programming |

**Description:** The template is used to directly link the inputs of a dual push button. The states are represented on a hierarchy level with two objects and made available to the project via the outputs bPresentValue1 and bPresentValue2. This template is used, for example, for the integration of blind buttons.

##### `FB_BA_SensorBinaryTLog` (FUNCTION_BLOCK) — §6.1.4.3.2.6.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `FB_BA_BI_Raw` | — |
| `TLogInput` | `FB_BA_Trend` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPresentValue` | `BOOL` | Current value of the binary input object Input to display a |

**Description:** The FB_BA_SensorBinaryTLog function block logs a binary input value from an I/O bus terminal and outputs it as a Boolean process value.

#### Programming › PLC › Templates › POUs › Universal

##### `SmokeDetector` (FUNCTION_BLOCK) — §6.1.4.3.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SmokeAlarm` | `FB_BA_BI_Raw` | — |
| `SystemFailure` | `FB_BA_BI_Raw` | — |
| `Contamination` | `FB_BA_BI_Raw` | — |
| `AirFlow` | `FB_BA_BI_Raw` | — |
| `RstHw` | `FB_BA_BO_Raw` | — |
| `RstRm` | `FB_BA_BV_Op` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSmokeAlarm` | `BOOL` | Current value of the binary input object SmokeAlarm. |
| `bSystemFailure` | `BOOL` | Current value of the binary input object SystemFailure. |
| `bContamination` | `BOOL` | Current value of the binary input object Contamination. |
| `bAirFlow` | `BOOL` | Current value of the binary input object AirFlow. Inputs CONSTANT |
| `SmokeAlarm` | `FB_BA_BI_Raw [} 209]` | Binary input object is used to process a smoke alarm. |
| `SystemFailure` | `FB_BA_BI_Raw [} 209]` | Binary input object is used to process a system fault of the |

**Description:** The template is used to display faults and warnings of a duct smoke detector.

#### Programming › PLC › Templates › POUs › Universal › Valve

##### `FB_BA_Vlv` (FUNCTION_BLOCK) — §6.1.4.3.2.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bAblkLock` | `BOOL` | A TRUE at this input variable interrupts the anti-blocking |

**Description:** Programming

##### `FB_BA_Vlv3pt` (FUNCTION_BLOCK) — §6.1.4.3.2.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fValPgm` | `REAL` | Continuous input signal for analog conversion to a three- |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOpen` | `BOOL` | Variable for the control Open of the 3-point valve. This |

**Description:** The template is used to control a three-point valve. It mainly consists of the base class FB_BA_ActuatorMO [} 984], the function block Anlg3Pnt [} 406] for the analog conversion of the input signal fValPgm into a three-point signal and the anti-blocking protection function Ablk [} 990].

#### Programming › PLC › Templates › POUs › Universal › WeatherStation › Thies

##### `Integration Thies weather station` — §6.1.4.3.2.9.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bRst` | `BOOL` | A rising edge at this input triggers a software reset within |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TXBuffer` | `ComBuffer` | — |
| `RXBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stWSC11Data` | `ST_BA_WSC11Data [} 715] Output of the data.` | — |
| `bErr` | `BOOL` | Error. |
| `sErrDescr` | `T_MaxString` | Textual error description. Inputs/outputs |
| `TXBuffer / RXBuffer ComBuffer` | `Serial data exchange with the reading function block` | — |

**Description:** This manual shows the integration of a Thies weather station Compact WSC11, 4.9056.10.000, using the "FB_BA_Weatherstation_Thies" template.

##### `FB_BA_ThiesData` (FUNCTION_BLOCK) — §6.1.4.3.2.9.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stWSC11Data` | `ST_BA_WSC11Data [} 715] Data from the Thies weather station.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stWeatherStation` | `ST_BA_WeatherStation [} 714]` | Converted data |

**Description:** In this function block, data from the Thies weather station are converted so that they can be represented with BACnet objects.

##### `FB_BA_Weatherstation_Thies` (FUNCTION_BLOCK) — §6.1.4.3.2.9.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SerialCommRst` | `FB_BA_Bv_OP` | — |
| `WthStRst` | `FB_BA_Bv_OP` | — |
| `SerialCommErr` | `FB_BA_BV` | — |
| `Dstb` | `FB_BA_BV` | — |
| `Rain` | `FB_BA_BI` | — |
| `WthT` | `FB_BA_AI` | — |
| `DewPtT` | `FB_BA_AI` | — |
| `PrssAbs` | `FB_BA_AI` | — |
| `PrssRel` | `FB_BA_AI` | — |
| `HumAbs` | `FB_BA_AI` | — |
| `HumRel` | `FB_BA_AI` | — |
| `Brightness_N` | `FB_BA_AI` | — |
| `Brightness_S` | `FB_BA_AI` | — |
| `Brightness_E` | `FB_BA_AI` | — |
| `Brightness_W` | `FB_BA_AI` | — |
| `Dawn` | `FB_BA_AI` | — |
| `GlobRadn` | `FB_BA_AI` | — |
| `WndDir` | `FB_BA_AI` | — |
| `WndSpd` | `FB_BA_AI` | — |
| `Latd` | `FB_BA_AI` | — |
| `Lngt` | `FB_BA_AI` | — |
| `SunAzm` | `FB_BA_AI` | — |
| `SunElv` | `FB_BA_AI` | — |
| `nSerialCommErrDelay` | `UDINT := 10` | — |
| `nDstbDelay` | `UDINT := 10` | — |

**Description:** This template prepares the data which are read from a Thies weather station via the FB_BA_ThiesWSC11 [} 1099] function block and makes them available in a globally declared variable structure stWeatherstation (see global variable list Site [} 718]).

#### Programming › PLC › Templates › POUs › Universal › WeatherStation

##### `FB_BA_Weatherstation_Draft` (FUNCTION_BLOCK) — §6.1.4.3.2.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Dstb` | `FB_BA_BI` | — |
| `Rain` | `FB_BA_BI` | — |
| `WthT` | `FB_BA_AI` | — |
| `DewPtT` | `FB_BA_AI` | — |
| `PrssAbs` | `FB_BA_AI` | — |
| `PrssRel` | `FB_BA_AI` | — |
| `HumAbs` | `FB_BA_AI` | — |
| `HumRel` | `FB_BA_AI` | — |
| `Brightness` | `FB_BA_AI` | — |
| `Dawn` | `FB_BA_AI` | — |
| `GlobRadn` | `FB_BA_AI` | — |
| `WndDir` | `FB_BA_AI` | — |
| `WndSpd` | `FB_BA_AI` | — |
| `Latd` | `FB_BA_AI` | — |
| `Lngt` | `FB_BA_AI` | — |
| `SunAzm` | `FB_BA_AI` | — |
| `SunElv` | `FB_BA_AI` | — |

**Description:** This template represents a programming template for a weather station for which no prepared template is available. To the objects Dstb ... SunElv the values of the existing weather station are to be linked, if available, as well as the time. There is no object for the time of the type TIMESTRUCT, here the placeholder variable must be replaced. This is only for error-free translation of the base project.

#### Programming › HMI › Functions

##### `ConvertRgbaToHexColor` — §6.2.6.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tchmi:general#/definitions/String` | `` | — |

**Description:** Converts an RGBA color to a hex color.

##### `ConvertRgbaToSolidColor` — §6.2.6.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tchmi:framework#/definitions/SolidColor` | `` | — |

**Description:** Converts an RGBA color to a TcHmi solid color.

##### `ConvertUnitToString` — §6.2.6.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tchmi:general#/definitions/String` | `` | — |

**Description:** Converts the value of a BA.Unit enumeration into a unit.

##### `GetBaIconPath` — §6.2.6.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tchmi:general#/definitions/String` | `` | — |

**Description:** Returns the full path of an icon for a partial path.

##### `GetCurrentUserName` — §6.2.6.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tchmi:general#/definitions/String` | `` | — |

**Description:** Returns the name of the active user (no authentication corresponds to __SystemGuest) or null if unknown (e.g. when loading).

##### `GetFadeColor` — §6.2.6.11

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tchmi:general#/definitions/SolidColor` | `` | — |

**Description:** Calculates a color between two colors to a corresponding value.

---
