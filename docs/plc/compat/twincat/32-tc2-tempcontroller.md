# Tc2_TempController

- **Manual:** `TF4110_TC3_Temperature_Controller_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Control · **Tier:** T0 · **API entries:** 3
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Temperature control algorithms: pure control math; sensor/actuator IO via image.

---

#### PLC libraries › Function Block

##### `FB_CTRL_TempController` — §5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eCtrlMode` | `E_CTRL_MODE` | — |
| `bSelSetpoint` | `BOOL` | — |
| `fW1` | `LREAL` | — |
| `fW2` | `LREAL` | — |
| `fX` | `LREAL` | — |
| `fYManual` | `LREAL` | — |
| `bOpenThermocouple` | `BOOL; (* thermocouple *)` | — |
| `bReverseThermocouple` | `BOOL` | — |
| `bBackVoltage` | `BOOL` | — |
| `bLeakage` | `BOOL; (* heating system *)` | — |
| `bShortCircuit` | `BOOL` | — |
| `bOpenCircuit` | `BOOL` | — |
| `sParaControllerExternal` | `ST_CTRL_ParaController` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sControllerParameter` | `ST_CTRL_TempCtrlParameter; (* controller parameter set *)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fYAnalog` | `LREAL` | — |
| `bYPWMPos` | `BOOL` | — |
| `bYPWMNeg` | `BOOL` | — |
| `bYDigPos` | `BOOL` | — |
| `bYDigNeg` | `BOOL` | — |
| `dwAlarm` | `DWORD` | — |
| `fMaxOverShoot` | `LREAL` | — |
| `tStartUpTime` | `TIME` | — |
| `eCtrlState` | `E_CTRL_STATE := eCTRL_STATE_IDLE` | — |
| `sParaControllerInternal` | `ST_CTRL_ParaController` | — |
| `bError` | `BOOL` | — |
| `eErrorId` | `E_CTRL_ErrorCodes` | — |

**Description:** The temperature controller function block FB_CTRL_TempController has various inputs and outputs, which are described below. All the controller's parameters are passed to it via structures.

##### `FB_TempController` — §5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bOn` | `BOOL` | — |
| `bInit` | `BOOL` | — |
| `bTune` | `BOOL` | — |
| `bManual` | `BOOL` | — |
| `bSelSetpoint` | `BOOL` | — |
| `bSelCtrlParameterSe` | `BOOL` | — |
| `bEnableSoftStart` | `BOOL` | — |
| `bEnableRamping` | `BOOL` | — |
| `fW1` | `LREAL` | — |
| `fW2` | `LREAL` | — |
| `fX` | `LREAL` | — |
| `bOpenThermocouple` | `BOOL` | — |
| `bReverseThermocouple` | `BOOL` | — |
| `bBackVoltage` | `BOOL` | — |
| `bLeakage` | `BOOL` | — |
| `bShortCircuit` | `BOOL` | — |
| `bOpenCircuit` | `BOOL` | — |
| `sParaControllerExternal` | `ST_ParaController` | — |
| `sLogData` | `ST_LogData := (bLog := FALSE, strLogFileName :='', strLogString :=` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sControllerParameter` | `ST_ControllerParameter` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fYAnalog` | `LREAL` | — |
| `bYPWM` | `BOOL` | — |
| `bYDig` | `BOOL` | — |
| `bYDigPos` | `BOOL` | — |
| `bYDigNeg` | `BOOL` | — |
| `dwAlarm` | `DWORD` | — |
| `iState` | `States        := TC_STATE_IDLE` | — |
| `sParaControllerInternal` | `ST_ParaController` | — |
| `bError` | `BOOL` | — |
| `iErrorId` | `ErrorCodes` | — |

**Description:** The temperature controller function block FB_TempController has a variety of inputs and outputs that are described below. All the controller's parameters are passed to it via structures. The definition of the structures and enums can be found here [} 26].

##### `FB_CTRL_TempController_DistComp` — §5.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eCtrlMode` | `E_CTRL_MODE` | — |
| `bSelSetpoint` | `BOOL` | — |
| `fW1` | `LREAL` | — |
| `fW2` | `LREAL` | — |
| `fX` | `LREAL` | — |
| `fYManual` | `LREAL` | — |
| `bOpenThermocouple` | `BOOL` | — |
| `bReverseThermocouple` | `BOOL` | — |
| `bBackVoltage` | `BOOL` | — |
| `bLeakage` | `BOOL` | — |
| `bShortCircuit` | `BOOL` | — |
| `bOpenCircuit` | `BOOL` | — |
| `fD` | `LREAL` | — |
| `bCompensateDisturbance` | `BOOL` | — |
| `stParaControllerExternal` | `ST_CTRL_ParaController` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sControllerParameter` | `ST_CTRL_TempCtrlParameter` | — |
| `sCompensatorParameter` | `ST_CTRL_DistCompParameter` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fYAnalog` | `LREAL` | — |
| `bYPWMPos` | `BOOL` | — |
| `bYPWMPos` | `BOOL` | — |
| `bYPWMNeg` | `BOOL` | — |
| `bYDigPos` | `BOOL` | — |
| `bYDigNeg` | `BOOL` | — |
| `dwAlarm` | `DWORD` | — |
| `fMaxOverShoot` | `LREAL` | — |
| `tStartUpTime` | `TIME` | — |
| `eCtrlState` | `E_CTRL_STATE` | — |
| `sParaControllerInternal` | `ST_CTRL_ParaController` | — |
| `bError` | `BOOL` | — |
| `eErrorId` | `E_CTRL_ErrorCodes` | — |

**Description:** This temperature controller function block adds disturbance compensation to the FB_CTRL_TempController function blocks. The structure is described here.
