# Tc2_RFID

- **Manual:** `TF6600_TC3_RFID_Reader_Communication_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 1
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** RFID reader communication: serial or TCP transport to vendor readers.

---

#### PLC API › Function block

##### `FB_RFIDReader` (FUNCTION_BLOCK) — §6.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | In order to receive messages from the RFID reader, the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RxBuffer` | `ComBuffer` | — |
| `TxBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | If the command call is valid, the bBusy output goes to TRUE for |

**Description:** The TwinCAT RFID library only consists of one function block.
