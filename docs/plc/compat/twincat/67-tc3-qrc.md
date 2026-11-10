# Tc3_QRC

- **Manual:** `QRC_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Industry · **Tier:** T0 · **API entries:** 60
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** QR code generation: pure encoding/bitmap math.

---

#### Programming › Function Blocks

##### `FB_Connect` (FUNCTION_BLOCK) — §4.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Is TRUE as long as the asynchronous request is still |

**Description:** This function block enables to establish or terminate a TCP connection.

#### Programming › Function Blocks › FB_Connect

##### `FB_init` — §4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrvNetID` | `T_AmsNetID := ''` | — |
| `sRemoteHost` | `T_IPv4Addr := '127.0.0.1'` | — |
| `tReconnect` | `TIME       := T#30s` | — |
| `iResExtract` | `I_ResExtract` | — |
| `sSrvNetID` | `AMS Net Id. For the local computer (default) an empty string may be specified.` | — |
| `sRemoteHost` | `Target IPv4 address.` | — |
| `tReconnect` | `Cooldown time for recreating a TCP connection after a TCP connection has been terminated.` | — |
| `iResExtract` | `The function block that implements the interface I_ResExtract [} 18].` | — |

**Description:** Syntax Method FB_init : BOOL

##### `I_Connect` (PROPERTY) → `ARRAY[0..QRC_NUMBER_OF_CONTROL]` — §4.1.1.2

**Return type:** `ARRAY[0..QRC_NUMBER_OF_CONTROL]`

#### Programming › Function Blocks

##### `FB_QRC_ResExtract` (FUNCTION_BLOCK) — §4.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Is set if an error occurs during the execution of the function block. |

**Description:** This function block enables the extraction of the received QRC frames.

#### Programming › Function Blocks › FB_QRC_ResExtract

##### `sEngineStatus` (PROPERTY) → `T_MaxString` — §4.1.2.1

**Return type:** `T_MaxString`

**Description:** This property enables to query the status information of the Q-SYS device.

##### `Clear` (METHOD) → `BOOL` — §4.1.2.2

**Return type:** `BOOL`

**Description:** This method enables clearing all saved snapshot properties that were queried via Poll [} 30] or AutoPoll [} 31].

##### `I_ResExtract` (PROPERTY) → `ARRAY[0..QRC_NUMBER_OF_CONTROL]` — §4.1.2.3

**Return type:** `ARRAY[0..QRC_NUMBER_OF_CONTROL]`

#### Programming › Function Blocks › FB_QRC_ResExtract › I_ResExtract

##### `ResExtract` (METHOD) → `BOOL` — §4.1.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSavOldRes` | `BOOL` | — |
| `bSavOldRes` | `This variable determines whether the referenced function block will save past control` | — |

**Description:** This method enables to extract control properties from a QRC response frame.

#### Programming › Function Blocks › QRC Commands

##### `FB_QRC_BasicCommand` (FUNCTION_BLOCK) — §4.1.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Set when an error has occurred. Error details are located |

**Description:** This function block enables the coding of a QRC basic command.

#### Programming › Function Blocks › QRC Commands › FB_QRC_BasicCommand

##### `FB_init` — §4.1.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iConnect` | `I_Connect` | — |
| `iConnect` | `The function block that implemented the interface I_Connect [} 15].` | — |

**Description:** Syntax Method FB_init : BOOL

##### `LogOn` (METHOD) → `I_Connect` — §4.1.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sUserName` | `STRING` | — |
| `nPassword` | `UDINT` | — |
| `sUserName` | `User name.` | — |
| `nPassword` | `Password.` | — |

**Description:** This method enables to log on the Q-SYS device.

##### `NoOp` (METHOD) → `I_Connect` — §4.1.3.1.3

**Return type:** `I_Connect`

**Description:** This method enables to keep a TCP connection alive.

##### `StatusGet` (METHOD) → `I_Connect` — §4.1.3.1.4

**Return type:** `I_Connect`

**Description:** This method enables to query status information of the Q-SYS device.

#### Programming › Function Blocks › QRC Commands

##### `FB_QRC_Control` (FUNCTION_BLOCK) — §4.1.3.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Set when an error has occurred. Error details are located |

**Description:** This function block enables the coding of QRC frames that are used to set or get control properties via

#### Programming › Function Blocks › QRC Commands › FB_QRC_Control

##### `FB_init` — §4.1.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iConnect` | `I_Connect` | — |
| `iConnect` | `The function block that implemented the interface I_Connect [} 15].` | — |

**Description:** Syntax

##### `Get` (METHOD) → `I_Connect` — §4.1.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aControlName` | `ARRAY[0..QRC_NUMBER_OF_CONTROL] OF STRING` | — |
| `aControlName` | `List of target Named Controls that will be queried.` | — |

**Description:** This method enables encoding of the QRC frames which are used for getting controls' properties via Named Controls.

##### `Set` (METHOD) → `I_Connect` — §4.1.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stControlValue` | `ST_Control` | — |
| `stControlValue` | `Properties of the target Named Control` | — |

**Description:** This method enables encoding of the QRC frames which are used for setting control properties via Named Control.

#### Programming › Function Blocks › QRC Commands

##### `FB_QRC_Component` (FUNCTION_BLOCK) — §4.1.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Set when an error has occurred. Error details are located |

**Description:** This function block enables the coding of the QRC frames that are used to set/get control properties via Named Component. It is also used to list all existing components via Named Component.

#### Programming › Function Blocks › QRC Commands › FB_QRC_Component

##### `FB_init` — §4.1.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iConnect` | `I_Connect` | — |
| `iConnect` | `The function block that implemented the interface I_Connect [} 15].` | — |

**Description:** Syntax Method FB_init : BOOL

##### `Set` (METHOD) → `I_Connect` — §4.1.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sComponentName` | `STRING` | — |
| `aControlValue` | `ARRAY[0..QRC_NUMBER_OF_CONTROL] OF ST_Control` | — |
| `sComponentName` | `Name property of target Named Component.` | — |
| `aControlValue` | `Target controls' properties of the Named Component.` | — |

**Description:** This method enables encoding a QRC frame that is used for setting one or more controls' properties of a

##### `Get` (METHOD) → `I_Connect` — §4.1.3.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sComponentName` | `STRING` | — |
| `aControlName` | `ARRAY[0..QRC_NUMBER_OF_CONTROL] OF STRING` | — |
| `sComponentName` | `Name property of target Named Component.` | — |
| `aControlName` | `Target controls' name of the Named Component.` | — |

**Description:** This method enables encoding a QRC frame that is used for getting one or more controls' properties on a

##### `GetComponent` (METHOD) → `I_Connect` — §4.1.3.3.4

**Return type:** `I_Connect`

**Description:** This method enables encoding a QRC frame that is used to get controls’ properties of all available Named Components.

#### Programming › Function Blocks › QRC Commands

##### `FB_QRC_ChangeGroup` (FUNCTION_BLOCK) — §4.1.3.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Set when an error has occurred. Error details are located |

**Description:** This function block enables the coding of the QRC frames that are used to edit or poll a change group.

#### Programming › Function Blocks › QRC Commands › FB_QRC_ChangeGroup

##### `FB_init` — §4.1.3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iConnect` | `I_Connect` | — |
| `iConnect` | `The function block that implemented the interface I_Connect [} 15].` | — |

**Description:** Syntax Method FB_init : BOOL

##### `AddControl` (METHOD) → `I_Connect` — §4.1.3.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sChangeGroupId` | `STRING` | — |
| `aControlName` | `ARRAY[0..QRC_NUMBER_OF_CONTROL] OF STRING` | — |
| `sChangeGroupId` | `Change group ID.` | — |
| `aControlName` | `List of target Named Controls.` | — |

**Description:** This method enables encoding a QRC frame that is used to add one or more controls via Named Control in a change group.

##### `AddComponentControl` (METHOD) → `I_Connect` — §4.1.3.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sChangeGroupId` | `STRING` | — |
| `sComponentName` | `STRING` | — |
| `aControlName` | `ARRAY[0..QRC_NUMBER_OF_CONTROL] OF STRING` | — |
| `sChangeGroupId` | `Change group ID.` | — |
| `sComponentName` | `Name property of target Named Component.` | — |
| `aControlName` | `Target controls' names of the Named Component.` | — |

**Description:** This method enables encoding a QRC frame that is used to add one or more controls within a Named Component in a change group.

##### `Remove` (METHOD) → `I_Connect` — §4.1.3.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sChangeGroupId` | `STRING` | — |
| `aControlName` | `ARRAY[0..QRC_NUMBER_OF_CONTROL] OF STRING` | — |
| `sChangeGroupId` | `Change group ID.` | — |
| `aControlName` | `Target controls' names.` | — |

**Description:** This method enables encoding a QRC frame that is used to remove one or more Named Controls from a change group.

##### `Poll` (METHOD) → `I_Connect` — §4.1.3.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sChangeGroupId` | `STRING` | — |
| `sChangeGroupId` | `Change group ID.` | — |

**Description:** This method enables encoding a QRC frame that is used to poll a change group.

##### `Destroy` (METHOD) → `I_Connect` — §4.1.3.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sChangeGroupId` | `STRING` | — |
| `sChangeGroupId` | `Change group ID.` | — |

**Description:** This method enables encoding a QRC frame that is used to destroy a change group. This change group will no longer exist.

##### `Clear` (METHOD) → `I_Connect` — §4.1.3.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sChangeGroupId` | `STRING` | — |
| `sChangeGroupId` | `Change group ID.` | — |

**Description:** This method enables encoding a QRC frame that is used to delete all Named Controls / Name Components from a change group. This change group is still existing.

##### `Invalidate` (METHOD) → `I_Connect` — §4.1.3.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sChangeGroupId` | `STRING` | — |
| `sChangeGroupId` | `Change group ID.` | — |

**Description:** This method enables encoding a QRC frame that is used to set all Named Controls / Name Components to "Dirty" state.

##### `AutoPoll` (METHOD) → `I_Connect` — §4.1.3.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sChangeGroupId` | `STRING` | — |
| `fRate` | `REAL` | — |
| `sChangeGroupId` | `Change group ID.` | — |
| `fRate` | `Polling interval in seconds. The minimum value of it is 0.1s.` | — |

**Description:** This method enables encoding a QRC frame that is used to set all Named Controls / Name Components to the "Dirty" state.

##### `AddSnapshotControl` (METHOD) → `I_ResExtract` — §4.1.3.4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sChangeGroupId` | `STRING` | — |
| `sComponentName` | `STRING` | — |
| `aSnapshotNr` | `ARRAY [0..23] OF USINT` | — |

**Description:** This method enables the encoding of a QRC frame that is used for joining multiple snapshots in a change control.

#### Programming › Function Blocks › QRC Commands

##### `FB_QRC_Mixer` (FUNCTION_BLOCK) — §4.1.3.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Set when an error has occurred. Error details are located |

**Description:** This function block allows several different values to be set on a named mixer.

#### Programming › Function Blocks › QRC Commands › FB_QRC_Mixer

##### `FB_init` — §4.1.3.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iConnect` | `I_Connect` | — |
| `iConnect` | `The function block that implemented the interface I_Connect [} 15].` | — |

**Description:** Syntax Method FB_init : BOOL

##### `SetCrossPointGain` (METHOD) → `I_Connect` — §4.1.3.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCrossSpec` | `ST_CrossSpec` | — |
| `stCrossSpec` | `Crosspoint gain properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to set the crosspoint gain value for the inputs and outputs of a named mixer.

##### `SetCrossPointDelay` (METHOD) → `I_Connect` — §4.1.3.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCrossSpec` | `ST_CrossSpec` | — |
| `stCrossSpec` | `Crosspoint delay properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to set the crosspoint delay value for inputs and outputs of a named mixer.

##### `SetCrossPointMute` (METHOD) → `I_Connect` — §4.1.3.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCrossSpec` | `ST_CrossSpec` | — |
| `stCrossSpec` | `Crosspoint mute properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to set the crosspoint muted or unmuted for inputs and outputs of a named mixer.

##### `SetCrossPointSolo` (METHOD) → `I_Connect` — §4.1.3.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCrossSpec` | `ST_CrossSpec` | — |
| `stCrossSpec` | `Crosspoint solo properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to enable or disable crosspoint solo for inputs and outputs of a named mixer.

##### `SetInputGain` (METHOD) → `I_Connect` — §4.1.3.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInputSpec` | `ST_InputSpec` | — |
| `stInputSpec` | `Input gain properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to set gain value for inputs of a named mixer.

##### `SetInputMute` (METHOD) → `I_Connect` — §4.1.3.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInputSpec` | `ST_InputSpec` | — |
| `stInputSpec` | `Input mute properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to set inputs muted or unmuted of a named mixer.

##### `SetInputSolo` (METHOD) → `I_Connect` — §4.1.3.5.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInputSpec` | `ST_InputSpec` | — |
| `stInputSpec` | `Input solo properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to enable or disable solo for inputs of a named mixer.

##### `SetOutputGain` (METHOD) → `I_Connect` — §4.1.3.5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stOutputSpec` | `ST_OutputSpec` | — |
| `stOutputSpec` | `Output gain properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to set gain value for outputs of a named mixer.

##### `SetOutputMute` (METHOD) → `I_Connect` — §4.1.3.5.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stOutputSpec` | `ST_OutputSpec` | — |
| `stOutputSpec` | `Output mute properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to mute or unmute for outputs of a named mixer.

##### `SetCueMute` (METHOD) → `I_Connect` — §4.1.3.5.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCueSpec` | `ST_CueSpec` | — |
| `stCueSpec` | `Cue mute properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to mute or unmute for mixer cues.

##### `SetCueGain` (METHOD) → `I_Connect` — §4.1.3.5.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCueSpec` | `ST_CueSpec` | — |
| `stCueSpec` | `Cue gain properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to set gain value for mixer cues.

##### `SetInputCueEnable` (METHOD) → `I_Connect` — §4.1.3.5.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInputCueSpec` | `ST_InputCueSpec` | — |
| `stInputCueSpec` | `Input and cue properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to enable or disable cues and inputs of named mixer.

##### `SetInputCueAfi` (METHOD) → `I_Connect` — §4.1.3.5.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInputCueSpec` | `ST_InputCueSpec` | — |
| `stInputCueSpec` | `Cue AFL properties.` | — |

**Description:** This method enables encoding a QRC frame that is used to enable or disable cue AFL (After Fader Level) for mixer inputs.

#### Programming › Function Blocks › QRC Commands

##### `FB_QRC_LoopPlayer` (FUNCTION_BLOCK) — §4.1.3.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Set when an error has occurred. Error details are located |

**Description:** This function block enables the query of a file playback on a named loop player.

#### Programming › Function Blocks › QRC Commands › FB_QRC_LoopPlayer

##### `FB_init` — §4.1.3.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iConnect` | `I_Connect` | — |
| `iConnect` | `The function block that implemented the interface I_Connect [} 15].` | — |

**Description:** Syntax Method FB_init : BOOL

##### `Start` (METHOD) → `I_Connect` — §4.1.3.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stJobSpec` | `ST_JobSpec` | — |
| `stJobSpec` | `Properties of the job that will be played back on a named loop player.` | — |

**Description:** This method enables to start playing on a named loop player.

##### `Stop` (METHOD) → `I_Connect` — §4.1.3.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sName` | `STRING` | — |
| `aOutput` | `ARRAY[0..QRC_NUMBER_OF_CONTROL] OF USINT` | — |
| `bLog` | `BOOL := FALSE` | — |
| `sName` | `The name of the loop player.` | — |
| `aOutput` | `Array of output channels.` | — |
| `bLog` | `Optional attribute for event message, FALSE in default.` | — |

**Description:** This method enables to stop playback on a named loop player.

##### `Cancel` (METHOD) → `I_Connect` — §4.1.3.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sName` | `STRING` | — |
| `aOutput` | `ARRAY[0..QRC_NUMBER_OF_CONTROL] OF USINT` | — |
| `bLog` | `BOOL := FALSE` | — |
| `sName` | `The name of the loop player.` | — |
| `aOutput` | `Array of output channels.` | — |
| `bLog` | `Optional attribute for event message, FALSE in default.` | — |

**Description:** This method enables to cancel a job on a named loop player.

#### Programming › Function Blocks › QRC Commands

##### `FB_QRC_Snapshot` (FUNCTION_BLOCK) — §4.1.3.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Set when an error has occurred. Error details are located |

**Description:** This function block enables the encoding of QRC frames that are used to load/save snapshots. In addition, it can be used to get multiple snapshot states.

#### Programming › Function Blocks › QRC Commands › FB_QRC_Snapshot

##### `Load` (METHOD) → `I_Connect` — §4.1.3.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sBankName` | `STRING` | — |
| `nSnapshotNr` | `USINT` | — |
| `fRamp` | `REAL := 0; (*Optional*)` | — |

**Description:** This method enables the encoding of a QRC frame that is used for triggering a snapshot.

##### `Save` (METHOD) → `I_Connect` — §4.1.3.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sBankName` | `STRING` | — |
| `nSnapshotNr` | `USINT` | — |
| `fRamp` | `REAL := 0; (*Optional*)` | — |

**Description:** This method enables the encoding of a QRC frame that is used for saving a snapshot.

##### `GetSnapshotState` (METHOD) → `I_Connect` — §4.1.3.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sComponentName` | `STRING` | — |
| `aSnapshotNr` | `ARRAY[0..23] OF USINT` | — |

**Description:** Each snapshot state can be determined by two related controls within the snapshot component. This method enables the encoding of a QRC frame that is used for querying multiple snapshot states.

#### Programming › Interfaces

##### `I_Connect` (PROPERTY) → `ARRAY[0..QRC_NUMBER_OF_CONTROL]` — §4.3.1

**Return type:** `ARRAY[0..QRC_NUMBER_OF_CONTROL]`

##### `I_ResExtract` (PROPERTY) → `ARRAY[0..QRC_NUMBER_OF_CONTROL]` — §4.3.2

**Return type:** `ARRAY[0..QRC_NUMBER_OF_CONTROL]`

#### Programming › Interfaces › I_ResExtract

##### `ResExtract` (METHOD) → `BOOL` — §4.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSavOldRes` | `BOOL` | — |
| `bSavOldRes` | `This variable determines whether the referenced function block will save past control` | — |

**Description:** This method enables to extract control properties from a QRC response frame.
