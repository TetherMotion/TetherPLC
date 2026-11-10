# Tc2_ProfinetDiag

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_ProfinetDiag_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T2 · **API entries:** 24
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Profinet diagnostics need a Profinet stack; none exists.

---

#### Function blocks › Controller › AlarmDiag

##### `FB_PN_ALARM_DIAG` — §3.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Enabling the function block. |
| `NETID` | `T_AmsNetId` | AMS Net ID of the controller |
| `PORT` | `T_AmsPort` | Port via which the controller communicates with the device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** 10

#### Function blocks › Controller › Identification and Maintenance

##### `FB_PN_IM0_READ` — §3.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** 12

##### `FB_PN_IM1_READ` — §3.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `iErrorID` | `; UDINT;` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The PROFINET controller uses the function block FB_PN_IM1_READ to read all I&M1 (Information & Maintenance) data from a device referenced via the Port input. The frame structure of the I&M1 function corresponds to the index 0xAFF1 according to the PROFINET standard.

##### `FB_PN_IM1_WRITE` — §3.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `14 Function blocks` | `` | — |
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The PROFINET controller uses the function block FB_PN_IM1_WRITE to write all I&M1 (Identification & Maintenance) data to a device referenced via the input Port. The frame structure of the I&M1 function corresponds to the index 0xAFF1 according to the PROFINET standard.

##### `FB_PN_IM2_READ` — §3.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The PROFINET controller uses the function block FB_PN_IM2_READ to read all I&M 2 (Identification & Maintenance) data from a device referenced via the input Port. The frame structure of the I&M2 function corresponds to the index 0xAFF2 according to the PROFINET standard.

##### `FB_PN_IM2_WRITE` — §3.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `16 Function blocks` | `` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The PROFINET controller uses the function block FB_PN_IM2_WRITE to write all I&M 2 (Identification & Maintenance) data to a device referenced via the input Port. The frame structure of the I&M2 function corresponds to the index 0xAFF2 according to the PROFINET standard.

##### `FB_PN_IM3_READ` — §3.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The PROFINET controller uses the function block FB_PN_IM3_READ to read all I&M3 (Identification & Maintenance) data from a device referenced via the input Port. The frame structure of the I&M3 function corresponds to the index 0xAFF3 according to the PROFINET standard.

##### `FB_PN_IM3_WRITE` — §3.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The PROFINET controller uses the function block FB_PN_IM3_WRITE to write all I&M3 (Identification & Maintenance) data to a device referenced via the input Port. The frame structure of the I&M3 function corresponds to the index 0xAFF3 according to the PROFINET standard.

##### `FB_PN_IM4_Read` — §3.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The PROFINET controller uses the function block FB_PN_IM4_READ to read all I&M4 (Identification & Maintenance) data from a device referenced via the input Port. The frame structure of the I&M4 function corresponds to the index 0xAFF4 according to the PROFINET standard.

##### `FB_PN_IM4_WRITE` — §3.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** Function blocks

#### Function blocks › Controller › PROFINET RT Controller

##### `FB_PN_ReadStateOfDevices` — §3.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** When called, the function block FB_PN_ReadDtateOfDevices returns the number of configured and missing devices and those with diagnosis information.

##### `FB_PN_ReadCompleteInfoOfDevices` — §3.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_PN_ReadCompleteInfoOfDevices creates a list with the complete information of the configured devices (in TwinCAT). For PROFINET RT controllers such as TF6271 or CCAT M930 interface or EL6631 v11(v.024).

#### Function blocks › Controller

##### `FB_SET_PN_NAME` — §3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `22 Function blocks` | `` | — |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_SET_PN_NAME assigns a name to the respective PROFINET device. When assigning, ensure that only PROFINET-compliant characters are used.

##### `FB_RESET_PN_TO_FACTORY_SETTINGS` — §3.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_RESET_PN_TO_FACTORY resets the respective PROFINET device to its vendor settings.

##### `FB_PN_SCAN` — §3.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `iFind_Devices` | `int` | Number of PROFINET devices in the configuration. |
| `ar_PN_DEVICE` | `ARRAY OF str_PN_SCAN PROFINET/IP settings of the PROFINET devices. [} 38]` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |
| `iFind_Devices` | `int` | Number of PROFINET devices in the configuration. |
| `ar_PN_DEVICE` | `ARRAY OF str_PN_SCAN PROFINET/IP settings of the PROFINET devices. [} 38]` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_PN_SCAN scans the PROFINET network and returns the number and a list with information on the PROFINET devices found.

#### Function blocks › Device

##### `FB_READ_PROFINET_NAME` — §3.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |
| `bWrite` | `BOOL` | A positive edge at this input enables the function block and |
| `bRead` | `BOOL` | A positive edge at this input enables the function block and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |
| `bBusy` | `BOOL 28` | When the function block is enabled, this output is set and Function blocks |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_READ_PROFINET_NAME reads the PROFINET name of an EL6631-0010 and, if configured, of a virtual EL6631-0010.

#### Function blocks › Device › Device via CCAT (CX-B930,TF6270)

##### `FB_PROFINET_READ_IM` — §3.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `str_IuM_Data` | `str_IuM_Data [} 41]` | Structural element with I&M data |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_PROFINET_READ_IM returns the I&M (Identification & Maintenance) data of the respective PROFINET device.

##### `FB_PROFINET_READ_NAME` — §3.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `30 Function blocks` | `` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_PROFINET_READ_NAME returns the PROFINET name of the respective PROFINET device and the information whether the PROFINET name can be changed by the PROFINET controller.

##### `FB_PROFINET_READ_PRM` — §3.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | AMS Net ID of the controller |
| `PORT` | `T-AmsPort` | Port via which the controller communicates with the device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `str_Diag_PN_Settin str_Diag_PN_Settings` | `PROFINET/IP settings` | — |
| `gs` | `[} 41]` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_PROFINET_READ_PRM scans the PROFINET network and returns the number and a list with information on the PROFINET devices found.

##### `FB_PROFINET_WRITE_IM` — §3.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_PROFINET_WRITE_IM can be used to write the I&M (Identification & Maintenance) data of the respective PROFINET device.

##### `FB_PROFINET_SET_NAME` — §3.2.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_PROFINET_SET_NAME can be used to change the PROFINET name of the respective PROFINET device. PROFINET driver version 06 (V00.34) or higher, TF6270, CCAT PN Interface(B930) required

#### Function blocks › Device

##### `FB_PN_SEND_ALARM` — §3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_PN_SEND_ALARM sends an alarm to the controller.

#### Function blocks

##### `FB_PN_GET_PORT_STATISTIC` — §3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** When called, the function block FB_PN_GET_PORT_STATISTIC provides the statistical data on the ports of a PROFINET device.

##### `FB_PN_READ_PORT_DIAG` — §3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_PN_READ_PORT_DIAG retrieves the diagnosis information of the ports of a PROFINET device.
