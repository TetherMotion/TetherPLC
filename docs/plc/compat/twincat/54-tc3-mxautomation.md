# Tc3_mxAutomation

- **Manual:** `TF5120_TC3_Robotics_mxAutomation_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/Robot · **Tier:** T4 · **API entries:** 95
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** mxAutomation robotics: licensed Beckhoff robotics engine; no public spec — compatibility limited to stub/shim unless licensed SDK exists.

---

#### Function blocks › Functions for motion programming

##### `Jogging to an end position` — §7.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `MoveType` | `INT` | Motion type for Cartesian or axis-specific jogging • 0: Axis-specific • 1: Cartesian • 2: Axis-specific, as spline motion • 3: Cartesian, as spline motion |
| `Velocity` | `INT • 0 … 100%` | Velocity |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = function block is active and waiting for a motion instruction |
| `Active` | `BOOL` | TRUE = motion is currently being executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Approaching the relative target position in the TOOL` — §7.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = motion is currently being executed |
| `Done` | `BOOL` | TRUE = motion has stopped |
| `Aborted` | `BOOL` | TRUE = statement/motion has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Approaching the relative Cartesian position manually with a` — §7.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = motion is currently being executed |
| `Done` | `BOOL` | TRUE = motion has stopped |
| `Aborted` | `BOOL` | TRUE = statement/motion has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Jogging to the end position (advanced)` — §7.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `MoveType` | `INT` | Motion type for Cartesian or axis-specific jogging |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = function block is active and waiting for a motion instruction |
| `Active` | `BOOL` | TRUE = motion is currently being executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

#### Function blocks › Functions for motion programming (PLC OPEN

##### `Approaching an absolute Cartesian position with a linear` — §7.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Execute` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `ComAcpt` | `BOOL` | TRUE = statement was completely transferred and confirmed by the |

**Description:** Description

##### `Approaching a relative Cartesian position with a linear motion` — §7.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Execute` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `ComAcpt` | `BOOL` | TRUE = statement was completely transferred and confirmed by the |

**Description:** Description

##### `Approaching an absolute Cartesian position as quickly as` — §7.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Execute` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `ComAcpt` | `BOOL` | TRUE = statement was completely transferred and confirmed by the |

**Description:** Description

##### `Approaching a relative Cartesian position as quickly as` — §7.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Execute` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `ComAcpt` | `BOOL` | TRUE = statement was completely transferred and confirmed by the |

**Description:** Description

##### `Approaching an axis-specific position as quickly as possible` — §7.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Execute` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the signal. |
| `AxisPosition` | `E6AXIS (>>> E6AXIS [} 18])` | Axis-specific end position |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `ComAcpt` | `BOOL` | TRUE = statement was completely transferred and confirmed by the |

**Description:** Description

##### `Approaching an absolute Cartesian position with a circular` — §7.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Execute` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `ComAcpt` | `BOOL` | TRUE = statement was completely transferred and confirmed by the |

**Description:** Description

##### `Approaching a relative Cartesian position with a circular` — §7.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Execute` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `ComAcpt` | `BOOL` | TRUE = statement was completely transferred and confirmed by the |

**Description:** Description

#### Function blocks › Functions for program execution control

##### `Canceling a program` — §7.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Canceling a program (advanced)` — §7.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `BrakeReaction` | `INT • 1: BRAKE • 3: BRAKE FF` | Braking reaction of the robot • 2: BRAKE F (default) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Pausing robot motion` — §7.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Execute` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `BrakeActive` | `BOOL` | TRUE = statement is active and robot is waiting for enabling |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |

**Description:** Description

##### `Continuing a program` — §7.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Enable` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |

**Description:** Description

##### `Waiting for a digital input` — §7.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Number` | `INT • 1 … 2 048` | Number of the digital input (corresponds to $IN[1 … 2048] on the robot controller) |
| `Value` | `BOOL` | Value of the digital input |
| `bContinue` | `BOOL` | TRUE = poll input in advance run |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed (robot is waiting for an input) |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

#### Function blocks › Functions for interrupt programming

##### `Declaring interrupts` — §7.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is buffered in the case of a rising edge of the signal. |
| `Interrupt` | `INT • 1…8` | Number of the interrupt |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been processed |

**Description:** Description

##### `Activating interrupts` — §7.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is buffered in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been processed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Deactivating interrupts` — §7.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is buffered in the case of a rising edge of the signal. |
| `Interrupt` | `INT • 1…8` | Number of the interrupt |
| `BufferMode` | `INT • 1: ABORTING • 2: BUFFERED` | Mode in which the statement is executed (>>> BufferMode [} 22]) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |

**Description:** Description

##### `Reading the state of an interrupt` — §7.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Interrupt` | `INT • 1…8` | Number of the interrupt |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Valid` | `BOOL` | TRUE = data are valid |
| `Value` | `INT` | State of specified interrupt |

**Description:** Description

#### Function blocks › Functions for path-related switching actions

##### `Activating a switching action for path points` — §7.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is buffered in the case of a rising edge of the signal. |
| `Distance` | `INT` | Switching point of the trigger • 0: Switching action at the start point • 1: Switching action at the end point |
| `Delay` | `INT` | Statement delay • Delay = 0 ms: No delay in time |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been processed |

**Description:** Description

##### `Activating a path-related switching action` — §7.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is buffered in the case of a rising edge of the signal. |
| `Path` | `REAL` | Statement offset |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been processed |

**Description:** Description

#### Function blocks › Functions for collision detection

##### `Activating a collision detection dataset` — §7.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Reading data of a collision detection dataset` — §7.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |
| `TresholdA1` | `INT` | Collision detection threshold for robot axis A1 … A6 |
| `TresholdA2` | `INT` | • 0 … 500 |
| `TresholdA3` | `INT` | — |
| `TresholdA4` | `INT` | — |
| `TresholdA5` | `INT` | — |
| `TresholdA6` | `INT` | — |
| `TresholdE1` | `INT` | Collision detection threshold for external axis E1 … E6 |
| `TresholdE2` | `INT` | • 0 … 500 |
| `TresholdE3` | `INT` | — |
| `TresholdE4` | `INT` | — |
| `TresholdE5` | `INT` | — |
| `TresholdE6` | `INT` | — |

**Description:** Description

##### `Reading the maximum of measured collision detection values` — §7.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |
| `MaxMeasuredA1` | `INT` | Maximum of measured collision detection values since the last |
| `MaxMeasuredA2` | `INT` | reset for robot axis A1 … A6 |
| `MaxMeasuredA3` | `INT` | • 0 … 500 |
| `MaxMeasuredA4` | `INT` | — |
| `MaxMeasuredA5` | `INT` | — |
| `MaxMeasuredA6` | `INT` | — |
| `MaxMeasuredE1` | `INT` | Maximum of measured collision detection values since the last |
| `MaxMeasuredE2` | `INT` | reset for external axis E1 … E6 |
| `MaxMeasuredE3` | `INT` | • 0 … 500 |
| `MaxMeasuredE4` | `INT` | — |
| `MaxMeasuredE5` | `INT` | — |

**Description:** Description

##### `Reading the collision detection status` — §7.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |
| `ActiveCom` | `BOOL` | System variable $COLLMON_ACTIVE_COM |

**Description:** Description

##### `Resetting the maximum of measured collision detection` — §7.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Starting the teaching of collision detection values` — §7.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Stopping the teaching of collision detection values` — §7.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |
| `TeachingResultA1` | `INT` | Learned collision detection value for robot axis A1 … A6 |
| `TeachingResultA2` | `INT` | If the parameter Apply was set to TRUE, the new values of the dataset |
| `TeachingResultA3` | `INT` | are returned. If the parameter Apply was set to FALSE, the learned |

**Description:** Description

##### `Writing data of selected collision detection dataset` — §7.7.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `BufferMode` | `INT • 0: DIRECT • 1: ABORTING • 2: BUFFERED` | Mode in which the statement is executed (>>> BufferMode [} 22]) |
| `DatasetIndex` | `INT • 1 … 30` | Index of the data set which is to be written |
| `TresholdA1` | `INT` | Collision detection threshold for robot axis A1 … A6 |
| `TresholdA2` | `INT` | • 0 … 500 |
| `TresholdA3` | `INT` | — |
| `TresholdA4` | `INT` | — |
| `TresholdA5` | `INT` | — |
| `TresholdA6` | `INT` | — |
| `TresholdE1` | `INT` | Collision detection threshold for external axis E1 … E6 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

#### Function blocks › Diagnostic functions

##### `Reading and acknowledging error states` — §7.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `MessageReset` | `BOOL` | Acknowledges error messages of the mxA interface and the robot controller TRUE = reset message |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `INT` | Error number |
| `NoHeartbeatKRC` | `BOOL` | The submit interpreter is not sending a life sign |
| `NoHeartbeatPCOS` | `BOOL` | Real-time driver from mxAutomation does not send a life sign |
| `NotOnline` | `BOOL` | No connection to robot controller |
| `NotInitialized` | `BOOL` | No statements can be executed, as the connection has not been |

**Description:** Description

##### `Reading the current state of the mxA interface` — §7.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Status` | `INT` | Current state of the mxA interface (>>> Status [} 67]) |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number Status |

**Description:** Description

##### `Reading error messages of the mxA interface` — §7.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |

**Description:** Description

##### `Resetting error messages of the mxA interface` — §7.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `MessageReset` | `BOOL` | TRUE = reset message |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |

**Description:** Description

##### `Reading error messages of the robot controller` — §7.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Offset` | `INT` | If there are more than 10 messages in the message buffer, the desired |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = data are valid |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |
| `STOPMESS` | `BOOL` | TRUE = safety circuit is interrupted (robot fault) |
| `MessageCount` | `INT` | Number of messages in the message buffer |
| `Message1 …` | `DINT` | The numbers of up to 10 messages in the message buffer can be output. Message10 |

**Description:** Description

##### `Reading diagnostic signals` — §7.8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ShowTrace` | `BOOL` | TRUE = activate display of the active function blocks in the message |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Valid` | `BOOL` | TRUE = data are valid |
| `QueueCount` | `INT • 1 … 90` | Number of buffered statements |
| `PosActValid` | `BOOL` | TRUE = position data are valid (BCO) |
| `BrakeActive` | `BOOL` | TRUE = robot is stopped by means of a BRAKE statement |
| `SubmitHeartbeat` | `INT • 1 … 245` | Heartbeat signal of the submit interpreter (counter is incremented by 1 every Submit cycle) |

**Description:** Description

#### Function blocks › General special functions

##### `Reading system variables` — §7.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Index` | `INT • 1: $ADVANCE` | Index of the system variable |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |
| `Value1 … Value10` | `REAL` | Value of the system variable |

**Description:** Description

##### `Writing system variables` — §7.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Index` | `INT • 1: $ADVANCE` | Index of the system variable |
| `Value1 … Value10` | `REAL` | Value of the system variable |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Calling a brake test` — §7.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = motion is currently being executed |
| `Done` | `BOOL` | TRUE = motion has stopped |
| `Aborted` | `BOOL` | TRUE = statement/motion has been aborted |
| `Result` | `DINT` | Result of the brake test |

**Description:** Description

##### `Reading the state of the TouchUp status keys` — §7.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Valid` | `BOOL` | TRUE = data are valid |
| `TouchUP` | `BOOL` | State of the TouchUp status key on the smartPAD |

**Description:** Description

##### `Teaching points` — §7.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `PositionArray` | `POSITION_` | Array with the taught points and the corresponding coordinate |
| `ARRAY` | `systems` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Modifying settings for the advance run` — §7.9.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is buffered in the case of a rising edge of the signal. |
| `Count` | `INT • 1 … 50` | Number of functions to be transferred before the first robot motion Default value: 2 |
| `MaxWaitTime` | `INT` | Maximum wait time before the beginning of program execution if the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement has been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |

**Description:** Description

##### `Reading settings for the advance run` — §7.9.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Count` | `INT • 1 … 50` | Number of functions to be transferred by the PLC before the first robot motion Default value: 2 |
| `MaxWaitTime` | `INT` | Maximum wait time before the beginning of program execution if the set |

**Description:** Description

##### `Calculating the Cartesian robot position from the axis angles` — §7.9.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts/buffers the statement at a rising edge of the signal. |
| `Axis_Values` | `E6AXIS (>>> E6AXIS [} 18])` | Axis-specific values that are to be converted to Cartesian coordinates |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Position` | `E6POS` | Cartesian robot position calculated from the specified axis angles |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Calculating the Cartesian robot position from the axis angles` — §7.9.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts/buffers the statement at a rising edge of the signal. |
| `Axis_Values` | `E6AXIS (>>> E6AXIS [} 18])` | Axis-specific values that are to be converted to Cartesian coordinates |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Position` | `E6POS` | Cartesian robot position calculated from the specified axis angles |

**Description:** Description

##### `Calculating axis angles from the Cartesian robot position` — §7.9.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts/buffers the statement at a rising edge of the signal. |
| `Position` | `E6POS` | Cartesian robot position |
| `PosValidS` | `BOOL` | TRUE = The Status value contained in the Position parameter is valid. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `AxisPosition` | `E6AXIS (>>> E6AXIS [} 18])` | Axis angles that have been calculated from the specified Cartesian robot position |

**Description:** Description

##### `Calculating axis angles from the Cartesian robot position` — §7.9.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts/buffers the statement at a rising edge of the signal. |
| `Position` | `E6POS` | Cartesian robot position |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |

**Description:** Description

##### `Executing KRL programs` — §7.9.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `BOOL_DATA` | `ARRAY [1 … 40] OF BOOL` | Array of type BOOL as transfer parameter |
| `INT_DATA` | `ARRAY [1 … 40] OF DINT` | Array of type DINT as transfer parameter |
| `REAL_DATA` | `ARRAY [1 … 40] OF REAL` | Array of type REAL as transfer parameter |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Executing KRL programs (advanced)` — §7.9.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `BOOL_DATA` | `ARRAY [1 … 40] OF BOOL` | Array of type BOOL as transfer parameter |
| `INT_DATA` | `ARRAY [1 … 40] OF DINT` | Array of type DINT as transfer parameter |
| `REAL_DATA` | `ARRAY [1 … 40] OF REAL` | Array of type REAL as transfer parameter |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |

**Description:** Description

##### `Displaying the current robot position in a different coordinate` — §7.9.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

#### Function blocks › Special functions for Conveyor

##### `Initialize conveyors` — §7.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts execution in the case of a rising edge of the signal. |
| `BufferMode` | `INT • 1: ABORTING • 2: BUFFERED` | Mode in which the statement is executed (>>> BufferMode [} 22]) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Checking whether a scene exists` — §7.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts execution in the case of a rising edge of the signal. |
| `ConveyorIndex` | `INT • 1…5` | Index of conveyor |
| `SceneSelection` | `INT` | Scene to be tracked • 1 (NextSyncable): Next syncable scene • 2 (Current): Last scene tracked |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Exists` | `BOOL` | TRUE = scene with given parameters exists at the moment |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Tracking a scene on the conveyor` — §7.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts execution in the case of a rising edge of the signal. |
| `ConveyorIndex` | `INT • 1…5` | Index of conveyor |
| `SceneSelection` | `INT` | Scene to be tracked • 1 (NextSyncable): Next syncable scene • 2 (Current): Last scene tracked |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Stopping the tracking of a scene` — §7.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts execution in the case of a rising edge of the signal. |
| `ExecuteType` | `INT` | Defines how the tracking is to be stopped |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Active` | `BOOL` | TRUE = statement is currently being executed |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Querying the current speed of the conveyor` — §7.10.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts execution in the case of a rising edge of the signal. |
| `ConveyorIndex` | `INT • 1…5` | Index of conveyor |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Speed` | `REAL` | Speed of the selected conveyor Unit: mm/s |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

#### Function blocks › Special functions for LoadDataDetermination

##### `Configuring load data determination` — §7.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `LoadA3Settings` | `INT` | • 0: No supplementary load on axis 3 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Carrying out a test run before load data determination` — §7.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |

**Description:** Description

##### `Carrying out load data determination` — §7.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Tool` | `INT` | Number of the tool for which load data determination is to be carried out • 1 … maximum number of tools |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Assigning the load data` — §7.11.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `BufferMode` | `INT • 1: ABORTING • 2: BUFFERED` | Mode in which the statement is executed (>>> BufferMode [} 22]) |
| `Tool` | `INT` | Number of the tool to which the load data are to be assigned • 1 … maximum number of tools |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

#### Function blocks › Special functions for workspaces

##### `Configuring Cartesian workspaces` — §7.12.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Reading the configuration of the Cartesian workspaces` — §7.12.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `WorkspaceMode` | `INT • 0: #OFF • 1: #INSIDE • 2: #OUTSIDE • 3: #INSIDE_STOP • 4: #OUTSIDE_STOP` | Mode for workspaces |

**Description:** Description

##### `Configuring axis-specific workspaces` — §7.12.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the signal. |
| `WorkspaceNo` | `INT • 1…8` | Number of the workspace |
| `WorkspaceMode` | `INT • 0: #OFF • 1: #INSIDE` | Mode for workspaces |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Reading the configuration of the axis-specific workspaces` — §7.12.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | Starts/buffers the motion in the case of a rising edge of the signal. |
| `WorkspaceNo` | `INT • 1…8` | Number of workspace |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `WorkspaceMode` | `INT • 0: #OFF • 1: #INSIDE • 2: #OUTSIDE • 3: #INSIDE_STOP • 4: #OUTSIDE_STOP` | Mode for workspaces |

**Description:** Description

##### `Reading the status of the workspaces` — §7.12.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Valid` | `BOOL` | TRUE = data are valid |
| `WORKSTATE1` | `BOOL` | Status of the workspaces |
| `WORKSTATE2` | `BOOL` | — |
| `WORKSTATE3` | `BOOL` | — |
| `WORKSTATE4` | `BOOL` | — |
| `WORKSTATE5` | `BOOL` | — |
| `WORKSTATE6` | `BOOL` | — |
| `WORKSTATE7` | `BOOL` | — |
| `WORKSTATE8` | `BOOL` | — |
| `AXWORKSTATE1` | `BOOL` | — |
| `AXWORKSTATE2` | `BOOL` | — |
| `AXWORKSTATE3` | `BOOL` | — |
| `AXWORKSTATE4` | `BOOL` | — |
| `AXWORKSTATE5` | `BOOL` | — |
| `AXWORKSTATE6` | `BOOL` | — |
| `AXWORKSTATE7` | `BOOL` | — |
| `AXWORKSTATE8` | `BOOL` | — |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |

**Description:** Description

#### Function blocks › Administrative functions

##### `Reading outputs of the robot system` — §7.13.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `KRC4_Input` | `POINTER TO BYTE` | Defines the start index of the output range of the robot controller |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `ErrorID` | `DINT` | Error number |
| `Error` | `BOOL` | TRUE = error in function block |

**Description:** Description

##### `Writing robot system inputs` — §7.13.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `KRC4_Output` | `POINTER TO BYTE Defines the start index of the input range of the robot controller` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |

**Description:** Description

##### `Initializing the mxA interface` — §7.13.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = initialization successfully completed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `KRC_Serial` | `DINT` | Serial number of the robot controller |
| `KRC_Major` | `DINT` | Version identifier of the mxA interface (1st digit) |
| `KRC_Minor` | `DINT` | Version identifier of the mxA interface (2nd digit) |
| `KRC_Revision` | `DINT` | Version identifier of the mxA interface (3rd digit) |
| `KRC_AbsAccur` | `DINT` | Version identifier of the positionally accurate robot model |

**Description:** Description

##### `Setting the program override (POV)` — §7.13.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `Override` | `INT • 0 … 100%` | Program override |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Valid` | `BOOL` | TRUE = data are valid |
| `ActualOverride` | `INT • 0 … 100%` | Current override setting |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |

**Description:** Description

##### `Activating and reading automatic external signals from the` — §7.13.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Signal name (robot Description controller) |
| `AxisGroupIdx` | `INT • 1…5` | — Index of axis group |
| `MOVE_ENABLE` | `BOOL` | $MOVE_ENABLE TRUE = motion enable for the robot Notice: This system variable is monitored by |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Signal name (robot Description controller) |
| `Valid` | `BOOL` | — TRUE = data are valid |
| `RC_RDY1` | `BOOL` | $RC_RDY1 TRUE = robot controller ready for program start |
| `ALARM_STOP` | `BOOL` | $ALARM_STOP FALSE = robot stop by EMERGENCY STOP |
| `USER_SAFE` | `BOOL` | $USER_SAF FALSE = operator safety violated |
| `PERI_RDY` | `BOOL` | $PERI_RDY TRUE = robot drives activated |
| `ROB_CAL` | `BOOL` | $ROB_CAL TRUE = robot axes mastered |
| `IO_ACTCONF` | `BOOL` | $IO_ACTCONF TRUE = Automatic External interface active |
| `STOPMESS` | `BOOL` | $STOPMESS TRUE = safety circuit interrupted (robot fault) |
| `INT_E_STOP` | `BOOL` | Int. E-Stop TRUE = external EMERGENCY STOP FALSE = EMERGENCY STOP device pressed on the smartPAD |
| `PRO_ACT` | `BOOL` | $PRO_ACT TRUE = process active at robot level |
| `APPL_RUN` | `BOOL` | APPL_RUN TRUE = robot program running |
| `PRO_MOVE` | `BOOL` | $PRO_MOVE TRUE = synchronous robot motion active |
| `ON_PATH` | `BOOL` | $ON_PATH TRUE = robot on programmed path |
| `NEAR_POSRET` | `BOOL` | $NEAR_POSRET TRUE = robot near most recently saved position on the programmed path (after leaving path) |
| `ROB_STOPPED` | `BOOL` | $ROB_STOPPED TRUE = robot is at standstill |
| `T1` | `BOOL` | $T1 TRUE = mode T1 selected |
| `T2` | `BOOL` | $T2 TRUE = mode T2 selected |
| `AUT` | `BOOL` | $AUT TRUE = operating mode Automatic selected |
| `EXT` | `BOOL` | $EXT TRUE = operating mode Automatic External selected |

**Description:** Description

##### `Setting KRC_AutomaticExternal inputs automatically` — §7.13.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteReset` | `BOOL` | Selects the mxAutomation robot program in the case of a rising edge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | The sequence is active but not yet completed. |
| `Done` | `BOOL` | The sequence is completed. |
| `DispActive` | `BOOL` | TRUE = robot program is active |
| `ResetValid` | `BOOL` | TRUE = conditions for a RESET at the function block KRC_AutomaticExternal are met |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |

**Description:** Description

##### `Reading the current robot position` — §7.13.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Valid` | `BOOL` | TRUE = data are valid |
| `Position` | `E6POS` | Current Cartesian actual position ($POS_ACT on the robot controller) |

**Description:** Description

##### `Reading the current axis position` — §7.13.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Valid` | `BOOL` | TRUE = data are valid |
| `AxisPosition` | `E6AXIS` | Current axis-specific robot position ($AXIS_ACT on the robot controller) |

**Description:** Description

##### `Reading the current path velocity` — §7.13.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Valid` | `BOOL` | TRUE = data are valid |
| `Value` | `REAL` | Current path velocity ($VEL_ACT on the robot controller) Unit: m/s |

**Description:** Description

##### `Reading the current axis velocity` — §7.13.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |
| `A1 … A6` | `INT` | Current motor speed (-100% … +100%) of A1 … A6, relative to the |

**Description:** Description

##### `Reading the current robot acceleration` — §7.13.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |
| `ACC_ABS` | `REAL` | Current Cartesian acceleration relative to the absolute value of the |

**Description:** Description

##### `Reading a digital input` — §7.13.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Number` | `INT • 1 … 2048` | Number of the digital input (corresponds to $IN[1 … 2048] on the robot controller) |
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Value` | `BOOL` | Value of the digital input |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |
| `Parameter` | `Type` | Description |
| `Valid` | `BOOL` | TRUE = data are valid |
| `IN1 … IN8` | `BOOL` | Actual value of the digital input $IN[1] … $IN[8] |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |

**Description:** Description

##### `Reading multiple digital inputs` — §7.13.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Startnumber` | `INT • 1 … 2048` | Number of the 1st digital input polled (corresponds to $IN[1 … 2048] on the robot controller) |
| `Length` | `INT • 1 … 200` | Number of inputs that are polled |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Values` | `BOOL[200]` | Values of the digital inputs |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Reading a digital output` — §7.13.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Number` | `INT • 1 … 2048` | Number of the digital output (corresponds to $OUT[1 … 2048] on the robot controller) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Value` | `BOOL` | Value of the digital output |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Writing a digital output` — §7.13.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Number` | `INT • 1 … 2 048` | Number of the digital output (corresponds to $OUT[1 … 2048] on the robot controller) |
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `OUT1 … OUT8` | `BOOL` | Setpoint value of the output $OUT[1] … $OUT[8] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |
| `Parameter` | `Type` | Description |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |

**Description:** Description

##### `Reading an analog input` — §7.13.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Number` | `INT • 1 … 32` | Number of the analog input ($ANIN[1 … 32] on the robot controller) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Value` | `REAL` | Value of the analog input |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Reading an analog output` — §7.13.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Number` | `INT • 1 … 32` | Number of the analog output ($ANOUT[1 … 32] on the robot controller) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Value` | `REAL` | Value of the analog output |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Writing an analog output` — §7.13.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Number` | `INT • 1 … 32` | Number of the analog output ($ANOUT[1 … 32] on the robot controller) |
| `Value` | `REAL` | Value of the analog output |
| `bContinue` | `BOOL` | TRUE = output written in advance run |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Selecting the tool, base and interpolation mode` — §7.13.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Reading TOOL data` — §7.13.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `ToolNo` | `INT • 1 … 16: TOOL_DATA[1 … 16]` | Number of the TOOL coordinate system |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `ToolData` | `FRAME` | The data structure FRAME contains the following TOOL data: |

**Description:** Description

##### `Writing TOOL data` — §7.13.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `ToolData` | `FRAME` | The data structure FRAME contains the following TOOL data: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |

**Description:** Description

##### `Reading BASE data` — §7.13.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `BaseNo` | `INT • 1 … 32: BASE_DATA[1 … 32]` | Number of the BASE coordinate system |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `BaseData` | `FRAME` | The data structure FRAME contains the following BASE data: |

**Description:** Description

##### `Writing BASE data` — §7.13.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `BaseNo` | `INT • 1 … 32: BASE_DATA[1 … 32]` | Number of the BASE coordinate system |
| `BaseData` | `FRAME` | The data structure FRAME contains the following BASE data: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Reading the load data` — §7.13.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `Tool` | `INT` | Number of the TOOL coordinate system for reading the payload data |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `M` | `REAL` | Mass |
| `X, Y, Z` | `REAL` | Position of the center of gravity relative to the flange |
| `A, B, C` | `REAL` | Orientation of the principal inertia axes relative to the flange |
| `JX, JY, JZ` | `REAL` | Mass moments of inertia |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Writing load data` — §7.13.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Reading the software limit switches of the robot axes` — §7.13.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `A1_Min … A6_Min` | `REAL` | Negative software limit switch of axis A1 … A6 Unit: mm or ° |
| `A1_Max … A6_Max` | `REAL` | Positive software limit switch of axis A1 … A6 Unit: mm or ° |
| `Error` | `BOOL` | TRUE = error in function block |
| `ErrorID` | `DINT` | Error number |
| `OrderID` | `DINT` | OrderID number |

**Description:** Description

##### `Reading the software limit switches of the external axes` — §7.13.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `E1_Min … E6_Min` | `REAL` | Negative software limit switch of axis E1 … E6 |

**Description:** Description

##### `Writing the software limit switches of the robot axes` — §7.13.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `A1_Min … A6_Min` | `REAL` | Negative software limit switch of axis A1 … A6 Unit: mm or ° |
| `A1_Max … A6_Max` | `REAL` | Positive software limit switch of axis A1 … A6 Unit: mm or ° |
| `BufferMode` | `INT • 0: DIRECT • 1: ABORTING • 2: BUFFERED` | Mode in which the statement is executed (>>> BufferMode [} 22]) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |

**Description:** Description

##### `Writing the software limit switches of the external axes` — §7.13.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `AxisGroupIdx` | `INT • 1…5` | Index of axis group |
| `ExecuteCmd` | `BOOL` | The statement is executed in the case of a rising edge of the signal. |
| `E1_Min … E6_Min` | `REAL` | Negative software limit switch of axis E1 … E6 Unit: mm or ° |
| `E1_Max … E6_Max` | `REAL` | Positive software limit switch of axis E1 … E6 Unit: mm or ° |
| `BufferMode` | `INT • 0: DIRECT • 1: ABORTING • 2: BUFFERED` | Mode in which the statement is executed (>>> BufferMode [} 22]) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parameter` | `Type` | Description |
| `Busy` | `BOOL` | TRUE = statement is currently being transferred or has already been transferred |
| `Done` | `BOOL` | TRUE = statement has been executed |
| `Aborted` | `BOOL` | TRUE = statement has been aborted |
| `Error` | `BOOL` | TRUE = error in function block |

**Description:** Description

#### (top level)

##### `INVALID REAL VALUE The programmed Real value is Program a valid value:` (FUNCTION) — §65

**Description:** Messages | Error messages of the mxA interface in the robot interpreter

##### `INVALID REAL VALUE The programmed Real value is Program a valid value:` (FUNCTION) — §465

**Description:** Messages | Errors in the function block
