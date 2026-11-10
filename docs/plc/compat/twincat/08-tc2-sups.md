# Tc2_SUPS

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_SUPS_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T2 · **API entries:** 4
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** UPS monitoring needs serial/USB HID access to UPS devices.

---

#### (top level)

##### `Function blocks` — §4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wo writing persistent data, w/wo shutdown) *) 10 Function blocks` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPowerFailDetect BOOL` | `TRUE during power failure. FALSE if the supply voltage is` | — |

**Description:** The function block FB_S_UPS_CB3011 can be used on PCs with a CB3011 board with 1-second UPS, in order to control the 1-second UPS from the PLC. In the event of a power failure, this enables the persistent data to be saved and/or a quick shutdown to be executed, depending on the selected mode. The default input values of the FB_S_UPS_CB3011 should be retained.

#### Function blocks

##### `FB_S_UPS` — §4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wo writing persistent data, w/wo shutdown) *) wo writing persistent data, w/wo shutdown) *) wo writing persistent data, w/wo shutdown) *)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPowerFailDetect BOOL` | `TRUE during power failure. FALSE if the supply voltage is` | — |
| `bPowerFailDetect BOOL` | `TRUE during power failure. FALSE if the supply voltage is` | — |
| `bPowerFailDetect BOOL` | `TRUE during power failure. FALSE if the supply voltage is` | — |

**Description:** The function block FB_S_UPS can be used on CX50x0 Embedded PCs with 1-second UPS, in order to control the 1-second UPS from the PLC. In the event of a power failure, this enables the persistent data to be saved and/or a quick shutdown to be executed, depending on the selected mode. The default input values of the FB_S_UPS should be retained.

#### Function blocks › BAPI

##### `FB_S_UPS_BAPI` — §4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wo writing persistent data, w/wo shutdown) *)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPowerFailDetect BOOL` | `TRUE during power failure. FALSE if the supply voltage is` | — |

**Description:** The function block FB_S_UPS_BAPI can be used on devices with 1-second UPS and with BIOS-API from version v1.15, in order to control the 1-second UPS from the PLC.

#### Function blocks

##### `FB_NT_QuickShutdown` — §4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | AmsNetID of the controller |
| `START` | `BOOL` | Rising edge leads to immediate reboot of the controller. |
| `TMOUT` | `TIME` | Timeout time |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | Quick shutdown is executed. |
| `ERR` | `BOOL` | Becomes TRUE, as soon as an error occurs. |
| `ERRID` | `UDINT` | Supplies the error number when the ERR output is set. |

**Description:** Loss of data The function block FB_NT_QuickShutdown is used internally by the various FB_S_UPS blocks. It must not be used independently, because this could result in data loss!
