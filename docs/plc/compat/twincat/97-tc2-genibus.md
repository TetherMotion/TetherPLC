# Tc2_GENIbus

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_GENIbus_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 6
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** GENIbus: legacy building protocol stack.

---

#### Programming › POUs › Base

##### `FB_GENIbusCommunication` — §4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bResetMaximumDe BOOL` | `A rising edge resets the stored value of the maximum` | — |
| `mandCounter` | `command buffer utilization, arrBufferMaximumDemandMeter (0 - 100%, see` | — |
| `bResetOverflowCou BOOL` | `A rising edge resets the stored value of the number of` | — |
| `nter` | `command buffer overflows, arrBufferOverflowCounter (see` | — |
| `byMasterAddr` | `BYTE` | Specifies the address that the TwinCAT controller should |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_GENIbusInData` | — |
| `stOutData` | `ST_GENIbusOutData` | — |
| `stCommandBuffer` | `ST_GENIbusCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Starting with the edge at bStart, this output remains TRUE |

**Description:** The function blocks for the GENIbus commands do not directly access the process image of the selected serial interface; instead, they place the individual GENIbus commands into three different buffers. The function block FB_GENIbusCommunication() sequentially reads the GENIbus commands from these three buffers and forwards the GENIbus commands to the serial interface. This prevents multiple function blocks accessing the process image of the serial interface at the same time. Each of these three buffers is processed with a different priority (high, medium or low). The parameter eCommandPriority [} 28], which is available for most function blocks, can be used to specify the priority with which the respective GENIbus command is processed by the function block FB_GENIbusCommunication().

#### Programming › POUs › Basic commands

##### `FB_GENIbusGet` — §4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The reading process is initiated by a rising edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_GENIbusCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Starting with the edge at bStart, this output remains TRUE |

**Description:** This function block reads a value from a GENIbus device.

##### `FB_GENIbusSet` — §4.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | A rising edge at this input starts the setting process. |
| `byAdress` | `BYTE` | Address of the GENIbus device to be addressed: valid |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_GENIbusCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | Starting with the edge at bStart, this output remains TRUE |

**Description:** This function block writes a value in a GENIbus device or executes a command (Class-3 IDs).

##### `FB_GENIbusInfo` — §4.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The reading process is initiated by a rising edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_GENIbusCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Starting with the edge at bStart, this output remains TRUE |

**Description:** This function block reads the information area of an ID.

##### `FB_GENIbusGetMValue` — §4.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The reading process is initiated by a rising edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_GENIbusCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Starting with the edge at bStart, this output remains TRUE |

**Description:** This function block reads a measured value from a GENIbus device. The operation is limited exclusively to values of class 2. Only the ID code of the high-byte and the length of the measured value need to be specified; the type of scaling and the unit of the measured value are determined by an internal INFO query. A structure at the stMValue output provides all important information about the value.

#### Programming › POUs › Pumps

##### `FB_GENIbusMagnaPump` — §4.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `22 Programming` | `` | — |
| `bEnable` | `BOOL` | The function block is enabled by setting this input. |
| `byAdress` | `BYTE` | Address of the GENIbus device to be addressed: valid |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_GENIbusCommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Starting with the edge at bStart, this output remains TRUE |

**Description:** This function block represents a universal application for a Grundfos Magna pump. The fundamental operating modes can be set and important parameters read out.
