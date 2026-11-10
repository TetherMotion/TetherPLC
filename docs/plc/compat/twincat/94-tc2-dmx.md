# Tc2_DMX

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_DMX_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 33
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** DMX512 lighting: real-time serial output at 250 kbaud — needs deterministic UART/RS485 service.

---

#### Programming › POUs › High Level

##### `FB_DMXDiscovery` — §4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this Programming |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `10 Programming` | `` | — |
| `bBusy` | `BOOL 12` | When the function block is activated the output is set, and Programming |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block searches for up to 50 DMX devices and automatically sets the start address (optional). The most important information for the devices found is displayed in a structure.

#### Programming › POUs › Low Level › Base

##### `FB_DMXSendRDMCommand` — §4.1.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `14 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block is for the general sending of a RDM command, defined by command number and, if necessary, transfer parameter.

##### `FB_EL6851Communication` — §4.1.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wSourceManufactur WORD` | `Unique manufacturer Id of the DMX device. Should be` | — |
| `erId` | `0x4241 according to the ESTA.` | — |
| `dwSourceDeviceId DWORD` | `Unique device Id of the DMX device. Can be freely` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stEL6851InData` | `ST_EL6851InData` | — |
| `stEL6851OutData` | `ST_EL6851OutData` | — |
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |
| `arrProcessData` | `ARRAY [1..512] OF BYTE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE as soon as an error |

**Description:** The EL6851 should always be accessed via this function block. This applies both to the transmission of the cyclic DMX data and to the transmission of the RDM commands.

##### `FB_EL6851CommunicationEx` — §4.1.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `18 Programming` | `` | — |
| `wSourceManufactur WORD` | `Unique manufacturer Id of the DMX device. Should be` | — |
| `erId` | `0x4241 according to the ESTA.` | — |
| `dwSourceDeviceId DWORD` | `Unique device Id of the DMX device. Can be freely` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stEL6851InData` | `ST_EL6851InData` | — |
| `stEL6851OutData` | `ST_EL6851OutData` | — |
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |
| `arrProcessData` | `ARRAY [1..512] OF BYTE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bError` | `BOOL` | This output is switched to TRUE as soon as an error |

**Description:** The EL6851 should always be accessed via this function block. This applies both to the transmission of the cyclic DMX data and to the transmission of the RDM commands.

#### Programming › POUs › Low Level › Device Control Parameter Message

##### `FB_DMXGetIdentifyDevice` — §4.1.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `20 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries whether or not the identification of a DMX device is active.

##### `FB_DMXSetIdentifyDevice` — §4.1.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `22 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block activates or deactivates the identification of a DMX device.

##### `FB_DMXSetResetDevice` — §4.1.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block activates a reset in a DMX device.

#### Programming › POUs › Low Level › Discovery Messages

##### `FB_DMXDiscMute` — §4.1.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block sets the mute flag of a DMX device. The mute flag specifies whether a DMX device reacts to the FB_DMXDiscUniqueBranch() [} 26] command (mute flag is not set) or not (mute flag is set).

##### `FB_DMXDiscUniqueBranch` — §4.1.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries whether DMX devices are located within a certain address range. This command is used for the discovery of the connected DMX devices.

##### `FB_DMXDiscUnMute` — §4.1.2.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `28 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block resets the mute flag of a DMX device. The mute flag specifies whether a DMX device reacts to the FB_DMXDiscUniqueBranch() [} 26] command (mute flag is not set) or not (mute flag is set).

#### Programming › POUs › Low Level › Power and Lamp Setting Parameter Messages

##### `FB_DMXGetLampHours` — §4.1.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `30 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block reads the number of hours in which the lamp was on. The function block FB_DMXSetLampHours() [} 32] can be used to edit the hour counter.

##### `FB_DMXGetLampOnMode` — §4.1.2.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block reads the parameter that defines the switch-on behavior of the DMX device. The function block FB_DMXSetLampOnMode() [} 34] can be used to edit the value.

##### `FB_DMXSetLampHours` — §4.1.2.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `32 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block sets the operating hours counter for the lamp. The function block FB_DMXGetLampHours() [} 30] can be used to read the counter.

##### `FB_DMXSetLampOnMode` — §4.1.2.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `34 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block defines the switch-on behavior of the DMX device. The function block FB_DMXGetLampOnMode() [} 31] can be used to read the set value.

#### Programming › POUs › Low Level › Product Information Messages

##### `FB_DMXGetDeviceInfo` — §4.1.2.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries all relevant information from a DMX device.

##### `FB_DMXGetDeviceLabel` — §4.1.2.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block reads a text from the DMX device, which contains a more detailed description of the device. The function block FB_DMXSetDeviceLabel() [} 44] can be used to edit the text.

##### `FB_DMXGetDeviceModelDescription` — §4.1.2.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `38 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries the description of the device type.

##### `FB_DMXGetManufacturerLabel` — §4.1.2.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `40 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries the description of the DMX device manufacturer.

##### `FB_DMXGetProductDetailIdList` — §4.1.2.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries the categories to which the DMX device belongs.

##### `FB_DMXGetSoftwareVersionLabel` — §4.1.2.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries the description of the software version of the DMX device.

##### `FB_DMXSetDeviceLabel` — §4.1.2.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `44 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block writes a description text into the DMX device. The function block FB_DMXGetDeviceLabel() [} 37] can be used to read the text.

#### Programming › POUs › Low Level › Queued and Status Messages

##### `FB_DMXClearStatusId` — §4.1.2.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `46 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block clears the message buffer in the DMX device.

##### `FB_DMXGetStatusIdDescription` — §4.1.2.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block reads the text of a certain status Id from the DMX device.

##### `FB_DMXGetStatusMessages` — §4.1.2.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `48 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block collects the status or error information of an DMX device.

#### Programming › POUs › Low Level › RDM Information Messages

##### `FB_DMXGetParameterDescription` — §4.1.2.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `50 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries the definition of manufacturer-specific PIDs.

##### `FB_DMXGetSupportedParameters` — §4.1.2.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries all supported parameters from a DMX device.

#### Programming › POUs › Low Level › Sensor Parameter Messages

##### `FB_DMXGetSensorDefinition` — §4.1.2.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries the definition of a particular sensor.

##### `FB_DMXGetSensorValue` — §4.1.2.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `54 Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries the current value of a sensor.

#### Programming › POUs › Low Level › Setup Messages

##### `FB_DMXGetDMX512PersonalityDescription` — §4.1.2.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block reads further Personality information from the DMX device. Some DMX devices support so-called Personalities. Changing the Personality can influence certain RDM parameters.

##### `FB_DMXGetDMX512StartAddress` — §4.1.2.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries the DMX512 start address. This lies within the range from 1 – 512. If the DMX device does not occupy any DMX slot, then the DMX512 start address is 0xFFFF (65535). Each sub-device and the root device occupy different DMX512 start addresses.

##### `FB_DMXGetSlotDescription` — §4.1.2.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries the text description for slot offsets.

##### `FB_DMXGetSlotInfo` — §4.1.2.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block queries the basic information of the functionality of the DMX512 slots of a DMX device.

##### `FB_DMXSetDMX512StartAddress` — §4.1.2.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_DMXCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated the output is set, and |

**Description:** This function block sets the DMX512 start address. This lies within the range from 1 – 512. Each sub-device and the root device occupy different DMX512 start addresses.
