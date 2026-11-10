# Tc2_MC2_Drive

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_MC2_Drive_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/NC2 · **Tier:** T3 · **API entries:** 41
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** MC function blocks for drives: axis backend + drive profile handling.

---

#### (top level)

##### `F_GetVersionTcMc2Drive` (FUNCTION) → `UINT` — §3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | — |

**Description:** This function can be used to read PLC library version information.

#### Function blocks › General Beckhoff › Function blocks for determining current values

##### `FB_ReadDriveParameter` — §4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is activated by a positive edge at this input. |
| `ParameterNumb` | `E_DriveParameterNumbe Specifies which parameter is to be read.` | — |
| `er` | `r [} 74]` | — |
| `Mode` | `E_ReadWriteMode` | Specifies whether the parameter is to be read once or cyclically [} 77] (as quickly as possible). |
| `Interval` | `TIME` | In addition to E_ReadWriteMode.Cyclic, an interval time can be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | This output is set if valid information has been written to the |

**Description:** The FB_ReadDriveParameter function block can be used to read various parameters of the Beckhoff servo hardware asynchronously.

##### `FB_WriteDriveParameter` — §4.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | — |
| `ParameterNumber` | `E_DriveParameterNumber` | — |
| `Mode` | `E_ReadWriteMode` | — |
| `Interval` | `TIME` | — |
| `Value` | `ANY` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | This output is set when valid information has been written to the |

**Description:** The FB_WriteDriveParameter function block can be used to write various parameters of the Beckhoff servo hardware asynchronously.

#### Function blocks › General Beckhoff

##### `FB_DeletePositionOffset` — §4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Feedback` | `E_PositionOffsetFeedback Specifies which feedback is considered. [} 72]` | — |
| `Memory` | `E_PositionOffsetMemory Specifies the memory from which the position offset is to be [} 72]` | deleted. Inputs/outputs |
| `Axis` | `AXIS_REF` | Axis data structure that unambiguously addresses an axis in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The FB_DeletePositionOffset function block can be used to delete a position offset in the memory of a Beckhoff servo hardware.

##### `FB_BrakeControl` — §4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Mode` | `E_BrakeMode [} 72]` | The mode with which the brake is controlled. Inputs/outputs |
| `Axis` | `AXIS_REF` | Axis data structure that unambiguously addresses an axis in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** Function blocks | General Beckhoff

##### `FB_SetPositionOffset` — §4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Position` | `LREAL` | New actual position of the NC axis |
| `Relative` | `BOOL` | When this flag is set, the position is interpreted relatively. |
| `Feedback` | `E_PositionOffsetFeedback Specifies which feedback is considered. [} 72]` | — |
| `Memory` | `E_PositionOffsetMemory Specifies which memory the newly calculated position offset [} 72]` | should be stored in. Inputs/outputs |
| `Axis` | `AXIS_REF` | Axis data structure that unambiguously addresses an axis in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | — |
| `Error` | `BOOL` | — |
| `ErrorID` | `UDINT` | — |

**Description:** The FB_SetPositionOffset function block can be used to store a position offset in the memory of a Beckhoff servo hardware. Saving in the digital nameplate of the connected motor encoder is also possible. To do this, the offset must first be configured with the DriveManager and then the same memory location (encoder / drive) must be used in the function block.

##### `FB_ReadDriveInfo` — §4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. Inputs/outputs |
| `Axis` | `AXIS_REF` | Axis data structure that unambiguously addresses an axis in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**Description:** The FB_ReadDriveInfo function block can be used to read basic information for communication with a Beckhoff servo hardware.

##### `FB_ParkAxis` — §4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Mode` | `E_ParkMode [} 73] [} 74]` | Mode for selecting whether the FB parks or releases the axis. Inputs/outputs |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The function block FB_ParkAxis enables the parking functionality of a supported Beckhoff drive. A parked channel of a drive is temporarily disabled.

#### Function blocks › General SoE

##### `FB_SoERead` — §4.2.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the FB_SoERead function block a parameter can be read.

##### `FB_SoEWrite` — §4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the FB_SoEWrite function block a parameter can be written.

##### `FB_SoEReset` — §4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** The drive (S-0-0099) can be reset with the function block FB_SoEReset. In the case of multiple-channel devices if necessary, both channels will have to perform a reset. The timeout time must be 10 s, as the reset can take up to 10 s depending on the error. The flag "Wait For WcState is OK" must be enabled in the advanced EtherCAT settings for the AX5000.

##### `FB_SoEWritePassword` — §4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the FB_SoEWritePassword function block (S-0-0267) the drive password can be set.

##### `FB_SoESetDataAccessMode` — §4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** The SoE parameter access mode is set with the function block FB_SoESetDataAccessMode. Attributes & Value are accessed in parallel by default. If this access mode is not supported by the device manufacturer, sequential access can be enforced. Generally, however, parallel access is faster.

#### Function blocks › General SoE › Function blocks for command

##### `FB_SoEExecuteCommand` — §4.2.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the FB_SoEExecuteCommand function block a command can be executed.

##### `FB_SoEWriteCommandControl` — §4.2.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the FB_SoEWriteCommandControl function block a command can be prepared, started or canceled.

##### `FB_SoEReadCommandState` — §4.2.6.3

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the FB_SoEReadCommandState function block the command execution can be checked.

#### Function blocks › General SoE › Function blocks for diagnostics

##### `FB_SoEReadDiagMessage` — §4.2.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the FB_SoEReadDiagMessage function block the diagnosis message can be read out as a Sercos string (S-0-0095).

##### `FB_SoEReadDiagNumber` — §4.2.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** Function blocks | General SoE

##### `FB_SoEReadDiagNumberList` — §4.2.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `busy` | `BOOL` | — |
| `Error` | `BOOL` | — |
| `AdsErrId` | `UINT` | — |
| `SercosErrId` | `UINT` | — |
| `Attribute` | `DWORD` | — |

**Description:** With the FB_SoEReadDiagNumberList function block a history of the diagnosis numbers can be read out as a list (S-0-0375).

##### `FB_SoEReadClassXDiag` — §4.2.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the function block FB_SoEReadClassXDiag, the current Class 1 diagnosis (S-0-0011) ... Class 3 diagnosis (S-0-0013) can be read out as WORD. There is the conversion function F_ConvWordToSTAX5000C1D for the evaluation of the Class 1 diagnosis as a structure ST_AX5000_C1D, (see TwinCAT 3 PLC Lib Tc2_Drive documentation).

#### Function blocks › General SoE › Function blocks for determining current values

##### `FB_SoEReadAmplifierTemperature` — §4.2.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**Description:** With the FB_SoEReadAmplifierTemperature function block the temperature of the drive (S-0-0384) can be read.

##### `FB_SoEReadMotorTemperature` — §4.2.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the function block FB_SoEReadMotorTemperature the temperature of the motor (S-0-0383) can be read. If the motor does not contain a temperature sensor, this is 0.0, i.e. 0.0 °C.

##### `FB_SoEReadDcBusCurrent` — §4.2.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the FB_SoEAX5000ReadDcBusCurrent function block the DC-Bus current (S-0-0381) can be read.

##### `FB_SoEReadDcBusVoltage` — §4.2.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**Description:** With the FB_SoEReadDcBusVoltage function block the Dc-Bus voltage of the drive (S-0-0380) can be read.

#### Function blocks › General CoE

##### `FB_CoERead` — §4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `cancelled.*)` | `` | — |
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** The function block FB_CoERead allows data to be read from an object directory of an EtherCAT slave through an SDO (Service Data Object) access. This requires the slave to have a mailbox and to support the CoE (CANopen over EtherCAT) protocol. With the help of the SubIndex and Index parameters a selection is made as to which object should be read out. Via CompleteAccess := TRUE the parameter can be read with sub-elements.

##### `FB_CoEWrite` — §4.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID;(*netID of PC with NC*)` | — |
| `Index` | `WORD;(*CoE object index*)` | — |
| `SubIndex` | `BYTE;(*CoE sub index*)` | — |
| `pSrcBuf` | `PVOID;(*Contains the address of the buffer containing the data to be send*)` | — |
| `BufLen` | `UDINT;(*Contains the max. number of bytes to be received*)` | — |
| `Execute` | `BOOL;(*Function block execution is triggered by a rising edge at this input.*)` | — |
| `Timeout` | `TIME := DEFAULT_ADS_TIMEOUT` | — |
| `CompleteAccess` | `BOOL;(*Function block reads the complete object with all sub index*)` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the function block FB_CoEWrite, an object from the object directory of an EtherCAT slave can be written via an SDO (Service Data Object) download. This requires the slave to have a mailbox and to support the CoE (CANopen over EtherCAT) protocol. With the help of the SubIndex and Index parameters a selection is made as to which object should be written. Via CompleteAccess := TRUE the parameter can be written with sub-elements.

##### `FB_CoEExecuteCommand` — §4.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** pDstBuf PVOID

#### Function blocks › AX5000 SoE

##### `FB_SoEAX5000ReadActMainVoltage` — §4.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the FB_SoEAX5000ReadActMainVoltage function block the current peak value of the mains voltage of the AX5000 (P-0-0200) can be read.

##### `FB_SoEAX5000SetMotorCtrlWord` — §4.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetId` | `T_AmsNetID` | String containing the AMS network ID of the PC. |
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the function block FB_SoEAX5000SetMotorCtrlWord the ForceLock bit (Bit 0) or the ForceUnlock bit can be set in the Motor Control Word (P-0-0096) to activate or release the brake. Normally the brake is automatically controlled via the Enable of the drive.

##### `FB_SoEAX5000FirmwareUpdate` — §4.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `0009*) \TwinCAT\Io\TcDriveManager\FirmwarePool" *)` | `` | — |
| `NetId` | `T_AmsNetID` | AMS-NetID of the controller (IPC) |
| `bExecute` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `10_v1_05_b0009.efw" *)` | `` | — |
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the FB_SoEAX5000FirmwareUpdate function block the Firmware of the AX5000 can be checked and automatically changed to a given version (Revision and Build) or to the current Build of the configured revision.

##### `FB_SoEAX5000SetPositionOffset` — §4.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the function block FB_SoEAX5000SetPositionOffset a position offset can be saved in the memory of the AX5000 or of the digital nameplate of the connected motor encoder. To do this, the offset must first be configured with the DriveManager and then the same memory location (encoder / drive) must be used in the function block.

##### `FB_SoEAX5000DeletePositionOffset` — §4.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, |

**Description:** With the function block FB_SoEAX5000DeletePositionOffset, a position offset can be deleted from the memory of the AX5000 or from the digital nameplate of the connected motor encoder.

##### `FB_SoEAX5000ParkAxis` — §4.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Mode` | `E_ParkMode [} 73]` | Mode for selecting whether the FB parks or releases the axis. Inputs/outputs |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** This function block enables the parking functionality of an AX5000. A parked channel of an AX5000 is temporarily disabled.

#### Function blocks › AX8000 CoE

##### `FB_CoEAX8000BrakeControl` — §4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Mode` | `E_BrakeMode [} 72] 60` | The mode with which the brake is controlled. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** This function block can be used to manually open or close the brake of a motor that is operated on an AX8000.

##### `FB_CoEAX8000BrakeTest` — §4.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Succeeded` | `BOOL` | Feedback that the brake has held the specified torque. This |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**Description:** The function block FB_CoEAX8000BrakeTest can be used to implement a functional brake test.

##### `FB_CoEAX8000SetPositionOffset` — §4.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Position` | `LREAL` | New actual position of the NC axis |
| `Relative` | `BOOL` | When this flag is set, the position is interpreted relatively. |
| `Feedback` | `E_PositionOffsetFeedback [} 72]` | Specifies which feedback is considered. |
| `Memory` | `E_PositionOffsetMemory` | Specifies which memory the newly calculated position offset [} 72] should be stored in. Inputs/outputs |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_CoEAX8000SetPositionOffset function block, a position offset can be saved in the memory of the AX8000 or of the digital nameplate of the connected motor encoder. To do this, the offset must first be configured with the DriveManager and then the same memory location (encoder / drive) must be used in the function block.

##### `FB_CoEAX8000DeletePositionOffset` — §4.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Position` | `LREAL` | New position of the NC axis is displayed. |
| `Relative` | `BOOL` | When this flag is set, the position is interpreted relatively. |
| `Feedback` | `E_PositionOffsetFeedback Specifies which feedback is considered. [} 72]` | — |
| `Memory` | `E_PositionOffsetMemory Specifies the memory from which the position offset is to be [} 72] 64` | deleted. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the function block FB_CoEAX8000DeletePositionOffset, a position offset can be deleted from the memory of the AX8000 or from the digital nameplate of the connected motor encoder.

##### `FB_CoEAX8000ParkAxis` — §4.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Mode` | `E_ParkMode [} 73] [} 74]` | Mode for selecting whether the FB parks or releases the axis. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** This function block enables the parking functionality of an AX8000. A parked channel of an AX8000 is temporarily disabled.

#### Function blocks › EL72xx CoE

##### `FB_CoEEL72xxBrakeControl` — §4.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL 66` | The function block is enabled via a positive edge at this input. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** This function block can be used to manually open or close the brake of a motor that is operated on an EL72xx.

##### `FB_CoEEL72xxSetPositionOffset` — §4.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | — |
| `Position` | `LREAL` | — |
| `Relative` | `BOOL` | — |
| `Feedback` | `E_PositionOffsetFeedback` | — |
| `Memory` | `E_PositionOffsetMemory` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the function block FB_CoEEL72xxSetPositionOffset, a position offset can be saved in the memory of the EL72xx. Saving in the digital nameplate of the connected motor encoder is currently not planned.

##### `FB_CoEEL72xxDeletePositionOffset` — §4.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The function block is enabled via a positive edge at this input. |
| `Position` | `LREAL` | New position of the NC axis is displayed. |
| `Relative` | `BOOL` | When this flag is set, the position is interpreted relatively. |
| `Feedback` | `E_PositionOffsetFeedback Specifies which feedback is considered. [} 72]` | — |
| `Memory` | `E_PositionOffsetMemory Specifies the memory from which the position offset is to be [} 72]` | deleted. Inputs/outputs |
| `Axis` | `AXIS_REF` | Axis data structure that unambiguously addresses an axis in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the function block FB_CoEEL72xxDeletePositionOffset, a position offset can be deleted from the memory of the EL72xx.
