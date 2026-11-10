# Tc2_MC2_XFC+Tc2_NciXFC+AdvancedHoming_XFC

- **Manual:** `TF5065_TC3_Motion_Control_XFC_XFC_NCI_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/NC2 · **Tier:** T3 · **API entries:** 30
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** XFC/NCI extensions + advanced homing: NC kernel features.

---

#### PLC libraries › Tc2_MC2_XFC › Function blocks › Basic blocks

##### `XFC_PositionAtTime` — §5.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DcTime` | `T_DCTIME32` | Distributed Clock System Time. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Position` | `LREAL` | Extrapolated position that will be or was reached at the specified |

**Description:** The function block XFC_PositionAtTime calculates an axis position, which will be or was valid at a given time.

##### `XFC_TimeOfPosition` — §5.1.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Position` | `LREAL` | Absolute axis position |
| `Options` | `ST_NcTimeConversionOptio Data structure with options for position extrapolation. ns [} 43]` | — |
| `/` | `Inputs/Outputs` | — |
| `Axis` | `AXIS_REF` | The axis data structure of type AXIS_REF addresses an axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DcTime` | `T_DCTIME32` | Distributed clock system time at which the position will be reached |

**Description:** The function block XFC_TimeOfPosition calculates the time at which the axis will be or was at a specified position.

##### `XFC_TimeToPosition` — §5.1.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Position` | `LREAL` | Absolute axis position |
| `Options` | `ST_NcTimeConversionOptio Data structure with options for position extrapolation. ns [} 43]` | — |
| `/` | `Inputs/Outputs` | — |
| `Axis` | `AXIS_REF` | The axis data structure of type AXIS_REF addresses an axis |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Duration` | `DINT` | Time duration in nanoseconds after which the position will be |

**Description:** The function block XFC_TimeToPosition calculates the time period within which an axis will reach a position, or the time that has elapsed since the axis passed this position.

##### `XFC_TimeToModuloPosition` — §5.1.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Position` | `LREAL` | Absolute axis position |
| `ModuloFac LREAL` | `Modulo divider to be used for the calculation. ModuloFactor can be` | — |
| `tor` | `identical to the modulo factor of the axis, e.g. 360. However, a` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Duration` | `DINT` | Time duration in nanoseconds after which the position will be |

**Description:** The function block XFC_TimeToModuloPosition calculates the time period within which an axis will reach a position, or the time that has elapsed since the axis passed this position. In this case the position is the nearest modulo position in the direction of travel.

#### PLC libraries › Tc2_MC2_XFC › Function blocks › Cam controller

##### `MC_DigitalCamSwitch` — §5.1.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | The cam controller is activated via the Enable input. The initial state remains unchanged, as long as |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `Switches` | `CAMSWITCH_REF` | — |
| `Output` | `OUTPUT_REF` | — |
| `TrackOptions` | `TRACK_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InOperation` | `BOOL` | InOperation is TRUE, as long as the cam controller is |

**Description:** The function block MC_DigitalCamSwitch is a digital cam controller with one or several cams on a digital output track.

##### `MC_DigitalCamSwitch_MultiEdge` — §5.1.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | The cam controller is activated via the Enable input. The initial state remains unchanged, as long as |
| `Enable` | `BOOL` | The cam controller is activated via the Enable input. The initial state remains unchanged, as long as |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `Switches` | `CAMSWITCH_REF` | — |
| `Output` | `OUTPUT_REF_MULTIEDGE` | — |
| `TrackOptions` | `TRACK_REF` | — |
| `Axis` | `AXIS_REF` | — |
| `Switches` | `CAMSWITCH_REF` | — |
| `TrackOptions` | `TRACK_REF` | — |
| `EL5131_IO` | `EL5131_IoInterface` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InOperation` | `BOOL` | InOperation is TRUE, as long as the cam controller is |
| `InOperation` | `BOOL` | InOperation is TRUE, as long as the cam controller is |

**Description:** The function block MC_DigitalCamSwitch_MultiEdge is a digital cam controller with one or several cams on a digital output track. The function block supplements the function block MC_DigitalCamSwitch by the capability of being able to perform multiple switching operations during a PLC cycle. The switching operations are defined by position cams. Further output tracks can be realized with independent instances of the function block.

##### `XFC_PositionCam` — §5.1.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `FirstOnPosition` | `LREAL` | First position from which the cam is switched on. |
| `LastOnPosition` | `LREAL` | Last position up to which the cam is switched on. The cam |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output` | `OUTPUT_REF` | — |
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Becomes TRUE if an error occurs. |
| `ErrorID` | `UDINT` | Returns a error number when the |

**Description:** The function block XFC_PositionCam realizes a position cam that switches a digital output on or off, depending on the position.

##### `XFC_TimeCam` — §5.1.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `FirstOnPosition` | `LREAL` | First position from which the cam is switched on. |
| `Duration` | `LREAL` | Switch-on duration of the cam in [s]. |
| `Modulo` | `BOOL` | If Modulo TRUE, all positions are interpreted as modulo. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output` | `OUTPUT_REF` | — |
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Becomes TRUE if an error occurs. |
| `ErrorID` | `UDINT` | Returns a error number when the |

**Description:** The function block XFC_TimeCam realizes a time cam that activates a digital output depending on the position and switches it off after a certain time.

##### `XFC_BreakCam` — §5.1.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `BreakRelease` | `LREAL` | Brake enable. The cam remains active as long as BreakRelease is TRUE. When BreakRelease becomes |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output` | `OUTPUT_REF` | — |
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Becomes TRUE if an error occurs. |
| `ErrorID` | `UDINT` | Returns a error number when the |

**Description:** The function block XFC_BreakCam realizes a brake cam, which deactivates a digital output depending on the position as soon as BreakRelease is withdrawn.

#### PLC libraries › Tc2_MC2_XFC › Function blocks › TouchProbe

##### `XFC_TouchProbe` — §5.1.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | If Execute is active, the axis position is recorded at the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `TriggerInput` | `XFC_TRIGGER_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | The value RecordedPosition is valid. |

**Description:** The function block XFC_TouchProbe records an axis position at the time of the edge of a digital input signal (measuring probe function).

##### `XFC_EL1258_TouchProbe` — §5.1.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | If Execute is active, the axis position is recorded at the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `TriggerInput` | `XFC_TRIGGER_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | The value RecordedPosition is valid. |

**Description:** The function block XFC_EL1258_TouchProbe records the axis positions at the time of the edges of a digital input signal (touch probe function).

#### PLC libraries › Tc2_MC2_XFC › Function blocks

##### `Output routines for digital terminals` — §5.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ForceWhenLate BOOL` | `If ForceWhenLate is TRUE, the output is activated even if` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output1` | `OUTPUT_REF` | — |
| `Output2` | `OUTPUT_REF` | — |
| `TrackOptions1` | `TRACK_REF` | — |
| `TrackOptions2` | `TRACK_REF` | — |
| `EL2212` | `EL2212_IoInterface` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Becomes TRUE if an error occurs. |
| `ErrorID` | `UDINT` | If an error output is set, this parameter supplies an error number |
| `ErrorOutputMiss BOOL` | `The exact switching point transferred in the Output1 or` | — |
| `ed` | `Output2 structures could not be maintained and the output` | — |

**Description:** The function block XFC_EL2212_V2 handles the output of a digital cam with the EL2212 XFC timestamp terminal.

#### PLC libraries › Tc2_MC2_XFC › Function blocks › Output routines for digital terminals

##### `XFC_EL2212_MultiEdge` — §5.1.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ForceWhenLate BOOL` | `If ForceWhenLate is TRUE, the output is activated even if` | — |
| `ForceWhenLate` | `BOOL` | If ForceWhenLate is TRUE, the output is activated even if |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output` | `OUTPUT_REF_MULTIEDGE` | — |
| `TrackOptions` | `TRACK_REF` | — |
| `EL2212` | `EL2212_IoInterface` | — |
| `Output1` | `OUTPUT_REF` | — |
| `Output2` | `OUTPUT_REF` | — |
| `TrackOptions1` | `TRACK_REF` | — |
| `TrackOptions2` | `TRACK_REF` | — |
| `EL2252` | `EL2252_IoInterface` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Becomes TRUE if an error occurs. |
| `ErrorID` | `UDINT` | Returns a error number when the error output is set. |
| `ErrorOutputMissed` | `BOOL` | One of the switching points passed in the Output structure |
| `Error` | `BOOL` | Becomes TRUE if an error occurs. |
| `ErrorID` | `UDINT` | If an error output is set, this parameter supplies an error number |
| `ErrorOutputMissed` | `BOOL` | The exact switching point transferred in the Output1 or |

**Description:** The function block XFC_EL2212_MultiEdge handles the output of a multi-edge cam controller MC_DigitalCamSwitch_MultiEdge [} 20] via the EL2212 XFC timestamp terminal.

##### `XFC_EL2258_MultiEdge` — §5.1.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ForceWhenLate` | `BOOL` | If ForceWhenLate is TRUE, the output is activated even if of the next cycle *) |
| `OversamplingFactor` | `UINT` | Oversampling factor for the EL2262 terminal |
| `OutputOneCycleDelayed BOOL` | `OutputOneCycleDelayed is TRUE, if the output of the` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output` | `OUTPUT_REF_MULTIEDGE` | — |
| `TrackOptions` | `TRACK_REF` | — |
| `EL2258` | `EL2258_IoInterface` | — |
| `Output` | `OUTPUT_REF` | — |
| `TrackOptions` | `TRACK_REF` | — |
| `EL2262` | `EL2262_IoInterface` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Becomes TRUE if an error occurs. |
| `ErrorID` | `UDINT` | If an error output is set, this parameter supplies an error number |
| `ErrorOutputMissed` | `BOOL` | One of the switching points transferred in the Output |
| `Error` | `BOOL` | Becomes TRUE if an error occurs. |
| `ErrorID` | `UDINT` | If an error output is set, this parameter supplies an error number |
| `ErrorOutputMissed` | `BOOL` | Indicates that a switching edge cannot be determined to an exact cycle and therefore cannot be output. The |

**Description:** The function block XFC_EL2258_MultiEdge handles the output of a multi-edge cam controller MC_DigitalCamSwitch_MultiEdge [} 20] via the EL2258 XFC timestamp terminal.

##### `XFC_EL2262_MultiEdge` — §5.1.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nning of the next cycle *)` | `` | — |
| `Reset` | `BOOL` | The terminal output is deactivated. |
| `Preset` | `BOOL` | The terminal output is activated. |
| `OversamplingFactor` | `UINT` | Oversampling factor for the EL2262 terminal |
| `OutputOneCycleDelay` | `BOOL` | OutputOneCycleDelayed is TRUE, if the output of the |
| `ed` | `process image is delayed by a cycle due to the set timing.` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Output` | `OUTPUT_REF_MULTIEDGE` | — |
| `TrackOptions` | `TRACK_REF` | — |
| `EL2262` | `EL2262_IoInterface` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Becomes TRUE if an error occurs. |
| `ErrorID` | `UDINT` | If an error output is set, this parameter supplies an error number |
| `ErrorOutputMissed` | `BOOL` | One of the switching points transferred in the Output |

**Description:** The function block XFC_EL2262_Multiedge handles the output of a digital cam with the EL2262 XFC oversampling terminal.

#### PLC libraries › Tc2_MC2_XFC › Functions › Basic functions

##### `XFC_GetCurDcTaskTime` (FUNCTION) → `T_DCTIME` — §5.1.2.1.1

**Return type:** `T_DCTIME`

**Description:** The function XFC_GetCurDcTaskTime determines the start time of the current PLC cycle.

##### `XFC_ExtendDcTime` (FUNCTION) → `T_DCTIME` — §5.1.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `TimeStamp32` | `T_DCTIME32` | Distributed Clock System Time. |

**Description:** The function XFC_ExtendDcTime extends a 32-bit time stamp to 64 bit.

#### PLC libraries › Tc2_NciXFC › Function blocks › Basic blocks

##### `XFC_PathPositionAtTime` — §5.2.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `GrpId` | `UDINT` | Group ID of the Nci group. This clearly identifies the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PathPosition` | `LREAL` | Extrapolated relative path up to the preset time DcTime. |
| `Error` | `BOOL` | Becomes TRUE if an error occurs |
| `ErrorID` | `UDINT` | If the error output is set, this parameter supplies the error |

**Description:** The function block XFC_PathPositionAtTime calculates a relative path at a given time in relation to the current path position.

##### `XFC_TimeOfPathPosition` — §5.2.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `GrpId` | `UDINT` | Group ID of the Nci group. This clearly identifies the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DcTime` | `T_DCTIME32` | Distributed clock system time at which the relative path |

**Description:** The function block XFC_TimeOfPathPosition calculates the time at which an Nci group has traveled or will have traveled a preset relative path.

##### `XFC_TimeToPathPosition` — §5.2.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `GrpId` | `UDINT` | Group ID of the Nci group. This clearly identifies the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Duration` | `DINT` | Time period in nanoseconds after which the relative path |

**Description:** The function block XFC_TimeToPathPosition calculates the time period within which an Nci group has traveled a relative path or which has elapsed since then.

#### PLC libraries › Tc2_NciXFC › Function blocks › Cam controller

##### `XFC_PathPositionCam` — §5.2.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `HParam` | `UDINT` | H-parameter value that corresponds to the switching state. |
| `FirstPathId` | `UDINT` | Unique ID that continually increases over the path and |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NciToPlc` | `NciChannelToPlc` | — |
| `Output` | `OUTPUT_REF` | — |

**Description:** The function block XFC_PathPositionCam realizes a path cam that switches a digital output on and off depending on the path.

##### `MC_PathDigitalCamSwitch_MultiEdge` — §5.2.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | The cam controller is activated via the Enable input. The initial state remains unchanged, as long as |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NciToPlc` | `NCTOPLC_NCICHANNEL_REF` | — |
| `Switches` | `PATH_CAMSWITCH_REF` | — |
| `Output` | `OUTPUT_REF_MULTIEDGE` | — |
| `TrackOptions` | `TRACK_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InOperation` | `BOOL` | InOperation is TRUE, as long as the cam controller is |

**Description:** The function block MC_PathDigitalCamSwitch_MultiEdge is a digital cam controller with one or several cams on a digital output track. The function block is capable of performing several switching operations during a PLC cycle. The switching operations are defined by position cams. Further output tracks can be realized with independent instances of the function block.

#### PLC libraries › Tc3_MC2_AdvancedHoming_XFC › Function blocks › Referencing functions (passive)

##### `XFC_StepReferenceFlyingSwitch` — §5.3.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at the |
| `Axis` | `AXIS_REF` | Axis data structure of the type AXIS_REF, which uniquely |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the command was completed |

**Description:** This function block MC_StepReferenceFlyingSwitch performs referencing during a running movement via an absolutely positioned external physical switch.

##### `XFC_StepReferenceFlyingRefPulse` — §5.3.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at the |
| `Axis` | `AXIS_REF` | Axis data structure of the type AXIS_REF, which uniquely |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the command was completed |

**Description:** This function block XFC_StepReferenceFlyingRefPulse performs referencing during a running movement to the zero pulse of an encoder.

#### PLC libraries › Tc3_MC2_AdvancedHoming_XFC › Function blocks › Step functions

##### `XFC_StepAbsoluteSwitch` — §5.3.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `Parameter` | `MC_HomingParameter` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the command was completed |

**Description:** This function block searches for an absolutely positioned, external physical switch.

##### `XFC_StepAbsoluteSwitchDetection` — §5.3.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `Parameter` | `MC_HomingParameter` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the command was completed |

**Description:** This function block searches for an absolutely positioned, external physical switch.

##### `XFC_StepLimitSwitch` — §5.3.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `Parameter` | `MC_HomingParameter` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the command was completed |

**Description:** This function block performs the search for a hardware limit switch.

##### `XFC_StepLimitSwitchDetection` — §5.3.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `Parameter` | `MC_HomingParameter` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the command was completed |

**Description:** This function block performs the search for a hardware limit switch.

##### `XFC_StepReferencePulse` — §5.3.2.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `Parameter` | `MC_HomingParameter` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the command was completed |

**Description:** This function block performs the search for a zero pulse of an encoder.

##### `XFC_StepReferencePulseDetection` — §5.3.2.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |
| `Parameter` | `MC_HomingParameter` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the command was completed |

**Description:** This function block performs the search for a zero pulse of an encoder.
