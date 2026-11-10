# Tc2_SystemC69xx

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_SystemC69xx_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T1 · **API entries:** 3
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Panel variant of Tc2_System; same service needs.

---

#### Function Blocks

##### `FB_C69xxSetLedColor` — §3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | Switching of the LED with rising edge. |
| `eNewColor` | `E_UserLED_Color` | Color value to be set |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eLastSetColor` | `E_UserLED_Color` | Last color value set with this function block |
| `bError` | `BOOL` | Error while setting the LED color |

**Description:** The function block FB_C69xxSetLedColor can be used to switch the user LED on the C69xx PC. The LED color is switched via a rising edge at bExecute and the eNewColor parameter. The LED can be switched off (eNewColor = eULED_Off) or set to red (eNewColor = eULED_Red), blue (eNewColor = eULED_Blue) or green (eNewColor = eULED_Green). TYPE E_UserLED_Color : ( eULED_Off := 0, eULED_Red := 1, eULED_Blue := 2, eULED_Green := 3 ); END_TYPE

##### `FB_C69xxSetWatchdog` — §3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeOut` | `TIME` | Watchdog time, after which a restart is performed. |
| `bEnable` | `BOOL` | Enabling/disabling the watchdog. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnabled` | `BOOL` | TRUE = Watchdog is enabled, FALSE = Watchdog is |

**Description:** The function block FB_C69xxSetWatchdog enables a hardware watchdog on the C69xx PC. The watchdog is enabled via bEnable = TRUE and the tTimeOut time. The tTimeOut time can be a minimum of 2 seconds and a maximum of 255 seconds.

#### [Obsolete functions]

##### `F_GetVersionTcSystemC69xx` (FUNCTION) → `UINT` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement INT` | `Version element to be read. Possible parameters:` | — |

**Description:** This function can be used to read PLC library version information.
