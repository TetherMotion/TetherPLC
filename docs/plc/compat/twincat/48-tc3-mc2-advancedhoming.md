# Tc3_MC2_AdvancedHoming

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_MC2_AdvancedHoming_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/NC2 · **Tier:** T3 · **API entries:** 16
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Advanced homing routines: axis backend homing state machines.

---

#### Function blocks › Finalizing functions

##### `MC_FinishHoming` — §3.1.1

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

**Description:** The MC_FinishHoming function block completes a successful homing sequence and ensures that modified parameters are reset to their original values.

##### `MC_HomeDirect` — §3.1.2

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

**Description:** The MC_HomeDirect function block completes a successful homing sequence, resets the axis to a defined value and ensures that modified parameters are reset to their original values. The axis leaves the homing state.

##### `MC_AbortHoming` — §3.1.3

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

**Description:** The MC_AbortHoming function block aborts a homing sequence and ensures that modified parameters are reset to their original values. The axis leaves the homing state.

#### Function blocks › Referencing functions (passive)

##### `MC_AbortPassiveHoming` — §3.2.1

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

##### `MC_StepReferenceFlyingRefPulse` — §3.2.2

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

**Description:** The function block MC_StepReferenceFlyingRefPulse performs referencing during a running movement to the zero pulse of an encoder.

##### `MC_StepReferenceFlyingSwitch` — §3.2.3

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

**Description:** Function blocks

#### Function blocks › Step functions

##### `MC_StepAbsoluteSwitch` — §3.3.1

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

##### `MC_StepAbsoluteSwitchDetection` — §3.3.2

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

##### `MC_StepBlock` — §3.3.3

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

**Description:** This function block performs the search for a physical object that mechanically blocks the movement.

##### `MC_StepBlockDetection` — §3.3.4

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
| `Function blocks` | `` | — |
| `Done` | `BOOL` | Becomes TRUE if the command was completed |

**Description:** This function block performs the search for a physical object that mechanically blocks the movement.

##### `MC_StepBlockLagBased` — §3.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `30 Function blocks` | `` | — |
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

**Description:** This function block performs the search for a physical object that mechanically blocks the movement.

##### `MC_StepBlockLagBasedDetection` — §3.3.6

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

**Description:** This function block performs a search for a physical object that mechanically blocks the movement.

##### `MC_StepLimitSwitch` — §3.3.7

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

**Description:** The function block MC_StepLimitSwitch performs the search for a hardware limit switch.

##### `MC_StepLimitSwitchDetection` — §3.3.8

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

**Description:** The function block MC_StepLimitSwitchDetection performs the search for a hardware limit switch.

##### `MC_StepReferencePulse` — §3.3.9

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

##### `MC_StepReferencePulseDetection` — §3.3.10

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
