# Tc2_MDP

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_MDP_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T1 · **API entries:** 17
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Media/device parameter handling needs a device-parameter service.

---

#### Function blocks › Generic › Advanced

##### `FB_MDP_Read` — §4.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL;             (* Function block execution is triggered by a rising edge at t` | — |
| `stMDP_DynAddr` | `ST_MDP_Addr` | — |
| `pDstBuf` | `DWORD;            (* Contains the address of the buffer for the received data. *` | — |
| `cbDstBufLen` | `UDINT;            (* Contains the max. number of bytes to be received. *)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled` | — |
| `sAmsNetId` | `T_AmsNetId;       (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrId` | `UDINT` | — |
| `nCount` | `UDINT` | — |

**Description:** The function block enables querying of an IPC diagnostics module element.

##### `FB_MDP_Write` — §4.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL;                  (* Function block execution is triggered by a rising edge a` | — |
| `stMDP_DynAddr` | `ST_MDP_Addr` | — |
| `pSrcBuf` | `DWORD;              (* Contains the address of the buffer for the sent data. *)` | — |
| `cbSrcBufLen` | `UDINT;              (* Contains the max. number of bytes to be sent. *)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelle` | — |
| `sAmsNetId` | `T_AmsNetId;            (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrId` | `UDINT` | — |

**Description:** The function block enables setting of an IPC diagnostics module element.

##### `FB_MDP_ReadIndex` — §4.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL;             (* Function block execution is triggered by a rising edge at thi` | — |
| `nIndex` | `WORD` | — |
| `nSubIndex` | `BYTE` | — |
| `pDstBuf` | `DWORD;            (* Contains the address of the buffer for the received data. *)` | — |
| `cbDstBufLen` | `UDINT;            (* Contains the max. number of bytes to be received. *)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled.` | — |
| `sAmsNetId` | `T_AmsNetId;       (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrId` | `UDINT` | — |
| `nCount` | `UDINT` | — |

**Description:** The function block enables querying of any IPC diagnostics element. In addition to the Configuration Area, data from the device area are also accessible.

#### Function blocks › Generic

##### `FB_MDP_ReadElement` — §4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `stMDP_Addr` | `ST_MDP_Addr;         (* includes all address parameters without the Dynamic Module` | — |
| `eModuleType` | `E_MDP_ModuleType;   (* chosen module type out of the module type list *)` | — |
| `iModIdx` | `USINT;              (* chosen index(0..n) of the demanded module type. E.g. second` | — |
| `pDstBuf` | `DWORD;              (* Contains the address of the buffer for the received data. *` | — |
| `cbDstBufLen` | `UDINT;              (* Contains the max. number of bytes to be received. *)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled.` | — |
| `sAmsNetId` | `T_AmsNetId;         (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL;             (* indicates if Read was successfull or not *)` | — |
| `nErrID` | `UDINT` | — |
| `nCount` | `UDINT` | — |
| `stMDP_DynAddr` | `ST_MDP_Addr;      (* includes the new dynamic module type id. *)` | — |
| `iModuleTypeCount` | `USINT;            (* returns the number of found modules equal the demanded modul` | — |
| `iModuleCount` | `USINT;            (* returns the number of all detected MDP modules. *)` | — |

**Description:** The function block enables querying of an individual MDP element. In this way, each element from each module of the Configuration Area can be read!

##### `FB_MDP_ReadModule` — §4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `stMDP_Addr` | `ST_MDP_Addr;        (* includes all address parameters without the Dynamic Module` | — |
| `eModuleType` | `E_MDP_ModuleType;    (* chosen module type out of the module type list *)` | — |
| `iModIdx` | `USINT;              (* chosen index(0..n) of the demanded module type. E.g. secon` | — |
| `iSubIdxCount` | `USINT` | — |
| `pDstBuf` | `DWORD;              (* Contains the address of the buffer for the received data.` | — |
| `cbDstBufLen` | `UDINT;               (* Contains the max. number of bytes to be received. *)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled.` | — |
| `sAmsNetId` | `T_AmsNetId;         (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL;                (* indicates if Read was successfull or not *)` | — |
| `nErrID` | `UDINT` | — |
| `iErrPos` | `USINT` | — |
| `stMDP_DynAddr` | `ST_MDP_Addr;   (* includes the new dynamic module type id. *)` | — |
| `iModuleTypeCount` | `USINT;         (* returns the number of found modules equal the demanded mo` | — |
| `iModuleCount` | `USINT;         (* returns the number of all detected MDP modules. *)` | — |
| `stMDP_ModuleHeader` | `ST_MDP_ModuleHeader` | — |
| `arrStartIdx` | `ARRAY[0..255] OF UINT; (* startindexes in bytes of each subindex element *)` | — |

**Description:** The function block enables an MDP module to be queried.

##### `FB_MDP_ReadModuleContent` — §4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `stMDP_DynAddr` | `ST_MDP_Addr;    (* includes the dynamic module type for which the module content` | — |
| `iSubIdxCount` | `USINT;           (* the number of SubIndexes to be requested *)` | — |
| `pDstBuf` | `DWORD;          (* Contains the address of the buffer for the received data. *)` | — |
| `cbDstBufLen` | `UDINT;          (* Contains the max. number of bytes to be received. *)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled` | — |
| `sAmsNetId` | `T_AmsNetId;     (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL;     (* indicates if Read was successfull or not *)` | — |
| `nErrID` | `UDINT` | — |
| `iErrPos` | `USINT` | — |
| `arrStartIdx` | `ARRAY[0..255] OF UINT;    (* startindexes in bytes of each subindex element *)` | — |

**Description:** The function block enables querying of the content of an IPC diagnostics module.

##### `FB_MDP_ReadModuleHeader` — §4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `nDynModuleId` | `BYTE;             (* the dynamic module id for which the module header is request` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled.` | — |
| `sAmsNetId` | `T_AmsNetId;       (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL;   (* indicates if Read was successfull or not *)` | — |
| `nErrID` | `UDINT` | — |
| `stMDP_ModHeader` | `ST_MDP_ModuleHeader` | — |

**Description:** The function block enables querying of the header of an IPC diagnostics module.

##### `FB_MDP_ScanModules` — §4.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `nModuleType` | `WORD;             (* chosen module type out of the module type list *)` | — |
| `iModIdx` | `USINT;            (* chosen index(0..n) of the demanded module type. E.g. second N` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled.` | — |
| `sAmsNetId` | `T_AmsNetId;       (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL;      (* indicates if Scan was successfull or not *)` | — |
| `nErrID` | `UDINT` | — |
| `nDynModuleId` | `BYTE; (* Dynamic Module Id *)` | — |
| `iModuleTypeCount` | `USINT; (* returns the number of found modules equal the demanded module type.` | — |
| `iModuleCount` | `USINT; (* returns the number of all detected MDP modules. *)` | — |

**Description:** The function block enables a device to be searched for a specific IPC diagnostics module. A selection can be made if the module type is present more than once. For the selected module type the dynamic module ID is determined by the function block. This is an important part of the MDP addressing, which is represented in the structure ST_MDP_Addr [} 29].

##### `FB_MDP_SplitErrorId` — §4.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nErrID` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eErrGroup` | `E_MDP_ErrGroup; (* type of transmitted error code *)` | — |
| `nErrCode` | `UINT;            (* error code [see specific error type table] *)` | — |

**Description:** The function block enables the nErrID to be split into an error group [} 34] and a specific error code.

#### Function blocks › Specific

##### `FB_MDP_CPU_Read` — §4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL;         (* Function block execution is triggered by a rising edge at this inpu` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled. *)` | — |
| `iModIdx` | `USINT := 0;  (* Index number of chosen MDP module *)` | — |
| `sAmsNetId` | `T_AmsNetId;  (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `iErrPos` | `USINT` | — |
| `stMDP_ModuleHeader` | `ST_MDP_ModuleHeader` | — |
| `stMDP_ModuleContent` | `ST_MDP_CPU` | — |

**Description:** The function block enables querying of the IPC diagnostics module CPU.

##### `FB_MDP_Device_Read_DevName` — §4.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL;              (* Function block execution is triggered by a rising edge at this in` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled. *)` | — |
| `sAmsNetId` | `T_AmsNetId;    (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sDevName` | `T_MaxString` | — |

**Description:** The function block enables the device name to be queried. This information can be found in the general area of the IPC diagnostics.

##### `FB_MDP_IdentityObj_Read` — §4.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL; (* Function block execution is triggered by a rising edge at this input.*)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled. *)` | — |
| `sAmsNetId` | `T_AmsNetId; (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `iErrPos` | `USINT` | — |
| `stMDP_ModuleContent` | `ST_MDP_IdentityObject` | — |

**Description:** Der Funktionsbaustein ermöglicht die Abfrage der Tabelle IdentityObject der General Area der IPC- Diagnose.

##### `FB_MDP_NIC_Read` — §4.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL;           (* Function block execution is triggered by a rising edge at this in` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled. *)` | — |
| `iModIdx` | `USINT := 0;    (* Index number of chosen MDP module *)` | — |
| `sAmsNetId` | `T_AmsNetId;    (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `iErrPos` | `USINT` | — |
| `stMDP_ModuleHeader` | `ST_MDP_ModuleHeader` | — |
| `stMDP_ModuleContent` | `ST_MDP_NIC_Properties` | — |

**Description:** The function block enables querying of the IPC diagnostics Moduls NIC (Network Interface Card).

##### `FB_MDP_NIC_Write_IP` — §4.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `nDynModuleId` | `BYTE;             (* the dynamic module id *)` | — |
| `sIPAddress` | `T_MaxString;      (* IP Address *)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled.` | — |
| `sAmsNetId` | `T_AmsNetId;       (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** The function block enables a new IP address to be set. This element is part of the IPC diagnostics Moduls NIC.

##### `FB_MDP_SiliconDrive_Read` — §4.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL;         (* Function block execution is triggered by a rising edge at this inpu` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled. *)` | — |
| `iModIdx` | `USINT := 0;    (* Index number of chosen MDP module *)` | — |
| `sAmsNetId` | `T_AmsNetId;    (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `iErrPos` | `USINT` | — |
| `stMDP_ModuleHeader` | `ST_MDP_ModuleHeader` | — |
| `stMDP_ModuleContent` | `ST_MDP_SiliconDrive` | — |

**Description:** The function block enables querying of the IPC diagnostics module SiliconDrive.

##### `FB_MDP_SW_Read_MdpVersion` — §4.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL;           (* Function block execution is triggered by a rising edge at this in` | — |
| `tTimeout` | `TIME :=DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled. *)` | — |
| `sAmsNetId` | `T_AmsNetId;    (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sMdpVersion` | `STRING(23); (* complete MDP version as string [e.g.: '1, 0, 4, 47'] *)` | — |
| `iMajorNbr` | `UINT;     (* major number [e.g.: 1] *)` | — |
| `iMinorNbr` | `UINT;     (* minor number [e.g.: 4] *)` | — |
| `iRevNbr` | `UINT;     (* revision number [e.g.: 47] *)` | — |

**Description:** The function block enables querying of the MDP version. This information can be found in the software module in the configuration area of the IPC diagnostics.

##### `FB_MDP_TwinCAT_Read` — §4.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL;         (* Function block execution is triggered by a rising edge at this inpu` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* States the time before the function is cancelled. *)` | — |
| `iModIdx` | `USINT := 0;    (* Index number of chosen MDP module *)` | — |
| `sAmsNetId` | `T_AmsNetId;    (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `iErrPos` | `USINT` | — |
| `stMDP_ModuleHeader` | `ST_MDP_ModuleHeader` | — |
| `stMDP_ModuleContent` | `ST_MDP_TwinCAT` | — |

**Description:** The function block enables querying of the IPC diagnostics module TwinCAT.
