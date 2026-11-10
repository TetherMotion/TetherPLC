# Tc3_MlServer

- **Manual:** `TF3820_TC3_Machine_Learning_Server_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Measurement · **Tier:** T4 · **API entries:** 1
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** ML inference server: external engine plus IPC transport.

---

#### Function blocks

##### `FB_MlSvrPrediction` (FUNCTION_BLOCK) — §6.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stPredictionParamete ST_PredictionParameter` | `Configuration structure of the Machine Learning Server` | — |
| `r [} 42]` | `session` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if the currently pending asynchronous request to the |

**Description:** FB_MlSvrPrediction is a TcMlServer client that provides asynchronous and optionally hardware- accelerated AI model inference for the PLC.
