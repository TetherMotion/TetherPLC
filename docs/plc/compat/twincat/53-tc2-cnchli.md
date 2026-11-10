# Tc2_CncHli

- **Manual:** `TF5200_HLI_INTERFACE_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/CNC · **Tier:** T3 · **API entries:** 9
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** CNC HLI (OPC UA companion spec for machines): axis backend + OPC UA server.

---

#### Axes › Description of the axis-specific interface

##### `Axis control commands` — §2.3.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ST element` | `.state_r Data type` | BOOL |
| `Value range` | `[TRUE = drive enable, FALSE = drive STOP] Redirection ST element` | .enable_w [} 16] Feedhold ON/OFF, axis |
| `Description` | `Axis-specific feedhold` | — |

**Description:** Feed rate enable, axis Special features     SERCOS drives Commanded, requested and return values ST element       .command_w

#### (top level)

##### `HLI_DIRECTIONAL_FEEDHOLD_ The axis is blocked in the` — §3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ST element` | `.state_r Data type` | BOOL Value range [TRUE = drive ON, FALSE = drive OFF] Redirection ST element .enable_w [} 16] Controller enable |
| `Description` | `Control unit enable ↔ axis-specific torque connection.` | — |
| `Data type` | `MC_CONTROL_BOOL_UNIT, see description of Control unit [} 15] Special features` | SERCOS drives |

**Description:** Redirection

#### Axes › Description of the axis-specific interface › Axis control commands

##### `Commanding axis couplings` — §2.3.6.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ST element` | `.state_r Signal flow` | CNC → PLC Data type HLI_STATE_AXIS_COUPLING [} 55] |
| `Access` | `PLC is reading Redirection ST element` | .enable_w [} 16] Axis couplings, state |
| `Description` | `Indicates whether the axis couplings are active for this axis and if so, which ones.` | — |
| `ST path` | `gpAx[axis_idx]^.lr_mc_control.axis_coupling.state_r` | — |
| `Access` | `PLC reads Elements of the data type` | — |
| `Element` | `.desc[ ]` | — |
| `Data type` | `ARRAY[0..HLI_AX_COUPLING_MAXIDX] OF HLI_AXIS_COUPLING_DESC [} 57]` | — |
| `Access` | `PLC reads` | — |
| `Element` | `.active_r Data type` | BOOL |
| `Access` | `PLC reads Axis couplings, command` | — |
| `Description` | `This entry defines a coupling all coupling specification for the axis. The maximum` | — |

**Description:** Command axis couplings Description     This control unit defines axis couplings for each axis. The motion of the axis can Data type       MC_CONTROL_AXIS_COUPLING, see description of Control unit [} 15] Commanded values ST element       .command_w Signal flow      PLC → CNC

#### Axes › Description of the axis-specific interface

##### `Control commands of a drive` — §2.3.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ST path` | `.state_r Data type` | UDINT Redirection ST path .enable_w [} 16] |
| `Special feature` | `Available as of Build V3.1.3081.21 or V3.1.3135.0` | — |

**Description:** Read/write drive data cyclically

#### HLI_AXIS_ACTIVE The axis is currently moved by the CNC due to › Description of the spindle-specific interface

##### `Control commands of a spindle` — §3.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ST element` | `.state_r Data type` | BOOL |
| `Value range` | `[TRUE = spindle reset is executed, FALSE] Redirection ST element` | .enable_w [} 16] Spindle error |
| `Description` | `Sets the spindle to error state` | — |
| `Data type` | `MC_CONTROL_BOOL_UNIT, see description of Control unit [} 15]` | — |
| `Access` | `PLC reads request_r + state_r and writes command_w + enable_w ST path` | gpAx[axis_idx]^.ipo_mc_control.spdl_error Commanded, requested and return values ST element .command_w .request_r .state_r Data type BOOL Value range [TRUE, FALSE] |
| `Description` | `Commanding this control unit is only effective if the axis is a spindle.` | — |

**Description:** Spindle stop at program end Commanded, requested and return values ST element       .command_w Redirection

#### (top level)

##### `Must: copy` — §1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ST element` | `.state_r Data type` | UDINT |
| `Unit` | `1 μm/s` | — |
| `Special features` | `The state_r element indicates the path velocity actually used in the interpolator,` | — |

**Description:** request

#### HLI_AXIS_HOLD Axis cannot be moved by the CNC since › Control commands to PLC

##### `Reset` — §5.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ST element` | `.state_w Signal flow` | PLC → CNC Data type BOOL Value range [TRUE = PLC executed reset, FALSE] Request ST element .enable_w [} 16] Signal flow PLC → CNC Data type BOOL |
| `Value range` | `[TRUE = PLC wants to be notified about requests by CNC, FALSE] PLC reset, channel` | — |
| `Description` | `The PLC can be requested to execute a reset via this channel-specific interface. ST element` | .state_w Signal flow PLC → CNC Data type BOOL Value range [TRUE = PLC executed reset, FALSE] Request ST element .enable_w [} 16] Signal flow PLC → CNC Data type BOOL |
| `Value range` | `[TRUE = PLC wants to be notified about requests by CNC, FALSE]` | — |

**Description:** PLC reset, axis Special features    Consumption data item Commanded value

#### Error messages

##### `Activating error filters` — §12.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ST element` | `.state_r Data type` | UDINT Redirection ST element .enable_w [} 16] Activate error filter rules - channel |
| `Description` | `Here, the individual error filters can be activated/deactivated by the 32-bit mask according ST element` | .state_r Data type UDINT Redirection ST element .enable_w [} 16] Activate error filter rules - axis |
| `Description` | `Here, the individual error filters can be activated/deactivated by the 32-bit mask according ST element` | .state_r Data type UDINT Redirection ST element .enable_w [} 16] HLI-Interface Platform data |

**Description:** Fig. 32: Activating the filter

#### Virtual drive › PLC main program frame

##### `Initialisation function UserInitialisations()` (FUNCTION) → `BOOL` — §16.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dummy` | `BOOL := FALSE; (* not_used *)` | — |
