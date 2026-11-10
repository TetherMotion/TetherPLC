# Tc3_Physics

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_Physics_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/MC3 · **Tier:** T0 · **API entries:** 38
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Physics helpers (kinematics/dynamics math for simulation): pure math; useful for a TetherPLC motion simulator.

---

#### Functions › Coordinates

##### `GetMcsCoordinateType` (FUNCTION) → `CoordinateType` — §3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `mcsType` | `E_McsCoordType [} 8]` | Mcs coordinate type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CoordinateType [} 9]` | `` | — |

**Description:** Creates a CoordinateType FB from E_McsCoordType.

##### `GetUninterpretedCoordinateType` (FUNCTION) → `CoordinateType` — §3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `mcsType` | `UDINT` | Mcs coordinate type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CoordinateType [} 9]` | `` | — |

**Description:** Creates a CoordinateType FB from an index.

#### Function Blocks › Dynamics › DynamicConstraint_CartesianXY

##### `SetValuesVAJ` (METHOD) — §4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `V` | `MC_LREAL` | Maximum velocity. |
| `A` | `MC_LREAL` | Maximum acceleration. |
| `J` | `MC_LREAL` | Maximum jerk. |

**Description:** Set the dynamic limits of this instance.

#### Function Blocks › Dynamics › DynamicConstraint_CartesianXYZ

##### `SetValuesVAJ` (METHOD) — §4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `V` | `MC_LREAL` | Maximum velocity. |
| `A` | `MC_LREAL` | Maximum acceleration. |
| `J` | `MC_LREAL` | Maximum jerk. |

**Description:** Set the dynamic limits of this instance.

#### Function Blocks › Dynamics › DynamicConstraint_Container

##### `Clear` (METHOD) — §4.1.3.1

**Description:** Removes all dynamic constraints from the container.

##### `AddConstraint` (METHOD) — §4.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Constraint` | `Reference To` | A reference to the constraint to be added. IPlcDynamicConstraint |

**Description:** Adds a copy of a dynamic constraint to the container. If the dynamic constraint changes afterwards, the value in the container will not reflect that change.

##### `AddConstraintByReference` (METHOD) — §4.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Constraint` | `Reference To` | A reference to the constraint to be added. IPlcDynamicConstraint |

**Description:** Adds a reference to a dynamic constraint to the container. If the dynamic constraint changes afterwards, the value in the container will reflect that change.

#### Function Blocks › Dynamics › DynamicConstraint_Coordinates

##### `AddLimit` (METHOD) — §4.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Coordinate` | `CoordinateType [} 9]` | Coordinate type. E.g. Coord_Mcs_X, Coord_Mcs_Y, ... |
| `V` | `MC_LREAL` | Maximum velocity. |
| `A` | `MC_LREAL` | Maximum acceleration. |
| `D` | `MC_LREAL` | Maximum deceleration. |
| `J` | `MC_LREAL` | Maximum jerk. |

**Description:** Adds limits for a specific coordinate. If the coordinate is already limited, limits are merged.

##### `SetLimit` (METHOD) — §4.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Coordinate` | `CoordinateType [} 9]` | Coordinate type. E.g. Coord_Mcs_X, Coord_Mcs_Y, ... |
| `V` | `MC_LREAL` | Maximum velocity. |
| `A` | `MC_LREAL` | Maximum acceleration. |
| `D` | `MC_LREAL` | Maximum deceleration. |
| `J` | `MC_LREAL` | Maximum jerk. |

**Description:** Sets limits for a specific coordinate. If the coordinate is already limited, existing limits are overwritten.

##### `RemoveLimit` (METHOD) — §4.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Coordinate` | `CoordinateType [} 9]` | Coordinate type. E.g. Coord_Mcs_X, Coord_Mcs_Y, ... |

**Description:** Removes limits for a specific coordinate.

##### `Clear` (METHOD) — §4.1.4.4

**Description:** Removes limits for all coordinates.

##### `GetV` (METHOD) → `MC_LREAL` — §4.1.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Coordinate` | `CoordinateType [} 9]` | Coordinate type. E.g. Coord_Mcs_X, Coord_Mcs_Y, ... |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MC_LREAL` | `` | — |

**Description:** 20

##### `GetA` (METHOD) → `MC_LREAL` — §4.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Coordinate` | `CoordinateType [} 9]` | Coordinate type. E.g. Coord_Mcs_X, Coord_Mcs_Y, ... |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MC_LREAL` | `` | — |

**Description:** Gets the contained acceleration limit for a specific coordinate. If no limits for the coordinate are set, returns MC_INVALID.

##### `GetD` (METHOD) → `MC_LREAL` — §4.1.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Coordinate` | `CoordinateType [} 9]` | Coordinate type. E.g. Coord_Mcs_X, Coord_Mcs_Y, ... |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MC_LREAL` | `` | — |

**Description:** Gets the contained deceleration limit for a specific coordinate. If no limits for the coordinate are set, returns MC_INVALID.

##### `GetJ` (METHOD) → `MC_LREAL` — §4.1.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Coordinate` | `CoordinateType [} 9]` | Coordinate type. E.g. Coord_Mcs_X, Coord_Mcs_Y, ... |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MC_LREAL` | `` | — |

**Description:** Gets the contained jerk limit for a specific coordinate. If no limits for the coordinate are set, returns MC_INVALID.

#### Function Blocks › Dynamics › DynamicConstraint_PathXY

##### `SetValuesVADJ` (METHOD) — §4.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `V` | `MC_LREAL` | Maximum velocity. |
| `A` | `MC_LREAL` | Maximum acceleration. |
| `D` | `MC_LREAL` | Maximum deceleration. |
| `J` | `MC_LREAL` | Maximum jerk. |

**Description:** Set the dynamic limits of this instance.

#### Function Blocks › Dynamics › DynamicConstraint_PathXYZ

##### `SetValuesVADJ` (METHOD) — §4.1.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `V` | `MC_LREAL` | Maximum velocity. |
| `A` | `MC_LREAL` | Maximum acceleration. |
| `D` | `MC_LREAL` | Maximum deceleration. |
| `J` | `MC_LREAL 24` | Maximum jerk. |

**Description:** Set the dynamic limits of this instance.

#### Function Blocks › Spatial › Positions › PositionXY

##### `SetZero` (METHOD) — §4.2.1.1.1

**Description:** Set the coordinates of this position to '0.0'.

##### `SetValues` (METHOD) — §4.2.1.1.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `positionXY` | `PositionXY` | — |

**Description:** Set the coordinates of this position.

##### `SetValuesXY` (METHOD) — §4.2.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `LREAL` | x-component of the position. |
| `y` | `LREAL` | y-component of the position. |

**Description:** x LREAL y LREAL

##### `SetValuesRP` (METHOD) — §4.2.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `radius` | `LREAL` | Distance from the origin. |
| `phi` | `LREAL` | Polar angle of this position in degrees [°]. |

**Description:** radius LREAL phi LREAL

##### `SetValuesXYReferenceId` (METHOD) — §4.2.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `LREAL` | x-component of the position. |
| `y` | `LREAL` | y-component of the position. |
| `ReferenceId` | `OTCID` | reference id of the position. |

**Description:** x LREAL y LREAL

##### `GetRadius` (METHOD) → `LREAL` — §4.2.1.1.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LREAL` | `` | — |

**Description:** Get the distance from the origin.

##### `GetPhi` (METHOD) → `LREAL` — §4.2.1.1.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LREAL` | `` | — |

**Description:** Get the polar angle scaled in degrees [°].

##### `Invert` (METHOD) — §4.2.1.1.8

**Description:** Invert this position.

##### `ConcatenateWith` (METHOD) — §4.2.1.1.9

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `position` | `PositionXY` | — |

**Description:** Concatenate with a 2nd position.

##### `ShiftByXY` (METHOD) — §4.2.1.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dx` | `LREAL` | Shift of the x-component of this position. |
| `dy` | `LREAL` | Shift of the y-component of this position. |

**Description:** dx LREAL dy LREAL

#### Function Blocks › Spatial › Positions › PositionXYC

##### `SetZero` (METHOD) — §4.2.1.2.1

**Description:** Set the components of this vector to '0.0'.

##### `SetValues` (METHOD) — §4.2.1.2.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `positionXYC` | `PositionXYC` | — |

**Description:** Set the components of this position.

##### `SetValuesXY` (METHOD) — §4.2.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `LREAL` | x-component of the position. |
| `y` | `LREAL` | y-component of the position. |

**Description:** x LREAL y LREAL

##### `SetValuesXYC` (METHOD) — §4.2.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `LREAL` | x-component of the position. |
| `y` | `LREAL` | y-component of the position. |
| `c` | `LREAL` | c-component of the position scaled in degrees [°]. |

**Description:** x LREAL y LREAL c LREAL

##### `SetValuesXYCReferenceId` (METHOD) — §4.2.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `LREAL` | x-component of the position. |
| `y` | `LREAL` | y-component of the position. |
| `c` | `LREAL 34` | c-component of the position scaled in degrees [°]. |

**Description:** x LREAL y LREAL c LREAL

##### `Invert` (METHOD) — §4.2.1.2.6

**Description:** Invert this position.

##### `ConcatenateWith` (METHOD) — §4.2.1.2.7

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `position` | `PositionXYC` | — |

**Description:** Multiply this position by a 2nd from the right.

##### `StepByLocalXY` (METHOD) — §4.2.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dx` | `LREAL` | The displacement within the local x-direction. |
| `dy` | `LREAL` | The displacement within the local y-direction. |

**Description:** dx LREAL dy LREAL

##### `ShiftByXY` (METHOD) — §4.2.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dx` | `LREAL` | Shift of the x-component of this position. |
| `dy` | `LREAL` | Shift of the y-component of this position. |

**Description:** dx LREAL dy LREAL

##### `TurnByC` (METHOD) — §4.2.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dc` | `LREAL` | Angular displacement given in degrees [°]. |

**Description:** dc LREAL

##### `Transform` (METHOD) — §4.2.1.2.11

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `positionXY` | `PositionXY` | — |

**Description:** Place a PositionXY into the coordinate system referenced by this position.
