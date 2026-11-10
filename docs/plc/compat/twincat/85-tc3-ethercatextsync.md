# Tc3_EtherCATExtSync

- **Manual:** `TF6225_TC3_EtherCAT_External_Sync_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 2
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** EtherCAT DC (distributed clocks) sync needs master sync hooks.

---

#### PLC API › Function blocks

##### `FB_EcExtSyncExtTimes` — §5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AMSNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bTS1Done` | `BOOL` | This output is set when the synchronization is enabled for the |

**Description:** The function block FB_EcExtSyncExtTimes enables or disables the synchronization of the internal and external clocks. The function block FB_EcExtSyncIsSynchronized [} 15] can be used to check synchronization.

##### `FB_EcExtSyncIsSynchronized` — §5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nSyncWindow` | `UDINT` | Time window [ns] within which the internal and external time |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSynchronized` | `BOOL` | TRUE if the difference between the internal and external time |

**Description:** This function block FB_EcExtSyncIsSynchronized checks whether the internal and external clocks are synchronized (bSynchronized). For this purpose, the system checks whether the difference between the clocks over 10 cycles is within the specified synchronization window nSyncWindow. In the event of invalid data (bNotConnected = TRUE), the synchronization is reported as failed.
