# Tc2_MC2_Camming

- **Manual:** `TF5050_TC3_NC_Camming_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/NC2 · **Tier:** T3 · **API entries:** 18
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Electronic camming: needs cam-table engine on top of the axis backend.

---

#### Cam plates

##### `MC_CamTableSelect` — §5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `Periodic` | `BOOL` | Periodic is TRUE if the cam plate is cyclically repeated. |
| `CamTableID` | `MC_CAM_ID` | ID of the cam plate used for the coupling. Inputs/outputs |
| `CamTable` | `MC_CAM_REF` | The data structure of type MC_CAM_REF [} 56] describes the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CamTable` | `MC_CAM_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the cam plate was created successfully. |
| `Busy` | `BOOL` | The Busy output becomes TRUE when the command is started |

**Description:** With the function block MC_CamTableSelect, a table can be specified and loaded into the NC. The function block creates a new table and simultaneously fills it with data provided by the PLC.

##### `MC_CamOut` — §5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `Options` | `ST_CamOutOptions Currently not implemented. Inputs/outputs` | — |
| `Slave` | `AXIS_REF` | Axis data structure of the Slave. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the cam plate was created successfully. |
| `Busy` | `BOOL` | The Busy output becomes TRUE when the command is started |

**Description:** The function block MC_CamOut disables a master-slave coupling.

##### `MC_CamIn` — §5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `MasterOffset` | `LREAL` | Offset to the master position of the cam plate. |
| `SlaveOffset` | `LREAL` | Offset to the slave position of the cam plate. |
| `MasterScaling` | `LREAL` | Scaling of the master position of the cam plate. |
| `SlaveScaling` | `LREAL` | Scaling of the slave position of the cam plate. |
| `StartMode` | `MC_StartMode` | StartMode [} 71] determines whether the cam positions are |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF` | — |
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InSync` | `BOOL` | Becomes TRUE if the coupling was successful and the cam plate |

**Description:** The function block MC_CamIn activates master-slave coupling with a certain cam plate. In addition it is possible to switch to a new cam plate in coupled state. The switching rules, in particular the time or position, can be specified.

##### `MC_CamScaling` — §5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `ActivationMode` | `MC_CamActivation` | ActivationMode [} 57] specifies the scaling time or position. Mode |
| `ActivationPosition` | `LREAL` | Master position at which a cam plate is scaled depending on the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the cam plate was created |

**Description:** A cam plate coupling can be scaled with the function block MC_CamScaling. The raw table data of the cam plate are not affected, however the scaling refers to an existing master/slave coupling. The following parameters can be modified: scaling factors for master and slave, and offsets for the cam plate within the coordinate system.

#### (top level)

##### `Multi cam plates` — §6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `ActivationMode` | `MC_CamActivationMode ActivationMode [} 57] specifies the scaling time or position.` | — |
| `ActivationPosition LREAL` | `Master position at which a cam plate is scaled depending on` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF` | — |
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InSync` | `BOOL` | Becomes TRUE if the cam plate operation was completed |

**Description:** MC_CamIn_V2 is a further development of the function block MC_CamIn [} 16] and is able to operate with several superimposed cam plates (Multi-Cam). When MC_CamIn_V2 is first called it creates a master/slave coupling with the cam plate. Subsequent calls during runtime can be used to superimpose additional cam plates for the same slave axis or remove them again. The switching rules, in particular the time or position, can be specified.

#### Multi cam plates

##### `MC_CamAdd` — §6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `ActivationMode` | `MC_CamActivationMode The ActivationMode [} 57] is used to specify the time or` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF` | — |
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InSync` | `BOOL` | Becomes TRUE if the cam plate operation was completed |

**Description:** MC_CamAdd adds a cam plate to a multi-cam coupling. The cam plate coupling is initially created with MC_CamIn_V2 [} 24].

##### `MC_CamExchange` — §6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `ActivationMode` | `MC_CamActivationMode The ActivationMode [} 57] is used to specify the time or` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF` | — |
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InSync` | `BOOL` | Becomes TRUE if the cam plate operation was completed |

**Description:** MC_CamExchange exchanges a cam plate in a multi-cam coupling. The cam plate coupling is initially created with MC_CamIn_V2 [} 24].

##### `MC_CamRemove` — §6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `ActivationMode` | `MC_CamActivationMode The ActivationMode [} 57] is used to specify the time or` | — |
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `ActivationMode` | `MC_CamActivation` | ActivationMode [} 57] specifies the scaling time or position. Mode |
| `ActivationPosition` | `LREAL` | Master position at which a cam plate is scaled depending on the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF` | — |
| `Slave` | `AXIS_REF` | — |
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the cam plate operation was completed |
| `Done` | `BOOL` | Becomes TRUE if scaling was successful. |
| `Busy` | `BOOL` | The Busy output becomes TRUE when the command is started |

**Description:** MC_CamRemove removes a cam plate from a multi-cam environment. See also MC_CamIn_V2 [} 24].

#### Motion functions

##### `MC_ReadMotionFunction` — §7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `CamTableID` | `MC_CAM_ID` | ID [} 55] of the loaded table. |
| `PointID` | `MC_MotionFunction Point ID [} 66] of the first point of the motion function to be` | — |
| `Point_ID` | `read.` | — |
| `NumPoints` | `UDINT` | Number of motion function points to be read. For reading all |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CamTable` | `MC_CAM_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the data have been read successfully. |
| `Busy` | `BOOL` | The Busy output becomes TRUE when the command is started |

**Description:** The MC_ReadMotionFunction function block can be used to read the data of a motion function. Either the complete function with all interpolation points or only a part can be read. The data are stored within the PLC in the structure described by CamTable [} 56].

##### `MC_ReadMotionFunctionPoint` — §7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `CamTableID` | `MC_CAM_ID` | ID [} 55] of the loaded table. |
| `PointID` | `MC_MotionFunction Point ID [} 66] of the first point of the motion function to be` | — |
| `Point_ID` | `read. Inputs/outputs` | — |
| `Point` | `MC_MotionFunctionPoint` | Data structure [} 65] containing the data of a motion function |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Point` | `MC_MotionFunctionPoint` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the data have been read successfully. |
| `Busy` | `BOOL` | The Busy output becomes TRUE when the command is started |

**Description:** The function block MC_ReadMotionFunctionPoint can be used to read the data of a motion function interpolation point.

##### `MC_WriteMotionFunction` — §7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `CamTableID` | `MC_CAM_ID` | ID [} 55] of the loaded table. |
| `PointID` | `MC_MotionFunction Point ID [} 66] of the first point of the motion function to be` | — |
| `Point_ID` | `read.` | — |
| `NumPoints` | `UDINT` | Number of motion function points to be written. |
| `CamTable` | `MC_CAM_REF` | Reference [} 56] to the table (structure). The start address of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the data have been read successfully. |
| `Busy` | `BOOL` | The Busy output becomes TRUE when the command is started |

**Description:** The MC_WriteMotionFunction function block can be used to write the data of a motion function into the NC. Either the complete function with all interpolation points or only a part can be written. First, the data are stored within the PLC in the structure described by CamTable [} 56].

##### `MC_WriteMotionFunctionPoint` — §7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `CamTableID` | `MC_CAM_ID` | ID [} 55] of the loaded table. |
| `PointID` | `MC_MotionFunction Point ID [} 66] of the first point of the motion function to be` | — |
| `Point_ID` | `written.` | — |
| `Options` | `ST_WriteMotionFun Data structure with additional options:` | — |
| `ctionOptions [} 75] SynchronousAccess` | `Writing takes place via synchronous access without a time delay. Should only be used for applications that are` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Point` | `MC_MotionFunctionPoint` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the data have been read successfully. |
| `Busy` | `BOOL` | The Busy output becomes TRUE when the command is started |

**Description:** The function block MC_WriteMotionFunctionPoint can be used to write the data of a motion function interpolation point.

##### `MC_SetCamOnlineChangeMode` — §7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `ActivationMode` | `MC_CamActivation` | Defines when and how scaling takes place. |
| `Mode` | `(MC_CamActivationMode [} 57])` | — |
| `ActivationPosition` | `LREAL` | Optional master position at which scaling is performed |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE when the function has been successfully |

**Description:** The function block MC_SetCamOnlineChangeMode specifies the mode for write access to cam plate data.

##### `MC_ReadMotionFunctionValues` — §7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with the rising edge. |
| `CamTableID` | `MC_CAM_ID` | ID [} 55] of the loaded table (motion function type). |
| `ValueSelectMask` | `UINT` | The selection mask can be used to specify which data are to be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CamTable` | `MC_CAM_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE if the data have been read successfully. |
| `Busy` | `BOOL` | The Busy output becomes TRUE when the command is started |

**Description:** The function block MC_ReadMotionFunctionValues can be used to read the interpolated data of a motion function in the form of a table.

#### (top level)

##### `MC_ReadCamTableSlaveDynamics` — §8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `CamTableID` | `MC_CAM_ID` | ID [} 55] of the loaded table. |
| `MasterPosition` | `LREAL` | Master position within the table for which the slave dynamics is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE, if the command was executed |

**Description:** The function block MC_ReadCamTableSlaveDynamics can be used to determine the slave dynamics at a certain point of a cam plate table. The function evaluates the raw table data. Any scaling of the cam plate is not taken into account.

##### `MC_CamInfo` — §8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `AtMasterPosition` | `BOOL` | If AtMasterPosition is TRUE, the data related to the specified |
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `CamTableId` | `BOOL` | For multi-cam plate coupling, this has to be transferred explicitly, |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Slave` | `AXIS_REF` | — |
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE, if the command was executed |
| `Done` | `BOOL` | Becomes TRUE, if the command was executed |

**Description:** The MC_CamInfo function block is used to determine data on the current state and the current parameterization of a cam plate coupling. The command assumes that the slave axis is coupled by a cam plate. If the AtMasterPosition input is TRUE, not the current state but the state related to the specified master position is determined. The data obtained is placed into the CamInfo data structure.

##### `MC_ReadCamTableCharacteristics` — §8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `CamTableId` | `MC_CAM_ID` | Table ID [} 55] |
| `Options` | `ST_ReadCamTable Reserved for future extensions CharacteristicsOptio ns Inputs/outputs` | — |
| `CamTableCharac` | `MC_TableCharacV Data structure [} 69] with characteristic parameters of the` | — |
| `alues` | `motion function` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CamTableCharac` | `MC_TableCharacValues` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE, if the calculation was carried out |

**Description:** The function block MC_ReadCamTableCharacteristics is used to calculate and read the characteristic parameters of a motion function. This includes minimum and maximum values of position, velocity, acceleration and jerk.

##### `MC_ReadCamTableMasterPosition` — §8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute input. |
| `CamTableId` | `MC_CAM_ID` | Table ID [} 55] |
| `SlavePosition` | `LREAL` | The slave position for which the master position is sought. |
| `MasterOffset` | `LREAL` | Offset to the master position of the cam plate |
| `SlaveOffset` | `LREAL` | Offset to the slave position of the cam plate |
| `MasterScaling` | `LREAL` | Scaling of the master position of the cam plate |
| `SlaveScaling` | `LREAL` | Scaling of the slave position of the cam plate |
| `MasterStartPosition` | `LREAL` | Start position of the master |
| `MasterAccuracy` | `LREAL` | Accuracy for the calculation |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Becomes TRUE, if the coupling was successful and the |

**Description:** The function block MC_ReadCamTableMasterPosition can be used to calculate the master position for a given slave position. While the slave position for a given master position must be unique, the inverse is not true. In order to limit the number of master output options for the function block, for a given master position (MasterStartPosition) the smaller (MasterLow) and larger master position (MasterHigh) for the slave value is output.
