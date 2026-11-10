# Tc2_SystemCX

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_SystemCX_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T1 · **API entries:** 32
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** CX-platform subset of Tc2_System; same service needs minus desktop-only calls.

---

#### Function Blocks › [obsolete]

##### `FB_CxGetDeviceIdentification` — §3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is executed with the rising edge. |
| `tTimeout` | `TIME` | States the time before the function is canceled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The data are read from the CX. After error-free |

**Description:** The function block FB_CxGetDeviceIdentification can be used to read device ID data of the CX.

##### `FB_CxGetDeviceIdentificationEx` — §3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is executed with a rising edge. |
| `tTimeout` | `TIME` | States the time before the function is canceled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The data are read from the CX. After error-free |

**Description:** The function block FB_CxGetDeviceIdentificationEx can be used to read device ID data of the CX. The function block is an extension of the function block FB_CxGetDeviceIdentification. The read device data are stored in the variable stDevIdent of type ST_CxDeviceIdentificationEx [} 42].

##### `FB_CX1010SetWatchdog` — §3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeOut` | `TIME` | Watchdog time, after which a restart is performed. |
| `bEnable` | `BOOL` | Activating/deactivating the watchdog. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnabled` | `BOOL` | TRUE = Watchdog is active, FALSE = Watchdog is not active. |
| `bError` | `BOOL` | Error when activating or deactivating the watchdog. |

**Description:** 10

##### `FB_CX1020SetWatchdog` — §3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeOut` | `TIME` | Watchdog time, after which a restart is performed. |
| `bEnable` | `BOOL` | Activating/deactivating the watchdog. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnabled` | `BOOL` | TRUE = Watchdog is active, FALSE = Watchdog is not active. |
| `bError` | `BOOL` | Error when activating or deactivating the watchdog. |

**Description:** The function block FB_CX1020SetWatchdog activates a hardware watchdog on the CX1020. The watchdog is activated via bEnable = TRUE and the tTimeOut time. The tTimeOut time can be a minimum of 2 seconds and a maximum of 255 seconds.

##### `FB_CX1030SetWatchdog` — §3.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeOut` | `TIME` | Watchdog time, after which a restart is performed. |
| `bEnable` | `BOOL` | Activating/deactivating the watchdog. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnabled` | `BOOL` | TRUE = Watchdog is active, FALSE = Watchdog is not active. |
| `bError` | `BOOL` | Error when activating or deactivating the watchdog. |

**Description:** The function block FB_CX1030SetWatchdog activates a hardware watchdog on the CX1030. The watchdog is activated via bEnable = TRUE and the tTimeOut time. The tTimeOut time can be a minimum of 2 seconds and a maximum of 255 seconds.

#### Function Blocks

##### `FB_CXProfiler` — §3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | A positive edge at this input starts the measurement of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This input is set at the start of the measuring procedure, and |

**Description:** The function block FB_CXProfiler can be used to measure the execution time of the PLC code under

##### `FB_CXReadKBusCycleUpdateTime` (FUNCTION_BLOCK) — §3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL;        // rising edge triggers read process` | — |
| `bReset` | `BOOL;        // set TRUE to reset the min. and max. values` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active and working. |
| `bError` | `BOOL` | The function block has an error. |
| `nErrorID` | `UDINT` | ADS error code |
| `nKBusMinUpdateTime UINT` | `Minimum update time in [µs] of the K-bus` | — |
| `nKBusMaxUpdateTime UINT` | `Maximum update time in [µs] of the K-bus` | — |
| `nKBusLastUpdateTime UINT` | `Last update time in [µs] of the K-bus` | — |
| `Development environment` | `Target platform` | PLC libraries to include |

**Description:** The function block can be used for all CXs that are operated directly with K-bus terminals. The function block determines the update time of the K-bus (K-bus runtime) with min. and max. values. The min. and max. values can be reset.

##### `FB_CXReadKBusError` (FUNCTION_BLOCK) — §3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is active and working. |
| `bError` | `BOOL` | The function block has an error. |
| `nErrorID` | `UDINT` | ADS error code |
| `bKBusError` | `BOOL` | The K-bus has an error. Further information is available at the outputs |

**Description:** The function block enables further information to be read out about a K-bus error in order to obtain a more precise error image and to enable better diagnosis.

##### `FB_CXSetTextDisplay` — §3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is executed with a rising edge. |
| `nDevID` | `UDINT` | Device ID of the CX1100 device. |
| `nMode` | `E_CX1100_DisplayModes` | Mode switching |
| `stLine` | `STRING` | String with 20 characters. This string is shown in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be accepted as long as |

**Description:** The functionblock FB_CXSetTextDisplay can be used to send text messages to the two line display of the CX1100.

##### `FB_CXSetTextDisplayUSB` — §3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is executed with a rising edge. |
| `sNetID` | `T_AmsNetID` | AMSNetID of the device |
| `nPort` | `T_AmsPort` | AmsPort of the device |
| `eMode` | `E_CX2100_DisplayModesRd` | Mode switching |
| `sLine1` | `STRING` | String with 80 characters. This string is displayed |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be accepted as long |

**Description:** The function block is used for the CX2100 and for the EL6090 terminal. When the function block is called, only the matching NetID and port number for the respective device has to be allocated.

##### `FB_CXGetTextDisplayUSB` — §3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The command is executed with a rising edge. |
| `sNetID` | `T_AmsNetID` | AMSNetID of the device |
| `nPort` | `T_AmsPort` | AmsPort of the device |
| `eMode` | `E_CX2100_DisplayModesRd` | Mode switching |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be accepted as long |

**Description:** The function block is used for the CX2100 and for the EL6090 terminal. When the function block is called, only the matching NetID and port number for the respective device has to be allocated.

##### `FB_CXSimpleUps` — §3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bDIPDisable` | `BOOL` | If bDIPDIsable = TRUE, then the position of the charge level switch |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPowerFailure` | `BOOL` | Becomes TRUE when a power failure of the supply voltage is |

**Description:** The function block FB_CXSimpleUps can be used on the CX1000 or CX1020 in order to activate the UPS CX1190-UPS from the PLC. In this case the UPS settings must be deactivated in the TwinCAT System Manager.

##### `FB_CX5010SetWatchdog` — §3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeOut` | `TIME` | Watchdog time, after which a restart is performed. |
| `bEnable` | `BOOL` | Activating/deactivating the watchdog. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnabled` | `BOOL` | TRUE = Watchdog is active, FALSE = Watchdog is not active. |
| `bError` | `BOOL` | Error when activating or deactivating the watchdog. |

**Description:** The function block FB_CX5010SetWatchdog activates a hardware watchdog on the CX5010. The watchdog is activated via bEnable = TRUE and the tTimeOut time. The tTimeOut time can be a minimum of 2 seconds and a maximum of 255 seconds.

##### `FB_CX5020SetWatchdog` — §3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeOut` | `TIME` | Watchdog time, after which a restart is performed. |
| `bEnable` | `BOOL` | Activating/deactivating the watchdog. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnabled` | `BOOL` | TRUE = Watchdog is active, FALSE = Watchdog is not active. |
| `bError` | `BOOL` | Error when activating or deactivating the watchdog. |

**Description:** The function block FB_CX5020SetWatchdog activates a hardware watchdog on the CX5020. The watchdog is activated via bEnable = TRUE and the tTimeOut time. The tTimeOut time can be a minimum of 2 seconds and a maximum of 255 seconds.

##### `FB_CX7000_LED_ERR` — §3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `set error` | `` | — |
| `bEnable` | `BOOL` | The function block controls the LED as soon as and as long |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;        // error flag` | — |
| `nErrorID` | `UDINT;       (* ADS Error ID. If nErrorID=DEVICE_SRVNOTSUPP probably the image` | — |

**Description:** The function block allows the use of the ERR LED on the CX7000. The function block is immediately active when it is called and controls the ERR LED via the mode.

##### `FB_CX7000_LED_WD` — §3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `set error` | `` | — |
| `bEnable` | `BOOL` | The function block controls the LED as soon as and as long |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;       // error flag` | — |
| `nErrorID` | `UDINT;      (* ADS Error ID. If nErrorID=DEVICE_SRVNOTSUPP probably the image v` | — |

**Description:** The function block allows the use of the WD LED on the CX7000. The function block is immediately active with the call and controls the WD LED via the mode.

##### `FB_CX7080_LED_ERR` — §3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rror` | `` | — |
| `bEnable` | `BOOL` | The function block activates the LED as soon as and as long as the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;             // error flag` | — |
| `nErrorID` | `UDINT;            (* ADS Error ID.` | — |

**Description:** You can use the WD/ERR LEDs of the CX7080 to display the states of the PLC program, communication, or other information.

##### `FB_CX7080_LED_WD` — §3.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rror` | `` | — |
| `bEnable` | `BOOL` | The function block activates the LED as soon as and as long as the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;             // error flag` | — |
| `nErrorID` | `UDINT;            (* ADS Error ID.` | — |

**Description:** You can use the WD/ERR LEDs of the CX7080 to display the states of the PLC program, communication, or other information.

##### `FB_CX70xx_RW_EEPROM` — §3.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A positive edge starts the function block. |
| `eMode` | `E_CX70xx_ ReadOnly: EEPROM read EEPROM_M WriteOnly: EEPROM write` | — |
| `ode` | `WriteAndRead: EEPROM write and read` | — |
| `pSrcBuf` | `PVOID` | Pointer to the data buffer to be written. |
| `cbSrcBufLen` | `UINT` | Length of data to be written (max. 120 bytes) |
| `pDstBuf` | `PVOID` | Pointer to the data buffer into which the contents of the EEPROM |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL;         // FB is working` | — |
| `bError` | `BOOL;         // FB has an Error` | — |
| `nErrorID` | `UDINT;        (* Error Code` | — |
| `nDataSizeEEPROM` | `UINT;          // current size of (read) EEPROM data in bytes (max.120 Bytes)` | — |
| `nWritesCycles` | `UINT;         // already performed EEPROM write cycles (maximum possible = 20` | — |

**Description:** The function block allows a maximum of 120 bytes to be written to the EEPROM (hardware) of the CX70xx. The EEPROM may be written to a maximum of 200 times. The memory is intended for one-time writing.

##### `FB_CX70xx_ResetOnBoardIO` (FUNCTION_BLOCK) — §3.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ommand` | `` | — |
| `bExecute` | `BOOL` | A positive edge starts the function block. |
| `sNetId` | `T_AmsNetID` | AMS Net ID of the OnBoard I/Os |
| `tTimeout` | `TIME` | States the length of the timeout that may not be exceeded by execution of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL;       // FB is working` | — |
| `bError` | `BOOL;       // FB has an Error` | — |
| `nErrorID` | `UDINT;      (* Error Code. If nErrorID=DEVICE_SRVNOTSUPP probably the image versio` | — |

**Description:** The function block allows to execute a reset from the OnBoard I/O of the CX70xx Embedded PC.

#### [obsolete]

##### `F_GetVersionTcCXSystem` — §4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | Version element to be read. Possible parameters: • 1: major number • 2: minor number • 3: revision number |
| `nVersionElement` | `INT 32` | Version element to be read. Possible parameters: • 1: major number • 2: minor number • 3: revision number |
| `nVersionElement` | `INT` | Version element to be read. Possible parameters: • 1: major number • 2: minor number • 3: revision number |
| `nVersionElement` | `INT` | Version element to be read. Possible parameters: • 1: major number • 2: minor number • 3: revision number |
| `nVersionElement` | `INT` | Version element to be read. Possible parameters: • 1: major number • 2: minor number • 3: revision number |
| `nVersionElement` | `INT` | Version element to be read. Possible parameters: • 1: major number • 2: minor number • 3: revision number |
| `nVersionElement` | `INT` | Version element to be read. Possible parameters: • 1: major number • 2: minor number • 3: revision number |

**Description:** The function returns library version info.

##### `F_CXSubTimeStamp` (FUNCTION) → `UDINT` — §4.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStampLoDW_A` | `UDINT; (* 2*32 bit time stamp A: low DWORD *)` | — |
| `nTimeStampHiDW_A` | `UDINT; (* 2*32 bit time stamp A: high DWORD *)` | — |
| `nTimeStampLoDW_B` | `UDINT; (* 2*32 bit time stamp B: low DWORD *)` | — |
| `nTimeStampHiDW_B` | `UDINT; (* 2*32 bit time stamp B: high DWORD *)` | — |
| `nTimeStampLoDW_A` | `lower 32bit of time stamp A.` | — |
| `nTimeStampHiDW_A` | `upper 32bit of time stamp A.` | — |
| `nTimeStampLoDW_B` | `lower 32bit of time stamp B.` | — |
| `nTimeStampHiDW_B` | `upper 32bit of time stamp B.` | — |

**Description:** The function F_CXSubTimeStamp executes a 64-bit subtraction time stamp A - time stamp B and converts the result to µs. The required 64-bit time stamps with 100 ns resolution can be read from the system with the function block GETCPUCOUNTER.

##### `F_CX1000SetWatchdog` (FUNCTION) → `BOOL` — §4.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeout` | `TIME` | — |
| `bEnable` | `BOOL` | — |

**Description:** Functions | [obsolete]

##### `F_CX9000SetWatchdog` (FUNCTION) → `BOOL` — §4.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeout` | `TIME` | — |
| `bEnable` | `BOOL` | — |

**Description:** The function F_CX9000SetWatchdog activates a hardware watchdog on the CX9000. The watchdog is activated via bEnable = TRUE and the tTimeOut time. The tTimeOut time can be a maximum of 65 seconds and 535 milliseconds.

##### `F_CX9010SetWatchdog` (FUNCTION) → `BOOL` — §4.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeout` | `TIME` | — |
| `bEnable` | `BOOL` | — |

**Description:** The function F_CX9010SetWatchdog activates a hardware watchdog on the CX9000. The watchdog is activated via bEnable = TRUE and the tTimeOut time. The tTimeOut time can be a maximum of 65 seconds and 535 milliseconds.

#### (top level)

##### `F_CXNaviSwitch` — §4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iCX1100_I USINT` | `Input value of the conversion N` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_CXNaviS E_CX1100_NaviSwitch` | `Output value of the conversion; input 'IN' witch` | — |

**Description:** The function F_CXNaviSwitch converts the value of the CX1100 navigation switch into an enum value of type E_CX1100_NaviSwitch.

##### `F_CXNaviSwitchUSB` (FUNCTION) → `E_CX2100_NaviSwitch` — §4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `icx2100_IN` | `USINT` | — |

**Description:** The function F_CXNaviSwitchUSB converts the value of the CX2100 navigation switch or an EL6090 into an enum value of type E_CX2100_NaviSwitch.

##### `F_CX81xx_ADDRESS` — §4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iCX_Typ` | `INT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_CX80xx_ADDRESS` | `: INT; 38` | — |
| `F_CX80xx_ INT` | `Position of the address selection switch;` | — |
| `ADDRESS` | `If -1, CX not implemented,` | — |

**Description:** This function reads the position of the address selection switch (DIP switch) of the CXxxxx. One possible application is that you can activate different program parts in the PLC depending on the switch position.

##### `F_CX8180_LED_ERR` — §4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `E_CX81x0_LED` | The operating mode of the LED, see also E_CX81x0_LED [} 44]. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ F_CX8180_LED_ERR` | `: INT;` | — |
| `F_CX8180 INT` | `-1: flash code not implemented;` | — |
| `_LED_ERR` | `0: OK` | — |

**Description:** As the CX8180 supports various protocols, the two LEDs WD and ERR on the CX8180 are not used by the firmware. This allows users to create their own diagnosis messages. The LEDs can be used to indicate, for example, whether the CX8180 has received an IP address from the DHCP server or whether devices are exchanging data.

##### `F_CX8180_LED_WD` — §4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `E_CX81x0_LED` | The operating mode of the LED, see also Data type E_CX81x0_LED |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ CX8180_LED_WD` | `: INT;` | — |
| `F_CX8180 INT` | `-1: flash code not implemented; 0: OK _LED_ERR` | — |

**Description:** Since the CX8180 supports various protocols, the two LEDs WD and ERR on the CX8180 are not used by the firmware. This allows users to create their own diagnosis messages. The LEDs can be used to indicate, for example, whether the CX8180 has received an IP address from the DHCP server or whether devices are exchanging data.

##### `F_CX8190_LED_ERR` — §4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `E_CX81x0_LED` | The operating mode of the LED, see also E_CX81x0_LED [} 44]. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ F_CX8190_LED_ERR` | `: INT;` | — |
| `F_CX8190 INT` | `-1: flash code not implemented; 0: OK _LED_ERR 40` | — |

**Description:** As the CX8190 supports various protocols, the two LEDs WD and ERR on the CX8190 are not used by the firmware. This allows users to create their own diagnosis messages. The LEDs can be used to indicate, for example, whether the CX8190 has received an IP address from the DHCP server or whether devices are exchanging data.

##### `F_CX8190_LED_WD` — §4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `E_CX81x0_LED` | The operating mode of the LED, see also E_CX81x0_LED [} 44]. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_ CX8190_LED_WD` | `: INT;` | — |
| `F_CX8190 INT` | `-1: flash code not implemented; 0: OK _LED_ERR` | — |

**Description:** Since the CX8190 supports various protocols, the two LEDs WD and ERR on the CX8190 are not used by the firmware. This allows users to create their own diagnosis messages. The LEDs can be used to indicate, for example, whether the CX8190 has received an IP address from the DHCP server or whether devices are exchanging data.
