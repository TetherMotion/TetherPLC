# Tc2_EthernetIP

- **Manual:** `TF6281_TC3_EthernetIP_Scanner_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 7
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** EtherNet/IP: socket service + CIP/EIP encapsulation + implicit I/O.

---

#### PLC API › Function blocks

##### `FB_GET_ATTRIBUTE_SINGLE` — §6.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_GET_ATTRIBUTE_SINGLE enables reading of parameters from an EtherNet/IP device.

##### `FB_SET_ATTRIBUTE_SINGLE` — §6.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | AMSNetId of the TwinCAT EtherNet/IP™ scanner via which the command is to run |
| `sIPv4Addr` | `T_IPv4Addr` | IP address of the target device. |
| `bExecute` | `BOOL` | A positive edge starts the command. |
| `nClass` | `WORD` | Class number of the CIP service |
| `nInstance` | `WORD` | Instance number of the CIP service |
| `nAttribute` | `WORD` | Attribute number of CIP service |
| `pSrc` | `POINTER TO BYTE` | Pointer to the variable that contains the value for sending the service (get the pointer with ADR) |
| `nSrcDataLen` | `WORD` | Size of the variable to which the pointer pSrc points (get with SizeOf) |
| `nSessionTimeoutMS DWORD` | `Timeout for the session; the default is 30 seconds ec` | — |
| `nCmdTimeoutMSec DWORD` | `Timeout for the command; the default is 7.5 seconds` | — |
| `bRackComm` | `BOOL` | TRUE if the CPU is modular, i.e. a CPU with a rack design, for example a CompactLogix |
| `nPort` | `BYTE` | Port number of the CPU (the TF6281 currently only supports port 1) |
| `nSlot` | `BYTE` | Slot number if the CPU is not plugged into slot 0 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The function block FB_SET_ATTRIBUTE_SINGLE enables writing of parameters in an EtherNet/IP device.

##### `FB_CUSTOM_SERVICE` — §6.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | — |
| `sIPv4Addr` | `T_IPv4Addr` | — |
| `bExecute` | `BOOL` | — |
| `nServiceCode` | `BYTE` | — |
| `nClass` | `WORD` | — |
| `nInstance` | `WORD` | — |
| `nAttribute` | `WORD` | — |
| `pDst` | `POINTER TO BYTE` | — |
| `nMaxLen` | `WORD` | — |
| `pSrc` | `POINTER TO BYTE` | — |
| `nSrcDataLen` | `WORD` | — |
| `nSessionTimeoutMSec` | `DWORD` | — |
| `nCmdTimeoutMSec` | `DWORD` | — |
| `bRackComm` | `BOOL` | — |
| `nPort` | `BYTE` | — |
| `nSlot` | `BYTE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** You can create almost any CIP service using the FB_CUSTOM_SERVICE function block.

##### `FB_CIP_DATA_TABLE_RDWR` — §6.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | — |
| `sIPv4Addr` | `T_IPv4Addr` | — |
| `bExecute` | `BOOL` | — |
| `bDataTableWrite` | `BOOL` | — |
| `sSrcElementName` | `WORD` | — |
| `sDstElementName` | `WORD` | — |
| `nNumberOfElements` | `POINTER TO BYTE` | — |
| `nLocalIndex` | `WORD` | — |
| `nRemoteIndex` | `DWORD` | — |
| `nSessionTimeoutMSec` | `DWORD` | — |
| `nCmdTimeoutMSec` | `DWORD` | — |
| `bRackComm` | `BOOL` | — |
| `nPort` | `BYTE` | — |
| `nSlot` | `BYTE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** Variables are read and written from TwinCAT via a function block that is part of the Tc2_EthernetIP.

#### PLC API › Functions

##### `RSL5KSTRING_TO_STRING` (FUNCTION) → `STRING` — §6.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `RSL5K_STRING` | — |

**Description:** The function converts an RSL5KString value [} 61] to a string value.

##### `STRING_TO_RSL5KSTRING` (FUNCTION) → `RSL5K_STRING` — §6.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `STRING(82)` | — |

**Description:** This function converts an RSL5KString value [} 61] to a string value

##### `F_GET_ETHERNETIP_ERROR_TEXT` (FUNCTION) → `STRING` — §6.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nErrorId` | `UDINT` | — |

**Description:** This function returns a descriptive text based on an error number.
