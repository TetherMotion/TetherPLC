# Tc3_Analytics

- **Manual:** `TF3510_TC3_Analytics_Library_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Measurement · **Tier:** T4 · **API entries:** 672
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Analytics engine: needs a time-series aggregation service; storage provider variant needs a database.

---

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_BatchNShift_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The BatchNShift 1Ch buffers the values of the input signal according to the buffer size and the sample mode. The number of output channels in which the buffered input values are stored corresponds to the buffer size. With the help of the sample mode it is possible to distinguish between two different operating modes of the algorithm. If the sample mode Flow is selected, a ring buffer or shift register is realized (Shift). The values are written to the buffer one after the other and shifted by one position of the buffer in each cycle. If the buffer is full, the last value falls out. In the Wait mode, the buffer is instead completely emptied and filled with new values whenever it is completely full, so that the values are processed in the form of batches (batch). At the beginning of an analysis, the system also waits until the buffer is completely filled before writing the values to the buffer. Therefore, the function block supplies valid values only from the cycle (BufferSize + 1).

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_BatchNShift_1Ch

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nBufferSize` | `UDINT` | Indicates the size of the buffer and thus the number of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initialization of the buffer size.

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eSampleMode` | `E_ALY_SampleMode` | The values from the buffer can be passed to the output |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.1.1.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `GetChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alue` | — |

**Description:** Fetching a channel-specific output value. The output value is only updated if the Call() method was called previously.

##### `GetOutputArray` (METHOD) → `BOOL` — §5.1.1.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArrayOut` | `POINTER TO LREAL` | Pointer to the output array: Dim: nBufferSize |
| `nArrayOutSize` | `UDINT` | Size of the output array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetOutputArray` | `BOOL` | Returns TRUE if successful. |

**Description:** Getting the entire output array. The number of elements corresponds to the configured buffer size.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_Bitmask` (FUNCTION_BLOCK) — §5.1.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Bitmask module can be used to evaluate individual bits. To do this, the individual bits of the input value are output as Boolean values at the output.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_Bitmask

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArrayOut` | `POINTER TO BOOL` | Pointer to the output array. Dim.: Size of input value * 8 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.1.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_ContinuousPieceCounter_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Continuous Piece Counter 1Ch counts the number of pieces within the configured interval. The counter is increased when the signal of the input channel passes the configured edge at a specific threshold. The calculation restarts when the time of the interval has elapsed. The algorithm provides the amount of pieces, the minimal and the maximal number of pieces as well as the time values of minimum and maximum.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_ContinuousPieceCounter_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tInterval` | `LTIME` | Interval time in which the events are to be considered |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.1.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Pause` (METHOD) → `BOOL` — §5.1.1.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Pause` | `BOOL` | Returns TRUE if successful. |

**Description:** With the Pause method it is possible to pause the execution of the function block. The internally running time interval does not continue after calling the pause function. A single call of the method is sufficient. The interval is not continued until the next call of the Call method.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_Downsampling_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Downsampling 1Ch processes the values of the input channel with a configurable downsampling factor. This achieves downsampling so that the output signal is a representation of the input signal at a lower sampling rate. This can be useful, for example, to better identify trends or to perform subsequent compression of highly sampled signals if only lower sampling rates are required within the analysis. This is a simple way to increase the performance of the analysis.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_Downsampling_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDownsamplingFact UDINT` | `The factor used for downsampling. For example, if the` | — |
| `or` | `downsampling factor is 100, only every 100th value is` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.1.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_EdgeCounter_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Edge Counter 1Ch counts the amount of raised events. An event is raised when the signal of the input channel passes the configured edge at a specific threshold.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_EdgeCounter_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.5.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.5.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.1.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCount` | `ULINT` | Initializes the number of detected edges. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_EdgeCounterOnOff_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Edge Counter On Off 1Ch counts the amount of raised on- and off-events. An on-event is raised when the signal of the input channel passes the configured edge at a specific on-threshold and an off-event is raised when the off-threshold is passed by the same signal.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_EdgeCounterOnOff_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.6.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdEdge` | `ST_ALY_ThresholdOnOff` | Combination of comparison operator and threshold for on- |
| `[} 427]` | `state and off-state` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.6.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.1.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCountOn` | `ULINT` | Initializes the number of detected On events. |
| `nCountOff` | `ULINT` | Initializes the number of detected Off events. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_EdgeCounterOnOff_2Ch` (FUNCTION_BLOCK) — §5.1.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Edge Counter On Off 2Ch counts the amount of raised on- and off-events. An on-event is raised when the signal of the first input channel passes the configured edge at a specific on-threshold and an off-event is raised when the off-threshold is passed by the signal of the second channel.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_EdgeCounterOnOff_2Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetOnMultipleOn BOOL` | `Reset in case of multiple On event` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: On 2: Off |
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.7.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: On 2: Off |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.1.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCountOn` | `ULINT` | Initializes the number of detected On events. |
| `nCountOff` | `ULINT` | Initializes the number of detected Off events. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_EventTimingAnalysis_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |
| `bInitWithThresholdL BOOL` | `If the value is TRUE, the algorithm uses a threshold to` | — |
| `evel` | `initialize the internal state, instead of waiting for an edge.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Event Timing Analysis 1Ch measures time differences between on- and off-event and counts the amount of raised events. An on-event is raised when the signal of the input channel passes the configured edge at a specific on-threshold and an off-event is raised when the off-threshold is passed by the same signal.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_EventTimingAnalysis_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.8.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdEdge` | `ST_ALY_ThresholdOnOff` | Combination of comparison operator and threshold for on- |
| `[} 427]` | `state and off-state.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.8.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_EventTimingAnalysis_2Ch` (FUNCTION_BLOCK) — §5.1.1.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |
| `bInitWithThresholdL BOOL` | `If the value is TRUE, the algorithm uses a threshold to` | — |
| `evel` | `initialize the internal state, instead of waiting for an edge.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Event Timing Analysis 2Ch measures time differences between on- and off-event and counts the amount of raised events. An on-event is raised when the signal of the first input channel passes the configured edge at a specific on-threshold and an off-event is raised when the off-threshold is passed by the signal of the second channel.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_EventTimingAnalysis_2Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetOnMultipleOn BOOL` | `Reset in case of multiple On event` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: On 2: Off |
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.9.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: On 2: Off |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_FlipFlop_2Ch` (FUNCTION_BLOCK) — §5.1.1.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Flip Flop 2Ch implements a bistable flip-flop. The dominance for setting (RS) or resetting (SR) the output value can be configured.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_FlipFlop_2Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetIsDominant` | `BOOL` | The dominance of setting (RS) or resetting (SR) can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Set (S) 2: Reset (R) |
| `stThresholdLevel` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.10.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.10.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Set (S) 2: Reset (R) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.1.10.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCount` | `ULINT` | Initializes the counter value. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_IntervalPieceCounter_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Interval Piece Counter 1Ch counts the amount of raised events within a configured interval, which starts when the value of the start interval flag is TRUE. An event is raised when the signal of the input channel passes the configured edge at a specific threshold. The calculation restarts when the time of the interval has elapsed and the value of the start interval flag is True again.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_IntervalPieceCounter_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp. |
| `bStartInterval` | `BOOL` | Start processing the configured interval. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetOnMultipleSt BOOL` | `Reset in case of multiple "Start" event art` | — |
| `tInterval` | `LTIME` | Interval time in which the events are to be considered |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.11.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.11.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Pause` (METHOD) → `BOOL` — §5.1.1.1.11.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Pause` | `BOOL` | Returns TRUE if successful. |

**Description:** With the Pause method it is possible to pause the execution of the function block. The internally running time interval does not continue after calling the pause function. A single call of the method is sufficient. The interval is not continued until the next call of the Call method.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_LatchingSwitch_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Latching Switch 1Ch realizes a virtual impulse switch. The output alternates between TRUE and FALSE on each edge detected at the input.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_LatchingSwitch_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.12.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.12.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.12.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.12.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.12.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.1.12.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCount` | `ULINT` | Initializes the counter value. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_LifecycleAnalysis_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Lifecycle Analysis 1Ch calculates the elapsed and the estimated remaining cycles of a device. When the signal of the input channel passes the configured edge at a specific threshold, the elapsed cycles are increased and the remaining cycles are decreased.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_LifecycleAnalysis_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.13.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.13.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCyclesEstimated` | `ULINT` | Estimated component cycles |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.13.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.13.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.13.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInitial` (METHOD) → `BOOL` — §5.1.1.1.13.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCyclesElapsed` | `ULINT` | Initializes the number of elapsed cycles. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_LifetimeAnalysis_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Lifetime Analysis 1Ch calculates the elapsed and the estimated remaining lifetime of a device. If the input value met the configured condition the lifetime will be reduced.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_LifetimeAnalysis_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.14.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.14.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tLifetimeEstimated` | `LTIME` | Estimated component lifetime |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.14.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdLevel` | `ST_ALY_Threshold [} 426] Combination of comparison operator and used threshold` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.14.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.14.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.1.14.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCount` | `ULINT` | Initializes the lifetime that has already expired. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_MinMaxAvg` (FUNCTION_BLOCK) — §5.1.1.1.15

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Min Max Avg calculates the minimum, maximum and the average of the input values from the beginning of the analysis up to the current moment. Compared to Min Max Avg 1Ch, any number of input channels are supported. The calculations are performed separately for each input channel. Additional output values are also calculated across all input channels.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_MinMaxAvg

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.1.15.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.15.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.15.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `GetChannelOutputArray` (METHOD) → `BOOL` — §5.1.1.1.15.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1…3) 1: Minimum values of all input channels 2: Maximum values for all input channels 3: Average values for all input channels |
| `pArrayOut` | `PVOID` | Pointer to an array into which the output values are to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputA BOOL` | `Returns TRUE if successful. rray` | — |

**Description:** Retrieves a channel-specific output array. The output values were updated in the previously called Call() method.

##### `Reset` — §5.1.1.1.15.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_MinMaxAvg_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Min Max Avg 1Ch calculates the minimum, maximum and the average of the input values from the beginning of the analysis up to the current moment. Furthermore, the time values of minimum and maximum are shown.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_MinMaxAvg_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.16.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.1.16.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.16.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_MinMaxAvgInterval_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.17

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Min Max Avg Interval 1Ch calculates the minimum, maximum and the average of the input values for the time period of the configured Interval. Furthermore the time values of minimum and maximum are shown. Note that all values are from the relative last interval and that they will only be updated when the interval is over. The calculation restarts when the time of the interval has elapsed.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_MinMaxAvgInterval_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.17.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.17.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tInterval` | `LTIME` | Interval time |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm.

##### `Reset` — §5.1.1.1.17.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.17.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Pause` (METHOD) → `BOOL` — §5.1.1.1.17.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Pause` | `BOOL` | Returns TRUE if successful. |

**Description:** With the Pause method it is possible to pause the execution of the function block. The internally running time interval does not continue after calling the pause function. A single call of the method is sufficient. The interval is not continued until the next call of the Call method.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_MovingAvg_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.18

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Moving Average 1Ch calculates the moving average, the minimum and the maximum of the most recent input values in an interval of specified length. Furthermore the time values of minimum and maximum are shown. The calculation of the moving average depends on the configuration parameters Num Values and

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_MovingAvg_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.18.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.18.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumValues` | `ULINT` | Number of values included in the calculation of the moving |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm.

##### `Reset` — §5.1.1.1.18.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.18.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_MovingIntervalCounter_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Moving Interval Counter 1Ch counts the amount of raised events within a configured interval. An event is raised when the signal of the input channel passes the configured edge at a specific threshold. The calculation restarts when the time of the interval has elapsed.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_MovingIntervalCounter_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.19.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.19.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tInterval` | `LTIME` | Time interval in which the values are to be calculated. |
| `nCountLimit` | `UDINT` | Limit of counts in the configured interval |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.19.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.19.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.19.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_OverallEquipmentEffectiveness` (FUNCTION_BLOCK) — §5.1.1.1.20

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Overall Equipment Effectiveness (OEE) calculates key figures that make it possible to compare the current state of the manufacturing process with its maximum potential.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_OverallEquipmentEffectiveness

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.20.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.20.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tIdealCycleTime` | `BOOL` | Ideal cycle time for the production of one unit. |
| `fThresholdLevelOk` | `LREAL` | The overall equipment effectiveness greater than the |
| `Warning` | `configured threshold is classified as OK. If the overall` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.1.20.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.20.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_ProductivityDiagnosis_3Ch` (FUNCTION_BLOCK) — §5.1.1.1.21

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Productivity Diagnosis 3Ch algorithm calculates the productivity of the process during a production interval. The diagram below schematically illustrates the relationship between the production process and the individual production cycles.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_ProductivityDiagnosis_3Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.21.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.21.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumProducedPiec ULINT` | `Setpoint for pieces produced during the configured time` | — |
| `es` | `interval (Production Time).` | — |
| `tProductionTime` | `LTIME` | Time interval in which nNumProducedPieces are |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.21.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Is producing (level) 2: Start production cycle (edge) 2: Stop production cycle (level) |
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.21.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.21.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Is producing (level) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_ProductivityInterval_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The algorithm Productivity Interval 1Ch calculates the productivity of the process during a given interval. The interval can be defined by the inputs tTimeStart and tTimeStop. The pieces produced are taken into account during execution. A produced element is counted when an edge is applied to the input. The estimated productivity of the current interval and the productivity of the last complete interval are provided as output values. The algorithm can be configured with the target value of the produced pieces within a given interval.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_ProductivityInterval_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.22.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeStart` | `LTIME` | Start time of the specified interval. Allows the interval to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.22.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nExpectedPieces` | `ULINT` | Specification of the pieces to be produced within the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.22.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold. This` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.22.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.22.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_SignalGenerator_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.23

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Signal Generator 1Ch can be used to generate various signal curves. The signal type, the frequency, the amplitude and the offset can be set individually.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_SignalGenerator_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.23.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Calling the algorithm.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.23.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eFunctionType` | `E_ALY_FunctionType` | Function type of the generated signal. [} 432] Const: Constant Rectangle: Rectangle function Sawtooth: Sawtooth function Sine: Sine function Triangle: Triangle function |
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed. |
| `fFrequency` | `LREAL` | Frequency of the generated signal. |
| `fAmplitude` | `LREAL` | Configuration of the signal amplitude. |
| `fOffset` | `LREAL` | Constant offset of the generated signal. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.1.23.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_TimeClock_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.24

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Time Clock 1Ch executes a time switch which can be configured with switch-on time, switch-off time and the days of the week on which the time switch should be active. A timestamp is required as a reference value because the algorithm needs a time context in which to operate.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_TimeClock_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.24.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.24.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeOn` | `LTIME` | Time at which the timer should switch on |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.1.24.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_Timer_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.25

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Timer 1Ch starts a timer which can be configured by timer mode and interval. According to the specific timer mode the timer is started, if the configured condition becomes TRUE (TON, TP) or the condition becomes FALSE (TOF).

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_Timer_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.25.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.25.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eTimerMode` | `E_ALY_TimerMode [} 431] Mode of the timer:` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.25.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdLevel` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.25.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.25.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Pause` (METHOD) → `BOOL` — §5.1.1.1.25.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Pause` | `BOOL` | Returns TRUE if successful. |

**Description:** With the Pause method it is possible to pause the execution of the function block. The internally running time interval does not continue after calling the pause function. A single call of the method is sufficient. The interval is not continued until the next call of the Call method.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_TimingAnalysis_1Ch` (FUNCTION_BLOCK) — §5.1.1.1.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Timing Analysis 1Ch measures time differences between on- and off-periods and counts the amount of on-periods. The on-period starts when the condition of operator and threshold is met.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_TimingAnalysis_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.26.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.1.26.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.1.26.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stThresholdLevel` | `ST_ALY_Threshold [} 426] Combination of comparison operator and used threshold` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.1.26.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.1.26.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Base

##### `FB_ALY_Calendar` (FUNCTION_BLOCK) — §5.1.1.1.27

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Calendar creates a recurring event at certain times. The repeat mode can be set individually. The modes of daily, weekly, monthly or annual repetition are available. A timestamp is required as a reference value, as the algorithm needs a time context in which to work. This reference timestamp is set automatically, if there is another algorithm in the configuration. Therefore, it is not possible to use the Calendar individually.

#### PLC API › Function blocks › Algorithms › Base › FB_ALY_Calendar

##### `Call` (METHOD) → `BOOL` — §5.1.1.1.27.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `ConfigureDaily` (METHOD) → `BOOL` — §5.1.1.1.27.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeEvent` | `LTIME` | Time of the event. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureDaily` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm for a daily repetition of the events.

##### `ConfigureWeekly` (METHOD) → `BOOL` — §5.1.1.1.27.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeEvent` | `LTIME` | Time of the event. |
| `nDayOfWeekMask` | `WORD` | Bit mask for selecting the day of the week on which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureWeekly` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm for a weekly repetition of the events.

##### `ConfigureMonthly` (METHOD) → `BOOL` — §5.1.1.1.27.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeEvent` | `LTIME` | Time of the event. |
| `nDay` | `UINT` | Day on which an event is to be created. |
| `eRecurrenceSpecific E_ALY_RecurrenceSpecific` | `Recurrence specification` | — |
| `ation` | `ation [} 433]` | Specific Day: The event should be created on a specific |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureMonthly` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm for a monthly repetition of the events.

##### `ConfigureYearly` (METHOD) → `BOOL` — §5.1.1.1.27.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeEvent` | `LTIME` | Time of the event |
| `nDay` | `UINT` | Day on which an event is to be created. |
| `eRecurrenceSpecific E_ALY_RecurrenceSpecific` | `Recurrence specification` | — |
| `ation` | `ation [} 433]` | Specific Day: The event should be created on a specific |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureYearly` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm for an annual repetition of the events.

##### `Reset` — §5.1.1.1.27.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_BandwidthClassificator_1Ch` (FUNCTION_BLOCK) — §5.1.1.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Bandwidth Classifier 1Ch determines whether the input signal is within the configured limits or is less than or greater than the limits.

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_BandwidthClassificator_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fLowerBound` | `LREAL` | Lower limit for comparison. |
| `fUpperBound` | `LREAL` | Upper limit for comparison. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.2.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_BandwidthClassificator_3Ch` (FUNCTION_BLOCK) — §5.1.1.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Bandwidth Classifier 3Ch determines whether the input signal is within the limits or is less than or greater than the limits. The limits can be configured with input signals, so it is possible to use curves as lower and upper band.

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_BandwidthClassificator_3Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Data in 2: Lower bound 3: Upper bound |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_CurveSketcher_1Ch` (FUNCTION_BLOCK) — §5.1.1.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Curve Sketcher 1Ch identifies inversions (peaks and valleys) in an input data stream. Furthermore, local maxima of the absolute difference between two consecutive values (referred to as Delta) can be identified. Analogous to a continuous curve, the identified peaks and valleys correspond to local maxima and minima. Delta corresponds to the slope, so that a maximum of the absolute values of Delta can be associated with an inflection point.

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_CurveSketcher_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.2.3.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.2.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fThresholdReversal` | `LREAL` | Threshold for peak analysis |
| `bCalcInflection` | `BOOL` | Max Delta evaluation |
| `fThresholdDelta` | `LREAL` | Threshold for Delta analysis |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_Histogram_1Ch` (FUNCTION_BLOCK) — §5.1.1.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Histogram 1Ch calculates the distribution of a single channel input value cyclically. It can be configured with minimal bin, maximal bin and the total amount of bins. The dimension of the output array is the number of bins + 2. Because values that are less than the minimal bin are stored in the first array element and values greater than the maximal bin are stored in the last array element.

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_Histogram_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pHistArrayOut` | `PVOID` | Pointer to histogram array. Dim.: nBins + 2 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.2.4.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.2.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eHistMode` | `E_ALY_HistMode [} 432]` | Operation mode of the histogram. Abs: Absolute values |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `GetResults` (METHOD) → `BOOL` — §5.1.1.2.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pHistArrayOut` | `POINTER TO ULINT` | Pointer to the histogram array. Dim.: nBins + 2 |
| `nHistArrayOutSize` | `UDINT` | Size of the histogram array. Dim.: nBins + 2 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetResults` | `BOOL` | Returns TRUE if successful |

**Description:** Get result array without adding new values.

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.2.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eStateHistMode` | `E_ALY_HistMode [} 432]` | Operation mode of the histogram. Abs: Absolute values |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_SectionTimer_1Ch` (FUNCTION_BLOCK) — §5.1.1.2.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Section Timer 1Ch calculates the timespan the input is in range of each configured section. It can be configured with the amount of sections and the borders of each section. Each section is defined with lower border (greater than or equal to) and upper border (less than). The following sections lower border is set by the previous upper border. Values that are less than the minimal border are stored in the first array element. Values that are greater or equal than the maximal border are stored in the last array element.

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_SectionTimer_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp. |
| `pTimespanArrayOut` | `POINTER TO LINT` | Pointer to the timespan array. These values are: Dim.: nNumSections + 2 |
| `nTimespanArrayOutSize` | `UDINT` | Size of the timespan array. Dim.: nNumSections + 2 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.2.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTimespanArrayIn` | `POINTER TO ULINT` | Pointer to timespan array to be set as output values: Dim.: nNumSections + 2 |
| `nTimespanArrayInSi UDINT` | `Size of the timespan array to be set as output values. Dim.:` | — |
| `ze` | `nNumSections + 2` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.2.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumSections` | `UDINT` | Configures the number of sections. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `SetSections` (METHOD) → `BOOL` — §5.1.1.2.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fFirstLowerBorder` | `LREAL` | Configuration of the first lower border. |
| `pSectionConfigArray POINTER TO LREAL` | `Pointer to array of upper section limits: Dim: nNumSections` | — |
| `nSectionConfigArray UDINT` | `Size of the section configuration array. Dim.:` | — |
| `Size` | `nNumSections` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetSections` | `BOOL` | Returns TRUE if successful |

**Description:** Set sections after the FB has been configured. Each section is defined with lower border (greater than or equal to) and upper border (less than). The following sections lower border is set by the previous upper border. The first lower border is set separately.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `GetResults` (METHOD) → `BOOL` — §5.1.1.2.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTimespanArrayOut POINTER TO LINT` | `Pointer to the timespan array. These values are: Dim.: nNumSections + 2` | — |
| `nTimespanArrayOut UDINT` | `Size of the timespan array. Dim.: nNumSections + 2 Size` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetResults` | `BOOL` | Returns TRUE if successful |

**Description:** Get result array without adding new values.

##### `Reset` — §5.1.1.2.5.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_StateHistogram_1Ch` (FUNCTION_BLOCK) — §5.1.1.2.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The State Histogram 1Ch counts how often the input signal (INT) has a specific value between the configured minimum and maximum and shows the distribution in a histogram. The first bar represents the boundary values which are smaller than the minimum and the last bar represents the boundary values which are greater than the maximum. The State Histogram 1Ch is suitable for state-machines to show how often the different states are executed.

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_StateHistogram_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pHistArrayOut` | `PVOID` | Pointer to histogram array. The data type depends on the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.2.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eStateHistMode` | `E_ALY_StateHistMode` | Operation mode of the histogram. [} 430] Abs: Absolute values |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `GetResults` (METHOD) → `BOOL` — §5.1.1.2.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pHistArrayOut` | `PVOID` | Pointer to the histogram array. The data type depends on |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetResults` | `BOOL` | Returns TRUE if successful |

**Description:** Get result array without adding new values.

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.2.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eStateHistMode` | `E_ALY_StateHistMode` | Operation mode of the histogram. [} 430] Abs: Absolute values |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_ThresholdClassificator_1Ch` (FUNCTION_BLOCK) — §5.1.1.2.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Threshold Classifier 1Ch classifies the input values into three different classes: OK, Warning and Alarm according to the configured thresholds.

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_ThresholdClassificator_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.2.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fThresholdLevelOk` | `LREAL` | The input values that are smaller than the configured |
| `Warning` | `threshold are classified as OK. The input values greater` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.2.7.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_ThresholdStringClassificator_1Ch` (FUNCTION_BLOCK) — §5.1.1.2.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Threshold String Classifier 1Ch algorithm classifies the input values into three different classes according to the configured thresholds. The class names (output string) can be configured individually as String 1, String 2 and String 3.

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_ThresholdStringClassificator_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.2.8.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.2.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fThresholdLevel12` | `LREAL` | The input values that are smaller than the configured |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_TimeBasedEnvelope_1Ch` (FUNCTION_BLOCK) — §5.1.1.2.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Time Based Envelope 1Ch algorithm checks whether the periodic input data is within a configured range of values read from a file. This can be a reference signal that was previously learned with Time Based Teach Path 1Ch, for example. The comparison starts when the signal of the Start Period flag is TRUE. It is recommended not to use Time Based Envelope 1Ch simultaneously with Time Based Teach Path 1Ch due to concurrent file access. Instead, a reference signal should first be taught in with Time Based Teach Path 1Ch and only then should the evaluation be carried out with the aid of the Time Based Envelope 1Ch.

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_TimeBasedEnvelope_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStartPeriod` | `BOOL` | Rising edge starts reading period. The FB must be in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.2.9.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.2.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eBandMode` | `E_ALY_BandMode [} 428] Configuring the band mode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `SwitchState_Idle` (METHOD) → `BOOL` — §5.1.1.2.9.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Idle` | `BOOL` | Returns TRUE if successful |

**Description:** Initiate switch from Read state to Idle state. See State diagram.

##### `SwitchState_Read` (METHOD) → `BOOL` — §5.1.1.2.9.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Read` | `BOOL` | Returns TRUE if successful |

**Description:** Initiate switch from Idle state to Read state. See State diagram.

##### `UpdateState` (METHOD) → `BOOL` — §5.1.1.2.9.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdateState` | `BOOL` | Returns TRUE if successful |

**Description:** Update state after state change was initiated and until target state is not reached.

##### `GetBusyState` (METHOD) → `BOOL` — §5.1.1.2.9.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetBusyState` | `BOOL` | Returns TRUE if the FB is active. |

**Description:** Returns TRUE if the function block is busy due to an asynchronous file access.

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_StepResponse_1Ch` (FUNCTION_BLOCK) — §5.1.1.2.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Step Response 1Ch identifies parameters of the step response of a PT2 track. These include the delay time Te, the compensation time Tb, the settling time Tcs, and the time of maximum tm.

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_StepResponse_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.2.10.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Data In 2: Setpoint |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.2.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fThresholdReversal LREAL` | `Threshold for peak analysis` | — |
| `bUseRelativeTolera BOOL` | `Boolean flag. If this flag is True, the parameter Error` | — |
| `nce` | `Tolerance refers to the setpoint at the input as a` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

#### PLC API › Function blocks › Algorithms › Classification

##### `FB_ALY_ParetoAnalysis` (FUNCTION_BLOCK) — §5.1.1.2.11

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Pareto Analysis module allows you to combine different input data to display a Pareto chart. This is used in the following applications, among others:

#### PLC API › Function blocks › Algorithms › Classification › FB_ALY_ParetoAnalysis

##### `Call` (METHOD) → `BOOL` — §5.1.1.2.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.2.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eParetoMode` | `E_ALY_ParetoMode` | Mode for calculating the output values |
| `[} 433]` | `Count: The cycles in which the respective input is TRUE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.2.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `stThresholdEdge` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for monitoring input values` | — |
| `sChannelName` | `STRING(255)` | Alias of the input channel |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.2.11.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSingleInput` | `BOOL` | If TRUE, the number of input channels is limited to one. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initialize the number of input channels.

##### `GetOutputArray` (METHOD) → `BOOL` — §5.1.1.2.11.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nSelection` | `UDINT` | Selection of the array to be output. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetResults` | `BOOL` | Returns TRUE if successful |

**Description:** Get result array without adding new values.

##### `Reset` — §5.1.1.2.11.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.2.11.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.2.11.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArrayAbsIn` | `POINTER TO ULINT` | Pointer to an array with sorted absolute values to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

#### PLC API › Function blocks › Algorithms › Clustering

##### `FB_ALY_DenStream` (FUNCTION_BLOCK) — §5.1.1.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** DenStream is an implementation of the unmonitored, density-based clustering algorithm of the same name [1]. The latter is based on the well-known clustering algorithm DBSCAN [2, 3] and is particularly suitable for data streams whose structures change over time.

#### PLC API › Function blocks › Algorithms › Clustering › FB_ALY_DenStream

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |
| `nNumClusters` | `UDINT` | Defines the number of clusters. |
| `nAggBufferSize` | `UDINT` | Specifies the number of cycles after which the cluster |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initializes the number of input channels, the number of cluster centers and the size of the aggregation buffer.

##### `Call` (METHOD) → `BOOL` — §5.1.1.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nEps` | `LREAL` | Threshold for the maximum radius of micro-clusters. |
| `fMuBeta` | `LREAL` | Threshold for the weight of a micro-cluster between outlier |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.3.1.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Clustering

##### `FB_ALY_SequentialKMeans` (FUNCTION_BLOCK) — §5.1.1.3.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Sequential k-Means algorithm is an implementation of the unmonitored clustering algorithm of the same name. It is a sequential variant of the widely used k-Means clustering algorithm for streaming data. The aim of the algorithm is to find clusters based on the structure of the data, each of which contains similar data points and separates different data points from each other.

#### PLC API › Function blocks › Algorithms › Clustering › FB_ALY_SequentialKMeans

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |
| `nNumClusters` | `UDINT` | Defines the number of clusters. |
| `nAggBufferSize` | `UDINT` | Specifies the number of cycles after which the cluster |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initializes the number of input channels, the number of cluster centers and the size of the aggregation buffer.

##### `Call` (METHOD) → `BOOL` — §5.1.1.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |
| `bUpdateMCs` | `BOOL` | TRUE: The micro-clusters are updated by the incoming |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nMaxIterations` | `UDINT` | Specifies how often to iterate over the values in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.3.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInitialBounds` (METHOD) → `BOOL` — §5.1.1.3.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pLowerBoundsConfi POINTER TO LREAL` | `Pointer to an array containing the lower values of the` | — |
| `gArray` | `bounds to be initialized. Dim.: nNumChannels` | — |
| `nLowerBoundsConfi UDINT` | `Size of the array with the lower values of the limits to be` | — |
| `gArraySize` | `initialized.` | — |
| `pUpperBoundsConfi POINTER TO LREAL` | `Pointer to an array containing the upper values of the` | — |
| `gArray` | `bounds to be initialized. Dim.: nNumChannels` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitialBounds` | `BOOL` | Returns TRUE if successful. |

**Description:** Sets the initial boundaries. Depends on the configured initialization mode.

##### `SetInitialClusterCenters` (METHOD) → `BOOL` — §5.1.1.3.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pInitialClusterCenter POINTER TO LREAL` | `Pointer to a matrix with initial cluster centers. Dim.:` | — |
| `Matrix` | `nNumChannels*nNumClusters` | — |
| `nInitialClusterCenter UDINT` | `Size of the matrix with initial cluster centers. MatrixSize` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitialClusterCent BOOL` | `Returns TRUE if successful. ers` | — |

**Description:** Sets the initial cluster centers. Depends on the configured initialization mode.

##### `GetResults` (METHOD) → `BOOL` — §5.1.1.3.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pClusterCenterMatri POINTER TO LREAL` | `Pointer to cluster center matrix. Dim.:` | — |
| `xOut` | `nNumChannels*nNumClusters` | — |
| `nClusterCenterMatri UDINT` | `Size of the cluster center matrix xOut` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetResults` | `BOOL` | Returns TRUE if successful. |

**Description:** Retrieves the result matrix without adding new values.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_Demultiplexer` (FUNCTION_BLOCK) — §5.1.1.4.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The demultiplexer selects an output channel based on the input value. For this purpose, the input value is interpreted as an integer. This value corresponds to the output channel. If the value is outside the configured number of channels, the output channel is set to 0.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_Demultiplexer

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initializes the number of output channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |
| `pDataOut` | `POINTER TO BOOL` | Pointer to the result array of the output channels. Dim.: nNumChannels+1 |
| `nDataOutSize` | `UDINT` | Size of the result array. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.4.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `GetResults` (METHOD) → `BOOL` — §5.1.1.4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDataOut` | `POINTER TO BOOL` | Pointer to the result array of the output channels. Dim.: nNumChannels+1 |
| `nDataOutSize` | `UDINT` | Size of the result array. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetResults` | `BOOL` | Returns TRUE if successful. |

**Description:** Retrieves a result array without adding new values.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_DetectStringChange_1Ch` (FUNCTION_BLOCK) — §5.1.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Detect String Change 1Ch detects and counts changes of string values. Therefore, case sensitivity can be taken into account or not.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_DetectStringChange_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.4.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bCaseSensitive` | `BOOL` | If TRUE, upper and lower case is taken into account. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_DynamicTimeWarping` (FUNCTION_BLOCK) — §5.1.1.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Dynamic Time Warping algorithm compares input data with previously recorded templates. The special feature of the algorithm is that signals with different speeds or even shifted signals can be compared. As a result, the distance between the input signal and the respective template is output. The smaller the distance, the more equal the compared signals are. If the distance is 0, both signals are identical. The amount of distance depends on the equality but also on the length of the signals.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_DynamicTimeWarping

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Number of comparison channels (templates) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initializes the number of comparison channels (templates).

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStartPeriod` | `BOOL` | Rising edge starts the calculation. The FB must be in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.4.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeout` | `TIME` | Timeout for asynchronous operations. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.4.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `sFilePath` | `STRING(255)` | Path to the taught file, e.g. C:\TwinCAT\3.1\Boot\Teach.tas |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `Reset` — §5.1.1.4.3.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `GetBusyState` (METHOD) → `BOOL` — §5.1.1.4.3.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetBusyState` | `BOOL` | Returns TRUE if the FB is active. |

**Description:** Returns TRUE if the function block is busy due to an asynchronous file access.

##### `GetChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.4.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alue` | — |

**Description:** Fetching a channel-specific output value. The output value corresponds to the distance between the input signal and the respective template. The smaller the distance, the more equal the compared signals are. The value is updated only if the Call() method has been called before.

##### `GetOutputArray` (METHOD) → `BOOL` — §5.1.1.4.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArrayOut` | `POINTER TO LREAL` | Pointer to the output array: Dim: nBufferSize |
| `nArrayOutSize` | `UDINT` | Size of the output array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetOutputArray` | `BOOL` | Returns TRUE if successful. |

**Description:** Getting the entire output array. The array elements correspond to the distance between the input signal and the respective template. The smaller the distance, the more equal the compared signals are. The number of elements corresponds to the configured number of comparison channels (templates). The values are updated only if the Call() method has been called before.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SwitchState_Idle` (METHOD) → `BOOL` — §5.1.1.4.3.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Idle` | `BOOL` | Returns TRUE if successful |

**Description:** Initiate switch from Read state to Idle state. See State diagram.

##### `SwitchState_Read` (METHOD) → `BOOL` — §5.1.1.4.3.11

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwithcState_Read` | `BOOL` | Returns TRUE if successful |

**Description:** Initiate switch from Idle state to Read state. See State diagram.

##### `UpdateState` (METHOD) → `BOOL` — §5.1.1.4.3.12

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdateState` | `BOOL` | Returns TRUE if successful |

**Description:** Update state after state change was initiated and until target state is not reached.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_DynamicTimeWarpingInterval` (FUNCTION_BLOCK) — §5.1.1.4.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Dynamic Time Warping Interval algorithm compares several input data with each other. The special feature of the algorithm is that signals with different speeds or even shifted signals can be compared. Only the signal interval of a configured window is considered for the comparison. New results are output after the window expires. As a result, the distance between the reference signal and the respective input signal is output. The smaller the distance, the more equal the compared signals are. If the distance is 0, both signals are identical. The amount of distance depends on the equality but also on the length of the signals.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_DynamicTimeWarpingInterval

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.4.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.4.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.4.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nWindowSize` | `UDINT` | Specifies the number of cycles over which a calculation is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.4.4.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `GetChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.4.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alue` | — |

**Description:** Fetching a channel-specific output value. The output value corresponds to the distance between the reference channel and the input channel. The smaller the distance, the more equal the compared signals are. The value is updated only if the Call() method has been called before.

##### `GetOutputArray` (METHOD) → `BOOL` — §5.1.1.4.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArrayOut` | `POINTER TO LREAL` | Pointer to the output array: Dim: nBufferSize |
| `nArrayOutSize` | `UDINT` | Size of the output array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetOutputArray` | `BOOL` | Returns TRUE if successful. |

**Description:** Getting the entire output array. The array elements correspond to the distance between the reference channel and the input channels. The smaller the distance, the more equal the compared signals are. The number of elements corresponds to the configured number of input channels. The values are updated only if the Call() method has been called before.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 0: reference channel |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_LogicOperationCounter` (FUNCTION_BLOCK) — §5.1.1.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Logic Operation Counter executes a logical operation on the values of two or more channels and provides the result of this logical operation. Therefore, each input value can be combined with a threshold and an operator. Furthermore, the logic operator and the count mode can be configured individually.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_LogicOperationCounter

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.4.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eLogicOperator` | `E_ALY_LogicOperator [} 429]` | Configured logical operator. |
| `eCountMode` | `E_ALY_CountMode [} 429] Mode of the result counter.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.4.5.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.4.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `stThresholdLevel` | `ST_ALY_Threshold [} 426] Combination of comparison operator and threshold for monitoring input values` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.4.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initialize the number of input channels.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_Multiplexer` (FUNCTION_BLOCK) — §5.1.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Multiplexer selects one channel out of one or more input channels. For each input channel a boolean input has to be provided additionally. The output corresponds to the first input channel, where the conditional input is TRUE. The priority of the configured channels is the order of configuration. If the condition is not met for any of the channels, the provided default channel is returned.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_Multiplexer

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.4.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the methods SetChannelConditionAndValue () and SetChannelDefaultValue().

##### `Reset` — §5.1.1.4.6.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValues` (METHOD) → `BOOL` — §5.1.1.4.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `bCondition` | `BOOL` | Channel condition. |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValues` | `BOOL` | Returns TRUE if successful |

**Description:** Set channel specific input value and condition. The input value is not processed until the Call() method has been invoked.

##### `SetDefaultChannelValue` (METHOD) → `BOOL` — §5.1.1.4.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetDefaultChannelV BOOL` | `Returns TRUE if successful alue` | — |

**Description:** Set default value which is set if no channel condition is TRUE. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_NumericalCompare_1Ch` (FUNCTION_BLOCK) — §5.1.1.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Numerical Compare 1Ch compares the input values with a reference value and provides the result of this comparison operation. The operator, the reference value and the count mode can be configured individually.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_NumericalCompare_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.4.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fReference` | `LREAL` | Reference value for the comparison operation. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.4.7.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_NumericalCompare_2Ch` (FUNCTION_BLOCK) — §5.1.1.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Numerical Compare 2Ch compares the input values of the first channel with the input values of the second channel and provides the result of this comparison operation. The operator and the count mode can be configured individually.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_NumericalCompare_2Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.4.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eComparisonOperator` | `E_ALY_ComparisonOperator` | — |
| `eCountMode` | `E_ALY_CountMode` | — |
| `bUseAbsValues` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.4.8.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Channel 1 2: Channel 2 |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_StringCompare_1Ch` (FUNCTION_BLOCK) — §5.1.1.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The String Compare 1Ch compares the input string with a reference string and counts the string matches. Therefore, case sensitivity can be taken into account or not and the count mode can be changed.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_StringCompare_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.4.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sReference` | `STRING(255)` | Reference string for the comparison. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.4.9.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY_STRING` | Input value of a string data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_StringCompare_2Ch` (FUNCTION_BLOCK) — §5.1.1.4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The String Compare 2Ch compares the values of the first input string with the values of the second string and counts the string matches. Therefore case sensitivity can be taken into account or not and the count mode can be changed.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_StringCompare_2Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.4.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eStringCompareMode` | `E_ALY_StringCompareMode` | — |
| `eCountMode` | `E_ALY_CountMode` | — |
| `bCaseSensitive` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.4.10.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Channel 1 2: Channel 2 |
| `Input` | `ANY_STRING` | Input value of a string data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_DetectValueChange_1Ch` (FUNCTION_BLOCK) — §5.1.1.4.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Detect Value Change 1Ch detects and counts changes in numeric input values.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_DetectValueChange_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.4.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fTolerance` | `LREAL` | The tolerance refers to the input value of the last detected |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.4.11.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.11.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Compare

##### `FB_ALY_TimespanCompare_1Ch` (FUNCTION_BLOCK) — §5.1.1.4.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Timespan Compare 1Ch compares the input value with a reference value and returns the result of this comparison operation. A timespan with a resolution of 1 ns is supported as an input. The operator, the reference value and the count mode can be configured individually.

#### PLC API › Function blocks › Algorithms › Compare › FB_ALY_TimespanCompare_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.4.12.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.4.12.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tReference` | `FB_ALY_Timespan [} 399] Reference value for the comparison operation. Resolution` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.4.12.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.4.12.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Energy

##### `FB_ALY_Energy` (FUNCTION_BLOCK) — §5.1.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Energy module is suitable for energy monitoring of various electrical components. The input data for current and voltage are averaged over a configurable interval and then calculated.

#### PLC API › Function blocks › Algorithms › Energy › FB_ALY_Energy

##### `Call` (METHOD) → `BOOL` — §5.1.1.5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed in Hz. |
| `tInterval` | `LTIME` | Interval time of averaging. Used to calculate the electrical |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.5.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Voltage 2: Current |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Energy

##### `FB_ALY_Energy_AX5xxx` (FUNCTION_BLOCK) — §5.1.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Energy AX5xxx module is suitable for energy monitoring of servo drives from the AX5xxx series. The input data is averaged over a configurable interval and then calculated. To ensure that the required input data is available, it must be added to the process image.

#### PLC API › Function blocks › Algorithms › Energy › FB_ALY_Energy_AX5xxx

##### `Call` (METHOD) → `BOOL` — §5.1.1.5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.5.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed in Hz. |
| `tInterval` | `LTIME` | Interval time of averaging. Used to calculate the electrical |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.5.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.5.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: DC bus voltage |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Energy

##### `FB_ALY_Energy_AX81xx` (FUNCTION_BLOCK) — §5.1.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Energy AX81xx module is suitable for energy monitoring of servo drives from the AX81xx series. The input data is averaged over a configurable interval and then calculated. To ensure that the required input data is available, it must be added to the process image.

#### PLC API › Function blocks › Algorithms › Energy › FB_ALY_Energy_AX81xx

##### `Call` (METHOD) → `BOOL` — §5.1.1.5.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.5.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed in Hz. |
| `tInterval` | `LTIME` | Interval time of averaging. Used to calculate the electrical |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.5.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.5.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Electrical power (Enable PDO-Input 0x1A32 -> Add CoE 0x3242:12 "Electrical power" to PDO content) 2: Mechanical power (Enable PDO-Input 0x1A32 -> Add CoE 0x3242:13 "Mechanical power" to PDO content) 3: DC link voltage |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Energy

##### `FB_ALY_Energy_AX82xx` (FUNCTION_BLOCK) — §5.1.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Energy AX82xx module is suitable for energy monitoring of servo drives from the AX82xx series. The input data is averaged over a configurable interval and then calculated. To ensure that the required input data is available, it must be added to the process image.

#### PLC API › Function blocks › Algorithms › Energy › FB_ALY_Energy_AX82xx

##### `Call` (METHOD) → `BOOL` — §5.1.1.5.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.5.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed in Hz. |
| `tInterval` | `LTIME` | Interval time of averaging. Used to calculate the electrical |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.5.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.5.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Energy

##### `FB_ALY_Energy_AX86xx` (FUNCTION_BLOCK) — §5.1.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Energy AX86xx module is suitable for energy monitoring of AX86xx series power supply modules. The input data is averaged over a configurable interval and then calculated. To ensure that the required input data is available, it must be added to the process image.

#### PLC API › Function blocks › Algorithms › Energy › FB_ALY_Energy_AX86xx

##### `Call` (METHOD) → `BOOL` — §5.1.1.5.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.5.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed in Hz. |
| `tInterval` | `LTIME` | Interval time of averaging. Used to calculate the electrical |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.5.5.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.5.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: DC link voltage (Enable PDO-Input 0x1A05) 2: DC link current (Enable PDO-Input 0x1A06) 3: Brake resistor continuous power (Enable PDO-Input 0x1A07) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Energy

##### `FB_ALY_Energy_EL259x` (FUNCTION_BLOCK) — §5.1.1.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Energy EL259x module is suitable for energy monitoring of EL259x EtherCAT Terminals. The input data is averaged over a configurable interval and then calculated. To ensure that the required input data is available, it must be added to the process image.

#### PLC API › Function blocks › Algorithms › Energy › FB_ALY_Energy_EL259x

##### `Call` (METHOD) → `BOOL` — §5.1.1.5.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.5.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed in Hz. |
| `tInterval` | `LTIME` | Interval time of averaging. Used to calculate the electrical |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.5.6.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.5.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Supply voltage |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Energy

##### `FB_ALY_Energy_EL72xx_EL74xx` (FUNCTION_BLOCK) — §5.1.1.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Energy AX72xx EL74xx module is suitable for energy monitoring of EL72xx and EL74xx EtherCAT Terminals. The input data is averaged over a configurable interval and then calculated. To ensure that the required input data is available, it must be added to the process image.

#### PLC API › Function blocks › Algorithms › Energy › FB_ALY_Energy_EL72xx_EL74xx

##### `Call` (METHOD) → `BOOL` — §5.1.1.5.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.5.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed in Hz. |
| `tInterval` | `LTIME` | Interval time of averaging. Used to calculate the electrical |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.5.7.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.5.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Actual velocity (Enable PDO-Input 0x1A02) 2: Actual torque (Enable PDO-Input 0x1A03) 3: DC link voltage |
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Energy

##### `Call` (METHOD) → `BOOL` — §5.1.1.5.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.5.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed in Hz. |
| `tInterval` | `LTIME` | Interval time of averaging. Used to calculate the electrical |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.5.8.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.5.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Device voltage Us (PDO Inputs Device) 2: Device current Us (PDO Inputs Device) 3: Device voltage Up (PDO Inputs Device) 4: Device current Up (PDO Inputs Device) |
| `Input` | `ANY` | Input value of any data type. |
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Call` (METHOD) → `BOOL` — §5.1.1.5.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.5.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed in Hz. |
| `tInterval` | `LTIME` | Interval time of averaging. Used to calculate the electrical |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.5.9.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.5.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Voltage Us (EPM Inputs Channel 1) 2: Current Is (EPM Inputs Channel 1) 3: Voltage Up (EPM Inputs Channel 2) 4: Current Ip (EPM Inputs Channel 2) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Math

##### `FB_ALY_Integrator_1Ch` (FUNCTION_BLOCK) — §5.1.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Integrator 1Ch integrates the input value over time with a base unit of one second and provides the result of this integration operation. For the approximation of this integral the trapezoidal rule is used. The

#### PLC API › Function blocks › Algorithms › Math › FB_ALY_Integrator_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.6.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.6.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eIntegrationMode` | `E_ALY_IntegrationMode` | Integration mode |
| `[} 429]` | `Direct: Integration of the input values` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.6.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.6.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Math

##### `FB_ALY_MathOperation` (FUNCTION_BLOCK) — §5.1.1.6.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Math Operation executes a mathematical operation on two or more different input channels and provides the result of the mathematical operation. The operator is the same for all operands.

#### PLC API › Function blocks › Algorithms › Math › FB_ALY_MathOperation

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.6.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.6.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.6.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMathOperator` | `E_ALY_MathOperator` | Mathematical operator [} 429] Addition |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm.

##### `Reset` — §5.1.1.6.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.6.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Math

##### `FB_ALY_MathOperation_1Ch` (FUNCTION_BLOCK) — §5.1.1.6.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Math Operation 1Ch executes a mathematical operation on the signal of the input channel and a reference value. The algorithm provides the result of the mathematical operation and the operator can be configured individually.

#### PLC API › Function blocks › Algorithms › Math › FB_ALY_MathOperation_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.6.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.6.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fOperand` | `LREAL` | Operand for the mathematical operation. |
| `eMathOperator` | `E_ALY_MathOperator` | Mathematical operator [} 429] Addition Subtraction Multiplication Division PowerOf Modulo Minimum Maximum |
| `bUseAbsValues` | `BOOL` | If TRUE, the absolute values of the two operands are |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.6.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.6.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Math

##### `FB_ALY_Mean_1Ch` (FUNCTION_BLOCK) — §5.1.1.6.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Mean 1Ch calculates the mean value over the input values according to the formula

#### PLC API › Function blocks › Algorithms › Math › FB_ALY_Mean_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.6.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |
| `pMeanArrayOut` | `POINTER TO LREAL` | Pointer to an array in which the mean value results are to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.6.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumCascades` | `UDINT` | Number of output cascades. |
| `fSampleRate` | `LREAL` | Sample rate of the system to be analyzed in Hz. |
| `eStartupBehaviour` | `E_ALY_CascadeStartupBeh Start-up behavior aviour [} 428]` | WaitUntilFilled |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm. The output cascades can be set using the SetCascades() method.

##### `Reset` — §5.1.1.6.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetCascades` (METHOD) → `BOOL` — §5.1.1.6.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pCascadesConfigArr POINTER TO LTIME` | `Pointer to an array used to configure the time intervals of` | — |
| `ay` | `the cascades. The time interval of the configured cascade` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetCascades` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuration of the output cascades. Before that, the number of cascades must be set with the Configure() method.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.6.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Math

##### `FB_ALY_RMS_1Ch` (FUNCTION_BLOCK) — §5.1.1.6.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** RMS 1Ch calculates the root mean square over the input values according to the formula

#### PLC API › Function blocks › Algorithms › Math › FB_ALY_RMS_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.6.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |
| `pRmsArrayOut` | `POINTER TO LREAL` | Pointer to an array in which the RMS results are to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.6.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumCascades` | `UDINT` | Number of output cascades |
| `fSampleRate` | `LREAL` | Sampling rate of the system to be analyzed in Hz |
| `eStartupBehaviour` | `E_ALY_CascadeStartupBeh Start-up behavior aviour [} 428]` | WaitUntilFilled |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm. The output cascades can be set using the SetCascades() method.

##### `Reset` — §5.1.1.6.5.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetCascades` (METHOD) → `BOOL` — §5.1.1.6.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pCascadesConfigArr POINTER TO LTIME` | `Pointer to an array used to configure the time intervals of` | — |
| `ay` | `the cascades. The time interval of the configured cascade` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetCascades` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuration of the output cascades. Before that, the number of cascades must be set with the Configure() method.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.6.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Math

##### `FB_ALY_SlopeAnalysis_1Ch` (FUNCTION_BLOCK) — §5.1.1.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Slope Analysis 1Ch calculates the slope between two values of the input stream. One of those two values is the current input value and the second value is the input value that occurred a defined number (configured by the parameter Num Values) of cycles before in the input stream. The difference between these two values is returned as Delta Value.

#### PLC API › Function blocks › Algorithms › Math › FB_ALY_SlopeAnalysis_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.6.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.6.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumValues` | `UDINT` | Distance between the two data sets for the calculation of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.6.6.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.6.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Math

##### `FB_ALY_TimespanOperation` (FUNCTION_BLOCK) — §5.1.1.6.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Timespan Operation performs a mathematical operation on two or more different input channels and returns the result of the mathematical operation. Timespans with a resolution of 1 ns are supported as inputs.

#### PLC API › Function blocks › Algorithms › Math › FB_ALY_TimespanOperation

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.6.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.6.7.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.6.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eTimespanOperator` | `E_ALY_TimespanOperator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm.

##### `Reset` — §5.1.1.6.7.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.6.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Preprocessing

##### `FB_ALY_ScalerFitting` (FUNCTION_BLOCK) — §5.1.1.7.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Scaler Fitting module can be used to determine the parameters for feature scaling and save them in a file. These parameters can then be used in the Scaler module.

#### PLC API › Function blocks › Algorithms › Preprocessing › FB_ALY_ScalerFitting

##### `Call` (METHOD) → `BOOL` — §5.1.1.7.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bFit` | `BOOL` | Parameter adjustment can be enabled or disabled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.7.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eScalingMode` | `E_ALY_ScalingMode` | Scaling mode |
| `[} 432]` | `Normalization Standardization` | — |
| `tTimeout` | `TIME` | Timeout for asynchronous operations. |
| `sFilePath` | `STRING(255)` | Path to the taught file, e.g. C: \TwinCAT\3.1\Boot\Scaling.tas |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `GetBusyState` (METHOD) → `BOOL` — §5.1.1.7.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetBusyState` | `BOOL` | Returns TRUE if the FB is active. |

**Description:** Returns TRUE if the function block is busy due to an asynchronous file access.

##### `Reset` — §5.1.1.7.1.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.7.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SwitchState_Idle` (METHOD) → `BOOL` — §5.1.1.7.1.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Idle` | `BOOL` | Returns TRUE if successful |

**Description:** Initiate switch from Teach state to Idle state. See State diagram.

##### `SwitchState_Teach` (METHOD) → `BOOL` — §5.1.1.7.1.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Teach BOOL` | `Returns TRUE if successful` | — |

**Description:** Initiate switch from Idle state to Teach state. See State diagram.

##### `UpdateState` (METHOD) → `BOOL` — §5.1.1.7.1.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdateState` | `BOOL` | Returns TRUE if successful |

**Description:** Update state after state change was initiated and until target state is not reached.

#### PLC API › Function blocks › Algorithms › Preprocessing

##### `FB_ALY_Scaler` (FUNCTION_BLOCK) — §5.1.1.7.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Scaler module can be used to perform feature scaling.

#### PLC API › Function blocks › Algorithms › Preprocessing › FB_ALY_Scaler

##### `Call` (METHOD) → `BOOL` — §5.1.1.7.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.7.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eScalingMode` | `E_ALY_ScalingMode` | Scaling mode |
| `[} 432]` | `Normalization: Result has a value range of 0..+1.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureFromFile` (METHOD) → `BOOL` — §5.1.1.7.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeout` | `TIME` | Timeout for asynchronous operations. |
| `sFilePath` | `STRING(255)` | Path to the taught file, e.g. C: \TwinCAT\3.1\Boot\Teach.json |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureFromFile` | `BOOL` | Returns TRUE if successful. |

**Description:** Configures the algorithm based on a previously created file. After calling this method, the method UpdateState() must be called until the status of the function block is no longer Pending.

##### `GetBusyState` (METHOD) → `BOOL` — §5.1.1.7.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetBusyState` | `BOOL` | Returns TRUE if the FB is active. |

**Description:** Returns TRUE if the function block is busy due to an asynchronous file access.

##### `Reset` — §5.1.1.7.2.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.7.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `UpdateState` (METHOD) → `BOOL` — §5.1.1.7.2.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdateState` | `BOOL` | Returns TRUE if successful. |

**Description:** Updates the state after a state change has been initiated by calling the ConfigureFromFile() method. The method must be called cyclically until the state of the function block is no longer Pending.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Collector

##### `Reporting Collector Edge` (FUNCTION_BLOCK) — §5.1.1.8.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Reporting Collector Edge collects the data from the input channels and sends the data to the Reporting Server after an event or after the buffer is filled, depending on the configuration. An event is triggered when the signal of the input channel passes the configured edge at a certain threshold.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Collector › Reporting Collector Edge

##### `Call` (METHOD) → `BOOL` — §5.1.1.8.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.8.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sConfigId` | `STRING` | Indicates the name of the report. The name must |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.8.1.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.8.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetDisplayName` (METHOD) → `BOOL` — §5.1.1.8.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `sDisplayName` | `STRING(255)` | Input value of type string. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetDisplayName` | `BOOL` | Returns TRUE if successful. |

**Description:** Setting a channel-specific description. In reporting, the description serves as a table heading for a table and as a key value for a key-value pair.

##### `SetEdgeValue` (METHOD) → `BOOL` — §5.1.1.8.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Collector

##### `Reporting Collector Interval` (FUNCTION_BLOCK) — §5.1.1.8.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Reporting Collector Interval collects the data from the input channels and sends the data to the Reporting Server after an event or after the buffer is filled, depending on the configuration. An event is triggered when the timespan of the configured interval has expired.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Collector › Reporting Collector Interval

##### `Call` (METHOD) → `BOOL` — §5.1.1.8.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.8.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sConfigId` | `STRING` | Indicates the name of the report. The name must |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.8.1.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.8.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetDisplayName` (METHOD) → `BOOL` — §5.1.1.8.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `sDisplayName` | `STRING(255)` | Input value of type string. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetDisplayName` | `BOOL` | Returns TRUE if successful. |

**Description:** Setting a channel-specific description. In reporting, the description serves as a table heading for a table and as a key value for a key-value pair.

##### `Pause` (METHOD) → `BOOL` — §5.1.1.8.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Pause` | `BOOL` | Returns TRUE if successful. |

**Description:** With the Pause method it is possible to pause the execution of the function block. The internally running time interval does not continue after calling the pause function. A single call of the method is sufficient. The interval is not continued until the next call of the Call method.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Collector

##### `Reporting Collector Time` (FUNCTION_BLOCK) — §5.1.1.8.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Reporting Collector Time collects the data from the input channels and sends the data to the Reporting Server after an event or after the buffer is filled, depending on the configuration. An event is triggered when the configured switch-on time is reached. The switch-on time and the days of the weeks can be configured.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Collector › Reporting Collector Time

##### `Call` (METHOD) → `BOOL` — §5.1.1.8.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.8.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sConfigId` | `STRING` | Indicates the name of the report. The name must |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.8.1.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.8.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetDisplayName` (METHOD) → `BOOL` — §5.1.1.8.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `sDisplayName` | `STRING(255)` | Input value of type string. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetDisplayName` | `BOOL` | Returns TRUE if successful. |

**Description:** Setting a channel-specific description. In reporting, the description serves as a table heading for a table and as a key value for a key-value pair.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Trigger

##### `Reporting Trigger Edge` (FUNCTION_BLOCK) — §5.1.1.8.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Reporting Trigger Edge triggers the creation of a report after an event is triggered. An event is triggered when the input channel signal exceeds the configured edge at a specified threshold. Internally, the inputs that were once True remain True. The inputs are only reset to False as soon as all inputs were True at least once. This allows the output bNewResult to be used as one input by multiple Reporting Collectors and once all Reporting Collectors have sent a data message, a trigger message is sent.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Trigger › Reporting Trigger Edge

##### `Call` (METHOD) → `BOOL` — §5.1.1.8.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.8.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sConfigId` | `STRING` | Indicates the name of the report. The name must |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.8.2.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.8.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Trigger

##### `Reporting Trigger Interval` (FUNCTION_BLOCK) — §5.1.1.8.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Reporting Trigger Interval triggers the creation of a report after an event has been triggered. An event is triggered when the timespan of the configured interval has expired.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Trigger › Reporting Trigger Interval

##### `Call` (METHOD) → `BOOL` — §5.1.1.8.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.8.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sConfigId` | `STRING` | Indicates the name of the report. The name must |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.8.2.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `Pause` (METHOD) → `BOOL` — §5.1.1.8.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Pause` | `BOOL` | Returns TRUE if successful. |

**Description:** With the Pause method it is possible to pause the execution of the function block. The internally running time interval does not continue after calling the pause function. A single call of the method is sufficient. The interval is not continued until the next call of the Call method.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Trigger

##### `Reporting Trigger Time` (FUNCTION_BLOCK) — §5.1.1.8.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Reporting Trigger Time triggers the creation of a report after an event has been triggered. An event is triggered when the configured switch-on time is reached. The switch-on time and the days of the weeks can be configured.

#### PLC API › Function blocks › Algorithms › Reporting › Reporting Trigger › Reporting Trigger Time

##### `Call` (METHOD) → `BOOL` — §5.1.1.8.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.8.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sConfigId` | `STRING` | Indicates the name of the report. The name must |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.8.2.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Specific › XTS

##### `FB_ALY_XtsAccelerationAnalysis_1Ch` (FUNCTION_BLOCK) — §5.1.1.9.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The XTS Acceleration Analysis 1Ch calculates the current acceleration of a XTS mover. For this purpose, the length of the XTS in millimeters must be declared and as input signal the mover position is required.

#### PLC API › Function blocks › Algorithms › Specific › XTS › FB_ALY_XtsAccelerationAnalysis_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.9.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.9.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nXtsLength` | `UDINT` | Length of the given XTS system in millimeters. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.9.1.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.9.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Specific › XTS

##### `FB_ALY_XtsDistanceIntegrator_1Ch` (FUNCTION_BLOCK) — §5.1.1.9.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The XTS Distance Integrator 1Ch calculates the distance covered by a XTS mover. The algorithm provides the total distance, the positive distance and the negative distance. For this purpose, the length of the XTS in millimeters must be declared and as input signal the mover position is required.

#### PLC API › Function blocks › Algorithms › Specific › XTS › FB_ALY_XtsDistanceIntegrator_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.9.1.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.9.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nXtsLength` | `UDINT` | Length of the given XTS system in millimeters. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.9.1.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.9.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Specific › XTS

##### `FB_ALY_XtsVelocityAnalysis_1Ch` (FUNCTION_BLOCK) — §5.1.1.9.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The XTS Velocity Analysis 1Ch calculates the current velocity of a XTS mover. For this purpose, the length of the XTS in millimeters must be declared and as input signal the mover position is required.

#### PLC API › Function blocks › Algorithms › Specific › XTS › FB_ALY_XtsVelocityAnalysis_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.9.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.9.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nXtsLength` | `UDINT` | Length of the given XTS system in millimeters. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.9.1.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.9.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Specific › XTS

##### `FB_ALY_XtsMovementAnalysis` (FUNCTION_BLOCK) — §5.1.1.9.1.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The XTS Movement Analysis calculates the position, speed, acceleration and jerk of an XTS mover. Various filter stages are chained together to smooth the signal. In order to be able to analyze additional, associated signals, the module enables the filtering of two further signals with the same filter stages.

#### PLC API › Function blocks › Algorithms › Specific › XTS › FB_ALY_XtsMovementAnalysis

##### `Call` (METHOD) → `BOOL` — §5.1.1.9.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.9.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nXtsLength` | `UDINT` | Length of the given XTS system in millimeters. |
| `fHardwarePosFactor LREAL` | `Factor for scaling the input signal in mm.` | — |
| `fSampleRate` | `LREAL` | Sampling rate (samples per second) of the input signal. |
| `aFilterOrder` | `ARRAY[1…4] OF UDINT` | Filter order of the PTn filter for the calculation. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.9.1.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.9.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Position of the mover (modulo) 2: External Signal 1 3: External Signal 2 |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Specific › Wind Turbine

##### `FB_ALY_WtTurbulence_1Ch` (FUNCTION_BLOCK) — §5.1.1.9.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The WT Turbulence 1Ch calculates the mean of the wind velocity, the turbulence, and the turbulence intensity according to the standard EN 61400-1. As input signal, the wind velocity is required. The output values are updated in a cycle of 10 minutes.

#### PLC API › Function blocks › Algorithms › Specific › Wind Turbine › FB_ALY_WtTurbulence_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.9.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.9.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumCycles` | `UDINT` | Indicates the number of PLC cycles that fit into the time |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.9.2.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.9.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Specific › B/SSD

##### `FB_ALY_AM8xxxSSD` (FUNCTION_BLOCK) — §5.1.1.9.3.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMess I_TcMessage` | `Contains more detailed information on` | — |
| `age` | `the current return value. For this special interface pointer, it is ensured internally` | — |

**Description:** PLC API | Function blocks

#### PLC API › Function blocks › Algorithms › Specific › B/SSD › FB_ALY_AM8xxxSSD

##### `Call` (METHOD) → `BOOL` — §5.1.1.9.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

##### `Configure` (METHOD) → `BOOL` — §5.1.1.9.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumSection UDINT` | `Configure the number of sections for` | — |
| `sHumidity` | `analyzing the humidity in the encoder.` | — |
| `nNumSection UDINT` | `Configure the number of sections for` | — |
| `sTemperature` | `analyzing the temperature in the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.9.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index: 1: Update Counter 2: Data Sensor 0 3: Data Sensor 1 4: Data Sensor 2 5: Data Sensor 3 |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelVa BOOL` | `Returns TRUE if successful. lue` | — |

**Description:** PLC API | Function blocks

##### `GetChannelOutputArray` (METHOD) → `BOOL` — §5.1.1.9.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to 4) 1: Temporal distribution of humidity within the encoder. (Section Timer) 2: Temporal distribution of the temperature within the encoder. (Section Timer) 3: Distribution of acceleration in the X direction. (Histogram) 4: Distribution of acceleration in the Y direction. (Histogram) |
| `pArrayOut` | `PVOID` | Pointer to an array into which the output |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelO BOOL` | `Returns TRUE if successful. utputArray` | — |

##### `Reset` — §5.1.1.9.3.1.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

#### PLC API › Function blocks › Algorithms › Specific › B/SSD

##### `FB_ALY_AM8xxxSSD_ValueExtractor` (FUNCTION_BLOCK) — §5.1.1.9.3.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMess I_TcMessage` | `Contains more detailed information on th` | — |
| `age` | `e current return value. For this special interface pointer, it is ens ured internally that it is always valid/` | — |

**Description:** PLC API | Function blocks

#### PLC API › Function blocks › Algorithms › Specific › B/SSD › FB_ALY_AM8xxxSSD_ValueExtractor

##### `Call` (METHOD) → `BOOL` — §5.1.1.9.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.9.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index: 1: Update Counter 2: Data Sensor 0 3: Data Sensor 1 4: Data Sensor 2 5: Data Sensor 3 |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelVa BOOL` | `Returns TRUE if successful. lue` | — |

##### `Reset` — §5.1.1.9.3.2.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

#### PLC API › Function blocks › Algorithms › Statistics

##### `FB_ALY_ArrayStatistics` (FUNCTION_BLOCK) — §5.1.1.10.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMess I_TcMessage` | `Contains more detailed information on` | — |
| `age` | `the current return value. For this special interface pointer, it is ensured internally` | — |

**Description:** PLC API | Function blocks

#### PLC API › Function blocks › Algorithms › Statistics › FB_ALY_ArrayStatistics

##### `Call` (METHOD) → `BOOL` — §5.1.1.10.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArrayIn` | `PVOID` | Pointer to the input array. |
| `nArrayInSize` | `UDINT` | Size of the input array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

##### `Configure` (METHOD) → `BOOL` — §5.1.1.10.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bUseBesselCorrecti` | `BOOL` | Use of the Bessel correction (see above). on |
| `fThresholdReversal` | `LREAL` | Threshold for identifying reversals (see above). |
| `fThresholdDelta` | `LREAL` | Threshold for identifying Delta maxima (see above). |
| `nLowerBound` | `DINT` | Lower limit of the array. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** PLC API | Function blocks

##### `Reset` — §5.1.1.10.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Statistics

##### `FB_ALY_CorrelationFunction` (FUNCTION_BLOCK) — §5.1.1.10.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Correlation Function function block calculates the discrete correlation function between a reference signal (Channel Ref) and one or more other signals (Channel 00, ..., Channel 0n). The correlation coefficients are calculated for time shifts of m cycles between the two signals, with the maximum and minimum values for m being limited by the parameters Minimum Lag (negative integer) and Maximum Lag (positive integer).

#### PLC API › Function blocks › Algorithms › Statistics › FB_ALY_CorrelationFunction

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.10.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used |

**Description:** Initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.10.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.10.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nMinLag` | `UDINT` | Specifies the minimum number of cycles by which the two |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.10.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `GetChannelOutputArray` (METHOD) → `BOOL` — §5.1.1.10.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `pCorrCoefsArrayOut PVOID` | `Pointer to an array to which the coefficients are to be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputA BOOL` | `Returns TRUE if successful. rray` | — |

**Description:** Retrieves a channel-specific output array. The output values were updated in the previously called Call() method.

##### `GetChannelOutputValues` (METHOD) → `BOOL` — §5.1.1.10.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `fMinCoef` | `LREAL` | Writing to variable takes place with the minimum coefficient |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fMinCoef` | `LREAL` | — |
| `fMaxCoef` | `LREAL` | — |
| `nMinimizingLag` | `DINT` | — |
| `nMaximizingLag` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alues` | — |

**Description:** Retrieves channel-specific output values. The output values were updated in the previously called Call() method.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.10.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (0 to nNumChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Statistics

##### `FB_ALY_CorrelationFunctionReference` (FUNCTION_BLOCK) — §5.1.1.10.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Correlation Function Reference function block calculates the discrete correlation function between a recorded signal (referred to below as reference signal), which is read from a tcab file, and one or more input signals (Channel 00, ..., Channel 0n).

#### PLC API › Function blocks › Algorithms › Statistics › FB_ALY_CorrelationFunctionReference

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.10.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (0 to nNumChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.10.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used. |

**Description:** Initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.10.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.10.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nMinLag` | `UDINT` | Specifies the minimum number by which the two signals |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.10.3.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `GetBusyState` (METHOD) → `BOOL` — §5.1.1.10.3.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetBusyState` | `BOOL` | Returns TRUE if the FB is active. |

**Description:** Returns TRUE if the function block is busy due to an asynchronous file access.

##### `GetChannelOutputArray` (METHOD) → `BOOL` — §5.1.1.10.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `pCorrCoefsArrayOut PVOID` | `Pointer to an array to which the coefficients are to be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputA BOOL` | `Returns TRUE if successful. rray` | — |

**Description:** Retrieves a channel-specific output array. The output values were updated in the previously called Call() method.

##### `GetChannelOutputValues` (METHOD) → `BOOL` — §5.1.1.10.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `fMinCoef` | `LREAL` | Writing to variable takes place with the minimum coefficient |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fMinCoef` | `LREAL` | — |
| `fMaxCoef` | `LREAL` | — |
| `nMinimizingLag` | `DINT` | — |
| `nMaximizingLag` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alues` | — |

**Description:** Retrieves channel-specific output values. The output values were updated in the previously called Call() method.

##### `SwitchState_Idle` (METHOD) → `BOOL` — §5.1.1.10.3.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Idle` | `BOOL` | Returns TRUE if successful |

**Description:** Initiate switch from Read state to Idle state. See State diagram.

##### `SwitchState_Read` (METHOD) → `BOOL` — §5.1.1.10.3.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Read` | `BOOL` | Returns TRUE if successful |

**Description:** Initiate switch from Idle state to Read state. See State diagram.

##### `UpdateState` (METHOD) → `BOOL` — §5.1.1.10.3.11

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdateState` | `BOOL` | Returns TRUE if successful. |

**Description:** Update state after state change was initiated and until target state is not reached.

#### PLC API › Function blocks › Algorithms › Statistics

##### `FB_ALY_LinearRegressionFitting` (FUNCTION_BLOCK) — §5.1.1.10.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Linear Regression Fitting function block approximates one variable (the Dependent input) by linear combination of several other variables (Input 01 ... Input 0n). This is done by the incremental stochastic gradient method. At the end of the analysis, the calculated coefficients are written to a file.

#### PLC API › Function blocks › Algorithms › Statistics › FB_ALY_LinearRegressionFitting

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.10.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used |

**Description:** Initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.10.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bFit` | `BOOL` | Parameter adjustment can be enabled or disabled. |
| `dependent` | `ANY` | Dependent value resulting from the linear combination of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.10.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bBias` | `BOOL` | If FALSE, the Bias Output 00 is set to zero and is not |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm.

##### `Reset` — §5.1.1.10.4.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.10.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `GetChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.10.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (0 (bias) to nNumChannels) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alue` | — |

**Description:** Fetching a channel-specific output value. The output value is only updated if the Call() method was called previously.

##### `SwitchState_Idle` (METHOD) → `BOOL` — §5.1.1.10.4.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Idle` | `BOOL` | Returns TRUE if successful |

**Description:** Initiate switch from Teach state to Idle state. See State diagram.

##### `SwitchState_Teach` (METHOD) → `BOOL` — §5.1.1.10.4.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Teach BOOL` | `Returns TRUE if successful` | — |

**Description:** Initiate switch from Idle state to Teach state. See State diagram.

##### `GetBusyState` (METHOD) → `BOOL` — §5.1.1.10.4.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetBusyState` | `BOOL` | Returns TRUE if the FB is active. |

**Description:** Returns TRUE if the function block is busy due to an asynchronous file access.

##### `UpdateState` (METHOD) → `BOOL` — §5.1.1.10.4.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdateState` | `BOOL` | Returns TRUE if successful |

**Description:** Update state after state change was initiated and until target state is not reached.

#### PLC API › Function blocks › Algorithms › Statistics

##### `FB_ALY_LinearRegressionInference` (FUNCTION_BLOCK) — §5.1.1.10.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Linear Regression Inference function block calculates the linear combination of the inputs (Input 01 ..

#### PLC API › Function blocks › Algorithms › Statistics › FB_ALY_LinearRegressionInference

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.10.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used |

**Description:** Initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.10.5.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.10.5.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuring the algorithm. The coefficients must be set beforehand with the ConfigureChannel() method.

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.10.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index for the coefficients. Starting at zero. |
| `fCoefficient` | `LREAL` | Coefficient for the calculation of the linear combination. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configuration of the coefficients for the calculation of the linear combination. The coefficients are only applied after the Configure() method has been called.

##### `ConfigureFromFile` (METHOD) → `BOOL` — §5.1.1.10.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeout` | `TIME` | Timeout for asynchronous operations. |
| `sFilePath` | `STRING(255)` | Path to the taught file, e.g. C: \TwinCAT\3.1\Boot\Teach.json |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureFromFile` | `BOOL` | Returns TRUE if successful. |

**Description:** Configures the algorithm based on a previously created file. After calling this method, the method UpdateState() must be called until the status of the function block is no longer Pending.

##### `UpdateState` (METHOD) → `BOOL` — §5.1.1.10.5.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdateState` | `BOOL` | Returns TRUE if successful |

**Description:** Updates the state after a state change has been initiated by calling the ConfigureFromFile() method. The method must be called cyclically until the state of the function block is no longer Pending.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.10.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `GetBusyState` (METHOD) → `BOOL` — §5.1.1.10.5.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetBusyState` | `BOOL` | Returns TRUE if the FB is active. |

**Description:** Returns TRUE if the function block is busy due to an asynchronous file access.

##### `Reset` — §5.1.1.10.5.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Statistics

##### `FB_ALY_StandardDeviation` (FUNCTION_BLOCK) — §5.1.1.10.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Standard Deviation algorithm calculates the empirical standard deviation for a configurable number of input channels. The number of input data to be included in the calculation and the type of calculation can be configured.

#### PLC API › Function blocks › Algorithms › Statistics › FB_ALY_StandardDeviation

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.10.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used |

**Description:** Initializes the number of independent input and output channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.10.6.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.10.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bUseBesselCorrecti BOOL` | `If the parameter bUseBesselCorrection is set to TRUE,` | — |
| `on` | `Bessel's correction is applied. This parameter must be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm.

##### `Reset` — §5.1.1.10.6.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.10.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `GetChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.10.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alue` | — |

**Description:** Fetching a channel-specific output value. The output value is only updated if the Call() method was called previously.

#### PLC API › Function blocks › Algorithms › Statistics

##### `FB_ALY_ClearanceFactor` (FUNCTION_BLOCK) — §5.1.1.10.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The algorithm Clearance Factor calculates the signal feature of the same name from the input values. The output value is calculated from the ratio of the peak value of the input signal to the squared mean of the square roots of the absolute input signal.

#### PLC API › Function blocks › Algorithms › Statistics › FB_ALY_ClearanceFactor

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.10.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used |

**Description:** Initializes the number of independent input and output channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.10.7.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.10.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eWindowMode` | `E_ALY_WindowMode` | Used window mode. Influences the amount of input data |
| `[} 431]` | `included in the calculation and the timing of the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm.

##### `Reset` — §5.1.1.10.7.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.10.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `GetChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.10.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alue` | — |

**Description:** Fetching a channel-specific output value. The output value is only updated if the Call() method was called previously.

#### PLC API › Function blocks › Algorithms › Statistics

##### `FB_ALY_ImpulseFactor` (FUNCTION_BLOCK) — §5.1.1.10.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The algorithm Impulse Factor calculates the signal feature of the same name from the input values. The output value is calculated from the ratio of the peak value of the input signal to the mean value of the input signal.

#### PLC API › Function blocks › Algorithms › Statistics › FB_ALY_ImpulseFactor

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.10.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used |

**Description:** Initializes the number of independent input and output channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.10.8.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.10.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eWindowMode` | `E_ALY_WindowMode` | Used window mode. Influences the amount of input data |
| `[} 431]` | `included in the calculation and the timing of the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm.

##### `Reset` — §5.1.1.10.8.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.10.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `GetChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.10.8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alue` | — |

**Description:** Fetching a channel-specific output value. The output value is only updated if the Call() method was called previously.

#### PLC API › Function blocks › Algorithms › Statistics

##### `FB_ALY_ShapeFactor` (FUNCTION_BLOCK) — §5.1.1.10.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The algorithm Shape Factor calculates the signal feature of the same name from the input values. The output value is calculated from the ratio of the root mean square (RMS) of the input signal to the mean of the absolute values of the input signal.

#### PLC API › Function blocks › Algorithms › Statistics › FB_ALY_ShapeFactor

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.10.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used |

**Description:** Initializes the number of independent input and output channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.10.9.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.10.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eWindowMode` | `E_ALY_WindowMode` | Used window mode. Influences the amount of input data |
| `[} 431]` | `included in the calculation and the timing of the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm.

##### `Reset` — §5.1.1.10.9.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.10.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `GetChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.10.9.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alue` | — |

**Description:** Fetching a channel-specific output value. The output value is only updated if the Call() method was called previously.

#### PLC API › Function blocks › Algorithms › Traning Base

##### `FB_ALY_TimeBasedTeachPath_1Ch` (FUNCTION_BLOCK) — §5.1.1.11.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** Time Based Teach Path 1Ch periodically writes the input data to a file according to the configured number of teach operations. This means that the values are not written sequentially for each period, but the values of a new period are compared with the existing values. The period can be defined by the input values Start Period and Stop Period (boolean signals are required). According to the teach mode, each value is overwritten or retained, so that the result is a taught input signal that can later be used as a reference signal for the Time Based Envelope 1Ch algorithm, for example.

#### PLC API › Function blocks › Algorithms › Traning Base › FB_ALY_TimeBasedTeachPath_1Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.11.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStartPeriod` | `BOOL` | Rising edge starts teach-in process. |
| `bStopPeriod` | `BOOL` | Rising edge stops teach-in process. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.11.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eTeachMode` | `E_ALY_TeachMode [} 430] Configuring the teach-in mode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `Reset` — §5.1.1.11.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `GetBusyState` (METHOD) → `BOOL` — §5.1.1.11.1.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetBusyState` | `BOOL` | Returns TRUE if the FB is active. |

**Description:** Returns TRUE if the function block is busy due to an asynchronous file access.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.11.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SwitchState_Idle` (METHOD) → `BOOL` — §5.1.1.11.1.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Idle` | `BOOL` | Returns TRUE if successful |

**Description:** Initiate switch from Teach state to Idle state. See State diagram.

##### `SwitchState_Teach` (METHOD) → `BOOL` — §5.1.1.11.1.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchState_Teach BOOL` | `Returns TRUE if successful` | — |

**Description:** Initiate switch from Idle state to Teach state. See State diagram.

##### `UpdateState` (METHOD) → `BOOL` — §5.1.1.11.1.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdateState` | `BOOL` | Returns TRUE if successful |

**Description:** Update state after state change was initiated and until target state is not reached.

#### PLC API › Function blocks › Algorithms › Visualization Only

##### `FB_ALY_TrendLine` (FUNCTION_BLOCK) — §5.1.1.12.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Trend Line function block provides for the forwarding of trends of the input signals for later use in the HMI dashboard. Any channels can be selected for forwarding; the number of channels can be controlled via NumChannels. One value per minute is then stored for each channel. This means that the function block is not suitable for signal forwarding, but rather for displaying trends for signals that change slowly.

#### PLC API › Function blocks › Algorithms › Visualization Only › FB_ALY_TrendLine

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.12.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used |

**Description:** The Init method is used to initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.12.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `GetChannelOutputArray` (METHOD) → `BOOL` — §5.1.1.12.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputA BOOL` | `Returns TRUE if successful. rray` | — |

**Description:** Retrieves a channel-specific output array. The output values were updated in the previously called Call() method.

##### `SetChannelInitial` (METHOD) → `BOOL` — §5.1.1.12.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `pArrayIn` | `POINTER TO LREAL` | Pointer to an array from which the trend results are to be initialized. Dim: 10080 elements |
| `nArrayInSize` | `UDINT` | Array size. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialization of the output values of a channel, for example with persistent data.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.12.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInital` (METHOD) → `BOOL` — §5.1.1.12.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tLastValue` | `ULINT` | Timestamp of the last data point added. |
| `nCountValues` | `UDINT` | Number of valid values. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInitial` | `BOOL` | Returns TRUE if successful. |

**Description:** Initialize internal values of the algorithm.

##### `Reset` — §5.1.1.12.1.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

#### PLC API › Function blocks › Algorithms › Visualization Only

##### `FB_ALY_PieChart` (FUNCTION_BLOCK) — §5.1.1.12.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Pie Chart function block can be used to create a pie chart for later use in the HMI Dashboard. The number of pie pieces can be parameterized via the parameterNum Channels.

#### PLC API › Function blocks › Algorithms › Visualization Only › FB_ALY_PieChart

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.12.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannels` | `UDINT` | Initializes the number of input channels. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used |

**Description:** The Init method is used to initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.12.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.12.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bRelativeOutput` | `BOOL` | If TRUE, the output is relative, otherwise absolute. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureChannel` (METHOD) → `BOOL` — §5.1.1.12.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `sChannelName` | `STRING(255)` | Alias of the input channel |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureChannel` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `GetChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.12.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannels) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelOutputV BOOL` | `Returns TRUE if successful. alue` | — |

**Description:** Fetching a channel-specific output value. The output value is only updated if the Call() method was called previously.

##### `GetOutputArray` (METHOD) → `BOOL` — §5.1.1.12.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArrayOut` | `POINTER TO LREAL` | Pointer to the output array: Dim: nBufferSize |
| `nArrayOutSize` | `UDINT` | Size of the output array |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetOutputArray` | `BOOL` | Returns TRUE if successful. |

**Description:** Getting the entire output array. The number of elements corresponds to the configured buffer size.

##### `Reset` — §5.1.1.12.2.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.12.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nChannels) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetInputArray` (METHOD) → `BOOL` — §5.1.1.12.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArrayIn` | `PVOID` | Pointer to an array of input data. Dim: nNumChannels |
| `nArrayInSize` | `UDINT` | Array size. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInputArray` | `BOOL` | Returns TRUE if successful. |

**Description:** Simultaneous setting of all input channels. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › Visualization Only

##### `FB_ALY_SankeyDiagram` (FUNCTION_BLOCK) — §5.1.1.12.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The Sankey Diagram function block can be used to create a Sankey diagram for later use in the HMI Dashboard. A distinction is made between inflows and outflows. The number of inflows and outflows can be individually parameterized via the parameters Num Channels Inflow and Num Channels Outflow respectively.

#### PLC API › Function blocks › Algorithms › Visualization Only › FB_ALY_SankeyDiagram

##### `FB_init` (METHOD) → `BOOL` — §5.1.1.12.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nNumChannelsInflo UDINT` | `Initializes the number of Inflow channels. w` | — |
| `nNumChannelsOutfl UDINT` | `Initializes the number of Outflow channels. ow` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_init` | `BOOL` | Not used |

**Description:** The Init method is used to initialize the number of input channels.

##### `Call` (METHOD) → `BOOL` — §5.1.1.12.3.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Configure` (METHOD) → `BOOL` — §5.1.1.12.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sChannelNameRest STRING(255)` | `Name of the residual output channel.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `ConfigureInflowChannel` (METHOD) → `BOOL` — §5.1.1.12.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannelsInflow) |
| `sChannelName` | `STRING(255)` | Alias of the input channel. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureInflowCha BOOL` | `Returns TRUE if successful. nnel` | — |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `ConfigureOutflowChannel` (METHOD) → `BOOL` — §5.1.1.12.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannelsOutflow) |
| `sChannelName` | `STRING(255)` | Alias of the input channel. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConfigureOutflowCh BOOL` | `Returns TRUE if successful. annel` | — |

**Description:** Configure channel specific parameters. The configuration will not be processed until the method Configure() is called.

##### `GetInflowChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.12.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannelsInflow) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetInflowChannelOu BOOL` | `Returns TRUE if successful. tputValue` | — |

**Description:** Fetching a channel-specific output value. The output value is only updated if the Call() method was called previously.

##### `GetInflowOutputArray` (METHOD) → `BOOL` — §5.1.1.12.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArrayOut` | `POINTER TO LREAL` | Pointer to the output array: Dim: nNumChannelsOutflow |
| `nArrayOutSize` | `UDINT` | Size of the output array. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetInflowOutputArra BOOL` | `Returns TRUE if successful. y` | — |

**Description:** Getting the entire output array. The number of elements corresponds to the configured inflow channels.

##### `GetOutflowChannelOutputValue` (METHOD) → `BOOL` — §5.1.1.12.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannelsOutflow) |
| `output` | `ANY` | Output value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetOutflowChannel BOOL` | `Returns TRUE if successful. OutputValue` | — |

**Description:** Fetching a channel-specific output value. The output value is only updated if the Call() method was called previously.

##### `GetOutflowOutputArray` (METHOD) → `BOOL` — §5.1.1.12.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArrayOut` | `POINTER TO LREAL` | Pointer to the output array: Dim: nNumChannelsOutflow |
| `nArrayOutSize` | `UDINT` | Size of the output array. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetOutflowOputArra BOOL` | `Returns TRUE if successful. y` | — |

**Description:** Getting the entire output array. The number of elements corresponds to the configured outflow channels.

##### `Reset` — §5.1.1.12.3.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetInflowChannelValue` (METHOD) → `BOOL` — §5.1.1.12.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannelsInflow) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInflowChannelVal BOOL` | `Returns TRUE if successful. ue` | — |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `SetOutflowChannelValue` (METHOD) → `BOOL` — §5.1.1.12.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index (1 to nNumChannelsOutflow) |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetOutflowChannelV BOOL` | `Returns TRUE if successful. alue` | — |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › XY Path Analysis

##### `FB_ALY_XyGateMonitor_2Ch` (FUNCTION_BLOCK) — §5.1.1.13.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The XY Gate Monitor 2Ch counts the amount of intersections of an XY input with a specified gate or its projection (straight line between the gate points) depending on the configured Gate Mode. The analysis period can be started with the inputs Start and Stop. The algorithm is direction sensitive, which means that just intersection in the right direction are counted. The direction interpretation depends on the order of the gate points (X1/Y1) and (X2/Y2). The possible intersection directions are visualized below.

#### PLC API › Function blocks › Algorithms › XY Path Analysis › FB_ALY_XyGateMonitor_2Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.13.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Start gate observe. |
| `bStop` | `BOOL` | Stop gate observe. |
| `tTimestamp` | `ULINT` | Current timestamp. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.13.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.13.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eGateMode` | `E_ALY_GateMode [} 431]` | Configured gate mode |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.13.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Value of X-coordinate 2: Value of Y-coordinate |
| `Input` | `ANY` | Input value of arbitrary data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › XY Path Analysis

##### `FB_ALY_XyShapeMonitor_Circle_2Ch` (FUNCTION_BLOCK) — §5.1.1.13.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The XY Shape Monitor Circle 2Ch count the amount of intersections of an XY input with a specified circle shape.

#### PLC API › Function blocks › Algorithms › XY Path Analysis › FB_ALY_XyShapeMonitor_Circle_2Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.13.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.13.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.13.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCentre` | `ST_ALY_XyPosition [} 427] Structure for saving the position of the circle center in XY` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.13.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Value of the X coordinate 2: Value of the Y coordinate |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › XY Path Analysis

##### `FB_ALY_XyShapeMonitor_Rectangle_2Ch` (FUNCTION_BLOCK) — §5.1.1.13.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The XY Shape Monitor Rectangle 2Ch count the amount of intersections of an XY input with a specified rectangle shape.

#### PLC API › Function blocks › Algorithms › XY Path Analysis › FB_ALY_XyShapeMonitor_Rectangle_2Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.13.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.13.3.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.13.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stLowerLeftCorner` | `ST_ALY_XyPosition [} 427] Structure for saving the position of the lower left corner of` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.13.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Value of the X coordinate 2: Value of the Y coordinate |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

#### PLC API › Function blocks › Algorithms › XY Path Analysis

##### `FB_ALY_XyShapeMonitor_Triangle_2Ch` (FUNCTION_BLOCK) — §5.1.1.13.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bPersistent` | `BOOL` | If the value is TRUE, the internal data is stored persistently. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Contains more detailed information on the current return |

**Description:** The XY Shape Monitor Triangle 2Ch counts the amount of intersections of an XY input with a specified triangle shape.

#### PLC API › Function blocks › Algorithms › XY Path Analysis › FB_ALY_XyShapeMonitor_Triangle_2Ch

##### `Call` (METHOD) → `BOOL` — §5.1.1.13.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `ULINT` | Current timestamp with a resolution of 1 ns. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call the algorithm after a new input value is set. A new input can be assigned by using the method SetChannelValue().

##### `Reset` — §5.1.1.13.4.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | Returns TRUE if the reset was successful. |

**Description:** Resetting the algorithm.

##### `SetChannelValue` (METHOD) → `BOOL` — §5.1.1.13.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nChannel` | `UDINT` | Channel index 1: Value of the X coordinate 2: Value of the Y coordinate |
| `Input` | `ANY` | Input value of any data type. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelValue` | `BOOL` | Returns TRUE if successful. |

**Description:** Set channel specific input value. The input value is not processed until the Call() method has been invoked.

##### `Configure` (METHOD) → `BOOL` — §5.1.1.13.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stCorner1` | `ST_ALY_XyPosition [} 427] Structure for saving the position of the first corner in XY` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Configure` | `BOOL` | Returns TRUE if successful. |

**Description:** Configure the algorithm. Channel specific parameter are set using the method ConfigueChannel().

#### PLC API › Function blocks › System › FB_ALY_DateTime

##### `AddRaw` (METHOD) → `ULINT` — §5.1.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRaw` | `LINT` | Raw value to be added to the timestamp. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddRaw` | `ULINT` | Returns the resulting timestamp value. |

**Description:** Call to add a timespan in raw format to the timestamp value.

##### `AddTimespan` (METHOD) → `ULINT` — §5.1.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipTimespan` | `I_ALY_Timespan` | Interface pointer to a timespan instance to be added to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddTimespan` | `ULINT` | Returns the resulting timestamp value. |

**Description:** Call to add a timespan to the timestamp value.

##### `SubRaw` (METHOD) → `BOOL` — §5.1.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRaw` | `LINT` | Raw value to be subtracted from the timestamp. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SubRaw` | `ULINT` | Returns resulting timestamp value. |

**Description:** Call to subtract a timespan in raw format from the timestamp value.

##### `SubTimespan` (METHOD) → `ULINT` — §5.1.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipTimespan` | `I_ALY_Timespan` | Interface pointer to a timespan instance to be added to the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SubTimespan` | `ULINT` | Returns the resulting timestamp value. |

**Description:** Call to subtract a timespan from the timestamp value.

##### `EqualsTo` (METHOD) → `BOOL` — §5.1.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipDateTime` | `I_ALY_DateTime` | Interface pointer to a DateTime instance for comparison. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsTo` | `BOOL` | Returns TRUE if the timestamps are equal. |

**Description:** Call to compare the current timestamp with the input timestamp.

##### `ToString` — §5.1.2.1.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ToString` | `STRING(29)` | Returns the formatted string. |

**Description:** Method to format the current raw value as string. The output format is YYYY-MM-DD-hh:mm:ss.nnnnnnnnn.

#### PLC API › Function blocks › System

##### `FB_ALY_GetSystemTime` (FUNCTION_BLOCK) — §5.1.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tSystemTime` | `ULINT` | Current system time. Methods |
| `Call()` | `Local` | Method for recording the current system time. Sample |

**Description:** Gets the local system time in resolution of 1ns. The accuracy of this time is 100ns. The value is updated by the Call()-method.

#### PLC API › Function blocks › System › FB_ALY_GetSystemTime

##### `Call` — §5.1.2.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | Returns TRUE if successful. |

**Description:** Call to gather the current system time. The system time is stored in the output data of the FB.

#### PLC API › Function blocks › System › FB_ALY_Timespan

##### `AddRaw` (METHOD) → `ULINT` — §5.1.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRaw` | `LINT` | Raw value to be added to the timespan. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddRaw` | `ULINT` | Returns the resulting timespan value. |

**Description:** Call to add a timespan in raw format to the timespan value.

##### `AddTimespan` (METHOD) → `ULINT` — §5.1.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipTimespan` | `I_ALY_Timespan` | Interface pointer to an Timespan instance to be added to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddTimespan` | `ULINT` | Returns resulting timespan value. |

**Description:** Call to add a timespan to the timespan value.

##### `SubRaw` (METHOD) → `BOOL` — §5.1.2.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRaw` | `LINT` | Raw value to be subtracted from the timespan. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SubRaw` | `ULINT` | Returns resulting timespan value. |

**Description:** Call to subtract a timespan in raw format from the timespan value.

##### `SubTimespan` (METHOD) → `ULINT` — §5.1.2.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipTimespan` | `I_ALY_Timespan` | Interface pointer to an Timespan instance to be added to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SubTimespan` | `ULINT` | Returns resulting timespan value. |

**Description:** Call to subtract a timespan from the timespan value.

##### `EqualsTo` (METHOD) → `BOOL` — §5.1.2.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipTimespan` | `I_ALY_Timespan` | Interface pointer to an Timespan instance to compare with. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsTo` | `BOOL` | Returns TRUE if timespans are equal. |

**Description:** Call to compare the current timespan with the input timespan.

##### `ToString` — §5.1.2.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eAccuracy` | `E_ALY_TimestampAccurac Define accuracy of output string. y [} 434]` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ToString` | `STRING` | Returns formatted string. |

**Description:** Method to format the current raw value as string with configurable accuracy. An example output is 1d2m3s4ms5ns.

##### `ToFormatString` — §5.1.2.3.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ToFormatString` | `STRING` | Returns formatted string. |

**Description:** Method to format the current raw value as string. The output format is dd:hh:mm:ss,f.

##### `TotalDays` — §5.1.2.3.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TotalDays` | `DINT` | Returns amount of total days. |

**Description:** Method to get the amount of total days.

##### `TotalHours` — §5.1.2.3.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TotalHours` | `DINT` | Returns amount of total hours. |

**Description:** Method to get the amount of total hours.

##### `TotalMinutes` — §5.1.2.3.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TotalMinutes` | `DINT` | Returns amount of total minutes. |

**Description:** Mehod to get the amount of total minutes.

##### `TotalSeconds` — §5.1.2.3.11

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TotalSeconds` | `LINT` | Returns amount of total seconds. |

**Description:** Method to get the amount of total seconds.

#### PLC API › Function blocks › IoT › IoT Symbol

##### `FB_ALY_IotSymbol_BOOL` (FUNCTION_BLOCK) — §5.1.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_BYTE` (FUNCTION_BLOCK) — §5.1.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_DINT` (FUNCTION_BLOCK) — §5.1.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_DWORD` (FUNCTION_BLOCK) — §5.1.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_INT` (FUNCTION_BLOCK) — §5.1.3.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_LINT` (FUNCTION_BLOCK) — §5.1.3.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_LREAL` (FUNCTION_BLOCK) — §5.1.3.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_LWORD` (FUNCTION_BLOCK) — §5.1.3.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_REAL` (FUNCTION_BLOCK) — §5.1.3.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_SINT` (FUNCTION_BLOCK) — §5.1.3.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_STRING` (FUNCTION_BLOCK) — §5.1.3.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_UDINT` (FUNCTION_BLOCK) — §5.1.3.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_UINT` (FUNCTION_BLOCK) — §5.1.3.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_ULINT` (FUNCTION_BLOCK) — §5.1.3.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_USINT` (FUNCTION_BLOCK) — §5.1.3.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

##### `FB_ALY_IotSymbol_WORD` (FUNCTION_BLOCK) — §5.1.3.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bSymbolHandlerAssi BOOL` | `TRUE if symbol handler assigned gned` | — |
| `bVariableFound` | `BOOL` | TRUE if variable found in stream |
| `sSymbolPath` | `STRING(255)` | Topic/Stream.Symbol |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nMaxNumElements UDINT` | `Maximum number of Symbols stored affected by StreamHelper` | — |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `GetValue` | `Local` | Get the value of the specified element |
| `GetOversamplingValue` | `Local` | Get the oversampling values of the specified element s |
| `GetArrayValues` | `Local` | Get the array values of the specified element |

**Description:** Syntax

#### PLC API › Function blocks › IoT

##### `FB_ALY_StreamHelper` (FUNCTION_BLOCK) — §5.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_ALY_IotSymbol_Config` | Struct to configure the FB |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `bNewResult` | `BOOL` | TRUE if a new result has been calculated |
| `bConfigured` | `BOOL` | TRUE if FB is configured successfully |
| `bConnected` | `BOOL` | TRUE if StreamHelper connected to message broker |
| `sStream` | `STRING(255)` | Topic/Stream |
| `nNumIotSymbolsRe` | `UDINT` | Amount of registered IoT symbols gistered |
| `tCycleTime` | `LTIME` | Cycle time of publishing system |
| `nNumElements` | `UDINT` | Number of elements received last call Methods |
| `Call` | `Local` | — |
| `Configure` | `Local` | — |
| `GetTimestampElement` | `Local` | — |
| `AddIotSymbol` | `Local` | — |
| `ContainsIotSymbol` | `Local` | — |
| `ReleaseAllIotSymbols` | `Local` | — |
| `ReleaseIotSymbol` | `Local` | — |

**Description:** Syntax

#### PLC API › Function blocks › IoT › BinaryStream

##### `FB_ALY_BinaryStream_File` (FUNCTION_BLOCK) — §5.1.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCycleTime` | `UDINT` | Logging cycle time |
| `stSystemID` | `GUID` | Twincat System ID |
| `sPath` | `T_MaxString` | Folder path to log in |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `ipResultMessage` | `I_TcMessage` | Eventlogger Methods |
| `Call [} 421]` | `Local` | Method for background communication with the TwinCAT |

**Description:** Syntax

#### PLC API › Function blocks › IoT › BinaryStream › FB_ALY_BinaryStream_File

##### `Call` (METHOD) → `BOOL` — §5.1.3.3.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | — |

**Description:** Syntax

##### `Close` (METHOD) → `BOOL` — §5.1.3.3.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Close` | `BOOL` | TRUE if done |

**Description:** Syntax

##### `CreateDescriptionFile` (METHOD) → `BOOL` — §5.1.3.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDataSize` | `UDINT` | Size of one Sample |
| `sLayout` | `GUID` | Layout Guid created by CreateSymbolic |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateDescripti BOOL` | `TRUE if done onFile` | — |

**Description:** Syntax

##### `CreateSymbolsFile` (METHOD) → `BOOL` — §5.1.3.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSymbol` | `PVOID` | Pointer to ADS Symbol |
| `nSymbolSize` | `UDINT` | Size of ADS Symbol |
| `sDataType` | `STRING` | Name of ADS Symbol Datatype |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sLayout` | `GUID` | Layout hash of symbol description |
| `CreateSymbols BOOL` | `TRUE if done File` | — |

**Description:** Syntax

##### `WriteData` (METHOD) → `BOOL` — §5.1.3.3.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pData` | `PVOID` | Pointer to sample array |
| `nDataSize` | `UDINT` | Size of sample array |
| `nDataCount` | `INT` | Count of sample array elements |
| `sLayout` | `GUID` | Symbolic layout |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WriteData` | `BOOL` | TRUE if done |

**Description:** Syntax

#### PLC API › Function blocks › IoT › BinaryStream

##### `FB_ALY_BinaryStream_Mqtt` (FUNCTION_BLOCK) — §5.1.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nCycleTime` | `UDINT` | Logging cycle time |
| `stSystemID` | `GUID` | Twincat System ID |
| `sStreamTopic` | `T_MaxString` | Topic to stream to |
| `stMqttConnSettings` | `ST_MqttConnectionSettings` | Settings for the mqtt connection to the message broker |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error has been occurred |
| `ipResultMessage` | `I_TcMessage` | Eventlogger |
| `eConnectionState` | `ETcIotMqttClientState` | Indicates the state of the connection between client and |

**Description:** Syntax

#### PLC API › Function blocks › IoT › BinaryStream › FB_ALY_BinaryStream_Mqtt

##### `Call` (METHOD) → `BOOL` — §5.1.3.3.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Call` | `BOOL` | — |

**Description:** Syntax

##### `CloseStream` (METHOD) → `BOOL` — §5.1.3.3.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CloseStream` | `BOOL` | TRUE if done |

**Description:** Syntax

##### `CreateDescription` (METHOD) → `BOOL` — §5.1.3.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDataSize` | `UDINT` | Size of one Sample |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateDescripti BOOL` | `TRUE if done on` | — |

**Description:** Syntax

##### `CreateSymbolic` (METHOD) → `BOOL` — §5.1.3.3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSymbol` | `PVOID` | Pointer to ADS Symbol |
| `nSymbolSize` | `UDINT` | Size of ADS Symbol |
| `sDataType` | `STRING` | Name of ADS Symbol Datatype |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sLayout` | `GUID` | Layout hash of symbol description |
| `CreateSymbolic BOOL` | `TRUE if done` | — |

**Description:** Syntax

##### `SendData` (METHOD) → `BOOL` — §5.1.3.3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pData` | `PVOID` | Pointer to sample array |
| `nDataSize` | `UDINT` | Size of sample array |
| `nDataCount` | `INT` | Count of sample array elements |
| `sLayout` | `GUID` | Symbolic layout |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendData` | `BOOL` | TRUE if done |

**Description:** Syntax

#### PLC API › Functions

##### `F_RawTimespan_TO_Structured` — §5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimespan` | `LINT` | Timespan to be converted. |

**Description:** Function to convert an raw timespan input to an output of type ST_ALY_Timespan [} 434].

##### `F_StructuredTimespan_TO_Raw` — §5.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stTimespan` | `ST_ALY_Timespan [} 434] Structured timespan to be converted.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_StructuredTimesp LINT` | `Returns raw timespan. an_TO_Raw` | — |

**Description:** Function to convert an input of type ST_ALY_Timespan [} 434] to a raw timespan.
