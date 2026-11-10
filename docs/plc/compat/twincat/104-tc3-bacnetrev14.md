# Tc3_BACnetRev14

- **Manual:** `TF8020_TC3_BACnet_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 3
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** BACnet/IP: UDP socket service + BACnet/APDU codec.

---

#### Available › The client FB Code dialog

##### `The FB Code window` (FUNCTION_BLOCK) — §7.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Client` | `FB_BACnet_Client:=` | — |
| `_101_Novos_Touch_BACnet_MSTP` | `FB_BACnetRM_Device:=` | — |
| `Internal_Fan_Stage` | `FB_BACnetRM_MI :=` | — |
| `ECO_Colour` | `FB_BACnetRM_MV :=` | — |
| `External_Fan_Stage` | `FB_BACnetRM_MV :=` | — |

**Description:** This window displays the code generated from the object information of the selected device in structured text (ST).

#### Manual Operator, e. g. BMS

##### `Creating and deleting own BACnet function blocks (FB)` (FUNCTION_BLOCK) — §8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInCopyCode` | `BOOL; // if TRUE, the exit method is called for exiting an instance that is copied` | — |

**Description:** If own BACnet FBs are to be instantiated in addition to the FBs available in the Tc3_BACnetRev14 library, these must be deleted in a method FB_exit when the PLC program is terminated so that the dynamically allocated memory is released again.

#### Available

##### `Receiving alarms and events from other devices` (FUNCTION_BLOCK) — §9.9

**Description:** This example shows the use of an Eventlog object as a receiver for alarms and events of an external BACnet device. The Eventlog object of the alarm receiver is entered in the notification class of the alarm transmitter. The assignment is made via the process identifier.
