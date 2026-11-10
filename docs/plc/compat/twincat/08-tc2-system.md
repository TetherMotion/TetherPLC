# Tc2_System

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_System_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T1 · **API entries:** 81
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Mixed: timers/clock/map T0-T1; ADS blocks need an AMS/ADS router service; task/memory introspection needs runtime service hooks.

---

#### Function blocks › General function blocks

##### `DRAND` — §3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Seed` | `INT` | Initial value for specification of the random number series. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Num` | `LREAL` | This output returns a pseudo-random number in the range 0.0 ... 1.0 with double |

**Description:** The function block permits generation of a (pseudo-) random number of type LREAL.

##### `FB_ReadTaskExceedCounter` — §3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A positive edge activates the function block. |
| `nTaskAdsPort` | `UINT` | ADS Port of the selected task |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `16 Function blocks` | `` | — |
| `bBusy` | `BOOL` | The function block is active and working. |
| `bError` | `BOOL` | An error has been detected in the function block. |
| `nErrorID` | `UDINT` | ADS error code. |
| `nExceedCount` | `UDINT` | Read value of the Exceed Counter. er |
| `Development environment` | `Target platform` | PLC libraries to include |

**Description:** The function block reads the Exceed Counter. The Exceed Counter is incremented by the system whenever the selected task exceeds the set task time. This means that maintaining real-time in the cycle was not possible.

##### `FB_ResetTaskExceedCounter` — §3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A positive edge activates the function block. |
| `nTaskAdsPort` | `UINT` | ADS Port of the selected task |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active and working. |
| `bError` | `BOOL` | An error has been detected in the function block. |
| `nErrorID` | `UDINT` | ADS error code. |
| `Development environment` | `Target platform` | PLC libraries to include |

**Description:** The function block can reset the Exceed Counter. The Exceed Counter is incremented whenever the selected task exceeds the set task time. This means that maintaining real-time in the cycle was not possible.

##### `FB_SetLedColor_BAPI` — §3.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the device (empty string or local network ID) (type T_AmsNetID [} 127]) |
| `eNewColor` | `E_UsrLED_Color` | New LED color (type E_UsrLED_Color [} 125]) |
| `bExecute` | `BOOL` | The command is executed with a rising edge. The input must be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE, as long as the function block is active. |
| `bError` | `BOOL` | TRUE if an error occurs during command execution. |
| `nErrID` | `UDINT` | Contains the ADS error code or the command-specific error code of |

**Description:** The function block FB_SetLedColor_BAPI can be used to switch the user LED to PCs and embedded PCs with BIOS API support. The LED color is switched via a positive edge at bExecute and the eNewColor parameter. The LED can be switched off (eNewColor = eUsrLED_Off) or set to red (eNewColor = eUsrLED_Red), blue (eNewColor = eUsrLED_Blue) or green (eNewColor = eUsrLED_Green).

##### `FB_SetLedColorEx_BAPI` — §3.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the device (empty string or local network ID) (type T_AmsNetID [} 127]) |
| `nLedID` | `USINT` | ID for the selection of the user LED: For devices with only one USR |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE, as long as the function block is active. |
| `bError` | `BOOL` | TRUE if an error occurs during command execution. |
| `nErrID` | `UDINT` | Contains the ADS error code or the command-specific error code of |

**Description:** The function block FB_SetLedColorEx_BAPI can be used to switch the user LEDs (USR, U1 or U2) on PCs and embedded PCs with BIOS API support. The LED color is switched via a rising edge at bExecute and the eNewColor parameter. The LED can be switched off (eNewColor = eUsrLED_Off) or set to red (eNewColor = eUsrLED_Red), blue (eNewColor = eUsrLED_Blue) or green (eNewColor = eUsrLED_Green).

##### `GETCURTASKINDEX` — §3.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `index` | `BYTE` | Returns the current task index of the calling task (1..4 ). |

**Description:** The function block GETCURTASKINDEX determines the task index of the task in which it is currently called.

##### `FB_CreateGUID` — §3.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `20 Function blocks` | `` | — |
| `bExecute` | `BOOL` | The function block is activated by a rising edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block generates a new GUID. It is possible to get a list of different new GUIDs with one call if an array of type GUID is specified as buffer at the input.

#### Function blocks › ADS function blocks › Control + State

##### `ADSRDSTATE` — §3.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `NETID` | `T_AmsNetId` | String containing the AMS network ID of the target device to which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | This output remains TRUE until the function block has executed a |

**Description:** The function block requests the state of an ADS device.

##### `ADSWRTCTL` — §3.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `NETID` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | This output remains TRUE until the function block has executed a command, but at |

**Description:** This function block permits the execution of an ADS control command to affect the state of an ADS device, e.g. to start, stop or reset a device.

##### `ADSRDDEVINFO` — §3.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | String containing the AMS network ID of the target device to which the ADS |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `BUSY` | `BOOL` | This output remains TRUE until the function block has executed a command, |

**Description:** The function block reads the general device information.

#### Function blocks › ADS function blocks › Indication + Response

##### `ADSREADIND` — §3.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CLEAR` | `BOOL` | With a rising edge at this input an Indication is reported as processed and the outputs |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `VALID` | `BOOL` | The output is set if an Indication was registered from the |

**Description:** The function block registers ADSREAD Requests at a PLC task as Indications and allows them to be processed. The queuing of an Indication is reported at the VALID output port by means of a rising edge. The Indication is reported as processed via a positive edge at the CLEAR input. A negative edge at the CLEAR input releases the function block for processing further Indications. After an Indication has been processed a response must be sent to the source device via the ADSREADRES [} 36] function block. The PORT and NETID parameters can be used to address the source device for this purpose. The INVOKEID parameter is used by the source device to assign the responses to the requests and is also sent back to the source device as a parameter.

##### `ADSREADINDEX` — §3.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CLEAR` | `BOOL` | With a rising edge at this input an Indication is reported as processed and the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `VALID` | `BOOL` | The output is set if an Indication was registered from the |

**Description:** The function block registers ADSREAD Requests at a PLC task as Indications and allows them to be processed. The queuing of an Indication is reported at the VALID output port by means of a rising edge. The Indication is reported as processed via a positive edge at the CLEAR input. A negative edge at the CLEAR input releases the function block for processing further Indications. After an Indication has been processed a response must be sent to the source device via the ADSREADRES [} 36] function block. The PORT and NETID parameters can be used to address the source device for this purpose. The INVOKEID parameter is used by the source device to assign the responses to the requests and is also sent back to the source device as a parameter.

##### `ADSWRITEIND` — §3.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CLEAR` | `BOOL` | With a rising edge at this input an indication is reported as processed |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `VALID` | `BOOL` | The output is set if an Indication was registered from the |

**Description:** The function block registers ADSWRITE Requests to a PLC task as Indications and allows them to be processed. The queuing of an Indication is reported at the VALID output port by means of a rising edge. The Indication is reported as processed via a positive edge at the CLEAR input. A falling edge releases the function block for the processing of further indications. After an Indication has been processed a response must be sent to the source device via the ADSWRITERES [} 37] function block. The PORT and NETID parameters can be used to address the source device for this purpose. The INVOKEID parameter is used by the source device to assign the responses to the requests and is also sent back to the source device as a parameter.

##### `ADSWRITEINDEX` — §3.2.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CLEAR` | `BOOL` | With a rising edge at this input an Indication is reported as processed and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `32 Function blocks` | `` | — |
| `VALID` | `BOOL` | The output is set if an Indication was registered from the |

**Description:** The function block registers ADSWRITE Requests to a PLC task as Indications and allows them to be processed. The queuing of an Indication is reported at the VALID output port by means of a rising edge. The Indication is reported as processed via a positive edge at the CLEAR input. A falling edge releases the function block for the processing of further indications. After an Indication has been processed a response must be sent to the source device via the ADSWRITERES [} 37] function block. The PORT and NETID

##### `ADSRDWRTIND` — §3.2.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CLEAR` | `BOOL` | With a rising edge at this input an indication is reported as processed and the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `VALID` | `BOOL` | The output is set if an Indication was registered from the function |

**Description:** The function block registers ADSRDWRT Requests to a PLC task as Indications and allows them to be processed. The queuing of an Indication is reported at the VALID output port by means of a rising edge. The Indication is reported as processed via a positive edge at the CLEAR input. A falling edge releases the function block for the processing of further indications. After an Indication has been processed a response must be sent to the source device via the ADSRDWRTRES [} 38] function block. The PORT and NETID parameters can be used to address the source device for this purpose. The INVOKEID parameter is used by the source device to assign the responses to the requests and is also sent back to the source device as a parameter.

##### `ADSRDWRTINDEX` — §3.2.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CLEAR` | `BOOL` | With a rising edge at this input an indication is reported as processed and the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `VALID` | `BOOL` | The output is set if an Indication was registered from the function |

**Description:** The function block registers ADSRDWRT Requests to a PLC task as Indications and allows them to be processed. The queuing of an Indication is reported at the VALID output port by means of a rising edge. The Indication is reported as processed via a positive edge at the CLEAR input. A falling edge releases the function block for the processing of further indications. After an Indication has been processed a response must be sent to the source device via the ADSRDWRTRES [} 38] function block. The PORT and NETID parameters can be used to address the source device for this purpose. The INVOKEID parameter is used by the source device to assign the responses to the requests and is also sent back to the source device as a parameter.

##### `ADSREADRES` — §3.2.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | String containing the AMS network ID of the source device, to which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**Description:** The ADSREADRES function block acknowledges Indications of a PLC task. A response is sent to the ADS source device via a positive edge on the RESPOND input. The source device is addressed via the PORT and NETID parameters. The INVOKEID parameter is used by the source device to assign the responses to the requests and is adopted by the output of the ADSREADIND [} 28] function block. An error code can be returned to the ADS source device via the RESULT parameter.

##### `ADSWRITERES` — §3.2.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | String containing the AMS network ID of the source device, to which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**Description:** The ADSWRITERES function block is used to acknowledge indications of a PLC task. A response is sent to the ADS source device via a rising edge on the RESPOND input. The source device is addressed via the PORT and NETID parameters. The INVOKEID parameter sorts the responses to the requests for the source device and is adopted by the output of the ADSWRITEIND [} 28] function block. An error code can be returned to the ADS source device via the RESULT parameter.

##### `ADSRDWRTRES` — §3.2.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | String containing the AMS network ID of the source device, to which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**Description:** The ADSRDWRTRES function block is used to acknowledge indications of a PLC task. A response is sent to the ADS source device via a rising edge on the RESPOND input. The source device is addressed via the PORT and NETID parameters. The INVOKEID parameter sorts the responses to the requests for the source device and is adopted by the output of the ADSRDWRTIND [} 33] function block. An error code can be returned to the ADS source device via the RESULT parameter.

#### Function blocks › ADS function blocks

##### `ADSREAD` — §3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_Ams` | String containing the AMS network ID of the target device to which the ADS |
| `NetId` | `command is addressed (type: T_AmsNetId [} 127]).` | — |
| `PORT` | `T_Ams` | Port number of the ADS device (type: T_AmsPort [} 127]) Port |
| `IDXGRP` | `UDINT` | Index group number (32-bit, unsigned) of the requested ADS service. This value is to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `BUSY` | `BOOL` | This output remains TRUE until the function block has executed a command, but at |

**Description:** The function block executes an ADS read command to request data from an ADS device.

##### `ADSREADEX` — §3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `40 Function blocks` | `` | — |
| `NETID` | `T_AmsN String containing the AMS network ID of the target device to which the ADS` | — |
| `etId` | `command is addressed (type: T_AmsNetId [} 127]).` | — |
| `PORT` | `T_AmsPo Port number of the ADS device (type: T_AmsPort [} 127]) rt` | — |
| `IDXGRP UDINT` | `Index group number (32-bit, unsigned) of the requested ADS service. This value is to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | This output remains TRUE until the function block has executed a command, but at |

**Description:** The function block executes an ADS read command to request data from an ADS device. The function block has the same functionality as the ADSREAD function block; in addition it returns the number of actually read data bytes as parameter.

##### `ADSWRITE` — §3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsN String containing the AMS network ID of the target device to which the ADS` | — |
| `etId` | `command is addressed (type: T_AmsNetId [} 127]).` | — |
| `PORT` | `T_AmsPo Port number of the ADS device (type: T_AmsPort [} 127]) rt` | — |
| `IDXGRP UDINT` | `Index group number (32-bit, unsigned) of the requested ADS service. This value is to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | This output remains TRUE until the function block has executed a command, but at |

**Description:** Function blocks

##### `ADSRDWRT` — §3.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `NETID` | `T_AmsN String containing the AMS network ID of the target device to which the ADS` | — |
| `etId` | `command is addressed (type: T_AmsNetId [} 127]).` | — |
| `PORT` | `T_AmsPo Port number of the ADS device (type: T_AmsPort [} 127]) rt` | — |
| `IDXGRP UDINT` | `Index group number (32-bit, unsigned) of the requested ADS service. This value is to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | This output remains TRUE until the function block has executed a command, but at |

**Description:** The function block executes a combined ADS read-write command. Data is transmitted to an ADS device (write) and its response data read with one call.

##### `ADSRDWRTEX` — §3.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `NETID` | `T_AmsN String containing the AMS network ID of the target device to which the ADS` | — |
| `etId` | `command is addressed (type: T_AmsNetId [} 127]).` | — |
| `PORT` | `T_AmsPo Port number of the ADS device (type: T_AmsPort [} 127]) rt` | — |
| `IDXGRP UDINT` | `Index group number (32-bit, unsigned) of the requested ADS service. This value is to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | This output remains TRUE until the function block has executed a command, but at |

**Description:** This block allows execution of a combined ADS write/read instruction. Data is transmitted to an ADS device (write) and its response data read with one call. Contrary to the ADSRDWRT function block ADSRDWRTEX supplies the number of actually read data bytes as parameter.

#### Function blocks › File function blocks

##### `FB_EOF` — §3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_EOF can check whether the end of the file was reached.

##### `FB_FileOpen` — §3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `48 Function blocks` | `` | — |
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_FileOpen creates a new file or opens an existing file for editing.

##### `FB_FileClose` — §3.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_FileClose closes the file, thereby putting it in a defined state for further processing by other programs.

##### `FB_FileLoad` — §3.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The contents of a file can be read out with the function block FB_FileLoad. The file is opened implicitly in binary mode, the contents are read out and the file is then closed again.

##### `FB_FileGets` — §3.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_FileGest reads strings from a file. The string is read up to and including the line feed character, or up to the end of the file or the maximum permitted length of sLine. The null termination is appended automatically. The file must have been opened in text mode.

##### `FB_FilePuts` — §3.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** Function blocks

##### `FB_FileRead` — §3.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** With the function block FB_FileRead the contents of an already opened file can be read. Before a read access, the file must have been opened in the corresponding mode. In addition to the FOPEN_MODEREAD, the appropriate format (FOPEN_MODEBINARY or FOPEN_MODETEXT) is also important to achieve the desired result.

##### `FB_FileWrite` — §3.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_FileWrite writes data into a file. For write access the file must have been opened in the corresponding mode, and it must be closed again for further processing by external programs.

##### `FB_FileSeek` — §3.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_FileSeek sets the file pointer of an open file to a definable position.

##### `FB_FileTell` — §3.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `62 Function blocks` | `` | — |
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_FileTell determines the current position of the file pointer. The position indicates the relative offset from the start of the file.

##### `FB_FileDelete` — §3.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_FileDelete deletes a file from the data storage device.

##### `FB_FileRename` — §3.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_FileRename can be used to rename a file.

##### `FB_CreateDir` — §3.3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_CreateDir can be used to create new directories on the data storage device.

##### `FB_RemoveDir` — §3.3.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `68 Function blocks` | `` | — |
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the target device |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is activated, this output is set to |

**Description:** The function block FB_RemoveDir can be used to delete a directory from the data storage device. A directory containing files cannot be deleted.

#### Function blocks › EventLogger function blocks

##### `ADSLOGEVENT` — §3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | String containing the AMS network ID of the target device to which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `70 Function blocks` | `` | — |
| `EventState` | `UDINT` | State of the event. |
| `Err` | `BOOL` | This output is switched to TRUE as soon as an error occurs during the |

**Description:** This function block allows the sending and acknowledgment of messages to the TwinCAT EventLogger.

##### `ADSCLEAREVENTS` — §3.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | String containing the AMS network ID of the target device to which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `72 Function blocks` | `` | — |
| `bBusy` | `BOOL` | When the function block is activated, this output is set to TRUE and |

**Description:** The function block clears messages from the TwinCAT EventLogger.

##### `FB_SimpleAdsLogEvent` — §3.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SourceId` | `INT` | ID of the source. Used to clearly identify the source in the EventLogger. |
| `EventId` | `INT` | ID of the event. Used to clearly identify the event in the EventLogger. |
| `bSetEvent` | `BOOL` | The "coming" of the event is signaled with the rising edge, the "going" of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ErrId` | `UDINT` | ADS error code [} 148] or command-specific error code of the last executed |

**Description:** This function block allows the sending and acknowledgment of messages to the TwinCAT EventLogger. As opposed to the ADSLOGEVENT block, events cannot be parameterized from the PLC with the FB_SimpleAdsLogEvent block; however, events can be set, reset and acknowledged in a simple manner.

#### Function blocks › Watchdog function blocks

##### `FB_PcWatchdog` — §3.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeOut` | `TIME` | Watchdog time, after which a restart is performed. |
| `bEnable` | `BOOL` | Enabling/disabling the watchdog. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnabled` | `BOOL` | TRUE = watchdog activated, FALSE = watchdog deactivated |
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block FB_PcWatchdog activates a hardware watchdog on the PC. The watchdog is activated via bEnable = TRUE and the timeout time. The timeout time can range between 1 and 255 seconds. The watchdog is activated via bEnable = TRUE and tTimeOut >= 1 s.

##### `FB_PcWatchDog_BAPI` — §3.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the device (empty string or local network ID) |
| `nWatchdogTimeS UDINT` | `Watchdog time in seconds, 0 = deactivated, >0 (max.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bEnabled` | `BOOL` | TRUE = watchdog activated, FALSE = watchdog deactivated |
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block FB_PcWatchdog_BAPI activates a hardware watchdog on the PC. The watchdog is activated via bExecute = TRUE and the watchdog time. The watchdog time can range between 1 and 15300 seconds (255 minutes). The watchdog is activated via bEnable = TRUE and nWatchdogTimeS >=1 s.

#### Function blocks › Time function blocks

##### `GETCPUACCOUNT` — §3.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `cpuAccountDW` | `UDINT` | Current value of the PLC task ticker |

**Description:** 84

##### `GETCPUCOUNTER` — §3.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `cpuCntLoDW` | `UDINT` | Least significant 4 bytes of the count value |
| `cpuCntHiDW` | `UDINT` | Most significant 4 bytes of the count value Function blocks |

**Description:** The CPU cycle counter can be read with this function block. The numerical value is a relative 64 bit integer, which, independently of the CPUs internal clock rate, is output in a form converted into 100 ns ticks. The number is refreshed to a precision of 100 ns with every call by the PLC system, and can be used, for instance, for timing tasks. One unit is equivalent to 100 ns. The reason for which this service is implemented as a block and not as a function is simply in the fact that two values must be returned, which, by definition, cannot be done by a function.

#### Functions › General functions

##### `F_CheckMemoryArea` (FUNCTION) → `E_TcMemoryArea` — §4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pData` | `PVOID` | Memory address of the variable |
| `nSize` | `UDINT` | Size of the variable in bytes |

**Description:** The function returns information about the memory area in which the requested variable with the specified size is located. A return value of type E_TcMemoryArea [} 125] is used for this purpose.

##### `F_CmpLibVersion` (FUNCTION) → `DINT` — §4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stVersion` | `ST_LibVersion` | Version of the existing library (type: ST_LibVersion) |
| `iMajor` | `UINT` | Required major number |
| `iMinor` | `UINT` | Required minor number |
| `iBuild` | `UINT` | Required build number |
| `iRevision` | `UINT` | Required revision number |
| `Return parameter` | `Version relationship` | — |
| `-1` | `Your version is lower than the required` | — |

**Description:** The function F_CmpLibVersion compares the version of the existing library with the required version. Each library has its own version information as a constant of type: ST_LibVersion. The name of the constant has the format: stLibVersion_libraryname.

##### `F_CreateIPv4Addr` (FUNCTION) → `T_IPv4Addr` — §4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIds` | `T_IPv4AddrArr [} 129]` | Byte array: Each byte corresponds to one address byte of |

**Description:** The function generates a formatted (IPv4) Internet protocol network address and returns it as a return parameter of type string (e.g. 172.16.7.199).

##### `F_CreateMacAddr` (FUNCTION) → `T_MacAddr` — §4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aMacAddress` | `T_MacAddrArr` | Byte array: Each byte corresponds to one address byte of |

**Description:** The function generates a formatted MAC address and returns it as a string-type return parameter (e.g. 1C-2C-3C-4C-5C-6C).

##### `F_ScanIPv4AddrIds` (FUNCTION) → `T_IPv4AddrArr` — §4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPv4` | `T_IPv4AddrArr` | Internet Protocol network address as a string (type: |

**Description:** The function F_ScanIPv4AddrIds converts a string with the (IPv4) Internet Protocol network address into single address bytes. The single address bytes are converted from left to right. They are returned as an array of bytes. The address bytes are represented in network byte order.

##### `F_GetCpuCoreIndex` (FUNCTION) → `DINT` — §4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTaskIndex` | `DINT` | Index of the task whose associated CPU index is to be |

**Description:** For a task index the function F_GetCpuCoreIndex returns the index of the CPU core on which the task runs.

##### `F_GetCpuCoreInfo` (FUNCTION) → `HRESULT` — §4.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCpuCoreIndex` | `DINT` | Index of the CPU core whose information is to be read. |
| `pInfo` | `POINTER TO ST_CpuCoreInf Address of the variable that is to receive the read data.` | — |
| `o` | `The address must point to an instance of type` | — |

**Description:** The function F_GetCpuCoreInfo returns information about the CPU core whose index is passed to the function. The information that is read out includes the base time and the core limit for the specified CPU core.

##### `F_GetMappingPartner` (FUNCTION) → `OTCID` — §4.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `P` | `PVOID` | Memory address of the variable |
| `n` | `UDINT` | Size of the variable in bytes |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** The function F_GetMappingPartner returns the object ID (data type: OTCID) of the partner side of the mapping.

##### `F_GetMappingStatus` (FUNCTION) → `EPlcMappingStatus` — §4.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `P` | `PVOID` | Memory address of the variable |
| `n` | `UDINT` | Size of the variable in bytes |
| `Development Environment` | `Target platform` | PLC libraries to include |

**Description:** The function F_GetMappingStatus returns the current mapping status of a PLC variable. The function returns an ENUM value (data type: EPlcMappingStatus [} 125]) with the values MS_Unmapped, MS_Mapped or MS_Partial.

##### `F_GetStructMemberAlignment` (FUNCTION) → `BYTE` — §4.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(* keine Eingangsparameter *)` | `` | — |
| `Return parameter` | `Description` | — |

**Description:** 92

##### `F_GetTaskInfo` (FUNCTION) → `PlcTaskSystemInfo` — §4.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*keine*) Return parameter` | `Description` | — |
| `PlcTaskSystemInfo` | `Task System Info of the task in which the function is called.` | — |

**Description:** The function determines the Task System Info of the task in which it is called.

##### `F_RaiseException` (FUNCTION) — §4.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ExceptionCode` | `UDINT` | Specification of the ExceptionCode. See __SYSTEM.ExceptionCode |

**Description:** Calling the function makes it possible to throw a runtime exception with a specific exception code.

##### `F_SplitPathName` (FUNCTION) → `BOOL` — §4.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sPathName` | `T_MaxString` | Complete file name as string (type: T_MaxString [} 129]) |
| `inVal32` | `DWORD` | 32-bit value to be changed. |
| `bitNo` | `SINT` | Number of the bit that is to be set (0-31). This number is |
| `inVal32` | `DWORD` | 32-bit value |
| `bitNo` | `SINT` | Number of the bit to be set or reset (0-31). This number is |
| `inVal32` | `DWORD` | 32-bit value |
| `bitNo` | `SINT` | Number of the bit to be read (0-31). This number is |
| `inVal32` | `DWORD` | 32-bit value to be changed. |
| `bitNo` | `SINT` | Number of the bit that is to be set (0-31). This number is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sDrive` | `STRING(3)` | — |
| `sDir` | `T_MaxString` | — |
| `sFileName` | `T_MaxString` | — |
| `sExt` | `T_MaxString` | — |

**Description:** This function splits a complete path name into its four components. These are stored in the strings named sDrive, sDir, sFileName and sExt.

##### `GETCURTASKINDEXEX` (FUNCTION) → `DINT` — §4.1.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*keine*) Return parameter` | `Description` | — |
| `-1` | `The function is called from the Windows context.` | — |

**Description:** The function determines the task index of the task in which it is called. In contrast to the function block GETCURTASKINDEX [} 20], a distinction can be made as to whether the function is called in a cyclic real-time context or not.

##### `LPTSIGNAL` (FUNCTION) → `BOOL` — §4.1.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `102 Functions` | `` | — |
| `PortAddr` | `UINT` | Address of the port which is available for the desired LPT |

**Description:** This function sets a defined output bit in a Centronics interface to a logical high or low level, and can, for example, be used for run-time measurements with an oscilloscope.

##### `TestAndSet` (FUNCTION_BLOCK) — §4.1.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Flag` | `BOOL` | Boolean flag to be checked |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Flag` | `BOOL; (* Flag to check if TRUE or FALSE *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLocked` | `BOOL` | — |

**Description:** You can use this function to check and set a flag without being interrupted by another task. The function performs an atomic operation. This allows data accesses to be synchronized. The mode of operation of a semaphore can be achieved with TestAndSet.

#### Functions › ADS functions

##### `ADSLOGDINT` (FUNCTION) → `DINT` — §4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `msgCtrlMask` | `DWORD` | Control mask which determines the type and effect of the message |

**Description:** When called, the function outputs a message box with a predefinable text to the screen and writes an entry to the system's event log. As PLC programs are processed cyclically, it is necessary for items such as message boxes to be output edge-triggered. This is most easily achieved with an R_TRIG or F_TRIG function block placed in series (see also samples below).

##### `ADSLOGLREAL` (FUNCTION) → `DINT` — §4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `msgCtrlMask` | `DWORD` | Control mask which determines the type and effect of the |

**Description:** When called, the function outputs a message box with a predefinable text to the screen and writes an entry to the system's event log. An LREAL value (floating-point number) can be incorporated into the text to be output at a position specified by the user. For this purpose, the created format string must contain the string '%f' at the desired position. Please keep in mind that the function must be called edge-triggered (see also note in description ADSLOGDINT [} 104]). The return parameter contains the function error code, or 0 if successful.

##### `ADSLOGSTR` (FUNCTION) → `DINT` — §4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `msgCtrlMask` | `DWORD` | Control mask which determines the type and effect of the |

**Description:** Functions

##### `F_CreateAmsNetId` (FUNCTION) → `T_AmsNetId` — §4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIds` | `T_AmsNetIdArr` | Byte array (type: T_AmsNetIdArr [} 127]). Each byte |

**Description:** The function generates a formatted NetID string (type: T_AmsNetID [} 127]) and returns it as return parameter (e.g. '127.16.17.3.1.1').

##### `F_ScanAmsNetIds` (FUNCTION) → `T_AmsNetIdArr` — §4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | TwinCAT network address as string (type: T_AmsNetId [} 127]). E.g. '127.16.17.3.1.1' |
| `Input parameters` | `Return parameter` | Description |
| `sNetID ≠ '' (empty string)` | `All bytes are null` | Error during the conversion, check |
| `and sNetID ≠ '0.0.0.0.0.0'` | `the formatting of the sNetID string.` | — |

**Description:** The function F_ScanAmsNetIds can be used to convert a string containing the TwinCAT network address to individual address bytes. The individual address bytes are converted from left to right and returned as an array of bytes (type: T_AmsNetIdArr [} 127]). The address bytes have a network byte order.

#### Functions › Character functions

##### `F_ToCHR` (FUNCTION) → `STRING` — §4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `c` | `BYTE` | ASCII code to be converted |

**Description:** The function converts ASCII Code to STRING.

##### `F_ToASC` (FUNCTION) → `BYTE` — §4.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `str` | `STRING` | String to convert |

**Description:** This function converts STRING to ASCII Code. Only the first sign of the STRING will be converted. An empty STRING delivers a zero.

#### Functions › I/O port access

##### `F_IOPortRead` (FUNCTION) → `DWORD` — §4.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nAddr` | `UDINT` | I/O port address |
| `eSize` | `E_IOAccessSize [} 123])` | Number of data bytes to be read (type: E_IOAccessSize |

**Description:** A digital I/O port is usually an I/O position with a width of 1 byte, which is mapped either in the memory or as a port. If a value is written at this point, the electrical signal at the output pins is modified according to the written bits. If a value is read from the input position, the current logical level at the input pins is returned as an individual bit value.

##### `F_IOPortWrite` (FUNCTION) → `BOOL` — §4.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nAddr` | `UDINT` | I/O port address |
| `eSize` | `E_IOAccessSize` | Number of data bytes to be written (type: E_IOAccessSize |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** The function F_IOPortWrite can be used to write to an I/O position with a width of eSize. See also description of the F_IOPortRead [} 112] function.

#### Functions › Memory functions

##### `MEMCMP` — §4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBuf1` | `PVOID` | Start address of the first memory area (the first data |

**Description:** The function MEMCMP allows the values of PLC variables in two different memory areas to be compared.

##### `MEMCPY` (FUNCTION) → `UDINT` — §4.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `destAddr` | `PVOID` | Start address of the destination memory area. |
| `srcAddr` | `PVOID` | Start address of the source memory area. |
| `n` | `UDINT` | Number of bytes to be copied. |

**Description:** Functions

##### `MEMMOVE` (FUNCTION) → `UDINT` — §4.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `destAddr` | `PVOID` | Start address of the destination memory area. |
| `srcAddr` | `PVOID` | Start address of the source memory area. |
| `n` | `UDINT` | Number of bytes to be copied. |

**Description:** Use the MEMMOVE function to copy values from one memory area to another; the memory areas may overlap.

##### `MEMSET` — §4.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `destAddr` | `PVOID` | Start address of the memory area that is to be set. |
| `fillByte` | `USINT` | Value of the filler byte. |
| `n` | `UDINT` | Number of bytes to be set. |

**Description:** The function MEMSET allows PLC variables in a particular memory area to be set to a particular value.

#### Functions › Time functions

##### `F_GetSystemTime` — §4.6.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GetSystemTime` | `ULINT` | — |

**Description:** This function can be used to read the operating system timestamp. The timestamp is a 64-bit integer value, with an accuracy of 100 ns, which is updated with every call of the PLC. Among other things, it can be used for timing tasks or time measurements. One unit corresponds to 100 ns. The time represents the number of 100 ns intervals since January 1, 1601 (UTC).

##### `F_GetTaskTime` — §4.6.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GetTaskTime` | `ULINT` | — |

**Description:** This function can be used to read the start time of the task (time at which the task should start). The function always returns the start time of the task in which the function was called. The time stamp is a 64-bit integer value with an accuracy of 100 ns. It can be used for timing tasks or time measurements, among other things. One unit corresponds to 100 ns. The time represents the number of 100 ns intervals since 1 January 1601.

##### `F_GetTaskTotalTime` (FUNCTION) → `UDINT` — §4.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTaskIndex` | `DINT` | Index of the task whose total execution time is to be |

**Description:** For a task index the function F_GetTaskTotalTime returns the total execution time of this task from the last cycle. The total execution time is the sum of the computing times of all modules that are registered on to the task.

#### Functions › [Obsolete]

##### `F_GetVersionTcSystem` (FUNCTION) → `UINT` — §4.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `120 Functions` | `` | — |
| `nVersionElement INT` | `nVersionElement : Version element to be read. Possible` | — |

**Description:** This function can be used to read PLC library version information.

##### `GETSYSTEMTIME` — §4.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `timeLoDW` | `UDINT` | Contains the low-value 4 bytes of the time stamp. |
| `timeHiDW` | `UDINT` | Contains the high-value 4 bytes of the time stamp. |

**Description:** With this function block the operating system time stamp can be read. The time stamp is a 64 bit integer value, with a precision of 100ns, which is updated with every call of the PLC. Amongst other uses, it can be utilized for timing tasks or time measurements. One unit corresponds to 100 ns. The time represents the number of 100 ns intervals since 1 January 1601.

##### `GETTASKTIME` — §4.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `timeLoDW` | `UDINT` | Contains the low-value 4 bytes of the time stamp. |
| `timeHiDW` | `UDINT` | Contains the high-value 4 bytes of the time stamp. |

**Description:** This function block can be used to read the start time of the task (time at which the task should start). The function block always returns the start time of the task in which the function block instance was called. The time stamp is a 64-bit integer value with an accuracy of 100 ns. It can be used for timing tasks or time measurements, among other things. One unit corresponds to 100 ns. The time represents the number of 100 ns intervals since 1 January 1601.

#### Samples

##### `File access from the PLC` (FUNCTION_BLOCK) — §7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSrcNetId` | `T_AmsNetId` | — |
| `sSrcPathName` | `T_MaxString` | — |
| `sDestNetId` | `T_AmsNetId` | — |
| `sDestPathName` | `T_MaxString` | — |
| `bExecute` | `BOOL` | — |
| `tTimeOut` | `TIME := DEFAULT_ADS_TIMEOUT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrId` | `UDINT` | — |

**Description:** The use of the PLC function blocks for data access from the Tc2_system library is introduced in this example. A new function block, FB_FileCopy, is implemented with the aid of the existing function blocks. Using the FB_FileCopy function block, binary files can be copied in the local TwinCAT system or between a local and a remote TwinCAT system.
