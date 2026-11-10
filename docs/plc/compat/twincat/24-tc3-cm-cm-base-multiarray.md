# Tc3_CM+CM_Base+MultiArray

- **Manual:** `TF3600_TC3_ConditionMonitoring_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Measurement · **Tier:** T0 · **API entries:** 53
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Condition-monitoring math (FFT, envelope, order analysis): pure-ST or Rust numeric kernels.

---

#### PLC API › Function blocks

##### `FB_CMA_AnalyticSignal` (METHOD) → `HRESULT` — §5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_AnalyticSignal_InitPars;   // init parameter` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                               // number of MultiArrays which should be ini` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                     // timeout checking off during access to int` | — |
| `stInitPars` | `ST_CM_AnalyticSignal_InitPars;            // init parameter` | — |
| `nOwnID` | `UDINT;                                    // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;          // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be ini` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                    // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                 // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculation of the analytical signal of a time series.

##### `FB_CMA_ArgSort` (METHOD) → `HRESULT` — §5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_ArgSort_InitPars;                   // init parameter` | — |
| `nOwnID` | `UDINT;                                    // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;          // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                               // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                     // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_ArgSort_InitPars;                   // init parameter` | — |
| `nOwnID` | `UDINT;                                    // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;          // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                 // number of MultiArrays which should be ini` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                   // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;          // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                  // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Sorts the incoming arguments

##### `FB_CMA_BufferConverting` (METHOD) → `HRESULT` — §5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_MA_MultiArray_InitPars;       // init parameter` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be ini` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                  // timeout checking off during access to int` | — |
| `nWorkDimIn` | `UDINT;                // It designates the dimension in the input MultiArray be` | — |
| `nWorkDimOut` | `UDINT;                // It designates the dimension in the output MultiArray b` | — |
| `nElements` | `UDINT;                // optional: default:0-` | — |
| `pStartIndexIn` | `POINTER TO UDINT;     (* optional: default:0-` | — |
| `pStartIndexOut` | `POINTER TO UDINT;    (* optional: default:0-` | — |
| `nOptionPars` | `DWORD;               // option mask` | — |
| `nOptionPars` | `DWORD;   // option mask` | — |
| `stInitPars` | `ST_MA_MultiArray_InitPars;       // init parameter` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be initi` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;                       // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                       // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;                     // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;     // TRUE every time when outgoing MultiArray was calculated and sent to` | — |
| `bError` | `BOOL;    // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT; // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Copies data from one MultiArray to another MultiArray.

##### `FB_CMA_CrestFactor` (METHOD) → `HRESULT` — §5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_CrestFactor_InitPars;      // init parameter` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be ini` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to int` | — |
| `stInitPars` | `ST_CM_CrestFactor_InitPars;      // init parameter` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be initi` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;           // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;         // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;      // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the crest factor for each channel for single and multi-channel time series.

##### `FB_CMA_CrestFactorPlus` (METHOD) → `HRESULT` — §5.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_CrestFactorPlus_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be ini` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;            // timeout checking off during access to int` | — |
| `pArg` | `POINTER TO LREAL;      // pointer to 2-dimensional array (LREAL) of arguments` | — |
| `nArgSize` | `UDINT;                 // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_CrestFactorPlus_InitPars; // init parameter` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be initi` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                    // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                 // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the crest factor plus for each channel for single and multi-channel time series.

##### `FB_CMA_ComplexDataHandling` (METHOD) → `HRESULT` — §5.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_ComplexDataHandling_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                        // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;              // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                   // number of MultiArrays which should` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                         // timeout checking off during access` | — |
| `pArg` | `PVOID;    // pointer to array of arguments` | — |
| `nArgSize` | `UDINT;    // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_ComplexDataHandling_InitPars;          // init parameter` | — |
| `nOwnID` | `UDINT;                                       // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;             // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be i` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                   // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;          // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                  // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Conversion of complex input data into real output data

##### `FB_CMA_ComplexFFT` (METHOD) → `HRESULT` — §5.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_ComplexFFT_InitPars;       // init parameter` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be ini` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;            // timeout checking off during access to int` | — |
| `stInitPars` | `ST_CM_ComplexFFT_InitPars;       // init parameter` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be initi` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                   // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;          // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                  // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculation of the Fast Fourier Transform (FFT) for complex-valued input signals.

##### `FB_CMA_Correlation` (METHOD) → `HRESULT` — §5.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_Correlation_InitPars;                 // init parameter` | — |
| `nOwnID` | `UDINT;                                      // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;            // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                 // number of MultiArrays which should be` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                       // timeout checking off during access to` | — |
| `pArg` | `PVOID;    // pointer to array of arguments` | — |
| `nArgSize` | `UDINT;    // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_Correlation_InitPars;                  // init parameter` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;             // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be in` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculation of the correlation function for single and multi-channel real-valued time series.

##### `FB_CMA_DiscreteClassification` (METHOD) → `HRESULT` — §5.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_DiscreteClassification_InitPars;          // init parameter` | — |
| `nOwnID` | `UDINT;                                          // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                     // number of MultiArrays which should` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                           // timeout checking off during access` | — |
| `pArg` | `POINTER TO LREAL; // pointer to array (LREAL) of arguments` | — |
| `nArgSize` | `UDINT;            // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_DiscreteClassification_InitPars;             // init parameter` | — |
| `nOwnID` | `UDINT;                                             // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                  // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                       // number of MultiArrays which should b` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Classification of multi-channel data based on configurable threshold values

##### `FB_CMA_Downsampling` (METHOD) → `HRESULT` — §5.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDivider` | `UDINT := 1;        // given input signal is sampled down by the specified divider. (1 =` | — |
| `nChannels` | `UDINT;              // number of channels in data buffer (=1:onedimensional dataset, >1:` | — |
| `pDataIn` | `POINTER TO BYTE;      // address of data buffer (e.g. oversampling data)` | — |
| `nDataInSize` | `UDINT;                // size of data buffer in bytes` | — |
| `pDataOut` | `POINTER TO BYTE;      // address of result buffer` | — |
| `nDataOutSize` | `UDINT;                 // size of data buffer in bytes` | — |
| `nElementSize` | `UDINT;                 // Size of element type in bytes` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;                 // TRUE every time when outgoing data buffer was calculated.` | — |
| `bError` | `BOOL;                // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;             // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Downsampling of signal data through copying of the signal data from one PLC buffer to another PLC buffer (array).

##### `FB_CMA_EmpiricalExcess` (METHOD) → `HRESULT` — §5.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_EmpiricalMoments_InitPars;        // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                 // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `nAppendData` | `UDINT;            // count of data buffers which are appended until calculation (1= cal` | — |
| `bResetData` | `BOOL;            // automatic reset of dataset buffer after each calculation` | — |
| `stInitPars` | `ST_CM_EmpiricalMoments_InitPars;         // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be ini` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;              // TRUE every time when outgoing MultiArray was calculated and sent` | — |
| `bError` | `BOOL;              // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;            // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the excess for single- and multi-channel real-valued time series.

##### `FB_CMA_EmpiricalMean` (METHOD) → `HRESULT` — §5.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_EmpiricalMoments_InitPars;        // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                 // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_EmpiricalMoments_InitPars;        // init parameter` | — |
| `nOwnID` | `UDINT;                                    // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;          // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be ini` | — |
| `nAppendData` | `UDINT;            // count of data buffers which are appended until calculation (1= cal` | — |
| `bResetData` | `BOOL;            // automatic reset of dataset buffer after each calculation` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;              // TRUE every time when outgoing MultiArray was calculated and sent` | — |
| `bError` | `BOOL;              // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;            // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the mean value for single- and multi-channel real-valued time series.

##### `FB_CMA_EmpiricalSkew` (METHOD) → `HRESULT` — §5.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_EmpiricalMoments_InitPars;        // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                 // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_EmpiricalMoments_InitPars;        // init parameter` | — |
| `nOwnID` | `UDINT;                                    // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;          // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be ini` | — |
| `nAppendData` | `UDINT;            // count of data buffers which are appended until calculation (1= cal` | — |
| `bResetData` | `BOOL;            // automatic reset of dataset buffer after each calculation` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;              // TRUE every time when outgoing MultiArray was calculated and sent` | — |
| `bError` | `BOOL;              // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;            // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the empirical skew for single- and multi-channel real-valued time series.

##### `FB_CMA_EmpiricalStandardDeviation` (METHOD) → `HRESULT` — §5.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_EmpiricalMoments_InitPars;         // init parameter` | — |
| `nOwnID` | `UDINT;                                   // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                  // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                              // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                    // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_EmpiricalMoments_InitPars;         // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be ini` | — |
| `nAppendData` | `UDINT;            // count of data buffers which are appended until calculation (1= cal` | — |
| `bResetData` | `BOOL;            // automatic reset of dataset buffer after each calculation` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;              // TRUE every time when outgoing MultiArray was calculated and sent` | — |
| `bError` | `BOOL;              // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;            // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the empirical standard deviation for single- and multi-channel real-valued time series.

##### `FB_CMA_Envelope` (METHOD) → `HRESULT` — §5.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_Envelope_InitPars;                // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;        // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_Envelope_InitPars;                   // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be init` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                    // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                 // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the envelope of a time signal.

##### `FB_CMA_EnvelopeSpectrum` (METHOD) → `HRESULT` — §5.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_EnvelopeSpectrum_InitPars;          // init parameter` | — |
| `nOwnID` | `UDINT;                                    // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;          // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                               // number of MultiArrays which should be` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                     // timeout checking off during access to` | — |
| `stInitPars` | `ST_CM_EnvelopeSpectrum_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                          // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be init` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                             // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;       // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the envelope spectrum of a time signal.

##### `FB_CMA_HistArray` (METHOD) → `HRESULT` — §5.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_HistArray_InitPars;               // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;        // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `nAppendData` | `UDINT;            // count of data buffers which are appended until calculation (1= cal` | — |
| `bResetData` | `BOOL;            // automatic reset of dataset buffer after each calculation` | — |
| `pArg` | `POINTER TO LREAL;        // pointer to 2-dimensional array (LREAL) of arguments` | — |
| `nArgSize` | `UDINT;                   // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_HistArray_InitPars;         // init parameter` | — |
| `nOwnID` | `UDINT;       // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4; // number of MultiArrays which should be initialized for results (` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;              // TRUE every time when outgoing MultiArray was calculated and sent` | — |
| `bError` | `BOOL;              // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;            // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the frequency distribution for single and multi-channel real-valued time series.

##### `FB_CMA_InstantaneousFrequency` (METHOD) → `HRESULT` — §5.1.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_InstantaneousFrequency_InitPars;         // init parameter` | — |
| `nOwnID` | `UDINT;                                         // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;               // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                    // number of MultiArrays which should` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                          // timeout checking off during access` | — |
| `stInitPars` | `ST_CM_InstantaneousFrequency_InitPars;        // init parameter` | — |
| `nOwnID` | `UDINT;                                 // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be initi` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                             // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;       // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculation of the instantaneous frequency of a time signal

##### `FB_CMA_InstantaneousPhase` (METHOD) → `HRESULT` — §5.1.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_InstantaneousPhase_InitPars;         // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                      // timeout checking off during access to` | — |
| `stInitPars` | `ST_CM_InstantaneousPhase_InitPars;       // init parameter` | — |
| `nOwnID` | `UDINT;                                // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                               // number of MultiArrays which should be initi` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculation of the instantaneous phase of a time signal

##### `FB_CMA_IntegratedRMS` (METHOD) → `HRESULT` — §5.1.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_IntegratedRMS_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;        // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `pArg` | `POINTER TO LREAL; // pointer to array (LREAL) of arguments` | — |
| `nArgSize` | `UDINT;            // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_IntegratedRMS_InitPars;            // init parameter` | — |
| `nOwnID` | `UDINT;                                   // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;         // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                               // number of MultiArrays which should be ini` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                    // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                 // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates (optionally integrated) RMS values for single- and multi-channel real-valued time series.

##### `FB_CMA_MagnitudeSpectrum` (METHOD) → `HRESULT` — §5.1.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_MagnitudeSpectrum_InitPars;          // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be i` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                      // timeout checking off during access to i` | — |
| `stInitPars` | `ST_CM_MagnitudeSpectrum_InitPars; // init parameter` | — |
| `nOwnID` | `UDINT;                            // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                       // number of MultiArrays which should be init` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the magnitude spectrum (also referred to as amplitude spectrum) of a real-valued input signal.

##### `FB_CMA_MeanStressCorrection` (METHOD) → `HRESULT` — §5.1.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_MeanStressCorrection_InitPars;         // init parameter` | — |
| `nOwnID` | `UDINT;                                       // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;             // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should b` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                        // timeout checking off during access t` | — |
| `pArg` | `POINTER TO UDINT;        // pointer to 1-dimensional array (UDINT) of arguments` | — |
| `nArgSize` | `UDINT;                    // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_MeanStressCorrection_InitPars;         // init parameter` | — |
| `nOwnID` | `UDINT;                                       // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;             // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                    // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                 // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates mean stress-free reversal points from classified reversal points.

##### `FB_CMA_MinersRule` (METHOD) → `HRESULT` — §5.1.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_MinersRule_InitPars;               // init parameter` | — |
| `nOwnID` | `UDINT;                                   // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;         // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                              // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                    // timeout checking off during access to in` | — |
| `pArg` | `POINTER TO LREAL;        // pointer to 1-dimensional array (LREAL) of arguments` | — |
| `nArgSize` | `UDINT;                    // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_RainflowCounting_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be init` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the fatigue damage of a part.

##### `FB_CMA_MomentCoefficients` (METHOD) → `HRESULT` — §5.1.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_MomentCoefficients_InitPars;         // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                      // timeout checking off during access to` | — |
| `stInitPars` | `ST_CM_MomentCoefficients_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                       // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;             // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be ini` | — |
| `nAppendData` | `UDINT;            // count of data buffers which are appended until calculation (1= cal` | — |
| `bResetData` | `BOOL;            // automatic reset of dataset buffer after each calculation` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                               // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                            // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;              // TRUE every time when outgoing MultiArray was calculated and sent` | — |
| `bError` | `BOOL;           // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;        // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the average value, the empirical standard deviation, the skew and the excess for single- and multi-channel real-valued time series.

##### `FB_CMA_MovingExcess` (METHOD) → `HRESULT` — §5.1.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_MovingMoments_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                 // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_MovingMoments_InitPars;                // init parameter` | — |
| `nOwnID` | `UDINT;                                       // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;             // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be ini` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the moving excess for single- and multi-channel real-valued time series.

##### `FB_CMA_MovingMean` (METHOD) → `HRESULT` — §5.1.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_MovingMoments_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                 // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_MovingMoments_InitPars;                // init parameter` | — |
| `nOwnID` | `UDINT;                                       // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;             // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be ini` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the moving mean value for single- and multi-channel real-valued time series.

##### `FB_CMA_MovingSkew` (METHOD) → `HRESULT` — §5.1.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_MovingMoments_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                 // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_MovingMoments_InitPars;                // init parameter` | — |
| `nOwnID` | `UDINT;                                       // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;             // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be ini` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                   // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;          // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                  // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the moving skew for single- and multi-channel real-valued time series.

##### `FB_CMA_MovingStandardDeviation` (METHOD) → `HRESULT` — §5.1.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_MovingMoments_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                 // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_MovingMoments_InitPars;                // init parameter` | — |
| `nOwnID` | `UDINT;                                       // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;             // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be ini` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                    // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                 // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the moving standard deviation for single- and multi-channel real-valued time series.

##### `FB_CMA_MultiBandRMS` (METHOD) → `HRESULT` — §5.1.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_MultiBandRMS_InitPars;            // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;        // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_MultiBandRMS_InitPars;      // init parameter` | — |
| `nOwnID` | `UDINT;        // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;   // number of MultiArrays which should be initialized for results` | — |
| `pArg` | `POINTER TO LREAL; // pointer to array (LREAL) of arguments` | — |
| `nArgSize` | `UDINT;            // size of arguments buffer in bytes` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                             // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;       // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                               // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculated RMS value for single- and multi-channel real-valued time series for configurable frequency bands

##### `FB_CMA_OrderPowerSpectrum` (METHOD) → `HRESULT` — §5.1.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_OrderPowerSpectrum_InitPars;          // init parameter` | — |
| `nOwnID` | `UDINT;                                      // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;            // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                 // number of MultiArrays which should be` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                       // timeout checking off during access to` | — |
| `stInitPars` | `ST_CM_OrderPowerSpectrum_InitPars;         // init parameter` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be in` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculation of the order spectrum of real-valued input signals.

##### `FB_CMA_PowerCepstrum` (METHOD) → `HRESULT` — §5.1.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_PowerCepstrum_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;        // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_PowerCepstrum_InitPars;     // init parameter` | — |
| `nOwnID` | `UDINT;         // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;    // number of MultiArrays which should be initialized for results` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;           // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;           // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** The function block calculates the power cepstrum for a real-valued input signal.

##### `FB_CMA_PowerSpectrum` (METHOD) → `HRESULT` — §5.1.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_PowerSpectrum_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;        // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_PowerSpectrum_InitPars;    // init parameter` | — |
| `nOwnID` | `UDINT;                                // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be initi` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                    // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                 // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculation of the power spectrum of a real-valued input signal, and optional decibel scaling.

##### `FB_CMA_Quantiles` (METHOD) → `HRESULT` — §5.1.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_Quantiles_InitPars;             // init parameter` | — |
| `nOwnID` | `UDINT;                                // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;      // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                           // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                    // timeout checking off during access to in` | — |
| `nAppendData` | `UDINT;          // count of data buffers which are appended until calculation (1= cal` | — |
| `bResetData` | `BOOL;          // automatic reset of dataset buffer after each calculation` | — |
| `pArg` | `POINTER TO LREAL;      // pointer to 2-dimensional array (LREAL) of arguments` | — |
| `nArgSize` | `UDINT;                 // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_Quantiles_InitPars; // init parameter` | — |
| `nOwnID` | `UDINT;                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be initi` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;           // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;           // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the quantile of the input value distribution for single- and multi-channel real-valued signals.

##### `FB_CMA_RainflowCounting` (METHOD) → `HRESULT` — §5.1.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_RainflowCounting_InitPars;        // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;        // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_RainflowCounting_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be init` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                               // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                            // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates classified reversal points in a stress-time sequence.

##### `FB_CMA_ReadCsvFile` — §5.1.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL := TRUE;           // starts execution (on rising edge)` | — |
| `bAbort` | `BOOL;                   // aborts execution (after finishing current ADS communica` | — |
| `sNetId` | `T_AmsNetId := '';       // TwinCAT system network address (could be kept empty for` | — |
| `tAdsTimeout` | `TIME := T#1S;           // ADS timeout (only 1 second as condition for a fast data` | — |
| `sFileName` | `T_MaxString := '';      // CSV source file path and name (located on the system wi` | — |
| `sSeparator` | `STRING(2) := '$T';      // CSV field separator [e.g. tab '$T' or comma ',' or semi` | — |
| `nHeaderLines` | `UDINT := 0;             // Number of lines in the CSV file which belongs to the he` | — |
| `bColumn` | `BOOL := TRUE;           // select whether the data is placed as column (e.g. TcSco` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aBuffer` | `ARRAY[*] OF LREAL;      // buffer with individual length (do not switch the buffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBufferFull` | `BOOL;    // signals that aBuffer is fully filled` | — |
| `nReadRecords` | `UDINT;   // shows the number of records saved to aBuffer` | — |
| `bBusy` | `BOOL;    // TRUE if execution is active` | — |
| `bError` | `BOOL;    // TRUE if an error occurred` | — |
| `hrErrorCode` | `HRESULT; // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;    // shows detailed information about occurred` | — |

**Description:** Reading a CSV file and sequential output of the data to a PLC buffer (array).

##### `FB_CMA_RealFFT` (METHOD) → `HRESULT` — §5.1.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_RealFFT_InitPars;                  // init parameter` | — |
| `nOwnID` | `UDINT;                                   // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;         // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                              // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                    // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_RealFFT_InitPars;                  // init parameter` | — |
| `nOwnID` | `UDINT;                                   // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;         // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                              // number of MultiArrays which should be init` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;           // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;           // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculation of the Fast Fourier Transform (FFT) for real-valued input signals.

##### `FB_CMA_RMS` (METHOD) → `HRESULT` — §5.1.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_RMS_InitPars;                     // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;        // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                   // timeout checking off during access to in` | — |
| `stInitPars` | `ST_CM_RMS_InitPars; // init parameter` | — |
| `nOwnID` | `UDINT;               // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4; // number of MultiArrays which should be initialized for results (` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                    // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                 // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculates the temporal RMS value for single and multi-channel real-valued signals.

##### `FB_CMA_SlidingDFT` (METHOD) → `HRESULT` — §5.1.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_SlidingDFT_InitPars;              // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;        // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                     // timeout checking off during access to i` | — |
| `pArg` | `PVOID;    // pointer to array of arguments` | — |
| `nArgSize` | `UDINT;    // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_SlidingDFT_InitPars;               // init parameter` | — |
| `nOwnID` | `UDINT;                                   // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;         // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                              // number of MultiArrays which should be init` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                             // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;       // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Online calculation of spectral values for real-valued input signals

##### `FB_CMA_SparseSpectrum` (METHOD) → `HRESULT` — §5.1.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_SparseSpectrum_InitPars;             // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be i` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                      // timeout checking off during access to i` | — |
| `pArg` | `PVOID;    // pointer to array of arguments` | — |
| `nArgSize` | `UDINT;    // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_SparseSpectrum_InitPars;             // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be init` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculation of individual spectral values for real-valued input signals

##### `FB_CMA_SpikeEnergySpectrum` (METHOD) → `HRESULT` — §5.1.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_SpikeEnergySpectrum_InitPars;        // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                      // timeout checking off during access to` | — |
| `pArg` | `PVOID;    // pointer to array of arguments` | — |
| `nArgSize` | `UDINT;    // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_SpikeEnergySpectrum_InitPars;          // init parameter` | — |
| `nOwnID` | `UDINT;                                       // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;             // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                  // number of MultiArrays which should be i` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                    // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                                 // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;           // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                   // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;            // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;          // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Analysis of peak energy of high-frequency signal components

##### `FB_CMA_Sink` (METHOD) → `HRESULT` — §5.1.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOwnID` | `UDINT;                     // ID for this FB instance` | — |
| `tTransferTimeout` | `LTIME := LTIME#40US;       // timeout checking off during access to inter-` | — |
| `pDataOut` | `POINTER TO BYTE;      // address of data buffer` | — |
| `nDataOutSize` | `UDINT;                 // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nWorkDim` | `UDINT:=0;             // It designates the dimension in the MultiArray being pro` | — |
| `nElements` | `UDINT:=0;             // optional: default:0->full` | — |
| `pStartIndex` | `POINTER TO UDINT;    (* optional: default:0->internally handled as [0,0,..]` | — |
| `nOptionPars` | `DWORD;                // option mask` | — |
| `pDataOut` | `POINTER TO BYTE;     // address of data buffer` | — |
| `nDataOutSize` | `UDINT;                // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nOptionPars` | `DWORD;               // option mask` | — |
| `pDataOut` | `POINTER TO BYTE; // address of data buffer` | — |
| `nDataOutSize` | `UDINT;              // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nWorkDim0` | `UDINT:=0;         // It designates the first dimension in the MultiArray being p` | — |
| `nWorkDim1` | `UDINT:=1;         // It designates the second dimension in the MultiArray being` | — |
| `nElementsDim0` | `UDINT:=0;          // optional: default:0-` | — |
| `nElementsDim1` | `UDINT:=0;          // optional: default:0-` | — |
| `pStartIndex` | `POINTER TO UDINT; (* optional: default:0->internally handled as [0,0,..]` | — |
| `nOptionPars` | `DWORD;           // option mask` | — |
| `pDataOut` | `POINTER TO BYTE; // address of data buffer` | — |
| `nDataOutSize` | `UDINT;             // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nOptionPars` | `DWORD;           // option mask` | — |
| `pDataOut` | `POINTER TO BYTE; // address of data buffer` | — |
| `nDataOutSize` | `UDINT;              // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nWorkDim0` | `UDINT:=0;         // It designates the first dimension in the MultiArray being p` | — |
| `nWorkDim1` | `UDINT:=1;         // It designates the second dimension in the MultiArray being` | — |
| `nWorkDim2` | `UDINT:=1;         // It designates the third dimension in the MultiArray being p` | — |
| `nElementsDim0` | `UDINT:=0;          // optional: default:0-` | — |
| `nElementsDim1` | `UDINT:=0;          // optional: default:0-` | — |
| `nElementsDim2` | `UDINT:=0;          // optional: default:0-` | — |
| `pStartIndex` | `POINTER TO UDINT; (* optional: default:0->internally handled as [0,0,..]` | — |
| `nOptionPars` | `DWORD;           // option mask` | — |
| `pDataOut` | `POINTER TO BYTE; // address of data buffer` | — |
| `nDataOutSize` | `UDINT;             // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nOptionPars` | `DWORD;           // option mask` | — |
| `nOwnID` | `UDINT; // ID for this FB instance` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;                 // TRUE every time when data was written from MultiArray t` | — |
| `bError` | `BOOL;                 // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;              // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;                // TRUE every time when data was written from MultiArray t` | — |
| `bError` | `BOOL;                // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;             // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;            // TRUE every time when data was written from MultiArray to dat` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;            // TRUE every time when data was written from MultiArray to dat` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;            // TRUE every time when data was written from MultiArray to dat` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;            // TRUE every time when data was written from MultiArray to dat` | — |
| `bError` | `BOOL;            // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** This function block writes data from a MultiArray buffer into an external PLC data buffer.

##### `FB_CMA_Source` (METHOD) → `HRESULT` — §5.1.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_MA_MultiArray_InitPars;       // init parameter` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be ini` | — |
| `tTransferTimeout` | `LTIME := LTIME#40US;                   // timeout checking off during access to int` | — |
| `pDataIn` | `POINTER TO BYTE;      // address of data buffer (e.g. oversampling data) as one-` | — |
| `nDataInSize` | `UDINT;                // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nWorkDim` | `UDINT;                // It designates the dimension in the multi array being pr` | — |
| `pStartIndex` | `POINTER TO UDINT;     (* optional: default:0-` | — |
| `nOptionPars` | `DWORD;                // option mask` | — |
| `pDataIn` | `POINTER TO BYTE;     // address of data buffer (e.g. oversampling data) as one-` | — |
| `nDataInSize` | `UDINT;               // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nOptionPars` | `DWORD;               // option mask` | — |
| `pDataIn` | `POINTER TO BYTE;      // address of data buffer (e.g. oversampling data) as twod` | — |
| `nDataInSize` | `UDINT;                // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nWorkDim0` | `UDINT:=0;             // It designates the first dimension in the MultiArray bei` | — |
| `nWorkDim1` | `UDINT:=1;             // It designates the second dimension in the MultiArray be` | — |
| `pStartIndex` | `POINTER TO UDINT;     (* optional: default:0->` | — |
| `nOptionPars` | `DWORD;                // option mask` | — |
| `pDataIn` | `POINTER TO BYTE;      // address of data buffer (e.g. oversampling data) as twod` | — |
| `nDataInSize` | `UDINT;                // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nOptionPars` | `DWORD;                // option mask` | — |
| `pDataIn` | `POINTER TO BYTE;      // address of data buffer (e.g. oversampling data) as twod` | — |
| `nDataInSize` | `UDINT;                // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nWorkDim0` | `UDINT:=0;             // It designates the first dimension in the MultiArray bei` | — |
| `nWorkDim1` | `UDINT:=1;             // It designates the second dimension in the MultiArray be` | — |
| `nWorkDim2` | `UDINT:=2;             // It designates the third dimension in the MultiArray bei` | — |
| `pStartIndex` | `POINTER TO UDINT;     (* optional: default:0->` | — |
| `nOptionPars` | `DWORD;                // option mask` | — |
| `pDataIn` | `POINTER TO BYTE;      // address of data buffer (e.g. oversampling data) as twod` | — |
| `nDataInSize` | `UDINT;                // size of data buffer in bytes` | — |
| `eElementType` | `E_MA_ElementTypeCode` | — |
| `nOptionPars` | `DWORD;                // option mask` | — |
| `stInitPars` | `ST_MA_MultiArray_InitPars;       // init parameter` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be initi` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;                 // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                 // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;              // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;                // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;             // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;                 // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                 // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;              // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;                 // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                 // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;              // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;                 // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                 // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;              // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;                      // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                           // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;                   // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** This function block writes data from an external PLC data buffer into a MultiArray buffer.

##### `FB_CMA_SourcePaired` (METHOD) → `HRESULT` — §5.1.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitParsA` | `ST_MA_MultiArray_InitPars;       // init parameters for first MultiArray` | — |
| `stInitParsB` | `ST_MA_MultiArray_InitPars;       // init parameters for second MultiArray` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                      // number of MultiArrays which should be ini` | — |
| `tTransferTimeout` | `LTIME := LTIME#40US;             // timeout checking off during access to int` | — |
| `pDataInA` | `POINTER TO BYTE;       // address of data buffer (e.g. oversampling data) as one-` | — |
| `nDataInSizeA` | `UDINT;                  // size of data buffer in bytes` | — |
| `pDataInB` | `POINTER TO BYTE;       // address of data buffer (e.g. oversampling data) as one-` | — |
| `nDataInSizeB` | `UDINT;                  // size of data buffer in bytes` | — |
| `eElementTypeA` | `E_MA_ElementTypeCode; // valid types: LREAL, INT32, UINT64, LCOMPLEX` | — |
| `eElementTypeB` | `E_MA_ElementTypeCode; // valid types: LREAL, INT32, UINT64, LCOMPLEX` | — |
| `nWorkDimA` | `UDINT;                 // It designates the dimension in the first multi array be` | — |
| `nWorkDimB` | `UDINT;                 // It designates the dimension in the second multi array b` | — |
| `pStartIndexA` | `POINTER TO UDINT;      (* optional: default:0-` | — |
| `pStartIndexB` | `POINTER TO UDINT;      // see pStartIndexA` | — |
| `nOptionPars` | `DWORD;                // option mask` | — |
| `pDataInA` | `POINTER TO BYTE;           // address of data buffer (e.g. oversampling data) as one-` | — |
| `nDataInSizeA` | `UDINT;                 // size of data buffer in bytes` | — |
| `pDataInB` | `POINTER TO BYTE;      // address of data buffer (e.g. oversampling data) as one-` | — |
| `nDataInSizeB` | `UDINT;                 // size of data buffer in bytes` | — |
| `eElementTypeA` | `E_MA_ElementTypeCode; // valid types: LREAL, INT32, UINT64, LCOMPLEX` | — |
| `eElementTypeB` | `E_MA_ElementTypeCode; // valid types: LREAL, INT32, UINT64, LCOMPLEX` | — |
| `nOptionPars` | `DWORD;               // option mask` | — |
| `pDataInA` | `POINTER TO BYTE;       // address of data buffer (e.g. oversampling data) as one-` | — |
| `nDataInSizeA` | `UDINT;                  // size of data buffer in bytes` | — |
| `pDataInB` | `POINTER TO BYTE;       // address of data buffer (e.g. oversampling data) as one-` | — |
| `nDataInSizeB` | `UDINT;                  // size of data buffer in bytes` | — |
| `eElementTypeA` | `E_MA_ElementTypeCode; // valid types: LREAL, INT32, UINT64, LCOMPLEX` | — |
| `eElementTypeB` | `E_MA_ElementTypeCode; // valid types: LREAL, INT32, UINT64, LCOMPLEX` | — |
| `nWorkDimA0` | `UDINT;                 // It designates the first dimension in the first multi ar` | — |
| `nWorkDimA1` | `UDINT;                 // It designates the second dimension in the first multi a` | — |
| `nWorkDimB0` | `UDINT;                 // It designates the first dimension in the second multi a` | — |
| `nWorkDimB1` | `UDINT;                 // It designates the second dimension in the second multi` | — |
| `pStartIndexA` | `POINTER TO UDINT;      (* optional: default:0-` | — |
| `pStartIndexB` | `POINTER TO UDINT;      // see pStartIndexA` | — |
| `nOptionPars` | `DWORD;                // option mask` | — |
| `pDataInA` | `POINTER TO BYTE;      // address of data buffer (e.g. oversampling data) as one-` | — |
| `nDataInSizeA` | `UDINT;                 // size of data buffer in bytes` | — |
| `pDataInB` | `POINTER TO BYTE;      // address of data buffer (e.g. oversampling data) as one-` | — |
| `nDataInSizeB` | `UDINT;                 // size of data buffer in bytes` | — |
| `eElementTypeA` | `E_MA_ElementTypeCode; // valid types: LREAL, INT32, UINT64, LCOMPLEX` | — |
| `eElementTypeB` | `E_MA_ElementTypeCode; // valid types: LREAL, INT32, UINT64, LCOMPLEX` | — |
| `nOptionPars` | `DWORD;               // option mask` | — |
| `stInitParsA` | `ST_MA_MultiArray_InitPars;       // init parameters for first MultiArray` | — |
| `stInitParsB` | `ST_MA_MultiArray_InitPars;       // init parameters for second MultiArray` | — |
| `nOwnID` | `UDINT;                           // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT; // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be initi` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                 // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                              // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;        // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;                 // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                 // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;              // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;                // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;             // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;                 // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                 // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;              // '< 0' = error; '> 0' = info; '0' = no error/info` | — |
| `bNewResult` | `BOOL;                // TRUE every time when outgoing MultiArray was calculated` | — |
| `bError` | `BOOL;                // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;             // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** This function block writes data from external PLC data buffers to a pair of MultiArray buffers.

##### `FB_CMA_VibrationAssessment` (METHOD) → `HRESULT` — §5.1.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_VibrationAssessment_InitPars;        // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                         // timeout checking off during access to` | — |
| `stInitPars` | `ST_CM_VibrationAssessment_InitPars;           // init parameter` | — |
| `nOwnID` | `UDINT;                                        // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;              // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                   // number of MultiArrays which should be i` | — |
| `pArg1` | `POINTER TO LREAL; // pointer to 2-dimensional` | — |
| `nArgSize1` | `UDINT;            // size of arguments buffer in bytes` | — |
| `pArg2` | `POINTER TO LREAL; // pointer to 2-dimensional` | — |
| `nArgSize2` | `UDINT;            // size of arguments buffer in bytes` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;           // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;           // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Vibration assessment of real-valued input signals.

##### `FB_CMA_WatchUpperThresholds` (METHOD) → `HRESULT` — §5.1.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_WatchUpperThresholds_InitPars;          // init parameter` | — |
| `nOwnID` | `UDINT;                                        // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;              // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                    // number of MultiArrays which should be` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                         // timeout checking off during access to` | — |
| `pArg` | `POINTER TO LREAL; // pointer to array (LREAL) of arguments` | — |
| `nArgSize` | `UDINT;            // size of arguments buffer in bytes` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;           // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;           // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Configurable threshold value monitoring of multi-channel data

##### `FB_CMA_WindowStatistics` (METHOD) → `HRESULT` — §5.1.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_WindowStatistics_InitPars;         // init parameter` | — |
| `nOwnID` | `UDINT;                                         // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;               // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                    // number of MultiArrays which should` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                           // timeout checking off during access` | — |
| `pArg` | `POINTER TO LREAL; // pointer to array (LREAL) of arguments` | — |
| `nArgSize` | `UDINT;            // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_WindowStatistics_InitPars;                  // init parameter` | — |
| `nOwnID` | `UDINT;                                            // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;                  // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                       // number of MultiArrays which should b` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;           // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;           // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Calculation of statistical values in scalable windows.

##### `FB_CMA_ZoomFFT` (METHOD) → `HRESULT` — §5.1.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stInitPars` | `ST_CM_ZoomFFT_InitPars;                 // init parameter` | — |
| `nOwnID` | `UDINT;                                  // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;        // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                             // number of MultiArrays which should be in` | — |
| `tTransferTimeout` | `LTIME := LTIME#500US;                    // timeout checking off during access to in` | — |
| `pArg` | `PVOID;    // pointer to array of arguments` | — |
| `nArgSize` | `UDINT;    // size of arguments buffer in bytes` | — |
| `stInitPars` | `ST_CM_ZoomFFT_InitPars;                    // init parameter` | — |
| `nOwnID` | `UDINT;                                     // ID for this FB instance` | — |
| `aDestIDs` | `ARRAY[1..cCMA_MaxDest] OF UDINT;           // IDs of destinations for output` | — |
| `nResultBuffers` | `UDINT := 4;                                // number of MultiArrays which should be init` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChannelErrors` | `ARRAY[*] OF HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL;                                  // TRUE if an error occurs. Reset by next metho` | — |
| `hrErrorCode` | `HRESULT;                               // '< 0' = error; '> 0' = info; '0' = no error/` | — |
| `ipErrorMessage` | `I_TcMessage := fbErrorMessage;         // Shows detailed information about occurred er` | — |
| `nCntResults` | `ULINT;                                 // Counts outgoing results (MultiArrays were ca` | — |
| `bNewResult` | `BOOL;           // TRUE every time when outgoing MultiArray was calculated and sent t` | — |
| `bError` | `BOOL;           // TRUE if an error occurs.` | — |
| `hrErrorCode` | `HRESULT;         // '< 0' = error; '> 0' = info; '0' = no error/info` | — |

**Description:** Efficient calculation of spectral values over a defined frequency band

#### PLC API › Functions

##### `F_MA_IsNAN` (FUNCTION) → `BOOL` — §5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fValue` | `LREAL` | — |

**Description:** This function tests for presence of a NaN (Not-a-Number) value, returning TRUE when the value is a NaN.

##### `F_CM_CalculateRecommendedOverlap` (FUNCTION) → `UDINT` — §5.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eWindowType` | `E_CM_WindowType;               (* Window type. *)` | — |
| `aWindowParameters` | `T_CM_WindowParameters;          (* Window parameters for specific windows, e.g. F` | — |
| `nWindowLength` | `UDINT;                         (* Length of analysis window. *)` | — |

**Description:** This function calculates a recommended overlap based on the amplitude flatness and overlap correlation of the window function. See section: Calculation of the recommended overlap [} 21].

##### `F_CM_CalculateWoehlerCurve` (FUNCTION) → `HRESULT` — §5.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSRI` | `LREAL;                              (* Stress Range Intercept. *)` | — |
| `fUTS` | `LREAL;                              (* Ultimate Tensile Strength OF the material.` | — |
| `bUseUTSCorrection` | `BOOL;                                (* Apply UTS correction. *)` | — |
| `fK1` | `LREAL;                              (* Gradient of Woehler Curve for region N = 1` | — |
| `fK2` | `LREAL;                              (* Gradient of Woehler Curve starting from nN` | — |
| `nNC1` | `ULINT;                              (* Transition Point for UTS correction. *)` | — |
| `nNC2` | `ULINT;                              (* Fatigue Transition Point between fK1 and f` | — |
| `fRRatio` | `LREAL;                              (* R-Ratio of the material test. *)` | — |
| `fStepsize` | `LREAL;                              (* Width of stress bins. *)` | — |
| `nCurveLength` | `UDINT;                              (* Length of curve. *)` | — |
| `pCurve` | `POINTER TO POINTER TO LREAL;        (* Pointer to array data. *)` | — |

**Description:** This function calculates a Wöhler curve from material parameters as input.

##### `F_CM_CalculateWindow` (FUNCTION) → `HRESULT` — §5.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eWindowType` | `E_CM_WindowType;                    (* Window type. *)` | — |
| `aWindowParameters` | `T_CM_WindowParameters;              (* Window parameters for specific windows, e.g` | — |
| `nWindowLength` | `UDINT;                              (* Length of analysis window. *)` | — |
| `pWindow` | `POINTER TO POINTER TO LREAL;        (* Pointer to array data. *)` | — |

**Description:** This function calculates window functions to given input parameters. See section: Window functions [} 19].

##### `F_CM_ApplySpectralScaling` (FUNCTION) → `HRESULT` — §5.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eScalingType` | `E_CM_ScalingType;            (* Scaling type. *)` | — |
| `nFFT_Length` | `UDINT;                       (* Length of FFT. *)` | — |
| `nOrder` | `UDINT;                       (* Order of spectrum: 1 = magnitude, 2 = powe` | — |
| `fSampleRate` | `LREAL;                       (* Sample rate in Hertz. *)` | — |
| `bTransformToDecibel` | `BOOL;                        (* Transform to decibel. *)` | — |
| `fDecibelThreshold` | `LREAL;                       (* Minimum argument of decadic logarithm for` | — |
| `nWindowLength` | `UDINT;                       (* Length of analysis window. *)` | — |
| `pWindow` | `POINTER TO POINTER TO LREAL; (* Pointer to window data. *)` | — |
| `nResultLength` | `UDINT;                       (* Length of spectrum, i.e. nFFT_Length or nF` | — |
| `pSpectrum` | `POINTER TO POINTER TO LREAL; (* Pointer to spectrum. *)` | — |

**Description:** This function performs a scaling of spectral values. See section: Scaling of spectra [} 25]

#### Samples

##### `Event-based frequency analysis` — §6.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bTrigger` | `BOOL;     // Trigger signal, start with rising edge` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aInputSignal` | `ARRAY[1..cOversamples] OF LREAL; // input time signal` | — |

**Description:** This sample implements an event based frequency analysis. The generated signal consists of a noisy sine signal with a frequency of 200 Hz and pure noise, which alternate every two seconds. Buffering of the signal begins when a rising edge is detected in the (generated) input signal. The collected data are then relayed via FB_CMA_Source [} 255] to the function block FB_CMA_MagnitudeSpectrum [} 168].
