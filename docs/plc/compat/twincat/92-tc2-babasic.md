# Tc2_BABasic

- **Manual:** `TF8010_TC3_BABasic_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 35
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Building automation basic: needs fieldbus transports (serial/Ethernet) per protocol.

---

#### Programming › POUs › Conversion functions

##### `F_Scale` (FUNCTION) → `LREAL` — §4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fRawData` | `LREAL` | — |
| `fRawDataLowerOffLimit` | `LREAL` | — |
| `fRawDataUpperOffLimit` | `LREAL` | — |
| `fScaleDataLowerOffLimit` | `LREAL` | — |
| `fScaleDataUpperOffLimit` | `LREAL` | — |
| `fRawData` | `raw value.` | — |
| `fRawDataLowerOffLimit` | `Lower limit for raw value.` | — |
| `fRawDataUpperOffLimit` | `Upper limit for raw value.` | — |
| `fScaleDataLowerOffLimit` | `Lower limit of scaled measured value.` | — |
| `fScaleDataUpperOffLimit` | `Upper limit of scaled measured value.` | — |

**Description:** A raw analog value is scaled to the specified measuring range and returned as the function value. If the raw value extends beyond the upper or lower measuring range, the corresponding limit value is output. There must be a difference of at least 0.01 between the upper and lower limit values for the raw data. If this is not the case, the lower limit value is output.

#### Programming › POUs › Energy management

##### `FB_MaximumDemandController` — §4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | — |
| `fMeterConstant` | `LREAL` | — |
| `fAgreedPower` | `LREAL` | — |
| `bPeriodPulse` | `BOOL` | — |
| `arrLoadParameter` | `ARRAY[1..8] OF ST_MDCLoadParameters` | — |
| `bStart` | `The function block is activated by a positive edge at this input.` | — |
| `fMeterConstant` | `Meter constant [pulses / kWh].` | — |
| `fAgreedPower` | `This is the agreed power limit which, as far as possible, should not be exceeded in the` | — |
| `bPeriodPulse` | `Synchronization pulse sent by the electricity supply company (ESC). This pulse starts the` | — |
| `arrLoadParameter` | `Parameter structure of the respective consumer (see ST_MDCLoadParameters [} 94]).` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInDataKL1501` | `ST_MDCInDataKL1501` | — |
| `stOutDataKL1501` | `ST_MDCOutDataKL1501` | — |
| `stInDataKL1501` | `Linked to KL1501 (see ST_MDCInDataKL1501 [} 93]).` | — |
| `stOutDataKL1501` | `Linked to KL1501 (see ST_MDCOutDataKL1502 [} 94]).` | — |
| `arrLoadParameters` | `ARRAY[1..8] OF ST_MDCLoadParameters` | — |
| `bPeriodPulse` | `BOOL` | — |
| `stInDataKL1501` | `ST_MDCInDataKL1501` | — |
| `stOutDataKL1501` | `ST_MDCOutDataKL1501` | — |
| `bLoadOut1` | `BOOL` | — |
| `bLoadOut2` | `BOOL` | — |
| `bLoadOut3` | `BOOL` | — |
| `bLoadOut4` | `BOOL` | — |
| `bLoadOut5` | `BOOL` | — |
| `bLoadOut6` | `BOOL` | — |
| `bLoadOut7` | `BOOL` | — |
| `bEmergencySignal` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrLoad` | `ARRAY[1..8] OF BOOL` | — |
| `fAgreedEnergy` | `LREAL` | — |
| `fInstantaneousEnergy` | `LREAL` | — |
| `fActualEnergy` | `LREAL` | — |
| `tRemainingTime` | `TIME` | — |
| `fLastPeriodEnergy` | `LREAL` | — |
| `bEmergencySignal` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `arrLoad` | `This is an array of data type BOOL; consumers that are switched on are TRUE.` | — |
| `fAgreedEnergy` | `Agreed energy consumption [kWh].` | — |
| `fInstantaneousEnergy` | `Current energy consumption [kWh] over an integration period of 15s (internal` | — |
| `fActualEnergy` | `Energy consumed at the "presently" observed point in time of the measuring period.` | — |
| `tRemainingTime` | `Time remaining until the next measurement interval.` | — |
| `fLastPeriodEnergy` | `Rated power from the preceding measuring period [kWh].` | — |
| `bEmergencySignal` | `This output is set as soon as the specified energy is exceeded.` | — |
| `bError` | `This output is switched to TRUE as soon as an error occurs during the execution of a command.` | — |
| `nErrorId` | `Contains the error code [} 92].` | — |

**Description:** Function block for peak load optimization, which ensures that the set power limit is maintained by switching up to eight consumers on or off. The consumers can be switched off according to their power and priority in such a manner that the production process is not disturbed.

#### Programming › POUs › Facade

##### `FB_RoofWindow` — §4.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bClose` | `BOOL` | — |
| `bOpen` | `BOOL` | — |
| `bStop` | `BOOL` | — |
| `bSafetyPosition` | `BOOL` | — |
| `bLimitSwitchClose` | `BOOL` | — |
| `bLimitSwitchOpen` | `BOOL` | — |
| `tTurnOffTime` | `TIME := t#60s` | — |
| `tSwitchOverDeadTime` | `TIME := t#400ms` | — |
| `bClose` | `Set output bWindowClose and reset output bWindowOpen. The output bWindowClose remains` | — |
| `bOpen` | `Set output bWindowOpen and reset output bWindowClose. The output bWindowOpen remains` | — |
| `bStop` | `Reset outputs bWindowClose and bWindowOpen` | — |
| `bSafetyPosition` | `The safety position is approached. The window is closed for the time tTurnOffTime.` | — |
| `bLimitSwitchClose` | `Optional limit switch. If bClose is set and is not set within` | — |
| `bLimitSwitchOpen` | `Optional limit switch. If bOpen is set and is not set within` | — |
| `tTurnOffTime` | `If no input is activated, then the outputs are reset after this period of time. The outputs are` | — |
| `tSwitchOverDeadTime` | `Dwell time at a change of direction. Both outputs are reset during this period.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bWindowOpen` | `BOOL` | — |
| `bWindowClose` | `BOOL` | — |
| `bErrorLimitSwitchClose` | `BOOL` | — |
| `bErrorLimitSwitchOpen` | `BOOL` | — |
| `bWindowOpen` | `The window opens.` | — |
| `bWindowClose` | `The window closes.` | — |
| `bErrorLimitSwitchClose` | `Error relating to the optional limit switch during closing.` | — |
| `bErrorLimitSwitchOpen` | `Error relating to the optional limit switch during opening.` | — |

**Description:** The outputs bWindowOpen or bWindowClose are set through a positive edge at the inputs bOpen or bClose. These remain asserted until the time tTurnOffTime has elapsed, or until the function block receives some other command. Both outputs are immediately reset by a positive edge at the bStop input.

##### `FB_VenetianBlind` — §4.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bUp` | `BOOL` | — |
| `bDown` | `BOOL` | — |
| `bStop` | `BOOL` | — |
| `bSwitchOverUp` | `BOOL` | — |
| `bSwitchOverDown` | `BOOL` | — |
| `tSwitchOverTime` | `TIME := t#500ms` | — |
| `bStepUp` | `BOOL` | — |
| `bStepDown` | `BOOL` | — |
| `tStepTime` | `TIME := t#200ms` | — |
| `bShadowPosition` | `BOOL` | — |
| `tShadowTurnAroundTime` | `TIME := t#0s` | — |
| `tShadowTurnOffTime` | `TIME := t#20s` | — |
| `bSafetyPosition` | `BOOL` | — |
| `tTurnOffTime` | `TIME := t#60s` | — |
| `tSwitchOverDeadTime` | `TIME := t#400ms` | — |
| `bUp` | `Set the bBlindUp output and reset the bBlindDown output. The bBlindUp output remains latched.` | — |
| `bDown` | `Set the bBlindDown output and reset the bBlindUp output. The bBlindDown output remains latched.` | — |
| `bStop` | `Reset the bBlindUp and bBlindDown outputs.` | — |
| `bSwitchOverUp` | `Set the bBlindUp output and reset the bBlindDown output. If the signal remains present for` | — |
| `bSwitchOverDown` | `Set the bBlindDown output and reset the bBlindUp output. If the signal remains present` | — |
| `tSwitchOverTime` | `Gives the time for which the bSwitchUp and bSwitchDown inputs must remain asserted` | — |
| `bStepUp` | `Reset the bBlindDown output and set the bBlindUp output for the time tStepTime.` | — |
| `bStepDown` | `Reset the bBlindUp output and set the bBlindDown output for the time tStepTime.` | — |
| `tStepTime` | `If the blind is controlled through the bStepUp or bStepDown inputs, the outputs remain asserted` | — |
| `bShadowPosition` | `The shading position is approached.` | — |
| `tShadowTurnAroundTime` | `The blind travels in the opposite direction for the period of time specified by` | — |
| `tShadowTurnOffTime` | `The time for which the bBlindDown output is set in order to reach the shading` | — |
| `bSafetyPosition` | `The safety position is approached. To do this, the blind is raised for the period of time` | — |
| `tTurnOffTime` | `If no input is activated, then the outputs are reset after this period of time. The outputs are` | — |
| `tSwitchOverDeadTime` | `Dwell time at a change of direction. Both outputs are reset during this period.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBlindUp` | `BOOL` | — |
| `bBlindDown` | `BOOL` | — |
| `bBlindUp` | `The blind opens.` | — |
| `bBlindDown` | `The blind closes.` | — |

**Description:** There are three different ways in which the blinds may be controlled: • A positive edge at the bUp or bDown inputs set the bBlindUp or bBlindDown outputs respectively. These remain asserted until the time tTurnOffTime has elapsed, or until the function block receives some other command. Both outputs are immediately reset by a positive edge at the bStop input. • Static signals are provided to the bSwitchOverUp or bSwitchOverDown inputs (e.g. by buttons). These set the bBlindUp and bBlindDown outputs. If this signal is asserted for longer than tSwitchOverTime, the outputs are latched. This means that the outputs will continue to be asserted, even if the signals at the inputs are removed again. In most cases, a value of 500 ms is sufficient for the tSwitchOverTime parameter. However, the output only remains asserted for the time tTurnOffTime, or until a new command is given to the function block. • This last variation can be useful if the user wants to alter the setting of the blind step by step. Each positive edge at the bStepUp or bStepDown inputs sets the corresponding output for the time tStepTime. A value of 200 ms has been found effective for tStepTime.

##### `FB_VenetianBlindEx` — §4.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bUp` | `BOOL` | — |
| `bDown` | `BOOL` | — |
| `bStop` | `BOOL` | — |
| `bSwitchOverUp` | `BOOL` | — |
| `bSwitchOverDown` | `BOOL` | — |
| `tSwitchOverTime` | `TIME := t#500ms` | — |
| `bStepUp` | `BOOL` | — |
| `bStepDown` | `BOOL` | — |
| `tStepTime` | `TIME := t#200ms` | — |
| `bPosition` | `BOOL` | — |
| `nSetPosition` | `USINT` | — |
| `bShadowPosition` | `BOOL` | — |
| `nShadowSetPosition` | `USINT := 80` | — |
| `tShadowTurnAroundTime` | `TIME:= t#0s` | — |
| `bSafetyPosition` | `BOOL` | — |
| `tDriveTime` | `TIME := t#60s` | — |
| `tDriveSwitchOverTime` | `TIME := t#200ms` | — |
| `tSwitchOverDeadTime` | `TIME := t#400ms` | — |
| `bUp` | `Set the bBlindUp output and reset the bBlindDown output. The bBlindUp output remains latched.` | — |
| `bDown` | `Set the bBlindDown output and reset the bBlindUp output. The bBlindDown output remains latched.` | — |
| `bStop` | `Reset the bBlindUp and bBlindDown outputs.` | — |
| `bSwitchOverUp` | `Set the bBlindUp output and reset the bBlindDown output. If the signal remains present for` | — |
| `bSwitchOverDown` | `Set the bBlindDown output and reset the bBlindUp output. If the signal remains present` | — |
| `tSwitchOverTime` | `Gives the time for which the bSwitchUp and bSwitchDown inputs must remain asserted` | — |
| `bStepUp` | `Reset the bBlindDown output and set the bBlindUp output for the time tStepTime.` | — |
| `bStepDown` | `Reset the bBlindUp output and set the bBlindDown output for the time tStepTime.` | — |
| `tStepTime` | `If the blind is controlled through the bStepUp or bStepDown inputs, the outputs remain asserted` | — |
| `bPosition` | `Move blind to specified position.` | — |
| `nSetPosition` | `Position (0%-100%) to which the blind is to be moved, after a positive edge has been applied` | — |
| `bShadowPosition` | `The shading position is approached.` | — |
| `nShadowSetPosition` | `Shading position (0%-100%) to which the blind is to be moved, after a positive edge` | — |
| `tShadowTurnAroundTime` | `Once the shading position has been reached, the blind is moved upwards for` | — |
| `bSafetyPosition` | `The safety position is approached. To do this, the blind is raised for the period tDriveTime` | — |
| `tDriveTime` | `Travel time of the blind from fully up to fully down. If no input is activated, the outputs are reset` | — |
| `tDriveSwitchOverTime` | `Period of time required for a change of direction of the blind.` | — |
| `tSwitchOverDeadTime` | `Dwell time at a change of direction. Both outputs are reset during this period.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBlindUp` | `BOOL` | — |
| `bBlindDown` | `BOOL` | — |
| `nActualPosition` | `USINT` | — |
| `bCalibrated` | `BOOL` | — |
| `bBlindUp` | `The blind opens.` | — |
| `bBlindDown` | `The blind closes.` | — |
| `nActualPosition` | `Current position in percent.` | — |
| `bCalibrated` | `Indicates whether the blind is calibrated.` | — |

**Description:** Four different methods are available for controlling the blind: • A positive edge at the bUp or bDown inputs set the bBlindUp or bBlindDown outputs respectively. These remain asserted until the time tDriveTime + 10% has elapsed, or until the function block receives some other command. Both outputs are immediately reset by a positive edge at the bStop input. • Static signals are provided to the bSwitchOverUp or bSwitchOverDown inputs (e.g. by buttons). These set the bBlindUp and bBlindDown outputs. If this signal is asserted for longer than tSwitchOverTime, the outputs are latched. This means that the outputs will continue to be asserted, even if the signals at the inputs are removed again. In most cases, a value of 500 ms is sufficient for the tSwitchOverTime parameter. However, the output only remains asserted for the time tDriveTime + 10%, or until a new command is given to the function block. • In certain applications it may be useful for the operator to be able to alter the blind position step by step. Each positive edge at the bStepUp or bStepDown inputs sets the corresponding output for the time tStepTime. A value of 200 ms has been found effective for tStepTime. • Unlike the FB_VenetianBlind() [} 21] function block, this function block also enables movement to an

##### `FB_VenetianBlindEx1Switch` — §4.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bUp` | `BOOL` | — |
| `bDown` | `BOOL` | — |
| `bStop` | `BOOL` | — |
| `bSwitch` | `BOOL` | — |
| `bStepUp` | `BOOL` | — |
| `bStepDown` | `BOOL` | — |
| `tStepTime` | `TIME := t#200ms` | — |
| `bPosition` | `BOOL` | — |
| `nSetPosition` | `USINT` | — |
| `bShadowPosition` | `BOOL` | — |
| `nShadowSetPosition` | `USINT := 80` | — |
| `tShadowTurnAroundTime` | `TIME := t#0s` | — |
| `bSafetyPosition` | `BOOL` | — |
| `tDriveTime` | `TIME := t#60s` | — |
| `tDriveSwitchOverTime` | `TIME := t#200ms` | — |
| `tSwitchOverDeadTime` | `TIME := t#400ms` | — |
| `bUp` | `Set the bBlindUp output and reset the bBlindDown output. The bBlindUp output remains latched.` | — |
| `bDown` | `Set the bBlindDown output and reset the bBlindUp output. The bBlindDown output remains latched.` | — |
| `bStop` | `Reset the bBlindUp and bBlindDown outputs.` | — |
| `bSwitch` | `Switching input for jogging the blind, see description above. Switches between moving, stopping,` | — |
| `bStepUp` | `Reset the bBlindDown output and set the bBlindUp output for the time tStepTime.` | — |
| `bStepDown` | `Reset the bBlindUp output and set the bBlindDown output for the time tStepTime.` | — |
| `tStepTime` | `If the blind is controlled through the bStepUp or bStepDown inputs, the outputs remain asserted` | — |
| `bPosition` | `Move blind to specified position.` | — |
| `nSetPosition` | `Position (0%-100%) to which the blind is to be moved after a positive edge has been applied` | — |
| `bShadowPosition` | `The shading position is approached.` | — |
| `nShadowSetPosition` | `Shading position (0%-100%) to which the blind is to be moved, after a positive edge` | — |
| `tShadowTurnAroundTime` | `Once the shading position has been reached, the blind is moved upwards for` | — |
| `bSafetyPosition` | `The safety position is approached. To do this, the blind is moved upwards for the period` | — |
| `tDriveTime` | `Travel time of the blind from fully up to fully down. If no input is activated, the outputs are reset` | — |
| `tDriveSwitchOverTime` | `Period of time required for a change of direction of the blind.` | — |
| `tSwitchOverDeadTime` | `Dwell time at a change of direction. Both outputs are reset during this period.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBlindUp` | `BOOL` | — |
| `bBlindDown` | `BOOL` | — |
| `nActualPosition` | `USINT` | — |
| `bCalibrated` | `BOOL` | — |
| `bBlindUp` | `The blind opens.` | — |
| `bBlindDown` | `The blind closes.` | — |
| `nActualPosition` | `Current position in percent.` | — |
| `bCalibrated` | `Indicates whether the blind is calibrated.` | — |

**Description:** Blind function block with the same functionality as FB_VenetianBlindEx() [} 23] but with an input bSwitch for jogging up and down. Four different methods are available for controlling the blind: • A positive edge at the bUp or bDown inputs set the bBlindUp or bBlindDown outputs respectively. These remain asserted until the time tDriveTime + 10% has elapsed, or until the function block receives some other command. Both outputs are immediately reset by a positive edge at the bStop input. • The input bSwitch, which is usually linked to a push button, can be used to switch the blind movement between up or down. In contrast to FB_VenetianBlindEx() [} 23], the outputs immediately switch to latching. Pressing the button switches between moving, stopping, moving in the opposite direction, stopping and moving again. The outputs remain active for a maximum period of tDriveTime + 10% or until a new command is issued for the function block. Switching is locked for the time tSwitchOverDeadTime. • In certain applications it may be useful for the operator to be able to alter the blind position step by step. Each positive edge at the bStepUp or bStepDown inputs sets the corresponding output for the

#### Programming › POUs

##### `Filter functions` — §4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `LREAL` | — |
| `fKp` | `LREAL := 1` | — |
| `tT1` | `TIME := t#10s` | — |
| `tCycletime` | `TIME := t#10ms` | — |
| `bSetActual` | `BOOL` | — |
| `fIn` | `Input value.` | — |
| `fkP` | `Gain factor, preset value: 1.` | — |
| `tT1` | `Damping time, preset value: 10 s.` | — |
| `tCycleTime` | `PLC cycle time, preset value: 10 ms.` | — |
| `bSetActual` | `Sets the output fOut directly to the input value fIn.` | — |
| `fIn` | `LREAL` | — |
| `fKp` | `LREAL := 1` | — |
| `tT1` | `TIME := t#10s` | — |
| `tT2` | `TIME := t#10s` | — |
| `tCycletime` | `TIME := t#10ms` | — |
| `bSetActual` | `BOOL` | — |
| `fIn` | `Input value.` | — |
| `fkP` | `Gain factor, preset value: 1.` | — |
| `tT1` | `Damping time 1, preset value: 10 s.` | — |
| `tT2` | `Damping time 2, preset value: 10 s.` | — |
| `tCycleTime` | `PLC cycle time, preset value: 10 ms.` | — |
| `bSetActual` | `Sets the output fOut directly to the input value fIn.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `LREAL` | — |
| `fOut` | `Output value.` | — |
| `fOut` | `LREAL` | — |
| `fOut` | `Output value.` | — |

**Description:** PT1 element for smoothing of input variables.

#### Programming › POUs › Lighting

##### `FB_ConstantLightControlEco` — §4.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `bToggle` | `BOOL` | — |
| `nSetpointValue` | `UINT := 16000` | — |
| `nActualValue` | `UINT` | — |
| `nHysteresis` | `UINT := 100` | — |
| `nMaxLevel` | `UINT := 32767` | — |
| `nMinLevel` | `UINT := 3276` | — |
| `nStepSize` | `UINT := 10` | — |
| `tDeadTime` | `TIME := t#50ms` | — |
| `nOptions` | `DWORD` | — |
| `bEnable` | `Enables the function block. If this input is FALSE, the inputs bOn, bOff and bToogle are disabled.` | — |
| `bOn` | `Switches the controlled devices to nMaxLevel and activates constant light control.` | — |
| `bOff` | `Switches the addressed devices off and disables constant light control.` | — |
| `bToggle` | `The lighting is switched on or off, depending on the state of the reference device.` | — |
| `nSetpointValue` | `This input is used for specifying the setpoint.` | — |
| `nActualValue` | `The actual value is applied at this input.` | — |
| `nHysteresis` | `Control hysteresis around the setpoint. If the actual value is within this range, the control` | — |
| `nMaxLevel` | `Maximum value of the control value at nLightLevel.` | — |
| `nMinLevel` | `Minimum value of the control value at nLightLevel. If this value is to be undercut by a dimming` | — |
| `nStepSize` | `Step size with which the control value nLightLevel is changed.` | — |
| `tDeadTime` | `Dead time between the individual steps (up or down) of the control value.` | — |
| `nOptions` | `Currently not used. Reserved for future extensions.` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nLightLevel` | `UINT` | — |
| `nLightLevel` | `Output control value of the function block and reference to the light output value. This value is` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nDeviation` | `INT` | — |
| `bControllerIsActive` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `nDeviation` | `Current control deviation (setpoint/actual value).` | — |
| `bControllerIsActive` | `This output is set once the control is activated.` | — |
| `bBusy` | `When the function block is activated, this output is always active when changes are made to the` | — |
| `bError` | `This output is switched to TRUE as soon as an error occurs during the execution of a command. The` | — |
| `nErrorId` | `Contains the command-specific error code of the most recently executed command. Is reset to "0"` | — |

**Description:** The function block is used for constant light control.

##### `FB_Dimmer1Switch` — §4.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchDimm` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `bSetDimmValue` | `BOOL` | — |
| `nDimmValue` | `UINT` | — |
| `tSwitchOverTime` | `TIME := t#500ms` | — |
| `tDimmTime` | `TIME := t#5s` | — |
| `tCycleDelay` | `TIME := t#10ms` | — |
| `bMemoryModeOn` | `BOOL := FALSE` | — |
| `nOnValueWithoutMemoryMode` | `UINT := 20000` | — |
| `bDimmOnMode` | `BOOL := FALSE` | — |
| `tDimmOnTime` | `TIME := t#0s` | — |
| `bDimmOffMode` | `BOOL := FALSE` | — |
| `tDimmOffTime` | `TIME := t#0s` | — |
| `nOutMin` | `UINT := 5000` | — |
| `nOutMax` | `UINT := 32767` | — |
| `bSwitchDimm` | `Switches or dims the output.` | — |
| `bOn` | `Switches the output to the last output value, or to the value specified by` | — |
| `bOff` | `Switches the output to 0.` | — |
| `bSetDimmValue` | `Switches the output to the value nDimmValue.` | — |
| `nDimmValue` | `The value is immediately applied to the output when there is a change.` | — |
| `tSwitchOverTime` | `Time for switching between the light on/off and dimming functions for the bSwitchDimm` | — |
| `tDimmTime` | `Time required for dimming to go from its minimum value to its maximum value.` | — |
| `tCycleDelay` | `Delay time, if either the minimum or maximum value is reached.` | — |
| `bMemoryModeOn` | `Switches over to use the memory function, so that the previous value is written to the` | — |
| `nOnValueWithoutMemoryMode` | `Switch-on value if the memory function is not active.` | — |
| `bDimmOnMode` | `The output value is increased in steps when switching on.` | — |
| `tDimmOnTime` | `Time period during which the light level is increased when the light is switched on.` | — |
| `bDimmOffMode` | `The output value is reduced in steps when switching off` | — |
| `tDimmOffTime` | `Time period during which the light is reduced when the light is switched off. bDimmOffMode` | — |
| `nOutMin` | `Minimum output value.` | — |
| `nOutMax` | `Maximum output value. If the parameter nOutMin is not smaller than nOutMax, the output will` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nOut` | `UINT` | — |
| `bLight` | `BOOL` | — |
| `nOut` | `analog output value.` | — |
| `bLight` | `digital output value. This is set if nOut is greater than 0.` | — |

**Description:** The function block is used to dim lights with a switch.

##### `FB_Dimmer1SwitchEco` — §4.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchDimm` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `tSwitchOverTime` | `TIME := t#500ms` | — |
| `tDimmTime` | `TIME := t#5s` | — |
| `tCycleDelay` | `TIME := t#500ms` | — |
| `tPLCCycle` | `TIME := t#10ms` | — |
| `bSwitchDimm` | `Switches or dims the output.` | — |
| `bOn` | `Switches the output to the last output value, or to the value specified by` | — |
| `bOff` | `Switches the output to 0.` | — |
| `tSwitchOverTime` | `Time for switching between the light on/off and dimming functions for the bSwitchDimm` | — |
| `tDimmTime` | `Time required for dimming to go from its minimum value to its maximum value.` | — |
| `tCycleDelay` | `Delay time, if either the minimum or maximum value is reached.` | — |
| `tPLCCycle` | `the set PLC cycle time.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nOut` | `UINT` | — |
| `bLight` | `BOOL` | — |
| `nOut` | `analog output value.` | — |
| `bLight` | `digital output value. This is set if nOut is greater than 0.` | — |

**Description:** This function block is a memory-saving variant of FB_Dimmer1Switch() [} 37]. It does not have the special functions "Set brightness value" and "Switch off memory function", which may not be required for many applications. Moreover, the values nOutMin and nOutMax of the FB_Dimmer1Switch() [} 37] are set internally here to 0 and 32767 respectively. This output span corresponds to the display range of an analog output terminal. The tPLCCycle input is important. This time is used to calculate internally the amount by which the nOut output must be increased per cycle - that saves additional time calculations.

##### `FB_Dimmer2Switch` — §4.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchDimmUp` | `BOOL` | — |
| `bSwitchDimmDown` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `bSetDimmValue` | `BOOL` | — |
| `nDimmValue` | `UINT` | — |
| `tSwitchOverTime` | `TIME := t#500ms` | — |
| `tDimmTime` | `TIME := t#5s` | — |
| `bMemoryModeOn` | `BOOL := FALSE` | — |
| `nOnValueWithoutMemoryMode` | `UINT := 20000` | — |
| `bDimmOnMode` | `BOOL := FALSE` | — |
| `tDimmOnTime` | `TIME := t#0s` | — |
| `bDimmOffMode` | `BOOL := FALSE` | — |
| `tDimmOffTime` | `TIME := t#0s` | — |
| `nOutMin` | `UINT := 5000` | — |
| `nOutMax` | `UINT := 32767` | — |
| `bSwitchDimmUp` | `Switches or dims the output Up.` | — |
| `bSwitchDimmDown` | `Switches or dims the output Down.` | — |
| `bOn` | `Switches the output to the last output value, or to the value specified by` | — |
| `bOff` | `Switches the output to 0.` | — |
| `bSetDimmValue` | `Switches the output to the value nDimmValue.` | — |
| `nDimmValue` | `The value is immediately applied to the output when there is a change.` | — |
| `tSwitchOverTime` | `Time for switching between the light on/off and dimming functions for the` | — |
| `tDimmTime` | `Time required for dimming to go from its minimum value to its maximum value.` | — |
| `bMemoryModeOn` | `Switches over to use the memory function, so that the previous value is written to the` | — |
| `nOnValueWithoutMemoryMode` | `Switch-on value if the memory function is not active.` | — |
| `bDimmOnMode` | `The output value is increased in steps when switching on.` | — |
| `tDimmOnTime` | `Time period during which the light level is increased when the light is switched on.` | — |
| `bDimmOffMode` | `The output value is reduced in steps when switching off` | — |
| `tDimmOffTime` | `Time period during which the light is reduced when the light is switched off. bDimmOffMode` | — |
| `nOutMin` | `Minimum output value.` | — |
| `nOutMax` | `Maximum output value. If the parameter nOutMin is not smaller than nOutMax, the output will` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nOut` | `UINT` | — |
| `bLight` | `BOOL` | — |
| `nOut` | `analog output value.` | — |
| `bLight` | `digital output value. This is set if nOut is greater than 0.` | — |

**Description:** The function range of the function block corresponds to that of the function block FB_Dimmer1Switch() [} 37]. The difference is that in the function block FB_Dimmer2Switch two switches are connected. This allows the user to choose specifically between dimming up or dimming down.

##### `FB_Dimmer2SwitchEco` — §4.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchDimmUp` | `BOOL` | — |
| `bSwitchDimmDown` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `tSwitchOverTime` | `TIME := t#500ms` | — |
| `tDimmTime` | `TIME := t#5s` | — |
| `tPLCCycle` | `TIME := t#10ms` | — |
| `bSwitchDimmUp` | `Switches or dims the output Up.` | — |
| `bSwitchDimmDown` | `Switches or dims the output Down.` | — |
| `bOn` | `Switches the output to the last output value.` | — |
| `bOff` | `Switches the output to 0.` | — |
| `tSwitchOverTime` | `Time for switching between the light on/off and dimming functions for the` | — |
| `tDimmTime` | `Time required for dimming to go from its minimum value to its maximum value.` | — |
| `tCycleDelay` | `Delay time, if either the minimum or maximum value is reached.` | — |
| `tPLCCycle` | `the set PLC cycle time.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nOut` | `UINT` | — |
| `bLight` | `BOOL` | — |
| `nOut` | `analog output value.` | — |
| `bLight` | `digital output value. This is set if nOut is greater than 0.` | — |

**Description:** This function block is a memory-saving variant of FB_Dimmer2Switch() [} 41]. It does not have the special functions "Set brightness value" and "Switch off memory function", which may not be required for many applications. Moreover, the values nOutMin and nOutMax of the FB_Dimmer2Switch() [} 41] are set internally here to 0 and 32767 respectively. This output span corresponds to the display range of an analog output terminal. The tPLCCycle input is important. This time is used to calculate internally the amount by which the nOut output must be increased per cycle - that saves additional time calculations.

##### `FB_Dimmer3Switch` — §4.1.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchDimm` | `BOOL` | — |
| `bSwitchDimmUp` | `BOOL` | — |
| `bSwitchDimmDown` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `bSetDimmValue` | `BOOL` | — |
| `nDimmValue` | `UINT` | — |
| `tSwitchOverTime` | `TIME := t#500ms` | — |
| `tDimmTime` | `TIME := t#5s` | — |
| `tCycleDelay` | `TIME := t#10ms` | — |
| `bMemoryModeOn` | `BOOL := FALSE` | — |
| `nOnValueWithoutMemoryMode` | `UINT := 20000` | — |
| `bDimmOnMode` | `BOOL := FALSE` | — |
| `tDimmOnTime` | `TIME := t#0s` | — |
| `bDimmOffMode` | `BOOL := FALSE` | — |
| `tDimmOffTime` | `TIME := t#0s` | — |
| `nOutMin` | `UINT := 5000` | — |
| `nOutMax` | `UINT := 32767` | — |
| `bSwitchDimm` | `Switches or dims the output.` | — |
| `bSwitchDimmUp` | `Switches or dims the output Up.` | — |
| `bSwitchDimmDown` | `Switches or dims the output Down.` | — |
| `bOn` | `Switches the output to the last output value, or to the value specified by` | — |
| `bOff` | `Switches the output to 0.` | — |
| `bSetDimmValue` | `Switches the output to the value nDimmValue.` | — |
| `nDimmValue` | `The value is immediately applied to the output when there is a change.` | — |
| `tSwitchOverTime` | `Time for switching between the light on/off and dimming functions for the` | — |
| `tDimmTime` | `Time required for dimming to go from its minimum value to its maximum value.` | — |
| `tCycleDelay` | `Delay time, if either the minimum or maximum value is reached.` | — |
| `bMemoryModeOn` | `Switches over to use the memory function, so that the previous value is written to the` | — |
| `nOnValueWithoutMemoryMode` | `Switch-on value if the memory function is not active.` | — |
| `bDimmOnMode` | `The output value is increased in steps when switching on.` | — |
| `tDimmOnTime` | `Time period during which the light level is increased when the light is switched on.` | — |
| `bDimmOffMode` | `The output value is reduced in steps when switching off` | — |
| `tDimmOffTime` | `Time period during which the light is reduced when the light is switched off. bDimmOffMode` | — |
| `nOutMin` | `Minimum output value.` | — |
| `nOutMax` | `Maximum output value. If the parameter nOutMin is not smaller than nOutMax, the output will` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nOut` | `UINT` | — |
| `bLight` | `BOOL` | — |
| `nOut` | `analog output value.` | — |
| `bLight` | `digital output value. This is set if nOut is greater than 0.` | — |

**Description:** The functionality of the function block corresponds to the function blocks FB_Dimmer1Switch() [} 37] and FB_Dimmer2Switch() [} 41].

##### `FB_Light` — §4.1.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `bToggle` | `BOOL` | — |
| `bOn` | `Switches the output on.` | — |
| `bOff` | `Switches the output off.` | — |
| `bToggle` | `Negates the state of the output.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLight` | `BOOL` | — |
| `bLight` | `The output is set with a positive edge at bOn .` | — |

**Description:** A positive edge at the bOn input sets the bLight output. The output is reset by a positive edge at the bOff input. If a positive edge is presented to bToggle, the output is negated; i.e., if On it goes Off, and if Off it goes On.

##### `FB_LightControl` — §4.1.5.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `nActualValue` | `UINT` | — |
| `tRampTime` | `TIME := t#30s` | — |
| `arrControlTable` | `ARRAY[1..30] OF ST_ControlTable` | — |
| `nOptions` | `DWORD` | — |
| `bEnable` | `The bOn and bOff inputs are active as long as this input is TRUE. A negative state deactivates the` | — |
| `bOn` | `A rising edge directly switches nLightLevel to the nearest control value.` | — |
| `bOff` | `A rising edge directly switches nLightLevel to "0".` | — |
| `nActualValue` | `Current brightness.` | — |
| `tRampTime` | `Time period during which the brightness value is controlled to the next control value. (Preset` | — |
| `value` | `30 s).` | — |
| `arrControlTable` | `Input value/control value table. arrControlTable[1] to arrControlTable[30] of type` | — |
| `nOptions` | `Reserved for future developments.` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nLightLevel` | `UINT` | — |
| `nLightLevel` | `Output control value of the function block and reference to the light output value.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLight` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bControlActive` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `bLight` | `This output is set as long as nLightLevel is greater than "0".` | — |
| `bBusy` | `This output is always active as long as the processing of a command (bOn, bOff, bToggle or ramp)` | — |
| `bControlActive` | `This output is active as long as the control is active.` | — |
| `bError` | `This output is switched to TRUE as soon as an error occurs during the execution of a command. The` | — |
| `nErrorId` | `Contains the specific error code of the most recently executed command. Is reset to "0" by the` | — |

**Description:** Function block for daylight-dependent lighting control with up to 30 interpolation points.

##### `FB_Ramp` — §4.1.5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `bToggle` | `BOOL` | — |
| `bStart` | `BOOL` | — |
| `nEndLevel` | `BYTE` | — |
| `tRampTime` | `TIME := t#10s` | — |
| `nOptions` | `DWORD` | — |
| `bEnable` | `The bOn, bOff, bToggle and bStart inputs are active as long as this input is TRUE. A negative` | — |
| `bOn` | `A rising edge directly switches nLightLevel to the maximum value (32767).` | — |
| `bOff` | `A rising edge directly switches nLightLevel to "0".` | — |
| `bToggle` | `Switches the light state between On (32767) and Off (0).` | — |
| `bStart` | `If a rising edge is applied to this input, the light is dimmed up or down from the current value to` | — |
| `nEndLevel` | `Target value of the dimming procedure.` | — |
| `tRampTime` | `Ramp time, see bStart. (Preset value: 10 seconds).` | — |
| `nOptions` | `Reserved for future developments.` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nLightLevel` | `UINT` | — |
| `nLightLevel` | `Output control value of the function block and reference to the light output value. This value is` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLight` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `bLight` | `This output is set as long as nLightLevel is greater than 0.` | — |
| `bBusy` | `This output is always active as long as the processing of a command (bOn, bOff, bToggle or ramp)` | — |
| `bError` | `This output is switched to TRUE as soon as an error occurs during the execution of a command. The` | — |
| `nErrorId` | `Contains the command-specific error code of the most recently executed command. Is reset to "0"` | — |

**Description:** Function block for realizing a light-ramp.

##### `FB_Sequencer` — §4.1.5.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |
| `bOn` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `bToggle` | `BOOL` | — |
| `bStart` | `BOOL` | — |
| `nStartIndex` | `USINT` | — |
| `arrSequenceTable` | `ARRAY[1..50] OF ST_SequenceTable` | — |
| `nOptions` | `DWORD` | — |
| `bEnable` | `The bOn, bOff, bToggle and bStart inputs are active as long as this input is TRUE. A negative` | — |
| `bOn` | `A rising edge directly switches nLightLevel to the maximum value (32767).` | — |
| `bOff` | `A rising edge directly switches nLightLevel to "0".` | — |
| `bToggle` | `Switches the light state between On (32767) and Off (0).` | — |
| `bStart` | `A positive edge starts a light sequence from the beginning defined under nStartIndex.` | — |
| `nStartIndex` | `See bStart.` | — |
| `arrSequenceTable` | `Light value table with the corresponding ramp and dwell times (see ST_SequenceTable` | — |
| `nOptions` | `Parameterization input. The setting (or non-setting) of the individual bits of this variable of the` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nLightLevel` | `UINT` | — |
| `nLightLevel` | `Output control value of the function block and reference to the light output value.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nActualIndex` | `USINT` | — |
| `bLight` | `BOOL` | — |
| `bSequenceActive` | `BOOL` | — |
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `nActualIndex` | `Reference to the current element in the sequence table. If a sequence has been completed` | — |
| `bLight` | `This output is set as long as nLightLevel is greater than "0".` | — |
| `bSequenceActive` | `On processing a sequence this output is set to TRUE.` | — |
| `bBusy` | `This output is always active as long as the processing of a command (bOn, bOff, bToggle or ramp)` | — |
| `bError` | `This output is switched to TRUE as soon as an error occurs during the execution of a command. The` | — |
| `nErrorId` | `Contains the specific error code of the most recently executed command. Is reset to "0" by the` | — |

**Description:** Function block for realizing light sequences with up to 50 interpolation points.

##### `FB_StairwellDimmer` — §4.1.5.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitch` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `nPresenceValue` | `UINT := 32767` | — |
| `nProlongValue` | `UINT := 10000` | — |
| `tPresenceDuration` | `TIME := t#120s` | — |
| `tFadeOffDuration` | `TIME := t#10s` | — |
| `tProlongDuration` | `TIME := t#20s` | — |
| `bSwitch` | `Upon a positive edge: nOut is set to nPresenceValue. Upon a negative edge: start of the presence` | — |
| `bOff` | `Switches nOut off immediately.` | — |
| `nPresenceValue` | `Value to which nOut should be set during the presence time. (Preset value: 32767).` | — |
| `nProlongValue` | `Value to which nOut should be set during the dwell time. (Preset value: 10000).` | — |
| `tPresenceDuration` | `Duration of the presence time in which nOut is set to nPresenceValue following a` | — |
| `tFadeOffDuration` | `Duration over which nOut is faded down to the dwell time following the presence time.` | — |
| `tProlongDuration` | `Duration of the dwell time. (Preset value: 20 seconds).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nOut` | `UINT` | — |
| `bLight` | `BOOL` | — |
| `nOut` | `Output of the momentary light value.` | — |
| `bLight` | `This output is set as long as nOut is greater than 0.` | — |

**Description:** A rising edge at the input bSwitch sets the analog output nOut to the value nPresenceValue. A falling edge on bSwitch starts or restarts a timer with the runtime tPresenceDuration. Following the expiry of this timer, nOut is dimmed to the value nProlongValue over the time period tFadeOffDuration. This value is maintained for the time period tProlongDuration. After that, nOut is set to 0. A positive edge at the input bOff switches the output nOut to 0 immediately. The digital output value bLight is always set when nOut is greater than 0.

##### `FB_StairwellLight` — §4.1.5.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitch` | `BOOL` | — |
| `bOff` | `BOOL` | — |
| `tLightDuration` | `TIME := t#120s` | — |
| `tPreWarningStart` | `TIME := t#110s` | — |
| `tPreWarningDuration` | `TIME := t#500ms` | — |
| `bSwitch` | `Switches the output on for the period of time given by tLightDuration.` | — |
| `bOff` | `Switches the output off.` | — |
| `tLightDuration` | `Period of time for which the output is set.` | — |
| `tPreWarningStart` | `Warning time.` | — |
| `tPreWarningDuration` | `Duration of the prewarning.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bLight` | `BOOL` | — |
| `bLight` | `The output is set for the duration of tLightDuration with a positive edge at bSwitch.` | — |

**Description:** A positive edge at the bSwitch input sets the bLight output. The output is reset once the tLightDuration time has elapsed. If a signal is presented again to the bSwitch input before this time has elapsed, the timer is restarted. When tPreWarningStart has elapsed, the light is switched off (as a prewarning) for the period tPreWarningDuration. If this prewarning is not to be given, the parameter tPreWarningStart must be set to 0. A positive edge at the bOff input switches the output off immediately.

#### Programming › POUs › Scene management

##### `FB_RoomOperation` — §4.1.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitch_A` | `BOOL` | — |
| `bSwitch_B` | `BOOL` | — |
| `bSwitch_1` | `BOOL` | — |
| `bSwitch_2` | `BOOL` | — |
| `bSwitch_3` | `BOOL` | — |
| `bSwitch_4` | `BOOL` | — |
| `bSwitch_5` | `BOOL` | — |
| `bSwitch_6` | `BOOL` | — |
| `bSwitch_7` | `BOOL` | — |
| `bSwitch_8` | `BOOL` | — |
| `bSwitch_9` | `BOOL` | — |
| `bSwitch_10` | `BOOL` | — |
| `bSwitch_11` | `BOOL` | — |
| `bSwitch_12` | `BOOL` | — |
| `bSwitch_13` | `BOOL` | — |
| `bSwitch_14` | `BOOL` | — |
| `bSwitchLightingMode` | `BOOL` | — |
| `bSwitchBlindingMode` | `BOOL` | — |
| `bFeedbackLighting_1` | `BOOL` | — |
| `bFeedbackLighting_2` | `BOOL` | — |
| `bFeedbackLighting_3` | `BOOL` | — |
| `bFeedbackLighting_4` | `BOOL` | — |
| `bFeedbackLighting_5` | `BOOL` | — |
| `bFeedbackLighting_6` | `BOOL` | — |
| `bFeedbackLighting_7` | `BOOL` | — |
| `bFeedbackLighting_8` | `BOOL` | — |
| `bFeedbackLighting_9` | `BOOL` | — |
| `bFeedbackLighting_10` | `BOOL` | — |
| `bFeedbackLighting_11` | `BOOL` | — |
| `bFeedbackLighting_12` | `BOOL` | — |
| `bFeedbackLighting_13` | `BOOL` | — |
| `bFeedbackLighting_14` | `BOOL` | — |
| `nFeedbackLighting_1` | `UINT` | — |
| `nFeedbackLighting_2` | `UINT` | — |
| `nFeedbackLighting_3` | `UINT` | — |
| `nFeedbackLighting_4` | `UINT` | — |
| `nFeedbackLighting_5` | `UINT` | — |
| `nFeedbackLighting_6` | `UINT` | — |
| `nFeedbackLighting_7` | `UINT` | — |
| `nFeedbackLighting_8` | `UINT` | — |
| `nFeedbackLighting_9` | `UINT` | — |
| `nFeedbackLighting_10` | `UINT` | — |
| `nFeedbackLighting_11` | `UINT` | — |
| `nFeedbackLighting_12` | `UINT` | — |
| `nFeedbackLighting_13` | `UINT` | — |
| `nFeedbackLighting_14` | `UINT` | — |
| `nFeedbackBlind_1` | `USINT` | — |
| `nFeedbackBlind_2` | `USINT` | — |
| `nFeedbackBlind_3` | `USINT` | — |
| `nFeedbackBlind_4` | `USINT` | — |
| `nFeedbackBlind_5` | `USINT` | — |
| `nFeedbackBlind_6` | `USINT` | — |
| `nFeedbackBlind_7` | `USINT` | — |
| `tCycleDelayDimmTime` | `TIME := t#500ms` | — |
| `tOperationTime` | `TIME := t#60s` | — |
| `bSwitchLightingMode` | `switches to the lighting mode.` | — |
| `bSwitchBlindingMode` | `switches to the blinding mode.` | — |
| `tCycleDelayDimmTime` | `switching time between dimming and calling a scene.` | — |
| `tOperationTime` | `if the blinding or lighting mode is active and no operation takes place, the mode is` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnableLightingMode` | `BOOL` | — |
| `bEnableBlindingMode` | `BOOL` | — |
| `bSwitchLighting_1` | `BOOL` | — |
| `bSwitchLighting_2` | `BOOL` | — |
| `bSwitchLighting_3` | `BOOL` | — |
| `bSwitchLighting_4` | `BOOL` | — |
| `bSwitchLighting_5` | `BOOL` | — |
| `bSwitchLighting_6` | `BOOL` | — |
| `bSwitchLighting_7` | `BOOL` | — |
| `bSwitchLighting_8` | `BOOL` | — |
| `bSwitchLighting_9` | `BOOL` | — |
| `bSwitchLighting_10` | `BOOL` | — |
| `bSwitchLighting_11` | `BOOL` | — |
| `bSwitchLighting_12` | `BOOL` | — |
| `bSwitchLighting_13` | `BOOL` | — |
| `bSwitchLighting_14` | `BOOL` | — |
| `bSwitchBlindUp_1` | `BOOL` | — |
| `bSwitchBlindDown_1` | `BOOL` | — |
| `bSwitchBlindUp_2` | `BOOL` | — |
| `bSwitchBlindDown_2` | `BOOL` | — |
| `bSwitchBlindUp_3` | `BOOL` | — |
| `bSwitchBlindDown_3` | `BOOL` | — |
| `bSwitchBlindUp_4` | `BOOL` | — |
| `bSwitchBlindDown_4` | `BOOL` | — |
| `bSwitchBlindUp_5` | `BOOL` | — |
| `bSwitchBlindDown_5` | `BOOL` | — |
| `bSwitchBlindUp_6` | `BOOL` | — |
| `bSwitchBlindDown_6` | `BOOL` | — |
| `bSwitchBlindUp_7` | `BOOL` | — |
| `bSwitchBlindDown_7` | `BOOL` | — |
| `bInvokeScene_A` | `BOOL` | — |
| `bInvokeScene_B` | `BOOL` | — |
| `bInvokeScene_1` | `BOOL` | — |
| `bInvokeScene_2` | `BOOL` | — |
| `bInvokeScene_3` | `BOOL` | — |
| `bInvokeScene_4` | `BOOL` | — |
| `bInvokeScene_5` | `BOOL` | — |
| `bInvokeScene_6` | `BOOL` | — |
| `bInvokeScene_7` | `BOOL` | — |
| `bInvokeScene_8` | `BOOL` | — |
| `bInvokeScene_9` | `BOOL` | — |
| `bInvokeScene_10` | `BOOL` | — |
| `bInvokeScene_11` | `BOOL` | — |
| `bInvokeScene_12` | `BOOL` | — |
| `bInvokeScene_13` | `BOOL` | — |
| `bInvokeScene_14` | `BOOL` | — |
| `bSaveScene_A` | `BOOL` | — |
| `bSaveScene_B` | `BOOL` | — |
| `bSaveScene_1` | `BOOL` | — |
| `bSaveScene_2` | `BOOL` | — |
| `bSaveScene_3` | `BOOL` | — |
| `bSaveScene_4` | `BOOL` | — |
| `bSaveScene_5` | `BOOL` | — |
| `bSaveScene_6` | `BOOL` | — |
| `bSaveScene_7` | `BOOL` | — |
| `bSaveScene_8` | `BOOL` | — |
| `bSaveScene_9` | `BOOL` | — |
| `bSaveScene_10` | `BOOL` | — |
| `bSaveScene_11` | `BOOL` | — |
| `bSaveScene_12` | `BOOL` | — |
| `bSaveScene_13` | `BOOL` | — |
| `bSaveScene_14` | `BOOL` | — |
| `bLEDSwitch_1` | `BOOL` | — |
| `bLEDSwitch_2` | `BOOL` | — |
| `bLEDSwitch_3` | `BOOL` | — |
| `bLEDSwitch_4` | `BOOL` | — |
| `bLEDSwitch_5` | `BOOL` | — |
| `bLEDSwitch_6` | `BOOL` | — |
| `bLEDSwitch_7` | `BOOL` | — |
| `bLEDSwitch_8` | `BOOL` | — |
| `bLEDSwitch_9` | `BOOL` | — |
| `bLEDSwitch_10` | `BOOL` | — |
| `bLEDSwitch_11` | `BOOL` | — |
| `bLEDSwitch_12` | `BOOL` | — |
| `bLEDSwitch_13` | `BOOL` | — |
| `bLEDSwitch_14` | `BOOL` | — |
| `bLEDLightingMode` | `BOOL` | — |
| `bLEDBlindingMode` | `BOOL` | — |
| `bEnableLightingMode` | `enables the memory function block FB_ScenesLighting() [} 63].` | — |
| `bEnableBlindingMode` | `enables the memory function block FB_ScenesVenetianBlind() [} 66].` | — |
| `bLEDLightingMode` | `this output is TRUE if lighting mode is active.` | — |
| `bLEDBlindingMode` | `this output is TRUE if blinding mode is active.` | — |

**Description:** Programming

##### `FB_ScenesLighting` — §4.1.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |
| `bInvokeScene_1` | `BOOL` | — |
| `bInvokeScene_2` | `BOOL` | — |
| `bInvokeScene_3` | `BOOL` | — |
| `bInvokeScene_4` | `BOOL` | — |
| `bInvokeScene_5` | `BOOL` | — |
| `bInvokeScene_6` | `BOOL` | — |
| `bInvokeScene_7` | `BOOL` | — |
| `bInvokeScene_8` | `BOOL` | — |
| `bInvokeScene_9` | `BOOL` | — |
| `bInvokeScene_10` | `BOOL` | — |
| `bInvokeScene_11` | `BOOL` | — |
| `bInvokeScene_12` | `BOOL` | — |
| `bInvokeScene_13` | `BOOL` | — |
| `bInvokeScene_14` | `BOOL` | — |
| `bInvokeScene_15` | `BOOL` | — |
| `bInvokeScene_16` | `BOOL` | — |
| `bSaveScene_1` | `BOOL` | — |
| `bSaveScene_2` | `BOOL` | — |
| `bSaveScene_3` | `BOOL` | — |
| `bSaveScene_4` | `BOOL` | — |
| `bSaveScene_5` | `BOOL` | — |
| `bSaveScene_6` | `BOOL` | — |
| `bSaveScene_7` | `BOOL` | — |
| `bSaveScene_8` | `BOOL` | — |
| `bSaveScene_9` | `BOOL` | — |
| `bSaveScene_10` | `BOOL` | — |
| `bSaveScene_11` | `BOOL` | — |
| `bSaveScene_12` | `BOOL` | — |
| `bSaveScene_13` | `BOOL` | — |
| `bSaveScene_14` | `BOOL` | — |
| `bSaveScene_15` | `BOOL` | — |
| `bSaveScene_16` | `BOOL` | — |
| `nActualValueLighting_1` | `UINT` | — |
| `nActualValueLighting_2` | `UINT` | — |
| `nActualValueLighting_3` | `UINT` | — |
| `nActualValueLighting_4` | `UINT` | — |
| `nActualValueLighting_5` | `UINT` | — |
| `nActualValueLighting_6` | `UINT` | — |
| `nActualValueLighting_7` | `UINT` | — |
| `nActualValueLighting_8` | `UINT` | — |
| `nActualValueLighting_9` | `UINT` | — |
| `nActualValueLighting_10` | `UINT` | — |
| `nActualValueLighting_11` | `UINT` | — |
| `nActualValueLighting_12` | `UINT` | — |
| `nActualValueLighting_13` | `UINT` | — |
| `nActualValueLighting_14` | `UINT` | — |
| `sFile` | `STRING` | — |
| `nOptions` | `UDINT` | — |
| `bEnable` | `enables the function block.` | — |
| `sFile` | `file name (without path and file extension) for saving the scenes. The file name must be unique in the` | — |
| `nOptions` | `Reserved for future developments.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetDimmValue_1` | `BOOL` | — |
| `nDimmValue_1` | `UINT` | — |
| `bSetDimmValue_2` | `BOOL` | — |
| `nDimmValue_2` | `UINT` | — |
| `bSetDimmValue_3` | `BOOL` | — |
| `nDimmValue_3` | `UINT` | — |
| `bSetDimmValue_4` | `BOOL` | — |
| `nDimmValue_4` | `UINT` | — |
| `bSetDimmValue_5` | `BOOL` | — |
| `nDimmValue_5` | `UINT` | — |
| `bSetDimmValue_6` | `BOOL` | — |
| `nDimmValue_6` | `UINT` | — |
| `bSetDimmValue_7` | `BOOL` | — |
| `nDimmValue_7` | `UINT` | — |
| `bSetDimmValue_8` | `BOOL` | — |
| `nDimmValue_8` | `UINT` | — |
| `bSetDimmValue_9` | `BOOL` | — |
| `nDimmValue_9` | `UINT` | — |
| `bSetDimmValue_10` | `BOOL` | — |
| `nDimmValue_10` | `UINT` | — |
| `bSetDimmValue_11` | `BOOL` | — |
| `nDimmValue_11` | `UINT` | — |
| `bSetDimmValue_12` | `BOOL` | — |
| `nDimmValue_12` | `UINT` | — |
| `bSetDimmValue_13` | `BOOL` | — |
| `nDimmValue_13` | `UINT` | — |
| `bSetDimmValue_14` | `BOOL` | — |
| `nDimmValue_14` | `UINT` | — |
| `bInit` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `bInit` | `this output goes TRUE as soon as the initialization of the function block is complete.` | — |
| `bError` | `this output is set to TRUE as soon as an error is detected during execution. The error code is` | — |
| `nErrorId` | `contains the error code as soon as bError goes TRUE. See Error codes [} 92].` | — |

**Description:** Programming

##### `FB_ScenesVenetianBlind` — §4.1.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |
| `bInvokeScene_1` | `BOOL` | — |
| `bInvokeScene_2` | `BOOL` | — |
| `bInvokeScene_4` | `BOOL` | — |
| `bInvokeScene_5` | `BOOL` | — |
| `bInvokeScene_6` | `BOOL` | — |
| `bInvokeScene_7` | `BOOL` | — |
| `bInvokeScene_8` | `BOOL` | — |
| `bInvokeScene_9` | `BOOL` | — |
| `bInvokeScene_10` | `BOOL` | — |
| `bInvokeScene_11` | `BOOL` | — |
| `bInvokeScene_12` | `BOOL` | — |
| `bInvokeScene_13` | `BOOL` | — |
| `bInvokeScene_14` | `BOOL` | — |
| `bInvokeScene_15` | `BOOL` | — |
| `bInvokeScene_16` | `BOOL` | — |
| `bSaveScene_1` | `BOOL` | — |
| `bSaveScene_2` | `BOOL` | — |
| `bSaveScene_3` | `BOOL` | — |
| `bSaveScene_4` | `BOOL` | — |
| `bSaveScene_5` | `BOOL` | — |
| `bSaveScene_6` | `BOOL` | — |
| `bSaveScene_7` | `BOOL` | — |
| `bSaveScene_8` | `BOOL` | — |
| `bSaveScene_9` | `BOOL` | — |
| `bSaveScene_10` | `BOOL` | — |
| `bSaveScene_11` | `BOOL` | — |
| `bSaveScene_12` | `BOOL` | — |
| `bSaveScene_13` | `BOOL` | — |
| `bSaveScene_14` | `BOOL` | — |
| `bSaveScene_15` | `BOOL` | — |
| `bSaveScene_16` | `BOOL` | — |
| `nActualValueBlinding_1` | `UINT` | — |
| `nActualValueBlinding_2` | `USINT` | — |
| `nActualValueBlinding_3` | `USINT` | — |
| `nActualValueBlinding_4` | `USINT` | — |
| `nActualValueBlinding_5` | `USINT` | — |
| `nActualValueBlinding_6` | `USINT` | — |
| `nActualValueBlinding_7` | `USINT` | — |
| `sFile` | `STRING` | — |
| `nOptions` | `DWORD` | — |
| `bEnable` | `enables the function block.` | — |
| `sFile` | `file name (without path and file extension) for saving the scenes. The file name must be unique in the` | — |
| `nOptions` | `Reserved for future developments.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetBlindValue_1` | `BOOL` | — |
| `nBlindValue_1` | `USINT` | — |
| `bSetBlindValue_2` | `BOOL` | — |
| `nBlindValue_2` | `USINT` | — |
| `bSetBlindValue_3` | `BOOL` | — |
| `nBlindValue_3` | `USINT` | — |
| `bSetBlindValue_4` | `BOOL` | — |
| `nBlindValue_4` | `USINT` | — |
| `bSetBlindValue_5` | `BOOL` | — |
| `nBlindValue_5` | `USINT` | — |
| `bSetBlindValue_6` | `BOOL` | — |
| `nBlindValue_6` | `USINT` | — |
| `bSetBlindValue_7` | `BOOL` | — |
| `nBlindValue_7` | `USINT` | — |
| `bInit` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `bInit` | `this output goes TRUE as soon as the initialization of the function block is complete.` | — |
| `bError` | `this output is set to TRUE as soon as an error is detected during execution. The error code is` | — |
| `nErrorId` | `contains the error code as soon as bError goes TRUE. See Error codes [} 92].` | — |

**Description:** This function block is intended for the management of blind scenes. The function block is enabled via the bEnable input. The loading of the saved scenes is started by a positive edge at the bEnable input. The input must remain TRUE until the operation is completed. The values of the scenes are saved non-volatile in the TwinCAT Boot directory as a *.bin file. The last data status is saved in a *.bak file as a backup.

#### Programming › POUs › Signal processing

##### `FB_ShortLongClick` — §4.1.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitch` | `BOOL` | — |
| `tSwitchTime` | `TIME := t#50ms` | — |
| `bSwitch` | `Input signal.` | — |
| `tSwitchTime` | `Duration above which the input signal is to be interpreted as a long button press.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bShortClick` | `BOOL` | — |
| `bLongClick` | `BOOL` | — |
| `bShortClick` | `Indicates a short push of the button.` | — |
| `bLongClick` | `Indicates a long push of the button.` | — |

**Description:** If the bSwitch input is longer than the tSwitchTime, the bLongClick output is set for one PLC cycle. Otherwise, the bShortClick output is set.

##### `FB_SignallingContact` — §4.1.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tDelayOnTime` | `TIME := t#100ms` | — |
| `tDelayOffTime` | `TIME := t#100ms` | — |
| `bQuitSignal` | `BOOL` | — |
| `bContact` | `BOOL` | — |
| `tDelayOnTime` | `Delay before setting the message signal.` | — |
| `tDelayOffTime` | `Delay before resetting the message signal.` | — |
| `bQuitSignal` | `Input to acknowledge message signal.` | — |
| `bContact` | `Input for the signaling contact.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nSignalState` | `WORD` | — |
| `nSignalState` | `Message status.` | — |

**Description:** The two inputs tDelayOnTime and tDelayOffTime allow slow operation and slow release delays to be set. If a message signal is to be acknowledged before this time can be ended, this is done by means of the bQuitSignal input. The state of the signaling contact is communicated to the function block via the bContact input.

##### `FB_SingleDoubleClick` — §4.1.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitch` | `BOOL` | — |
| `tSwitchTime` | `TIME := t#500ms` | — |
| `bSwitch` | `Input signal.` | — |
| `tSwitchTime` | `Duration above which the input signal is to be interpreted as a double button press.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSingleClick` | `BOOL` | — |
| `bDoublelick` | `BOOL` | — |
| `bSingleClick` | `Indicates a single push of the button.` | — |
| `bDoublelick` | `Indicates a double push of the button.` | — |

**Description:** If the input signal is presented twice within the time tSwitchTime, the bDoubleClick output is set for one PLC cycle. Otherwise, the bSingleClick output is set.

##### `FB_ThresholdSwitch` — §4.1.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSignal` | `LREAL` | — |
| `fLowerLimit` | `LREAL := 16000` | — |
| `fUpperLimit` | `LREAL := 17000` | — |
| `tLowerLimitDelay` | `TIME := t#100ms` | — |
| `tUpperLimitDelay` | `TIME := t#100ms` | — |
| `fSignal` | `Input signal.` | — |
| `fLowerLimit` | `Lower limit value.` | — |
| `fUpperLimit` | `Upper limit value.` | — |
| `tLowerLimitDelay` | `Switching delay when passing beyond the lower limit.` | — |
| `tUpperLimitDelay` | `Switching delay when passing beyond the upper limit.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchingSignal` | `BOOL` | — |
| `bCrossLowerLimit` | `BOOL` | — |
| `bCrossUpperLimit` | `BOOL` | — |
| `bSwitchingSignal` | `State depends on bCrossLowerLimit and bCrossUpperLimit.` | — |
| `bCrossLowerLimit` | `Becomes TRUE for a cycle if fLowerLimit has fallen below the time tLowerLimitDelay.` | — |
| `bCrossUpperLimit` | `Becomes TRUE for a cycle if fUpperLimit is exceeded for the time tUpperLimitDelay. At` | — |

**Description:** If the input signal exceeds the limit value fUpperLimit for the duration specified by tUpperLimitDelay, the output bCrossUpperLimit is set for one PLC cycle. The bSwitchingSignal output is also set. This remains set until the input signal passes below the value of fLowerLimit for the duration specified by tLowerLimitDelay. In this case, the output fCrossLowerLimit is set for one PLC cycle.

#### Programming › POUs › Timer functions

##### `FB_DailyScheduler` — §4.1.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `uiPeriodicity` | `UINT` | — |
| `uiBegin` | `UINT` | — |
| `eStartEnd` | `E_StartEnd` | — |
| `stStartEnd` | `ST_StartEnd` | — |
| `stSystemtime` | `TIMESTRUCT` | — |
| `uiPeriodicity` | `Periodicity or interval. May be within the range 1...365.` | — |
| `uiBegin` | `Starting value for the day counter. May be within the range 1...365.` | — |
| `eStartEnd` | `Selection of the start/end definition (see E_StartEnd [} 93]).` | — |
| `stStartEnd` | `Structure with the parameters defining the start and end. Unused variables, for example the` | — |
| `stSystemtime` | `current time in TIMESTRUCT format. It is important to count every second.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bTriggerOn` | `BOOL` | — |
| `bNoEventNextYear` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `bOut` | `Control output that is switched on or off by the start and end event.` | — |
| `bTriggerOn` | `Trigger output for switch-on events. This output is intended for logging switch-on events. If two` | — |
| `bNoEventNextYear` | `No day matching the parameterization was found within the next 366 days.` | — |
| `bError` | `This output is set to TRUE if the parameterization is faulty. The command-specific error code is` | — |
| `nErrorId` | `Contains the command-specific error code. Reset to 0 once the parameterization is correct. See` | — |

**Description:** Function block for triggering actions every n-th day of the year.

##### `FB_WeeklyScheduler` — §4.1.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `uiPeriodicity` | `UINT` | — |
| `uiBegin` | `UINT` | — |
| `arrActiveWeekday` | `ARRAY[0..6] OF BOOL` | — |
| `eStartEnd` | `E_StartEnd` | — |
| `stStartEnd` | `ST_StartEnd` | — |
| `stSystemtime` | `TIMESTRUCT` | — |
| `uiPeriodicity` | `Periodicity or interval. May be within the range 1...52.` | — |
| `uiBegin` | `Starting value for the week. May be within the range 1...52.` | — |
| `arrActiveWeekday` | `Day of the week on which switching is to take place - arrActiveWeekday[0] => Sunday ..` | — |
| `eStartEnd` | `Selection of the start/end definition (see E_StartEnd [} 93]).` | — |
| `stStartEnd` | `Structure with the parameters defining the start and end. Unused variables, for example the` | — |
| `stSystemtime` | `Current time in TIMESTRUCT format. It is important to count every second.` | — |
| `arrActiveMonth` | `ARRAY[1..12] OF BOOL` | — |
| `uiActiveWeekday` | `UINT` | — |
| `eStartEnd` | `E_StartEnd` | — |
| `stStartEnd` | `ST_StartEnd` | — |
| `stSystemtime` | `TIMESTRUCT` | — |
| `arrActiveMonth` | `Month in which an action is to be triggered - arrActiveMonth[1]=>January ..` | — |
| `uiActiveWeekday` | `Day of the week on which an action is to be triggered in the selected months.` | — |
| `eStartEnd` | `Selection of the start/end definition (see E_StartEnd [} 93]).` | — |
| `stStartEnd` | `Structure with the parameters defining the start and end. Unused variables, for example the` | — |
| `stSystemtime` | `Current time in TIMESTRUCT format. It is important to count every second.` | — |
| `arrActiveMonth` | `ARRAY[1..12] OF BOOL` | — |
| `uiActiveWeekday` | `UINT` | — |
| `eStartEnd` | `E_StartEnd` | — |
| `stStartEnd` | `ST_StartEnd` | — |
| `stSystemtime` | `TIMESTRUCT` | — |
| `arrActiveMonth` | `Month in which an action is to be triggered - arrActiveMonth[1]=>January ...` | — |
| `uiActiveDay` | `Day of the month on which an action is to be triggered. Multiple selections are not possible.` | — |
| `eStartEnd` | `Selection of the start/end definition (see E_StartEnd [} 93]).` | — |
| `stStartEnd` | `Structure with the parameters defining the start and end. Unused variables, for example the` | — |
| `stSystemtime` | `Current time in TIMESTRUCT format. It is important to count every second.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bTriggerOn` | `BOOL` | — |
| `bNoEventNextYear` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `bOut` | `Control output that is switched on or off by the start and end event.` | — |
| `bTriggerOn` | `Trigger output for switch-on events. This output is intended for logging switch-on events. If two` | — |
| `bNoEventNextYear` | `No day matching the parameterization was found within the next 366 days.` | — |
| `bError` | `This output is set to TRUE if the parameterization is faulty. The command-specific error code is` | — |
| `nErrorId` | `Contains the command-specific error code. Reset to 0 once the parameterization is correct. See` | — |
| `bOut` | `BOOL` | — |
| `bTriggerOn` | `BOOL` | — |
| `bNoEventNextYear` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `bOut` | `Control output that is switched on or off by the start and end event.` | — |
| `bTriggerOn` | `Trigger output for switch-on events. This output is intended for logging switch-on events. If two` | — |
| `bNoEventNextYear` | `No day matching the parameterization was found within the next 366 days.` | — |
| `bError` | `This output is set to TRUE if the parameterization is faulty. The command-specific error code is` | — |
| `nErrorId` | `Contains the command-specific error code. Reset to 0 once the parameterization is correct. See` | — |
| `bOut` | `BOOL` | — |
| `bTriggerOn` | `BOOL` | — |
| `bNoEventNextYear` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `bOut` | `Control output that is switched on or off by the start and end event.` | — |
| `bTriggerOn` | `Trigger output for switch-on events. This output is intended for logging switch-on events. If two` | — |
| `bNoEventNextYear` | `No day matching the parameterization was found within the next 366 days.` | — |
| `bError` | `This output is set to TRUE if the parameterization is faulty. The command-specific error code is` | — |
| `nErrorId` | `Contains the command-specific error code. Reset to 0 once the parameterization is correct. See` | — |

**Description:** Function block for triggering actions on certain weekdays in each n-th week of the year.

##### `FB_YearlyScheduler` — §4.1.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `uiMonth` | `UINT` | — |
| `uiDay` | `UINT` | — |
| `eStartEnd` | `E_StartEnd` | — |
| `stStartEnd` | `ST_StartEnd` | — |
| `stSystemtime` | `TIMESTRUCT` | — |
| `uiMonth` | `Month in which an action is to be triggered. Multiple selections are not possible.` | — |
| `uiDay` | `Day on which an action is to be triggered. Multiple selections are not possible.` | — |
| `eStartEnd` | `Selection of the start/end definition (see E_StartEnd [} 93]).` | — |
| `stStartEnd` | `Structure with the parameters defining the start and end. Unused variables, for example the` | — |
| `stSystemtime` | `Current time in TIMESTRUCT format. It is important to count every second.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOut` | `BOOL` | — |
| `bTriggerOn` | `BOOL` | — |
| `bNoEventNextYear` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrorId` | `UDINT` | — |
| `bOut` | `Control output that is switched on or off by the start and end event.` | — |
| `bTriggerOn` | `Trigger output for switch-on events. This output is intended for logging switch-on events. If two` | — |
| `bNoEventNextYear` | `No day matching the parameterization was found within the next 366 days.` | — |
| `bError` | `This output is set to TRUE if the parameterization is faulty. The command-specific error code is` | — |
| `nErrorId` | `Contains the command-specific error code. Reset to 0 once the parameterization is correct. See` | — |

**Description:** Function block for triggering actions on a certain day of the year.

##### `FB_WeeklyTimeSwitch` — §4.1.8.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | — |
| `tCurrentDateTime` | `DATE_AND_TIME` | — |
| `tSwitchOnTime` | `TOD` | — |
| `tSwitchOffTime` | `TOD` | — |
| `bSunday` | `BOOL` | — |
| `bMonday` | `BOOL` | — |
| `bTuesday` | `BOOL` | — |
| `bWednesday` | `BOOL` | — |
| `bThursday` | `BOOL` | — |
| `bFriday` | `BOOL` | — |
| `bSaturday` | `BOOL` | — |
| `bEnable` | `Function block enable.` | — |
| `tCurrentDateTime` | `current date and time.` | — |
| `tSwitchOnTime` | `switch-on time.` | — |
| `tSwitchOffTime` | `switch-off time.` | — |
| `bSunday` | `consider timer on Sundays.` | — |
| `bMonday` | `consider timer on Mondays.` | — |
| `bTuesday` | `consider timer on Tuesdays.` | — |
| `bWednesday` | `consider timer on Wednesdays.` | — |
| `bThursday` | `consider timer on Thursdays.` | — |
| `bFriday` | `consider timer on Fridays.` | — |
| `bSaturday` | `consider timer on Saturdays.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bOutput` | `BOOL` | — |
| `bEdgeOn` | `BOOL` | — |
| `bEdgeOff` | `BOOL` | — |
| `bOutput` | `output becomes TRUE if the current time is between the switch-on time and the switch-off time.` | — |
| `bEdgeOn` | `the output is set to TRUE for one PLC cycle when the time channel becomes active.` | — |
| `bEdgeOff` | `the output is set to TRUE for one PLC cycle when the time channel becomes deactivated.` | — |

**Description:** The two parameters tSwitchOnTime and tSwitchOffTime define a time period during which the output bOutput is to be active. If the timer is only to apply on certain days of the week, this can be set via the inputs bSunday, bMonday, ..., bSaturday. Several time channels can be switched by creating several instances of the function block. Each instance is responsible for one time channel.

##### `FB_CalcSunPosition` — §4.1.8.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fDegreeOfLongitude` | `LREAL := 8.5` | — |
| `fDegreeOfLatitude` | `LREAL := 51.9` | — |
| `dtGMT` | `TIMESTRUCT` | — |
| `fDegreeOfLongitude` | `Longitude in degrees.` | — |
| `fDegreeofLatitude` | `Latitude in degrees.` | — |
| `dtGMT` | `Current time as Greenwich Mean Time (GMT).` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fSunAzimuth` | `LREAL` | — |
| `fSunElevation` | `LREAL` | — |
| `fSunAzimuth` | `Direction of the sun (northern hemisphere: 0° north ... 90° east ... 180° south ... 270° west ... /` | — |
| `fSunElevation` | `Height of the sun (0° horizontal ... 90° vertical).` | — |

**Description:** Calculation of sun position based on the date, time, longitude and latitude.

##### `FB_CalcSunriseSunset` — §4.1.8.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fDegreeOfLongitude` | `LREAL := 8.5` | — |
| `fDegreeOfLatitude` | `LREAL := 51.9` | — |
| `fReferenceMeridian` | `LREAL` | — |
| `dCurrentDate` | `DATE` | — |
| `fDegreeOfLongitude` | `Longitude in degrees.` | — |
| `fDegreeofLatitude` | `Latitude in degrees.` | — |
| `fReferenceMeridian` | `Reference meridian of the time zone.` | — |
| `dCurrentDate` | `current date.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `todSunrise` | `TOD` | — |
| `todSunset` | `TOD` | — |
| `todSunrise` | `Sunrise. Output of hour and minute.` | — |
| `todSunset` | `Sunset. Output of hour and minute.` | — |

**Description:** Function block for calculating sunrise and sunset based on the longitude, latitude, reference meridian and time.

##### `FB_CalcPublicHolidaysDE` — §4.1.8.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dCurrentDate` | `DATE` | — |
| `dCurrentDate` | `current date.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `dNewYearsDay` | `DATE` | — |
| `dEpiphany` | `DATE` | — |
| `dGoodFriday` | `DATE` | — |
| `dEasterSunday` | `DATE` | — |
| `dEasterMonday` | `DATE` | — |
| `dLabourDay` | `DATE` | — |
| `dAscensionDay` | `DATE` | — |
| `dWhitSunday` | `DATE` | — |
| `dWhitMonday` | `DATE` | — |
| `dCorpusChristi` | `DATE` | — |
| `dAssumptionDay` | `DATE` | — |
| `dGermanUnificationDay` | `DATE` | — |
| `dReformationDay` | `DATE` | — |
| `dAllSaintsDay` | `DATE` | — |
| `dPenanceDay` | `DATE` | — |
| `dChristmasEve` | `DATE` | — |
| `d1stChristmasDay` | `DATE` | — |
| `d2ndChristmasDay` | `DATE` | — |
| `dNewYearsEve` | `DATE` | — |
| `bNewYearsDay` | `BOOL` | — |
| `bEpiphany` | `BOOL` | — |
| `bGoodFriday` | `BOOL` | — |
| `bEasterSunday` | `BOOL` | — |
| `bEasterMonday` | `BOOL` | — |
| `bLabourDay` | `BOOL` | — |
| `bAscensionDay` | `BOOL` | — |
| `bWhitSunday` | `BOOL` | — |
| `bWhitMonday` | `BOOL` | — |
| `bCorpusChristi` | `BOOL` | — |
| `bAssumptionDay` | `BOOL` | — |
| `bGermanUnificationDay` | `BOOL` | — |
| `bReformationDay` | `BOOL` | — |
| `bAllSaintsDay` | `BOOL` | — |
| `bPenanceDay` | `BOOL` | — |
| `bChristmasEve` | `BOOL` | — |
| `b1stChristmasDay` | `BOOL` | — |
| `b2ndChristmasDay` | `BOOL` | — |
| `bNewYearsEve` | `BOOL` | — |
| `dxxxxxx` | `Date of the respective holiday.` | — |
| `bxxxxxx` | `Boolean statement indicating whether today is the respective holiday.` | — |

**Description:** German holidays for the current year are calculated based on the date entered. A boolean output indicates whether the entered date matches one of the calculated holidays. To ensure international readability the function block was translated into English. The parameters have the following meaning:

##### `FB_CalcPublicHolidaysUS` — §4.1.8.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dCurrentDate` | `DATE` | — |
| `dCurrentDate` | `current date.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `dNewYearsDay` | `DATE` | — |
| `dEpiphany` | `DATE` | — |
| `dGoodFriday` | `DATE` | — |
| `dEasterSunday` | `DATE` | — |
| `dEasterMonday` | `DATE` | — |
| `dLabourDay` | `DATE` | — |
| `dAscensionDay` | `DATE` | — |
| `dWhitSunday` | `DATE` | — |
| `dWhitMonday` | `DATE` | — |
| `dCorpusChristi` | `DATE` | — |
| `dAssumptionDay` | `DATE` | — |
| `dGermanUnificationDay` | `DATE` | — |
| `dReformationDay` | `DATE` | — |
| `dAllSaintsDay` | `DATE` | — |
| `dPenanceDay` | `DATE` | — |
| `dChristmasEve` | `DATE` | — |
| `d1stChristmasDay` | `DATE` | — |
| `d2ndChristmasDay` | `DATE` | — |
| `dNewYearsEve` | `DATE` | — |
| `bNewYearsDay` | `BOOL` | — |
| `bEpiphany` | `BOOL` | — |
| `bGoodFriday` | `BOOL` | — |
| `bEasterSunday` | `BOOL` | — |
| `bEasterMonday` | `BOOL` | — |
| `bLabourDay` | `BOOL` | — |
| `bAscensionDay` | `BOOL` | — |
| `bWhitSunday` | `BOOL` | — |
| `bWhitMonday` | `BOOL` | — |
| `bCorpusChristi` | `BOOL` | — |
| `bAssumptionDay` | `BOOL` | — |
| `bGermanUnificationDay` | `BOOL` | — |
| `bReformationDay` | `BOOL` | — |
| `bAllSaintsDay` | `BOOL` | — |
| `bPenanceDay` | `BOOL` | — |
| `bChristmasEve` | `BOOL` | — |
| `b1stChristmasDay` | `BOOL` | — |
| `b2ndChristmasDay` | `BOOL` | — |
| `bNewYearsEve` | `BOOL` | — |
| `dxxxxxx` | `Date of the respective holiday.` | — |
| `bxxxxxx` | `Boolean statement indicating whether today is the respective holiday.` | — |

**Description:** Calculation of the United States public holidays.

##### `FB_CalcFederalHolidaysUS` — §4.1.8.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dCurrentDate` | `DATE` | — |
| `dCurrentDate` | `current date.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `dNewYearsDay` | `DATE` | — |
| `dMartinLutherKingJrDay` | `DATE` | — |
| `dPresidentsDay` | `DATE` | — |
| `dMemorialDay` | `DATE` | — |
| `dIndependenceDay` | `DATE` | — |
| `dLaborDay` | `DATE` | — |
| `dColumbusDay` | `DATE` | — |
| `dVeteransDay` | `DATE` | — |
| `dThanksgivingDay` | `DATE` | — |
| `dChristmasDay` | `DATE` | — |
| `dNewYearsDay` | `BOOL` | — |
| `dMartinLutherKingJrDay` | `BOOL` | — |
| `dPresidentsDay` | `BOOL` | — |
| `dMemorialDay` | `BOOL` | — |
| `dIndependenceDay` | `BOOL` | — |
| `dLaborDay` | `BOOL` | — |
| `dColumbusDay` | `BOOL` | — |
| `dVeteransDay` | `BOOL` | — |
| `dThanksgivingDay` | `BOOL` | — |
| `dChristmasDay` | `BOOL` | — |
| `dxxxxxx` | `Date of the respective holiday.` | — |
| `bxxxxxx` | `Boolean statement indicating whether today is the respective holiday.` | — |

**Description:** US federal holidays for the current year are calculated based on the date entered. A boolean output indicates whether the entered date matches one of the calculated holidays. To ensure international readability the function block was translated into English. The parameters have the following meaning:
