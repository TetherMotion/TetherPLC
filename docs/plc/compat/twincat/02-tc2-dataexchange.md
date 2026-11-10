# Tc2_DataExchange

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_DataExchange_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T2 · **API entries:** 10
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Symbolic data exchange needs a named-variable transport (ADS-compatible or native).

---

#### Event driven data exchange

##### `FB_ReadAdsSymByName` — §3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bRead` | `BOOL` | The function block reads the content of the variables |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The transmission is active. |
| `bError` | `BOOL` | An error occurred during the transmission. |
| `nErrorId` | `UDINT` | ADS error number if an error has occurred. |

**Description:** The function block FB_ReadAdsSymByName enables reading of any value from another controller using the symbol name.

##### `FB_WriteAdsSymByName` — §3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bWrite` | `BOOL` | This function block writes the contents of the variable to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The transmission is active. |
| `bError` | `BOOL` | An error occurred during the transmission. |
| `nErrorId` | `UDINT` | ADS error number if an error has occurred. |

**Description:** The function block FB_WriteAdsSymByName enables writing of any value to another controller using the symbol name.

##### `FB_WriteBoolOnDelta` — §3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Enable function block. |
| `sNetId` | `T_AmsNetID` | AMS NetID of the ADS device to which the value is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The transmission is active. |
| `bLastSignal` | `BOOL` | Most recently transmitted value. |
| `bError` | `BOOL` | An error occurred during the transmission. |
| `nErrorId` | `UDINT` | ADS error number if an error has occurred. |
| `nErrorCnt` | `UDINT 12` | Number of failed transmission attempts. Event driven data exchange |

##### `FB_WriteByteOnDelta` — §3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Event driven data exchange` | `` | — |
| `bEnable` | `BOOL` | Enable function block. |
| `sNetId` | `T_AmsNetID` | AMS NetID of the ADS device to which the value is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The transmission is active. |
| `nLastSignal` | `BYTE` | Most recently transmitted value. |
| `bError` | `BOOL` | An error occurred during the transmission. |
| `nErrorId` | `UDINT` | ADS error number if an error has occurred. |
| `nErrorCnt` | `UDINT` | Number of failed transmission attempts. |

**Description:** The function block enables event-driven writing of a variable of type BYTE.

##### `FB_WriteWordOnDelta` — §3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Enable function block. |
| `sNetId` | `T_AmsNetID` | AMS NetID of the ADS device to which the value is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The transmission is active. |
| `nLastSignal` | `WORD` | Most recently transmitted value. |
| `bError` | `BOOL` | An error occurred during the transmission. |
| `nErrorId` | `UDINT` | ADS error number if an error has occurred. |
| `nErrorCnt` | `UDINT` | Number of failed transmission attempts. Event driven data exchange |

**Description:** The function block enables event-driven writing of a variable of type WORD.

##### `FB_WriteDWordOnDelta` — §3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `16 Event driven data exchange` | `` | — |
| `bEnable` | `BOOL` | Enable function block. |
| `sNetId` | `T_AmsNetID` | AMS NetID of the ADS device to which the value is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The transmission is active. |
| `nLastSignal` | `DWORD` | Most recently transmitted value. |
| `bError` | `BOOL` | An error occurred during the transmission. |
| `nErrorId` | `UDINT` | ADS error number if an error has occurred. |
| `nErrorCnt` | `UDINT` | Number of failed transmission attempts. |

**Description:** The function block enables event-driven writing of a variable of type DWORD.

##### `FB_WriteRealOnDelta` — §3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Enable function block. |
| `sNetId` | `T_AmsNetID` | AMS NetID of the ADS device to which the value is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `18 Event driven data exchange` | `` | — |
| `bBusy` | `BOOL` | The transmission is active. |
| `fLastSignal` | `REAL` | Most recently transmitted value. |
| `bError` | `BOOL` | An error occurred during the transmission. |
| `nErrorId` | `UDINT` | ADS error number if an error has occurred. |
| `nErrorCnt` | `UDINT` | Number of failed transmission attempts. |

**Description:** The function block enables event-driven writing of a variable of type REAL.

##### `FB_WriteLRealOnDelta` — §3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Event driven data exchange` | `` | — |
| `bEnable` | `BOOL` | Enable function block. |
| `sNetId` | `T_AmsNetID` | AMS NetID of the ADS device to which the value is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The transmission is active. |
| `fLastSignal` | `LREAL` | Most recently transmitted value. |
| `bError` | `BOOL` | An error occurred during the transmission. |
| `nErrorId` | `UDINT` | ADS error number if an error has occurred. |
| `nErrorCnt` | `UDINT` | Number of failed transmission attempts. |

**Description:** The function block enables event-driven writing of a variable of type LREAL.

#### Watchdog function blocks

##### `FB_CheckWatchdog` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Enable function block. |
| `tWatchdogTime` | `TIME` | Duration during which nCnt has to change. |
| `nCnt` | `UDINT` | Current counter value of the watchdog signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bWatchdog` | `BOOL` | FALSE indicates a valid monitoring signal. The output will |

**Description:** Monitoring of a watchdog signal, which is transmitted with the function block FB_WriteWatchdog [} 22].

##### `FB_WriteWatchdog` — §4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Enable function block. |
| `sNetId` | `T_AmsNetID` | AMS NetID of the ADS device to which the watchdog |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `22 Watchdog function blocks` | `` | — |
| `bBusy` | `BOOL` | The transmission is active. |
| `nLastCnt` | `BOOL` | Most recently transmitted counter value. |
| `bError` | `BOOL` | An error occurred during the transmission. |
| `nErrorId` | `UDINT` | ADS error number if an error has occurred. |

**Description:** Writing of a watchdog signal to another ADS device (TwinCAT PLC, Bus Terminal Controller, ...).
