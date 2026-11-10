# PfwLib_Processing

- **Manual:** `TF8540_Plastic_Processing_Framework_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Industry · **Tier:** T0 · **API entries:** 26
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Plastic-processing application library: process-control logic over image IO; machine specifics via fieldbus.

---

#### PLC temperature controller › Function blocks

##### `FB_TempCtrlMainBody_TcPfw` — §4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigEnable` | `BOOL` | If TRUE, the configuration is valid. |
| `tCycle` | `LREAL` | Cycle time |
| `Looptest_Enable` | `BOOL` | A current measurement is performed. |
| `Callback_Enable` | `BOOL` | Checks once (by calling FB_TempCtrlCallback_TcPfw()) the type of the |
| `iParamLoadCheck` | `I_ParamLoadCheck` | Optional interface for (external) |

**Description:** This function block must be called in the application. It organizes internally the complete temperature control.

##### `FB_TempCtrlEnableZone_TcPfw` — §4.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ModuleId` | `INT` | ModuleId of the temperature zones to be controlled. |
| `ZoneId` | `INT` | ZoneId of the temperature zones to be controlled. |
| `Enable` | `BOOL` | New state of the temperature zones. |

**Description:** One or more function blocks of this type are called from a function block of the application to switch individual zones, zone groups or all zones of the temperature control to the active or passive state.

##### `FB_TempCtrlStandByZone_TcPfw` — §4.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ModuleId` | `INT` | ModuleId of the temperature zones to be controlled. |
| `ZoneId` | `INT` | ZoneId of the temperature zones to be controlled. |
| `StandBy` | `BOOL` | New state of the temperature zones. |

**Description:** One or more function blocks of this type are called from a function block of the application to switch individual zones, zone groups or all zones of the temperature control into the standby or active state.

##### `FB_TempCtrlState_TcPfw` — §4.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ModuleId` | `INT` | ModuleId of the temperature zones to be controlled. |
| `ZoneId` | `INT` | ZoneId of the temperature zones to be controlled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Disabled` | `BOOL` | TRUE if the zone is not enabled. |
| `Enabled` | `BOOL` | TRUE if the zone is enabled. |
| `OnStandBy` | `BOOL` | TRUE if the zone is enabled and switched to lowering. |
| `Error` | `BOOL` | TRUE if the zone is in an error state. |
| `Tuning` | `BOOL` | TRUE if the automatic tuning function of the zone is active. |
| `Alarm_LL` | `BOOL` | TRUE if the actual temperature of the zone falls below the outer |

**Description:** This function block determines a set of state information of a zone.

##### `FB_TempCtrlCallback_TcPfw` — §4.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tCycle` | `LREAL` | Cycle time PLC temperature controller |

**Description:** This function block checks the type of the linked terminal once and parameterizes it according to the specified sensor type.

##### `FB_FcMainBody_TcPfw` — §4.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Activate` | `BOOL` | Only with a TRUE the forced cooling is really output. Inputs/outputs |
| `TempToHmi` | `ST_TcPfw_TempToHmi_Itf` | A reference to the data on the HMI of the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TempToHmi` | `ST_TcPfw_TempToHmi_Itf` | — |
| `TempCtrl` | `ST_TcPfw_TempCtrl_Itf` | — |
| `TempOut` | `ST_TcPfw_TempCtrlOutput` | — |
| `Mparam` | `ST_TcPfw_TempMparamFromHmi_Itf` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Indicates when something is misconfigured. |
| `ErrorID` | `BOOL` | Not used. |

**Description:** This function block must be called by the application after the TempCtrl_FB_TempCtrlMainBody_TcPfw. The function block organizes the intermittent switching on of the cooling independently of the controller output.

#### PLC temperature controller › Utilities › Stand Alone

##### `FB_PowerMeasurement_TcPfw` — §4.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPowerInput` | `POINTER TO BYTE` | Pointer to the input structure of the power |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stPowerState` | `ST_TcPfw_xL3403_State` | Here the current power, current and error states are |

**Description:** This module manages the current monitoring function. It must be called cyclically in the application to ensure functionality.

##### `FB_FcMainBody_TcPfw` — §4.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Activate` | `BOOL` | Only with a TRUE the forced cooling is really output. Inputs/outputs |
| `TempToHmi` | `ST_TcPfw_TempToHmi_Itf` | A reference to the data on the HMI of the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TempToHmi` | `ST_TcPfw_TempToHmi_Itf` | — |
| `TempCtrl` | `ST_TcPfw_TempCtrl_Itf` | — |
| `TempOut` | `ST_TcPfw_TempCtrlOutput` | — |
| `Mparam` | `ST_TcPfw_TempMparamFromHmi_Itf` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Indicates when something is misconfigured. |
| `ErrorID` | `BOOL` | Not used. |

**Description:** This function block must be called by the application after the TempCtrl_FB_TempCtrlMainBody_TcPfw. The function block organizes the intermittent switching on of the cooling independently of the controller output.

##### `FB_TempParamSave_TcPfw` — §4.4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the process. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Mparam` | `ST_TcPfw_TempMparamFromHmi_Itf` | — |
| `Pparam` | `ST_TcPfw_TempPparamFromHmi_Itf` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | A TRUE indicates here the successful processing of the command. |
| `Error` | `BOOL` | A TRUE indicates here the occurrence of a problem during the processing of the |

**Description:** This function block writes the parameters of a zone into a file. A FB_TempParamLoad_TcPfw function block must be used to read the file.

##### `FB_TempParamSaveP_TcPfw` — §4.4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The process is initiated by a rising edge at this input. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Mparam` | `ST_TcPfw_TempMparamFromHmi_Itf` | — |
| `Pparam` | `ST_TcPfw_TempPparamFromHmi_Itf` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | A TRUE indicates here the successful processing of the command. |
| `Error` | `BOOL` | A TRUE indicates here the occurrence of a problem during the processing of the |

**Description:** This function block writes the product parameters of a zone into a file. A FB_TempParamLoad_TcPfw function block must be used to read the file.

##### `FB_TempParamLoad_TcPfw` — §4.4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The process is initiated by a rising edge at this input. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Mparam` | `ST_TcPfw_TempMparamFromHmi_Itf` | — |
| `Pparam` | `ST_TcPfw_TempPparamFromHmi_Itf` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | A TRUE indicates here the successful processing of the command. |
| `Error` | `BOOL` | A TRUE indicates here the occurrence of a problem during the processing of the |

**Description:** This function block reads the parameters of a zone from a file. For writing the file a FB_TempParamSave_TcPfw function block must be used.

#### PLC temperature controller › Utilities

##### `FB_xL3403_TcPfw` — §4.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `I_ratio` | `LREAL` | The ratio of the current transformers must be entered here. |
| `CycleTime` | `LREAL` | Cycle time with which this function block is called. |
| `ReadVoltage` | `BOOL` | A TRUE causes the currently measured voltage to be read out instead of the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ST_TcPfw_xL3403_Input` | — |
| `Output` | `ST_TcPfw_xL3403_Output` | — |
| `State` | `ST_TcPfw_xL3403_State` | — |

**Description:** This function block processes the data (voltage, line) determined by an xL3403 and makes it available to the application.

##### `FB_TempCtrlAdaptFm33xx_TcPfw` — §4.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `FM_Input` | `POINTER TO` | The address of a single variable or an array of type |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Any problems with call parameters or zone parameters are signaled here. |

**Description:** The I/O data of FM3312 or FM3332 fieldbus modules are adapted to the I/O structures of the library.

##### `FB_TermCoeRead_TcPfw` — §4.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge starts the process. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TempIn` | `ST_TcPfw_TempCtrlInput` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | The ongoing activity of the function block is signaled here. |
| `Done` | `BOOL` | The successful completion of the operation is reported here. |
| `CommandAborted` | `BOOL` | A TRUE indicates here that the process was aborted. |
| `Error` | `BOOL` | A TRUE indicates here the occurrence of a problem. |
| `ErrorID` | `UDINT` | In the event of an error, coded information is provided here. |

**Description:** A function block of this type is used by FB_TempCtrlCallback_TcPfw for read access to EL terminals.

##### `FB_TermCoeWrite_TcPfw` — §4.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge starts the process. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TempIn` | `ST_TcPfw_TempCtrlInput` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | The ongoing activity of the function block is signaled here. |
| `Done` | `BOOL` | The successful completion of the operation is reported here. |
| `CommandAborted` | `BOOL` | A TRUE indicates here that the process was aborted. |
| `Error` | `BOOL` | A TRUE indicates here the occurrence of a problem. |
| `ErrorID` | `UDINT` | In the event of an error, coded information is provided here. |

**Description:** A function block of this type is used by FB_TempCtrlCallback_TcPfw for write access to EL terminals.

##### `FB_TermRegRead_TcPfw` — §4.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge starts the process. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Ctrl` | `USINT` | — |
| `State` | `USINT` | — |
| `InData` | `INT` | — |
| `OutData` | `INT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RegData` | `WORD` | If successfully executed, the read register content is provided here. |
| `Busy` | `BOOL` | The ongoing activity of the function block is signaled here. |
| `Done` | `BOOL` | The successful completion of the operation is reported here. |
| `CommandAborted` | `BOOL` | A TRUE indicates here that the process was aborted. |
| `Error` | `BOOL` | A TRUE indicates here the occurrence of a problem. |
| `ErrorID` | `UDINT` | In the event of an error, coded information is provided here. |

**Description:** A function block of this type is used by FB_TempCtrlCallback_TcPfw for read access to EL terminals.

##### `FB_TermRegWrite_TcPfw` — §4.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge starts the process. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Ctrl` | `USINT` | — |
| `State` | `USINT` | — |
| `InData` | `INT` | — |
| `OutData` | `INT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | The ongoing activity of the function block is signaled here. |
| `Done` | `BOOL` | The successful completion of the operation is reported here. |
| `CommandAborted` | `BOOL` | A TRUE indicates here that the process was aborted. |
| `Error` | `BOOL` | A TRUE indicates here the occurrence of a problem. |
| `ErrorID` | `UDINT` | In the event of an error, coded information is provided here. |

**Description:** A function block of this type is used by FB_TempCtrlCallback_TcPfw for write access to KL terminals.

#### PLC timer › Function blocks

##### `FB_ClockTimer_TcPfw` — §5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `udisecond` | `UDINT` | Time of the task in microseconds |

**Description:** The actual timer is implemented in this function block. It must be called cyclically and enables or disables the corresponding output ST_TcPfw_ClockTimerItf.Q in the set time.

##### `FB_ClockTimerParamSave_TcPfw` — §5.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Idx` | `INT` | Timer number |
| `Execute` | `BOOL` | The storage process is started with a rising edge. |
| `PathName` | `STRING` | Path name where to save the timer settings. |
| `PreFix` | `STRING` | Prefix to be used before the filename. Inputs/outputs |
| `ClockItf` | `ST_TcPfw_ClockTi` | Current settings of the timer merItf |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ClockItf` | `ST_TcPfw_ClockTimerItf` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Feedback that saving was successful. |
| `Error` | `BOOL` | Error while saving timer settings. |
| `ErrorId` | `UDINT` | Error number |
| `sFileName` | `STRING` | Name of the saved file. |

**Description:** This function block saves the current settings of the timer as a binary file.

##### `FB_ClockTimerParamLoad_TcPfw` — §5.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Idx` | `INT` | Timer number |
| `Execute` | `BOOL` | The loading process is started with a rising edge. |
| `PathName` | `STRING` | Path name where the timer settings can be loaded. Inputs/outputs |
| `ClockItf` | `ST_TcPfw_ClockTime Current settings of the timer. rItf` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ClockItf` | `ST_TcPfw_ClockTimerItf` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Feedback that loading was successful. |
| `Error` | `BOOL` | Error while loading timer settings. |
| `ErrorId` | `UDINT` | Error number |

**Description:** This function block loads the saved settings of the timer.

#### PLC alarm visualization › Function blocks

##### `FB_MsgAppend_TcTvA` — §6.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Idx1` | `INT` | Zone number or drive number of the faulty zone. |
| `Idx2` | `INT` | Module number of the faulty zone. |
| `ErrorId` | `DINT` | If necessary, the error number can also be read as a plain text |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTime` | `TIMESTRUCT` | — |

**Description:** This function block appends active alarms to an array of type ST_TcTvA_Alarm_Itf. To get as much information as possible about the error in the visualization, the parameters listed below can be given to the error message.

##### `FB_MsgClearPending_TcTvA` — §6.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IdxFirst` | `INT` | The first index in the alarm buffer. |
| `IdxLast` | `INT` | The last index in the alarm buffer. |
| `pAlarmBuffer` | `POINTER TO` | The address of the alarm buffer with the type |
| `ST_TcTvA_Alarm_Itf` | `ST_TcTvA_Alarm_Itf.` | — |

**Description:** This function block deletes all inactive messages from the alarm history.

##### `FB_MsgClearSignal_TcTvA` — §6.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IdxFirst` | `INT` | The first index in the alarm buffer. |
| `IdxLast` | `INT` | The last index in the alarm buffer. |
| `pAlarmBuffer` | `POINTER TO` | The address of the alarm buffer with the type |
| `ST_TcTvA_Alarm_Itf` | `ST_TcTvA_Alarm_Itf.` | — |
| `Idx1` | `INT` | Zone number of the faulty zone. |
| `Idx2` | `INT` | Module number of the faulty zone. |
| `ErrorId` | `DINT` | The error number |
| `Prio` | `INT` | The priority of the error, where 3=Alarm 2=Warning 1=Note |

**Description:** This function block disables the boolean expression ST_TcTvA_Alarm_Itf.Signal.

##### `FB_MsgDeactivate_TcTvA` — §6.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Idx1` | `INT` | Zone number of the faulty zone. |
| `Idx2` | `INT` | Module number of the faulty zone. |
| `ErrorId` | `DINT` | The error number |
| `Prio` | `INT` | The priority of the error, where 3=Alarm 2=Warning 1=Note |

**Description:** This function block disables an inactive message in the alarm history (type: ST_TcTvA_Alarm_Itf). The message is not deleted from the alarm history.

##### `FB_MsgGarbageCollect_TcTvA` — §6.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IdxFirst` | `INT` | The first index in the alarm buffer. |
| `IdxLast` | `INT` | The last index in the alarm buffer. |
| `sync_idx` | `INT` | Reserved; with 0 the array is automatically searched for |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `signaled` | `BOOL` | Signals whether a message is present. |

**Description:** This function block checks whether the alarm history (array of type ST_TcTvA_Alarm_Itf) contains empty cells and moves subsequent messages up.

##### `FB_MsgUpdateTime_TcTvA` — §6.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTaskId` | `INT` | Task in which the alarm handling takes place. Inputs/outputs |
| `stTime` | `TIMESTRUCT` | Windows system time. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTime` | `TIMESTRUCT` | — |

**Description:** This function block determines the Windows system time and returns it to the application.
