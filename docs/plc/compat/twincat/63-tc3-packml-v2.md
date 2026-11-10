# Tc3_PackML_V2

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_PackML_V2_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Industry · **Tier:** T0 · **API entries:** 21
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** PackML V2: same, extended states.

---

#### Packaging Machine State › Function Blocks › Packaging Machine State

##### `PML_StateMachine` — §2.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `DINT` | Current PML UnitMode |
| `eCommand` | `E_PMLCommand` | Enumeration [} 10] with the various PML commands of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eState` | `E_PMLState` | Enumeration [} 10] that delivers the current PML state of |

**Description:** In the updated form the PML_StateMachine function block has a common interface with the PackML Machine State Model V3. It is assumed that application-specific logic, such as state transitions, is programmed in external function blocks and that the PML_StateMachine function block deals with the central logic of the state machine and the state representation. The Machine State Model has a different appearance due to the currently active UnitMode (eMode). Three basic modules are preconfigured for this (E_PMLProtectedUnitMode [} 11]).

##### `PML_UnitModeConfig` — §2.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `DINT` | — |
| `sName` | `STRING` | — |
| `bDisableClearing` | `BOOL` | — |
| `bDisableStarting` | `BOOL` | — |
| `bDisableSuspended` | `BOOL` | — |
| `bDisableStopping` | `BOOL` | — |
| `bDisableAborting` | `BOOL` | — |
| `bDisableHolding` | `BOOL` | — |
| `bDisableHeld` | `BOOL` | — |
| `bDisableUnholding` | `BOOL` | — |
| `bDisableSuspending` | `BOOL` | — |
| `bDisableUnsuspending` | `BOOL` | — |
| `bDisableResetting` | `BOOL` | — |
| `bDisableIdle` | `BOOL` | — |
| `bDisableCompleting` | `BOOL` | — |
| `bDisableComplete` | `BOOL` | — |
| `bEnableUnitModeChangeStopped` | `BOOL` | — |
| `bEnableUnitModeChangeIdle` | `BOOL` | — |
| `bEnableUnitModeChangeSuspended` | `BOOL` | — |
| `bEnableUnitModeChangeExecute` | `BOOL` | — |
| `bEnableUnitModeChangeAborted` | `BOOL` | — |
| `bEnableUnitModeChangeHeld` | `BOOL` | — |
| `bEnableUnitModeChangeComplete` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error occurs. |
| `nErrorId` | `UDINT` | Supplies the error number when the bError output is set. |

**Description:** Machines may have unit modes other than "Production", "Maintenance" and "Manual". This function block enables the user to configure further models (UnitModes).

##### `PML_UnitModeManager` — §2.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | Mode change on rising edge |
| `eModeCommand` | `DINT` | Requested mode |
| `ePMLState` | `E_PMLState` | Enumeration [} 10] that delivers the current PML state of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eModeStatus` | `DINT` | Current PML UnitMode |
| `sModeStatus` | `STRING` | Name of the current PML UnitMode |
| `bDone` | `BOOL` | Becomes TRUE as soon as the mode change has been |

**Description:** Machines have other system modes apart from “Production”. Each unit mode is defined by its own state model. A "Mode Manager" must be defined for transitions between the modes. The “Mode Manager” decides how and in which state a machine can change unit modes; i.e. built-in barriers prevent the machine from changing to unsuitable states. These barriers are permanently defined for the “Production”, “Maintenance” and “Manual” base modes, as the illustration below shows. This can be individually specified for other modes defined via the PML_UnitModeConfig [} 17] function block.

#### Packaging Machine State › Function Blocks › General › PML_AdminAlarm

##### `M_SetAlarm` (METHOD) → `BOOL` — §2.3.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stAlarm` | `ST_Alarm` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method inserts an alarm in the Admin-Tags. Alarm[].Trigger is set to TRUE and the value from Admin.PlcDateTime is entered in Alarm[].DateTime. The other values are taken from the transferred alarm structure. The method returns TRUE if the alarm was entered successfully

##### `M_AcknowledgeAlarm` (METHOD) → `BOOL` — §2.3.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stAlarm` | `ST_Alarm` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method acknowledges an alarm in the Admin-Tags. Alarm[].Trigger is set to FALSE and the value from Admin.PlcDateTime is entered in Alarm[].AckDateTime. The method returns TRUE if the alarm was found and acknowledged successfully. Acknowledging the alarm does not delete it. The alarm remains in the Alarm array until an M_ClearAlarm has been called, then it is moved to the AlarmHistory array. If the AlarmHistory array is already full of entries, the oldest entry is deleted as a result.

##### `M_ClearAlarm` (METHOD) → `BOOL` — §2.3.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stAlarm` | `ST_Alarm` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method deletes an alarm from the Admin-Tags. Alarm[].Trigger is set to FALSE. The method returns TRUE if the alarm was deleted successfully. The alarm remains in the Alarm array until an M_AcknowledgeAlarm has been called, then it is moved to the AlarmHistory array. If the AlarmHistory array is already full of entries, the oldest entry is deleted as a result.

##### `M_SetWarning` (METHOD) → `BOOL` — §2.3.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stWarning` | `ST_Alarm` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method inserts a warning in the Admin-Tags. Warning[].Trigger is set to TRUE and the value from Admin.PlcDateTime is entered in Warning[].DateTime. The other values are taken from the transferred warning structure. The method returns TRUE if the warning was entered successfully. If the Warning array is already full of entries, the oldest entry is deleted as a result.

##### `M_AcknowledgeWarning` (METHOD) → `BOOL` — §2.3.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stWarning` | `ST_Alarm` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method acknowledges a warning in the Admin-Tags. Warning[].Trigger is set to FALSE and the value from Admin.PlcDateTime is entered in Warning[].AckDateTime. The method returns TRUE if the warning was found and acknowledged successfully. The warning remains in the Warning array until it is pushed out of the array by the next warning.

##### `M_ClearWarning` (METHOD) → `BOOL` — §2.3.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stWarning` | `ST_Alarm` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method deletes a warning from the Admin-Tags. Warning[].Trigger is set to FALSE. The method returns TRUE if the warning was deleted successfully. The warning remains in the Warning array until it is pushed out of the array by the next warning.

##### `M_SetStopReason` (METHOD) → `BOOL` — §2.3.2.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stStopReason` | `ST_Alarm` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method inserts a StopReason in the Admin-Tags. StopReason[].Trigger is set to TRUE and the value from Admin.PlcDateTime is entered in StopReason[].DateTime. The other values are taken from the transferred StopReason structure. The method returns TRUE if the StopReason was entered successfully. If the StopReason array is already full of entries, the oldest entry is deleted as a result.

##### `M_AcknowledgeStopReason` (METHOD) → `BOOL` — §2.3.2.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stStopReason` | `ST_Alarm` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method acknowledges a StopReason in the Admin-Tags. StopReason[].Trigger is set to FALSE and the value from Admin.PlcDateTime is entered in StopReason[].AckDateTime. The method returns TRUE if the StopReason was found and acknowledged successfully. The StopReason remains in the StopReason array until it is pushed out of the array by the next StopReason.

##### `M_ClearStopReason` (METHOD) → `BOOL` — §2.3.2.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stStopReason` | `ST_Alarm` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |

**Description:** This method deletes a StopReason from the Admin-Tags. StopReason[].Trigger is set to FALSE. The method returns TRUE if the StopReason was deleted successfully. The StopReason remains in the StopReason array until it is pushed out of the array by the next StopReason.

#### Packaging Machine State › Function Blocks › General

##### `PML_AdminTime` — §2.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bReset` | `BOOL` | A signal at this input resets the recorded times |
| `stOptions` | `ST_AdminTimeOptions` | Additional options of the function block |
| `stAdmin` | `ST_PMLa` | Transfer of the Admin-PackTags |
| `stStatus` | `ST_PMLs` | Transfer of the Status-PackTags |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAdmin` | `ST_PMLa` | — |
| `stStatus` | `ST_PMLs` | — |

**Description:** This function block should be called cyclically and it then fills the following Admin-PackTags: • PlcDateTime • AccTimeSinceReset • ModeCurrentTime[ ] • ModeCumulativeTime[ ] • StateCurrentTime[ ][ ] • StateCumulativeTime[ ][ ]

#### Packaging Machine State › Function Blocks › Conversion › Time

##### `LTIME_TO_PackMLTime` (FUNCTION) → `ARRAY` — §2.3.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `LTIME` | The time value to be converted. |

**Description:** This function converts a time value in LTIME format into the PackML-compliant array.

##### `TIME_TO_PackMLTime` (FUNCTION) → `ARRAY` — §2.3.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `TIME` | The time value to be converted. |

**Description:** This function converts a time value in TIME format into the PackML-compliant array.

##### `ULINT_TO_PackMLTime` (FUNCTION) → `ARRAY` — §2.3.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ULINT` | The time value to be converted. |

**Description:** This function converts a time value in ULINT format into the PackML-compliant array.

#### Packaging Machine State › Function Blocks › Conversion › Timestamp

##### `DCTIME64_TO_PackMLTime` (FUNCTION) → `ARRAY` — §2.3.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `DCTIME64` | The time to be converted |

**Description:** This function converts a time in DCTIME64 format into the PackML-compliant array.

##### `DT_TO_PackMLTime` (FUNCTION) → `ARRAY` — §2.3.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `DT` | The time to be converted |

**Description:** This function converts a time in DT format into the PackML-compliant array.

##### `TIMESTRUCT_TO_PackMLTime` (FUNCTION) → `ARRAY` — §2.3.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `TIMESTRUCT` | The time to be converted |

**Description:** This function converts a time in TIMESTRUCT format into the PackML-compliant array.

#### Packaging Machine State › Function Blocks › Conversion

##### `F_StateCommandToString` (FUNCTION) → `STRING;` — §2.3.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eStateCommand` | `E_PMLCommand` | The state command for which the name is to be determined |

**Description:** This function outputs the name of a state command as a string.

##### `F_UnitModeToString` (FUNCTION) → `STRING;` — §2.3.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `DINT` | The Unit Mode for which the name is to be determined |

**Description:** This function returns the name of a Unit Mode as a string.
