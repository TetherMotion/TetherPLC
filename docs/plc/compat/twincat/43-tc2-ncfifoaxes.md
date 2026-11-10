# Tc2_NcFifoAxes

- **Manual:** `TF5060_TCNC_FIFOAXES_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Motion/NC2 · **Tier:** T3 · **API entries:** 8
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** FIFO axes (synchronous data streams): NC kernel + deterministic buffers.

---

#### PLC Library

##### `FiFoGetDimension` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iChannelId` | `UDINT` | Channel ID of the FIFO channel |
| `bExecute` | `BOOL` | Edge-triggered signal for execution of the command |
| `tTimeout` | `TIME` | ADS timeout (about 1 second) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Becomes TRUE with a rising edge at bExecute, and remains TRUE until the block has executed the |

**Description:** The function block FiFoGetDimension determines the dimensioning, i.e. the number of the axes and the maximum number of FIFO entries in a FIFO channel.

##### `FiFoGroupIntegrate` — §4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iChannelId` | `UDINT` | Channel ID of the FIFO channel |
| `iAxisId` | `UDINT` | Axis ID of an axis that is to be inserted into the FIFO channel |
| `iGroupPosition` | `UDINT` | Position of the axis within the FIFO channel (iGroupPosition>=1) |
| `bExecute` | `BOOL` | Edge-triggered signal for execution of the command |
| `tTimeout` | `TIME` | ADS timeout (about 1 second) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Becomes TRUE with a rising edge at bExecute, and remains TRUE until the block has executed the |

**Description:** bExecute BOOL tTimeout TIME

##### `FiFoGroupDisintegrate` — §4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iChannelId` | `UDINT` | Channel ID of the FIFO channel |
| `bExecute` | `BOOL` | Edge-triggered signal for execution of the command |
| `tTimeout` | `TIME` | ADS timeout (about 1 second) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Becomes TRUE with a rising edge at bExecute, and remains TRUE until the block has executed the |

**Description:** The function block FiFoGroupDisintegrate dismantles a FIFO channel. This means that all the axes are removed from the FIFO channel and are placed in the standard PTP channel as independent PTP axes.

##### `FiFoWrite` — §4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `PLC Library` | `` | — |
| `iChannelId` | `UDINT` | Channel ID of the FIFO channel |
| `AdrDataArray` | `POINTER TO LREAL` | The address of a data field containing position data for the master and slave axes. The first of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Becomes TRUE with a rising edge at bExecute, and remains TRUE until the block has executed the |

**Description:** iRowsToWrite UDINT bExecute BOOL tTimeout TIME

##### `FiFoOverwrite` — §4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iChannelId` | `UDINT` | Channel ID of the FIFO channel |
| `AdrDataArray` | `POINTER TO LREAL` | The address of a data field containing position data for the master and slave axes. The first of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Becomes TRUE with a rising edge at bExecute, and remains TRUE until the block has executed the |

**Description:** iRowsToWrite UDINT bExecute BOOL tTimeout TIME

##### `FiFoStart` — §4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iChannelId` | `UDINT` | Channel ID of the FIFO channel |
| `bExecute` | `BOOL` | Edge-triggered signal for execution of the command PLC Library |
| `tTimeout` | `TIME` | ADS timeout (about 1 second) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Becomes TRUE with a rising edge at bExecute, and remains TRUE until the block has executed the |

**Description:** The function block FiFoStart initiates processing of the FIFO channel, so that all the axes in the FIFO channel are driven in accordance with the position data previously provided to the FIFO.

##### `FiFoStop` — §4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iChannelId` | `UDINT` | Channel ID of the FIFO channel |
| `bExecute` | `BOOL` | Edge-triggered signal for execution of the command |
| `tTimeout` | `TIME` | ADS timeout (about 1 second) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Becomes TRUE with a rising edge at bExecute, and remains TRUE until the block has executed the |

**Description:** The function block FiFoStop halts a FIFO channel. Processing of the position data from the FIFO stops, and all the axes in the FIFO channel are halted.

##### `FiFoSetChannelOverride` — §4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iChannelId` | `UDINT` | Channel ID of the FIFO channel |
| `iOverride` | `UDINT` | Channel override for the speed of the FIFO |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Becomes TRUE with a rising edge at bExecute, and remains TRUE until the block has executed the |

**Description:** tTimeout TIME
