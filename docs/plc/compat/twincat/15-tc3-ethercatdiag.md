# Tc3_EtherCATDiag

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_EtherCATDiag_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T2 · **API entries:** 2
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** EtherCAT diagnostics need master control-plane hooks (AL state, SII, PDO config). Tether (sibling repo) has an EtherCAT master; bridging is cross-repo work.

---

#### Function blocks

##### `FB_CoE_DiagHistory_Read` — §3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | NetID of the EtherCAT master |
| `nSlaveAddr` | `UINT` | Port address of the slave |
| `bEnable` | `BOOL` | Pulse: the Diag History is read out once. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aDiagHistory` | `ARRAY[*] OF ST_CoE_DiagHistory_Entry` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Simple busy signal, indicates that the function block is |

**Description:** The function block FB_ReadCoEDiagHistory can read EtherCAT slaves that support the CoE Diag History Object. The EtherCAT slave must comply with the ETG standard.

##### `FB_CoE_DiagHistory_ReadAxis` — §3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | This input is not used and should not be assigned. |
| `nSlaveAddr` | `UINT` | This input is not used and should not be assigned. |
| `bEnable` | `BOOL` | Pulse: the Diag History is read out once. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aDiagHistory` | `ARRAY[*] OF ST_CoE_DiagHistoryEntry` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Simple busy signal, indicates that the function block is |

**Description:** The function block FB_ReadCoEDiagHistory_Axis is a variation of the function block FB_ReadCoEDiagHistory [} 9], which can be assigned an AXIS_REF structure instead of an address. Only messages that belong either to the device or to the specific axis are output (e.g. for an AX8206 dual-axis module).
