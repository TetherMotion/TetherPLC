# Tc3_DriveMotionControl

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_DriveMotionControl_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/Drive · **Tier:** T3 · **API entries:** 14
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Drive motion control: axis backend + drive profiles.

---

#### Organization blocks › Axis functions

##### `MC_Power` — §5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | General software enable for the axis. Inputs/outputs |
| `Axis` | `AXIS_REF [} 39]` | Axis data structure that unambiguously addresses an axis in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Status` | `BOOL` | TRUE when the axis is ready for operation. |
| `Busy` | `BOOL` | TRUE, as long as the function block is called with |

**Description:** The function block MC_Power switches the software enabling of an axis. At Status output operational readiness of the axis is indicated.

##### `MC_Reset` — §5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge. Inputs/outputs |
| `Axis` | `AXIS_REF [} 39]` | Axis data structure that unambiguously addresses an axis in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | TRUE, if the reset was executed successfully. |
| `Busy` | `BOOL` | TRUE, as long as the function block is called with |

**Description:** The function block MC_Reset resets the NC axis. In many cases this also leads to a reset of a connected drive device. Depending on the bus system or drive types, in some cases a separate reset may be required for the drive device.

##### `MC_SetPosition` — §5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge. |
| `Position` | `LREAL` | Position value to which the axis position is to be set. In |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Organization blocks` | `` | — |
| `Done` | `BOOL` | TRUE if the position was set successfully. |
| `Busy` | `BOOL` | TRUE as soon as the command is started with "Execute" |

**Description:** The function block MC_SetPosition sets the current axis position to a parameterizable value.

#### Organization blocks › Touch probe

##### `MC_AbortTrigger` — §5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with the rising edge and the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `TriggerInput` | `TRIGGER_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | TRUE as soon as the touch probe cycle has been |

**Description:** The function block MC_AbortTrigger cancels a touch probe cycle started by MC_TouchProbe. MC_TouchProbe starts a touch probe cycle by activating a position latch in the drive hardware. The function block MC_AbortTrigger can be used to terminate the procedure before the trigger signal has activated the position latch. If the touch probe cycle has completed successfully, it is not necessary to call up this function block.

##### `MC_TouchProbe` — §5.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge, and the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `TriggerInput` | `TRIGGER_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | TRUE if an axis position was successfully detected. The |

**Description:** The function block MC_TouchProbe records an axis position at the time of a digital signal (touch probe function). The position is captured via an external hardware latch and is therefore highly accurate and independent of the cycle time. The function block controls this mechanism and determines the externally recorded position.

#### Motion function blocks

##### `MC_Home (Homing)` — §6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge. |
| `Position` | `LREAL` | Absolute reference position to which the axis is set after homing. Alternatively, the constant DEFAULT_HOME_POSITION can be used here. In this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | TRUE when the axis has been calibrated and the |

**Description:** An axis reference run is carried out with the function block MC_Home.

##### `MC_Jog (Manual Motion)` — §6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Motion function blocks` | `` | — |
| `JogForward` | `BOOL` | The command is executed with a rising edge, and the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | TRUE if a movement was successfully completed. |
| `Busy` | `BOOL` | TRUE as soon as the function block is active. FALSE if it |

**Description:** The function block MC_Jog enables an axis to be moved via manual keys. The key signal can be linked directly with the "JogForward" and "JogBackwards" inputs.

#### Motion function blocks › Point to point motion

##### `MC_Halt` — §6.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge. |
| `Deceleration` | `LREAL` | Deceleration |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Motion function blocks` | `` | — |
| `Done` | `BOOL` | TRUE if the axis has been stopped and is stationary. |
| `Busy` | `BOOL` | TRUE as soon as the command is started with "Execute" |

**Description:** The function block MC_Halt stops an axis with a defined deceleration ramp.

##### `MC_MoveAbsolute` — §6.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `26 Motion function blocks` | `` | — |
| `Execute` | `BOOL` | The command is executed with a rising edge. |
| `Position` | `LREAL` | Absolute target position to be used for positioning. |
| `Velocity` | `LREAL` | Maximum travel velocity (>0). |
| `Acceleration` | `LREAL` | Acceleration (>0) |
| `Deceleration` | `LREAL` | Deceleration (>0) |
| `Options` | `ST_MoveOptions` | Data structure (ST_MoveOptions [} 41]) containing |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | TRUE when the target position has been reached. |
| `Busy` | `BOOL` | TRUE as soon as the command is started with "Execute" |

**Description:** The function block MC_MoveAbsolute starts the positioning to an absolute target position and monitors the axis movement over the entire travel path. The "Done" output is set once the target position has been reached. Otherwise, the output "CommandAborted" or, in case of an error, the output "Error" is set.

##### `MC_MoveModulo` — §6.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL 28` | The command is executed with a rising edge. Motion function blocks |
| `Position` | `LREAL` | Modulo target position to be used for positioning. If the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | TRUE when the target position has been reached. |
| `Busy` | `BOOL` | TRUE as soon as the command is started with "Execute" |

**Description:** The function block MC_MoveModulo is used to execute a positioning operation, which refers to the modulo position of an axis. The basis for a modulo rotation is the adjustable parameter "Modulo Factor" of the AXIS_REF structure (Axis.Parameter.ModuloFactor, e.g. 360°). A distinction is made between three possible start types, depending on the "Direction" input. • Positioning in positive direction • Positioning in negative direction • Positioning along shortest path

##### `MC_MoveRelative` — §6.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge. |
| `Distance` | `LREAL` | Relative distance to be used for positioning. |
| `Velocity` | `LREAL` | Maximum travel velocity (>0). |
| `Acceleration` | `LREAL` | Acceleration (≥0) |
| `Deceleration` | `LREAL` | Deceleration (≥0) |
| `Options` | `ST_MoveOptions` | Data structure (ST_MoveOptions [} 41]) containing |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `34 Motion function blocks` | `` | — |
| `Done` | `BOOL` | TRUE when the target position has been reached. |
| `Busy` | `BOOL` | TRUE as soon as the command is started with "Execute" |

**Description:** The function block MC_MoveRelative starts a relative positioning procedure based on the current set position and monitors the axis movement over the whole travel path. The "Done" output is set once the target position has been reached. Otherwise, the output "CommandAborted" or, in case of an error, the output "Error" is set.

##### `MC_MoveVelocity` — §6.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge. Motion function blocks |
| `ContinuousUpdat BOOL` | `If ContinuousUpdate is TRUE, the dynamics can be` | — |
| `e` | `changed according to the Velocity, Acceleration and` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InVelocity` | `BOOL` | After the axis acceleration, the InVelocity output assumes |

**Description:** The function block MC_MoveVelocity starts an endless travel with a specified velocity and direction. The movement can be stopped through a Stop command.

##### `MC_Stop` — §6.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge. The axis is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | TRUE if the axis has been stopped and is stationary. |
| `Busy` | `BOOL` | TRUE as soon as the command is started with Execute |

**Description:** The function block MC_Stop stops an axis with a defined deceleration ramp and locks the axis against other motion commands. The function block is therefore suitable for stops in special situations, in which further axis movements are to be prevented.

#### Data types › Axis interface

##### `AXIS_REF` — §7.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `PlcToDrive` | `PLCTODRIVE_AXIS_REF` | — |
| `Parameter` | `ST_AxisParameters` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DriveToPlc` | `DRIVETOPLC_AXIS_REF` | — |
| `WcState` | `DRIVETOPLC_WCSTATE` | — |
| `InfoData` | `DRIVETOPLC_IFODATA` | — |
| `Status` | `ST_AxisStatus` | — |

**Description:** The AXIS_REF data type contains axis information. AXIS_REF is an interface between the PLC and the DRIVE. It is added to MC function blocks as axis reference. TYPE AXIS_REF :
