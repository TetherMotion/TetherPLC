# Tc3_BA_Common

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_BA_Common_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T0 · **API entries:** 21
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** BA common types: shared type layer.

---

#### Programming › POUs › Controller

##### `FB_BA_PIDCtrl` — §4.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `udiOpMode` | `UDINT` | — |
| `bActn` | `BOOL` | — |
| `rKp` | `REAL` | — |
| `udiTn_ms` | `UDINT` | — |
| `udiTv_ms` | `UDINT` | — |
| `udiTd_ms` | `UDINT` | — |
| `rYMax` | `REAL` | — |
| `rYMin` | `REAL` | — |
| `rNZ` | `REAL` | — |
| `udiCycCl` | `UDINT` | — |
| `bSync` | `BOOL` | — |
| `rSync` | `REAL` | — |
| `bEn` | `Controller activation.` | — |
| `rW` | `Set value.` | — |
| `rX` | `Actual value.` | — |
| `udiOpMode` | `udiMode = 0: Controller with upstream proportional component, udiMode = 1: Controller in` | — |
| `bActn` | `Direction of action [} 14] of the controller.` | — |
| `rKp` | `Controller gain. Only affects the proportional component. Internally limited to a minimum value of 0.` | — |
| `udiTn_ms` | `Integral action time of the I component [ms]. A zero value at this parameter disables the I` | — |
| `udiTv_ms` | `Rate time of the D component [ms]. A zero value at this parameter disables the D component.` | — |
| `udiTd_ms` | `Damping time of the D component [s]. Internally limited to a minimum value of 0.` | — |
| `rYMax` | `Upper controller output limit. Selectable range: 0..100%.` | — |
| `rYMin` | `Lower controller output limit [%]. Selectable range: 0..100%. The value rYMin is upwardly limited by` | — |
| `rNZ` | `Neutral zone.` | — |
| `udiCycCl` | `Call cycle of the function block as a multiple of the cycle time. Internally limited to a minimum` | — |
| `Example` | `tTaskCycleTime = 20ms, udiCtrlCycleCall =10 -> The control algorithm is called every 200 ms.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rE` | `REAL` | — |
| `bARW` | `BOOL` | — |
| `rY` | `Control value. Range limited by rYMin and rYMax.` | — |
| `rE` | `Control deviation (The calculation depends on the direction of action [} 14]).` | — |
| `bARW` | `Anti-Reset-Windup function is active.` | — |

**Description:** Universal PID controller, alternatively in parallel structure or with upstream proportional component.

##### `FB_BA_SeqCtrl` — §4.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `udiOpMode` | `UDINT` | — |
| `bActn` | `BOOL` | — |
| `rKp` | `REAL` | — |
| `udiTn_ms` | `UDINT` | — |
| `udiTv_ms` | `UDINT` | — |
| `udiTd_ms` | `UDINT` | — |
| `rYMax` | `REAL` | — |
| `rYMin` | `REAL` | — |
| `rNZ` | `REAL` | — |
| `udiCycCl` | `UDINT` | — |
| `bSync` | `BOOL` | — |
| `rSync` | `REAL` | — |
| `rYSeqInit` | `REAL` | — |
| `udiMyNum` | `UDINT` | — |
| `bEn` | `Activation of the sequence controller` | — |
| `rW` | `Set value` | — |
| `rX` | `Actual value` | — |
| `udiOpMode` | `udiOpMode=0: Controller with upstream proportional component, udiOpMode = 1: Controller in` | — |
| `bActn` | `Direction of action reversal of the controller. For heating/cooling operation: bActn=FALSE` | — |
| `rKp` | `Controller gain. Only affects the proportional component. Internally limited to a minimum value of 0.` | — |
| `udiTn_ms` | `Integral action time of the I component [ms]. A zero value at this parameter disables the I` | — |
| `udiTv_ms` | `Rate time of the D component [ms]. A zero value at this parameter disables the D component.` | — |
| `udiTd_ms` | `Damping time of the D component [s]. Internally limited to a minimum value of 0.` | — |
| `rYMax` | `Upper controller output limit [%]. Selectable range: 0..100%.` | — |
| `rYMin` | `Lower controller output limit [%]. Selectable range: 0..100%. The value lrMin is upwardly limited by` | — |
| `rNZ` | `neutral zone (see Deadband diagram). Internally limited to a minimum value of 0. Mode of action is` | — |
| `udiCycCl` | `Call cycle of the function block as a multiple of the cycle time. Internally limited to a minimum` | — |
| `Example` | `tTaskCycleTime = 20 ms, udiCycCl = 10 -> the control algorithm is called every 200 ms. Thus the` | — |
| `rYSeqInit` | `Starting value of the controller after restart of the whole control sequence.` | — |
| `udiMyNum` | `Ordinal number of the sequence controller. Internally limited to values ranging from 0 to` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSeqLink` | `ST_BA_SeqLink` | — |
| `stSeqLink` | `Data and command structure (see ST_BA_SeqLink / ST_BA_SeqLinkData [} 40]) between the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rE` | `REAL` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `rY` | `Control value. Section: 0..100%, unless limited further by rYMin and rYMax.` | — |
| `rE` | `Control deviation (The calculation depends on the direction of action [} 14]).` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** PID controller as part of a sequence.

##### `FB_BA_SeqLink` — §4.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | — |
| `udiSttCtrl` | `UDINT` | — |
| `udiIniSwiOvrDly_sec` | `UDINT` | — |
| `rX` | `REAL` | — |
| `bEn` | `Activation of the sequence controller.` | — |
| `udiSttCtrl` | `Ordinal number of the sequence controller that should be the start controller upon general` | — |
| `udiIniSwiOvrDly_sec` | `The first controller remains active for at least this time [s] in the sequence before` | — |
| `rX` | `Actual value of the control.` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSeqLink` | `ST_BA_SeqLink` | — |
| `stSeqLink` | `Data and command structure (see ST_BA_SeqLink / ST_BA_SeqLinkData [} 40]) between the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `udiCurCtrl` | `UDINT` | — |
| `bSeqActv` | `BOOL` | — |
| `bNotRead` | `BOOL` | — |
| `bNoneOp` | `BOOL` | — |
| `udiRemTiIniSwiOvrDly_sec` | `UDINT` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `udiCurCtrl` | `Ordinal number of the currently active sequence controller. If no controller is active, 0 is output` | — |
| `bSeqActv` | `The sequence function block is enabled (bEn) and has no error resulting in switch-off, see error` | — |
| `bNotRead` | `Each sequence controller transfers data to the control function block via the structure stSeqLink.` | — |
| `bNoneOp` | `This output is switched to TRUE, if none of the sequence controller is enabled (bEn=TRUE).` | — |
| `udiRemTiIniSwiOvrDly_sec` | `Remaining initialization time [s] before switching for the first time (see` | — |
| `bErr` | `This output is switched to TRUE if the parameters entered are erroneous. This function block may not` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** This function block represents the higher-level control unit, which specifies which sequence controller is currently active.

#### Programming › POUs › Universal › Analog inputs/outputs

##### `FB_BA_KL32xxConfig` — §4.1.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `TI_usiState` | `USINT` | — |
| `TI_iDataIn` | `INT` | — |
| `bConfigurate` | `BOOL` | — |
| `bReadConfig` | `BOOL` | — |
| `eTerminal` | `E_BA_TERMINAL_KL` | — |
| `eSensor` | `E_BA_SENSOR` | — |
| `TI_usiState` | `Linking with the corresponding status byte of the Bus Terminal in the I/O area of the program.` | — |
| `TI_iDataIn` | `Linking with the corresponding raw data (Data In) of the Bus Terminal in the I/O area of the` | — |
| `bConfigurate` | `A rising edge starts the configuration of the Bus Terminal.` | — |
| `bReadConfig` | `A rising edge starts the reading of the Bus Terminal.` | — |
| `eTerminal` | `Selection of the respective Bus Terminal (see E_BA_Terminal_KL [} 41]).` | — |
| `eSensor` | `Selection of the sensor type (see E_BA_Sensor [} 42]).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TO_usiCtrl` | `USINT` | — |
| `TO_iDataOut` | `INT` | — |
| `usiState` | `USINT` | — |
| `iData` | `INT` | — |
| `rVal` | `REAL` | — |
| `bWireBreak` | `BOOL` | — |
| `bShortCircuit` | `BOOL` | — |
| `wTerminalType` | `WORD` | — |
| `wSpecialType` | `WORD` | — |
| `wFirmwareVersion` | `WORD` | — |
| `sDescription` | `STRING` | — |
| `sSensorType` | `STRING` | — |
| `bErr` | `BOOL` | — |
| `sErrDescr` | `T_MAXSTRING` | — |
| `TO_usiCtrl` | `Linking with the corresponding control byte of the Bus Terminal in the I/O area of the program.` | — |
| `TO_iDataOut` | `Linking with the corresponding raw data (Data Out) of the Bus Terminal in the I/O area of the` | — |
| `usiState` | `Output of the present terminal status.` | — |
| `iData` | `Output of the present process data.` | — |
| `rVal` | `Scaled output value.` | — |
| `bWireBreak` | `Display of the channel status, sensor wire breakage.` | — |
| `bShortCircuit` | `Display of the channel status, sensor short-circuit.` | — |
| `wTerminalType` | `Display of the terminal type.` | — |
| `wSpecialType` | `Display of the special version of the terminal.` | — |
| `wFirmwareVersion` | `Display of the terminal firmware.` | — |
| `sDescription` | `Display of the terminal type and firmware.` | — |
| `sSensorType` | `Display of the sensor type.` | — |
| `bErr` | `Error in the terminal configuration.` | — |
| `sErrDescr` | `Contains the error description.` | — |

**Description:** 22

#### Programming › POUs › Universal › Array

##### `FB_BA_DynamicArray` — §4.1.2.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReady` | `BOOL` | — |
| `discount` | `DINT` | — |
| `bReady` | `Status of the allocated memory. (TRUE if at least one entry is contained in the array and memory is` | — |
| `diCount` | `Current number of entries contained.` | — |

**Description:** The function block generates and deletes memory areas dynamically so that entries can be added and removed at runtime.

#### Programming › POUs › Universal › Array › FB_BA_DynamicArray

##### `AddEntry` — §4.1.2.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pEntry` | `PVOID` | — |
| `pEntry` | `Pointer to the entry to be added.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddEntry` | `BOOL` | — |
| `diResultIndex` | `DINT` | — |
| `AddEntry` | `Result of the function.` | — |
| `diResultIndex` | `Index position of the entry added.` | — |

**Description:** Creates a new data set at the end of the array and copies the content of the specified entry to the internal memory.

##### `FindEntry` — §4.1.2.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pEntry` | `PVOID` | — |
| `pEntry` | `Pointer to the entry sought.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindEntry` | `BOOL` | — |
| `diResultIndex` | `DINT` | — |
| `FindEntry` | `Result of the function.` | — |
| `diResultIndex` | `Index position of the entry added.` | — |

**Description:** Determines the position of the specified entry in the array by comparing its content with the data sets of the array.

##### `GetEntry` — §4.1.2.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `diIndex` | `DINT` | — |
| `pResultEntry` | `PVOID` | — |
| `diIndex` | `Index position of the data set to be output.` | — |
| `pResultEntry` | `Pointer to the memory area that is to be used to output the data record.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetEntry` | `BOOL` | — |
| `GetEntry` | `Result of the function.` | — |

**Description:** Copies the contents of the data set to a certain position in the specified memory area.

##### `GetEntryEx` — §4.1.2.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `diIndex` | `DINT` | — |
| `diIndex` | `Index position of the data set to be output.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetEntryEx` | `BOOL` | — |
| `pMemoryPtr` | `POINTER TO PVOID` | — |
| `GetEntry` | `Result of the function.` | — |
| `pMemoryPtr` | `Pointer that is to be used to output the data set.` | — |

**Description:** Determines a pointer to the internal memory of the specified data set.

##### `RemoveEntry` — §4.1.2.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `diIndex` | `DINT` | — |
| `diIndex` | `Index position of the data set to be removed.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveEntry` | `BOOL` | — |
| `RemoveEntry` | `Result of the function.` | — |

**Description:** Removes the data set at the specified index position from the array.

##### `RemoveEntryEx` — §4.1.2.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pEntry` | `PVOID` | — |
| `pEntry` | `Pointer to the entry to be removed.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveEntryEx` | `BOOL` | — |
| `RemoveEntryEx` | `Result of the function.` | — |

**Description:** Determines the position of the specified entry and deletes it from the array.

##### `SetEntry` — §4.1.2.2.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `diIndex` | `DINT` | — |
| `pEntry` | `PVOID` | — |
| `diIndex` | `Index position of the data set to be replaced.` | — |
| `pEntry` | `Pointer to the entry to be removed.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetEntry` | `BOOL` | — |
| `SetEntry` | `Result of the function.` | — |

**Description:** Replaces the existing data set with a new one by overwriting the internal memory area of the existing data set with the value of the new entry.

##### `AddEntryPtr` — §4.1.2.2.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pEntry` | `PVOID` | — |
| `pEntry` | `Pointer to the entry to be added.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddEntryPtr` | `BOOL` | — |
| `diResultIndex` | `DINT` | — |
| `AddEntryPtr` | `Result of the function.` | — |
| `diResultIndex` | `Index position of the entry added.` | — |

**Description:** Creates a new entry at the end of the array and copies its memory address (i.e. the address to which the pointer pEntry points) to the internal memory.

##### `FindEntryPtr` — §4.1.2.2.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pEntry` | `PVOID` | — |
| `pEntry` | `Pointer to the entry sought.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindEntryPtr` | `BOOL` | — |
| `diResultIndex` | `DINT` | — |
| `FindEntryPtr` | `Result of the function.` | — |
| `diResultIndex` | `Index position of the entry sought.` | — |

**Description:** Determines the position of an entry in the array by comparing its address with the addresses stored in the array.

##### `GetEntryExPtr` — §4.1.2.2.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `diIndex` | `DINT` | — |
| `diIndex` | `Index position of the entry to be output.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetEntryExPtr` | `BOOL` | — |
| `pEntryPtr` | `POINTER TO PVOID` | — |
| `GetEntryExPtr` | `Result of the function.` | — |
| `pEntryPtr` | `Pointer that is to be used to output the entry.` | — |

##### `RemoveEntryExPtr` — §4.1.2.2.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pEntry` | `PVOID` | — |
| `pEntry` | `Pointer to the entry to be removed.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveEntryExPtr` | `BOOL` | — |
| `RemoveEntryExPtr` | `Result of the function.` | — |

**Description:** Determines the position of the specified entry and deletes it from the array.

##### `SetEntryPtr` — §4.1.2.2.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `diIndex` | `DINT` | — |
| `pEntry` | `PVOID` | — |
| `diIndex` | `Index position of the entry to be replaced.` | — |
| `pEntry` | `Pointer to the entry to be replaced.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetEntryPtr` | `BOOL` | — |
| `SetEntryPtr` | `Result of the function.` | — |

**Description:** Replaces an existing entry by a new one by overwriting the memory address of the existing entry with the memory address of the new entry.

#### Programming › POUs › Universal › Log

##### `FB_BA_LogMessage` (FUNCTION_BLOCK) — §4.1.2.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sResult` | `T_MaxSTRING` | — |
| `sResult` | `Content of the message last output.` | — |

**Description:** The function block outputs messages in the TwinCAT display window.

#### Programming › POUs › Universal › Log › FB_BA_LogMessage

##### `Show` — §4.1.2.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dLogType` | `DWORD` | — |
| `sLogCode` | `T_MaxString` | — |
| `sLogText` | `T_MaxString` | — |
| `bIgnoreBlock` | `BOOL` | — |
| `dLogType` | `Log type of the message to be displayed.` | — |
| `sLogCode` | `Optional, context-related additional information [} 34].` | — |
| `sLogText` | `Content of the message.` | — |
| `bIgnoreBlock` | `Prevents the suppression of cyclically repeated messages [} 34].` | — |
| `dLogType` | `DWORD` | — |
| `sLogCode` | `T_MaxString` | — |
| `sLogText` | `T_MaxString` | — |
| `tArg1` | `T_Arg` | — |
| `bIgnoreBlock` | `BOOL` | — |
| `dLogType` | `Log type of the message to be displayed.` | — |
| `sLogCode` | `Optional, context-related additional information [} 34].` | — |
| `sLogText` | `Content of the message.` | — |
| `tArg1` | `Value to be formatted (see T_Arg).` | — |
| `bIgnoreBlock` | `Prevents the suppression of cyclically repeated messages [} 34].` | — |
| `dLogType` | `DWORD` | — |
| `sLogCode` | `T_MaxString` | — |
| `sLogText` | `T_MaxString` | — |
| `tArg1` | `T_Arg` | — |
| `tArg2` | `T_Arg` | — |
| `bIgnoreBlock` | `BOOL` | — |
| `dLogType` | `Log type of the message to be displayed.` | — |
| `sLogCode` | `Optional, context-related additional information [} 34].` | — |
| `sLogText` | `Content of the message.` | — |
| `tArg1` | `First value to be formatted (see T_Arg).` | — |
| `tArg2` | `Second value to be formatted (see T_Arg).` | — |
| `bIgnoreBlock` | `Prevents the suppression of cyclically repeated messages [} 34].` | — |
| `dLogType` | `DWORD` | — |
| `sLogCode` | `T_MaxString` | — |
| `sLogText` | `T_MaxString` | — |
| `tArg1` | `T_Arg` | — |
| `tArg2` | `T_Arg` | — |
| `tArg3` | `T_Arg` | — |
| `bIgnoreBlock` | `BOOL` | — |
| `dLogType` | `Log type of the message to be displayed.` | — |
| `sLogCode` | `Optional, context-related additional information [} 34].` | — |
| `sLogText` | `Content of the message.` | — |
| `tArg1` | `First value to be formatted (see T_Arg).` | — |
| `tArg2` | `Second value to be formatted (see T_Arg).` | — |
| `tArg3` | `Second value to be formatted (see T_Arg).` | — |
| `bIgnoreBlock` | `Prevents the suppression of cyclically repeated messages [} 34].` | — |
| `dLogType` | `DWORD` | — |
| `sLogCode` | `T_MaxString` | — |
| `sLogText` | `T_MaxString` | — |
| `tArg1` | `T_Arg` | — |
| `tArg2` | `T_Arg` | — |
| `tArg3` | `T_Arg` | — |
| `tArg4` | `T_Arg` | — |
| `bIgnoreBlock` | `BOOL` | — |
| `dLogType` | `Log type of the message to be displayed.` | — |
| `sLogCode` | `Optional, context-related additional information [} 34].` | — |
| `sLogText` | `Content of the message.` | — |
| `tArg1` | `First value to be formatted (see T_Arg).` | — |
| `tArg2` | `Second value to be formatted (see T_Arg).` | — |
| `tArg3` | `Second value to be formatted (see T_Arg).` | — |
| `tArg4` | `Second value to be formatted (see T_Arg).` | — |
| `bIgnoreBlock` | `Prevents the suppression of cyclically repeated messages [} 34].` | — |
| `dLogType` | `DWORD` | — |
| `sLogCode` | `T_MaxString` | — |
| `sLogText` | `T_MaxString` | — |
| `tArg1` | `T_Arg` | — |
| `tArg2` | `T_Arg` | — |
| `tArg3` | `T_Arg` | — |
| `tArg4` | `T_Arg` | — |
| `tArg5` | `T_Arg` | — |
| `bIgnoreBlock` | `BOOL` | — |
| `dLogType` | `Log type of the message to be displayed.` | — |
| `sLogCode` | `Optional, context-related additional information [} 34].` | — |
| `sLogText` | `Content of the message.` | — |
| `tArg1` | `First value to be formatted (see T_Arg).` | — |
| `tArg2` | `Second value to be formatted (see T_Arg).` | — |
| `tArg3` | `Second value to be formatted (see T_Arg).` | — |
| `tArg4` | `Second value to be formatted (see T_Arg).` | — |
| `tArg5` | `Second value to be formatted (see T_Arg).` | — |
| `bIgnoreBlock` | `Prevents the suppression of cyclically repeated messages [} 34].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Show` | `BOOL` | — |
| `Show` | `Indicator of whether a message was output (TRUE) or discarded (FALSE).` | — |
| `Show1` | `BOOL` | — |
| `Show1` | `Indicator of whether a message was output (TRUE) or discarded (FALSE).` | — |
| `Show2` | `BOOL` | — |
| `Show2` | `Indicator of whether a message was output (TRUE) or discarded (FALSE).` | — |
| `Show3` | `BOOL` | — |
| `Show3` | `Indicator of whether a message was output (TRUE) or discarded (FALSE).` | — |
| `Show4` | `BOOL` | — |
| `Show4` | `Indicator of whether a message was output (TRUE) or discarded (FALSE).` | — |
| `Show5` | `BOOL` | — |
| `Show5` | `Indicator of whether a message was output (TRUE) or discarded (FALSE).` | — |

#### Programming › POUs › Universal › Trigger

##### `FB_BA_ATrigCOV` — §4.1.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `xValue` | `ANY` | — |
| `bForce` | `BOOL` | — |
| `xValue` | `Value to be monitored.` | — |
| `bForce` | `Forces a positive comparison ("bQ=TRUE").` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReady` | `BOOL` | — |
| `bQ` | `BOOL` | — |
| `bReady` | `Indicates operability:` | — |
| `bQ` | `Result of the last comparison (TRUE if the value has changed).` | — |

**Description:** The function block monitors the value xValue for changes (Change of Value).

##### `FB_BA_RFTrig` — §4.1.2.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bValue` | `BOOL` | — |
| `bValue` | `Value to be monitored.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | — |
| `Qr` | `BOOL` | — |
| `Qf` | `BOOL` | — |
| `Q` | `TRUE if an edge is detected.` | — |
| `Qr` | `Result of the last comparison (TRUE as soon as the monitored value changes from FALSE to TRUE).` | — |
| `Qf` | `Result of the last comparison (TRUE as soon as the monitored value changes from TRUE to FALSE).` | — |

**Description:** Function block for the detection of a rising or falling edge on a Boolean variable. The use of the separate function blocks R_TRIG and F_TRIG can be avoided with the block.
