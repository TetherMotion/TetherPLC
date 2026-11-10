# Tc3_IoLink

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_IOLink_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T2 · **API entries:** 55
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Fieldbus I/O linking needs a master-side IO service per fieldbus.

---

#### (top level)

##### `F_IolGetAdsErrorTxt` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nAdsErr` | `UDINT` | ADS Return Codes |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_IolGetAdsErrorTxt` | `STRING` | Displays the ADS return code as text string. Sample |
| `hr` | `:HRESULT;` | — |
| `nADSError` | `:E_AdsErr := E_AdsErr.NOERR;` | — |
| `sADSErrorText` | `:STRING;` | — |

**Description:** Function converts an ADS error into text form.

##### `F_IolGetChannelStateTxt` — §4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIolChState` | `USINT` | IO-Link Channel State |
| `nPort` | `E_IolPort [} 12]` | IO-Link port |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_IolGetChannelStateTxt String(255)` | `Converts the channel state into text information.` | — |

**Description:** Function converts state channel and port into text form.

##### `F_IolGetErrorTxt` — §4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_IolGetErrorTxt` | `String(255)` | Shows the IO-Link error as a string |

**Description:** Returns the IO-Link error as string format in text form.

##### `F_IolGetIolError` — §4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_IolGetIolError` | `E_IolError [} 14]` | IO-Link error code as enum Sample |
| `hr` | `:HRESULT;` | — |
| `nIOLError` | `:E_IolErr := E_IolErr.NOERR;` | — |

**Description:** Function returns an IO-Link error code as E_IolError [} 14].

##### `F_IolGetIolErrorTxt` — §4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIolErr` | `UDINT` | IO-Link error |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_IolGetIolErrorTxt` | `String` | IO-Link error code as text Sample |
| `hr` | `:HRESULT;` | — |
| `nIolError` | `:E_IolErr := E_IolErr.NOERR;` | — |

**Description:** The function returns the IO-Link error code as a string

##### `F_IolGetPortError` — §4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIolChState` | `USINT` | IO-Link Channel State |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `E_IolGetPortError` | `E_IolPortError [} 12]` | Display IO-Link port error |

**Description:** Function returns the state channel as E_IolPortError.

##### `F_IolGetPortState` — §4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIolChState` | `USINT` | IO-Link channel state |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `E_IoLGetPortState` | `E_IolPortState [} 13]` | Displays the port state as an enum Sample nStateCh2 AT%I* :USINT; |
| `nPortState` | `:E_IolPortState;` | — |

**Description:** 22

##### `F_IolGetProfileIdTxt` — §4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `profileId` | `E_IolProfileIdentifier [} 10] IO-Link profiles` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_IoLGetProfileIdTxt` | `String(255)` | IO-Link profiles as text form |

**Description:** Converts the E_IolProfileIdentifier type to a text format.

##### `F_IolIsBusy` — §4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_IolIsBusy` | `BOOL` | True if function block is still being processed Sample |
| `hr` | `:HRESULT;` | — |

**Description:** Function indicates whether a function block is still being processed.

#### Function blocks

##### `FB_IolRead` (FUNCTION_BLOCK) — §5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | Rising edge to start the function block |
| `sNetid` | `T-AmsNetid` | AMS NETID of the IO-Link master |
| `nlolPort` | `E_lolPort [} 12]` | IO-Link port of the IO-Link device |
| `nIndex` | `WORD` | Index of the IO-Link device parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link device parameter |
| `pDSTBuf` | `PVOID` | Pointer to target buffer |
| `cbBufLen` | `UDINT` | Size of the data bytes to be read |
| `tTimeout` | `TIME` | Maximum allowed execution time |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Busy flag of the function block |
| `bDone` | `BOOL` | Done flag of the function block |
| `bError` | `BOOL` | Error flag of the function block |
| `hResult` | `HRESULT` | Return value of this function block |
| `nADSError` | `E_AdsErr` | ADS error code |
| `nIolError` | `E_IolError [} 14]` | IO-Link error code Sample |

**Description:** Function block for reading IO-Link parameters in a non-object-oriented solution.

##### `FB_IolWrite` (FUNCTION_BLOCK) — §5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | Rising edge to start the function block |
| `sNetid` | `T-AmsNetid` | AMS NETID of the IO-Link master |
| `nlolPort` | `E_lolPort [} 12]` | IO-Link port of the IO-Link device |
| `nIndex` | `WORD` | Index of the IO-Link device parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link device parameter |
| `pSCRBuf` | `PVOID` | Pointer to source buffer |
| `cbBufLen` | `UDINT` | Size of the data bytes to be read |
| `tTimeout` | `TIME` | Maximum allowed execution time |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Busy flag of the function block |
| `bDone` | `BOOL` | Done flag of the function block |
| `bError` | `BOOL` | Error flag of the function block |
| `hResult` | `HRESULT` | Return value of this function block |
| `nADSError` | `E_AdsErr` | ADS error code |
| `nIolError` | `E_IolError [} 14]` | IO-Link error code Sample |

**Description:** Function block for writing IO-Link parameters in a non-object-oriented solution.

##### `IOL_IdentificationAndDiagnosis` (FUNCTION_BLOCK) — §5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A trigger causes the function selected with the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | The signal is set when the FB has completed a |

**Description:** Function block for identification and diagnosis of an IO-Link device. This function block is implemented by the Common IO-Link Profile. The ApplicationSpecificTag, the FunctionTag and the LocationTag of an IO- Link device can be written. The function classes and ProfileIDs of a device can be read. Additionally the function block gives a diagnosis on the state of a device and its identification, which can include the Vendor Name, Vendor ID, Device Name, and Device ID.

##### `IOL_MeasurementDataChannel` (FUNCTION_BLOCK) — §5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Activates the use of the function block |
| `ProfileID` | `WORD` | Selected ProfileID, or process data layout |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Valid` | `BOOL` | If true, the supplied values are valid and can be used |

**Description:** The measuring data channel defines the process data structure, functions, and presentation of measuring sensors. The function block is defined to provide derived status signals and enable a standardized interface for user programs.

#### Object-oriented function blocks › FB_IOlSlave › Methods

##### `READ` — §6.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |
| `nIndex` | `WORD` | Index of the IO-Link parameter |
| `nIndex` | `WORD` | Index of the IO-Link parameter |
| `nIndex` | `WORD` | Index of the IO-Link parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link parameter |
| `nIndex` | `WORD` | Index of the IO-Link parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link parameter |
| `nIndex` | `WORD` | Index of the IO-Link parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link parameter |
| `nIndex` | `WORD` | Index of the IO-Link parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link parameter |
| `nIndex` | `WORD` | Index of the IO-Link parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `READ` | `HRESULT` | Return value |
| `Read_Uint16` | `HRESULT` | Return value |
| `Read_Uint32` | `HRESULT` | Return value |
| `Read_UINT64` | `HRESULT` | Return value |
| `Read_INT16` | `HRESULT` | Return value |
| `Read_INT32` | `HRESULT` | Return value |
| `Read_INT64` | `HRESULT` | Return value |
| `Read_INT8` | `HRESULT` | Return value |

**Description:** Reads out a parameter of the IO-Link device.

##### `READ_BOOL` — §6.2.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read_BOOL` | `HRESULT` | Return value |

**Description:** Reads a BOOL parameter of the IO-Link device.

##### `READ_OctedString` — §6.2.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read_OctedString` | `HRESULT 40` | Return value |

**Description:** Reads an OctetString parameter of the IO-Link device.

##### `READ_Real` — §6.2.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read_REAL` | `HRESULT` | Return value |

**Description:** Reads a REAL parameter of the IO-Link device and switches the bytes to high endian.

##### `READ_String` — §6.2.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read_STRING` | `HRESULT` | Return value |

**Description:** Reads a STRING parameter of the IO-Link device and switches the bytes to high endian.

##### `READ_String_FixedLen` — §6.2.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read_String_FixedLen` | `HRESULT` | Return value |
| `Read_UINT8` | `HRESULT` | Return value |

**Description:** Reads a STRING parameter with a limited length of the IO-Link device and switches the bytes to high endian.

##### `Write` — §6.2.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Write` | `HRESULT` | Return value |

**Description:** Writes a parameter of an IO-Link device.

##### `Write_BOOL` — §6.2.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |
| `Write_INT16` | `HRESULT` | Return value |
| `Write_INT32` | `HRESULT` | Return value |
| `Write_INT64` | `HRESULT` | Return value |
| `Write_INT8` | `HRESULT` | Return value |

**Description:** Writes a BOOL parameter of the IO-Link device.

##### `Write_OCTEDSTRING` — §6.2.1.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Write_OCTEDSTRING` | `HRESULT` | Return value |

**Description:** Writes an OCTEDSTRING parameter of the IO-Link device.

##### `Write_REAL` — §6.2.1.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Write_Real` | `HRESULT` | Return value |

**Description:** 46

##### `Write_STRING` — §6.2.1.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Write_STRING` | `HRESULT` | Return value |

**Description:** Writes a string parameter of the IO-Link device.

##### `Write_STRING_FixedLen` — §6.2.1.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |
| `nIndex` | `WORD` | Index of the IO-Link Parameter |
| `nSubindex` | `BYTE` | Subindex of the IO-Link Parameter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Write_STRING_FixedLen HRESULT` | `Return value` | — |
| `Write_UINT16` | `HRESULT` | Return value |
| `Write_UINT32` | `HRESULT` | Return value |
| `Write_UINT64` | `HRESULT` | Return value |
| `Write_UINT8` | `HRESULT` | Return value |

**Description:** Object-oriented function blocks | FB_IOlSlave

#### Object-oriented function blocks › FB_IOlSlave

##### `Sample` (FUNCTION_BLOCK) — §6.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL := FALSE` | — |
| `sApplicationTag` | `STRING(32) := ''` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL := FALSE` | — |
| `bDone` | `BOOL := FALSE` | — |
| `bError` | `BOOL := FALSE` | — |
| `nAdsError` | `E_AdsErr := E_AdsErr.NOERR` | — |
| `nIolError` | `E_IolError := E_IolError.NoError` | — |
| `sErrorTxt` | `STRING(255) := ''` | — |

**Description:** The use of individual functions can look like this. In this sample, the ApplicationSpecificTag is written via Write and then read and checked via Read method.

#### Object-oriented function blocks › FB_IolStdVarSlave › Methods

##### `ReadApplicationSpecificTag` — §6.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to STRING(32) Variable of the buffer that is to receive the read data.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the ApplicationSpecificTag.

##### `ReadDetailedDeviceStatus` — §6.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DetailedStatus` | `Reference to [} 17]` | Variable of the buffer that is to receive the read data. T_IolDetailedDeviceStatus |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the Detailed Device Status of an IO-Link device.

##### `ReadDeviceAccessLocks` — §6.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to` | Specifies the device access lock ST_IolAcessLocks [} 16] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the device access lock of an IO-Link device.

##### `ReadDeviceStatus` — §6.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to` | Variable of the buffer that is to receive the read data. E_IolDeviceStatus [} 14] |
| `data` | `Reference to [} 16]` | Variable of the buffer that is to receive the read data. ST_IolDirectParameter1 |
| `data` | `Reference to [} 17]` | Variable of the buffer that is to receive the read data. ST_IolDirectParameter2 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |
| `hresult` | `HRESULT` | Return value |
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the device status of the IO-Link device.

##### `ReadFirmwareVersion` — §6.3.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to String(64)` | Variable of the buffer that is to receive the read data. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the firmware version of an IO-Link device.

##### `ReadHardwareVersion` — §6.3.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to STRING(64) Variable of the buffer that is to receive the read data.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the hardware version of the IO-Link device.

##### `ReadProductID` — §6.3.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to STRING(64) Variable of the buffer that is to receive the read data.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the product ID of the IO-Link device.

##### `ReadProductName` — §6.3.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to STRING(64) Variable of the buffer that is to receive the read data.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT 56` | Return value |

**Description:** Reads out the product name of the IO-Link device.

##### `ReadProductText` — §6.3.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to STRING(64) Variable of the buffer that is to receive the read data.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the product text of an IO-Link device.

##### `ReadProfileCharacteristic` — §6.3.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to [} 17]` | Variable of the buffer that is to receive the read data. T_IolProfileCharacteristics |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the profile characteristics of an IO-Link device.

##### `ReadSerialNumber` — §6.3.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to STRING(16) Variable of the buffer that is to receive the read data.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Method reads out the serial number of the IO-Link device.

##### `ReadVendorName` — §6.3.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to STRING(64) Variable of the buffer that is to receive the read data.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the vendor name of the IO-Link device.

##### `ReadVendorText` — §6.3.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to STRING(64) Variable of the buffer that is to receive the read data.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Reads out the vendor text of the IO-Link device.

##### `SysCmdFactoryReset` — §6.3.1.16

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Resets the IO-Link device to factory settings.

##### `SysCmdApplicationReset` — §6.3.1.17

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Resets the IO-Link device to the application settings.

##### `SysCmdDeviceReset` — §6.3.1.18

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Resets the IO-Link device to device settings.

##### `SysCmdBlockParamStart` — §6.3.1.19

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Starts the block parameterization.

##### `SysCmdBlockParamEnd` — §6.3.1.20

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT 60` | Return value |

**Description:** Ends the block parameterization.

##### `SysCmdStoreParameterServer` — §6.3.1.21

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Sets the data storage via the PLC.

##### `WriteApplicationSpecificTag` — §6.3.1.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `STRING(32)` | Variable of the buffer that is to receive the read data. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Writes the IO-Link parameter ApplicationSpecifigTag.

##### `WriteDeviceAccessLocks` — §6.3.1.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to` | Variable of the buffer that is to receive the written |
| `ST_IolAccessLocks [} 16]` | `data.` | — |
| `data` | `Reference to [} 17]` | Variable of the buffer that is to receive the read data. ST_IolDirectParameter2 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |
| `hresult` | `HRESULT` | Return value |

**Description:** Writes the DeviceAcessLocks of an IO-Link device.

##### `WriteProfileCharacteristic` — §6.3.1.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to` | Variable of the buffer that is to receive the written |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Writes the ProfileCharacteristic parameter of the IO-Link device.

##### `WriteSystemCommand` — §6.3.1.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to` | Variable of the buffer that is to receive the written |
| `E_IolSystemcommand` | `data. [} 15]` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Writes any system command for the IO-Link device.

#### Object-oriented function blocks › FB_IolStdVarSlave

##### `Sample` (FUNCTION_BLOCK) — §6.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL := FALSE` | — |
| `sApplicationTag` | `STRING(32) := ''` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL := FALSE` | — |
| `bDone` | `BOOL := FALSE` | — |
| `bError` | `BOOL := FALSE` | — |
| `nAdsError` | `E_AdsErr := E_AdsErr.NOERR` | — |
| `nIolError` | `E_IolError := E_IolError.NoError` | — |
| `sErrorTxt` | `STRING(255) := ''` | — |

**Description:** In this sample, the ApplicationSpecificTag is written using the WriteApplicationspecificTag methods and then read and checked using the ReadApplicationSpecificTag method.

#### Object-oriented function blocks › FB_IolExtIdentSlave › Methods

##### `ReadFunctionTag` — §6.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to STRING(32) Variable of the buffer that is to receive the read data.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Function to read the FunctionTag.

##### `ReadLocationTag` — §6.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `Reference to STRING(32) Variable of the buffer that is to receive the read data.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Function to read out the LocationTag. The LocationTag is a parameter that contains the location information of the IO-Link device.

##### `WriteFunctionTag` — §6.4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `STRING(32)` | Variable of the buffer that is to receive the read data. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Writes the FunctionTag parameter.

##### `WriteLocationTag` — §6.4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `data` | `STRING(32)` | Variable of the buffer that is to receive the read data. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hresult` | `HRESULT` | Return value |

**Description:** Writes the LocationTag of an IO-Link device.

#### Object-oriented function blocks › FB_IolExtIdentSlave

##### `Sample` (FUNCTION_BLOCK) — §6.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL := FALSE` | — |
| `sApplicationTag` | `STRING(32) := ''` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL := FALSE` | — |
| `bDone` | `BOOL := FALSE` | — |
| `bError` | `BOOL := FALSE` | — |
| `nAdsError` | `E_AdsErr := E_AdsErr.NOERR` | — |
| `nIolError` | `E_IolError := E_IolError.NoError` | — |
| `sErrorTxt` | `STRING(255) := ''` | — |

**Description:** In this sample, the ApplicationSpecificTag is written using the WriteApplicationspecificTag methods and then read and checked using the ReadApplicationSpecificTag method.
