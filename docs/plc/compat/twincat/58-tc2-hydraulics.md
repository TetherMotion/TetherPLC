# Tc2_Hydraulics

- **Manual:** `TF5810_TC3_Hydraulic_Positioning_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion · **Tier:** T3 · **API entries:** 118
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Hydraulic positioning: axis backend with hydraulic drive model.

---

#### PLCopen Motion Control › Administrative

##### `MC_Power_BkPlcMc` — §4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | A TRUE at this input activates an external actuator |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `State` | `BOOL` | Readiness for operation is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded error message is provided here. Behavior of the function block |

**Description:** The function block is used to control an external actuator. Further information on this topic can be found under FAQ #9 [} 333].

##### `MC_ReadActualPosition_BkPlcMc` — §4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Updating of the position value is initiated by a rising edge at this input. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc should be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Valid` | `BOOL` | Successful determination of the actual position is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Position` | `LREAL` | [mm] The actual position. |

**Description:** The function block determines the current position of an axis.

##### `MC_ReadActualTorque_BkPlcMc` — §4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | A rising edge at this input triggers an update of the actual value. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | This indicates successful determination of the actual value. |
| `Busy` | `BOOL` | This output is TRUE while the command is being processed. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Torque` | `LREAL` | The actual force or actual pressure. Behavior of the function block |

**Description:** The function block determines the current actual force or actual pressure of an axis.

##### `MC_ReadActualVelocity_BkPlcMc` — §4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | A rising edge at this input triggers an update of the velocity value. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | This indicates successful determination of the velocity. |
| `Busy` | `BOOL` | This output is TRUE while the command is being processed. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Velocity` | `LREAL` | [mm/s] The actual velocity. Behavior of the function block |

**Description:** The function block determines the current velocity of an axis.

##### `MC_ReadAxisError_BkPlcMc` — §4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | TRUE at this input triggers an update of the error code. Inputs/outputs |
| `Axis` | `AXIS_RE Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should be` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Valid` | `BOOL` | Successful determination of the actual position is indicated here. |
| `Error` | `BOOL` | Indicates TRUE, if the function block was unable to execute the required |

**Description:** This function block determines the current error code of an axis.

##### `MC_ReadBoolParameter_BkPlcMc` — §4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | A reading process is initiated by a rising edge at this input. |
| `ParameterNumber` | `INT` | This code number specifies the parameter that is to be read. Only named |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Valid` | `BOOL` | Successful execution of the reading process is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Value` | `BOOL` | The value of the parameter is made available here. Behavior of the function block |

**Description:** This function block reads the boolean parameters of an axis. The function block MC_ReadParameter_BkPlcMc [} 36] is available for non-boolean parameters.

##### `MC_ReadDigitalOutput_BkPlcMc` — §4.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | A rising edge at this input triggers an update of the state. |
| `OutputNumber` | `INT` | The number of the output to be determined. Inputs/outputs |
| `Output` | `OUTPUT_REF_BkPlcMc` | Here, the address of a variable of type OUTPUT_REF_BkPlcMc |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output` | `OUTPUT_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | This indicates successful determination of the state. |
| `Busy` | `BOOL` | This output is TRUE while the command is being processed. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Value` | `BOOL` | The state of the digital output. Behavior of the function block |

**Description:** The function block determines the current state of a digital output of a cam controller.

##### `MC_ReadParameter_BkPlcMc` — §4.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | A reading process is initiated by a rising edge at this input. |
| `ParameterNumber` | `INT` | This code number specifies the parameter that is to be read. Only named |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Valid` | `BOOL` | Successful execution of the reading process is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Value` | `LREAL` | The value of the parameter is made available here. Behavior of the function block |

**Description:** This function block reads the non-boolean parameters of an axis. The function block MC_ReadBoolParameter_BkPlcMc [} 33] is available for boolean parameters.

##### `MC_ReadStatus_BkPlcMc` — §4.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | A TRUE state at this input triggers an update of the function block. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Valid` | `BOOL` | Successful determination of the actual position is indicated here. |
| `Error` | `BOOL` | This output reports any problems relating to the function of the function |

**Description:** The function block determines the current state of an axis.

##### `MC_Reset_BkPlcMc` — §4.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | An axis reset is initiated by a rising edge at this input. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful execution of the axis reset is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |

**Description:** The function block eliminates an error state and puts the axis in an operational state.

##### `MC_ResetAndStop_BkPlcMc` — §4.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input triggers an axis reset and a |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful execution of the axis reset is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block puts a faulty axis in an operational state. If the axis is processing a travel command, this is aborted, and the associated required stop operation is monitored.

##### `MC_SetOverride_BkPlcMc` — §4.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | An active state at this input sets the override of the axis. |
| `VelFactor` | `LREAL` | [1] The new override of the axis. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Enabled` | `BOOL` | This indicates the active state of the function block. |
| `Busy` | `BOOL` | This output is TRUE while the command is being processed. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block sets the override of an axis.

##### `MC_SetPosition_BkPlcMc` — §4.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input sets the actual position of the axis. |
| `Position` | `LREAL` | [mm] The new actual position of the axis. |
| `Mode` | `BOOL` | This parameter specifies the operating mode. If Mode = TRUE, the actual position is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This indicates successful processing of the command. |
| `Busy` | `BOOL` | This output is TRUE while the command is being processed. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block sets the actual position of an axis.

##### `MC_SetReferenceFlag_BkPlcMc` — §4.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input sets the referencing flag of the axis. |
| `ReferenceFlag` | `BOOL` | The new state of the referencing flag of the axis. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This indicates successful processing of the command. |
| `Busy` | `BOOL` | This output is TRUE while the command is being processed. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** (Function is not defined by PLCopen) The function block defines the referencing flag of the axis.

##### `MC_WriteBoolParameter_BkPlcMc` — §4.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A write process is initiated by a rising edge at this input. |
| `ParameterNumber` | `INT` | This code number specifies the parameter that is to be read. Only named |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful execution of the writing process is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** This function block writes the boolean parameters of an axis. The function block MC_WriteParameter_BkPlcMc [} 48] is available for non-boolean parameters.

##### `MC_WriteDigitalOutput_BkPlcMc` — §4.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input triggers an update of the state. |
| `OutputNumber` | `INT` | The number of the output to be determined. |
| `Value` | `BOOL` | The state of the digital output. Inputs/outputs |
| `Output` | `OUTPUT_REF_BkPlcMc` | Here, the address of a variable of type OUTPUT_REF_BkPlcMc |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output` | `OUTPUT_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This indicates successful determination of the state. |
| `Busy` | `BOOL` | This output is TRUE while the command is being processed. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block determines the state of a digital output of a cam controller.

##### `MC_WriteParameter_BkPlcMc` — §4.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | A write process is initiated by a rising edge at this input. |
| `ParameterNumber` | `INT` | This code number specifies the parameter that is to be read. Only named |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful execution of the writing process is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** This function block writes the non-boolean parameters of an axis. The function block MC_WriteBoolParameter_BkPlcMc [} 46] is available for boolean parameters.

#### PLCopen Motion Control › Motion

##### `MC_CamIn_BkPlcMc` — §4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the coupling. |
| `MasterOffset` | `LREAL` | [mm, 1] This value is offset against with the actual |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF_BkPlcMc` | — |
| `Slave` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `InSync` | `BOOL` | This indicates the first successful synchronization of the axes. The signal |

**Description:** The function block starts and monitors a cam plate coupling between two axes. To release the coupling, an MC_CamOut_BkPlcMc [} 51] function block should be used.

##### `MC_CamOut_BkPlcMc` — §4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ND_VAR` | `` | — |
| `Execute` | `BOOL` | A rising edge at this input starts the coupling. Inputs/outputs |
| `Slave` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Slave` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful processing of the command. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block releases a cam plate coupling between two axes, which was started through an MC_CamIn_BkPlcMc [} 49] function block.

##### `MC_CamTableSelect_BkPlcMc` — §4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ND_VAR` | `` | — |
| `Execute` | `BOOL` | A rising edge at this input starts the command. |
| `Periodic` | `BOOL` | Not supported: FALSE is currently to be passed here. |
| `MasterAbsolute` | `BOOL` | Not supported: TRUE is currently to be passed here. |
| `SlaveAbsolute` | `BOOL` | Not supported: TRUE is currently to be passed here. Inputs/outputs |
| `Master` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF_BkPlcMc` | — |
| `Slave` | `AXIS_REF_BkPlcMc` | — |
| `CamTable` | `MC_CAM_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful initialization of CamTableId. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided |

**Description:** The function block initializes a variable of type MC_CAM_ID_BkPlcMc [} 122], thereby preparing a cam plate for the coupling of two axes.

##### `MC_DigitalCamSwitch_BkPlcMc` — §4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | This input controls all activities of the function block. |
| `EnableMask` | `DWORD` | A mask with bits that specify the activation of the outputs in Outputs. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |
| `Switches` | `CAMSWITCH_REF_BkPlcMc` | — |
| `Outputs` | `OUTPUT_REF_BkPlcMc` | — |
| `TrackOptions` | `TRACK_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InOperation` | `BOOL` | This indicates whether the function block is active. |
| `Busy` | `BOOL` | This output is TRUE while the command is being processed. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block generates software cams depending on the position, direction of travel and velocity of an axis.

##### `MC_EmergencyStop_BkPlcMc` — §4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input ends a movement being carried out by the axis. |
| `RampTime` | `LREAL` | [s] The required stopping time. |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful processing of the operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Active` | `BOOL` | Indicates that a command is being processed. |
| `CommandAborted` | `BOOL` | Indicates that processing of this command was aborted by another |

**Description:** The function block cancels a current axis motion and monitors the emergency stop operation.

##### `MC_FlyingGear_BkPlcMc` — §4.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | This signal allows the function block to become active as soon as the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF_BkPlcMc` | — |
| `Slave` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartSync` | `BOOL` | This signal is TRUE if the master is within the section for the establishment of the |

**Description:** The function block takes over the control of a flying gear coupling.

##### `MC_GearIn_BkPlcMc` — §4.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the coupling. |
| `RatioNumerator` | `INT` | [1, 1] These parameters describe the coupling |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF_BkPlcMc` | — |
| `Slave` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `InGear` | `BOOL` | This indicates the first successful synchronization of the axes. The signal |

**Description:** The function block starts and monitors a coupling between two axes. To release the coupling, an MC_GearOut_BkPlcMc [} 67] function block should be used.

##### `MC_GearInPos_BkPlcMc` — §4.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the coupling. |
| `RatioNumerator` | `INT` | [1, 1] These parameters describe the coupling factor in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF_BkPlcMc` | — |
| `Slave` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartSync` | `BOOL` | Indicates the transition phase between idle state and fully active |

**Description:** The function block starts and monitors an on-the-fly coupling between two axes. To release the coupling, an MC_GearOut_BkPlcMc [} 67] function block should be used.

##### `MC_GearOut_BkPlcMc` — §4.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input releases the coupling. Inputs/outputs |
| `Slave` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Slave` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful processing of the movement is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block releases a coupling between two axes. This coupling must have been established with an MC_GearIn_BkPlcMc [} 63] or an MC_GearInPos_BkPlcMc [} 65] function block.

##### `MC_Home_BkPlcMc` — §4.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The homing is initiated by a rising edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful processing of the homing is indicated here. |
| `CommandAborted` | `BOOL` | Abortion of homing is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** This function block starts and monitors the homing of an axis.

##### `MC_Halt_BkPlcMc` — §4.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input ends a movement being |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful processing of the operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Active` | `BOOL` | Indicates that a command is being processed. |
| `CommandAborted` | `BOOL` | Indicates that processing of this command was aborted by another |

**Description:** The function block cancels a current axis motion and monitors the stop operation.

##### `MC_ImediateStop_BkPlcMc` — §4.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input ends a movement being carried out by the axis. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful processing of the operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Active` | `BOOL` | Indicates that a command is being processed. |
| `CommandAborted` | `BOOL` | Indicates that processing of this command was aborted by another |

**Description:** The function block cancels a current axis motion.

##### `MC_MoveAbsolute_BkPlcMc` — §4.2.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The movement is initiated by a rising edge at this input. |
| `Position` | `LREAL` | [mm] The target position of the movement in actual |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful processing of the movement is indicated here. |
| `CommandAborted` | `BOOL` | Abortion of the movement is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** This function block starts and monitors the movement of an axis.

##### `MC_MoveJoySticked_BkPlcMc` — §4.2.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The movement is initiated by a rising edge at this input. |
| `JoyStick` | `LREAL` | [1] The velocity specified via the control unit, normalized to the range ±1.0. |
| `Acceleration` | `LREAL` | [mm/s2] The required acceleration in actual value units of the axis per square |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `CommandAborted` | `BOOL` | Abortion of the movement is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** This function block starts and monitors the movement of an axis.

##### `MC_MoveRelative_BkPlcMc` — §4.2.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The movement is initiated by a rising edge at this input. |
| `Distance` | `LREAL` | [mm] The distance to the target position of the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful processing of the movement is indicated here. |
| `CommandAborted` | `BOOL` | Abortion of the movement is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** This function block starts and monitors the movement of an axis.

##### `MC_MoveVelocity_BkPlcMc` — §4.2.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The movement is initiated by a rising edge at this input. |
| `Velocity` | `LREAL` | [mm/s] The required motion velocity in actual value units of |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `InVelocity` | `BOOL` | This output becomes TRUE when the axis reaches the required velocity |

**Description:** This function block starts and monitors the movement of an axis.

##### `MC_RampedStop_BkPlcMc` — §4.2.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input ends a movement being carried out by the axis. |
| `RampTime` | `LREAL` | [s] The required stopping time. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful processing of the operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Active` | `BOOL` | Indicates that a command is being processed. |
| `CommandAborted` | `BOOL` | Indicates that processing of this command was aborted by another |

**Description:** The function block cancels a movement that is currently being executed.

##### `MC_Stop_BkPlcMc` — §4.2.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input ends a movement being |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful processing of the operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Active` | `BOOL` | Indicates that a command is being processed. |
| `CommandAborted` | `BOOL` | Indicates that processing of this command was aborted by another |

**Description:** The function block cancels a current axis motion and monitors the stop operation.

##### `MC_MoveJog_BkPlcMc` — §4.2.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `JogForward` | `BOOL` | The command is executed with rising edge and the axis moved |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful processing of the movement is indicated here. |
| `CommandAborted` | `BOOL` | Abortion of the movement is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |

**Description:** This function block starts and monitors the movement of an axis.

#### PLCopen Motion Control › System › Controller

##### `MC_AxCtrlAutoZero_BkPlcMc` — §4.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | This input controls the activity of the compensation. |
| `EnableOnMoving` | `BOOL` | This input controls the activity of the compensation. |
| `OffsetLimit` | `LREAL` | [V] The value in fZeroCompensation is limited to this value. |
| `Tn` | `LREAL` | [s] The integral action time of the compensation. This is the time for a |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Active` | `BOOL` | Indicates that the function block actively adjusts the value of fZeroCompensation in |

**Description:** The function block executes an automatic zero compensation. This function block may only be used for zero overlap valves.

##### `MC_AxCtrlPressure_BkPlcMc` — §4.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | TRUE at this input activates the controller. |
| `Reset` | `BOOL` | TRUE at this input resets the controller. The memory of |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `InWindup` | `UDINT` | This output becomes TRUE if the I part is limited by WindupLimit. |

**Description:** kP LREAL

##### `MC_AxCtrlPressureFF_BkPlcMc` — §4.4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | TRUE at this input activates the controller. |
| `Enable_P` | `BOOL` | A TRUE at this input activates the proportional component of the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | A TRUE here signals an error. |
| `ErrorID` | `UDINT` | A numerically encoded error indication. |
| `InWindup` | `UDINT` | A TRUE here signals that the integrator is limited by the WindupLimit. |
| `Deviation` | `LREAL` | The current controller deviation. |
| `Proportional` | `LREAL` | The current proportional component. |
| `Integrator` | `LREAL` | The current integral component. |
| `Differential` | `LREAL` | The current differential component. |
| `Response` | `LREAL` | The output of the controller. |

**Description:** pParam Pointer To ST_TcPctrlParam

##### `MC_AxCtrlSlowDownOnPressure_BkPlcMc` — §4.4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `E_TcMcPressureReadingMode [} 119]` | `` | — |
| `EnableP` | `BOOL` | TRUE at this input enables the controller to influence the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Response` | `LREAL` | The output value of a pressure controller. |
| `Active` | `BOOL` | TRUE at this output indicates that the function block generates a response in |

**Description:** kP LREAL

##### `MC_AxCtrlSlowDownOnPressureEx_BkPlcMc` — §4.4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | A TRUE at this input enables the controller. |
| `EnableRelief` | `BOOL` | A TRUE at this input allows the controller to actively back off if |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | A TRUE at this output indicates that the function block is enabled. This does not |

**Description:** pParam Pointer To ST_TcPctrlParam

##### `MC_AxCtrlStepperDeStall_BkPlcMc` — §4.4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `EnableAcc` | `BOOL` | These inputs determine whether the monitoring may intervene during the |
| `EnableDec` | `BOOL` | acceleration and braking phases. |
| `Reset` | `BOOL` | This input controls the activity of the controller. |
| `UseKL2531State` | `BOOL` | If TRUE is transferred here, the function block evaluates |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Active` | `BOOL` | Indicates that a stall situation was detected. |
| `Activated` | `BOOL` | Indicates that a stall situation was detected since the last start of an active axis |

**Description:** The function block monitors the motion of a stepper motor axis, which is operated with an encoder.

##### `MC_AxRtPosPiControllerEx_BkPlcMc` — §4.4.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | This input deletes all internal and external controller responses. |
| `I_Enable` | `BOOL` | This input controls the activity of the I part. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetPos` | `LREAL` | [mm] The set position that becomes effective at the internal controller. |
| `SetVelo` | `LREAL` | [mm/s] The set velocity that becomes effective at the internal controller. |
| `Response` | `LREAL` | [mm/s] The controller response. |
| `InWindup` | `BOOL` | Here, the limitation of the I part that has become active is signaled. Purpose of the function block |

**Description:** The function block can be used as an alternative to the default position controller. It is called after the MC_AxRuntime_BkPlcMc() function block (setpoint generator and default position controller). This arrangement overwrites the responses of the default position controller.

#### PLCopen Motion Control › System › Drive

##### `MC_AxRtDrive_BkPlcMc` — §4.4.2.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |

**Description:** The function block performs preparation of the control value for the axis for it to be output on a hardware module. To this end a function block is called depending on the value set as nDrive_Type in Axis.ST_TcHydAxParam [} 133], which takes into account the special features of the hardware module.

#### PLCopen Motion Control › System › Encoder

##### `MC_AxRtEncoder_BkPlcMc` — §4.4.3.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `AboveLimit` | `BOOL` | Exceeding of the upper software limit switch is indicated by the actual position. |
| `BelowLimit` | `BOOL` | If the value falls below the lower software limit switch, this is indicated by the |

**Description:** This function block determines the actual position of the axis from the input information of a hardware module. To this end a function block is called depending on the value set as nEnc_Type in Axis.ST_TcHydAxParam [} 133], which takes into account the special features of the hardware module.

##### `MC_AxRtReadForceDiff_BkPlcMc` — §4.4.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AdcValueA` | `INT` | These parameters are used to transfer the input data of the |
| `AdcValueB` | `INT` | analog terminals. |
| `ScaleFactorA` | `LREAL` | [N/ADC_INC] This value represents the weighting. It |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is |

**Description:** The function block handles determination of the actual force of the axis from the input data of two analog input terminals. The actual pressure on the A- and B-sides is converted to the force acting on the load, taking into account the areas and the sliding friction.

##### `MC_AxRtReadForceSingle_BkPlcMc` — §4.4.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AdcValueA` | `INT` | These parameters are used to transfer the input data of |
| `AdcValueB` | `INT` | the analog terminals. |
| `ScaleFactorA` | `LREAL` | [N/ADC_INC] This value represents the weighting. It |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Error: The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | ErrorID: Here, a coded cause of error is provided. |

**Description:** The function block handles determination of the actual force of the axis from the input data of an analog input terminal. The actual pressure on the A- or B-sides is converted to the force acting on the load, taking into account the area and the sliding friction.

##### `MC_AxRtReadPressureDiff_BkPlcMc` — §4.4.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AdcValueA` | `INT` | These parameters are used to transfer the input data of |
| `AdcValueB` | `INT` | the analog terminals. |
| `ScaleFactorA` | `LREAL` | [bar/ADC_INC] This value represents the weighting. It |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is |

**Description:** The function block handles determination of the actual pressure of the axis from the input data of two analog input terminals.

##### `MC_AxRtReadPressureSingle_BkPlcMc` — §4.4.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AdcValue` | `INT` | These parameters are used to transfer the input data of |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |

**Description:** The function block handles determination of the actual pressure of the axis from the input data of an analog input terminal.

##### `MC_AxRtHybridAxisActuals_BkPlcMc` — §4.4.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stSystem_PrsIn` | `ST_TcPlcInputAnalog [} 168] If a pressure sensor is present at the pressurized` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorId` | `BOOL` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** stBside_PrsIn ST_TcPlcInputAnalog nTorqueFeedback INT udiMotorEnc_Count UDINT bUpdateActForce BOOL bPumpSwitch_Ext BOOL bAreaSwitch_Ext BOOL

#### PLCopen Motion Control › System › FunctionGenerator

##### `MC_FunctionGeneratorFD_BkPlcMc` — §4.4.4.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTimeBase` | `ST_FunctionGeneratorTB_BkPlcMc` | — |
| `stFunctionDef` | `ST_FunctionGeneratorFD_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Sine` | `LREAL` | The output signals of the function generator. |
| `Cosine` | `LREAL` | — |
| `Rectangle` | `LREAL` | — |
| `SawTooth` | `LREAL Behavior of the function block` | — |

**Description:** The function block calculates the signals of a function generator.

##### `MC_FunctionGeneratorSetFrq_BkPlcMc` — §4.4.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Frequency` | `LREAL` | The operating frequency to be used. |
| `CycleTime` | `LREAL` | The cycle time of the calling task. |
| `stTimeBase` | `ST_FunctionGeneratorTB_Bk A structure with the parameters of the time base of one or` | — |
| `PlcMc` | `several function generators [} 128]. Behavior of the function block` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTimeBase` | `ST_FunctionGeneratorTB_BkPlcMc` | — |

**Description:** The function block updates the operating frequency of a time base for one or several function generators [} 230].

##### `MC_FunctionGeneratorTB_BkPlcMc` — §4.4.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CycleTime` | `LREAL` | The cycle time of the calling task. Inputs/outputs |
| `stTimeBase` | `ST_FunctionGeneratorTB_BkPl A structure with the parameters of the time base of one or` | — |
| `cMc` | `several function generators [} 128]. Behavior of the function block` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTimeBase` | `ST_FunctionGeneratorTB_BkPlcMc` | — |

**Description:** The function block updates a time base for one or several function generators [} 230].

#### PLCopen Motion Control › System › TableFunctions

##### `MC_AxTableFromAsciFile_BkPlcMc` — §4.4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the read process. |
| `pTable` | `POINTER TO LREAL` | This parameter is used to transfer the address of an |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful processing of the homing is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `LastIdx` | `INT` | This parameter is used to indicate the index of the last table row defined by the read |

**Description:** The function block reads the contents of a table from a text file.

##### `MC_AxTableFromBinFile_BkPlcMc` — §4.4.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the read process. |
| `pTable` | `POINTER TO LREAL` | This parameter is used to transfer the address of an |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful processing of the homing is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `LastIdx` | `INT` | This parameter is used to indicate the index of the last table row defined by the read |

**Description:** The function block reads the contents of a table from a binary file.

##### `MC_AxTableReadOutNonCyclic_BkPlcMc` — §4.4.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTable` | `POINTER TO` | This parameter is used to transfer the address of an |
| `LREAL` | `ARRAY[nFirstIdx..nLastIdx,1..2].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fSlaveValue` | `LREAL` | This parameter is used to output the slave value belonging to fMasterValue. |
| `fSlaveGear` | `LREAL` | This parameter is used to output the local slope of the slave values at the |

**Description:** The function block determines the slave values assigned to a master value with the aid of a table.

##### `MC_AxTableToAsciFile_BkPlcMc` — §4.4.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The writing process is initiated by a rising edge at this input. |
| `pTable` | `POINTER TO LREAL` | This parameter is used to transfer the address of an |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful processing of the homing is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block writes the contents of a table to a text file.

##### `MC_AxTableToBinFile_BkPlcMc` — §4.4.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The writing process is initiated by a rising edge at this input. |
| `pTable` | `POINTER TO LREAL` | This parameter is used to transfer the address of an |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful processing of the homing is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block writes the contents of a table to a binary file.

#### PLCopen Motion Control › System › Generators

##### `MC_AxRtGenerator_BkPlcMc` — §4.4.6.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `LagError` | `BOOL` | If the lag error exceeds the set limits, it is indicated here. This signal is also |

**Description:** This function block performs the task of a setpoint generator. To this end a profile-specific function block is called, depending on the value set as nProfileType in Axis.ST_TcHydAxParam [} 133].

##### `MC_AxRuntime_BkPlcMc` — §4.4.6.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `LagError` | `BOOL` | If the lag error exceeds the set limits, it is indicated here. This signal is also |

**Description:** The function block integrates a function block of the type MC_AxRtGenerator_BkPlcMc() [} 241] and a function block of the type MC_AxRtController_BkPlcMc() [} 249]. The outputs of the generator are forwarded.

#### PLCopen Motion Control › System › Runtime

##### `MC_AxRtCheckSyncDistance_BkPlcMc` — §4.4.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MaxDistance` | `LREAL` | [mm] This parameter is used to specify the maximum permitted distance |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Active` | `BOOL` | Indicates that the axis has left the cam has and expects the zero pulse of the |

**Description:** The function block checks for an invalid path (distance) after leaving the cam during homing.

##### `MC_AxRtController_BkPlcMc` — §4.4.7.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**Description:** This function block contains the standard position controller of the axis.

##### `MC_AxRtFinish_BkPlcMc` — §4.4.7.3

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** This function block adapts the control value that has been generated to the special features of the particular axis. An MC_AxRtFinishLinear_BkPlcMc [} 251] function block should be used if a characteristic curve linearization is required.

##### `MC_AxRtFinishLinear_BkPlcMc` — §4.4.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `EnableLinearisation` | `BOOL` | TRUE at this input activates the linearization. |
| `ValveTable` | `POINTER TO` | The address of the linearization table should be transferred |
| `LREAL` | `here. If possible, this should be the ValveCharacteristicTable of` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**Description:** The function block deals with the adjustment of the generated control value to the special features of the axis, taking into account a characteristic curve.

##### `MC_AxRtGoErrorState_BkPlcMc` — §4.4.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Trigger` | `BOOL` | A rising edge at this input places the axis in a fault state. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `NoLogging` | `BOOL` | TRUE at this input suppresses the output of a message. |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**Description:** (not recommended) This function block places the axis into a fault state.

##### `MC_AxRtMoveChecking_BkPlcMc` — §4.4.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | TRUE at this input activates the monitoring. |
| `MinDistance` | `LREAL` | [mm] The required minimum distance must be transferred here. |
| `Filter` | `LREAL` | [s] The measuring time must be specified here. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**Description:** The function block monitors the response of an axis.

##### `MC_AxRtSetDirectOutput_BkPlcMc` — §4.4.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | TRUE at this input activates the output. |
| `OutValue` | `LREAL` | The control value to be output should be transferred here. |
| `RampTime` | `LREAL` | [s] Here, the time should be specified in which the control value would reach full |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**Description:** The function block issues a control value, regardless of a profile generation.

##### `MC_AxRtSetExtGenValues_BkPlcMc` — §4.4.7.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | TRUE at this input activates the transfer of the command variables provided. |
| `Position` | `LREAL` | [mm] Set position value to be transferred cyclically. |
| `Velocity` | `LREAL` | [mm/s] Set velocity value to be transferred cyclically. |
| `TargetPosition` | `LREAL` | [mm] Target position value for the current motion to be transferred cyclically. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OUTPUT` | `` | — |
| `Error:` | `BOOL;` | — |
| `ErrorID:` | `UDINT;` | — |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block supplies an axis with command variables, which do not originate from the axis' own generator.

##### `MC_AxStandardBody_BkPlcMc` — §4.4.7.9

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** This function block calls a function block of each of the following types: MC_AxRtEncoder_BkPlcMc [} 202], MC_AxRuntime_BkPlcMc [} 241], MC_AxRtFinish_BkPlcMc [} 250] and MC_AxRtDrive_BkPlcMc [} 191].

##### `MC_AxUtiStandardInit_BkPlcMc` — §4.4.7.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AxisName` | `STRING` | Here, the text-based name of the axis (example: 'Axis_1') should be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates that a command has been successfully processed. |
| `Ready` | `BOOL` | This indicates that a command has been successfully processed and the axis |

**Description:** pStAxAutoParams Pointer To ST_TcMcAutoIdent pStAxLinCurveBuffer Pointer To ST_TcMcAxLinCurveBuffer pStAxCommandBuf Pointer To ST_TcPlcCmdBuffer_BkPlcMc nLogLevel DWORD fSaveDelay LREAL

##### `MC_AxRtCmdBufferExecute_BkPlcMc` — §4.4.7.11

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

#### PLCopen Motion Control › System › Message logging

##### `MC_AxRtLogAxisEntry_BkPlcMc` — §4.4.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBuffer` | `POINTER` | Here the address of a variable of type ST_TcPlcMcLogBuffer [} 160] is to be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `POINTER TO AXIS_REF_BkPlcMc` | — |

**Description:** pBuffer Pointer To ST_TcPlcMcLogBuffer diArg DINT lrArg LREAL sArg STRING(255)

##### `MC_AxRtLogClear_BkPlcMc` — §4.4.8.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pBuffer` | `POINTER TO ST_TcPlcMcLogBuffer` | — |

**Description:** pBuffer Pointer To ST_TcPlcMcLogBuffer

##### `MC_AxRtLogEntry_BkPlcMc` — §4.4.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBuffer` | `POINTER` | Here the address of a variable of type ST_TcPlcMcLogBuffer [} 160] is to be |

**Description:** pBuffer Pointer To ST_TcPlcMcLogBuffer Message STRING(255) diArg DINT lrArg LREAL sArg STRING(255)

##### `MC_AxRtLoggerDeSpool_BkPlcMc` — §4.4.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `spare` | `INT` | The required number of free messages in the LogBuffer. Inputs/outputs |
| `pBuffer` | `POINTER` | Here the address of a variable of type |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pBuffer` | `POINTER TO ST_TcPlcMcLogBuffer` | — |

**Description:** pBuffer Pointer To ST_TcPlcMcLogBuffer

##### `MC_AxRtLoggerRead_BkPlcMc` — §4.4.8.5

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Entry` | `INT:=0` | — |
| `pBuffer` | `POINTER TO ST_TcPlcMcLogBuffer` | — |
| `pEntry` | `POINTER TO ST_TcPlcMcLogEntry` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Result` | `DWORD` | Here, a coded cause of error is provided. Behavior of the function block |

**Description:** pBuffer Pointer To ST_TcPlcMcLogBuffer pEntry Pointer To ST_TcPlcMcLogEntry

##### `MC_AxRtLoggerSpool_BkPlcMc` — §4.4.8.6

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pBuffer` | `POINTER TO ST_TcPlcMcLogBuffer` | — |

**Description:** pBuffer Pointer To ST_TcPlcMcLogBuffer

#### PLCopen Motion Control › System › Utilities

##### `MC_AxParamDelayedSave_BkPlcMc` — §4.4.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Delay` | `LREAL` | [s] Delay when triggering the parameter backup. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | This output indicates any problems that may have occurred. |
| `ErrorID` | `UDINT` | In the event of an error, coded information about the type of problem is reported here. Behavior of the function block |

**Description:** The function block triggers a time-delayed writing of the parameter file after a parameter has been changed.

##### `MC_AxRtCommandsLocked_BkPlcMc : DWORD` — §4.4.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nStateDWord` | `DWORD` | The current state of the status double word. |
| `bProtect` | `BOOL` | The required state of the protective function. Behavior of the function |

**Description:** The function simplifies setting and deleting of a protective function in the status double word of an axis.

##### `MC_AxRtFollowUp_BkPlcMc` — §4.4.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | A TRUE at this input enables the function block. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type Axis_Ref_BkPlcMc |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**Description:** The function block updates the offset compensation.

##### `MC_AxRtUpdateExternalEncoder_BkPlcMc` — §4.4.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ActualPos` | `LREAL` | The new value for the actual position. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type Axis_Ref_BkPlcMc [} 86] |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**Description:** The function block updates the actual position of an axis with a value determined by the application.

##### `MC_AxUtiOffsetLatch_BkPlcMc` — §4.4.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge triggers the update. |
| `OffsetLimit` | `LREAL` | [V] The maximum permissible value range for the offset compensation. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type Axis_Ref_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | A successful update is indicated here. |
| `Error` | `BOOL` | This output indicates any problems that may have occurred. |
| `ErrorId` | `UDINT` | In the event of an error, coded information about the type of problem is reported here. |
| `Latched` | `BOOL` | This output signals that the update was successfully completed. |
| `Offset` | `LREAL` | [V] This output reports the offset value. It is only accepted as a new compensation |

**Description:** The function block updates the offset compensation.

#### PLCopen Motion Control › System › Utilities › Filters

##### `MC_AxUtiAverageDerivative_BkPlcMc` — §4.4.9.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `LREAL` | The raw value of the parameter to be filtered. |
| `MinIdx` | `DINT` | The index of the first filter buffer element to be used. |
| `MaxIdx` | `DINT` | The index of the last filter buffer element to be used. |
| `pBuffer` | `POINTER` | The address of the first filter buffer element. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output` | `LREAL` | The filtered value. |
| `Error` | `BOOL` | This output indicates problems with the transferred parameters. |
| `ErrorID` | `UDINT` | In the event of an error, coded information about the type of problem is reported here. Behavior of the function block |

**Description:** pBuffer Pointer To LREAL

##### `MC_AxUtiPT1_BkPlcMc` — §4.4.9.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fInput` | `LREAL` | The raw value of the parameter to be filtered. |
| `fCycletime` | `LREAL` | [s] The cycle time of the calling PLC task. |
| `fT0` | `LREAL` | [s] The filter time constant. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOutput` | `LREAL` | The filtered value. |
| `bError` | `BOOL` | This output indicates problems with the transferred parameters. |
| `nErrorId` | `UDINT` | In the event of an error, coded information about the type of problem is reported here. Behavior of the function block |

**Description:** The function block calculates a first-order low-pass.

##### `MC_AxUtiPT2_BkPlcMc` — §4.4.9.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fInput` | `LREAL` | The raw value of the parameter to be filtered. |
| `fCycletime` | `LREAL` | [s] The cycle time of the calling PLC task. |
| `fTheta` | `LREAL` | The attenuation. |
| `fT0` | `LREAL` | The filter time constant. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOutput` | `LREAL` | The filtered value. |
| `bError` | `BOOL` | This output indicates problems with the transferred parameters. |
| `nErrorId` | `UDINT` | In the event of an error, coded information about the type of problem is reported here. Behavior of the function block |

**Description:** fT0 LREAL

##### `MC_AxUtiSlewRateLimitter_BkPlcMc` — §4.4.9.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fInput` | `LREAL` | The raw value of the parameter to be filtered. |
| `fCycletime` | `DINT` | [s] The cycle time of the calling PLC task in seconds. |
| `fMaxRate` | `DINT` | The magnitude of the maximum permitted rate of change at the output as change |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOutput` | `LREAL` | [1/s] The filtered value. |
| `bError` | `BOOL` | This output indicates problems with the transferred parameters. |
| `nErrorId` | `UDINT` | In the event of an error, coded error information is output here. Behavior of the function block |

**Description:** The function block generates a rise-limited ramp.

##### `MC_AxUtiSlidingAverage_BkPlcMc` — §4.4.9.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `LREAL` | The raw value of the parameter to be filtered. |
| `MinIdx` | `DINT` | The index of the first filter buffer element to be used. |
| `MaxIdx` | `DINT` | The index of the last filter buffer element to be used. |
| `pBuffer` | `POINTER` | The address of the first filter buffer element. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output` | `LREAL` | Output: The filtered value. Behavior of the function block |

**Description:** pBuffer Pointer To LREAL

#### PLCopen Motion Control › System › Utilities › Identification

##### `MC_AxUtiAutoIdent_BkPlcMc` — §4.4.9.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input triggers the identification. |
| `Wait` | `BOOL` | (From TwinCAT 2 V3.0.44 / TwinCAT 3 V3.3.1.22) If this input is set to TRUE, the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful identification. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `Step` | `INT` | The current step of the internal sequence processor. |
| `InRecovery` | `BOOL` | (From TwinCAT 2 V3.0.44 / TwinCAT 3 V3.3.1.22) This indicates that a recovery |

**Description:** The function block automatic determines a number of axis parameters.

##### `MC_LinTableExportToAsciFile_BkPlcMc` — §4.4.9.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge initiates the export. |
| `FileName` | `STRING Inputs/outputs` | — |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful identification. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block exports a linearization table to a file in ASCI format.

##### `MC_LinTableExportToBinFile_BkPlcMc` — §4.4.9.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge initiates the export. |
| `FileName` | `STRING Inputs/outputs` | — |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful identification. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block exports a linearization table to a file in binary format.

##### `MC_LinTableImportFromAsciFile_BkPlcMc` — §4.4.9.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge initiates the import. |
| `FileName` | `STRING Inputs/outputs` | — |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful identification. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `LastIdx` | `INT Behavior of the function block` | — |

**Description:** The function block imports a linearization table from a file in ASCI format.

##### `MC_LinTableImportFromBinFile_BkPlcMc` — §4.4.9.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge initiates the import. |
| `FileName` | `STRING Inputs/outputs` | — |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | This indicates successful identification. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |
| `LastIdx` | `INT Behavior of the function block` | — |

**Description:** The function block imports a linearization table from a file in binary format.

#### PLCopen Motion Control › Parameter

##### `MC_AxAdsCommServer_BkPlcMc` — §4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFirstAxisIndex` | `INT` | This parameter is used to specify the dimensioning of the AXIS_REF_BkPlcMc |
| `nLastAxisIndex` | `INT` | [} 86] array. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pAxItf` | `POINTER TO AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PlcMcManOffline` | `BOOL` | This parameter corresponds to the parameter of the same name of the |

**Description:** nLastAxisIndex INT pAxItf Pointer To AXIS_REF_BkPlcMc

##### `MC_Communications_BkPlcMc` — §4.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFirstAxisIndex` | `INT` | These parameters correspond to the parameters of the same name of |
| `nLastAxisIndex` | `INT` | the MC_AxAdsCommServer_BkPlcMc function block. |
| `pAxItf` | `POINTER` | Here the address of the AXIS_REF_BkPlcMc [} 86] array is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PlcMcManOffline` | `BOOL` | This parameter corresponds to the parameter of the same name of the |

**Description:** The function block internally calls the function blocks MC_AxAdsCommServer_BkPlcMc [} 283] and MC_AxRtLoggerSpool_BkPlcMc [} 265]. It also provides a message buffer.

##### `MC_AxAdsPtrArrCommServer_BkPlcMc` — §4.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFirstAxisIndex` | `INT` | This parameter is used to specify the dimensioning of the |
| `nLastAxisIndex` | `INT` | AXIS_REF_BkPlcMc [} 86] array. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pAxItfArr` | `POINTER TO DWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PlcMcManOffline` | `BOOL` | This parameter corresponds to the parameter of the same name of the |

**Description:** nLastAxisIndex INT pAxItfArr Pointer To Pointer To AXIS_REF_BkPlcMc

##### `MC_AxAdsReadDecoder_BkPlcMc` — §4.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFirstAxisIndex` | `INT` | This parameter is used to specify the dimensioning of the |
| `nLastAxisIndex` | `INT` | AXIS_REF_BkPlcMc [} 86] array. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DeadManCount` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bClear` | `BOOL` | Indicates that an ADS access indicated with bValid should be acknowledged. |
| `bPending` | `BOOL` | Indicates that an ADS access indicated with bValid is being processed. Behavior of the function block |

**Description:** bReset BOOL bValid BOOL sNetId STRING(80) nPort UINT nInvokeId UDINT nIdxGrp UDINT nIdxOffs UDINT cbReadLen UDINT pAxItf Pointer To AXIS_REF_BkPlcMc

##### `MC_AxAdsWriteDecoder_BkPlcMc` — §4.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFirstAxisIndex` | `INT` | The dimensions of the AXIS_REF_BkPlcMc [} 86] array must be specified |
| `nLastAxisIndex` | `INT` | here. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DeadManCount` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bClear` | `BOOL` | Indicates that an ADS access indicated with bValid should be |

**Description:** bValid BOOL sNetId STRING(80) nPort UINT nInvokeId UDINT nIdxGrp UDINT nIdxOffs UDINT cbWriteLen UDINT pWriteBuff XWORD pAxItf Pointer To AXIS_REF_BkPlcMc

##### `MC_AxParamAuxLabelsLoad_BkPlcMc` — §4.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The loading process is initiated by a rising edge at this input. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Successful loading of the parameters is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block loads the label texts for the customer-specific axis parameters from a file. These texts can be generated with a simple text editor such as Microsoft Notepad.

##### `MC_AxParamLoad_BkPlcMc` — §4.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The loading process is initiated by a rising edge at this input. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Successful loading of the parameters is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block loads the parameters for an axis from a file. A function block of type MC_AxParamSave_BkPlcMc [} 292] must be used to generate a compatible parameter file.

##### `MC_AxParamSave_BkPlcMc` — §4.5.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The writing process is initiated by a rising edge at this input. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Successful writing of the parameters is indicated here. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block writes the parameters for an axis into a file. A function block of type MC_AxParamLoad_BkPlcMc [} 291] must be used to read the file.

##### `MC_AxUtiReadCoeDriveTerm_BkPlcMc` — §4.5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the read process. |
| `Pdata` | `POINTER` | Here, the address of the variable is specified, in which the read value is to be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful loading of the parameter is indicated here. |
| `CommandAborted` | `BOOL` | Indicates abortion of the read operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block reads the contents of a register from the EL terminal, which is used as drive interface for the axis.

##### `MC_AxUtiReadCoeEncTerm_BkPlcMc` — §4.5.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the read process. |
| `Pdata` | `POINTER` | Here, the address of the variable is specified, in which the read value is to be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful loading of the parameter is indicated here. |
| `CommandAborted` | `BOOL` | Indicates abortion of the read operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block reads the contents of a register from the EL terminal, which is used as encoder interface for the axis.

##### `MC_AxUtiReadRegDriveTerm_BkPlcMc` — §4.5.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the read process. |
| `Select` | `INT` | The register number should be transferred here. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RegData` | `WORD` | The read value is output here. |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful loading of the parameter is indicated here. |
| `CommandAborted` | `BOOL` | Indicates abortion of the read operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block reads the contents of a register from the KL terminal, which is used as drive interface for the axis.

##### `MC_AxUtiReadRegEncTerm_BkPlcMc` — §4.5.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the read process. |
| `Select` | `INT` | The register number should be transferred here. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RegData` | `WORD` | The read value is output here. |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Successful loading of the parameter is indicated here. |
| `CommandAborted` | `BOOL` | Indicates abortion of the read operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block reads the contents of a register from the KL terminal, which is used as encoder interface for the axis.

##### `MC_AxUtiUpdateRegDriveTerm_BkPlcMc` — §4.5.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The writing process is initiated by a rising edge at this input. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |
| `RegData` | `ST_TcPlcRegDataTable` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Indicates successful writing of the parameter. |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `CommandAborted` | `BOOL` | Indicates abortion of the read operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block writes a parameter set into the registers of a KL terminal. It uses MC_AxUtiReadRegDriveTerm_BkPlcMc [} 297] and MC_AxUtiWriteRegDriveTerm_BkPlcMc [} 306] function blocks for this purpose.

##### `MC_AxUtiUpdateRegEncTerm_BkPlcMc` — §4.5.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The writing process is initiated by a rising edge at this input. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |
| `RegData` | `ST_TcPlcRegDataTable` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Indicates successful writing of the parameter. |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `CommandAborted` | `BOOL` | Indicates abortion of the read operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block writes a parameter set into the registers of a KL terminal. It uses MC_AxUtiReadRegEncTerm_BkPlcMc [} 298] and MC_AxUtiWriteRegEncTerm_BkPlcMc [} 307] function blocks for this purpose.

##### `MC_AxUtiWriteCoeDriveTerm_BkPlcMc` — §4.5.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The writing process is initiated by a rising edge at this input. |
| `Pdata` | `POINTER` | The address of the variable whose content is to be written to the terminal |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Indicates successful writing of the parameter. |
| `CommandAborted` | `BOOL` | Indicates abortion of the read operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. |

**Description:** The function block writes the contents of a register of the EL terminal, which is used as drive interface for the axis.

##### `MC_AxUtiWriteCoeEncTerm_BkPlcMc` — §4.5.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The writing process is initiated by a rising edge at this input. |
| `Pdata` | `POINTER` | The address of the variable whose content is to be written to the terminal |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Indicates successful writing of the parameter. |
| `CommandAborted` | `BOOL` | Indicates abortion of the read operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block writes the contents of a register of the EL terminal, which is used as encoder interface for the axis.

##### `MC_AxUtiWriteRegDriveTerm_BkPlcMc` — §4.5.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The writing process is initiated by a rising edge at this input. |
| `Select` | `INT` | The register number should be transferred here. |
| `RegData` | `WORD` | The value to be written should be transferred here. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Indicates successful writing of the parameter. |
| `CommandAborted` | `BOOL` | Indicates abortion of the read operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block writes the contents of a register of the KL terminal, which is used as drive interface for the axis.

##### `MC_AxUtiWriteRegEncTerm_BkPlcMc` — §4.5.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The writing process is initiated by a rising edge at this input. |
| `Select` | `INT` | The register number should be transferred here. |
| `RegData` | `WORD` | The value to be written should be transferred here. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Done` | `BOOL` | Indicates successful writing of the parameter. |
| `CommandAborted` | `BOOL` | Indicates abortion of the read operation. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded indication of the cause of the error is provided here. Behavior of the function block |

**Description:** The function block writes the contents of a register of the KL terminal, which is used as encoder interface for the axis.

#### PLCopen Motion Control › Part 5 Homing › FinalizingFunctions

##### `MC_AbortHoming_BkPlcMc` — §4.6.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the abort. Inputs/outputs |
| `Axis` | `AXIS_REF_BkPlcMc` | Here, the address of a variable of type AXIS_REF_BkPlcMc [} 86] should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Successful processing is indicated here |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Active` | `BOOL` | Readiness for operation is indicated here. |
| `CommandAborted` | `BOOL` | Here an abort of the command is indicated. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded error message is provided here. |

**Description:** The function block is used to cancel a referencing process.

##### `MC_FinishHoming_BkPlcMc` — §4.6.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at this input starts the movement and terminates |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Successful processing is indicated here |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Active` | `BOOL` | Readiness for operation is indicated here. |
| `CommandAborted` | `BOOL` | Here an abort of the command is indicated. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded error message is provided here. Behavior of the function block |

**Description:** The function block is used to cancel a referencing process.

#### PLCopen Motion Control › Part 5 Homing › StepFunctions

##### `MC_StepAbsoluteSwitch_BkPlcMc` — §4.6.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | — |
| `Direction` | `MC_Direction_BkPlcMc` | — |
| `SwitchMode` | `MC_SwitchMode_BkPlcMc` | — |
| `ReferenceSignal` | `MC_Ref_Signal_Ref_BkPlcMc` | — |
| `Velocity` | `LREAL` | — |
| `Acceleration` | `LREAL` | — |
| `Deceleration` | `LREAL` | — |
| `Jerk` | `LREAL` | — |
| `SetPosition` | `LREAL` | — |
| `TorqueLimit` | `LREAL` | — |
| `TimeLimit` | `TIME` | — |
| `DistanceLimit` | `LREAL` | — |
| `BufferMode` | `MC_BufferMode_BkPlcMc:=Aborting_BkPlcMc` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Successful processing is indicated here |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Active` | `BOOL` | Readiness for operation is indicated here. |
| `CommandAborte` | `BOOL` | Here an abort of the command is indicated. d |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded error message is provided here. |

**Description:** The function block is used for referencing via a limit switch. The function block triggers a position setting internally after the cam is found.

##### `MC_StepAbsoluteSwitchDetection_BkPlcMc` — §4.6.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | — |
| `Direction` | `MC_Direction_BkPlcMc` | — |
| `SwitchMode` | `MC_SwitchMode_BkPlcMc` | — |
| `ReferenceSignal` | `MC_Ref_Signal_Ref_BkPlcMc` | — |
| `Velocity` | `LREAL` | — |
| `Acceleration` | `LREAL` | — |
| `Deceleration` | `LREAL` | — |
| `Jerk` | `LREAL` | — |
| `SetPosition` | `LREAL` | — |
| `TorqueLimit` | `LREAL` | — |
| `TimeLimit` | `TIME` | — |
| `DistanceLimit` | `LREAL` | — |
| `BufferMode` | `MC_BufferMode_BkPlcMc:=Aborting_BkPlcMc` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Successful processing is indicated here |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Active` | `BOOL` | Readiness for operation is indicated here. |
| `CommandAborted` | `BOOL` | Here an abort of the command is indicated. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded error message is provided here. |
| `RecordedPosition` | `LREAL` | [mm] Determined position of the referencing cam. |

**Description:** The function block is used for referencing via a limit switch. The function block outputs the position of the cam to the outside via RecordedPosition. No position is set.

##### `MC_StepBlock_BkPlcMc` — §4.6.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | — |
| `Direction` | `MC_Direction_BkPlcMc` | — |
| `Velocity` | `LREAL` | — |
| `Acceleration` | `LREAL` | — |
| `Deceleration` | `LREAL` | — |
| `Jerk` | `LREAL` | — |
| `SetPosition` | `LREAL` | — |
| `DetectionVelocityLimit` | `LREAL` | — |
| `DetectionVelocityTime` | `LREAL` | — |
| `TorqueLimit` | `LREAL` | — |
| `TorqueTolerance` | `LREAL` | — |
| `TimeLimit` | `TIME` | — |
| `DistanceLimit` | `LREAL` | — |
| `BufferMode` | `MC_BufferMode_BkPlcMc:=Aborting_BkPlcMc` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Successful processing is indicated here |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Active` | `BOOL` | Readiness for operation is indicated here. |
| `CommandAborted` | `BOOL` | Here an abort of the command is indicated. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded error message is provided here. |

**Description:** The function block is used for referencing via a fixed stop. The function block triggers a position setting internally after the cam is found.

##### `MC_StepBlockDetection_BkPlcMc` — §4.6.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | — |
| `Direction` | `MC_Direction_BkPlcMc` | — |
| `Velocity` | `LREAL` | — |
| `Acceleration` | `LREAL` | — |
| `Deceleration` | `LREAL` | — |
| `Jerk` | `LREAL` | — |
| `SetPosition` | `LREAL` | — |
| `DetectionVelocityLimit` | `LREAL` | — |
| `DetectionVelocityTime` | `LREAL` | — |
| `TorqueLimit` | `LREAL` | — |
| `TorqueTolerance` | `LREAL` | — |
| `TimeLimit` | `TIME` | — |
| `DistanceLimit` | `LREAL` | — |
| `BufferMode` | `MC_BufferMode_BkPlcMc:=Aborting_BkPlcMc` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**Description:** The function block is used for referencing via a limit switch. The function block outputs the position of the cam to the outside via RecordedPosition. No position is set.

##### `MC_StepReferencePulse_BkPlcMc` — §4.6.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | — |
| `Direction` | `MC_Direction_BkPlcMc` | — |
| `Velocity` | `LREAL` | — |
| `Acceleration` | `LREAL` | — |
| `Deceleration` | `LREAL` | — |
| `Jerk` | `LREAL` | — |
| `SetPosition` | `LREAL` | — |
| `TorqueLimit` | `LREAL` | — |
| `TimeLimit` | `TIME` | — |
| `DistanceLimit` | `LREAL` | — |
| `BufferMode` | `MC_BufferMode_BkPlcMc:=Aborting_BkPlcMc` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Successful processing is indicated here |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Active` | `BOOL` | Readiness for operation is indicated here. |
| `CommandAborted` | `BOOL` | Here an abort of the command is indicated. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded error message is provided here. Behavior of the function block |

**Description:** The function block is used to reference an incremental position measuring system. An actual value setting is carried out internally via the referencing pulse.

##### `MC_StepReferencePulseDetection_BkPlcMc` — §4.6.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The homing is initiated by a rising edge at this input. |
| `Direction` | `MC_Direction_BkPlcMc` | The direction is specified via MC_Direction_BkPlcMc |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF_BkPlcMc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Successful processing is indicated here |
| `Busy` | `BOOL` | Indicates that a command is being processed. |
| `Active` | `BOOL` | Readiness for operation is indicated here. |
| `CommandAborted` | `BOOL` | Here an abort of the command is indicated. |
| `Error` | `BOOL` | The occurrence of an error is indicated here. |
| `ErrorID` | `UDINT` | An encoded error message is provided here. |
| `RecordedPosition` | `LREAL` | [mm] Determined position at the reference pulse. Behavior of the function block |

**Description:** The function block is used to reference an incremental position measuring system. The function block outputs the position of the reference pulse to the outside via RecordedPosition. No position is set.
