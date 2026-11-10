# Tc2_Utilities

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_Utilities_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T2 · **API entries:** 325
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Mixed: BCD/hash/format/memory-buffer/enum helpers are pure IEC (T0 subset); file, registry, license, AMS router, timezone and NT_* entries need host OS services (T2).

---

#### Function blocks › [obsolete]

##### `FB_AdsReadEvents` — §3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AMSNetId` | AmsNetId of the device from which the EventLogger |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aEvents` | `ARRAY[1..80] OF` | This array is used by the function block to make the read |
| `ST_ReadEvent` | `messages available. The array can store a maximum of 80 messages. (see ST_ReadEvent [} 349])` | — |
| `nNumberOfEvents` | `UDINT` | Indicates how many messages are currently stored in the |

**Description:** The function block queries the active messages of the EventLogger via ADS and makes them available in the form of an array aEvents. To display the messages in the visualization element Event table, the array aEvents has to be entered in its property Message data array.

##### `FB_GetDeviceIdentification` — §3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this input. |
| `tTimeout` | `TIME` | States the length of the timeout that may not be exceeded by execution of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The block reads the device ID.

##### `FB_FileTimeToTzSpecificLocalTime` — §3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `in` | `T_FILETIME` | UTC time (file time format) to be converted (type: T_FILETIME |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `out` | `T_FILETIME` | Converted local time (file time format, type: T_FILETIME [} 372]). |
| `eTzID` | `E_TimeZoneID` | Additional summer/winter time information (type:E_TimeZoneID [} 357]). |
| `bB` | `BOOL` | TRUE => B time (e.g.:02:05:00 CET B), FALSE => other time (z. B.: |

**Description:** The function block converts the UTC time (file time format) to local time (file time format), taking into account the specified time zone information. The function block: FB_SystemTimeToTzSpecificLocalTime [} 121] has a similar functionality but uses a different time format (structured system time format).

##### `FB_TzSpecificLocalTimeToFileTime` — §3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_FILETIME` | Local time (file time format) to be converted (type: T_FILETIME |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `out` | `T_FILETIME` | Converted UTC time (file time format) (type: T_FILETIME [} 372]). |
| `eTzID` | `E_TimeZoneID` | Additional summer/winter time information (type:E_TimeZoneID [} 357]). |
| `bB` | `BOOL` | TRUE => B time (e.g.: 02:05:00 CET B), FALSE => other time (e.g.: |

**Description:** The function block converts the local time (file time format) to UTC time (file time format), taking into account the specified time zone information. The function block FB_TzSpecificLocalTimeToSystemTime [} 125] has similar functionality but converts a different time format (structured system time format).

#### Function blocks

##### `BCD_TO_DEC` — §3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `START` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | This output is set at the start of the conversion procedure, |

**Description:** The function block "BCD_TO_DEC" allows BCD numbers to be converted into decimal format. The BCD number to be converted is checked for admissibility of the values.

##### `DCF77_TIME` — §3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DCF_PULSE` | `BOOL` | The DCF-77 signal. |
| `RUN` | `BOOL` | A rising edge at this input initializes the function block and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | This output is set at the start of the conversion procedure, |

**Description:** The "DCF77_TIME" function block can be used to decode the DCF-77 radio clock signal. A rising edge at the RUN input starts the decoding process, which continues as long as the RUN input remains set. In the worst case, the function block requires a maximum of 1 minute to synchronize itself, plus a further minute to decode the data for the following minute. During this time, the missing 59th second marker is waited for. Internally the function block is sampling the DCF-77 signal. In order to be able to sample the edges without error the function block should be called once in each PLC cycle. Satisfactory results can be obtained with a cycle time of <= 25 ms. In case of a missing or faulty DCF-77 signal, the ERR output is set to TRUE and a corresponding error code is set at the ERRID output. The ERR and ERRID outputs are reset the next time a correct signal is received. Some receivers provide an inverted DCF-77 signal. In such cases the signal must first be inverted before being passed to the DCF_PULSE input. When operating without errors, the current time is updated at the CDT output every minute. Thereby the READY output is set to TRUE for one cycle of the PLC at the zeroth second. At this time the DCF-77 time at the CDT output is valid, and can be evaluated by the PLC program. The READY output is only set if no error was detected in the data for the following minute. The transferred parity bits are used for error detection. In the event of poor reception conditions, 100% erro

##### `DCF77_TIME_EX` — §3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DCF_PULSE` | `BOOL` | The DCF-77 signal. |
| `RUN` | `BOOL` | A rising edge at this input initializes the function block and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | This output is set when the function block is activated. |
| `ERR` | `BOOL` | This output is set when an error occurs in the decoding. |
| `ERRID` | `UDINT` | If the ERR output is set, this parameter supplies the error |

**Description:** The function block "DCF77_TIME_EX" can be used to decode the DCF-77 radio clock signal. In contrast to the "DCF77_TIME [} 34]" function block, this block checks two consecutive telegrams for plausibility as standard.

##### `DEC_TO_BCD` — §3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `START` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `BUSY` | `BOOL` | This output is set at the start of the conversion procedure, |

**Description:** The function block "DEC_TO_BCD" allows decimal numbers to be converted to BCD format. The number to be converted is checked for admissibility of the values.

##### `FB_AddRouteEntry` — §3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block can be used to add a new AMS router connection (remote route) to a TwinCAT system.

##### `FB_AddRouteEntryEx` — §3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block can be used to add a new AMS router connection (remote route) to a TwinCAT system.

##### `FB_AmsLogger` — §3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** 44

##### `FB_BasicPID` — §3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `LREAL` | Setpoint of the controlled variable |
| `fActualValue` | `LREAL` | Actual value of the controlled variable |
| `bReset` | `BOOL` | TRUE at this input resets the internal state variables and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fCtrlOutput` | `LREAL` | Output of the PID-element |
| `nErrorStatus` | `UINT` | Indicates the error number in the event of an error |

**Description:** The function block is a simple discretized PID element.

##### `FB_CalcHashValue` (METHOD) → `BOOL` — §3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hashMode` | `E_HashMode` | — |
| `pData` | `PVOID` | — |
| `nData` | `UDINT` | — |
| `pHash` | `PVOID` | — |
| `nHash` | `UDINT` | — |

**Description:** This function block calculates a hash value.

##### `FB_CheckLicense` — §3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_CheckLicense determines the TwinCAT 3 license status for a given license ID.

##### `FB_CheckOemLicense` — §3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_CheckOemLicense determines the TwinCAT 3 license status for a given license ID and a given OEM ID.

##### `FB_CSVMemBufferReader` — §3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eCmd` | `E_EnumCmdType [} 352]` | Control parameters for the buffer component. eEnumCmd_First reads the first data field, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | TRUE = success, FALSE = faulty data/faulty input |

**Description:** This function block can be used to decompose/interpret data sets stored in an external buffer into individual data fields. The buffer data could first be read from a file with the aid of the function blocks for file access, for example. The function block reads the first or the next data field and returns its value either as a string at the getValue output or as an address/byte value at the pValue/cbValue output.

##### `FB_CSVMemBufferWriter` — §3.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eCmd` | `E_EnumCmdType [} 352]` | eEnumCmd_First adds the first data field to the buffer, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | TRUE = success, FALSE = buffer overflow or faulty input |

**Description:** This function block can be used to generate data sets in an external buffer in CSV format from individual data fields. The content of the buffer can then be written into a file, e.g. with the aid of the function blocks for file access. The new data field can be transferred to the function block either via the putValue variable(string) or via the optional pValue and cbValue variables. This depends on whether you want to write data fields without control characters (string) or data fields with control characters or binary data to the data set. The function block can generate several data sets in the buffer until the maximum available buffer size is reached. The end of the data set (last data field in the current data set) is automatically appended to the data field if the bCRLF variable was set to TRUE during writing of the data field. The function block automatically adds the data field separators. The default data field separator is a semicolon. The separator can be configured from semicolon to comma via the global PLC variable DEFAULT_CSV_FIELD_SEP.

##### `FB_EnumFindFileEntry` — §3.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block searches a directory for a file or a subdirectory whose name is similar to the specified name. Any entries found can be read individually. See also description of the FB_EnumFindFileList function block. The input parameter eCmd is used for navigating through the list of entries. The eCmd input determines whether the first or the next input is read, for example.

##### `FB_EnumFindFileList` — §3.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block searches a directory for a file or a subdirectory whose name is similar to the specified name. Any entries found can be read individually. See also description of the FB_EnumFindFileEntry function block. The input parameter eCmd is used for navigating through the list of entries. The eCmd input determines whether the first or the next input is read, for example.

##### `FB_EnumRouteEntry` — §3.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block is used to transfer information to other TwinCAT systems via the AMS router connections (remote routes). If several connections are used the function block has to be called up repeatedly. Only one entry can be handled for each call. The input parameter eCmd is used for navigating through the list of entries. The eCmd input determines whether the first or the next input is read, for example.

##### `FB_EnumStringNumbers` — §3.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSearch` | `T_MaxString` | Search string to be searched for numbers |
| `eCmd` | `E_EnumCmdType [} 352]` | Control command for the enumeration block |
| `eType` | `E_NumGroupTypes [} 355] Number format of the searched number. This parameter` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sNumber` | `T_MaxString` | The last found number as string |
| `nPos` | `INT` | This variable always returns the position after the last |

**Description:** This function block can be used to search a string in a REPEAT or WHILE loop for numbers. The string may contain several numbers. Any numbers that are found are output as sub-strings at the function block output. The function searches from the current position for the first character that can be interpreted as a numeral. The search is aborted if a character is found that cannot be interpreted as a number. The eCmd parameter determines whether the search is for the first number or the next number. The eType parameter determines the format of the numbers in the search string.

##### `FB_FileProperties` — §3.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block outputs the file properties of a selected file.

##### `FB_FileRingBuffer` — §3.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_FileRingBuffer allows data sets of varying lengths to be written into a ring buffer file, or for data sets that have previously been written there to be removed from the ring buffer file. The written data sets are read out according to the FIFO principle in the same order in which they were previously written to the ring buffer file. This means that the oldest entries are the first ones that are read. Opening, closing, writing and reading the data sets is controlled by action calls. The function block features the following tasks:

##### `FB_FileTime64ToTzSpecificLocalTime` — §3.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_FILETIME64 [} 373]` | UTC time (file time format) to be converted. |
| `tzInfo` | `ST_TimeZoneInformation` | Structure variable with the current time zone information of [} 370] the operating system. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `out` | `T_FILETIME64 [} 373]` | Converted local time (file time format) |
| `eTzID` | `E_TimeZoneID [} 357]` | Additional summer/winter time information. |
| `bB` | `BOOL` | TRUE => B time (e.g.: 02:05:00 CET B), FALSE => other |

**Description:** The function block converts the UTC time (file time format) to local time (file time format), taking into account the specified time zone information. The function block: FB_SystemTimeToTzSpecificLocalTime [} 121] has a similar functionality but uses a different time format (structured system time format).

##### `FB_FormatString` — §3.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sFormat` | `T_MaxString` | Format specification as string (e.g. '%+20.5f' or 'Measure |
| `pFormatString` | `STRING` | Pointer to the format specification as a string. The address |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurred during the formatting. |
| `nErrId` | `UDINT` | Returns the Format error code [} 409] if the bError output |
| `bError` | `BOOL` | TRUE if an error occurred during the formatting. |
| `nErrId` | `UDINT` | Returns the Format error code [} 409] if the bError output |

**Description:** This function block can be used for converting up to 10 arguments (similar to fprintf) into a string and formatting them according to the format specification [} 406]. The formatting takes place in the same PLC cycle. This means that the output string is available immediately after calling the FB.

##### `FB_GetAdaptersInfo` — §3.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block can be used to read adapter information for a TwinCAT PC. The maximum number of adapter information that can be read is currently limited to MAX_LOCAL_ADAPTERS + 1 (default = 6).

##### `FB_GetAdaptersInfoEx` (METHOD) → `BOOL` — §3.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block can be used to read adapter information for a TwinCAT PC. The maximum number of adapter information read is limited to 64.

##### `FB_GetDeviceIdentificationEx` — §3.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block reads the device ID. Allows longer strings for hardware model and hardware serial number than FB_GetDeviceIdentification [} 27].

##### `FB_GetDongleSystemID` — §3.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_GetDongleSystemID reads the system ID and the volume ID of the TwinCAT 3 license dongle as GUID.

##### `FB_GetHostAddrByName` — §3.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `74 Function blocks` | `` | — |
| `sNetID` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block can be used to read the (IPv4) Internet protocol network address for the specified host name. The address is returned as string and byte array.

##### `FB_GetHostName` — §3.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block can be used to read the host name of a TwinCAT PC.

##### `FB_GetLicenseDongles` — §3.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** 76

##### `FB_GetLicenses` — §3.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block reads the valid and invalid TwinCAT licenses.

##### `FB_GetLicensesEx` — §3.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_GetLicensesEx determines the status of all TwinCAT 3 licenses and OEM licenses.

##### `FB_GetLocalAmsNetId` — §3.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block can be used to read the network address (AmsNetID) of a local TwinCAT PC.

##### `FB_GetOemOfLicenseId` — §3.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_GetOemLicenseId returns the OEM ID of a given license ID.

##### `FB_GetRouterStatusInfo` — §3.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `82 Function blocks` | `` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_GetRouterStatusInfo can be used to read status information for the TwinCAT router from the PLC (available memory, number of registered ports etc.).

##### `FB_GetSystemId` — §3.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block reads the system ID of type GUID (see "About TwinCAT..." at the TwinCAT icon in the system tray).

##### `FB_GetTimeZoneInformation` — §3.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AmsNetId (AMS network identifier) of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block can be used to read the time zone settings of the operating system.

##### `FB_GetVolumeId` — §3.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_GetVolumeId reads the system ID and the volume system ID as a GUID.

##### `FB_HashTableCtrl` — §3.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `DWORD` | Key (unsigned 32 bit number or pointer). This key enables |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hTable` | `T_HHASHTABLE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | Returns TRUE if a new data element was added to/ |

**Description:** The hash table can be used to find an individual data element quickly among a larger number of data elements. The data objects must have a unique key. The key enables the data objects to be identified unambiguously and found quickly in the table.

##### `FB_LicFileCopyFromDongle` — §3.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetIdSrc` | `T_AmsNetId` | AmsNetId (AMS network ID) of the license dongle. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block copies a file from the license dongle to the hard disk. If the file is larger than the buffer (cbCopyLen), the file copying procedure is automatically split into several read and write operations until the whole file is copied. Only then does bBusy switch to FALSE.

##### `FB_LicFileCopyToDongle` — §3.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetIdSrc` | `T_AmsNetId` | AmsNetId (AMS network identifier) of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block copies a file from the hard disk to the license dongle. If the file is larger than the buffer (cbCopyLen), the file copying procedure is automatically split into several read and write operations until the whole file is copied. Only then does bBusy switch to FALSE.

##### `FB_LicFileCreate` — §3.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `90 Function blocks` | `` | — |
| `sNetId` | `T_AmsNetId` | AmsNetId (AMS network ID) of the license dongle |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block creates a file on the license dongle. A rising edge at bExecute writes the data from the buffer (pWriteBuff and cbWriteLen) directly onto a new file on the dongle.

##### `FB_LicFileDelete` — §3.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | AmsNetId (AMS network ID) of the license dongle |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** Function blocks

##### `FB_LicFileGetStorageInfo` — §3.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | AmsNetId (AMS network ID) of the license dongle |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stStorageInfo` | `ST_LicStorageInfo` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block reads the StorageInfo of the license dongle and the file directory.

##### `FB_LicFileRead` — §3.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | AmsNetId (AMS network ID) of the license dongle |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block reads a file from the license dongle into a provided buffer (pDestBuff and cbReadLen) via a rising edge at bExecute. The buffer must be large enough for the file, otherwise only the first part of the file is read.

##### `FB_LinkedListCtrl` — §3.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `putValue` | `PVOID` | Value/data element (input parameter, 32/64 bit, unsigned |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hList` | `T_HLINKEDLIST` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | Result of the last action called. Returns TRUE if a new |

**Description:** The function block FB_LinkedListCtrl can be used to implement a linked list in the PLC project. A double- linked list is created. A linked list allows values (known as nodes) to be stored. It is possible to iterate the list from the back to the front or the other way. Nodes can quickly be added or deleted.

##### `FB_LocalSystemTime` — §3.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string containing the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL` | The time at the systemTime output is invalid if this output |

**Description:** In some applications the local Windows system time is synchronized via the SNTP time server or a radio clock. In many cases the local Windows system time has to be used in the PLC (e.g. in the form of timestamp log messages to the HMI). The local Windows system time is displayed in the taskbar. For such applications the FB_LocalSystemTime function block can be useful.

##### `FB_MemBufferMerge` — §3.48

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eCmd` | `E_EnumCmdType [} 352]` | Control parameters for the enumeration module. eEnumCmd_First adds the first segment, eEnumCmd_Next adds the next segment. No other |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | TRUE = success, FALSE = buffer overflow or faulty input |

**Description:** This function block consolidates individual smaller data segments to form a larger data segment. The destination buffer must be transferred as input parameter to the function block. No further data bytes are added, if the segment to be added exceeds the remaining free buffer size.

##### `FB_MemBufferSplit` — §3.49

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eCmd` | `E_EnumCmdType [} 352]` | Control parameters for the function block. eEnumCmd_First returns the first segment, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | TRUE = success, FALSE = error, invalid parameter value |

**Description:** This function block splits a memory area (data buffer) into several smaller segments of certain maximum length as required. The function block returns a smaller partial segment, if the length of the last segment is smaller than required.

##### `FB_MemRingBuffer` — §3.50

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `pWrite` | `ARRAY OF BYTE` | The address of the PLC variable or of a buffer variable that |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | Returns TRUE, if a new data set was added or removed |

**Description:** The function block FB_MemRingBuffer can be used to write data sets with different lengths in a ring buffer or to read previously written data sets from the ring buffer. The written data sets are read out according to the FIFO principle in the same order in which they were previously written to the ring buffer. This means that the oldest entries are the first ones that are read. The buffer memory is made available to the function block via the pBuffer/cbBuffer input variables. Writing/reading of data sets is controlled via action calls. The function block features the following tasks: • A_AddTail (writes a new data set into the ring buffer.) • A_GetHead (reads the oldest data set in the ring buffer, but does not remove it.) • A_RemoveHead (reads and removes the oldest data set from the ring buffer.) • A_Reset (deletes all data sets in the ring buffer.)

##### `FB_MemRingBufferEx` — §3.51

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pWrite` | `ARRAY OF BYTE` | The address of the PLC variable or of a buffer variable that |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | Returns TRUE, if a new data set was added or removed |

**Description:** The function block FB_MemRingBufferEx can be used to write data sets with different lengths in a ring buffer or to read previously written data sets from the ring buffer. The written data sets are read out according to the FIFO principle in the same order in which they were previously written to the ring buffer. This means that the oldest entries are the first ones that are read. The buffer memory is made available to the function block via the pBuffer/cbBuffer input variables. Writing/reading of data sets is controlled via action calls.

##### `FB_MemStackBuffer` — §3.52

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pWrite` | `ARRAY OF BYTE` | The address of the PLC variable or of a buffer variable that |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | Returns TRUE, if a new data set was added or removed |

**Description:** The function block FB_MemStackBuffer can be used to write data sets with different lengths in a buffer or to read previously written data sets from the buffer. The data sets are read based on the LIFO principle (last in - first out), i.e. in the reverse order in which they were written into the buffer. In other words, the latest entries

##### `FB_RegQueryValue` — §3.53

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The system registry is a hierarchically structured tree. A node in the tree is referred to as a key. Each key may contain subkeys and data values. The function block "FB_RegQueryValue" can be used to read individual system registry values from the branch with the predefined handle HKEY_LOCAL_MACHINE. If successful cbData data bytes are copied into the buffer with the address pData. The function block can be used to read any value types (e.g. REG_DWORD, REG_SZ) or binary data with unlimited byte length (REG_BINARY).

##### `FB_RegSetValue` — §3.54

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The system registry is a hierarchically structured tree. A node in the tree is referred to as a key. Each key may contain subkeys and data values.

##### `FB_RemoveRouteEntry` — §3.55

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block can be used to delete an existing connection to a TwinCAT system from the list of AMS router connections (remote routes).

##### `FB_ScopeServerControl` (FUNCTION_BLOCK) — §3.56

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `sNetID` | `T_AmsNetID` | A string containing the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_ScopeServerControl enables the PLC to collect data for subsequent display with TwinCAT Scope 2.

##### `FB_SetTimeZoneInformation` — §3.57

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `118 Function blocks` | `` | — |
| `sNetID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** This function block can be used to modify or set the time zone settings of the operating system.

##### `FB_StringRingBuffer` — §3.58

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `120 Function blocks` | `` | — |
| `bOverwrite` | `BOOL` | If TRUE and in the event of a buffer overflow the oldest |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | Returns TRUE, if a new string was added or removed |

**Description:** The function block FB_StringRingBuffer can be used to write string variables into the ring buffer or read previously written string variables from the ring buffer. The written strings are read out according to the FIFO principle in the same order in which they were previously written to the ring buffer. This means that the oldest entries are the first ones that are read. The buffer memory is made available to the function block via the pBuffer/cbBuffer input variables. Writing/reading of strings is controlled via action calls. The function block features the following tasks: • A_AddTail (writes a new string into the ring buffer.) • A_GetHead (reads the oldest string in the ring buffer, but does not remove it.) • A_RemoveHead (reads and removes the oldest string from the ring buffer.) • A_Reset (deletes all strings from the ring buffer.)

##### `FB_SystemTimeToTzSpecificLocalTime` — §3.59

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `TIMESTRUCT [} 377]` | UTC time (structured system time format) to be converted. |
| `tzInfo` | `ST_TimeZoneInformation` | Structure variable with the current time zone information of [} 370] the operating system. |
| `in` | `T_FILETIME [} 373]` | Local time (file time format) to be converted. |
| `tzInfo` | `ST_TimeZoneInformation` | Structure variable with the current time zone information of [} 370] the operating system. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `out` | `TIMESTRUCT [} 377]` | Converted local time (structured system time format) |
| `eTzID` | `E_TimeZoneID [} 357]` | Additional summer/winter time information. |
| `bB` | `BOOL` | TRUE => B time (e.g.: 02:05:00 CET B), FALSE => other |
| `out` | `T_FILETIME [} 373]` | Converted UTC time (file time format) |
| `eTzID` | `E_TimeZoneID [} 357]` | Additional summer/winter time information. |
| `bB` | `BOOL` | TRUE => B time (e.g.: 02:05:00 CET B), FALSE => other |

**Description:** The function block converts the UTC time (structured system time format) to local time (structured system time format), taking into account the specified time zone information. The function block FB_FileTime64ToTzSpecificLocalTime [} 65] has similar functionality but uses a different time format (file time format).

##### `FB_TzSpecificLocalTimeToSystemTime` — §3.61

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `TIMESTRUCT [} 377]` | Local time (structured system time format) to be converted Function blocks |
| `tzInfo` | `ST_TimeZoneInformation` | Structure variable with the current time zone information of [} 370] the operating system. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `out` | `TIMESTRUCT [} 377]` | Converted UTC time (structured system time format) |
| `eTzID` | `E_TimeZoneID [} 357]` | Additional summer/winter time information. |
| `bB` | `BOOL` | TRUE => B time (e.g.: 02:05:00 CET B), FALSE => other |

**Description:** The function block converts the local time (structured system time format) to UTC time (structured system time format), taking into account the specified time zone information. The function block FB_TzSpecificLocalTimeToFileTime64 [} 123] has similar functionality but uses a different time format (file time format).

##### `FB_WritePersistentData` — §3.62

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer whose |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_WritePersistentData is an extended version of the WritePersistentData [} 162] function block. The system behavior for writing the persistent data [} 406] (data consistency/task cycle time overrun) can be influenced via the MODE parameter.

##### `FB_GetRtPerformanceData` — §3.63

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_GetRtPerformanceData can be used to determine the current RealTime Performance data of a TwinCAT system.

##### `GetRemotePCInfo` — §3.64

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer whose |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block GetRemotePCInfo can be used to read information about configured remote PCs in the TwinCAT router. After successful execution the structure "RemotePCInfo" contains the NetIDs and names of the remote PCs as strings in the order in which they were stored in the TwinCAT router. The function block allows router information relating to either a local or to a remote TwinCAT system to be read.

##### `NT_AbortShutdown` — §3.65

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block NT_AbortShutdown can be used to abort a shutdown command that has previously been called with the NT_Shutdown [} 136] function block.

##### `NT_GetTime` — §3.66

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer whose |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block NT_GetTime can be used to determine the local Windows system time of a TwinCAT system (the local Windows system time is displayed in the taskbar). The year, month, day, day of the week, hour, minute, second and millisecond are stored in variables of the structure TIMESTRUCT [} 377].

##### `NT_Reboot` — §3.67

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer that is to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The Windows NT operating system can be restarted with the aid of the function block NT_Reboot. The function largely corresponds to the Restart command on the Windows taskbar. A delay before execution of the Restart command can be defined via the parameter DELAY.

##### `NT_SetLocalTime` — §3.68

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer whose |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block NT_SetLocalTime can be used to set the local Windows system time of a TwinCAT system (the local Windows system time is displayed in the taskbar).

##### `NT_SetTimeToRTCTime` — §3.69

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block NT_SetTimeToRTCTime can be used to synchronize the local Windows system time (displayed in the taskbar) with the real-time clock of the PC (RTC time in the BIOS).

##### `NT_Shutdown` — §3.70

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer whose |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The Windows NT operating system can be shut down with the aid of the function block NT_Shutdown. The function largely corresponds to the Shut Down command on the Windows taskbar. A delay before execution of the Shut Down command can be defined via the DELAY parameter.

##### `NT_StartProcess` — §3.71

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block NT_StartProcess can be used to start a Windows application from the PLC. The function block can also be used to run applications on a remote PC.

##### `PLC_ReadSymInfo` — §3.72

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer whose |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block PLC_ReadSymInfo can be used to determine information regarding the symbols (variables) of a PLC run-time system.

##### `PLC_ReadSymInfoByName` — §3.73

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block PLC_ReadSymInfoByName allows additional information about a PLC symbolic variables (e.g. data type identification, index group, index offset, comment...) to be read using the symbol name. After successful execution, the data is available in the data structure SymInfo, whose type is SYMINFOSTRUCT. The maximum length of the symbol name that is transferred to the function block as a parameter is limited to 255 characters.

##### `PLC_ReadSymInfoByNameEx` — §3.74

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block PLC_ReadSymInfoByNameEx has similar functionality as the function block PLC_ReadSymInfoByName [} 140]. Both function blocks can read symbol information through the symbol name. The difference between the two function blocks is that the function block described here does not return an error if the available buffer size is exceeded and may output incomplete information. In this case the comment and/or the data type identification may have been truncated. Two additional output variables indicate this, i.e. OVTYPE and OVCOMMENT, so that the application can respond accordingly.

##### `PLC_Reset` — §3.75

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block PLC_Reset can be used to reset a PLC runtime system. When the PLC is reset, the PLC variables are filled with their initial values, and the execution of the PLC program is stopped.

##### `PLC_Start` — §3.76

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block PLC_Start can be used to start a PLC runtime system on a TwinCAT computer. The function block can, for instance, be used to start the PLC on a remote PC.

##### `PLC_Stop` — §3.77

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block PLC_Stop can be used to stop a PLC runtime system on a TwinCAT computer. The function block can, for instance, be used to stop the PLC on a remote or a local PC.

##### `Profiler` — §3.78

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `START` | `BOOL` | A positive edge at this input starts the measurement of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | This output is set at the start of the measuring procedure, |

**Description:** The function block Profiler can be used to allow the execution time of PLC code to be measured. Internally, an instance of the GETCPUACCOUNT function block is called. The measurement is started by a rising edge at the START input, and is stopped by a falling edge. The measurements are evaluated internally, and are then made available for further processing at the DATA output in a structure of type PROFILERSTRUCT. In addition to the current, minimum and maximum execution times, the function block calculates the mean execution time for the last 10 measurements. The number of averaged measured values can be configured via the global variable MAX_AVERAGE_MEASURES [} 379] between 2 and 100. The times measured are given in microseconds. The output variable DATA.MeasureCycle [} 360] provides information about the number of measurements that have already been carried out. In order to measure the execution time for a specific segment of the PLC program the measurement must be started by a rising edge at the START input when the segment to be measured starts, and stopped by a falling edge at the START input at the end of the segment. All values at the DATA output can be reset if a rising edge is generated at the RESET input at the same time as the rising edge at START. The old measured values are then reset when a new measurement starts and are recalculated from the subsequent calls of the function block.

##### `RTC` — §3.79

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `EN` | `BOOL` | The RTC_EX2 function block is re-initialized with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | This output is set if the function block has been initialized |

**Description:** The function block RTC (Real Time Clock) can be used to realize an internal software clock in the TwinCAT PLC. The clock must be initialized with a starting date and time. After the initialization the time and date are updated with each call of the function block. A CPU system clock is used to calculate the current time and date. The function block should be called in every PLC cycle, so that the current time can be calculated. The current date and time are available in the usual DATE_AND_TIME (DT) format at the function block's output. Multiple instances of the RTC function block can be created within one PLC program.

##### `RTC_EX` — §3.80

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `EN` | `BOOL` | The RTC_EX2 function block is re-initialized with a |
| `EN` | `BOOL` | The RTC_EX2 function block is re-initialized with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | This output is set if the function block has been initialized |
| `Q` | `BOOL` | This output is set if the function block has been initialized |

**Description:** The function block RTC_EX (Extended Real Time Clock) allows an internal software clock to be implemented in TwinCAT PLC. The clock must be initialized with a starting date and time. After the initialization the time and date are updated with each call of the function block. A CPU system clock is used to calculate the current time and date. The function block should be called in every PLC cycle, so that the current time can be calculated. At the output of the function block the current date and time are available in the common DATE_AND_TIME (DT) format. In contrast to the RTC [} 148] function block, RTC_EX has an accuracy of one millisecond. Multiple instances of the RTC_EX function block can be created within one PLC program.

##### `TC_Config` — §3.82

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | Network address of the TwinCAT computer on which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** A TwinCAT system in RUN mode (green TwinCAT system icon) can be switched to CONFIG mode (blue TwinCAT system icon) via the function block "TC_Config". If the system is already in CONFIG mode, it is first switched to STOP mode (red TwinCAT system icon) and then to CONFIG mode.

##### `TC_CoreBoostMonitor` — §3.83

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCoreId` | `DINT` | This input is used to set the CPU core to be monitored. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE if an error occurs. |
| `hrErrorCode` | `HRESULT` | Returns the error code for the error that has occurred. Methods |
| `GetAllRtCoreThrottling` | `Returns via references whether frequency throttling is occurring on any of the` | — |
| `[} 154]` | `real-time cores due to the thermal or power limits being exceeded.` | — |
| `GetCoreFrequency [} 154]` | `Returns the configured and current clock frequency of a defined real-time core` | — |

**Description:** The TwinCAT Core Boost function offers the option of increasing the clock frequency for individual real-time cores (see Settings tab of the real-time settings). This leads to higher power consumption of the real-time cores, which can also lead to an increase in temperature. To ensure that the defined limits of the system are not permanently exceeded, it is advisable to monitor the system.

#### Function blocks › TC_CoreBoostMonitor

##### `GetAllRtCoreThrottling` — §3.83.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bRtInThermalThrottli REFERENCE TO BOOL` | `Is TRUE if one of the real-time cores is currently throttling` | — |
| `ng` | `its frequency due to the temperature limits being` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetAllRtCoreThrottli HRESULT` | `In the event of an error, returns the error code for the error` | — |
| `ng` | `that has occurred.` | — |

**Description:** This method returns via references whether frequency throttling is occurring on any of the real-time cores due to the thermal or power limits being exceeded.

##### `GetCoreFrequency` — §3.83.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCurrentCoreFrequency` | `REFERENCE TO UDINT` | — |
| `nConfiguredCoreFrequency` | `REFERENCE TO UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetCoreFrequency` | `HRESULT` | In the event of an error, returns the error code for the error |

**Description:** This method returns the set and current clock frequency of a defined real-time core via references. These can differ if the core has to throttle its frequency.

##### `GetCoreTemperature` — §3.83.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCurrentTemp` | `REFERENCE TO UDINT Returns the current temperature [°C] of the CPU core.` | — |
| `nMaxTemp` | `REFERENCE TO UDINT Returns the maximum temperature [°C] of the CPU core` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetCoreTemperatur HRESULT` | `In the event of an error, returns the error code for the error` | — |
| `e` | `that has occurred.` | — |

**Description:** This method returns the current temperature, the maximum temperature that has occurred since the start of real-time and the temperature limit of a defined real-time core via references.

##### `GetCoreThrottling` — §3.83.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInThermalThrottling REFERENCE TO BOOL` | `Is TRUE if the CPU core is currently throttling its frequency` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetCoreThrottling` | `HRESULT` | In the event of an error, returns the error code for the error |

**Description:** This method returns via references whether a defined real-time kernel is currently being throttled due to the thermal or power limits being exceeded.

##### `GetPowerConsumption` — §3.83.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCurrentPackagePo REFERENCE TO UDINT` | `Returns the total power in watts of the platform package. werConsumption` | — |
| `nPackagePowerLimi REFERENCE TO UDINT` | `Returns the performance limit in watts of the platform` | — |
| `t` | `package. Exceeding these limits can lead to frequency` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetPowerConsumpti HRESULT` | `In the event of an error, returns the error code for the error` | — |
| `on` | `that has occurred.` | — |

**Description:** This method returns the current power consumption or the power consumption limit of the package of a defined real-time core via references. The value is the same for all real-time cores of the same package.

#### Function blocks

##### `TC_CpuUsage` — §3.84

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | Network address of the TwinCAT computer on which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** 156

##### `TC_CpuUsageEx` — §3.85

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block TC_CpuUsageEx can be used to determine the current CPU usage of a TwinCAT CPU core. The function corresponds to the display of the CPU usage in the TwinCAT system menu under the real-time settings for the corresponding CPU core.

##### `TC_Restart` — §3.86

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | Network address of the TwinCAT computer on which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block TC_Restart can be used to restart the TwinCAT system. The function corresponds to the Restart command on the TwinCAT system menu (on the right of the Windows taskbar). Restarting the TwinCAT system involves the TwinCAT system first being stopped, and then immediately started again.

##### `TC_Stop` — §3.87

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block TC_Stop can be used to stop the TwinCAT system. The function corresponds to the Stop command on the TwinCAT system menu (on the right of the Windows taskbar).

##### `TC_SysLatency` — §3.88

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** 160

##### `TC_SysLatencyEx` — §3.89

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** Function blocks

##### `WritePersistentData` — §3.90

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetID` | Network address of the TwinCAT computer on which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** 162

#### Functions

##### `Time functions` (FUNCTION) → `T_FILETIME64` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DTIN` | `DT` | The date and time to be converted, in DATE_AND_TIME format |

**Description:** The function "DT_TO_FILETIME64" can be used to convert a PLC variable in DATE_AND_TIME format (DT) to FILETIME format (64 bit).

#### Functions › Time functions

##### `DT_TO_SYSTEMTIME` (FUNCTION) → `TIMESTRUCT` — §4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DTIN` | `DT` | The date and time to be converted, in DATE_AND_TIME format |

**Description:** The "DT_TO_SYSTEMTIME" function allows a PLC variable in DATE_AND_TIME format (DT) to be converted to a Windows system time structure. The system time has a resolution of 1 ms, while the resolution of DATE_AND_TIME is 1 s. The "wMilliseconds" variable in the system time structure therefore always returns the value zero.

##### `F_EuropeanLocalTime` — §4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `UTC` | `TIMESTRUCT [} 377]` | UTC time (structured system time format) to be converted. |
| `UTC_Offset` | `INT` | Time zone offset in minutes. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_EuropeanLocalTi TIMESTRUCT [} 377]` | `Output of the local time me` | — |
| `bDaylightSavingTim BOOL` | `Additional daylight saving time/standard time information. If` | — |
| `e` | `the output is TRUE, this is the daylight saving time.` | — |

**Description:** The function converts the UTC time (structured system time format) to local time (structured system time format), taking into account the specified offset information. The function may only be used within Europe and is not valid for any time zone in the world.

##### `F_GetDayOfMonthEx` (FUNCTION) → `WORD` — §4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wYear` | `WORD` | Year (1601 to 30827) |
| `wMonth` | `WORD` | Month (1 to 12) |
| `wWOM` | `WORD` | Week of the month (1 to 5). The value 1 corresponds to week 1, 2 |

**Description:** The function calculates the date of the first, second etc. weekday in a particular month and year (e.g. the date of the second Monday in January 2011 ).

##### `F_GetDayOfWeek` (FUNCTION) → `WORD` — §4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DT` | The date whose weekday number is to be determined. |

**Description:** The function returns the number of the weekday according to DIN 1355 / ISO 8601. According to this standard the weekdays are numbered as follows: Monday = 1, Tuesday = 2, ... Sunday = 7.

##### `F_GetDOYOfYearMonthDay` (FUNCTION) → `WORD` — §4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wYear` | `WORD` | Year (0 ~ 2999) |
| `wMonth` | `WORD` | Month (1 ~ 12) |
| `wDay` | `WORD` | Day (1 ~ 31) |
| `Return parameter` | `Description` | — |

**Description:** The function calculates the number of the day in the year.

##### `F_GetMaxMonthDays` (FUNCTION) → `WORD` — §4.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wYear` | `WORD` | Year |
| `wMonth` | `WORD` | Month (1 to 12) |
| `Return parameter` | `Description` | — |

**Description:** The function returns the maximum number of days in the month in a certain month and year.

##### `F_GetMonthOfDOY` (FUNCTION) → `WORD` — §4.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wYear` | `WORD` | Year (0 ~ 2999) |
| `wDOY` | `WORD` | Number of the day in the specified year whose month is to be |

**Description:** The function calculates the month based on the day number in the year.

##### `F_GetWeekOfTheYear` (FUNCTION) → `WORD` — §4.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DT` | The date whose calendar week is to be determined. |

**Description:** The function returns the calendar week number for a specified date according to the DIN 1355 / ISO 8601 standard. • The first calendar week is defined as the first week that contains a minimum of four days of the new year (DIN 1355 / ISO 8601). • The calendar weeks start on a Monday. Each calendar week contains 7 days. • The returned value in the first calendar week has the number 1. • 29, 30 and 31 December may belong to the first calendar week of the following year. • 1, 2 and 3 January may belong to the last calendar week of the previous year.

##### `F_TranslateFileTime64Bias` (FUNCTION) → `T_FILETIME64` — §4.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_FILETIME64 [} 373]` | Input time to be converted. |
| `bias` | `DINT` | Difference between UTC time and local time in minutes (positive or negative values are permitted) |
| `toUTC` | `BOOL` | This parameter can be used to specify the direction in which the |

**Description:** This function converts the input time to the time in another time zone based on the specified bias time shift. The function can be used to convert the local time to UTC time (Universal Time Coordinates) and vice versa, for example.

##### `F_YearIsLeapYear` (FUNCTION) → `BOOL` — §4.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wYear` | `WORD` | Year |
| `Return parameter` | `Description` | — |
| `TRUE` | `Leap year` | — |
| `FALSE` | `No leap year` | — |

**Description:** The function determines whether a year is a leap year.

##### `FILETIME64_TO_DT` (FUNCTION) → `DT` — §4.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fileTime` | `T_FILETIME64 [} 373]` | The time to be converted in FILETIME format |

**Description:** Functions

##### `FILETIME64_TO_SYSTEMTIME` (FUNCTION) → `TIMESTRUCT` — §4.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fileTime` | `T_FILETIME64 [} 373]` | The time to be converted in FILETIME format |

**Description:** The function "FILETIME64_TO_SYSTEMTIME" converts the time in FILETIME format into the "readable" SYSTEMTIME format. The conversion fails if the most significant bit of the 64-bit fileTime variable is set. In this case the TIMESTRUCT member variables have the value zero.

##### `FILETIME64_TO_TOD` (FUNCTION) → `TOD` — §4.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fileTime` | `T_FILETIME64 [} 373]` | The time to be converted in FILETIME format Functions |
| `Return parameter` | `Description` | — |

**Description:** This function can be used to extract the 'Time of Day' from a time in FILETIME format.

##### `OTSTRUCT_TO_TIME` (FUNCTION) → `TIME` — §4.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `OTIN` | `OTSTRUCT [} 359]` | The structure to be converted |

**Description:** The function "OTSTRUCT_TO_TIME" can be used to convert a structure with resolved milliseconds, seconds, minutes, hours, days and weeks into a TIME variable.

##### `STRING_TO_SYSTEMTIME` (FUNCTION) → `TIMESTRUCT` — §4.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `STRING(23)` | The string to be converted. The string must have the following |

**Description:** The function converts a string into the Windows SYSTEMTIME format.

##### `SYSTEMTIME_TO_DT` (FUNCTION) → `DT` — §4.1.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `TIMESTR` | `TIMESTRUCT [} 377]` | The structure with the Windows system time to be converted |
| `systemTim TIMESTRUCT [} 377]` | `The structure with the Windows system time to be converted e` | — |
| `Return parameter` | `Description` | — |

**Description:** The "SYSTEMTIME_TO_DT" function allows the Windows system time structure to be converted to the DATE_AND_TIME format (DT) usual in a PLC. The system time has a resolution of 1 ms and the DATE_AND_TIME has a resolution of 1 s. The milliseconds from the system time are taken into account during the conversion and rounded up to the DATE_AND_TIME return value accordingly. To disable rounding, set the wMilliseconds element in the Windows system time structure to zero.

#### (top level)

##### `Error, wrong SystemTime parameter value` (FUNCTION) → `STRING` — §0

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `systemTime` | `TIMESTRUCT;       (* Input time in system time format (struct) *)` | — |
| `nBias` | `INT;             (* Specifies the current bias, in minutes, for local time translation` | — |
| `bUTC` | `BOOL;        (* Specifies whether the systemTime is UTC or local time. *)` | — |
| `nPrecision` | `USINT(0..9); (* Precision. Number of decimal places of seconds. (0..9) *)` | — |

#### Functions › Time functions

##### `SYSTEMTIME_TO_STRING` (FUNCTION) → `STRING` — §4.1.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `TIMESTRUCT [} 377]` | The structure with the Windows system time to be converted |

**Description:** The function converts the Windows system time structure into a string with the following format: YYYY-MM- DD-hh:mm:ss.xxx : • YYYY: year (1601..9999) • MM: month (01..12) • DD: day (01..31) • hh: hour (00..23) • mm: minutes (00..59) • ss: seconds (00..59) • xxx: millisecond (000..999)

##### `SYSTEMTIME_TO_TOD` (FUNCTION) → `TOD` — §4.1.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `systemTim TIMESTRUCT [} 377]` | `The structure with the Windows system time to be converted e` | — |
| `Return parameter` | `Description` | — |

**Description:** This function can be used to extract the 'Time of Day' from the Windows system time structure.

##### `TIME_TO_OTSTRUCT` (FUNCTION) → `OTSTRUCT` — §4.1.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `TIN` | `TIME` | The TIME variable to be converted |

**Description:** The function "TIME_TO_OTSTRUCT" can be used to convert a TIME constant or variable into a structure with the resolved milliseconds, seconds, minutes, hours, days and weeks.

#### Functions › Extended STRING functions

##### `CHAR_TO_WCHAR` — §4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sTextIn` | `STRING(1)` | STRING variable to be converted. |
| `pSrcString1` | `POINTER TO STRIN` | Pointer to the first of the STRING variables to be concatenated |
| `G` | `(input string)` | — |
| `pSrcString2` | `POINTER TO STRIN` | Pointer to the second of the STRING variables to be concatenated |
| `G` | `(input string)` | — |
| `pDstString` | `POINTER TO STRIN` | Pointer to the resulting STRING variable following the |
| `G` | `concatenation (output string)` | — |
| `nDstSize` | `UDINT` | Size of the resulting STRING variable (output string) in bytes. The ed) |
| `pSrcData` | `POINTER TO BYTE` | Start address (pointer) to the binary data to be converted. The |
| `pSrcString` | `POINTER TO STRING` | Pointer to the STRING variable (input string) |
| `pDstString` | `POINTER TO STRING` | Pointer to the resulting STRING variable (output string) |
| `nDstSize` | `UDINT` | Size of the resulting STRING variable (output string) in bytes. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CHAR_TO_WCHAR` | `WSTRING(1)` | Converts a variable of data type STRING into a variable of |
| `CONCAT2` | `BOOL` | Concatenates two strings of any length of data type STRING and |
| `DATA_TO_HEXSTR2` | `UDINT` | Converts binary data into a hexadecimal string. |
| `DELETE2` | `BOOL` | Deletes nLen characters from a string starting at position nPos. |

**Description:** The function converts a variable of the data type STRING into a variable of the data type WSTRING (with null termination).

##### `F_StringIsASCII` — §4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pString` | `POINTER TO STRING 184` | Pointer to the STRING variable Functions |
| `pSrcString` | `POINTER TO STRING` | Pointer to the STRING variable whose string is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_StringIsASCII` | `BOOL` | The return value is TRUE if the string contains only ASCII characters. |
| `nLen` | `UDINT` | Number of ASCII characters in the string |
| `FIND2` | `UDINT` | Finds a string that may occur more than once in another |

**Description:** The function checks whether a string contains only ASCII characters (0x000 to 0x7F) and returns the number of ASCII characters. The string is directly compatible to UTF-8 if it contains only ASCII characters.

##### `FindAndDelete` — §4.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSrcString` | `POINTER TO STRING` | Pointer to the STRING variable whose string is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindAndDelete` | `UDINT` | Finds a string that may occur more than once in another string and deletes |

**Description:** The function finds a string, which may occur more than once, in another string and removes it.

##### `FindAndDeleteChar` — §4.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSrcString` | `POINTER TO STRING` | Pointer to the STRING variable whose string is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindAndDeleteChar` | `UDINT` | Finds a character that may occur more than once in a string and deletes it |

**Description:** The function finds a character, which may occur more than once, in a string and removes it.

##### `FindAndReplace` — §4.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSrcString` | `POINTER TO STRING` | Pointer to the STRING variable whose string is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindAndReplace` | `UDINT` | Finds a string that may occur more than once in another string and |

**Description:** The function finds a string, which may occur more than once, in another string and replaces it with another string.

##### `FindAndReplaceChar` — §4.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `188 Functions` | `` | — |
| `pSrcString` | `POINTER TO STRING` | Pointer to the STRING variable whose string is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindAndReplaceChar` | `UDINT` | Finds a character, that may occur more than once, in a string and |

**Description:** The function finds a character, which may occur more than once, in a string and replaces it with another character.

##### `FindAndSplit` — §4.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSeparator` | `POINTER TO STRING` | Pointer to the STRING variable that represents the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindAndSplit` | `BOOL` | The return value is TRUE if the separator was found and the division and |

**Description:** The function splits a string into two strings.

##### `FindAndSplitChar` — §4.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSeparatorChar` | `STRING(1)` | Character representing the separator. |
| `pSrcString` | `POINTER TO STRING` | Pointer to the STRING variable that represents the |
| `pSrcHexStr` | `POINTER TO STRING` | Start address (pointer) to the hexadecimal string to be |
| `pSrcString` | `POINTER TO STRING` | Pointer to the STRING variable whose string is to be |
| `pSTRING` | `POINTER TO STRING` | Pointer to the STRING variable |
| `pSrcString` | `POINTER TO STRING` | Pointer to the STRING variable whose string is to be |
| `sLiteral` | `STRING` | STRING to be converted. |
| `pDstUTF8` | `PVOID` | Pointer to the string in UTF-8 format (output string) |
| `pSrcSTRING` | `POINTER TO STRING` | Pointer to the STRING variable to be converted (input string) |
| `nDstSize` | `UDINT` | Size of the resulting variable (output string) in bytes. The |
| `pDstWSTRING` | `POINTER TO WSTRING` | Pointer to the converted WSTRING variable (output string) |
| `pSrcSTRING` | `POINTER TO STRING` | Pointer to the STRING variable to be converted (input string) |
| `nDstSize` | `UDINT` | Size of the resulting WSTRING variable (output string) |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sLiteral` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindAndSplitChar` | `BOOL` | The return value is TRUE if the separator was found and the division and |
| `HEXSTR_TO_DATA2` | `UDINT` | Converts a hexadecimal string into binary data. |
| `INSERT2` | `BOOL` | Inserts a string into another string after position nPos. Functions |
| `LEN2` | `UDINT` | Returns the number of characters in a string (length of the STRING). |
| `REPLACE2` | `BOOL` | Replaces nLen characters of a string with another string, starting at position |
| `sLiteral_TO_UTF8` | `STRING(511)` | Converts any string of the data type STRING into a string in |
| `STRING_TO_UTF8` | `BOOL` | Converts any string of a variable of the data type STRING to a string in |
| `STRING_TO_WSTRING2` | `BOOL` | Converts a variable of data type STRING into a variable of data |

**Description:** The function splits a string into two strings.

##### `STRNCPY` — §4.2.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDst` | `POINTER TO STRING 198` | Pointer to the copied STRING variable (input string) Functions |
| `pSrc` | `POINTER TO STRING` | Pointer to the STRING variable to be copied (output string) |
| `nDstSize` | `UDINT` | Size of the resulting STRING variable (output string) in bytes; the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `STRNCPY` | `BOOL` | Copies the string of a variable of the data type STRING and checks whether the |
| `nSrcLen` | `UDINT` | Length of the specified STRING variable to be copied |
| `nDstLen` | `UDINT` | Length of the copied WSTRING variables |

**Description:** The function copies the character string of a variable of the data type STRING and checks whether the character string was completely copied.

##### `UTF8_TO_STRING` — §4.2.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDstSTRING` | `POINTER TO STRING` | Pointer to the converted STRING variable (output string) |
| `pSrc UTF8` | `PVOID` | Pointer variable (input string) |
| `nDstSize` | `UDINT` | Size of the resulting STRING variable (output string) in |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UTF8_TO_STRING` | `BOOL` | The return value is TRUE if the conversion was successful. |
| `nDstLen` | `UDINT` | Actual length of the output string as number of characters |

**Description:** The function converts a string in UTF-8 format (pointer variable of the data type PVOID) into a string (variable) of the data type STRING.

##### `UTF8_TO_WSTRING` — §4.2.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDstWSTRING` | `POINTER TO WSTRING` | Pointer to the converted WSTRING variable (output string) |
| `pSrc UTF8` | `PVOID` | Pointer variable (input string) |
| `nDstSize` | `UDINT` | Size of the resulting STRING variable (output string) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UTF8_TO_WSTRING` | `BOOL` | The return value is TRUE if the conversion was successful. |
| `nDstLen` | `UDINT` | Actual length of the output string as number of characters |

**Description:** The function converts a string in UTF-8 format into a string (variable) of the data type WSTRING.

##### `UTF8Len` — §4.2.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pUTF8` | `PVOID` | Pointer to the null-terminated UTF-8 string |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UTF8Len` | `UDINT` | The return value returns the number of characters in the UTF-8 string. |
| `bASCII` | `BOOL` | TRUE if the UTF-8 characters are valid ASCII characters. |
| `nSize` | `UDINT` | Size of the string in bytes (without null termination). Depending |

**Description:** The function returns the number of characters in a UTF-8 string.

##### `WCHAR_TO_CHAR` — §4.2.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sTextIn` | `WSTRING(1)` | WSTRING variable to be converted. |
| `pSrcWString1` | `POINTER TO WSTRING` | Pointer to the first of the WSTRING variables to be concatenated (input string) |
| `pSrcWString2` | `POINTER TO WSTRING` | Pointer to the second of the WSTRING variables to be concatenated (input string) |
| `pDstWString` | `POINTER TO WSTRING` | Pointer to the resulting WSTRING variable following the concatenation (output string) |
| `nDstSize` | `UDINT` | Size of the resulting WSTRING variable (output string) Functions |
| `pWSTRING` | `POINTER TO WSTRING` | Pointer to the WSTRING variable |
| `wsLiteral` | `WSTRING` | WSTRING to be converted. |
| `pDstString` | `POINTER TO WSTRING` | Pointer to the converted STRING variable (output string) |
| `pSrcWString` | `POINTER TO WSTRING` | Pointer to the WSTRING variable to be converted (input string) |
| `nDstSize` | `UDINT` | Size of the resulting STRING variable (output string) in |
| `pDstUTF8` | `PVOID` | Pointer variable (output string) |
| `pSrcWSTRING` | `POINTER TO WSTRING` | Pointer to the WSTRING variable (input string) |
| `nDstSize` | `UDINT` | Size of the resulting variable (output string) in bytes; the SIZEOF() operator can be used for the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `wsLiteral` | `WSTRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WCHAR_TO_CHAR` | `STRING(1)` | Converts a variable of the data type WSTRING into a variable of |
| `WCONCAT2` | `BOOL` | Concatenates two strings of any length of data type WSTRING and checks |
| `WLEN2` | `UDINT` | Returns the number of characters in a Unicode string of data type WSTRING (length |
| `wsLiteral_TO_UTF8` | `STRING(511)` | Converts any string of the data type WSTRING into a string in |
| `WSTRING_TO_STRING2` | `BOOL` | Converts a variable of the data type WSTRING into a variable |
| `WSTRING_TO_UTF8` | `BOOL` | Converts a string of a variable of data type WSTRING into a |

**Description:** The function converts a variable of the data type WSTRING into a variable of the data type STRING (with null termination). The conversion is possible only if the WSTRING character corresponds to the STRING character. Otherwise no character is returned.

##### `WSTRNCPY` — §4.2.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDst` | `POINTER TO WSTRING` | Pointer to the copied WSTRING variable (input string) |
| `pSrc` | `POINTER TO WSTRING` | Pointer to the WSTRING variable to be copied (output string) |
| `nDstSize` | `UDINT` | Size of the resulting WSTRING variable (output string) in |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WSTRNCPY` | `BOOL` | Copies the string of a variable of the data type WSTRING and checks whether |
| `nSrcLen` | `UDINT` | Length of the specified WSTRING variables to be copied |
| `nDstLen` | `UDINT` | Length of the copied WSTRING variables |

**Description:** The function copies the string of a variable of the data type WSTRING and checks whether the string was completely copied.

#### Functions › Byte order converting functions

##### `Host Byte Order / Network Byte Order` — §4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `WORD` | Number to be converted. |
| `in` | `DWORD` | Number to be converted. |
| `in` | `T_ULARGE_INTEGER [} 376]` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HOST_TO_BE16` | `WORD` | — |
| `HOST_TO_BE32` | `DWORD` | — |
| `HOST_TO_BE64` | `T_ULARGE_INTEGER` | — |

**Description:** The byte order is fixed in network protocols. This is called the Network Byte Order. The natural byte order in the TwinCAT system is called the Host Byte Order. In most cases, the required Network Byte Order corresponds to the Big Endian Format (MOTOROLA). However, the TwinCAT PLC system uses the Little Endian Format (Intel). So that the error-free exchange of data between the TwinCAT PLC system and a different platform can take place, the byte order in the application program must be converted accordingly.

##### `HOST_TO_BE64EX` — §4.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LWORD` | Number to be converted. |
| `in` | `T_UHUGE_INTEGER [} 376]` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HOST_TO_BE64EX` | `LWORD` | — |
| `HOST_TO_BE128` | `T_UHUGE_INTEGER` | — |

**Description:** The function performs a host-to-network conversion of a 64-bit number (“native” type: LWORD). See also: Byte Order [} 207].

##### `BE16_TO_HOST` — §4.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `WORD` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BE16_TO_HOST` | `WORD` | — |

**Description:** The function performs a network-to-host conversion of a 16-bit number. See also: Byte Order [} 207].

##### `BE32_TO_HOST` — §4.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DWORD` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BE32_TO_HOST` | `DWORD` | — |

**Description:** The function performs a network-to-host conversion of a 32-bit number. See also: Byte Order [} 207].

##### `BE64_TO_HOST` — §4.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_ULARGE_INTEGER [} 376]` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BE64_TO_HOST` | `T_ULARGE_INTEGER` | — |

**Description:** The function performs a network-to-host conversion of a 64-bit number (“legacy” type: T_ULARGE_INTEGER). See also: Byte Order [} 207].

##### `BE64_TO_HOSTEX` — §4.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LWORD` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BE64_TO_HOSTEX` | `LWORD` | — |

**Description:** The function performs a network-to-host conversion of a 64-bit number (“native” type: LWORD). See also: Byte Order [} 207].

##### `BE128_TO_HOST` — §4.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_UHUGE_INTEGER [} 376]` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BE128_TO_HOST` | `T_UHUGE_INTEGER` | — |

**Description:** The function performs a network-to-host conversion of a 128-bit number (“legacy” type: T_UHUGE_INTEGER [} 376]). See also: Byte Order [} 207].

#### Functions › FLOAT functions › [obsolete]

##### `BOOL_TO_FLOAT` — §4.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `BOOL` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL_TO_FLOAT LREAL` | `.` | — |

##### `DINT_TO_FLOAT` — §4.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DINT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DINT_TO_FLOAT` | `FLOAT` | — |

##### `FLOAT_TO_BOOL` — §4.4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LREAL` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FLOAT_TO_BOOL` | `BOOL` | — |

##### `FLOAT_TO_DINT` — §4.4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `FLOAT Number to be converted.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FLOAT_TO_DINT` | `DINT` | — |

##### `FLOAT_TO_INT` — §4.4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `FLOAT Number to be converted.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FLOAT_TO_INT` | `INT` | — |

##### `FLOAT_TO_SINT` — §4.4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `FLOAT Number to be converted.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FLOAT_TO_SINT` | `SINT` | — |

##### `FLOAT_TO_STRING` — §4.4.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `FLOAT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FLOAT_TO_STRING` | `STRING` | — |

##### `FLOAT_TO_TIME` — §4.4.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `FLOAT Number to be converted.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FLOAT_TO_TIME` | `TIME` | — |

##### `FLOAT_TO_UDINT` — §4.4.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `FLOAT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FLOAT_TO_UDINT` | `UDINT` | — |

##### `FLOAT_TO_UINT` — §4.4.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `FLOAT Number to be converted.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FLOAT_TO_UINT` | `UINT` | — |

##### `INT_TO_FLOAT` — §4.4.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `INT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `INT_TO_FLOAT` | `FLOAT` | — |

##### `SINT_TO_FLOAT` — §4.4.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `SINT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SINT_TO_FLOAT` | `FLOAT` | — |

##### `TIME_TO_FLOAT` — §4.4.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `TIME` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TIME_TO_FLOAT` | `FLOAT` | — |

##### `UDINT_TO_FLOAT` — §4.4.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `UDINT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT_TO_FLOAT` | `FLOAT` | — |

##### `UINT_TO_FLOAT` — §4.4.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `UINT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UINT_TO_FLOAT` | `FLOAT` | — |

#### Functions › FLOAT functions

##### `LrealIsFinite` — §4.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `REFERENCE TO LREAL` | Transfers the value being tested. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LrealIsFinite` | `BOOL` | If the return value is TRUE, then |

**Description:** This function tests whether a value is finite. If the return value is TRUE, then the value is finite.

##### `LrealIsNaN` — §4.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `REFERENCE TO LREAL` | Transfers the value being tested. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LrealIsNaN` | `BOOL` | If the return value is TRUE, the |

**Description:** This function tests whether a value is NaN (Not a Number). If the return value is TRUE, the value is NaN.

##### `RealIsFinite` — §4.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `REFERENCE TO REAL` | Transfers the value being tested. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RealIsFinite` | `BOOL` | If the return value is TRUE, then |

**Description:** This function tests whether a value is finite. If the return value is TRUE, then the value is finite.

##### `RealIsNaN` — §4.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `REFERENCE TO REAL` | Transfers the value being tested. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RealIsNaN` | `BOOL` | If the return value is TRUE, the |

**Description:** This function tests whether a value is NaN (Not a Number). If the return value is TRUE, the value is NaN.

#### Functions › LCOMPLEX functions

##### `LcomplexIsNaN` — §4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Z` | `REFERENCE TO LCOMPLEX` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LcomplexIsNaN` | `BOOL` | — |

**Description:** The function returns TRUE if the argument of type LCOMPLEX has an undefined value (NaN).

##### `LcomplexAbs` — §4.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Z` | `LCOMPLEX Functions` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LcomplexAbs` | `LREAL` | — |

**Description:** The function returns the absolute value of the complex number transferred.

#### Functions › P[TYPE]_TO_[TYPE] converting functions

##### `PBOOL_TO_BOOL` — §4.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO BOOL` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PBOOL_TO_BOOL` | `BOOL` | — |

**Description:** The function returns the content of a BOOL pointer variable.

##### `PBYTE_TO_BYTE` — §4.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO BYTE 224` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PBYTE_TO_BYTE` | `BYTE` | — |

**Description:** The function returns the content of a BYTE pointer variable.

##### `PDATE_TO_DATE` — §4.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO DATE` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PDATE_TO_DATE` | `DATE` | — |

**Description:** The function returns the content of a DATE pointer variable.

##### `PDINT_TO_DINT` — §4.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO DINT` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PDINT_TO_DINT` | `DINT` | — |

**Description:** The function returns the content of a DINT pointer variable.

##### `PDT_TO_DT` — §4.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO DT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PDT_TO_DT` | `DT` | — |

**Description:** The function returns the content of a DT pointer variable.

##### `PDWORD_TO_DWORD` — §4.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO DWORD 226` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PDWORD_TO_DWORD` | `DWORD` | — |

**Description:** The function returns the content of a DWORD pointer variable.

##### `PHUGE_TO_HUGE` — §4.6.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO T_HUGE_INTEGER` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PHUGE_TO_HUGE` | `T_HUGE_INTEGER` | — |

**Description:** The function returns the content of a T_HUGE_INTEGER [} 375] pointer variable.

##### `PINT_TO_INT` — §4.6.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO INT` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PINT_TO_INT` | `INT` | — |

**Description:** The function returns the content of an INT pointer variable.

##### `PLARGE_TO_LARGE` — §4.6.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO T_LARGE_INTEGER` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PLARGE_TO_LARGE` | `T_LARGE_INTEGER` | — |

**Description:** The function returns the content of a T_LARGE_INTEGER [} 375] pointer variable.

##### `PLINT_TO_LINT` — §4.6.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO LINT 228` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PLINT_TO_LINT` | `LINT` | — |

**Description:** The function returns the content of a LINT pointer variable.

##### `PLREAL_TO_LREAL` — §4.6.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO LREAL` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PLREAL_TO_LREAL` | `LREAL` | — |

**Description:** The function returns the content of a LREAL pointer variable.

##### `PLWORD_TO_LWORD` — §4.6.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO LWORD` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PLWORD_TO_LWORD` | `LWORD` | — |

**Description:** The function returns the content of a LWORD pointer variable.

##### `PMAXSTRING_TO_MAXSTRING` — §4.6.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO T_MaxString` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PMAXSTRING_TO_MAXSTRING` | `T_MaxString` | — |

**Description:** The function returns the content of a T_MaxString pointer variable.

##### `PREAL_TO_REAL` — §4.6.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO REAL 230` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PREAL_TO_REAL` | `REAL` | — |

**Description:** The function returns the content of a REAL pointer variable.

##### `PSINT_TO_SINT` — §4.6.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO SINT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PSINT_TO_SINT` | `SINT` | — |

**Description:** The function returns the content of a SINT pointer variable.

##### `PSTRING_TO_STRING` — §4.6.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO String` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PSTRING_TO_STRING` | `STRING` | — |

**Description:** The function returns the content of a STRING pointer variable.

##### `PTIME_TO_TIME` — §4.6.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO TIME` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PTIME_TO_TIME` | `TIME` | — |

**Description:** The function returns the content of a TIME pointer variable.

##### `PTOD_TO_TOD` — §4.6.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO TOD 232` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PTOD_TO_TOD` | `TOD` | — |

**Description:** The function returns the content of a TOD pointer variable.

##### `PUDINT_TO_UDINT` — §4.6.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO UDINT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PUDINT_TO_UDINT` | `UDINT` | — |

**Description:** The function returns the content of a UDINT pointer variable.

##### `PUHUGE_TO_UHUGE` — §4.6.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO T_UHUGE_INTEGER` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PUHUGE_TO_UHUGE` | `_UHUGE_INTEGER` | — |

**Description:** The function returns the content of a T_UHUGE_INTEGER [} 376] pointer variable.

##### `PUINT_TO_UINT` — §4.6.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO UINT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PUINT_TO_UINT` | `UINT` | — |

**Description:** The function returns the content of a UINT pointer variable.

##### `PULARGE_TO_ULARGE` — §4.6.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO T_ULARGE_INTEGER 234` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PULARGE_TO_ULARGE` | `_ULARGE_INTEGER` | — |

**Description:** The function returns the content of a T_ULARGE_INTEGER [} 376] pointer variable.

##### `PULINT_TO_ULINT` — §4.6.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO ULINT` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PULINT_TO_ULINT` | `ULINT` | — |

**Description:** The function returns the content of a ULINT pointer variable.

##### `PUSINT_TO_USINT` — §4.6.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO USINT` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PUSINT_TO_USINT` | `USINT` | — |

**Description:** The function returns the content of a USINT pointer variable.

##### `PWORD_TO_WORD` — §4.6.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `POINTER TO WORD` | Number to be converted. |
| `in` | `POINTER TO T_ULARGE_INTEGER 236` | Number to be converted. Functions |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PWORD_TO_WORD` | `WORD` | — |
| `PUINT64_TO_UINT64` | `T_ULARGE_INTEGER` | — |

**Description:** The function returns the content of a WORD pointer variable.

#### Functions › 16 bit fixed point number functions (signed)

##### `FIX16_TO_LREAL` — §4.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_FIX16 [} 373]` | The fixed-point number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FIX16_TO_LREAL` | `LREAL` | — |

**Description:** Converts a signed 16-bit fixed-point number into a floating-point number of the type: LREAL.

##### `FIX16_TO_WORD` — §4.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_FIX16 [} 373]` | The fixed-point number to be converted |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FIX16_TO_WORD` | `WORD Functions` | — |

**Description:** This function converts a 16-bit fixed-point number to a WORD variable (the WORD variable contains the digits and decimal places for the fixed-point number).

##### `FIX16Add` — §4.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `augend` | `T_FIX16 [} 373]` | The first summand |
| `addend` | `T_FIX16 [} 373]` | The second summand |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FIX16Add` | `T_FIX16 [} 373]` | — |

**Description:** This function adds two signed 16-bit fixed-point numbers. The resolution (number of decimal places) of the numbers does not have to be the same. The resolution of the number with the higher number of decimal places is reduced before addition. This means that the decimal places of the number with the higher resolution are truncated. The result of the addition is a signed 16-bit fixed-point number.

##### `FIX16Align` — §4.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_FIX16 [} 373] Fixed-point number whose resolution is to be modified` | — |
| `n` | `BYTE(0..15)` | Fixed-point number whose resolution is to be modified |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FIX16Align` | `T_FIX16 [} 373]` | — |

**Description:** This function can be used to change the resolution (number of decimal places) of a signed 16-bit fixed-point number. The function supplies the new fixed-point number as return parameter.

##### `FIX16Div` — §4.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dividend` | `T_FIX16 [} 373]` | Number that is divided |
| `divisor` | `T_FIX16 [} 373]` | Number by which the dividend is divided |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FIX16Div` | `T_FIX16 [} 373]` | — |

**Description:** This function divides two signed 16-bit fixed-point numbers. The resolution (number of decimal places) of the numbers does not have to be the same. The resolution of the number with the higher number of decimal places is reduced before division. This means that the decimal places of the number with the higher resolution are truncated. The result of the division is a signed 16-bit fixed-point number.

##### `FIX16Mul` — §4.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `multiA` | `T_FIX16 [} 373]` | The first multiplier |
| `multiB` | `T_FIX16 [} 373]` | The second multiplier |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FIX16Mul` | `T_FIX16 [} 373]` | — |

**Description:** This function multiplies two signed 16-bit fixed-point numbers. The resolution (number of decimal places) of the numbers does not have to be the same. The resolution of the number with the higher number of decimal places is reduced before multiplication. This means that the decimal places of the number with the higher resolution are truncated. The result of the multiplication is a signed 16-bit fixed-point number.

##### `FIX16Sub` — §4.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Functions` | `` | — |
| `minuend` | `T_FIX16 [} 373]` | Number from which a value is subtracted |
| `subtrahend` | `T_FIX16 [} 373]` | Number that is subtracted |
| `in` | `LREAL` | The LREAL number to be converted. |
| `n` | `WORD(0..15)` | Number of required decimal places. |
| `in` | `WORD` | 16-bit fixed-point number |
| `n` | `WORD(0..15)` | number of fractional bits |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FIX16Sub` | `T_FIX16 [} 373]` | — |
| `LREAL_TO_FIX16` | `T_FIX16 [} 373]` | — |
| `WORD_TO_FIX16` | `T_FIX16 [} 373]` | — |

**Description:** This function subtracts two signed 16-bit fixed-point numbers. The resolution (number of decimal places) of the numbers does not have to be the same. The resolution of the number with the higher number of decimal places is reduced before subtraction. This means that the decimal places of the number with the higher resolution are truncated. The result of the subtraction is a signed 16-bit fixed-point number.

#### Functions › 64 bit functions (signed)

##### `INT64_TO_LREAL` — §4.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_LARGE_INTEGER` | — |
| `i64a` | `T_LARGE_INTEGER` | — |
| `i64b` | `T_LARGE_INTEGER 244 Functions` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `INT64_TO_LREAL` | `LREAL` | — |
| `Int64Add64` | `T_LARGE_INTEGER` | — |

**Description:** The function converts a TwinCAT 2 signed 64-bit number (“legacy” type: T_LARGE_INTEGER [} 375]) into a floating-point number of type LREAL.

##### `Int64Add64Ex` — §4.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `augend` | `T_LARGE_INTEGER` | — |
| `addend` | `T_LARGE_INTEGER` | — |
| `/` | `Inputs/outputs` | — |
| `bOV` | `BOOL` | Arithmetic overflow. TRUE => overflow, FALSE => no overflow. |
| `i64a` | `T_LARGE_INTEGER` | If return parameter -1: i64a less than i64b |
| `i64b` | `T_LARGE_INTEGER` | 0: i64a identical to i64b 1: i64a greater than i64b |
| `Return parameter` | `Description` | — |
| `-1` | `i64a less than i64b` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOV` | `BOOL;(* TRUE => arithmetic overflow, FALSE => no overflow *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Int64Add64Ex` | `T_LARGE_INTEGER` | — |
| `Int64Cmp64` | `DINT` | — |

**Description:** The function adds two TwinCAT 2 signed 64-bit numbers (“legacy” type: T_LARGE_INTEGER [} 375]). The result is a signed 64-bit number.

##### `Int64Div64Ex` — §4.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dividend` | `T_LARGE_INTEGER` | Number that is divided |
| `divisor` | `T_LARGE_INTEGER 246` | Number by which the dividend is divided Functions |
| `/` | `Inputs/outputs` | — |
| `remainder` | `T_LARGE_INTEGER` | Rest |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `remainder` | `T_LARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Int64Div64Ex` | `T_LARGE_INTEGER` | — |

**Description:** The function divides two TwinCAT 2 signed 64-bit numbers (“legacy” type: T_LARGE_INTEGER [} 375]). The result is a signed 64-bit number.

##### `Int64IsZero` — §4.8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `i64` | `T_LARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Int64isZero` | `BOOL` | — |

**Description:** The function returns TRUE if the TwinCAT 2 signed 64-bit number (“legacy” type: T_LARGE_INTEGER [} 375]) has the value zero.

##### `Int64Negate` — §4.8.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `i64` | `T_LARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Int64Negate` | `T_LARGE_INTEGER Functions` | — |

**Description:** The function negates a TwinCAT 2 signed 64-bit number (“legacy” type: T_LARGE_INTEGER [} 375]).

##### `Int64Not` — §4.8.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `i64` | `T_LARGE_INTEGER` | — |
| `i64a` | `T_LARGE_INTEGER` | — |
| `i64b` | `T_LARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Int64Not` | `T_LARGE_INTEGER` | — |
| `Int64Sub64` | `T_LARGE_INTEGER 248 Functions` | — |

**Description:** Bitwise NOT of a TwinCAT 2 signed 64-bit number (“legacy” type: T_LARGE_INTEGER [} 375]). The result is a signed 64-bit number.

##### `LARGE_INTEGER` — §4.8.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dwHighPart` | `DWORD` | The upper 32 bits |
| `dwLowPart` | `DWORD` | The lower 32 bits |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LARGE_INTEGER` | `T_LARGE_INTEGER` | — |

**Description:** The function initializes a TwinCAT 2 signed 64-bit number (“legacy” type: T_LARGE_INTEGER [} 375]).

##### `LARGE_TO_LINT` — §4.8.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_LARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LARGE_TO_LINT` | `LINT` | — |

**Description:** The function converts a TwinCAT 2 signed 64-bit number (“legacy” type: T_LARGE_INTEGER [} 375]) into a TwinCAT 3 signed 64-bit number (“native” type).

##### `LARGE_TO_ULARGE` — §4.8.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_LARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LARGE_TO_ULARGE` | `T_ULARGE_INTEGER` | — |

**Description:** The function converts a TwinCAT 2 signed 64-bit number (“legacy” type: T_LARGE_INTEGER [} 375]) into a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]).

##### `LINT_TO_LARGE` — §4.8.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LINT` | — |
| `in` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LINT_TO_LARGE` | `T_LARGE_INTEGER` | — |
| `LREAL_TO_INT64` | `T_LARGE_INTEGER` | — |

**Description:** The function converts a TwinCAT 3 signed 64-bit number (“native” type) into a TwinCAT 2 signed 64-bit number (“legacy” type: T_LARGE_INTEGER [} 375]).

##### `ULARGE_TO_LARGE` — §4.8.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ULARGE_TO_LARGE` | `T_LARGE_INTEGER` | — |

**Description:** The function converts a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]) into a TwinCAT 2 signed 64-bit number (“legacy” type: T_LARGE_INTEGER [} 375]).

#### Functions

##### `64 bit integer functions (unsigned)` — §4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LREAL` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LREAL_TO_UINT64` | `T_ULARGE_INTEGER` | — |

**Description:** The function converts a LREAL number into a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]).

#### Functions › 64 bit integer functions (unsigned)

##### `LWORD_TO_ULARGE` — §4.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LWORD` | Number to be converted. |
| `in` | `STRING(21)` | Number to be converted. |
| `ui32a` | `DWORD` | — |
| `ui32b` | `DWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LWORD_TO_ULARGE` | `T_ULARGE_INTEGER` | — |
| `STRING_TO_UINT64` | `T_ULARGE_INTEGER` | — |
| `UInt32x32To64` | `T_ULARGE_INTEGER` | — |

**Description:** 252

##### `UINT64_TO_LREAL` — §4.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_ULARGE_INTEGER` | Number to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UINT64_TO_LREAL` | `LREAL` | — |

**Description:** The function converts a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]) into a floating-point number of type LREAL.

##### `UINT64_TO_STRING` — §4.9.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_ULARGE_INTEGER` | Number to be converted. |
| `ui64a` | `T_ULARGE_INTEGER` | — |
| `ui64b` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UINT64_TO_STRING` | `STRING(21)` | — |
| `UInt64Add64` | `T_ULARGE_INTEGER` | — |

**Description:** 254

##### `UInt64Add64Ex` — §4.9.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `augend` | `T_ULARGE_INTEGER` | — |
| `addend` | `T_ULARGE_INTEGER` | — |
| `/` | `Inputs/outputs` | — |
| `bOV` | `BOOL` | Arithmetic overflow. TRUE => overflow, FALSE => no overflow. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOV` | `BOOL;(* TRUE => arithmetic overflow, FALSE => no overflow *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Add64Ex` | `T_ULARGE_INTEGER` | — |

**Description:** The function adds two TwinCAT 2 unsigned 64-bit numbers (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `UInt64And` — §4.9.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64a` | `T_ULARGE_INTEGER` | — |
| `ui64b` | `T_ULARGE_INTEGER` | — |
| `ui64a` | `T_ULARGE_INTEGER` | -1: ui64a less than ui64b |
| `ui64b` | `T_ULARGE_INTEGER` | 0: ui64a identical to ui64b 1: ui64a greater than ui64b Return parameter Description |
| `-1` | `ui64a less than ui64b` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64And` | `T_ULARGE_INTEGER 256 Functions` | — |
| `UInt64Cmp64` | `DINT` | — |

**Description:** Bitwise AND of two TwinCAT 2 unsigned 64-bit numbers (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `UInt64Div16Ex` — §4.9.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dividend` | `T_ULARGE_INTEGER` | Number that is divided |
| `divisor` | `WORD` | Number by which the dividend is divided |
| `/` | `Inputs/outputs` | — |
| `remainder` | `T_ULARGE_INTEGER` | Rest |
| `dividend` | `T_ULARGE_INTEGER` | Number that is divided |
| `divisor` | `T_ULARGE_INTEGER` | Number by which the dividend is divided |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `remainder` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Div16Ex` | `T_ULARGE_INTEGER` | — |
| `UInt64Div64` | `T_ULARGE_INTEGER 258 Functions` | — |

**Description:** The function divides a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]) by a 16-bit unsigned number. The result is an unsigned 64-bit number.

##### `UInt64Div64Ex` — §4.9.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dividend` | `T_ULARGE_INTEGER` | Number that is divided |
| `divisor` | `T_ULARGE_INTEGER` | Number by which the dividend is divided |
| `/` | `Inputs/outputs` | — |
| `remainder` | `T_ULARGE_INTEGER` | Rest |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `remainder` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Div64Ex` | `T_ULARGE_INTEGER` | — |

**Description:** The function divides two TwinCAT 2 unsigned 64-bit numbers (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `UInt64isZero` — §4.9.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64isZero` | `BOOL` | — |

**Description:** The function returns TRUE if the TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]) has the value zero.

##### `UInt64Limit` — §4.9.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64min` | `T_ULARGE_INTEGER` | — |
| `ui64in` | `T_ULARGE_INTEGER` | — |
| `ui64max` | `T_ULARGE_INTEGER 260 Functions` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Limit` | `T_ULARGE_INTEGER` | — |

**Description:** Limitation. The result is a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]).

##### `UInt64Max` — §4.9.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64a` | `T_ULARGE_INTEGER` | — |
| `ui64b` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Max` | `T_ULARGE_INTEGER` | — |

**Description:** Maximum function. Returns the larger of two values (“legacy” type: T_ULARGE_INTEGER [} 376]).

##### `UInt64Min` — §4.9.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64a` | `T_ULARGE_INTEGER Functions` | — |
| `ui64b` | `T_ULARGE_INTEGER` | — |
| `dividend` | `T_ULARGE_INTEGER` | Number that is divided |
| `divisor` | `T_ULARGE_INTEGER` | Number by which the dividend is divided |
| `multiplicand` | `T_ULARGE_INTEGER` | — |
| `multiplier` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Min` | `T_ULARGE_INTEGER` | — |
| `UInt64Mod64` | `T_ULARGE_INTEGER` | — |
| `UInt64Mul64` | `T_ULARGE_INTEGER 262 Functions` | — |

**Description:** Minimum function. Returns the smaller of two values (“legacy” type: T_ULARGE_INEGER [} 376]).

##### `UInt64Mul64Ex` — §4.9.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `multiplicand` | `T_ULARGE_INTEGER` | — |
| `multiplier` | `T_ULARGE_INTEGER` | — |
| `/` | `Inputs/outputs` | — |
| `bOV` | `BOOL` | Arithmetic overflow. TRUE => overflow, FALSE => no overflow. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOV` | `BOOL;(* TRUE => Arithmetic overflow, FALSE => no overflow *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Mul64Ex` | `T_ULARGE_INTEGER` | — |

**Description:** The function multiplies two TwinCAT 2 unsigned 64-bit numbers (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `UInt64Not` — §4.9.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Not` | `T_ULARGE_INTEGER` | — |

**Description:** Bitwise NOT of a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `UInt64Or` — §4.9.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64a` | `T_ULARGE_INTEGER` | — |
| `ui64b` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Or` | `T_ULARGE_INTEGER` | — |

**Description:** Bitwise OR of two TwinCAT 2 unsigned 64-bit numbers (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `UInt64Rol` — §4.9.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64` | `T_ULARGE_INTEGER` | — |
| `n` | `DWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Rol` | `T_ULARGE_INTEGER` | — |

**Description:** Bitwise left rotation of a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `UInt64Ror` — §4.9.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64` | `T_ULARGE_INTEGER` | — |
| `n` | `DWORD Functions` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Ror` | `T_ULARGE_INTEGER` | — |

**Description:** Bitwise right rotation of a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `UInt64Shl` — §4.9.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64` | `T_ULARGE_INTEGER` | — |
| `n` | `DWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Shl` | `T_ULARGE_INTEGER` | — |

**Description:** Bitwise left-shift of a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `UInt64Shr` — §4.9.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `266 Functions` | `` | — |
| `ui64` | `T_ULARGE_INTEGER` | — |
| `n` | `DWORD` | — |
| `ui64a` | `T_ULARGE_INTEGER` | — |
| `ui64b` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Shr` | `T_ULARGE_INTEGER` | — |
| `UInt64Sub64` | `T_ULARGE_INTEGER` | — |

**Description:** Bitwise right-shift of a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `UInt64Xor` — §4.9.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ui64a` | `T_ULARGE_INTEGER` | — |
| `ui64b` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UInt64Xor` | `T_ULARGE_INTEGER Functions` | — |

**Description:** Bitwise XOR of two TwinCAT 2 unsigned 64-bit numbers (“legacy” type: T_ULARGE_INTEGER [} 376]). The result is an unsigned 64-bit number.

##### `ULARGE_INTEGER` — §4.9.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dwHighPart` | `DWORD` | Upper 32 bits |
| `dwLowPart` | `DWORD` | Lower 32 bits |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ULARGE_INTEGER` | `T_ULARGE_INTEGER` | — |

**Description:** The function initializes a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]).

##### `ULARGE_TO_ULINT` — §4.9.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ULARGE_TO_ULINT` | `ULINT` | — |

**Description:** The function converts a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]) into a TwinCAT 3 unsigned 64-bit number (“native” type).

##### `ULARGE_TO_LWORD` — §4.9.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ULARGE_TO_LWORD` | `LWORD` | — |

**Description:** The function converts a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]) into a TwinCAT 3 unsigned 64-bit number (“native” type).

#### Functions › T_Arg help functions

##### `F_ARGCMP` — §4.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `typeSafe` | `BOOL` | If TRUE => identical types can be compared (type-safe comparison). FALSE |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ARGCMP` | `DINT` | — |

**Description:** Functions

##### `F_ARGCPY` — §4.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `typeSafe` | `BOOL` | If TRUE => identical types can be compared (type-safe comparison). FALSE |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `dest` | `T_Arg` | — |
| `src` | `T_Arg` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ARGCPY` | `UDINT 270 Functions` | — |

**Description:** This function copies the value of a variable of type T_Arg into another variable and supplies the number of successfully copied data bytes as return parameter.

##### `F_ARGISZERO` — §4.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `arg` | `T_Arg` | Variable to be checked (type : T_Arg [} 372]). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ARGISZERO` | `BOOL` | — |

**Description:** This function returns TRUE if one of the T_Arg member variables has the value zero or was not initialized.

##### `F_BIGTYPE` — §4.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pData` | `POINTER TO BYTE` | Address pointer (can be determined with the ADR operator). |
| `cbLen` | `DWORD` | Number of bytes occupied in the memory (can be determined with the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_BIGTYPE` | `T_Arg` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a Struct or Array variable.

##### `F_BOOL` — §4.10.5

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_BOOL` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `BOOL` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a BOOL variable.

##### `F_BYTE` — §4.10.6

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `BYTE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_BYTE` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `BYTE` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a BYTE variable.

##### `F_DINT` — §4.10.7

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_DINT` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `DINT` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a DINT variable.

##### `F_DWORD` — §4.10.8

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_DWORD` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `DWORD` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a DWORD variable.

##### `F_HUGE` — §4.10.9

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_HUGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_HUGE` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `T_HUGE_INTEGER` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a T_HUGE_INTEGER [} 375] variable (signed 128-bit integer, TwinCAT 2 “legacy” type).

##### `F_INT` — §4.10.10

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `INT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_INT` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `INT` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a INT variable.

##### `F_LARGE` — §4.10.11

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_LARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_LARGE` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `T_LARGE_INTEGER` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a T_LARGE_INTEGER [} 375] variable (signed 64-bit integer, TwinCAT 2 “legacy” type).

##### `F_LINT` — §4.10.12

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_LINT` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `LINT` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a LINT variable.

##### `F_LREAL` — §4.10.13

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_LREAL` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `LREAL` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a LREAL variable.

##### `F_LWORD` — §4.10.14

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_LWORD` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `LWORD` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a LWORD variable.

##### `F_REAL` — §4.10.15

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `REAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_REAL` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `REAL` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a REAL variable.

##### `F_SINT` — §4.10.16

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `SINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_SINT` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `SINT` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a SINT variable.

##### `F_STRING` — §4.10.17

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_MaxString` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_STRING` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `T_MaxString` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a T_MaxString variable.

##### `F_STRINGEx` — §4.10.18

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_STRINGEx` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `STRING` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) about a STRING variable. Unlike the F_STRING [} 278] function, the length of the transferred STRING variable is arbitrary.

##### `F_UDINT` — §4.10.19

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_UDINT` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `UDINT` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a UDINT variable.

##### `F_UHUGE` — §4.10.20

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_UHUGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_UHUGE` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `T_UHUGE_INTEGER` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a T_UHUGE_INTEGER [} 376] variable (unsigned 128-bit integer, TwinCAT 2 “legacy” type).

##### `F_UINT` — §4.10.21

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `UINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_UINT` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `UINT` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a UINT variable.

##### `F_ULARGE` — §4.10.22

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_ULARGE_INTEGER` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ULARGE` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `T_ULARGE_INTEGER` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a T_LARGE_INTEGER [} 376] variable (unsigned 64-bit integer, TwinCAT 2 “legacy” type).

##### `F_ULINT` — §4.10.23

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `ULINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ULINT` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `ULINT` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a ULINT variable.

##### `F_USINT` — §4.10.24

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `USINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_USINT` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `USINT` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a USINT variable.

##### `F_WORD` — §4.10.25

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `WORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_WORD` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `WORD` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a WORD variable.

##### `F_PVOID` — §4.10.26

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `PVOID` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_PVOID` | `T_Arg` | — |
| `/` | `Inputs/outputs` | — |
| `in` | `PVOID` | — |

**Description:** Helper function that returns a structure with information (type: T_Arg [} 372]) on a PVOID variable.

##### `IsFinite` — §4.10.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `T_Arg` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsFinite` | `BOOL` | — |

**Description:** The function IsFinite() returns TRUE, if its argument has a finite value (INF < x < +INF). The function returns FALSE, if the argument is infinite or NaN (NaN = Not a number). IsFinite() checks whether the formatting of an LREAL or REAL variable complies with IEEE.

#### Functions › [Obsolete] › Time functions

##### `DT_TO_FILETIME` — §4.11.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DTIN` | `DT` | The date and time to be converted, in DATE_AND_TIME format. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DT_TO_FILETIME` | `T_FILETIME [} 372]` | — |

**Description:** The function "DT_TO_FILETIME" can be used to convert a PLC variable in DATE_AND_TIME format (DT) to FILETIME format (64-bit).

##### `F_TranslateFileTimeBias` — §4.11.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_FILETIME` | Input time that is to be converted (type: T_FILETIME [} 372]). |
| `bias` | `DINT` | Difference between UTC time and local time in minutes (positive or negative |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_TranslateFileTimeBias` | `T_FILETIME [} 372] Functions` | — |

**Description:** This function converts the input time to the time in another time zone based on the specified bias time shift. The function can be used to convert the local time to UTC time (Universal Time Coordinates) and vice versa, for example.

##### `FILETIME_TO_DT` — §4.11.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fileTime` | `T_FILETIME [} 372]` | The time to be converted in the FILETIME format (type: T_FILETIME). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FILETIME_TO_DT` | `DT` | — |

**Description:** The function "FILETIME_TO_DT" converts the time in FILETIME format to DATE_AND_TIME format (DT). The DT format has a smaller value range than the FILETIME format and only offers second accuracy. For this reason the FILETIME value to be converted is limited. The permitted minimum corresponds to value DT#1970-01-01-00:00:00 and the maximum to value DT#2106-02-06-06:28:15. Milliseconds are not considered in the conversion and are rounded down to the DATE_AND_TIME return value accordingly.

##### `FILETIME_TO_SYSTEMTIME` — §4.11.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fileTime` | `T_FILETIME [} 372]` | The time to be converted in the FILETIME format (type: T_FILETIME). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FILETIME_TO_SYSTEMTIME` | `TIMESTRUCT [} 377]` | — |

**Description:** The function "FILETIME_TO_SYSTEMTIME" converts the time in FILETIME format into the "readable" SYSTEMTIME format. The conversion fails if the most significant bit of the 64-bit fileTime variable is set. In this case the TIMESTRUCT member variables have the value zero.

##### `SYSTEMTIME_TO_FILETIME` — §4.11.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `systemTime` | `TIMESTRUCT [} 377] 288` | TIMESTRUCT). Structure with the Windows system time to be converted (type: Functions |
| `Return parameter` | `Description` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SYSTEMTIME_TO_FILETIME` | `T_FILETIME [} 372]` | — |

**Description:** The function can be used to convert the Windows system time structure into the FILETIME format. The day of the week wDayOfWeek of the SystemTime variable is ignored. The system time year must be greater than 1601 and less than 30827.

#### Functions › [Obsolete]

##### `FUNCTION F_GetVersionTcUtilities` — §4.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | Version element to be read. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GetVersionTcUtilities` | `UINT` | — |

**Description:** This function is obsolete and should not be used. Please use the global constant: stLibVersion_Tc2_Utilities [} 378] to read version information from the PLC library.

##### `FLOATIsFinite` — §4.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FLOATIsFinite` | `BOOL Functions` | — |

##### `FLOATIsNaN` — §4.11.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `x` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FLOATIsNaN` | `BOOL` | — |

#### Functions

##### `ARG_TO_CSVFIELD` — §4.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_Arg` | PLC source variable whose value is to be converted into a data |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ARG_TO_CSVFIELD` | `UDINT` | If successful, this returns the length of the converted data as the |

**Description:** The function converts the value of a PLC variable to a data field in CSV format. Single quotation marks in the source file are replaced with double quotation marks. If the bQM parameter is set (QM = quotation marks) the outer quotation marks (enclosing the CSV data field) are also added. If successful the function returns the length of the converted data as the result. The function returns zero if a conversion error occurred or in the event of missing data. The result is written into the byte buffer provided. The application must ensure that the buffer is large enough to contain the result.

##### `BIC_TO_BTN` — §4.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sBICValue` | `STRING` | This input must contain the Beckhoff Identification Code (BIC) that the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BIC_TO_BTN` | `STRING(9)` | Returns the Beckhoff Traceability |

**Description:** The function BIC_TO_BTN extracts the Beckhoff Traceability Number (BTN) from the Beckhoff Identification Code (BIC) in sBICValue and returns it as return value. Spaces at the end of the BTN are automatically removed. If no BTN is found, then an empty string is returned.

##### `BYTE_TO_BINSTR` — §4.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `BYTE` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BYTE_TO_BINSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a binary string (base 2).

##### `BYTE_TO_DECSTR` — §4.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `BYTE` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BYTE_TO_DECSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a decimal string (base 10).

##### `BYTE_TO_HEXSTR` — §4.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `BYTE 294` | The decimal number to be converted. Functions |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BYTE_TO_HEXSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a hexadecimal string (base 16).

##### `BYTE_TO_LREALEX` — §4.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `BYTE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BYTE_TO_LREALEX` | `LREAL` | — |

**Description:** The conversion of unsigned numbers into floating-point numbers of the type LREAL is not supported in TwinCAT 2 on the Arm® platform. Unsigned numbers with the most significant bit set may possibly be implicitly converted into negative floating-point numbers. The function described here allows the explicit conversion of the type BYTE into a positive floating-point number of the type LREAL in TwinCAT 2 (even if the most significant bit was set and without compiler warning). You only need this function to be able to compile converted TwinCAT 2 projects without changes in TwinCAT 3.

##### `BYTE_TO_OCTSTR` — §4.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `BYTE` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BYTE_TO_OCTSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into an octal string (base 8).

##### `BYTEARR_TO_MAXSTRING` — §4.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `ARRAY[0..MAX_STRING_LENGTH] OF BYTE` | Byte-Array variable ( MAX_STRING_LENGTH default |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BYTEARR_TO_MAXSTRING` | `T_MaxString` | — |

**Description:** Converts the individual ASCII codes of a byte array into a string.

##### `CSVFIELD_TO_ARG` — §4.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInput` | `POINTER TO BYTE` | Start address (pointer) to a byte buffer containing the data field to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CSVFIELD_TO_ARG` | `UDINT` | — |

**Description:** The function converts the value from a data field in CSV format which is present as byte buffer into a PLC variable. Double quotation marks in the data field are replaced with simple quotation marks. If the bQM parameter is set (QM = quotation marks) the outer quotation marks (enclosing the data field) are removed from the input data. If successful the function returns the length of the converted data. In the event of an error or if the length of the input data is zero the function returns the value zero. The application must ensure the PLC target variable is large enough to accommodate the value.

##### `CSVFIELD_TO_STRING` — §4.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_MaxString` | Source string with a data field in CSV format that is to be converted into a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CSFIELD_TO_STRING` | `T_MaxString` | — |

**Description:** The function converts a data field in CSV data field format that is present as a source string into a value in PLC string format. Double quotation marks in the source string are replaced with simple quotation marks. If the bQM parameter is set (QM = quotation marks) the outer quotation marks (enclosing the data field) are removed from the source string. If successful the function returns the converted string as the result. The function returns an empty string if an error occurred during the conversion, but only if the source string was not an empty string.

##### `DATA_TO_HEXSTR` — §4.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pData` | `POINTER TO BYTE` | Start address (pointer) to the binary data to be converted. The |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DATA_TO_HEXSTR` | `T_MaxString` | — |

**Description:** Functions

##### `DEG_TO_RAD` — §4.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ANGLE` | `LREAL` | The angle to be converted in degrees. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DEG_TO_RAD` | `LREAL` | — |

**Description:** The function converts a degree angle to radian.

##### `DINT_TO_DECSTR` — §4.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DINT` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DINT_TO_DECSTR` | `T_MaxString Functions` | — |

**Description:** This function converts a signed decimal number into a decimal string (base 10).

##### `DWORD_TO_BINSTR` — §4.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DWORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DWORD_TO_BINSTR` | `T_MaxString 302 Functions` | — |

**Description:** This function converts a decimal number into a binary string (base 2).

##### `DWORD_TO_DECSTR` — §4.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Functions` | `` | — |
| `in` | `DWORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DWORD_TO_DECSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a decimal string (base 10).

##### `DWORD_TO_HEXSTR` — §4.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DWORD 304` | The decimal number to be converted. Functions |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DWORD_TO_HEXSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a hexadecimal string (base 16).

##### `DWORD_TO_LREALEX` — §4.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DWORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DWORD_TO_LREALEX` | `LREAL Functions` | — |

**Description:** The conversion of unsigned numbers into floating-point numbers of the type LREAL is not supported in TwinCAT 2 on the Arm® platform. Unsigned numbers with the most significant bit set may possibly be implicitly converted into negative floating-point numbers. The function described here allows the explicit conversion of the type DWORD into a positive floating-point number of the type LREAL in TwinCAT 2 (even if the most significant bit was set and without compiler warning). You only need this function to be able to compile converted TwinCAT 2 projects without changes in TwinCAT 3.

##### `DWORD_TO_OCTSTR` — §4.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DWORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DWORD_TO_OCTSTR` | `T_MaxString 306 Functions` | — |

**Description:** This function converts a decimal number into an octal string (base 8).

##### `F_BYTE_TO_CRC16_CCITT` — §4.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `BYTE` | The data byte to be converted. |
| `crc` | `WORD` | Initial value = 16#FFFF or 16#0000 or the last CRC. |
| `dwSrcAddr` | `POINTER TO BYTE` | Address of the data buffer. |
| `cbLen` | `UDINT` | Length of the data buffer. |
| `wChkSum` | `WORD` | : Initial value = 0 or last checksum. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_BYTE_TO_CRC16_CCITT` | `WORD` | — |
| `F_CheckSum16` | `WORD` | — |

**Description:** The function "F_BYTE_TO_CRC16_CCITT" can be used to determine a 16-bit CRC CCITT (cyclic redundancy check) for individual data bytes.

##### `F_CreateHashTableHnd` — §4.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pEntries` | `POINTER TO T_HashTableEntry` | Address of the first T_HashTableEntry array element. The address can be determined with the ADR |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hTable` | `T_HHASHTABLE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_CreateHashTableHnd` | `BOOL` | — |

**Description:** The function initializes the hash table handle. The table handle must be initialized once by calling the F_CreateHashTableHnd function.

##### `F_CreateLinkedListHnd` — §4.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pEntries` | `POINTER TO T_LinkedListEntry` | The address of the first T_LinkedListEntry array element. The address can be determined with the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hList` | `T_HLINKEDLIST` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_CreateLinkedListHnd` | `BOOL` | — |

**Description:** The function initializes the Linked List Handle. The List Handle must be initialized once by calling the F_CreateLinkedListHnd function.

##### `F_DATA_TO_CRC16_CCITT` — §4.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pData` | `POINTER TO BYTE` | Address of the data buffer. |
| `cbData` | `UDINT` | Length of the data buffer. |
| `crc` | `WORD` | Initial value = 16#FFFF or 16#0000 or the last CRC. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_DATA_TO_CRC16_CCITT` | `WORD` | — |

**Description:** The function "F_DATA_TO_CRC16_CCITT" can be used to determine a 16-bit CRC CCITT (cyclic redundancy check) for any data. Internally the function F_BYTE_TO_CRC16_CCITT [} 307] is used.

##### `F_FormatArgToStr` — §4.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSign` | `BOOL` | The sign flag. |
| `bBlank` | `BOOL` | The blank flag. |
| `bNull` | `BOOL` | The null flag. |
| `bHash` | `BOOL` | The hash prefix flag. |
| `bLAlign` | `BOOL` | The alignment flag (TRUE=left align). |
| `bWidth` | `BOOL` | If TRUE, the iWidth parameter is evaluated, otherwise not. |
| `iWidth` | `INT` | Width parameter. |
| `iPrecision` | `INT` | : Precision parameter. |
| `eFmtType` | `E_TypeFieldParam` | Type parameter (type: E_TypeFieldParam [} 358]). |
| `arg` | `T_Arg` | The argument to be formatted. The following helper functions can |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sOut` | `T_MaxString` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_FormatArgToStr` | `UDINT` | — |

**Description:** Format helper function. This function is used internally by the FB_FormatString [} 67] function block. The function can be used to convert a variable of type T_Arg [} 372] into a formatted string according to the format specification [} 406].

##### `F_GenerateHashValue` — §4.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hashMode` | `E_HashMode` | A hash mode, such as SHA 512, is specified here. See E_HashMode |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GenerateHashValue` | `BOOL` | — |

**Description:** The function can be used to calculate a hash value.

##### `F_GetClassIdVersioned` — §4.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sLibraryId` | `STRING(255); // 'vendorName\|libraryName\|libraryVersion' (e.g. 'C+` | — |
| `clsId` | `CLSID` | — |
| `clsIdVersioned` | `REFERENCE TO CLSID` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_GetClassIdVersioned` | `BOOL 314 Functions` | — |

**Description:** The function calculates the versioned Class ID based on the Class ID and the Library ID. This is used for versioned C++ projects.

##### `F_LTrim` — §4.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_MaxString` | The string to be converted (type: T_MaxString). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_LTrim` | `T_MaxString` | — |

**Description:** Removes leading spaces from the string and returns the reduced string.

##### `F_RTrim` — §4.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_MaxString` | The string to be converted (type: T_MaxString). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_RTrim` | `T_MaxString` | — |

**Description:** Truncates all trailing spaces from the specified value and returns the result.

##### `F_SplitBIC` — §4.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sBICValue` | `STRING` | This input must contain the Beckhoff Identification Code (BIC), which is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_SplitBIC` | `ST_SplittedBIC 316 Functions` | — |

**Description:** The F_SplitBIC function splits the Beckhoff Identification Code (BIC) sBICValue into its components using known identifiers and returns the recognized substrings in a structure ST_SplittedBIC [} 368] as return value. Spaces at the end of substrings are automatically removed. Unused substrings are returned as empty strings. If an unknown identifier is found, then the remaining string of the BIC is passed on as sUndefined in the return structure. The BIC can be read e.g. with the function blocks FB_EcCoEReadBIC or FB_EcReadBIC (Tc2_EtherCAT Library) from EtherCAT slaves.

##### `F_SwapRealEx` — §4.41

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fVal` | `REAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_SwapRealEx` | `BOOL` | — |
| `/` | `Inputs/outputs` | — |
| `fVal` | `REAL` | The REAL value to be converted. |
| `Return parameter` | `Meaning` | — |
| `TRUE` | `No error` | — |
| `FALSE` | `Error during function execution` | — |

**Description:** The memory representation of a REAL number on a bus terminal controller (e.g. BC2000, BC3100, BC9000) differs from the memory representation of a REAL number on an x86/x64/Arm® system (IPC or embedded PC). To correctly represent a REAL number of a bus terminal controller on an IPC, the Hi and Lo words of the REAL number must be swapped. Under the programming environment this is already done in online or simulation mode. In order to request REAL data from a bus controller over the network (ADS protocol, ADSDLL, AdsOcx, etc.) and represent it correctly on an x86/x64/Arm® IPC, the REAL data must be converted to the correct format. This can be done on the bus terminal controller side or the IPC side.

##### `F_ToLCase` — §4.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_MaxString` | The string to be converted (type: T_MaxString). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ToLCase` | `T_MaxString` | — |

**Description:** The F_ToLCase function converts all upper-case letters in a string into lower-case letters.

##### `F_ToUCase` — §4.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_MaxString` | The string to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ToUCase` | `T_MaxString` | — |

**Description:** 318

##### `GUID_TO_REGSTRING` — §4.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Functions` | `` | — |
| `in` | `GUID` | — |
| `Return value` | `Meaning` | — |
| `'{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}'` | `No error (‘x’ is a hexadecimal nibble)` | — |
| `'{00000000-0000-0000-0000-000000000000}'` | `No error, GUID has the initial value (all bytes are zero)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GUID_TO_REGSTRING` | `STRING(38)` | — |

**Description:** This function converts a structured GUID [} 359] variable into a registry GUID string variable (enclosed in curly brackets).

##### `GUID_TO_STRING` — §4.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stIn` | `GUID` | — |
| `Return value` | `Meaning` | — |
| `'xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx'` | `No error (‘x’ is a hexadecimal nibble)` | — |
| `00000000-0000-0000-0000-000000000000'` | `No error, GUID has the initial value (all bytes are zero)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GUID_TO_STRING` | `STRING` | — |

**Description:** This function converts a structured GUID [} 359] variable into a GUID string variable (without curly brackets).

##### `GuidsEqualByVal` — §4.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `guidA` | `GUID` | — |
| `guidB` | `GUID` | — |
| `Return value` | `Meaning` | — |
| `FALSE` | `guidA <> guidB` | — |
| `TRUE` | `guidA = guidB` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GuidsEqualByVal` | `BOOL` | — |

**Description:** This function compares two GUID values.

##### `HEXASCNIBBLE_TO_BYTE` — §4.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `asc` | `BYTE` | Ascii code of a hexadecimal nibble (ascii code from: ‘0’ to ‘9’ or ‘a’ to ‘f’ or ‘A’ to ‘F’). |
| `Return value` | `Meaning` | — |
| `0 to 15` | `Successful, no error.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HEXASCNIBBLE_TO_BYTE` | `BYTE` | — |

**Description:** This function converts the ASCII code of a hexadecimal nibble into the decimal value.

##### `HEXCHRNIBBLE_TO_BYTE` — §4.48

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `chr` | `STRING(1)` | Hexadecimal nibble (‘0’ to ‘9’ or ‘a’ to ‘f’ or ‘A’ to ‘F’). |
| `Return value` | `Meaning` | — |
| `0 to 15` | `Successful, no error.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HEXCHRNIBBLE_TO_BYTE` | `BYTE` | — |

**Description:** This function converts a hexadecimal nibble into its decimal value.

##### `HEXSTR_TO_DATA` — §4.49

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `322 Functions` | `` | — |
| `sHex` | `T_MaxString` | The hexadecimal string to be converted (type: T_MaxString, e.g.: ‘AB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HEXSTR_TO_DATA` | `UDINT` | — |

**Description:** The function converts a hexadecimal string into binary data and returns the number of successfully converted data bytes as result. Only spaces may be used as separators in the hexadecimal string to be converted. Lower and upper case letters are permitted as hex characters. In the event of an error or an illegal character the conversion is aborted and a zero length is returned as result.

##### `LINT_TO_DECSTR` — §4.50

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LINT` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LINT_TO_DECSTR` | `T_MaxString Functions` | — |

**Description:** This function converts a signed decimal number into a decimal string (base 10).

##### `LREAL_TO_FMTSTR` — §4.51

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LREAL` | Floating-point number that is to be converted and formatted. |
| `iPrecision` | `INT` | Precision. The value determines the number of digits after the decimal point. With |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LREAL_TO_FMTSTR` | `STRING(510)` | — |

**Description:** The function converts and formats a floating-point number into a string variable with the following format: [ - ]dddd.dddd (dddd are decimal numbers). The number of digits before the decimal point depends on the value of the floating-point number. The number of digits after the decimal point depends on the required precision. The sign only appears for negative values. '#INF' is returned for infinite positive values, '-#INF' for infinite negative values. If the transferred variable has an invalid value (NaN, Not-a-Number), then ‘#QNAN’ or ‘-#QNAN’ is returned. If the length of the formatted string exceeds the maximum permissible length of the resulting string, '#OVF' or '-#OVF' is returned.

##### `LWORD_TO_BASE36STR` — §4.52

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LWORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LWORD_TO_BASE36STR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a Base36 string (base 16). For numbers with base 16, the letters A-Z are used in addition to the digits 0-9.

##### `LWORD_TO_BINSTR` — §4.53

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LWORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LWORD_TO_BINSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a binary string (base 2).

##### `LWORD_TO_DECSTR` — §4.54

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LWORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LWORD_TO_DECSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a decimal string (base 10).

##### `LWORD_TO_HEXSTR` — §4.55

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LWORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LWORD_TO_HEXSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a hexadecimal string (base 16).

##### `LWORD_TO_OCTSTR` — §4.56

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `LWORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LWORD_TO_OCTSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into an octal string (base 8).

##### `MAXSTRING_TO_BYTEARR` — §4.57

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_MaxString` | String to be converted (type: T_MaxString). |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MAXSTRING_TO_BYTEARR` | `ARRAY[0..MAX_STRING_LENG TH] OF BYTE 330 Functions` | — |

**Description:** Converts a string into individual ASCII codes of a byte array.

##### `PVOID_TO_BINSTR` — §4.58

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `PVOID` | The pointer variable to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PVOID_TO_BINSTR` | `T_MaxString` | — |

**Description:** The function converts the value of a pointer variable of type PVOID into a binary string (basis 2).

##### `PVOID_TO_DECSTR` — §4.59

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `PVOID` | The pointer variable to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PVOID_TO_DECSTR` | `T_MaxString` | — |

**Description:** The function converts the value of a pointer variable of type PVOID into a decimal string (basis 10).

##### `PVOID_TO_HEXSTR` — §4.60

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `PVOID` | The pointer variable to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PVOID_TO_HEXSTR` | `T_MaxString` | — |

**Description:** The function converts the value of a pointer variable of type PVOID into a hexadecimal string (basis 16).

##### `PVOID_TO_OCTSTR` — §4.61

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `PVOID` | The pointer variable to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PVOID_TO_OCTSTR` | `T_MaxString` | — |

**Description:** The function converts the value of a pointer variable of type PVOID into an octal string (basis 8).

##### `PVOID_TO_STRING` — §4.62

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `PVOID` | The pointer variable to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PVOID_TO_STRING` | `T_MaxString` | — |

**Description:** The function converts the value of a pointer variable of type PVOID into a hexadecimal string (basis 16). The hexadecimal string has the PLC prefix: '16#'. The resolution is fixed at 8 digits in a 32-bit system and 16 digits in 64-bit system.

##### `RAD_TO_DEG` — §4.63

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ANGLE` | `LREAL` | Radian to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RAD_TO_DEG` | `LREAL` | — |

**Description:** The function converts radian into a degree angle.

##### `REGSTRING_TO_GUID` — §4.64

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `STRING(38)` | — |
| `Return value` | `Meaning` | — |
| `'{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}'` | `No error (‘x’ is a hexadecimal nibble).` | — |
| `'{00000000-0000-0000-0000-000000000000}'` | `Conversion failed or GUID has initial value (all bytes are zero)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `REGSTRING_TO_GUID` | `GUID` | — |

**Description:** This function converts a registry GUID string variable (enclosed in curly brackets) into a structured GUID [} 359] variable.

##### `ROUTETRANSPORT_TO_STRING` — §4.65

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eType` | `E_RouteTransportType` | The transport layer identifier to be converted (type: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ROUTETRANSPORT_TO_STRING` | `STRING` | — |

**Description:** The function converts the AMS message router transport layer ID into a string.

##### `STRING_TO_CSVFIELD` — §4.66

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `338 Functions` | `` | — |
| `in` | `T_MaxString` | Source string whose value is to be converted into a data field in CSV format |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `STRING_TO_CSVFIELD` | `T_MaxString` | — |

**Description:** The function converts the value of a PLC string variable into a data field in CSV format as a string. Single quotation marks in the source string are replaced by double quotation marks. If the bQM parameter is set (QM = quotation marks) the outer quotation marks (enclosing the CSV data field) are also added. If successful the function returns the converted string as the result. The function returns an empty string if an error occurred during the conversion, but only if the source string was not an empty string.

##### `STRING_TO_GUID` — §4.67

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `STRING(36)` | — |
| `Return value` | `Meaning` | — |
| `'xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx'` | `No error (‘x’ is a hexadecimal nibble)` | — |
| `'00000000-0000-0000-0000-000000000000'` | `Conversion failed or GUID has initial value (all bytes are zero)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `STRING_TO_GUID` | `GUID` | — |

**Description:** This function converts a GUID string variable (without curly brackets) into a structured GUID [} 359] variable.

##### `STRING_TO_PVOID` — §4.68

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `STRING` | String variable to be converted. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `STRING_TO_PVOID` | `PVOID` | — |

**Description:** The function converts a string variable into a pointer variable of type PVOID. The function returns zero if the input string contains incorrect characters and cannot be interpreted as an address.

##### `UDINT_TO_LREALEX` — §4.69

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UDINT_TO_LREALEX` | `LREAL` | — |

**Description:** The conversion of unsigned numbers into floating-point numbers of the type LREAL is not supported in TwinCAT 2 on the Arm® platform. Unsigned numbers with the most significant bit set may possibly be implicitly converted into negative floating-point numbers. The function described here allows the explicit conversion of the type UDINT into a positive floating-point number of the type LREAL in TwinCAT 2 (even if the most significant bit was set and without compiler warning). You only need this function to be able to compile converted TwinCAT 2 projects without changes in TwinCAT 3.

##### `UINT_TO_LREALEX` — §4.70

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `UINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UINT_TO_LREALEX` | `LREAL` | — |

**Description:** The conversion of unsigned numbers into floating-point numbers of the type LREAL is not supported in TwinCAT 2 on the Arm® platform. Unsigned numbers with the most significant bit set may possibly be implicitly converted into negative floating-point numbers. The function described here allows the explicit conversion of the type UINT into a positive floating-point number of the type LREAL in TwinCAT 2 (even if the most significant bit was set and without compiler warning). You only need this function to be able to compile converted TwinCAT 2 projects without changes in TwinCAT 3.

##### `ULINT_TO_ULARGE` — §4.71

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `ULINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ULINT_TO_ULARGE` | `T_ULARGE_INTEGER` | — |

**Description:** The function converts a TwinCAT 3 unsigned 64-bit number (“native” type) into a TwinCAT 2 unsigned 64-bit number (“legacy” type: T_ULARGE_INTEGER [} 376]).

##### `USINT_TO_LREALEX` — §4.72

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `USINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `USINT_TO_LREALEX` | `LREAL` | — |

**Description:** The conversion of unsigned numbers into floating-point numbers of the type LREAL is not supported in TwinCAT 2 on the Arm® platform. Unsigned numbers with the most significant bit set may possibly be implicitly converted into negative floating-point numbers. The function described here allows the explicit conversion of the type USINT into a positive floating-point number of the type LREAL in TwinCAT 2 (even if the most significant bit was set and without compiler warning). You only need this function to be able to compile converted TwinCAT 2 projects without changes in TwinCAT 3.

##### `WORD_TO_BINSTR` — §4.73

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `WORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WORD_TO_BINSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a binary string (base 2).

##### `WORD_TO_DECSTR` — §4.74

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `WORD` | The decimal number to be converted. Functions |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WORD_TO_DECSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a decimal string (base 10).

##### `WORD_TO_HEXSTR` — §4.75

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `WORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WORD_TO_HEXSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into a hexadecimal string (base 16).

##### `WORD_TO_LREALEX` — §4.76

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `WORD` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WORD_TO_LREALEX` | `LREAL` | — |

**Description:** 346

##### `WORD_TO_OCTSTR` — §4.77

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `WORD` | The decimal number to be converted. |
| `iPrecision` | `INT` | Minimum number of displayed digits. If the actual number of significant digits is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WORD_TO_OCTSTR` | `T_MaxString` | — |

**Description:** This function converts a decimal number into an octal string (base 8).

#### Samples

##### `Example: Memory ring FiFo (FB_MemRingBuffer)` (FUNCTION_BLOCK) — §9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bOverwrite` | `BOOL` | — |
| `in` | `ST_DataSetEntry` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOk` | `BOOL` | — |
| `nCount` | `UDINT` | — |
| `nLoad` | `UDINT` | — |
| `out` | `ST_DataSetEntry` | — |

**Description:** The complete sources can be found here: https://infosys.beckhoff.com/content/1033/TcPlcLib_Tc2_Utilities/ Resources/803399691.zip
