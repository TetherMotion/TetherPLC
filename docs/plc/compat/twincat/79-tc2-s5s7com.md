# Tc2_S5S7Com

- **Manual:** `TF6610_TC3_S5S7_Communication_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 5
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Siemens S7 protocol (RFC1006): socket service + S7 packet codec.

---

#### API › PLC Reference › Function blocks

##### `FB_NLinkOpen` — §6.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stSettings` | `ST_NLinkSettings := ( sHost := '127.0.0.1', nPort := 2002 )` | — |
| `nOpt` | `DWORD := 0` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME := T#45s` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hLink` | `T_HNLINK` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** This function block (actively) establishes a network connection to the third-party system. If successful, the internal parameters of the hLink variable are initialised. The same instance of the hLink variable must then be transferred to the other instances of the communication blocks FB_NLinkFetch [} 22], FB_NLinkWrite [} 24] or FB_NLinkClose [} 21]. In a PLC application one instance of this variables is required for each network connection (TwinCAT system<->Ethernet LAN adapter<->external controller).

##### `FB_NLinkClose` — §6.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOpt` | `DWORD := 0` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hLink` | `T_HNLINK` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** This function block closes the network connection to the third-party system.

##### `FB_NLinkFetch` — §6.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `devAddr` | `BYTE := 2` | — |
| `eArea` | `E_NLinkArea := eNLinkArea_DB` | — |
| `nDB` | `WORD := 0` | — |
| `nOffset` | `WORD := 0` | — |
| `cbBuffer` | `UDINT(0..MAX_NLink_DATALENGTH) := 0` | — |
| `pBuffer` | `POINTER TO BYTE := 0` | — |
| `nOpt` | `DWORD := 0` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hLink` | `T_HNLINK` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `cbRead` | `UDINT := 0; (* Number of recend realy returned data bytes *)` | — |

**Description:** This function block reads data from the third-party system and writes them to the local TwinCAT system (FETCH service).

##### `FB_NLinkWrite` — §6.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `devAddr` | `BYTE := 2` | — |
| `eArea` | `E_NLinkArea := eNLinkArea_DB` | — |
| `nDB` | `WORD := 0` | — |
| `nOffset` | `WORD := 0` | — |
| `cbBuffer` | `UDINT(0..MAX_NLink_DATALENGTH) := 0` | — |
| `pBuffer` | `POINTER TO BYTE := 0` | — |
| `nOpt` | `DWORD := 0` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME := DEFAULT_ADS_TIMEOUT` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hLink` | `T_HNLINK` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `cbWrite` | `UDINT := 0;(* Number of successfull written data bytes *)` | — |

**Description:** This function block sends data from the local TwinCAT system to the third-party system (WRITE service).

#### API › PLC Reference › Functions [obsolete]

##### `F_GetVersionTcS5S7Comm` (FUNCTION) → `UINT` — §6.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | — |

**Description:** This function can be used to read PLC library version information.
