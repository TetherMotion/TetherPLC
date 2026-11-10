# Tc3_PackML_V3

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_PackML_V3_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Industry · **Tier:** T0 · **API entries:** 29
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** PackML V3: same, extended states.

---

#### Function Blocks › Conversion › Time

##### `LTIME_TO_PMLTime` (FUNCTION) → `ST_PMLDateAndTime` — §4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `LTIME` | The time value to be converted |

**Description:** The LTIME_TO_PMLTime function converts a time value in LTIME format to the PackML-compliant format.

##### `TIME_TO_PMLTime` (FUNCTION) → `ST_PMLDateAndTime` — §4.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `TIME` | The time value to be converted |

**Description:** The TIME_TO_PMLTime function converts a time value in TIME format to the PackML-compliant format.

##### `ULINT_TO_PMLTime` (FUNCTION) → `ST_PMLDateAndTime` — §4.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ULINT` | The time value to be converted |

**Description:** The ULINT_TO_PMLTime function converts a time value in ULINT format to the PackML-compliant format.

#### Function Blocks › Conversion › Timestamp

##### `DCTIME64_TO_PMLTime` (FUNCTION) → `ST_PMLDateAndTime` — §4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `DCTIME64` | The time to be converted. |

**Description:** The DCTIME64_TO_PMLTime function converts a time in DCTIME64 format to the PackML-compliant format.

##### `DT_TO_PMLTime` (FUNCTION) → `ST_PMLDateAndTime` — §4.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `DATE_AND_TIME` | The time to be converted. |

**Description:** The DT_TO_PMLTime function converts a time in DT format to PackML-compliant format.

##### `TIMESTRUCT_TO_PMLTime` (FUNCTION) → `ST_PMLDateAndTime` — §4.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `TIMESTRUCT` | The time to be converted. |

**Description:** The TIMESTRUCT_TO_PMLTime function converts a time in TIMESTRUCT format to the PackML- compliant format.

#### Function Blocks › Conversion

##### `F_PMLStateCommandToString` (FUNCTION) → `STRING;` — §4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eStateCommand` | `E_PMLCommand` | The state command for which the name is to be |

**Description:** The F_PMLStateCommandToString function outputs the name of a state command as a string.

##### `F_PMLUnitModeToString` (FUNCTION) → `STRING;` — §4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `DINT` | The Unit Mode for which the name is to be determined. |

**Description:** The F_PMLUnitModeToString function returns the name of a Unit Mode as a string.

#### Function Blocks › General › FB_PMLAdminAlarm › Alarm

##### `M_AcknowledgeAlarm` (METHOD) → `BOOL` — §4.2.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stAlarm` | `ST_PMLEvent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** stAlarm ST_PMLEvent

##### `M_AcknowledgeAllAlarms` (METHOD) → `BOOL` — §4.2.1.1.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method acknowledges all alarms in the admin tags. Alarm[].Trigger is set to FALSE and the value from Admin.PlcDateTime is entered in Alarm[].AckDateTime. If all alarms have been acknowledged, the method returns TRUE. Acknowledging the alarms does not delete them. The alarms remain in the alarm array until an M_ClearAlarm or M_ClearAllAlarms call has also been made, at which point the alarms in question are moved to the AlarmHistory array. If the AlarmHistory array is already full of entries, the oldest entries are deleted as a result.

##### `M_ClearAlarm` (METHOD) → `BOOL` — §4.2.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stAlarm` | `ST_PMLEvent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** stAlarm ST_PMLEvent

##### `M_ClearAllAlarms` (METHOD) → `BOOL` — §4.2.1.1.4

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method deletes all alarms in the Admin-Tags. Alarm[].Trigger is set to FALSE. If the alarms were successfully deleted, the method returns TRUE. The alarms remain in the alarm array until an M_AcknowledgeAlarm or M_AcknowledgeAllAlarms call has also been made, at which point they are moved to the AlarmHistory array. If the AlarmHistory array is already full of entries, the oldest entries are deleted as a result.

##### `M_GetAlarmCategory` (METHOD) → `BOOL` — §4.2.1.1.5

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method returns the highest AlarmCategory (smallest value) in the alarms of the Admin-Tags.

##### `M_HasAlarm` (METHOD) → `BOOL` — §4.2.1.1.6

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method checks whether an active alarm is entered in the Admin-Tags and the method returns TRUE

##### `M_SetAlarm` (METHOD) → `BOOL` — §4.2.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stAlarm` | `ST_PMLEvent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** stAlarm ST_PMLEvent

#### Function Blocks › General › FB_PMLAdminAlarm › StopReason

##### `M_AcknowledgeStopReason` (METHOD) → `BOOL` — §4.2.1.2.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** 26

##### `M_ClearStopReason` (METHOD) → `BOOL` — §4.2.1.2.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method deletes the StopReason from the Admin-Tags. StopReason.Trigger is set to FALSE. The method returns TRUE if the StopReason was deleted successfully. The StopReason remains in the Admin- Tags until it is replaced by a subsequent StopReason.

##### `M_HasStopReason` (METHOD) → `BOOL` — §4.2.1.2.3

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method checks whether an active StopReason is entered in the Admin-Tags and the method returns TRUE

##### `M_SetStopReason` (METHOD) → `BOOL` — §4.2.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stStopReason` | `ST_PMLEvent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** stStopReason ST_PMLEvent

#### Function Blocks › General › FB_PMLAdminAlarm › Warning

##### `M_AcknowledgeAllWarning` (METHOD) → `BOOL` — §4.2.1.3.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method acknowledges all warnings in the Admin-Tags. Warning[].Trigger is set to FALSE and the value from Admin.PlcDateTime is entered in Warning[].AckDateTime. If all warnings are acknowledged, the method returns TRUE. The warnings remain in the Warning array until they are pushed out of the array by the next warnings.

##### `M_AcknowledgeWarning` (METHOD) → `BOOL` — §4.2.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stWarning` | `ST_PMLEvent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** stWarning ST_PMLEvent

##### `M_ClearAllWarning` (METHOD) → `BOOL` — §4.2.1.3.3

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method deletes all warnings in the Admin-Tags. Warning[].Trigger is set to FALSE. The method returns TRUE if the warning was deleted successfully. The warnings remain in the Warning array until they are pushed out of the array by the next warnings.

##### `M_ClearWarning` (METHOD) → `BOOL` — §4.2.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stWarning` | `ST_PMLEvent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** stWarning ST_PMLEvent

##### `M_HasWarning` (METHOD) → `BOOL` — §4.2.1.3.5

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method checks whether an active warning is entered in the Admin-Tags and the method returns TRUE

##### `M_SetWarning` (METHOD) → `BOOL` — §4.2.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stWarning` | `ST_PMLEvent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** stWarning ST_PMLEvent

#### Function Blocks › General

##### `FB_PMLAdminTime` (METHOD) → `BOOL` — §4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stOptions` | `ST_PMLAdminTimeOptio Additional options of the function block ns` | — |
| `stAdmin` | `ST_PMLa` | Transfer of the Admin-PackTags |
| `stStatus` | `ST_PMLs` | Transfer of the Status-PackTags M_ResetCumulativeTime M_ResetCumulativeTime |
| `CumulativeTimesIdx UDINT` | `BOOL M_ResetCumulativeTime` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |
| `stStatus` | `ST_PMLs` | — |

**Description:** stOptions ST_PMLAdminTimeOptions

#### Function Blocks › Packaging Machine State

##### `FB_PMLStateMachine` — §4.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eState` | `E_PMLState` | Enumeration [} 46] that delivers the current PML state of |

**Description:** In its updated form, the PML_StateMachine function block has a common interface to the PackML Machine State Model V3. It is expected that application-specific logic, such as state transitions, will be programmed in external function blocks and that the PML_StateMachine function block will take over the central logic of the state machine and the state representation. The Machine State Model has a different appearance due to the currently active UnitMode (eMode). Three basic models are preconfigured for this purpose (E_PMLProtectedUnitMode [} 44]).

##### `FB_PMLUnitModeConfig` — §4.3.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error occurs. |
| `nErrorId` | `UDINT` | Supplies the error number when the bError output is set. |

**Description:** bDisableClearing BOOL bDisableStarting BOOL bDisableSuspended BOOL bDisableStopping BOOL bDisableAborting BOOL bDisableHolding BOOL bDisableHeld BOOL bDisableUnholding BOOL bDisableSuspending BOOL bDisableUnsuspending BOOL bDisableResetting BOOL bDisableIdle BOOL bDisableCompleting BOOL bDisableComplete BOOL bEnableUnitModeChangeStopped BOOL

##### `FB_PMLUnitModeManager` — §4.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | Mode change on rising edge |
| `eModeCommand` | `DINT` | Requested mode |
| `eState` | `E_PMLState` | Enumeration [} 46] that delivers the current PML state of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eModeStatus` | `DINT` | Current PML UnitMode |
| `sModeStatus` | `STRING` | Name of the current PML UnitMode |
| `bDone` | `BOOL` | Becomes TRUE as soon as the mode change has been |

**Description:** Machines have other system modes apart from “Production”. Each unit mode is defined by its own state model. A "Mode Manager" must be defined for transitions between the modes. The “Mode Manager” decides how and in which state a machine can change unit modes; i.e. built-in barriers prevent the machine from changing to unsuitable states. These barriers are predefined for the base modes “Production”, “Maintenance” and “Manual”, as the illustration below shows. This can be individually specified for other modes defined via the FB_PMLUnitModeConfig [} 38] function block.
