# Tc3_Filter

- **Manual:** `TF3680_TC3_FILTER_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Measurement · **Tier:** T0 · **API entries:** 61
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** IIR/FIR filter blocks: pure numeric kernels.

---

#### PLC API

##### `Function blocks` (FUNCTION_BLOCK) — §5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_<type>` | — |
| `stConfig` | `ST_FTR_<type>` | — |
| `pIn` | `POINTER TO LREAL; (*address of input array*)` | — |
| `nSizeIn` | `UDINT;             (*size of input array*)` | — |
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

##### `FB_FTR_IIRCoeff` (FUNCTION_BLOCK) — §5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_IIRCoeff [} 75]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The function block implements an IIR filter (Infinite Impulse Response filter). The general transfer function is:

#### PLC API › Function blocks › FB_FTR_IIRCoeff

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_IIRCoeff [} 75]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.1.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

#### PLC API › Function blocks

##### `FB_FTR_IIRSos` (FUNCTION_BLOCK) — §5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_IIRSos [} 75]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The function block implements an IIR filter (Infinite-Impulse-Response filter). The transfer function is:

#### PLC API › Function blocks › FB_FTR_IIRSos

##### `Call` (METHOD) → `BOOL` — §5.1.2.1

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

##### `Reset` (METHOD) → `BOOL` — §5.1.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

##### `Configure` (METHOD) → `BOOL` — §5.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_IIRSos [} 75]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

#### PLC API › Function blocks

##### `FB_FTR_IIRSpec` (FUNCTION_BLOCK) — §5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_IIRSpec [} 76]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The function block implements an IIR filter (Infinite Impulse Response filter).

#### PLC API › Function blocks › FB_FTR_IIRSpec

##### `Configure` (METHOD) → `BOOL` — §5.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_IIRSpec [} 76]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pIn` | `POINTER TO LREAL` | Address of the input array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if a manipulated output signal has been |

**Description:** The method calculates a manipulated output signal from an input signal that is transferred in the form of a pointer.

##### `Reset` (METHOD) → `BOOL` — §5.1.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

#### PLC API › Function blocks

##### `FB_FTR_MovAvg` (FUNCTION_BLOCK) — §5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_MovAvg [} 77]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The function block implements a moving average filter. The filter calculates the mean value of M input values x[n-k] with k = 0 .. M - 1.

#### PLC API › Function blocks › FB_FTR_MovAvg

##### `Configure` (METHOD) → `BOOL` — §5.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_MovAvg [} 77]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.4.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PT1 [} 77]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

#### PLC API › Function blocks

##### `Configure` (METHOD) → `BOOL` — §5.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PT1 [} 77]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.5.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PT2 [} 78]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

##### `Configure` (METHOD) → `BOOL` — §5.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PT2 [} 78]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.6.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PT3 [} 78]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

##### `Configure` (METHOD) → `BOOL` — §5.1.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PT3 [} 78]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.7.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.7.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

##### `FB_FTR_PTn` (FUNCTION_BLOCK) — §5.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PTn [} 79]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The function block FB_FTR_PTn implements a nth-order delay element with the same time constants. The complex transfer function (Laplace space):

#### PLC API › Function blocks › FB_FTR_PTn

##### `Configure` (METHOD) → `BOOL` — §5.1.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PTn [} 79]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.8.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.8.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

#### PLC API › Function blocks

##### `FB_FTR_Notch` (FUNCTION_BLOCK) — §5.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_Notch [} 79]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The function block FB_FTR_Notch implements a band-stop filter with narrow bandwidth. The complex transfer function (Laplace space):

#### PLC API › Function blocks › FB_FTR_Notch

##### `Configure` (METHOD) → `BOOL` — §5.1.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_Notch [} 79]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.9.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.9.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

#### PLC API › Function blocks

##### `FB_FTR_LeadLag` (FUNCTION_BLOCK) — §5.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_LeadLag [} 80]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |

**Description:** The function block FB_FTR_LeadLag implements a first order phase correction element with the complex transfer function (Laplace space):

#### PLC API › Function blocks › FB_FTR_LeadLag

##### `Configure` (METHOD) → `BOOL` — §5.1.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_Notch [} 80]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.10.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.10.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

#### PLC API › Function blocks

##### `FB_FTR_PT2oscillation` (FUNCTION_BLOCK) — §5.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PT2oscillation [} 81]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The function block FB_FTR_PT2oscillation implements an oscillating second-order delay element with the complex transfer function (Laplace space):

#### PLC API › Function blocks › FB_FTR_PT2oscillation

##### `Configure` (METHOD) → `BOOL` — §5.1.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PT2oscillation [} 81]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.11.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.11.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

#### PLC API › Function blocks

##### `FB_FTR_PTt` (FUNCTION_BLOCK) — §5.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PTt [} 81]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The function block FB_FTR_PTt implements a dead time element (PTt element) with the complex transfer function (Laplace space):

#### PLC API › Function blocks › FB_FTR_PTt

##### `Configure` (METHOD) → `BOOL` — §5.1.12.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PTt [} 81]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.12.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.12.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

#### PLC API › Function blocks

##### `FB_FTR_Median` (FUNCTION_BLOCK) — §5.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_Median [} 82]` | Structure for configuring the filter behavior. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The function block FB_FTR_Median implements a median filter. The median is the middle value in a list of values sorted by size. This means that at least half of the values are less than or equal to the median, and at least half are greater than or equal to the median. If the list contains an even number of values, the median is equal to the arithmetic mean of the two middle values.

#### PLC API › Function blocks › FB_FTR_Median

##### `Configure` (METHOD) → `BOOL` — §5.1.13.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_Median [} 82]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.13.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.13.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

#### PLC API › Function blocks

##### `FB_FTR_ActualValue` (FUNCTION_BLOCK) — §5.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_ActualValue [} 82]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for |

**Description:** The function block allows a measured input value to be checked for plausibility and filtered.

#### PLC API › Function blocks › FB_FTR_ActualValue

##### `Configure` (METHOD) → `BOOL` — §5.1.14.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_Median [} 82]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.14.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.14.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

##### `GetFilterActive` (METHOD) → `BOOL` — §5.1.14.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pFilterActive` | `POINTER TO BOOL` | Address of the input array |
| `nSizeGetFilterActive UDINT` | `Size of the input array` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetFilterActive` | `BOOL` | Returns TRUE if active state was calculated. Sample |

**Description:** The method calculates which elements of the input signal were changed, starting from the last Call().

##### `GetFilterActiveTimestamps` (METHOD) → `BOOL` — §5.1.14.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pFilterActiveTimesta POINTER TO ULINT` | `Address of the input array mps` | — |
| `nSizeFilterActiveTim UDINT` | `Size of the input array estamps` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetFilterActiveTimes BOOL` | `Returns TRUE if the timestamps have been calculated. tamps Sample` | — |

**Description:** The method calculates when the filter was last active, based on the last Call().

#### PLC API › Function blocks

##### `FB_FTR_Gaussian` (FUNCTION_BLOCK) — §5.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_Gaussian [} 82]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE, if an error occurs. |
| `bConfigured` | `BOOL` | TRUE if the configuration was successful. |
| `ipResultMessage` | `I_TCMessage` | Interface that provides properties and methods for message handling Methods |
| `Configure()` | `Local` | Loads a new (or initial) configuration structure. |
| `Call()` | `Local` | Calculates the output signal for a given input signal and |

**Description:** The function block FB_FTR_Gaussian implements a Gaussian filter. The filter calculates the weighted average of M input values. The weights are calculated using a Gaussian function, where the standard deviation σ determines the cut-off frequency of the filter.

#### PLC API › Function blocks › FB_FTR_Gaussian

##### `Configure` (METHOD) → `BOOL` — §5.1.15.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_FTR_PT1 [} 77]` | Structure for configuring the filter behavior |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | TRUE if the filter instance was configured successfully. Sample (*Declaration without configuration*) |

**Description:** This method can be used at runtime to initially configure the instance of a filter (if it was not already configured in the declaration) or to reconfigure it.

##### `Call` (METHOD) → `BOOL` — §5.1.15.2

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

##### `Reset` (METHOD) → `BOOL` — §5.1.15.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the internal status of the filter was |

**Description:** The method resets the internal status of the filter. The current output value y[n] consists of the current input value x[n] and past input and output values (x[n-k] and y[n-k], k > 0). Resetting the function block returns the filter to its original state, i.e. without any influence from the past. The filter is thus reset to the last configuration state.

#### (top level)

##### `Declaring and calling filters with FB_FTR_<type>` (FUNCTION_BLOCK) — §6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `aBuffer` | `ARRAY [1..MAIN.cChannels] OF ARRAY [1..MAIN.cOversamples] OF LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aOutput` | `ARRAY [1..MAIN.cChannels] OF ARRAY [1..MAIN.cOversamples] OF LREAL` | — |

**Description:** This sample shows how the individual filter function blocks of the PLC library Tc3_Filter are declared and called.
