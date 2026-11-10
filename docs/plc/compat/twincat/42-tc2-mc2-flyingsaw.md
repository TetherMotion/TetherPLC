# Tc2_MC2_FlyingSaw

- **Manual:** `TF5055_TC3_NC_Flying_Saw_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/NC2 · **Tier:** T3 · **API entries:** 3
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Flying saw: specialized motion kernel on the axis backend.

---

#### Flying saw

##### `MC_GearInVelo` — §5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF` | — |
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartSync` | `BOOL` | Becomes TRUE when the synchronization with the |

**Description:** The function block MC_GearInVelo activates a linear master-slave coupling (gear coupling). If the master axis is already moving, the slave axis synchronizes to the master velocity. The function block accepts a fixed gear ratio in numerator/denominator format.

##### `MC_GearInPos` — §5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The command is executed with a rising edge at Execute |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Master` | `AXIS_REF` | — |
| `Slave` | `AXIS_REF` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartSync` | `BOOL` | Becomes TRUE when the synchronization with the |

**Description:** The function block MC_GearInPos synchronizes a slave axis precisely with a master axis (flying saw). The synchronous velocity is achieved exactly at the synchronous point of the master and slave.

##### `MC_ReadFlyingSawCharacteristics` — §5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge initiates reading the characteristic values |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Slave` | `AXIS_REF` | — |
| `CamTableCharac` | `MC_FlyingSawCharacValues` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | Is set to TRUE when the data set has been successfully |

**Description:** The function block MC_ReadFlyingSawCharacteristics allows the characteristic values for the synchronization phase of the Universal Flying Saw to be read.
