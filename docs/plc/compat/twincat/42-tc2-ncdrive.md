# Tc2_NcDrive

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_NcDrive_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/NC2 · **Tier:** T3 · **API entries:** 21
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** NC drive function blocks: axis backend.

---

#### Function blocks › General SoE

##### `FB_SoEReset` — §3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The drive (S-0-0099) can be reset with the function block FB_SoEReset. In the case of multiple-channel devices if necessary, both channels will have to perform a reset. The timeout time must be 10 s, as the reset can take up to 10 s depending on the error. The flag "Wait For WcState is OK" must be enabled in the advanced EtherCAT settings for the AX5000.

##### `FB_SoEWritePassword` — §3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEWritePassword function block (S-0-0267) the drive password can be set.

#### Function blocks › General SoE › Function blocks for commands

##### `FB_SoEExecuteCommand` — §3.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEExecuteCommand function block a command can be executed.

##### `FB_SoEWriteCommandControl` — §3.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** tTimeout TIME

##### `FB_SoEReadCommandState` — §3.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadCommandState function block the command execution can be checked.

#### Function blocks › General SoE › Function blocks for diagnostics

##### `FB_SoEReadDiagMessage` — §3.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadDiagMessage function block the diagnosis message can be read out as a Sercos string (S-0-0095).

##### `FB_SoEReadDiagNumber` — §3.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadDiagNumber function block the current diagnostic number can be read out as UDINT (S-0-0390).

##### `FB_SoEReadDiagNumberList` — §3.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadDiagNumberList function block a history of the diagnosis numbers can be read out as a list (S-0-0375).

##### `FB_SoEReadClassXDiag` — §3.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the function block FB_SoEReadClassXDiag, the current Class 1 diagnosis (S-0-0011) ... Class 3 diagnosis (S-0-0013) can be read out as WORD. There is the conversion function F_ConvWordToSTAX5000C1D for the evaluation of the Class 1 diagnosis as a structure ST_AX5000_C1D, (see TwinCAT 3 PLC Lib Tc2_Drive documentation).

#### Function blocks › General SoE › Function blocks for determining current values

##### `FB_SoERead` — §3.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** bExecute BOOL tTimeout TIME

##### `FB_SoEWrite` — §3.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** cbBufLen UDINT bExecute BOOL tTimeout TIME sPassword ST_SoE_String

##### `FB_SoEReadAmplifierTemperature` — §3.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `26 Function blocks` | `` | — |
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadAmplifierTemperature function block the temperature of the drive (S-0-0384) can be read.

##### `FB_SoEReadMotorTemperature` — §3.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the function block FB_SoEReadMotorTemperature the temperature of the motor (S-0-0383) can be read. If the motor does not contain a temperature sensor, this is 0.0, i.e. 0.0 °C.

##### `FB_SoEReadDcBusCurrent` — §3.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the function block FB_SoEAX5000ReadDcBusCurrent the DC-Bus current (S-0-0381) can be read.

##### `FB_SoEReadDcBusVoltage` — §3.1.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEReadDcBusVoltage function block the Dc-Bus voltage of the drive (S-0-0380) can be read.

#### Function blocks › General CoE › Function blocks for determining current values

##### `FB_CoERead` — §3.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId;(*netID of PC with NC*)` | — |
| `iIndex` | `WORD;(*CoE object index*)` | — |
| `iSubIndex` | `BYTE;(*CoE sub index*)` | — |
| `pDstBuf` | `PVOID;(*Contains the address of the buffer for the received data*)` | — |
| `iBufLen` | `UDINT;(*Contains the max. number of bytes to be received*)` | — |
| `bExecute` | `BOOL;(*Function block execution is triggered by a rising edge at this input.*)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT` | — |
| `bCompleteAccess` | `BOOL;(*Function block reads the complete object with all sub index*)` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** iBufLen UDINT bExecute BOOL tTimeout TIME bCompleteAccess BOOL

##### `FB_CoEWrite` — §3.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId;(*netID of PC with NC*)` | — |
| `iIndex` | `WORD;(*CoE object index*)` | — |
| `iSubIndex` | `BYTE;(*CoE sub index*)` | — |
| `pDstBuf` | `PVOID;(*Contains the address of the buffer for the received data*)` | — |
| `iBufLen` | `UDINT;(*Contains the max. number of bytes to be received*)` | — |
| `bExecute` | `BOOL;(*Function block execution is triggered by a rising edge at this input.*)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT` | — |
| `bCompleteAccess` | `BOOL;(*Function block reads the complete object with all sub index*)` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** iBufLen UDINT bExecute BOOL tTimeout TIME bCompleteAccess BOOL

#### Function blocks › AX5000 SoE

##### `FB_SoEAX5000ReadActMainVoltage` — §3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEAX5000ReadActMainVoltage function block the current peak value of the mains voltage of the AX5000 (P-0-0200) can be read.

##### `FB_SoEAX5000SetMotorCtrlWord` — §3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String, which contains the AMS Network ID of the PC |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** bForceUnlock BOOL

##### `FB_SoEAX5000FirmwareUpdate` — §3.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `0009*) \TwinCAT\Io\TcDriveManager\FirmwarePool" *)` | `` | — |
| `sNetId` | `T_AmsNetId` | AMS-NetID of the controller (IPC). |
| `bExecute` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `NCTOPLC_AXIS_REF;(* reference to NC axis *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `1_05_b0009.efw" *)` | `` | — |
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** With the FB_SoEAX5000FirmwareUpdate function block the Firmware of the AX5000 can be checked and automatically changed to a given version (Revision and Build) or to the current Build of the configured revision.

#### Function blocks

##### `F_GetVersionTcNcDrive` (FUNCTION) → `UINT` — §3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | — |

**Description:** This function can be used to read PLC library version information.
