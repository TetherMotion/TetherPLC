# Tc2_ControllerToolbox

- **Manual:** `TF4100_TC3_Controller_Toolbox_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Control · **Tier:** T0 · **API entries:** 55
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Controller toolbox (PID, filters, function generators): pure control math.

---

#### PLC API › Reference › Function blocks › Auxiliary

##### `FB_CTRL_GET_SYSTEM_TIME` — §4.2.1.1.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_GET_SYSTEM_TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TYPE TIMESTRUCT PLC API STRUCT` | `` | — |
| `wYear` | `: WORD;` | — |
| `wMonth` | `: WORD;` | — |
| `wDayOfWeek` | `: WORD;` | — |
| `wDay` | `: WORD;` | — |
| `wHour` | `: WORD;` | — |
| `wMinute` | `: WORD;` | — |
| `wSecond` | `: WORD;` | — |
| `wMilliseconds` | `: WORD; END_STRUCT END_TYPE` | — |
| `stSystemTime` | `TIME STRUCT` | Structure in which the system time is output. |
| `eErrorId` | `E_CTRL_ ERROR CODES` | Supplies the error number [} 173] when the output bError is set. |
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `wYear` | `WORD` | The year: 1970 ~ 2106; |
| `wMonth` | `WORD` | The month: 1 ~ 12 (January = 1, February = 2 etc.); |
| `wDayOfWeek` | `WORD` | The day of the week: 0 ~ 6 (Sunday = 0, Monday = 1 etc.); |
| `wDay` | `WORD` | The day of the month: 1 ~ 31; |
| `wHour` | `WORD` | Hour: 0 ~ 23; |
| `wMinute` | `WORD` | Minute: 0 ~ 59; |
| `wSecond` | `WORD` | Second: 0 ~ 59; |
| `WMilliseconds` | `WORD` | Millisecond: 0 ~ 999; Inputs/ outputs |
| `stParams` | `ST_CTRL_ GET_SYSTEM_TI ME` | Parameter structure of the function block |

**Description:** The function block reads the current Windows system time and makes it available in the SystemTimeStruct.

##### `FB_CTRL_GET_TASK_CYCLETIME` — §4.2.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tTaskCycleTime TIME` | `This output indicates the current task cycle time, with a resolution of 1` | — |

**Description:** This function block allows the task cycle time of a program to be determined with a resolution of 1 ms.

##### `FB_CTRL_LOOP_SCHEDULER` — §4.2.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nManValue` | `DWORD` | This input allows the first 32 bits in the output vector to be set in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_LOOP_SCHEDULER_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_ LOOP_ SCHEDULER_ PARAMS` | Parameter structure for the loop scheduler. |

**Description:** This function block allows the system load to be distributed over a number of control loops that are parameterized using the same tCtrlCycleTime and for which the condition "tCtrlCycleTime > tTaskCycleTime" is true. The output vector calculated by this function block is used to start the individual control loops at different times, so that the system loading is distributed.

#### PLC API › Reference › Function blocks › Base

##### `FB_CTRL_HYSTERESIS` — §4.2.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input of the hysteresis element |
| `bManSync` | `BOOL` | Input through which the hysteresis element can be set to one of the |
| `Value` | `two junctions.` | — |
| `bSync` | `BOOL` | A rising edge at this input sets the hysteresis element to the value |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_HYSTERESIS_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | Output of the hysteresis element |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_ HYSTERESIS_ PARAMS` | Parameter structure of the hysteresis element |

**Description:** The function block provides a hysteresis transfer element in a functional diagram.

##### `FB_CTRL_P` — §4.2.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input of the P element |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_P_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the P element |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_P_ PARAMS` | Parameter structure of the P element |

**Description:** The function block provides an P-transfer element in the functional diagram.

##### `FB_CTRL_I` — §4.2.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input of the I element |
| `fManSyncValue` | `FLOAT` | Input to which the I-element can be synchronized, or whose value is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_I_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the I element |
| `bARWactive` | `BOOL` | A TRUE at this output indicates that the integrator is being restricted. |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_I_ PARAMS` | Parameter structure of the I element |

**Description:** The function block provides an I-transfer element in the functional diagram.

##### `FB_CTRL_I_WITH_DRIFTCOMPENSATION` — §4.2.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input of the I element |
| `fManSyncValue` | `FLOAT` | Input to which the I-element can be synchronized, or whose value is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_I_WITH_DRIFTCOMPENSATION_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the I element |
| `bARWactive` | `BOOL` | A TRUE at this output indicates that the integrator is being restricted. |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_I_ WITH_DRIFTCO MPENSATION_ PARAMS` | Parameter structure of the I element |

**Description:** The function block represents an I transfer element with drift compensation.

##### `FB_CTRL_D` — §4.2.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input of the D element |
| `fManSyncValue` | `FLOAT` | Input to which the D-element can be synchronized, or whose value is |
| `fIn` | `FLOAT` | Input to the transfer function |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode of the function block. |
| `fIn` | `FLOAT` | Input to the transfer function |
| `fManValue` | `FLOAT` | Input whose value is present at the output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_D_PARAMS` | — |
| `stParams` | `ST_CTRL_TRANSFERFUNCTION_1_PARAMS` | — |
| `stParams` | `ST_CTRL_TRANSFERFUNCTION_2_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the D element |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_D_ PARAMS` | Parameter structure of the D-element |
| `fOut` | `FLOAT` | Output from the transfer function |
| `eState` | `E_CTRL_STATE` | State of the function block |
| `bError` | `BOOL` | Supplies the error number [} 173] when the output bError is set. |
| `eErrorId` | `E_CTRL_ERROR Becomes TRUE, as soon as an error occurs. CODES Inputs/ outputs` | — |
| `stParams` | `ST_CTRL_TRAN Parameter structure of the function block SFERFUNCTION _1_PARAMS` | — |
| `fOut` | `FLOAT` | Output from the transfer function |
| `eState` | `E_CTRL_STATE` | State of the function block |
| `bError` | `BOOL` | Supplies the error number [} 173] when the output bError is set. |
| `eErrorId` | `E_CTRL_ERROR Becomes TRUE, as soon as an error occurs. CODES Inputs/ outputs` | — |
| `stParams` | `ST_CTRL_TRAN Parameter structure of the function block SFERFUNCTION _2_PARAMS` | — |

**Description:** The function block provides a DT1 transfer element (a real D-element) in a functional diagram.

#### PLC API › Reference › Function blocks › Controller

##### `FB_CTRL_2POINT` — §4.2.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `bManSyncValue` | `BOOL` | Input through which the 2-point element can be set to one of the two |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_2POINT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | Output of the 2-point element |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Input/ outputs |
| `stParams` | `ST_CTRL_ 2POINT_ PARAMS` | Parameter structure of the 2-point element |

**Description:** The function block provides a 2-point transfer element in the functional diagram.

##### `FB_CTRL_2POINT_PWM_ADAPTIVE` — §4.2.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `fManSync` | `FLOAT` | Input to which the controller's mark-to-space ratio can be set, or with |
| `Value` | `which the output can be set in Manual Mode. The output is set in` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_2POINT_PWM_ADAPTIVE_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | Controller output |
| `fPWMDutyCycle` | `FLOAT` | Current mark-to-space ratio of the internal PWM function block |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_2POIN Parameter structure of the 2-point element T_PWM_ ADAPTIVE_ PARAMS` | — |

**Description:** The function block provides an adaptive two-position controller. It is particularly suitable for single-area controllers in which high inlet temperatures are present and which make use of a thermal actuator.

##### `FB_CTRL_3POINT` — §4.2.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `nManSyncValue` | `INT` | Input with which the 3-point link can be set to one of the three |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_3POINT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPosOut` | `BOOL` | This output from the 3-point element is TRUE if the upper junction of |

**Description:** The function block provides a 3-point transfer element in the functional diagram.

##### `FB_CTRL_3POINT_EXT` — §4.2.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `fManSyncValue` | `FLOAT` | Input through which the extended 3-point element can be set to one of |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_3POINT_EXT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the extended 3-point element |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_ 3POINT_EXT_ PARAMS` | Parameter structure of the extended 3-point element |

**Description:** The function block provides an extended 3-point element in the functional diagram.

##### `FB_CTRL_nPOINT` — §4.2.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `fManValue` | `BOOL` | Input whose value is output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_nPOINT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `nPOINT_CTRL_ Output of the n-point element TABLE_ELEMEN T` | — |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_nPOIN Parameter structure of the n-point element T_PARAMS` | — |

**Description:** The function block provides an n-point transfer element in the functional diagram.

##### `FB_CTRL_PARAMETER_SWITCH` — §4.2.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fManipulated` | `FLOAT` | Input value of the FB_Parameter_Switch. This should be equal to |
| `Variable` | `the output value of the controller.` | — |
| `nManSync` | `eCTRL_PARAME The input with which the function block can be set to one of the` | — |
| `Value` | `TER_RECORD_ parameter ranges. HEATING` | — |
| `bSync` | `BOOL` | A rising edge at this input sets the function block to the value |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PARAMETER_SWITCH_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eParameterReco E_CTRL_` | `The output of the function block, identifying the parameter range.` | — |
| `rd` | `PARAMETER_ RECORD` | — |
| `fOutMaxLimit` | `FLOAT` | The maximum output value of which the controller is limited. (This |

**Description:** This function block can be used to switch the parameter set used by FB_CTRL_PID_SPLITRANGE.

##### `FB_CTRL_PI` — §4.2.1.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `fManSyncValue` | `FLOAT` | Input with which the PI element can be set. |
| `bSync` | `BOOL` | A rising edge at this input sets the PI-element to the value |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PI_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the PI element |
| `bARWactive` | `BOOL` | A TRUE at this output indicates that the PI-element is being restricted. |
| `eState` | `E_CTRL_STATE` | State of the function block |
| `eErrorId` | `E_CTRL_ ERRORCODES` | Supplies the error number [} 173] when the output bError is set. |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_PI_ PARAMS` | Parameter structure of the PI element |

**Description:** The function block provides a PI transfer element in the functional diagram.

##### `FB_CTRL_PI_PID` — §4.2.1.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValueOute FLOAT` | `The actual value of the controlled variable that is fed back to the PI` | — |
| `rLoop` | `controller of the outer control loop.` | — |
| `fActualValueInne FLOAT` | `The actual value of the controlled variable that is fed back to the PID` | — |
| `rLoop` | `controller of the inner control loop.` | — |
| `fPreControl` | `FLOAT` | Pre-control that is connected behind the PI controller. |
| `fManSyncValueIn FLOAT` | `Input, to whose value it is possible to set the internal state of the PID-` | — |
| `nerLoop` | `element (the inner control loop).` | — |
| `bSyncInnerLoop BOOL` | `A rising edge at this input sets the PID-element (the inner control` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PI_PID_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the PI-PID-element |

**Description:** The function block provides a cascaded PI-PID controller in the functional diagram. Internally, this function block uses the FB_CTRL_PI, FB_CTRL_LIMITER and FB_CTRL_PID transfer elements.

##### `FB_CTRL_PID` — §4.2.1.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `fManSyncValue` | `FLOAT` | Input, to whose value it is possible to set the internal state of the PID- |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PID_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the PID element |
| `bARWactive` | `BOOL` | A TRUE at this output indicates that the PID-element is being |

**Description:** The function block provides a PID transfer element in the functional diagram.

##### `FB_CTRL_PID_EXT_SPLITRANGE` — §4.2.1.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue FLOAT` | `Setpoint of the controlled variable` | — |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `eParameterReco E_CTRL_PARAM` | `Index of the active parameter set` | — |
| `rd` | `ETER_RECORD` | — |
| `fManSyncValue FLOAT` | `Input with which the PI element can be set.` | — |
| `bSync` | `BOOL` | A rising edge at this input sets the PI-element to the value |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PID_EXT_SPLITRANGE_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOutPos` | `FLOAT` | Output of the PID-element when the control value is positive. A zero is |

**Description:** The function block provides an extended PID transfer element in the functional diagram. With this controller it is possible to switch between two different parameter sets while the regulation is active. The functionalities of the inner and outer windows and of the input and output dead bands are available in addition.

##### `FB_CTRL_PID_EXT` — §4.2.1.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `fManSyncValue` | `FLOAT` | Input with which the PID-element can be set. |
| `bSync` | `BOOL` | A rising edge at this input sets the PID-element to the value |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PID_EXT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the PID element |
| `bMaxLimit` | `BOOL` | The output is TRUE when the function block is at its upper limit. Reached |
| `bMinLimit` | `BOOL` | The output is TRUE when the function block is at its lower limit. Reached |
| `bARWActive` | `BOOL` | A TRUE at this output indicates that the PID-element is being |
| `fInnerWindow` | `REAL` | See description above. |
| `fOuterWindow` | `REAL` | See description above. |
| `fOutMaxLimit` | `FLOAT` | Upper limit at which integration is halted and to which the output is |

**Description:** Transfer function

##### `FB_CTRL_PID_SPLITRANGE` — §4.2.1.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `eParameter` | `E_CTRL_` | Index of the active parameter set |
| `Record` | `PARAMETER_ RECORD` | — |
| `fManSyncValue` | `FLOAT` | Input with which the PI element can be set. |
| `bSync` | `BOOL` | A rising edge at this input sets the PI-element to the value |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PID_SPLITRANGE_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOutPos` | `FLOAT` | Output of the PID-element when the control value is positive. A zero is |

**Description:** The function block provides an extended PID transfer element in the functional diagram. This controller enables switching between two parameter sets when control is active.

##### `FB_CTRL_STEP_PID` — §4.2.1.3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `LREAL` | Setpoint of the controlled variable |
| `fActualValue` | `LREAL` | Actual value of the control value |
| `bLimitSwitchPos` | `BOOL` | Limit switch, becomes TRUE when the upper stop is reached. |
| `bLimitSwitchNeg` | `BOOL` | Limit switch, becomes TRUE when the lower stop is reached. |
| `bAutoReset` | `BOOL` | If the variable is TRUE, it resets the controller to initial conditions. |
| `nSensorType` | `E_CTRL_STEP_ SENSORTYPES` | Enables the selection of the correct thermocouple type. |
| `fDisturbance` | `REAL` | Actual value of the disturbance variable Value |
| `bManualDigital` | `BOOL` | If the variable is TRUE, the motor is manually set to the positive |
| `Pos` | `direction` | — |
| `bManualDigital` | `BOOL` | If TRUE, the motor is manually set to the negative direction Neg |
| `eMode` | `E_CTRL_MODE` | Input specifying the operation mode of the controller |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCtrlParams` | `ST_CTRL_STEP_PID_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bCtrlDigitalPos` | `BOOL` | Output required to run the motor in a positive direction. |
| `bCtrlDigitalNeg` | `BOOL` | Output required to run the motor in a negative direction. |
| `fCtrlAnalog` | `LREAL` | Analog output of the controller |
| `eState` | `E_CTRL_STATE Actual state of the controller` | — |
| `eErrorId` | `E_CTRL_ERROR Returns the error number if bError is TRUE. CODES` | — |
| `bError` | `BOOL` | TRUE if an error occurs in the function block. |
| `bBusy` | `BOOL` | TRUE when the function block is active. |
| `bMinLimitNeg` | `BOOL` | TRUE at this output indicates that the controller has reached its |

**Description:** The function block controls a digitally manipulated variable for the integration of actuators. It is based on a standard PID controller with an additional binary output signal in conjunction with the analog output and works without a precise position feedback signal.

#### PLC API › Reference › Function blocks › Filter / ControlledSystemSimulation

##### `FB_CTRL_ACTUAL_VALUE_FILTER` — §4.2.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value of the filter |
| `fManValue` | `FLOAT` | Input value that is output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_ACTUAL_VALUE_FILTER_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the function block |
| `eState` | `E_CTRL_ STATE` | State of the function block |
| `eErrorId` | `E_CTRL_ ERROR CODES` | Supplies the error number [} 173] when the output bError is set. |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_ACTU Parameter structure of the actual value filter element AL_VALUE_FILTE R_ PARAMS` | — |

**Description:** The function block allows a measured input value to be checked for plausibility and filtered.

##### `FB_CTRL_ARITHMETIC_MEAN` — §4.2.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value for the mean value filter |
| `fManSync` | `FLOAT` | Input value to which the mean value filter can be set, or that is issued |
| `Value` | `at the output in manual mode.` | — |
| `bSync` | `BOOL` | A rising edge at this input sets the mean value filter to the value |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_ARITHMETIC_MEAN_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the mean value filter |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_ ARITHMETIC_ MEAN_PARAMS` | Parameter structure of the mean value filter |

**Description:** 1 c=     å cn n n

##### `FB_CTRL_MEDIAN_FILTER` — §4.2.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value of the median filter |
| `fManSyncValue` | `FLOAT` | Input value to which the median filter can be set or which is output in |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tCtrlCycleTime` | `TIME := T#0ms` | — |
| `tTaskCycleTime` | `TIME := T#0ms` | — |
| `nSamplesToFilter` | `UINT` | — |
| `pWorkArray_ADR` | `POINTER TO FLOAT := 0` | — |
| `nWorkArray_SIZEOF` | `UINT := 0` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the median filter |
| `eState` | `E_CTRL_STATE Status of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `tCtrlCycleTime` | `TIME` | Cycle time with which the control loop is processed. This must be |

**Description:** The function block specifies the median. The median is the mean value of a list of values ordered by size. This means that one half of the collected data values is smaller than the median value, the other half is larger.

##### `FB_CTRL_DIGITAL_FILTER` — §4.2.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input of the digital filter |
| `fManValue` | `FLOAT` | Input whose value is present at the output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_DIGITAL_FILTER_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the digital filter |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES Inputs/ outputs` | — |
| `stParams` | `ST_CTRL_ DIGITAL_FILTER _PARAMS` | Parameter structure of the function block |

**Description:** This function block calculates a discrete transfer function with the structure described below. This structure allows either an FIR filter (Finite Impulse Response) or an IIR filter (Infinite Impulse Response) to be implemented. The transfer function here can be of any order, n.

##### `FB_CTRL_MOVING_AVERAGE` — §4.2.1.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value for the moving average filter |
| `fManSyncValue` | `FLOAT` | Input value to which the moving average filter can be set, or that is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_MOVING_AVERAGE_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the moving average filter |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_MOVIN Parameter structure of the moving average filter G_AVERAGE_PA RAMS` | — |

**Description:** The function block provides a moving average filter in the functional diagram.

##### `FB_CTRL_LEAD_LAG` — §4.2.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value of the notch filter |
| `fManSyncValue` | `FLOAT` | Input value that is output at the output in manual mode. |
| `bSync` | `FLOAT` | reserve |
| `fManValue` | `FLOAT` | Input that specifies the operation mode [} 173] of the function block. |
| `eMode` | `E_CTRL_MODE` | A TRUE at this input will hold the internal state (and therefore also the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_LEAD_LAG_FILTER_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Lead/lag filter output |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set.` | — |
| `CODES` | `(p>)` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_LEAD_ Parameter structure of the lead/lag filter LAG_FILTER_ PARAMS` | — |

**Description:** The function block represents a digital lead/lag filter.

##### `FB_CTRL_NOISE_GENERATOR` — §4.2.1.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fManSyncValue` | `FLOAT` | Input value that is output at the output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_NOISE_GENERATOR_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the noise generator |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_NOISE_ Parameter structure of the noise generator GENERATOR_ PARAMS` | — |

**Description:** This function block generates a noise signal on the basis of the pseudo-random number in the range [- fAmplitude ... fAmplitude].

##### `FB_CTRL_NOTCH_FILTER` — §4.2.1.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value of the notch filter |
| `fManValue` | `FLOAT` | Input value that is output at the output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |
| `fIn` | `FLOAT` | Input value of the PT1 element |
| `fManSyncValue` | `FLOAT` | Input value to which the PT1 element can be set, or that is issued at |
| `fIn` | `FLOAT` | Input value of the PT2 element |
| `fManValue` | `FLOAT` | Input value that is output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |
| `bHold` | `BOOL` | A TRUE at this input will hold the internal state (and therefore also the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_NOTCH_FILTER_PARAMS` | — |
| `stParams` | `ST_CTRL_PT1_PARAMS` | — |
| `stParams` | `ST_CTRL_PT2_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Notch filter output |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_ NOTCH_FILTER_ PARAMS` | Parameter structure of the notch filter |
| `fOut` | `FLOAT` | Output of the PT1 element |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. |
| `stParams` | `ST_CTRL_PT1_ PARAMS` | Parameter structure of the PT1 element |
| `fOut` | `FLOAT` | Output of the PT2 element |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_PT2_ PARAMS` | Parameter structure of the PT2 element |

**Description:** The function block represents a digital notch filter.

##### `FB_CTRL_PT2oscillation` — §4.2.1.4.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value of the oscillating PT2 element |
| `fManValue` | `FLOAT` | Input value that is output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |
| `bHold` | `BOOL` | A TRUE at this input will hold the internal state (and therefore also the |
| `fIn` | `FLOAT` | Input value of the PT3 element |
| `fManValue` | `FLOAT` | Input value that is output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |
| `bHold` | `BOOL` | A TRUE at this input will hold the internal state (and therefore also the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PT2oscillation_PARAMS` | — |
| `stParams` | `ST_CTRL_PT3_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the PT2 element |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_PT2 PARAMS` | Parameter structure of the oscillating PT2 element oscillation_ |
| `fOut` | `FLOAT` | Output of the PT3 element |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_PT3_ PARAMS` | Parameter structure of the PT3 element |

**Description:** The function block provides an oscillating PT2 transfer element in the functional diagram.

##### `FB_CTRL_PTn` — §4.2.1.4.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value of the PTn element |
| `fManValue` | `FLOAT` | Input value that is output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |
| `bHold` | `BOOL` | A TRUE at this input will hold the internal state (and therefore also the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PTn_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the PTn element |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_PTn_ PARAMS` | Parameter structure of the PTn element |

**Description:** This function block provides a non-oscillating PTn transfer element with "n<= 10" and equal time constants in the functional diagram.

##### `FB_CTRL_PTt` — §4.2.1.4.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value of the PTt element |
| `fManSyncValue` | `FLOAT` | Input value to which the PTt element can be set, or that is issued at |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PTt_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the PTt element |
| `bSampleRate` | `BOOL` | Output that indicates whether the function block has internally |
| `Changed` | `reduced the sampling rate because of the array being used to delay` | — |

**Description:** The function block provides an PTt transfer element in the functional diagram.

##### `FB_CTRL_SERVO_MOTOR_SIMULATION` — §4.2.1.4.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bMovePos` | `BOOL` | Input that moves the simulated actuator in the positive direction. |
| `bMoveNeg` | `BOOL` | Input that moves the simulated actuator in the negative direction. |
| `fManSyncValue` | `FLOAT` | Input with which the simulated motor position can be set, or the value |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_SERVO_MOTOR_SIMULATION_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fMotorPositon` | `FLOAT` | Simulated motor position in the range [ fMovingRangeMin ... fMovingRangeMax ] |
| `fMotorState` | `FLOAT` | Simulated motor position in the range [ 0 … 100.0 ] |
| `bUpperLimit` | `BOOL` | Simulated limit switch at the actuator's positive stop Switch |
| `bLowerLimit` | `BOOL` | Simulated limit switch at the actuator's negative stop. Switch |
| `eState` | `E_CTRL_ STATE` | State of the function block |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_ SERVO_MOTOR_ SIMULATION_ PARAMS` | Parameter structure of the function block |

**Description:** The behavior of an actuator can be simulated with this function block.

##### `FB_CTRL_TuTg` — §4.2.1.4.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value of the TuTg element |
| `fManSyncValue` | `FLOAT` | Input value to which the TuTg element can be set, or that is issued at |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_TuTg_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the TuTg element |
| `bSampleRate` | `BOOL` | Output that indicates whether the function block has internally |
| `Changed` | `reduced the sampling rate because of the array being used to delay` | — |

**Description:** The function block provides a TuTg transfer element (a dead time delay element) in the functional diagram.

##### `FB_CTRL_ZERO_ZONE_DAMPING` — §4.2.1.4.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Setpoint of the controlled variable |
| `fActualValue` | `FLOAT` | Actual value of the controlled variable |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_ZERO_ZONE_DAMPING_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetpointValue` | `FLOAT` | Filtered setpoint to controller Out |
| `fActualValueOut` | `FLOAT` | Actual value to controller |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_ZERO_ Parameter structure of the transfer element ZONE_ DAMPING_ PARAMS` | — |

**Description:** This function block enables zero point damping to be realized in order to minimize control interventions in the range | actual value - setpoint | < ε.

#### PLC API › Reference › Function blocks › Interpolation

##### `FB_CTRL_LIN_INTERPOLATION` — §4.2.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value for the interpolation function block |
| `fManValue` | `FLOAT` | Input value that is output in manual mode. |
| `bExtrapolate` | `BOOL` | If this input is FALSE, then the value of the last interpolation point is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_LIN_INTERPOLATION_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Table value from linear interpolation |
| `bInIsGreater` | `BOOL` | A TRUE at this output indicates that the input value is greater than the |
| `ThanMax` | `largest interpolation point. Element` | — |
| `bInIsLessThan` | `BOOL` | A TRUE at this output indicates that the input value is smaller than the |
| `Min` | `smallest interpolation point. Element` | — |
| `eState` | `E_CTRL_ STATE` | State of the function block |
| `eErrorId` | `E_CTRL_ ERRORCODES` | Supplies the error number [} 173] when the output bError is set. |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_LIN_IN Parameter structure of the interpolation element TERPOLATION_ PARAMS` | — |

**Description:** This function block performs linear interpolation to obtain values on the basis of a sampling points table.

##### `FB_CTRL_NORMALIZE` — §4.2.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value |
| `fManValue` | `FLOAT` | Input value that is output in manual mode. |
| `bExtrapolate` | `BOOL` | If this input is FALSE, then the value of the last interpolation point is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_NORMALIZE_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Table value from linear interpolation |
| `bInIsGreaterThan BOOL` | `A TRUE at this output indicates that the input value is greater than the` | — |
| `MaxElement` | `largest interpolation point.` | — |
| `bInIsLessThanMi BOOL` | `A TRUE at this output indicates that the input value is smaller than the` | — |
| `nElement` | `smallest interpolation point.` | — |
| `eState` | `E_CTRL_ STATE` | State of the function block |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_NORM Parameter structure of the function block ALIZE_PARAMS` | — |

**Description:** This function block can be used to linearize a non-linear transfer element, with the aid of an inverse characteristic curve.

#### PLC API › Reference › Function blocks › Monitoring / Alarming

##### `FB_CTRL_CHECK_IF_IN_BAND` — §4.2.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value that is monitored. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_CHECK_IF_IN_BAND_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bInBand` | `BOOL` | A TRUE at this output indicates that the input value is within the |

**Description:** The function block monitors whether the input value is within the range [fMin ... fMax], i.e. whether the inequality fMin £ fIn £ fMax is fulfilled.

##### `FB_CTRL_LOG_DATA` — §4.2.1.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fLogData` | `T_CTRL_LOGGER_DATA` | — |
| `eMode` | `E_CTRL_MODE` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_LOG_DATA_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eState` | `E_CTRL_STATE` | State of the function block |
| `bFileOpen` | `BOOL` | A TRUE at this output indicates that the file has successfully been |

**Description:** This function block allows a log file to be created in *.csv format (comma separated values), in which a maximum of 10 variables may be recorded. The column headings specified by the user are written in the first row of this file. The input data is written at equal time intervals in the following lines. The individual entries are separated by a comma. The time interval between the entries is specified in the tLogCycleTime parameter. If, for instance, tLogCycleTime := T\#2s is chosen, then an entry is made in the file every 2s. The files that were generated can be analyzed with a spreadsheet program, for example.

##### `FB_CTRL_LOG_MAT_FILE` — §4.2.1.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fLogData` | `T_CTRL_LOGGER_DATA` | — |
| `eMode` | `E_CTRL_MODE` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_LOG_MAT_FILE_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eState` | `E_CTRL_STATE` | State of the function block |
| `bFileOpen` | `BOOL` | A TRUE at this output indicates that the file has successfully been |

**Description:** This function block allows a log file to be created in Matlab 5 (*.mat) format, in which a maximum of 10 magnitudes may be recorded.

#### PLC API › Reference › Function blocks › Output To Controlling Equipment

##### `FB_CTRL_DEADBAND` — §4.2.1.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_DEADBAND_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the function block |
| `bInIsUnder` | `BOOL` | A TRUE at this output indicates that the input value is within the dead |
| `Threshold` | `band.` | — |
| `eState` | `E_CTRL_STATE` | State of the function block |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_DEAD Parameter structure of the function block BAND_PARAMS` | — |

**Description:** This function block provides a dead band for the input signal. If the input signal is within the dead band, this is indicated by the bInIsUnderThreshold output.

##### `FB_CTRL_LIMITER` — §4.2.1.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value for the function block |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_LIMITER_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AR_OUTPUT` | `` | — |
| `fOut` | `: FLOAT;` | — |
| `bMinLimit` | `: BOOL;` | — |
| `bMaxLimit` | `: BOOL;` | — |
| `eErrorId` | `: E_CTRL_ERRORCODES;` | — |
| `bError` | `: BOOL;` | — |
| `fOut` | `FLOAT` | Output of the function block |
| `bMinLimit` | `BOOL` | A TRUE at this output indicates that the output has reached the lower |

**Description:** This function block limits an input signal to a parameterizable interval.

##### `FB_CTRL_MULTIPLE_PWM_OUT` — §4.2.1.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PWM_OUT_EXT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eState` | `E_CTRL_STATE` | State of the function block |
| `bError` | `BOOL` | Supplies the error number [} 173] when the output bError is set. |
| `eErrorId` | `E_CTRL_ERROR Becomes TRUE, as soon as an error occurs. CODES Inputs/ outputs` | — |
| `stParams` | `ST_CTRL_PWM_ Parameter structure of the PWM element OUT_EXT_ PARAMS pPwmInputArray POINTER TO` | Address of the PWM input array |
| `_ADR` | `FLOAT` | — |
| `nPwmInputArray UINT` | `Size of the PWM input array in bytes _SIZEOF pPwmTimesConfi POINTER TO` | Address of the PWM times array |
| `g_ADR` | `ST_CTRL_ MULTIPLE_PWM _OUT_TIMES` | — |
| `nPwmTimesConfi UINT` | `Size of the PWM times array in bytes g_SIZEOF pPwmOutputArra POINTER TO` | Address of the PWM output array |
| `y_ADR` | `ST_CTRL_ MULTIPLE_PWM _OUT_OUTPUTS` | — |
| `nPwmOutputArra UINT` | `Size of the PWM output array in bytes y_SIZEOF` | — |
| `pPwmData_ADR POINTER TO` | `Address of the internal PWM data array ST_CTRL_ MULTIPLE_PWM _OUT_DATA` | — |
| `nPwmData_SIZE UINT` | `Size of the internal PWM data array in bytes OF` | — |

**Description:** This function block creates PWM modulated output signals from a number of input signals in such a way that the temporal relationships between the output signals are arranged so that as few outputs as possible are switched on at any one time. This temporal arrangement reduces the maximum power necessary required for the actuators.

##### `FB_CTRL_PWM_OUT` — §4.2.1.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fPwmInput` | `FLOAT` | Input value |
| `eMode` | `E_CTRL_MODE PLC API` | Input that specifies the operation mode of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PWM_OUT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPwmOutBitPos` | `BOOL` | PWM signal, when fPwmInput > 0.0. |
| `bPwmOutBitNeg` | `BOOL` | PWM signal, when fPwmInput < 0.0. |
| `eState` | `E_CTRL_STATE` | State of the function block |
| `bError` | `BOOL` | Supplies the error number [} 173] when the output bError is set. |
| `eErrorId` | `E_CTRL_ERROR Becomes TRUE, as soon as an error occurs. CODES Inputs/ outputs` | — |
| `stParams` | `ST_CTRL_PWM_ Parameter structure of the PWM element OUT_PARAMS` | — |

**Description:** This function block creates a PWM modulated signal on the basis of the input signal.

##### `FB_CTRL_PWM_OUT_EXT` — §4.2.1.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fPwmInput` | `FLOAT` | Input value for the function block |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_PWM_OUT_EXT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPwmOutBitPos` | `BOOL` | PWM signal, when fPwmInput > 0.0 |
| `bPwmOutBitNeg` | `BOOL` | PWM signal, when fPwmInput < 0.0 |
| `eState` | `E_CTRL_STATE` | State of the function block |
| `bError` | `BOOL` | Supplies the error number [} 173] when the output bError is set. |
| `eErrorId` | `E_CTRL_ERROR Becomes TRUE, as soon as an error occurs. CODES Inputs/ outputs` | — |
| `stParams` | `ST_CTRL_PWM_ Parameter structure of the PWM element OUT_EXT_ PARAMS` | — |

**Description:** This function block creates a PWM modulated signal on the basis of the input signal. Both the minimum switch-on time and the minimum switch-off time can be parameterized, in addition to the mark-to-space ratio, in this extended function block.

##### `FB_CTRL_SCALE` — §4.2.1.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT PLC API` | Input value for the function block |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_SCALE_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Scaled output value |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_ SCALE_PARAMS` | Parameter structure of the function block |

**Description:** This function block makes it possible to adjust the signal in a linear manner between two value ranges.

##### `FB_CTRL_SERVO_MOTOR_OUT` — §4.2.1.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Control value of the controller over the range |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_SERVO_MOTOR_OUT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOutBitPos` | `BOOL` | Output required to drive the motor in a positive direction. |
| `bOutBitNeg` | `BOOL` | Output required to drive the motor in a negative direction. |
| `fActualState` | `FLOAT` | Current motor position in the range |

**Description:** This function block generates pulses with which a servomotor can be driven to a defined position.

##### `FB_CTRL_SPLITRANGE` — §4.2.1.7.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Input value for the function block |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_SPLITRANGE_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOutPos` | `FLOAT` | Positive part of fIn |
| `fOutNeg` | `FLOAT` | Negative part of fIn bOutPosIsActive BOOL TRUE indicates "fIn > 0.0" bOutNegIsActive BOOL TRUE indicates "fIn < 0.0" |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_SPLIT Parameter structure of the function block RANGE_ PARAMS` | — |

**Description:** This function block divides an input signal into positive and negative components. The parameters bDisablePosOut and bDisableNegOut can be used to deactivate the positive or negative output → heating mode only during the winter, cooling mode only during the summer. The bInvertNegOutput parameter allows the negative output to be inverted.

##### `FB_CTRL_STEPPING_MOTOR_OUT` — §4.2.1.7.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `FLOAT` | Controller's control value (controller output) |
| `bUpperLimit` | `BOOL` | Limit switch, becomes TRUE when the upper stop is reached. Switch |
| `bLowerLimit` | `BOOL` | Limit switch, becomes TRUE when the lower stop is reached. Switch |
| `fManSyncValue` | `FLOAT` | Input with which the internal state of the motor setting can be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_STEPPING_MOTOR_OUT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOutBitPos` | `BOOL` | Output required to drive the motor in a positive direction. |
| `bOutBitNeg` | `BOOL` | Output required to drive the motor in a negative direction. |
| `nActualStep` | `DINT` | Actual step at which the motor is positioned. |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_STEPP Parameter structure of the function block ING_MOTOR_ OUT_PARAMS` | — |

**Description:** This function block generates a control value for a stepper motor.

#### PLC API › Reference › Function blocks › Setpointgeneration

##### `FB_CTRL_3PHASE_SETPOINT_GENERATOR` — §4.2.1.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Setpoint generation begins when a positive edge appears at the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_3PHASE_SETPOINT_GENERATOR_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetPos` | `LREAL` | Set position |
| `fSetVelo` | `LREAL` | Set velocity |
| `fSetAcc` | `LREAL` | Set acceleration |
| `nSetDirection` | `INT` | Direction of movement [-1, 0, 1], |

**Description:** This function block represents a 3-phase setpoint generator.

##### `FB_CTRL_FLOW_TEMP_SETPOINT_GEN` — §4.2.1.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fAmbient` | `FLOAT` | Start of ramp generation Temperature |
| `fOffset` | `FLOAT` | Start value of the ramp Temperature |
| `bLimitFlowTemp` | `BOOL` | Target value of the ramp |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_FLOW_TEMP_SETPOINT_GEN_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Setpoint of the flow temperature |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `stParams` | `ST_CTRL_FLOW Parameter structure of the ramp generator _TEMP_ SETPOINT_GEN_ PARAMS` | — |

**Description:** This function block enables specification of a flow temperature depending on the outdoor temperature.

##### `FB_CTRL_RAMP_GENERATOR` — §4.2.1.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Start of ramp generation |
| `fStart` | `FLOAT` | Start value of the ramp |
| `fTarget` | `FLOAT` | Target value of the ramp |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_RAMP_GENERATOR_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the ramp generator |
| `fVeloOut` | `FLOAT` | Current velocity of the ramp generator |
| `bValueReached` | `BOOL` | This output indicates by going TRUE that the output fOut has |

**Description:** The function block provides a parameterizable ramp generator.

##### `FB_CTRL_RAMP_GENERATOR_EXT` — §4.2.1.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fStartValue` | `FLOAT` | Start value of the ramp |
| `fTargetValue` | `FLOAT` | Target value of the ramp |
| `fManValue` | `FLOAT` | Input value to which the output in eCTRL_MODE_MANUAL is set. |
| `bHold` | `BOOL` | Calculation of the ramp is halted at the current value. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_RAMP_GENERATOR_EXT_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the ramp generator |
| `fVeloOut` | `FLOAT` | Current velocity of the ramp generator |
| `bValueReached` | `BOOL` | This output indicates by going TRUE that the output fOut has |

**Description:** This function block represents a parameterizable ramp generator. In contrast to FB_CTRL_RAMP_GENERATOR it supports E_CTRL_MODE.

##### `FB_CTRL_SETPOINT_GENERATOR` — §4.2.1.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nSetpointIndex` | `INT` | Index of the selected setpoint |
| `fManValue` | `FLOAT` | Input whose value is output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_SETPOINT_GENERATOR_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `SETPOINT_ TABLE_ELEMEN T` | Output of the setpoint generator |
| `bValueReached` | `BOOL` | The output is TRUE when the selected setpoint has been reached, |

**Description:** The function block provides a setpoint generator that outputs the setpoint selected from a table. Changing from one setpoint to another can be implemented continuously or discontinuously.

##### `FB_CTRL_SIGNAL_GENERATOR` — §4.2.1.8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fManValue` | `FLOAT` | Input whose value is present at the output in manual mode. |
| `eMode` | `E_CTRL_MODE` | Input that specifies the operation mode [} 173] of the function block. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParams` | `ST_CTRL_SIGNAL_GENERATOR_PARAMS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `FLOAT` | Output of the signal generator |
| `eState` | `E_CTRL_STATE State of the function block` | — |
| `eErrorId` | `E_CTRL_ERROR Supplies the error number [} 173] when the output bError is set. CODES` | — |
| `bError` | `BOOL` | Becomes TRUE, as soon as an error occurs. Inputs/ outputs |
| `stParams` | `ST_CTRL_SIGNA Parameter structure of the function block L_ GENERATOR_PA RAMS PLC API` | — |

**Description:** The function block provides a signal generator offering triangular, sine and sawtooth signal forms.
