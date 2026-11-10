# Tc2_HVAC

- **Manual:** `TF8000_TC3_HVAC_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T0 · **API entries:** 120
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** HVAC function blocks (sequencing, economizer logic): mostly pure control logic over image IO; plant I/O via fieldbus.

---

#### Programming › POUs › HVAC Actuators

##### `FB_HVAC2PointActuator` — §5.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bIn` | `BOOL` | — |
| `eCtrlModeActuator` | `E_HVAC2PointActuatorMode` | — |
| `bManSwitch` | `BOOL` | — |
| `bLimitSwitchClose` | `BOOL` | — |
| `bLimitSwitchOpen` | `BOOL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled by the PLC program with the input variable bEnable. The actuator` | — |
| `bIn` | `In automatic mode the actuator is closed by a FALSE and opened by a TRUE.` | — |
| `eCtrlModeActuator` | `Enum that defines the operation mode (see E_HVAC2PointActuatorMode [} 487]).` | — |
| `bManSwitch` | `if the two-point drive has a manual/emergency switch in the control cabinet, this can be` | — |
| `bLimitSwitchClose` | `Actuator feedback TRUE when the actuator is completely closed.` | — |
| `bLimitSwitchOpen` | `Actuator feedback TRUE when the actuator is completely open.` | — |
| `bCtrlVoltage` | `The parameter bCtrlVoltage serves to check the control voltage. The control voltage is` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `bEnableLimitSwitch` | `BOOL` | — |
| `tStrokeTime` | `TIME` | — |
| `bEnableLimitSwitch` | `If the input is TRUE, then the function control of the drive is activated by means of the` | — |
| `tStrokeTime` | `In order to set the function control correctly, the stroke time of the drive from fully closed to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `byState` | `BYTE` | — |
| `eStateModeActuator` | `E_HVAC2PointActuatorMode` | — |
| `bErrorLimitSwitch` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOut` | `The actuator is connected to this output (FALSE = closing drive; TRUE = opening drive).` | — |
| `byState` | `Shows the status of the control of the actuator:` | — |
| `eStateModeActuator` | `Indicates which operation mode the actuator is in (see E_HVAC2PointActuatorMode` | — |
| `bErrorLimitSwitch` | `Goes TRUE if no limit switch is triggered after the preset stroke time. bErrorLimitSwitch` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** This function block serves to control two-point valves or two-point dampers.

##### `FB_HVAC3PointActuator` — §5.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bAutoOpen` | `BOOL` | — |
| `bAutoClose` | `BOOL` | — |
| `eCtrlModeActuator` | `E_HVAC3PointActuatorMode` | — |
| `bManSwitch` | `BOOL` | — |
| `rFeedb` | `REAL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled by the PLC program with the input variable bEnable. The three-point` | — |
| `eCtrlModeActuator` | `Enum that defines the operation mode (see E_HVAC3PointActuatorMode [} 487]).` | — |
| `bManSwitch` | `If the three-point actuator has a manual/emergency switch in the control cabinet, this can be` | — |
| `rFeedb` | `Analog position feedback from the actuator (0%..100%).` | — |
| `bCtrlVoltage` | `Parameter for controlling the control voltage. The control voltage is present if the bCtrlVoltage` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `bEnableFeedbCtrl` | `BOOL` | — |
| `rMinValueFeedb` | `REAL` | — |
| `rMaxValueFeedb` | `REAL` | — |
| `rHysteresisFeedb` | `REAL` | — |
| `tStrokeTime` | `TIME:` | — |
| `bEnableFeedbCtrl` | `If the input is TRUE, then the control of the feedback signal is activated. The variable is` | — |
| `rMinValueFeedb` | `Serves to scale the analog position feedback. rMinValueFeedb contains the value of the` | — |
| `rMaxValueFeedb` | `Serves to scale the analog position feedback. rMaxValueFeedb contains the value of the` | — |
| `rHysteresisFeedb` | `Due to the stroke time of the drive, the position feedback always lags in the case of a` | — |
| `tStrokeTime` | `Due to the lagging of the actual position in relation to the set position, the activation of the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOpen` | `BOOL` | — |
| `bClose` | `BOOL` | — |
| `byState` | `BYTE` | — |
| `eStateModeActuator` | `E_HVAC3PointActuatorMode` | — |
| `bErrorFeedb` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOpen` | `The signal for the opening drive of the three-point actuator is connected to this output.` | — |
| `bClose` | `The signal for the closing drive of the three-point actuator is connected to this output.` | — |
| `byState` | `Shows the status of the control of the actuator:` | — |
| `eStateModeActuator` | `Indicates which operation mode the actuator is in (see E_HVAC3PointActuatorMode` | — |
| `bErrorFeedb` | `When setting the output bClose, the position of the drive must have decreased by at least the` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** This function block serves to control three-point valves or three-point dampers, with or without constant position feedback. The function block is often used in conjunction with the FB_HVACAnalogTo3Point [} 76] function block.

##### `FB_HVACCirculationPump` — §5.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bAuto` | `BOOL` | — |
| `eCtrlModeActuator` | `E_HVACActuatorMode` | — |
| `rOutsideTemp` | `REAL` | — |
| `rValvePosition` | `REAL` | — |
| `bFeedbPump` | `BOOL` | — |
| `bPumpProtec` | `BOOL` | — |
| `bAntiBlocking` | `BOOL` | — |
| `bManSwitch` | `BOOL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE.` | — |
| `bAuto` | `Input for the request from the automatic program. The request is overridden by eCtrlModeActuator.` | — |
| `eCtrlModeActuator` | `Enum that defines the operation mode (see E_HVACActuatorMode [} 488]).` | — |
| `rOutsideTemp` | `Input for transmitting the outside temperature value.` | — |
| `rValvePosition` | `If there is a control valve in the hydraulic circuit of the pump, the position of the control` | — |
| `bFeedbPump` | `This input is for displaying the operating message in a visualization system and for` | — |
| `bPumpProtec` | `A pump error message is connected to the input bPumpProtec. There is a pump error if the` | — |
| `bAntiBlocking` | `Input for transferring the anti-blocking request, i.e. if TRUE the request is active.` | — |
| `bManSwitch` | `If the pump has a manual/emergency switch in the control cabinet, this can be connected to` | — |
| `bCtrlVoltage` | `In order to suppress a torrent of messages, the error message from bPumpProtec is only` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `tStopDelay` | `TIME` | — |
| `tFeedbPumpDelay` | `TIME` | — |
| `rOutsideTempLimit` | `REAL` | — |
| `rValvePositionLimit` | `REAL` | — |
| `eReqOutsideTemp` | `E_HVACReqOutsideTemp` | — |
| `eReqValve` | `E_HVACReqValve` | — |
| `tStopDelay` | `The time tStopDelay [s] delays the switching off of the pump after the switch-on conditions are` | — |
| `tFeedbPumpDelay` | `The monitoring function of the pump feedback message [s] is only active if` | — |
| `rOutsideTempLimit` | `Value [°C] above or below which the pump is switched on or off depending on the` | — |
| `rValvePositionLimit` | `Threshold value for the position of a control valve associated with a pump, from which` | — |
| `eReqOutsideTemp` | `Depending on the outside temperature, the pump can be compulsorily switched on, e.g.` | — |
| `eReqValve` | `Depending on the position of the valve associated with the pump, the pump can be switched on` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPump` | `BOOL` | — |
| `byState` | `BYTE` | — |
| `bStateManSwitch` | `BOOL` | — |
| `bReqOutsideTemp` | `BOOL` | — |
| `bReqValve` | `BOOL` | — |
| `eStateModeActuator` | `E_HVACActuatorMode` | — |
| `bErrorPumpProtec` | `BOOL` | — |
| `bErrorPumpFeedb` | `BOOL` | — |
| `bErrorGeneral` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bPump` | `Output variable for controlling a pump.` | — |
| `byState` | `Output of the pump status as a byte.` | — |
| `bStateManSwitch` | `Indicates the state of the manual/emergency switch. A TRUE signals that the manual/` | — |
| `bReqOutsideTemp` | `If the condition for requesting the pump depending on the outside temperature is` | — |
| `bReqValve` | `If the condition for requesting the pump depending on the valve position is reached, the variable` | — |
| `eStateModeActuator` | `Indicates which operation mode the actuator is in (see E_HVACActuatorMode` | — |
| `bErrorPumpProtec` | `Pump error.` | — |
| `bErrorPumpFeedb` | `If the input bFeedbPump does not go TRUE within the time of tFeedbPumpDelay` | — |
| `bErrorGeneral` | `There is a general error.` | — |
| `byError` | `Output of the error as a byte.` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** This function block serves to control pumps in HVAC systems.

##### `FB_HVACCirculationPumpEx` — §5.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bExternal` | `BOOL` | — |
| `eCtrlModeActuator` | `E_HVACActuatorMode` | — |
| `rOutsideTemp` | `REAL` | — |
| `rValvePosition` | `REAL` | — |
| `bFeedbPump` | `BOOL` | — |
| `bPumpProtec` | `BOOL` | — |
| `bManSwitch` | `BOOL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If bEnable = FALSE, then the output bPump is` | — |
| `bExternal` | `The output bPump can be switched on or off directly via the input bExternal. The following` | — |
| `eCtrlModeActuator` | `Enum that specifies the operation mode. The following operation modes are supported` | — |
| `rOutsideTemp` | `Input for transmitting the outside temperature value. Depending on the outside temperature,` | — |
| `rValvePosition` | `Input for transmitting the valve position of the control loop. Depending on the position` | — |
| `bFeedbPump` | `Feedback from the pump or a relay contact that the pump is switched on. If the output` | — |
| `bPumpProtec` | `The motor protection switch for the pump is connected to the input bPumpProtec. A pump` | — |
| `bManSwitch` | `If the pump has a manual / emergency switch, this can be connected to the input bManSwitch.` | — |
| `bCtrlVoltage` | `The control voltage monitoring is applied to the input bCtrlVoltage. If bCtrlVoltage = FALSE,` | — |
| `bReset` | `Acknowledgment input in the case of a fault following its rectification.` | — |
| `udiSecStopDelay` | `UDINT` | — |
| `udiSecFeedbPumpDelay` | `UDINT` | — |
| `rOutsideTempHighLimit` | `REAL` | — |
| `rOutsideTempLowLimit` | `REAL` | — |
| `rValvePositionLimitOn` | `REAL` | — |
| `rValvePositionLimitOff` | `REAL` | — |
| `eReqPump` | `E_HVACReqPump` | — |
| `udiSecStopDelay` | `The time delay udiSecStopDelay [s] delays the switching off of the pump after the switch-` | — |
| `udiSecFeedbPumpDelay` | `If the output bPump = TRUE, then the input bFeedbPump must be TRUE within` | — |
| `rOutsideTempHighLimit` | `Value above which the pump is switched on or off depending on the outside` | — |
| `rOutsideTempLowLimit` | `Value below which the pump is switched on or off depending on the outside` | — |
| `rValvePositionLimitOn` | `Threshold value for the position rValvePosition of a control valve associated with a` | — |
| `rValvePositionLimitOff` | `Threshold value for the position rValvePosition of a control valve associated with a` | — |
| `eReqPump` | `Using the Enum E_HVACReqPump [} 491], switch-on conditions or combinations of switch-on` | — |
| `eHVACReqPump_No` | `There is no request on the part of the Enum to control the pump` | — |
| `eHVACReqPump_OT_LL` | `The outside temperature (OT = rOutsideTemp) must be smaller than` | — |
| `eHVACReqPump_OT_HL` | `The outside temperature (OT = rOutsideTemp) must be larger than` | — |
| `eHVACReqPump_VP` | `The valve position (VP = rValvePosition) must be larger than rValvePositionLimitOn` | — |
| `eHVACReqPump_OT_LL_OR_VP` | `The outside temperature (OT = rOutsideTemp) must be smaller than` | — |
| `eHVACReqPump_OT_HL_OR_VP` | `The outside temperature (OT = rOutsideTemp) must be larger than` | — |
| `eHVACReqPump_OT_LL_AND_VP` | `The outside temperature (OT = rOutsideTemp) must be smaller than` | — |
| `eHVACReqPump_OT_HL_AND_VP` | `The outside temperature (OT = rOutsideTemp) must be larger than` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bPump` | `BOOL` | — |
| `byState` | `BYTE` | — |
| `bReqExternal` | `BOOL` | — |
| `bReqOutsideTemp` | `BOOL` | — |
| `bReqValve` | `BOOL` | — |
| `bStateManSwitch` | `BOOL` | — |
| `udiSecRT_StopDelay` | `UDINT` | — |
| `udiSecRT_FeedbPumpDelay` | `UDINT` | — |
| `eStateModeActuator` | `E_HVACActuatorMode` | — |
| `bError` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bErrorPumpProtec` | `BOOL` | — |
| `bErrorFeedb` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bPump` | `Output variable for controlling a pump. The following conditions must first be fulfilled in order to` | — |
| `byState` | `Pump status output` | — |
| `bReqExternal` | `If the condition bExternal = TRUE is fulfilled for switching on the pump via bPump, then the` | — |
| `bReqOutsideTemp` | `If the condition is achieved for requesting the pump via bPump depending on the` | — |
| `bReqValve` | `If the condition is achieved for requesting the pump via bPump depending on the valve position` | — |
| `bStateManSwitch` | `Indicates the state of the manual/emergency switch. A TRUE signals that the manual/` | — |
| `udiSecRT_StopDelay` | `If the operation mode eHVACActuatorMode_Auto_BMS OR` | — |
| `udiSecRT_FeedbPumpDelay` | `If the output bPump = TRUE, then the input bFeedbPump must be TRUE` | — |
| `eStateModeActuator` | `Indicates the operation mode of the function block. eStateModeActuator is equivalent` | — |
| `bError` | `bError goes TRUE if bErrorPumpProtec is TRUE. However, only the error bErrorPumpProtec leads` | — |
| `byError` | `Output of the error as a byte.` | — |
| `bErrorPumpProtec` | `The motor protection switch for the pump is connected to the input bPumpProtec.` | — |
| `bErrorFeedb` | `If the output bPump = TRUE, then the input bFeedbPump must be TRUE within the time` | — |
| `bInvalidParameter` | `Goes TRUE if an error occurs during the plausibility check of the following variables:` | — |

**Description:** This function block serves to control pumps in HVAC systems.

##### `FB_HVACMotor1Speed` — §5.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bAuto` | `BOOL` | — |
| `eCtrlModeActuator` | `E_HVACActuatorMode` | — |
| `bMotorProtec` | `BOOL` | — |
| `bFeedbContactor` | `BOOL` | — |
| `bFeedbProcess` | `BOOL` | — |
| `bRepairSwitch` | `BOOL` | — |
| `bManSwitch` | `BOOL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If bEnable = FALSE, the drive will be switched` | — |
| `bAuto` | `bAuto is then only active if the operation mode eCtrlModeActuator is set either to` | — |
| `eCtrlModeActuator` | `Enum that specifies the operation mode of the motor (see E_HVACActuatorMode` | — |
| `bMotorProtec` | `Input for the motor protection switch. There is a motor protection switch error if the input` | — |
| `bFeedbContactor` | `Feedback for the power section of the motor. The operating feedback is present if the` | — |
| `bFeedbProcess` | `A process feedback signal, for example from a v-belt monitor or a flow monitor can be` | — |
| `bRepairSwitch` | `The state of the repair switch is monitored with the input bRepairSwitch. The motor can only` | — |
| `bManSwitch` | `The state of the manual/emergency switch is monitored with the input bManSwitch. The motor` | — |
| `bCtrlVoltage` | `The state of the control voltage is monitored with the input bCtrlVoltage. The motor can only` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `tStartDelay` | `TIME` | — |
| `tStopDelay` | `TIME` | — |
| `tDelayFeedbContactor` | `TIME` | — |
| `tFeedbProcessTimer` | `TIME` | — |
| `tDelayFeedbProcess` | `TIME` | — |
| `tStartDelay` | `The start-up of the motor after enabling and switching on via the operation mode of the motor is` | — |
| `tStopDelay` | `The switching off of the motor via the operation mode of the motor is delayed by the time` | — |
| `tDelayFeedbContactor` | `Time delay [ms] of the feedback of the power section after switching on the motor.` | — |
| `tFeedbProcessTimer` | `Time delay of the process feedback bFeedbProcess [s] after switching on the motor.` | — |
| `tDelayFeedbProcess` | `In order to avoid undesired switching off of the drive during operation due to the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bMotor` | `BOOL` | — |
| `byState` | `BYTE` | — |
| `bStateAuto` | `BOOL` | — |
| `bStateRepairSwitch` | `BOOL` | — |
| `bStateManSwitch` | `BOOL` | — |
| `eStateModeActuator` | `E_HVACActuatorMode` | — |
| `bErrorGeneral` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bErrorMotorprotec` | `BOOL` | — |
| `bErrorFeedbContactor` | `BOOL` | — |
| `bErrorFeedbProcess` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bMotor` | `Output variable for controlling a single-speed motor.` | — |
| `byState` | `Status byte indicating the operating state of the function block.` | — |
| `bStateAuto` | `Indicates the state for automatic preselection if the operation mode eCtrlModeActuator [} 488]` | — |
| `bStateRepairSwitch` | `Indicates the state of the repair switch. TRUE indicates that the repair switch is` | — |
| `bStateManSwitch` | `Indicates the state of the manual/emergency switch. A TRUE signals that the manual/` | — |
| `eStateModeActuator` | `Enum via which the state of the operation mode of the motor is fed back to the` | — |
| `bErrorGeneral` | `The error message bErrorGeneral becomes TRUE as soon as one of the error messages` | — |
| `byError` | `Delivers all error messages and warnings from the function block.` | — |
| `bErrorMotorprotec` | `Motor protection switch error, see input variable bMotorProtec.` | — |
| `bErrorFeedbContactor` | `Power section feedback error, see input variable bFeedbContactor.` | — |
| `bErrorFeedbProcess` | `Process feedback error, see input variable bFeedbackProcess.` | — |
| `bInvalidParameter` | `Indicates that an incorrect parameter is present at one of the variables` | — |

**Description:** This function block serves to control a single-stage drive in HVAC systems. It is suitable for fans.

##### `FB_HVACMotor2Speed` — §5.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bAutoSpeed1` | `BOOL` | — |
| `bAutoSpeed2` | `BOOL` | — |
| `eCtrlModeActuator` | `E_HVACActuatorMode` | — |
| `bMotorProtecSpeed1` | `BOOL` | — |
| `bMotorProtecSpeed2` | `BOOL` | — |
| `bFeedbContactorSpeed1` | `BOOL` | — |
| `bFeedbContactorSpeed2` | `BOOL` | — |
| `bFeedbProcess` | `BOOL` | — |
| `bRepairSwitch` | `BOOL` | — |
| `bManSwitch` | `BOOL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If bEnable = FALSE, the drive will be switched` | — |
| `bAutoSpeed1` | `bAutoSpeed1 is then only active if the operation mode eCtrlModeActuator is either` | — |
| `bAutoSpeed2` | `bAutoSpeed2 is then only active if the operation mode eCtrlModeActuator is either` | — |
| `eCtrlModeActuator` | `Enum that specifies the operation mode of the motor (see E_HVACActuatorMode` | — |
| `bMotorProtecSpeed1` | `Input for the motor protection speed 1. There is a motor protection error if the input` | — |
| `bMotorProtecSpeed2` | `Input for the motor protection speed 2. There is a motor protection error if the input` | — |
| `bFeedbContactorSpeed1` | `Feedback from the power section of the motor for speed 1. The operating` | — |
| `bFeedbContactorSpeed2` | `Feedback from the power section of the motor for speed 2. The operating` | — |
| `bFeedbProcess` | `A process feedback signal, for example from a v-belt monitor or a flow monitor can be` | — |
| `bRepairSwitch` | `The state of the repair switch is monitored with the input bRepairSwitch. The motor can only` | — |
| `bManSwitch` | `The state of the manual/emergency switch is monitored with the input bManSwitch. The motor` | — |
| `bCtrlVoltage` | `The state of the control voltage is monitored with the input bCtrlVoltage. The motor can only` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `tStartDelay` | `TIME` | — |
| `tStopDelay` | `TIME` | — |
| `tDelayFeedbContactorSpeed1` | `TIME` | — |
| `tDelayFeedbContactorSpeed2` | `TIME` | — |
| `tFeedbProcessTimer` | `TIME` | — |
| `tDelayFeedbProcess` | `TIME` | — |
| `tDelaySpeed1` | `TIME` | — |
| `tDelaySpeed1ToSpeed2` | `TIME` | — |
| `tDelaySpeed2ToSpeed1` | `TIME` | — |
| `tStartDelay` | `The start-up of the motor after enabling and switching on via the operation mode of the motor is` | — |
| `tStopDelay` | `The switch-off of the motor in the respective switch-on stage, either by switching the enable` | — |
| `tDelayFeedbContactorSpeed1` | `Time delay of the feedback of the power section after switching on the` | — |
| `tDelayFeedbContactorSpeed2` | `Time delay of the feedback of the power section after switching on the` | — |
| `tFeedbackProcessTimer` | `Time delay [s] of the process feedback bFeedbProcess during the motor start-up` | — |
| `tDelayFeedbProcess` | `In order to avoid undesired switching off of the drive during operation due to the` | — |
| `tDelaySpeed1` | `Time delay [s] for the start-up phase of the motor in speed 1 (1s..3600s). The variable is` | — |
| `tDelaySpeed1ToSpeed2` | `Time delay for the motor switchover phase from speed 1 to speed 2, so that both` | — |
| `tDelaySpeed2ToSpeed1` | `Time delay [s] for the motor switchover phase from speed 2 to speed 1` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bMotorSpeed1` | `BOOL` | — |
| `bMotorSpeed2` | `BOOL` | — |
| `byState` | `BYTE` | — |
| `bStateAutoSpeed1` | `BOOL` | — |
| `bStateAutoSpeed2` | `BOOL` | — |
| `bStateRepairSwitch` | `BOOL` | — |
| `bStateManSwitch` | `BOOL` | — |
| `eStateModeActuator` | `E_HVACActuatorMode` | — |
| `bErrorGeneral` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bErrorMotorprotecSpeed1` | `BOOL` | — |
| `bErrorMotorprotecSpeed2` | `BOOL` | — |
| `bErrorFeedbContactorSpeed1` | `BOOL` | — |
| `bErrorFeedbContactorSpeed2` | `BOOL` | — |
| `bErrorFeedbProcess` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bMotorSpeed1` | `Output variable for controlling speed 1 of the two-speed drive.` | — |
| `bMotorSpeed2` | `Output variable for controlling speed 2 of the two-speed drive.` | — |
| `byState` | `Status byte indicating the operating state of the function block` | — |
| `bStateAutoSpeed1` | `Indicates the state for automatic preselection of speed 1 if the operation mode` | — |
| `bStateAutoSpeed2` | `Indicates the state for automatic preselection of speed 2 if the operation mode` | — |
| `bStateRepairSwitch` | `Indicates the state of the repair switch. TRUE indicates that the repair switch is` | — |
| `bStateManSwitch` | `Indicates the state of the manual/emergency switch. A TRUE signals that the manual/` | — |
| `eStateModeActuator` | `Enum via which the state of the operation mode of the motor is fed back to the` | — |
| `bErrorGeneral` | `The error message bErrorGeneral becomes TRUE as soon as one of the error messages` | — |
| `byError` | `Delivers all error messages and warnings from the function block.` | — |
| `bErrorMotorprotecSpeed1` | `Motor protection error, see input variable bMotorProtecSpeed1` | — |
| `bErrorMotorprotecSpeed2` | `Motor protection error, see input variable bMotorProtecSpeed2` | — |
| `bErrorFeedbContactorSpeed1` | `Power section feedback error, see input variable bFeedbContactorSpeed1` | — |
| `bErrorFeedbContactorSpeed2` | `Power section feedback error, see input variable bFeedbContactorSpeed2` | — |
| `bErrorFeedbProcess` | `Process feedback error, see input variable bFeedbackProcess` | — |
| `bInvalidParameter` | `Indicates that an incorrect parameter is present at one of the variables` | — |

**Description:** This function block serves to control a two-stage drive in HVAC systems. The function block always runs in stage 1, the lower power stage. It cannot be switched on directly in stage 2. In the event of a restart, disablement, an error or switching off of the motor via the operation mode, restart of the motor is blocked for the duration of tDelaySpeed2toSpeed1.

##### `FB_HVACMotor3Speed` — §5.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bAutoSpeed1` | `BOOL` | — |
| `bAutoSpeed2` | `BOOL` | — |
| `bAutoSpeed3` | `BOOL` | — |
| `eCtrlModeActuator` | `E_HVACActuatorMode` | — |
| `bMotorProtecSpeed1` | `BOOL` | — |
| `bMotorProtecSpeed2` | `BOOL` | — |
| `bMotorProtecSpeed3` | `BOOL` | — |
| `bFeedbContactorSpeed1` | `BOOL` | — |
| `bFeedbContactorSpeed2` | `BOOL` | — |
| `bFeedbContactorSpeed3` | `BOOL` | — |
| `bFeedbProcess` | `BOOL` | — |
| `bRepairSwitch` | `BOOL` | — |
| `bManSwitch` | `BOOL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |
| `arrFiFo` | `Array[1..8] of INT` | — |
| `arrFiFo` | `Contains the assignment table with a maximum of eight values. The first value indicates where the` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If bEnable = FALSE, the drive will be switched` | — |
| `bAutoSpeed1` | `bAutoSpeed1 is then only active if the operation mode eCtrlModeActuator [} 488] is either` | — |
| `bAutoSpeed2` | `bAutoSpeed2 is then only active if the operation mode eCtrlModeActuator [} 488] is either` | — |
| `bAutoSpeed3` | `bAutoSpeed3 is then only active if the operation mode eCtrlModeActuator [} 488] is either` | — |
| `eCtrlModeActuator` | `Enum that specifies the operation mode of the motor (see E_HVACActuatorMode` | — |
| `bMotorProtecSpeed1` | `Input for the motor protection speed 1. There is a motor protection error if the input` | — |
| `bMotorProtecSpeed2` | `Input for the motor protection speed 2. There is a motor protection error if the input` | — |
| `bMotorProtecSpeed3` | `Input for the motor protection speed 3. There is a motor protection error if the input` | — |
| `bFeedbContactorSpeed1` | `Feedback from the power section of the motor for speed 1. The operating` | — |
| `bFeedbContactorSpeed2` | `Feedback from the power section of the motor for speed 2. The operating` | — |
| `bFeedbContactorSpeed3` | `Feedback from the power section of the motor for speed 3. The operating` | — |
| `bFeedbProcess` | `A process feedback signal, for example from a v-belt monitor or a flow monitor can be` | — |
| `bRepairSwitch` | `The state of the repair switch is monitored with the input bRepairSwitch. The motor can only` | — |
| `bManSwitch` | `The state of the manual/emergency switch is monitored with the input bManSwitch. The motor` | — |
| `bCtrlVoltage` | `The state of the control voltage is monitored with the input bCtrlVoltage. The motor can only` | — |
| `bReset` | `Input for acknowledgement of faults via a rising edge.` | — |
| `tStartDelay` | `TIME` | — |
| `tStopDelay` | `TIME` | — |
| `tDelayFeedbContactorSpeed1` | `TIME` | — |
| `tDelayFeedbContactorSpeed2` | `TIME` | — |
| `tDelayFeedbContactorSpeed3` | `TIME` | — |
| `tFeedbProcessTimer` | `TIME` | — |
| `tDelayFeedbProcess` | `TIME` | — |
| `tDelaySpeed1` | `TIME` | — |
| `tDelaySpeed2` | `TIME` | — |
| `tDelaySpeed1ToSpeed2ToSpeed3` | `TIME` | — |
| `tDelaySpeed2ToSpeed1` | `TIME` | — |
| `tDelaySpeed3ToSpeed2` | `TIME` | — |
| `tStartDelay` | `The start-up of the motor after enabling and switching on via the operation mode of the motor is` | — |
| `tStopDelay` | `The switching off of the motor in the respective switch-on stage, either by switching the enable` | — |
| `tDelayFeedbContactorSpeed1` | `Time delay of the feedback of the power section after switching on the` | — |
| `tDelayFeedbContactorSpeed2` | `Time delay of the feedback of the power section after switching on the` | — |
| `tDelayFeedbContactorSpeed3` | `Time delay of the feedback of the power section after switching on the` | — |
| `tFeedbackProcessTimer` | `Time delay [s] of the process feedback bFeedbProcess during the motor start-up` | — |
| `tDelayFeedbProcess` | `In order to avoid undesired switching off of the drive during operation due to the` | — |
| `tDelaySpeed1` | `Time delay [s] for the start-up phase of the motor in speed 1 (1s..3600s). The variable is` | — |
| `tDelaySpeed2` | `Time delay [s] for the start-up phase of the motor in speed 2 (1s..3600s). The variable is` | — |
| `tDelaySpeed1ToSpeed2ToSpeed3` | `Time delay for the motor switchover phase from speed 1 to speed 2, or` | — |
| `tDelaySpeed2ToSpeed1` | `Time delay [s] for the motor switchover phase from speed 2 to speed 1` | — |
| `tDelaySpeed3ToSpeed2` | `Time delay [s] for the motor switchover phase from speed 3 to speed 2` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bMotorSpeed1` | `BOOL` | — |
| `bMotorSpeed2` | `BOOL` | — |
| `bMotorSpeed3` | `BOOL` | — |
| `wState` | `WORD` | — |
| `bStateAutoSpeed1` | `BOOL` | — |
| `bStateAutoSpeed2` | `BOOL` | — |
| `bStateAutoSpeed3` | `BOOL` | — |
| `bStateRepairSwitch` | `BOOL` | — |
| `bStateManSwitch` | `BOOL` | — |
| `eStateModeActuator` | `E_HVACActuatorMode` | — |
| `bErrorGeneral` | `BOOL` | — |
| `wError` | `WORD` | — |
| `bErrorMotorprotecSpeed1` | `BOOL` | — |
| `bErrorMotorprotecSpeed2` | `BOOL` | — |
| `bErrorMotorprotecSpeed3` | `BOOL` | — |
| `bErrorFeedbContactorSpeed1` | `BOOL` | — |
| `bErrorFeedbContactorSpeed2` | `BOOL` | — |
| `bErrorFeedbContactorSpeed3` | `BOOL` | — |
| `bErrorFeedbProcess` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bMotorSpeed1` | `Output variable for controlling speed 1 of the three-speed drive.` | — |
| `bMotorSpeed2` | `Output variable for controlling speed 2 of the three-speed drive.` | — |
| `bMotorSpeed3` | `Output variable for controlling speed 3 of the three-speed drive.` | — |
| `wState` | `Status word indicating the operating state of the function block.` | — |
| `bStateAutoSpeed1` | `Indicates the state for automatic preselection of speed 1 if the operation mode` | — |
| `bStateAutoSpeed2` | `Indicates the state for automatic preselection of speed 2 if the operation mode` | — |
| `bStateAutoSpeed3` | `Indicates the state for automatic preselection of speed 3 if the operation mode` | — |
| `bStateRepairSwitch` | `Indicates the state of the repair switch. TRUE indicates that the repair switch is` | — |
| `bStateManSwitch` | `Indicates the state of the manual/emergency switch. A TRUE signals that the manual/` | — |
| `eStateModeActuator` | `Enum via which the operation mode status of the motor is fed back to the controller.` | — |
| `bErrorGeneral` | `The error message bErrorGeneral becomes TRUE as soon as one of the error messages` | — |
| `wError` | `Delivers all error messages and warnings from the function block.` | — |
| `bErrorMotorprotecSpeed1` | `Motor protection error, see input variable bMotorProtecSpeed1.` | — |
| `bErrorMotorprotecSpeed2` | `Motor protection error, see input variable bMotorProtecSpeed2.` | — |
| `bErrorMotorprotecSpeed3` | `Motor protection error, see input variable bMotorProtecSpeed3.` | — |
| `bErrorFeedbContactorSpeed1` | `Power section feedback error, see input variable bFeedbContactorSpeed1.` | — |
| `bErrorFeedbContactorSpeed2` | `Power section feedback error, see input variable bFeedbContactorSpeed2` | — |
| `bErrorFeedbContactorSpeed3` | `Power section feedback error, see input variable bFeedbContactorSpeed3` | — |
| `bErrorFeedbProcess` | `Process feedback error, see input variable bFeedbackProcess.` | — |
| `bInvalidParameter` | `Indicates that an incorrect parameter is present at one of the variables` | — |
| `bErr` | `BOOL` | — |
| `bErr` | `Value range in the FiFo table was exceeded; 0 to 8 is permissible.` | — |

**Description:** This function block serves to control a three-speed drive in HVAC systems. The function block always runs in stage 1, the lowest power stage, and switches to stage 2 or stage 3 depending on requirements. It cannot be switched on directly in stage 2 or 3. In the event of a restart, disablement, an error or switching off of the motor via the operation mode, restart of the motor is blocked for the duration of tDelaySpeed2ToSpeed1 + tDelaySpeed3ToSpeed2.

##### `FB_HVACMux8Ex` — §5.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `arrFiFo` | `Array[1..8] of INT` | — |
| `arrFiFo` | `Contains the assignment table with a maximum of eight values. The first value indicates where the` | — |

**Description:** This function block evaluates the FIFO memory from FB_HVACRedundancyCtrlEx [} 65]. The inputs are mapped to the respective outputs according to a FIFO table.

##### `FB_HVACMux8_BOOL` — §5.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `arrFiFo` | `Array[1..8] of INT` | — |
| `arrFiFo` | `Contains the assignment table with a maximum of eight values. The first value indicates where the` | — |

**Description:** This function block evaluates the FIFO memory from FB_HVACRedundancyCtrlEx [} 65]. The inputs are mapped to the respective outputs according to a FIFO table.

##### `FB_HVACRedundancyCtrl` — §5.1.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `iCountCtrl` | `INT` | — |
| `bForceChange` | `BOOL` | — |
| `dtSystemTime` | `DT` | — |
| `bSysTakt1Hz` | `BOOL` | — |
| `bResetAllHours` | `BOOL` | — |
| `bResetAllCounter` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `iCountCtrl` | `Number of actuators to be switched on (0..8).` | — |
| `bForceChange` | `A positive edge at the input switches off the first actuator in the FIFO and switches on the` | — |
| `dtSystemTime` | `System time.` | — |
| `bSysTakt1Hz` | `1 Hz clock signal as a replacement for dtSystemTime; if dtSystemTime is not available, or` | — |
| `bResetAllHours` | `Resets all operating hour meters.` | — |
| `bResetAllCounter` | `Resets all switch-on meters.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rHours` | `REAL` | — |
| `uiCount` | `UINT` | — |
| `tOverlap1Actuator` | `TIME` | — |
| `uiMaxOnLevel` | `UINT` | — |
| `tNoFeedbActuator` | `TIME` | — |
| `tOverlap1Actuator` | `Overlap time for the case where an exchange between two actuators takes place` | — |
| `uiMaxOnLevel` | `Max. time in hours [h] that an actuator may be switched on (0h..1000h). Forces an actuator` | — |
| `tNoFeedbActuator` | `Time that is allowed to elapse before the lack of an operating feedback from the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bErrorGeneral` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `eErrorCode` | `Indicates which actuator has not returned an operating feedback within the prespecified time` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator1` | `= 15,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator2` | `= 16,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator3` | `= 17,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator4` | `= 18,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator5` | `= 19,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator6` | `= 20,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator7` | `= 21,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator8` | `= 22,` | — |
| `bErrorGeneral` | `Error during evaluation of the operating feedback.` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** This function block serves to control a certain number of actuators, e.g. of pumps, from a pool of 8 actuators. From all enabled actuators, the logic searches for those with the shortest runtimes and switches them on according to the runtime order. This continues until the number specified via iCountCtrl is reached. The actuators are routed internally via a FIFO memory so that they can be switched off again in the same order. Changeover during operation can be performed with the time specification uiMaxOnLevel or by the trigger bForceChange. The actuator that has been switched on longest is switched off, and the one with the shortest runtime is switched on. In order to avoid a hydraulic shock in the pumps, an overlap time can be set via tOverlap1Actuator. This applies only to cases where an exchange between two actuators takes place. In order to determine the runtimes, the function block FB_HVACWork [} 458] is instanced internally and the IN-OUT variables are relayed to the outside (rHours and uiCount). The recording of the operating time is controlled by the Feedback inputs. If no feedback signal from the actuator is available, the actuator output must be fed back to the feedback input.

##### `FB_HVACRedundancyCtrlEx` — §5.1.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `iCountCtrl` | `INT` | — |
| `bForceChange` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `iCountCtrl` | `Number of actuators to be switched on (0..8). A change in the sequence of the actuators to be` | — |
| `bForceChange` | `A positive edge at the input switches off the first actuator in the FIFO and switches on the` | — |
| `rHours` | `Operating hours` | — |
| `tOverlap1Actuator` | `TIME` | — |
| `uiMaxOnLevel` | `UINT` | — |
| `tNoFeedbActuator` | `TIME` | — |
| `tOverlap1Actuator` | `Overlap time for the case where an exchange between two actuators takes place` | — |
| `uiMaxOnLevel` | `Max. time in hours that an actuator may be switched on (0h..1000h). Forces an actuator` | — |
| `tNoFeedbActuator` | `Time that is allowed to elapse before the lack of an operating feedback from the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrFiFo` | `Array[1..8] of INT` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bErrorGeneral` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `arrFiFo` | `Table containing the information showing which actuator is switched on (actuator number). The` | — |
| `eErrorCode` | `Indicates which actuator has not returned an operating feedback within the prespecified time` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator1` | `= 15,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator2` | `= 16,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator3` | `= 17,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator4` | `= 18,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator5` | `= 19,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator6` | `= 20,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator7` | `= 21,` | — |
| `eHVACErrorCodes_Error_NoFeedbackActuator8` | `= 22,` | — |
| `bErrorGeneral` | `Error during evaluation of the operating feedback.` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** This function block serves to control a certain number of actuators, e.g. of pumps, from a pool of 8 actuators. From all enabled actuators, the logic searches for those with the shortest runtimes and switches them on according to the runtime order. This continues until the number specified via iCountCtrl is reached. The actuators are routed internally via a FIFO memory so that they can be switched off again in the same order. Changeover during operation can take place with the time specification uiMaxOnLevel, a change of iCountCtrl or by the trigger bForceChange. The actuator that has been switched on longest is switched off, and the one with the shortest runtime is switched on. In order to avoid a hydraulic stroke in the pumps, an overlap time can be set via tOverlap1Actuator. This applies only to cases where an exchange between two actuators takes place. In contrast to FB_HVACRedundancyCtrl [} 62] no internal timer is used for determining the runtimes. Instead, the times have to be specified externally as hourly values via Var_Input.

#### Programming › POUs › HVAC Analog modules

##### `FB_HVACAnalogInput` — §5.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `iRawValue` | `INT` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `bEnableLimitCtrl` | `BOOL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `iRawValue` | `The raw value is transferred from the terminal to the function block with the parameter` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `Cycle time with which the function block is processed. This must be greater than or equal to` | — |
| `bEnableLimitCtrl` | `Limit value monitoring is only activated if the variable bEnableLimitCtrl is TRUE. This` | — |
| `bCtrlVoltage` | `By means of checking the sensor supply voltage with the input bCtrlVoltage, error messages` | — |
| `bReset` | `Acknowledge input in the event of an error. In addition the 2nd order filter can be synchronized via` | — |
| `iHighRange` | `INT` | — |
| `iLowRange` | `INT` | — |
| `rHighRange` | `REAL` | — |
| `rLowRange` | `REAL` | — |
| `rHighLimit` | `REAL` | — |
| `rLowLimit` | `REAL` | — |
| `tFilterTime` | `TIME` | — |
| `rOffset` | `REAL` | — |
| `iHighRange` | `Upper raw value of the input variable iRawValue. The variable is saved persistently. Preset to` | — |
| `iLowRange` | `Lower raw value of the input variable iRawValue. The variable is saved persistently. Preset to` | — |
| `rHighRange` | `The upper scaled measured value. The variable is saved persistently. Preset to 100.` | — |
| `rLowRange` | `The lower scaled measured value. The variable is saved persistently. Preset to 0.` | — |
| `rHighLimit` | `If the scaled measured value is larger than the upper limit value rHighLimit, an impermissibly` | — |
| `rLowLimit` | `If the scaled measured value is smaller than the lower limit value rLowLimit, an impermissibly` | — |
| `tFilterTime` | `To avoid large fluctuations and jumps in the measuring signal, the function block is provided` | — |
| `rOffset` | `With this offset the linear equation determined by means of the two conversion points is shifted` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rPresentValue` | `REAL` | — |
| `byState` | `BYTE` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bErrorHighLimit` | `BOOL` | — |
| `bErrorLowLimit` | `BOOL` | — |
| `bErrorGeneral` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rPresentValue` | `Determined output value.` | — |
| `byState` | `State of the function block.` | — |
| `bError` | `The variable bError changes to TRUE in the event of an internal error in the function block.` | — |
| `eErrorCode` | `Contains the specific error code related to the output bError (see E_HVACErrorCodes [} 490]).` | — |
| `bErrorHighLimit` | `TRUE if the upper limit value has been reached.` | — |
| `bErrorLowLimit` | `TRUE if the lower limit value is reached.` | — |
| `bErrorGeneral` | `TRUE if a single error message from the process is present.` | — |
| `byError` | `Output of the error as a byte.` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** This function block serves the acquisition and scaling of analog input signals. Using the KL30xx, KL31xx and KL32xx terminals, the standard signals 0-20 mA, 4-20 mA, 0-10 V and 10-5000 Ω can be acquired and converted to physical values.

##### `FB_HVACAnalogOutput` — §5.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rSetpoint` | `REAL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `eCtrlModeAnalogOutput` | `E_HVACAnalogOutputMode` | — |
| `rYManual` | `REAL` | — |
| `rFeedb` | `REAL` | — |
| `bFrost` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rSetpoint` | `REAL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `eCtrlModeAnalogOutput` | `E_HVACAnalogOutputMode` | — |
| `rYManual` | `REAL` | — |
| `rFeedb` | `REAL` | — |
| `bFrost` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If the function block is not enabled the value` | — |
| `rSetpoint` | `The setpoint for the analog output is transferred to the function block with the variable rSetpoint.` | — |
| `bCtrlVoltage` | `The control voltage is present if the bCtrlVoltage variable is TRUE. The feedback control is` | — |
| `eCtrlModeAnalogOutput` | `Enum that defines the operation mode (see E_HVACAnalogOutputMode [} 488]).` | — |
| `eHVACAnalogOutputMode_Auto_BMS` | `= 0,` | — |
| `eHVACAnalogOutputMode_Manual_BMS` | `= 1,` | — |
| `eHVACAnalogOutputMode_Auto_OP` | `= 2,` | — |
| `eHVACAnalogOutputMode_Manual_OP` | `= 3` | — |
| `rYManual` | `Analog input value, which is relayed to the output rY in manual mode.` | — |
| `rFeedb` | `Analog positional feedback from the actuator.` | — |
| `bFrost` | `This input serves to protect an air heater against frost. As soon as the input bFrost is TRUE, rY is` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rRangeHigh` | `REAL` | — |
| `rRangeLow` | `REAL` | — |
| `bDirection` | `BOOL` | — |
| `bEnableFeedbCtrl` | `BOOL` | — |
| `rHysteresisFeedbCtrl` | `REAL` | — |
| `tDelayFeedbCtrl` | `TIME` | — |
| `Example` | `rSetpoint = 100, rRangeLow = 0, rRangeHigh = 200,` | — |
| `bDirection` | `The output signal of rY is inverted by the variable bDirection. FALSE corresponds to the direct` | — |
| `bEnableFeedbCtrl` | `Analog actuators often have positional feedback. The function of the actuator is` | — |
| `rHysteresisFeedbCtrl` | `Due to the stroke time of typical drives used in heating and air conditioning systems,` | — |
| `tDelayFeedbCtrl` | `If the difference between the set position and the actual position of the actuator is greater` | — |
| `is` | `y=m*x+b.` | — |
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If the function block is not enabled the value` | — |
| `rSetpoint` | `The setpoint for the analog output is transferred to the function block with the variable rSetpoint.` | — |
| `bCtrlVoltage` | `The control voltage is present if the bCtrlVoltage variable is TRUE. The feedback control is` | — |
| `eCtrlModeAnalogOutput` | `Enum that defines the operation mode (see E_HVACAnalogOutputMode [} 488]).` | — |
| `eHVACAnalogOutputMode_Auto_BMS` | `= 0,` | — |
| `eHVACAnalogOutputMode_Manual_BMS` | `= 1,` | — |
| `eHVACAnalogOutputMode_Auto_OP` | `= 2,` | — |
| `eHVACAnalogOutputMode_Manual_OP` | `= 3` | — |
| `rYManual` | `Analog input value, which is relayed to the output rY in manual mode.` | — |
| `rFeedb` | `Analog positional feedback from the actuator.` | — |
| `bFrost` | `This input serves to protect an air heater against frost. As soon as the input bFrost is TRUE, rY is` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rX2` | `REAL` | — |
| `rX1` | `REAL` | — |
| `iY2` | `INT` | — |
| `iY1` | `INT` | — |
| `bDirection` | `BOOL` | — |
| `bEnableFeedbCtrl` | `BOOL` | — |
| `rHysteresisFeedbCtrl` | `REAL` | — |
| `tDelayFeedbCtrl` | `TIME` | — |
| `rX2` | `Upper limit value on the X-axis (-32767..32767). The variable is saved persistently.` | — |
| `rX1` | `Lower limit value on the X-axis (-32767..32767). The variable is saved persistently.` | — |
| `rY2` | `Upper limit value on the Y-axis (0..32767). The variable is saved persistently.` | — |
| `rY1` | `Lower limit value on the Y-axis (0..32767). The variable is saved persistently.` | — |
| `bDirection` | `The output signal of rY is inverted by the variable bDirection. FALSE corresponds to the direct` | — |
| `bEnableFeedbCtrl` | `Analog actuators often have positional feedback. The function of the actuator is` | — |
| `rHysteresisFeedbCtrl` | `Due to the stroke time of typical drives used in heating and air conditioning systems,` | — |
| `tDelayFeedbCtrl` | `If the difference between the set position and the actual position of the actuator is greater` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `iYTerminal` | `INT` | — |
| `eStateModeAnalogOutput` | `E_HVACAnalogOutputMode` | — |
| `bManualMode` | `BOOL` | — |
| `byState` | `BYTE` | — |
| `bErrorFeedb` | `BOOL` | — |
| `bErrorGeneral` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Shows the current magnitude of the control signal in % (0%..100%).` | — |
| `iYTerminal` | `Represents the size of the output signal scaled to the value range 0 to 32767.` | — |
| `eStateModeAnalogOutput` | `Enum that indicates the operation mode (see E_HVACAnalogOutputMode` | — |
| `bManualMode` | `The analog output is in manual operation mode.` | — |
| `byState` | `Status byte indicating the operating state of the function block.` | — |
| `bErrorFeedb` | `Error feedback signal.` | — |
| `bErrorGeneral` | `This is a collective message for all function block errors.` | — |
| `byError` | `Delivers all error messages and warnings.` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |
| `rY` | `REAL` | — |
| `iYTerminal` | `INT` | — |
| `eStateModeAnalogOutput` | `E_HVACAnalogOutputMode` | — |
| `bManualMode` | `BOOL` | — |
| `byState` | `BYTE` | — |
| `bErrorFeedb` | `BOOL` | — |
| `bErrorGeneral` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Shows the current magnitude of the control signal in % (0%..100%).` | — |
| `iYTerminal` | `Represents the size of the output signal scaled to the value range 0 to 32767.` | — |
| `eStateModeAnalogOutput` | `Enum that indicates the operation mode (see E_HVACAnalogOutputMode` | — |
| `bManualMode` | `The analog output is in manual operation mode.` | — |
| `byState` | `Status byte indicating the operating state of the function block.` | — |
| `bErrorFeedb` | `Error feedback signal.` | — |
| `bErrorGeneral` | `This is a collective message for all function block errors.` | — |
| `byError` | `Delivers all error messages and warnings.` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** This function block serves to control analog actuators, such as valves or dampers, with positional feedback.

##### `FB_HVACAnalogTo3Point` — §5.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rSetpoint` | `REAL` | — |
| `rFeedb` | `REAL` | — |
| `bLimitSwitchOpen` | `BOOL` | — |
| `bLimitSwitchClose` | `BOOL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If control voltage is present and there are no` | — |
| `rSetpoint` | `The setpoint for the position of the damper or the valve is transferred to the function block with` | — |
| `rFeedb` | `The input variable rFeedb (0.0..32767) is only active if the referencing mode of the function block` | — |
| `bCtrlVoltage` | `A check is made of whether control voltage is present via the input variable bCtrlVoltage. Both` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `rMinSetpoint` | `REAL` | — |
| `rMaxSetpoint` | `REAL` | — |
| `rThreshold` | `REAL` | — |
| `rTolerance` | `REAL` | — |
| `tPauseMin` | `TIME` | — |
| `tPauseMax` | `TIME` | — |
| `tInterval` | `TIME` | — |
| `tStrokeTime` | `TIME` | — |
| `tSwitchOverDeadTime` | `TIME` | — |
| `eReferencingMode` | `E_HVACReferencingMode` | — |
| `tStrokeTime` | `The variable tStrokeTime [s] specifies the complete stroke time of the drive. If the drive has no` | — |
| `tSwitchOverDeadTime` | `Dwell time when changing direction (0..3600). Both outputs are reset during this` | — |
| `eReferencingMode` | `Enum that specifies the referencing mode of the function block (see` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOpen` | `BOOL` | — |
| `bClose` | `BOOL` | — |
| `byState` | `BYTE` | — |
| `rPosition` | `REAL` | — |
| `bErrorGeneral` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bErrorLimitSwitch` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOpen` | `Output for opening drive of the three-point actuator.` | — |
| `bClose` | `Output for closing drive of the three-point actuator.` | — |
| `byState` | `Status byte indicating the operating state of the function block.` | — |
| `rPosition` | `With rPosition either the measured or the calculated position of the drive is fed back to the` | — |
| `bErrorGeneral` | `The error message bErrorGeneral becomes TRUE as soon as bErrorGeneralLimitSwitch =` | — |
| `byError` | `Delivers all error messages and warnings from the function block` | — |
| `bErrorLimitSwitch` | `Becomes TRUE if both limit switches are activated simultaneously or if ((rMaxSetpoint -` | — |
| `bInvalidParameter` | `Indicates that an incorrect parameter is present at one of the variables rMinSetpoint,` | — |

**Description:** This function block serves to convert an analog control signal into a three-point step signal. Hence three- point dampers or valves can be controlled by a controller with a continuous control signal. The function block FB_HVACAnalogTo3Point works with or without a continuous positional feedback signal from the drive.

##### `FB_HVACConfigureKL32xx` — §5.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `byStatusKL32xx` | `BYTE` | — |
| `iDataInKL32xx` | `INT` | — |
| `bSetSensor` | `BOOL` | — |
| `bScanSensor` | `BOOL` | — |
| `eSensorType` | `E_HVACSensorType` | — |
| `tTimeOut` | `TIME` | — |
| `byStatusKL32xx` | `The status byte (Status) of the Bus Terminal must be assigned here in the TwinCAT I/O` | — |
| `iDataInKL32xx` | `The raw value (Data In) of the Bus Terminal must be assigned here in the TwinCAT I/O` | — |
| `bSetSensor` | `The type of sensor specified on the Enum eSensorType [} 492] is set in the Bus Terminal with` | — |
| `bScanSensor` | `The scanning of the respective channel of the Bus Terminal is activated by a rising edge on` | — |
| `eSensorType` | `Enum with which the sensor type for the respective channel is specified for setting via` | — |
| `tTimeOut` | `Specifies the time until the abortion of the function and bErrorCommunication goes TRUE. If no` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReady` | `BOOL` | — |
| `byOutStatus` | `BYTE` | — |
| `iOutDataIn` | `INT` | — |
| `byCtrlKL32xx` | `BYTE` | — |
| `iDataOutKL32xx` | `INT` | — |
| `eBusTerminalKL32xx` | `E_HVACBusTerminal_KL32xx` | — |
| `eStatusScanSensorType` | `E_HVACSensorType` | — |
| `bErrorGeneral` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bErrorCommunication` | `BOOL` | — |
| `bErrorBusTerminalNotSupported` | `BOOL` | — |
| `bErrorSensorType` | `BOOL` | — |
| `bErrorScanSensor` | `BOOL` | — |
| `bReady` | `If the function block is activated via a rising edge on bSetSensor/bScanSensor, then bReady goes` | — |
| `byOutStatus` | `If scanning or setting has been concluded with a rising edge on bSetSensor/bScanSensor,` | — |
| `iOutDataIn` | `If scanning or setting has been concluded with a rising edge on bSetSensor/bScanSensor, then` | — |
| `byCtrlKL32xx` | `The control byte (Control) of the Bus Terminal must be assigned here in the TwinCAT I/O` | — |
| `iDataOutKL32xx` | `The data output (Data Out) of the Bus Terminal must be assigned here in the TwinCAT` | — |
| `eBusTerminalKL32xx` | `Enum that displays the type of Bus Terminal if the function block has been activated` | — |
| `eStatusScanSensorType` | `Enum that displays the type of sensor for the respective channel if the function` | — |
| `bErrorGeneral` | `Goes TRUE as soon as either bErrorCommunication, bErrorBusTerminalNotSupported,` | — |
| `byError` | `Delivers all error messages and warnings from the function block` | — |
| `bErrorCommunication` | `Goes TRUE if, for example, the variables byCtrlKL32xx, iDataOutKL32xx,` | — |
| `bErrorBusTerminalNotSupported` | `Becomes TRUE if the Bus Terminal is not supported by the function` | — |
| `bErrorSensorType` | `Becomes TRUE if the type of sensor specified at eSensorType is not supported by the` | — |
| `bErrorScanSensor` | `Becomes TRUE if, when scanning, the type of sensor is not supported by the Bus` | — |

**Description:** With the function block FB_HVACConfigureKL32xx, the type of sensor of a channel of the KL3201/02/04, the KL3208-0010 or the KL3228 can be set from the PLC. To do this, the variables byStatusKL32xx, iDataInKL32xx, byCtrlKL32xx and iDataOutKL32xx must be linked in the TwinCAT I/O area with the variables Status, Data In, Control and Data Out of a channel of the KL3201/02/04 or the KL3228. The resistor elements specified in Table 1 can be selected with the Enum eSensorType [} 492]. The KS2000 configuration software is no longer needed for setting the temperature sensor.

##### `FB_HVACScale` — §5.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `rRawData` | `REAL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `rRawData` | `Raw value from the terminal.` | — |
| `rRawDataLowOffLimit` | `REAL` | — |
| `rRawDataHighOffLimit` | `REAL` | — |
| `rScaleDataLowOffLimit` | `REAL` | — |
| `rScaleDataHighOffLimit` | `REAL` | — |
| `rRawDataLowOffLimit` | `Lower limit for raw value. The variable is saved persistently. Preset to 0.` | — |
| `rRawDataHighOffLimit` | `Upper limit for raw value. The variable is saved persistently. Preset to 32767.` | — |
| `rScaleDataLowOffLimit` | `Lower limit of scaled measured value. The variable is saved persistently. Preset to` | — |
| `rScaleDataHighOffLimit` | `Upper limit of scaled measured value. The variable is saved persistently. Preset to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rScaledValue` | `REAL` | — |
| `rScaledValue` | `Scaled value.` | — |

**Description:** A raw analog value is scaled to the specified measuring range and returned as the function value. If the raw value extends beyond the upper or lower measuring range, the corresponding limit value is output. There must be a difference of at least 0.01 between the upper and lower limit values for the raw data. If this is not the case, the lower limit value is output. The difference between the limits is necessary so as to avoid division by zero during the internal calculation of the linear equation.

##### `FB_HVACScale_nPoint` — §5.1.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rX` | `REAL` | — |
| `stParameterScale_nPoint` | `ST_HVACParameterScale_nPoint` | — |
| `rX` | `Raw value that scales to the indicated measured value of the structure stParameterScale_nPoint [} 496]` | — |
| `stParameterScale_nPoint` | `Structure via which the individual points of the XY coordinates are given their` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `stStateParameterScale_nPoint` | `ST_HVACParameterScale_nPoint` | — |
| `bErrorNumberOfPoint` | `BOOL` | — |
| `bErrorParameter` | `BOOL` | — |
| `uiErrorPositionX_Parameter` | `INT` | — |
| `rY` | `rY is the scaled temperature value of the specified raw value rX.` | — |
| `stStateParameterScale_nPoint` | `State of the structure stParameterScale_nPoint. If bErrorNumberOfPoint` | — |
| `bErrorNumberOfPoint` | `IfiNumberOfPoint < 2 or iNumberOfPoint > g_iMaxNoOfScale_nPoint, then` | — |
| `bErrorParameter` | `Care must be taken when parameterizing the structure stParameterScale_nPoint [} 496]` | — |
| `uiErrorPositionX_Parameter` | `If bErrorParameter = TRUE, then the exact field position in the array` | — |
| `g_iMaxNoOfScale_nPoint` | `INT := 60` | — |
| `g_iMaxNoOfScale_nPoint` | `Global constant that specifies the maximum number of points of the XY` | — |

**Description:** Curves can be reproduced in the PLC with the function block FB_HVACScale_nPoint. These can be, for example, positive or negative temperature coefficients (PTC/NTC). The analog raw resistance value of a PTC/NTC is applied to the input rX and output as a temperature value via the output rY. The individual parameters and the number of individual points on the XY axis of the characteristic curve are specified by the structure stParameterScale_nPoint [} 496]. Analog raw resistance values can be represented in the PLC with the KL32xx Bus Terminals. Detailed information can be found in the documentation on FB_HVACConfigureKL32xx [} 82].

##### `FB_HVACScaleXX` — §5.1.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `lrX` | `LREAL` | — |
| `lrX0n` | `LREAL` | — |
| `lrY0n` | `LREAL` | — |
| `bLmt` | `BOOL` | — |
| `lrX` | `Applied raw value.` | — |
| `lrX0n` | `X interpolation point of the characteristic curve. The characteristic curve has 2, 4 or 7 interpolation` | — |
| `lrY0n` | `X interpolation point of the characteristic curve. The characteristic curve has 2, 4 or 7 interpolation` | — |
| `bLmt` | `Input for limiting the output Y-range to values between lrY01 and lrY0n, depending on the maximum` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrY` | `LREAL` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `udiErrorArgument` | `UDINT` | — |
| `lrY` | `Calculated Y-value of the characteristic curve.` | — |
| `bError` | `Incorrect entry of the X interpolation points. The values must be entered in ascending order. No two` | — |
| `eErrorCode` | `The error message eHVACErrorCodes_ErrIdlrX0x [} 490] will be displayed.` | — |
| `udiErrorArgument` | `Indication of which interpolation point has been entered incorrectly.` | — |

**Description:** Characteristic curves with 2, 4 or 7 interpolation points can be mapped with the function blocks FB_HVACScale02, FB_HVACScale04 and FB_HVACScale07 (shown here). The analog raw value is applied to the input IrX and output as the calculated Y value at the output IrY. Different slopes of the characteristic curve can be specified in sections by specifying various interpolation points. The range is limited internally to the values lrY01 and lrY07 with the aid of the input bLmt.

##### `FB_HVACTemperatureCurve` — §5.1.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eTemperatureCurve` | `E_HVACTemperatureCurve` | — |
| `eTemperatureCurve` | `Enum via which a temperature curve stored internally in the function block can be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParameterScale_nPoint` | `ST_HVACParameterScale_nPoint` | — |
| `stParameterScale_nPoint` | `Structure containing the number and valences of the points of the XY` | — |

**Description:** A temperature curve stored internally in the function block is selected from Tables 1 and 2 via the Enum eTemperatureCurve [} 494] and output via the structure stParameterScale_nPoint. Characteristic curves can then be reproduced with this structure in conjunction with the function block FB_HVACScale_nPoint [} 87]. In order to arrive at the raw resistance value of a sensor, the KL32xx Bus Terminals must be set to resistance measurement; see FB_HVACConfigureKL32xx [} 82].

##### `FB_HVACTemperatureSensor` — §5.1.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `iRawValue` | `INT` | — |
| `byStatusRawValue` | `BYTE` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `bEnableLimitCtrl` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `iRawValue` | `Raw value of the temperature sensor in 1/10°C from the bus terminal` | — |
| `byStatusRawValue` | `Status byte of the temperature sensor from the bus terminal. Serves for fault diagnosis,` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `The variable tCtrlCycleTime specifies the cycle time with which the 2nd order filter is` | — |
| `bEnableLimitCtrl` | `Enables rHighLimit and rLowLimit for limit monitoring` | — |
| `bReset` | `Acknowledge input in the event of an error. In addition the 2nd order filter can be synchronized via` | — |
| `rOffset` | `REAL` | — |
| `rHighLimit` | `REAL` | — |
| `rLowLimit` | `REAL` | — |
| `rReplacementValue` | `REAL` | — |
| `tFilterTime` | `TIME` | — |
| `eTemperatureSensorMode` | `E_HVACTemperatureSensorMode` | — |
| `rOffset` | `Temperature compensation in Kelvin (-50..+50), rPresentValue = (iRawValue / 10.0) + rOffset. The` | — |
| `rHighLimit` | `Upper limit value (-250..+850); the output bErrorHighLimit is set if rPresentValue >= rHighLimit.` | — |
| `rLowLimit` | `Lower limit value (-250..+850); the output bErrorLowLimit is set if rPresentValue <= rLowLimit.` | — |
| `rReplacementValue` | `Replacement value (-250..+850) that is output at rPresentValue in the case of the` | — |
| `tFilterTime` | `Filter constant (1ms..100s). To avoid large fluctuations and jumps in the measuring signal, the` | — |
| `eTemperatureSensorMode` | `Enum that specifies the operation mode of the function block (see` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rPresentValue` | `REAL` | — |
| `bErrorGeneral` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bErrorShortCircuit` | `BOOL` | — |
| `bErrorBrokenSensor` | `BOOL` | — |
| `bErrorHighLimit` | `BOOL` | — |
| `bErrorLowLimit` | `BOOL` | — |
| `bErrorCycleTime` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rPresentValue` | `Temperature output variable with one decimal place, rPresentValue = (iRawValue / 10.0) +` | — |
| `bErrorGeneral` | `The error message bErrorGeneral becomes TRUE as soon as one of the error messages` | — |
| `byError` | `Delivers all error messages and warnings,` | — |
| `bErrorShortCircuit` | `Error, short circuit in the connected temperature sensor. After rectification of the error,` | — |
| `bErrorBrokenSensor` | `Error, wire breakage in the connected temperature sensor. After rectification of the` | — |
| `bErrorHighLimit` | `Warning upper limit value exceeded; becomes TRUE if rPresentValue >= rHighLimit. The` | — |
| `bErrorLowLimit` | `Warning: lower limit value undershot; goes TRUE if rPresentValue <= rLowLimit. The` | — |
| `bErrorCycleTime` | `Error caused by an incorrect time input at the input variables tTaskCycleTime and` | — |
| `bInvalidParameter` | `Indicates that an incorrect parameter is present at one of the variables rOffset,` | — |

**Description:** This function block serves the acquisition and further processing of temperature values for sensor types PT100, PT200, PT1000, NI100, NI120 and NI1000. The function block FB_HVACTemperatureSensor is matched to the KL320x bus terminals. These Bus Terminals can either be ordered preconfigured or set to the corresponding sensor types in the software.

##### `FB_HVACTemperatureSensorEx` — §5.1.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `iRawValue` | `INT` | — |
| `byStatusRawValue` | `BYTE` | — |
| `bEnableLimitCtrl` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If bEnable = FALSE, the value of` | — |
| `iRawValue` | `Raw value of the temperature sensor in 1/10°C from the bus terminal.` | — |
| `byStatusRawValue` | `Status byte of the temperature sensor from the bus terminal. Serves for fault diagnosis,` | — |
| `bEnableLimitCtrl` | `Enables rHighLimit and rLowLimit for limit monitoring` | — |
| `bReset` | `Acknowledgement input in the event of an error with rising edge from bReset. Depending on mode` | — |
| `rOffset` | `REAL` | — |
| `rHighLimit` | `REAL` | — |
| `rLowLimit` | `REAL` | — |
| `rReplacementValue` | `REAL` | — |
| `rSmoothFactor` | `REAL` | — |
| `eTemperatureSensorMode` | `E_HVACTemperatureSensorMode` | — |
| `rOffset` | `Temperature compensation in Kelvin, rPresentValue = (iRawValue / 10.0) + rOffset. The variable is` | — |
| `rHighLimit` | `Upper limit value. If rPresentValue >= rHighLimit , then output bErrorHighLimit is set.` | — |
| `rLowLimit` | `Lower limit value. If rPresentValue <= rLowLimit , then output bErrorLowLimit is set. The variable` | — |
| `rReplacementValue` | `Replacement value that is output at rPresentValue in the case of the error` | — |
| `rSmoothFactor` | `Smoothing factor (>= 1) for the output value rPresentValue. The variable is saved` | — |
| `rPresentValue` | `= ((iRawValue / 10 + rOffset) - rPresentValueOld) / rSmoothFactor + rPresentValueOld` | — |
| `eTemperatureSensorMode` | `Enum that specifies the operation mode of the function block (see` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rPresentValue` | `REAL` | — |
| `bErrorGeneral` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bErrorSensor` | `BOOL` | — |
| `bErrorHighLimit` | `BOOL` | — |
| `bErrorLowLimit` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rPresentValue` | `Temperature output variable with one decimal place.` | — |
| `rPresentValue` | `= ((iRawValue / 10 + rOffset) - rPresentValueOld) / rSmoothFactor + rPresentValueOld` | — |
| `bErrorGeneral` | `The error message bErrorGeneral becomes TRUE if one of the error messages` | — |
| `byError` | `Delivers all error messages and warnings,` | — |
| `bErrorSensor` | `Becomes TRUE if byError.5/bErrorShortCircuit or byError.6/bErrorBrokenSensor is TRUE.` | — |
| `bErrorHighLimit` | `Warning upper limit value exceeded; becomes TRUE if rPresentValue >= rHighLimit. The` | — |
| `bErrorLowLimit` | `Warning: lower limit value undershot; goes TRUE if rPresentValue <= rLowLimit. The` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect parameter to one of the variables rHighLimit,` | — |

**Description:** This function block is used for the acquisition and subsequent processing of temperature values, e.g. for the sensor types PT100, PT200, PT1000, NI100, NI120, NI1000, NI1000Tk5000. The function block FB_HVACTemperatureSensorEx is adapted to the Bus Terminals KL3201/02/04 and KL3228. These Bus Terminals can either be ordered preconfigured or set to the respective sensor type via the software. The raw temperature value is transferred to the function block in 1/10 °C via the input variable iRawValue and output as a floating point number via rPresentValue . iRawValue, for example, can be linked directly with the raw temperature value of the following Bus Terminals: KL3201/02/04 and KL3228.

##### `FB_HVACTemperatureSensorEx` — §5.1.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `iRawValue` | `INT` | — |
| `byStatusRawValue` | `BYTE` | — |
| `iConversionFactor` | `INT` | — |
| `bEnableLimitCtrl` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If bEnable = FALSE, the value of` | — |
| `iRawValue` | `Raw value of the temperature sensor in 1/10 or 1/100 °C from the bus terminal.` | — |
| `byStatusRawValue` | `Status byte of the temperature sensor from the bus terminal. Serves for fault diagnosis,` | — |
| `iConversionFactor` | `Conversion factor (0..1) for the output value rPresentValue.` | — |
| `bEnableLimitCtrl` | `Enables rHighLimit and rLowLimit for limit monitoring` | — |
| `bReset` | `Acknowledgement input in the event of an error with rising edge at bReset. Depending on mode` | — |
| `rOffset` | `REAL` | — |
| `rHighLimit` | `REAL` | — |
| `rLowLimit` | `REAL` | — |
| `rReplacementValue` | `REAL` | — |
| `rSmoothFactor` | `REAL` | — |
| `iTemperatureSensorMode` | `INT` | — |
| `rOffset` | `Temperature compensation in Kelvin. The variable is saved persistently. Preset to 0.` | — |
| `rHighLimit` | `Upper limit value. The variable is saved persistently. Preset to 120.` | — |
| `rLowLimit` | `Lower limit value. The variable is saved persistently. Preset to -60.` | — |
| `rReplacementValue` | `Replacement value that is output at rPresentValue in the case of the errors` | — |
| `rSmoothFactor` | `Smoothing factor (>= 1) for the output value rPresentValue.` | — |
| `iTemperatureSensorMode` | `Specifies the operation mode of the function block.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rPresentValue` | `REAL` | — |
| `bErrorGeneral` | `BOOL` | — |
| `udiError` | `UDINT` | — |
| `bErrorSensor` | `BOOL` | — |
| `bErrorHighLimit` | `BOOL` | — |
| `bErrorLowLimit` | `BOOL` | — |
| `bErrorConversionFactor` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rPresentValue` | `Temperature output variable with two decimal places.` | — |
| `bErrorGeneral` | `The error message bErrorGeneral becomes TRUE if one of the error messages` | — |
| `udiError` | `Delivers all error messages and warnings,` | — |
| `bErrorSensor` | `Goes TRUE if byError.5 / bErrorShortCircuit or byError.6 / bErrorBrokenSensor = TRUE.` | — |
| `bErrorHighLimit` | `Warning upper limit value exceeded; becomes TRUE if rPresentValue >= rHighLimit. The` | — |
| `bErrorLowLimit` | `Warning: lower limit value undershot; goes TRUE if rPresentValue <= rLowLimit. The` | — |
| `bErrorConversionFactor` | `If a value outside its range of 0 to 1 is specified at iConversionFactor, this is` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect parameter on one of the variables rHighLimit,` | — |

**Description:** This function block is used for the acquisition and subsequent processing of temperature values, e.g. for the sensor types PT100, PT200, PT1000, NI100, NI120, NI1000, NI1000Tk5000. The function block FB_HVACTemperatureSensorEx2 is tailored to the Bus Terminals KL3201/02/04, KL3222, KL3228 and KL3208-0010. These Bus Terminals can either be ordered preconfigured or set to the respective sensor type via the software. The raw temperature value is transferred either in 1/10 or 1/100 °C to the function block via the input variable

#### Programming › POUs › HVAC Controller

##### `FB_HVAC2PointCtrl` — §5.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The controller is enabled via a TRUE and is then active.` | — |
| `rW` | `The setpoint is transferred with the rW variable.` | — |
| `rX` | `The actual value of the control loop is transferred with the rX variable.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rHighLimit` | `REAL` | — |
| `rLowLimit` | `REAL` | — |
| `bDirection` | `BOOL` | — |
| `tDelayOn` | `TIME` | — |
| `tDelayOff` | `TIME` | — |
| `rHighLimit` | `Upper limit of the control deviation (0..32767). The variable is saved persistently. Preset to 0.` | — |
| `rLowLimit` | `Lower limit of the control deviation (0..32767). The variable is saved persistently. Preset to 0.` | — |
| `bDirection` | `The control direction of the controller is determined by bDirection. FALSE = heating mode` | — |
| `tDelayOn` | `Switch-on delay [s]. The variable is saved persistently. Preset to 0 s.` | — |
| `tDelayOff` | `Switch-off delay [s]. The variable is saved persistently. Preset to 0 s.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOut` | `Switching output of the two-point controller.` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** This function block represents a 2-point controller. The controller is enabled via bEnable = TRUE and is then active.

##### `FB_HVACI_CtrlStep` — §5.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `iManualStep` | `INT` | — |
| `bStepUp` | `BOOL` | — |
| `bStepDown` | `BOOL` | — |
| `bDirection` | `BOOL` | — |
| `iNumberOfStepInProfile` | `INT` | — |
| `bBlock` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then iStep is set constantly to 0. The` | — |
| `rW` | `The setpoint is transferred with the rW variable.` | — |
| `rX` | `The variable rX is used to transfer the actual value.` | — |
| `eCtrlMode` | `Enum that specifies the operation mode of the function block (see E_HVACCtrlMode [} 489]). If` | — |
| `iManualStep` | `If the operation mode eCtrlMode = eHVACCtrlMode_Manual AND bEnable = TRUE AND` | — |
| `bStepUp` | `If bEnable = TRUE AND bError = FALSE AND eCtrlMode = eHVACCtrlMode_Auto AND bBlock =` | — |
| `bStepDown` | `If bEnable = TRUE AND bError = FALSE AND eCtrlMode = eHVACCtrlMode_Auto AND` | — |
| `bDirection` | `bDirection determines the control direction of the function block. FALSE = heating mode; TRUE` | — |
| `iNumberOfStepInProfile` | `Number of steps in the power step sequence. iNumberOfStepInProfile can be 0 at` | — |
| `bBlock` | `Using the input variable bBlock, the control of the function block can either be released or the output` | — |
| `bReset` | `Input for acknowledgement of the faults once they have been corrected. Internally the system` | — |
| `rHysteresisHigh` | `REAL` | — |
| `rHysteresisLow` | `REAL` | — |
| `udiSecDelayHigh` | `UDINT` | — |
| `udiSecDelayLow` | `UDINT` | — |
| `udiSecTiHigh` | `UDINT` | — |
| `udiSecTiLow` | `UDINT` | — |
| `rIntegralHigh` | `REAL` | — |
| `rIntegralLow` | `REAL` | — |
| `rHysteresisHigh` | `Positive value of the upper limit of the control deviation; see Functional description [} 112]` | — |
| `rHysteresisLow` | `Positive value of the lower limit of the control deviation; see Functional description [} 112]` | — |
| `udiSecDelayHigh` | `Delay time (0..g_udiMaxSec) after whose expiry iStep is incremented or` | — |
| `udiSecDelayLow` | `Delay time (0..g_udiMaxSec) after whose expiry iStep is incremented or` | — |
| `udiSecTiHigh` | `Integration time (0..g_udiMaxSec) for the upper limit of the I-transfer element in seconds; see` | — |
| `udiSecTiLow` | `Integration time (0..g_udiMaxSec) for the lower limit of the I-transfer element; see Functional` | — |
| `rIntegralHigh` | `Positive value for the upper limit at which the integration of the I-transfer element is stopped` | — |
| `rIntegralLow` | `Positive value for the lower limit at which the integration of the I-transfer element is stopped` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `iStep` | `INT` | — |
| `rI_Ctrl` | `REAL` | — |
| `rE` | `REAL` | — |
| `rW_High` | `REAL` | — |
| `rW_Low` | `REAL` | — |
| `b_rW_High` | `BOOL` | — |
| `b_rW_Low` | `BOOL` | — |
| `udiSecRT_DelayHigh` | `UDINT` | — |
| `udiSecRT_DelayLow` | `UDINT` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `iStep` | `The output variable iStep indicates the step in a power step sequence. On starting the function block,` | — |
| `rI_Ctrl` | `Output of the I-transfer element.` | — |
| `rE` | `Control deviation with which the internal I-transfer element works: rE = rW - rX` | — |
| `rW_High` | `rW_High := rW + rHysteresisHigh – upper setpoint limit which, when exceeded by rX, causes the` | — |
| `rW_Low` | `rW_Low := rW - rHysteresisLow – lower setpoint limit which, when undershot by rX, causes the I-` | — |
| `b_rW_High` | `b_rW_High goes TRUE if rX > rW_High.` | — |
| `b_rW_Low` | `b_rW_Low goes TRUE if rX < rW_Low.` | — |
| `udiSecRT_DelayHigh` | `Remaining time of the delay time udiSecDelayHigh.` | — |
| `udiSecRT_DelayLow` | `Remaining time of the delay time udiSecDelayLow.` | — |
| `bError` | `This output signals with a TRUE that there is an error and that there is a wrong parameter on the` | — |
| `eErrorCode` | `returns the error number if an output bError is set (see E_HVACErrorCodes [} 490]). The` | — |

**Description:** The function block serves the sequential control of power generators.

##### `FB_HVACI_CtrlStepEx` — §5.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `iStartStep` | `INT` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `iManualStep` | `INT` | — |
| `bStepUp` | `BOOL` | — |
| `bStepDown` | `BOOL` | — |
| `bDirection` | `BOOL` | — |
| `iNumberOfStepInProfile` | `INT` | — |
| `bBlock` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then iStep is set constantly to 0. The` | — |
| `rW` | `The setpoint is transferred with the rW variable.` | — |
| `rX` | `The variable rX is used to transfer the actual value.` | — |
| `iStartStep` | `If the function block is enabled (bEnable = TRUE) and there is no error (bError = FALSE), then` | — |
| `eCtrlMode` | `Enum that specifies the operation mode of the function block. If bEnable = TRUE AND bError =` | — |
| `iManualStep` | `If the operation mode eCtrlMode = eHVACCtrlMode_Manual AND bEnable = TRUE AND` | — |
| `bStepUp` | `If bEnable = TRUE AND bError = FALSE AND eCtrlMode = eHVACCtrlMode_Auto AND bBlock =` | — |
| `bStepDown` | `If bEnable = TRUE AND bError = FALSE AND eCtrlMode = eHVACCtrlMode_Auto AND` | — |
| `bDirection` | `bDirection determines the control direction of the function block. FALSE = heating mode; TRUE` | — |
| `iNumberOfStepInProfile` | `Number of steps in the power step sequence. iNumberOfStepInProfile can be 0 at` | — |
| `bBlock` | `Using the input variable bBlock, the control of the function block can either be released or the output` | — |
| `bReset` | `Input for acknowledgement of the faults once they have been corrected. Internally the system` | — |
| `rHysteresisHigh` | `REAL` | — |
| `rHysteresisLow` | `REAL` | — |
| `udiSecDelayHigh` | `UDINT` | — |
| `udiSecDelayLow` | `UDINT` | — |
| `udiSecTiHigh` | `UDINT` | — |
| `udiSecTiLow` | `UDINT` | — |
| `rIntegralHigh` | `REAL` | — |
| `rIntegralLow` | `REAL` | — |
| `rHysteresisHigh` | `Positive value of the upper limit of the control deviation; see Functional description [} 119]` | — |
| `rHysteresisLow` | `Positive value of the lower limit of the control deviation; see Functional description [} 119]` | — |
| `udiSecDelayHigh` | `Delay time (0..g_udiMaxSec) after whose expiry iStep is incremented or` | — |
| `udiSecDelayLow` | `Delay time (0..g_udiMaxSec) after whose expiry iStep is incremented or` | — |
| `udiSecTiHigh` | `Integration time (0..g_udiMaxSec) for the upper limit of the I-transfer element in seconds; see` | — |
| `udiSecTiLow` | `Integration time (0..g_udiMaxSec) for the lower limit of the I-transfer element; see Functional` | — |
| `rIntegralHigh` | `Positive value for the upper limit at which the integration of the I-transfer element is stopped` | — |
| `rIntegralLow` | `Positive value for the lower limit at which the integration of the I-transfer element is stopped` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `iStep` | `INT` | — |
| `rI_Ctrl` | `REAL` | — |
| `rE` | `REAL` | — |
| `rW_High` | `REAL` | — |
| `rW_Low` | `REAL` | — |
| `b_rW_High` | `BOOL` | — |
| `b_rW_Low` | `BOOL` | — |
| `udiSecRT_DelayHigh` | `UDINT` | — |
| `udiSecRT_DelayLow` | `UDINT` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `iStep` | `The output variable iStep indicates the step in a power step sequence. iStep = iStartStep on starting` | — |
| `rI_Ctrl` | `Output of the I-transfer element.` | — |
| `rE` | `Control deviation with which the internal I-transfer element works: rE = rW - rX` | — |
| `rW_High` | `rW_High := rW + rHysteresisHigh – upper setpoint limit which, when exceeded by rX, causes the` | — |
| `rW_Low` | `rW_Low := rW - rHysteresisLow – lower setpoint limit which, when undershot by rX, causes the I-` | — |
| `b_rW_High` | `b_rW_High goes TRUE if rX > rW_High.` | — |
| `b_rW_Low` | `b_rW_Low goes TRUE if rX < rW_Low.` | — |
| `udiSecRT_DelayHigh` | `Remaining time of the delay time udiSecDelayHigh.` | — |
| `udiSecRT_DelayLow` | `Remaining time of the delay time udiSecDelayLow.` | — |
| `bError` | `This output signals with a TRUE that there is an error and that there is a wrong parameter on the` | — |
| `eErrorCode` | `Returns the error number if a bError output is set (see E_HVACErrorCodes [} 490]). The` | — |

**Description:** Functional description

##### `FB_HVACPIDCtrl` — §5.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rYManual` | `REAL` | — |
| `rInitialValue` | `REAL` | — |
| `bResetController` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `Input variable for enabling the controller. The controller is enabled if bEnable = TRUE.` | — |
| `rW` | `The setpoint is transferred to the controller with the variable rW.` | — |
| `rX` | `rX acquires the actual value of the control loop.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `The variable tCtrlCycleTime specifies the cycle time with which the PID controller is` | — |
| `eCtrlMode` | `The operation mode is selected via this enum (see E_HVACCtrlMode [} 489]).` | — |
| `rYManual` | `The controller output rY can be overridden for test purposes. The setpoint of rYManual will be` | — |
| `rInitialValue` | `The restart behavior of the controller is influenced by rInitialValue. The initial values are` | — |
| `bResetController` | `A positive edge on the input bResetController resets the PID controller.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rDeadRange` | `REAL` | — |
| `bDirection` | `BOOL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `rDeadRange` | `In order to avoid unnecessary driving and hence premature wear of the valves or damper` | — |
| `bDirection` | `The control direction of the controller can be changed with the parameter bDirection. If` | — |
| `rKp` | `Proportional factor (0.01..100) gain. The variable is saved persistently. Preset to 1.0.` | — |
| `tTi` | `Integration time [s]. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time [s]. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time [s]. The variable is saved persistently. Preset to 0 s.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `eState` | `E_HVACState` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Control signal output of the PID controller.` | — |
| `rXW` | `Control deviation.` | — |
| `bMaxLimit` | `The output bMaxLimit is TRUE if the output rY has reached the value rYMax.` | — |
| `bMinLimit` | `The output bMinLimit is TRUE if the output rY has reached the value rYMin.` | — |
| `bActive` | `bActive is TRUE if the controller is active and enabled.` | — |
| `bARWactive` | `bARWactive is TRUE if the integral component of the controller has reached the lower or` | — |
| `eState` | `State of the controller (see E_HVACState) [} 493].` | — |
| `bError` | `This output indicates with a TRUE that there is an error.` | — |
| `eErrorCode` | `Contains the command-specific error code (see E_HVACErrorCodes [} 490]).` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** The PID controller is a standard controller for heating and air conditioning applications. It can be used to control temperatures, pressures, volumetric flows or humidity. The values of Ti and Tv can be set to zero. This allows a P, PI, or PD controller characteristic to be set. The controller has an anti-reset-windup function. This prevents continuous integration in the event of a perpetual pending control deviation.

##### `FB_HVACPIDCtrl_Ex` — §5.1.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `uiCycleCall` | `UINT` | — |
| `bSetSyncValue` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `Controller activation. At the moment of activation the controller reacts directly to the control` | — |
| `rW` | `Setpoint` | — |
| `rX` | `Actual value` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `uiCycleCall` | `Call cycle of the function block as a multiple of the cycle time. A zero entry is automatically` | — |
| `Example` | `tTaskCycleTime = 20 ms, uiCycleCall = 10 -> the control algorithm is called every 200 ms. Thus` | — |
| `bSetSyncValue` | `The control value rY is set to the value rSyncValue (VAR_IN_OUT) on a rising edge at the` | — |
| `rDeadRange` | `REAL` | — |
| `rSyncValue` | `REAL` | — |
| `bDirection` | `BOOL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `tMinToMax` | `TIME` | — |
| `tMaxToMin` | `TIME` | — |
| `rDeadRange` | `A dead range can be set for the control deviation in order to avoid unnecessary movement` | — |
| `rSyncValue` | `The control value rY is set to this value on a rising edge at the input bSync. In addition, the I-` | — |
| `bDirection` | `The control direction [} 131] of the controller can be changed with the parameter bDirection. If` | — |
| `rKp` | `Proportional factor amplification. Only affects the P-component. The variable is saved persistently.` | — |
| `tTi` | `Integration time [s]. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time [s]. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time [s]. The variable is saved persistently. Preset to 0 s.` | — |
| `tMinToMax` | `Slope limitation [s] of the controller output for increase: tMinToMax in seconds in reference to a` | — |
| `tMaxToMin` | `Slope limitation [s] of the controller output for decrease: tMaxToMin in seconds in reference to a` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bDecLimit` | `BOOL` | — |
| `bIncLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `b2Ptactive` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `rY` | `Control value. Range limited by rYMin and rYMax.` | — |
| `rXW` | `Control deviation (calculation dependent on control direction [} 131])` | — |
| `bMaxLimit` | `The control value has reached its upper limit value.` | — |
| `bMinLimit` | `The control value has reached its lower limit value.` | — |
| `bDecLimit` | `The control value slope has reached its limit value for the maximum decrease, see tMaxToMin` | — |
| `bIncLimit` | `The control value slope has reached its limit value for the maximum increase, see tMinToMax` | — |
| `bActive` | `The controller is active, i.e. enabled (bEnable = TRUE) and not in the error state (bError = FALSE).` | — |
| `bARWactive` | `Anti-Reset-Windup function is active.` | — |
| `b2Ptactive` | `The 2-point behavior of the controller is active.` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `eErrorCode` | `Contains the command-specific error code (see E_HVACErrorCodes [} 490]).` | — |

**Description:** Universal PID controller.

##### `FB_HVACPowerRangeTable` — §5.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `iNumberOfProfiles` | `INT` | — |
| `iProfile` | `INT` | — |
| `iStep` | `INT` | — |
| `eCtrlModeProfile` | `E_HVACCtrlMode` | — |
| `iManualProfile` | `INT` | — |
| `eCtrlModeStep` | `E_HVACCtrlMode` | — |
| `iManualStep` | `INT` | — |
| `iNumberOfAggregates` | `INT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then all output variables and output` | — |
| `iNumberOfProfiles` | `The number of parameterized profiles in the power range table arrPowerRangeTable.` | — |
| `iProfile` | `Indicates the current profile with which the function block from the power range table` | — |
| `iStep` | `Indicates the current step with which the function block from the power range table` | — |
| `eCtrlModeProfile` | `The input variable via which the profile from the power range table arrPowerRangeTable` | — |
| `iManualProfile` | `Indicates which profile from the power range table arrPowerRangeTable[iCurrentProfile,` | — |
| `eCtrlModeStep` | `The input variable via which the steps from the power range table arrPowerRangeTable` | — |
| `iManualStep` | `Indicates which step from the power range table arrPowerRangeTable[iCurrentProfile,` | — |
| `iNumberOfAggregates` | `iNumberOfAggregates specifies the number of units in the power generator` | — |
| `bReset` | `Input for acknowledgement of the faults once they have been corrected. Internally the system` | — |
| `arrPowerRangeTable` | `ARRAY [1..g_iMaxNumberOfProfiles,0..g_iMaxNumberOfSteps] OF ST_HVACPowerRange` | — |
| `arrPowerRangeTable` | `All information or parameters relevant to system control are summarized in the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnablePowerRangeTable` | `BOOL` | — |
| `iNumberOfStepInProfile` | `INT` | — |
| `iCurrentProfile` | `INT` | — |
| `iCurrentStep` | `INT` | — |
| `stI_Ctrl` | `ST_HVACI_Ctrl` | — |
| `stAggregate1` | `ST_HVACAggregate` | — |
| `stAggregate2` | `ST_HVACAggregate` | — |
| `stAggregate3` | `ST_HVACAggregate` | — |
| `stAggregate4` | `ST_HVACAggregate` | — |
| `stAggregate5` | `ST_HVACAggregate` | — |
| `stAggregate6` | `ST_HVACAggregate` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `iErrorPosArrayProfile` | `INT` | — |
| `iErrorPosArrayStep` | `INT` | — |
| `bEnablePowerRangeTable` | `Indicates that the function block is released. bEnablePowerRangeTable is` | — |
| `iNumberOfStepInProfile` | `Indicates the number of parameterized steps from the specified profile` | — |
| `iCurrentProfile` | `Indicates the current profile with which the function block from the power range table` | — |
| `iCurrentStep` | `Indicates the current power step with which the function block from the power range table` | — |
| `stI_Ctrl` | `Output structure of the parameter ST_HVACI_Ctrl [} 495] for the function block FB_HVACI_CtrlStep` | — |
| `bError` | `This output indicates with a TRUE that there is an error. The execution of the function block is` | — |
| `eErrorCode` | `Returns the error number if a bError output is set (see E_HVACErrorCodes [} 490]).` | — |
| `eHVACErrorCodes_InvalidParam_iStep` | `Error when checking the specified steps. The value of iStep must` | — |
| `eHVACErrorCodes_InvalidParam_iNumberOfStepInProfile` | `Error when checking the number of profiles,` | — |
| `eHVACErrorCodes_InvalidParam_iNumberOfAggregates` | `Error when checking the specified number of` | — |
| `eHVACErrorCodes_InvalidParam_iAggregateSteps` | `Error when checking the specified steps for the unit.` | — |
| `eHVACErrorCodes_InvalidParam_iNumberOfProfiles` | `Error when checking the number of profiles. The value` | — |
| `eHVACErrorCodes_InvalidParam_iProfile` | `Error when checking the specified profiles. The value of iProfile` | — |
| `eHVACErrorCodes_InvalidParam_iAggregate` | `Error when checking the specified units. The value of` | — |
| `eHVACErrorCodes_InvalidParam_udiSecDelayUp` | `Error when checking` | — |
| `eHVACErrorCodes_InvalidParam_udiSecDelayDown` | `Error when checking` | — |
| `iErrorPosArrayProfile` | `The variable iErrorPosArrayProfile indicates the profile in which the incorrect place` | — |
| `iErrorPosArrayStep` | `If iErrorPosArrayProfile > 0, then the variable iErrorPosArrayStep indicates the step in` | — |

**Description:** This function block represents a power range table and serves the sequential control of power generators such as boilers or refrigerating machines. The power steps are determined by the upstream controller FB_HVACI_Ctrl_Step [} 112] and transferred to the power range table via the input iStep.

#### Programming › POUs › HVAC Controller › Sequence-Controller

##### `FB_HVAC2PointCtrlSequence` — §5.1.4.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `e2PointCtrlMode` | `E_HVAC2PointCtrlMode` | — |
| `bDirection` | `BOOL` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `iNumberOfSequences` | `INT` | — |
| `iMyNumberInSequence` | `INT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, the 2-point sequence controller is` | — |
| `e2PointCtrlMode` | `Enum that specifies the operation mode of the 2-point sequence controller (see` | — |
| `bDirection` | `bDirection determines the control direction of the internal 2-point controller. FALSE = heating` | — |
| `rW` | `The setpoint is transferred with the rW variable.` | — |
| `rX` | `The variable rX is used to transfer the actual value.` | — |
| `iNumberOfSequences` | `Number of controllers in the sequence (1..32). If iNumberOfSequences <= 0 an` | — |
| `iMyNumberInSequence` | `The 2-point controller's own number in the sequence (1..32). If` | — |
| `bReset` | `Input for acknowledgement of the faults once they have been corrected. Internally the system` | — |
| `rW_HighLimit` | `REAL` | — |
| `rW_LowLimit` | `REAL` | — |
| `tDelayIncreaseSequence` | `TIME` | — |
| `tDelayDecreaseSequence` | `TIME` | — |
| `tCtrl_I_Ti_HighLimit` | `TIME` | — |
| `tCtrl_I_Ti_LowLimit` | `TIME` | — |
| `rCtrl_I_HighLimit` | `REAL` | — |
| `rCtrl_I_LowLimit` | `REAL` | — |
| `iCurrentSequence` | `INT` | — |
| `rW_HighLimit` | `Positive value of the upper control deviation limit. st2PointCtrlSequence.rW_Max` | — |
| `rW_LowLimit` | `Positive value of the lower control deviation limit. st2PointCtrlSequence.rW_Min =rW_Min :=` | — |
| `tDelayIncreaseSequence` | `Delay time after whose expiry iCurrentSequence is incremented by 1, see` | — |
| `tDelayDecreaseSequence` | `Delay time after which iCurrentSequence is decreased by 1, see Transfer` | — |
| `tCtrl_I_Ti_HighLimit` | `Integration time for the upper limit of the internal I-transfer element, see Transfer` | — |
| `tCtrl_I_Ti_LowLimit` | `Integration time for the lower limit of the internal I-transfer element, see Transfer` | — |
| `rCtrl_I_HighLimit` | `Positive value for the upper limit at which the integration of the internal I-transfer element` | — |
| `rCtrl_I_LowLimit` | `Negative value for the lower limit at which the integration of the internal I-transfer element` | — |
| `iCurrentSequence` | `Number of the active controller in the sequence (0..32). The number of devices in a` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bActiveCtrl` | `BOOL` | — |
| `b_rW_Max` | `BOOL` | — |
| `b_rW_Min` | `BOOL` | — |
| `rE` | `REAL` | — |
| `e2PointCtrlState` | `E_HVAC2PointCtrlMode` | — |
| `st2PointCtrlSequence` | `ST_HVAC2PointCtrlSequence` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bOut` | `Output of the 2-point sequence controller.` | — |
| `bActiveCtrl` | `bActiveCtrl indicates with TRUE that the function block is the active one in the sequence.` | — |
| `b_rW_Max` | `b_rW_Max goes TRUE if rX > st2PointCtrlSequence.rW_Max.` | — |
| `b_rW_Min` | `b_rW_Min goes TRUE if rX < st2PointCtrlSequence.rW_Min.` | — |
| `rE` | `Control deviation: rE = rW - rX` | — |
| `e2PointCtrlState` | `Enum that indicates the state of the operation mode e2PointCtrlMode of the 2-point` | — |
| `st2PointCtrlSequence` | `The structure indicates various states, inputs and outputs of the function block (see` | — |
| `bError` | `The output is TRUE if an error is active and one of the variables iNumberOfSequences,` | — |
| `eErrorCode` | `returns the error number if an output bError is set (E_HVACErrorCodes [} 490]` | — |

**Description:** This function block represents a 2-point sequence controller. It is used for sequential staged control of discontinuous units. If can be used for cascades of vessels and in sequences of refrigerating machines or recooling plants for controlling the power stages. For each power stage an instance of the function block is used.

##### `FB_HVACBasicSequenceCtrl` — §5.1.4.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `iNumberOfSequences` | `INT` | — |
| `iMyNumberInSequence` | `INT` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rYManual` | `REAL` | — |
| `rInitialValue` | `REAL` | — |
| `bResetController` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `Input variable for enabling the controller. The controller is enabled if bEnable = TRUE.` | — |
| `iNumberOfSequences` | `Number of sequence controllers in the system.` | — |
| `iMyNumberInSequence` | `The controller’s own number in the sequence.` | — |
| `rW` | `The setpoint is transferred to the controller with the variable rW.` | — |
| `rX` | `Actual value of the control loop.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `Cycle time with which the control loop is processed. This must be greater than or equal to` | — |
| `eCtrlMode` | `The operation mode is selected via this enum (see E_HVACCtrlMode [} 489]).` | — |
| `rYManual` | `Manual value.` | — |
| `rInitialValue` | `The restart behavior of the controller is influenced by rInitialValue.` | — |
| `bResetContoller` | `A positive edge on the input bResetController resets the PID controller.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rDeadRange` | `REAL` | — |
| `bDirection` | `BOOL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rDeadBand` | `REAL` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `iCurrentSequence` | `INT` | — |
| `rDeadRange` | `In order to avoid unnecessary driving and hence premature wear of the valves or damper` | — |
| `bDirection` | `The control direction of the controller can be changed with the parameter bDirection. If` | — |
| `rKp` | `Proportional factor amplification. The variable is saved persistently. Preset to 1.` | — |
| `tTi` | `Integration time. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time. The variable is saved persistently. Preset to 0 s.` | — |
| `rDeadBand` | `If the control value is at the lower or upper limit of a controller and the actual value of the` | — |
| `rYMin` | `Lower limit of the working range of the controller. The variable is saved persistently. Preset to 0.` | — |
| `rYMax` | `Upper limit of the working range of the controller. The variable is saved persistently. Preset to 100.` | — |
| `iCurrentSequence` | `Currently active controller in the sequence.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `eState` | `E_HVACState` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Control signal output of the PID controller.` | — |
| `rXW` | `Control deviation` | — |
| `bMaxLimit` | `The output bMaxLimit is TRUE if the output rY has reached the value rYMax.` | — |
| `bMinLimit` | `The output bMinLimit is TRUE if the output rY has reached the value rYMin.` | — |
| `bActive` | `bActive is TRUE if the controller is active and enabled.` | — |
| `bARWactive` | `bARWactive is TRUE if the integral component of the controller has reached the lower or` | — |
| `eState` | `State of the controller (see E_HVACState) [} 493].` | — |
| `bError` | `This output indicates with a TRUE that there is an error.` | — |
| `eErrorCode` | `Contains the command-specific error code (see E_HVACErrorCodes [} 490]).` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** Unlike the sequence controllers FB_HVACPIDCooling [} 162], FB_HVACDehumidify [} 165], FB_HVACEnergyRecovery [} 168], FB_HVACPIDHumidify [} 171], FB_HVACPIDMixedAir [} 175], FB_HVACPIDPreHeating [} 178] and FB_HVACPIDReHeating [} 181], the function block FB_HVACBasicSequenceCtrl has no special system-specific extension or application. It is more general. The control direction of the controller is not automatically determined depending on the room air and outside air as is the case with the function blocks for heat recovery or the mixed air chamber.

##### `FB_HVACMasterSequenceCtrl` — §5.1.4.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `eModeSeqCtrl` | `E_HVACSequenceCtrlMode` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rYManual` | `REAL` | — |
| `bResetController` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `eModeSeqCtrl` | `The system operation mode is transferred to the controller with the enum eModeSeqCtrl` | — |
| `rX` | `Acquires the actual value of the control loop.` | — |
| `rW` | `The setpoint is transferred to the controller with the variable rW.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `Cycle time with which the control loop is processed. This must be greater than or equal to` | — |
| `eCtrlMode` | `The operation mode is selected via this Enum. Manual or automatic mode (see` | — |
| `rYManual` | `Manual value for the manual operation mode.` | — |
| `bResetController` | `The internal variables of the PID controller are reset.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rDeadRange` | `REAL` | — |
| `rInitialValue` | `REAL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `rDeadRange` | `In order to avoid unnecessary driving and hence premature wear of the valves or damper` | — |
| `rInitialValue` | `The restart behavior of the controller is influenced by rInitialValue (0..32767). The variable is` | — |
| `rKp` | `Proportional factor amplification. The variable is saved persistently. Preset to 1.` | — |
| `tTi` | `Integration time. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time. The variable is saved persistently. Preset to 0 s.` | — |
| `rYMin` | `Lower limit of the working range of the controller. The variable is saved persistently. Preset to 16.` | — |
| `rYMax` | `Upper limit of the working range of the controller. The variable is saved persistently. Preset to 25.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `eState` | `E_HVACState` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Set temperature value for the supply air temperature controller.` | — |
| `rXW` | `Control deviation.` | — |
| `bMaxLimit` | `The output bMaxLimit is TRUE if the output rY has reached the value rYMax.` | — |
| `bMinLimit` | `The output bMinLimit is TRUE if the output rY has reached the value rYMin.` | — |
| `bActive` | `bActive is TRUE if the controller is active and enabled.` | — |
| `bARWactive` | `bARWactive is TRUE if the integral component of the controller has reached the lower or` | — |
| `eState` | `State of the controller (see E_HVACState [} 493]).` | — |
| `bError` | `This output indicates with a TRUE that there is an error.` | — |
| `eErrorCode` | `Contains the command-specific error code (see E_HVACErrorCodes [} 490]).` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** Improved controllability can be achieved with the aid of the exhaust air or room temperature cascade control. The master controller measures the room or exhaust air temperature and adapts the setpoints for the supply air temperature to the room conditions. The setpoint for the supply air is limited by a minimum and a maximum value.

##### `FB_HVACPIDCooling` — §5.1.4.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `eModeSeqCtrl` | `E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `INT` | — |
| `iMyNumberInSequence` | `INT` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rYManual` | `REAL` | — |
| `bResetController` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `eModeSeqCtrl` | `Among other things, control is enabled via this enumeration (see E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `Number of sequence controllers in the system.` | — |
| `iMyNumberInSequence` | `The controller's own number from the sequence.` | — |
| `rW` | `The setpoint is transferred to the controller with the variable rW.` | — |
| `rX` | `Actual value of the control loop.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `The variable tCtrlCycleTime specifies the cycle time with which the PID controller is` | — |
| `eCtrlMode` | `The operation mode is selected via this enumeration (see E_HVACCtrlMode [} 489]). Manual or` | — |
| `rYManual` | `Manual value that is set at the output rY if eCtrlMode = eHVACCtrlMode_Manual.` | — |
| `bResetController` | `A positive edge on the input bResetController resets the PID controller.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rDeadRange` | `REAL` | — |
| `rInitialValue` | `REAL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rDeadBand` | `REAL` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `iCurrentSequence` | `INT` | — |
| `rDeadRange` | `In order to avoid unnecessary driving and hence premature wear of the valves or damper` | — |
| `rInitialValue` | `The restart behavior of the controller is influenced by rInitialValue. The variable is saved` | — |
| `rKp` | `Proportional factor amplification. The variable is saved persistently. Preset to 0.` | — |
| `tTi` | `Integration time. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time. The variable is saved persistently. Preset to 0 s.` | — |
| `rDeadBand` | `If the control value is at the lower or upper limit of a controller and the actual value of the` | — |
| `rYMin` | `Lower limit of the working range of the controller. The variable is saved persistently. Preset to 0.` | — |
| `rYMax` | `Upper limit of the working range of the controller. The variable is saved persistently. Preset to 100.` | — |
| `iCurrentSequence` | `Number of the active controller from the sequence.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `eState` | `E_HVACState` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Control signal output of the PID controller.` | — |
| `rXW` | `Control deviation.` | — |
| `bMaxLimit` | `The output bMaxLimit is TRUE if the output rY has reached the value rYMax.` | — |
| `bMinLimit` | `The output bMinLimit is TRUE if the output rY has reached the value rYMin.` | — |
| `bActive` | `bActive is TRUE if the controller is active and enabled.` | — |
| `bARWactive` | `bARWactive is TRUE if the integral component of the controller has reached the lower or` | — |
| `eState` | `State of the controller (see E_HVACState [} 493]).` | — |
| `bError` | `This output indicates with a TRUE that there is an error.` | — |
| `eErrorCode` | `E_HVACErrorCodes (see E_HVACErrorCodes [} 490]).` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** The control direction of the cooling controller is inverted compared to the heating controller. If the supply air setpoint is reduced, the sequence controller raises its control signal. In the operation mode Overheating Protection, the cooler regulates the supply air temperature to the minimum. Outside the normal sequence control operation the controller is also activated if overheating protection is active.

##### `FB_HVACPIDDehumidify` — §5.1.4.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `eModeSeqCtrl` | `E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `INT` | — |
| `iMyNumberInSequence` | `INT` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `rYFromPIDCooling` | `REAL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rYManual` | `REAL` | — |
| `bResetcontroller` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `eModeSeqCtrl` | `Among other things, control is enabled via this enum (see E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `Number of sequence controllers in the system.` | — |
| `iMyNumberInSequence` | `The controller’s own number in the sequence.` | — |
| `rW` | `The setpoint is transferred to the controller with the variable rW.` | — |
| `rX` | `Actual value of the control loop.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `The variable tCtrlCycleTime specifies the cycle time with which the PID controller is` | — |
| `rYFromPIDCooling` | `In the case of a VAC system with dehumidification, the output of the cooling controller` | — |
| `eCtrlMode` | `The operation mode is selected via this enum (see E_HVACCtrlMode [} 489]). Manual or` | — |
| `rYManual` | `Manual value that is set at the output rY if eCtrlMode = eHVACCtrlMode_Manual.` | — |
| `bResetController` | `A positive edge on the input bResetController resets the PID controller.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rDeadRange` | `REAL` | — |
| `rInitialValue` | `REAL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rDeadBand` | `REAL` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `iCurrentSequence` | `INT` | — |
| `rDeadRange` | `In order to avoid unnecessary driving and hence premature wear of the valves or damper` | — |
| `rInitialValue` | `The restart behavior of the controller is influenced by rInitialValue. The variable is saved` | — |
| `rKp` | `Proportional factor amplification. The variable is saved persistently. Preset to 0.` | — |
| `tTi` | `Integration time. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time. The variable is saved persistently. Preset to 0 s.` | — |
| `rDeadBand` | `If the control value is at the lower or upper limit of a controller and the actual value of the` | — |
| `rYMin` | `Lower limit of the working range of the controller. The variable is saved persistently. Preset to 0.` | — |
| `rYMax` | `Upper limit of the working range of the controller. The variable is saved persistently. Preset to 100.` | — |
| `iCurrentSequence` | `Number of the active controller from the sequence.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `eState` | `E_HVACState` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Control signal output of the PID controller.` | — |
| `rXW` | `Control deviation.` | — |
| `bMaxLimit` | `The output bMaxLimit is TRUE if the output rY has reached the value rYMax.` | — |
| `bMinLimit` | `The output bMinLimit is TRUE if the output rY has reached the value rYMin.` | — |
| `bActive` | `bActive is TRUE if the controller is active and enabled.` | — |
| `bARWactive` | `bARWactive is TRUE if the integral component of the controller has reached the lower or` | — |
| `eState` | `State of the controller (see E_HVACState [} 493]).` | — |
| `bError` | `This output indicates with a TRUE that there is an error.` | — |
| `eErrorCode` | `E_HVACErrorCodes (see E_HVACErrorCodes [} 490]).` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** In order to dehumidify the supply air, it is cooled down by the cooler. The dehumidification is constantly controlled by dosing the coolant at the cooling register with the cooler valve. Two controllers, FB_HVACPIDCooling [} 162] and FB_HVACPIDDehumidify, act on the cooler valve. First of all the control signal rY is relayed by FB_HVACPIDCooling [} 162] to the dehumidification sequence controller FB_HVACPIDDehumidify. Inside the FB_HVACPIDDehumidify function block, the larger of the two control signals is routed through to the controller output. If the air humidity is too high, the dehumidification controller takes precedence over the cooling controller. However, in order that the correct supply air temperature can still be achieved, the pre-heater is disabled if dehumidification is in operation. The reheater is put into operation in order to reheat the air.

##### `FB_HVACPIDEnergyRecovery` — §5.1.4.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `eModeSeqCtrl` | `E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `INT` | — |
| `iMyNumberInSequence` | `INT` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `rOutsideTemp` | `REAL` | — |
| `rReturnAirTemp` | `REAL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rYManual` | `REAL` | — |
| `bResetController` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `eModeSeqCtrl` | `Among other things, control is enabled via this enum (see E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `Number of sequence controllers in the system.` | — |
| `iMyNumberInSequence` | `The controller’s own number in the sequence.` | — |
| `rW` | `The setpoint is transferred to the controller with the variable rW.` | — |
| `rX` | `Actual value of the control loop.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `The variable tCtrlCycleTime specifies the cycle time with which the PID controller is` | — |
| `rOutsideTemp` | `Outside temperature.` | — |
| `rReturnAirTemp` | `Exhaust air temperature.` | — |
| `eCtrlMode` | `The operation mode is selected via this Enum. Manual or automatic mode (see` | — |
| `rYManual` | `Manual value that is set at the output rY if eCtrlMode = eHVACCtrlMode_Manual.` | — |
| `bResetController` | `A positive edge on the input bResetController resets the PID controller.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rDeadRange` | `REAL` | — |
| `rInitialValue` | `REAL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rDeadBand` | `REAL` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `iCurrentSequence` | `INT` | — |
| `rDeadRange` | `In order to avoid unnecessary driving and hence premature wear of the valves or damper` | — |
| `rInitialValue` | `The restart behavior of the controller is influenced by rInitialValue. The variable is saved` | — |
| `rKp` | `Proportional factor amplification. The variable is saved persistently. Preset to 0.` | — |
| `tTi` | `Integration time. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time. The variable is saved persistently. Preset to 0 s.` | — |
| `rDeadBand` | `If the control value is at the lower or upper limit of a controller and the actual value of the` | — |
| `rYMin` | `Lower limit of the working range of the controller. The variable is saved persistently. Preset to 0.` | — |
| `rYMax` | `Upper limit of the working range of the controller. The variable is saved persistently. Preset to 100.` | — |
| `iCurrentSequence` | `Number of the active controller from the sequence.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `eState` | `E_HVACState` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Control signal output of the PID controller.` | — |
| `rXW` | `Control deviation.` | — |
| `bMaxLimit` | `The output bMaxLimit is TRUE if the output rY has reached the value rYMax.` | — |
| `bMinLimit` | `The output bMinLimit is TRUE if the output rY has reached the value rYMin.` | — |
| `bActive` | `bActive is TRUE if the controller is active and enabled.` | — |
| `bARWactive` | `bARWactive is TRUE if the integral component of the controller has reached the lower or` | — |
| `eState` | `State of the controller (see E_HVACState [} 493]).` | — |
| `bError` | `This output indicates with a TRUE that there is an error.` | — |
| `eErrorCode` | `E_HVACErrorCodes (see E_HVACErrorCodes [} 490]).` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** One of the special features of the heat recovery controller is the reversal of its control direction depending on the outside temperature and the exhaust air temperature. The enthalpies of the outside air and the exhaust air can be used instead of their temperatures.

##### `FB_HVACPIDHumidify` — §5.1.4.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `eModeSeqCtrl` | `E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `INT` | — |
| `iMyNumberInSequence` | `INT` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `bHumidistat` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rYManual` | `REAL` | — |
| `bResetContoller` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `eModeSeqCtrl` | `Among other things, control is enabled via this enum (see E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `Number of sequence controllers in the system.` | — |
| `iMyNumberInSequence` | `The controller’s own number in the sequence.` | — |
| `rW` | `The setpoint is transferred to the controller with the variable rW.` | — |
| `rX` | `Actual value of the control loop.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `The variable tCtrlCycleTime specifies the cycle time with which the PID controller is` | — |
| `bHumidistat` | `A TRUE on this input enables the humidity control. The hygrostat is connected to this input so` | — |
| `eCtrlMode` | `The operation mode is selected via this Enum. Manual or automatic mode (see` | — |
| `rYManual` | `Manual value that is set at the output rY if eCtrlMode = eHVACCtrlMode_Manual.` | — |
| `bResetController` | `A positive edge on the input bResetController resets the PID controller.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rDeadRange` | `REAL` | — |
| `rInitialValue` | `REAL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rDeadBand` | `REAL` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `iCurrentSequence` | `INT` | — |
| `rDeadRange` | `In order to avoid unnecessary driving and hence premature wear of the valves or damper` | — |
| `rInitialValue` | `The restart behavior of the controller is influenced by rInitialValue. The variable is saved` | — |
| `rKp` | `Proportional factor amplification. The variable is saved persistently. Preset to 0.` | — |
| `tTi` | `Integration time. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time. The variable is saved persistently. Preset to 0 s.` | — |
| `rDeadBand` | `If the control value is at the lower or upper limit of a controller and the actual value of the` | — |
| `rYMin` | `Lower limit of the working range of the controller. The variable is saved persistently. Preset to 0.` | — |
| `rYMax` | `Upper limit of the working range of the controller. The variable is saved persistently. Preset to 100.` | — |
| `iCurrentSequence` | `Number of the active controller from the sequence.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `eState` | `E_HVACState` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Control signal output of the PID controller.` | — |
| `rXW` | `Control deviation.` | — |
| `bMaxLimit` | `The output bMaxLimit is TRUE if the output rY has reached the value rYMax.` | — |
| `bMinLimit` | `The output bMinLimit is TRUE if the output rY has reached the value rYMin.` | — |
| `bActive` | `bActive is TRUE if the controller is active and enabled.` | — |
| `bARWactive` | `bARWactive is TRUE if the integral component of the controller has reached the lower or` | — |
| `eState` | `State of the controller (see E_HVACState [} 493]).` | — |
| `bError` | `This output indicates with a TRUE that there is an error.` | — |
| `eErrorCode` | `E_HVACErrorCodes (see E_HVACErrorCodes [} 490]).` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** Besides the temperature, the humidity of the air is also controlled in some air conditioning systems. Steam humidifiers or air washers are used for humidification. For dehumidification the supply air can be cooled down to the dew point by a cooler. As a result, the moisture in the air condenses on the air cooler. The control of humidification and dehumidification is realized with a sequence incuding a humidification controller and a dehumidification controller.

##### `FB_HVACPIDMixedAir` — §5.1.4.7.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `eModeSeqCtrl` | `E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `INT` | — |
| `iMyNumberInSequence` | `INT` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `rOutsideTemp` | `REAL` | — |
| `rReturnAirTemp` | `REAL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rYManual` | `REAL` | — |
| `rYMin` | `REAL` | — |
| `bResetController` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `eModeSeqCtrl` | `Among other things, control is enabled via this enum (see E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `Number of sequence controllers in the system.` | — |
| `iMyNumberInSequence` | `The controller’s own number from the sequence.` | — |
| `rW` | `The setpoint is transferred to the controller with the variable rW.` | — |
| `rX` | `Actual value of the control loop.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `The variable tCtrlCycleTime specifies the cycle time with which the PID controller is` | — |
| `rOutsideTemp` | `Outside temperature` | — |
| `rReturnAirTemp` | `Exhaust air temperature` | — |
| `eCtrlMode` | `The operation mode is selected via this enum (see E_HVACCtrlMode [} 489]). Manual or` | — |
| `rYManual` | `Manual value that is set at the output rY if eCtrlMode = eHVACCtrlMode_Manual.` | — |
| `rYMin` | `Lower limit of the working range of the controller.` | — |
| `bResetController` | `A positive edge on the input bResetController resets the PID controller.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rDeadRange` | `REAL` | — |
| `rInitialValue` | `REAL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rDeadBand` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `iCurrentSequence` | `INT` | — |
| `rDeadRange` | `In order to avoid unnecessary driving and hence premature wear of the valves or damper` | — |
| `rInitialValue` | `The restart behavior of the controller is influenced by rInitialValue. The variable is saved` | — |
| `rKp` | `Proportional factor amplification. The variable is saved persistently. Preset to 0.` | — |
| `tTi` | `Integration time. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time. The variable is saved persistently. Preset to 0 s.` | — |
| `rDeadBand` | `If the control value is at the lower or upper limit of a controller and the actual value of the` | — |
| `rYMax` | `Upper limit of the working range of the controller. The variable is saved persistently. Preset to 100.` | — |
| `iCurrentSequence` | `Number of the active controller from the sequence.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `eState` | `E_HVACState` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Control signal output of the PID controller.` | — |
| `rXW` | `Control deviation.` | — |
| `bMaxLimit` | `The output bMaxLimit is TRUE if the output rY has reached the value rYMax.` | — |
| `bMinLimit` | `The output bMinLimit is TRUE if the output rY has reached the value rYMin.` | — |
| `bActive` | `bActive is TRUE if the controller is active and enabled.` | — |
| `bARWactive` | `bARWactive is TRUE if the integral component of the controller has reached the lower or` | — |
| `eState` | `State of the controller (see E_HVACState [} 493]).` | — |
| `bError` | `This output indicates with a TRUE that there is an error.` | — |
| `eErrorCode` | `E_HVACErrorCodes (see E_HVACErrorCodes [} 490]).` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** One of the special features of the mixed air controller is the reversal of its control direction depending on the outside temperature and the exhaust air temperature. The enthalpies of the outside air and the exhaust air can be used instead of their temperatures.

##### `FB_HVACPIDPreHeating` — §5.1.4.7.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `eModeSeqCtrl` | `E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `INT` | — |
| `iMyNumberInSequence` | `INT` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `rYDehumidify` | `REAL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rYManual` | `REAL` | — |
| `rInitialValue` | `REAL` | — |
| `bResetController` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `eModeSeqCtrl` | `Among other things, control is enabled via this enum (see E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `Number of sequence controllers in the system.` | — |
| `iMyNumberInSequence` | `The controller’s own number in the sequence.` | — |
| `rW` | `The setpoint is transferred to the controller with the variable rW.` | — |
| `rX` | `Actual value of the control loop.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `The variable tCtrlCycleTime specifies the cycle time with which the PID controller is` | — |
| `rYDehumidify` | `In the case of an VAC system with dehumidification, the control value of the dehumidification` | — |
| `eCtrlMode` | `The operation mode is selected via this enum (see E_HVACCtrlMode [} 489]). Manual or` | — |
| `rYManual` | `Manual value that is set at the output rY if eCtrlMode = eHVACCtrlMode_Manual.` | — |
| `rInitialValue` | `The restart behavior of the controller is influenced by rInitialValue.` | — |
| `bResetController` | `A positive edge on the input bResetController resets the PID controller.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rDeadRange` | `REAL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rDeadBand` | `REAL` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `iCurrentSequence` | `INT` | — |
| `rDeadRange` | `In order to avoid unnecessary driving and hence premature wear of the valves or damper` | — |
| `rKp` | `Proportional factor amplification. The variable is saved persistently. Preset to 0.` | — |
| `tTi` | `Integration time. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time. The variable is saved persistently. Preset to 0 s.` | — |
| `rDeadBand` | `If the control value is at the lower or upper limit of a controller and the actual value of the` | — |
| `rYMin` | `Lower limit of the working range of the controller. The variable is saved persistently. Preset to 0.` | — |
| `rYMax` | `Upper limit of the working range of the controller. The variable is saved persistently. Preset to 100.` | — |
| `iCurrentSequence` | `Number of the active controller from the sequence.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `eState` | `E_HVACState` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Control signal output of the PID controller.` | — |
| `rXW` | `Control deviation.` | — |
| `bMaxLimit` | `The output bMaxLimit is TRUE if the output rY has reached the value rYMax.` | — |
| `bMinLimit` | `The output bMinLimit is TRUE if the output rY has reached the value rYMin.` | — |
| `bActive` | `bActive is TRUE if the controller is active and enabled.` | — |
| `bARWactive` | `bARWactive is TRUE if the integral component of the controller has reached the lower or` | — |
| `eState` | `State of the controller (see E_HVACState [} 493]).` | — |
| `bError` | `This output indicates with a TRUE that there is an error.` | — |
| `eErrorCode` | `E_HVACErrorCodes (see E_HVACErrorCodes [} 490]).` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** This function block is a special application of a PID controller. Besides the PID control algorithm, the block contains the conditions for changing to the sequence of the previous or next sequence controller. A special feature of the pre-heating controller is the input rYDehumidify. As soon as the signal of rYDehumidify is greater than zero, the preheater is disabled and the value of iCurrentSequence is incremented by one. As a result of this the reheater FB_HVACPIDReHeating [} 181] is activated in place of the preheater. If the value of rYDehumidify returns to zero, the preheater is automatically reactivated. Outside the normal sequence control operation the controller is also activated in the case of backup operation (freeze protection).

##### `FB_HVACPIDReHeating` — §5.1.4.7.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `eModeSeqCtrl` | `E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `INT` | — |
| `iMyNumberInSequence` | `INT` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCtrlCycleTime` | `TIME` | — |
| `rYDehumidify` | `REAL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rYManual` | `REAL` | — |
| `bResetController` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `eModeSeqCtrl` | `Among other things, control is enabled via this enum (see E_HVACSequenceCtrlMode` | — |
| `iNumberOfSequences` | `Number of sequence controllers in the system.` | — |
| `iMyNumberInSequence` | `The controller’s own number in the sequence.` | — |
| `rW` | `The setpoint is transferred to the controller with the variable rW.` | — |
| `rX` | `Actual value of the control loop.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `tCtrlCycleTime` | `The variable tCtrlCycleTime specifies the cycle time with which the PID controller is` | — |
| `rYDehumidify` | `Constant value.` | — |
| `eCtrlMode` | `The operation mode is selected via this enum (see E_HVACCtrlMode [} 489]). Manual or` | — |
| `rYManual` | `Manual value that is set at the output rY if eCtrlMode = eHVACCtrlMode_Manual.` | — |
| `bResetController` | `A positive edge on the input bResetController resets the PID controller.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rDeadRange` | `REAL` | — |
| `rInitialValue` | `REAL` | — |
| `rKp` | `REAL` | — |
| `tTi` | `TIME` | — |
| `tTv` | `TIME` | — |
| `tTd` | `TIME` | — |
| `rDeadBand` | `REAL` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `iCurrentSequence` | `INT` | — |
| `rDeadRange` | `In order to avoid unnecessary driving and hence premature wear of the valves or damper` | — |
| `rInitialValue` | `The restart behavior of the controller is influenced by rInitialValue. The variable is saved` | — |
| `rKp` | `Proportional factor amplification. The variable is saved persistently. Preset to 0.` | — |
| `tTi` | `Integration time. The I-component corrects the residual control deviation following correction of the P-` | — |
| `tTv` | `Rate time. The larger tTv is, the stronger the controller corrects. The control loop becomes unstable if` | — |
| `tTd` | `Damping time. The variable is saved persistently. Preset to 0 s.` | — |
| `rDeadBand` | `If the control value is at the lower or upper limit of a controller and the actual value of the` | — |
| `rYMin` | `Lower limit of the working range of the controller. The variable is saved persistently. Preset to 0.` | — |
| `rYMax` | `Upper limit of the working range of the controller. The variable is saved persistently. Preset to 100.` | — |
| `iCurrentSequence` | `Number of the active controller from the sequence.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rXW` | `REAL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bActive` | `BOOL` | — |
| `bARWactive` | `BOOL` | — |
| `eState` | `E_HVACState` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rY` | `Control signal output of the PID controller.` | — |
| `rXW` | `Control deviation.` | — |
| `bMaxLimit` | `The output bMaxLimit is TRUE if the output rY has reached the value rYMax.` | — |
| `bMinLimit` | `The output bMinLimit is TRUE if the output rY has reached the value rYMin.` | — |
| `bActive` | `bActive is TRUE if the controller is active and enabled.` | — |
| `bARWactive` | `bARWactive is TRUE if the integral component of the controller has reached the lower or` | — |
| `eState` | `State of the controller (see E_HVACState [} 493]).` | — |
| `bError` | `This output indicates with a TRUE that there is an error.` | — |
| `eErrorCode` | `E_HVACErrorCodes (see E_HVACErrorCodes [} 490]).` | — |
| `bInvalidParameter` | `TRUE if an error has occurred during the plausibility check. The message must be` | — |

**Description:** The reheater is used to reheat the supply air that was cooled down by the cooler for dehumidification. The controller can only be enabled within the normal sequence control operation.

#### Programming › POUs › HVAC Room Functions › Air conditioning

##### `FB_BAREnergyLevel` — §5.1.5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bWindow` | `BOOL` | — |
| `bProtection` | `BOOL` | — |
| `bEconomy` | `BOOL` | — |
| `bPreComfort` | `BOOL` | — |
| `bComfort` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is activated by a TRUE at this input.` | — |
| `bWindow` | `The window contact is connected to this input. TRUE means that the window is OPEN. FALSE` | — |
| `bProtection` | `Protection mode is activated with the input bProtection. Protection mode is active if the input is` | — |
| `bEconomy` | `Economy mode is activated with the input bEconomy. Economy mode is active if the input is` | — |
| `bPreComfort` | `The Pre-comfort level is activated with this input. The Pre-comfort level is active if the input is` | — |
| `bComfort` | `The Comfort level is activated with this input if the room is occupied.` | — |
| `eCtrlMode` | `E_BAREnergyLevel` | — |
| `eCtrlMode` | `Using this ENUM the operation mode can be preselected from the building management level` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eEnergyLevel` | `E_BAREnergyLevel` | — |
| `bStateProtection` | `BOOL` | — |
| `bStateEconomy` | `BOOL` | — |
| `bStatePreComfort` | `BOOL` | — |
| `bStateComfort` | `BOOL` | — |
| `eEnergyLevel` | `This output contains the current energy level (see E_BAREnergyLevel [} 498]).` | — |
| `bStateProtection` | `The state of the bProtection input is relayed to the outside in the operation modes` | — |
| `bStateEconomy` | `The state of the bEconomy input is relayed to the outside in the operation modes` | — |
| `bStatePreComfort` | `The state of the bPreComfort input is relayed to the outside in the operation modes` | — |
| `bStateComfort` | `The state of the bComfort input is relayed to the outside in the operation modes` | — |

**Description:** This function block is for the adaptation of the supply of energy for the use of the building. The type of room utilization is set by the BMS. The longer a building or room is not used, the further the energy level can be lowered. The energy level currently selected by the function block is transferred to the room temperature controller.

##### `FB_BARFanCoil` — §5.1.5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `eCtrlFct` | `E_BARCtrlFct` | — |
| `rSetpoint` | `REAL` | — |
| `rRoomTemp` | `REAL` | — |
| `bStageUp` | `BOOL` | — |
| `bStageDown` | `BOOL` | — |
| `iStageManual` | `INT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is activated if the variable bEnable is TRUE. No fan stage is activated with a` | — |
| `eCtrlFct` | `This input is connected to the output eCtrlFct [} 498] of FB_BARFctSelection. This information is` | — |
| `rSetpoint` | `Input for the set temperature.` | — |
| `rRoomTemp` | `Input for the room temperature.` | — |
| `bStageUp` | `Local adjustment of the fan stage, stepwise increase by pushbutton.` | — |
| `bStageDown` | `Local adjustment of the fan stage, stepwise decrease by pushbutton.` | — |
| `iStageManual` | `The manual fan stage can be/is centrally adjusted via this input.` | — |
| `iStageManual` | `= 0, corresponds to the fan stage OFF` | — |
| `iStageManual` | `= 1, corresponds to the fan stage AUTO` | — |
| `iStageManual` | `= 2, corresponds to fan stage 01 active` | — |
| `iStageManual` | `= 3, corresponds to fan stage 02 active` | — |
| `iStageManual` | `= 4, corresponds to fan stage 03 active` | — |
| `bReset` | `Acknowledge input in case of a fault or an incorrect parameter.` | — |
| `rDeviationXW_Stage01` | `REAL` | — |
| `rDeviationXW_Stage02` | `REAL` | — |
| `rDeviationXW_Stage03` | `REAL` | — |
| `rHysteresisRange` | `REAL` | — |
| `udiSecMINPowerOnTime` | `UDINT` | — |
| `iFctModeFanCoil` | `INT` | — |
| `udiTimePeriod` | `UDINT` | — |
| `rDeviationXW_Stage01` | `Limit value of the control deviation for fan stage 01. The variable is saved` | — |
| `rDeviationXW_Stage02` | `Limit value of the control deviation for fan stage 02. The variable is saved` | — |
| `rDeviationXW_Stage03` | `Limit value of the control deviation for fan stage 03. The variable is saved` | — |
| `rHysteresisRange` | `Hysteresis range placed around the limit value.` | — |
| `Example` | `A limit value of 0.7 and a hysteresis range of 0.2 results in fan stage 01 being switched on at a` | — |
| `udiSecMINPowerOnTime` | `Minimum switch-on time that a fan must run for in a stage before switching to a` | — |
| `iFctModeFanCoil` | `The user has the possibility to activate the fan controller for heating mode or cooling` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bStageOff` | `BOOL` | — |
| `bStageAuto` | `BOOL` | — |
| `bStage01` | `BOOL` | — |
| `bStage02` | `BOOL` | — |
| `bStage03` | `BOOL` | — |
| `bFanCoilActive` | `BOOL` | — |
| `bPresence` | `BOOL` | — |
| `byState` | `BYTE` | — |
| `byError` | `BYTE` | — |
| `udiSecRT_MINPowerOnTime` | `UDINT` | — |
| `udiRT_TimePeriod` | `UDINT` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bStageOff` | `TRUE, fan stages are switched off.` | — |
| `bStageAuto` | `TRUE, fan controller is in automatic mode.` | — |
| `bStage01` | `TRUE, fan stage 01 active.` | — |
| `bStage02` | `TRUE, fan stage 02 active.` | — |
| `bStage03` | `TRUE, fan stage 03 active.` | — |
| `bFanCoilActive` | `TRUE if one of the three fan stages is active. This output can be/is used to enable` | — |
| `bPresence` | `TRUE means that presence was detected via the bStageUp, bStageDown or iStageManual` | — |
| `byState` | `Indicates the state of the fan controller.` | — |
| `byError` | `Output of the error as a byte.` | — |
| `udiSecRT_MINPowerOnTime` | `Indicates the remaining time of the minimum switch-on time.` | — |
| `udiRT_TimePeriod` | `Indicates the remaining time of the manual override.` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |

**Description:** The function block maps a 3-speed fan with the corresponding switching hysteresis, which is the same for all three speeds. The speed is set stepwise via the control deviation of the actual room temperature value from the set room temperature value. Furthermore there is a possibility to manually override the fan controller via the iStageManual or bStageUp or bStageDown input. A minimum switch-on time can be set via the udiSecMINPowerOnTime input, which is then valid for each stage.

#### (top level)

##### `1 0 1 Cooling TRUE OFF` — §1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bPipeSystem` | `BOOL` | — |
| `bFeedbHeatMedium` | `BOOL` | — |
| `bFeedbCoolMedium` | `BOOL` | — |
| `bDewPoint` | `BOOL` | — |
| `rRoomTemp` | `REAL` | — |
| `stSetpoint` | `ST_BARSetpointRoom` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `TRUE enables the function block. If FALSE the function block is disabled and eCtrlFct :=` | — |
| `bPipeSystem` | `FALSE means that a 2 pipe system is present. TRUE means that a 4 pipe system is present.` | — |
| `bFeedbHeatMedium` | `Signal from the power generation or distribution that heating medium is available.` | — |
| `bFeedbCoolMedium` | `Signal from the power generation or distribution that cooling medium is available.` | — |
| `bDewPoint` | `The dew point sensor is connected to this input. If this is triggered the cooling control function is` | — |
| `rRoomTemp` | `This input variable transfers the current room temperature to the function block.` | — |
| `stSetpoint` | `STRUCTURE containing the setpoints for the individual energy levels (see` | — |
| `uiChangeOverDelay` | `UINT` | — |
| `uiChangeOverDelay` | `Changeover time between the control functions. Must be specified in seconds. If the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eCtrlFct` | `E_BARCtrlFct` | — |
| `uiReqHeating` | `UINT` | — |
| `uiReqCooling` | `UINT` | — |
| `udiRT_ChangeOverDelay` | `UDINT` | — |
| `eCtrlFct` | `This output contains the current control function (see E_BARCtrlFct [} 498]).` | — |
| `uiReqHeating` | `is 1 if the room/zone requests heating energy. It is 0 if there is no heating requirement.` | — |
| `uiReqCooling` | `is 1 if the room/zone requests cooling energy. It is 0 if there is no cooling requirement.` | — |
| `udiRT_ChangeOverDelay` | `Indicates the time remaining until the active control function is changed over.` | — |

#### Programming › POUs › HVAC Room Functions › Air conditioning

##### `FB_BARSetpointRoom` — §5.1.5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rSetpointShiftHeat` | `REAL` | — |
| `rSetpointShiftCool` | `REAL` | — |
| `rSetpointShiftUser` | `REAL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is activated if the variable bEnable is TRUE.` | — |
| `rSetpointShiftHeat` | `the rSetpointShiftHeat variable is used for the adaptation of the ComfortHeating` | — |
| `rSetpointShiftCool` | `The rSetpointShiftCool variable is used for the adaptation of the ComfortCooling` | — |
| `rSetpointShiftUser` | `The rSetpointShiftUser variable is used for the user's local setpoint value shift.` | — |
| `bReset` | `Acknowledge input in case of a fault or an incorrect parameter.` | — |
| `rSetpointComfortHeat` | `REAL` | — |
| `rSetpointPreComfortHeat` | `REAL` | — |
| `rSetpointEconomyHeat` | `REAL` | — |
| `rSetpointProtectionHeat` | `REAL` | — |
| `rSetpointComfortCool` | `REAL` | — |
| `rSetpointPreComfortCool` | `REAL` | — |
| `rSetpointEconomyCool` | `REAL` | — |
| `rSetpointProtectionCool` | `REAL` | — |
| `rSetpointComfortHeat` | `Setpoint for the Comfort heating energy level. The variable is saved persistently.` | — |
| `rSetpointPreComfortHeat` | `Setpoint for the PreComfort heating energy level. The variable is saved` | — |
| `rSetpointEconomyHeat` | `Setpoint for the Economy heating energy level. The variable is saved persistently.` | — |
| `rSetpointProtectionHeat` | `Setpoint for the Protection heating energy level. The variable is saved` | — |
| `rSetpointComfortCool` | `Setpoint for the Comfort cooling energy level. The variable is saved persistently.` | — |
| `rSetpointPreComfortCool` | `Setpoint for the PreComfort cooling energy level. The variable is saved` | — |
| `rSetpointEconomyCool` | `Setpoint for the Economy cooling energy level. The variable is saved persistently.` | — |
| `rSetpointProtectionCool` | `Setpoint for the Protection cooling energy level. The variable is saved` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSetpoint` | `ST_BARSetpointRoom` | — |
| `bInvalidParameter` | `BOOL` | — |
| `stSetpoint` | `Structure containing the setpoints for all the energy levels (see ST_BARSetpointRoom [} 502]).` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |

**Description:** The function block FB_BARSetpointRoom assigns setpoints for cooling and heating operation to each of the energy levels Protection, Economy, PreComfort and Comfort. In connection with the function block FB_BAREnergyLevel the room temperature controller is provided with an energetical optimum setpoint regarding the use of the room and the selected heating or cooling operation.

#### Programming › POUs › HVAC Room Functions › Controller

##### `FB_BARPICtrl` — §5.1.5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rW` | `REAL` | — |
| `rX` | `REAL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `uiCtrlCycleCall` | `UINT` | — |
| `bSync` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `Controller activation` | — |
| `rW` | `Setpoint.` | — |
| `rX` | `Actual value.` | — |
| `tTaskCycleTime` | `Cycle time with which the function block is called. If the function block is called in every` | — |
| `uiCtrlCycleCall` | `Call cycle of the function block as a multiple of the cycle time. A zero entry is automatically` | — |
| `Example` | `tTaskCycleTime = 20ms, uiCtrlCycleCall =10 -> The control algorithm is called every 200 ms. Thus` | — |
| `bSync` | `A rising edge at this input sets the (internal) I-component such that rSyncValue is output at the` | — |
| `bDirection` | `BOOL` | — |
| `rXp` | `REAL` | — |
| `tTn` | `TIME` | — |
| `rYMin` | `REAL` | — |
| `rYMax` | `REAL` | — |
| `rSyncValue` | `REAL` | — |
| `bDirection` | `The control direction [} 201] of the controller can be changed with the parameter bDirection. If` | — |
| `rXp` | `Proportional band. This defines the internal proportional factor, see below. The proportionality factor or` | — |
| `tTn` | `Integral action time in seconds. The I-component corrects the residual control deviation following the` | — |
| `rSyncValue` | `The control value rY is set to this value on a rising edge at the input bSync. In addition, the I-` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rY` | `REAL` | — |
| `rE` | `REAL` | — |
| `bARW` | `BOOL` | — |
| `bMaxLimit` | `BOOL` | — |
| `bMinLimit` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `rY` | `Control value.` | — |
| `rE` | `Control deviation (calculation dependent on control direction [} 201])` | — |
| `rEMin` | `lower control deviation limit value, which results from the input proportional band.` | — |
| `rEMax` | `upper control deviation limit value, which results from the input proportional band.` | — |
| `bARW` | `Anti-Reset-Windup function is active.` | — |
| `bMaxLimit` | `The control value has reached its upper limit value.` | — |
| `bMinLimit` | `The control value has reached its lower limit value.` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Simple PI controller. The control gain has no influence on the I-component.

#### Programming › POUs › HVAC Room Functions › Lighting functions – overview

##### `FB_BARAutomaticLight` — §5.1.5.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `uiLightCtrlMode` | `UINT` | — |
| `bSwitch` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiLightCtrlMode` | `Operation mode.` | — |
| `bSwitch` | `A rising edge switches the light on in automatic mode (uiLightCtrlMode=0), a falling edge starts` | — |
| `uiHoldTime` | `UINT` | — |
| `lrOnDimValue` | `LREAL` | — |
| `lrManualDimValue` | `LREAL` | — |
| `uiHoldTime` | `Holding time [s] of the automatic light controller after a falling edge on bSwitch.` | — |
| `lrOnDimValue` | `A dimming value in automatic mode (uiLightCtrlMode=0).` | — |
| `lrManualDimValue` | `Output dimming value in Manual On mode (uiLightCtrlMode=1).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrDimValue` | `LREAL` | — |
| `uiRemainingHoldTime` | `UINT` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `lrDimValue` | `Output dimming value for the lighting in percent.` | — |
| `uiRemainingTimeHold` | `Remaining holding time in seconds. If the light is off or if manual operation mode is` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Function block for an automatic light circuit as used in corridors or sanitary facilities.

##### `FB_BARConstantLightControl` — §5.1.5.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `uiLightCtrlMode` | `UINT` | — |
| `bSwitch` | `BOOL` | — |
| `bPresence` | `BOOL` | — |
| `bCentralOn` | `BOOL` | — |
| `bCentralOff` | `BOOL` | — |
| `uiBrightness` | `UINT` | — |
| `uiSetpoint` | `UINT` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiLightCtrlMode` | `Operation mode.` | — |
| `bSwitch` | `Rising edges on this input switch the control on and off alternately. When switching off the output` | — |
| `bPresence` | `A continuous TRUE signal on this input activates the control if the presence function is` | — |
| `bCentralOn` | `A positive edge on this input switches the control on. This command input is active only in` | — |
| `bCentralOff` | `A positive edge on this input switches the control off and sets the output lrDimValue to 0.0.` | — |
| `uiBrightness` | `Light actual value [lux].` | — |
| `uiSetpointValue` | `Light setpoint value [lux].` | — |
| `uiSwitchOverTime` | `UINT` | — |
| `uiAdjustRampTime` | `UINT` | — |
| `bPresenceOnActive` | `BOOL` | — |
| `bInitialMode` | `BOOL` | — |
| `uiTargetRange` | `UINT` | — |
| `uiControlRampTime` | `UINT` | — |
| `uiOnDelay` | `UINT` | — |
| `uiOffDelay` | `UINT` | — |
| `lrMinDimValue` | `LREAL` | — |
| `lrMaxDimValue` | `LREAL` | — |
| `lrStartDimValue` | `LREAL` | — |
| `lrManualDimValue` | `LREAL` | — |
| `uiSwitchOverTime` | `Switching time in milliseconds for the input bSwitch for the recognition of short and long` | — |
| `uiAdjustRampTime` | `Ramp time in seconds with which the setpoint value is changed during manual` | — |
| `bPresenceOnActive` | `If this input is TRUE, then the control is activated by a positive edge on bPresence if` | — |
| `bInitialMode` | `A TRUE signal on this input makes the function block begin with the setpoint value on` | — |
| `uiTargetRange` | `Target range around the setpoint value in which no further control is to take place.` | — |
| `uiControlRampTime` | `Ramp time in seconds (time required to dim from 0% to 100%).` | — |
| `uiOnDelay` | `Switch-on delay in seconds around the minimum value lrMinDimValue.` | — |
| `uiOffDelay` | `Switch-off delay in seconds around the minimum value lrMinDimValue.` | — |
| `lrMinDimValue` | `Lower limit value for dimming, see introduction [} 206].` | — |
| `lrMaxDimValue` | `Upper limit to which the output lrDimValue can be controlled.` | — |
| `lrStartDimValue` | `Value to which the light should jump on activating the control.` | — |
| `lrManualDimValue` | `Output dimming value in Manual On mode (uiLightCtrlMode=1).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrDimValue` | `LREAL` | — |
| `bControlActive` | `BOOL` | — |
| `bAdjustedSetpointActive` | `BOOL` | — |
| `uiAdjustedSetpoint` | `UINT` | — |
| `diActDeviation` | `DINT` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `lrDimValue` | `Light output value, 0..100%.` | — |
| `bControlActive` | `This output is TRUE if the function block is in automatic mode and the control is activated.` | — |
| `bAdjustedSetpointActive` | `If the control is active and the setpoint value has been manually adjusted (see` | — |
| `uiAdjustedSetpoint` | `This output indicates the active setpoint value if this has been manually adjusted` | — |
| `diActDeviation` | `Current control deviation in lux. This output indicates a valid value only if the function block` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** The constant light control function block controls the lighting when the room is occupied such that the lighting intensity does not fall below a preset minimum. This ensures adequate lighting whilst at the same time minimizing energy consumption.

##### `FB_BARDaylightControl` — §5.1.5.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `uiLightCtrlMode` | `UINT` | — |
| `bSwitch` | `BOOL` | — |
| `bCentralOn` | `BOOL` | — |
| `bCentralOff` | `BOOL` | — |
| `uiBrightness` | `UINT` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiLightCtrlMode` | `Operation mode.` | — |
| `bSwitch` | `Rising edges on this input switch the control on and off alternately. When switching off the output` | — |
| `bCentralOn` | `A positive edge on this input switches the control on. This command input is active only in` | — |
| `bCentralOff` | `A positive edge on this input switches the control off and sets the output lrDimValue to 0.0.` | — |
| `uiBrightness` | `Light actual value [lux].` | — |
| `uiSwitchOnValue` | `UINT` | — |
| `uiSwitchOffOffset` | `UINT` | — |
| `uiLightOnTime` | `UINT` | — |
| `uiOnDelay` | `UINT` | — |
| `uiOffDelay` | `UINT` | — |
| `uiSwitchOnValue` | `Light switch-on value. The lighting is switched on after the expiry of the switch-on delay if` | — |
| `uiSwitchOffOffset` | `If the light is switched on for the time uiLightOnTime (see above), the light switch-off` | — |
| `uiLightOnTime` | `The lighting does not reach its true switch-on value immediately. In order to judge the` | — |
| `uiOnDelay` | `Switch-on delay in seconds. Constant switching on and off of the lighting is perceived as very` | — |
| `uiOffDelay` | `Switch-off delay in seconds. If the external brightness increases in such a manner that the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrDimValue` | `LREAL` | — |
| `bOn` | `BOOL` | — |
| `uiRemainingOnDelay` | `UINT` | — |
| `uiRemainingOffDelay` | `UINT` | — |
| `uiPreOnValue` | `UINT` | — |
| `uiLightGain` | `UINT` | — |
| `uiSwitchOffValue` | `UINT` | — |
| `bControlActive` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `lrDimValue` | `In order to keep this function the same as the other light user functions, a light output value` | — |
| `bOn` | `Switch output for the lighting.` | — |
| `uiRemainingOnDelay` | `Countdown of the switch-on delay in seconds. This output is 0 as long as no` | — |
| `uiRemainingOffDelay` | `Countdown of the switch-off delay in seconds. This output is 0 as long as no` | — |
| `uiPreOnValue` | `Measured light value immediately before switching the lighting on. This output is "0" if the` | — |
| `uiLightGain` | `Calculated brightness increase after switching on the lighting and expiry of an additional` | — |
| `uiSwitchOffValue` | `Determined re-switch-off value, wherein the measured brightness must be larger. During` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Daylight switch. Unlike the constant light control [} 206], this automatic switch operates not with dimming values, but merely switches the light on or off depending on the measured brightness.

##### `FB_BARLightActuator` — §5.1.5.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `lrTargetValue` | `LREAL` | — |
| `usiSelectedScene` | `USINT` | — |
| `bCallScene` | `BOOL` | — |
| `bStopScene` | `BOOL` | — |
| `bSaveScene` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrTargetValue` | `Target value of the light output in 0..100%` | — |
| `usiSelectedScene` | `Selected lighting scene, 0..20` | — |
| `bCallScene` | `Sets the output value, controlled by a ramp, to the light value entered in the index` | — |
| `bStopScene` | `Resets the output value to the value lrTargetValue. The change is likewise controlled by a` | — |
| `bSaveScene` | `Saves the current light value at the output lrDimvalue in the light value table arrLightScenes` | — |
| `uiSecDimTime` | `UINT` | — |
| `arrLightScenes` | `ARRAY[0..20] OF LREAL` | — |
| `uiSecDimTime` | `Ramp time in seconds. This is the time that the light actuator function requires in order to` | — |
| `arrLightScenes` | `Table of saved light values.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrDimValue` | `LREAL` | — |
| `iDimValue` | `INT` | — |
| `bOn` | `BOOL` | — |
| `bSceneMode` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `lrDimValue` | `Output light value in 0..100%.` | — |
| `iDimValue` | `Output light value in 0..32767.` | — |
| `bOn` | `Output light state: lrDimmValue=0.0 => bOn=FALSE - lrDimmValue>0.0 => bOn=TRUE.` | — |
| `bSceneMode` | `The function block presently outputs a scene value and not the value lrTargetValue on the` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block serves to control a conventional light actuator. The outputs cover the value ranges 0..100%, 0..32767 and On/Off. On top of that the function block contains a scene memory in which up to 21 different light values can be saved.

##### `FB_BARLightCircuit` — §5.1.5.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `uiLightCtrlMode` | `UINT` | — |
| `bSwitch` | `BOOL` | — |
| `bCentralOn` | `BOOL` | — |
| `bCentralOff` | `BOOL` | — |
| `uiLightCtrlMode` | `Operation mode.` | — |
| `bSwitch` | `Rising edges on bSwitch make the output lrDimValue change each time between 0.0 and 100.0.` | — |
| `bCentralOn` | `Switches the output lrDimValueOut to 100.0.` | — |
| `bCentralOff` | `Switches the output lrDimValueOut to 0.0.` | — |
| `Note` | `All switching commands, bSwitch, bCentralOn and bCentralOff are effective only in automatic mode.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrDimValue` | `LREAL` | — |
| `bLight` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `lrDimValue` | `Light output value in percent; 0.0 if the light is switched off and 100.0 if the light is switched on.` | — |
| `bLight` | `FALSE if lrDimmValue = 0.0, otherwise TRUE.` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block represents a simple light switching circuit without a dimming function.

##### `FB_BARLightCircuitDim` — §5.1.5.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `uiLightCtrlMode` | `UINT` | — |
| `bSwitch` | `BOOL` | — |
| `bCentralOn` | `BOOL` | — |
| `bCentralOff` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiLightCtrlMode` | `Operation mode.` | — |
| `bSwitch` | `A short TRUE signal that is shorter than uiSwitchOverTime in milliseconds switches the output` | — |
| `bCentralOn` | `Switches the output lrDimValueOut to the previously saved (at the last switch-off) dimming` | — |
| `bCentralOff` | `Switches the output lrDimValueOut to 0.0.` | — |
| `Note` | `All switching commands, bSwitch, bCentralOn and bCentralOff are effective only in automatic mode.` | — |
| `bLight` | `FALSE if lrDimmValue = 0.0, otherwise TRUE.` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |
| `uiSwitchOverTime` | `UINT` | — |
| `uiDimRampTime` | `UINT` | — |
| `uiCycleDelay` | `UINT` | — |
| `lrMinDimValue` | `LREAL` | — |
| `lrMaxDimValue` | `LREAL` | — |
| `lrManualDimValue` | `LREAL` | — |
| `uiSwitchOverTime` | `Switching time in milliseconds from button to dimming mode.` | — |
| `uiDimRampTime` | `Ramp time in seconds (time required to dim from 0% to 100%).` | — |
| `uiCycleDelay` | `Switching time in milliseconds of the automatic change between dimming up and dimming` | — |
| `lrDimMinValue` | `Minimum light value in % that can be reached by dimming.` | — |
| `lrDimMaxValue` | `Maximum light value in %.` | — |
| `lrManualDimValue` | `Output dimming value in Manual On mode (uiLightCtrlMode=1).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrDimValue` | `LREAL` | — |
| `bLight` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `lrDimValue` | `Light output value; can adopt values between lrMinDimValue and lrMaxDimValue, see` | — |

**Description:** This function block represents a light switching circuit with a dimming function.

##### `FB_BARStairwellAutomatic` — §5.1.5.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `uiLightCtrlMode` | `UINT` | — |
| `bSwitch` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiLightCtrlMode` | `Operation mode.` | — |
| `bSwitch` | `A rising edge switches the light on in automatic mode (uiLightCtrlMode=0), a falling edge starts` | — |
| `uiHoldTime` | `UINT` | — |
| `uiPulseTimeOn` | `UINT` | — |
| `uiPulseTimeOff` | `UINT` | — |
| `uiNumberPulses` | `UINT` | — |
| `uiHoldTime` | `Holding time [s] of the stairwell controller after a falling edge on bSwitch.` | — |
| `uiPulseTimeOn` | `ON-time of the early warning pulses in milliseconds.` | — |
| `uiPulseTimeOff` | `OFF-time of the early warning pulses in milliseconds.` | — |
| `uiNumberPulses` | `Number of early warning pulses.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrDimValue` | `LREAL` | — |
| `bOn` | `BOOL` | — |
| `uiRemainingHoldTime` | `UINT` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `lrDimValue` | `In order to keep this function the same as the other light user functions, a light output value` | — |
| `bOn` | `Switch output for the lighting.` | — |
| `uiRemainingTimeHold` | `Remaining holding time in seconds. If the light is off or if manual operation mode is` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Function block for a stairwell light circuit.

##### `FB_BARTwilightAutomatic` — §5.1.5.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `uiLightCtrlMode` | `UINT` | — |
| `bSwitch` | `BOOL` | — |
| `bCentralOn` | `BOOL` | — |
| `bCentralOff` | `BOOL` | — |
| `uiBrightness` | `UINT` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiLightCtrlMode` | `Operation mode.` | — |
| `bSwitch` | `Positive edges on this input switch the automatic twilight function on and off alternately. When` | — |
| `bCentralOn` | `A positive edge on this input switches the automatic twilight function on. This command input is` | — |
| `bCentralOff` | `A positive edge on this input switches the automatic twilight function off and sets the output` | — |
| `uiBrightness` | `Light actual value [lux].` | — |
| `uiSwitchOnValue` | `UINT` | — |
| `uiSwitchOffValue` | `UINT` | — |
| `uiOnDelay` | `UINT` | — |
| `uiOffDelay` | `UINT` | — |
| `lrDimOnValue` | `LREAL` | — |
| `lrDimOffValue` | `LREAL` | — |
| `lrManualDimValue` | `LREAL` | — |
| `uiSwitchOnValue` | `Switch-on threshold value. This is compared with the brightness value at the input` | — |
| `uiSwitchOffValue` | `Switch-off threshold value. This is compared with the brightness value at the input` | — |
| `uiOnDelay` | `Switch-on delay in seconds.` | — |
| `uiOffDelay` | `Switch-off delay in seconds.` | — |
| `lrDimOnValue` | `Switch-on light value in %.` | — |
| `lrDimOffValue` | `Switch-off light value in %.` | — |
| `lrManualDimValue` | `Output dimming value in Manual On mode (uiLightCtrlMode=1).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrDimValue` | `LREAL` | — |
| `uiRemainingOnDelay` | `UINT` | — |
| `uiRemainingOffDelay` | `UINT` | — |
| `bControlActive` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `lrDimValue` | `Light output value, 0..100%.` | — |
| `uiRemainingOnDelay` | `Countdown of the switch-on delay in seconds. This output is 0 as long as no` | — |
| `uiRemainingOffDelay` | `Countdown of the switch-off delay in seconds. This output is 0 as long as no` | — |
| `bControlActive` | `This output is TRUE if the function block is in automatic mode and the automatic twilight` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Automatic twilight function

#### Programming › POUs › HVAC Room Functions › Sun protection

##### `FB_BARBlindPositionEntry` — §5.1.5.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `lrSunElevation1` | `REAL` | — |
| `uiBlindPosition1` | `UINT` | — |
| `lrSunElevation2` | `REAL` | — |
| `uiBlindPosition2` | `UINT` | — |
| `lrSunElevation3` | `REAL` | — |
| `uiBlindPosition3` | `UINT` | — |
| `lrSunElevation4` | `REAL` | — |
| `uiBlindPosition4` | `UINT` | — |
| `lrSunElevation1` | `Sun position at the first interpolation point (0°..90°).` | — |
| `uiBlindPosition1` | `Blind position (degree of closure) at the first interpolation point (0%..100%).` | — |
| `lrSunElevation2` | `Sun position at the second interpolation point (0°..90°).` | — |
| `uiBlindPosition2` | `Blind position (degree of closure) at the second interpolation point (0%..100%).` | — |
| `lrSunElevation3` | `Sun position at the third interpolation point (0°..90°).` | — |
| `uiBlindPosition3` | `Blind position (degree of closure) at the third interpolation point (0%..100%).` | — |
| `lrSunElevation4` | `Sun position at the fourth interpolation point (0°..90°).` | — |
| `uiBlindPosition4` | `Blind position (degree of closure) at the fourth interpolation point (0%..100%).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL` | — |
| `uiErrorId` | `UDINT` | — |
| `stBlindPositionTable` | `ST_BARBlindPositionTable` | — |
| `bValid` | `this output will be TRUE as long as the entries correspond to the criteria listed above.` | — |
| `uiErrorId` | `contains the error code, if the entries should not correspond to the criteria listed above. See Error` | — |
| `stBlindPositionTable` | `Transfer structure of the interpolation points, see ST_BARBlindPositionTable [} 501].` | — |

**Description:** This function block serves for the input of interpolation points for the function block FB_BARSunProtectionEx [} 305], if this should be operated in the height positioning mode with the help of a table, see E_BARPosMode [} 500].

##### `FB_BARDelayedHysteresis` — §5.1.5.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `uiBrightness` | `UINT` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiBrightness` | `External brightness in lux.` | — |
| `uiOnValue` | `UINT` | — |
| `uiOffValue` | `UINT` | — |
| `uiOnDelay` | `UINT` | — |
| `uiOnDelay` | `UINT` | — |
| `uiOnValue` | `Switch-on threshold value in lux. This must be larger than the switch-off threshold value` | — |
| `uiOffValue` | `Switch-off threshold value in lux. This must be smaller than the switch-on threshold value` | — |
| `uiOnDelay` | `Switch-on delay in seconds.` | — |
| `uiOffDelay` | `Switch-off delay in seconds.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `bOut` | `binary delayed output of the threshold switch` | — |
| `bError` | `this output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block represents a threshold switch for brightness. The switch-on and switch-off behavior can additionally be delayed.

##### `FB_BARFacadeElementEntry` — §5.1.5.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `iColumn` | `INT` | — |
| `iRow` | `INT` | — |
| `bWrite` | `BOOL` | — |
| `bRead` | `BOOL` | — |
| `usiGroup` | `USINT` | — |
| `lrCorner1X` | `LREAL` | — |
| `lrCorner1Y` | `LREAL` | — |
| `lrWindowWidth` | `LREAL` | — |
| `lrWindowHeight` | `LREAL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `iColumn` | `Column index of the selected component on the facade. This refers to the selection of a field` | — |
| `iRow` | `dto. Row index. iRow and iColumn may not be zero! This arises from the field definition, see above.` | — |
| `bRead` | `With a positive edge on this input, the information from the selected element arrFacadeElement` | — |
| `bWrite` | `A rising edge writes both the entered values and the calculated values into the selected field` | — |
| `usiGroup` | `Group membership,` | — |
| `lrCorner1X` | `X-coordinate of corner point 1 in meters.` | — |
| `lrCorner1Y` | `Y-coordinate of corner point 1 in meters.` | — |
| `lrWindowWidth` | `Window width in meters.` | — |
| `lrWindowHeight` | `Window height in meters.` | — |
| `arrFacadeElement` | `ARRAY[1..iColumnsPerFacade, 1..iRowsPerFacade] OF ST_BARFacadeElement` | — |
| `arrFacadeElement` | `List of facade elements [} 315].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrCorner2X` | `LREAL` | — |
| `lrCorner2Y` | `LREAL` | — |
| `lrCorner3X` | `LREAL` | — |
| `lrCorner3Y` | `LREAL` | — |
| `lrCorner4X` | `LREAL` | — |
| `lrCorner4Y` | `LREAL` | — |
| `bValid` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `lrCorner2X` | `determined X-coordinate of corner 2 of the window in metres. See "Note" above.` | — |
| `lrCorner2Y` | `determined Y-coordinate of corner 2 of the window in metres. See "Note" above.` | — |
| `lrCorner3X` | `determined X-coordinate of corner 3 of the window in metres. See "Note" above.` | — |
| `lrCorner3Y` | `determined Y-coordinate of corner 3 of the window in metres. See "Note" above.` | — |
| `lrCorner4X` | `determined X-coordinate of corner 4 of the window in metres. See "Note" above.` | — |
| `lrCorner4Y` | `determined Y-coordinate of corner 4 of the window in metres. See "Note" above.` | — |
| `bValid` | `result check for the entered values.` | — |
| `udiErrorId` | `contains the error code if the values entered are not OK. See Error codes [} 317].` | — |

**Description:** This function block serves the administration of all facade elements (windows) in a facade, which are saved globally in a list of facade elements [} 315]. It is intended to facilitate the input of the element information - also with regard to the use of the target visualization. A schematic illustration of the objects with description of the coordinates is given in Shading correction: principles and definitions [} 239]. The facade elements are declared in the global variables as a two-dimensional field above the window columns and rows:

##### `FB_BARReadFacadeElementList` — §5.1.5.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | — |
| `sDataFile` | `STRING` | — |
| `sLogFile` | `STRING` | — |
| `sNetId` | `STRING` | — |
| `bStart` | `A TRUE edge on this input starts the reading process.` | — |
| `sDataFile` | `Contains the path and file name for the data file to be opened. This must have been saved in` | — |
| `sLogFile` | `dto. Log file for the accumulating errors. This file is overwritten each time the function block is` | — |
| `sNetId` | `A string can be entered here with the AMS Net ID of the TwinCAT computer on which the files are to` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrFacadeElement` | `ARRAY[1..iColumnsPerFacade, 1..iRowsPerFacade] OF ST_BARFacadeElement` | — |
| `arrFacadeElement` | `List of facade elements [} 315].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `uiAmountOfSetsRead` | `UINT` | — |
| `bBusy` | `This output is TRUE as long as elements are being read from the file.` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous or if an error has occurred` | — |
| `udiErrorId` | `Contains the error code of the error that occurred last. See Error codes [} 317] or ADS_Error` | — |
| `uiAmountOfSetsRead` | `Number of data sets read` | — |

**Description:** With the help of this function block, data for facade elements (windows) can be imported from a pre-defined Excel table in csv format into the list of facade elements [} 315]. In addition the imported data are checked for plausibility and errors are written to a log file.

##### `FB_BARReadShadingObjectsList` — §5.1.5.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | — |
| `sDataFile` | `STRING` | — |
| `sLogFile` | `STRING` | — |
| `sNetId` | `STRING` | — |
| `bStart` | `A TRUE edge on this input starts the reading process.` | — |
| `sDataFile` | `Contains the path and file name for the data file to be opened. This must have been saved in` | — |
| `sNetId` | `A string can be entered here with the AMS Net ID of the TwinCAT computer on which the files are to` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrShadingObject` | `ARRAY[1..iShadingObjects] OF ST_BARShadingObject` | — |
| `arrShadingObject` | `List of shading objects [} 316].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `uiAmountOfSetsRead` | `UINT` | — |
| `bBusy` | `This output is TRUE as long as elements are being read from the file.` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous or if an error has occurred` | — |
| `udiErrorId` | `Contains the error code of the error that occurred last. See Error codes [} 317] or ADS_Error` | — |
| `uiAmountOfSetsRead` | `Number of data sets read.` | — |

**Description:** With the help of this function block, data for shading objects can be imported from a pre-defined Excel table in csv format into the list of shading objects [} 316]. In addition the imported data are checked for plausibility and errors are written to a log file.

##### `FB_BARShadingCorrection` — §5.1.5.4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `stTimeAct` | `TIMESTRUCT` | — |
| `lrAzimuth` | `LREAL` | — |
| `lrElevation` | `LREAL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTimeAct` | `Entry of the current time in GMT (Greenwich Mean Time).` | — |
| `lrAzimuth` | `Sun direction in degrees at the time of observation.` | — |
| `lrElevation` | `Sun elevation in degrees at the time of observation.` | — |
| `arrFacadeElement` | `ARRAY[1..iColumnsPerFacade, 1..iRowsPerFacade] OF ST_BARFacadeElement` | — |
| `arrShadingObject` | `ARRAY[1..iShadingObjects] OF ST_BARShadingObject` | — |
| `lrFacadeOrientation` | `LREAL` | — |
| `usiGroupID` | `USINT` | — |
| `arrFacadeElement` | `List of facade elements [} 315].` | — |
| `arrShadingObject` | `List of shading objects [} 316].` | — |
| `lrFacadeOrientation` | `Facade orientation, see illustration above.` | — |
| `usiGroupId` | `Window group regarded. The group 0 is reserved here for unused window elements, see` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bGroupNotShaded` | `BOOL` | — |
| `bFacadeSunlit` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `bGroupNotShaded` | `is TRUE as long as the window group is not calculated as shaded.` | — |
| `bFacadeSunlit` | `this output is set to TRUE if the sun is shining on the facade. See description above.` | — |
| `bError` | `this output is set to TRUE if an error is detected during the execution of the function block.` | — |
| `udiErrorId` | `contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Function block for the shading evaluation of a window group on a facade.

##### `FB_BARShadingCorrectionSouth` — §5.1.5.4.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `stTimeAct` | `TIMESTRUCT` | — |
| `lrAzimuth` | `LREAL` | — |
| `lrElevation` | `LREAL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTimeAct` | `Entry of the current time in GMT (Greenwich Mean Time).` | — |
| `lrAzimuth` | `Sun direction in degrees at the time of observation.` | — |
| `lrElevation` | `Sun elevation in degrees at the time of observation.` | — |
| `arrFacadeElement` | `ARRAY[1..iColumnsPerFacade, 1..iRowsPerFacade] OF ST_BARFacadeElement` | — |
| `arrShadingObject` | `ARRAY[1..iShadingObjects] OF ST_BARShadingObject` | — |
| `lrFacadeOrientation` | `LREAL` | — |
| `usiGroupID` | `USINT` | — |
| `arrFacadeElement` | `List of facade elements [} 315].` | — |
| `arrShadingObject` | `List of shading objects [} 316].` | — |
| `lrFacadeOrientation` | `Facade orientation, see illustration above.` | — |
| `usiGroupId` | `Window group regarded. The group 0 is reserved here for unused window elements, see` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bGroupNotShaded` | `BOOL` | — |
| `bFacadeSunlit` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `bGroupNotShaded` | `is TRUE as long as the window group is not calculated as shaded.` | — |
| `bFacadeSunlit` | `this output is set to TRUE if the sun is shining on the facade. See description above.` | — |
| `bError` | `this output is set to TRUE if an error is detected during the execution of the function block.` | — |
| `udiErrorId` | `contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Function block for the shading evaluation of a window group on a facade.

##### `FB_BARShadingObjectsEntry` — §5.1.5.4.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iId` | `INT` | — |
| `bRead` | `BOOL` | — |
| `bWrite` | `BOOL` | — |
| `lrP1x` | `LREAL` | — |
| `lrP1y` | `LREAL` | — |
| `lrP1z` | `LREAL` | — |
| `lrP2y` | `LREAL` | — |
| `lrP3x` | `LREAL` | — |
| `lrP3y` | `LREAL` | — |
| `lrP3z` | `LREAL` | — |
| `lrMx` | `LREAL` | — |
| `lrMy` | `LREAL` | — |
| `lrMz` | `LREAL` | — |
| `lrRadius` | `LREAL` | — |
| `usiBeginMonth` | `USINT` | — |
| `usiEndMonth` | `USINT` | — |
| `eType` | `E_BARShadingObjectType` | — |
| `iId` | `Index of the selected element. This refers to the selection of a field element of the array saved in the IN-` | — |
| `arrShadingObject` | `ARRAY[1..iShadingObjects] OF ST_BARShadingObject` | — |
| `bRead` | `With a rising edge on this input the information of the selected element, arrShadingObject [iId], is` | — |
| `bWrite` | `A positive edge writes the values on the inputs rP1x to eType as well as the values determined and` | — |
| `lrP1x` | `X-coordinate of point 1 of the shading element (square) in meters.` | — |
| `lrP1y` | `Y-coordinate of point 1 of the shading element (square) in meters.` | — |
| `lrP1z` | `Z-coordinate of point 1 of the shading element (square) in meters.` | — |
| `lrP2y` | `Y-coordinate of point 2 of the shading element (square) in meters.` | — |
| `lrP3x` | `X-coordinate of point 3 of the shading element (square) in meters.` | — |
| `lrP3y` | `Y-coordinate of point 3 of the shading element (square) in meters.` | — |
| `lrP3z` | `Z-coordinate of point 3 of the shading element (square) in meters.` | — |
| `lrMx` | `X-coordinate of the center of the shading element (ball) in meters.` | — |
| `lrMy` | `Y-coordinate of the center of the shading element (ball) in meters.` | — |
| `lrMz` | `Z-coordinate of the center of the shading element (ball) in meters.` | — |
| `lrRadius` | `Radius of the shading element (ball) in meters.` | — |
| `usiBeginMonth` | `Beginning of the shading period (month).` | — |
| `usiEndMonth` | `End of the shading period (month).` | — |
| `eType` | `Selected type of element: Rectangle or ball. See E_BARShadingObjectType [} 500].` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrShadingObject` | `ARRAY[1..iShadingObjects] OF ST_BARShadingObject` | — |
| `arrShadingObject` | `List of shading objects [} 316]. Values are persistently saved.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrP2x` | `LREAL` | — |
| `lrP2z` | `LREAL` | — |
| `lrP4x` | `LREAL` | — |
| `lrP4y` | `LREAL` | — |
| `lrP4z` | `LREAL` | — |
| `bValid` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `lrP2x` | `determined X-coordinate of point 2 of the shading element (square) in metres. See "Note" above.` | — |
| `lrP2z` | `determined Z-coordinate of point 2 of the shading element (square) in metres. See "Note" above.` | — |
| `lrP4x` | `determined X-coordinate of point 4 of the shading element (square) in metres. See "Note" above.` | — |
| `lrP4y` | `determined Y-coordinate of point 4 of the shading element (square) in metres. See "Note" above.` | — |
| `lrP4z` | `determined Z-coordinate of point 4 of the shading element (square) in metres. See "Note" above.` | — |
| `bValid` | `Result of the plausibility check for the values entered. With respect to a square it is required that the` | — |
| `udiErrorId` | `contains the error code, if the entries should not correspond to the criteria mentioned. See Error` | — |

**Description:** This function block serves for the administration of all shading elements in a facade, which is globally saved in a list of shading elements [} 316]. It is intended to facilitate the input of the element information - also with regard to the use of the target visualization. A schematic illustration of the objects with description of the coordinates is given in Shading correction: principles and definitions [} 239]. The shading elements are declared in the global variables:

##### `FB_BARSunblindActuator` — §5.1.5.4.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `stSunblind` | `ST_BARSunblind` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `Enable input for the function block. As long as this input is TRUE, the actuator function block` | — |
| `stSunblind` | `Positioning telegram, see ST_BARSunblind [} 503].` | — |
| `uiBacklashTimeUp` | `UINT` | — |
| `uiBacklashTimeDown` | `UINT` | — |
| `uiTurningTimeUp` | `UINT` | — |
| `uiTurningTimeDown` | `UINT` | — |
| `uiTotalTimeUp` | `UINT` | — |
| `uiTotalTimeDown` | `UINT` | — |
| `uiBlindHeight` | `UINT` | — |
| `iAngleLimitUp` | `INT` | — |
| `iAngleLimitDown` | `INT` | — |
| `uiBacklashTimeUp` | `Time to traverse the backlash in the upward direction in ms.` | — |
| `uiBacklashTimeDown` | `Time to traverse the backlash in the downward direction in ms.` | — |
| `uiTurningTimeUp` | `Time for turning the slats in the upward direction in ms.` | — |
| `uiTurningTimeDown` | `Time for turning the slats in the downward direction in ms.` | — |
| `uiTotalTimeUp` | `Complete time for driving up in ms.` | — |
| `uiTotalTimeDown` | `Complete time for driving down in ms.` | — |
| `uiBlindHeight` | `This input is currently not used and does not need to be described by the user.` | — |
| `iAngleLimitUp` | `Highest position of the slats in degrees.` | — |
| `iAngleLimitDown` | `Lowest position of the slats in degrees.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBlindUp` | `BOOL` | — |
| `bBlindDown` | `BOOL` | — |
| `uiActPosition` | `UINT` | — |
| `uiActAngle` | `UINT` | — |
| `bReferencing` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `bBlindUp` | `control output for blind up.` | — |
| `bBlindDown` | `control output for blind down.` | — |
| `uiActPosition` | `current position in percent.` | — |
| `uiActAngle` | `current louvre angle in degrees.` | — |
| `bReferencing` | `the blind is in the reference-process, which means, that the output bBlindUp will be set for` | — |
| `bBusy` | `a positioning or a referencing procedure is in progress.` | — |
| `bError` | `this output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block is used for positioning of a slatted blind via two outputs: drive up and drive down. The blind can be driven to any desired (height) position and slat angle via the positioning telegram stSunblind [} 503]. On top of that, the positioning telegram stSunblind [} 503] also contains manual commands with which the blind can be moved individually to certain positions. These manual commands are controlled by the function block FB_BARSunblindSwitch [} 291].

##### `FB_BARSunblindActuatorEx` — §5.1.5.4.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `stSunblind` | `ST_BARSunblind` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `Enable input for the function block. As long as this input is TRUE, the actuator function block` | — |
| `stSunblind` | `Positioning telegram, see ST_BARSunblind [} 503].` | — |
| `uiBacklashTimeUp` | `UINT` | — |
| `uiBacklashTimeDown` | `UINT` | — |
| `uiTurningTimeUp` | `UINT` | — |
| `uiTurningTimeDown` | `UINT` | — |
| `udiTotalTimeUp` | `UDINT` | — |
| `udiTotalTimeDown` | `UDINT` | — |
| `uiBlindHeight` | `UINT` | — |
| `iAngleLimitUp` | `INT` | — |
| `iAngleLimitDown` | `INT` | — |
| `uiBacklashTimeUp` | `Time to traverse the backlash in the upward direction in ms.` | — |
| `uiBacklashTimeDown` | `Time to traverse the backlash in the downward direction in ms.` | — |
| `uiTurningTimeUp` | `Time for turning the slats in the upward direction in ms.` | — |
| `uiTurningTimeDown` | `Time for turning the slats in the downward direction in ms.` | — |
| `udiTotalTimeUp` | `Complete time for driving up in ms.` | — |
| `udiTotalTimeDown` | `Complete time for driving down in ms.` | — |
| `uiBlindHeight` | `This input is currently not used and does not need to be described by the user.` | — |
| `iAngleLimitUp` | `Highest position of the slats in degrees.` | — |
| `iAngleLimitDown` | `Lowest position of the slats in degrees.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBlindUp` | `BOOL` | — |
| `bBlindDown` | `BOOL` | — |
| `uiActPosition` | `UINT` | — |
| `uiActAngle` | `UINT` | — |
| `bReferencing` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `bBlindUp` | `control output for blind up.` | — |
| `bBlindDown` | `control output for blind down.` | — |
| `uiActPosition` | `current position in percent.` | — |
| `uiActAngle` | `current louvre angle in degrees.` | — |
| `bReferencing` | `the blind is in the reference-process, which means, that the output bBlindUp will be set for` | — |
| `bBusy` | `a positioning or a referencing procedure is in progress.` | — |
| `bError` | `this output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block is used for positioning of a slatted blind via two outputs: drive up and drive down. The blind can be driven to any desired (height) position and slat angle via the positioning telegram stSunblind [} 503]. On top of that, the positioning telegram stSunblind [} 503] also contains manual commands with which the blind can be moved individually to certain positions. These manual commands are controlled by the function block FB_BARSunblindSwitch [} 291].

##### `FB_BARSunblindEvent` — §5.1.5.4.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bActivate` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bActivate` | `A TRUE signal on this input activates the function block and transfers the entered setpoint values` | — |
| `uiPositionPark` | `UINT` | — |
| `iAnglePark` | `INT` | — |
| `uiPositionEvent` | `Height position of the blind in % in case of activation.` | — |
| `iAngleEvent` | `Slat angle of the blind in degrees in case of activation.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunblind` | `ST_BARSunblind` | — |
| `bActive` | `BOOL` | — |
| `bActive` | `Corresponds to the Boolean value bActive in the blind telegram ST_BARSunblind [} 503] and serves` | — |
| `stSunblind` | `Output structure of the blind positions, see ST_BARSunblind [} 503]` | — |

**Description:** This function block serves to preset the position and angle for any desired event. It can be used, for example, in order to drive to a parking position or to drive the blind upward for maintenance.

##### `FB_BARSunblindPrioritySwitch` — §5.1.5.4.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunblind_Prio1` | `ST_BARSunblind` | — |
| `stSunblind_Prio2` | `ST_BARSunblind` | — |
| `stSunblind_Prio3` | `ST_BARSunblind` | — |
| `stSunblind_Prio4` | `ST_BARSunblind` | — |
| `stSunblind_Prio5` | `ST_BARSunblind` | — |
| `stSunblind_Prio6` | `ST_BARSunblind` | — |
| `stSunblind_Prio7` | `ST_BARSunblind` | — |
| `stSunblind_Prio8` | `ST_BARSunblind` | — |
| `stSunblind_Prio9` | `ST_BARSunblind` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunblind` | `ST_BARSunblind` | — |
| `stSunblind` | `resulting positioning telegram.` | — |

**Description:** Priority controller for up to 9 positioning telegrams (stSunblind_Prio1 ... stSunblind_Prio9) of the type ST_BARSunblind [} 503].

##### `FB_BARRollerBlind` — §5.1.5.4.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `stSunblind` | `ST_BARSunblind` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `Enable input for the function block. As long as this input is TRUE, the actuator function block` | — |
| `stSunblind` | `Positioning telegram, see ST_BARSunblind [} 503].` | — |
| `udiTotalTimeUp` | `UDINT` | — |
| `udiTotalTimeDown` | `UDINT` | — |
| `udiTotalTimeUp` | `Complete time for driving up in ms.` | — |
| `udiTotalTimeDown` | `Complete time for driving down in ms.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBlindUp` | `BOOL` | — |
| `bBlindDown` | `BOOL` | — |
| `uiActPosition` | `UINT` | — |
| `bReferencing` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `bBlindUp` | `control output for blind up.` | — |
| `bBlindDown` | `control output for blind down.` | — |
| `uiActPosition` | `current position in percent.` | — |
| `bReferencing` | `the blind is in the reference-process, which means, that the output bBlindUp will be set for` | — |
| `bBusy` | `a positioning or a referencing procedure is in progress.` | — |
| `bError` | `this output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block is used for positioning a roller blind via two outputs: blind up and blind down. The blind can be driven to any desired position with the position telegram stSunblind [} 503]. In addition, the positioning telegram stSunblind [} 503] offers manual commands, which can be used to move the blind to particular positions. These manual commands are controlled by the function block FB_BARSunblindSwitch [} 291].

##### `FB_BARSunblindScene` — §5.1.5.4.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `usiSelectedScene` | `USINT` | — |
| `bSwitchUp` | `BOOL` | — |
| `bSwitchDown` | `BOOL` | — |
| `bInvokeScene` | `BOOL` | — |
| `bSaveScene` | `BOOL` | — |
| `uiSetpointPosition` | `UINT` | — |
| `iSetpointAngle` | `INT` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `The function block has no function if this input is FALSE. 0 is output for both position and angle in` | — |
| `usiSelectedScene` | `Selected scene, which is to be either stored (bSaveScene) or called (bInvokeScene).` | — |
| `bSwitchUp` | `Command input for blind up.` | — |
| `bSwitchDown` | `Command input for blind down.` | — |
| `bInvokeScene` | `Call selected scene.` | — |
| `bSaveScene` | `Save selected scene.` | — |
| `uiSetpointPosition` | `Setpoint position in % that is to be saved in the selected scene. Must be linked to the` | — |
| `iSetpointAngle` | `dto. Slat angle in degrees.` | — |
| `uiSwitchOverTime` | `UINT` | — |
| `arrBlindScenes` | `ARRAY[0..20] OF ST_BARSunblindScene` | — |
| `uiSwitchOverTime` | `Time in milliseconds until the corresponding manual command in the positioning` | — |
| `arrBlindScenes` | `Table with the scene entries of the type ST_BARSunblindScene [} 503]. Up to 21 scenes` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunblind` | `ST_BARSunblind` | — |
| `bActive` | `BOOL` | — |
| `uiPositionActScene` | `UINT` | — |
| `iAnglePositionActScene` | `INT` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `stSunblind` | `Positioning telegram, see ST_BARSunblind [} 503].` | — |
| `bActive` | `Corresponds to the Boolean value bActive in the blind telegram ST_BARSunblind [} 503] and` | — |
| `uiPositionActScene` | `Indicates the saved relative blind height position in % for the currently selected` | — |
| `iAnglePositionActScene` | `dto. Slat angle in degrees.` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block represents an extension of the manual operation FB_BARSunblindSwitch [} 291] by a scene memory and a call function. The blind actuator FB_BARSunblindActuator [} 272] or the roller shutter actuator FB_BARRollerblind [} 286] can thus be controlled in manual operation mode and can also drive directly to previously saved positions (scenes). Up to 21 scenes can be saved.

##### `FB_BARSunblindSwitch` — §5.1.5.4.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `bSwitchUp` | `BOOL` | — |
| `bSwitchDown` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `The function block has no function if this input is FALSE. 0 is output for both position and angle in` | — |
| `bSwitchUp` | `Command input for blind up.` | — |
| `bSwitchDown` | `Command input for blind down.` | — |
| `uiSwitchOverTime` | `UINT` | — |
| `uiSwitchOverTime` | `Time in milliseconds until the corresponding manual command in the positioning` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunblind` | `ST_BARSunblind` | — |
| `bActive` | `BOOL` | — |
| `stSunblind` | `positioning telegram, see ST_BARSunblind [} 503].` | — |
| `bActive` | `corresponds to the Boolean value bActive in the blind telegram ST_BARSunblind [} 503] and serves` | — |

**Description:** With the help of this function block the blind controller FB_BARSunblindActuator [} 272] or the roller shutter controller FB_BARRollerblind [} 286] can be controlled in manual operation mode. The connection takes place via the positioning telegram stSunblind [} 503] either directly or with an additional priority controller.

##### `FB_BARSunblindThermoAutomatic` — §5.1.5.4.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `uiBrightness` | `UINT` | — |
| `bPresence` | `BOOL` | — |
| `lrActRoomTemperature` | `LREAL` | — |
| `stBARSetpointRoom` | `ST_BARSetpointRoom` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `The function block has no function if this input is FALSE. 0 is output for both position and angle in` | — |
| `uiBrightness` | `External brightness in lux.` | — |
| `bPresence` | `The automatic function is active only it the room is unoccupied, since otherwise the sun shading` | — |
| `lrActRoomTemperature` | `Input for the room temperature in °C.` | — |
| `stBARSetpointRoom` | `Structure of the type ST_BARSetpointRoom [} 293], which contains the information` | — |
| `uiBrightnessActivateValue` | `UINT;(*lux*)` | — |
| `uiBrightnessDeactivateValue` | `UINT;(*lux*)` | — |
| `uiActivateDelay` | `UINT;(*sec*)` | — |
| `uiDeactivateDelay` | `UINT;(*sec*)` | — |
| `bComfortHeatingAsSwitchPointCooling` | `BOOL` | — |
| `bComfortCoolingAsSwitchPointHeating` | `BOOL` | — |
| `uiPositionHeatingMode` | `UINT` | — |
| `iAngleHeatingMode` | `INT` | — |
| `uiPositionCoolingMode` | `UINT` | — |
| `iAngleCoolingMode` | `INT` | — |
| `bComfortHeatingAsSwitchPointCooling` | `If this input is TRUE, then the value` | — |
| `bComfortCoolingAsSwitchPointHeating` | `If this input is TRUE, then the value` | — |
| `uiPositionHeatingMode` | `Height position of the blind in % in the heating case (intended heat irradiation).` | — |
| `iAngleHeatingMode` | `Slat angle of the blind in degrees in the heating case (intended heat irradiation).` | — |
| `uiPositionCoolingMode` | `Height position of the blind in % in the cooling case (sun protection). The` | — |
| `iAngleCoolingMode` | `Slat angle of the blind in degrees in the cooling case (sun protection).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunblind` | `ST_BARSunblind` | — |
| `bActive` | `BOOL` | — |
| `uiRemainingTimeActivate` | `UINT` | — |
| `uiRemainingTimeDeactivate` | `UINT` | — |
| `bHeating` | `BOOL` | — |
| `bCooling` | `BOOL` | — |
| `lrUpperSwitchPointHeating` | `LREAL` | — |
| `lrLowerSwitchPointHeating` | `LREAL` | — |
| `lrUpperSwitchPointCooling` | `LREAL` | — |
| `lrLowerSwitchPointCooling` | `LREAL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `stSunblind` | `Output structure of the blind positions, see ST_BARSunblind [} 503].` | — |
| `bActive` | `Corresponds to the Boolean value bActive in the blind telegram ST_BARSunblind [} 503] and` | — |
| `uiRemainingTimeActivate` | `The thermo-automatic function is active if no presence is detected AND the` | — |
| `uiRemainingTimeDeactivate` | `The thermo-automatic function is not active if presence is detected OR if the` | — |
| `bHeating` | `Indicates whether the heating case is active.` | — |
| `bCooling` | `Indicates whether the cooling case is active.` | — |
| `lrUpperSwitchPointHeating` | `Indicates the selected upper switch point in heating mode at which the blind is` | — |
| `lrLowerSwitchPointHeating` | `Indicates the lower switch point in heating mode at which the blind is driven to` | — |
| `lrUpperSwitchPointCooling` | `Indicates the selected upper switch point in cooling mode at which the blind is` | — |
| `lrLowerSwitchPointCooling` | `Indicates the selected lower switch point in cooling mode at which the blind is` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block controls the blind in relation to the temperature inside the room if the room is unoccupied. The objective is to block radiant heat when cooling and to increase the input of heat by opening the blinds when heating.

##### `FB_BARSunblindTwilightAutomatic` — §5.1.5.4.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `uiBrightness` | `UINT` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `The function block has no function if this input is FALSE. 0 is output for both position and angle in` | — |
| `uiBrightness` | `External brightness in lux.` | — |
| `uiActivateValue` | `UINT` | — |
| `uiDeactivateValue` | `UINT` | — |
| `uiActivateDelay` | `UINT` | — |
| `uiDeactivateDelay` | `UINT` | — |
| `uiPositionActiveMode` | `UINT` | — |
| `iAngleActiveMode` | `INT` | — |
| `uiActivateValue` | `Activation limit value in lux.` | — |
| `uiDeactivateValue` | `Deactivation limit value in lux.` | — |
| `uiActivateDelay` | `Activation delay in seconds.` | — |
| `uiDeactivateDelay` | `Deactivation delay in seconds.` | — |
| `uiPositionActiveMode` | `Height position of the blind in % if the automatic twilight function is active.` | — |
| `iAngleActiveMode` | `Slat angle of the blind in degrees if the automatic twilight function is active.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunblind` | `ST_BARSunblind` | — |
| `bActive` | `BOOL` | — |
| `uiRemainingTimeActivate` | `UINT` | — |
| `uiRemainingTimeDeactivate` | `UINT` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `stSunblind` | `Output structure of the blind positions, see ST_BARSunblind [} 503].` | — |
| `bActive` | `Corresponds to the Boolean value bActive in the blind telegram ST_BARSunblind [} 503] and` | — |
| `uiRemainingTimeActivate` | `Indicates the time remaining in seconds until the activation of the automatic` | — |
| `uiRemainingTimeDeactivate` | `Indicates the time remaining in seconds until the deactivation of the` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block controls the blind if the outdoor brightness has fallen below a limit value.

##### `FB_BARSunblindWeatherProtection` — §5.1.5.4.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `lrWindSpeed` | `LREAL` | — |
| `lrOutsideTemperature` | `LREAL` | — |
| `bRainSensor` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `The function block has no function if this input is FALSE. 0 is output for both position and angle in` | — |
| `lrWindSpeed` | `Wind speed. The unit of entry is arbitrary, but it is important that no value is smaller than 0` | — |
| `lrOutsideTemperature` | `Outside temperature in degrees Celsius.` | — |
| `bRainSensor` | `Input for a rain sensor.` | — |
| `lrFrostTemperature` | `LREAL` | — |
| `lrWindSpeedStormUpcoming` | `LREAL` | — |
| `lrWindSpeedStormAbating` | `LREAL` | — |
| `uiDeiceTime` | `UINT` | — |
| `uiDelayStormUpcoming` | `UINT` | — |
| `uiDelayStormAbating` | `UINT` | — |
| `uiPositionProtection` | `UINT` | — |
| `iAngleProtection` | `INT` | — |
| `lrFrostTemperature` | `Icing up temperature limit value in degrees Celsius. This value may not be greater` | — |
| `lrWindSpeedStormUpcoming` | `Wind speed limit value for the activation of the storm alarm. This value may` | — |
| `lrWindSpeedStormAbating` | `Wind speed limit value for the deactivation of the storm alarm. This value may` | — |
| `uiDeiceTime` | `Time until the deicing of the blind after icing up (in minutes). After that the icing up alarm is` | — |
| `uiDelayStormUpcoming` | `Time delay in seconds until the storm alarm is triggered.` | — |
| `uiDelayStormUpcoming` | `Time delay in seconds until the storm alarm is reset.` | — |
| `uiPositionProtection` | `Height position of the blind in % in the case of protection.` | — |
| `iAngleProtection` | `Slat angle of the blind in degrees in the case of protection.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunblind` | `ST_BARSunblind` | — |
| `bIceAlarm` | `BOOL` | — |
| `udiRemainingTimeIceAlarm` | `UDINT` | — |
| `bStormAlarm` | `BOOL` | — |
| `uiRemainingTimeStormDetect` | `UINT` | — |
| `uiRemainingTimeStormAlarm` | `UINT` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `stSunblind` | `Output structure of the blind positions, see ST_BARSunblind [} 503].` | — |
| `bActive` | `Corresponds to the Boolean value bActive in the blind telegram ST_BARSunblind [} 503] and` | — |
| `bIceAlarm` | `Indicates the icing up alarm.` | — |
| `udiRemainingTimeIceAlarm` | `in the case of impending icing up (bIceAlarm=TRUE), this second counter is` | — |
| `bStormAlarm` | `Indicates the storm alarm.` | — |
| `uiRemainingTimeStormDetect` | `In an uncritical case this second counter constantly indicates the alarm` | — |
| `uiRemainingTimeStormAlarm` | `As soon as the storm alarm is initiated, this second counter initially` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** The weather protection has the highest priority in the blind controller (see overview [} 230]) and is intended to ensure that the blind is not damaged by ice or wind.

##### `FB_BARSunProtectionEx` — §5.1.5.4.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `stTimeAct` | `TIMESTRUCT` | — |
| `stBlindPositionTable` | `ST_BARBlindPositionTable` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `If this input is set to FALSE the positioning is inactive, i.e. the active bit (bActive) is reset in the` | — |
| `stTimeAct` | `Entry of the current time in GMT (Greenwich Mean Time).` | — |
| `stBlindPositionTable` | `Table of 6 interpolation points, 4 of which are parameterizable, from which a blind` | — |
| `uiPositioningInterval` | `UINT` | — |
| `lrDegreeOfLongitude` | `LREAL` | — |
| `lrDegreeOfLatitude` | `LREAL` | — |
| `lrFacadeOrientation` | `LREAL` | — |
| `lrFacadeAngle` | `LREAL` | — |
| `uiLouvreWidth` | `UINT` | — |
| `uiLouvreSpacing` | `UINT` | — |
| `uiFixedShutterHeight` | `UINT` | — |
| `uiMaxLightIncidence` | `UINT` | — |
| `uiWindowHeight` | `UINT` | — |
| `uiWindowDistanceFloor` | `UINT` | — |
| `ePositioningMode` | `E_BARPOSMODE` | — |
| `uiPositioningInterval` | `Positioning interval in seconds – time interval between two outputs of blind positions.` | — |
| `lrDegreeOfLongitude` | `Longitude in degrees. Valid range: -180°...180°.` | — |
| `lrDegreeOfLatitude` | `Latitude in degrees. Valid range: -90°..90°.` | — |
| `lrFacadeOrientation` | `Facade orientation in degrees:` | — |
| `lrFacadeAngle` | `Facade inclination in degrees. See facade inclination.` | — |
| `uiLouvreWidth` | `Width of the slats in mm, see sketch.` | — |
| `uiLouvreSpacing` | `Slat spacing in mm, see sketch.` | — |
| `uiFixedShutterHeight` | `Fixed (constant) shutter height [0..100%]. Valid if ePositioningMode =` | — |
| `uiMaxLightIncidence` | `Maximum desired incidence of light in mm measured from the outside of the wall` | — |
| `uiWindowHeight` | `Window height in mm for the calculation of the shutter height if the mode "maximum` | — |
| `uiWindowDistanceFloor` | `Distance between the floor and the window sill in mm for the calculation of the` | — |
| `ePositioningMode` | `Selection of the positioning mode, see enumerator E_BARPosMode [} 500].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSunblind` | `ST_BARSunblind` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `stSunblind` | `Output structure of the blind positions, see ST_BARSunblind [} 503]` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Function block for the control of glare protection with the aid of a slatted blind.

##### `FB_BARWithinRangeAzimuth` — §5.1.5.4.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `lrAzimuth` | `LREAL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrAzimuth` | `Current azimuth angle.` | — |
| `lrBeginRange` | `LREAL` | — |
| `lrEndRange` | `LREAL` | — |
| `lrBeginRange` | `Beginning of the range in degrees.` | — |
| `lrEndRange` | `End of the range in degrees.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `bOut` | `binary delayed output of the threshold switch` | — |
| `bError` | `this output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block checks whether the current azimuth angle (horizontal position of the sun) lies within the limits entered. As can be seen in the overview [} 232], the function block provides an additionally evaluation as to whether the sun shading of a window group should be activated. Therefore the observations in the remainder of the text always apply to one window group.

##### `FB_BARWithinRangeElevation` — §5.1.5.4.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `lrElevation` | `LREAL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `lrElevation` | `Current angle of elevation in degrees.` | — |
| `lrLowLimit` | `LREAL` | — |
| `lrHighLimit` | `LREAL` | — |
| `lrLowLimit` | `Lower limit value in degrees.` | — |
| `lrHighLimit` | `Upper limit value in degrees.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `bOut` | `binary delayed output of the threshold switch` | — |
| `bError` | `this output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** This function block checks whether the current angle of elevation (vertical position of the sun) lies within the limits entered. As can be seen in the overview [} 232], the function block provides an additionally evaluation as to whether the sun shading of a window group should be activated. Therefore the observations in the remainder of the text always apply to one window group.

#### Programming › POUs › HVAC Setpoint modules

##### `FB_HVACHeatingCurve` — §5.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `rOutsideTempDamped` | `REAL` | — |
| `bDayOperation` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `rOutsideTempDamped` | `This input variable transfers the current damped outside temperature to the` | — |
| `bDayOperation` | `TRUE = day operation, FALSE = night operation.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rMinTemp` | `REAL` | — |
| `rMaxTemp` | `REAL` | — |
| `rNightSetback` | `REAL` | — |
| `rTempSetpoint_Y1` | `REAL` | — |
| `rTempSetpoint_Y2` | `REAL` | — |
| `rTempSetpoint_Y3` | `REAL` | — |
| `rTempSetpoint_Y4` | `REAL` | — |
| `rOutsideTemp_X1` | `REAL` | — |
| `rOutsideTemp_X2` | `REAL` | — |
| `rOutsideTemp_X3` | `REAL` | — |
| `rOutsideTemp_X4` | `REAL` | — |
| `rMinTemp` | `The minimum value for the setpoint of the flow temperature is defined by this variable. The` | — |
| `rMaxTemp` | `The maximum value for the setpoint of the flow temperature is defined by this variable. The` | — |
| `rNightSetback` | `The value for the night setback is specified by this variable. The variable is saved` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rTempSetpoint` | `REAL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rTempSetpoint` | `The calculated setpoint for the flow temperature (0°C..500°C).` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |

**Description:** The four-point heating curve serves to determine the setpoint for the flow temperature of a heating circuit, depending on the outside temperature.

##### `FB_HVACHeatingCurveEx` — §5.1.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `rOutsideTempDamped` | `REAL` | — |
| `bDayOperation` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `rOutsideTempDamped` | `This input variable transfers the current damped outside temperature to the` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rMinTempSetpoint` | `REAL` | — |
| `rMaxTempSetpoint` | `REAL` | — |
| `rDesignTemp` | `REAL` | — |
| `rSlope` | `REAL` | — |
| `rCurve` | `REAL` | — |
| `rMinTempSetpoint` | `The minimum value for the setpoint of the flow temperature [°C] is defined by this` | — |
| `rMaxTempSetpoint` | `The maximum value for the setpoint of the flow temperature [°C] is defined by this` | — |
| `rDesignTemp` | `Design temperature [°C] for the dimensioning of a heating system. Typical values for` | — |
| `rSlope` | `Factor for the slope. The variable is saved persistently. Preset to 1.` | — |
| `rCurve` | `Factor for the curve. The variable is saved persistently. Preset to 0.5.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rTempSetpoint` | `REAL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rTempSetpoint` | `The calculated setpoint [°C] for the flow temperature.` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |

**Description:** Programming

##### `FB_HVACOutsideTempDamped` — §5.1.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |
| `rOutsideTemp` | `REAL` | — |
| `dtSystemTime` | `DT` | — |
| `bResetInternalValues` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rManualValue` | `REAL` | — |
| `bEnable` | `The function block is enabled by the PLC program with the input variable bEnable. If bEnable =` | — |
| `rOutsideTemp` | `This input variable transfers the current outside temperature to the function block.` | — |
| `dtSystemTime` | `This input variable transfers the date and time to the function block.` | — |
| `bResetInternalValues` | `Using this input variable, the internally stored outside temperatures are reset and` | — |
| `eCtrlMode` | `The operation mode is selected via this Enum. Manual or automatic mode (see` | — |
| `rManualValue` | `Manual value that is set at the output rOutsideTempDamped if eCtrlMode =` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rOutsideTempDamped` | `REAL` | — |
| `rOutsideTempDamped` | `Damped outside temperature.` | — |

**Description:** This function block serves to determine the average or damped outside temperature. In automatic mode the mean outside temperature is calculated from the outside temperature values at 7 am, 2 pm and 9 pm. The measurement taken at 9 pm receives double the weighting.

##### `FB_HVACSetpointHeating` — §5.1.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rSetpointIn` | `REAL` | — |
| `rOutsideTempDamped` | `REAL` | — |
| `dtSystemTime` | `DT` | — |
| `bDayOperation` | `BOOL` | — |
| `bFreezeProtec` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is activated if the variable bEnable is TRUE. If it is FALSE, zero will be output at` | — |
| `rSetpointIn` | `In the case of a heating circuit controlled by the room temperature, the value of a room setpoint` | — |
| `rOutsideTempDamped` | `Short-term fluctuations in the outside temperature must not be allowed to have an` | — |
| `dtSystemtime` | `This variable transfers the computer system time to the function block.` | — |
| `bDayOperation` | `The output variable of the program is transferred at this input variable. If bDayOperation is` | — |
| `bFreezeProtec` | `Input to which the frost protection signal is applied.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `eSetpointHeatingMode` | `E_HVACSetpointHeatingMode` | — |
| `rHeatingLimit` | `REAL` | — |
| `uiOn_Day` | `UINT` | — |
| `uiOn_Month` | `UINT` | — |
| `uiOff_Day` | `UINT` | — |
| `uiOff_Month` | `UINT` | — |
| `rFreezeProtecSetpoint` | `REAL` | — |
| `rNightSetback` | `REAL` | — |
| `tDelayHeatingOn` | `TIME` | — |
| `tDelayHeatingOff` | `TIME` | — |
| `eSetpointHeatingMode` | `Enum that specifies the operation mode of the heating circuit (see` | — |
| `rHeatingLimit` | `Heating limit (-60°C..100°C). Necessary for the operation modes of the heating circuit` | — |
| `uiOn_Day` | `Switch-on day of the month. Necessary for the operation mode ‘heating period according to` | — |
| `uiOn_Month` | `Switch-on month of the year. Necessary for the operation mode ‘heating period according to` | — |
| `uiOff_Day` | `Switch-off day of the month. Necessary for the operation mode ‘heating period according to` | — |
| `uiOff_Month` | `Switch-off month of the year. Necessary for the operation mode ‘heating period according to` | — |
| `rFreezeProtecSetpoint` | `Setpoint for the heating circuit in frost protection mode (0°C..100°C). The variable` | — |
| `rNightSetback` | `rNightSetback specifies the value for the night setback in °C (0°C..100°C). rNightSetback is` | — |
| `tDelayHeatingOn` | `Time delay for the setting of the output bHeating to TRUE after the damped outside` | — |
| `tDelayHeatingOff` | `Time delay for the resetting of the output bHeating to FALSE after the damped outside` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rSetpointOut` | `REAL` | — |
| `bHeating` | `BOOL` | — |
| `bReqOutsideTemp` | `BOOL` | — |
| `bReqDate` | `BOOL` | — |
| `bReqPermanent` | `BOOL` | — |
| `bReqFreezeProtec` | `BOOL` | — |
| `bReqNight` | `BOOL` | — |
| `bReqDay` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rSetpointOut` | `Setpoint for the supply air temperature control.` | — |
| `bHeating` | `The output bHeating is set (TRUE) or reset (FALSE) immediately or after a delay, depending on` | — |
| `bReqOutsideTemp` | `The heating circuit is in the operating mode ‘heating period according to outside` | — |
| `bReqDate` | `The heating circuit is in the operating mode ‘heating period according to date’.` | — |
| `bReqPermanent` | `The heating circuit is in the operating mode ‘heating circuit always on’.` | — |
| `bReqFreezeProtec` | `The heating circuit is in the operating mode ‘frost protection mode’.` | — |
| `bReqNight` | `The heating circuit is in the operating mode ‘night operation’.` | — |
| `bReqDay` | `The heating circuit is in the operating mode ‘day operation’.` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |

**Description:** By using this function block a heating circuit can be switched through different operation modes. Depending on the operation mode a corresponding setpoint for controlling the supply air temperature of a static heating circuit is output from the function block. The output bHeating is set or reset depending on the operation mode and the state of the parameters described below.

##### `FB_HVACSetpointRamp` — §5.1.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rSetpointIn` | `REAL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `Enables the function block.` | — |
| `rSetpointIn` | `Setpoint to which the output rSetpointOut is regulated, depending on the absolute temperature` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rMinSetpoint` | `REAL` | — |
| `rMaxSetpoint` | `REAL` | — |
| `rSetpointSlide` | `REAL` | — |
| `tRampTime` | `TIME` | — |
| `rSetpointSlide` | `Absolute temperature change in [Kelvin/tRampTime] with which the output is converted in` | — |
| `tRampTime` | `Ramp time (1s..24h) during which the final setpoint is reached in sliding steps, depending on` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rSetpointOut` | `REAL` | — |
| `tRemainingTime` | `TIME` | — |
| `bValueReached` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rSetpointOut` | `Output of the ramp generator` | — |
| `tRemainingTime` | `Time duration in which the output rSetpointOut has reached the target value rSetpointIn.` | — |
| `bValueReached` | `A TRUE at this output indicates that the output rSetpointOut has reached the target value` | — |
| `bInvalidParameter` | `Indicates that an incorrect parameter is present at one of the variables rSetpointSlide or` | — |

**Description:** The function block generates a sliding setpoint ramp. This function block must be called in every PLC cycle, because the calculation for reaching the final setpoint depends on the TaskTime.

##### `FB_HVACSummerCompensation` — §5.1.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `rSetpointIn` | `REAL` | — |
| `rSetpointExternal` | `REAL` | — |
| `rOutsideTemp` | `REAL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `rSetpointIn` | `The setpoint for the room is applied to this input.` | — |
| `rSetpointExternal` | `The setpoint adjustment or correction, e.g. from a potentiometer, is applied to this input.` | — |
| `rOutsideTemp` | `Input for the outside temperature.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rSetpointHighLimit` | `REAL` | — |
| `rSetpointShiftAbsolute` | `REAL` | — |
| `eExternalMode` | `E_HVACExternalMode` | — |
| `eSetpointMode` | `E_HVACSetpointMode` | — |
| `rSetpointHighLimit` | `Upper limit for the room temperature setpoint (0°C..100°C). The variable is saved` | — |
| `rSetpointShiftAbsolute` | `Parameter value that defines the permissible difference between the outside` | — |
| `eExternalMode` | `The external setpoint is activated/deactivated via the enum E_HVACExternalMode (see` | — |
| `eSetpointMode` | `Enum that specifies the type of setpoint determination (see E_HVACSetpointMode [} 493]).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rSetpointOut` | `REAL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rSetpointOut` | `Setpoint for the room.` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |

**Description:** This function block is a setpoint generator for the exhaust air temperature or room temperature of an air conditioning system. With the aid of this summer compensation, overly large temperature differences between the outside temperature and the exhaust air/room temperature are avoided. This helps to save energy whilst at the same time increasing comfort.

#### Programming › POUs › HVAC Special functions

##### `FB_HVACAirConditioning2Speed` — §5.1.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bTimeScheduler` | `BOOL` | — |
| `rSetpoint` | `REAL` | — |
| `rActualValue` | `REAL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled by the PLC program with the input variable bEnable= TRUE. The` | — |
| `bTimeScheduler` | `Request by a timer program for the second fan stage. If bEnable and bTimeScheduler are` | — |
| `rSetpoint` | `Setpoint of room or exhaust temperature` | — |
| `rActualValue` | `actual value of room air or exhaust air temperature` | — |
| `bReset` | `Input for acknowledging an incorrect parameter entry or an error via a rising edge.` | — |
| `rHysteresisOn` | `REAL` | — |
| `rHysteresisOff` | `REAL` | — |
| `tDelayHyteresisOn` | `TIME` | — |
| `tDelayHyteresisOff` | `TIME` | — |
| `eAirConditioning2SpeedMode` | `E_HVACAirConditioning2SpeedMode` | — |
| `rHysteresisOn` | `Parameter to switch on the second fan stage via a hysteresis loop, load-dependent, via the` | — |
| `rHysteresisOff` | `Parameter to switch off the second fan stage via a hysteresis loop, load-dependent, via the` | — |
| `tDelayHysteresisOn` | `Switch-on delay of the output variable bSpeed2 in the event of a load-dependent` | — |
| `tDelayHysteresisOff` | `Switch-off delay of the output variable bSpeed2 in the event of a load-dependent` | — |
| `eAirConditioning2SpeedMode` | `Enum that specifies the load-dependent operation mode for switching on` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSpeed2` | `BOOL` | — |
| `bRequestScheduler` | `BOOL` | — |
| `bRequestTemperature` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bSpeed2` | `Output for controlling the second fan speed.` | — |
| `bRequestScheduler` | `Request for the second fan speed via the input variable bTimeScheduler.` | — |
| `bRequestTemperature` | `Request for the second stage, depending on the load, via a difference between the` | — |
| `bInvalidParameter` | `Indicates that an incorrect parameter is present at one of the variables rHysteresisOn ,` | — |

**Description:** The function block FB_HVACAirConditioning2Speed controls switching to the second stage of air conditioning systems with two-speed fans. Switching to the second stage via the output variable bSpeed2 can be performed either by a scheduler via the input variable bTimeScheduler or load-dependent via a difference between the setpoint of the room or exhaust temperature, rSetpoint, and the actual value of the room or exhaust temperature, rActualValue, depending on the time delays tDelayHysteresisOn/ tDelayHysteresisOff, the switch-on and release hystereses rHysteresisOn/rHysteresisOff and the setting of the operation mode via the Enum eAirConditioning2SpeedMode.

##### `FB_HVACAlarm` — §5.1.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bAlarmIn` | `BOOL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bAlarmIn` | `A TRUE on the input bAlarmIn indicates that an alarm signal is present.` | — |
| `bCtrlVoltage` | `Via the input bCtrlVoltage a check is made of whether control voltage is present.` | — |
| `bReset` | `Acknowledge input.` | — |
| `tAlarmDelay` | `TIME` | — |
| `tAlarmDelay` | `Delay after the alarm is first signaled if, during the preset time, bAlarmIn was constantly` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bAlarm` | `BOOL` | — |
| `bAlarm` | `TRUE if an alarm is announced.` | — |

**Description:** A TRUE on the input bAlarmIn indicates that an alarm signal is present. The alarm is routed through to the output bAlarm only if bAlarmIn is present for longer than the preset time tAlarmDelay. Furthermore, the input bCtrlVoltage must be TRUE in order for an alarm to be announced. Output bAlarm remains set to TRUE until bAlarmIn = FALSE and the alarm is acknowledged by a positive edge on bReset.

##### `FB_HVACAntiBlockingDamper` — §5.1.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bDisable` | `BOOL` | — |
| `ryDamper` | `REAL` | — |
| `dtSystemTime` | `DT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The AntiBlocking function is enabled by the input variable bEnable. If bEnable = FALSE, only the` | — |
| `bDisable` | `Resets the outputs rQDamper, iQDamper and bQAntiBlocking to 0.` | — |
| `ryDamper` | `Control value from the controller to the valve, which is fed to the outputs rQDamper and` | — |
| `dtSystemTime` | `System time` | — |
| `bReset` | `Acknowledge input in the event of a fault. Resets the flag bInvalidParameter.` | — |
| `eAntiBlockingMode` | `E_HVACAntiBlockingMode` | — |
| `iHoursDowntime` | `INT` | — |
| `iDayWeekly` | `INT` | — |
| `iHour` | `INT` | — |
| `iMinute` | `INT` | — |
| `iLengthImpulse` | `INT` | — |
| `eHVACAntiBlockingMode` | `Enumeration value that specifies the type of Antiblocking method (see` | — |
| `eHVACAntiBlockingMode_Off` | `= 0: Off` | — |
| `eHVACAntiBlockingMode_Downtime` | `= 1: After the expiry of a standstill time in hours, the Antiblocking-pulse` | — |
| `eHVACAntiBlockingMode_Weekly` | `= 2: The antiblocking pulse is only generated on a certain day of the week` | — |
| `iHoursDowntime` | `For eHVACAntiBlockingMode_Downtime, the time in hours for which the input bIn may` | — |
| `iDayWeekly` | `For eHVACAntiBlockingMode_Weekly, AntiBlocking pulse is formed on this day. Sun=0,` | — |
| `iHour` | `Switch-on time in hours (0..23 h). The variable is saved persistently. Preset to 12.` | — |
| `iMinute` | `Switch-on time in minutes (0min..59min). The variable is saved persistently. Preset to 0.` | — |
| `iLengthImpulse` | `Switch-on time in seconds (0s..600s). The variable is saved persistently. Preset to 150.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rQDamper` | `REAL` | — |
| `iQDamper` | `INT` | — |
| `bQAntiBlocking` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rQDamper` | `This output corresponds to the input value of the control value (0%..100%). As soon as the input` | — |
| `iQDamper` | `This output behaves like rQDamper, but in order to suit an analog output it is output as an` | — |
| `bQAntiBlocking` | `This output is independent of the switch-on signal and shows only the Antiblocking-pulse.` | — |
| `bInvalidParameter` | `An error occurred during the plausibility check. It is deleted again by bReset.` | — |

**Description:** This function block prevents the blocking of an actuator over long time intervals with an unchanged control value. When activated, the blocking protection forces the actuator to drive from fully closed to fully open.

##### `FB_HVACAntiBlockingPump` — §5.1.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `bDisable` | `BOOL` | — |
| `bIn` | `BOOL` | — |
| `dtSystemTime` | `DT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The AntiBlocking function is enabled by the input variable bEnable. If bEnable = FALSE, only the` | — |
| `bDisable` | `Resets the outputs bOut, bOutAndAntiBlocking and bQAntiBlocking to 0. (see example below)` | — |
| `bIn` | `Switch-on condition that is relayed to the output bOut.` | — |
| `dtSystemTime` | `System time` | — |
| `bReset` | `Acknowledge input in the event of a fault. Resets the flag bInvalidParameter.` | — |
| `eAntiBlockingMode` | `E_HVACAntiBlockingMode` | — |
| `iHoursDowntime` | `INT` | — |
| `iDayWeekly` | `INT` | — |
| `iHour` | `INT` | — |
| `iMinute` | `INT` | — |
| `iLengthImpulse` | `INT` | — |
| `eHVACAntiBlockingMode` | `Enumeration value that specifies the type of Antiblocking method (see` | — |
| `eHVACAntiBlockingMode_Off` | `=0 : Off` | — |
| `eHVACAntiBlockingMode_Downtime` | `=1 : After the expiry of a standstill time in hours, the Antiblocking-pulse` | — |
| `eHVACAntiBlockingMode_Weekly` | `=2 : The antiblocking pulse is only generated on a certain day of the week` | — |
| `iHoursDowntime` | `For eHVACAntiBlockingMode_Downtime, the time in hours for which the input bIn may` | — |
| `iDayWeekly` | `For eHVACAntiBlockingMode_Weekly, AntiBlocking pulse is formed on this day. Sun=0,` | — |
| `iHour` | `Switch-on time in hours (0h..23h). The variable is saved persistently. Preset to 12.` | — |
| `iMinute` | `Switch-on time in minutes (0min..23min). The variable is saved persistently. Preset to 0.` | — |
| `iLengthImpulse` | `Switch-on time in seconds (0s..600s). The variable is saved persistently. Preset to 150.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bOutAndAntiBlocking` | `BOOL` | — |
| `bQAntiBlocking` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOut` | `If TRUE, then the request comes from the pump function block. This output corresponds to the switch-` | — |
| `bOutAndAntiBlocking` | `The output can go TRUE if the request comes from the pump function block OR` | — |
| `bQAntiBlocking` | `If TRUE, then request comes from the anti-blocking function. This output is independent of` | — |
| `bInvalidParameter` | `An error occurred during the plausibility check. It is deleted again by bReset.` | — |

**Description:** This function block relays the switch-on condition for a pump drive and can generate an additional switch-on pulse, depending on the mode and the switch-off time, in order e.g. to prevent the blockage of a pump.

##### `FB_HVACBlink` — §5.1.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `REAL` | — |
| `tTimeOn` | `TIME` | — |
| `tTimeOff` | `TIME` | — |
| `bEnable` | `Enables the function block.` | — |
| `tTimeOn` | `Switch-on duration of the flash pulse, bBlink = TRUE.` | — |
| `tTimeOff` | `Switch-off duration of the flash pulse, bBlink = FALSE.` | — |
| `eDST` | `E_HVACDataSecurityType` | — |
| `bEn` | `BOOL` | — |
| `bCmd` | `BOOL` | — |
| `bEvt1` | `BOOL` | — |
| `bEvt2` | `BOOL` | — |
| `bEvt3` | `BOOL` | — |
| `bEvt4` | `BOOL` | — |
| `bEvt5` | `BOOL` | — |
| `bEvt6` | `BOOL` | — |
| `bEvt7` | `BOOL` | — |
| `bEvt8` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `Enables the function block. The function block is disabled if bEn is FALSE. All outputs go to FALSE` | — |
| `bCmd` | `The switch-on or switch-off sequence of the function block is defined with bCmd. If bCmd is TRUE,` | — |
| `bEvt1` | `The event bEvt1 counts as one of the switch-on conditions of bQ1. In the switch-on phase of bQ1,` | — |
| `bEvt2` | `The event bEvt2 counts as one of the switch-on conditions of bQ2. In the switch-on phase of bQ2,` | — |
| `bEvt3` | `The event bEvt3 counts as one of the switch-on conditions of bQ3. In the switch-on phase of bQ3,` | — |
| `bEvt4` | `The event bEvt4 counts as one of the switch-on conditions of bQ4. In the switch-on phase of bQ4,` | — |
| `bEvt5` | `The event bEvt5 counts as one of the switch-on conditions of bQ5. In the switch-on phase of bQ5,` | — |
| `bEvt6` | `The event bEvt6 counts as one of the switch-on conditions of bQ6. In the switch-on phase of bQ6,` | — |
| `bEvt7` | `The event bEvt7 counts as one of the switch-on conditions of bQ7. In the switch-on phase of bQ7,` | — |
| `bEvt8` | `The event bEvt8 counts as one of the switch-on conditions of bQ8. In the switch-on phase of bQ8,` | — |
| `stParamCmd` | `ST_HVACCmdCtrl_8Param` | — |
| `stParam1` | `ST_HVACCmdCtrl_8Param` | — |
| `stParam2` | `ST_HVACCmdCtrl_8Param` | — |
| `stParam3` | `ST_HVACCmdCtrl_8Param` | — |
| `stParam4` | `ST_HVACCmdCtrl_8Param` | — |
| `stParam5` | `ST_HVACCmdCtrl_8Param` | — |
| `stParam6` | `ST_HVACCmdCtrl_8Param` | — |
| `stParam7` | `ST_HVACCmdCtrl_8Param` | — |
| `stParam8` | `ST_HVACCmdCtrl_8Param` | — |
| `stParamCmd` | `The structure contains time variables that can be used to define a switch-on or switch-off` | — |
| `stParam1` | `The structure contains time variables that can be used to define a switch-on or switch-off delay` | — |
| `stParam2` | `The structure contains time variables that can be used to define a switch-on or switch-off delay` | — |
| `stParam3` | `The structure contains time variables that can be used to define a switch-on or switch-off delay` | — |
| `stParam4` | `The structure contains time variables that can be used to define a switch-on or switch-off delay` | — |
| `stParam5` | `The structure contains time variables that can be used to define a switch-on or switch-off delay` | — |
| `stParam6` | `The structure contains time variables that can be used to define a switch-on or switch-off delay` | — |
| `stParam7` | `The structure contains time variables that can be used to define a switch-on or switch-off delay` | — |
| `stParam8` | `The structure contains time variables that can be used to define a switch-on or switch-off delay` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBlink` | `BOOL` | — |
| `tNextSwitching` | `TIME` | — |
| `bBlink` | `Flash output` | — |
| `tNextSwitching` | `Time until the next change of state of the output bBlink.` | — |
| `bQ_Cmd` | `BOOL` | — |
| `bQ1` | `BOOL` | — |
| `bQ2` | `BOOL` | — |
| `bQ3` | `BOOL` | — |
| `bQ4` | `BOOL` | — |
| `bQ5` | `BOOL` | — |
| `bQ6` | `BOOL` | — |
| `bQ7` | `BOOL` | — |
| `bQ8` | `BOOL` | — |
| `stState` | `ST_HVACCmdCtrl_8State` | — |
| `bQ_Cmd` | `Output to enable a device. If the output bQ_Cmd has been set in the switch-on sequence, then it` | — |
| `bQ1` | `Output to enable a device. If the output bQ1 has been set in the switch-on sequence, then it remains` | — |
| `bQ2` | `Output to enable a device. If the output bQ2 has been set in the switch-on sequence, then it remains` | — |
| `bQ3` | `Output to enable a device. If the output bQ3 has been set in the switch-on sequence, then it remains` | — |
| `bQ4` | `Output to enable a device. If the output bQ4 has been set in the switch-on sequence, then it remains` | — |
| `bQ5` | `Output to enable a device. If the output bQ5 has been set in the switch-on sequence, then it remains` | — |
| `bQ6` | `Output to enable a device. If the output bQ6 has been set in the switch-on sequence, then it remains` | — |
| `bQ7` | `Output to enable a device. If the output bQ7 has been set in the switch-on sequence, then it remains` | — |
| `bQ8` | `Output to enable a device. If the output bQ8 has been set in the switch-on sequence, then it remains` | — |
| `stState` | `This structure indicates the remaining times of the switch-on or switch-off delay of the active step` | — |
| `Example` | `If stState.udiStep = 5, then the function block is in step 5. The time variable of the parameter` | — |
| `Example` | `If stState.udiStep = 0, then the function block is in step 0. The time variable` | — |

**Description:** The function block delivers a flash sequence at the output bBlink that is dependent on the two adjustable times tTimeOn and tTimeOff.

##### `FB_HVACCmdCtrlSystem1Stage` — §5.1.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `eCtrlModeActuator` | `E_HVACActuatorMode` | — |
| `bExternalRequest` | `BOOL` | — |
| `bTimeScheduler` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If bEnable = FALSE, then bOn is also FALSE.` | — |
| `eCtrlModeActuator` | `Enum that specifies the system operation modes Manual, Auto and Off (see` | — |
| `bExternalRequest` | `External request for the system, e.g. from a control panel via a button or switch.` | — |
| `bTimeScheduler` | `Request for the system via a timer program.` | — |
| `bReset` | `Acknowledge input in the event of an error` | — |
| `ePlantMode` | `E_HVACPlantMode` | — |
| `eExternalRequestMode` | `E_HVACExternalRequestMode` | — |
| `tOffDelay` | `TIME` | — |
| `ePlantMode` | `With this enumeration variable various system functions can be executed in automatic mode,` | — |
| `eExternalRequestMode` | `The enum eExternalRequestMode specifies the mode of operation of the input` | — |
| `tOffDelay` | `Time value for the extension of the system utilization time. The extension of the utilization time` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOn` | `BOOL` | — |
| `eStateModeActuator` | `E_HVACActuatorMode` | — |
| `bRequestScheduler` | `BOOL` | — |
| `bRequestExternal` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOn` | `This output variable enables the system.` | — |
| `eStateModeActuator` | `Enum via which the state of the operation mode of the motor is fed back to the` | — |
| `bRequestScheduler` | `This output indicates that the system is requested by the input variable` | — |
| `bRequestExternal` | `This output indicates that the system is requested by the input variable` | — |
| `bInvalidParameter` | `Indicates that an incorrect parameter is present at one of the variables` | — |

**Description:** This function block FB_HVACCmdCtrlSystem1Stage is a system switch. It is used, for example, to switch a single-stage ventilation system to automatic or manual operation mode. In automatic operation mode the system can be controlled via a timer program or via the request from a control panel. The function block FB_HVACCmdCtrlSystem1Stage is active if the input variable bEnable is TRUE and eCtrlModeActuator = eHVACActuatorMode_Auto_BMS or eHVACActuatorMode_Auto_OP.

##### `FB_HVACCmdCtrlSystem2Stage` — §5.1.7.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `eCtrlModeActuator` | `E_HVACActuatorMode` | — |
| `bExternalRequest1` | `BOOL` | — |
| `bExternalRequest2` | `BOOL` | — |
| `bTimeScheduler1` | `BOOL` | — |
| `bTimeScheduler2` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE. If bEnable = FALSE, then bOn1 and bOn2 are` | — |
| `eCtrlModeActuator` | `Enum that specifies the system operation modes Manual, Auto and Off (see` | — |
| `bExternalRequest1` | `External request for the system in stage 1, e.g. from a control panel via a button or` | — |
| `bExternalRequest2` | `External request for the system in stage 2, e.g. from a control panel via a button or` | — |
| `bTimeScheduler1` | `Request for the system in stage 1 via a timer program.` | — |
| `bTimeScheduler2` | `Request for the system in stage 2 via a timer program.` | — |
| `bReset` | `Acknowledge input in the event of an error.` | — |
| `ePlantMode` | `E_HVACPlantMode` | — |
| `eExternalRequestMode` | `E_HVACExternalRequestMode` | — |
| `tOffDelay1` | `TIME` | — |
| `tOffDelay2` | `TIME` | — |
| `ePlantMode` | `With this enumeration variable various system functions can be executed in automatic` | — |
| `eExternalRequestMode` | `The enum eExternalRequestMode specifies the mode of operation of the input` | — |
| `tOffDelay1` | `Time value for the extension of the system utilization time in stage 1. The extension of the` | — |
| `tOffDelay2` | `Time value for the extension of the system utilization time in stage 2. The extension of the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOn1` | `BOOL` | — |
| `bOn2` | `BOOL` | — |
| `eStateModeActuator` | `E_HVACActuatorMode` | — |
| `bRequestScheduler1` | `BOOL` | — |
| `bRequestScheduler2` | `BOOL` | — |
| `bRequestExternal1` | `BOOL` | — |
| `bRequestExternal2` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOn1` | `The system is enabled for stage 1 by this output variable.` | — |
| `bOn2` | `The system is enabled for stage 2 by this output variable. If stage 2 is enabled, then stage 1 is` | — |
| `eStateModeActuator` | `Enum via which the state of the operation mode of the motor is fed back to the` | — |
| `bRequestScheduler1` | `This output indicates that the system is requested by the input variable` | — |
| `bRequestScheduler2` | `This output indicates that the system is requested by the input variable` | — |
| `bRequestExternal1` | `This output indicates that the system is requested by the input variable` | — |
| `bRequestExternal2` | `This output indicates that the system is requested by the input variable` | — |
| `bInvalidParameter` | `Indicates that an incorrect parameter is present at one of the variables` | — |

**Description:** This function block FB_HVACCmdCtrlSystem2Stage is a system switch. It is used, for example, to switch a two-stage ventilation system to automatic or manual operation mode. In automatic operation mode the system can be controlled via a timer program or via the request from a control panel. The FB_HVACCmdCtrlSystem2Stage function block is active if the input variable bEnable is TRUE and eCtrlModeActuator = eHVACActuatorMode_Auto_BMS or eHVACActuatorMode_Auto_OP.

##### `FB_HVACConvertEnum` — §5.1.7.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pEnum` | `UDINT` | — |
| `udiLenEnum` | `UDINT` | — |
| `pInt` | `UDINT` | — |
| `udiLenInt` | `UDINT` | — |
| `pEnum` | `Address of the enum to be converted. The address is determined with the ADR operator.` | — |
| `udiLenEnum` | `Number of bytes of which the data type Enum consists. The number is determined with the` | — |
| `pInt` | `Address the integer variable to be converted. The address is determined with the ADR operator.` | — |
| `udiLenInt` | `Number of bytes of which the data type Integer consists. The number is determined with the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `bError` | `This output indicates with a TRUE that there is an error. (The number of bytes of which the data` | — |
| `eErrorCode` | `Supplies the error number [} 490] when the output bError is set. The following errors can occur` | — |

**Description:** This function block converts an Enum into an integer value and vice versa. This conversion is particularly suitable for Enums that are used as VAR_IN_OUT variables on function blocks.

##### `FB_HVACEnthalpy` — §5.1.7.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rTemperature` | `REAL` | — |
| `rRelativeHumidity` | `REAL` | — |
| `rBarometricPressure` | `REAL` | — |
| `rTemperature` | `Input for the temperature value [°C].` | — |
| `rRelativeHumidity` | `Input for the relative humidity [%]. The value must equal or greater 1.` | — |
| `rBarometricPressure` | `Input for the air pressure [hPa].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rDewPoint` | `REAL` | — |
| `rEnthalpy` | `REAL` | — |
| `rAbsoluteHumidity` | `REAL` | — |
| `rRho` | `REAL` | — |
| `rYpsilon` | `REAL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `rDewPoint` | `Dew point [°C ].` | — |
| `rEnthalpy` | `Enthalpy [kJ / kg].` | — |
| `rAbsoluteHumidity` | `Absolute humidity [kg/kg]. The result must be multiplied by 1000 in order to obtain the` | — |
| `rRho` | `Density of moist air, which specifies the mixture as a mass in relation to the volume. The unit is [kg/` | — |
| `rYpsilon` | `Specific volume per 1 kg of dry air. The unit is [m³/kg] dry air.` | — |
| `bInvalidParameter` | `Set to TRUE if the input variable rRelativeHumidity or rBarometricPressure is smaller` | — |
| `rTemperature` | `= 20 °C` | — |
| `rRelativeHumidity` | `= 50%` | — |
| `rBarometricPressure` | `= 1013.15hPa` | — |
| `rEnthalpy` | `= 38.6kJ/kg` | — |
| `rAbsoluteHumidity` | `= 0.0072kg/kg, converted to g/kg ==> 7.2 g/kg` | — |
| `rTemperature` | `= 25 °C` | — |
| `rRelativeHumidity` | `= 60%` | — |
| `rBarometricPressure` | `= 1013.15hPa` | — |
| `rEnthalpy` | `= 55.6kJ/kg` | — |
| `rAbsoluteHumidity` | `= 0.0119kg/kg, converted to g/kg ==> 11.9 g/kg` | — |

**Description:** This function block is used to calculate the dew point, the specific enthalpy and the absolute humidity. The temperature, the relative humidity and the barometric pressure are required for calculating these parameters. The enthalpy is a measure for the energy of a thermodynamic system.

##### `FB_HVACFixedLimit` — §5.1.7.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rInputValue` | `REAL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `The default values of the VAR_IN_OUT variables are adopted with a rising edge at this input` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE.` | — |
| `rInputValue` | `Input signal` | — |
| `bReset` | `Input for acknowledging the incorrect parameter entry via a rising edge.` | — |
| `bModeFixedLimit` | `BOOL` | — |
| `rHighLimit` | `REAL` | — |
| `rLowLimit` | `REAL` | — |
| `tDelayHighLimit` | `TIME` | — |
| `tDelayLowLimit` | `TIME` | — |
| `bModeFixedLimit` | `If bModeFixedLimit = TRUE then the output variable bEvent becomes TRUE if the input` | — |
| `rHighLimit` | `Upper limit value.` | — |
| `rLowLimit` | `Lower limit value.` | — |
| `tDelayHighLimit` | `Switching delay [s] when the upper limit value is exceeded.` | — |
| `tDelayLowLimit` | `Switching delay [s] when the lower limit value is undershot.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEvent` | `BOOL` | — |
| `bEdgeHighLimit` | `BOOL` | — |
| `bEdgeLowLimit` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bEvent` | `Output signal, which is switched depending on the time delays tDelayHighLimit / tDelayLowLimit,` | — |
| `bEdgeHighLimit` | `Is TRUE for one PLC cycle after the input signal rInputValue has exceeded the variable` | — |
| `bEdgeLowLimit` | `Is TRUE for one PLC cycle after the input signal rInputValue has undershot the variable` | — |
| `bInvalidParameter` | `Indicates that an incorrect parameter is present at one of the variables rHighLimit,` | — |

**Description:** This function block represents a limit value switch, whose output signal is switched depending on the time delays tDelayHighLimit / tDelayLowLimit, the mode bModeFixedLimit, the limit values rHighLimit / rLowLimit and the input signal rInputValue.

##### `FB_HVACFreezeProtectionHeater` — §5.1.7.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bThermostat` | `BOOL` | — |
| `rReturnWaterTemp` | `REAL` | — |
| `bCtrlVoltage` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `iNumberOfValues` | `INT` | — |
| `bEdgeNewValue` | `BOOL` | — |
| `bDirection` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `iManualValue` | `INT` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `iNumberOfValues` | `INT` | — |
| `bEdgeNewValue` | `BOOL` | — |
| `bDirection` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `iManualValue` | `INT` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `iNumberOfValues` | `INT` | — |
| `bEdgeNewValue` | `BOOL` | — |
| `bDirection` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `iManualValue` | `INT` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `iNumberOfValues` | `INT` | — |
| `bEdgeNewValue` | `BOOL` | — |
| `bDirection` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `iManualValue` | `INT` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bThermostat` | `signal from the frost protection thermostat. In the normal case TRUE.` | — |
| `rReturnWaterTemp` | `water temperature in the return flow of the air heater.` | — |
| `bCtrlVoltage` | `control voltage monitor. If control voltage is present bCtrlVoltage = TRUE.` | — |
| `bReset` | `Input for acknowledging the fault.` | — |
| `rReturnWaterAlarmLimit` | `REAL` | — |
| `rReturnWaterNormalLimit` | `REAL` | — |
| `tDelayHardwareReset` | `TIME` | — |
| `tMonitoringAfterReset` | `TIME` | — |
| `rReturnWaterAlarmLimit` | `The frost alarm is activated below the temperature value rReturnWaterAlarmLimit` | — |
| `rReturnWaterNormalLimit` | `The air heater returns to its normal state when the temperature is once again` | — |
| `tDelayHardwareReset` | `Time that elapses, after input, between the input of the frost error and output of a` | — |
| `tMonitoringAfterReset` | `Monitoring time after the first frost warning. The variable is saved persistently.` | — |
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then iValueOut and iValueNumber` | — |
| `iNumberOfValues` | `Indicates the number of variables iValue1-8 that can be output via iValueOut. If` | — |
| `bEdgeNewValue` | `iValueNumber is incremented or decremented if there is a rising edge on` | — |
| `bDirection` | `bDirection determines the control direction of the function block.` | — |
| `eCtrlMode` | `This enum specifies the operation mode of the function block (see E_HVACCtrlMode [} 489]).` | — |
| `iManualValue` | `If the operation mode eCtrlMode = eHVACCtrlMode_Manual AND bEnable = TRUE AND` | — |
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then iValueOut and iValueNumber` | — |
| `iNumberOfValues` | `Indicates the number of variables iValue1-16 that can be output via iValueOut. If` | — |
| `bEdgeNewValue` | `iValueNumber is incremented or decremented if there is a rising edge on` | — |
| `bDirection` | `bDirection determines the control direction of the function block.` | — |
| `eCtrlMode` | `The operation mode of the function block is specified by means of this Enum.` | — |
| `iManualValue` | `If the operation mode eCtrlMode = eHVACCtrlMode_Manual AND bEnable = TRUE AND` | — |
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then rValueOut and iValueNumber` | — |
| `iNumberOfValues` | `Indicates the number of variables rValue1-8 that can be output via rValueOut. If` | — |
| `bEdgeNewValue` | `iValueNumber is incremented or decremented if there is a rising edge on` | — |
| `bDirection` | `bDirection determines the control direction of the function block.` | — |
| `eCtrlMode` | `This enum specifies the operation mode of the function block (see E_HVACCtrlMode [} 489]).` | — |
| `iManualValue` | `If the operation mode eCtrlMode = eHVACCtrlMode_Manual AND bEnable = TRUE AND` | — |
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then rValueOut and iValueNumber` | — |
| `iNumberOfValues` | `Indicates the number of variables rValue1-16 that can be output via rValueOut. If` | — |
| `bEdgeNewValue` | `iValueNumber is incremented or decremented if there is a rising edge on` | — |
| `bDirection` | `bDirection determines the control direction of the function block.` | — |
| `eCtrlMode` | `This enum specifies the operation mode of the function block (see E_HVACCtrlMode [} 489]).` | — |
| `iManualValue` | `If the operation mode eCtrlMode = eHVACCtrlMode_Manual AND bEnable = TRUE AND` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `byState` | `BYTE` | — |
| `bFrostProtecRunning` | `BOOL` | — |
| `bHardwareReset` | `BOOL` | — |
| `bErrorGeneral` | `BOOL` | — |
| `bAlarmReturnWaterTemp` | `BOOL` | — |
| `bAlarmThermostat` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bInvalidParameter` | `BOOL` | — |
| `byState` | `Output of the state as a byte.` | — |
| `bFrostProtecRunning` | `TRUE if the frost protection program is active.` | — |
| `bHardwareReset` | `Output that indicates with a 1 s pulse that the frost protection relay is self-latched again` | — |
| `bErrorGeneral` | `There is a general error.` | — |
| `bAlarmFrost` | `TRUE if there is a frost warning.` | — |
| `bAlarmReturnWaterTemp` | `The water temperature limit in the return flow of the air heater was undershot.` | — |
| `bAlarmThermostat` | `TRUE if there is an error in the frost protection thermostat.` | — |
| `byError` | `Output of the error as a byte.` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |
| `iValueOut` | `INT` | — |
| `bActive` | `BOOL` | — |
| `bEdgeNewValueOut` | `BOOL` | — |
| `iValueNumber` | `INT` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `iValueOut` | `The value of the variable iValue1-8 is output via iValueOut.` | — |
| `bActive` | `bActive goes TRUE if` | — |
| `bEdgeNewValueOut` | `Is TRUE for one PLC cycle if bEnable = TRUE AND bError = FALSE AND` | — |
| `iValueNumber` | `The output iValueNumber always indicates the number of the variable iValueX, whose` | — |
| `bError` | `The output signals with a TRUE that there is an error and that there is a wrong parameter on the` | — |
| `eErrorCode` | `Supplies the error number [} 490] when the output bError is set. The following error can occur` | — |
| `eHVACErrorCodes_InvalidParam_iNumberOfValues` | `There is an incorrect value on iNumberOfValues.` | — |
| `iValueOut` | `INT` | — |
| `bActive` | `BOOL` | — |
| `bEdgeNewValueOut` | `BOOL` | — |
| `iValueNumber` | `INT` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `iValueOut` | `The value of the variable iValue1-16 is output via iValueOut.` | — |
| `bActive` | `bActive goes TRUE if` | — |
| `bEdgeNewValueOut` | `Is TRUE for one PLC cycle if bEnable = TRUE AND bError = FALSE AND` | — |
| `iValueNumber` | `The output iValueNumber always indicates the number of the variable iValueX, whose` | — |
| `bError` | `The output signals with a TRUE that there is an error and that there is a wrong parameter on the` | — |
| `eErrorCode` | `Supplies the error number [} 490] when the output bError is set. The following error can occur` | — |
| `eHVACErrorCodes_InvalidParam_iNumberOfValues` | `There is an incorrect value on iNumberOfValues.` | — |
| `rValueOut` | `INT` | — |
| `bActive` | `BOOL` | — |
| `bEdgeNewValueOut` | `BOOL` | — |
| `iValueNumber` | `INT` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `rValueOut` | `The value of the variable rValue1-8 is output via rValueOut.` | — |
| `bActive` | `bActive goes TRUE if` | — |
| `bEdgeNewValueOut` | `Is TRUE for one PLC cycle if bEnable = TRUE AND bError = FALSE AND` | — |
| `iValueNumber` | `The output iValueNumber always indicates the number of the variable rValueX, whose` | — |
| `bError` | `The output signals with a TRUE that there is an error and that there is a wrong parameter on the` | — |
| `eErrorCode` | `Supplies the error number [} 490] when the output bError is set. The following error can occur` | — |
| `eHVACErrorCodes_InvalidParam_iNumberOfValues` | `There is an incorrect value on iNumberOfValues.` | — |
| `rValueOut` | `REAL` | — |
| `bActive` | `BOOL` | — |
| `bEdgeNewValueOut` | `BOOL` | — |
| `iValueNumber` | `INT` | — |
| `bError` | `BOOL` | — |
| `eErrorCode` | `E_HVACErrorCodes` | — |
| `rValueOut` | `The value of the variable rValue1-16 is output via rValueOut.` | — |
| `bActive` | `bActive goes TRUE if` | — |
| `bEdgeNewValueOut` | `Is TRUE for one PLC cycle if bEnable = TRUE AND bError = FALSE AND` | — |
| `iValueNumber` | `The output iValueNumber always indicates the number of the variable rValueX, whose` | — |
| `bError` | `The output signals with a TRUE that there is an error and that there is a wrong parameter on the` | — |
| `eErrorCode` | `Supplies the error number [} 490] when the output bError is set. The following error can occur` | — |
| `eHVACErrorCodes_InvalidParam_iNumberOfValues` | `There is an incorrect value on iNumberOfValues.` | — |

**Description:** This function block is for the frost protection monitoring of an air heater in an air conditioning system. Frost protection monitoring takes place both on the air side via a frost protection thermostat and on the water side via a temperature sensor in the heater return flow.

##### `FB_HVACOverwriteAnalog` — §5.1.7.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rValueIn` | `REAL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `rManualValueIn` | `REAL` | — |
| `rValueIn` | `Analog input value that is forwarded to the output rValueOut if eCtlrMode = eHVACCtrlMode_Auto.` | — |
| `eCtrlMode` | `The operation mode is selected via this enum (see E_HVACCtrlMode [} 489]). Manual or` | — |
| `rManualValueIn` | `Analog manual input value that is forwarded to the output rValueOut if eCtrlMode =` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValueOut` | `REAL` | — |
| `rValueOut` | `Analog output value.` | — |

**Description:** This function block overrides the input rValueIn if eCtrlMode = eHVACCtrlMode_Manual and forwards rManualValueIn to the output rValueOut.

##### `FB_HVACOverwriteDigital` — §5.1.7.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bIn` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `bManualIn` | `BOOL` | — |
| `bIn` | `Digital input variable whose state is forwarded to the output bOut if eCtlrMode = eHVACCtrlMode_Auto.` | — |
| `eCtrlMode` | `The operation mode is selected via this enum (see E_HVACCtrlMode [} 489]). Manual or` | — |
| `bManualIn` | `Digital input variable whose state is forwarded to the output bOut if eCtlrMode =` | — |
| `diCurrTransFactor` | `DINT` | — |
| `eModePowerMeasurement` | `E_HVACPowerMeasurementMode` | — |
| `bDelEnergyConsumption` | `BOOL` | — |
| `wDataLx` | `Input data from the three channels of the KL3403.` | — |
| `bySBLx` | `Status byte of the three channels of the KL3403. Reports which value can be read via the input` | — |
| `diCurrTransFactor` | `Transformation factor of the current transformer, used for conversion to the actual string` | — |
| `eModePowerMeasurement` | `If the value of this parameter lies between 1 and 8, the automatic read-out of` | — |
| `eHVACPowerMeasurementMode_AutoAllValues` | `= 0,` | — |
| `eHVACPowerMeasurementMode_Current` | `= 1,` | — |
| `eHVACPowerMeasurementMode_Voltage` | `= 2,` | — |
| `eHVACPowerMeasurementMode_EffectivePower` | `= 3,` | — |
| `eHVACPowerMeasurementMode_PowerFactor` | `= 4,` | — |
| `eHVACPowerMeasurementMode_EnergyConsumption` | `= 5,` | — |
| `eHVACPowerMeasurementMode_PeakCurrentValu` | `= 6, 1)` | — |
| `eHVACPowerMeasurementMode_PeakVoltageValue` | `= 7, 1)` | — |
| `eHVACPowerMeasurementMode_PeakPowerValue` | `= 81)` | — |
| `bDelEnergyConsumption` | `A positive edge at this input deletes the energy consumption in the EEPROM.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bOut` | `Digital output variable.` | — |
| `diPg` | `DINT;       kW     Pges Auflösung: 0,1 kW` | — |
| `diCosPhi` | `DINT;       cosPhiges Auflösung: 0,01` | — |
| `diWg` | `DINT;       kWh    Energieverbrauch Auflösung: 1 kWh` | — |
| `diSg` | `DINT;       kVA    Sges Auflösung: 0,1 kVA` | — |
| `diQg` | `DINT;       kvar   Qges Auflösung: 0,1 kvar` | — |
| `dummy` | `DINT;       Reserve, füllt die Struktur auf 120Bytes auf` | — |
| `bDelEnergyMeasuredValuesBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `iErrID` | `UDINT` | — |
| `stQ_PowerMeasurement` | `ST_HVACPowerMeasurement` | — |
| `bDelEnergyMeasuredValuesBusy` | `Since the measured energy values have to be deleted from the internal` | — |
| `bError` | `If TRUE, an error has occurred in the register communication.` | — |
| `iErrID` | `Error ID in the register communication` | — |
| `byCBLx` | `This output serves the selection of the desired input value and register communication in order to` | — |
| `iDataOutLx` | `This output serves the register communication in order to delete the energy consumption.` | — |
| `stQ_PowerMeasurement` | `The results of the power measurement as an output data structure (see` | — |

**Description:** This function block overrides the state of the input bIn if eCtrlMode = eHVACCtrlMode_Manual and forwards it to the output bOut.

##### `FB_HVACPowerMeasurementKL3403Ex` — §5.1.7.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `diCurrTransFactor` | `LREAL` | — |
| `bDelEnergyConsumption` | `BOOL` | — |
| `wDataLx` | `Input data from the three channels of the KL3403.` | — |
| `bySBLx` | `Status byte of the three channels of the KL3403. Reports which value can be read via the input` | — |
| `diCurrTransFactor` | `Transformation factor of the current transformer, used for conversion to the actual string` | — |
| `bDelEnergyConsumption` | `A positive edge at this input deletes the energy consumption in the EEPROM.` | — |
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `iManualValue` | `INT;             0..8` | — |
| `bDirection` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `iManualValue` | `INT;             0..16` | — |
| `bDirection` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `iManualValue` | `INT;             0..8` | — |
| `bDirection` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bEnable` | `BOOL` | — |
| `eCtrlMode` | `E_HVACCtrlMode` | — |
| `iManualValue` | `INT;             0..16` | — |
| `bDirection` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then iValueOut is set constantly to 0.` | — |
| `eCtrlMode` | `This enum specifies the operation mode of the function block (see E_HVACCtrlMode [} 489]).` | — |
| `iManualProfile` | `If the operation mode eCtrlMode = eHVACCtrlMode_Manual (Multiplexer) AND bEnable =` | — |
| `bDirection` | `The function block can be used to prioritize events if eCtrlMode = eHVACCtrlMode_Auto. The` | — |
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then iValueOut is set constantly to 0.` | — |
| `eCtrlMode` | `This enum specifies the operation mode of the function block (see E_HVACCtrlMode [} 489]).` | — |
| `iManualProfile` | `If the operation mode eCtrlMode = eHVACCtrlMode_Manual (Multiplexer) AND bEnable =` | — |
| `bDirection` | `The function block can be used to prioritize events if eCtrlMode = eHVACCtrlMode_Auto. The` | — |
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then rValueOut is set constantly to` | — |
| `eCtrlMode` | `This enum specifies the operation mode of the function block (see E_HVACCtrlMode [} 489]).` | — |
| `iManualProfile` | `If the operation mode eCtrlMode = eHVACCtrlMode_Manual (Multiplexer) AND bEnable =` | — |
| `bDirection` | `The function block can be used to prioritize events if eCtrlMode = eHVACCtrlMode_Auto. The` | — |
| `bEnable` | `The function block is enabled via TRUE. If bEnable = FALSE, then rValueOut is set constantly to` | — |
| `eCtrlMode` | `This enum specifies the operation mode of the function block (see E_HVACCtrlMode [} 489]).` | — |
| `iManualProfile` | `If the operation mode eCtrlMode = eHVACCtrlMode_Manual (Multiplexer) AND bEnable =` | — |
| `bDirection` | `The function block can be used to prioritize events if eCtrlMode = eHVACCtrlMode_Auto. The` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDelEnergyMeasuredValuesBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `iErrID` | `UDINT` | — |
| `stQ_PowerMeasurementEx` | `ST_HVACPowerMeasurementEx` | — |
| `bDelEnergyMeasuredValuesBusy` | `Since the measured energy values are deleted from the internal` | — |
| `bError` | `If TRUE, an error has occurred in the register communication.` | — |
| `iErrID` | `Error ID in the register communication` | — |
| `byCBLx` | `This output serves the selection of the desired input value and register communication in order to` | — |
| `iDataOutLx` | `This output serves the register communication in order to delete the energy consumption.` | — |
| `stQ_PowerMeasurementEx` | `The results of the power measurement as an output data structure (see` | — |
| `iValueOut` | `INT` | — |
| `bActive` | `BOOL` | — |
| `bEdgeNewEvent` | `BOOL` | — |
| `iValueNumber` | `INT` | — |
| `iValueOut` | `This function block can be used to prioritize events or as a multiplexer. This selection is made` | — |
| `bActive` | `bActive goes TRUE if` | — |
| `bEdgeNewEvent` | `Is TRUE for one PLC cycle if` | — |
| `iValueNumber` | `The variable iValueNumber indicates the variable from which the value on iValueOut is` | — |
| `iValueOut` | `INT` | — |
| `bActive` | `BOOL` | — |
| `bEdgeNewEvent` | `BOOL` | — |
| `iValueNumber` | `INT` | — |
| `iValueOut` | `This function block can be used to prioritize events or as a multiplexer. This selection is made` | — |
| `bActive` | `bActive goes TRUE if` | — |
| `bEdgeNewEvent` | `Is TRUE for one PLC cycle if` | — |
| `iValueNumber` | `The variable iValueNumber indicates the variable from which the value on iValueOut is` | — |
| `rValueOut` | `REAL` | — |
| `bActive` | `BOOL` | — |
| `bEdgeNewEvent` | `BOOL` | — |
| `iValueNumber` | `INT` | — |
| `rValueOut` | `This function block can be used to prioritize events or as a multiplexer. This selection is made` | — |
| `bActive` | `bActive goes TRUE if` | — |
| `bEdgeNewEvent` | `Is TRUE for one PLC cycle if` | — |
| `iValueNumber` | `The variable iValueNumber indicates the variable from which the value on rValueOut is` | — |
| `rValueOut` | `REAL` | — |
| `bActive` | `BOOL` | — |
| `bEdgeNewEvent` | `BOOL` | — |
| `iValueOut` | `INT` | — |
| `rValueOut` | `This function block can be used to prioritize events or as a multiplexer. This selection is made` | — |
| `bActive` | `bActive goes TRUE if` | — |
| `bEdgeNewEvent` | `Is TRUE for one PLC cycle if` | — |
| `iValueNumber` | `The variable iValueNumber indicates the variable from which the value on rValueOut is` | — |

**Description:** This function block serves to control a 3-phase power measurement terminal (KL/KS3403). The terminal data is read out and all variables that depend on it are derived. The Bus Terminal KL3403 enables the measurement of all relevant electrical data of the supply network. The voltage is measured via the direct connection of L1, L2, L3 and N. The current of the three phases L1, L2 and L3 is fed via simple current transformers. All measured currents and voltages are available as RMS values. In the KL3403 version, the active power and the energy consumption for each phase are calculated. Through the relationship of the RMS values of voltage U * current I and the active power P, all other information such as apparent power or phase shift angle cos φ can be derived. For each fieldbus, KL3403 provides a comprehensive network analysis and an energy management option. The data is read out in 8 groups one after the other. Dependent values are calculated cyclically. The energy measurement is read out from the terminal as a 32-bit value with the overflow from register 1. The input bDelEnergyConsumption can be used to delete this value.

##### `FB_HVACOptimizedOn` — §5.1.7.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `eCtrlFct` | `E_BARCtrlFct` | — |
| `bEnable` | `BOOL` | — |
| `bSchedulerActive` | `BOOL` | — |
| `bWithAdaption` | `BOOL` | — |
| `bDisableAdaption` | `BOOL` | — |
| `rOutsideTemperature` | `REAL` | — |
| `rRoomTemperature` | `REAL` | — |
| `rRoomSetpointValue` | `REAL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eCtrlFct` | `eCtrlFct = eBARCtrlFct_Heating indicates to the function block that it is to be used in heating mode` | — |
| `bEnable` | `A FALSE signal at this input suppresses the optimized switch-on of the connected timers. The` | — |
| `bSchedulerActive` | `A rising edge at this input starts the internal countdown of the pre-start time. The output` | — |
| `bWithAdaption` | `If this input is set simultaneously with the input bSchedulerActive, an adaptation takes` | — |
| `bDisableAdaption` | `A TRUE signal at this input merely suppresses the adaptation that follows the` | — |
| `rOutsideTemperature` | `Outside temperature in degrees Celsius.` | — |
| `rRoomTemperature` | `Room temperature in degrees Celsius.` | — |
| `rRoomSetpointValue` | `Room temperature setpoint in degrees Celsius.` | — |
| `uiMaxPrestartTime` | `UINT` | — |
| `rAdaptionFactor` | `REAL` | — |
| `rAdaptionTolerance` | `REAL` | — |
| `stTempChangeFunction` | `ST_HVACTempChangeFunction` | — |
| `uiMaxPrestartTime` | `Due to the adaptation the pre-start times are shortened and lengthened within the` | — |
| `rAdaptionFactor` | `In the adaptation step the temperature change ΔT/Δt that has taken place is calculated for` | — |
| `rAdaptionTolerance` | `If the countdown has expired and the room temperature setpoint has been reached` | — |
| `stTempChangeFunction` | `Structure variable of the type ST_HVACTempChangeFunction [} 498] that contains` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiPrestartTime` | `UINT` | — |
| `bPrestartActive` | `BOOL` | — |
| `udiCountdownTime` | `UDINT` | — |
| `bError` | `BOOL` | — |
| `udiErrorID` | `UDINT` | — |
| `uiPrestartTime` | `Output value of the optimized pre-start time to the timers concerned in minutes. This value` | — |
| `bPrestartActive` | `The function block is in pre-start mode as long as the internal countdown is running and` | — |
| `udiCountdownTime` | `This output indicates the elapsing of the internal countdown in seconds. This output is` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Function block for the optimized switch-on of boilers and air conditioning systems in conjunction with switching time function blocks.

##### `FB_HVACOptimizedOff` — §5.1.7.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `eCtrlFct` | `E_BARCtrlFct` | — |
| `bEnable` | `BOOL` | — |
| `bSchedulerOff` | `BOOL` | — |
| `bWithAdaption` | `BOOL` | — |
| `bDisableAdaption` | `BOOL` | — |
| `rOutsideTemperature` | `REAL` | — |
| `rRoomTemperature` | `REAL` | — |
| `rRequestedTempDeviation` | `REAL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eCtrlFct` | `eCtrlFct = eBARCtrlFct_Heating indicates to the function block that it is to be used in heating mode` | — |
| `bEnable` | `A FALSE signal at this input suppresses the optimized switch-off of the connected timers. The` | — |
| `bSchedulerOff` | `A rising edge at this input starts the internal countdown of the pre-stop time. The output` | — |
| `bWithAdaption` | `If this input is set simultaneously with the input bSchedulerOff, an adaptation takes place` | — |
| `bDisableAdaption` | `A TRUE signal at this input merely suppresses the adaptation that follows the` | — |
| `rOutsideTemperature` | `Outside temperature in degrees Celsius.` | — |
| `rRoomTemperature` | `Room temperature in degrees Celsius.` | — |
| `rRequestedTempDeviation` | `Tolerated downward (heating mode) or upward (cooling mode) temperature` | — |
| `uiMaxPrestopTime` | `UINT` | — |
| `rAdaptionFactor` | `REAL` | — |
| `rAdaptionTolerance` | `REAL` | — |
| `stTempChangeFunction` | `ST_HVACTempChangeFunction` | — |
| `uiMaxPrestopTime` | `Due to the adaptation the pre-stop times are shortened and lengthened within the` | — |
| `rAdaptionFactor` | `In the adaptation step the temperature change ΔT/Δt that has taken place is calculated for` | — |
| `rAdaptionTolerance` | `If the countdown has elapsed and the requested deviation rRequestedDeviation is` | — |
| `stTempChangeFunction` | `Structure variable of the type ST_HVACTempChangeFunction [} 498] that contains` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiPrestopTime` | `UINT` | — |
| `bPrestopActive` | `BOOL` | — |
| `udiCountdownTime` | `UDINT` | — |
| `bError` | `BOOL` | — |
| `udiErrorID` | `UDINT` | — |
| `uiPrestopTime` | `Output value of the optimized pre-stop time to the timers concerned in minutes. This value` | — |
| `bPrestopActive` | `The function block is in pre-stop mode as long as the internal countdown is running and` | — |
| `udiCountdownTime` | `This output indicates the elapsing of the internal countdown in seconds. This output is` | — |
| `bError` | `This output is switched to TRUE if the parameters entered are erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Function block for the optimized switch-off (pre-stop) of boilers and air conditioning systems in conjunction with switching time function blocks.

##### `FB_HVACTempChangeFunction` — §5.1.7.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bWrite` | `BOOL` | — |
| `rOutsideTemp_1` | `REAL` | — |
| `rRoomTempChange_1` | `REAL` | — |
| `rOutsideTemp_2` | `REAL` | — |
| `rRoomTempChange_2` | `REAL` | — |
| `rOutsideTemp_3` | `REAL` | — |
| `rRoomTempChange_3` | `REAL` | — |
| `rOutsideTemp_4` | `REAL` | — |
| `rRoomTempChange_4` | `REAL` | — |
| `rOutsideTemp_5` | `REAL` | — |
| `rRoomTempChange_5` | `REAL` | — |
| `rOutsideTemp_6` | `REAL` | — |
| `rRoomTempChange_6` | `REAL` | — |
| `rOutsideTemp_7` | `REAL` | — |
| `rRoomTempChange_7` | `REAL` | — |
| `rOutsideTemp_8` | `REAL` | — |
| `rRoomTempChange_8` | `REAL` | — |
| `rOutsideTemp_9` | `REAL` | — |
| `rRoomTempChange_9` | `REAL` | — |
| `rOutsideTemp_10` | `REAL` | — |
| `rRoomTempChange_10` | `REAL` | — |
| `bWrite` | `A rising edge at this input copies the values entered at the inputs to the pre-start function.` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTempChangeFunction` | `ST_HVACTempChangeFunction` | — |
| `stTempChangeFunction` | `structure variable of type ST_HVACTempChangeFunction [} 498], which contains` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL` | — |
| `udiErrorID` | `UDINT` | — |
| `bValid` | `This output is switched to TRUE if the parameters entered are not erroneous.` | — |
| `udiErrorId` | `Contains the error code if the values entered should be erroneous. See Error codes [} 317].` | — |

**Description:** Function block for the input of the interpolation points of the pre-start function.

##### `FB_HVACPWM` — §5.1.7.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rYIn` | `REAL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled if bEnable = TRUE.` | — |
| `rYIn` | `Analog input variables of the function block (0%..100%).` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `tPWMPeriod` | `TIME` | — |
| `tMINPowerOnTime` | `TIME` | — |
| `tPWMPeriod` | `Period of the PWM signal. The variable is saved persistently. Preset to 30 min.` | — |
| `tMINPowerOnTime` | `Minimum switch-on time of the pulsed output bOut. The variable is saved persistently.` | — |
| `Figure` | `Scope2 recording for additional explanation of the mode of operation of the function block. The` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `rYOut` | `REAL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOut` | `PWM signal.` | — |
| `rYOut` | `Output of the input variable of the function block (0%..100%).` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |

**Description:** This function block generates a pulse width modulated signal from the analog input signal rYIn. Furthermore, a minimum switch-on time can be parameterized.

##### `FB_HVACStartAirConditioning` — §5.1.7.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `iStartSeqWinter` | `INT` | — |
| `iStartSeqSummer` | `INT` | — |
| `bTimeScheduler` | `BOOL` | — |
| `bNightCooling` | `BOOL` | — |
| `bFreezeProtec` | `BOOL` | — |
| `bOverHeatingProtec` | `BOOL` | — |
| `rOutsideTemp` | `REAL` | — |
| `rReturnWaterTemp` | `REAL` | — |
| `rSupplyAirTemp` | `REAL` | — |
| `rReturnAirTemp` | `REAL` | — |
| `bDampersFeedbOpen` | `BOOL` | — |
| `bEmergencyStop` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `iStartSeqWinter` | `The number of the sequence controller for the pre-heater must be entered here.` | — |
| `iStartSeqSummer` | `The number of the sequence controller for the mixed air chamber or for the heat` | — |
| `bTimeScheduler` | `Input for the switch command.` | — |
| `bNightCooling` | `Input for the night cooling function.` | — |
| `bFreezeProtec` | `Input for the frost protection function.` | — |
| `bOverHeatingProtec` | `Input for the overheating protection function.` | — |
| `rOutsideTemp` | `Input for the outside temperature.` | — |
| `rReturnWaterTemp` | `Input for the water temperature from the return flow of the air heater.` | — |
| `rSupplyAirTemp` | `Input for the supply air temperature.` | — |
| `rReturnAirTemp` | `Input for the room temperature or the exhaust air temperature.` | — |
| `bDampersFeedbOpen` | `Input for the feedback from the dampers.` | — |
| `bEmergencyStop` | `A collection of all VAC system error messages that lead to the system switching off can` | — |
| `bReset` | `Input for acknowledging the fault.` | — |
| `eConvection` | `E_HVACConvectionMode` | — |
| `tDelaySupplyVent` | `TIME` | — |
| `tDelayExhaustVent` | `TIME` | — |
| `tDelayTempCtrl` | `TIME` | — |
| `rDifferenceTemp` | `REAL` | — |
| `tDelayLimitCtrl` | `TIME` | — |
| `rPreRinseOutsideTemp` | `REAL` | — |
| `rPreRinseLimitTemp` | `REAL` | — |
| `tPreRinseMaxTime` | `TIME` | — |
| `iCurrentSequence` | `INT` | — |
| `eConvection` | `ENUM via which summer night cooling can take place as forced ventilation with fans or as` | — |
| `tDelaySupplyVent` | `Time delay that delays the start-up of the supply air fans. The variable is saved` | — |
| `tDelayExhaustVent` | `Time delay that delays the start-up of the exhaust air fans. The variable is saved` | — |
| `tDelayTempCtrl` | `Time delay for the controller. The variable is saved persistently. Preset to 10 s.` | — |
| `rDifferenceTemp` | `The controller is enabled if the difference between the room temperature and the supply` | — |
| `tDelayLimitCtrl` | `Time delay before limit monitoring of the temperature sensors is enabled. See the variable` | — |
| `rPreRinseOutsideTemp` | `Outside temperature below which the VAC system should start up with the` | — |
| `rPreRinseLimitTemp` | `Temperature of the return flow from the heating coil that must be reached in order to` | — |
| `tPreRinseMaxTime` | `Maximum duration of the pre-rinsing procedure. The variable is saved persistently.` | — |
| `iCurrentSequence` | `The starting controller at controller enable is specified here. The value is written by the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eModeSeqCtrl` | `E_HVACSequenceCtrlMode` | — |
| `bPreRinse` | `BOOL` | — |
| `bDampersOpen` | `BOOL` | — |
| `bSupplyVentilation` | `BOOL` | — |
| `bExhaustVentilation` | `BOOL` | — |
| `bStartPIDControllers` | `BOOL` | — |
| `bEnableLimitCtrl` | `BOOL` | — |
| `bErrorGeneral` | `BOOL` | — |
| `bErrorNoHotWater` | `BOOL` | — |
| `bErrorDampers` | `BOOL` | — |
| `byError` | `BYTE` | — |
| `bInvalidParameter` | `BOOL` | — |
| `eModeSeqCtrl` | `Enum that notifies the operation request (see E_HVACSequenceCtrlMode [} 492]).` | — |
| `bPreRinse` | `TRUE if the pre-rinsing procedure is active.` | — |
| `bDampersOpen` | `If TRUE, then the outside and exhaust air dampers are open.` | — |
| `bSupplyVentilation` | `Switches on the supply air fan if TRUE.` | — |
| `bExhaustVentilation` | `Switches on the exhaust air fan if TRUE.` | — |
| `bStartPIDControllers` | `Enables the controllers.` | — |
| `bEnableLimitCtrl` | `After the timer tDelayLimitCtrl has elapsed, the output bEnableLimitCtrl is enabled and,` | — |
| `bErrorGeneral` | `There is a general error.` | — |
| `bErrorNoHotWater` | `Is set to TRUE if the rinsing procedure has ended and the set temperature in the return` | — |
| `bErrorDampers` | `There is an error in the dampers.` | — |
| `byError` | `Output of the error as a byte.` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |

**Description:** The air conditioning system is started up stepwise with the start program FB_HVACStartAirConditioning. The shut-off dampers, the fans, the controller and the limit value monitoring of the analog inputs are thereby enabled one after the other.

##### `FB_HVACSummerNightCooling` — §5.1.7.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rOutsideTemp` | `REAL` | — |
| `rRoomTemp` | `REAL` | — |
| `rSetpointRoomTemp` | `REAL` | — |
| `dtSystemTime` | `DT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled by the PLC program with the input variable bEnable.` | — |
| `rOutsideTemp` | `Input for the outside temperature.` | — |
| `rRoomTemp` | `Input for the room temperature.` | — |
| `rSetpointRoomTemp` | `Setpoint of the room temperature` | — |
| `dtSystemTime` | `This variable transfers the computer system time to the function block.` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rMinimumOutsideTemp` | `REAL` | — |
| `rDifferOutsideRoomTempOn` | `REAL` | — |
| `rDifferOutsideRoomTempOff` | `REAL` | — |
| `rDifferRoomSetpointTemp` | `REAL` | — |
| `todStartTime` | `TOD` | — |
| `tMaxRuntime` | `TIME` | — |
| `rMinimumOutsideTemp` | `If the outside temperature rOutsideTemp falls below the limit value` | — |
| `rDifferOutsideRoomTempOn` | `The difference rRoomTemp - rOutsideTemp must be greater than the value` | — |
| `rDifferOutsideRoomTempOff` | `The difference rRoomTemp - rOutsideTemp must be less than the absolute` | — |
| `rDifferRoomSetpointTemp` | `The room temperature must exceed the room temperature setpoint by this` | — |
| `todStartTime` | `Start time for the timeframe within which the summer night cooling can be activated (0..24).` | — |
| `tMaxRuntime` | `Maximum runtime of the summer night cooling function within the timeframe between` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOn` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOn` | `If bOn = TRUE summer night cooling is active.` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |

**Description:** With this function block, rooms that were heated up on the day before can be cooled down during the night using cool outside air. The summer night cooling function serves to improve the quality of the air and to save electrical energy. Electrical energy for cooling is saved during the first hours of the next summer day.

##### `FB_HVACSummerNightCoolingEx` — §5.1.7.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `rOutsideTemp` | `REAL` | — |
| `rRoomTemp` | `REAL` | — |
| `rSetpointRoomTemp` | `REAL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled by the PLC program with the input variable bEnable. Once the time` | — |
| `rOutsideTemp` | `Input for the outside temperature.` | — |
| `rRoomTemp` | `Input for the room temperature` | — |
| `rSetpointRoomTemp` | `Setpoint of the room temperature` | — |
| `bReset` | `Acknowledge input in the event of a fault.` | — |
| `rMinimumOutsideTemp` | `REAL` | — |
| `rDifferOutsideTempRoomTempOn` | `REAL` | — |
| `rDifferOutsideTempRoomTempOff` | `REAL` | — |
| `rDifferRoomTempSetpointRoomTempOn` | `REAL` | — |
| `udiSecMaxRuntime` | `UDINT` | — |
| `rMinimumOutsideTemp` | `If the outside temperature rOutsideTemp falls below the limit value` | — |
| `rDifferOutsideTempRoomTempOn` | `The difference rRoomTemp - rOutsideTemp must be greater than the` | — |
| `rDifferOutsideTempRoomTempOff` | `The difference rRoomTemp - rOutsideTemp must be less than the` | — |
| `rDifferRoomTempSetpointRoomTempOn` | `The room temperature must exceed the room temperature` | — |
| `udiSecMaxRuntime` | `Maximum runtime of the summer night cooling function. Summer night cooling can be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOn` | `BOOL` | — |
| `bStateDifferOutsideTempMinOutsideTemp` | `BOOL` | — |
| `rStateDifferOutsideTempMinOutsideTemp` | `REAL` | — |
| `bStateDifferRoomTempOutsideTemp` | `BOOL` | — |
| `rStateDifferRoomTempOutsideTemp` | `REAL` | — |
| `bStateDifferRoomTempSetpointRoomTemp` | `BOOL` | — |
| `rStateDifferRoomTempSetpointRoomTemp` | `REAL` | — |
| `udiSecRT_MaxRuntime` | `UDINT` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOn` | `If bOn = TRUE summer night cooling is active.` | — |
| `bStateDifferOutsideTempMinOutsideTemp` | `Status display. bStateDifferOutsideTempMinOutsideTemp is` | — |
| `rStateDifferOutsideTempMinOutsideTemp` | `Value of the difference rOutsideTemp -` | — |
| `bStateDifferRoomTempOutsideTemp` | `Status display. bStateDifferRoomTempOutsideTemp is TRUE if` | — |
| `rStateDifferRoomTempOutsideTemp` | `Value of the difference rRoomTemp - rOutsideTemp` | — |
| `bStateDifferRoomTempSetpointRoomTemp` | `Status display. bStateDifferRoomTempSetpointRoomTemp` | — |
| `rStateDifferRoomTempSetpointRoomTemp` | `Value of the difference rRoomTemp - rSetpointRoomTemp` | — |
| `udiSecRT_MaxRuntime` | `Remaining summer night cooling time. If udiSecRT_MaxRuntime = 0 the summer` | — |
| `bInvalidParameter` | `Indicates that there is an incorrect input parameter. bInvalidParameter must be` | — |

**Description:** With this function block, rooms that were heated up on the day before can be cooled down during the night using cool outside air. The summer night cooling function serves to improve the quality of the air and to save electrical energy. Electrical energy for cooling is saved during the first hours of the next summer day.

##### `FB_HVACTimeCon` — §5.1.7.32

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `udiSec` | `UDINT` | — |
| `udiMin` | `UDINT` | — |
| `udiHour` | `UDINT` | — |
| `tTime` | `TIME` | — |
| `udiSec` | `Variable that displays the seconds (0..59).` | — |
| `udiMin` | `Variable that displays the minutes (0..59).` | — |
| `udiHour` | `Variable that displays the hours (0..1191).` | — |
| `tTime` | `Variable of the type Time.` | — |

**Description:** This function block converts a TIME variable tTime into the three UDINT variables udiSec, udiMin and udiHour, which indicate the seconds, minutes and hours. This conversion is reversible, so that a TIME variable can be formed from seconds, minutes and hours.

##### `FB_HVACTimeConSec` — §5.1.7.33

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `udiSec` | `UDINT` | — |
| `tTime` | `TIME` | — |
| `udiSec` | `Variable that displays the seconds (0..4294967)` | — |
| `tTime` | `Variable of the type Time.` | — |

**Description:** This function block converts a TIME variable tTime into a UDINT variable udiSec, which indicates the seconds. This conversion is reversible, so that a TIME variable can be formed from seconds.

##### `FB_HVACTimeConSecMs` — §5.1.7.34

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `udiMs` | `UDINT` | — |
| `udiSec` | `UDINT` | — |
| `tTime` | `TIME` | — |
| `udiMs` | `Variable that displays the milliseconds (0..999).` | — |
| `udiSec` | `Variable that displays the seconds (0..4294966).` | — |
| `tTime` | `Variable of the type Time.` | — |

**Description:** This function block converts a TIME variable tTime into two UDINT variables, udiSec and udiMs, which indicate the seconds and milliseconds respectively. The conversion is also reversible, so that a TIME variable can be formed from the seconds and milliseconds.

##### `FB_HVACWork` — §5.1.7.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bResetHours` | `BOOL` | — |
| `bResetCount` | `BOOL` | — |
| `dtSystemtime` | `DT` | — |
| `bStartWarnLevel` | `BOOL` | — |
| `bStartAlarmLevel` | `BOOL` | — |
| `bSysTakt1Hz` | `BOOL` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bOn` | `Operation signal; the operating time is totaled up as long as the signal is present. The rising edge is` | — |
| `bResetHours` | `A rising edge sets the operating hour counter to 0.` | — |
| `bResetCount` | `A rising edge sets the switch-on cycle counter to 0.` | — |
| `dtSystemtime` | `System time with seconds display.` | — |
| `bStartWarnLevel` | `A rising edge at the input resets the output bQWarnLevel and restarts the time` | — |
| `bStartAlarmLevel` | `A rising edge at the input resets the output bQAlarmLevel and restarts the time` | — |
| `bSysTakt1Hz` | `1 Hz clock signal as a replacement for dtSystemTime; if dtSystemTime is not available, or` | — |
| `bReset` | `Acknowledge input in the event of a fault. Resets the flag bInvalidParameter.` | — |
| `rHours` | `REAL` | — |
| `uiCount` | `UINT` | — |
| `uiWarnLevel` | `UINT` | — |
| `uiAlarmLevel` | `UINT` | — |
| `bEnableWarnLevel` | `BOOL` | — |
| `bEnableAlarmLevel` | `BOOL` | — |
| `rHours` | `Operating hours with a resolution of 1/100 hours (internally with 1 s). The variable is saved` | — |
| `uiCount` | `Switch-on cycle counter. The variable is saved persistently.` | — |
| `uiWarnLevel` | `Number of operating hours after which a warning is output (0..50000). Upon a rising edge at` | — |
| `uiAlarmLevel` | `Number of operating hours after which an alarm is output (0..50000). Upon a rising edge at` | — |
| `bEnableWarnLevel` | `Enables the output bQWarnLevel. The variable is saved persistently. Preset to FALSE.` | — |
| `bEnableAlarmLevel` | `Enables the output bQAlarmLevel. The variable is saved persistently. Preset to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQAlarmLevel` | `BOOL` | — |
| `bQWarnLevel` | `BOOL` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bQAlarmLevel` | `Operating hour counter has reached the alarm level.` | — |
| `bQWarnLevel` | `Operating hour counter has reached the warning level.` | — |
| `bInvalidParameter` | `An error occurred during the plausibility check. It is deleted again by bReset.` | — |

**Description:** This function block serves the recording of the operating hours and the switch-on cycles. Furthermore it is possible to report specified operating times being reached.

#### Programming › POUs › HVAC Time schedule

##### `FB_HVACScheduler1ch` — §5.1.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `dtSystemTime` | `DT` | — |
| `uiOffsetBefore` | `UINT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled by the variable bEnable. If bEnable = FALSE there is no switching at` | — |
| `dtSystemTime` | `The variable dtSystemTime transfers the computer system time to the timer program.` | — |
| `uiOffsetBefore` | `The switch-on time is brought forward by this number of minutes. This function is useful for` | — |
| `bReset` | `Acknowledge input in the event of a fault. Resets the flag bInvalidParameter` | — |
| `stTimeChannel` | `ST_HVACTimeChannel` | — |
| `stTimeChannel` | `With the variable stTimeChannel it is possible to generate one switch-on time per day of` | — |
| `stTimeChannel` | `ST_HVACTimeChannel` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOutput` | `BOOL` | — |
| `bEdgeOn` | `BOOL` | — |
| `bEdgeOff` | `BOOL` | — |
| `tNextOn` | `TIME` | — |
| `tNextOff` | `TIME` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOutput` | `Is TRUE if one of the channels has switched on.` | — |
| `bEdgeOn` | `Is TRUE for one PLC cycle after bOutput switches on.` | — |
| `bEdgeOff` | `Is TRUE for one PLC cycle after bOutput switches off.` | — |
| `tNextOn` | `Time until the next switch-on of the program. Maximum value 10080 minutes (1 week).` | — |
| `tNextOff` | `Time until the next switch-off of the program. Maximum forecast until midnight.` | — |
| `bInvalidParameter` | `An error occurred during the plausibility check. It is deleted again by bReset.` | — |

**Description:** This function block is a weekly scheduler with one channel. Any desired combination of days of the week can be assigned to the channel. The channel can be defined for one-shot or recurring events. Besides the switching signal, the switch-on and switch-off edges plus the next switch-on or switch-off time as a countdown are available as outputs. The data of the channel are transferred as a reference. The function block is therefore able to check the data for changes and to save it as persistent data. After booting, the data are thus available to the BMS again. The switch-on and switch-off points can be specified to the exact second via the variables tiOn_ss and tiOff_ss of the respective time channel.

##### `FB_HVACScheduler7ch` — §5.1.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `dtSystemTime` | `DT` | — |
| `uiOffsetBefore` | `UINT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled by the variable bEnable. If bEnable = FALSE there is no switching at` | — |
| `dtSystemTime` | `The variable dtSystemTime transfers the computer system time to the timer program.` | — |
| `uiOffsetBefore` | `The switch-on time is brought forward by this number of minutes. This function is useful for` | — |
| `bReset` | `Acknowledge input in the event of a fault. Resets the flag bInvalidParameter.` | — |
| `arrTimeChannel` | `ARRAY[1..7] OF ST_HVACTimeChannel` | — |
| `arrTimeChannel` | `The variable arrTimeChannel is an array with 7 declarations of the structure variable` | — |
| `arrTimeChannel` | `ARRAY[1..7] OF ST_HVACTimeChannel` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOutput` | `BOOL` | — |
| `bEdgeOn` | `BOOL` | — |
| `bEdgeOff` | `BOOL` | — |
| `tNextOn` | `TIME` | — |
| `tNextOff` | `TIME` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOutput` | `Is TRUE if one of the channels has switched on.` | — |
| `bEdgeOn` | `Is TRUE for one PLC cycle after bOutput switches on.` | — |
| `bEdgeOff` | `Is TRUE for one PLC cycle after bOutput switches off.` | — |
| `tNextOn` | `Time until the next switch-on of the program. Maximum value 10080 minutes (1 week).` | — |
| `tNextOff` | `Time until the next switch-off of the program. Maximum forecast until midnight (1440 min).` | — |
| `bInvalidParameter` | `An error occurred during the plausibility check. It is deleted again by bReset.` | — |

**Description:** This function block is a weekly scheduler with 7 channels. Any desired combination of days of the week can be assigned to any channel. The channels can be defined for non-recurring or recurring events. Besides the switching signal, the switch-on and switch-off edges plus the next switch-on or switch-off time as a countdown are available as outputs. The data of the channels is transferred as a reference. The function block is therefore able to check the data for changes and to save it as persistent data. After booting, the data are thus available to the BMS again. The switch-on and switch-off points can be specified to the exact second via the variables tiOn_ss and tiOff_ss of the respective time channel.

##### `FB_HVACScheduler7TCHandling` — §5.1.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bGoChannelUp` | `BOOL` | — |
| `bGoChannelDown` | `BOOL` | — |
| `bGoChannelUp` | `With a positive edge the focus is placed on the next higher index in the data array` | — |
| `bGoChannelDown` | `With a positive edge the focus is placed on the next lower index in the data array` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arr7TimeChannel` | `ARRAY[1..7] OF ST_HVACTimeChannel` | — |
| `stCurrChannel` | `ST_HVACTimeChannel` | — |
| `iCurrChannelNumber` | `INT` | — |
| `iNumOfChannelsToUse` | `INT` | — |
| `arr7TimeChannel` | `Array of the data sets of the timer FB_HVACScheduler7ch (see ST_HVACTimeChannel` | — |
| `stCurrChannel` | `Contains the contents of the data set from the array arr7TimeChannel, which is selected via` | — |
| `iCurrChannelNumber` | `Selection index for the array arr7TimeChannel.` | — |
| `iNumOfChannelsToUse` | `Maximum (7) permissible index for iCurrChannelNumber in case not all channels` | — |

**Description:** This function block can be used to select and modify an individual line from the data array of a weekly timer. In this way it is easily possible to edit the data sets with a visualization that only offers a low graphic resolution, without having to show all the data of the array. The data set from arr7TimeChannel with index iCurrChannelNumber is copied to data structure stCurrChannel. Modifications of the data in structure stCurrChannel take effect immediately and are transferred to array arr7TimeChannel .

##### `FB_HVACScheduler28ch` — §5.1.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `dtSystemTime` | `DT` | — |
| `uiOffsetBefore` | `UINT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled by the variable bEnable. If bEnable = FALSE there is no switching at` | — |
| `dtSystemTime` | `The variable dtSystemTime transfers the computer system time to the timer program.` | — |
| `uiOffsetBefore` | `The switch-on time is brought forward by this number of minutes. This function is useful for` | — |
| `bReset` | `Acknowledge input in the event of a fault. Resets the flag bInvalidParameter.` | — |
| `arrTimeChannel` | `ARRAY[1..28] OF ST_HVACTimeChannel` | — |
| `arrTimeChannel` | `The variable arrTimeChannel is an array with 28 declarations of the structure variable` | — |
| `arrTimeChannel` | `ARRAY[1..28] OF ST_HVACTimeChannel` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOutput` | `BOOL` | — |
| `bEdgeOn` | `BOOL` | — |
| `bEdgeOff` | `BOOL` | — |
| `tNextOn` | `TIME` | — |
| `tNextOff` | `TIME` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOutput` | `Is TRUE if one of the channels has switched on.` | — |
| `bEdgeOn` | `Is TRUE for one PLC cycle after bOutput switches on.` | — |
| `bEdgeOff` | `Is TRUE for one PLC cycle after bOutput switches off.` | — |
| `tNextOn` | `Time until the next switch-on of the program. Maximum value 10080 minutes (1 week).` | — |
| `tNextOff` | `Time until the next switch-off of the program. Maximum forecast until midnight. Maximum value` | — |
| `bInvalidParameter` | `An error occurred during the plausibility check. It is deleted again by bReset.` | — |

**Description:** This function block is a weekly scheduler with 28 channels. Any desired combination of days of the week can be assigned to any channel. The channels can be defined for non-recurring or recurring events. Besides the switching signal, the switch-on and switch-off edges plus the next switch-on or switch-off time as a countdown are available as outputs. The data of the channels is transferred as a reference. The function block is therefore able to check the data for changes and to save it as persistent data. After booting, the data are thus available to the BMS again. The switch-on and switch-off points can be specified to the exact second via the variables tiOn_ss and tiOff_ss of the respective time channel.

##### `FB_HVACScheduler28TCHandling` — §5.1.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bGoChannelUp` | `BOOL` | — |
| `bGoChannelDown` | `BOOL` | — |
| `bGoChannelUp` | `With a positive edge the focus is placed on the next higher index in the data array` | — |
| `bGoChannelDown` | `With a positive edge the focus is placed on the next lower index in the data array` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arr28TimeChannel` | `ARRAY[1..28]OF ST_HVACTimeChannel` | — |
| `stCurrChannel` | `ST_HVACTimeChannel` | — |
| `iCurrChannelNumber` | `INT` | — |
| `iNumOfChannelsToUse` | `INT` | — |
| `arr28TimeChannel` | `Array of the data sets (28) of the timer FB_HVACScheduler28CH.` | — |
| `stCurrChannel` | `Contains the contents of the data set from the array arr28TimeChannel, which is selected` | — |
| `iCurrChannelNumber` | `Selection index for the array arr28TimeChannel.` | — |
| `iNumOfChannelsToUse` | `Maximum permissible index for iCurrChannelNumber, if not all channels are used.` | — |

**Description:** This function block can be used to select and modify an individual line from the data array of a weekly timer. In this way it is easily possible to edit the data sets with a visualization that only offers a low graphic resolution, without having to show all the data of the array. The data set from arr7TimeChannel with index iCurrChannelNumber is copied to data structure stCurrChannel. Modifications of the data in structure stCurrChannel take effect immediately and are transferred to array arr28TimeChannel .

##### `FB_HVACSchedulerPublicHolidays` — §5.1.8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `dtSystemTime` | `DT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled by the variable bEnable. If bEnable = FALSE there is no switching at` | — |
| `dtSystemTime` | `The variable dtSystemTime transfers the computer system time to the timer program.` | — |
| `bReset` | `Acknowledge input in the event of a fault. Resets the flag bInvalidParameter.` | — |
| `arrHoliday` | `ARRAY[1..20] OF ST_HVACHoliday` | — |
| `arrHoliday` | `The variable arrHoliday is an array with 20 declarations of the structure variable` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOutput` | `BOOL` | — |
| `bEdgeOn` | `BOOL` | — |
| `bEdgeOff` | `BOOL` | — |
| `tNextOn` | `TIME` | — |
| `tNextOff` | `TIME` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOutput` | `Is TRUE if one of the timer switch channels has switched on.` | — |
| `bEdgeOn` | `Is TRUE for one PLC cycle after bOutput switches on.` | — |
| `bEdgeOff` | `Is TRUE for one PLC cycle after bOutput switches off.` | — |
| `tNextOn` | `Time until the next switch-on of the timer program (max. 50 days).` | — |
| `tNextOff` | `Time until the next switch-off of the timer program (max. 50 days).` | — |
| `bInvalidParameter` | `An error occurred during the plausibility check. It is deleted again by bReset.` | — |

**Description:** This function block is a yearly scheduler in which, e.g., school holidays or factory shutdowns can be entered. The function block is enabled by the input variable bEnable. The input dtSystemTime is linked to the current system time. The output bOutput is set if the switching conditions are satisfied. Besides the switching signal, the switch-on and switch-off edges plus the next switch-on or switch-off time as a countdown are available

##### `FB_HVACSchedulerSpecialPeriods` — §5.1.8.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDataSecurityType` | `E_HVACDataSecurityType` | — |
| `bSetDefault` | `BOOL` | — |
| `bEnable` | `BOOL` | — |
| `dtSystemTime` | `DT` | — |
| `bReset` | `BOOL` | — |
| `eDataSecurityType` | `If eDataSecurityType [} 490]:= eHVACDataSecurityType_Persistent, the persistent` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `If the variable is TRUE, the default values of the VAR_IN_OUT variables are adopted.` | — |
| `bEnable` | `The function block is enabled by the variable bEnable. If bEnable = FALSE there is no switching at` | — |
| `dtSystemTime` | `The variable dtSystemTime transfers the computer system time to the timer program.` | — |
| `bReset` | `Acknowledge input in the event of a fault. Resets the flag bInvalidParameter.` | — |
| `arrPeriod` | `ARRAY[1..20] OF ST_HVACPeriod` | — |
| `arrPeriod` | `The variable arrPeriod is an array with 20 declarations of the structure variable ST_HVACPeriod` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOutput` | `BOOL` | — |
| `bEdgeOn` | `BOOL` | — |
| `bEdgeOff` | `BOOL` | — |
| `tNextOn` | `TIME` | — |
| `tNextOff` | `TIME` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bOutput` | `Is TRUE if one of the channels has switched on.` | — |
| `bEdgeOn` | `Is TRUE for one PLC cycle after bOutput switches on.` | — |
| `bEdgeOff` | `Is TRUE for one PLC cycle after bOutput switches off.` | — |
| `tNextOn` | `Time until the next switch-on of the timer program (max. 50 days).` | — |
| `tNextOff` | `Time until the next switch-off of the timer program (max. 50 days).` | — |
| `bInvalidParameter` | `An error occurred during the plausibility check. It is deleted again by bReset.` | — |

**Description:** This function block is a yearly scheduler in which, e.g., school holidays or factory shutdowns can be entered. The function block is enabled by the input variable bEnable. The input dtSystemTime is linked to the current system time. The output bOutput is set if the switching conditions are satisfied. Besides the switching signal,

#### Programming › POUs › HVAC System

##### `FB_HVACGetSystemTime` — §5.1.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |
| `sNETID` | `T_AmsNetId` | — |
| `tGetSystemTime` | `TIME` | — |
| `bGetSystemTime` | `BOOL` | — |
| `bEnable` | `Enables the function block. If bEnable = TRUE, then the RTC clock is initialized with the NT` | — |
| `sNETID` | `It is possible here to provide the AmsNetId of the TwinCAT computer whose NT system time is to` | — |
| `tGetSystemTime` | `Time specification with which the RTC clock is regularly synchronized with the NT system` | — |
| `bGetSystemTime` | `The RTC clock is synchronized with the NT system time with a rising edge at this input.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReadyGetSystemTime` | `BOOL` | — |
| `bReadyRTC` | `BOOL` | — |
| `stSystemTime` | `TIMESTRUCT` | — |
| `dtSystemTime` | `DT` | — |
| `uiYear` | `UINT` | — |
| `uiMonth` | `UINT` | — |
| `uiDay` | `UINT` | — |
| `uiDayOfWeek` | `UINT` | — |
| `uiHour` | `UINT` | — |
| `uiMinute` | `UINT` | — |
| `uiSecond` | `UINT` | — |
| `uiMilliseconds` | `UINT` | — |
| `bErrorGetSystemTime` | `BOOL` | — |
| `udiErrorIdGetSystemTime` | `UDINT` | — |
| `bReadyGetSystemTime` | `The function block was successfully initialized or synchronized with the NT system` | — |
| `bReadyRTC` | `This output is set if the function block has been initialized at least once. If this output is set,` | — |
| `stSystemTime` | `Structure with the current RTC time.` | — |
| `dtSystemTime` | `Date and time of day of the RTC time.` | — |
| `uiYear` | `The year: 1970 ~ 2106` | — |
| `uiMonth` | `The month: 1 ~ 12 (January = 1, February = 2 etc.)` | — |
| `uiDay` | `The day of the month: 1 ~ 31` | — |
| `uiDayOfWeek` | `The day of the week: 0 ~ 6 (Sunday = 0, Monday = 1 etc.)` | — |
| `uiHour` | `Hour: 0 ~ 23` | — |
| `uiMinute` | `Minute: 0 ~ 59` | — |
| `uiSecond` | `Second: 0 ~ 59` | — |
| `uiMilliseconds` | `Millisecond: 0 ~ 999` | — |
| `bErrorGetSystemTime` | `This output is set in the event that an ADS error occurs when transmitting the` | — |
| `udiErrorIdGetSystemTime` | `Returns the ADS error number (see ADS Return Codes) if a` | — |

**Description:** With this function block an internal clock (Real Time Clock RTC) can be implemented in the TwinCAT PLC. When the function block is enabled via bEnable, the RTC clock is initialized with the current NT system time. One system cycle of the CPU is used to calculate the current RTC time. The function block must be called once per PLC cycle in order for the current time to be calculated. Internally an instance of the function blocks NT_GetTime and RTC_EX2 is called in the function block from the Utilities library.

##### `FB_HVACNOVRAMDataHandling` — §5.1.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNETID` | `T_AmsNetId` | — |
| `TMOUT` | `TIME` | — |
| `udiDevID` | `UDINT` | — |
| `usiBlockSize` | `USINT` | — |
| `sNETID` | `AmsNetId of the TwinCAT computer on which the function should be executed. If it is to be run on` | — |
| `TMOUT` | `States the length of the timeout that may not be exceeded by execution of the ADS command.` | — |
| `udiDevID` | `The Device ID specifies the NOVRAM of the CX90xx or CX10xx to be accessed with the function` | — |
| `usiBlockSize` | `The block size to be accessed per read/write cycle is specified in percent; e.g. 20, i.e. 5 read/` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `udiStatus` | `UDINT` | — |
| `iNovramReadCount` | `INT` | — |
| `iNovramWriteCount` | `INT` | — |
| `bDone` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorID` | `UDINT` | — |
| `udiStatus` | `= 0 , no state` | — |
| `iNovramReadCount` | `Counter that is incremented by 1 when NOVRAM is read from.` | — |
| `iNovramWriteCount` | `Counter that is incremented by 1 when NOVRAM is written to.` | — |
| `bDone` | `Will be set to TRUE if the function block is executed.` | — |
| `bBusy` | `When the function block is activated the output is set, and it remains active until execution of the` | — |
| `bError` | `This output is switched to TRUE as soon as an error occurs during the execution of a command. The` | — |
| `udiErrorID` | `Contains the command-specific error code (see ADS Return Codes).` | — |

**Description:** Programming

##### `FB_HVACPersistentDataHandling` — §5.1.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNETID` | `T_AmsNetId` | — |
| `TMOUT` | `TIME` | — |
| `ePersistentMode` | `E_PersistentMode` | — |
| `sNETID` | `AmsNetId of the TwinCAT computer on which the function should be executed. If it is to be run on` | — |
| `TMOUT` | `States the length of the timeout that may not be exceeded by execution of the ADS command.` | — |
| `ePersistentMode` | `Mode in which the persistent data should be written. Also see E_PersistentMode (see` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `udiStatus` | `UDINT` | — |
| `iPersistCount` | `INT` | — |
| `bDone` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorID` | `UDINT` | — |
| `udiStatus` | `0 = No status` | — |
| `iPersistCount` | `Counter that is incremented by 1 after successful writing.` | — |
| `bDone` | `Will be set to TRUE if the function block is executed.` | — |
| `bBusy` | `When the function block is activated the output is set, and it remains active until execution of the` | — |
| `bError` | `This output is set to TRUE if an error occurs during the execution of a command.` | — |
| `udiErrorID` | `Contains the command-specific error code (see ADS Return Codes).` | — |

**Description:** With this function block, all PLC variables that are declared as persistent (VAR PERSISTENT) are written fail-safe to a file. The PLC variables declared as persistent within the function blocks from the Tc2_HVAC are written to this file in case of a change. This file is saved in the ..\TwinCAT\3.1\Boot\PLC directory and is named as follows: Port_c.bootdata (x = number of the runtime system). So that the new status is not saved immediately after a change of the persistent variables, a timeframe of 5 sec is built into the standard setting. This timeframe can be changed by the user. He must assign the new time to the globally declared constant parameter g_tHVACWriteBackupDataTime: TIME := t#5s.

##### `FB_HVACPersistentDataFileCopy` — §5.1.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `srcNETID` | `T_AmsNetId` | — |
| `srcFileName` | `T_MaxString` | — |
| `destNETID` | `T_AmsNetId` | — |
| `destFileName` | `T_MaxString` | — |
| `bExecute` | `BOOL` | — |
| `tAdsTimeOut` | `TIME` | — |
| `srcNETID` | `AmsNetId of the TwinCAT computer on which the function should be executed. If it is to be run` | — |
| `srcFileName` | `Contains the path and file name for the file to be opened. Note: The path can only point to the` | — |
| `destNETID` | `AmsNetId of the TwinCAT computer on which the file should be copied.` | — |
| `destFileName` | `Contains the path and the file name of the destination file. Note: The path can only point to` | — |
| `bExecute` | `The steps listed above are executed upon a rising edge at input bExecute.` | — |
| `tAdsTimeOut` | `States the length of the timeout that may not be exceeded by execution of the ADS` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bDone` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorID` | `UDINT` | — |
| `bDone` | `Will be switched to TRUE if the function block is executed.` | — |
| `bBusy` | `When the function block is activated the output is set, and it remains active until execution of the` | — |
| `bError` | `This output is switched to TRUE as soon as an error occurs during the execution of a command.` | — |
| `udiErrorID` | `Contains the command-specific error code (see ADS Return Codes).` | — |

**Description:** This function block can be used, for instance, to copy binary files on the local TwinCAT PC or from a remote TwinCAT PC to the local TwinCAT PC. Network drives cannot be accessed. The following steps are executed upon a rising edge at the input bExecute:

##### `FB_HVACSetLocalTime` — §5.1.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | — |
| `bSetLocalTime` | `BOOL` | — |
| `uiYear` | `UINT` | — |
| `uiMonth` | `UINT` | — |
| `uiDay` | `UINT` | — |
| `uiHour` | `UINT` | — |
| `uiMinute` | `UINT` | — |
| `uiSecond` | `UINT` | — |
| `bEnableStruct` | `BOOL` | — |
| `stSystemtime` | `TIMESTRUCT` | — |
| `tTimeout` | `TIME` | — |
| `sNETID` | `This parameter can be used to specify the AmsNetID of the TwinCAT computer, whose local NT` | — |
| `bSetLocalTime` | `Activation of the function block with a rising edge.` | — |
| `uiYear` | `The year: 1970 ~ 2106; the variable is active if bEnableStruct = FALSE. If specified incorrectly,` | — |
| `uiMonth` | `The month: 1 ~ 12 (January = 1, February = 2 and so on); the variable is active if bEnableStruct =` | — |
| `uiDay` | `The day of the month: 1 ~ 31; February with 28 or 29 days and the months with 30 or 31 days are` | — |
| `uiHour` | `Hour: 0 ~ 23; the variable is active if bEnableStruct = FALSE. If specified incorrectly,` | — |
| `uiMinute` | `Minute: 0 ~ 59; the variable is active if bEnableStruct = FALSE. If specified incorrectly,` | — |
| `uiSecond` | `Second: 0 ~ 59; the variable is active if bEnableStruct = FALSE. If specified incorrectly,` | — |
| `bEnableStruct` | `If bEnableStruct = TRUE, the new local NT system time is set via the input variable` | — |
| `stSystemtime` | `Structure with the new local NT system time. The structure is active if bEnableStruct =` | — |
| `tTimeout` | `Indicates the timeout time [s], which must not be exceeded during execution.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `udiErrorId` | `UDINT` | — |
| `bInvalidParameter` | `BOOL` | — |
| `bBusy` | `If the function block is activated via a rising edge at bSetLocalTime, this output is set and remains` | — |
| `bError` | `This output is set if an error occurs during the transfer of the NT system time. bError is reset by the` | — |
| `udiErrorId` | `Returns the ADS error code if bError is set (see ADS Return Codes)` | — |
| `bInvalidParameter` | `bInvalidParameter is TRUE if the ranges of the variables for the time of day and the` | — |

**Description:** The local NT system time and the date of a TwinCAT system can be set with the function block FB_HVACSetLocalTime (the local NT system time is shown in the taskbar). The system time can be specified either via the individual variables uiYear, uiMonth, uiDay, uiHour, uiMinute and uiSecond or via the structure stSystemtime, see bEnableStruct. Internally, an instance of the function block NT_SetLocalTime from the TcUtilities library is called in the function block.

##### `FB_HVACSystemTaskInfo` — §5.1.9.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReady` | `BOOL` | — |
| `tTaskCycleTime` | `TIME` | — |
| `tCycleTime` | `TIME` | — |
| `usiSystemTask` | `USINT` | — |
| `bReady` | `The variable is TRUE if the system information has been read out.` | — |
| `tTaskCycleTime` | `Set task cycle time.` | — |
| `tCycleTime` | `Cycle time required for the last cycle.` | — |
| `usiSystemTask` | `Task index of the task.` | — |

**Description:** The function block determines system variables of the task with a resolution of 1 ms, in which it is currently called. If the current cycle time is less than 1 ms, the set task cycle time tTaskCycleTime is output on the output variable tCycleTime.

#### Programming › POUs › BackupVar

##### `FB_HVACNOVRAM_XX` — §5.1.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `BOOL` | — |
| `bVar_Default` | `BOOL` | — |
| `bSetDefault` | `If the variable is TRUE, the value of the input variable bVar_Default is adopted.` | — |
| `bVar_Default` | `Default value defined by the user. The variable is of the data type BOOL.` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bVar` | `BOOL` | — |
| `bVar` | `Variable that is programmed fail-safe by the user.` | — |

**Description:** The variables of various types (BOOL, BYTE, DINT, DWORD, INT, LREAL, REAL, SINT, TIME, UDINT, UINT, USINT, WORD) are saved fail-safe in the NOVRAM with the aid of the function blocks FB_HVACNOVRAM_XX. By way of example the function block FB_HVACNOVRAM_BOOL is described here.

##### `FB_HVACPersistent_XX` — §5.1.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDefault` | `BOOL` | — |
| `byVar_Default` | `BYTE` | — |
| `bSetDefault` | `If the variable is TRUE, the value of the input variable byVar_Default is adopted.` | — |
| `byVar_Default` | `Default value defined by the user. The variable is of the data type BYTE.` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `byVar` | `BYTE` | — |
| `byVar` | `Variable that is programmed fail-safe by the user.` | — |

**Description:** The variables of various types (BOOL, BYTE, DINT, DWORD, INT, LREAL, REAL, SINT, STRING, STRUCT, TIME, UDINT, UINT, USINT, WORD) are saved persistently with the aid of the function blocks FB_HVACPersistent_XX. By way of example the function block FB_HVACPersistent_BYTE is described here.

#### Programming › POUs › Functions

##### `F_RoundLREAL` — §5.1.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `lrIN` | `LREAL` | — |

**Description:** The function F_RoundLREAL rounds the input variable IrIN of data type LREAL to 1 decimal place. This function can also be used for REAL data types.

##### `F_RoundLREAL_EX` — §5.1.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Name` | `Type` | — |
| `lrIN` | `LREAL` | — |
| `iPrecision` | `INT` | — |
| `lrIN` | `Floating point number that is to be formatted.` | — |
| `iPrecision` | `Precision. The value determines the number of digits after the decimal point. The range of` | — |

**Description:** The function F_RoundLREAL_EX rounds the input variable lrIN of the data type LREAL to the number of decimal places specified by iPrecision. This function can also be used for REAL data types. An instance of the function LTRUNC from the library TcMath.lib is used internally in the function.
