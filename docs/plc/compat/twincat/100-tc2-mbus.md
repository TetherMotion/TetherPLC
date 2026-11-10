# Tc2_MBus

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_MBus_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 65
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** M-Bus (meter bus): serial master service.

---

#### disabled.

##### `POUs` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiRetries` | `USINT` | Number of retries in the event of errors |
| `bStart` | `BOOL` | All meters are read once on the positive edge of this input. |
| `bDisabled` | `BOOL` | TRUE = deselection of the function block. Inputs/outputs |
| `stComIn` | `ST_KL6781inData22B [} 214]` | Process image of the inputs |
| `stComOut` | `ST_KL6781outData22B [} 214]` | Process image of the outputs |
| `stCom` | `ST_MBUS_Communication This structure is used to link the function block with the [} 215]` | meter blocks. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stComIn` | `ST_KL6781inData22B` | — |
| `stComOut` | `ST_KL6781outData22B` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The vendor-specific function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General() [} 80], FB_MBUS_General_Ext() [} 84] or FB_MBUS_General_Param() [} 88] from the "General [} 78]" folder should be used. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send() [} 90] can be used.

#### disabled. › POUs › ABB overview

##### `FB_MBUS_ABB_DZ` — §4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_ABB_DZ is used to read electricity meters from ABB:

#### disabled. › POUs › Actaris overview

##### `FB_MBUS_ACW_CF` — §4.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_ACW_CF is used to read heat meters from Actaris:

##### `FB_MBUS_ACW_PlusM` — §4.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_ACW_PlusM is used to read Actaris water meters:

#### disabled. › POUs › Aquametro overview

##### `FB_MBUS_AMT_AMBUS` — §4.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_AMT_AMBUS is used to read pulse collectors from Aquametro:

##### `FB_MBUS_AMT_AMTRON` — §4.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_AMT_AMTRON is used to read out heat meters from Aquametro:

##### `FB_MBUS_AMT_CALEC` — §4.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_AMT_CALEC is used to read heat meters from Aquametro:

##### `FB_MBUS_AMT_SAPHIR` — §4.1.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** 36

#### disabled. › POUs › Berg overview

##### `FB_MBUS_BEC_DCMi` — §4.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_BEC_DCMi is used to read electricity meters from Berg:

##### `FB_MBUS_BEC_DZ` — §4.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_BEC_DZ is used to read electricity meters from Berg:

#### disabled. › POUs › Brunata overview

##### `FB_MBUS_BHG_HGx` — §4.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_BHG_HGx is used to read heat meters from Brunata:

#### disabled. › POUs

##### `Carlo Gavazzi overview` — §4.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

#### disabled. › POUs › Cynox

##### `FB_MBUS_CYN_MCount2C` — §4.1.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_CYN_MCount2C is used to read pulse counters from Cynox.

#### disabled. › POUs

##### `Elster overview` — §4.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

##### `elvaco overview` — §4.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

#### disabled. › POUs › EMH overview

##### `FB_MBUS_EMH_DIZ` — §4.1.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_EMH_DIZ is used to read electricity meters from EMH:

##### `FB_MBUS_EMH_EIZE` — §4.1.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_EMH_EIZE is used to read electricity meters from EMH:

##### `FB_MBUS_EMH_EIZG` — §4.1.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_EMH_EIZG is used to read electricity meters from EMH:

##### `FB_MBUS_EMH_MIZ` — §4.1.11.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_EMH_MIZ is used to read electricity meters from EMH:

#### disabled. › POUs

##### `EMU overview` — §4.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

#### disabled. › POUs › EMU overview

##### `FB_MBUS_EMU_3_5_Allrounder` — §4.1.12.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_EMU_3_5_Allrounder is used to read electricity meters from EMU.

#### disabled. › POUs › Engelmann overview

##### `FB_MBUS_EFE_SensoStar2C` — §4.1.13.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_EFE_SensoStar2C is used to read heat meters from Engelmann:

#### disabled. › POUs › General device blocks

##### `FB_MBUS_ChangeAdr` — §4.1.14.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAdrOld` | `USINT` | Old primary address |
| `usiAdrNew` | `USINT` | New primary address |
| `eBaudrate` | `E_MBUS_Baudrate [} 209] 300, 600, 1200, 2400, 4800, 9600 baud` | — |
| `bStart` | `BOOL` | A positive edge at this input triggers a change of the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The primary address can be changed using the function block FB_MBUS_ChangeAdr.

##### `FB_MBUS_General` — §4.1.14.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_General is used to read any M-Bus devices. The variable arrData [} 216] provides a maximum of cMBUS_MaxData [} 219] values of the 1st telegram. String values and vendor- specific information are not shown correctly.

##### `FB_MBUS_General_Electricity` — §4.1.14.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_General_Electricity is used to read electricity meters.

##### `FB_MBUS_General_Ext` — §4.1.14.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** Some M-Bus devices send values distributed over several telegrams. The function block FB_MBUS_General_Ext can be used to read all telegrams from any M-Bus devices.

##### `FB_MBUS_General_Heat` — §4.1.14.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_General_Heat is used to read heat meters.

##### `FB_MBUS_General_Param` — §4.1.14.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_General_Param is used to read any M-Bus devices. The variable arrData [} 216] supplies cMBUS_MaxDataParam values.

##### `FB_MBUS_General_Send` — §4.1.14.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_General_Send is used to send data to any M-Bus devices (the primary address of the meter can be changed with this function block, for example).

##### `FB_MBUS_General_Water` — §4.1.14.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stComIn` | `ST_KL6781inData22B` | — |
| `stComOut` | `ST_KL6781outData22B` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_General_Water is used to read water meters.

##### `FB_MBUS_RawData` — §4.1.14.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_RawData is used to read any M-Bus devices. The variable arrData supplies the raw data of the M-Bus device. Only the first telegram is evaluated.

##### `FB_MBUS_Scan` — §4.1.14.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The search is started with a positive edge at this input. |
| `bStop` | `BOOL` | The search is stopped with a positive edge at this input. |
| `eBaudrate` | `E_MBUS_Baudrate [} 209] 300, 600, 1200, 2400, 4800, 9600 baud` | — |
| `bDisabled` | `BOOL` | TRUE = deselection of the function block Inputs/outputs |
| `stCom` | `ST_MBUS_Communication This structure is used to link the function block` | — |
| `[} 215]` | `FB_MBUSKL6781() [} 215] with the meter blocks.` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_Scan can be used to scan the M-Bus. All primary addresses (0..250) are queried successively. The array arrDevice is used to show certain device information.

#### disabled. › POUs › Gossen Metrawatt overview

##### `FB_MBUS_GMC_Electricity` — §4.1.15.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_GMC_Electricity is used to read electricity meters from Gossen Metrawatt:

#### disabled. › POUs › GWF overview

##### `FB_MBUS_GWF_Coder` — §4.1.16.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_GWF_Coder is used to read meters from GWF:

#### disabled. › POUs › Hydrometer overview

##### `FB_MBUS_HYD_Flypper` — §4.1.17.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_HYD_Flypper is used to read water meters from Hydrometer:

##### `FB_MBUS_HYD_PortAnalog` — §4.1.17.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_HYD_PortAnalog is used to read energy meters with an analog output from Hydrometer:

##### `FB_MBUS_HYD_PortPulse` — §4.1.17.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_HYD_PortPulse is used to read energy meters with pulse output from Hydrometer:

##### `FB_MBUS_HYD_Sharky` — §4.1.17.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL 112` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_HYD_Sharky is used to read energy meters of the companies:

#### disabled. › POUs › ista overview

##### `FB_MBUS_IST_Istameter` — §4.1.18.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_IST_Istameter is used to read water meters from Ista:

##### `FB_MBUS_IST_IstameterIII` — §4.1.18.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_IST_IstameterIII is used to read water meters from Ista:

##### `FB_MBUS_IST_PulsonicII` — §4.1.18.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_IST_PulsonicII is used to read energy meters with pulse output from Ista:

##### `FB_MBUS_IST_SensonicII` — §4.1.18.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_IST_SensonicII is used to read heat meters from Ista:

#### disabled. › POUs › Itron

##### `FB_MBUS_ITR_IntegralVUltraLite` — §4.1.19.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_ITR_IntegralVUltraLite is used to read energy meters from Itron.

#### disabled. › POUs › Janitza overview

##### `FB_MBUS_JAN_UMG96S` — §4.1.20.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_JAN_UMG96S is used to read electricity meters from Janitza:

#### disabled. › POUs › Kamstrup overview

##### `FB_MBUS_KAM_KamstrupE` — §4.1.21.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_KAM_KamstrupE is used to read electricity meters from Kamstrup:

##### `FB_MBUS_KAM_Maxical_III` — §4.1.21.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_KAM_Maxical_III is used to read heat/cold meters from Kamstrup:

##### `FB_MBUS_KAM_Multical` — §4.1.21.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_KAM_Multical is used to read heat/cold meters from Kamstrup:

#### disabled. › POUs

##### `Kundo overview` — §4.1.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT 146` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

#### disabled. › POUs › Kundo overview

##### `FB_MBUS_KST_him1Puls` — §4.1.22.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_KST_him1Puls is used to read M-Bus modules from KUNDO System Technik:

#### disabled. › POUs › Landis & Gyr overview

##### `FB_MBUS_LUG_Heat` — §4.1.23.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_LUG_Heat is used to read heat/cold meters from Landis & Gyr:

#### disabled. › POUs

##### `Metrima overview` — §4.1.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

#### disabled. › POUs › Metrima overview

##### `FB_MBUS_SVM_F22_Ext` — §4.1.24.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_SVM_F22_Ext is used to read heat meters from Metrima:

#### disabled. › POUs

##### `NZR overview` — §4.1.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

##### `OPTEC overview` — §4.1.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `usiRecivedAd r` | `: USINT;` | — |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

#### disabled. › POUs › Relay overview

##### `FB_MBUS_REL_AnDi` — §4.1.27.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL 168` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL 170` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL 172` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_REL_AnDi is used to read analog converters from Relay:

#### disabled. › POUs

##### `Saia-Burgess overview` — §4.1.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL 180` | The bBusy output is TRUE as long as the meter is being |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

#### disabled. › POUs › Schlumberger overview

##### `FB_MBUS_SLB_CFEchoI` — §4.1.29.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_SLB_CFEchoI is used to read heat meters from Schlumberger:

##### `FB_MBUS_SLB_MK_MaXX` — §4.1.29.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_SLB_MK_MaXX is used to read heat meters from Schlumberger:

#### disabled. › POUs

##### `Schneider Electric` — §4.1.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

#### disabled. › POUs › Sensus overview

##### `FB_MBUS_SEN_Pollu` — §4.1.31.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_SEN_Pollu is used to read heat/cold meters from Sensus:

##### `FB_MBUS_SEN_Water` — §4.1.31.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_SEN_Water is used to read water meters from Sensus.

#### disabled. › POUs

##### `Sontex overview` — §4.1.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

#### disabled. › POUs › TIP

##### `FB_MBUS_TIP_SINUS85M` — §4.1.33.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_TIP_SINUS85M is used to read electricity meters from Thüringer Industrie

#### disabled. › POUs

##### `Zenner overview` — §4.1.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function blocks only output a selection of the most common data. These data are described on the respective pages under "Outputs". If more or all data is required, the function blocks FB_MBUS_General [} 80], FB_MBUS_General_Ext [} 84] or FB_MBUS_General_Param [} 88] from the "General [} 78]" folder should be used. Note that these function blocks cannot be run on BC and BX systems. If data must be sent to the device (e.g. setting the primary address), the function block FB_MBUS_General_Send [} 90] can be used.

#### disabled. › POUs › Zenner overview

##### `FB_MBUS_ZRM_zelsiusZR` — §4.1.34.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `usiAddress` | `USINT` | Primary address [} 11] of the meter to be read with this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCom` | `ST_MBUS_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as the meter is being |

**Description:** The function block FB_MBUS_ZRM_zelsiusZR is used to read heat meters from Zenner.
