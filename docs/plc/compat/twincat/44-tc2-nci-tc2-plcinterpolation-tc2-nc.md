# Tc2_NCI+Tc2_PlcInterpolation+Tc2_NC

- **Manual:** `TF5100_TC3_NC_I_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/NC2 · **Tier:** T3 · **API entries:** 103
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** PLC interpolation + NC kernel: trajectory/spline engine on the axis backend.

---

#### GST Reference Manual › G-Code (DIN 66025)

##### `R-Parameters` (FUNCTION) → `LREAL` — §5.4.16

**Return type:** `LREAL`

**Description:** The arithmetic parameters, for short known as R-parameters, are interpreter variables that are named by an expression of the form “R<n>”. Since ‘n’ is an integer in the range 0..999, a total of 1000 R-parameters are available. The first 900 values R0..R899 of these are local variables for the NC channel. They can only be accessed by the interpreter of the channel. The R-parameters R900..R999 are declared globally. They exist only once for each NC, and all channels access the same storage. This kind of accessibility organization makes it possible to exchange data (e.g. for part tracing, collision avoidance etc.) beyond channel boundaries.

#### GST Reference Manual › CNC Functions

##### `Programming reference` (FUNCTION) — §5.6.21

**Description:** programmingReferenceSet

#### GST Reference Manual › Transformations

##### `Restoration of Stack` (FUNCTION) — §5.7.5

**Description:** The function transDepth() yields the current depth of the stack. The function transRestore(depth) removes transformations from the stack until the given depth is reached. Typically, the two functions are combined to save and restore the state of the transformation stack.

#### radius (added to the radius value of column 4) › PLC Library: Tc2_NCI › Configuration

##### `CfgBuild3DGroup` — §7.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nGroupId` | `UDINT` | ID of the 3D group |
| `nXAxisId` | `UDINT` | ID of the PTP axes |
| `nYAxisId` | `UDINT` | ID of the PTP axes |
| `nZAxisId` | `UDINT` | ID of the PTP axes |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nYAxisId UDINT nZAxisId UDINT tTimeOut TIME

##### `CfgBuildExt3DGroup` — §7.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is executed when a positive edge is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nYAxisId UDINT nZAxisId UDINT nQ1AxisId UDINT nQ2AxisId UDINT nQ3AxisId UDINT nQ4AxisId UDINT nQ5AxisId UDINT tTimeOut TIME

##### `CfgAddAxisToGroup` — §7.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nGroupId` | `UDINT` | ID of the target group |
| `nAxisId` | `UDINT` | ID of the axis to be configured |
| `nIndex` | `UDINT` | Position of the axis within the group; can have values |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nIndex UDINT tTimeOut TIME

##### `CfgReconfigGroup` — §7.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nGroupId` | `UDINT` | ID of the group to be resolved |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `nErrId` | `UDINT` | — |

**Description:** The function block CfgReconfigGroup removes the axis allocation of an existing group (NCI or FIFO), returning the axes to their personal PTP group.

##### `CfgReconfigAxis` — §7.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nAxisId` | `UDINT` | ID of the axis to be returned |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block CfgReconfigAxis returns a single axis from, for example, a 3D group, to its personal PTP group.

##### `CfgRead3DAxisIds` — §7.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nGroupId` | `UDINT` | ID of the 3D group |
| `pAddr` | `PVOID` | Address of the variable into which the function block writes the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `CfgReadExt3DAxisIds` — §7.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nGroupId` | `UDINT` | ID of the 3D group |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `stExt3dGroup` | `NCI_EXT3DGROUP` | Instance of the structure NCI_EXT3DGROUP (enter axis IDs of the current interpolation group here) |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stExt3dGroup` | `NCI_EXT3DGROUP` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block CfgReadExt3DAxisIds reads the axis configuration of the extended 3D group.

#### radius (added to the radius value of column 4) › PLC Library: Tc2_NCI › NCI POUs

##### `ItpConfirmHsk` — §7.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |
| `sPlcToNci` | `PLCTONC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpConfirmHsk confirms the currently present M-function.

##### `ItpDelDtgEx` — §7.1.2.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpDelDtgEx triggers "Delete distance to go". There is a more detailed description in the Interpreter [} 159] documentation.

##### `ItpEnableDefaultGCode` — §7.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `bUseDefaultGCode BOOL` | `If this variable is TRUE, the default G-Code is activated` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpEnableDefaultGCode enables execution of a user-defined G-Code before the start of each NC program from the PLC. The default program is executed before the loaded program when the actual NC program starts.

##### `ItpEStopEx` — §7.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `fDec` | `LREAL` | Max. deceleration during stopping. If fDec is smaller than the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpGetBlockNumber` — §7.1.2.5

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetBlockNumber UDINT` | `Block number of the active geometry segment Sample` | — |
| `nBlockNumber` | `: UDINT; sNciToPlc AT%I*` | : NCTOPLC_NCICHANNEL_REF; |

**Description:** The function ItpGetBlockNumber returns the block number of the NC program for the cyclic interface.

##### `ItpGetBottleNeckLookAheadEx` — §7.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANN The structure of the cyclic channel interface from the NCI to` | — |
| `EL_REF` | `the PLC. This structure is read-only (type:` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** PLC NCI Libraries | PLC Library: Tc2_NCI

##### `ItpGetBottleNeckModeEx` — §7.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANN The structure of the cyclic channel interface from the NCI to` | — |
| `EL_REF` | `the PLC. This structure is read-only (type:` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpGetBottleNeckModeEx reads the behavior in the event of a contour collision (bottleneck).

##### `ItpGetChannelId` — §7.1.2.8

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetChannelId` | `UDINT` | Channel ID Sample |
| `nChnId` | `: UINT;` | — |

**Description:** The function ItpGetChannelId determines the channel ID from the cyclic interface.

##### `ItpGetChannelType` — §7.1.2.9

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetChannelType E_ItpChannelType` | `Channel type` | — |

**Description:** The function ItpGetChannelType returns the channel type of the cyclic interface.

##### `ItpGetCyclicLrealOffsets` — §7.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpGetCyclicLRealOffsets is used to read the current configuration of the cyclic channel interface for LREAL variables.

##### `ItpGetCyclicUDintOffsets` — §7.1.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANN The structure of the cyclic channel interface from the NCI to` | — |
| `EL_REF` | `the PLC. This structure is read-only (type:` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpGetCyclicUDintOffsets is used to read the current configuration of the cyclic channel interface for UDINT variables.

##### `ItpGetError` — §7.1.2.12

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetError` | `UDINT` | Error number |

**Description:** The function ItpGetError returns the error number, see description of NC error codes.

##### `ItpGetGeoInfoAndHParamEx` — §7.1.2.13

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTab` | `ST_ItpPreViewTabEx` | Structure containing the segment data. See |

**Description:** The function block ItpGetGeoInfoAndHParamEx reads information of the currently active segment, as well as past and future segments. These include block number, H-parameter and residual path length on the segment.

##### `ItpGetGroupAxisIds` (FUNCTION) — §7.1.2.14

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nNciAxisIds` | `ARRAY[1..8] OF DWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetGroupAxisIds` | `DWORD` | Error number |

**Description:** ItpGetGroupAxisIds is a function that returns an array of axes IDs that were configured for the group.

##### `ItpGetGroupId` — §7.1.2.15

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetGroupId` | `UINT` | Group ID |

**Description:** The function ItpGetGroupId determines the group ID from the cyclic interface.

##### `ItpGetHParam` — §7.1.2.16

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetHParam` | `DINT` | H parameter |

**Description:** The function ItpGetHParam returns the current H-parameter.

##### `ItpGetHskMFunc` — §7.1.2.17

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetHskMFunc` | `INT` | Number of the M-function |

**Description:** The function ItpGetHskMFunc supplies the number of the M-function of type handshake.

##### `ItpGetItfVersion` — §7.1.2.18

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetItfVersion` | `UINT` | Version number of the cyclic interface |

**Description:** The function ItpGetItfVersion determines the version number of the cyclic interface.

##### `ItpGetOverridePercent` — §7.1.2.19

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sPlcToNci` | `PLCTONC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetOverridePercent` | `LREAL` | Override in percent (100 = 100 %) |

**Description:** PLC NCI Libraries | PLC Library: Tc2_NCI

##### `ItpGetSetPathVelocity` — §7.1.2.20

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetSetPathVelocity` | `LREAL` | Current bath set velocity |

**Description:** The function ItpGetSetPathVelocity reads the current path set velocity from the cyclic interface.

##### `ItpGetSParam` — §7.1.2.21

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetSParam` | `UINT` | S parameter |

**Description:** The function ItpGetSParam returns the current S-parameter.

##### `ItpGetStateInterpreter` — §7.1.2.22

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetStateInterpreter` | `UDINT` | Current Interpreter state [} 16] |

**Description:** The function ItpGetStateInterpreter returns the interpreter state.

##### `ItpGetTParam` — §7.1.2.23

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetTParam` | `UINT` | T parameter |

**Description:** The function ItpGetTParam returns the current T-parameter.

##### `ItpGoAheadEx` — §7.1.2.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANN The structure of the cyclic channel interface from the NCI to` | — |
| `EL_REF` | `the PLC. This structure is read-only (type:` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The ItpGoAheadEx function block releases a decoding stop. This is added to the job queue by an '@717' [} 169] by the Classic interpreter or a wait() by the GST interpreter and blocks it until it is released.

##### `ItpHasError` — §7.1.2.25

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpHasError` | `BOOL` | If there is an error, the function returns TRUE. |

**Description:** The function ItpHasError determines whether the interpreter is in an error state.

##### `ItpIsFastMFunc` (FUNCTION) — §7.1.2.26

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpIsFastMFunc` | `BOOL` | The function returns TRUE if the fast bit of the M-function |

**Description:** The function ItpIsFastMFunc determines whether the fast M-function is set for the supplied M-function number.

##### `ItpIsEStopEx` — §7.1.2.27

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpIsEStopEx` | `BOOL` | If the return value is TRUE, then an EStop was |

**Description:** The function ItpIsEStopEx indicates whether an EStop command was triggered.

##### `ItpIsHskMFunc` — §7.1.2.28

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpIsHskMFunc` | `BOOL` | The function returns TRUE if an M-function of type |

**Description:** The function ItpIsHskMFunc determines whether an M-function of type handshake is present.

##### `ItpLoadProgEx` — §7.1.2.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block reads the NC program when a rising |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |
| `ItpReadCyclicLReal LREAL` | `Return value Parameter 1 of type LREAL Param1` | — |
| `ItpReadCyclicUdint UDINT` | `Return value Parameter 1 of type UDINT Param1` | — |

**Description:** tTimeOut TIME

##### `ItpReadRParamsEx` — §7.1.2.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the read operation. |
| `pAddr` | `PVOID` | Address of the target variables of the data to be read. Data |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nCount DINT tTimeOut TIME

##### `ItpReadToolDescEx` — §7.1.2.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the read operation. |
| `nDNo` | `UDINT` | D-word for which the tool parameters are to be read. nDNo |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |
| `sToolDesc` | `ToolDesc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpReadToolDescEx reads the tool parameters for the supplied D-word.

##### `ItpReadZeroShiftEx` — §7.1.2.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the read operation. |
| `nZsNo` | `UDINT` | Number of the zero offset shift. G54 to G59 are zero offset |
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS timeout delay (the bBusy signal can be active for longer than tTimeOut) |
| `/` | `Inputs/Outputs` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |
| `sZeroShiftDesc` | `ZeroShiftDesc` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpReadZeroShiftEx reads the shift components X, Y and Z for the given zero offset shift.

##### `ItpResetFastMFuncEx` — §7.1.2.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The fast M-function [} 164] nMFuncNo is reset with a rising edge at input bExecute. In the event of the M- function not being available, no error is returned.

##### `ItpSetBottleNeckLookAheadEx` — §7.1.2.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nLookAhead` | `UDINT` | Specifies the look-ahead value |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANN The structure of the cyclic channel interface from the NCI to` | — |
| `EL_REF` | `the PLC. This structure is read-only (type:` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpSetBottleNeckLookAheadEx determines the maximum number of segments the system may look ahead for bottleneck detection (contour collision monitoring). Note that segments, which were added as a result of radius compensation (e.g. additional segments at acute angles) are taken into account.

##### `ItpSetBottleNeckModeEx` — §7.1.2.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `eBottleNeckMode` | `E_ItpBnMode` | Enum for the behavior in the event of a contour collision |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANN The structure of the cyclic channel interface from the NCI to` | — |
| `EL_REF` | `the PLC. This structure is read-only (type:` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpSetBottleNeckModeEx reads the behavior in the event of a contour collision (bottleneck).

##### `ItpSetCyclicLrealOffsets` — §7.1.2.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `nIndexOffsetParam` | `UDINT 1` | Group state (Index Offset) for parameter 1 |
| `nIndexOffsetParam` | `UDINT 2` | Group state (Index Offset) for parameter 2 |
| `nIndexOffsetParam` | `UDINT 3` | Group state (Index Offset) for parameter 3 |
| `nIndexOffsetParam` | `UDINT 4` | Group state (Index Offset) for parameter 4 |
| `/` | `Inputs/Outputs` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nIndexOffsetParam2 UDINT nIndexOffsetParam3 UDINT nIndexOffsetParam4 UDINT

##### `ItpSetCyclicUDintOffsets` — §7.1.2.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `tTimeOut` | `TIME` | — |
| `nIndexOffsetParam1` | `UDINT` | — |
| `nIndexOffsetParam2` | `UDINT` | — |
| `nIndexOffsetParam3` | `UDINT` | — |
| `nIndexOffsetParam4` | `UDINT` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nIndexOffsetParam2 UDINT nIndexOffsetParam3 UDINT nIndexOffsetParam4 UDINT

##### `ItpSetOverridePercent` (FUNCTION) — §7.1.2.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fOverridePercent` | `LREAL` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sPlcToNci` | `PLCTONC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpSetOverridePercent` | `BOOL` | always TRUE |

**Description:** The function ItpSetOverridePercent writes the axes channel override into the cyclic interface of the NCI. The override is passed as a percentage.

##### `ItpSetSubroutinePathEx` — §7.1.2.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `sPath` | `STRING` | Optional path for subroutines. Is disabled with an empty |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpSetToolDescNullEx` — §7.1.2.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge results in overwriting of all tool parameters of |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpSetToolDescNullEx overwrites all tool parameters (incl. number & type) of the channel with zero.

##### `ItpSetZeroShiftNullEx` — §7.1.2.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge results in overwriting of all zero offset shifts of |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpSetZeroShiftNullEx overwrites all zero offset shifts of the channel with zero.

##### `ItpSingleBlock` — §7.1.2.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecuteModeCha BOOL` | `Single block mode (nMode) is activated through a rising edge` | — |
| `nge` | `at this input.` | — |
| `nMode` | `E_ItpSingleBlockMode` | Operation mode for single block (cf. single block mode): • ItpSingleBlockOff: Single block off • ItpSingleBlockNck: Single block in NC kernel |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpStartStopEx` — §7.1.2.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | A positive edge starts the NC channel. |
| `bStop` | `BOOL` | A positive edge stops the NC channel. A stop command |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpStartStopEx starts or stops the NC channel.

##### `ItpStepOnAfterEStopEx` — §7.1.2.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANN The structure of the cyclic channel interface from the NCI to` | — |
| `EL_REF` | `the PLC. This structure is read-only (type:` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpStepOnAfterEStopEx enables further processing of the parts program after a programmed EStopEx.

##### `ItpWriteRParamsEx` — §7.1.2.48

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the write operation. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nCount DINT tTimeOut TIME

##### `ItpWriteToolDescEx` — §7.1.2.49

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nDNo` | `UDINT` | D-word for which the tool parameters are to be read. nDNo |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |
| `sToolDesc` | `ToolDesc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `nErrId` | `UDINT` | — |

**Description:** The function block ItpWriteToolDescEx writes a block of tool parameters.

##### `ItpWriteZeroShiftEx` — §7.1.2.50

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nZsNo` | `UDINT` | Number of the zero offset shift |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sZeroShiftDesc` | `ZeroShiftDesc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpWriteZeroShiftEx writes the shift components X, Y and Z for the specified zero offset shift.

#### radius (added to the radius value of column 4) › PLC Library: Tc2_NCI › NCI POUs › Blocksearch

##### `ItpBlocksearch` — §7.1.2.51.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Remains TRUE until the function block has executed a |

**Description:** sPrgName STRING(255) nPrgLength UDINT tTimeOut TIME sAxesList ST_ItpAxes sOptions ST_ItpBlockSearchOptions

##### `ItpGetBlocksearchData` — §7.1.2.51.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Remains TRUE until the function block has executed a |

**Description:** The function block ItpGetBlocksearchData reads the current position on the path. Usually this command is called at standstill. Subsequently ItpBlockSearch [} 261] can be used to set the interpreter to the position stored in sBlockSearchData.

##### `ItpStepOnAfterBlocksearch` — §7.1.2.51.3

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpStepOnAfterBlocksearch starts the movement after a Blocksearch has been executed.

#### radius (added to the radius value of column 4) › PLC Library: Tc2_NCI › NCI POUs › Retrace

##### `ItpEnableFeederBackup` — §7.1.2.52.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | TRUE: Enables feeder backup FALSE: Disables feeder backup |
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANN The structure of the cyclic channel interface from the NCI to` | — |
| `EL_REF` | `the PLC. This structure is read-only (type:` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpEnableFeederBackup enables storing of the path for retracing. It has to be activated once before the NC program (G-Code) is started. If the functionality Blocksearch [} 261] is used, ItpEnableFeederBackup has to be activated before ItpBlocksearch [} 261] is called. Feeder backup is executed as long as a TwinCAT restart or bEnable = FALSE is triggered with a rising edge at bExecute.

##### `ItpIsFeederBackupEnabled` — §7.1.2.52.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANN The structure of the cyclic channel interface from the NCI to` | — |
| `EL_REF` | `the PLC. This structure is read-only (type:` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output remains TRUE until the function block has |

**Description:** The function block ItpIsFeederBackupEnabled indicates whether feeder backup is enabled. Feeder backup must be enabled before reversing can take place. This activates storing of the path.

##### `ItpIsFeedFromBackupList` — §7.1.2.52.3

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**Description:** The function ItpIsFeedFromBackupList becomes TRUE when the feed entries (SAF & SVB) were sent from the backup list. During backward movement all entries are sent from the backup list. If the program is executed in forward mode, the first entries usually also originate from the backup list. This is dependent of the number of retraced entries and the number of entries in the SVB and SAF tables at the time at which tracing was called. All further commands originate from the ‚original’ code.

##### `ItpIsFirstSegmentReached` — §7.1.2.52.4

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpIsFirstSegmentR BOOL` | `The function returns TRUE when the start position of the G-` | — |
| `eached` | `Code program is reached. If the version number of the cyclic` | — |

**Description:** The function ItpIsFirstSegmentReached determines whether the program start position is reached during retracing, based on the cyclic channel interface.

##### `ItpIsMovingBackwards` — §7.1.2.52.5

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpIsMovingBackwar BOOL` | `The function returns TRUE when backward movement takes` | — |
| `ds` | `place on the path. If the version number of the cyclic channel` | — |

**Description:** The function ItpIsMovingBackwards determines whether backward movement takes place on the path of the current G-Code program, based on the cyclic channel interface.

##### `ItpRetraceMoveBackward` — §7.1.2.52.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |
| `/` | `Inputs/Outputs` | — |
| `sNciToPlc` | `NCTOPLC_NCICHANN The structure of the cyclic channel interface from the NCI to` | — |
| `EL_REF` | `the PLC. This structure is read-only (type:` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**Description:** The function block ItpRetraceMoveBackward deals with the geometric entries at the actual position at the start of the part program (G-Code).

##### `ItpRetraceMoveForward` — §7.1.2.52.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpRetraceMoveForward transfers all entries from the current block (e.g. position) in forward travel direction to the NC kernel. It is called to reverse the direction after ItpRetraceMoveBackward [} 271] was called.

#### radius (added to the radius value of column 4) › PLC Library: Tc2_NCI › Parts program generator

##### `ItpPpgAppendGenericBlock` — §7.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `sPathName` | `STRING` | Name of the parts program including path name |
| `sBlock` | `STRING` | Generic line to be added to the parts program. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpPpgAppendGeoCircleByRadius` — §7.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `sPathName` | `STRING` | Name of the parts program including path name |
| `bClockWise` | `BOOL` | If TRUE, the movement along the circle is clockwise, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** fTargetXPos LREAL fTargetYPos LREAL fTargetZPos LREAL fRadius LREAL fPathVelo LREAL nBlockNo UDINT tTimeOut TIME

##### `ItpPpgAppendGeoLine` — §7.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `sPathName` | `STRING` | Name of the parts program including path name |
| `fTargetXPos` | `LREAL` | Target position of the X axis |
| `fTargetYPos` | `LREAL` | Target position of the Y axis |
| `fTargetZPos` | `LREAL` | Target position of the Z axis |
| `fPathVelo` | `LREAL` | Path velocity |
| `nBlockNo` | `UDINT` | Line number in the parts program |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** fTargetYPos LREAL fTargetZPos LREAL fPathVelo LREAL nBlockNo UDINT tTimeOut TIME

##### `ItpPpgCloseMain` — §7.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `sPathName` | `STRING` | Name of the parts program including path name |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `nErrId` | `UDINT` | — |

**Description:** The function block ItpPpgCloseMain completes the main program with the corresponding code for the interpreter (M02).

##### `ItpPpgCloseSubroutine` — §7.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `sPathName` | `STRING` | Name of the parts program including path name |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpPpgCloseSubroutine completes the subroutine with the corresponding code for the interpreter (M17).

##### `ItpPpgCreateMain` — §7.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `sPathName` | `STRING` | Name of the parts program including path name |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpPpgCreateMain generates a new file, which can later be processed as main program. If the file does not yet exist, it is created, otherwise it is overwritten.

##### `ItpPpgCreateSubroutine` — §7.1.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `sPathName` | `STRING` | Name of the parts program including path name |
| `nSubroutineId` | `UDINT` | Number of the subroutine |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

#### radius (added to the radius value of column 4) › PLC Library: Tc2_NCI › Function blocks for compatibility with existing programs

##### `ItpDelDtg` — §7.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nChnId` | `UDINT` | Channel ID |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpDelDtg triggers "Delete distance to go". There is a more detailed description in the Interpreter [} 159] documentation.

##### `ItpEStop` — §7.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nGrpId` | `UDINT` | Group ID |
| `fDec` | `LREAL` | Max. deceleration during stopping. If fDec is smaller than the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpEStop triggers the NCI EStop and enables a controlled stop on the path. The limit values for the deceleration and the jerk are transferred as parameters. If these are smaller than the currently active dynamic parameters, the transferred parameters are rejected.

##### `ItpGetBottleNeckLookAhead` — §7.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nChnId` | `UDINT` | Channel ID |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**Description:** The function block ItpGetBottleNeckLookAhead determines the maximum size of the LookAhead for the bottleneck detection (contour collision monitoring).

##### `ItpGetBottleNeckMode` — §7.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nChnId` | `UDINT` | Channel ID |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `nErrId` | `UDINT` | — |
| `eBottleNeckMode` | `E_ItpBnMode` | — |

**Description:** The function block ItpGetBottleNeckMode reads the behavior in the event of a contour collision (bottleneck).

##### `ItpGetGeoInfoAndHParam` — §7.1.4.5

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTab` | `ST_ItpPreViewTabEx` | Structure containing the segment data. |
| `nErrId` | `UDINT` | Contains the command-specific error code of the most |

**Description:** The function block ItpGetGeoInfoAndHParam reads information of the currently active segment and past and future segments. These include block number, H-parameter and residual path length on the segment.

##### `ItpGoAhead` — §7.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nChnId` | `UDINT` | Channel ID |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpGoAhead may only be used in association with the decoder stop '@717' [} 169]. There is a more detailed description of this decoder stop in the interpreter documentation [} 126].

##### `ItpIsEStop` — §7.1.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nGrpId` | `UDINT` | Group ID |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**Description:** Via bEStop, the function block ItpIsEStop provides information as to whether an EStop command was triggered. If bEStop is TRUE, then an EStop was initiated (e.g. ItpEStop). The flag does not provide information as to whether the axes have already stopped or are still on the deceleration ramp.

##### `ItpLoadProg` — §7.1.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge at this input triggers execution of the NC |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nLength UDINT tTimeOut TIME

##### `ItpReadRParams` — §7.1.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the read operation. |
| `nChnId` | `UDINT` | ID of the NC channel whose R-parameters are to be read. |
| `pAddr` | `PVOID` | Address of the target variables of the data to be read. The |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nIndex DINT nCount DINT tTimeOut TIME

##### `ItpReadToolDesc` — §7.1.4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the read operation. |
| `nChnId` | `UDINT` | Channel ID |
| `nDNo` | `UDINT` | D-word for which the tool parameters are to be read. nDoNo |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sToolDesc` | `ToolDesc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpReadZeroShift` — §7.1.4.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the read operation. |
| `nChnId` | `UDINT` | Channel ID |
| `nZsNo` | `UDINT` | Number of the zero offset shift; on the NC side G54 to G59 |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sZeroShiftDesc` | `ZeroShiftDesc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpReset` — §7.1.4.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the read operation. |
| `nChnId` | `UDINT` | Channel ID |
| `tTimeOut` | `TIME NOTICE` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** PLC NCI Libraries | PLC Library: Tc2_NCI

##### `ItpResetEx` — §7.1.4.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nGrpId` | `UDINT` | Group ID |
| `nChnId` | `UDINT` | Channel ID |
| `tTimeOut` | `TIME` | ADS timeout delay (the bBusy signal can be active for longer than tTimeOut) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpResetFastMFunc` — §7.1.4.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nChnId` | `UDINT` | Channel ID |
| `nMFuncNo` | `UINT` | Flying M-function that is to be reset. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpSetBottleNeckLookAhead` — §7.1.4.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nChnId` | `UDINT` | Channel ID |
| `nLookAhead` | `UDINT` | Specifies the look-ahead value. |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpSetBottleNeckMode` — §7.1.4.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nChnId` | `UDINT` | Channel ID |
| `eBottleNeckMode` | `E_ItpBnMode` | Enum for the behavior in the event of a contour collision |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpSetSubroutinePath` — §7.1.4.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nChnId` | `UDINT` | Channel ID |
| `sPath` | `STRING` | Optional path for subroutines; is disabled with an empty string. |
| `nLength` | `UDINT` | String length |
| `tTimeOut` | `TIME` | ADS Timeout-Delay |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nLength UDINT tTimeOut TIME

##### `ItpSetToolDescNull` — §7.1.4.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge results in overwriting of all tool parameters of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpSetToolDescNull overwrites all tool parameters (incl. number & type) of the channel with null.

##### `ItpSetZeroShiftNull` — §7.1.4.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge results in overwriting of all zero offset shifts of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpSetZeroShiftNull overwrites all zero offset shifts of the channel with zero.

##### `ItpStartStop` — §7.1.4.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | A positive edge starts the NC channel. |
| `bStop` | `BOOL` | A positive edge stops the NC channel. A stop command |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

##### `ItpStepOnAfterEStop` — §7.1.4.21

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block ItpStepOnAfterEStop enables further processing of the parts program after a programmed EStop.

##### `ItpWriteRParams` — §7.1.4.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nChnId` | `UDINT` | ID of the NC channel whose R-parameters are to be written. |
| `pAddr` | `PVOID` | Address of the variables containing the data to be written. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** nIndex DINT nCount DINT tTimeOut TIME

##### `ItpWriteToolDesc` — §7.1.4.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `nChnId` | `UDINT` | Channel ID |
| `nDNo` | `UDINT` | D-word for which the tool parameters are to be read. nDoNo |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sToolDesc` | `ToolDesc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bErr` | `BOOL` | — |
| `nErrId` | `UDINT` | — |

**Description:** tTimeOut TIME

##### `ItpWriteZeroShift` — §7.1.4.24

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sZeroShiftDesc` | `ZeroShiftDesc` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** tTimeOut TIME

#### radius (added to the radius value of column 4) › PLC Library: Tc2_NCI › Obsolete

##### `F_GetVersionTcNciUtilities` (FUNCTION) — §7.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GetVersionNciUti UINT` | `Version number lities` | — |

**Description:** This function returns a part of the three-part version number of the TcNciUtilities.lib TwinCAT 2 PLC library as UINT.

##### `Get_TcNcCfg_Version` — §7.1.5.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Get_TcNcCfg_Versi STRING(20)` | `Version number on` | — |

**Description:** This function returns the version number of the TcNcCfg.lib TwinCAT 2 PLC library as a string.

##### `ItpGetVersion` (FUNCTION) — §7.1.5.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ItpGetVersion` | `String(20)` | Version number Sample |

**Description:** ItpGetVersion is a function that returns the version number of the TcNC.lib TwinCAT 2 PLC library as a string.

#### radius (added to the radius value of column 4) › PLC Library: Tc2_PlcInterpolation

##### `FB_NciFeedTablePreparation` — §7.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nEntryType` | `E_NciEntryType` | — |
| `pEntry` | `POINTER TO ST_NciGeoLine` | — |
| `bResetTable` | `BOOL` | — |
| `bResetAll` | `BOOL` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stFeedGroupTable` | `ST_NciFeedGroupTable` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nFilledRows` | `INT` | Number of filled rows |
| `bError` | `BOOL` | Becomes TRUE as soon as an error has occurred. |
| `nErrorId` | `UDINT` | Contains the command-specific error code of the most |

**Description:** The function block FB_NciFeedTablePreparation appends an entry of a specific type to the feed table (stFeedGroupTable). An appended entry can generate more than one row in the table. If the table has not enough free rows, an error is returned and no entry is added to the table. In this case the entry either has to be added to another table or to the same table, after FB_NciFeedTable was executed. This function block deals with modal functions, such as tangential following. It is therefore important to always use the same instance of this function block. The function block can be called repeatedly in a PLC cycle.

##### `FB_NciFeedTable` — §7.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `bReset` | `BOOL` | Triggers a channel reset and also resets the function block. |
| `bLogFeederEntries` | `BOOL` | If TRUE, a log file 'PlcItpFeed.log' is written in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stFeedGroupTable` | `ST_NciFeedGroupTable` | — |
| `stNciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bFeedingDone` | `BOOL` | Becomes TRUE once all table rows have been sent to the NC |

**Description:** The function block FB_NciFeedTable transfers a given table to the NC kernel. If the override is set and the approvals are enabled, execution is started immediately. bFeedingDone becomes TRUE when the transfer is complete. This signal can be used for overwriting the table with NciFeedTablePreparation [} 314]. In NciFeedTablePreparation the table first has to be reset.
