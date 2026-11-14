# Tc3_McCollisionAvoidance+Tc3_McCompensations+Tc3_Mc3Definitions

- **Manual:** `TF5410_TC3_Motion_Collision_Avoidance_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/NC2 · **Tier:** T3 · **API entries:** 36
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Collision avoidance + axis compensation: motion-planning layer on the axis backend.

**Status:** PARTIAL — the 12 group-administration APIs are implemented in
`compat/tc3/tc3_mcgroup.st` over the in-process `tplc-mcgroup` service +
`FB_McGroup` builtin (ctest `tc3_mcgroup`, mask `0xfff`):
`MC_AddAxisToGroup`, `MC_RemoveAxisFromGroup`, `MC_UngroupAllAxes`,
`MC_GroupEnable`, `MC_GroupDisable`, `MC_GroupReset`,
`MC_GroupReadStatus`, `MC_GroupReadError`, `MC_GroupSetOverride`,
`MC_GroupSetSetpointSource`, `MC_SetCoordinateTransform` and
`UDINT_TO_IDENTINGROUP`. `AXES_GROUP_REF`/`AXIS_REF`/
`IDENT_IN_GROUP_REF`/`MC_COORD_REF` flatten to UDINT handles;
`ECmSetpointSource` maps 1/2/3 -> Group/MCS/ACS. Group states
approximate mcGroupState; the service is synchronous so `Busy` stays
FALSE. The remaining 24 APIs — collision-avoidance moves
(`MC_MoveAbsoluteCA`, `MC_MoveRelativeCA`, `MC_HaltCA`,
`MC_GearInPosCA`), track following (`MC_ReadTrackPositions`,
`MC_ActivateTrack`, `MC_TrackConveyorBelt`), compensation
(`MC_RegisterCompensation`, `MC_TransformPosition`), all
`MC_MovePath`/preparation/blocker/dwell FBs and
`MC_GroupReadTrajectoryInformation` — require the coordinated-motion
trajectory engine and are deferred.

---

#### If "bIsControlLoopClosed" is TRUE for all axes and the group is not empty. › Tc3_McCollisionAvoidance › Function Blocks › Motion

##### `MC_MoveAbsoluteCA` — §8.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was |

**Description:** The function block MC_MoveAbsoluteCA instructs a single axis to move to the absolute position defined in the function block, based on Collision Avoidance. Collision Avoidance has higher priority than the motion command. Therefore, the axis may slow down or wait while the motion command is executed to avoid a collision. The function block does not output the signal Done until the axis has reached its target position.

##### `MC_MoveRelativeCA` — §8.1.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ContinuousUpdate` | `BOOL` | In this version, continuous updating is only available for the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was |

**Description:** The function block MC_MoveRelativeCA instructs a single axis to move over the relative distance defined in the function block, based on Collision Avoidance. Collision Avoidance has higher priority than the motion command. The axis may slow down or wait while the motion command is executed to avoid a collision. The function block does not output the signal Done until the axis has traveled the specified distance.

##### `MC_HaltCA` — §8.1.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and |

**Description:** The function block MC_HaltCA commands a single axis to stop with Collision Avoidance.

##### `MC_GearInPosCA` — §8.1.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `ContinuousUpdate` | `BOOL` | In this version, continuous updating is only available for the Gap. |
| `RatioNumerator` | `MC_LREAL` | A gear ratio can be set by entering integer values at the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF` | — |
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartSync` | `BOOL` | This output is set when the slave actively starts synchronization and reset |

**Description:** The function block MC_GearInPosCA couples a slave axis to a master axis. The set values always form the source for the master values. Collision Avoidance has higher priority than axis coupling. The slave axis can be decoupled by sending a motion command using the buffer mode BufferMode mcAborting.

##### `MC_ReadTrackPositions` — §8.1.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | The command is executed as long as Enable is active. Inputs/outputs |
| `Axis` | `AXIS_REF Reference to an axis (see AXIS_REF).` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | This output indicates that other output values are valid for this function block. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and |

**Description:** The function block MC_ReadTrackPositions returns the current XTS track and XTS part target positions with the corresponding object IDs. The axis must be in a CA group for the function block to supply valid values. If no track is activated for the axis, the current absolute setpoints are returned with TrackId/PartId = 0.

##### `MC_ActivateTrack` — §8.1.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL The command is triggered by a rising edge at this input.` | — |
| `TrackId` | `OTCID Object ID of the active track reference system. Inputs/outputs` | — |
| `Axis` | `AXIS_RE Reference to an axis (see AXIS_REF). F` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and remains |

**Description:** The function block MC_ActivateTrack activates a track as a reference system, which can then be used in various motion function blocks for positioning. The XTS track object must be created under the XTS Processing Unit and is then selected via the object Id. The XTS tracks are configured via the XTS Configurator (see XTS documentation for more information). ObjectID 0 can be used to reactivate the absolute reference system.

#### If "bIsControlLoopClosed" is TRUE for all axes and the group is not empty. › Tc3_McCompensations › Function Blocks

##### `MC_RegisterCompensation` — §8.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | This function block activates the selected compensation type when a |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and |

**Description:** Reference to a mover template: An axis refers to a mover template via the function block MC_RegisterCompensation. • This function block selects the compensation type. • This function block influences axis behavior.

##### `MC_TransformPosition` — §8.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | This function block outputs the target position when a rising edge is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE if the command was executed and the |

**Description:** • This function block calculates a coordinate transformation. • A position specified in the origin coordinate system is returned in the target coordinate system. • An origin mover template object refers to the origin coordinate system. • A destination mover template object refers to the target coordinate system. • Object ID 0, Oid = 0, refers to the absolute coordinate system. • The origin mover template object can refer to the absolute coordinate system, and the destination mover template object can refer to the coordinate system of the Tool Center Point: thus, a calculation of the coordinate transformation from the coordinate system of the Tool Center Point to the absolute coordinate system is to be performed. • The origin mover template object can refer to the coordinate system of the Tool Center Point, and the destination mover template object can refer to the absolute coordinate system: thus, a calculation of the coordinate transformation from the coordinate system of the Tool Center Point to the absolute coordinate system is to be performed. • For information purposes only: without effect on the setpoints.

#### If "bIsControlLoopClosed" is TRUE for all axes and the group is not empty. › Tc3_McCoordinatedMotion › Function Blocks › Administrative

##### `MC_AddAxisToGroup` — §8.3.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |
| `Axis` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and remains |

**Description:** Avoidance

##### `MC_GroupDisable` — §8.3.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL The command is triggered by a rising edge at this input. Inputs/outputs` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and remains |

**Description:** Avoidance

##### `MC_GroupEnable` — §8.3.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and remains |

**Description:** Avoidance

##### `MC_GroupReadError` — §8.3.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | The command is executed as long as Enable is active. Inputs/outputs |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | This output indicates that other output values are valid for this function block. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Enable and |

**Description:** Avoidance

##### `MC_GroupReadStatus` — §8.3.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | The command is executed as long as Enable is active. Inputs/outputs |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | This output indicates that other output values are valid for this function block. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Enable and |

**Description:** Avoidance

##### `MC_GroupReadTrajectoryInformation` — §8.3.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | The command is executed as long as Enable is active. |
| `/` | `Inputs/Outputs` | — |
| `Axes` | `AXES_GROUP_REF` | Reference to a group of axes (see Cyclic Group Interface). |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | This output indicates that other output values are valid for this function |

**Description:** Avoidance

##### `MC_GroupReset` — §8.3.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL The command is triggered by a rising edge at this input. Inputs/outputs` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and remains |

**Description:** Avoidance

##### `MC_GroupSetOverride` — §8.3.1.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | The command is executed as long as Enable is active. |
| `VelFactor` | `MC_LREAL` | The override is set to this value (value range between 0 [0 %] and 1 [100 %]). Inputs/outputs |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Enabled` | `BOOL` | This output signals that the VelFactor has been set successfully. The |

**Description:** Avoidance

##### `MC_GroupSetSetpointSource` — §8.3.1.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `SetpointSource` | `ECmSetpointSource [} 116]` | Definition of the operating mode of the CM group Inputs/outputs |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and remains |

**Description:** Avoidance

##### `MC_RemoveAxisFromGroup` — §8.3.1.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `IdentInGroup` | `IDENT_IN_GROUP_REF` | Defines the interpretation of the axis to be added to the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and remains |

**Description:** Avoidance

##### `MC_SetCoordinateTransform` — §8.3.1.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `CoordTransform` | `MC_COORD_REF` | Reference to a coordinate system (see MC_COORD_REF |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and |

**Description:** Avoidance

##### `MC_SetCoordinateTransformPreparation` — §8.3.1.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `PathData` | `MC_PATH_DATA_REF [} 112]` | Table containing the segments of a path. The table is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | This output becomes TRUE if an error has occurred |

**Description:** Avoidance

##### `MC_TrackConveyorBelt` — §8.3.1.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |
| `ConveyorBelt` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InSync` | `BOOL` | The output InSync becomes TRUE for the first time when the slave is |

**Description:** Avoidance

##### `MC_UngroupAllAxes` — §8.3.1.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL The command is triggered by a rising edge at this input. Inputs/outputs` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and remains |

**Description:** Avoidance

##### `UDINT_TO_IDENTINGROUP` — §8.3.1.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `id` | `UDINT` | The unique identifier an axis shall have in the group. This does not have to be the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT_TO_IDENTINGROUP` | `IDENT_IN_GROUP_REF [} 106]` | Converts an integer value, so a PTP |

**Description:** Avoidance

#### If "bIsControlLoopClosed" is TRUE for all axes and the group is not empty. › Tc3_McCoordinatedMotion › Function Blocks › Motion

##### `MC_GroupHalt` — §8.3.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE when the group has been stopped and has come to a |

**Description:** Avoidance

##### `MC_GroupStop` — §8.3.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `Deceleration` | `MC_LREAL` | [mm/s²]. The deceleration can be programmed as a scalar value (≥1), or |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE when the group has been stopped and has come to a |

**Description:** Avoidance

##### `MC_MoveLinearAbsolutePreparation` — §8.3.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Position` | `POINTER TO LREAL` | Pointer to an array [1..PositionCount] of the target position vector. |
| `PositionCount` | `UDINT` | Dimension of the position vector. Must match the number of axes in |
| `PathData` | `MC_PATH_DATA_RE Table containing the segments of a path. The table is written by` | — |
| `F` | `MC_Move…Preparation and executed by MC_MovePath [} 97] (see` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PathData` | `MC_PATH_DATA_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | This output becomes TRUE if an error has occurred during command |

**Description:** Avoidance

##### `MC_MoveCircularAbsolutePreparation` — §8.3.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CircMode` | `MC_CIRC_MOD Specifies which circle definition is used to program the circle. Specifies` | — |
| `E` | `the meaning of the "AuxPoint" input signal (see MC_CIRC_MODE` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PathData` | `MC_PATH_DATA_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | This output becomes TRUE if an error has occurred during command |

**Description:** Avoidance

##### `MC_MoveDirectPreparation` — §8.3.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Position` | `POINTER TO LREAL` | Pointer to an array [1..PositionCount] of the target position vector |
| `PositionCount` | `UDINT` | Dimension of the position vector. Must match the number of axes in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PathData` | `MC_PATH_DATA_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | This output becomes TRUE if an error has occurred during command |

**Description:** Avoidance

##### `MC_MoveBlendingPreparation` — §8.3.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Velocity` | `MC_LREAL` | The maximum velocity for the programmed segment. The velocity |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PathData` | `MC_PATH_DATA_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | This output becomes TRUE if an error has occurred during command |

**Description:** TwinCAT 3 Motion Collision       TwinCAT 3 Motion Pick-and-Place Avoidance

##### `MC_MovePath` — §8.3.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. Inputs/outputs |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |
| `PathData` | `MC_PATH_DATA_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |

**Description:** Avoidance

##### `MC_BlockerPreparation` — §8.3.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `BlockerId` | `UDINT` | Id of the blocker. Can be any UDINT >0. |
| `BufferMode` | `MC_BUFFER_M Defines how successive motion commands are to be processed (see` | — |
| `ODE` | `MC_BUFFER_MODE [} 118]). Only mcBuffered and mcAborting are allowed` | — |
| `PathData` | `MC_PATH_DAT Table containing the segments of a path. The table is written by the` | — |
| `A_REF` | `Preparation function blocks, like this one, and executed by MC_MovePath` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PathData` | `MC_PATH_DATA_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | This output becomes TRUE if an error has occurred during command |

**Description:** Avoidance

##### `MC_ReleaseBlocker` — §8.3.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `BlockerId` | `UDINT` | Id of the blocker. Can be any UDINT >0. Inputs/outputs |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | This output becomes TRUE when the command was successfully executed. |
| `Busy` | `BOOL` | This output becomes TRUE when the command is started with Execute and |

**Description:** Avoidance

##### `MC_GroupReadBlockerStatus` — §8.3.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Enables reading of the current blocker status. Inputs/outputs |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | Returns TRUE if a valid group type is used. Only group type MC Group Coordinated |

**Description:** Avoidance

##### `MC_DwellTimePreparation` — §8.3.1.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DwellTime` | `Time` | Time during which the path is stationary at velocity 0. Any timespan |
| `PathData` | `MC_PATH_DATA_REF Table containing the segments of a path. The table is written by the` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PathData` | `MC_PATH_DATA_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | This output becomes TRUE if an error has occurred during |

**Description:** Avoidance

##### `MC_CalcTrafo` — §8.3.1.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `Forward` | `BOOL` | Determines whether the forward or backward transformation is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `AXES_GROUP_REF` | — |
| `MetaInfoIn` | `U_CalcTrafoMetaInfo` | — |
| `MetaInfoOut` | `U_CalcTrafoMetaInfo` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | This output becomes TRUE if an error has occurred during command |

**Description:** Avoidance

#### If "bIsControlLoopClosed" is TRUE for all axes and the group is not empty. › Tc3_McCoordinatedMotion › Datatypes

##### `MC_PATH_DATA_REF` — §8.3.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FilledRows` | `UDIN T` | Number of path entries (e.g. path segments). |
| `OccupiedBuffer UDIN` | `Occupied buffer size in byte. By analyzing this output the user can analyze if the` | — |
| `T` | `end of the defined buffer will be reached. Sample` | — |

**Description:** Avoidance
