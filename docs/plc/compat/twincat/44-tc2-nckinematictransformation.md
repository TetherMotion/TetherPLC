# Tc2_NcKinematicTransformation

- **Manual:** `TF5110-TF5113_TC3_Kinematic_Transformation_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/NC2 · **Tier:** T3 · **API entries:** 12
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Kinematic transformations (robot-like chains): kinematics engine on the axis backend.

---

#### Plc Library › Function Blocks

##### `FB_KinConfigGroup` — §8.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `bCartesianMode BOOL` | `If FALSE, the ACS axes (joint) can be moved directly. If TRUE, the movement described in the MCS axes` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAxesList` | `ST_KinAxes` | — |
| `stKinRefIn` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output becomes TRUE when the command is started |

**Description:** The function block FB_KinConfigGroup is used to configure axes according to the kinematic transformation. These are axes for the ACS (joint) and the MCS (Cartesian). The function block takes the ACS and MCS axes defined in the stAxesList and configures them in the kinematic group of stKinRefIn.

##### `FB_KinResetGroup` — §8.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAxesList` | `ST_KinAxes` | — |
| `stKinRefIn` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output becomes TRUE when the command is started |

**Description:** The kinematics group is reset with the function block FB_KinResetGroup. All ACS and MCS axes are reset. In addition, the input nItpChannelId can be used for specifying the corresponding interpolation channel. The channel is reset, if the nItpChannelId is not 0.

##### `FB_KinCalcTrafo` — §8.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `bForward` | `BOOL` | Determines whether the forward or backward |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAxesPosIn` | `ARRAY[1..8] OF LREAL` | — |
| `stAxesPosOut` | `ARRAY[1..8] OF LREAL` | — |
| `uMetaInfoIn` | `U_KinMetaInfo` | — |
| `uMetaInfoOut` | `U_KinMetaInfo` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output becomes TRUE when the command is started |

**Description:** The function block FB_KinCalcTrafo is used to calculate the forward or backward transformation, even if no kinematic group has been created with FB_KinConfigGroup [} 78].

##### `FB_KinCalcMultiTrafo` — §8.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `bForward` | `BOOL` | — |
| `oidTrafo` | `UDINT` | — |
| `pDataIn` | `Pointer to BYTE` | — |
| `nSizeIn` | `UDINT` | — |
| `pDataOut` | `Pointer to BYTE` | — |
| `nSizeOut` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output becomes TRUE when the command is started |

**Description:** nSizeIn UDINT pDataOut Pointer To BYTE nSizeOut UDINT

##### `FB_KinUnlockTrafoParam` — §8.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `oidTrafo` | `UDINT` | Object ID of the kinematic transformation object. See |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output becomes TRUE when the command is started |

**Description:** The function block FB_KinUnlockTrafoParam is used to unlock transformation parameters that have an influence on the position so that these can be written.

##### `FB_KinLockTrafoParam` — §8.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. |
| `oidTrafo` | `UDINT` | Object ID of the kinematic transformation object. See |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output becomes TRUE when the command is started |

**Description:** Once the transformation parameters have been modified with the aid of FB_KinUnlockTrafoParam [} 86], the function block FB_KinLockTrafoParam locks the transformation parameters again, so that write access is no longer possible.

##### `FB_KinExtendedRotationRange` — §8.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is executed with a rising edge. |
| `oidTrafo` | `UDINT` | Object ID (OTCID) of the kinematic transformation object. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output becomes TRUE when the command is started |

**Description:** The function block FB_KinExtendedRotationRange is used to save and restore the rotation state of the kinematic group.

##### `FB_KinPresetRotation` — §8.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is executed with a rising edge. |
| `oidTrafo` | `UDINT` | Object ID (OTCID) of the kinematic transformation object. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stRotation` | `ARRAY[1..3] OF LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output becomes TRUE when the command is started |

**Description:** The function block FB_KinPresetRotation is used to set the rotational state.

#### Plc Library › Functions

##### `F_KinGetChnOperationState` — §8.2.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stKinRefIn` | `NCTOPLC_NCICHANNEL_REF` | — |

**Description:** This function returns the operating state of the kinematic channel.

##### `F_KinGetAcsMcsAxisIds` (FUNCTION) → `UDINT` — §8.2.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAxesList` | `ST_KinAxes` | — |
| `stKinRefIn` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT: error code Sample` | `` | — |
| `stAxesList` | `: ST_KinAxes; stKinRefIn AT %I*` | : NCTOPLC_NCICHANNEL_REF; |
| `nErrId` | `: UDINT;` | — |

**Description:** This function reads the configured ACS and MCS axes of the cyclic interface. The IDs are written to stAxesList.

##### `F_KinAxesInTolerance` — §8.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stAxesPosIn1` | `ARRAY[1..8] OF LREAL` | — |
| `stAxesPosIn2` | `ARRAY[1..8] OF LREAL` | — |
| `stAxesTolerance` | `ARRAY[1..8] OF LREAL` | — |

**Description:** The function block F_KinAxesInTolerance compares two arrays element by element.

#### Plc Library › Legacy

##### `FB_KinCheckActualStatus` — §8.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is triggered by a rising edge at this input. Inputs/outputs |
| `stAxesList` | `ST_KinAxes` | Determines the ACS and MCS axes included in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stAxesList` | `ST_KinAxes` | — |
| `stKinRefIn` | `NCTOPLC_NCICHANNEL_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eKinStatus` | `E_KINSTATUS` | Returns the status of the kinematic channel. See |

**Description:** The function block FB_KinCheckActualStatus returns the status of the kinematic channel.
