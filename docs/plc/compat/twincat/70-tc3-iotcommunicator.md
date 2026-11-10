# TC3_IotCommunicator

- **Manual:** `TF6730_TC3_IoT_Communicator_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 13
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** IoT communicator: broker connectivity (MQTT/AMQP) via socket service.

---

#### PLC API › Function blocks

##### `FB_IotCommunicator` (FUNCTION_BLOCK) — §6.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sHostName` | `STRING` | sHostName can be specified as the host name or as the IP address. If no information is provided, the local |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError |

**Description:** The function block enables communication with an MQTT broker.

#### PLC API › Function blocks › FB_IotCommunicator

##### `Execute` (METHOD) — §6.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConnect` | `BOOL` | The connection to the broker is established when bConnect is set to TRUE. |

**Description:** This method must be called cyclically in order to ensure the background communication with the MQTT broker.

##### `SendData` (METHOD) → `BOOL` — §6.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pMachineStruct` | `PVOID` | Address for the structure in which the device variables are declared. |
| `nStructSize` | `UINT` | Size of the structure specified in pMachineStruct. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendData` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once to send data to the broker.

##### `SendMessage` (METHOD) → `BOOL` — §6.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sMessage` | `STRING` | Text of the (push) message to be sent to the broker. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendMessage` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once to send a (push) message to the broker. This message is then displayed in the app. This is not a message that is visible as a push message on the mobile phone.

##### `SendMessageEx` (METHOD) → `BOOL` — §6.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sMessage` | `STRING` | Text of the (push) message to be sent to the broker. |
| `sType` | `STRING` | Category of the (push) message. Currently available categories are: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendMessageEx` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once to send a categorized (push) message to the broker. This message is then displayed in the app. This is not a message that is visible as a push message on the mobile phone.

##### `SendDataAsString` (METHOD) → `BOOL` — §6.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonString` | `POINTER TO STRING` | Pointer to the JSON string to be sent. |
| `nJsonLen` | `UINT` | Length of the JSON string |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendDataAsString` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once to send data to the broker. Unlike the classic SendData [} 60]() method, the JSON document is passed directly at this point. In this way, the user achieves greater flexibility, but in return must send a correctly constructed JSON document to the app.

##### `SendData_OnChange` (METHOD) → `BOOL` — §6.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pMachineStruct` | `PVOID` | Address for the structure in which the device variables are declared. |
| `nStructSize` | `UINT` | Size of the structure specified in pMachineStruct. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendData_On BOOL` | `The method returns the return value TRUE if the call was successful. Change` | — |

**Description:** This method is called once to send data to the broker.

##### `SendDataAsString_OnChange` (METHOD) → `BOOL` — §6.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonString` | `POINTER TO STRING` | Pointer to the JSON string to be sent. |
| `nJsonLen` | `UINT` | Length of the JSON string |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendDataAsString_On BOOL` | `The method returns the return value TRUE if the call was successful. Change` | — |

**Description:** This method is called once to send data to the broker. Unlike the classic SendData [} 60]() method, the JSON document is passed directly at this point. In this way, the user achieves greater flexibility, but in return must send a correctly constructed JSON document to the app.

##### `SendCustomDeviceIcon` (METHOD) → `BOOL` — §6.1.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBase64Encode POINTER TO STRING` | `Pointer to the Base64 string that contains the image. The` | — |
| `dIconString` | `image must be converted from an image format to a Base64` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendCustomDeviceIco BOOL` | `The method returns the return value TRUE if the call was successful. n` | — |

**Description:** This method allows the user to send a user-defined icon to be displayed at the top level of a Communicator function block. The TwinCAT cube is displayed here by default. The icon is transmitted as a Base64- encoded string and can also be changed at runtime.

##### `SendCustomMessageTypes` (METHOD) → `BOOL` — §6.1.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aCustomTypes` | `ARRAY[1…5] User-defined categories of (push) messages. Exactly five categories` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendCustomMessag BOOL` | `The method returns the return value TRUE if the call was successful. eTypes` | — |

**Description:** This method is called once to overwrite the standard categories (Default, Info, Warning, Error, Critical). Categories can be overwritten again by calling them up again. The user-defined categories can then also be used in the SendMessageEx [} 61] method.

#### PLC API › Function blocks

##### `FB_IotCommand` (FUNCTION) — §6.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError |

**Description:** The function block FB_IotCommand provides functions for evaluating received commands. It must not instantiated, since it is already declared at the output of the FB_IotCommunicator [} 57] instance and is used to access the outputs and received commands.

#### PLC API › Function blocks › FB_IotCommand

##### `GetValue` (METHOD) → `BOOL` — §6.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pValue` | `PVOID` | Address of the variable to which the received value is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetValue` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called to access the value of the variable in the current command.

##### `Remove` — §6.1.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Remove` | `BOOL` | This value is set to TRUE if the method is called successfully. |

**Description:** This method is called to remove the currently available command from the memory.
