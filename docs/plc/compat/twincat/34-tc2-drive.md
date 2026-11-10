# Tc2_Drive

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_Drive_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/Drive · **Tier:** T3 · **API entries:** 26
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Drive-specific function blocks: axis backend + CiA-402-style drive profiles (Tether DS402 work is directly relevant).

---

#### Function blocks › General SoE

##### `FB_SoEReset_ByDriveRef` — §4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The drive (S-0-0099) can be reset with the function block FB_SoEReset_ByDriveRef. In the case of multiple- channel devices if necessary, both channels will have to perform a reset. The timeout time must be 10 s, as the reset can take up to 10 s depending on the error. An NC reset will not be performed.

##### `FB_SoEWritePassword_ByDriveRef` — §4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** Using the FB_SoEWritePassword_ByDriveRef function block, the drive password (S-0-0267) can be set.

##### `FB_SoEExecuteCommand_ByDriveRef` — §4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEExecuteCommand_ByDriveRef function block a command can be executed.

#### Function blocks › General SoE › Function blocks for commands

##### `FB_SoEWriteCommandControl_ByDriveRef` — §4.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** tTimeout TIME

##### `FB_SoEReadCommandState_ByDriveRef` — §4.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadCommandState_ByDriveRef function block, the execution of the command can be checked.

#### Function blocks › General SoE › Function blocks for diagnostics

##### `FB_SoEReadDiagMessage_ByDriveRef` — §4.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadDiagMessage_ByDriveRef function block the diagnosis message can be read as a Sercos String (S-0-0095).

##### `FB_SoEReadDiagNumber_ByDriveRef` — §4.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadDiagNumber_ByDriveRef function block the current diagnostic number can be read as UDINT (S-0-0390).

##### `FB_SoEReadDiagNumberList_ByDriveRef` — §4.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadDiagNumberList_ByDriveRef function block a history of the diagnostic numbers can be read as a list (S-0-0375).

##### `FB_SoEReadClassXDiag_ByDriveRef` — §4.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `24 Function blocks` | `` | — |
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadClassXDiag_ByDriveRef function block the current class 1 diagnosis (S-0-0011) ... class 3 diagnosis (S-0-0013) can be read as WORD. For the evaluation of the class 1 diagnosis as structure ST_AX5000_C1D [} 49] there is a conversion function F_ConvWordToSTAX5000C1D [} 30].

#### Function blocks › General SoE › Function blocks for determining current values

##### `FB_SoEReadAmplifierTemperature_ByDriveRef` — §4.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** Function blocks

##### `FB_SoEReadMotorTemperature_ByDriveRef` — §4.1.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the function block FB_SoEReadMotorTemperature_ByDriveRef the temperature of the motor (S-0-0383) can be read. If the motor does not contain a temperature sensor, this is 0.0, i.e. 0.0 °C.

##### `FB_SoEReadDcBusVoltage_ByDriveRef` — §4.1.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadDcBusVoltage_ByDriveRef function block, the DC-Bus voltage of the drive (S-0-0380) can be read.

##### `FB_SoEReadDcBusCurrent_ByDriveRef` — §4.1.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEAX5000ReadDcBusCurrent_ByDriveRef function block the DC-Bus current (S-0-0381) can be read.

#### Function blocks › AX5000 SoE › Conversion functions

##### `F_ConvWordToSTAX5000C1D` (FUNCTION) → `ST_AX5000_C1D` — §4.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wClass1D WORD` | `Class 1 diagnosis Word from S-0-0011 (see` | — |
| `iag` | `FB_SoEReadClassXDiag_ByDriveRef [} 24])` | — |

**Description:** 30

##### `F_ConvWordToSTAX5000DriveStatus` (FUNCTION) → `ST_AX5000DriveStatus` — §4.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wDriveSta WORD` | `Drive status word from S-0-0135 (can be read with` | — |
| `tus` | `FB_SoE_Read_ByDriveRef, can be mapped if necessary).` | — |

**Description:** With this function the Drive status word (S-0-0135) can be changed to a structure ST_AX5000DriveStatus [} 49].

#### Function blocks › AX5000 SoE

##### `FB_SoEAX5000ReadActMainVoltage_ByDriveRef` — §4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The current peak value of the mains voltage for the AX5000 (P-0-0200) can be read with the FB_SoEAX5000ReadActMainVoltage_ByDriveRef function block.

##### `FB_SoEAX5000SetMotorCtrlWord_ByDriveRef` — §4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** bForceUnlock BOOL

##### `FB_SoEAX5000FirmwareUpdate_ByDriveRef` — §4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `0009*) \TwinCAT\Io\TcDriveManager\FirmwarePool" *)` | `` | — |
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `_v1_05_b0009.efw" *)` | `` | — |
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEAX5000FirmwareUpdate_ByDriveRef function block the Firmware of the AX5000 can be checked and automatically changed to a given version (Revision and Build) or to the current Build of the configured revision.

##### `FB_SoEAX5000ParkAxis_ByDriveRef` — §4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDriveRef` | `ST_DriveRef` | Reference to the drive. The reference to the drive can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The parking functionality of an AX5000 is activated with the FB_SoEAX5000ParkAxis_ByDriveRef function block. A parked channel of an AX5000 is temporarily disabled.

#### Function blocks › IndraDrive Cs › Conversion functions

##### `F_ConvWordToSTIndraDriveCsC1D` (FUNCTION) → `ST_IndraDriveCs_C1D` — §4.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wClass1D WORD` | `Class 1 diagnosis Word from S-0-0011` | — |
| `iag` | `(see FB_SoEReadClassXDiag_ByDriveRef [} 24]).` | — |

**Description:** With this function the Class 1 diagnosis FB_SoEReadClassXDiag_ByDriveRef [} 24] (S-0-0011) can be changed to an ST_IndraDriveCs_C1D [} 50] structure.

##### `F_ConvWordToSTIndraDriveCsDriveStatus` (FUNCTION) → `ST_IndraDriveCsDriveStatus` — §4.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wClass1D WORD` | `Drive status word from S-0-0135` | — |
| `iag` | `Can be read with FB_SoE_Read_ByDriveRef, can be mapped if` | — |

**Description:** With this function the Drive status word (S-0-0135) can be changed to a structure ST_IndraDriveCsDriveStatus [} 51] .

#### Function blocks

##### `F_GetVersionTcDrive` (FUNCTION) → `UINT` — §4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionE INT` | `nVersionElement: version element to be read.` | — |
| `lement` | `Possible parameters:` | — |

**Description:** This function can be used to read PLC library version information.

#### Function blocks › SimplePlcMotion

##### `FB_CoEDriveEnable` — §4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `40 Function blocks` | `` | — |
| `bEnable` | `BOOL` | Activates the CoE drive. |
| `bReset` | `BOOL` | Performs a drive reset in the event of a fault. "Bit 7" is set |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCoeDriveIoInterface` | `ST_CoeDriveIoInterface` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bStatus` | `BOOL` | If bStatus=TRUE, the drive is ready for operation and |

**Description:** The FB_CoEDriveEnable function block enables a CoE drive in order to be able to subsequently supply it with setpoints using the function block FB_CoEDriveMoveVelocity [} 41].

##### `FB_CoEDriveMoveVelocity` — §4.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bEnable` | `BOOL` | Activates the setpoint generation. |
| `fVelocity` | `LREAL` | Target velocity. fVelocity can be changed during operation. |
| `fAcceleration1` | `LREAL` | Acceleration 1 is used below the parameterized velocity |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCoEDriveIoInterface` | `ST_CoeDriveIoInterface` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bInVelocity` | `BOOL` | Target velocity is reached. |
| `bBusy` | `BOOL` | bBusy is TRUE as long as the function block is active and |

**Description:** fDeceleration2 LREAL bNegativeDirection BOOL stOptions ST_DriveMoveVelocityOptions

##### `FB_SoEDriveEnable` — §4.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Activates the SoE drive. |
| `/` | `Inputs/Outputs` | — |
| `stSoeDriveIoInterfac ST_SoEDriveIoInterface` | `Data structure to which the process image of the SoE drive` | — |
| `e` | `[} 53]` | must be linked. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSoeDriveIoInterface` | `ST_SoEDriveIoInterface` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bStatus` | `BOOL` | If bStatus=TRUE, the drive is ready for operation and |

**Description:** The FB_SoEDriveEnable function block enables a SoE drive in order to be able to subsequently supply it with setpoints using the function block FB_SoEDriveMoveVelocity [} 43].

##### `FB_SoEDriveMoveVelocity` — §4.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Activates the setpoint generation. |
| `fVelocity` | `LREAL` | Target velocity. fVelocity can be changed during operation. |
| `fAcceleration1` | `LREAL` | Acceleration 1 is used below the parameterized velocity |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSoEDriveIoInterface` | `ST_SoeDriveIoInterface` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bInVelocity` | `BOOL` | Target velocity is reached. |
| `bBusy` | `BOOL` | bBusy is TRUE as long as the function block is active and |

**Description:** fDeceleration2 LREAL bNegativeDirection BOOL stOptions ST_DriveMoveVelocityOptions
