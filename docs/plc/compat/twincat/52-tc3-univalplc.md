# Tc3_uniValPlc

- **Manual:** `TF5130_TC3_Robotics_uniVAL_PLC_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/Robot · **Tier:** T4 · **API entries:** 49
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** uniVAL robotics PLC layer: same licensed-engine boundary as mxAutomation.

---

#### PLC API › Function blocks › Database

##### `VAL_GetBankSize` (FUNCTION_BLOCK) — §6.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Use user database; False=Use plc database |
| `DataType` | `INT` | Type of data to resize (1 = point, 2 = joint, 3 = tool, 4 = motion descriptor) |
| `BankID` | `INT` | Number of the bank of point to resize. This parameter |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |

**Description:** The function block VAL_GetBankSize returns the size of the specified bank of data from the selected database.

##### `VAL_LoadDatabase` (FUNCTION_BLOCK) — §6.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge starts function execution. |
| `DatabaseID` | `INT` | Identifier of the database to be loaded. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |

**Description:** The function block VAL_LoadDatabase loads a database identified by the ID given as input parameter.

##### `VAL_Move` (FUNCTION_BLOCK) — §6.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `MovementID` | `INT` | Identifier for this commanded movement |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_Move allows to command a motion to the specified end position located inside a database on robot side.

##### `VAL_MoveRelative` (FUNCTION_BLOCK) — §6.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `InterpolateJ` | `BOOL` | True=Joint interpolated movement; False=Linear movement |
| `EnableBlending` | `BOOL` | True=use blending parameter of motion descriptor; |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |

**Description:** The function block VAL_MoveRelative allows to command a movement to a position calculated with specified offsets from a point located in a bank of a database.

##### `VAL_Movec` (FUNCTION_BLOCK) — §6.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge starts function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used |
| `EnableBlending` | `BOOL` | True=use blending parameter of motion descriptor; |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `MovementID` | `INT` | Identifier for this commanded movement |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_Movec commands a circular interpolated movement to a point located inside a database on robot side.

##### `VAL_Position` (FUNCTION_BLOCK) — §6.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used |
| `BankID` | `INT` | Number of the bank in which the point is located. |
| `PointIdx` | `INT` | Index of the point in the bank |
| `BankIDDest` | `INT` | Number of the bank in which the coordinates of the position are calculated |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `PositionDest` | `T_CartesianPos [} 75]` | New Position in the destination coordsystem |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_Position returns the coordinates of the position in the user frame associated with the destination BankID.

##### `VAL_ReadFrame` (FUNCTION_BLOCK) — §6.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True= Data of the user database will be used |
| `BankID` | `INT` | Number of the bank for which coordinate system |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `FrameValue` | `T_Trsf [} 80]` | Coordinates of the requested user frame |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ReadFrame returns the value of the user frame associated to a bank of point.

##### `VAL_ReadJoint` (FUNCTION_BLOCK) — §6.1.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used |
| `JointIdx` | `INT` | Index of the joint variable in the bank |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `JointValue` | `T_JointPos [} 77]` | Coordinates of joint position |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ReadJoint returns the coordinates of a joint position in the selected database.

##### `VAL_ReadMdesc` (FUNCTION_BLOCK) — §6.1.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used |
| `MdescIdx` | `INT` | Index of the motion descriptor in the bank |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `MdescValue` | `T_Mdesc [} 77]` | Values for the motion descriptor |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ReadMdesc returns the current value of a motion descriptor in the selected database.

##### `VAL_ReadPoint` (FUNCTION_BLOCK) — §6.1.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used |
| `BankID` | `INT` | Number of the bank in which the point is located. |
| `PointIdx` | `INT` | Index of the point in the bank |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `PointValue` | `T_CartesianPos [} 75]` | Coordinates of the requested point |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ReadPoint returns the coordinates of a cartesian point located in a bank of a the database.

##### `VAL_ReadTool` (FUNCTION_BLOCK) — §6.1.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used |
| `ToolIdx` | `INT` | Index of the Tool in the bank of tool located in the database |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `ToolValue` | `T_Trsf [} 80]` | Coordinates of the requested Tool |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ReadTool returns the coordinates of a Tool Center Point located in a database.

##### `VAL_SetBankSize` (FUNCTION_BLOCK) — §6.1.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Use user database; False=Use plc database |
| `DataType` | `INT` | Type of data to resize (1 = point, 2 = joint, 3 = tool, 4 = motion descriptor) |
| `DataSize` | `INT` | New size for the selected bank |
| `BankID` | `INT` | Number of the bank of point to resize. This parameter |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_SetBankSize resizes the bank of the specified type in user or plc database.

##### `VAL_WriteFrame` (FUNCTION_BLOCK) — §6.1.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used |
| `BankID` | `INT` | Number of the bank for which user frame is returned. |
| `FrameX` | `REAL` | Value for X coordinate |
| `FrameY` | `REAL` | Value for Y coordinate |
| `FrameZ` | `REAL` | Value for Z coordinate |
| `FrameRX` | `REAL` | Value for RX coordinate |
| `FrameRY` | `REAL` | Value for RY coordinate |
| `FrameRZ` | `REAL` | Value for RZ coordinate |
| `SaveOnDisk` | `BOOL` | True=Tool data stored onto the controller's flashdisk. False=Data not stored--> faster FB execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_WriteFrame writes the coordinates of a user frame associated to a bank of point located in a database.

##### `VAL_WriteJoint` (FUNCTION_BLOCK) — §6.1.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used; False= Use data of the plc database |
| `JointIdx` | `INT` | Index of the joint position in the bank |
| `Joint1` | `REAL` | Coordinate for axis 1 |
| `Joint2` | `REAL` | Coordinate for axis 2 |
| `Joint3` | `REAL` | Coordinate for axis 3 |
| `Joint4` | `REAL` | Coordinate for axis 4 |
| `Joint5` | `REAL` | Coordinate for axis 5 - not applicable for 4 axis robots- |
| `Joint6` | `REAL` | Coordinate for axis 6 - not applicable for 4 axis robots- |
| `SaveOnDisk` | `BOOL` | True=Tool data stored onto the controller's flashdisk. False=Data not stored--> faster FB execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_WriteJoint allows to initialize a joint position inside a database

##### `VAL_WriteMdesc` (FUNCTION_BLOCK) — §6.1.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used |
| `MdescIdx` | `INT` | Index of the motion descriptor in the bank. |
| `Acceleration` | `INT` | Joint acceleration. Percentage of the nominal acceleration of the robot |
| `Velocity` | `INT` | Joint velocity. Percentage of the nominal speed of the robot |
| `Deceleration` | `INT` | Joint deceleration. Percentage of the nominal deceleration of the robot |
| `Tvel` | `REAL` | Maximum cartesian velocity (mm/sec or inches/sec). Unit depends on settings on robot controller |
| `Rvel` | `REAL` | Maximum rotational velocity (mm/sec or inches/sec). Unit depends on settings on robot controller |
| `BlendLeave` | `REAL` | Distance from the destination point at which the nominal trajectory is left |
| `BlendReach` | `REAL` | Distance from the destination point at which the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_WriteMdesc allows to set the properties of a motion descriptor located in a database.

##### `VAL_WritePoint` (FUNCTION_BLOCK) — §6.1.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used |
| `BankID` | `INT` | Number of the bank in which the point is located. |
| `PointIdx` | `INT` | Index of the cartesian point in the bank |
| `PointX` | `REAL` | Value for X coordinate |
| `PointY` | `REAL` | Value for Y coordinate |
| `PointZ` | `REAL` | Value for Z coordinate |
| `PointRX` | `REAL` | Value for RX coordinate |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_WritePoint allows to modify the coordinates of a cartesian position located in a database.

##### `VAL_WriteTool` (FUNCTION_BLOCK) — §6.1.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `UserDatabase` | `BOOL` | True=Data of the user database will be used |
| `ToolIdx` | `INT` | Index of the tool position in the bank |
| `ToolX` | `REAL` | Value for X coordinate |
| `ToolY` | `REAL` | Value for Y coordinate |
| `ToolZ` | `REAL` | Value for Z coordinate |
| `ToolRX` | `REAL` | Value for RX coordinate |
| `ToolRY` | `REAL` | Value for RY coordinate |
| `ToolRZ` | `REAL` | Value for RZ coordinate |
| `SaveOnDisk` | `BOOL` | True=Tool data stored onto the controller's flashdisk. False=Data not stored--> faster FB execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_WriteTool allows to modify the coordinates of a tool located in a database.

#### PLC API › Function blocks

##### `MC_GroupContinue` (FUNCTION_BLOCK) — §6.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge starts function execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block MC_GroupContinue allows to continue the movement that has been previously stopped by the MC_Interrupt function block.

##### `MC_GroupInterrupt` (FUNCTION_BLOCK) — §6.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge starts function execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block MC_GroupInterrupt commands a controlled stop of the robot along the commanded trajectory. The robot power remains enabled. When velocity zero is reached the DONE output set. Unlike MC_GroupStop, all commanded movements are not cancelled, this means the robot can continue its trajectory as soon as MC_GroupContinue is executed. At the interrupted FB the output CommandAborted will not be set, Busy is still high and Active is reset.

##### `MC_GroupJog` (FUNCTION_BLOCK) — §6.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | True= content of the function block is executed |
| `JogMode` | `UINT` | Select jog mode. 1=Joint ; 2=Frame ; 3=Tool |
| `JogStep` | `REAL` | When this value is non zero, the robot is jogged in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | A valid jog command is sent to the robot |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block MC_GroupJog allows to move the robot from a user-supplied teach pendant in manual mode only. • The robot can be moved in speed or in position mode depending on the value of the JogStep input parameter. • Several axes can be moved simultaneously when their dedicated input are set. • The robot speed still depends on the override value. • If JogMode input is Frame(2) or Tool(3) the robot will move along Frame and Tool coordinate systems selected by Staeubli_Robot.Command.CoordSystemCmd and Staeubli_Robot.Command.ToolCmd

##### `MC_GroupPower` (FUNCTION_BLOCK) — §6.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | TRUE= Command to switch power on ; FALSE= |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Status` | `BOOL` | Effective state of the robot power. |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block MC_GroupPower manages the arm power ON/OFF sequence. Switching arm power ON/OFF is possible only if remote automatic operation mode is selected. The operation mode is selected with the Working Mode Selector box. The arm power ON / arm power OFF sequences on the robot side can take up to 1 second until robot is effectively powered ON/OFF. During that time Enable input and Status output have different values. • Enable=TRUE: Command to switch robot power ON. • Enable=FALSE: Command to switch robot power OFF.

##### `MC_GroupReadActualPosition` (FUNCTION_BLOCK) — §6.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | TRUE = Enable the execution of the function block |
| `Tool` | `UINT` | Number of the Tool for which position is reported |
| `CoordSystem` | `UINT` | Number of the user frame in which the position of selected Tool Center Point is reported |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | TRUE = A valid set of data are available. |
| `Busy` | `BOOL` | TRUE = function block is proceeding... |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `Position` | `T_CartesianPos [} 75]` | Current cartesian position of the selected Tool in the selected user frame |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block MC_GroupReadActualPosition provides a convenient way of reading the current position of the selected Tool Center Point in the selected user frame.

##### `MC_GroupReset` (FUNCTION_BLOCK) — §6.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block MC_GroupReset allows to reset all pending errors on robot side.

##### `MC_GroupStop` (FUNCTION_BLOCK) — §6.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge starts function execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block MC_GroupStop commands a controlled motion stop. Moreover, this function block cancels any movement that has been commanded and buffered on robot side.

##### `MC_MoveAxisAbsolute` (FUNCTION_BLOCK) — §6.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `JointPosition` | `T_JointPos [} 77]` | Absolute end position for each axis |
| `Velocity` | `REAL` | Joint velocity. Percentage of the nominal speed of the robot. Range [0.01 .. 500] |
| `Acceleration` | `REAL` | Joint acceleration. Percentage of the nominal acceleration of the robot. Range [0.01 .. 500] |
| `Deceleration` | `REAL` | Joint deceleration. Percentage of the nominal deceleration of the robot. Range [0.01 .. 500] |
| `CartesianVel` | `REAL` | Optional: Maximum cartesian velocity at Tool Center Point in mm/s |
| `RotationalVel` | `REAL` | Optional: Maximum rotational velocity at Tool Center Point in degree/s |
| `BufferMode` | `eMC_BUFFER_MODE [} 83]` | 0 = aborting / 1= buffered / 6-7 = blending |
| `TransitionMode` | `UINT` | Defines the profile of the robot trajectory nearby the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Active` | `BOOL` | Set when commanded movement is currently executed by the robot |
| `CommandTransferred` | `BOOL` | Commanded motion is successfully buffered inside robot controler |
| `CommandAborted` | `BOOL` | Command is aborted by another command |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `MovementID` | `INT` | Identifier for this commanded movement |

**Description:** The function block MC_MoveAxisAbsolute commands a joint interpolated movement to an end-position. The values specified for the end-position are interpreted as joint values. They represent the position of each axis in degrees.

##### `MC_MoveCircularAbsolute` (FUNCTION_BLOCK) — §6.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `CircMode` | `INT` | Specifies the meaning of the input signals 'AuxPoint' and 'CircDirection'. Only 'BORDER' mode is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Active` | `BOOL` | Set when commanded movement is currently executed by the robot |

**Description:** BufferMode eMC_BUFFER_MODE

##### `MC_MoveDirectAbsolute` (FUNCTION_BLOCK) — §6.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `Position` | `T_CartesianPos [} 75]` | Absolute end positions for each dimension in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |

**Description:** BufferMode eMC_BUFFER_MODE

##### `MC_MoveLinearAbsolute` (FUNCTION_BLOCK) — §6.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `Position` | `T_CartesianPos [} 75]` | Absolute end positions for each dimension in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |

**Description:** BufferMode eMC_BUFFER_MODE

##### `VAL_ComputeFrame` (FUNCTION_BLOCK) — §6.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `CoordSystem` | `INT` | Number of the user frame to compute. The user |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `UframeData` | `T_Trsf [} 80]` | Coordinates of the computed user frame |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ComputeFrame computes the coordinates of a user frame associated to a bank of cartesian point in the plc database. A user frame is computed with 3 cartesian positions that shall be taught as shown below. To achieve maximal accuracy, it is recommend to teach Ox vector along the longest side of the object for which a user frame will be defined.

##### `VAL_GetForwardTransformation` (FUNCTION_BLOCK) — §6.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `JointPosition` | `T_JointPos [} 77]` | Position of each axis. |
| `ToolNumber` | `UINT` | Number of the Tool Center Point for which the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `CartesianPos` | `T_CartesianPos [} 75]` | Coordinates of the computed position |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block GetForwardTransformation converts a joint position into a cartesian position.

##### `VAL_GetInverseTransformation` (FUNCTION_BLOCK) — §6.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `Position` | `T_CartesianPos [} 75]` | Cartesian position to convert into a joint position |
| `ToolNumber` | `UINT` | Number of the Tool Center Point used for the computation. The Tool Center Point is in a plc dedicated database on robot side |
| `CoordSystem` | `UINT` | Number of the user frame to which the cartesian |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `JointPos` | `T_JointPos [} 77]` | Coordinates of the computed joint position |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_GetInverseTransformation converts a cartesian position into a joint position.

#### (top level)

##### `Temperature motor 6 could not be gathered` (FUNCTION_BLOCK) — §5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `MotorTemp1` | `REAL` | Temperature of motor 1 |
| `MotorTemp2` | `REAL` | Temperature of motor 2 |
| `MotorTemp3` | `REAL` | Temperature of motor 3 |
| `MotorTemp4` | `REAL` | Temperature of motor 4 |
| `MotorTemp5` | `REAL` | Temperature of motor 5 - Not applicable for 4 axis robots |
| `MotorTemp6` | `REAL` | Temperature of motor 6 - Not applicable for 4 axis robots |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** Syntax

#### PLC API › Function blocks

##### `VAL_MoveSafeReference` (FUNCTION_BLOCK) — §6.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `Index` | `UINT` | Number of the safe position to reach |
| `Velocity` | `REAL` | Joint velocity. Percentage of the nominal speed of the robot |
| `Acceleration` | `REAL` | Joint acceleration. Percentage of the nominal acceleration of the robot |
| `Deceleration` | `REAL` | Joint deceleration. Percentage of the nominal deceleration of the robot |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Active` | `BOOL` | Set when commanded movement is currently executed by the robot |
| `CommandTransferred` | `BOOL` | Commanded motion is successfully buffered inside robot controler |
| `CommandAborted` | `BOOL` | Command is aborted by another command |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `MovementID` | `INT` | Identifier for this commanded movement |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_MoveSafeReference commands a joint interpolated movement to an end-position. The values specified for the end-position are interpreted as joint values. They represent the position of each axis in degrees.

##### `VAL_ReadAxesGroup` (FUNCTION_BLOCK) — §6.1.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | TRUE = Content of the FB is executed |
| `InputAddress` | `POINTER TO` | Base address of the Inputs within the IO interface |
| `T_FromRobot [} 74]` | `with a robot. Use 'ADR' operator for setting this input` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Error flag |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ReadAxesGroup initializes the values of the interface data block by reading the inputs of the communication interface. It provides the data for the other blocks of the library. It must be called as the first network of a robot’s program.

##### `VAL_ReadSafeReference` (FUNCTION_BLOCK) — §6.1.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `Index` | `INT` | Number of the safe reference. Range [1..2] |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `SafeReference` | `T_JointPos [} 77]` | Coordinates of the selected safe reference position |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ReadSafeReference returns the coordinates of the safe reference position.

##### `VAL_ReadWriteArmIO` (FUNCTION_BLOCK) — §6.1.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Function block content is executed as long as this input is set TRUE |
| `Dout0` | `BOOL` | Command state of digital output channel 0 of the ArmIO board |
| `Dout1` | `BOOL` | Command state of digital output channel 1 of the ArmIO board |
| `Dout2` | `BOOL` | Command state of digital output channel 2 of the ArmIO board |
| `Dout3` | `BOOL` | Command state of digital output channel 3 of the ArmIO board |
| `Dout4` | `BOOL` | Command state of digital output channel 4 of the ArmIO board |
| `Dout5` | `BOOL` | Command state of digital output channel 5 of the ArmIO board |
| `Dout6` | `BOOL` | Command state of digital output channel 6 of the ArmIO board |
| `Dout7` | `BOOL` | Command state of digital output channel 7 of the ArmIO board |
| `Aout0` | `INT` | Command state of analog output channel 0 of the ArmIO board. Unit is millivolt |
| `Aout1` | `INT` | Command state of analog output channel 1 of the ArmIO board. Unit is millivolt |
| `Aout2` | `INT` | Command state of analog output channel 2 of the ArmIO board. Unit is millivolt |
| `Aout3` | `INT` | Command state of analog output channel 3 of the ArmIO board. Unit is millivolt |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Din0` | `BOOL` | Command state of digital input channel 0 of the ArmIO board |
| `Din1` | `BOOL` | Command state of digital input channel 1 of the ArmIO board |
| `Din2` | `BOOL` | Command state of digital input channel 2 of the ArmIO board |
| `Din3` | `BOOL` | Command state of digital input channel 3 of the ArmIO board |
| `Din4` | `BOOL` | Command state of digital input channel 4 of the ArmIO board |
| `Din5` | `BOOL` | Command state of digital input channel 5 of the ArmIO board |
| `Din6` | `BOOL` | Command state of digital input channel 6 of the ArmIO board |
| `Din7` | `BOOL` | Command state of digital input channel 7 of the ArmIO board |
| `Ain0` | `INT` | Command state of analog input channel 0 of the ArmIO board. Unit is millivolt |
| `Ain1` | `INT` | Command state of analog input channel 1 of the ArmIO board. Unit is millivolt |
| `Ain2` | `INT` | Command state of analog input channel 2 of the ArmIO board. Unit is millivolt |
| `Ain3` | `INT` | Command state of analog input channel 3 of the ArmIO board. Unit is millivolt |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** Syntax

#### (top level)

##### `1 Bxx` (FUNCTION_BLOCK) — §1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Function block content is executed as long as this input is set TRUE |
| `Valve1` | `BOOL` | Command the state of the internal valve connected to A1&B1 outlets on the forearm of 6 axis robot |
| `Valve2` | `BOOL` | Command the state of the internal valve connected to A2&B2 outlets on the forearm of 6 axis robot |
| `Fout0` | `BOOL` | Command Fast output provided on J111-9 / J111-12 |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `UsrInput0` | `BOOL` | State of the digital user input 0 provided on J109-11 / J109-30 connector of the CS8C controller |
| `UsrInput1` | `BOOL` | State of the digital user input 1 provided on J109-16 / J109-35 connector of the CS8C controller |
| `Fin0` | `BOOL` | State of the fast digital input provided on J111-2 / J111-7 connector of the CS8C controller |
| `Fin1` | `BOOL` | State of the fast digital input provided on J111-3 / J111-8 connector of the CS8C controller |
| `ValvesSafeState` | `BOOL` | CS9 controlling 5-3 ways valves ONLY. The valves |

**Description:** Comment: xx = 1 or 2 depending on robot model

#### PLC API › Function blocks

##### `VAL_SafeBrakeTest` (FUNCTION_BLOCK) — §6.1.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `BrakeStatus` | `ARRAY [0..5] OF INT` | Result of the brake test for each axis |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_SafeBrakeTest commands a test of all the brakes of the connected robot. For further details about CS9 safety features, please consult 'CS9 Safety Manual'

##### `VAL_ScopeInit` (FUNCTION_BLOCK) — §6.1.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `ProductionLine` | `STRING(16)` | Name of the line in which robot is installed |
| `RobotName` | `STRING(16)` | Name of the robot within the production line |
| `RobotJob` | `STRING(16)` | Name of the task operated by the robot |
| `Parameters` | `T_ScopeParam [} 81]` | Parameters |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ScopeInit initializes the SCOPE. SCOPE is a secure solution for continuous EDGE data mining.

##### `VAL_ScopeRecStart` (FUNCTION_BLOCK) — §6.1.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `RecordID` | `STRING(16)` | Name of the record |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ScopeRecStart starts a recording of robot data.

##### `VAL_ScopeRecStop` (FUNCTION_BLOCK) — §6.1.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ScopeRecStop stops a recording of robot data.

##### `VAL_SetMovementID` (FUNCTION_BLOCK) — §6.1.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `MovementID` | `INT` | Value for setting movement ID on robot side |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | s when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block (instruction) VAL_SetMovementID changes the movement ID for the next commanded movement. It is useful to arrange so that the same path always uses the same movement ID values.

##### `VAL_ShiftPoint` (FUNCTION_BLOCK) — §6.1.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Rising edge triggers function execution |
| `CoordSystem` | `INT` | Index of the user frame (starting with 0) |
| `Position` | `T_CartesianPos [} 75]` | Absolute end positions for each dimension in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Set when function block is executing. Reset when Done or Error is set |
| `Done` | `BOOL` | This output is set when function block has terminated with success |
| `Error` | `BOOL` | This output is set when function block has terminated with error |
| `ErrorID` | `UDINT` | Error code |
| `relativePosition` | `T_CartesianPos [} 75]` | New Position after the offets have been applied |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_ShiftPoint computes a new cartesian position by applying a geometrical transformation.

##### `VAL_TouchProbe` (FUNCTION_BLOCK) — §6.1.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Rising edge triggers function execution |
| `FastInputNumber` | `INT` | Index of the fast input which is used to capture robot |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | This output is set when function block has terminated with success |
| `Active` | `BOOL` | TRUE = Fast robot position capture is activated on |

**Description:** The function block VAL_TouchProbe enables high accurate ‘on the fly’ capture of the robot position. A sensor should be connected to a dedicated fast digital input of the robot controller. The position capture occurs on a rising edge of this dedicated sensor. number

##### `VAL_WriteAxesGroup` (FUNCTION_BLOCK) — §6.1.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | TRUE = Content of the FB is executed |
| `OutputAddress` | `POINTER TO T_ToRobot Base address of the Outputs within the IO interface` | — |
| `[} 74]` | `with a robot. Use 'ADR' operator for setting this input` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AxesGroup` | `T_StaeubliRobot` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | Error flag |
| `ErrorID` | `UDINT` | Error code |
| `/` | `Inputs/Outputs` | — |
| `AxesGroup` | `T_StaeubliRobot [} 78]` | Data block for a robot |

**Description:** The function block VAL_WriteAxesGroup writes data into the communication interface. It should be called at the end of a robot’s program.

#### PLC API › Functions

##### `VAL_DefineCartesianPos` (FUNCTION) → `T_CartesianPos` — §6.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `X` | `REAL` | Value for X coordinate |
| `Y` | `REAL` | Value for Y coordinate |
| `Z` | `REAL` | Value for Z coordinate |
| `RX` | `REAL` | Value for RX coordinate |
| `RY` | `REAL` | Value for RY coordinate |
| `RZ` | `REAL` | Value for RZ coordinate |
| `CS` | `UINT` | Value for shoulder configuration (1=> sfree; 2=> ssame; 3=> righty; 4=> lefty) |
| `CE` | `UINT` | Value for elbow configuration (1=> sfree; 2=> ssame; 3=> righty; 4=> lefty) |
| `CW` | `UINT` | Value for wrist configuration (1=> sfree; 2=> ssame; 3=> righty; 4=> lefty) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `VAL_DefineCartesianPos T_CartesianPos [} 75]` | `` | — |

**Description:** The function VAL_DefineCartesianPos : T_CartesianPos [} 75] might be used to initialize a cartesian position.It provides a way to have a more compact code. The following code below myPoint := VAL_DefineCartesianPos(10,-13.125,115.10,45.5,25.3,-90.25,2,2,2);

##### `VAL_DefineJointPos` (FUNCTION) → `T_JointPos` — §6.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `J1` | `REAL` | Coordinate for axis 1 |
| `J2` | `REAL` | Coordinate for axis 2 |
| `J3` | `REAL` | Coordinate for axis 3 |
| `J4` | `REAL` | Coordinate for axis 4 |
| `J5` | `REAL` | Coordinate for axis 5 - not applicable for 4 axis robots- |
| `J6` | `REAL` | Coordinate for axis 6 - not applicable for 4 axis robots- |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `VAL_DefineJointPos` | `T_JointPos [} 77]` | — |

**Description:** The function VAL_DefineJointPos : T_JointPos [} 77] might be used to initialize a joint position. It provides a way to have a more compact code. The following below myJoint := VAL_DefineJointPos(10,-13.125,115.10,45.5,25.3,-90.25);

##### `VAL_DefineTrsf` (FUNCTION) → `T_Trsf` — §6.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `X` | `REAL` | Value for X coordinate |
| `Y` | `REAL` | Value for Y coordinate |
| `Z` | `REAL` | Value for Z coordinate |
| `RX` | `REAL` | Value for RX coordinate |
| `RY` | `REAL` | Value for RY coordinate |
| `RZ` | `REAL` | Value for RZ coordinate |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `VAL_DefineTrsf` | `T_Trsf [} 80]` | — |

**Description:** The function VAL_DefineTrsf : T_Trsf [} 80] might be used to initialize a T_Trsf variable. It provides a way to have a more compact code. The following code below: myTrsf := VAL_DefineTrsf(10,-13.125,115.10,45.5,25.3,-90.25);

##### `VAL_EnableManualMove` (FUNCTION) → `BOOL` — §6.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `EnableMove` | `BOOL` | If this input is TRUE, robot movement is enabled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `VAL_EnableManualMove BOOL` | `` | — |

**Description:** The function VAL_EnableManualMove : BOOL enables movement of the robot when manual mode is selected. It provides the hold-to-run function described hereafter:
