# Tc2_CncPlcopenP4

- **Manual:** `TF5200_JOBMANAGER_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/CNC · **Tier:** T4 · **API entries:** 1
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** PLCopen P4 (packaging): state machines are T0 logic; full conformance needs the motion/packaging backends.

---

#### PLCopen function blocks

##### `MCV_GrpReadJobAck` — §5.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `JobState` | `MCV_JOB_STATE                    Job information` | — |

**Description:** The function block is used to evaluate the end or abort operations of jobs commanded by an MC_MovePath [} 27] . The instance of the MC_MovePath [} 27] whose job acknowledgements are evaluated is identified via the “Fb” input.
