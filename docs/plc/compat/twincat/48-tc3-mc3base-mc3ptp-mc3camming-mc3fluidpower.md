# Tc3_Mc3Base+Mc3Ptp+Mc3Camming+Mc3FluidPower

- **Manual:** `TF55xx_TC3_MC3_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/MC3 · **Tier:** T3 · **API entries:** 89
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** MC3 motion function blocks (PTP, camming, fluid power): axis backend.

**Status:** PARTIAL — the single-axis PTP core lives in
`compat/tc2/tc2_mc2.st` (power/reset/position/moves/jog/home/
parameters/touch-probe/gear-in). `compat/tc3/tc3_mc3ptp.st` adds the
remaining tractable PTP FBs (ctest `tc3_mc3ptp` + `tc3_mc3ptp_io`,
mask `0x1ff`): `MC_ReadActualTorque` (0x6077),
`MC_ReadHardwareParameter`/`MC_WriteHardwareParameter` (OD index
space), `MC_WaitForStableActualPosition` (velocity-window settle),
`MC_TriggerError` (fault inject via sim-ext object 0x200D latching
the CiA402 fault state; 0x603F reports the code, MC_Reset clears)
and `MC_SetHomingState`/`MC_LoadControl`/
`MC_ActivateExternalSetpointGeneration` as accepted no-ops. Phasing
subset done in `compat/tc3/tc3_mc3cam.st` (e2e `tc3_mc3cam{,_io}`,
mask `0x3f`): `MC_PhasingAbsolute`, `MC_PhasingRelative`,
`MC_GearInPos` and `MC_HaltPhasing` as finite position-convergence
commands over a PV-mode feedforward+proportional loop; Done/InSync
fire once the slave position error stays inside the window, and the
command ends there (persistent cam coupling needs the NC kernel).
Velocity/Acceleration/Deceleration/Jerk inputs are accepted but not
enforced; the simulated drive applies its own dynamics limits.
Cam-table set done in `compat/tc3/tc3_mc3cam.st` (e2e
`tc3_mc3camtbl{,_io}`, mask `0xff`): `FB_CamTable` is the flattened
`MC_CamObject` (UDINT handle into the new `tplc-cam` service holding
the point table, periodic/continuity flags and the per-slave binding
registry with coupling scalings); `MC_CamTableSelect`,
`MC_CamIn`, `MC_CamAdd`, `MC_CamExchange`, `MC_CamRemove`,
`MC_CamScaling` and `MC_ReadCamInfo` bind/unbind/query the slave and
stream interpolated 0x607A position targets while the binding lives.
Deviations: one binding per slave (multi-plate summation and
superposition need the NC kernel), `MC_CamObject` METHODs flatten
to `FB_CamTable` edges, `MC_CAM_REF`/`Type_CouplingFunction1D`/
`CamScalingData`/`CamActivationOptions` flatten to UDINT handles and
LREAL scalars, continuity is stored not enforced, ActivationOptions
and BufferMode are accepted but not modeled. Deferred: fluid power
(`MC_ValveCharacterization`) and the TcCOM `AXIS_REF`/
`ENCODER_AXIS_REF` object model (METHOD-based, not expressible in
the ST dialect).

---

#### Development environment › MOTION › Default Axis › Drive modules › Pulse Width Current Drive (MDP250) (both channels)

##### `Data Area` — §3.2.3.8.8.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Channel 2 Enable` | `BIT` | Dithering for Channel 2 is "enabled”. dithering |
| `Channel 2 Enable` | `BIT` | Enable dithering for channel 2. |
| `Channel 2 Reset` | `BIT` | Resetting an error on Channel 2. |
| `Channel 2 PWM` | `INT` | Output from the PWM terminal on channel 2. NOTICE |

**Description:** In general, TcCOM objects can contain data areas that can be used by the environment (e.g., by a PLC instance or by the TwinCAT I/O area). For TwinCAT MC3 TcCOM objects, you have a choice of data structures to use via the Data Area tab.

#### Development environment › MOTION › Default Axis › Touch probe modules › Binary Touch Probe

##### `Data Area` — §3.2.3.12.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable (de- Name` | `Type` | Description fault) |
| `Signal` | `BIT` | Input for the trigger signal. For example, it can be linked to a variable from the PLC |

**Description:** In general, TcCOM objects can contain data areas that can be used by the environment (e.g., by a PLC instance or by the TwinCAT I/O area). For TwinCAT MC3 TcCOM objects, you have a choice of data structures to use via the Data Area tab.

#### Development environment › MOTION › Default Axis › Touch probe modules › Timestamp Touch Probe

##### `Data Area` — §3.2.3.12.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable (de- Name` | `Type` | Description fault) |
| `Timestamp 32bit` | `UDINT` | A timestamp whose value changes are |

**Description:** In general, TcCOM objects can contain data areas that can be used by the environment (e.g., by a PLC instance or by the TwinCAT I/O area). For TwinCAT MC3 TcCOM objects, you have a choice of data structures to use via the Data Area tab.

#### Development environment › MOTION › Default Axis › Homing modules › Homing Cam

##### `Data Area` — §3.2.3.13.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable (de- Name` | `Type` | Description fault) |
| `Signal` | `BIT` | Input for the trigger signal. For example, it can be linked to a variable from the PLC |

**Description:** The data areas originate from the nested Touch Probe object [} 107].

#### Global data types and MC3 types › Interfaces › ITcMcExternalSetpointGenerator

##### `OnPreparation` (METHOD) → `HRESULT` — §4.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `InvokeId` | `XWORD` | Unique identifier, the interpretation is user specific. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT` | `` | — |

**Description:** Called once when the external setpoint generator is being prepared for activation.

##### `OnStartPointInitialization` (METHOD) → `HRESULT` — §4.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `InvokeId` | `XWORD` | Unique identifier, the interpretation is user specific. |
| `StartDcTime` | `LINT` | DC time at which the generation starts. |
| `Position` | `LREAL` | Start position of the axis. |
| `Velocity` | `LREAL` | Start velocity of the axis. |
| `Acceleration` | `LREAL` | Start acceleration of the axis. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT` | `` | — |

**Description:** Called once to provide the initial motion state before cyclic setpoint generation begins.

##### `OnCyclicSetpointGeneration` (METHOD) → `HRESULT` — §4.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `InvokeId` | `XWORD` | Unique identifier, the interpretation is user specific. |
| `TimeInGenerati LREAL` | `Elapsed time since the start of setpoint generation (input from` | — |
| `on` | `PLC side of view).` | — |
| `Position` | `Reference To LREAL` | Reference to receive the computed position setpoint. Value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT` | `` | — |

**Description:** Called every cycle to compute the next setpoint values for position, velocity, and acceleration.

##### `OnDeactivation` (METHOD) → `HRESULT` — §4.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `InvokeId` | `XWORD` | Unique identifier, the interpretation is user specific. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT` | `` | — |

**Description:** Called when the external setpoint generator is deactivated/aborted. Use this method to tidy up, e. g. release created objects.

#### PLC libraries › Tc3_Mc3Base › Interfaces › Types › Type_Camming1D

##### `GetCamming1DOid` (METHOD) → `OTCID` — §5.2.2.1.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Base › Interfaces › Types › Type_ErrorTriggerable

##### `GetErrorTriggerableOid` (METHOD) → `OTCID` — §5.2.2.1.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Base › Interfaces › Types › Type_Master1D

##### `GetMaster1DOid` (METHOD) → `OTCID` — §5.2.2.1.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Base › Interfaces › Types › Type_MotionObject

##### `GetMotionObjectOid` (METHOD) → `OTCID` — §5.2.2.1.4.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Base › Interfaces › Types › Type_Phasing1D

##### `GetPhasing1DOid` (METHOD) → `OTCID` — §5.2.2.1.5.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Base › Function Blocks › TestCommands

##### `MC_TriggerError` (FUNCTION_BLOCK) — §5.2.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MotionObject` | `Type_ErrorTriggerable` | Interface pointer to the object on which the error should be [} 230] triggered (e.g. Axis). |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `ErrorCode` | `UDINT` | Error code to be triggered |
| `ControlError` | `BOOL` | Trigger a ControlError (MC has no control over the axis, i.e. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Error has been successfully triggered. |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block triggers an error in the target motion object.

#### PLC libraries › Tc3_Mc3Ptp › Interfaces › Types › Type_PtpErrorResettable

##### `GetPtpErrorResettableOid` (METHOD) → `OTCID` — §5.3.4.1.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Ptp › Interfaces › Types › Type_PtpReadWriteParameter

##### `GetPtpReadWriteParameterOid` (METHOD) → `OTCID` — §5.3.4.1.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Ptp › Interfaces › Types › Type_PtpSetHomingState

##### `GetPtpSetHomingStateOid` (METHOD) → `OTCID` — §5.3.4.1.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Ptp › Interfaces › Types › Type_PtpSetPosition

##### `GetPtpSetPositionOid` (METHOD) → `OTCID` — §5.3.4.1.4.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Ptp › Interfaces › Types › Type_PtpTouchProbe

##### `GetPtpTouchProbeOid` (METHOD) → `OTCID` — §5.3.4.1.5.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › Administrative

##### `MC_Power` (FUNCTION_BLOCK) — §5.3.5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | General software enable for the axis |
| `EnablePositive BOOL` | `false` | Feed enable in positive direction. Only effective if Enable = TRUE |
| `EnableNegativ BOOL` | `false` | Feed enable in negative direction. Only |
| `e` | `effective if Enable = TRUE` | — |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Status` | `BOOL` | Axis is ready for motion commands. |
| `Error` | `BOOL` | Error occurred within function block or axis already signaling an |

**Description:** This function block controls the power stage (on or off).

##### `MC_Reset` (FUNCTION_BLOCK) — §5.3.5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Type_PtpErrorResettable` | Interface pointer to the object on which the command should be |
| `[} 249]` | `performed (e.g. Axis, Linear Mover).` | — |
| `Execute` | `BOOL` | Trigger the command with rising edge. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Potential axis and/or drive error has been reset. State Standstill, |

**Description:** This function block resets all motion object-related errors and transitions the motion object out of state ErrorStop. The resulting state is Disabled, if no Enable is requested for the motion object (e.g. via MC_Power). It results in DiscreteMotion, if the motion object has a closed control loop and a controlled error reaction is still ongoing. Otherwise, Standstill is the resulting state. This function block does not affect the output of motion function block instances.

##### `MC_SetOverride` (FUNCTION_BLOCK) — §5.3.5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Write the velocity override factor |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Enabled` | `BOOL` | The commanded override is set. |
| `Error` | `BOOL` | Error occurred within function block. |
| `ErrorId` | `UDINT` | Error identifier |

**Description:** This function block sets the velocity override factor that is applied to movement commands of the axis. The override factor does not apply to movement in the synchronized motion state (e.g. movement induced by MC_GearIn, MC_GearInPos, ...).

##### `MC_SetPosition` (FUNCTION_BLOCK) — §5.3.5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Type_PtpSetPosition [} 251]` | Reference to the axis |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Position` | `LREAL` | #Invalid New position value ([PosUnit]) or distance if Relative = TRUE |
| `Relative` | `BOOL` | false Shift axis position by value |
| `ClearPositionL BOOL` | `Resets position lag to zero. ag` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Position has new value. |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block shifts the coordinate system of an axis by manipulating both the setpoint position as well as the actual position of an axis with the same value without any movement caused (Re-calibration with same following error). This can be used for instance for a reference situation.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › Administrative/Parameter

##### `MC_ReadBoolParameter` (FUNCTION_BLOCK) — §5.3.5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference to the axis Type_PtpReadWriteParam eter [} 250]` | — |
| `Enable` | `BOOL` | Triggers a read on rising edge (EReadMode.Once) or every cycle while |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | Valid output available at function block. |
| `Busy` | `BOOL` | Function block is not finished. Always FALSE with ReadMode |

**Description:** This function block reads a boolean parameter value from the axis. The command is typically processed synchronously.

##### `MC_ReadHardwareParameter` (FUNCTION_BLOCK) — §5.3.5.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Index` | `UINT` | 0 CoE object index or SoE IDN |
| `SubIndex` | `USINT` | 0 CoE sub index or SoE element |
| `ress` | `buffer.` | — |
| `DataSize` | `UDINT` | 0 Contains the size of the destination buffer. In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Value is successfully read from the target. |
| `Busy` | `BOOL` | Function block is not finished. |
| `Active` | `BOOL` | Function block is active. |
| `Error` | `BOOL` | Error occurred within function block. |
| `ErrorId` | `UDINT` | Error identifier Possible error codes |

**Description:** This function block allows to read parameters from the hardware drive asynchronously via ADS. The function block supports CoE and SoE drives.

##### `MC_ReadParameter` (FUNCTION_BLOCK) — §5.3.5.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference to the axis Type_PtpReadWriteParam eter [} 250]` | — |
| `Enable` | `BOOL` | Triggers a read on rising edge (EReadMode.Once) or every cycle while |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | Valid output available at function block. |
| `Busy` | `BOOL` | Function block is not finished. Always FALSE with ReadMode |

**Description:** This function block reads a numeric (LREAL) parameter value from the axis. The command is typically processed synchronously.

##### `MC_WriteBoolParameter` (FUNCTION_BLOCK) — §5.3.5.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference to the axis Type_PtpReadWriteParam eter [} 250]` | — |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `ParameterId` | `EAxisParameterId [} 239]` | Invalid Identifier of the parameter to write. |
| `Value` | `BOOL` | Value of the parameter to write. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Parameter value has been successfully written. |
| `Busy` | `BOOL` | Function block is not finished. |
| `Error` | `BOOL` | Error occurred within function block. |
| `ErrorId` | `UDINT` | Error identifier |

**Description:** This function block writes a boolean parameter value to the axis. The command is typically processed synchronously.

##### `MC_WriteHardwareParameter` (FUNCTION_BLOCK) — §5.3.5.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers the write operation. |
| `Index` | `UINT` | 0 CoE object index or SoE IDN |
| `SubIndex` | `USINT` | 0 CoE sub index or SoE element |
| `DataSize` | `UDINT` | 0 Contains the size of the data. In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Value is successfully sent to the target. |
| `Busy` | `BOOL` | Function block is not finished. |
| `Active` | `BOOL` | Function block is active. |
| `Error` | `BOOL` | Error occurred within function block. |
| `ErrorId` | `UDINT` | Error identifier Possible error codes |

**Description:** This function block allows to write parameters to the hardware drive asynchronously via ADS. It supports CoE and SoE drives.

##### `MC_WriteParameter` (FUNCTION_BLOCK) — §5.3.5.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference to the axis Type_PtpReadWriteParam eter [} 250]` | — |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `ParameterId` | `EAxisParameterId [} 239]` | Invalid Identifier of the parameter to write. |
| `Value` | `LREAL` | Value of the parameter to write. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Parameter value has been successfully written. |
| `Busy` | `BOOL` | Function block is not finished. |
| `Error` | `BOOL` | Error occurred within function block. |
| `ErrorId` | `UDINT` | Error identifier Possible error codes |

**Description:** This function block writes a numeric (LREAL) parameter value to the axis. The command is typically processed synchronously.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › Administrative/Status

##### `MC_ReadActualPosition` (FUNCTION_BLOCK) — §5.3.5.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Read the actual position continuously while Enable = TRUE. In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | Valid output available at function block |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block reads the current axis actual position. In case of an invalid actual position value, the Position output is set to 0 and the Valid output is set to FALSE.

##### `MC_ReadActualTorque` (FUNCTION_BLOCK) — §5.3.5.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Read the actual torque continuously while Enable = TRUE. In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | Valid output available at function block |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block reads the current axis actual torque. In case of an invalid actual torque value, the Torque output is set to 0 and the Valid output is set to FALSE.

##### `MC_ReadActualVelocity` (FUNCTION_BLOCK) — §5.3.5.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Read the actual velocity continuously while Enable = TRUE. In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | Valid output available at function block |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block reads the current axis actual velocity. In case of an invalid actual velocity value, the Velocity output is set to 0 and the Valid output is set to FALSE.

##### `MC_ReadAxisError` (FUNCTION_BLOCK) — §5.3.5.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Read the axis error identifier continuously while Enable = TRUE. In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | Valid output available at function block |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block reads the current axis error identifier.

##### `MC_ReadStatus` (FUNCTION_BLOCK) — §5.3.5.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Read the axis state continuously while Enable = TRUE. In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | Valid output available at function block |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block reads the current axis state.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › Continuous Motion

##### `MC_Jog` (FUNCTION_BLOCK) — §5.3.5.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `JogForward` | `BOOL` | The command is executed with a rising edge, and the axis is moved in positive direction. Depending on the mode, the axis moves as long as the signal remains TRUE, or it stops automatically after a |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Function block has succeeded. |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block enables an axis to be moved via manual keys in different modi.

##### `MC_MoveVelocity` (FUNCTION_BLOCK) — §5.3.5.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Velocity` | `LREAL` | #Invalid Commanded velocity ([PosUnit]/s, the sign of this parameter determines the direction |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InVelocity` | `BOOL` | Commanded velocity reached |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block commands an endless movement with a specified velocity.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › Discrete Motion

##### `MC_Halt` (FUNCTION_BLOCK) — §5.3.5.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Deceleration` | `LREAL` | #Default Maximum deceleration ([PosUnit]/s^2, positive) |
| `Jerk` | `LREAL` | #Default Maximum jerk ([PosUnit]/s^3, positive) In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Zero dynamics has been reached, axis state is transferred to |

**Description:** This function block commands a motion stop. The axis is transferred to the state DiscreteMotion, until zero dynamics is reached.

##### `MC_MoveAbsolute` (FUNCTION_BLOCK) — §5.3.5.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Position` | `LREAL` | #Invalid Commanded position ([PosUnit]) |
| `Velocity` | `LREAL` | #Invalid Maximum velocity ([PosUnit]/s, positive) |
| `Acceleration` | `LREAL` | #Default Maximum acceleration ([PosUnit]/s^2, positive) |
| `Deceleration` | `LREAL` | #Default Maximum deceleration ([PosUnit]/s^2, positive) |
| `Jerk` | `LREAL` | #Default Maximum jerk ([PosUnit]/s^3, positive) |
| `BufferMode` | `EBufferMode [} 225]` | Aborting Chronological sequence of the command In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Commanded position finally reached |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block commands a motion to a specified absolute position.

##### `MC_MoveModulo` (FUNCTION_BLOCK) — §5.3.5.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Position` | `LREAL` | #Invalid Commanded modulo position ([PosUnit]) |
| `AdditionalTurn` | `UDINT` | 0 Number of additional modulo turns in the |
| `s` | `commanded direction. A value > 0 requires Direction <>` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Commanded position finally reached |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block commands a motion to a specified modulo position.

##### `MC_MoveRelative` (FUNCTION_BLOCK) — §5.3.5.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Distance` | `LREAL` | #Invalid Relative distance ([PosUnit]) |
| `Velocity` | `LREAL` | #Invalid Maximum velocity ([PosUnit]/s, positive) |
| `Acceleration` | `LREAL` | #Default Maximum acceleration ([PosUnit]/s^2, positive) |
| `Deceleration` | `LREAL` | #Default Maximum deceleration ([PosUnit]/s^2, positive) |
| `Jerk` | `LREAL` | #Default Maximum jerk ([PosUnit]/s^3, positive) |
| `BufferMode` | `EBufferMode [} 225]` | Aborting Chronological sequence of the command In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Commanded distance reached |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block commands a motion of a specified distance relative to the set position at the time the command gets active.

##### `MC_Stop` (FUNCTION_BLOCK) — §5.3.5.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Trigger the command with rising edge. While this is TRUE, other function blocks |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Zero dynamics has been reached. |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block commands a motion stop and transfers the axis to the state Stopping. It aborts any ongoing function block execution. While the axis is in state Stopping, no other function block can perform any motion on the same axis. The Done output is set immediately after zero dynamics is reached. The axis remains in the state Stopping as long as Execute is TRUE or zero dynamics is not yet reached. As soon as Done is SET and Execute is FALSE the axis goes to state Standstill.

##### `MC_WaitForStableActualPosition` (FUNCTION_BLOCK) — §5.3.5.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `PositionTolera LREAL` | `#Invalid` | Position tolerance around the set position |
| `nce` | `([PosUnit], positive). The actual position must remain within this window for the` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InPositionWind BOOL` | `Actual position is currently inside the position window. ow` | — |
| `Done` | `BOOL` | Actual position has been consecutively inside the position |

**Description:** This function block waits for a stable actual position. The command is always buffered and may only be triggered when the previous command ends with zero dynamics.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › External Setpoint Generator

##### `MC_ActivateExternalSetpointGeneration` (FUNCTION_BLOCK) — §5.3.5.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `ExternalSetpoi ITcMcExternalSetpointGe` | `Function block that implements (cyclic)` | — |
| `ntGenerator` | `nerator` | methods for external setpoint generation |
| `AllowOtherCon BOOL` | `false` | Allow the calling task to differ from the MC |
| `text` | `task (MET task). When TRUE, ensure compatible update rates and safe data access in the` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block activates external setpoint generation. Setpoints are generated by a function block, which is called through the ITcMcExternalSetpointGenerator interface from the MC context. Additionally custom startup data can be passed through the generator.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › External Setpoint Generator › MC_ExternalSetpointGenerator

##### `FB_reinit` (METHOD) → `BOOL` — §5.3.5.6.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** Internal method

##### `OnPreparation` (METHOD) → `HRESULT` — §5.3.5.6.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `InvokeId` | `XWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT` | `` | — |

**Description:** Called once when the external setpoint generator is being prepared for activation.

##### `OnStartPointInitialization` (METHOD) → `HRESULT` — §5.3.5.6.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `InvokeId` | `XWORD` | — |
| `StartDcTime` | `LINT` | — |
| `Position` | `LREAL` | — |
| `Velocity` | `LREAL` | — |
| `Acceleration` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT` | `` | — |

**Description:** Called to initialize the starting point before cyclic setpoint generation begins.

##### `OnCyclicSetpointGeneration` (METHOD) → `HRESULT` — §5.3.5.6.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `InvokeId` | `XWORD TimeInGenerati LREAL on` | — |
| `Position` | `Reference To LREAL` | — |
| `Velocity` | `Reference To LREAL` | — |
| `Acceleration` | `Reference To LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT` | `` | — |

**Description:** Called every MC cycle to compute the next setpoint values. Must be overridden.

##### `OnDeactivation` (METHOD) → `HRESULT` — §5.3.5.6.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `InvokeId` | `XWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HRESULT` | `` | — |

**Description:** Called when the external setpoint generator is deactivated/aborted.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › Homing

##### `MC_Home` (FUNCTION_BLOCK) — §5.3.5.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Position` | `LREAL` | #Default Absolute position at the reference signal ([PosUnit]) In/Outputs |
| `Axis` | `Reference To AXIS_REF [} 307]` | Reference to the axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | 'Search home' sequence succeeded |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block commands the axis to perform the homing sequence. The homing sequence is added via the homing TCOM object. The calculated offset is stored in 'Active Homing Offset'. Offsets from MC_SetPosition are discarded.

##### `MC_SetHomingState` (FUNCTION_BLOCK) — §5.3.5.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Type_PtpSetHomingState [} 251]` | Reference to the axis |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Position` | `LREAL` | #Invalid New position value ([PosUnit]) |
| `Mode` | `EHomingMode [} 242]` | ForceIsHomed Homing mode selection |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Homing succeeded |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block allows setting and resetting the IsPositionValid flag in the CDT_MCTOPLC_AXIS_STD. Optionally, it can also set the Position.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › Load Control

##### `MC_LoadControl` (FUNCTION_BLOCK) — §5.3.5.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Start the motion at rising edge. |
| `Load` | `LREAL` | #Invalid Commanded load value ([LoadUnit]). It is approached using a defined ramp |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InLoad` | `BOOL` | Commanded load value reached |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block continuously exerts a torque, force or pressure of the specified value. Positive torque, force and differential pressure is in the positive direction of velocity, pressure is physically unsigned. LoadUnit must match the unit of the value measured by the corresponding sensor.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › Motion Objects

##### `AXIS_REF` (FUNCTION_BLOCK) — §5.3.5.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `PlcToMc` | `AXIS_REF_TOMC [} 244]` | Data area for cyclic PLC-to-MC data exchange (mapped to MC |

**Description:** An MC3 axis used with PLCopen motion control function blocks.

##### `ENCODER_AXIS_REF` (FUNCTION_BLOCK) — §5.3.5.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `McToPlc` | `Data area for cyclic MC-to-PLC data exchange (mapped to MC` | — |

**Description:** An MC3 encoder axis used with PLCopen motion control function blocks.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › Synchronized Motion

##### `MC_GearIn` (FUNCTION_BLOCK) — §5.3.5.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `Type_Master1D [} 230]` | Reference to the master axis |
| `Slave` | `Reference To AXIS_REF [} 307]` | Reference to the slave axis |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `r` | `synchronous, the slave's velocity, acceleration and position changes are (RatioNumerator / RatioDenominator) times the master's values. A negative value moves the slave opposite to the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InGear` | `BOOL` | TRUE once the slave has reached the master's velocity and |

**Description:** Synchronizes the slave onto the master's velocity and acceleration scaled by a constant gear ratio. Synchronizes as fast as the function block's inputs Acceleration, Deceleration and Jerk allow. MC_GearIn has no input Velocity. The axis' parameter Maximum Velocity is used while synchronizing. Once the slave has reached the scaled velocity and acceleration, the function block's output InGear signals true and the slave is coupled to the scaled master. From then on the slave's dynamics and position are coupled to the scaled master's, ignoring any dynamic restrictions (commanded dynamics as well as axis parameters). The position relationship when reaching InGear is consequently not configurable. Use MC_GearInPos when a specific position relationship is required. The command may be ended by sending another command. Using input BufferMode := Aborting makes the next command take over immediately while input BufferMode := Buffered queues the next command. It will then take over as soon as InGear is reached.

##### `MC_GearInPos` (FUNCTION_BLOCK) — §5.3.5.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `Type_Master1D [} 230]` | Reference to the master axis |
| `Slave` | `Reference To AXIS_REF [} 307]` | Reference to the slave axis |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `r` | `synchronous the slave's velocity, acceleration and position changes are (RatioNumerator / RatioDenominator) times the master's. A negative value` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InSync` | `BOOL` | TRUE once the output SyncState reaches value InSync: the |

**Description:** Synchronizes the slave onto the master's velocity and acceleration scaled by a constant gear ratio, anchoring at slave position = SlaveSyncPosition + (RatioNumerator / RatioDenominator) * (master position - MasterSyncPosition). Synchronizes as fast as the function block's inputs Velocity, Acceleration, Deceleration and Jerk allow. Once the slave has reached the scaled velocity and acceleration as well as the position described above, the slave is coupled to the master. From then on any dynamic restrictions (commanded dynamics as well as axis parameters) are ignored. If it is not necessary to configure the position relationship in advance, use MC_GearIn instead. Synchronicity is reached as early as possible and is thus independent of whether commanded synchronous positions have been reached by either master or slave. The outputs InSync and SyncState provide detailed information on the progress of synchronization. The direction in which the slave may move while synchronizing can be restricted via the input AllowedSlaveSyncDirections. The command may be ended by sending another command. Using input BufferMode := Aborting for the next command makes it take over immediately while input BufferMode := Buffered for the next command queues it. It will then take over as soon as output SyncState reaches InSync.

##### `MC_HaltPhasing` (FUNCTION_BLOCK) — §5.3.5.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `Type_Master1D [} 230]` | Reference to the master axis |
| `Slave` | `Type_Phasing1D [} 231]` | Reference to the slave axis |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Deceleration` | `LREAL` | #Invalid Maximum deceleration ([PosUnit]/s^2, positive) |
| `Jerk` | `LREAL` | #Invalid Maximum jerk ([PosUnit]/s^3, positive) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Zero dynamics has been reached, axis state is transferred to |

**Description:** This function block commands a phase shift stop in the master position of a synchronized slave axis. It is only valid for camming synchronization. The master axis is not aware of the phase shift.

##### `MC_PhasingAbsolute` (FUNCTION_BLOCK) — §5.3.5.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `Type_Master1D [} 230]` | Reference to the master axis |
| `Slave` | `Type_Phasing1D [} 231]` | Reference to the slave axis |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `PhaseShift` | `LREAL` | #Invalid Absolute phase difference in master position of the slave axis ([PosUnit]) |
| `Velocity` | `LREAL` | #Invalid Maximum velocity ([PosUnit]/s, positive) |
| `Acceleration` | `LREAL` | #Invalid Maximum acceleration ([PosUnit]/s^2, positive) |
| `Deceleration` | `LREAL` | #Invalid Maximum deceleration ([PosUnit]/s^2, positive) |
| `Jerk` | `LREAL` | #Invalid Maximum jerk ([PosUnit]/s^3, positive) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Commanded phasing reached |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block commands an absolute phase shift in the master position of a synchronized slave axis. It is only valid for camming synchronization. The master axis is not aware of the phase shift.

##### `MC_PhasingRelative` (FUNCTION_BLOCK) — §5.3.5.10.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `Type_Master1D [} 230]` | Reference to the master axis |
| `Slave` | `Type_Phasing1D [} 231]` | Reference to the slave axis |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `PhaseShift` | `LREAL` | #Invalid Additional phase difference in master position of the slave axis ([PosUnit]) |
| `Velocity` | `LREAL` | #Invalid Maximum velocity ([PosUnit]/s, positive) |
| `Acceleration` | `LREAL` | #Invalid Maximum acceleration ([PosUnit]/s^2, positive) |
| `Deceleration` | `LREAL` | #Invalid Maximum deceleration ([PosUnit]/s^2, positive) |
| `Jerk` | `LREAL` | #Invalid Maximum jerk ([PosUnit]/s^3, positive) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Commanded phasing reached |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block commands a relative phase shift in the master position of a synchronized slave axis. It is only valid for camming synchronization. The master axis is not aware of the phase shift.

#### PLC libraries › Tc3_Mc3Ptp › Function Blocks › TouchProbe

##### `MC_AbortTrigger` (FUNCTION_BLOCK) — §5.3.5.11.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Type_PtpTouchProbe [} 252]` | Reference to the axis |
| `Execute` | `BOOL` | Aborts the monitoring of trigger event at rising edge. In/Outputs |
| `TriggerInput` | `Reference To` | Reference to the trigger signal source like DriveTouchProbeRef TRIGGER_REF [} 249] or BinaryTouchProbeRef |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TriggerInput` | `Reference To TRIGGER_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Abort Trigger succeeded |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This Function Block is used to abort function blocks which are used to record trigger events (e.g. MC_TouchProbe).

##### `MC_TouchProbe` (FUNCTION_BLOCK) — §5.3.5.11.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Type_PtpTouchProbe [} 252]` | Reference to the axis |
| `Execute` | `BOOL` | Trigger the command with rising edge. |
| `Options` | `TouchProbeOptions [} 248]` | Options for the touch probe In/Outputs |
| `TriggerInput` | `Reference To` | Reference to the trigger signal source like DriveTouchProbeRef |
| `TRIGGER_REF [} 249]` | `or BinaryTouchProbeRef` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TriggerInput` | `Reference To TRIGGER_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Touch probe process succeeded |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block controls the recording of an axis position at a trigger event. One of WindowFirstPosition or WindowLastPosition can be left as 'Ignore' if no corresponding limit is desired.

#### PLC libraries › Tc3_Mc3Camming › Interfaces › Types › Type_CouplingFunction1D

##### `GetCouplingFunc1DOid` (METHOD) → `OTCID` — §5.4.2.1.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID` | `` | — |

**Description:** Syntax

#### PLC libraries › Tc3_Mc3Camming › Function Blocks › Administrative

##### `MC_CamScaling` (FUNCTION_BLOCK) — §5.4.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Slave` | `Type_Camming1D [} 229] Reference to the slave axis` | — |
| `CamObject` | `Type_CouplingFunction1D Reference to the cam object [} 348]` | — |
| `Execute` | `BOOL` | Apply cam scaling at rising edge |
| `Scaling` | `CamScalingData [} 346]` | New cam scaling |
| `ActivationOptio CamActivationOptions` | `Options controlling how the cam coupling is activated (mode,` | — |
| `ns` | `[} 340]` | position, reference object). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Cam scaling succeeded |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block applies the scaling factor of an existing cam table coupling on a slave. InSync of the existing cam table coupling is not affected by this function block.

##### `MC_CamTableSelect` (FUNCTION_BLOCK) — §5.4.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CamRef` | `MC_CAM_REF [} 346]` | Data point information. |
| `Execute` | `BOOL` | Start cam select at rising edge. |
| `Periodic` | `BOOL` | Set to TRUE for a periodic cam table. |
| `ContinuityRequ EContinuity [} 200]` | `Minimum continuity requirement. The` | — |
| `irement` | `higher the requirement, the stricter the` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CamObject` | `Reference To MC_CamObject` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Selection succeeded |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

##### `MC_ReadCamInfo` (FUNCTION_BLOCK) — §5.4.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Slave` | `Type_Camming1D [} 229] Reference to the slave axis` | — |
| `Execute` | `BOOL` | Read cam coupling information at rising edge |
| `CouplingInfo Pointer To` | `Pointer to array of CamCouplingInfo structures to fill CamCouplingInfo [} 344]` | — |
| `NumberOfCou UDINT` | `Capacity of the CouplingInfo array. plings` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | TRUE when the cam coupling information has been read successfully |
| `Busy` | `BOOL` | TRUE while the function block is executing |
| `Error` | `BOOL` | TRUE if an error occurred |
| `ErrorId` | `UDINT` | Error code |
| `NumberOfCou` | `UDINT` | Number of cam table couplings found on the slave axis (at most |
| `plingsFound` | `8).` | — |

**Description:** This function block reads information about active and buffered cam table couplings on a slave axis.

#### PLC libraries › Tc3_Mc3Camming › Function Blocks › Motion

##### `MC_CamAdd` (FUNCTION_BLOCK) — §5.4.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `Type_Master1D [} 230]` | Reference to the master axis |
| `Slave` | `Type_Camming1D [} 229]` | Reference to the slave axis |
| `CamObject` | `Type_CouplingFunction1D [} 348]` | Reference to the cam object |
| `Execute` | `BOOL` | Start cam add at rising edge. |
| `Scaling` | `CamScalingData [} 346]` | Scaling of the new coupling |
| `ActivationOptio CamActivationOptions` | `Options controlling how the cam coupling` | — |
| `ns` | `[} 340]` | is activated (mode, position, reference |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InSync` | `BOOL` | Synchronization succeeded |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block adds a cam table coupling on a slave. Generated setpoints from single cam table couplings are internally added. As MC_CamIn, this function block can also be used to initiate camming-based synchronization. As this function block is used for multi-camming, some special rules apply: The same master can be used in multiple cam table couplings. A cam table can only be used once in a slave.

##### `MC_CamExchange` (FUNCTION_BLOCK) — §5.4.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `Type_Master1D [} 230]` | Reference to the master axis |
| `Slave` | `Type_Camming1D [} 229]` | Reference to the slave axis |
| `CamObjectToR Type_CouplingFunction1D` | `Cam object to be removed` | — |
| `emove` | `[} 348]` | — |
| `CamObjectToA Type_CouplingFunction1D` | `Cam object to be added` | — |
| `dd` | `[} 348]` | — |
| `Execute` | `BOOL` | Start cam exchange at rising edge. |
| `Scaling` | `CamScalingData [} 346]` | Scaling of the new coupling |
| `ActivationOptio CamActivationOptions` | `Options controlling how the cam coupling` | — |
| `ns` | `[} 340]` | is activated (mode, position, reference |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InSync` | `BOOL` | Synchronization succeeded |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block exchanges cam table couplings on a slave. The cam table coupling that should be removed has to be active when this function block is executed and the activation condition is met, otherwise an error is triggered. Activation might depend either on the new entered cam table coupling (ActivationOptions.ReferenceObject = ECamActivationRef.NewlyEnteredCamTable) or on the one to be removed (ActivationOptions.ReferenceObject = ECamActivationRef.CamTableAlreadyInUse)

##### `MC_CamIn` (FUNCTION_BLOCK) — §5.4.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `Type_Master1D [} 230]` | Reference to the master axis |
| `Slave` | `Type_Camming1D [} 229]` | Reference to the slave axis |
| `CamObject` | `Type_CouplingFunction1D [} 348]` | Reference to the cam object |
| `Execute` | `BOOL` | Start cam in at rising edge. |
| `Scaling` | `CamScalingData [} 346]` | Scaling of the new coupling |
| `ActivationOptio CamActivationOptions` | `Options controlling how the cam coupling` | — |
| `ns` | `[} 340]` | is activated (mode, position, reference |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InSync` | `BOOL` | Synchronization succeeded |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block initiates camming-based synchronization on a slave. If the slave is already coupled to one or more cam table couplings, it switches to the cam table coupling that is entered in this function block. Note: Use MC_CamAdd to switch from single-camming to multi-camming. Switch back and forth between single-camming and multi-camming is possible and not limited.

##### `MC_CamRemove` (FUNCTION_BLOCK) — §5.4.3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Slave` | `Type_Camming1D [} 229] Reference to the slave axis` | — |
| `CamObject` | `Type_CouplingFunction1D Cam object to be removed (reference to the object) [} 348]` | — |
| `Execute` | `BOOL` | Start cam remove at rising edge. |
| `ActivationOptio CamActivationOptions` | `Options controlling how the cam coupling is activated (mode,` | — |
| `ns` | `[} 340]` | position, reference object). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Removal succeeded |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block removes a cam table coupling from a slave. As a cam table can only be used once in a coupled slave, the master information is not needed. However, the cam table must be in use when this function block is executed and when the activation condition is met, otherwise an error is triggered. If there is no active coupling left and there are no pending commands to add couplings, the camming synchronization ends by itself and axis state changes to Standstill.

#### PLC libraries › Tc3_Mc3Camming › Function Blocks › Motion Object › MC_CamObject

##### `Connect` (METHOD) → `UDINT` — §5.4.3.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `OptionalOid` | `OTCID` | Cam table object ID, used when McToPlc is not linked. If both |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT Possible error codes` | `` | — |

**Description:** Stores a reference to an existing cam table object. The cam table must have been previously linked to McToPlc, or the object ID must be passed as parameter. Returns 0 on success or a non-zero error code.

##### `Create` (METHOD) → `UDINT` — §5.4.3.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Name` | `STRING(80)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT Possible error codes` | `` | — |

**Description:** Creates a new cam table object and stores a reference to it. Returns 0 on success or a non-zero error code.

##### `WriteData` (METHOD) → `UDINT` — §5.4.3.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SourceAddress Pointer To` | `Pointer to the new set of data points. MotionFunctionPoint [} 347]` | — |
| `NumberOfPoint UDINT` | `Number of data points in the source array. s` | — |
| `Periodic` | `BOOL` | TRUE for a periodic cam table. |
| `ContinuityRequ EContinuity [} 200]` | `Minimum continuity requirement. The higher the requirement,` | — |
| `irement` | `the stricter the point validation.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT Possible error codes` | `` | — |

**Description:** Calculates the entire cam table from a new set of data points. Returns 0 on success or a non-zero error code.

##### `ReadData` (METHOD) → `UDINT` — §5.4.3.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DestinationAdd Pointer To` | `Pointer to the destination buffer for the requested data points.` | — |
| `ress` | `MotionFunctionPoint [} 347]` | — |
| `NumberOfPoint UDINT` | `Number of points to read. s` | — |
| `StartIndex` | `UDINT` | Index of the first point to read. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NumberOfPoint UDINT` | `The actual number of points read, which can be less than or` | — |
| `sRead` | `equal to the specified NumberOfPoints. UDINT Possible error codes` | — |

**Description:** Reads data points from index StartIndex to StartIndex + NumberOfPoints - 1. The first valid index is 1. Returns 0 on success or a non-zero error code.

##### `OverwriteData` (METHOD) → `UDINT` — §5.4.3.3.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SourceAddress Pointer To` | `Pointer to the new data points to write. MotionFunctionPoint [} 347]` | — |
| `NumberOfPoint UDINT` | `Number of points to overwrite. s` | — |
| `StartIndex` | `UDINT` | Index of the first point to overwrite. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT Further information` | `` | — |

**Description:** Overwrites data points from index StartIndex to StartIndex + NumberOfPoints - 1 and recalculates the modified cam table segments. The first valid index is 1. Returns 0 on success or a non-zero error code.

##### `ReadSlaveDynamics` (METHOD) → `UDINT` — §5.4.3.3.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MasterPosition LREAL` | `Master cam position at which to evaluate the slave dynamics.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SlavePosition LREAL` | `Computed slave position at the given master cam position.` | — |
| `SlaveVelocity LREAL` | `Computed slave velocity at the given master cam position.` | — |
| `SlaveAccelerati LREAL` | `Computed slave acceleration at the given master cam position. on UDINT Possible error codes` | — |

**Description:** Evaluates slave position, velocity and acceleration at a specific master position within the cam table profile. Returns 0 on success or a non-zero error code.

##### `ComputeMasterPositions` (METHOD) → `UDINT` — §5.4.3.3.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SlavePosition` | `LREAL` | Position of the slave axis for which the master positions are to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NumberOfMast UDINT` | `Number of master positions found matching the given slave` | — |
| `erPositionsFou` | `position and master position range. nd UDINT Possible error codes` | — |

**Description:** Finds the master position(s) for a given slave position. Returns 0 on success or a non-zero error code.

##### `ReadMasterRange` (METHOD) → `UDINT` — §5.4.3.3.1.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MasterMinPosit LREAL` | `Leftmost master position of the cam table profile. ion` | — |
| `MasterMaxPosi LREAL` | `Rightmost master position of the cam table profile. tion UDINT Possible error codes` | — |

**Description:** Returns leftmost and rightmost master position of the cam table profile. Returns 0 on success or a non-zero error code.

##### `ComputeCharacteristics` (METHOD) → `UDINT` — §5.4.3.3.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NominalMaster LREAL` | `Nominal master velocity used for computing absolute slave` | — |
| `Velocity` | `dynamics.` | — |
| `Destination` | `Reference To` | Reference to a CamCharacteristics structure to receive the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT Possible error codes` | `` | — |

**Description:** Computes cam table characteristics such as extreme slave dynamics for a given nominal master velocity. Returns 0 on success or a non-zero error code.

##### `ClearData` (METHOD) → `UDINT` — §5.4.3.3.1.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT Possible error codes` | `` | — |

**Description:** Removes all stored data. The existing reference remains intact. Returns 0 on success or a non-zero error code.

##### `CopyFrom` (METHOD) → `UDINT` — §5.4.3.3.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Source` | `Reference To` | Source MC_CamObject to copy from. MC_CamObject [} 367] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT Possible error codes` | `` | — |

**Description:** Copies data points and cam table segments from another valid MC_CamObject. Returns 0 on success or a non-zero error code.

#### PLC libraries › Tc3_Mc3FluidPower › Function Blocks › Valve Characteristic Curve › MC_ValveCharacteristicCurve

##### `Connect` (METHOD) → `UDINT` — §5.5.2.1.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT` | `` | — |

**Description:** Stores a reference to an existing valve characteristic curve object. The valve characteristic curve must have been previously linked to McToPlc.

##### `ReadData` (METHOD) → `UDINT` — §5.5.2.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DestinationAdd Pointer To` | `Pointer to ARRAY[1..NumberOfPoints] of` | — |
| `ress` | `ValveCharacteristicPoint [} 384]` | ValveCharacteristicPoint |
| `NumberOfPoint UDINT` | `Capacity of the destination array (maximum number of data` | — |
| `s` | `points to read).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT` | `` | — |

**Description:** Reads all stored data points into the given array. Returns the number of data points read.

##### `OverwriteData` (METHOD) → `UDINT` — §5.5.2.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SourceAddress Pointer To` | `Pointer to ARRAY[1..NumberOfPoints] of` | — |
| `ValveCharacteristicPoint` | `ValveCharacteristicPoint [} 384]` | — |
| `NumberOfPoint UDINT` | `Number of points to be written. s` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT` | `` | — |

**Description:** Overwrites all stored data points with data points from the given array.

##### `ClearData` (METHOD) → `UDINT` — §5.5.2.1.1.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT` | `` | — |

**Description:** Removes all stored data points. The existing reference remains intact.

##### `CopyFrom` (METHOD) → `UDINT` — §5.5.2.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Source` | `Reference To` | Source MC_ValveCharacteristicCurve to copy from. MC_ValveCharacteristicCu rve [} 386] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT` | `` | — |

**Description:** Copies all stored data points from another MC_ValveCharacteristicCurve.

##### `Enable` (METHOD) → `UDINT` — §5.5.2.1.1.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT` | `` | — |

**Description:** Enables valve linearization.

##### `Disable` (METHOD) → `UDINT` — §5.5.2.1.1.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT` | `` | — |

**Description:** Disables valve linearization.

##### `ComputeVelocityValue` (METHOD) → `LREAL` — §5.5.2.1.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SpoolPosition` | `LREAL` | Spool position at which to evaluate the velocity. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LREAL` | `` | — |

**Description:** Computes velocity value at the given SpoolPosition.

##### `ComputeSpoolPositionValue` (METHOD) → `LREAL` — §5.5.2.1.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Velocity` | `LREAL` | Velocity at which to evaluate the spool position. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LREAL` | `` | — |

**Description:** Computes spool position value at the given Velocity.

#### PLC libraries › Tc3_Mc3FluidPower › Function Blocks › Valve Characterization

##### `MC_ValveCharacterization` (FUNCTION_BLOCK) — §5.5.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Start the valve characterization process at |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `Reference To AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Valve characterization finished successfully |
| `Busy` | `BOOL` | Function block is not finished and new output values are to be |

**Description:** This function block characterizes the valve characteristic curve of the axis through a series of velocity measurements. For each measurement the function block generates an output value for the drive based on the
