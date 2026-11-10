# Tc3_MLL

- **Manual:** `TF38x0_TC3_ML_NN_Inference_Engine_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Measurement · **Tier:** T4 · **API entries:** 15
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** ML neural-network inference needs an inference engine (ONNX/TFLite-class) or a licensed Beckhoff runtime.

---

#### PLC API › Function Blocks

##### `FB_MllPrediction` (FUNCTION_BLOCK) — §6.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stPredictionParameter` | `ST_MllPredictionParameters` | — |
| `eTraceLevel` | `TcEventSeverity` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | True if a asynchronous action is taking place |
| `bError` | `BOOL` | Indicates error in method |
| `ipResultMessa I_TcMessage` | `Contains the last invoked error ge` | — |
| `hrErrorCode` | `HRESULT` | Unique error code |

**Description:** Syntax

#### PLC API › Function Blocks › FB_MllPrediction

##### `CheckPreferredIODataTypes` (METHOD) → `BOOL` — §6.2.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fmtInputType` | `ETcMllDataType [} 81]` | Input type to check |
| `fmtOutputType ETcMllDataType [} 81]` | `Output type to check` | — |
| `IsPreferred` | `Reference To BOOL` | Return true if preferred typo |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** fmtOutputType ETcMllDataType

##### `CheckSupportedIODataTypes` (METHOD) → `BOOL` — §6.2.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fmtInputType` | `ETcMllDataType [} 81]` | Input type to check |
| `fmtOutputType ETcMllDataType [} 81]` | `Output type to check` | — |
| `IsSupported` | `Reference To BOOL` | returns true if supported |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** fmtOutputType ETcMllDataType

##### `Configure` (METHOD) → `BOOL` — §6.2.2.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** Syntax

##### `GetCustomAttribute_array` (METHOD) → `BOOL` — §6.2.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sCustomAttribu T_MaxString` | `Name of the custom attribute teName` | — |
| `fmtAttributeDat Reference To` | `Data format of the custom attribute` | — |
| `aType` | `ETcMllDataType [} 81]` | — |
| `pDataBuffer` | `PVOID` | Destination data buffer, where the custom attribute is copied into |
| `nDataBufferLe UDINT` | `Length of the destination data buffer in bytes n` | — |
| `nArrayLength Reference To UDINT` | `Number of data type elements (i.e. number of fp32 values) found in the custom attribute` | — |
| `pnBytesWritten Pointer To UDINT` | `Returns the number of bytes that have been written to the destination buffer` | — |
| `sCustomAttribu T_MaxString` | `Name of the custom fp64 attribute teName` | — |
| `lrDataOut` | `Reference To LREAL` | Output value of the custom fp64 attribute |
| `sCustomAttribu T_MaxString` | `Name of the custom int64 attribute teName` | — |
| `nDataOut` | `Reference To LINT` | Output value of the custom int64 attribute |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL BOOL BOOL` | `` | — |

**Description:** fmtAttributeDataType Reference To ETcMllDataType pDataBuffer PVOID nDataBufferLen UDINT nArrayLength Reference To UDINT pnBytesWritten Pointer To UDINT

##### `GetCustomAttribute_str` (METHOD) → `BOOL` — §6.2.2.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sCustomAttribu T_MaxString` | `Name of the custom string attribute teName` | — |
| `sDstAttributeSt Reference To` | `Pointer to a string buffer to write the custom string attribute into` | — |
| `ringBuffer` | `T_MaxString` | — |
| `pnStringLen` | `Pointer To UDINT` | (Optional) Actual length of the string attribute |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** sDstAttributeStringBuffer Reference To T_MaxString pnStringLen Pointer To UDINT

##### `GetEngineIdFromRef` (METHOD) → `BOOL` — §6.2.2.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sEngineRef` | `T_MaxString` | Reference string of model engine (or parameter set) used for |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** nEngineId Reference To UDINT

##### `GetInputDim` (METHOD) → `BOOL` — §6.2.2.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nInputDim` | `Reference To UDINT` | Size of the input data array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** Syntax

##### `GetMaxConcurrency` (METHOD) → `BOOL` — §6.2.2.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConcurrency` | `Reference To UDINT` | Maximum supported number of concurrently processing threads |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** Syntax

##### `GetModelName` (METHOD) → `BOOL` — §6.2.2.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sDstModelNam Reference To` | `Name of the loaded model` | — |
| `eBuffer` | `T_MaxString` | — |
| `pnStringLen` | `Pointer To UDINT` | (Optional) Actual length of the string attribute |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** pnStringLen Pointer To UDINT

##### `GetOutputDim` (METHOD) → `BOOL` — §6.2.2.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nOutputDim` | `Reference To UDINT` | Size of the output data array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** Syntax

##### `Predict` (METHOD) → `BOOL` — §6.2.2.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDataInp` | `PVOID PLC)` | Pointer to the input data array (e.g. ARRAY[0..10] OF REAL in |
| `nDataInpDim UDINT` | `Number of inputs in the current vector` | — |
| `fmtDataInpTyp ETcMllDataType [} 81]` | `ETcMllDataType data type of input array e` | — |
| `pDataOut` | `PVOID PLC)` | Pointer to the output data array (e.g. ARRAY[0..10] OF REAL in |
| `nDataOutDim UDINT` | `Number of outputs in the current vector` | — |
| `fmtDataOutTyp ETcMllDataType [} 81]` | `ETcMllDataType data type of output array e` | — |
| `nEngineId` | `UDINT` | Id of model engine (or parameter set) used for prediction, use |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** nDataInpDim UDINT fmtDataInpType ETcMllDataType pDataOut PVOID nDataOutDim UDINT fmtDataOutType ETcMllDataType nEngineId UDINT nConcurrencyId UDINT

##### `PredictRef` (METHOD) → `BOOL` — §6.2.2.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDataInp` | `PVOID PLC)` | Pointer to the input data array (e.g. ARRAY[0..10] OF REAL in |
| `nDataInpDim UDINT` | `Number of inputs in the current vector` | — |
| `fmtDataInpTyp ETcMllDataType [} 81]` | `ETcMllDataType data type of input array e` | — |
| `pDataOut` | `PVOID PLC)` | Pointer to the output data array (e.g. ARRAY[0..10] OF REAL in |
| `nDataOutDim UDINT` | `Number of outputs in the current vector` | — |
| `fmtDataOutTyp ETcMllDataType [} 81]` | `ETcMllDataType data type of output array e` | — |
| `sEngineRef` | `T_MaxString` | Reference string of model engine (or parameter set) used for |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** nDataInpDim UDINT fmtDataInpType ETcMllDataType pDataOut PVOID nDataOutDim UDINT fmtDataOutType ETcMllDataType sEngineRef T_MaxString nConcurrencyId UDINT

##### `Reset` (METHOD) → `BOOL` — §6.2.2.1.15

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** Syntax

##### `SetActiveEngineOptions` (METHOD) → `BOOL` — §6.2.2.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sEngineOption T_MaxString s` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BOOL` | `` | — |

**Description:** Syntax
