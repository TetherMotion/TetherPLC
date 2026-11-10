# Tc2_DALI

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_DALI_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 290
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** DALI lighting bus: dedicated serial master hardware/service.

---

#### Programming › POUs › High-Level commands › Part 102 (control gears) › Addressing

##### `FB_DALIV2AddressingIntRandomAddressing` — §4.1.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | A positive edge at this input activates the function block, |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2AddressingIntRandomAddressing addresses the control gears at random. The user has no influence over which short address is assigned to which control gear. Short addresses are allocated in ascending order.

##### `FB_DALIV2AddressingPhysicalSelection` — §4.1.1.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | A positive edge at this input activates the function block, |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2AddressingPhysicalSelection addresses the control gears through 'physical selection' based on the addressing technique. This means that the individual control gears are selected (and therefore addressed) by removing the lamps. Short addresses are assigned (ascending) in the same order in which the lamps are removed.

##### `FB_DALIV2AddressingRandomAddressing` — §4.1.1.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | A positive edge at this input activates the function block, |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2AddressingRandomAddressing addresses the control gears at random. The user has no influence over which short address is assigned to which control gear. Short addresses are allocated in ascending order.

#### (top level)

##### `Error whilst programming a short address.` — §5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | A positive edge at this input activates the function block, |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrChangeAddressList` | `ARRAY [0.. 63] OF ST_DALIV2ChangeAddressList` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

#### Programming › POUs › High-Level commands › Part 102 (control gears) › Addressing

##### `FB_DALIV2SwapShortAddress` — §4.1.1.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The short addresses of two control gears can be swapped using the function block FB_DALIV2SwapShortAddress. However, this requires a free, unused short address to be available in the DALI line.

##### `FB_DALIV2SwapShortAddressList` — §4.1.1.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrSwapShortAddressList` | `ARRAY [0..63] OF ST_DALIV2SwapShortAddressList` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The short addresses of several control gears can be swapped using the function block FB_DALIV2SwapShortAddressList. However, this requires a free, unused short address to be available in the DALI line.

#### Programming › POUs › High-Level commands › Part 102 (control gears) › Power control

##### `FB_DALIV2ConstantLightControlEco` — §4.1.1.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Enables the function block. If this input is FALSE, the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActualLevelMaster BYTE` | `Current output value of the master device (always the` | — |
| `Dev` | `respectively addressed device if eAddrType = eAddrTypeShort)` | — |
| `nDeviation` | `INT` | Current control deviation (setpoint/actual value) |
| `bControllerIsActive` | `BOOL` | This output is set once the control is activated. |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2ConstantLightControlEco is used for constant light control with DALI control gears.

##### `FB_DALIV2Dimmer1Switch` — §4.1.1.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchDimm` | `BOOL` | Switches or dims the addressed devices. |
| `bOn` | `BOOL` | Switches the addressed devices to the last output value, or |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActualLevelMaster BYTE` | `Current output value of the master device (always the` | — |
| `Dev` | `respectively addressed device if eAddrType = eAddrTypeShort)` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2Dimmer1Switch switches and dims a single DALI lamp, a DALI group or a complete DALI line via a push button.

##### `FB_DALIV2Dimmer1SwitchEco` — §4.1.1.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchDimm` | `BOOL` | Switches or dims the addressed devices. |
| `bOn` | `BOOL` | Switches the addressed devices to the last output value, or |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `nActualLevelMaster BYTE` | `Current output value of the master device (always the` | — |
| `Dev` | `respectively addressed device if eAddrType = eAddrTypeShort)` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIDimmer1SwitchEco is the memory-saving variant of FB_DALIV2Dimmer1Switch [} 25]. It is not equipped with the special function “Switch off memory function”.

##### `FB_DALIV2Dimmer1SwitchMultiple` — §4.1.1.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `30 Programming` | `` | — |
| `bSwitchDimm` | `BOOL` | Switches or dims the addressed devices. |
| `bOn` | `BOOL` | Switches the addressed devices to the last output value, or |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer00` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer01` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer02` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer03` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer04` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActualLevelMaster BYTE` | `Current output value of the master device (always the` | — |
| `Dev` | `respectively addressed device if eAddrType = eAddrTypeShort)` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2Dimmer1SwitchMultiple is used to switch and dim DALI devices with a push button. For applications in which up to five DALI lines (0..4) can be installed. The basic function of this function block can be found in the description of FB_DALIV2Dimmer1Switch [} 25].

##### `FB_DALIV2Dimmer2Switch` — §4.1.1.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bSwitchDimmUp` | `BOOL` | Switches or dims the addressed devices up. |
| `bSwitchDimmDown` | `BOOL` | Switches or dims the addressed devices down. |
| `bOn` | `BOOL` | Switches the addressed devices to the last output value, or |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `34 Programming` | `` | — |
| `nActualLevelMaster BYTE` | `Current output value of the master device (always the` | — |
| `Dev` | `respectively addressed device if eAddrType = eAddrTypeShort)` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIDimmer2Switch corresponds to the function block FB_DALIV2Dimmer1Switch [} 25]. The difference is that two switches are connected to the function block FB_DALIDimmer2Switch. This allows the operator to specifically dim up or down.

##### `FB_DALIV2Dimmer2SwitchEco` — §4.1.1.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchDimmUp` | `BOOL` | Switches or dims the addressed devices up. |
| `bSwitchDimmDown` | `BOOL` | Switches or dims the addressed devices down. |
| `bOn` | `BOOL` | Switches the addressed devices to the last output value, or |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActualLevelMaster BYTE` | `Current output value of the master device (always the` | — |
| `Dev` | `respectively addressed device if eAddrType = eAddrTypeShort)` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIDimmer2SwitchEco is the memory-saving variant of FB_DALIV2Dimmer2Switch [} 32]. It is not equipped with the special function “Switch off memory function”.

##### `FB_DALIV2Light` — §4.1.1.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bOn` | `BOOL` | Switches the addressed devices on (to the value MAX |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLight` | `BOOL` | The status of the lamp or group after the function block has been called |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2Light is a simple function block for switching DALI lamps on and off.

##### `FB_DALIV2LightControl` — §4.1.1.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | The inputs bOn and bOff are active as long as this input is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActualLevelMaster BYTE` | `Current output value of the master device (always the` | — |
| `Dev` | `respectively addressed device if eAddrType = eAddrTypeShort)` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2LightControl is used for daylight-dependent lighting control with up to 30 interpolation points.

##### `FB_DALIV2Ramp` — §4.1.1.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | The bOn, bOff, bToggle and bStart inputs are active as |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActualLevelMaster BYTE` | `Current output value of the master device (always the` | — |
| `Dev` | `respectively addressed device if eAddrType = eAddrTypeShort)` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2Ramp is used to implement a light ramp.

##### `FB_DALIV2Sequencer` — §4.1.1.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | The bOn, bOff, bToggle and bStart inputs are active as |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActualLevelMaster BYTE` | `Current output value of the master device (always the` | — |
| `Dev` | `respectively addressed device if eAddrType = eAddrTypeShort) 48 Programming` | — |
| `nActualIndex` | `USINT` | Reference to the current element in the sequence table. |

**Description:** The function block FB_DALIV2Sequencer is used to implement light sequences with up to 50 interpolation points.

##### `FB_DALIV2StairwellDimmer` — §4.1.1.1.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | The inputs bOn and bOff are active as long as this input is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActualLevelMaster BYTE` | `Current output value of the master device (always the` | — |
| `Dev` | `respectively addressed device if eAddrType = eAddrTypeShort)` | — |
| `bBusy` | `BOOL` | This output is always set when a change of light is active, |

**Description:** The function block FB_DALIV2StairwellDimmer is used to control stairwell lighting.

#### (top level)

##### `Error whilst reading the variable PHYSICAL MIN LEVEL [} 88].` — §14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `arrDALIDeviceSettings` | `ARRAY [0..63] OF ST_DALIV2DeviceSettings` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

##### `Error whilst reading the variable PHYSICAL MIN LEVEL [} 88].` — §14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stDALIDeviceSettings` | `ST_DALIV2DeviceSettings` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** After processing the function block, the output bBusy changes from TRUE to FALSE.

##### `Error whilst writing the variables SCENE 0 [} 88] to SCENE 15 [} 88].` — §10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `arrDALIDeviceSettings` | `ARRAY [0..63] OF ST_DALIV2DeviceSettings` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

#### Programming › POUs › High-Level commands › Part 103 (control units) › Addressing

##### `FB_DALIV2xAddressingIntRandomAddressing` — §4.1.1.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xAddressingIntRandomAddressing addresses the control devices (sensors) at random. The user has no influence on which control device is assigned which short address. Short addresses are allocated in ascending order.

#### (top level)

##### `The value of the variable nOldAddress is equal to the value of the variable` — §6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrChangeAddressList` | `ARRAY [0..63] OF ST_DALIV2ChangeAddressList` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

#### Programming › POUs › High-Level commands › Part 202 (emergency lighting)

##### `FB_DALIV2EmergencyLightingDT` — §4.1.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `fbStringRingBuffer` | `FB_MemRingBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tElapsedTestTime` | `TIME` | Test duration. The time output is set to 0 by a rising edge |

**Description:** The function block FB_DALIV2EmergencyLightingDT is used for the duration test of a DALI emergency lighting device. At the end of the test, the test results are written to a FIFO buffer (IN-OUT variable fbStringRingBuffer), which in turn is read into a file by the function block FB_DALIV2FileLogging. Events that hinder or interrupt the test are displayed in addition to the result message at the bError and nErrorID outputs.

##### `FB_DALIV2EmergencyLightingFT` — §4.1.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `fbStringRingBuffer` | `FB_MemRingBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `66 Programming` | `` | — |
| `tElapsedTestTime` | `TIME` | Test duration. The time output is set to 0 by a rising edge |

**Description:** The function block FB_DALIV2EmergencyLightingFT is used for the function test of a DALI emergency lighting device. At the end of the test, the test results are written to a FIFO buffer (IN-OUT variable fbStringRingBuffer), which in turn is read into a file by the function block FB_DALIV2FileLogging. Events that hinder or interrupt the test are displayed in addition to the result message at the bError and nErrorID outputs.

##### `FB_DALIV2FileLogging` — §4.1.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fbStringRingBuffer` | `FB_MemRingBuffer` | — |

**Description:** The function block FB_DALIV2FileLogging reads the FIFO buffers written in the function blocks FB_DALIV2EmergencyLightingFT and FB_DALIV2EmergencyLightingDT (IN-OUT variable fbStringRingBuffer) and writes the contents to a log file.

#### (top level)

##### `Error whilst reading the variable EMERGENCY STATUS [} 250]` — §18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `arrDALIDeviceSettingsType01` | `ARRAY [0..63] OF ST_DALIV2DeviceSettingsType01` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** After processing the function block, the output bBusy changes from TRUE to FALSE. Processing this function block can take several seconds, depending on how many control gears are connected.

##### `Error whilst writing the variable PROLONG TIME [} 250]` — §14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `arrDALIDeviceSettingsType01` | `ARRAY [0..63] OF ST_DALIV2DeviceSettingsType01` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** When the function block has been processed, the output bBusy changes from TRUE to FALSE. Processing this function block can take several seconds, depending on how many control gears are connected.

#### Programming › POUs › Low-Level commands › Base

##### `FB_DALIV2GetEventData` — §4.1.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Enables the function block. If this input is set to FALSE, no |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bNewData` | `BOOL` | If an event was received that corresponds to the event |

**Description:** The function block FB_DALIV2GetEventData filters out an event specified by the event scheme.

##### `FB_DALIV2xSendDALICommand` — §4.1.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**Description:** The function block FB_DALIV2xSendDALICommand is used for the general sending of a DALI command, defined by command number and - if necessary - transfer parameter. Moreover, it is possible to set whether the command is sent twice in succession and whether to wait for a response. The latter can be used, for example, to realize a fast sequence of step-up commands.

#### (top level)

##### `Sequencer` — §9

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

#### Programming › POUs › Low-Level commands › Base

##### `FB_KL6811Communication` — §4.1.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetMaximumDe BOOL` | `A positive edge resets the stored value of the maximum` | — |
| `mandCounter` | `command buffer utilization, arrBufferMaximumDemandMeter (0 - 100 %, see` | — |
| `bResetOverflowCou BOOL` | `A positive edge resets the stored value of the number of` | — |
| `nter` | `command buffer overflows, arrBufferOverflowCounter (see Inputs/outputs` | — |
| `stDALIInData` | `ST_KL6811InData [} 467] Structure in the input process image of the KL6811. It is` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDALIInData` | `ST_KL6811InData` | — |
| `stDALIOutData` | `ST_KL6811OutData` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function blocks for the DALI commands do not directly access the process image of the KL6811, but store the individual DALI commands in three different buffers. The function block FB_KL6811Communication sequentially reads the DALI commands from these three buffers and forwards the DALI commands to the KL6811. This prevents multiple function blocks accessing the KL6811 process image simultaneously. Each of these three buffers is processed with a different priority (high, medium or low). The parameter eCommandPriority [} 462], which is available for most function blocks, can be used to influence the priority with which the respective DALI command is processed by the function block FB_KL6811Communication.

##### `FB_KL6811ConfigNew` — §4.1.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConfigurate` | `BOOL` | Configuration of the bus terminal is started by a positive |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInDataTerminal` | `ST_KL6811InData` | — |
| `stOutDataTerminal` | `ST_KL6811OutData` | — |
| `stInData` | `ST_KL6811InData` | — |
| `stOutData` | `ST_KL6811OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_KL6811ConfigNew is used to configure the KL6811. The configuration is executed when the PLC program starts, or it can be triggered by a positive edge at the input bConfigurate. The parameters are stored in the respective registers of the KL6811 in a fail-safe manner. In addition, some general information, such as the firmware version, is read from the KL6811.

##### `FB_KL6821Communication` — §4.1.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetMaximumDe BOOL` | `A positive edge resets the stored value of the maximum` | — |
| `mandCounter` | `command buffer utilization, arrBufferMaximumDemandMeter (0 - 100 %, see 82 Programming` | — |
| `bResetOverflowCou BOOL` | `A positive edge resets the stored value of the number of` | — |
| `nter` | `command buffer overflows, arrBufferOverflowCounter (see` | — |
| `bResetInactiveProce BOOL` | `A positive edge cancels the blocking of the process image` | — |
| `ssImage` | `of the terminal. The outputs bProcessImageInactive,` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_KL6821InData` | — |
| `stOutData` | `ST_KL6821OutData` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function blocks for the DALI commands do not directly access the process image of the DALI Bus Terminal, but store the individual DALI commands in three different buffers. The function block FB_KL6821Communication sequentially reads the DALI commands from these three buffers and forwards the DALI commands to the KL6821. This prevents several function block accessing the process image of the bus terminal at the same time. Each of these three buffers is processed with a different priority (high, medium or low). The parameter eCommandPriority, which is available for most function blocks, can be used by the PLC library user to influence the priority with which the respective DALI command is processed by the function block FB_KL6821Communication.

##### `FB_KL6821Config` — §4.1.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConfigurate` | `BOOL` | Configuration of the bus terminal is started by a positive |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInDataTerminal` | `ST_KL6821InData` | — |
| `stOutDataTerminal` | `ST_KL6821OutData` | — |
| `stInData` | `ST_KL6821InData` | — |
| `stOutData` | `ST_KL6821OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_KL6821Config is used to configure the KL6821. The configuration is executed when the PLC program starts, or it can be triggered by a positive edge at the input bConfigurate. The parameters are stored in the respective registers of the KL6821 in a fail-safe manner. In addition, some general information, such as the firmware version, is read from the KL6821.

#### (top level)

##### `Lamp is switched to the maximum brightness` — §255

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Control gears with a DSI interface do not have a short address. All the control gears on a DSI line receive the same value.

##### `Sequencer` — §9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

#### Programming › POUs › Low-Level commands › Part 102 (control gears) › Application-related expansion commands

##### `FB_DALIV2QueryExtendedVersionnumber` — §4.1.2.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable EXTENDED VERSION NUMBER [} 254] is read from the control gear.

##### `FB_DALIV2StartIdentification` — §4.1.2.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StartIdentification is used to start a 10-second identification procedure of the control gear. The behavior is specified by the manufacturer. The lamp is usually switched on and off according to a certain pattern.

#### Programming › POUs › Low-Level commands › Part 102 (control gears) › Configuration

##### `FB_DALIV2AddToGroup` — §4.1.2.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2AddToGroup is used to add the addressed control gears to the corresponding group (nGroup). A valid group number lies in the range between 0 and 15.

##### `FB_DALIV2RemoveFromGroup` — §4.1.2.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2RemoveFromGroup is used to remove one or more control gears from a group. A valid group number lies in the range between 0 and 15.

##### `FB_DALIV2RemoveFromScene` — §4.1.2.3.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2RemoveFromScene is used to remove one or more control gears from a scene. A valid scene number lies in the range between 0 and 15.

##### `FB_DALIV2Reset` — §4.1.2.3.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2Reset resets all variables of the control gear to the reset values.

##### `FB_DALIV2SetFadeRate` — §4.1.2.3.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SetFadeRate writes a value to the variable FADE RATE [} 88]. The range of possible values extends from 1 to 15.

##### `FB_DALIV2SetFadeTime` — §4.1.2.3.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SetFadeTime writes a value to the variable FADE TIME [} 88]. The range of possible values extends from 0 to 15.

##### `FB_DALIV2SetMaxLevel` — §4.1.2.3.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SetMaxLevel writes to the variable MAX LEVEL [} 89]. If the value provided is smaller than MIN LEVEL then the value is simply set to MIN LEVEL.

##### `FB_DALIV2SetMinLevel` — §4.1.2.3.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SetMinLevel writes to the variable MIN LEVEL [} 89]. If the value provided is larger than MAX LEVEL then the value is simply set to MAX LEVEL.

##### `FB_DALIV2SetPowerOnLevel` — §4.1.2.3.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** With the help of the function block FB_DALIV2SetPowerOnLevel, one or more control gears receive a defined switch-on value on the variable POWER ON LEVEL [} 88].

##### `FB_DALIV2SetScene` — §4.1.2.3.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SetScene saves a lamp power value nSceneLevel for a specified scene nScene. The value range for the scene number extends from 0 to 15.

##### `FB_DALIV2SetShortAddress` — §4.1.2.3.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SetShortAddress is used to assign a (new) short address to one or more control gears. Valid short addresses lie in the range between 0 and 63. If 255 is specified as the short address, the short address in the control gear is deleted.

##### `FB_DALIV2SetSystemFailureLevel` — §4.1.2.3.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable SYSTEM FAILURE LEVEL [} 88] (lamp power value at system error) is written to the control gear. If a fault (such as the absence of the supply voltage) is detected on the DALI bus, the control gear switches the lamp to this power value.

#### Programming › POUs › Low-Level commands › Part 102 (control gears) › Power control

##### `FB_DALIV2DirectArcPowerControl` — §4.1.2.3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** If the parameter nArcPowerLevel is outside of MAX VALUE [} 88] or MIN VALUE [} 88], the lamp is switched to the corresponding minimum or maximum value using the function block FB_DALIV2DirectArcPowerControl. If the lamp is switched off, it is switched on by the command. The rate at which the specified setpoint is to be reached is specified by the variable FADE TIME [} 88].

##### `FB_DALIV2Down` — §4.1.2.3.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The lamp is dimmed over a period of 200 ms. If the lamp power has already reached the MIN LEVEL [} 88] value the brightness is not changed. The lamp is not switched off with this command. The rate at which dimming is to take place within the 200 ms is specified by the variable FADE RATE [} 88].

##### `FB_DALIV2EnableDAPCSequence` — §4.1.2.3.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This command starts a Direct Arc Power Control (DAPC) sequence. Following this command, DirectArcPowerControl commands must be sent using the function block FB_DALIV2DirectArcPowerControl [} 113]. There must be no more than 200 ms between the individual commands; otherwise the sequence is terminated.

##### `FB_DALIV2GoToScene` — §4.1.2.3.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `116 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block sets the lamp power value to the value saved for the scene nScene. If the control gear does not belong to the scene, the lamp power value is not changed. If the lamp is switched off, it is switched on with the command. The rate at which the lamp power value is to be reached is specified by the variable FADE TIME [} 88].

##### `FB_DALIV2Off` — §4.1.2.3.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2Off switches the DALI lamps off immediately.

##### `FB_DALIV2OnAndStepUp` — §4.1.2.3.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2OnAndStepUp immediately sets the current lamp power value one level higher. If the lamp is switched off then it is switched on and set to MIN LEVEL [} 88].

##### `FB_DALIV2RecallMaxLevel` — §4.1.2.3.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2RecallMaxLevel sets the current lamp power value continuously to MAX LEVEL [} 88]. If the lamp is switched off this command will switch it on.

##### `FB_DALIV2RecallMinLevel` — §4.1.2.3.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2RecallMinLevel sets the current lamp power value continuously to MIN LEVEL [} 88]. If the lamp is switched off this command will switch it on.

##### `FB_DALIV2StepDown` — §4.1.2.3.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StepDown immediately sets the current lamp power value one level lower. The lamps are not switched off by this command. The power is not further reduced if the lamp power has already reached MIN LEVEL [} 88].

##### `FB_DALIV2StepDownAndOff` — §4.1.2.3.4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StepDownAndOff immediately sets the current lamp power value one level lower. The lamp is switched off if the power value has already reached MIN LEVEL [} 88].

##### `FB_DALIV2StepUp` — §4.1.2.3.4.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StepUp immediately sets the current lamp power value one level higher. The lamps are not switched on by this command. The power is not further increased if the lamp power has already reached MAX LEVEL [} 88].

##### `FB_DALIV2Up` — §4.1.2.3.4.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The lamp is dimmed brighter for 200 ms with the function block FB_DALIV2Up. If the lamp power has already reached the MAX LEVEL [} 88] the brightness is not changed. The lamp is not switched on with this command. The rate at which dimming is to take place within 200 ms is specified by the variable FADE RATE [} 88].

#### Programming › POUs › Low-Level commands › Part 102 (control gears) › Queries

##### `FB_DALIV2QueryActualLevel` — §4.1.2.3.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL 130` | The function block is activated by a positive edge at this Programming |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryActualLevel reads the ACTUAL DIM LEVEL [} 88] variable (current lamp power) from the control gear.

##### `FB_DALIV2QueryControlGearPresent` — §4.1.2.3.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryControlGearPresent provides information on whether a specific control gear is ready to operate.

#### (top level)

##### `Device for light emitting diodes (LEDs).` — §6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

#### Programming › POUs › Low-Level commands › Part 102 (control gears) › Queries

##### `FB_DALIV2QueryFadeTimeFadeRate` — §4.1.2.3.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryFadeTimeFadeRate reads the variables FADE TIME [} 88] and FADE RATE [} 88] from the control gear.

##### `FB_DALIV2QueryGroups` — §4.1.2.3.5.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and Programming |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** 136

##### `FB_DALIV2QueryLampFailure` — §4.1.2.3.5.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryLampFailure provides information on whether a particular control gear has a lamp problem.

##### `FB_DALIV2QueryLampPowerOn` — §4.1.2.3.5.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryLampPowerOn provides information on whether the lamp is switched on by a specific control gear.

##### `FB_DALIV2QueryLimitError` — §4.1.2.3.5.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryLimitError provides information on whether the last lamp power value on a specific control gear was not adhered to because it was above MAX LEVEL [} 88] or below MIN LEVEL [} 88].

##### `FB_DALIV2QueryMaxLevel` — §4.1.2.3.5.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryMaxLevel reads the variable MAX LEVEL [} 88] (maximum permitted lamp power) from the control gear. This value specifies the upper limit for lamp power commands.

##### `FB_DALIV2QueryMinLevel` — §4.1.2.3.5.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable MIN LEVEL [} 88] (minimum permissible lamp power) is read from the control gear. This value specifies the lower limit for lamp power commands.

##### `FB_DALIV2QueryMissingShortAddress` — §4.1.2.3.5.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryMissingShortAddress provides information on whether a specific control gear does not have a short address.

##### `FB_DALIV2QueryPhysicalMinLevel` — §4.1.2.3.5.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryPhysicalMinLevel reads the variable PHYSICAL MIN LEVEL [} 88] (physically lowest possible lamp power) from the control gear. This value can only be read, and is specified by the manufacturer.

##### `FB_DALIV2QueryPowerFailure` — §4.1.2.3.5.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryPowerFailure queries whether the control gear has not received a reset or lamp power control command since it was switched on.

##### `FB_DALIV2QueryPowerOnLevel` — §4.1.2.3.5.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryPowerOnLevel reads the variable POWER ON LEVEL [} 88] (switch-on lamp power) from the control gear. The lamp switches itself to this power value immediately after the power is connected.

##### `FB_DALIV2QueryRandomAddress` — §4.1.2.3.5.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable RANDOM ADDRESS [} 88] is read from the control gear.

##### `FB_DALIV2QueryRandomAddressH` — §4.1.2.3.5.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The high-order byte of the variable RANDOM ADDRESS [} 88] is read from the control gear.

##### `FB_DALIV2QueryRandomAddressL` — §4.1.2.3.5.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryRandomAddressL reads the low-order byte of the variable RANDOM ADDRESS [} 88] from the control gear.

##### `FB_DALIV2QueryRandomAddressM` — §4.1.2.3.5.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `154 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryRandomAddressM reads the middle byte of the variable RANDOM ADDRESS [} 88] from the control gear.

##### `FB_DALIV2QueryResetState` — §4.1.2.3.5.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryResetState provides information on whether a specific control gear is in the reset state.

##### `FB_DALIV2QuerySceneLevel` — §4.1.2.3.5.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QuerySceneLevel reads the lamp power value of the corresponding scene from the control gear.

#### (top level)

##### `Short address missing. 0: no.` — §6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

#### Programming › POUs › Low-Level commands › Part 102 (control gears) › Queries

##### `FB_DALIV2QuerySystemFailureLevel` — §4.1.2.3.5.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable SYSTEM FAILURE LEVEL [} 88] (lamp power value on system error) is read from the control gear. If a fault (such as the absence of the supply voltage) is detected on the DALI bus, the control gear switches the lamp to this power value.

##### `FB_DALIV2QueryVersionNumber` — §4.1.2.3.5.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** 160

##### `FB_DALIV2ReadMemoryLocation` — §4.1.2.3.5.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2ReadMemoryLocation is used to read a byte from the memory of the control gear. The exact memory bank is specified by the parameter nMemoryBank and the address within the memory bank by the parameter nOffset.

#### Programming › POUs › Low-Level commands › Part 102 (control gears) › Special commands

##### `FB_DALIV2Compare` — §4.1.2.3.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `164 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The control gear compares its RANDOM ADDRESS [} 88] with the SEARCH ADDRESS [} 88]. If the random address is smaller than or equal to the search address, and if the control gear is not connected, then the output bAnswer is set to TRUE.

##### `FB_DALIV2Initialise` — §4.1.2.3.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `VAR INPUT` | `` | — |
| `bStart` | `: BOOL;` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2Initialise is used to start the addressing of the control gears. The addressing has to be terminated again with the function block FB_DALIV2Terminate [} 178]. The maximum duration is limited to 15 minutes. Each control gear ends addressing automatically after 15 minutes. The reaction of the control gears that receive this command depends on the parameter nParameter:

##### `FB_DALIV2PhysicalSelection` — §4.1.2.3.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2PhysicalSelection controls that all control gears that receive this command go into physical selection mode. In this mode, the comparison of the RANDOM ADDRESS [} 88] with the SEARCH ADDRESS [} 88] is blocked.

##### `FB_DALIV2ProgramShortAddress` — §4.1.2.3.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2ProgramShortAddress controls that all selected control gears save the value nShortAddress (SHORT ADDRESS [} 88]) as a short address.

##### `FB_DALIV2QueryShortAddress` — §4.1.2.3.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Once the control gear has been selected, it sends its SHORT ADDRESS [} 88].

##### `FB_DALIV2Randomise` — §4.1.2.3.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The control gears generate a new RANDOM ADDRESS [} 88] with the function block FB_DALIV2Randomize.

##### `FB_DALIV2SearchAddr` — §4.1.2.3.6.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SearchAddr sets the SEARCH ADDRESS [} 88].

##### `FB_DALIV2SearchAddrH` — §4.1.2.3.6.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SearchAddrH sets the upper 8 bits of the 24-bit SEARCH ADDRESS [} 88].

##### `FB_DALIV2SearchAddrL` — §4.1.2.3.6.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** 172

##### `FB_DALIV2SearchAddrM` — §4.1.2.3.6.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SearchAddrM sets the middle 8 bits of the 24-bit SEARCH ADDRESS [} 88].

##### `FB_DALIV2Terminate` — §4.1.2.3.6.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2Terminate terminates addressing for all control gears.

##### `FB_DALIV2VerifyShortAddress` — §4.1.2.3.6.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** If the short address in the control gear is equal to the parameter nShortAddress, the output bAnswer is set to TRUE.

##### `FB_DALIV2Withdraw` — §4.1.2.3.6.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Control gears for which the RANDOM ADDRESS [} 88] is the same as the SEARCH ADDRESS [} 88] may no longer respond to the command FB_DALIV2Compare [} 164].

##### `FB_DALIV2WriteMemoryLocation` — §4.1.2.3.6.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The value nValue is written in the memory bank of the control gear. The exact memory bank is specified by nMemoryBank and the address within the memory bank by nOffset.

#### Programming › POUs › Low-Level commands › Part 103 (control units) › Configuration

##### `FB_DALIV2xAddToDeviceGroups` — §4.1.2.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xAddToDeviceGroups assigns one or more groups to the control device.

##### `FB_DALIV2xDisableInstance` — §4.1.2.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xDisableInstance disables the instance of the control device.

##### `FB_DALIV2xDisablePowerCycleNotification` — §4.1.2.4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xDisablePowerCycleNotification disables the Power Cycle Notification event.

##### `FB_DALIV2xEnableInstance` — §4.1.2.4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xEnableInstance enables the instance of the control device.

##### `FB_DALIV2xEnablePowerCycleNotification` — §4.1.2.4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xEnablePowerCycleNotification enables the Power Cycle Notification event.

##### `FB_DALIV2xIdentifyDevice` — §4.1.2.4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xIdentifyDevice starts the identification routine for the control device.

##### `FB_DALIV2xRemoveFromDeviceGroups` — §4.1.2.4.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `190 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xRemoveFromDeviceGroups removes the control device from one or more groups.

##### `FB_DALIV2xReset` — §4.1.2.4.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xReset resets all parameters to the reset values.

##### `FB_DALIV2xSetEventFilter` — §4.1.2.4.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xSetEventFilter sets the event filter for the respective instance of the control device.

##### `FB_DALIV2xSetEventScheme` — §4.1.2.4.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xSetEventScheme can be used to define the addressing scheme for the events of the respective instance of the control device.

##### `FB_DALIV2xSetOperatingMode` — §4.1.2.4.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xSetOperatingMode sets the Operating Mode for the control device.

##### `FB_DALIV2xSetShortAddress` — §4.1.2.4.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xSetShortAddress sets the short address of the control device.

##### `FB_DALIV2xStartQuiescentMode` — §4.1.2.4.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xStartQuiescentMode is used to start the Quiescent Mode of the control device.

##### `FB_DALIV2xStopQuiescentMode` — §4.1.2.4.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xStopQuiescentMode is used to stop the quiescent mode of the control device.

#### Programming › POUs › Low-Level commands › Part 103 (control units)

##### `Query` — §4.1.2.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Function blocks

#### Programming › POUs › Low-Level commands › Part 103 (control units) › Query

##### `FB_DALIV2xQueryDeviceGroups` — §4.1.2.4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryDeviceGroups queries the group assignments of the control device.

##### `FB_DALIV2xQueryDeviceStatus` — §4.1.2.4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryDeviceStatus reads the Device Status of the control device.

##### `FB_DALIV2xQueryEventFilter` — §4.1.2.4.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryEventFilter queries the event filter for the respective instance of the control device.

##### `FB_DALIV2xQueryEventScheme` — §4.1.2.4.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryEventScheme can be used to query the addressing scheme for the events of the respective instance of the control device.

##### `FB_DALIV2xQueryInputDeviceError` — §4.1.2.4.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryInputDeviceError reads the Input Device Error of the control device.

##### `FB_DALIV2xQueryInputValue` — §4.1.2.4.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryInputValue queries the first byte of the input value of the instance of the control device.

##### `FB_DALIV2xQueryInputValueLatch` — §4.1.2.4.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryInputValueLatch queries the following byte of the input value of the instance of the control device.

##### `FB_DALIV2xQueryInstanceEnabled` — §4.1.2.4.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `214 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryInstanceEnabled queries whether the instance of the control device is enabled.

##### `FB_DALIV2xQueryInstanceError` — §4.1.2.4.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryInstanceError reads the Instance Error of the control device.

##### `FB_DALIV2xQueryInstanceStatus` — §4.1.2.4.2.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryInstanceStatus reads the Instance Status of the control device.

##### `FB_DALIV2xQueryMissingShortAddress` — §4.1.2.4.2.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryMissingShortAddress queries whether the control device does not have a valid short address.

##### `FB_DALIV2xQueryNumberOfInstances` — §4.1.2.4.2.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryNumberOfInstances queries the number of instances that the control device has.

##### `FB_DALIV2xQueryOperatingMode` — §4.1.2.4.2.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `220 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryOperatingMode queries the Operating Mode of the control device.

##### `FB_DALIV2xQueryPowerCycleNotification` — §4.1.2.4.2.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryPowerCycleNotification queries whether the Power Cycle Notification event has been enabled.

##### `FB_DALIV2xQueryRandomAddressH` — §4.1.2.4.2.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryRandomAddressH reads the high-order byte of the random address from the control device.

##### `FB_DALIV2xQueryRandomAddressL` — §4.1.2.4.2.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryRandomAddressL reads the low-order byte of the random address from the control device.

##### `FB_DALIV2xQueryRandomAddressM` — §4.1.2.4.2.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryRandomAddressM reads the middle byte of the random address from the control device.

##### `FB_DALIV2xQueryResetState` — §4.1.2.4.2.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `226 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryResetState queries whether the parameters of the control device have their reset values.

##### `FB_DALIV2xQueryResolution` — §4.1.2.4.2.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryResolution queries the resolution of the input values of the control device.

##### `FB_DALIV2xQueryVersionNumber` — §4.1.2.4.2.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xQueryVersionNumber queries the version number of the control device.

##### `FB_DALIV2xReadMemoryLocation` — §4.1.2.4.2.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xReadMemoryLocation is used to read a byte from the control device's memory. The exact memory bank is specified by the parameter nMemoryBank and the address within the memory bank by the parameter nOffset.

#### Programming › POUs › Low-Level commands › Part 103 (control units) › Special commands

##### `FB_DALIV2xCompare` — §4.1.2.4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The control gear compares its random address with the search address. If the random address is less than or equal to the search address and the control device is not excluded, then the output bAnswer is set to TRUE.

##### `FB_DALIV2xInitialise` — §4.1.2.4.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This command starts the addressing procedure for the control devices. Addressing must be terminated again using the function block FB_DALIV2xTerminate [} 244]. The maximum duration is limited to 15 minutes. Each control device automatically terminates addressing after 15 minutes. The response of the control devices receiving this command depends on the parameter nParameter:

##### `FB_DALIV2xProgramShortAddress` — §4.1.2.4.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** All selected control devices save the value nShortAddress as a short address.

##### `FB_DALIV2xQueryShortAddress` — §4.1.2.4.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** If the control device is selected, it sends its short address.

##### `FB_DALIV2xRandomise` — §4.1.2.4.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Programming

##### `FB_DALIV2xSearchAddressH` — §4.1.2.4.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** 240

##### `FB_DALIV2xSearchAddressL` — §4.1.2.4.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xSearchAddressL sets the lower 8 bits of the 24-bit search address.

##### `FB_DALIV2xSearchAddressM` — §4.1.2.4.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xSearchAddressM sets the middle 8 bits of the 24-bit search address.

##### `FB_DALIV2xTerminate` — §4.1.2.4.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2xTerminate terminates addressing for all control devices.

##### `FB_DALIV2xVerifyShortAddress` — §4.1.2.4.3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** If the short address of the control device is equal to the parameter nShortAddress, the output bAnswer is set to TRUE.

##### `FB_DALIV2xWithdraw` — §4.1.2.4.3.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Control devices whose random address is the same as the search address may no longer respond to the command FB_DALIV2xCompare [} 231].

##### `FB_DALIV2xWriteMemoryLocation` — §4.1.2.4.3.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The value nValue is written to the memory bank of the control device. The exact memory bank is specified by nMemoryBank and the address within the memory bank by nOffset.

#### Programming › POUs › Low-Level commands › Part 202 (emergency lighting)

##### `Variables` (FUNCTION) — §4.1.2.5.1

**Description:** Each DALI emergency lighting supply unit has a certain number of variables (parameters) from which various information can be read or individual parameters be modified.

##### `FB_DALIV2Inhibit` — §4.1.2.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Prevents the control gear from switching to emergency mode for 15 minutes. The function block FB_DALIReLightResetInhibit [} 273] can be used to disable inhibition of emergency mode.

##### `FB_DALIV2QueryBatteryCharge` — §4.1.2.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable BATTERY CHARGE [} 250] (state of battery charge) is read from the control gear. 255 is returned if the control gear is unable to determine the value.

##### `FB_DALIV2QueryDurationTestResult` — §4.1.2.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable DURATION TEST RESULT [} 250] is read from the control gear. The unit is 2 minutes per step. 255 means a maximum value of 8.5 hours or more. The value is only valid if bit 2 is set in the variable EMERGENCY STATUS [} 250].

##### `FB_DALIV2QueryEmergencyLevel` — §4.1.2.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable EMERGENCY LEVEL [} 250] (emergency illuminance) is read from the control gear. 255 is returned if the control gear is unable to determine the value.

##### `FB_DALIV2QueryEmergencyMaxLevel` — §4.1.2.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable EMERGENCY MAX LEVEL [} 250] is read from the control gear. 255 is returned if the control gear is unable to determine the value.

##### `FB_DALIV2QueryEmergencyMinLevel` — §4.1.2.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable EMERGENCY MIN LEVEL [} 250] is read from the control gear. 255 is returned if the control gear is unable to determine the value.

#### (top level)

##### `Connected mains voltage is active. 0: OFF.` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** 262

##### `Selected. 0: no` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

##### `Duration test failed. 0: no` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

##### `Reserve` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

#### Programming › POUs › Low-Level commands › Part 202 (emergency lighting)

##### `FB_DALIV2QueryLampEmergencyTime` — §4.1.2.5.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable LAMP EMERGENCY TIME [} 250] is read from the control gear. The unit is 1 hour per step. 255 means a maximum value of 254 hours or more. The variable is always incremented at the start of the 1-hour interval. Once the maximum value of 255 is reached, it is not increased further. The variable is reset via the command FB_DALIV2ResetLampTime [} 277].

##### `FB_DALIV2QueryLampTotalOperationTime` — §4.1.2.5.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable LAMP TOTAL OPERATION TIME [} 250] is read from the control gear. The unit is 4 hours per step. 255 means a maximum value of 1016 hours or more. The variable is always incremented at the start of the 4-hour interval. Once the maximum value of 255 is reached, it is not increased further. The variable is reset via the command FB_DALIV2ResetLampTime [} 277].

##### `FB_DALIV2QueryRatedDuration` — §4.1.2.5.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable RATED DURATION [} 250] is read from the control gear. The unit is 2 minutes per step.

##### `FB_DALIV2QueryTestTiming` — §4.1.2.5.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryTestTiming reads the test times (intervals, duration until the next event, etc.) from the control gear. This takes place depending on the value applied at the nDTR input. The background to this is that the DALI basic command 242 “Query test timing” works together with the contents of the Data Transfer Register (DTR), which must be written to accordingly beforehand. The function block FB_DALIV2QueryTestTiming combines these two actions. The following data are read from the device, depending on the nDTR value:

##### `FB_DALIV2ReLightResetInhibit` — §4.1.2.5.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The control gear is switched back to emergency mode (in the absence of mains voltage). This deactivates the function of the function block FB_DALIV2Inhibit [} 255].

##### `FB_DALIV2ResetDurationTestDoneFlag` — §4.1.2.5.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The 'Duration test completed and result is valid' flag is reset. The flag is bit 2 of the variable EMERGENCY STATUS [} 250] and can be queried with the function block FB_DALIQueryEmergencyStatus [} 264].

##### `FB_DALIV2ResetFunctionTestDoneFlag` — §4.1.2.5.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The 'Function test completed and result is valid' flag is reset. The flag is bit 1 of the variable EMERGENCY STATUS [} 250] and can be queried with the function block FB_DALIQueryEmergencyStatus [} 264].

##### `FB_DALIV2ResetLampTime` — §4.1.2.5.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2ResetLampTime resets the variable LAMP EMERGENCY TIME [} 250] and LAMP TOTAL OPERATION TIME [} 250].

##### `FB_DALIV2Rest` — §4.1.2.5.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2Rest is used to switch off the lamp when emergency operation is active. Otherwise the system switches to normal operation if mains voltage is available again, or if the function block FB_DALIV2ReLightResetInhibit [} 273] was called.

##### `FB_DALIV2StartDurationTest` — §4.1.2.5.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StartDurationTest is used to start the duration test. If the duration test is started with a delay, this is indicated in bit 5 of the variable EMERGENCY STATUS [} 250].

##### `FB_DALIV2StartFunctionTest` — §4.1.2.5.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StartFunctionTest is used to start the function test. If the function test is started with a delay, this is indicated in bit 4 of the variable EMERGENCY STATUS [} 250].

##### `FB_DALIV2StopTest` — §4.1.2.5.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StopTest stops any type of function test or duration test.

##### `FB_DALIV2StoreDTRAsDurationTestInterval` — §4.1.2.5.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsDurationTestInterval writes the value of the DTR (Data Transfer Register) to the variable DURATION TEST INTERVAL [} 250].

##### `FB_DALIV2StoreDTRAsEmergencyLevel` — §4.1.2.5.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsEmergencyLevel writes the value of the DTR (Data Transfer Register) to the variable EMERGENCY LEVEL [} 250].

##### `FB_DALIV2StoreDTRAsFunctionTestInterval` — §4.1.2.5.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsFunctionTestInterval writes the value of the DTR (Data Transfer Register) to the variable FUNCTION TEST INTERVAL [} 250].

##### `FB_DALIV2StoreDTRAsProlongTime` — §4.1.2.5.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsProlongTime writes the value of the DTR (Data Transfer Register) to the variable PROLONG TIME [} 250].

##### `FB_DALIV2StoreDTRAsTestDelayTimeHighByte` — §4.1.2.5.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsTestDelayTimeHighByte writes the value of the DTR (Data Transfer Register) to the high-order byte of the variable TEST DELAY TIME [} 250].

##### `FB_DALIV2StoreDTRAsTestDelayTimeLowByte` — §4.1.2.5.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsTestDelayTimeLowByte writes the value of the DTR (Data Transfer Register) to the low-order byte of the variable TEST DELAY TIME [} 250].

##### `FB_DALIV2StoreDTRAsTestExecutionTimeout` — §4.1.2.5.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsTestExecutionTimeout writes the value of the DTR (Data Transfer Register) to the variable TEST EXECUTION TIMEOUT [} 250] (maximum execution time for a test).

#### (top level)

##### `Lamp cycle error. 0: no.` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

##### `Physical selection is supported. 0: no.` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

##### `Reserve` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Programming

##### `Lamp cycle error. 0: no.` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

#### Programming › POUs › Low-Level commands › Part 203 (discharge lamps)

##### `FB_DALIV2QueryThermalLoad` — §4.1.2.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable THERMAL LOAD [} 292] is read. This contains a percentage figure in the range from 0 % to 127.5 % in steps of 0.5 %.

##### `FB_DALIV2QueryThermalOverloadTime` — §4.1.2.6.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The 16-bit variable THERMAL OVERLOAD TIME [} 292] is read.

##### `FB_DALIV2QueryThermalOverloadTimeHB` — §4.1.2.6.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The high-order byte of the 16-bit variable THERMAL OVERLOAD TIME [} 292] is read.

##### `FB_DALIV2QueryThermalOverloadTimeLB` — §4.1.2.6.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryThermalOverloadTimeLB reads the low-order byte of the 16-bit variable THERMAL OVERLOAD TIME [} 292].

##### `FB_DALIV2ResetStoredHIDFailure` — §4.1.2.6.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable STORED HID FAILURE [} 292] (stored lamp faults) is reset.

#### Programming › POUs › Low-Level commands › Part 207 (LED modules)

##### `FB_DALIV2DisableCurrentProtector` — §4.1.2.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2DisableCurrentProtector disables the current protection device of the control gear.

##### `FB_DALIV2EnableCurrentProtector` — §4.1.2.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2EnableCurrentProtector enables the current protection device of the control gear.

##### `FB_DALIV2QueryCurrentProtectorActive` — §4.1.2.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryCurrentProtectorActive queries whether the current protection device is active.

##### `FB_DALIV2QueryCurrentProtectorEnabled` — §4.1.2.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryCurrentProtectorEnabled queries whether the current protection device is enabled.

##### `FB_DALIV2QueryDimmingCurve` — §4.1.2.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryDimmingCurve reads the dimming curve of the control gear.

##### `FB_DALIV2QueryFastFadeTime` — §4.1.2.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryFastFadeTime queries the value of FAST FADE TIME.

##### `FB_DALIV2QueryGearType` — §4.1.2.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryGearType queries the value of GEAR TYPE.

##### `FB_DALIV2QueryLedFailureStatus` — §4.1.2.7.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `316 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryLedFailureStatus queries the value of FAILURE STATUS.

##### `FB_DALIV2QueryLedFeatures` — §4.1.2.7.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryLedFeatures queries the value of FEATURES.

##### `FB_DALIV2QueryLoadDecrease` — §4.1.2.7.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryLoadDecrease queries whether a significant load decrease (in comparison with the reference power of the system) has been detected.

##### `FB_DALIV2QueryLoadIncrease` — §4.1.2.7.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryLoadIncrease queries whether a significant load increase (in comparison with the reference power of the system) has been detected.

##### `FB_DALIV2QueryMinFastFadeTime` — §4.1.2.7.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryMinFastFadeTime queries the value of MIN FAST FADE TIME.

##### `FB_DALIV2QueryOpenCircuit` — §4.1.2.7.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryOpenCircuit queries whether an open circuit has been detected.

##### `FB_DALIV2QueryOperatingMode` — §4.1.2.7.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryOperatingMode queries the value of OPERATING MODE.

##### `FB_DALIV2QueryPossibleOperatingModes` — §4.1.2.7.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryPossibleOperatingModes queries the value of OPERATING MODE.

##### `FB_DALIV2QueryReferenceMeasurementFailed` — §4.1.2.7.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryReferenceMeasurementFailed queries whether a started reference measurement has failed.

##### `FB_DALIV2QueryReferenceRunning` — §4.1.2.7.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryReferenceRunning queries whether a reference measurement of the system power is active.

##### `FB_DALIV2QueryShortCircuit` — §4.1.2.7.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryShortCircuit queries whether a short circuit has been detected.

##### `FB_DALIV2QueryThermalOverload` — §4.1.2.7.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryThermalOverload queries whether there is a thermal overload with a reduction in luminous flux.

##### `FB_DALIV2QueryThermalShutDown` — §4.1.2.7.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryThermalShutDown queries whether a thermal shutdown has taken place.

##### `FB_DALIV2ReferenceSystemPower` — §4.1.2.7.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The control gear measures and stores the performance level of the system, in order to detect load increase and load decrease.

##### `FB_DALIV2SelectDimmingCurve` — §4.1.2.7.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SelectDimmingCurve selects the dimming curve of the control gear.

##### `FB_DALIV2SetFastFadeTime` — §4.1.2.7.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SetFastFadeTime sets the FAST FADE TIME in the control gear.

#### Programming › POUs › Low-Level commands › Part 209 (color/color temperature control)

##### `FB_DALIV2Activate` — §4.1.2.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** A current cross-fade is ended and a new cross-fade is started. Only the color is changed here.

#### (top level)

##### `Freely selectable color` — §6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block changes the DTR (Data Transfer Register) for all DALI control gears of the DALI line.

#### Programming › POUs › Low-Level commands › Part 209 (color/color temperature control)

##### `FB_DALIV2ColourTemperatureTcStepCooler` — §4.1.2.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The COLOUR TEMPERATURE Tc [} 338] value is decremented by 1 Mirek without cross-fading. If the value COLOUR TEMPERATURE Tc [} 338] already has the value COLOUR TEMPERATURE Tc COOLEST [} 338], no change is made.

##### `FB_DALIV2ColourTemperatureTcStepWarmer` — §4.1.2.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The COLOUR TEMPERATURE Tc [} 338] value is incremented by 1 Mirek without cross-fading. If the value COLOUR TEMPERATURE Tc [} 338] already has the value COLOUR TEMPERATURE Tc WARMEST [} 338], no change is made.

##### `FB_DALIV2CopyReportToTemporary` — §4.1.2.8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The color settings report is copied to the temporary color settings.

#### (top level)

##### `Freely selectable color` — §6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `350 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block changes the DTR (Data Transfer Register) for all DALI control gears of the DALI line.

##### `Color representation RGBWAF active.` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block supports the color representations: • xy coordinates • Color temperature Tc • Primary (color) N • RGBWAF

#### Programming › POUs › Low-Level commands › Part 209 (color/color temperature control)

##### `FB_DALIV2QueryColourTypeFeatures` — §4.1.2.8.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable COLOUR TYPE FEATURES [} 338] is read from the DALI control gear. This contains the color representations supported by the DALI control gear.

#### (top level)

##### `REPORT COLOUR TYPE` — §240

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Responses that concern an active color representation are valid only if the color representation of the requested color value is active (see FB_DALIV2QueryColourStatus [} 351]) or if the control gear is able to perform a recalculation of the demanded color value from the active color representation to a color value of another color representation.

##### `Restoration of the automatic calibration is supported.` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Programming

##### `Output channel 5 / free selectable color` — §5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** If an output channel or a color is not supported, then the corresponding bit is FALSE.

#### Programming › POUs › Low-Level commands › Part 209 (color/color temperature control)

##### `FB_DALIV2SetTemporaryColourTemperatureTc` — §4.1.2.8.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Saves the value in the variable TEMPORARY COLOUR TEMPERATURE Tc [} 338] of the DALI control gear. The value can be read with the function block FB_DALIV2QueryColourValue [} 354].

##### `FB_DALIV2SetTemporaryPrimaryNDimlevel` — §4.1.2.8.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Saves the value in the variable TEMPORARY PRIMARY N DIMLEVEL [} 338] of the control gear. The value can be read with the function block FB_DALIV2QueryColourValue [} 354].

##### `FB_DALIV2SetTemporaryRGBDimlevel` — §4.1.2.8.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Saves the values in the variables TEMPORARY RED DIMLEVEL [} 338], TEMPORARY GREEN DIMLEVEL [} 338] and TEMPORARY BLUE DIMLEVEL [} 338] of the DALI control gear. The values can be read with the function block FB_DALIV2QueryColourValue [} 354].

#### (top level)

##### `Output channel 5 / free selectable color` — §5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block changes the DTR (Data Transfer Register) for all DALI control gears of the DALI line.

#### Programming › POUs › Low-Level commands › Part 209 (color/color temperature control)

##### `FB_DALIV2SetTemporaryWAFDimlevel` — §4.1.2.8.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Saves the values in the variables TEMPORARY WHITE DIMLEVEL [} 338], TEMPORARY AMBER DIMLEVEL [} 338] and TEMPORARY FREECOLOUR DIMLEVEL [} 338] of the DALI control gear. The values can be read with the function block FB_DALIV2QueryColourValue [} 354].

##### `FB_DALIV2SetTemporaryXCoordinate` — §4.1.2.8.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `368 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Saves the value in the variable TEMPORARY x-COORDINATE [} 338] of the DALI control gear. The value can be read with the function block FB_DALIV2QueryColourValue [} 354].

##### `FB_DALIV2SetTemporaryYCoordinate` — §4.1.2.8.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Saves the value in the variable TEMPORARY y-COORDINATE [} 338] of the DALI control gear. The value can be read with the function block FB_DALIV2QueryColourValue [} 354].

##### `FB_DALIV2StartAutoCalibration` — §4.1.2.8.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The calibration procedure is started in order to measure the x-coordinate, the y-coordinate and the TY value of all supported primary colors.

#### (top level)

##### `COLOUR TEMPERATURE Tc highest possible physical value.` — §3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The functions KELVIN_TO_MIREK [} 384] and MIREK_TO_KELVIN [} 385] are available for converting from or to Kelvin.

##### `Automatic activation.` — §0

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block changes the DTR (Data Transfer Register) for all DALI control gears of the DALI line.

#### Programming › POUs › Low-Level commands › Part 209 (color/color temperature control)

##### `FB_DALIV2StoreTYPrimaryN` — §4.1.2.8.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Saves the value in the variable TY PRIMARY N [} 338] of the DALI control gear.

##### `FB_DALIV2StoreXyCoordinatePrimaryN` — §4.1.2.8.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Copies the value from the variables TEMPORARY x-COORDINATE [} 338] and TEMPORARY y-COORDINATE [} 338] to the variables x-COORDINATE PRIMARY N [} 338] and y-COORDINATE PRIMARY N [} 338].

##### `FB_DALIV2XCoordinateStepDown` — §4.1.2.8.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable x-COORDINATE is reduced by 256 steps (256 / 65536) without cross-fading.

##### `FB_DALIV2XCoordinateStepUp` — §4.1.2.8.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `380 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable x-COORDINATE is increased by 256 steps (256 / 65536) without cross-fading.

##### `FB_DALIV2YCoordinateStepDown` — §4.1.2.8.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable y-COORDINATE is reduced by 256 steps (256 / 65536) without cross-fading.

##### `FB_DALIV2YCoordinateStepUp` — §4.1.2.8.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The variable y-COORDINATE is increased by 256 steps (256 / 65536) without cross-fading.

##### `KELVIN_TO_MIREK` — §4.1.2.8.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fKelvin` | `LREAL 384` | Color temperature in Kelvin Programming |

**Description:** Conversion of the color temperature from Kelvin to Mirek.

##### `MIREK_TO_KELVIN` — §4.1.2.8.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nMirek` | `UINT` | Color temperature in Mirek |

**Description:** Conversion of the color temperature from Mirek to Kelvin.

#### Programming › POUs › Low-Level commands › Part 301 (button)

##### `FB_DALIV2x301QueryDoubleTimer` — §4.1.2.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `386 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x301QueryDoubleTimer queries the value of the DOUBLE TIMER.

##### `FB_DALIV2x301QueryDoubleTimerMin` — §4.1.2.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x301QueryDoubleTimerMin queries the minimum value of the DOUBLE TIMER.

##### `FB_DALIV2x301QueryRepeatTimer` — §4.1.2.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x301QueryRepeatTimer queries the value of the REPEAT TIMER.

##### `FB_DALIV2x301QueryShortTimer` — §4.1.2.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `390 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x301QueryShortTimer queries the value of the SHORT TIMER.

##### `FB_DALIV2x301QueryShortTimerMin` — §4.1.2.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x301QueryShortTimerMin queries the minimum value of the SHORT TIMER.

##### `FB_DALIV2x301QueryStuckTimer` — §4.1.2.9.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `392 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x301QueryStuckTimer queries the value of the STUCK TIMER.

##### `FB_DALIV2x301SetDoubleTimer` — §4.1.2.9.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `394 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x301SetDoubleTimer sets the value of the DOUBLE TIMER.

##### `FB_DALIV2x301SetRepeatTimer` — §4.1.2.9.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x301SetRepeatTimer sets the value of the REPEAT TIMER.

##### `FB_DALIV2x301SetShortTimer` — §4.1.2.9.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `396 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x301SetShortTimer sets the value of the SHORT TIMER.

##### `FB_DALIV2x301SetStuckTimer` — §4.1.2.9.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `398 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x301SetStuckTimer sets the value of the STUCK TIMER.

#### Programming › POUs › Low-Level commands › Part 303 (occupancy sensor)

##### `FB_DALIV2x303CancelHoldTimer` — §4.1.2.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x303CancelHoldTimer terminates the HOLD TIMER prematurely.

##### `FB_DALIV2x303CatchMovement` — §4.1.2.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** After calling this command, an event is sent once if a movement is detected.

##### `FB_DALIV2x303QueryCatching` — §4.1.2.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x303QueryCatching queries whether the system is waiting for a movement to be detected. This function can be activated by FB_DALIV2x303CatchMovement [} 401].

##### `FB_DALIV2x303QueryDeadtimeTimer` — §4.1.2.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Queries the value of the DEADTIME TIMER.

##### `FB_DALIV2x303QueryHoldTimer` — §4.1.2.10.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x303QueryHoldTimer queries the value of the HOLD TIMER.

##### `FB_DALIV2x303QueryReportTimer` — §4.1.2.10.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x303QueryReportTimer queries the value of the REPORT TIMER.

##### `FB_DALIV2x303SetDeadtimeTimer` — §4.1.2.10.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x303SetDeadtimeTimer sets the value of the DEADTIME TIMER.

##### `FB_DALIV2x303SetHoldTimer` — §4.1.2.10.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `408 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x303SetHoldTimer sets the value of the HOLD TIMER.

##### `FB_DALIV2x303SetReportTimer` — §4.1.2.10.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `410 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x303SetReportTimer sets the value of the REPORT TIMER.

#### Programming › POUs › Low-Level commands › Part 304 (brightness sensors)

##### `FB_DALIV2x304QueryDeadtimeTimer` — §4.1.2.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x304QueryDeadtimeTimer queries the value of the DEADTIME TIMER.

##### `FB_DALIV2x304QueryHysteresis` — §4.1.2.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x304QueryHysteresis queries the value for the hysteresis. The hysteresis is given in % and is within the range 0 % to 25 %.

##### `FB_DALIV2x304QueryHysteresisMin` — §4.1.2.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x304QueryHysteresisMin queries the value for the minimum possible hysteresis.

##### `FB_DALIV2x304QueryReportTimer` — §4.1.2.11.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x304QueryReportTimer queries the value of the REPORT TIMER.

##### `FB_DALIV2x304SetDeadtimeTimer` — §4.1.2.11.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** 416

##### `FB_DALIV2x304SetHysteresis` — §4.1.2.11.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x304SetHysteresis sets the value of the hysteresis.

##### `FB_DALIV2x304SetHysteresisMin` — §4.1.2.11.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x304SetHysteresisMin sets the value for the minimum possible hysteresis.

##### `FB_DALIV2x304SetReportTimer` — §4.1.2.11.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2x304SetReportTimer sets the value of the REPORT TIMER.

#### Programming › POUs › Third-party function blocks › Interior Automation

##### `FB_DALIV2IAPIR` — §4.1.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bReadDirectly` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOccupancy` | `BOOL` | The output is set if the sensor detects occupancy. |
| `nBrightness` | `INT` | Measured brightness in lux |
| `nRawValue` | `BYTE` | The value of the sensor read before the conversion |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block FB_DALIV2IAPIR cyclically reads the status of an IA PIR sensor and scales the measured brightness and detected occupancy based on the received value.

#### Programming › POUs › Third-party function blocks › B.E.G.

##### `FB_DALIV2xBEGLuxomat` — §4.1.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(* Occupancy Sensor Parameters *) 424 Programming (* Light Sensor Parameters *)` | `` | — |
| `bInitialize` | `BOOL` | The DALI control device is initialized via a positive edge at |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `(* Occupancy Sensor *) (* Light Sensor *)` | `` | — |
| `bInitializing` | `BOOL` | This output is TRUE during initialization. |
| `bError` | `BOOL` | This output is switched to TRUE as soon as an error |

**Description:** The function block FB_DALIV2xBEGLuxomat evaluates the measured brightness and the occupancy of the B.E.G. Luxomat DALI control device. This function block can also be used to initialize the DALI control device.

#### Programming › POUs › Third-party function blocks › Osram

##### `FB_DALIV2xOsramProfPushbuttonCoupler` — §4.1.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The DALI control device is initialized via a positive edge at |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitializing` | `BOOL` | This output is TRUE during initialization. Programming |
| `bError` | `BOOL` | This output is switched to TRUE as soon as an error |

**Description:** The function block FB_DALIV2xOsramProfPushbuttonCoupler evaluates the state of the digital input of the Osram DALI Professional Pushbutton Coupler. This function block can also be used to initialize the DALI control device.

##### `FB_DALIV2xOsramProfSensorCoupler` — §4.1.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(* Occupancy Sensor Parameters *) (* Light Sensor Parameters *)` | `` | — |
| `bInitialize` | `BOOL` | The DALI control device is initialized via a positive edge at |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `(* Occupancy Sensor *) (* Light Level *)` | `` | — |
| `bInitializing` | `BOOL` | This output is TRUE during initialization. |
| `bError` | `BOOL` | This output is switched to TRUE as soon as an error |

**Description:** The function block FB_DALIV2xOsramProfSensorCoupler evaluates the measured brightness and the occupancy of the Osram DALI Professional Sensor Coupler. This function block can also be used to initialize the DALI control device.

#### Programming › POUs › Third-party function blocks › Steinel

##### `FB_DALIV2xSteinelLiveLinkMotionSensor` — §4.1.3.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(* Occupancy Sensor Parameters *) (* Light Sensor Parameters *)` | `` | — |
| `bInitialize` | `BOOL` | The DALI control device is initialized via a positive edge at |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `(* Occupancy Sensor *) (* Light Sensor *)` | `` | — |
| `bInitializing` | `BOOL` | This output is TRUE during initialization. |
| `bError` | `BOOL` | This output is switched to TRUE as soon as an error |

**Description:** The function block FB_DALIV2xSteinelLiveLinkMotionSensor evaluates the measured brightness and the occupancy of the Steinel LiveLink DALI control device. This function block can also be used to initialize the DALI control device.

#### Programming › POUs › Third-party function blocks › Tridonic

##### `FB_DALIV2SmartSPOT` — §4.1.3.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `434 Programming` | `` | — |
| `bReadDirectly` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOccupancy` | `BOOL` | The output is set if the sensor detects occupancy. |
| `nBrightness` | `INT` | Measured brightness in lux |
| `nRawValue` | `BYTE` | The value of the sensor read before the conversion |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2SmartSPOT cyclically reads the status of a smartSPOT sensor or MSensor 02 and scales the measured brightness and the detected occupancy from the received value.

#### Programming › POUs › Third-party function blocks

##### `Theben HTS` — §4.1.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(* Occupancy Sensor Parameters *) (* Light Sensor Parameters *) 436 Programming` | `` | — |
| `bInitialize` | `BOOL` | The DALI control device is initialized via a positive edge at |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `(* Occupancy Sensor *) (* Light Sensor *) Programming` | `` | — |
| `bInitializing` | `BOOL` | This output is TRUE during initialization. |
| `bError` | `BOOL` | This output is switched to TRUE as soon as an error |

**Description:** Function blocks

#### Programming › POUs › [obsolete]

##### `FB_DALIV2Communication` — §4.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetMaximumDe BOOL` | `A positive edge resets the stored value of the maximum` | — |
| `mandCounter` | `command buffer utilization,` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDALIInData` | `ST_DALIV2InData` | — |
| `stDALIOutData` | `ST_DALIV2OutData` | — |
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrBufferDemandMe ARRAY` | `Demand of the respective buffer (0 - 100 %) ter` | — |
| `arrBufferMaximumD ARRAY` | `Previous maximum demand of the respective buffer (0 -` | — |
| `emandMeter` | `100 %)` | — |
| `arrBufferOverflowCo ARRAY` | `Number of buffer overflows so far unter 442 Programming` | — |
| `bLineIsBusy` | `BOOL` | This output is set as long as the function block |

**Description:** The function blocks for the DALI commands do not directly access the process image of the KL6811, but store the individual DALI commands in three different buffers. The function block FB_DALIV2Communication sequentially reads the DALI commands from these three buffers and passes the DALI commands to the KL6811. This prevents several function blocks accessing the process image of the KL6811 at the same time. Each of these three buffers is processed with a different priority (high, medium or low). You can use the eCommandPriority [} 462] parameter, which is available for most function blocks, to influence the priority with which the respective DALI command is to be processed by the function block FB_DALIV2Communication.

##### `FB_DALIV2EnableWriteMemory` — §4.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Enables write access via FB_DALIV2WriteMemoryLocation [} 181] to the internal memory of the control gear.

##### `FB_DALIV2QueryBallast` — §4.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2QueryBallast provides information on whether a specific control gear is ready to operate.

##### `FB_DALIV2QueryContentDTR` — §4.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The content of the DTR (Data Transfer Register) is read from the control gear.

##### `FB_DALIV2SendDALICommand` — §4.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**Description:** The function block FB_DALIV2SendDALICommand is used for the general sending of a DALI command, defined by command number and, if necessary, transfer parameters. Moreover, it is possible to set whether the command is sent twice in succession and whether to wait for a response. The latter can be used, for example, to realize a fast sequence of step-up commands.

#### (top level)

##### `Device for light emitting diodes (LEDs).` — §6

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

#### Programming › POUs › [obsolete]

##### `FB_DALIV2SetDTR` — §4.1.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This command is only available as a broadcast. Data is written to the DTR of all the control gears.

##### `FB_DALIV2StoreActualLevelInDTR` — §4.1.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** Programming

##### `FB_DALIV2StoreDTRAsFadeRate` — §4.1.5.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsFadeRate writes the value of the DTR to the variable FADE RATE [} 88]. The range of possible values extends from 1 to 15.

##### `FB_DALIV2StoreDTRAsFadeTime` — §4.1.5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsFadeTime writes the value of the DTR to the variable FADE TIME [} 88]. The range of possible values extends from 0 to 15.

##### `FB_DALIV2StoreDTRAsMaxLevel` — §4.1.5.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsMaxLevel writes the value of the DTR to the variable MAX LEVEL [} 88]. If the value provided is smaller than MIN LEVEL then the value is simply set to MIN LEVEL.

##### `FB_DALIV2StoreDTRAsMinLevel` — §4.1.5.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsMinLevel writes the value of the DTR to the variable MIN LEVEL [} 88] (minimum permitted lamp power). If the value provided is larger than MAX LEVEL [} 88] then the value is simply set to MAX LEVEL. If the value provided is smaller than the PHYSICAL MIN LEVEL [} 88] then MIN LEVEL will be set to PHYSICAL MIN LEVEL.

##### `FB_DALIV2StoreDTRAsPowerOnLevel` — §4.1.5.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsPowerOnLevel writes the value of the DTR to the variable POWER ON LEVEL [} 88].

##### `FB_DALIV2StoreDTRAsScene` — §4.1.5.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The content of the DTR is saved as the value of the lamp power for the given scene. The value range for the scene number extends from 0 to 15.

##### `FB_DALIV2StoreDTRAsShortAddress` — §4.1.5.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The content of the DTR (Data Transfer Register) is saved as the short address at the corresponding control gear. The structure of the DTR is 0AAA AAA1 (A: significant address bit) or 1111 1111 (mask). If the DTR contains 1111 1111 the short address is deleted from the control gear.

##### `FB_DALIV2StoreDTRAsSystemFailureLevel` — §4.1.5.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DALIV2CommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_DALIV2StoreDTRAsSystemFailureLevel writes the value of the DTR to the variable SYSTEM FAILURE LEVEL [} 88].

##### `FB_KL6811Config` — §4.1.5.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConfigurate` | `BOOL` | Configuration of the bus terminal is started by a positive |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInDataTerminal` | `ST_DALIV2InData` | — |
| `stOutDataTerminal` | `ST_DALIV2OutData` | — |
| `stInData` | `ST_DALIV2InData` | — |
| `stOutData` | `ST_DALIV2OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** The function block FB_KL6811Config is used to configure the KL6811. The configuration is executed when the PLC program starts, or it can be triggered by a positive edge at the input bConfigurate. The parameters are stored in the respective registers of the KL6811 in a fail-safe manner. In addition, some general information, such as the firmware version, is read from the KL6811.
