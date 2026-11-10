# Tc3_PowerMonitoring

- **Manual:** `TF3650_TC3_Power_Monitoring_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Measurement · **Tier:** T1 · **API entries:** 133
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Power calculations T0; instrument acquisition needs protocol drivers (Modbus/S7/IEC61850).

---

#### PLC API › General function blocks

##### `FB_PMA_Scaling` (FUNCTION) — §5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_PMA_Scaling_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Scaling is used for scaling raw values. The raw values can be scaled individually or as an array, for example as oversampling values. In addition, it is possible to use single-phase and three-phase input signals.

#### PLC API › General function blocks › FB_PMA_Scaling

##### `Call_1Ph_16Bit` (METHOD) → `BOOL` — §5.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputBuffer_U` | `POINTER TO INT` | Pointer to an array of voltage values. These can be added individually or as an oversampling |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call_1Ph_16Bit` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called to scale the 16-bit input data of type INT according to the configured parameters.

##### `Call_1Ph_32Bit` (METHOD) → `BOOL` — §5.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputBuffer_U` | `POINTER TO DINT` | Pointer to an array of voltage values. These can be added individually or as an oversampling |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call_1Ph_32Bit` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called to scale the 32-bit input data of type DINT according to the configured parameters.

##### `Call_3Ph_16Bit` (METHOD) → `BOOL` — §5.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputBuffer_UL1 .. UL3 POINTER TO INT` | `Pointer to an array of voltage values. These can be added individually or as an oversampling` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call_3Ph_16Bit` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called to scale the 16-bit input data of type INT according to the configured parameters.

##### `Call_3Ph_32Bit` (METHOD) → `BOOL` — §5.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputBuffer_UL1 .. UL3 POINTER TO DINT` | `Pointer to an array of voltage values. These can be added individually or as an oversampling` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call_3Ph_32Bit` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called to scale the 32-bit input data of type DINT according to the configured parameters.

##### `Init` (METHOD) → `BOOL` — §5.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_PMA_Scaling_InitPars [} 126] Function block-specific structure with initialization parameters` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference).

##### `Reconfigure` (METHOD) → `BOOL` — §5.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fOffsetVoltage` | `LREAL` | Indicates a user-defined offset for the voltage |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The method is called in order to reconfigure the function block during the runtime.

#### PLC API › General function blocks

##### `Call_1Ph` (METHOD) → `BOOL` — §5.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputBuffer_U` | `POINTER TO INT` | Pointer to an array of voltage values. These can be added individually or as an oversampling |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call_1Ph` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called to scale the input data in a single-phase system of type INT according to the configured parameters. LREAL is output.

##### `Call_3Ph` (METHOD) → `BOOL` — §5.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputBuffer_UL1 .. UL3 POINTER TO INT` | `Pointer to an array of voltage values. These can be added individually or as an oversampling` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call_3Ph` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called to scale the input data in a three-phase system of type INT according to the configured parameters. LREAL is output.

##### `Init` (METHOD) → `BOOL` — §5.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `Function block-specific structure with initialization ST_PMA_Scaling_EL3773_InitPar parameters s [} 125]` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference).

##### `Reconfigure` (METHOD) → `BOOL` — §5.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fOffsetVoltage` | `LREAL` | Indicates a user-defined offset for the voltage |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The method is called in order to reconfigure the function block during the runtime.

##### `Call_1Ph` (METHOD) → `BOOL` — §5.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputBuffer_U` | `POINTER TO INT` | Pointer to an array of voltage values. These can be added individually or as an oversampling |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call_1Ph` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called to scale the input data in a single-phase system of type INT according to the configured parameters. LREAL is output.

##### `Call_1Ph_Autorange` (METHOD) → `BOOL` — §5.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputBuffer_U` | `POINTER TO INT` | Pointer to an array of voltage values. These can be added individually or as an oversampling |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call_1Ph_Autorange` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called to scale the input data in a single-phase system of type INT according to the configured parameters. LREAL is output. The EL3783 operates in Autorange mode.

##### `Call_3Ph` (METHOD) → `BOOL` — §5.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputBuffer_UL1 .. UL3 POINTER TO INT` | `Pointer to an array of voltage values. These can be added individually or as an oversampling` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call_3Ph` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called to scale the input data in a three-phase system of type INT according to the configured parameters. LREAL is output.

##### `Call_3Ph_Autorange` (METHOD) → `BOOL` — §5.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputBuffer_UL1 .. UL3 POINTER TO INT` | `Pointer to an array of voltage values. These can be added individually or as an oversampling` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call_3Ph_Autorange` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called to scale the input data in a three-phase system of type INT according to the configured parameters. LREAL is output. The EL3783 operates in Autorange mode.

##### `Init` (METHOD) → `BOOL` — §5.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_PMA_Scaling_EL3783_InitPar Function block-specific structure with initialization s [} 126]` | parameters |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference).

##### `Reconfigure` (METHOD) → `BOOL` — §5.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fOffsetVoltage` | `LREAL` | Indicates a user-defined offset for the voltage |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

##### `FB_PMA_TaskTransfer_Send` (FUNCTION) — §5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `aDestIDs` | `ARRAY[1..GVL_PMA.cMA_MaxDest] OF UDINT` | — |
| `nResultBuffers` | `UDINT := 4` | — |
| `tTransferTimeout` | `LTIME := LTIME#40US` | — |
| `stInitPars` | `ST_PMA_TaskTransfer_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_TaskTransfer_Send can be used to transfer data from a slow task back to a fast task. This may be necessary if computationally intensive algorithms were swapped out to a slow task and the results are required in another task.

#### PLC API › General function blocks › FB_PMA_TaskTransfer_Send

##### `Call` (METHOD) → `HRESULT` — §5.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInputData` | `POINTER TO LREAL` | Pointer to an input buffer of the data to be sent. |
| `nDataInSize` | `UDINT` | Indicates the size of the input buffer in bytes. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to write the values to the output buffer. The output buffer is sent cyclically.

##### `Init` (METHOD) → `HRESULT` — §5.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

#### PLC API › General function blocks

##### `FB_PMA_TaskTransfer_Receive` (FUNCTION) — §5.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_TaskTransfer_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_TaskTransfer_Receive can be used to transfer data from a slow task back to a fast task. This may be necessary if computationally intensive algorithms were swapped out to a slow task and the results are required in another task.

#### PLC API › General function blocks › FB_PMA_TaskTransfer_Receive

##### `Call` (METHOD) → `HRESULT` — §5.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pOutputData` | `POINTER TO LREAL` | Pointer to the output buffer. |
| `nDataOutSize` | `UDINT` | Indicates the size of the output buffer in bytes. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to read the values from the input buffer when new data is present. The function block waits for input data if the method outputs neither new results nor an error. This is a regular behavior in the process of the analysis chain.

##### `Init` (METHOD) → `HRESULT` — §5.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

#### PLC API › General function blocks

##### `FB_PMA_Protection` (FUNCTION) — §5.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_PMA_Protection_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The FB_PMA_Protection function block is used to monitor network data or results using thresholds and hysteresis. Any number of input channels can be monitored simultaneously. An additional output summarizes whether at least one threshold value has been exceeded or not reached.

#### PLC API › General function blocks › FB_PMA_Protection

##### `Call` (METHOD) → `BOOL` — §5.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |
| `pArrayOut` | `POINTER TO BOOL` | Pointer to an array in which the results of the threshold |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Init` (METHOD) → `BOOL` — §5.1.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_PMA_Protection_InitPars` | Function-block-specific structure with initialization [} 128] parameters |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference).

##### `Reconfigure` (METHOD) → `BOOL` — §5.1.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpoint` | `LREAL` | Setpoint of the input channels to be monitored |
| `fOffsetVoltage` | `LREAL` | Minimum threshold value. fSetpoint * fMinThreshold |
| `fGainVoltage` | `LREAL` | Hysteresis of the minimum threshold value. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

##### `Reset` (METHOD) → `BOOL` — §5.1.6.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** The current calculations are reset with the method.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index |
| `fIn` | `LREAL` | Input value of the channel. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks, single-phase

##### `FB_PMA_Source_1Ph` (FUNCTION) — §5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `aDestIDs` | `ARRAY[1..GVL_PMA.cMA_MaxDest] OF UDINT` | — |
| `nResultBuffers` | `UDINT := 4` | — |
| `tTransferTimeout` | `LTIME := LTIME#40US` | — |
| `stInitPars` | `ST_PMA_Source_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Source_1Ph writes data from an external PLC data buffer into a multi-array buffer.

#### PLC API › Function blocks, single-phase › FB_PMA_Source_1Ph

##### `Call` (METHOD) → `BOOL` — §5.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBuffer_U` | `POINTER TO LREAL` | Pointer to an array of voltage values. These can be added individually or as an oversampling |

**Description:** The method is called in each cycle to write the values to the output buffer. The output buffer is sent as soon as it is filled.

##### `ResetData` (METHOD) → `BOOL` — §5.2.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ResetData` | `BOOL` | Indicates whether the method was executed |

**Description:** This method can be used to reset the data currently in the buffer.

##### `Init` (METHOD) → `BOOL` — §5.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `ResetAnalysisChain` (METHOD) → `BOOL` — §5.2.1.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ResetAnalysisChain` | `BOOL` | Indicates whether the method was executed |

**Description:** Calling this method causes an automatic reset of all algorithms in the full analysis chain. Internally, a ResetData() is carried out each time before accepting the new data set.

#### PLC API › Function blocks, single-phase › Based on the signal period

##### `FB_PMA_Frequency_Period_1Ph` (FUNCTION) — §5.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_Frequency_Period_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Frequency_Period_1Ph calculates the base frequency of the given input signal. To do this, the signal is first filtered with a Butterworth low-pass filter. The zero crossings of the input signal are then determined from the filtered values, and the frequency is calculated from their difference. The results refer to one or more periods, depending on the configuration. The statistical results refer to the entire runtime or the time at which the statistical results were last reset.

#### PLC API › Function blocks, single-phase › Based on the signal period › FB_PMA_Frequency_Period_1Ph

##### `Call` (METHOD) → `BOOL` — §5.2.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetStatistics` | `BOOL` | TRUE resets the minimum and maximum values |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.2.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.2.2.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_1Ph [} 40] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reconfigure` (METHOD) → `BOOL` — §5.2.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fMinFreq` | `LREAL` | Minimum expected measuring frequency |
| `fMaxFreq` | `LREAL` | Maximum expected measuring frequency |
| `nPeriods` | `UDINT` | Number of periods that influence the calculation. (Period length = sample rate/frequency) |
| `nFilterOrder` | `UINT` | Indicates the order of the low-pass filter. The |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

##### `Reset` (METHOD) → `BOOL` — §5.2.2.1.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** The current calculations are reset with the method.

#### PLC API › Function blocks, single-phase › Based on the signal period

##### `FB_PMA_BasicValues_Period_1Ph` (FUNCTION) — §5.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_BasicValues_Period_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_BasicValues_Period_1Ph calculates analysis values for the signal sequence of current and voltage in a single-phase system. These include the mean value, the RMS value, the peak value, the rectified value, the crest factor and the form factor for current and voltage. The results refer to a configurable number of signal periods. The period duration refers to the frequency specified at the start of the period at the input of the Call [} 50] method. The statistical results refer to the entire runtime or the time at which the statistical results were last reset.

#### PLC API › Function blocks, single-phase › Based on the signal period › FB_PMA_BasicValues_Period_1Ph

##### `Init` (METHOD) → `BOOL` — §5.2.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `Call` (METHOD) → `BOOL` — §5.2.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fFreq` | `LREAL` | Current frequency of the input signal. Is used to determine the length of the period at the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `PassInputs` (METHOD) → `BOOL` — §5.2.2.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_1Ph [} 40] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reconfigure` (METHOD) → `BOOL` — §5.2.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fMinInputCurrent` | `LREAL` | Minimum input value (RMS) of the current. This |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

##### `Reset` (METHOD) → `BOOL` — §5.2.2.2.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** The current calculations are reset with the method.

#### PLC API › Function blocks, single-phase › Based on the signal period

##### `FB_PMA_PowerValues_Period_1Ph` (FUNCTION) — §5.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_PowerValues_Period_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_PowerValues_Period_1Ph calculates the power values of the connected consumer. These include the fundamental components and the phase shift angle. For this purpose, only the first harmonics of the input signal are used for the calculation in addition to the signal sequence in the time domain. The advantage of these algorithms is the high dynamics of the calculations. The results refer to a configurable number of signal periods. The period duration refers to the frequency specified at the start of the period at the input of the Call [} 55] method. The statistical results refer to the entire runtime or the time at which the statistical results were last reset.

#### PLC API › Function blocks, single-phase › Based on the signal period › FB_PMA_PowerValues_Period_1Ph

##### `Call` (METHOD) → `BOOL` — §5.2.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fFreq` | `LREAL` | Current frequency of the input signal. Is used to determine the length of the period at the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.2.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.2.2.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_1Ph [} 40] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reconfigure` (METHOD) → `BOOL` — §5.2.2.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fMinInputCurrent` | `LREAL` | Minimum input value (RMS) of the current. This |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

##### `Reset` (METHOD) → `BOOL` — §5.2.2.3.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** The current calculations are reset with the method.

#### PLC API › Function blocks, single-phase › Based on the signal period

##### `FB_PMA_Harmonics_Period_1Ph` (FUNCTION) — §5.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_Harmonics_Period_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Harmonics_Period_1Ph calculates the current and voltage harmonics. In addition, the THD of the input values is calculated from the harmonics. In contrast to the function block FB_PMA_Harmonics_1Ph [} 61], the results refer to a configurable number of signal periods. The period value refers to the frequency specified at the start of the period at the input of the Call [} 59] method. The statistical results refer to the entire runtime or the time at which the statistical results were last reset.

#### PLC API › Function blocks, single-phase › Based on the signal period › FB_PMA_Harmonics_Period_1Ph

##### `Call` (METHOD) → `BOOL` — §5.2.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fFreq` | `LREAL` | Current frequency of the input signal. Is used to determine the length of the period at the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.2.2.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.2.2.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_1Ph [} 40] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reset` (METHOD) → `BOOL` — §5.2.2.4.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** The current calculations are reset with the method.

#### PLC API › Function blocks, single-phase › Based on the frequency range

##### `FB_PMA_Harmonics_1Ph` — §5.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_Harmonics_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Harmonics_1Ph calculates the RMS bands of the individual current and voltage harmonics. In addition, the calculated RMS bands are used to calculate the THD of the input variables.

#### PLC API › Function blocks, single-phase › Based on the frequency range › FB_PMA_Harmonics_1Ph

##### `Call` (METHOD) → `BOOL` — §5.2.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pHarmonicsRMS_U` | `POINTER TO LREAL` | Pointer to an array of type LREAL with the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.2.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.2.3.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_1Ph [} 40] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reconfigure` (METHOD) → `BOOL` — §5.2.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fBaseFreq` | `LREAL` | Frequency of the first harmonic. |
| `fBandwidth` | `LREAL` | Total bandwidth of each RMS band. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

##### `Reset` (METHOD) → `BOOL` — §5.2.3.1.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** This method deletes all the data sets already added. In addition, the calculated output values are reset.

#### PLC API › Function blocks, single-phase › Based on the frequency range

##### `FB_PMA_PowerValues_1Ph` (FUNCTION) — §5.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_PowerValues_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_PowerValues_1Ph calculates the power values of the connected consumer. These include the fundamental components and the phase shift angle. Internally, the individual harmonics and their phase angle are determined for the calculations.

#### PLC API › Function blocks, single-phase › Based on the frequency range › FB_PMA_PowerValues_1Ph

##### `Call` (METHOD) → `BOOL` — §5.2.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetEnergyCalc` | `BOOL` | TRUE resets the calculated values of the energy |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.2.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.2.3.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_1Ph [} 40] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reset` (METHOD) → `BOOL` — §5.2.3.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** This method deletes all the data sets already added. In addition, the calculated output values are reset.

##### `Reconfigure` (METHOD) → `BOOL` — §5.2.3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fBaseFreq` | `LREAL` | Frequency of the first harmonic. |
| `fBandwidth` | `LREAL` | Total bandwidth of each RMS band. |
| `fMinInputCurrent` | `LREAL` | Minimum input value (RMS) of the current. This |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

#### PLC API › Function blocks, single-phase › Based on the frequency range

##### `FB_PMA_Spectrum_1Ph` (FUNCTION) — §5.2.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_Spectrum_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Spectrum_1Ph calculates the magnitude spectra of the current and voltage values. These are suitable for analyzing the input signals in the frequency range.

#### PLC API › Function blocks, single-phase › Based on the frequency range › FB_PMA_Spectrum_1Ph

##### `Call` (METHOD) → `BOOL` — §5.2.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pMagnitudeSpectrum_U POINTER TO LREAL` | `Pointer to an array of type LREAL with the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.2.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.2.3.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_1Ph [} 40] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reset` (METHOD) → `BOOL` — §5.2.3.3.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** This method deletes all the data sets already added.

#### PLC API › Function blocks, single-phase › Based on the frequency range

##### `FB_PMA_Spectrum_Quantiles_1Ph` (FUNCTION) — §5.2.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_Spectrum_Quantiles_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Spectrum_Quantiles_1Ph calculates the magnitude spectra of the current and voltage values like the function block FB_PMA_Spectrum_1Ph [} 71]. In addition, p-quantiles of the spectrum distribution can be calculated. The quantiles and their number can be configured individually.

#### PLC API › Function blocks, single-phase › Based on the frequency range › FB_PMA_Spectrum_Quantiles_1Ph

##### `Call` (METHOD) → `BOOL` — §5.2.3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pMagnitudeSpectrum_U POINTER TO LREAL` | `Pointer to an array of type LREAL with the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.2.3.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.2.3.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_1Ph [} 40] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `CallEx` (METHOD) → `BOOL` — §5.2.3.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nAppendData` | `UDINT` | Number of spectra to be calculated until the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CallEx` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Reset` (METHOD) → `BOOL` — §5.2.3.4.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** This method deletes all the data sets already added. Alternatively, automatic resetting can be used on the CallEx [} 79] method.

##### `Reconfigure` (METHOD) → `BOOL` — §5.2.3.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aQuantiles` | `ARRAY[0 -` | Indicates the quantile limit. It must be between |
| `GVL_PMA.cMaxQuantiles-1]` | `0.0 and 1.0. For example, 0.2 corresponds to the OF LREAL` | 20% quantile. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

#### PLC API › Function blocks, three-phase

##### `FB_PMA_Source_3Ph` (FUNCTION) — §5.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `aDestIDs` | `ARRAY[1..GVL_PMA.cMA_MaxDest] OF UDINT` | — |
| `nResultBuffers` | `UDINT := 4` | — |
| `tTransferTimeout` | `LTIME := LTIME#40US` | — |
| `stInitPars` | `ST_PMA_Source_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Source_3Ph writes data from an external PLC data buffer into a multi-array buffer.

#### PLC API › Function blocks, three-phase › FB_PMA_Source_3Ph

##### `Call` (METHOD) → `BOOL` — §5.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBuffer_UL1 - UL3` | `POINTER TO LREAL` | Pointer to an array of voltage values. These can be added individually or as an oversampling |

**Description:** The method is called in each cycle to write the values to the output buffer. The output buffer is sent as soon as it is filled.

##### `ResetData` (METHOD) → `BOOL` — §5.3.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ResetData` | `BOOL` | Indicates whether the method was executed |

**Description:** This method can be used to reset the data currently in the buffer.

##### `Init` (METHOD) → `BOOL` — §5.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `ResetAnalysisChain` (METHOD) → `BOOL` — §5.3.1.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ResetAnalysisChain` | `BOOL` | Indicates whether the method was executed |

**Description:** Calling this method causes an automatic reset of all algorithms in the full analysis chain. Internally, a ResetData() is carried out each time before accepting the new data set.

#### PLC API › Function blocks, three-phase › Based on the signal period

##### `FB_PMA_Frequency_Period_3Ph` (FUNCTION) — §5.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_Frequency_Period_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Frequency_Period_3Ph calculates the base frequency of the given input signal for all three phases. To do this, the signal is first filtered with a Butterworth low-pass filter. The zero crossings of the input signal are then determined from the filtered values, and the frequency is calculated from their difference. The results refer to one or more periods, depending on the configuration. The statistical results refer to the entire runtime or the time at which the statistical results were last reset.

#### PLC API › Function blocks, three-phase › Based on the signal period › FB_PMA_Frequency_Period_3Ph

##### `Call` (METHOD) → `BOOL` — §5.3.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetStatistics` | `BOOL` | TRUE resets the minimum and maximum values |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.3.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.3.2.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_3Ph [} 81] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reset` (METHOD) → `BOOL` — §5.3.2.1.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** The current calculations are reset with the method.

##### `Reconfigure` (METHOD) → `BOOL` — §5.3.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fMinFreq` | `LREAL` | Minimum expected measuring frequency |
| `fMaxFreq` | `LREAL` | Maximum expected measuring frequency |
| `nPeriods` | `UDINT` | Number of periods that influence the calculation. (Period length = sample rate/frequency) |
| `nFilterOrder` | `UINT` | Indicates the order of the low-pass filter. The |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

#### PLC API › Function blocks, three-phase › Based on the signal period

##### `FB_PMA_BasicValues_Period_3Ph` (FUNCTION) — §5.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_BasicValues_Period_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_BasicValues_Period_3Ph calculates analysis values for the signal sequence of current and voltage in a three-phase system. These include the mean value, the RMS value, the peak value, the rectified value, the crest factor and the form factor for the individual currents and voltages. In addition, the voltage values between the individual phases are calculated. The results refer to a configurable number of signal periods. The period duration refers to the frequency specified at the start of the period at the input of the Call [} 91] method. The statistical results refer to the entire runtime or the time at which the statistical results were last reset.

#### PLC API › Function blocks, three-phase › Based on the signal period › FB_PMA_BasicValues_Period_3Ph

##### `Call` (METHOD) → `BOOL` — §5.3.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fFreq` | `LREAL` | Current frequency of the input signal. Is used to determine the length of the period at the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.3.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.3.2.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_3Ph [} 81] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reset` (METHOD) → `BOOL` — §5.3.2.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** The current calculations are reset with the method.

##### `Reconfigure` (METHOD) → `BOOL` — §5.3.2.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fMinInputCurrent` | `LREAL` | Minimum input value (RMS) of the current. This |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

#### PLC API › Function blocks, three-phase › Based on the signal period

##### `FB_PMA_PowerValues_Period_3Ph` (FUNCTION) — §5.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_PowerValues_Period_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_PowerValues_Period_3Ph calculates the power values of the connected consumer in a three-phase grid. These include the fundamental components and the phase shift angle. For this purpose, only the first harmonics of the input signal are used for the calculation in addition to the signal sequence in the time domain. The advantage of these algorithms is the high dynamics of the calculations. The results refer to a configurable number of signal periods. The period duration refers to the frequency specified at the start of the period at the input of the Call [} 86] method. The statistical results refer to the entire runtime or the time at which the statistical results were last reset.

#### PLC API › Function blocks, three-phase › Based on the signal period › FB_PMA_PowerValues_Period_3Ph

##### `Call` (METHOD) → `BOOL` — §5.3.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fFreq` | `LREAL` | Current frequency of the input signal. Is used to determine the length of the period at the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.3.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.3.2.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_3Ph [} 81] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reset` (METHOD) → `BOOL` — §5.3.2.3.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** The current calculations are reset with the method.

##### `Reconfigure` (METHOD) → `BOOL` — §5.3.2.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fMinInputCurrent` | `LREAL` | Minimum input value (RMS) of the current. This |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

#### PLC API › Function blocks, three-phase › Based on the signal period

##### `FB_PMA_Harmonics_Period_3Ph` (FUNCTION) — §5.3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_Harmonics_Period_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Harmonics_Period_3Ph calculates the current and voltage harmonics. In addition, the THD of the input values is calculated from the harmonics. In contrast to the function block FB_PMA_Harmonics_3Ph [} 104], the results refer to a configurable number of signal periods. The period value refers to the frequency specified at the start of the period at the input of the Call [} 86] method. The statistical results refer to the entire runtime or the time at which the statistical results were last reset.

#### PLC API › Function blocks, three-phase › Based on the signal period › FB_PMA_Harmonics_Period_3Ph

##### `Call` (METHOD) → `BOOL` — §5.3.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fFreq` | `LREAL` | Current frequency of the input signal. Is used to determine the length of the period at the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.3.2.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.3.2.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_3Ph [} 81] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reset` (METHOD) → `BOOL` — §5.3.2.4.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** The current calculations are reset with the method.

#### PLC API › Function blocks, three-phase › Based on the frequency range

##### `FB_PMA_Harmonics_3Ph` — §5.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_Harmonics_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Harmonics_3Ph calculates the RMS bands of the individual current and voltage harmonics in a three-phase system. In addition, the calculated RMS bands are used to calculate the THD of the input variables.

#### PLC API › Function blocks, three-phase › Based on the frequency range › FB_PMA_Harmonics_3Ph

##### `Call` (METHOD) → `BOOL` — §5.3.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pHarmonicsRMS_UL1 .. POINTER TO LREAL` | `Pointer to an array of type LREAL with the` | — |
| `UL3` | `dimension: Number of harmonics. If the individual` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.3.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.3.3.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_3Ph [} 81] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reconfigure` (METHOD) → `BOOL` — §5.3.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fBaseFreq` | `LREAL` | Frequency of the first harmonic. |
| `fBandwidth` | `LREAL` | Total bandwidth of each RMS band. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

##### `Reset` (METHOD) → `BOOL` — §5.3.3.1.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** This method deletes all the data sets already added. In addition, the calculated output values are reset.

#### PLC API › Function blocks, three-phase › Based on the frequency range

##### `FB_PMA_PowerValues_3Ph` (FUNCTION) — §5.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_PowerValues_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_PowerValues_3Ph calculates the power values of the connected consumer in a three-phase grid. These include the fundamental components and the phase shift angle. Internally, the individual harmonics and their phase angle are determined for the calculations.

#### PLC API › Function blocks, three-phase › Based on the frequency range › FB_PMA_PowerValues_3Ph

##### `Call` (METHOD) → `BOOL` — §5.3.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetEnergyCalc` | `BOOL` | TRUE resets the calculated values of the energy |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.3.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.3.3.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_3Ph [} 81] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reconfigure` (METHOD) → `BOOL` — §5.3.3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fBaseFreq` | `LREAL` | Frequency of the first harmonic. |
| `fBandwidth` | `LREAL` | Total bandwidth of each RMS band. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.

##### `Reset` (METHOD) → `BOOL` — §5.3.3.2.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** This method deletes all the data sets already added. In addition, the calculated output values are reset.

#### PLC API › Function blocks, three-phase › Based on the frequency range

##### `FB_PMA_Spectrum_3Ph` (FUNCTION) — §5.3.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_Spectrum_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Spectrum_3Ph calculates the magnitude spectra of the current and voltage values. These are suitable for analyzing the input signals in the frequency range.

#### PLC API › Function blocks, three-phase › Based on the frequency range › FB_PMA_Spectrum_3Ph

##### `Call` (METHOD) → `BOOL` — §5.3.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pMagnitudeSpectrum_U POINTER TO LREAL` | `Pointer to an array of type LREAL with the` | — |
| `L1 .. UL3` | `dimension: FFT length/2+1. If the spectrum is not` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.3.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.3.3.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_3Ph [} 81] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `Reset` (METHOD) → `BOOL` — §5.3.3.3.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** This method deletes all the data sets already added.

#### PLC API › Function blocks, three-phase › Based on the frequency range

##### `FB_PMA_Spectrum_Quantiles_3Ph` (FUNCTION) — §5.3.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US` | — |
| `stInitPars` | `ST_PMA_Spectrum_Quantiles_InitPars` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error occurs. |
| `ipResultMessage` | `I_TcMessage` | The interface offers detailed information about the |

**Description:** The function block FB_PMA_Spectrum_Quantiles_3Ph calculates the magnitude spectra of the current and voltage values like the function block FB_PMA_Spectrum_3Ph [} 114]. In addition, p-quantiles of the spectrum distribution can be calculated. The quantiles and their number can be configured individually.

#### PLC API › Function blocks, three-phase › Based on the frequency range › FB_PMA_Spectrum_Quantiles_3Ph

##### `Call` (METHOD) → `BOOL` — §5.3.3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pMagnitudeSpectrum_U POINTER TO LREAL` | `Pointer to an array of type LREAL with the` | — |
| `L1 .. UL3` | `dimension: FFT length/2+1. If the spectrum is not` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Init` (METHOD) → `BOOL` — §5.3.3.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT` | Identifies the function block instance with a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | Indicates whether the method was executed |

**Description:** The Init method is usually not required in a Power Monitoring application. It offers an alternative to function block initialization, which enables encapsulation of the function block. The method FB_init or the attribute 'call_after_init' must be used for this (see TwinCAT 3 PLC > Programming Reference). The Init method may only be called during the initialization phase of the PLC. It cannot be used at runtime.

##### `PassInputs` (METHOD) → `BOOL` — §5.3.3.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PassInputs` | `BOOL` | Indicates whether the method was executed |

**Description:** As long as an instance of the function block FB_PMA_Source_3Ph [} 81] is called and thus signal data is transferred to a target block, all other function blocks of the analysis chain must be called cyclically (see Parallel Processing in Transfer Tray).

##### `CallEx` (METHOD) → `BOOL` — §5.3.3.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nAppendData` | `UDINT` | Number of spectra to be calculated until the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CallEx` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in each cycle to execute the calculations from the input buffer when new data is present.

##### `Reset` (METHOD) → `BOOL` — §5.3.3.4.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Indicates whether the method was executed |

**Description:** This method deletes all the data sets already added. Alternatively, automatic resetting can be used on the CallEx [} 122] method.

##### `Reconfigure` (METHOD) → `BOOL` — §5.3.3.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aQuantiles` | `ARRAY[0 -` | Indicates the quantile limit. It must be between |
| `GVL_PMA.cMaxQuantiles-1]` | `0.0 and 1.0. For example, 0.2 corresponds to the` | — |
| `OF LREAL` | `20% quantile.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reconfigure` | `BOOL` | Indicates whether the method was executed |

**Description:** The method is called in order to reconfigure the function block during the runtime.
