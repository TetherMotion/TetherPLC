# Tc2_EnOcean

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_EnOcean_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 17
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** EnOcean wireless: needs an EnOcean transceiver interface.

---

#### Programming › POUs

##### `FB_EnOceanReceive` — §4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | A positive signal at this input activates the function block. A |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stEnOceanInData` | `ST_EnOceanInData` | — |
| `stEnOceanOutData` | `ST_EnOceanOutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** The function block FB_EnOceanReceive() is a receive block, which makes the telegrams sent by the EnOcean® modules available in the structure stEnOceanReceivedData. This structure can then be analyzed with further function blocks. The documentation for this function blocks also includes sample programs, which illustrate the operating principle.

##### `Read` — §4.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | A positive signal at this input enables the function block. A |
| `bEnable` | `BOOL` | A positive signal at this input enables the function block. A |
| `bEnable` | `BOOL` | A positive signal at this input enables the function block. A |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitches` | `ARRAY OF BOOL` | This field of 8 Boolean values describes the states of the 8 |
| `bSwitches` | `ARRAY OF BOOL` | This field of 4 Boolean values describes the states of the 4 |
| `eEnOceanRotarySwi E_EnOceanRotarySwitch` | `The output at this output describes the position of the` | — |
| `tch` | `[} 38]` | rotary switch on the room control unit. |
| `nSetpoint` | `INT` | This output variable indicates the setpoint value set on the |

**Description:** The function block FB_EnOceanPTM100() provides a user-friendly evaluation of the state of an EnOcean® PTM100 module. The function block FB_EnOceanReceive() [} 14] is required for this purpose.

#### Programming › POUs › Read

##### `FB_EnOceanSTM100Generic` — §4.1.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | A positive signal at this input enables the function block. A |
| `bEnable` | `BOOL` | A positive signal at this input enables the function block. A |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nDataBytes` | `ARRAY OF BYTE` | 4-byte array with the user data sent by the STM100 module. The purpose of the individual bytes is |
| `bState` | `BOOL` | When the reed contact on the STM250 module is |

**Description:** The function block FB_EnOceanSTM100Generic() provides a user-friendly evaluation of the data of an EnOcean® STM100 module. The function block FB_EnOceanReceive() [} 14] is required for this purpose.

#### Programming › POUs

##### `Error codes` — §4.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInit` | `BOOL` | Activates the function block that configures the KL6301 |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stKL6581_in` | `KL6581_Input` | — |
| `stKL6581_out` | `KL6581_Output` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReady` | `BOOL` | The function block is ready for sending and receiving data. |
| `bBusy` | `BOOL` | The function block is active. No new data can be sent yet. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block takes care of communication with the KL6581 EnOcean® Bus Terminal. This function block is used to configure the KL6581 and start data exchange with the EnOcean® network.

#### Programming › POUs › Read

##### `FB_Rec_Generic` — §4.1.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `str_KL6581` | `STR_KL6581 [} 44]` | Is connected to the data structure of the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ar_Value` | `ARRAY OF BYTE` | EnOcean® data 4 bytes. |
| `by_Node` | `BYTE` | Node number of the KL6583 that has received the |

**Description:** This function block receives all data that were received via EnOcean®. This function block can be used for all types of EnOcean® telegrams.

##### `FB_Rec_1BS` — §4.1.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `str_KL6581` | `STR_KL6581 [} 44]` | Is connected to the data structure of the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOpen` | `BOOL` | Contact open |
| `bClose` | `BOOL` | Contact closed |
| `bLRN` | `BOOL` | LRN button pressed |
| `by_Node` | `BYTE` | Node number of the KL6583 that has received the |

**Description:** This function block receives data that were received via EnOcean®. This function block is used for integration of window contacts, for example (ORG FIELD 6).

##### `FB_Rec_RPS_Switch` — §4.1.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `str_KL6581` | `STR_KL6581 [} 44]` | Is connected to the data structure of the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `str_EnOceanSwitch STR_EnOceanSwitch` | `Switch data [} 43]` | — |
| `by_Node` | `BYTE` | Node number of the KL6583 that has received the |

**Description:** This function block receives data from a switch that were received via EnOcean®. The function block outputs the data in a data structure (ORG Field 5).

##### `FB_Rec_RPS_Window_Handle` — §4.1.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `str_KL6581` | `STR_KL6581 [} 44]` | Is connected to the data structure of the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ar_Data` | `AR_EnOceanWindow [} 43]` | Window handle data |
| `by_Node` | `BYTE` | Node number of the KL6583 that has received the |

**Description:** This function block receives data from a window handle that were received via EnOcean®. The function block outputs the data in a data structure (ORG Field 5).

#### Programming › POUs › Send

##### `FB_Send_Generic` — §4.1.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Positive edge sends the data. |
| `by_Node` | `BYTE` | Address of the KL6583 module to which the telegram is to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. No new data can be sent yet. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** 28

##### `FB_Send_4BS` — §4.1.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Positive edge sends the data. |
| `by_Node` | `BYTE` | Address of the KL6583 module to which the telegram is to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. No new data can be sent yet. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends data via EnOcean®. The ORG field is set permanently to 7.

##### `FB_Send_RPS_Switch` — §4.1.2.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `30 Programming` | `` | — |
| `bStart` | `BOOL` | Positive edge sends the data. |
| `by_Node` | `BYTE` | Address of the KL6583 module to which the telegram is to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. No new data can be sent yet. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends EnOcean telegrams in the format of a switch. The value of bData is sent with a positive edge of bStart. In order to simulate a button press, the function block usually has to be started twice, once with bData = TRUE, once with bData = FALSE. For simpler handling the function block FB_Send_RPS_SwitchAuto() [} 31] can be used.

##### `FB_Send_RPS_SwitchAuto` — §4.1.2.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | Positive edge sends the data. |
| `bData` | `BOOL` | Value to be transmitted. |
| `by_Node` | `BYTE` | Address of the KL6583 module to which the telegram is to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. No new data can be sent yet. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends EnOcean telegrams in the format of a switch. The value of bData is sent with a positive edge of bStart. The signal "Release switch" is sent once the time t_SwitchDelay has elapsed.

#### Programming › POUs › Other

##### `FB_EnOcean_Search` — §4.1.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | If TRUE the function block is enabled, if FALSE it is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReceive` | `BOOL` | On receiving an EnOcean® telegram this value is set to |

**Description:** This function block displays all EnOcean® IDs that it has received and enters them in a receive array (ar_ID). Up to 256 EnOcean® devices can be recognized. Alternatively the function block can also be created separately for each KL6583. This can be used to recognize whether an EnOcean® device is being received by several KL6583s.

##### `FB_Rec_Teach_In` — §4.1.2.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | If TRUE the function block is enabled, if FALSE it is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLearnType` | `BOOL` | If the bit is set you will find further data in the structure |

**Description:** This function block indicates when a Learn button is pressed on an EnOcean® device. If the flag bLearnType is set, further information about the EnOcean® device can be read. This function must be provided by the EnOcean® device. So far, however, it is only supported by very few EnOcean® devices.

##### `FB_Rec_Teach_In_Ex` — §4.1.2.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | If TRUE the function block is enabled, if FALSE it is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLearnType` | `BOOL` | If the bit is set you will find further data in the structure |

**Description:** This function block indicates when a Learn button is pressed on an EnOcean® device. If the flag bLearnType is set, further information about the EnOcean® device can be read. This function must be provided by the EnOcean® device. So far, however, it is only supported by very few EnOcean® devices.

#### Programming › POUs › Function

##### `F_Byte_to_Temp : REAL` — §4.1.2.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `byData` | `BYTE` | Raw data |
| `minTemp` | `REAL` | Minimum temperature |
| `maxTemp` | `REAL` | Maximum temperature |

**Description:** This function converts a byte raw value into a REAL variable.

##### `F_Byte_to_TurnSwitch` — §4.1.2.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `byData` | `BYTE` | Raw data |

**Description:** This function converts a raw byte value to a Boolean array in the form of a data structure (see STREnOceanTurnSwitch [} 45]).
