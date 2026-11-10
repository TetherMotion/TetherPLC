# Tc3_PlasticBaseApplication

- **Manual:** `TF85xx_TC3_Plastic_Application_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Industry · **Tier:** T0 · **API entries:** 255
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Plastic base application framework: process logic over image IO.

---

#### PLC-API - Class overview › Alarms - Handling of the Tc3_EventLogger

##### `FB_AppMessage` (FUNCTION_BLOCK) — §6.1.1

**Description:** This class is part of the FB_Base class and provides an interface to the EventLogger for almost every TwinCAT 3 Plastic Base Application object.

#### PLC-API - Class overview › Alarms - Handling of the Tc3_EventLogger › FB_AppMessage

##### `AnnounceAlarmArg()` (METHOD) → `HRESULT` — §6.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry` | Event from an event configuration. |
| `sString` | `STRING(255)` | Additional information that will be attached to the event. |
| `bWithConfirm` | `BOOL` | The event requires an acknowledgement. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AnnounceAlarmArg` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Announces arguments for an upcoming alarm of the transferred type. The order of the arguments is determined by the order of the calls as follows: 1. Call: fbMessage.AnnounceAlarmArg(stEvent, 'Argument1', TRUE) 2. Call: fbMessage.AnnounceAlarmArg(stEvent, 'Argument2', TRUE) 3. Call: fbMessage.SendAlarm(stEvent, 'Argument3', TRUE) ð In the event text of stEvent, {0} is replaced with 'Argument1', {1} with 'Argument2' and {2} with 'Argument3'. The order of the calls determines which argument is used for which placeholder.

##### `AnnounceAlarmRef()` (METHOD) → `HRESULT` — §6.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry` | Event from an event configuration. |
| `stEventReference` | `TcEventEntry` | An event to be included as an argument. |
| `bWithConfirm` | `BOOL` | The event requires an acknowledgement. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AnnounceAlarmRef` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Announces arguments for an upcoming alarm of the transferred type. The order of the arguments is determined by the order of the calls as follows (see AnnounceAlarmArg() [} 51]):

##### `SendAlarm()` (METHOD) → `HRESULT` — §6.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry` | Event from an event configuration. |
| `sString` | `STRING(255)` | Additional information that will be attached to the event. |
| `bWithConfirm` | `BOOL` | The event requires an acknowledgement. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendAlarm` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Triggers an alarm.

##### `SendEqualMessage()` (METHOD) → `HRESULT` — §6.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry` | Event from an event configuration. |
| `sString` | `STRING(255)` | Additional information that will be attached to the event. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendEqualMessage` | `HRESULT` | Return value with feedback on the success of the |

**Description:** Logs a message in the EventLogger.

##### `SendHresult()` (METHOD) → `HRESULT` — §6.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry` | Event from an event configuration. |
| `hr` | `HRESULT` | Error code to be displayed. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendHresult` | `HRESULT` | Return value with feedback on the success of the |

**Description:** Logs a message in the EventLogger with the hexadecimal representation of the error code.

##### `SendMessage()` (METHOD) → `HRESULT` — §6.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry` | Event from an event configuration. |
| `sString` | `STRING(255)` | Additional information that will be attached to the event. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendMessage` | `HRESULT` | Return value with feedback on the success of the |

**Description:** Logs a message in EventLogger, unless the message has been sent before.

#### PLC-API - Class overview › Alarms - Handling of the Tc3_EventLogger

##### `FB_AlarmHandler` (FUNCTION_BLOCK) — §6.1.2

**Description:** The class implements a handling of pending alarms to influence the process sequence. This means, for example, that an alarm with the severity "Error" can be responded to by stopping the machine directly.

##### `F_SeverityToString` (FUNCTION) → `T_MaxString` — §6.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eSeverity` | `TcEventSeverity` | Severity that is to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_SeverityToString` | `T_MaxString` | Human-readable ASCII string. |

**Description:** Converts a TcEventSeverity into a human-readable ASCII-based string.

#### PLC-API - Class overview › Analog - analog value scaling

##### `FB_ScaleAnalogHmi` (FUNCTION_BLOCK) — §6.2.1

**Description:** Implements functions for scaling analog values with parameterization via the HMI.

##### `FB_Monitoring` (FUNCTION_BLOCK) — §6.2.2

**Description:** Implements a monitoring function for analog signals. The threshold values for monitoring can be set variably via the HMI object [} 62].

#### PLC-API - Class overview › Analog - analog value scaling › FB_Monitoring

##### `FB_MonitoringTemp` (FUNCTION_BLOCK) — §6.2.2.1

**Description:** Implements the monitoring of a temperature channel with direct connection to TF8540. The adjustable limits are synchronized with the temperature zone.

##### `FB_MonitoringIPC` (FUNCTION_BLOCK) — §6.2.2.2

**Description:** Implements the monitoring of the CPU temperature of a Beckhoff IPC.

#### PLC-API - Class overview › Analog - analog value scaling

##### `FB_MonitoringHmi` (FUNCTION_BLOCK) — §6.2.3

**Description:** HMI parallel class to the FB_Monitoring.

##### `FB_Setpoints` (FUNCTION_BLOCK) — §6.2.4

**Description:** Implements scaling of analog setpoints that can be adjusted via the HMI.

##### `FB_SetpointsHmi` (FUNCTION_BLOCK) — §6.2.5

**Description:** HMI parallel class to the FB_Setpoints.

#### PLC-API - Class overview › Base - Base classes

##### `FB_Base` (FUNCTION_BLOCK) — §6.3.1

**Description:** FB_Base is the base class for most of the available classes of the Plastic Base Application.

##### `FB_BaseHmi` (FUNCTION_BLOCK) — §6.3.2

**Description:** FB_BaseHmi is the base class of all HMI parallel classes. The provided Init(ipBase) method predefines that the parallel object is passed via the I_Base interface.

##### `FB_BaseMd` (FUNCTION_BLOCK) — §6.3.3

**Description:** Extends the base class FB_Base by the functions of the machine data handling. The structure of the machine data is explained in more detail in the chapter Machine data [} 145].

##### `FB_BaseMdHmi` (FUNCTION_BLOCK) — §6.3.4

**Description:** HMI class in parallel with the base machine data class FB_BaseMd. Is necessary for loading and saving the machine data via the HMI.

##### `F_IsSameInstance` (FUNCTION) → `BOOL` — §6.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iCmp1` | `I_Comparable` | First object to be compared. |
| `iCmp2` | `I_Comparable` | Second object to be compared. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_IsSameInstance` | `BOOL` | Result of the comparison: TRUE -> Identical, FALSE -> Different |

**Description:** Compares two objects to see if they are exactly the same instance. If the transferred objects are each an instance of the same class but not the identical object, FALSE is returned.

#### PLC-API - Class overview › BlowMolding - Blow molding specific classes

##### `FB_Blowing` (FUNCTION_BLOCK) — §6.4.1

**Description:** Implements a typical blowing sequence in two blowing phases with adjustable pressure.

##### `FB_IntervalBlowing` (FUNCTION_BLOCK) — §6.4.2

**Description:** Extends the FB_Blowing class with an alternating blowing interval during the main blowing phase.

##### `FB_Blowpin` (FUNCTION_BLOCK) — §6.4.3

**Description:** Implements blowpin-specific functions.

##### `FB_BlowpinHmi` (FUNCTION_BLOCK) — §6.4.4

**Description:** HMI parallel class of FB_Blowpin class.

##### `FB_BlowMoldingExtruder` (FUNCTION_BLOCK) — §6.4.5

**Description:** Implements specific functions for extruders in the blow molding process.

#### PLC-API - Class overview › BlowMolding - Blow molding specific classes › FB_BlowMoldingExtruder

##### `FB_ParisonLengthControl` (FUNCTION_BLOCK) — §6.4.5.1

**Description:** Implements parison length control in the typical use case of the blow molding process.

#### PLC-API - Class overview › BlowMolding - Blow molding specific classes

##### `FB_BlowMoldingExtruderHmi` (FUNCTION_BLOCK) — §6.4.6

**Description:** HMI parallel class of FB_BlowMoldingExtruder class.

##### `FB_Wtc` (FUNCTION_BLOCK) — §6.4.7

**Description:** Implements wall thickness control for the extruded hose in blow molding applications.

##### `FB_WtcHmi` (FUNCTION_BLOCK) — §6.4.8

**Description:** HMI parallel class of FB_Wtc class.

##### `FB_WtcTimer` (FUNCTION_BLOCK) — §6.4.9

**Description:** Implements a timer as master for WTC camming.

##### `FB_WtcTimerHmi` (FUNCTION_BLOCK) — §6.4.10

**Description:** HMI parallel class of the FB_WtcTimer.

##### `FB_WtcAccumulator` (FUNCTION_BLOCK) — §6.4.11

**Description:** Implements an accumulator as the master for a WTC.

##### `FB_WtcAccumulatorHmi` (FUNCTION_BLOCK) — §6.4.12

**Description:** HMI parallel class of FB_WtcAccumulator class.

##### `FB_SupportAir` (FUNCTION_BLOCK) — §6.4.13

**Description:** Implements a support air function using a cam and a WTC master.

##### `FB_SupportAirHmi` (FUNCTION_BLOCK) — §6.4.14

**Description:** HMI parallel class to the FB_SupportAir.

#### PLC-API - Class overview › InjectionMolding - Injection molding-specific classes

##### `FB_Injector` (FUNCTION_BLOCK) — §6.5.1

**Description:** Implements specific functions of an injection axis (injector). Together with an FB_MeteringScrew [} 125], the combination of both axes results in an injection unit.

##### `FB_InjectorHmi` (FUNCTION_BLOCK) — §6.5.2

**Description:** HMI parallel class of FB_InjectorHmi class.

#### PLC-API - Class overview › InjectionMolding - Injection molding-specific classes › FB_InjectorHmi

##### `FB_SwitchOverHmi` (FUNCTION_BLOCK) — §6.5.2.1

**Description:** HMI class for setting the holding pressure switchover of an FB_Injector [} 114].

#### PLC-API - Class overview › InjectionMolding - Injection molding-specific classes

##### `FB_MeteringScrew` (FUNCTION_BLOCK) — §6.5.3

**Description:** Implements the metering screw of an injection unit. A combination with an FB_Injector [} 114] completes the components of an injection unit.

##### `FB_MeteringScrewHmi` (FUNCTION_BLOCK) — §6.5.4

**Description:** HMI parallel class of FB_MeteringScrew class.

#### PLC-API - Class overview › InjectionMolding - Injection molding-specific classes › FB_MeteringScrewHmi

##### `FB_MovabililtyCheckHmi` (FUNCTION_BLOCK) — §6.5.4.1

**Description:** HMI class for setting the movability check of an FB_MeteringScrew [} 125]. The movability check is used to ensure that the viscosity of the heated plastic is already low enough to allow the screw to rotate slowly with low torque.

#### PLC-API - Class overview › Logging - Recording user input › CSV

##### `FB_CsvBase` (FUNCTION_BLOCK) — §6.6.1.1

**Description:** The base class for CSV-based classes.

##### `FB_CsvLogger` (FUNCTION_BLOCK) — §6.6.1.2

**Description:** Implements the logging of values in a CSV file.

##### `FB_CsvReader` (FUNCTION_BLOCK) — §6.6.1.3

**Description:** Implements the reading of values from a CSV file.

#### PLC-API - Class overview › Logging - Recording user input › UserLogging

##### `FB_UserLogging` (FUNCTION_BLOCK) — §6.6.2.1

**Description:** Implements the logic of the logging database of TF8550 UserLogging.

##### `FB_UserLoggingHmi` (FUNCTION_BLOCK) — §6.6.2.2

**Description:** HMI parallel class to the FB_UserLogging.

#### PLC-API - Class overview › MachineData - Machine data › Base

##### `FB_MdBaseContainer` (FUNCTION_BLOCK) — §6.7.1.1

**Description:** The class extends the TF8560 FB_MdContainer by the machine data encoding of the TwinCAT 3 Plastic Application. In addition, errors that occur can be evaluated using Boolean values and file storage is simplified.

##### `FB_MdBaseComponent` (FUNCTION_BLOCK) — §6.7.1.2

**Description:** The class extends the standard component FB_MdComponent of the TF8560 utilities with index coding functions from the Plastic Base Application.

##### `FB_MdCollection` (FUNCTION_BLOCK) — §6.7.1.3

**Description:** The class can store a collection of components of a class type. When attaching the class to FB_BaseRuntime, components of objects implementing the interface I_AttachableMdInterface can be collected and attached to the collection.

#### PLC-API - Class overview › MachineData - Machine data › Components

##### `FB_MdActuator` (FUNCTION_BLOCK) — §6.7.2.1

**Description:** This machine data component contains parameters that are relevant for motion objects with linear Boolean actuators.

##### `FB_MdAnalogValue` (FUNCTION_BLOCK) — §6.7.2.2

**Description:** This machine data component contains parameters that are relevant for analog scaled I/Os.

##### `FB_MdAxis` (FUNCTION_BLOCK) — §6.7.2.3

**Description:** This machine data component contains the additional parameters of a base axis.

##### `FB_MdBlowpin` (FUNCTION_BLOCK) — §6.7.2.4

**Description:** This machine data component contains parameters that are specifically relevant for a blow pin.

##### `FB_MdCarriage` (FUNCTION_BLOCK) — §6.7.2.5

**Description:** This machine data component contains parameters that are specifically relevant for a trolley unit.

##### `FB_MdClamp` (FUNCTION_BLOCK) — §6.7.2.6

**Description:** This machine data component contains parameters that are specifically relevant for a clamp.

##### `FB_MdContinuousMotion` (FUNCTION_BLOCK) — §6.7.2.7

**Description:** This machine data component contains parameters that are relevant for continuous motion such as an extruder.

##### `FB_MdFileInfo` (FUNCTION_BLOCK) — §6.7.2.8

**Description:** This machine data component contains information about the previous saving processes of a machine data file.

##### `FB_MdHoming` (FUNCTION_BLOCK) — §6.7.2.9

**Description:** This machine data component contains parameters that are relevant for homing an axis.

##### `FB_MdNc` (FUNCTION_BLOCK) — §6.7.2.10

**Description:** This machine data component contains parameters that are relevant for NC-based axes.

##### `FB_MdPCtrlParams` (FUNCTION_BLOCK) — §6.7.2.11

**Description:** This machine data component saves the parameter set of a TF8560 pressure controller.

##### `FB_MdPtpMotion` (FUNCTION_BLOCK) — §6.7.2.12

**Description:** This machine data component contains parameters that are relevant for linear motion.

##### `FB_MdTempChannel` (FUNCTION_BLOCK) — §6.7.2.13

**Description:** This machine data component contains parameters that are relevant for a temperature channel.

##### `FB_MdTempGroup` (FUNCTION_BLOCK) — §6.7.2.14

**Description:** This machine data component contains parameters that are relevant for a temperature group configuration.

##### `FB_MdTempGrouping` (FUNCTION_BLOCK) — §6.7.2.15

**Description:** This machine data component contains parameters that are relevant for managing the temperature channels in a group. This creates compatibility with FB_MdTempChannel [} 172] so that parameters of a temperature channel can be loaded.

##### `FB_MdTempSupply` (FUNCTION_BLOCK) — §6.7.2.16

**Description:** This machine data component contains parameters that are relevant for a supply group.

##### `FB_MdTempTimer` (FUNCTION_BLOCK) — §6.7.2.17

**Description:** This machine data component contains parameters that are relevant for the temperature control timer.

##### `FB_MdTempZone` (FUNCTION_BLOCK) — §6.7.2.18

**Description:** This machine data component contains parameters that are relevant for a TF8540 temperature zone. The global structure aaaPfwTempMparamFromHmi[x] is mapped.

##### `FB_MdTrafo` (FUNCTION_BLOCK) — §6.7.2.19

**Description:** This machine data component contains parameters that are relevant for transforming axes.

##### `FB_MdWeekdayTiming` (FUNCTION_BLOCK) — §6.7.2.20

**Description:** This machine data component contains parameters that are relevant for a weekly timer.

##### `FB_MdWtc` (FUNCTION_BLOCK) — §6.7.2.21

**Description:** This machine data component contains parameters that are relevant for a WTC axis.

##### `FB_MdWtcTimer` (FUNCTION_BLOCK) — §6.7.2.22

**Description:** This machine data component contains parameters that are relevant for a WTC Timer Master.

#### PLC-API - Class overview › ManualFunction - Manual functions

##### `FB_ManualFunctionHmi` (FUNCTION_BLOCK) — §6.8.1

**Description:** The class is used to connect to a TF8550 ManualOperation control.

#### PLC-API - Class overview › ManualFunction - Manual functions › FB_ManualFunctionHmi

##### `FB_ManualFunctionCmdHmi` (FUNCTION_BLOCK) — §6.8.1.1

**Description:** Summarizes the commands of the HMI in FB_ManualFunctionHmi.

##### `FB_ManualFunctionStateHmi` (FUNCTION_BLOCK) — §6.8.1.2

**Description:** Summarizes the feedback signals of the HMI in FB_ManualFunctionHmi.

#### PLC-API - Class overview › ManualFunction - Manual functions

##### `FB_Manual` (FUNCTION_BLOCK) — §6.8.2

**Description:** Implements an abstract basis for standardized manual functions.

##### `FB_ManualMotion` (FUNCTION_BLOCK) — §6.8.3

**Description:** Implements an abstract basis for standardized manual motion functions.

##### `FB_ManualBaseWork` (FUNCTION_BLOCK) — §6.8.4

**Description:** Implements a ready-made version of a manual function for controlling the end positions of a motion object.

#### PLC-API - Class overview › ManualFunction - Manual functions › FB_ManualBaseWork

##### `SetWorkBasePos()` (METHOD) — §6.8.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fWorkPos` | `LREAL` | Target position of the working position. |
| `fBasePos` | `LREAL` | Target position of the base position. |
| `fTolerance` | `LREAL` | > 0 to indicate an active movement per actual position. |

**Description:** Configures the end positions for indication in the TF8550 ManualOperation control.

#### PLC-API - Class overview › ManualFunction - Manual functions

##### `FB_ManualPower` (FUNCTION_BLOCK) — §6.8.5

**Description:** Implements a predefined version of a manual function for switching on the axis control.

##### `FB_ManualTurnrate` (FUNCTION_BLOCK) — §6.8.6

**Description:** Implements a standard limit and display for tactile manual functions to adjust the turn rate of an axis.

#### PLC-API - Class overview › ManualFunction - Manual functions › FB_ManualTurnrate

##### `SetTurnrateLimits()` (METHOD) → `HRESULT` — §6.8.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fMinTurnrate` | `LREAL` | Turn rate at which the base position button lights up with |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetTurnrateLimits` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Configures the behavior for indication in the TF8550 ManualOperation control.

#### PLC-API - Class overview › ManualFunction - Manual functions

##### `FB_ManualWheel` (FUNCTION_BLOCK) — §6.8.7

**Description:** Implements the handling of a central operating unit as incremental encoder of a Beckhoff BlowMolding Panel, implemented as FB_CP22xx-1025 [} 374].

#### PLC-API - Class overview › Motion - Motion objects

##### `FB_Motion` (FUNCTION_BLOCK) — §6.9.1

**Description:** Represents the standard class for all motion objects, based on TF8560 motion objects.

#### PLC-API - Class overview › Motion - Motion objects › FB_Motion

##### `CheckAxisState()` (METHOD) → `BOOL` — §6.9.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eState` | `E_AxisState` | Axis condition to be checked |
| `bSendEvent` | `BOOL` | Logging an event when the checked state does not match. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CheckAxisState` | `BOOL` | Is TRUE if the axis state matches. Possible events |
| `ID` | `Description` | Alarm/Message |

**Description:** Checks the current state of the TF8560 axis.

##### `FB_MotionSpecific` (FUNCTION_BLOCK) — §6.9.1.2

**Description:** PLC-API - Class overview | Motion - Motion objects

#### PLC-API - Class overview › Motion - Motion objects › FB_Motion › FB_MotionSpecific

##### `FB_ActuatorFunctions` (FUNCTION_BLOCK) — §6.9.1.2.1

**Description:** Implements specific functions for a TF8560 actuator.

##### `FB_HydraulicFunctions` (FUNCTION_BLOCK) — §6.9.1.2.2

**Description:** Implements hydraulic-specific functions of a TF8560 axis.

##### `FB_InverterFunctions` (FUNCTION_BLOCK) — §6.9.1.2.3

**Description:** Implements inverter-specific functions of a turnrate-based TF8560 axis.

##### `FB_NcFunctions` (FUNCTION_BLOCK) — §6.9.1.2.4

**Description:** Implements specific functions for an NC-based TF8560 axis.

##### `FB_TrafoFunctions` (FUNCTION_BLOCK) — §6.9.1.2.5

**Description:** Implements specific functions for a transforming TF8560 axis.

#### PLC-API - Class overview › Motion - Motion objects

##### `FB_MotionHmi` (FUNCTION_BLOCK) — §6.9.2

**Description:** HMI class parallel to the standard class FB_Motion for motion objects

##### `FB_Axis` (FUNCTION_BLOCK) — §6.9.3

**Description:** Represents the standard class for all axis objects based on TF8560 axes.

#### PLC-API - Class overview › Motion - Motion objects › FB_Axis

##### `FB_Referencing` (FUNCTION_BLOCK) — §6.9.3.1

**Description:** The Homing class implements standard procedures for referencing an axis.

#### PLC-API - Class overview › Motion - Motion objects

##### `FB_AxisHmi` (FUNCTION_BLOCK) — §6.9.4

**Description:** HMI class parallel to the standard class FB_Axis for axes

#### PLC-API - Class overview › Motion - Motion objects › FB_AxisHmi

##### `FB_ReferencingHmi` (FUNCTION_BLOCK) — §6.9.4.1

**Description:** HMI class parallel to the referencing class of an axis.

#### PLC-API - Class overview › Motion - Motion objects › FB_AxisHmi › FB_ReferencingHmi

##### `FB_HomingBaseHmi` (FUNCTION_BLOCK) — §6.9.4.1.1

**Description:** This class summarizes all parameters that are required in all homing type referencing methods.

##### `FB_HomingBlockBaseHmi` (FUNCTION_BLOCK) — §6.9.4.1.2

**Description:** This class extends the basic parameters of the homing type referencing methods specifically for applications with an end stop.

##### `FB_HomingBlockDetectHmi` (FUNCTION_BLOCK) — §6.9.4.1.3

**Description:** This class provides the parameters for measuring an end stop.

##### `FB_HomingBlockHmi` (FUNCTION_BLOCK) — §6.9.4.1.4

**Description:** This class provides the parameters for homing an end stop.

##### `FB_HomingFinishHmi` (FUNCTION_BLOCK) — §6.9.4.1.5

**Description:** This class extends the basic parameters of the homing type referencing methods specifically for applications with an end stop.

##### `FB_HomingSwitchBaseHmi` (FUNCTION_BLOCK) — §6.9.4.1.6

**Description:** This class extends the basic parameters of the homing type referencing methods specifically for applications with a homing switch/sensor.

##### `FB_HomingSwitchDetectHmi` (FUNCTION_BLOCK) — §6.9.4.1.7

**Description:** This class provides the parameters for measuring an end position sensor/switch.

##### `FB_HomingSwitchHmi` (FUNCTION_BLOCK) — §6.9.4.1.8

**Description:** This class provides the parameters for homing using an end position sensor/switch.

#### PLC-API - Class overview › Motion - Motion objects

##### `FB_CamAxis` (FUNCTION_BLOCK) — §6.9.5

**Description:** This class (FB) defines a uniform coupling of cams to an axis via the TF8550 CurveEditor. Any instance can be assigned for operation during runtime.

##### `FB_CamAxisHmi` (FUNCTION_BLOCK) — §6.9.6

**Description:** Extends the FB_AxisHmi [} 226] class for camming based axes with the necessary interface to the TF8550 CurveEditor via FB_CurveHmi.

##### `FB_Carriage` (FUNCTION_BLOCK) — §6.9.7

**Description:** Implements additional functions for the operation of a trolley movement (e.g. using a crank drive).

##### `FB_CarriageHmi` (FUNCTION_BLOCK) — §6.9.8

**Description:** HMI parallel class of FB_Carriage class.

##### `FB_Clamp` (FUNCTION_BLOCK) — §6.9.9

**Description:** Implements specific functions for a clamp. • Adds a closing segment to the PTP movement, which is permanently parameterized in the machine data.

##### `FB_ClampHmi` (FUNCTION_BLOCK) — §6.9.10

**Description:** HMI parallel class of FB_Clamp class.

##### `FB_Extruder` (FUNCTION_BLOCK) — §6.9.11

**Description:** Inherits from the FB_Axis class and implements further functions for continuous motion of an extruder.

##### `FB_ExtruderHmi` (FUNCTION_BLOCK) — §6.9.12

**Description:** HMI parallel class of FB_Extruder class.

##### `FB_PtpMotion` (FUNCTION_BLOCK) — §6.9.13

**Description:** Implements a segment logic for PTP-based movements. The TF8550 Control Arrow Motion Graph is supported via the FB_PtpMotionHmi class.

#### PLC-API - Class overview › Motion - Motion objects › FB_PtpMotion

##### `MovePtp()` (METHOD) → `HRESULT` — §6.9.13.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bCommand` | `BOOL` | TRUE to trigger a movement, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MovePtp` | `HRESULT` | Return value with feedback on the success of the |

**Description:** Triggers the processing of the configured PTP segments.

##### `MoveToPoint()` (METHOD) → `HRESULT` — §6.9.13.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bCommand` | `BOOL` | TRUE to trigger a movement, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MoveToPoint` | `HRESULT` | Return value with feedback on the success of the |

**Description:** Moves directly to the position of a configured PTP segment without processing the segments between the start position and the target point.

#### PLC-API - Class overview › Motion - Motion objects

##### `FB_PtpMotionHmi` (FUNCTION_BLOCK) — §6.9.14

**Description:** HMI parallel class of FB_PtpMotion class.

#### PLC-API - Class overview › Motion - Motion objects › FB_PtpMotionHmi

##### `FB_AxisMoveHmi` (FUNCTION_BLOCK) — §6.9.14.1

**Description:** Combines all Move Clusters.

#### PLC-API - Class overview › Motion - Motion objects › FB_PtpMotionHmi › FB_AxisMoveHmi

##### `FB_MoveCfgHmi` (FUNCTION_BLOCK) — §6.9.14.1.1

**Description:** Summarizes the information of a Move Cluster.

#### PLC-API - Class overview › Motion - Motion objects › FB_PtpMotionHmi › FB_AxisMoveHmi › FB_MoveCfgHmi

##### `FB_SegCfgHmi` (FUNCTION_BLOCK) — §6.9.14.1.1.1

**Description:** Summarizes the information of a segment.

#### PLC-API - Class overview › Motion - Motion objects › Linkable HMI classes

##### `FB_BaseCamsHmi` (FUNCTION_BLOCK) — §6.9.15.1

**Description:** This base class (FB) is a TF8550 CamControl based cam switch list. It represents a common basis for the internal cam switch list of an FB_PtpMotionHmi [} 272] and a stand-alone FB_CamsHmi [} 281] instance.

##### `FB_CamsHmi` (FUNCTION_BLOCK) — §6.9.15.2

**Description:** This class (FB) implements a stand-alone cam switch list for a TF8550 CamControl.

#### PLC-API - Class overview › Motion - Motion objects › Linkable HMI classes › FB_CamsHmi

##### `FB_CamSwitchHmi` (FUNCTION_BLOCK) — §6.9.15.2.1

**Description:** This class (FB) provides exactly one cam switch.

#### PLC-API - Class overview › Motion - Motion objects › Linkable HMI classes

##### `FB_BaseCurveHmi` (FUNCTION_BLOCK) — §6.9.15.3

**Description:** This base class (FB) is a TF8550 CurveEditor based cams. It represents a common basis for the internal cam of an FB_CamAxisHmi [} 243] and a stand-alone FB_CurveHmi [} 286] instance.

##### `FB_CurveHmi` (FUNCTION_BLOCK) — §6.9.15.4

**Description:** This class (FB) implements a stand-alone TF8550 CurveEditor based cam.

#### PLC-API - Class overview › OperationData - Production data and statistics

##### `FB_ProductionCounter` (FUNCTION_BLOCK) — §6.10.1

**Description:** Implements a general counter for production-dependent data such as a piece counter or a production time counter.

#### PLC-API - Class overview › OperationData - Production data and statistics › FB_ProductionCounter

##### `FB_ProductionCounterComponent` (FUNCTION_BLOCK) — §6.10.1.1

**Description:** Contains the counter data for a defined period.

#### PLC-API - Class overview › OperationData - Production data and statistics

##### `FB_ProductionCounterHmi` (FUNCTION_BLOCK) — §6.10.2

**Description:** HMI class parallel to the base class FB_ProductionCounter for production counters.

##### `FB_EnergyMeasurement` (FUNCTION_BLOCK) — §6.10.3

**Description:** Implements the measurement of a power measurement terminal over time. This allows the machine's energy consumption to be analyzed.

#### PLC-API - Class overview › OperationData - Production data and statistics › FB_EnergyMeasurement

##### `FB_EnergyCalc` (FUNCTION_BLOCK) — §6.10.3.1

**Description:** Contains the energy meter data for a defined period. The current and previous time periods are always saved.

##### `FB_EmPhase` (FUNCTION_BLOCK) — §6.10.3.2.1

**Description:** Defines the minimum amount of information for a measured phase.

##### `FB_EmPhaseGuarded` (FUNCTION_BLOCK) — §6.10.3.2.2

**Description:** Defines the extended scope of information for a measured phase.

#### PLC-API - Class overview › Recipe - recipe management

##### `FB_PlcStateToHmi` (FUNCTION_BLOCK) — §6.11.1

**Description:** Implements the handshake procedure for the TF8550 recipe management on the PLC side.

##### `FB_Recipe` (FUNCTION_BLOCK) — §6.11.3

**Description:** Recipe management class for summarizing structured recipe values. These are used in integrated TcHMI controls.

##### `FB_TempRecipe` (FUNCTION_BLOCK) — §6.11.4

**Description:** Recipe management class of temperature control.

##### `FB_MoveClusterRecipe` (FUNCTION_BLOCK) — §6.11.5

**Description:** Recipe management class of a PTP axes.

#### PLC-API - Class overview › Recipe - recipe management › FB_MoveClusterRecipe

##### `FB_MoveSegRecipe` (FUNCTION_BLOCK) — §6.11.5.1

**Description:** Recipe management class of a PTP segment.

#### PLC-API - Class overview › Runtime - Handling of initialization and cycle calls

##### `FB_BaseRuntime` (FUNCTION_BLOCK) — §6.12.1

**Description:** The class automates the initialization and cyclic calls of instantiated control objects such as axes, sequence algorithms, temperature controllers, etc.

#### PLC-API - Class overview › Runtime - Handling of initialization and cycle calls › FB_BaseRuntime

##### `Append()` (METHOD) → `VOID` — §6.12.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObj` | `I_RuntimeInterface` | Object to be attached to the runtime. |
| `iObjHmi` | `I_BaseHmi` | Associated HMI object which is to be linked to the object. |

**Description:** Appends a control object to the runtime.

#### PLC-API - Class overview › ShotScope - TF8550 Injection Integration

##### `FB_ShotScopeHmi` (FUNCTION_BLOCK) — §6.13.1

**Description:** Implements the handshake procedure for the TF8550 ShotScope on the PLC side.

#### PLC-API - Class overview › ShotScope - TF8550 Injection Integration › FB_ShotScopeHmi

##### `FB_ShotScopeAnalysis` (FUNCTION_BLOCK) — §6.13.1.1

**Description:** This class consolidates the settings and information related to shot analysis for the TF8550 ShotScope.

##### `FB_ShotScopeLogDetails` (FUNCTION_BLOCK) — §6.13.1.2

**Description:** This class aggregates the information about a shot that is to be recorded by the TF8550 ShotScope HMI.

##### `FB_ShotScopeQM` (FUNCTION_BLOCK) — §6.13.1.3

**Description:** This class handles quality monitoring for the TF8550 ShotScope.

#### PLC-API - Class overview › Temperature - TF8540 Temperature Interface

##### `FB_Temperature` (FUNCTION_BLOCK) — §6.14.1

**Description:** Main class of temperature control to manage all temperature channels and groups.

#### PLC-API - Class overview › Temperature - TF8540 Temperature Interface › FB_Temperature

##### `CreateDefaultParams()` (METHOD) → `HRESULT` — §6.14.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bAllInUse` | `BOOL` | All channels are initialized as "InUse". FALSE |
| `eSensor` | `E_TcPfw_TempS Sensor type - NoSensor in simulation mode eTcPfwTempSensT_NoSensor ensType` | — |
| `eTerminal E_TcPfwTerminal Terminal type - NoTerminal in simulation` | `eTcPfwTermT_NoTerminal` | — |
| `Type` | `mode` | — |
| `nChPerTer INT` | `Number of channels per terminal` | 8 m |
| `eOutHeatin E_TcPfw_TctrlOu Output type of the heating output -` | `eTcPfwTcOut_PWM` | — |
| `g` | `tSelect` | NoSignal, to disable the heating function (e.g. for measuring zones) |
| `eOutCoolin E_TcPfw_TctrlOu Cooling output type` | `eTcPfwTcOut_NoSignal` | — |
| `g` | `tSelect` | — |
| `fSetpoint` | `LREAL` | Temperature setpoint for all channels 180.0 |
| `fStandbySet LREAL` | `Temperature setpoint for standby` | 18.0 |
| `p` | `temperature of all channels` | — |
| `fPwmCycle LREAL` | `PWM cycle time for all outputs (dutyCycle = 1.0` | — |
| `Time` | `fPwmCycleTime * 0.1)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateDefaultParams` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Creates a default parameterization for all temperature channels

##### `EnableAll()` (METHOD) — §6.14.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bCmd` | `BOOL` | TRUE to grant the release, FALSE to withdraw the release. |
| `bGroups` | `BOOL` | The enable only takes into account channels that are |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EnableAll` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Enables all temperature channels on the PLC side.

##### `LinkGroup()` (METHOD) → `HRESULT` — §6.14.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nStartIdx` | `INT` | Index of the first channel to be assigned |
| `nEndIdx` | `INT` | Index of the last channel to be assigned |
| `nGroupIdx` | `INT` | Index of the group to which the channels are to be assigned |
| `bOverwrite1` | `BOOL` | Zones are assigned even if the group contains already |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LinkGroup` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Assigns a number of temperature channels to a group.

##### `LinkSupply()` (METHOD) → `HRESULT` — §6.14.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGroupIdx` | `INT` | Index of the group to which a supply unit is to be assigned |
| `nSupplyIdx` | `INT` | Index of the supply unit to be assigned to the group |
| `nLine` | `INT` | Supply channel to which the group members are connected • 1; 2; 3 - phase L1, L2 or L3 • 4 – Between phases without connection to N |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LinkSupply` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Assigns a supply unit to a group.

##### `LinkZone()` (METHOD) → `HRESULT` — §6.14.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLinearIdx` | `INT` | Index of the channel to be assigned |
| `nGroupIdx` | `INT` | Index of the group to which the channel is to be assigned |
| `nGroupMemberIdx1` | `INT` | Index in the target group |
| `bOverwrite` | `BOOL` | Zone is assigned even if the index is already occupied in |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LinkZone` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Assigns a single temperature channel (zone) to a group.

##### `StandbyAll()` (METHOD) → `HRESULT` — §6.14.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bCmd` | `BOOL` | TRUE to enable the standby temperature, FALSE to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StandbyAll` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Sets all temperature channels to standby temperature.

#### PLC-API - Class overview › Temperature - TF8540 Temperature Interface

##### `FB_TemperatureHmi` (FUNCTION_BLOCK) — §6.14.2

**Description:** HMI parallel class to the FB_Temperature.

##### `FB_TemperatureGroup` (FUNCTION_BLOCK) — §6.14.3

**Description:** Class for group control of the temperature channels. The internal processing of the configuration can be traced using the state machine in GroupStates() [} 335].

##### `FB_TemperatureGroupHmi` (FUNCTION_BLOCK) — §6.14.4

**Description:** HMI parallel class to the FB_TemperatureGroup class.

##### `FB_TempChannelBase` (FUNCTION_BLOCK) — §6.14.5

**Description:** Base class for a single temperature channel.

##### `FB_TempChannel` (FUNCTION_BLOCK) — §6.14.6

**Description:** Class for single control of a temperature channel.

##### `FB_TempChannelHmi` (FUNCTION_BLOCK) — §6.14.7

**Description:** HMI parallel class to the FB_TempChannel class

##### `FB_TempSupply` (FUNCTION_BLOCK) — §6.14.8

**Description:** Implements a supply unit of temperature control.

##### `FB_TempSupplyLine` (FUNCTION_BLOCK) — §6.14.9

**Description:** Implements a supply unit of temperature control.

##### `FB_TempSchedule` (FUNCTION_BLOCK) — §6.14.10

**Description:** Class for the use of weekday timers in connection with temperature control.

##### `FB_TempScheduleHmi` (FUNCTION_BLOCK) — §6.14.11

**Description:** HMI parallel class to FB_TempSchedule class.

##### `FB_TimerTempHmi` (FUNCTION_BLOCK) — §6.14.12

**Description:** Extension of FB_TimerWeekdayHmi with additional temperature-relevant attributes.

#### PLC-API - Class overview › Temperature - TF8540 Temperature Interface › FB_TimerTempHmi

##### `Members()` (METHOD) → `I_TemperatureGroup` — §6.14.12.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Idx` | `INT` | Index of the member in the list all members |
| `iTemp` | `I_Temperature` | Reference of the temperature control to determine the temperature group instance |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Members` | `I_TemperatureGroup` | Requested temperature group |

**Description:** Returns an interface to the idx member (group)

#### PLC-API - Class overview › Temperature - TF8540 Temperature Interface

##### `FB_TempConfigQueryHmi` (FUNCTION_BLOCK) — §6.14.13

**Description:** This class (FB) is used to iteratively query the temperature configurations of a temperature group.

#### PLC-API - Class overview › Time - Time-related functions

##### `FB_TimerHmi` (FUNCTION_BLOCK) — §6.15.1

**Description:** HMI interface for IEC 61131-3 timer for setting process timings with a TF8550 TimerControl.

##### `FB_TimerTon` (FUNCTION_BLOCK) — §6.15.2

**Description:** Implements IEC 61131-3 timer function with integrated interface for PLC and HMI.

##### `FB_TimerTof` (FUNCTION_BLOCK) — §6.15.3

**Description:** Implements IEC 61131-3 timer function with integrated interface for PLC and HMI.

##### `FB_TimerTp` (FUNCTION_BLOCK) — §6.15.4

**Description:** Implements IEC 61131-3 timer function with integrated interface for PLC and HMI.

##### `FB_TimerWeekdayHmi` (FUNCTION_BLOCK) — §6.15.5

**Description:** HMI interface for a weekday timer for use with the TF8550 Control ProcessScheduler.

##### `FB_TimerWeekdayMaster` (FUNCTION_BLOCK) — §6.15.6

**Description:** Implements a weekday timer, compatible with IEC 61131-3 timers.

##### `FB_Time` (FUNCTION_BLOCK) — §6.15.7

**Description:** Class (FB) saves a point in time. The class makes it possible to interpret the time in various partial information and data types.

##### `F_GetCycleTime()` (FUNCTION) → `LREAL` — §6.15.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GetCycleTime` | `LREAL` | Cycle time in seconds as floating point value |

**Description:** Returns the cycle time of the calling task as LREAL floating point value in seconds.

##### `F_UpdateLocalSystemtime` (FUNCTION) — §6.15.9

**Description:** Updates the system time that can be retrieved via F_GetLocalSystemtime [} 360] and F_GetLocalSystemtimeEx [} 361]. This function only needs to be called cyclically if the application is operated without FB_BaseRuntime [} 311].

##### `F_GetLocalSystemtime` (FUNCTION) → `ULINT` — §6.15.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_` | `ULINT` | System time, based on the definition of T_FILETIME GetLocalSystemtime Possible events |
| `ID` | `Description` | Alarm/Message |
| `30x` | `Error when retrieving the local system time` | Message |

**Description:** Returns the local system time, taking into account the time zone. Is suitable for creating system time-related timestamps.

##### `F_GetLocalSystemtimeEx` (FUNCTION) → `ULINT` — §6.15.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | Net-ID of the system to be read |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_` | `ULINT` | System time, based on the definition of T_FILETIME GetLocalSystemtimeEx Possible events |
| `ID` | `Description` | Alarm/Message |
| `30x` | `Error when retrieving the local system time` | Message |

**Description:** Returns the system time of a device with AMS-NetID, taking into account the time zone. Is suitable for creating system time-related timestamps.

##### `F_SecondsToTime()` (FUNCTION) → `TIME` — §6.15.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fTime` | `LREAL` | Time value in seconds as floating point number |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_SecondsToTime` | `TIME` | Time value in milliseconds |

**Description:** Converts a time in seconds of type LREAL to milliseconds of type TIME.

##### `F_TimeToSeconds()` (FUNCTION) → `LREAL` — §6.15.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTime` | `TIME` | Time value in milliseconds |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_TimeToSeconds` | `LREAL` | Time value in seconds as floating point number |

**Description:** Converts a time in milliseconds of type TIME to seconds of type LREAL.

#### PLC-API - Class overview › Utilities › Files

##### `FB_LimitMemory` (FUNCTION_BLOCK) — §6.16.1.1

**Description:** The class analyzes the size of files in a directory. By setting limits, the files can be cleaned up as required by deleting the oldest variants.

##### `FB_CreateDirRecurse` (FUNCTION_BLOCK) — §6.16.1.2

**Description:** Implements the FB_CreateDir function and calls it several times until all components of the transferred path have been created.

##### `F_BaseDirectory` (FUNCTION) → `T_MaxString` — §6.16.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_BaseDirectory` | `T_MaxString` | Path to the default directory. |

**Description:** Creates the path to the default directory of the Plastic Base Application.

##### `F_ConcatFilepaths` (FUNCTION) → `T_MaxString` — §6.16.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sBase` | `T_MaxString` | Base path (relative or absolute path possible). |
| `sSub` | `T_MaxString` | Subpath (relative path) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ConcatFilepaths` | `T_MaxString` | Resulting overall path. |

**Description:** Handles the merging of two file paths. This guarantees that a separator is inserted between the two paths. Accordingly, a missing or double separator is avoided.

##### `F_GetBasePath` (FUNCTION) → `T_MaxString` — §6.16.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sPath` | `T_MaxString` | Absolute file path |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GetBasePath` | `T_MaxString` | Resulting base path |

**Description:** Extracts the base path from the provided path. Use the path separator specified in Tc3_PlasticBaseAppStaticParams [} 409].

##### `F_JoinFilename` (FUNCTION) → `T_MaxString` — §6.16.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sFile` | `T_MaxString` | File name or relative path. |
| `sBase` | `T_MaxString` | Absolute base path. Uses F_BaseDirectory() [} 367] if not |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_JoinFilename` | `T_MaxString` | Resulting overall path |

**Description:** Combines a filename or a relative path with an absolute base path and a file extension.

#### PLC-API - Class overview › Utilities

##### `FB_AdaptableSequence` (FUNCTION_BLOCK) — §6.16.2

**Description:** Allows variable extension of implemented procedures of a class. The class itself is used to manage all sequences and to indicate the state of a single sequence with reference to the management (master).

#### PLC-API - Class overview › Utilities › FB_AdaptableSequence

##### `Check()` (METHOD) → `BOOL` — §6.16.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Check` | `BOOL` | TRUE if the current step has been completed and the next |

**Description:** Checks whether the current step of the procedure has been processed. If this is the case, the method also causes switching to the next sequence.

##### `Clear()` (METHOD) → `HRESULT` — §6.16.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Clear` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Deletes all sequences from the master.

##### `Exists()` (METHOD) → `HRESULT` — §6.16.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iSeq` | `I_AdaptableSeqItf` | Process step to be checked |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Exists` | `HRESULT` | Return value with feedback on the success of the check |

**Description:** Checks whether a process step is included in the master.

##### `Idx()` (METHOD) → `I_AdaptableSeqItf` — §6.16.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The requested index |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Idx` | `I_AdaptableSeqItf` | Found step of the procedure Invalid indexes |

**Description:** Returns the xth step from the procedure.

##### `Insert()` (METHOD) → `HRESULT` — §6.16.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iCurrent` | `I_AdaptableSeqItf` | The current participant to be moved behind the new |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Insert` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Adds another step to the procedure.

##### `Jump()` (METHOD) → `HRESULT` — §6.16.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iTarget` | `I_AdaptableSeqItf` | Sequence to be jumped to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Jump` | `HRESULT` | Return value with feedback on the success of the request |

**Description:** Requests jumping to a sequence that does not follow.

##### `I_AdaptableSeqExt` (FUNCTION_BLOCK) — §6.16.2.7

**Description:** Allows the implementation of procedure steps outside the procedure-implementing class.

#### PLC-API - Class overview › Utilities

##### `FB_Button` (FUNCTION_BLOCK) — §6.16.3.1

**Description:** Implements a push button from a control panel.

##### `FB_ButtonRGB` (FUNCTION_BLOCK) — §6.16.3.2

**Description:** Implements a push button from a control panel with RGB indicator light.

##### `FB_CP22xx_Wheel` (FUNCTION_BLOCK) — §6.16.3.3

**Description:** Implements the CP22xx_1025 specific incremental encoder including the 10-segment LED ring.

##### `FB_RGBLed` (FUNCTION_BLOCK) — §6.16.3.4

**Description:** Implements an RGB LED with digitally controlled colors.

##### `FB_FlexValue` (FUNCTION_BLOCK) — §6.16.4

**Description:** Class (FB) comparable to the structured data type ST_FlexValue from TF8560. Represents a variable without a specified data type.

##### `FB_HandshakeHmi` (FUNCTION_BLOCK) — §6.16.5

**Description:** Implements a general handshake procedure with life sign mechanism and companion identification.

##### `FB_ListQueryHmi` (FUNCTION_BLOCK) — §6.16.6

**Description:** This class (FB) is the basis for an iterative data exchange with the corresponding HMI function ListQueryPlc. To transmit information, inherit from this class and add class properties with the data per entry. Communication takes place according to the following sequence scheme:

##### `FB_ScaleHmi` (FUNCTION_BLOCK) — §6.16.7

**Description:** Used as a common basis for scaling values.

##### `FB_TrendHmi` (FUNCTION_BLOCK) — §6.16.8

**Description:** Support class for the TwinCAT HMI SQLiteTrend with pre-implemented views. The TF8550 function TcHmi.Functions.Plastic.UpdateTrend() is required for use. The selected view is additionally determined by the property SelectedView. The following objects with corresponding interface can be attached to the trend:

#### PLC-API - Class overview › Utilities › FB_TrendHmi

##### `Append()` (METHOD) → `HRESULT` — §6.16.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObj` | `I_Base` | Object to be included in the trend |
| `nStandardView` | `INT` | Standard view in which the value is displayed |
| `nYAxisID` | `INT` | Y-axis in the HMI on which the value is displayed |

**Description:** Appends a value to the trend.

#### (top level)

##### `RPM Turnrate` — §4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Append` | `HRESULT` | Return value with feedback on the success of the execution |

#### PLC-API - Class overview › Utilities › FB_TrendHmi

##### `AppendTempChannels()` (METHOD) → `HRESULT` — §6.16.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipTemp` | `I_Temperature` | Instance of the temperature control class (FB) |
| `nYAxisID` | `INT` | Y-axis in the HMI on which the value is displayed |

**Description:** Appends the temperature configuration to the trend. If the group configuration of all zones changes, this is automatically taken over by FB_TrendHmi.

#### (top level)

##### `RPM Turnrate` — §4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendTempChannels HRESULT` | `Return value with feedback on the success of the execution` | — |

#### PLC-API - Class overview › Utilities › FB_TrendHmi

##### `ConfigDisplayName()` (METHOD) → `HRESULT` — §6.16.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObj` | `I_Base` | Object whose name is to be adjusted |
| `sName` | `STRING(255)` | String to be displayed |
| `bAppend` | `BOOL` | Passed string is to be appended to the existing display |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigDisplayName` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Adjusts the display name for a trend value.

##### `ConfigDisplayLocalize()` (METHOD) → `HRESULT` — §6.16.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObj` | `I_Base` | Object whose name is to be adjusted. |
| `sLocalization` | `T_MaxString` | Localization key to be displayed. |
| `bAppend` | `BOOL` | Passed localization key is to be appended to the existing |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigDisplayLocalisati HRESULT` | `Return value with feedback on the success of the execution on` | — |

**Description:** Adjusts the display name for a trend value. The transferred key is embedded in the TF8550 function Plastic.Localize().

##### `ConfigView()` (METHOD) → `HRESULT` — §6.16.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iObj` | `I_Base` | Object to be adjusted in a view |
| `nViewIdx` | `INT` | View (ID) to be customized |
| `bShow` | `BOOL` | Object is displayed (TRUE). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigView` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Changes the composition of a selectable view.

#### PLC-API - Class overview › Utilities › Math

##### `F_CutDecimals` (FUNCTION) → `LREAL` — §6.16.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fValue` | `LREAL` | Value whose decimal places are truncated. |
| `nDecimals` | `UINT` | Number of decimal places to be retained. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_CutDecimals` | `LREAL` | Value with truncated decimal places. |

**Description:** Truncates the decimal places of a floating point number.

##### `F_Round` (FUNCTION) → `LREAL` — §6.16.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fValue` | `LREAL` | Value whose decimal places are truncated. |
| `nDecimals` | `UINT` | Number of decimal places to be retained. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_Round` | `LREAL` | Value with rounded decimal places. |

**Description:** Rounds the decimal places of a floating point number.

##### `F_TryDivide()` (FUNCTION) → `HRESULT` — §6.16.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fNominator` | `LREAL` | Value to be divided |
| `fDenominator` | `LREAL` | Value by which to divide |
| `refResult` | `REFERENCE TO LREAL` | Result of the division |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_TryDivide` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Divides two values without throwing an exception.

#### PLC-API - Class overview › Utilities › ParameterList

##### `FB_Parameter` (FUNCTION_BLOCK) — §6.16.10.1

**Description:** Allows concatenation of multiple 64-bit floating-point (LREAL) values with assignable name.

##### `FB_ParameterHmi` (FUNCTION_BLOCK) — §6.16.10.2

**Description:** HMI class parallel to the base parameter class. This enables the provision of FB_Parameter instances in the HMI.

#### PLC-API - Class overview › Utilities › String

##### `F_FindC` (FUNCTION) → `INT` — §6.16.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sInp` | `T_MaxString` | String to be searched. |
| `sFind` | `T_MaxString` | String to search for. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_FindC` | `INT` | Number of occurrences of the search string. If it doesn't |

**Description:** Counts the number of occurrences of the passed string.

##### `F_FindL` (FUNCTION) → `INT` — §6.16.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sInp` | `T_MaxString` | String to be searched. |
| `sFind` | `T_MaxString` | String to search for. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_FindC` | `INT` | Position of the last occurrence of the search string. If it |

**Description:** Finds the position of the first character of a given string within another string. Unlike the FIND() function, F_FindL returns the last occurrence. This is useful, for example, when the search string appears multiple times.

#### PLC-API - Class overview › Utilities › TableGenerator

##### `FB_FileTableGenerator` (FUNCTION_BLOCK) — §6.16.13.1

**Description:** Implements the basic functionality for file-based table reading mechanisms.

##### `FB_CsvTableGenerator` (FUNCTION_BLOCK) — §6.16.13.2

**Description:** Loads a table from a CSV-based file.

##### `FB_TableGeneratorAsciiFile` (FUNCTION_BLOCK) — §6.16.13.3

**Description:** Implements a file read mechanism for a user-defined table (e.g. for use as a transformation table).

#### PLC-API - Class overview › Utilities › Trigger

##### `FB_LatchTrigger` (FUNCTION_BLOCK) — §6.16.14.1

**Description:** Extends the FB_Trigger class by a memory function of the edges.

##### `FB_Trigger` (FUNCTION_BLOCK) — §6.16.14.2

**Description:** Combines triggers of type R_TRIG and F_TRIG with object-oriented interface.

#### PLC-API - Class overview › Utilities › Version

##### `FB_LibVersion` (FUNCTION_BLOCK) — §6.16.15.1

**Description:** Defines the structure of the version number of a library.

##### `FB_LibVersionBeta` (FUNCTION_BLOCK) — §6.16.15.2

**Description:** Defines the structure of the version number of a library, plus a tag. The property FB_LibVersionBeta.Version thus presents itself as 'v5.3.32.9-beta554', for example.

##### `F_CheckTcVersion` (FUNCTION) → `BOOL` — §6.16.15.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_CheckTcVersion` | `BOOL` | TRUE as soon as the TwinCAT version has been |

**Description:** This function queries the TwinCAT version installed on the running system in an asynchronous process. This function must be called several cycles until the return value is TRUE.

##### `F_GetTcVersion` (FUNCTION) → `I_LibVersionRead` — §6.16.15.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GetTcVersion` | `I_LibVersionRead` | Interface to an object of type FB_LibVersion [} 406]. |

**Description:** This function returns an interface of type I_LibVersionRead [} 406]. The TwinCAT version that is active on the executing system can be queried via the interface.

#### PLC › Object orientation

##### `Adding a variable to a class (FB)` (FUNCTION_BLOCK) — §8.1.2.1

**Description:** If the scope of the internal variables of a class (FB) is to be extended, proceed as follows. In the following example, a variable is added to the default axis type FB_Axis. 1. Create a new class (FB) and remove VAR_INPUT and VAR_OUTPUT.

##### `Adding a property or method to a class (FB)` (FUNCTION_BLOCK) — §8.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bCommand` | `BOOL` | — |
| `bCommand` | `BOOL` | — |
| `bCommand` | `BOOL` | — |

**Description:** In many cases, the scope of methods and properties of a class should be changed. This includes adding new elements as well as changing or removing existing elements. In the following samples, these three procedures are explained using the standard axle type FB_Axis as an example.

##### `Adapting inner procedures of a class (FB)` (FUNCTION_BLOCK) — §8.1.2.3

**Description:** Some classes contain inner flows/procedures which are to be extended/changed by inheritance levels or adapted by the application. This is realized with the FB_AdaptableSequence [} 369] class. 1. Create a new class (FB) and let it inherit from a class with inner procedure. VAR

##### `Extending the HMI parallel class (FB)` (FUNCTION_BLOCK) — §8.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipBaseHmi` | `I_BaseHmi;           // interface on hmi object` | — |
| `ipBase` | `I_Base;    // Base interface on linked control class (FB)` | — |

**Description:** In many use cases, the number of displayed values on the surface is to be extended. For this purpose, the HMI parallel class can be extended so that the new values are accessible in the HMI environment at a suitable location. ü For the complete implementation the extension of the base class (FB) is necessary. 1. Perform steps 1 to 5 of Base class extension (FB) for the base class. 2. Create a new class (FB) and remove VAR_INPUT and VAR_OUTPUT. VAR

#### PLC › Axes

##### `Integrating manual function into an axis` (FUNCTION_BLOCK) — §8.1.3.3

**Description:** A manual function can be created in several places in the application. The following sample explains how a manual function for moving and switching on the axis can be integrated into it. 1. Extend an existing class by inheritance. In this sample, the FB_Carriage class is used for this purpose.

#### PLC › Temperature control

##### `Instantiating and initiating temperature control` (METHOD) — §8.1.5.1

**Description:** For certain applications, only the temperature control should be used from the scope of the TwinCAT 3 Plastic Application. In this, but also in the normal use case, the temperature controller must be capable of being instantiated and initiated individually for this purpose. In the following sample, we will go through this process step by step.

#### Blow Molding Panel PC

##### `Central operating function` (FUNCTION_BLOCK) — §8.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInit` | `BOOL;                        // TRUE, if the external states should insert themse` | — |
| `iMaster` | `I_AdaptableSeqMaster;        // Interface on the state master` | — |
| `bInit` | `BOOL;                       // TRUE, if the external states should insert thems` | — |
| `iMaster` | `I_AdaptableSeqMaster;       // Interface on the state master` | — |

**Description:** A central operating function, such as the incremental encoder of the CP2219-1025-0030 [} 588] BlowMolding Panel PC, can always be connected to exactly one task. The ManualOperation Control in the HMI allows you to select this central operating function. On the PLC side, it must also be defined how the interaction should take place logically. 1. Implement the interfaces I_AdaptableSeqExt [} 373] and an interface derived from I_RuntimeInterface [} 313] in a class (here FB_Sample as an example). This class represents the location where you want to program the manual function.

#### PLC-API (outdated)

##### `F_TryDevide()` (FUNCTION) → `HRESULT` — §9.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fNominator` | `LREAL` | Value to be divided |
| `fDenominator` | `LREAL` | Value by which to divide |
| `refResult` | `REFERENCE TO LREAL` | Result of the division |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_TryDevide` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Alternative: F_TryDivide [} 393]

##### `FB_TrafoTableGenerator` (FUNCTION_BLOCK) — §9.1.2

**Description:** Alternative: TF8560 –Tc3_PlasticFunctions.FB_TrafoTableGenerator

#### PLC-API (outdated) › FB_TrafoTableGenerator

##### `FB_TrafoScope` (FUNCTION_BLOCK) — §9.1.2.1

**Description:** Alternative: - (completely removed)

##### `FB_TableGeneratorCrankStandard` (FUNCTION_BLOCK) — §9.1.2.3

**Description:** Alternative: TF8560 – Tc3_PlasticFunctions.FB_CrankTableGenerator

##### `FB_TableGeneratorScotchYoke` (FUNCTION_BLOCK) — §9.1.2.4

**Description:** Alternative: TF8560 – Tc3_PlasticFunctions.FB_ScotchYokeTableGenerator

#### PLC-API (outdated)

##### `FB_MonitoringZone` (FUNCTION_BLOCK) — §9.1.3

**Description:** Alternative: FB_MonitoringTemp [} 60]

##### `FB_TempCtrl` (FUNCTION_BLOCK) — §9.1.4

**Description:** Alternative: FB_Temperature [} 323]

#### PLC-API (outdated) › FB_TempCtrl

##### `CreateDefaultParams()` (METHOD) → `HRESULT` — §9.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bAllInUse` | `BOOL` | All zones are initialized as "InUse". TRUE |
| `eSensor` | `E_TcPfw_TempS Sensor type - NoSensor in simulation mode eTcPfwTempSensT_NoSensor ensType` | — |
| `eTerminal E_TcPfwTerminal Terminal type - NoTerminal in simulation` | `eTcPfwTermT_NoTerminal` | — |
| `Type` | `mode` | — |
| `nChPerTer INT` | `Number of channels per terminal` | 8 m |
| `eOutHeatin E_TcPfw_TctrlOu Output type of the heating output -` | `eTcPfwTcOut_PWM` | — |
| `g` | `tSelect` | NoSignal, to disable the heating function (e.g. for measuring zones) |
| `eOutCoolin E_TcPfw_TctrlOu Cooling output type` | `eTcPfwTcOut_NoSignal` | — |
| `g` | `tSelect` | — |
| `fSetpoint` | `LREAL` | Temperature setpoint for all zones 180.0 |
| `fStandbySet LREAL` | `Temperature setpoint for standby` | 18.0 |
| `p` | `temperature of all zones` | — |
| `fPwmCycle LREAL` | `PWM cycle time for all outputs (dutyCycle = 1.0` | — |
| `Time` | `fPwmCycleTime * 0.1)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateDefaultParams` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Creates a default parameterization for all temperature zones.

##### `EnableAll()` (METHOD) — §9.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bCmd` | `BOOL` | TRUE to grant the release, FALSE to withdraw the release. |
| `bGroups` | `BOOL` | The enable only takes into account zones that are assigned |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EnableAll` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Enables all temperature zones on the PLC side.

##### `LinkGroup()` (METHOD) → `HRESULT` — §9.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nStartIdx` | `INT` | Index of the first zone to be assigned from the linear TF8540 library arrangement |
| `nEndIdx` | `INT` | Index of the last zone to be assigned from the linear TF8540 library arrangement |
| `nGroupIdx` | `INT` | Index of the group to which the zones are to be assigned |
| `bOverwrite` | `BOOL` | Zones are assigned even if the group contains already |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LinkGroup` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Assigns a set of zones to a group.

##### `LinkSupply()` (METHOD) → `HRESULT` — §9.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGroupIdx` | `INT` | Index of the group to which a supply unit is to be assigned |
| `nSupplyIdx` | `INT` | Index of the supply unit to be assigned to the group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LinkSupply` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Assigns a supply unit to a group.

##### `LinkZone()` (METHOD) → `HRESULT` — §9.1.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLinearIdx` | `INT` | Index of the zone from the linear TF8540 library arrangement that is to be assigned |
| `nGroupIdx` | `INT` | Index of the group to which the zone should be assigned |
| `nGroupMemberIdx` | `INT` | Index in the target group |
| `bOverwrite` | `BOOL` | Zone is assigned even if the index is already occupied in |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LinkZone` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Assigns a single zone to a group.

##### `SetOpMode()` (METHOD) → `HRESULT` — §9.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `OpMode` | `INT` | Selection parameters: 0 – None, 1 – Simple, 2 - Standard |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetOpMode` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Defines the current OpMode of the temperature control. • OpMode None (0) • OpMode Simple (1) • OpMode Standard (2)

##### `StandbyAll()` (METHOD) → `HRESULT` — §9.1.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bCmd` | `BOOL` | TRUE to enable the standby temperature, FALSE to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StandbyAll` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** Sets all zones to standby temperature.

#### PLC-API (outdated)

##### `FB_TempCtrlHmi` (FUNCTION_BLOCK) — §9.1.5

**Description:** Alternative: FB_TemperatureHmi [} 330]

##### `FB_TempGroup` (FUNCTION_BLOCK) — §9.1.6

**Description:** Alternative: FB_TemperatureGroup [} 332]

##### `FB_TempGroupHmi` (FUNCTION_BLOCK) — §9.1.7

**Description:** Alternative: FB_TemperatureGroupHmi [} 337]

##### `FB_TempGroupOpModeHmi` (FUNCTION_BLOCK) — §9.1.8

**Description:** Alternative: - (integrated in FB_TemperatureGroupHmi [} 337])

##### `FB_TempZone` (FUNCTION_BLOCK) — §9.1.9

**Description:** Alternative: FB_TempChannel [} 341]

##### `FB_TempZoneHmi` (FUNCTION_BLOCK) — §9.1.10

**Description:** Alternative: FB_TempChannelBase [} 340]

##### `FB_AxisSpecific` (FUNCTION_BLOCK) — §9.1.11

**Description:** Alternative: FB_MotionSpecific [} 213]

##### `FB_WtcTimeMaster` (FUNCTION_BLOCK) — §9.1.12

**Description:** Alternative: FB_WtcTimer [} 100]

##### `FB_WtcAccuMaster` (FUNCTION_BLOCK) — §9.1.13

**Description:** Alternative: FB_WtcAccumulator [} 103]

##### `FB_ManualPtp` (FUNCTION_BLOCK) — §9.1.14

**Description:** Alternative: FB_ManualBaseWork [} 202]

##### `FB_BaseCammingHmi` (FUNCTION_BLOCK) — §9.1.16

**Description:** Alternative: FB_CamAxisHmi [} 243] and FB_CurveHmi [} 286]

##### `FB_Homing` (FUNCTION_BLOCK) — §9.1.17

**Description:** Alternative: FB_Referencing [} 222]

##### `FB_CamCfgHmi` (FUNCTION_BLOCK) — §9.1.18

**Description:** Alternative: FB_CamsHmi [} 281] (consisting of FB_CamSwitchHmi [} 283])
