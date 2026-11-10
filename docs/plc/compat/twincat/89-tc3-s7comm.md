# Tc3_S7Comm

- **Manual:** `TF6620_TC3_S7_Communication_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 24
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** S7 communication (Put/Get, read/write): same S7 codec requirement.

---

#### PLC API › Tc3_S7Comm › Function blocks

##### `FB_S7CommConnection` — §5.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The connection to the S7 Controller |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Switches to TRUE if an error |

**Description:** With the function block FB_S7CommConnection, a TCP/IP based connection to a Siemens S7 Controller can be established. It is possible to tell whether the connection was successfully established via the output bIsConnected. Errors concerning setting up the connection are displayed via the output bError and sErrorTxt.

#### PLC API › Tc3_S7Comm › Function blocks › FB_S7CommConnection

##### `AddRequest` (METHOD) → `BOOL` — §5.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipRequest` | `ITF_S7CommRequest` | Single/Cyclic Request Object |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddRequest` | `BOOL` | TRUE if the method was carried out correctly. In the event of an error, the |

**Description:** Adds a Single/Cyclic Request (FB_S7CommSingleRequest [} 48], FB_S7CommCyclicRequest [} 37]) to an S7 communication connection (FB_S7CommConnection [} 35]). Read/Write commands can be defined on the respective request type of the added object.

#### PLC API › Tc3_S7Comm › Function blocks

##### `FB_S7CommCyclicRequest` — §5.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is executed by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Switches to TRUE if an error |

**Description:** Cyclic processing (read/write) of data points of an S7 communication connection can be configured with the FB_S7CommCyclicRequest function block. The bError output states whether the request was successfully carried out. Any errors occurring at the time of the request are displayed via the output sErrorTxt and nErrorID.

#### PLC API › Tc3_S7Comm › Function blocks › FB_S7CommCyclicRequest

##### `AddReadVar` (METHOD) → `HRESULT` — §5.1.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be read from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReadVar` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a read command on an S7 data point to a request. The data point is specified via its absolute address in the S7 Controller.

##### `AddReadBit` (METHOD) → `HRESULT` — §5.1.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteOff` | `WORD` | Byte length of the data type to be read from the S7 Controller |
| `nBitOff` | `BYTE` | Bit offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReadBit` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a read command on a BIT-type S7 data point to a request. The data point is specified via its absolute address in the S7 Controller. The address of the target variables passed on must be a BOOL data type (not BIT).

##### `AddReadString` (METHOD) → `HRESULT` — §5.1.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be read from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReadString` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a read command on a STRING-type S7 data point to a request. The data point is specified via its absolute address in the S7 Controller. The length of the string should not exceed 254 characters.

##### `AddReadByteArray` (METHOD) → `HRESULT` — §5.1.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be read from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReadVar` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a read command to the request in an optimized [} 29] manner if several data points located consecutively in the memory of the S7 Controller are to be read.

##### `AddWriteVar` (METHOD) → `HRESULT` — §5.1.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the source variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be written from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddWriteVar` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a write command on an S7 data point to a request. The data point is specified via its absolute address in the S7 Controller.

##### `AddWriteBit` (METHOD) → `HRESULT` — §5.1.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteOff` | `WORD` | Byte length of the data type to be written from the S7 Controller |
| `nBitOff` | `BYTE` | Bit offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddWriteBit` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a read command on a BIT-type S7 data point to a request. The data point is specified via its absolute address in the S7 Controller. The address of the source variables passed on must be a BOOL data type (not BIT).

##### `AddWriteString` (METHOD) → `HRESULT` — §5.1.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the source variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be written from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddWriteString` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a write command on a STRING-type S7 data point to a request. The data point is specified via its absolute address in the S7 Controller. The maximum length of the string should not exceed 254 characters.

##### `AddWriteByteArray` (METHOD) → `HRESULT` — §5.1.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be read from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReadVar` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a write command to the request in an optimized [} 29] manner if several data points located consecutively in the memory of the S7 Controller are to be written.

##### `RemoveRead` (METHOD) → `HRESULT` — §5.1.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the variables in the TwinCAT PLC |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveRead` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Removes a variable from a read request. The variable is specified via its address in the TwinCAT PLC. No read request may be pending at the time that the method is executed, i.e. the output bBusy of the function block must be FALSE.

##### `RemoveWrite` (METHOD) → `HRESULT` — §5.1.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the variables in the TwinCAT PLC |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveWrite` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Removes a variable from a write request. The variable is specified via its address in the TwinCAT PLC. No write request may be pending at the time that the method is executed, i.e. the output bBusy of the function block must be FALSE.

#### PLC API › Tc3_S7Comm › Function blocks

##### `FB_S7CommSingleRequest` — §5.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is executed by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Switches to TRUE if an error occurs during execution. |
| `sErrorTxt` | `STRING` | Contains the error text in the event of an error. |
| `nErrorId` | `WORD` | Outputs an error code in the event of an error. |
| `bBusy` | `BOOL` | TRUE until the function block has executed a command. As long as |

**Description:** With the FB_S7CommSingleRequest function block, a read/write request can be carried out on a data point of an S7 communication connection. The bError output states whether the request was successfully carried out. Any errors occurring at the time of the request are displayed via the output sErrorTxt and nErrorID.

#### PLC API › Tc3_S7Comm › Function blocks › FB_S7CommSingleRequest

##### `AddReadVar` (METHOD) → `HRESULT` — §5.1.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be read from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReadVar` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a read command on an S7 data point to a request. The data point is specified via its absolute address in the S7 Controller.

##### `AddReadBit` (METHOD) → `HRESULT` — §5.1.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteOff` | `WORD` | Byte length of the data type to be read from the S7 Controller |
| `nBitOff` | `BYTE` | Bit offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReadBit` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a read command on a BIT-type S7 data point to a request. The data point is specified via its absolute address in the S7 Controller. The address of the target variables passed on must be a BOOL data type (not BIT).

##### `AddReadString` (METHOD) → `HRESULT` — §5.1.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be read from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReadString` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a read command on a STRING-type S7 data point to a request. The data point is specified via its absolute address in the S7 Controller. The length of the string should not exceed 254 characters.

##### `AddReadByteArray` (METHOD) → `HRESULT` — §5.1.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be read from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReadVar` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a read command to the request in an optimized [} 29] manner if several data points located consecutively in the memory of the S7 Controller are to be read.

##### `AddWriteVar` (METHOD) → `HRESULT` — §5.1.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the source variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be written from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddWriteVar` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a write command on an S7 data point to a request. The data point is specified via its absolute address in the S7 Controller.

##### `AddWriteBit` (METHOD) → `HRESULT` — §5.1.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteOff` | `WORD` | Byte length of the data type to be written from the S7 Controller |
| `nBitOff` | `BYTE` | Bit offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddWriteBit` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a read command on a BIT-type S7 data point to a request. The data point is specified via its absolute address in the S7 Controller. The address of the source variables passed on must be a BOOL data type (not BIT).

##### `AddWriteString` (METHOD) → `HRESULT` — §5.1.1.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the source variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be written from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddWriteString` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a write command on a STRING-type S7 data point to a request. The data point is specified via its absolute address in the S7 Controller. The maximum length of the string should not exceed 254 characters.

##### `AddWriteByteArray` (METHOD) → `HRESULT` — §5.1.1.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the target variables in the TwinCAT PLC |
| `nByteSize` | `WORD` | Byte length of the data type to be read from the S7 Controller |
| `nByteOff` | `WORD` | Byte offset in the S7 Controller |
| `nArea` | `E_S7COMM_DATAAREA [} 60]` | S7 data area |
| `nDatablock` | `WORD E_S7COMM_DATAAREA.DATA_B` | ID of the data block. Is only sent if |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddReadVar` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Adds a write command to the request in an optimized [} 29] manner if several data points located consecutively in the memory of the S7 Controller are to be written.

##### `RemoveRead` (METHOD) → `HRESULT` — §5.1.1.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the variables in the TwinCAT PLC |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveRead` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Removes a variable from a read request. The variable is specified via its address in the TwinCAT PLC. No read request may be pending at the time that the method is executed, i.e. the output bBusy of the function block must be FALSE.

##### `RemoveWrite` (METHOD) → `HRESULT` — §5.1.1.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVar` | `PVOID` | Address of the variables in the TwinCAT PLC |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveWrite` | `HRESULT` | E_HRESULTAdsErr.NOTINIT = Function block has not been |

**Description:** Removes a variable from a write request. The variable is specified via its address in the TwinCAT PLC. No write request may be pending at the time that the method is executed, i.e. the output bBusy of the function block must be FALSE.
