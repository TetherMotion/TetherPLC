# Tc3_Mc3PlanarMotion

- **Manual:** `tf5430_tc3_planar_motion_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/MC3 · **Tier:** T3 · **API entries:** 87
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Planar motion (XY/Gantry): multi-axis interpolation on the axis backend.

---

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks › MC_PlanarEnvironment

##### `Clear` (METHOD) — §8.2.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `AddStator` (METHOD) — §8.2.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |
| `lowerX` | `LREAL` | The lower x position [mm] of the stator. |
| `lowerY` | `LREAL` | The lower y position [mm] of the stator. |

**Description:** commandFeedback Reference To MC_PlanarFeedback lowerX LREAL lowerY LREAL

##### `CreateBoundary` (METHOD) — §8.2.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `Update` (METHOD) — §8.2.2.1.4

**Description:** Synchronizes the object state. Call once per PLC cycle.

##### `AddToGroup` (METHOD) — §8.2.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188] In/Outputs` | — |
| `group` | `MC_PlanarGroup [} 192]` | The Planar Group that the mover joins. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `group` | `MC_PlanarGroup` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `RemoveFromGroup` (METHOD) — §8.2.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `GetPlanarObjectInfo` (METHOD) → `PlanarObjectInfo` — §8.2.2.1.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PlanarObjectInfo [} 178]` | `` | — |

**Description:** Returns environment object info (type: environment, id: OID of nc environment).

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks

##### `MC_PlanarFeedback` (FUNCTION_BLOCK) — §8.2.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Active` | `BOOL` | Indicates an active command, i.e. command was accepted and |

**Description:** Displays specific command status information for an associated command, given back by the MC Project.

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks › MC_PlanarFeedback

##### `Update` (METHOD) — §8.2.2.2.1

**Description:** Updates internal state of the object.

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks

##### `MC_PlanarFeedbackGearInPosOnTrack` (FUNCTION_BLOCK) — §8.2.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Active` | `BOOL` | Indicates an active command, i.e. command was accepted and |

**Description:** Displays specific command status information for an associated GearInPosOnTrack command, given back by the MC Project.

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks › MC_PlanarFeedbackGearInPosOnTrack

##### `Update` (METHOD) — §8.2.2.4.1

**Description:** Updates internal state of the object.

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks

##### `MC_PlanarFeedbackGearInPosOnTrackWithMasterMover` (FUNCTION_BLOCK) — §8.2.2.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Active` | `BOOL` | Indicates an active command, i.e. command was accepted and |

**Description:** Displays specific command status information for an associated GearInPosOnTrack command, given back by the MC Project.

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks › MC_PlanarFeedbackGearInPosOnTrackWithMasterMover

##### `Update` (METHOD) — §8.2.2.5.1

**Description:** Updates internal state of the object.

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks › MC_PlanarGroup

##### `Enable` (METHOD) — §8.2.2.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `Disable` (METHOD) — §8.2.2.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `Reset` (METHOD) — §8.2.2.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `Update` (METHOD) — §8.2.2.7.4

**Description:** Updates internal state of the object, must be triggered each cycle.

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks › MC_PlanarMover

##### `MoveToPosition` (METHOD) — §8.2.2.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |
| `options` | `Reference To` | Options for the movement. ST_MoveToPositionOptio ns In/Outputs |
| `targetPosition` | `PositionXYC` | Target position for the movement. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `targetPosition` | `PositionXYC` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint options Reference To ST_MoveToPositionOptions

##### `JoinTrack` (METHOD) — §8.2.2.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |
| `options` | `Reference To` | Options for the movement. ST_JoinTrackOptions In/Outputs |
| `targetTrack` | `MC_PlanarTrack` | Target track for the movement. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `targetTrack` | `MC_PlanarTrack` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint options Reference To ST_JoinTrackOptions

##### `LeaveTrack` (METHOD) — §8.2.2.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |
| `options` | `Reference To` | Options for the movement. ST_LeaveTrackOptions In/Outputs |
| `targetPosition` | `PositionXYC` | Target position for the movement. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `targetPosition` | `PositionXYC` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint options Reference To ST_LeaveTrackOptions

##### `MoveOnTrack` (METHOD) — §8.2.2.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetTrack` | `Reference To` | Target track for the movement. If none is specified, this defaults |
| `MC_PlanarTrack` | `to the current track.` | — |
| `targetPositionO LREAL` | `Target position [mm] on the target track. nTrack` | — |
| `constraint` | `Reference To` | Constraint on maximal dynamics during the movement |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetTrack Reference To MC_PlanarTrack targetPositionOnTrack LREAL constraint Reference To DynamicConstraint_PathXY options Reference To ST_MoveOnTrackOptions

##### `GearInPosOnTrack` (METHOD) — §8.2.2.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The command specific feedback object for the command.` | — |
| `back` | `MC_PlanarFeedbackGear InPosOnTrack` | — |
| `masterAxis` | `OTCID` | Master axis being followed. |
| `trackTrail` | `Reference To` | Track trail determining along which tracks the GearInPos |
| `MC_PlanarTrackTrail` | `movement is allowed to proceed.` | — |
| `masterSyncPo LREAL` | `Position of the master axis (interpreted as [mm]) at which the` | — |
| `sition` | `slave is inSync.` | — |
| `slaveSyncPosit LREAL` | `Arc length [mm] on track given by slaveSyncPositionTrackOID` | — |
| `ion` | `at which the slave is inSync. Possibly interpreted in modulo` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `slaveSyncPositionTrack` | `MC_PlanarTrack` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedbackGearInPosOnTrack masterAxis OTCID trackTrail Reference To MC_PlanarTrackTrail masterSyncPosition LREAL slaveSyncPosition LREAL constraint Reference To DynamicConstraint_PathXY options Reference To ST_GearInPosOnTrackOptions

##### `GearInPosOnTrackWithMasterMover` (METHOD) — §8.2.2.8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The command specific feedback object for the command.` | — |
| `back` | `MC_PlanarFeedbackGear InPosOnTrackWithMaster Mover` | — |
| `trackTrail` | `Reference To` | Track trail determining along which tracks the GearInPos |
| `MC_PlanarTrackTrail` | `movement is allowed to proceed.` | — |
| `masterSyncPo LREAL` | `Position of the master axis (interpreted as [mm]) at which the` | — |
| `sition` | `slave is inSync.` | — |
| `slaveSyncPosit LREAL` | `Arc length on track [mm] given by slaveSyncPositionTrackOID` | — |
| `ion` | `at which the slave is inSync. Possibly interpreted in modulo` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `masterMover` | `MC_PlanarMover` | — |
| `masterSyncPositionTrack` | `MC_PlanarTrack` | — |
| `slaveSyncPositionTrack` | `MC_PlanarTrack` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedbackGearInPosOnTrackWithMasterMover trackTrail Reference To MC_PlanarTrackTrail masterSyncPosition LREAL slaveSyncPosition LREAL constraint Reference To DynamicConstraint_PathXY options Reference To ST_GearInPosOnTrackWithMasterMoverOptions

##### `HaltOnTrack` (METHOD) — §8.2.2.8.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Constraint on maximal dynamics during the movement |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To DynamicConstraint_PathXY

##### `MoveX` (METHOD) — §8.2.2.8.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetPosition LREAL` | `Target position [mm] for the movement.` | — |
| `refSys` | `OTCID` | Reference system of the target position. |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetPosition LREAL refSys OTCID constraint Reference To IPlcDynamicConstraint

##### `MoveY` (METHOD) — §8.2.2.8.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetPosition LREAL` | `Target position [mm] for the movement.` | — |
| `refSys` | `OTCID` | Reference system of the target position. |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetPosition LREAL refSys OTCID constraint Reference To IPlcDynamicConstraint

##### `MoveZ` (METHOD) — §8.2.2.8.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetPosition LREAL` | `Target position [mm] for the movement.` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetPosition LREAL constraint Reference To IPlcDynamicConstraint

##### `MoveA` (METHOD) — §8.2.2.8.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetPosition LREAL` | `Target position [deg] for the movement.` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetPosition LREAL constraint Reference To IPlcDynamicConstraint

##### `MoveB` (METHOD) — §8.2.2.8.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetPosition LREAL` | `Target position [deg] for the movement.` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetPosition LREAL constraint Reference To IPlcDynamicConstraint

##### `MoveC` (METHOD) — §8.2.2.8.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetPosition LREAL` | `Target position [deg] for the movement.` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |
| `options` | `Reference To` | Options for the rotation. ST_MoveCOptions |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetPosition LREAL constraint Reference To IPlcDynamicConstraint options Reference To ST_MoveCOptions

##### `MoveRelativeX` (METHOD) — §8.2.2.8.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetPosition LREAL` | `Target position [mm] for the movement.` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetPosition LREAL constraint Reference To IPlcDynamicConstraint

##### `MoveRelativeY` (METHOD) — §8.2.2.8.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetPosition LREAL` | `Target position [mm] for the movement.` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetPosition LREAL constraint Reference To IPlcDynamicConstraint

##### `MoveRelativeC` (METHOD) — §8.2.2.8.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetPosition LREAL` | `Target position [deg] for the movement.` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetPosition LREAL constraint Reference To IPlcDynamicConstraint

##### `MoveVelocityX` (METHOD) — §8.2.2.8.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetVelocity LREAL` | `Target velocity [mm/s] for the movement.` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetVelocity LREAL constraint Reference To IPlcDynamicConstraint

##### `MoveVelocityY` (METHOD) — §8.2.2.8.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetVelocity LREAL` | `Target velocity [mm/s] for the movement.` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetVelocity LREAL constraint Reference To IPlcDynamicConstraint

##### `MoveVelocityC` (METHOD) — §8.2.2.8.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `targetVelocity LREAL` | `Target velocity [deg/s] for the movement.` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback targetVelocity LREAL constraint Reference To IPlcDynamicConstraint

##### `HaltX` (METHOD) — §8.2.2.8.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint

##### `HaltY` (METHOD) — §8.2.2.8.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint

##### `HaltZ` (METHOD) — §8.2.2.8.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint

##### `HaltA` (METHOD) — §8.2.2.8.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint

##### `HaltB` (METHOD) — §8.2.2.8.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint

##### `HaltC` (METHOD) — §8.2.2.8.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint

##### `AdoptTrackOrientation` (METHOD) — §8.2.2.8.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |
| `options` | `Reference To` | Options for the rotation. ST_AdoptTrackOrientatio nOptions |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint options Reference To ST_AdoptTrackOrientationOptions

##### `Halt` (METHOD) — §8.2.2.8.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `constraint` | `Reference To` | Dynamic constraints for this movement. IPlcDynamicConstraint |

**Description:** commandFeedback Reference To MC_PlanarFeedback constraint Reference To IPlcDynamicConstraint

##### `Enable` (METHOD) — §8.2.2.8.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `Disable` (METHOD) — §8.2.2.8.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `Reset` (METHOD) — §8.2.2.8.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `Update` (METHOD) — §8.2.2.8.31

**Description:** Synchronizes the object state. Call once per PLC cycle.

##### `SetPosition` (METHOD) — §8.2.2.8.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `position` | `PositionXYC` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `StartExternalSetpointGeneration` (METHOD) — §8.2.2.8.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `options` | `Reference To` | Options for the movement. ST_ExternalSetpointGene rationOptions |

**Description:** commandFeedback Reference To MC_PlanarFeedback options Reference To ST_ExternalSetpointGenerationOptions

##### `StopExternalSetpointGeneration` (METHOD) — §8.2.2.8.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `SetExternalSetpoint` (METHOD) — §8.2.2.8.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `setPosition` | `MoverVector` | Position that is sent to the Planar Mover. |
| `setVelocity` | `MoverVector` | Velocity that is sent to the Planar Mover. |
| `setAcceleration MoverVector` | `Acceleration that is sent to the Planar Mover.` | — |

**Description:** setPosition MoverVector setVelocity MoverVector setAcceleration MoverVector

##### `SetExternalSetpointReferenceId` (METHOD) — §8.2.2.8.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |
| `setPosition` | `MoverVector` | Position that is sent to the Planar Mover. |
| `setVelocity` | `MoverVector` | Velocity that is sent to the Planar Mover. |
| `setAcceleration MoverVector` | `Acceleration that is sent to the Planar Mover.` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback setPosition MoverVector setVelocity MoverVector setAcceleration MoverVector referenceSystemOid OTCID

##### `AddToGroup` (METHOD) — §8.2.2.8.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback In/Outputs` | — |
| `group` | `MC_PlanarGroup` | The Planar Group that the Planar Mover joins. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `group` | `MC_PlanarGroup` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `RemoveFromGroup` (METHOD) — §8.2.2.8.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `GetPositionOnCurrentPart` (METHOD) — §8.2.2.8.39

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `position` | `PositionXYC` | — |

**Description:** Sets the values of the given position to the movers position values on the current part.

##### `GetTrackTrailInformation` (METHOD) — §8.2.2.8.40

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `trackTrail` | `MC_PlanarTrackTrail` | — |

**Description:** Gets the planned track trail for the mover's current track movement.

##### `GetPlanarObjectInfo` (METHOD) → `PlanarObjectInfo` — §8.2.2.8.41

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PlanarObjectInfo` | `` | — |

**Description:** Returns mover object info (type: mover, id: OID of nc mover).

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks › MC_PlanarPart

##### `Initialize` (METHOD) — §8.2.2.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |
| `partOID` | `OTCID` | OID of the part. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `environment` | `MC_PlanarEnvironment` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback partOID OTCID

##### `ActivatePosition` (METHOD) — §8.2.2.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |
| `positionIndex UDINT` | `Index of the position.` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback positionIndex UDINT

##### `ForceEnable` (METHOD) — §8.2.2.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `AllowEnable` (METHOD) — §8.2.2.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `ForceDisable` (METHOD) — §8.2.2.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `Reset` (METHOD) — §8.2.2.9.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `GetPosition` (METHOD) — §8.2.2.9.7

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `position` | `PositionXYC` | — |

**Description:** Sets the values of the given position to the parts position values.

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks › MC_PlanarTrack

##### `Clear` (METHOD) — §8.2.2.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `AppendPosition` (METHOD) — §8.2.2.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188] In/Outputs` | — |
| `position` | `PositionXYC` | Position that is the new endpoint of the Planar Track. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `position` | `PositionXYC` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `AppendLine` (METHOD) — §8.2.2.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `start` | `PositionXYC` | — |
| `end` | `PositionXYC` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `AppendCircle` (METHOD) — §8.2.2.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |
| `clockwise` | `BOOL` | Indicates if the clockwise circle is appended. In/Outputs |
| `start` | `PositionXYC` | Start position of the circular arc. |
| `end` | `PositionXYC` | End position of the circular arc, this position is the new endpoint |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `start` | `PositionXYC` | — |
| `end` | `PositionXYC` | — |
| `center` | `PositionXY` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback clockwise BOOL

##### `CloseLoop` (METHOD) — §8.2.2.10.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `StartFromTrack` (METHOD) — §8.2.2.10.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188] In/Outputs` | — |
| `track` | `MC_PlanarTrack [} 220]` | The other Planar Track. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `track` | `MC_PlanarTrack` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `EndAtTrack` (METHOD) — §8.2.2.10.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188] In/Outputs` | — |
| `track` | `MC_PlanarTrack [} 220]` | The other Planar Track. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `track` | `MC_PlanarTrack` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `StartFromTrackAdvanced` (METHOD) — §8.2.2.10.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |
| `options` | `Reference To` | Options for the connection, i.e. which connections are closed. ST_StartFromTrackAdvanc edOptions [} 184] |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `track` | `MC_PlanarTrack` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback options Reference To ST_StartFromTrackAdvancedOptions

##### `EndAtTrackAdvanced` (METHOD) — §8.2.2.10.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |
| `options` | `Reference To` | Options for the connection, i.e. which connections are closed. ST_EndAtTrackAdvancedO ptions [} 179] In/Outputs |
| `track` | `MC_PlanarTrack [} 220]` | The other Planar Track. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `track` | `MC_PlanarTrack` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback options Reference To ST_EndAtTrackAdvancedOptions

##### `Enable` (METHOD) — §8.2.2.10.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `Disable` (METHOD) — §8.2.2.10.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `Reset` (METHOD) — §8.2.2.10.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `GetArcLengthClosestTo` (METHOD) → `LREAL` — §8.2.2.10.13

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `geometry` | `IPlcGeometry2D` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LREAL` | `` | — |

**Description:** PLC Libraries | Tc3_Mc3PlanarMotion API

##### `GetPositionAt` (METHOD) — §8.2.2.10.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `arcLength` | `LREAL` | Arc length value where the position is evaluated. In/Outputs |
| `position` | `PositionXYC` | The position at the specified arc parameter. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `position` | `PositionXYC` | — |

**Description:** arcLength LREAL

##### `GetLength` (METHOD) → `LREAL` — §8.2.2.10.15

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LREAL` | `` | — |

**Description:** Returns the Planar Track's length [mm], -1 return value indicates no connection to Nc Track.

##### `GetPlanarObjectInfo` (METHOD) → `PlanarObjectInfo` — §8.2.2.10.16

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PlanarObjectInfo [} 178]` | `` | — |

**Description:** Returns track object info (type: track, id: OID of nc track).

##### `Update` (METHOD) — §8.2.2.10.17

**Description:** Synchronizes the object state. Call once per PLC cycle.

##### `AddToGroup` (METHOD) — §8.2.2.10.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188] In/Outputs` | — |
| `group` | `MC_PlanarGroup [} 192]` | The Planar Group that the mover joins. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `group` | `MC_PlanarGroup` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

##### `RemoveFromGroup` (METHOD) — §8.2.2.10.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `commandFeed Reference To` | `The feedback object for the command.` | — |
| `back` | `MC_PlanarFeedback [} 188]` | — |

**Description:** commandFeedback Reference To MC_PlanarFeedback

#### PLC Libraries › Tc3_Mc3PlanarMotion API › Function Blocks › MC_PlanarTrackTrail

##### `Clear` (METHOD) — §8.2.2.11.1

**Description:** Clears the TrackTrail.

##### `AddTrack` (METHOD) — §8.2.2.11.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `track` | `MC_PlanarTrack` | — |

**Description:** Adds a track to the TrackTrail. The track should start at the end vertex of the currently last track.

##### `GetTrackOidAt` (METHOD) → `OTCID` — §8.2.2.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `index` | `UDINT` | Index of the track in the trail [1, ..., size]. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OTCID Support and Service` | `` | — |

**Description:** Returns the oid of the track at the specified index [1, ..., size] in the trail. Zero is returned on error.
