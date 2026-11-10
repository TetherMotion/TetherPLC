# Tc3_PJLink

- **Manual:** `PLC_Lib_PJLink_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Industry · **Tier:** T2 · **API entries:** 2
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** PJLink projector control: TCP socket service (simple text protocol).

---

#### Programming

##### `FB_PJLink` — §3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |
| `sProjector` | `T_IPv4Addr` | — |
| `sServerNetID` | `T_AmsNetId` | — |
| `sSecurityPW` | `STRING(32)` | — |
| `bKeepAliveSocket` | `BOOL := FALSE` | — |
| `tStatusUpdateTime` | `TIME := T#5S` | — |
| `eTraceLevel` | `TcEventSeverity` | — |
| `bEnable` | `Setting this parameter TRUE activates the function module. Communication to a projector is` | — |
| `sProjector` | `The local IP address (IPv4) of the TCP/IP client/server socket as a string (e.g.` | — |
| `sServerNetID` | `String containing the AMS network address of the TwinCAT TCP/IP Connection Server. An` | — |
| `sSecurityPW` | `Password for the Security Authorization function (optional). The identical (case sensitive)` | — |
| `bKeepAliveSocket` | `When TRUE, activates a Keep Alive mode for communication with the projector. Only` | — |
| `tStatusUpdateTime` | `Update interval of the status information request from the projector. (Default=5s; higher` | — |
| `eTraceLevel` | `Determines the event types that are sent via the Event Logger. Events of greater or equal` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `eStep` | `E_PJLink_step` | — |
| `bError` | `BOOL` | — |
| `ipResultMessage` | `I_TcMessage` | — |
| `nErrID` | `UDINT` | — |
| `sErrBlock` | `STRING(20)` | — |
| `stProjStatus` | `ST_PJLink_Projector_Status` | — |
| `bBusy` | `This output is set as long as the connection to the socket is established.` | — |
| `eStep` | `This output parameter specifies the current step in the FB_PJLink State machine.` | — |
| `bError` | `If an error occurs, this output is set. (e.g.: connection to the socket is not possible).` | — |
| `ipResultMessage` | `This interface is used to send events via the TC3 Event Logger.` | — |
| `nErrID` | `This parameter returns the TwinCAT TCP/IP Connection Server error number when the bError` | — |
| `sErrBlock` | `Provides further information on the error that occurred. (Causing function block)` | — |
| `stProjStatus` | `Structure with the projector status information. (After sending a command, the response of the` | — |
| `Example` | `Command = "%1POWR 1$R" → Projector response = "%1POWR=ERR3" →` | — |

**Description:** This function block enables communication to a projector via TCP/IP and can be used for control and monitoring. Communication to a projector is established automatically by the function block. Any information received from the controller is evaluated and displayed in the structure ST_PJLink_Projector_Status (output parameter stProjStatus). In addition, there is an automatic status update function that is executed periodically. Information, warnings and errors are displayed using the TC3 Event Logger.

#### Programming › FB_PJLink

##### `Methods` (METHOD) → `hresult` — §3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bOn` | `BOOL` | — |
| `bExecute` | `BOOL` | — |
| `eMuteType` | `E_PJLINK_MuteType` | — |
| `bMuteOn` | `BOOL` | — |
| `bExecute` | `BOOL` | — |
| `eInputType` | `E_PJLINK_InputType` | — |
| `nInputChannel` | `STRING(1)` | — |
| `bExecute` | `BOOL` | — |
| `eUpdateType` | `E_PJLINK_UpdateType` | — |
| `bExecute` | `BOOL` | — |
| `bFreeze` | `BOOL` | — |
| `bExecute` | `BOOL` | — |
| `bIncrease` | `BOOL` | — |
| `bExecute` | `BOOL` | — |
| `bIncrease` | `BOOL` | — |
| `bExecute` | `BOOL` | — |

**Description:** Power
