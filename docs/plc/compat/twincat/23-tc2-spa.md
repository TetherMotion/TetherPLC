# Tc2_SPA

- **Manual:** `TF3900_TC3_Solar_Position_Algorithm_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Measurement · **Tier:** T0 · **API entries:** 1
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Solar position algorithm: pure astronomy math.

---

#### API › PLC Reference › Function blocks

##### `FB_SPA` — §4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `second) *)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fZenith` | `LREAL` | The zenith angle of the sun is defined as the angle |

**Description:** At the input all available values for the location definition and type of calculation are specified.
