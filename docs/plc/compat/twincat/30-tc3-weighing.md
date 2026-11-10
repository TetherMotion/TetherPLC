# Tc3_Weighing

- **Manual:** `TF3685_TC3_Weighing_Library_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Measurement · **Tier:** T0 · **API entries:** 19
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Weighing algorithms (filtering, stabilization): T0; load-cell acquisition is T2 via fieldbus/ADC.

---

#### PLC API

##### `Function blocks` (FUNCTION_BLOCK) — §6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_WG_<type>` | — |
| `stConfig` | `ST_WG_<type>` | — |
| `pIn` | `POINTER TO LREAL; (*address of input array*)` | — |
| `nSizeIn` | `UDINT;            (*size of input array*)` | — |
| `pOut` | `POINTER TO LREAL; (*address of output array*)` | — |
| `nSizeOut` | `UDINT;            (*size of output array*)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | — |
| `bConfigured` | `BOOL` | — |
| `ipResultMessage` | `I_TCMessage` | — |

**Description:** Basic structure of the function blocks

#### PLC API › Function blocks

##### `FB_WG_ComboFilter` (FUNCTION_BLOCK) — §6.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_WG_ComboFilter [} 34]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for |

**Description:** The function block FB_WG_ComboFilter implements a PTn, Moving Average and Notch filter connected in series.

#### PLC API › Function blocks › FB_WG_ComboFilter

##### `Configure` (METHOD) → `BOOL` — §6.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_WG_ComboFilter [} 34]` | Structure for configuring the filter behavior Sample (*Declaration without configuration*) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the weighing instance was configured |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §6.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pIn` | `POINTER TO LREAL` | Address of the input array |
| `nSizeIn` | `UDINT` | Size of the input array |
| `pOut` | `POINTER TO LREAL` | Address of the output array |
| `nSizeOut` | `UDINT` | Size of the output array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if a manipulated output signal has been |

**Description:** The method calculates a manipulated output signal from an input signal that is transferred in the form of a pointer.

##### `Reset` (METHOD) → `BOOL` — §6.1.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the weighing |

**Description:** The method resets the internal status of the weighing instance. By resetting the function block, the weighing instance is reset to its original state, i.e. without any influence from the past. The weighing instance is therefore reset to the last configuration status.

#### PLC API › Function blocks

##### `FB_WG_Scaling` (FUNCTION_BLOCK) — §6.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_WG_Scaling [} 35]` | Function block-specific configuration structure |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for |

**Description:** The function block FB_WG_Scaling is used for scaling raw values. The raw values can be scaled individually or as an array, for example as oversampling values.

#### PLC API › Function blocks › FB_WG_Scaling

##### `Configure` (METHOD) → `BOOL` — §6.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_WG_Scaling [} 35]` | Structure for configuring the filter behavior Sample (*Declaration without configuration*) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the weighing instance was configured |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §6.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pIn` | `POINTER TO LREAL` | Address of the input array |
| `nSizeIn` | `UDINT` | Size of the input array |
| `pOut` | `POINTER TO LREAL` | Address of the output array |
| `nSizeOut` | `UDINT` | Size of the output array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if a manipulated output signal has been |

**Description:** The method calculates a manipulated output signal from an input signal that is transferred in the form of a pointer.

##### `Reset` (METHOD) → `BOOL` — §6.1.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the weighing |

**Description:** The method resets the internal status of the weighing instance. By resetting the function block, the weighing instance is reset to its original state, i.e. without any influence from the past. The weighing instance is therefore reset to the last configuration status.

##### `ApplyCalibration` (METHOD) → `BOOL` — §6.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bAccept` | `BOOL` | If TRUE, the CalibrateRefHigh()/CalibrateRefLow() results |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ApplyCalibration` | `BOOL` | TRUE if the method was executed successfully. |

**Description:** The method can be used at runtime to complete or cancel the triggered calibration process ( CalibrateRefHigh() [} 28]/CalibrateRefLow() [} 28]).

##### `CalibrateRefHigh` (METHOD) → `BOOL` — §6.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDurationInSamples UDINT` | `Number of samples to be averaged.` | — |
| `fRefHigh` | `LREAL` | New fReferenceHigh value. Sample |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CalibrateRefHigh` | `BOOL` | TRUE if the method was executed successfully. |

**Description:** The method can be used at runtime to trigger the fReferenceHigh calibration.

##### `CalibrateRefLow` (METHOD) → `BOOL` — §6.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDurationInSamples UDINT` | `Number of samples to be averaged.` | — |
| `fRefLow` | `LREAL` | New fReferenceLow value. Sample |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CalibrateRefLow` | `BOOL` | TRUE if the method was executed successfully. |

**Description:** The method can be used at runtime to trigger the fReferenceLow calibration.

##### `Tare` (METHOD) → `BOOL` — §6.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDurationInSamples UDINT` | `Number of samples to be averaged. Sample` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Tare` | `BOOL` | TRUE if the method was executed successfully. |

**Description:** The method can be used at runtime to tare the function block. The average is calculated via nDurationInSamples - output values. Finally, the result is passed to the method UpdateTareOffset [} 30]().

##### `UpdateTareOffset` (METHOD) → `=` — §6.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fOffset` | `LREAL` | The new tare weight. Sample |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdateTareOffset` | `BOOL` | TRUE if the method was executed successfully. |

**Description:** The method can be used at runtime to perform manual taring. This means that the fOffset value (weight) is subtracted from the calculated output values. In addition, the function block outputs tLastTare and fCurrentTareOffset (= fCurrentTareOffset- fOffset) are updated.

#### PLC API › Function blocks

##### `FB_WG_Weighing` (FUNCTION_BLOCK) — §6.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_WG_Weighing [} 35]` | Function block-specific configuration structure |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling |
| `bValidMeasurement BOOL` | `TRUE if the ST_WG_Weighing_Validation [} 36] conditions` | — |

**Description:** The function block FB_WG_Weighing is used to determine a measured weight.

#### PLC API › Function blocks › FB_WG_Weighing

##### `Configure` (METHOD) → `=` — §6.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_WG_Weighing [} 35]` | Configuration structure Sample (*Declaration without configuration*) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the weighing instance was configured |

**Description:** The method can be used at runtime to initially configure the weighing instance (if not already done in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §6.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pIn` | `POINTER TO LREAL` | Address of the input array |
| `nSizeIn` | `UDINT` | Size of the input array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if a manipulated output signal has been |

**Description:** The method calculates a manipulated output signal from an input signal that is transferred in the form of a pointer. If oversampling is used, not all information is displayed. The results of the function block outputs refer to the oldest input value of Call().

##### `Reset` (METHOD) → `BOOL` — §6.1.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the weighing |

**Description:** The method resets the internal status of the weighing instance. By resetting the function block, the weighing instance is reset to its original state, i.e. without any influence from the past. The weighing instance is therefore reset to the last configuration status.

##### `AutoTare` (METHOD) → `=` — §6.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IScaling` | `I_WG_Scaling` | Function block with the interface I_WG_Scaling |
| `eAutoTareType` | `E_WG_AutoTareType [} 39]` | Behavior for automatic taring Sample // Scaling stParamsScaling : ST_WG_Scaling := ( fRawLow:=0.0, fReferenceLow:=0.0, fRawHigh:=1000.0, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AutoTare` | `BOOL` | TRUE if the method call was executed successfully. |

**Description:** The method can be used at runtime to automatically tare a FB_WG_Scaling [} 24] instance.
