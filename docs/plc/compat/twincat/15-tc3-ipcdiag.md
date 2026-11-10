# Tc3_IPCDiag

- **Manual:** `TcPlcLib_Tc3_IPCDiag_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T2 · **API entries:** 7
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** IPC diagnostics need OS process/IPC introspection.

---

#### Function blocks

##### `FB_IPCDiag_ReadParameter` (METHOD) → `HRESULT` — §3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `eParameterKey` | `E_IPCDiag_ParameterKey` | — |
| `nModuleIdx` | `USINT := 0;   (* optional module selection for parameters of configuration area` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* ADS communication timeout *)` | — |
| `sNetId` | `T_AmsNetId := '';            (* keep empty '' for the local device *)` | — |
| `pBuffer` | `PVOID;       // buffer for parameter with a given size of nParameterSize` | — |
| `nBufferSize` | `UDINT;       // buffer size in bytes (for one or more values)` | — |
| `pBuffer` | `PVOID;                // buffer for parameter with a given size of nParameterSize` | — |
| `nBufferSize` | `UDINT;                // buffer size in bytes (for one or more values)` | — |
| `nParameterIdx` | `USINT(1..255);        // selection of parameter value (1..nReadParameterValues)` | — |
| `pBuffer` | `PVOID;    // buffer for parameter with a given size of nParameterSize` | — |
| `nBufferSize` | `UDINT;    // buffer size in bytes (for one or more values)` | — |
| `nStrings` | `USINT;    // number of strings to be copied (each string with size=nBufferSize/` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fbRegister` | `FB_IPCDiag_Register` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL;       (* read data available (equal IF NOT bBusy AND NOT bError) *)` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |
| `ipErrorMessage` | `I_TcMessage` | — |
| `nReadParameterValues` | `USINT;   (* number of read parameter values *)` | — |
| `nModuleCount` | `USINT;   (* number of module instances (configuration area) with the dem` | — |

**Description:** The function block allows an IPC diagnostic parameter to be queried. The desired parameter is selected from the enumeration E_IPCDiag_ParameterKey [} 19].

##### `FB_IPCDiag_ReadParameterPeriodic` (METHOD) → `HRESULT` — §3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `eParameterKey` | `E_IPCDiag_ParameterKey` | — |
| `nModuleIdx` | `USINT := 0;    (* optional module selection for parameters of configuration area` | — |
| `tPeriod` | `TIME := T#60S;                (* set tPeriod >= tTimeout for periodical reading` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT; (* ADS communication timeout *)` | — |
| `sNetId` | `T_AmsNetId := '';             (* keep empty '' for the local device *)` | — |
| `pBuffer` | `PVOID;       // buffer for parameter with a given size of nParameterSize` | — |
| `nBufferSize` | `UDINT;       // buffer size in bytes (for one or more values)` | — |
| `pBuffer` | `PVOID;                // buffer for parameter with a given size of nParameterSize` | — |
| `nBufferSize` | `UDINT;                // buffer size in bytes (for one or more values)` | — |
| `nParameterIdx` | `USINT(1..255);        // selection of parameter value (1..nReadParameterValues)` | — |
| `pBuffer` | `PVOID;    // buffer for parameter with a given size of nParameterSize` | — |
| `nBufferSize` | `UDINT;    // buffer size in bytes (for one or more values)` | — |
| `nStrings` | `USINT;    // number of strings to be copied (each string with size=nBufferSize/` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fbRegister` | `FB_IPCDiag_Register` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL;     (* read data available (equal IF NOT bBusy AND NOT bError) *)` | — |
| `bError` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |
| `ipErrorMessage` | `I_TcMessage` | — |
| `nReadParameterValues` | `USINT;    (* number of read parameter values *)` | — |
| `nModuleCount` | `USINT;    (* number of module instances (configuration area) with the dem` | — |

**Description:** This function block allows an IPC diagnostic parameter to be queried periodically. The desired parameter is selected from the enumeration E_IPCDiag_ParameterKey [} 19].

##### `FB_IPCDiag_WriteParameter` — §3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `eParameterKey` | `E_IPCDiag_ParameterKey` | — |
| `pBuffer` | `PVOID;                            (* parameter buffer with a given size of nBufferS` | — |
| `nBufferSize` | `UDINT` | — |
| `nModuleIdx` | `USINT := 1;                        (* optional module selection *)` | — |
| `nListIdx` | `USINT := 1;                        (* optional list entry selection *)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT;       (* States the time before the function is cancell` | — |
| `sNetId` | `T_AmsNetId := '';                  (* keep empty '' for the local device *)` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fbRegister` | `FB_IPCDiag_Register` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |
| `ipErrorMessage` | `I_TcMessage` | — |

**Description:** The function block allows an IPC diagnostic parameter to be written. The desired parameter is selected from the enumeration E_IPCDiag_ParameterKey [} 19].

##### `FB_IPCDiag_Register` — §3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT;       (* States the time before the function is cancelled` | — |
| `sNetId` | `T_AmsNetId := '';                  (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL; (* read data available =(NOT bBusy AND NOT bError) *)` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |
| `ipErrorMessage` | `I_TcMessage` | — |
| `stModuleList` | `ST_IPCDiag_ModuleList` | — |

**Description:** The function block checks which IPC diagnostic parameters are available and creates a register for this purpose. Specifically, a list of all available parameter modules is created. The function block only needs to determine this once, typically at the start of the program. If diagnostic parameters of a remote system are to be queried by means of a non-local AMS Net Id, the function block must be executed after its start.

#### Function blocks › Access by index specification

##### `FB_IPCDiag_ReadParameterByMdpIdx` — §3.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `nIndex` | `WORD;       (* F_IPCDiag_GetMdpIndex should be used if parameter is out of configu` | — |
| `nFlags` | `BYTE` | — |
| `nSubIndex` | `BYTE` | — |
| `pBuffer` | `PVOID;    (* Contains the address of the buffer for the received data. *)` | — |
| `nBufferSize` | `UDINT;    (* Contains the max. number of bytes to be received. *)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT;   (* States the time before the function is cancell` | — |
| `sNetId` | `T_AmsNetId := '';                  (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |
| `ipErrorMessage` | `I_TcMessage` | — |
| `nCount` | `UDINT;       (* returns the number of bytes received *)` | — |

**Description:** In individual cases, this function block can be used as an alternative to the simpler FB_IPCDiag_ReadParameter [} 9] in order to query any desired parameters of the IPC diagnostics.

##### `FB_IPCDiag_WriteParameterByMdpIdx` — §3.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |
| `nIndex` | `WORD;       (* F_IPCDiag_GetMdpIndex should be used if parameter is out of configu` | — |
| `nFlags` | `BYTE` | — |
| `nSubIndex` | `BYTE` | — |
| `pBuffer` | `PVOID;    (* Contains the address of the buffer for data to be written. *)` | — |
| `nBufferSize` | `UDINT;    (* Contains the max. number of bytes to be written. *)` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT;   (* States the time before the function is cancell` | — |
| `sNetId` | `T_AmsNetId := '';                  (* keep empty '' for the local device *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |
| `ipErrorMessage` | `I_TcMessage` | — |

**Description:** In individual cases, this function block can be used as an alternative to the simpler FB_IPCDiag_WriteParameter [} 13] in order to write any desired parameters of the IPC diagnostics.

##### `F_IPCDiag_GetMdpIdx` — §3.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eModuleType` | `E_IPCDiag_ModuleType` | — |
| `nModuleIdx` | `USINT; (* 1= first instance of this module type (= default) *)` | — |
| `nTableID` | `BYTE` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fbRegister` | `FB_IPCDiag_Register` | — |

**Description:** The MDP (Modular Device Profile) information model lists all parameters with the help of areas, modules and tables. As the generic access to each parameter, an addressing is defined whose main component is the MDP index (2 bytes).
