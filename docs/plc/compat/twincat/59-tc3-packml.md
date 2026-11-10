# Tc3_PackML

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_PackML_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Industry · **Tier:** T0 · **API entries:** 5
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** PackML state machines: pure logic over image IO.

---

#### Packaging Machine State › Function Blocks

##### `PS_PackML_StateMachine_Auto` — §3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Start` | `BOOL` | — |
| `Hold` | `BOOL` | — |
| `unHold` | `BOOL` | — |
| `Suspend` | `BOOL` | — |
| `unSuspend` | `BOOL` | — |
| `Abort` | `BOOL` | — |
| `Stop` | `BOOL` | — |
| `Complete` | `BOOL` | — |
| `Clear` | `BOOL` | — |
| `Reset` | `BOOL` | — |
| `StateComplete` | `BOOL` | — |

**Description:** The function blocks for Packaging Machine State have a common interface to the PackML Machine State Model V3 in the updated form. It is expected that application-specific logic, such as state transitions, is programmed in external function blocks and that the Pack_ML_State_Machine function block handles the central logic of the state machine and the state representation. For that reason there is a recommendation for this function block with regard to how it can be combined with other logic.

##### `PS_PackML_StateMachine_Maintenance` — §3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Start` | `BOOL` | — |
| `Hold` | `BOOL` | — |
| `unHold` | `BOOL` | — |
| `Suspend` | `BOOL` | — |
| `unSuspend` | `BOOL` | — |
| `Abort` | `BOOL` | — |
| `Stop` | `BOOL` | — |
| `Complete` | `BOOL` | — |
| `Clear` | `BOOL` | — |
| `Reset` | `BOOL` | — |
| `StateComplete` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Status` | `WORD` | — |
| `ST_Starting` | `BOOL` | — |
| `ST_Completing` | `BOOL` | — |
| `ST_Resetting` | `BOOL` | — |
| `ST_Holding` | `BOOL` | — |
| `ST_UnHolding` | `BOOL` | — |
| `ST_Suspending` | `BOOL` | — |
| `ST_UnSuspending` | `BOOL` | — |
| `ST_Clearing` | `BOOL` | — |
| `ST_Stopping` | `BOOL` | — |
| `ST_Aborting` | `BOOL` | — |
| `ST_Execute` | `BOOL` | — |
| `ST_Complete` | `BOOL` | — |
| `ST_Idle` | `BOOL` | — |
| `ST_Held` | `BOOL` | — |
| `ST_Suspended` | `BOOL` | — |
| `ST_Stopped` | `BOOL` | — |
| `ST_Aborted` | `BOOL` | — |
| `Error` | `BOOL` | — |
| `ErrorID` | `UDINT` | — |
| `ePMLState` | `E_PMLState` | — |

**Description:** The function blocks for Packaging Machine State have a common interface to the PackML Machine State Model V3 in the updated form. It is expected that application-specific logic, such as state transitions, is programmed in external function blocks and that the Pack_ML_State_Machine function block handles the central logic of the state machine and the state representation. For that reason there is a recommendation for this function block with regard to how it can be combined with other logic.

##### `PS_PackML_StateMachine_Manual` — §3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Start` | `BOOL` | — |
| `Hold` | `BOOL` | — |
| `unHold` | `BOOL` | — |
| `Suspend` | `BOOL` | — |
| `unSuspend` | `BOOL` | — |
| `Abort` | `BOOL` | — |
| `Stop` | `BOOL` | — |
| `Complete` | `BOOL` | — |
| `Clear` | `BOOL` | — |
| `Reset` | `BOOL` | — |
| `StateComplete` | `BOOL` | — |

**Description:** 16

##### `PS_PackML_StateMachine_SemiAuto` — §3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Start` | `BOOL` | — |
| `Hold` | `BOOL` | — |
| `unHold` | `BOOL` | — |
| `Suspend` | `BOOL` | — |
| `unSuspend` | `BOOL` | — |
| `Abort` | `BOOL` | — |
| `Stop` | `BOOL` | — |
| `Complete` | `BOOL` | — |
| `Clear` | `BOOL` | — |
| `Reset` | `BOOL` | — |
| `StateComplete` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `State` | `WORD` | Status word representing the state of the state machine. |
| `ST_Starting` | `BOOL` | True if state machine is in Starting state. |
| `ST_Completing` | `BOOL` | True if state machine is in Completing state. |
| `ST_Resetting` | `BOOL` | True if state machine is in Resetting state. |
| `ST_Holding` | `BOOL` | True if state machine is in Holding state. |
| `ST_UnHolding` | `BOOL` | True if state machine is in Unholding state. |
| `ST_Suspending` | `BOOL` | True if state machine is in Suspending state. |
| `ST_UnSuspending` | `BOOL` | True if state machine is in Unsuspending state. |
| `ST_Clearing` | `BOOL` | True if state machine is in Clearing state. |
| `ST_Stopping` | `BOOL` | True if state machine is in Stopping state. |
| `ST_Aborting` | `BOOL` | True if state machine is in Aborting state. |
| `ST_Execute` | `BOOL` | True if state machine is in Execute state. |
| `ST_Complete` | `BOOL` | True if state machine in Complete state. |
| `ST_Idle` | `BOOL` | True if state machine is in Idle state. |
| `ST_Held` | `BOOL` | True if state machine in Held state. |
| `ST_Suspended` | `BOOL` | True if state machine is in Suspended state. |
| `ST_Stopped` | `BOOL` | True if state machine is in Stopped state. |
| `ST_Aborted` | `BOOL` | True if state machine is in Aborted state. |
| `Error` | `BOOL` | Becomes TRUE, as soon as an error occurs. |
| `ErrorID` | `UDINT` | If the error output is set, this parameter supplies the error |

**Description:** The Packaging Machine State function blocks have a common interface to the PackML Machine State Model V3 in the updated form. It is expected that application-specific logic, such as state transitions, is programmed in external function blocks and that the Pack_ML_State_Machine function block handles the central logic of the state machine and the state representation. For that reason there is a recommendation for this function block with regard to how it can be combined with other logic.

##### `PS_UnitModeManager` — §3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `22 trigger event *)` | `` | — |
| `Machine_ID` | `MACHINE_REF` | Identification of the machine executed by the state model |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Machine_ID` | `MACHINE_REF;     (* Identifies the axis which position shall be latched at the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eModeStatus` | `E_PMLUnitMode` | Current unit mode (E_PMLUnitMode [} 10]) |
| `Done` | `BOOL` | True if mode change was successful. |
| `Error` | `BOOL` | Signals an error in the function block, e.g. "Mode change |

**Description:** Packaging machines have different unit modes and not only the "automatic" mode. Each unit mode is defined by its own state model. A "Mode Manager" must be defined for transitions between the modes. The "Mode Manager" determines how and in which state a machine can change unit modes; i.e. built-in barriers prevent the machine from changing to unsuitable states.
