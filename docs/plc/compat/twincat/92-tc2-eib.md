# Tc2_EIB

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_EIB_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 48
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** EIB/KNX building bus: needs an EIB/KNX stack.

---

#### POUs

##### `Function blocks details` — §4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bActivate` | `BOOL` | Activates the function block that configures the KL6301 |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `KL6301_IN` | `ARRAY [1..24] OF BYTE` | — |
| `KL6301_OUT` | `ARRAY [1..24] OF BYTE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bActive` | `BOOL` | The function block was activated. |
| `bReady` | `BOOL` | The function block is ready for sending and receiving data. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** UAL TROL NTROL N SIGN _SIGN _UNSIGN _FLOAT _SIGN _UNSIGN _FLOAT A_TYPES_EX

##### `KL6301_EX` — §4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bActivate` | `BOOL` | Activates the function block that configures the KL6301 |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `KL6301_IN` | `ARRAY [1..24] OF BYTE` | — |
| `KL6301_OUT` | `ARRAY [1..24] OF BYTE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bActive` | `BOOL` | The function block was activated. |
| `bReady` | `BOOL` | The function block is ready for sending and receiving data. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block deals with the KL6301 EIB Bus Terminal communication. It is used for configuring the KL6301 and for starting the data exchange with the EIB network.

#### POUs › Read

##### `EIB_2OCTET_FLOAT_REC` — §4.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 2 bytes of float EIB data on the set group address and converts them into an IEC61131-3 REAL variable.

##### `EIB_2OCTET_SIGN_REC` — §4.2.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 2 bytes of sign EIB data on the set group address and converts them into an IEC61131-3 INT variable.

##### `EIB_2OCTET_UNSIGN_REC` — §4.2.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 2 bytes of unsign EIB data on the set group address and converts them into an IEC61131-3 UINT variable.

##### `EIB_3BIT_CONTROL_REC` — §4.2.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 4 bits of EIB data on the set group address and converts them into an IEC61131-3 BOOL variable and a BYTE variable.

##### `EIB_4OCTET_FLOAT_REC` — §4.2.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 4 bytes of float EIB data on the set group address and converts them into an IEC61131-3 REAL variable.

##### `EIB_4OCTET_SIGN_REC` — §4.2.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 4 bytes of sign EIB data on the set group address and converts them into an IEC61131-3 DINT variable.

##### `EIB_4OCTET_UNSIGN_REC` — §4.2.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 4 bytes of unsign EIB data on the set group address and converts them into an IEC61131-3 UDINT variable.

##### `EIB_8BIT_SIGN_REC` — §4.2.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 8 bits of EIB data on the set group address and converts them into an IEC61131-3 INT variable. In addition the value may be converted automatically.

##### `EIB_8BIT_UNSIGN_REC` — §4.2.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 8 bits of EIB data on the set group address and converts them into an IEC61131-3 BYTE variable.

##### `EIB_ALL_DATA_TYPES_REC` — §4.2.4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives variable EIB data sizes on the set group address and outputs the raw data as a byte ARRAY.

##### `EIB_ALL_DATA_TYPES_REC_EX` — §4.2.4.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `strData_Rec` | `EIB_REC [} 76]` | The data structure with which the function block KL6301() |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives variable EIB data for all group addresses and outputs the raw data as a byte array.

##### `EIB_BIT_CONTROL_REC` — §4.2.4.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 2 bits of EIB data on the set group address and converts them into two IEC61131-3 BOOL variables.

##### `EIB_BIT_REC` — §4.2.4.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 1 bit of EIB data on the set group address and converts them into an IEC61131-3 BOOL variable.

##### `EIB_DATE_REC` — §4.2.4.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 3 bytes of EIB data on the set group address and converts them into three IEC61131-3 WORD variables.

##### `EIB_TIME_REC` — §4.2.4.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDataReceive` | `BOOL` | This bit is set to FALSE for exactly one cycle when an EIB |

**Description:** This function block receives 3 bytes of EIB data on the set group address and converts them into three IEC61131-3 WORD variables.

#### POUs › Send

##### `EIB_2OCTET_FLOAT_SEND` — §4.2.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address from which the data were sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** 30

##### `EIB_2OCTET_FLOAT_SEND_EX` — §4.2.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 2-byte float EIB value to the set group address. An IEC61131-3 REAL value is available as input value. The data can be sent in Manual, Polling or OnChange mode (iMode).

##### `EIB_2OCTET_SIGN_SEND` — §4.2.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 2-byte sign EIB value to the set group address. An IEC61131-3 INT value is available as input value. The data are only transferred if there is a change. If the value changes again within 1 second, new data are only sent to the EIB device after another second has passed (see diagram). No new EIB telegram is sent if the value changes within the "min. send time" but falls back to the old, already sent value within the "min. send time".

##### `EIB_2OCTET_SIGN_SEND_EX` — §4.2.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Activates the function block. The function block starts |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 2-byte sign EIB value to the set group address. An IEC61131-3 INT value is available as the input value. The data can be sent in Manual, Polling or OnChange depending on the set mode (iMode).

##### `EIB_2OCTET_UNSIGN_SEND` — §4.2.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 2-byte unsign EIB value to the set group address. An IEC61131-3 UINT value is available as input value. The data are only transferred if there is a change. If the value changes again within 1 second, new data are only sent to the EIB device after another second has passed (see diagram). No new EIB telegram is sent if the value changes within the "min. send time" but falls back to the old, already sent value within the "min. send time".

##### `EIB_2OCTET_UNSIGN_SEND_EX` — §4.2.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Activates the function block. The function block starts |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 2-byte Unsign EIB value to the set group address. An IEC61131-3 UINT value is available as input value. The data can be sent in Manual, Polling or OnChange depending on the set mode (iMode).

##### `EIB_3BIT_CONTROL_SEND` — §4.2.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 4-bit EIB value to the set group address. An IEC61131-3 BOOL and a BYTE value are available as input value. The data are only transferred if there is a change in one of the two data types. If the value changes again within 200 milliseconds, new data are only sent to the EIB device after another 200 millisecond has passed (see diagram). No new EIB telegram is sent if the value changes within the "min. send time" but falls back to the old, already sent value within the "min. send time".

##### `EIB_3BIT_CONTROL_SEND_EX` — §4.2.5.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Activates the function block. The function block starts |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 4-bit EIB value to the set group address. An IEC61131-3 BOOL and a BYTE value are available as input value. The data can be sent in Manual, Polling or OnChange depending on the set mode (iMode).

##### `EIB_4OCTET_FLOAT_SEND` — §4.2.5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** 40

##### `EIB_4OCTET_FLOAT_SEND_EX` — §4.2.5.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Activates the function block. The function block starts |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 4-byte float EIB value to the set group address. An IEC61131-3 REAL value is available as input value. The data can be sent in Manual, Polling or OnChange mode (iMode).

##### `EIB_4OCTET_SIGN_SEND` — §4.2.5.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 4-byte sign EIB value to the set group address. An IEC61131-3 DINT value is available as input value. The data are only transferred if there is a change. If the value changes again within 1 second, new data are only sent to the EIB device after another second has passed (see diagram). No new EIB telegram is sent if the value changes within the "min. send time" but falls back to the old, already sent value within the "min. send time".

##### `EIB_4OCTET_SIGN_SEND_EX` — §4.2.5.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 4-byte sign EIB value to the set group address. An IEC61131-3 DINT value is available as input value. The data can be sent in Manual, Polling or OnChange mode (iMode).

##### `EIB_4OCTET_UNSIGN_SEND` — §4.2.5.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 4-byte unsign EIB value to the set group address. An IEC61131-3 UDINT value is available as input value. The data are only transferred if there is a change. If the value changes again within 1 second, new data are only sent to the EIB device after another second has passed (see diagram). No new EIB telegram is sent if the value changes within the "min. send time" but falls back to the old, already sent value within the "min. send time".

##### `EIB_4OCTET_UNSIGN_SEND_EX` — §4.2.5.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 4-byte Unsign EIB value to the set group address. An IEC61131-3 UDINT value is available as input value. The data can be sent in Manual, Polling or OnChange depending on the set mode (iMode).

##### `EIB_8BIT_SIGN_SEND` — §4.2.5.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 8-bit EIB value to the set group address. An IEC61131-3 INT value is available as input value. Scaling_Mode can be used to scale the input data value. The data are only transferred if there is a change in the data value. If the value changes again within 1 second, new data are only sent to the EIB device after "minSendTime" has passed (see diagram). No new EIB telegram is sent if the value changes within the "min. send time" but falls back to the old, already sent value within the "min. send time".

##### `EIB_8BIT_SIGN_SEND_EX` — §4.2.5.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 8-bit EIB value to the set group address. An IEC61131-3 INT value is available as input value. Scaling_Mode can be used to scale the input data value. The data can be sent in Manual, Polling or OnChange mode (iMode).

##### `EIB_8BIT_UNSIGN_SEND` — §4.2.5.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 8-bit EIB value to the set group address. An IEC61131-3 byte value is available as input value. The data are only transferred if there is a change in the data value. If the value changes again within 1 second, new data are only sent to the EIB device after "minSendTime" has passed (see diagram). No new EIB telegram is sent if the value changes within the "min. send time" but falls back to the old, already sent value within the "min. send time".

##### `EIB_8BIT_UNSIGN_SEND_EX` — §4.2.5.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 8-bit EIB value to the set group address. An IEC61131-3 byte value is available as input value. The data can be sent in Manual, Polling or OnChange mode (iMode).

##### `EIB_ALL_DATA_TYPES_SEND` — §4.2.5.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | If the mode is set to 0, an EIB telegram with positive edge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a freely selectable EIB value to the set group address. An IEC61131-3 byte ARRAY variable is available as input value. The data are sent depending on the set mode.

##### `EIB_BIT_CONTROL_SEND` — §4.2.5.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 2-bit EIB value to the set group address. Two IEC61131-3 BOOL variables are available as input values. The data are only transferred if there is a change in one of the two data types. If the value changes again within 200 milliseconds, new data are only sent to the EIB device after another 200 millisecond has passed (see diagram). No new EIB telegram is sent if the value changes within the "min. send time" but falls back to the old, already sent value within the "min. send time".

##### `EIB_BIT_CONTROL_SEND_EX` — §4.2.5.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 2-bit EIB value to the set group address. Two IEC61131-3 BOOL variables are available as input values. The data can be sent in Manual, Polling or OnChange depending on the set mode (iMode).

##### `EIB_BIT_SEND` — §4.2.5.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 1-bit EIB value to the set group address. An IEC61131-3 BOOL variable is available as input value. The data are only transferred if there is a change in the data value. If the value changes again within 200 milliseconds, new data are only sent to the EIB device after another 200 millisecond has passed (see diagram). No new EIB telegram is sent if the value changes within the "min. send time" but falls back to the old, already sent value within the "min. send time".

##### `EIB_BIT_SEND_EX` — §4.2.5.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Activates the function block. The function block starts |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a Bool value to the set group address. The data can be sent in Manual, Polling or OnChange mode (iMode).

##### `EIB_BIT_SEND_MANUAL` — §4.2.5.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 1-bit EIB value to the set group address. An IEC61131-3 BOOL variable is available as input value. The data is sent when bSend has a positive edge. bBusy is set as long the function block is active. bBusy is set to FALSE once the EIB command was sent or if an error occurs. An error is indicated by setting the bError variable. The error code is indicated in the iErrorID.

##### `EIB_DATE_SEND` — §4.2.5.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 3-byte EIB value to the set group address. Three IEC61131-3 word variables are available as input values. The data are sent when the block is called for the first time the and then every 5 minutes.

##### `EIB_DATE_SEND_EX` — §4.2.5.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 3-byte EIB value to the set group address. Three IEC61131-3 word variables are available as input values. The data can be sent in Manual, Polling or OnChange depending on the set mode (iMode).

##### `EIB_READ_SEND` — §4.2.5.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a Read_Group_Req to the set group address. For receiving a Read_Group_Res the group address filter of the KL6301 has to be parameterized accordingly.

##### `EIB_TIME_SEND` — §4.2.5.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR [} 74]` | Group address to which the data are sent. This group |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends a 3-byte EIB value to the set group address. Three IEC61131-3 word variables are available as input values. The data are sent when the block is called for the first time the and then every 5 minutes.

##### `EIB_TIME_SEND_EX` — §4.2.5.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active. Wait for new functions, until |

**Description:** This function block sends a 3-byte EIB value to the set group address. Three IEC61131-3 word variables are available as input values. The data can be sent in Manual, Polling or OnChange depending on the set mode (iMode).

#### POUs › Functions

##### `F_CONV_2GROUP_TO_3GROUP` — §4.2.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Group_Address` | `EIB_GROUP_ADDR_2GRO 2-stage group address UP [} 75]` | — |

**Description:** Conversion of a 2-stage group address to a 3-stage group address.

##### `F_CONV_3GROUP_TO_2GROUP` — §4.2.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IN` | `EIB_GROUP_ADDR [} 74]` | 3-stage group address |

**Description:** Conversion of a 3-stage group address to a 2-stage group address.
