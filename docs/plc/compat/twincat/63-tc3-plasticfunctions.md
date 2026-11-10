# Tc3_PlasticFunctions

- **Manual:** `TF8560_Plastic_Technology_Functions_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Industry · **Tier:** T0 · **API entries:** 218
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Plastic processing function blocks: pure process logic.

---

#### Axis types › FB_AxisNcTrafoBase › How a transformation axis works

##### `FB_TrafoTableGenerator` (FUNCTION_BLOCK) — §4.2.2.1.1

**Description:** Represents the base class for generation algorithms of transformation cam plates.

##### `FB_ClampTableGenerator` (FUNCTION_BLOCK) — §4.2.2.1.2

**Description:** Implements a calculation algorithm for a standard clamp mechanism.

##### `FB_CrankTableGenerator` (FUNCTION_BLOCK) — §4.2.2.1.3

**Description:** Implements a calculation algorithm for a crank mechanism.

##### `FB_ScotchYokeTableGenerator` (FUNCTION_BLOCK) — §4.2.2.1.4

**Description:** Implements a calculation algorithm for a scotch yoke mechanism.

##### `FB_CrankDoubleShaftTableGenerator` (FUNCTION_BLOCK) — §4.2.2.1.5

**Description:** Implements a calculation algorithm for a crank mechanism with drive levers on both sides.

#### Core functions › Basic structure and variations

##### `Core scope (FB_Corefunction)` (FUNCTION_BLOCK) — §5.1.1

**Description:** All Corefunctions have a number of common features that are implemented in a ABSTRACT FB_Corefunction. A number of Corefunctions are derived from FB_CorefunctionFeedback [} 71] and provide further properties and methods. Through the inheritance of FB_Corefunction, each Corefunction receives the properties and methods described here.

##### `Commandable variants (FB_CorefunctionFeedback)` (FUNCTION_BLOCK) — §5.1.2

**Description:** These Corefunctions are activated by a command. The functions of this group are derived from a ABSTRACT FB_CorefunctionFeedback. As a result, all Corefunctions of this group have additional properties compared to the variants derived from FB_Corefunction.

#### Core functions

##### `Actuals` (FUNCTION_BLOCK) — §5.2

**Description:** This core function is not instantiated directly. It is used as a common part of type-specific core functions like ActualsHydraulics, ActualsNc or ActualsActuator.

##### `ActualsActuator` (FUNCTION_BLOCK) — §5.3

**Description:** This core function is used to display a compilation of actual values of an actuator. It is an extension of Actuals and extends the range of its parent element by adding specific interpretations for actuators.

##### `ActualsHydraulics` (FUNCTION_BLOCK) — §5.4

**Description:** This core function is used to display a compilation of actual values of the axis. It is an extension of Actuals and extends the range of its parent element by adding specific values for hydraulic axes.

##### `ActualsNc` (FUNCTION_BLOCK) — §5.5

**Description:** This core function is used to display a compilation of actual values of the axis. It is an extension of Actuals and extends the range of its parent element by adding specific values for NC axes.

##### `AutoIdent` (FUNCTION_BLOCK) — §5.6

**Description:** This core function is used to analyze the non-linear transfer characteristic of hydraulic axes. It belongs to the group of statically controlled Corefunctions.

#### Core functions › AutoIdent

##### `DoAutoIdent()` (METHOD) → `HRESULT` — §5.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the identification. A falling edge cancels a still active |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoAutoIdent` | `HRESULT` | See below |

**Description:** This method is used to activate the core function.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `EnableArreaRatio` | `BOOL` | A TRUE will request a separate identification of effects |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | See below |

**Description:** This method can be used to set the parameters for identification.

#### Core functions

##### `Camming` (FUNCTION_BLOCK) — §5.7

**Description:** The property provides two options: Camming.Pos for position camming and Camming.Velo for velocity camming. To do this, two classes of the same type are instantiated, but with different parameterizations.

#### Core functions › Camming

##### `DoCamming` (METHOD) → `HRESULT` — §5.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | A TRUE activates the camming. A FALSE ends the camming and slows down a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoCamming` | `HRESULT` | See below |

**Description:** This method is used to enable and disable the core function.

##### `FB_CammingLookUp` (FUNCTION_BLOCK) — §5.7.2

**Description:** This class is able to dynamically allocate memory and make it available as a table. Similarly, this function block acts as a LookUp table for the cam plate core function or the transformation table of a transformation axis.

#### Core functions › Camming › FB_CammingLookUp

##### `GetPoint()` (METHOD) → `HRESULT` — §5.7.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | Index of the requested table point |
| `fSlave` | `REFERENCE Slave value of the table point TO LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetPoint` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** This method returns a point in the table addressed by nIdx

##### `GetPoints()` (METHOD) → `HRESULT` — §5.7.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nPoints` | `REFERENCE Number of available points TO INT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetPoint` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** This method returns the number of points available in the table

##### `GetPointNonEqui()` (METHOD) → `HRESULT` — §5.7.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | Index of the table point to be read. |
| `fMaster` | `REFERNCE TO LREAL` | Master value of the table point. |
| `fSlave` | `REFERNCE TO LREAL` | Slave value of the table point. |
| `fLimiting` | `REFERNCE TO LREAL` | Limiting value (secondary slave value) of the table point. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetPointNonEq HRESULT` | `Return value with feedback on the success of the execution. ui` | — |

**Description:** This method reads a point in the table addressed by nIdx that has a specific master position. If the equidistant operation mode (Equidistant= FALSE) is active, this function returns a 0 as the master value.

##### `GetProperties()` (METHOD) → `HRESULT` — §5.7.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `REFERENCE The table is validated TO BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetProperties` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** This method returns the master limit values of the table. These are used to break down the master position in equidistant operation modes of the table.

##### `LookDown()` (METHOD) → `HRESULT` — §5.7.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Slave` | `LREAL` | Primary slave value |
| `SlaveVelo` | `LREAL` | Tertiary slave value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LookDown` | `HRESULT` | Return value with feedback on the success of the execution |

**Description:** This method translates a slave value to a matching master value. If the value lies between two defined points in the table, the selected operation mode is used for interpolation.

##### `LookUp()` (METHOD) → `HRESULT` — §5.7.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fGuidingValue` | `LREAL` | Master value |
| `fGuidingVelo` | `LREAL` | Reserved for later use. |
| `Slave` | `REFERENCE Resulting slave value. TO LREAL` | — |
| `SlaveVelo` | `REFERENCE Reserved for later use. TO LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LookDown` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** This method translates a master value to a matching slave value. If the value lies between two defined points in the table, the selected operation mode is used for interpolation.

##### `SetPoint()` (METHOD) → `HRESULT` — §5.7.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | Index of the table point to be set. |
| `fSlave` | `LREAL` | Slave value of the table point. |
| `fTorqueLimiting LREAL` | `TorqueLimiting value (secondary slave value) of the table point.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetPoint` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** This method sets a point in the table addressed via nIdx.

##### `SetPointNonEquidistant()` (METHOD) → `HRESULT` — §5.7.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | Index of the table point to be set. |
| `fMaster` | `LREAL` | Master value of the table point. |
| `fSlave` | `LREAL` | Slave value of the table point. |
| `fTorqueLimiting LREAL` | `TorqueLimiting value (secondary slave value) of the table point.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetPointNonEq HRESULT` | `Return value with feedback on the success of the execution. uidistant` | — |

**Description:** This method sets a point in the table addressed via nIdx that has a specific master position. If the equidistant operation mode (Equidistant= FALSE) is active, the master value is ignored.

##### `SetPoints()` (METHOD) → `HRESULT` — §5.7.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nRequired` | `INT` | Number of points requested. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetPoint` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** This method sets the number of available points in the table.

##### `SetProperties()` (METHOD) → `HRESULT` — §5.7.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL` | Defines the valid initialization of the table. |
| `fMaxGuidingVal LREAL` | `Maximum value of the master. ue` | — |
| `fMinGuidingVal LREAL` | `Minimum value of the master. ue` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetProperties` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** This method sets the master limit values of the table. These are used to break down the master position in equidistant operation modes of the table.

#### Core functions

##### `DirectOutput` (FUNCTION_BLOCK) — §5.8

**Description:** This core function is used to directly control the output of a hydraulic axis. There will be no monitoring of position limits.

#### Core functions › DirectOutput

##### `DoActivate` (METHOD) → `HRESULT` — §5.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A TRUE enables the output. A FALSE ramps the output to zero and disables |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoActivate` | `HRESULT` | See below |

**Description:** This method enables and disables the direct output.

#### Core functions

##### `DisableSoftEnd` (FUNCTION_BLOCK) — §5.9

**Description:** This core function is used to temporarily disable and re-enable the software position limits of the axis. • DisableSoftEnd is a member of the group of edge-triggered core functions.

#### Core functions › DisableSoftEnd

##### `DoDisable` (METHOD) → `HRESULT` — §5.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers the disabling. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoDisable` | `HRESULT` | See below |

**Description:** This method disables the set software position limits.

##### `ReEnable` (METHOD) → `HRESULT` — §5.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers the re-enabling. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReEnable` | `HRESULT` | See below |

**Description:** This method re-enables the set software position limits.

#### Core functions

##### `ExternalGenerating` (FUNCTION_BLOCK) — §5.10

**Description:** This core function is used for setpoint generation generated in TwinCAT 3 Plastic Technlogy Functions. It belongs to the group of statically controlled core functions.

#### Core functions › ExternalGenerating

##### `DoEnable()` (METHOD) → `HRESULT` — §5.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | A rising edge triggers the external setpoint generation. A falling edge stops the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoEnable` | `HRESULT` | See below |

**Description:** This method enables the external setpoint generation.

#### Core functions

##### `Estop` (FUNCTION_BLOCK) — §5.11

**Description:** This Corefunction is used to trigger an emergency stop operation. It will use the maximum dynamic parameters that are allowed for this axis by the underlying motion technology.

#### Core functions › Estop

##### `DoEstop` (METHOD) → `HRESULT` — §5.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge at this input triggers the stop. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoEstop` | `HRESULT` | See below |

**Description:** This method is used to trigger the stop.

#### Core functions

##### `Homing` (FUNCTION_BLOCK) — §5.12

**Description:** The property offers access to a range of homing functions.

#### Core functions › Homing › Abort

##### `DoAbort()` (METHOD) → `HRESULT` — §5.12.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers the abort. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoAbort` | `HRESULT` | See below |

**Description:** This method triggers the abort in case of a rising edge.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.12.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bOptionsDisableDriveAccess` | `BOOL` | A TRUE prevents ADS communication via the fieldbus from |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** The procedure for returning to normal operation can be specified.

#### Core functions › Homing › AbsoluteSwitch

##### `DoHoming()` (METHOD) → `HRESULT` — §5.12.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers homing. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoHoming` | `HRESULT` | See below |

**Description:** This method triggers the homing.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.12.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetPosition` | `Specifies the value that is applied to the actual position at the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A set of parameters specific to this homing procedure is set.

##### `SetParameterGeneral()` (METHOD) → `HRESULT` — §5.12.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDirection` | `E_AdaptableHomingDirection` | The movement with which the homing event is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameterGeneral` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A general set of travel parameters for the homing procedure is set.

#### Core functions › Homing › AbsoluteSwitchDetect

##### `DoHoming()` (METHOD) → `HRESULT` — §5.12.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers homing. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoHoming` | `HRESULT` | See below |

**Description:** This method triggers the homing.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.12.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eSwitchMode` | `Defines how the AbsoluteSwitch is interpreted as a homing` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A set of parameters specific to this homing procedure is set.

##### `SetParameterGeneral()` (METHOD) → `HRESULT` — §5.12.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDirection` | `E_AdaptableHomingDirection` | The movement with which the homing event is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameterGeneral` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A general set of travel parameters for the homing procedure is set.

#### Core functions › Homing › Block

##### `DoHoming()` (METHOD) → `HRESULT` — §5.12.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers homing. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoHoming` | `HRESULT` | See below |

**Description:** This method triggers the homing.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.12.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetPosition` | `The position to be assigned to the homing event.` | — |
| `fDetectionVelocityLimit` | `A velocity threshold for the detection of the homing event. For` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A set of parameters specific to this homing procedure is set.

##### `SetParameterGeneral()` (METHOD) → `HRESULT` — §5.12.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDirection` | `E_AdaptableHomingDirection` | The movement with which the homing event is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameterGeneral` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A general set of travel parameters for the homing procedure is set.

#### Core functions › Homing › BlockDetect

##### `DoHoming()` (METHOD) → `HRESULT` — §5.12.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers homing. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoHoming` | `HRESULT` | See below |

**Description:** This method triggers the homing.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.12.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fDetectionVelocityLimit` | `A velocity threshold for the detection of the homing event. For` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A set of parameters specific to this homing procedure is set.

##### `SetParameterGeneral()` (METHOD) → `HRESULT` — §5.12.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDirection` | `E_AdaptableHomingDirection` | The movement with which the homing event is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameterGeneral` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A general set of travel parameters for the homing procedure is set.

#### Core functions › Homing › Finish

##### `DoFinish()` (METHOD) → `HRESULT` — §5.12.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers the termination. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoFinish` | `HRESULT` | See below |

**Description:** This method triggers the termination on a rising edge.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.12.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fDistance` | `LREAL` | Here you can define a distance by which the axis should |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** The procedure for returning to normal operation can be specified.

#### Core functions › Homing › LimitSwitch

##### `DoHoming()` (METHOD) → `HRESULT` — §5.12.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers homing. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoHoming` | `HRESULT` | See below |

**Description:** This method triggers the homing.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.12.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSetPosition` | `Specifies the value that is applied to the actual position at the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A set of parameters specific to this homing procedure is set.

##### `SetParameterGeneral()` (METHOD) → `HRESULT` — §5.12.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDirection` | `E_AdaptableHomingDirection` | The movement with which the homing event is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameterGeneral` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A general set of travel parameters for the homing procedure is set.

#### Core functions › Homing › LimitSwitchDetect

##### `DoHoming()` (METHOD) → `HRESULT` — §5.12.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers homing. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoHoming` | `HRESULT` | See below |

**Description:** This method triggers the homing.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.12.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eSwitchMode` | `Defines how the AbsoluteSwitch is interpreted as a homing` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A set of parameters specific to this homing procedure is set.

##### `SetParameterGeneral()` (METHOD) → `HRESULT` — §5.12.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDirection` | `E_AdaptableHomingDirection` | The movement with which the homing event is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameterGeneral` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A general set of travel parameters for the homing procedure is set.

#### Core functions › Jog

##### `DoJogM()` (METHOD) → `HRESULT` — §5.13.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | A rising edge starts a jogging in negative direction. A falling edge triggers a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoJogM` | `HRESULT` | See below |

**Description:** This method performs the start and stop of the axis in the negative direction.

##### `DoJogP()` (METHOD) → `HRESULT` — §5.13.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | A rising edge starts a jogging in positive direction. A falling edge triggers a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoJogP` | `HRESULT` | See below |

**Description:** This method performs the start and stop of the axis in the positive direction.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.13.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fAcceleration` | `LREAL` | The commanded acceleration. |
| `fDeceleration` | `LREAL` | The commanded deceleration. |
| `fJerk` | `LREAL` | The commanded jerk. |
| `fVelocity` | `LREAL` | The commanded velocity. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** This method is used to set the parameters of a movement in jogging mode.

#### Core functions

##### `Limiting` (FUNCTION_BLOCK) — §5.14

**Description:** This Corefunction is responsible for limiting all axis functions. The type of limitation depends on the drive technology. For example, the torque of electric axes and the pressure of hydraulic axes can be limited. The following function blocks are automatically available internally for axes of type FB_AxisNcBase: • FB_LimitingOutputCoE [} 258] – DS402 (AX8000)-based TorqueLimiting • FB_LimitingOutputSoE [} 259] – AX5000-Based torque limiting

##### `MotionParams` (FUNCTION_BLOCK) — §5.15

**Description:** This Corefunction is used to display a range of parameter values for a motion unit.

#### Core functions › MotionParams

##### `ActuatorParamsH` (FUNCTION_BLOCK) — §5.15.1

**Description:** This core function combines several parameters per end position of an actuator.

##### `MdMotionParams` (FUNCTION_BLOCK) — §5.15.2

**Description:** This class implements a machine data component to store the writable MotionParams [} 150] of an axis.

#### Core functions

##### `MotionSetpoints` (FUNCTION_BLOCK) — §5.16

**Description:** This core function offers a range of current setpoints.

##### `Power` (FUNCTION_BLOCK) — §5.17

**Description:** This core function is used to enable or disable active operation of the controlled device.

#### Core functions › Power

##### `DoPower()` (METHOD) → `HRESULT` — §5.17.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | A rising edge starts the enable process. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoPower` | `HRESULT` | See below |

**Description:** This method is used to enable or disable the axis for active operation. If this requires an exchange of signals with a device, this exchange is performed and monitored.

##### `FeedEnable()` (METHOD) → `HRESULT` — §5.17.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bFeedEnaPositive` | `BOOL` | A TRUE enables active movements in positive direction. |
| `bFeedEnaNegative` | `BOOL` | A TRUE enables active movements in negative direction. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FeedEnable` | `HRESULT` | See below |

**Description:** This method is used to define directional enables for active movements of the axis.

#### Core functions

##### `PressureControl` (FUNCTION_BLOCK) — §5.18

**Description:** This Corefunction is provided to allow access to a number of controller types. There is currently an extended PID controller [} 162] and a supervisor for reading the pressure value and switching to pressure control.

#### Core functions › PressureControl › PressureControl.PID

##### `Activate` (METHOD) → `HRESULT` — §5.18.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | Enable for the controller. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Activate` | `HRESULT` | See below |

**Description:** This method is used to activate and deactivate the controller.

##### `SwitchOver` (METHOD) → `HRESULT` — §5.18.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchOver` | `BOOL` | The activation signal of the pressure handler. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SwitchOver` | `HRESULT` | See below |

**Description:** This method can be used by a pressure handler function block.

##### `FB_PressureControlParams_PID` (FUNCTION_BLOCK) — §5.18.1.3

**Description:** This function block contains a parameter set to be used by a FB_PressureControl_PID [} 162] function block.

#### Core functions › PressureControl › PressureControl.PID › FB_PressureControlParams_PID

##### `GetBoolParameter` (METHOD) → `HRESULT` — §5.18.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eSelect` | `E_PressureControlParam` | The selection of the parameter. |
| `bValue` | `REFERENCE TO BOOL` | A reference to the variable to be updated with the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetBoolParameter` | `HRESULT` | See below |

**Description:** This method is used to read BOOL parameters of the controller. For details see E_PressureControlParam [} 170].

##### `GetFloatParameter` (METHOD) → `HRESULT` — §5.18.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eSelect` | `E_PressureControlParam` | The selection of the parameter. |
| `fValue` | `REFERENCE TO LREAL` | A reference to the variable to be updated with the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetFloatParameter` | `HRESULT` | See below |

**Description:** This method is used to read LREAL parameters of the controller. For details see E_PressureControlParam [} 170].

##### `SetBoolParameter` (METHOD) → `HRESULT` — §5.18.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eSelect` | `E_PressureControlParam` | The selection of the parameter. |
| `bValue` | `BOOL` | The value with which the parameter is to be defined. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetBoolParameter` | `HRESULT` | See below |

**Description:** This method is used to define BOOL parameters of the controller. For details see E_PressureControlParam [} 170].

##### `SetFloatParameter` (METHOD) → `HRESULT` — §5.18.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eSelect` | `E_PressureControlParam` | The selection of the parameter. |
| `fValue` | `LREAL` | The value with which the parameter is to be defined. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetFloatParameter` | `HRESULT` | See below |

**Description:** This method is used to define LREAL parameters of the controller. For details see E_PressureControlParam [} 170].

#### Core functions › PressureControl

##### `PressureControl.Supervisor` (FUNCTION_BLOCK) — §5.18.2

**Description:** This Corefunction organizes the pressure control of the axis and provides the infrastructure interfaces. This is reflected in an interface for a pressure sensor and the switching function for pressure switchover.

#### Core functions

##### `Ptp` (FUNCTION_BLOCK) — §5.19

**Description:** This core function is used to perform multi-segment movements with the possibility to switch to torque or constant output clamping at the end.

#### Core functions › Ptp › ClampingPoint

##### `AsStruct()` (METHOD) → `ST_LookUpClamping` — §5.19.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the interpolation point in the table. |
| `refWrite` | `REFERENCE TO` | Reference to a variable that is to be written to the lookup ST_LookUpClamping [} 178] table. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AsStruct` | `ST_LookUpClamping [} 178]` | If the method is used for reading, the requested point is returned. |

**Description:** A clamping point of the lookup table can be retrieved or written as a structure using this method.

#### Core functions › Ptp

##### `DoMove()` (METHOD) → `HRESULT` — §5.19.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge triggers the command. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoMove` | `HRESULT` | See below |

**Description:** The PTP movement is triggered.

#### Core functions › Ptp › PtpPoint

##### `AsStruct()` (METHOD) → `ST_LookUpPtpPoint` — §5.19.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the interpolation point in the table. |
| `refWrite` | `REFERENCE TO` | Reference to a variable that is to be written to the lookup |
| `ST_LookUpPtpPoint [} 184]` | `table.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AsStruct` | `ST_LookUpPtpPoint [} 184] If the method is used for reading, the requested point is returned.` | — |

**Description:** A point in the lookup table can be retrieved or written as a structure using this method.

#### Core functions › PtpLookUp

##### `GetPoint()` (METHOD) → `HRESULT` — §5.20.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the point. Permissible range 1 ... |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetPoint` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A point from the lookup table is read back.

##### `ReadMaster()` (METHOD) → `HRESULT` — §5.20.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fSlave` | `LREAL` | The specified slave value. |
| `fMaster` | `REFERENCE TO LREAL` | A reference to the variable to be updated with the master position. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReadMaster` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** This method finds a master value that corresponds to the specified slave value.

##### `SetPoint()` (METHOD) → `HRESULT` — §5.20.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the point. Permissible range 1 … NumberOfPoints. |
| `stPoint` | `REFERENCE TO` | A reference to the variable to be updated with the point data. ST_LookUpPtpPoint [} 184] |
| `bForce` | `BOOL` | If the value is TRUE, the segment is updated even when the axis |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetPoint` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** A point of the lookup table is defined.

##### `UpdatePosition()` (METHOD) → `HRESULT` — §5.20.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the point. Permissible range 1 … NumberOfPoints. |
| `fPosition` | `LREAL` | The value to be used for the update. |
| `bSwap` | `BOOL` | With bSwap=TRUE the direction-related enables-(PositiveOnly, NegativeOnly) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdatePosition` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** In some use cases, the target position of the segments must be modified after definition. This method is used for transforming axes, for example.

#### Core functions

##### `Reset` (FUNCTION_BLOCK) — §5.21

**Description:** This core function resets the error state of an axis in a controlled manner. The DoReset() method can be accessed directly via the motion interface I_MotionBase and the interfaces inherited from it.

##### `SetPosition` (FUNCTION_BLOCK) — §5.22

**Description:** This Corefunction is used to change the actual position without physically moving the axis. It updates the offset of the position encoder function.

#### Core functions › SetPosition

##### `DoSetPosition()` (METHOD) → `HRESULT` — §5.22.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the core function. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoSetPosition` | `HRESULT` | See below |

**Description:** This method triggers the core function.

##### `SetParameter()` (METHOD) → `HRESULT` — §5.22.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fPosition` | `LREAL` | The position value to be used is defined here. |
| `bRelative` | `BOOL` | bRelative = TRUE causes the actual position to be changed by an amount |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** Here the new position and the operation mode of the core function are defined.

#### Core functions

##### `Stop` (FUNCTION_BLOCK) — §5.23

**Description:** This Corefunction is used to perform a stop operation using certain parameters.

#### Core functions › Stop

##### `DoStop()` (METHOD) → `HRESULT` — §5.23.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge at this input triggers the stop. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoStop` | `HRESULT` | See below |

**Description:** This method is used to trigger the stop.

#### Core functions

##### `Teach` (FUNCTION_BLOCK) — §5.24

**Description:** This core function is used for referencing analog encoder systems.

##### `ToolAdaption` (FUNCTION_BLOCK) — §5.25

**Description:** This Corefunction is used to convert between axis coordinates and tool working position. An offset and an optional inversion are taken into account.

##### `Transformation` (FUNCTION_BLOCK) — §5.26

**Description:** This core function manages the coupling of the two internal axes of a transforming axis.

#### Core functions › Transformation

##### `TranslateFromDrive()` (METHOD) → `HRESULT` — §5.26.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the asynchronous execution of this method. |
| `fDrivePosition` | `LREAL` | Drive position to be translated. |
| `fLoadPosition` | `REFERENCE Translated load position. TO LREAL` | — |
| `nSlot` | `INT` | The slot to be used for the asynchronous query. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TranslateFromDriv HRESULT` | `Return value with feedback on the success of the execution. e` | — |

**Description:** This method is used to calculate a load position using the transformation table of the load table.

##### `TranslateFromLoad()` (METHOD) → `HRESULT` — §5.26.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the asynchronous execution of this method. |
| `fLoadPosition` | `LREAL` | Load position to be translated. |
| `fDrivePosition` | `REFERENCE Translated drive position. TO LREAL` | — |
| `nSlot` | `INT` | The slot to be used for the asynchronous query. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TranslateFromLoa HRESULT` | `Return value with feedback on the success of the execution. d` | — |

**Description:** This method is used to calculate a drive position using the transformation table of the drive side.

##### `TranslateToDrive()` (METHOD) → `HRESULT` — §5.26.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the asynchronous execution of this method. |
| `fLoadPosition` | `LRELA` | Load position to be translated. |
| `fLoadVelocity` | `LREAL` | Load velocity to be translated. |
| `fDrivePosition` | `REFERENCE Translated load position. TO LREAL` | — |
| `fDriveVelocity` | `REFERENCE Translated load velocity. TO LREAL` | — |
| `nSlot` | `INT` | The slot to be used for the asynchronous query. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TranslateToDrive` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** This method is used to calculate a drive position using the transformation table of the load table.

##### `TranslateToLoad()` (METHOD) → `HRESULT` — §5.26.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge starts the asynchronous execution of this method. |
| `fDrivePosition` | `LREAL` | Drive position to be translated. |
| `fDriveVelocity` | `LREAL` | Drive velocity to be translated. |
| `fLoadPosition` | `REFERENCE Translated load position. TO LREAL` | — |
| `fLoadVelocity` | `REFERENCE Translated load velocity. TO LREAL` | — |
| `nSlot` | `INT` | The slot to be used for the asynchronous query. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TranslateToLoad` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** This method is used to calculate a load position via the transformation table of the drive side.

##### `UpdateToLinLoad()` (METHOD) → `LREAL` — §5.26.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fDrivePosition` | `LREAL` | Current drive position. |
| `fLoadPosition` | `LREAL` | Current load position. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdateToLinLoad LREAL` | `Linearized load position.` | — |

**Description:** This method is used to translate a position in a linearized way, depending on the set LowerLimit property.

#### Core functions

##### `Tuning` (FUNCTION_BLOCK) — §5.27

**Description:** This core function is used to automatically optimize an actuator.

#### Core functions › VelocityFeed

##### `DoFeed()` (METHOD) → `HRESULT` — §5.28.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | A rising edge triggers the core function, while a falling edge causes it to stop |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DoFeed` | `HRESULT` | See below |

**Description:** A rising edge at bEnable triggers the core function, while a falling edge causes it to stop and return to idle.

#### I/O › Base (Abstract)

##### `IoBase` (FUNCTION_BLOCK) — §6.1.1

**Description:** This class is the basis for all I/O elements. It defines a rudimentary basic scope of methods.

#### I/O › Base (Abstract) › IoBase

##### `GetProcessValue()` (METHOD) → `HRESULT` — §6.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `REFERENCE TO` | Reference to a variable to which the input/output value is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetProcessVal HRESULT` | `Return value with feedback on the success of the execution. ue` | — |

**Description:** This method is used to read the terminal-specific input/output value.

##### `SetProcessValue()` (METHOD) → `HRESULT` — §6.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `REFERENCE TO` | Reference to a variable whose value is to be set to the input/output |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetProcessVal HRESULT` | `Return value with feedback on the success of the execution. ue` | — |

**Description:** This method is used to write the terminal-specific input/output value.

#### I/O › Base (Abstract)

##### `IoEc` (FUNCTION_BLOCK) — §6.1.2

**Description:** This class is the basis for EtherCAT-based I/O elements.

#### I/O › Base (Abstract) › IoEc

##### `GetProcessIoStatus()` (METHOD) → `HRESULT` — §6.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bWcState` | `REFERENCE TO BOOL` | Reference to a variable to which the WcState is to be written. |
| `bToggle` | `REFERENCE TO BOOL` | Reference to a variable to which the toggle is to be written. |
| `nInfoDataState` | `REFERENCE TO UINT` | Reference to a variable to which the InfoData state is to be written. |
| `sAdsAddr` | `REFERENCE TO AMSADDR` | Reference to a variable to which the AdsAddr is to be written. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetProcessIoSt HRESULT` | `Return value with feedback on the success of the execution. atus` | — |

**Description:** This method is used to read the EtherCAT status of an I/O element.

##### `SetProcessIoStatus()` (METHOD) → `HRESULT` — §6.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bWcState` | `BOOL` | WcState to be written to the input variable. |
| `bToggle` | `BOOL` | Toggle to be written to the input variable. |
| `nInfoDataState` | `UINT` | InfoData State to be written to the input variable. |
| `sAdsAddr` | `AMSADDR` | AdsAddr to be written to the input variable. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetProcessIoSt HRESULT` | `Return value with feedback on the success of the execution. atus` | — |

**Description:** This method is used to write the EtherCAT status of an I/O element.

#### I/O › Base (Abstract)

##### `IoEcAds` (FUNCTION_BLOCK) — §6.1.3

**Description:** This class is the basis for EtherCAT-based I/O elements with ADS interface.

##### `InputBase` (FUNCTION_BLOCK) — §6.1.4

**Description:** This class is the basis for I/O input elements.

#### I/O › Base (Abstract) › InputBase

##### `GetChannelState()` (METHOD) → `HRESULT` — §6.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDeviceState` | `REFERENCE TO` | Reference to a variable to which the device state is to be written. ST_FlexValue [} 260] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelSta HRESULT` | `Return value with feedback on the success of the execution. te` | — |

**Description:** This method is used to read the channel state of an I/O element.

##### `SetChannelState()` (METHOD) → `HRESULT` — §6.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDeviceState` | `REFERENCE TO` | Variable with the state to be written to the input variable. ST_FlexValue [} 260] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelStat HRESULT` | `Return value with feedback on the success of the execution. e` | — |

**Description:** This method is used to write the channel state of an I/O element.

#### I/O › Base (Abstract)

##### `OutputBase` (FUNCTION_BLOCK) — §6.1.5

**Description:** This class is the basis for I/O output elements.

#### I/O › Base (Abstract) › OutputBase

##### `GetChannelState()` (METHOD) → `HRESULT` — §6.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDeviceState` | `REFERENCE TO` | Reference to a variable to which the device state is to be written. ST_FlexValue [} 260] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChannelSta HRESULT` | `Return value with feedback on the success of the execution. te` | — |

**Description:** This method is used to read the channel state of an I/O element.

##### `GetControlword()` (METHOD) → `HRESULT` — §6.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDriveControlW REFERENCE TO` | `Reference to a variable to which the channel state is to be written.` | — |
| `ord` | `UINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetControlword HRESULT` | `Return value with feedback on the success of the execution.` | — |

**Description:** This method is used to read the control word of an I/O output element.

##### `GetProcessFeedback()` (METHOD) → `HRESULT` — §6.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `REFERENCE TO` | Reference to a variable to which the check value is to be written. ST_FlexValue [} 260] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetProcessFee HRESULT` | `Return value with feedback on the success of the execution. dback` | — |

**Description:** This method is used to read the check signal of an I/O output element.

##### `SetChannelState()` (METHOD) → `HRESULT` — §6.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDeviceState` | `REFERENCE TO` | Variable with the state to be written to the input variable. ST_FlexValue [} 260] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChannelStat HRESULT` | `Return value with feedback on the success of the execution. e` | — |

**Description:** This method is used to write the channel state of an I/O element.

##### `SetControlword()` (METHOD) → `HRESULT` — §6.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDriveControlW UINT` | `Control word to be written to the input variable. ord` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetControlword HRESULT` | `Return value with feedback on the success of the execution.` | — |

**Description:** This method is used to write the control word of an I/O output element.

##### `SetProcessFeedback()` (METHOD) → `HRESULT` — §6.1.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `REFERENCE TO` | Variable with the check signal that is to be written to the input |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetProcessFee HRESULT` | `Return value with feedback on the success of the execution. dback` | — |

**Description:** This method is used to write the check signal of an I/O output element.

#### I/O

##### `Analog` (FUNCTION_BLOCK) — §6.2

**Description:** This class can be used for a 16-bit analog input element.

#### I/O › Analog

##### `InputAnalog16Status` (FUNCTION_BLOCK) — §6.2.2

**Description:** This class can be used for a 16-bit analog input element with additional status information.

#### I/O › Analog › InputAnalog16Status

##### `U_InputAnalog16_Status` (FUNCTION_BLOCK) — §6.2.2.1

**Description:** Union for decoding an analog input status word.

#### I/O › CoE

##### `InputCoE_Polling` (FUNCTION_BLOCK) — §6.3.1

**Description:** This class implements a virtual input for an asynchronous CoE value from an EtherCAT device that is read via ADS. This function can be used for cyclic reading of internal terminal information (such as the internal temperature).

##### `OutputCoE408_P` (FUNCTION_BLOCK) — §6.3.4

**Description:** This class is an extension of the CoE408-based valve with integrated pressure measurement.

#### I/O › Digital

##### `IoDigital` (FUNCTION_BLOCK) — §6.4.1

**Description:** This class is the basis for digital I/O elements.

##### `InputDigital` (FUNCTION_BLOCK) — §6.4.2

**Description:** This class is used for digital input signals.

##### `InputDigitalCam` (FUNCTION_BLOCK) — §6.4.3

**Description:** This class is used for the digital feedback of an FB_AxisHydraulicBase [} 30].

##### `OutputDigital` (FUNCTION_BLOCK) — §6.4.4

**Description:** This class is used for digital output signals.

#### I/O › Serial

##### `InputSsi` (FUNCTION_BLOCK) — §6.5.1

**Description:** This class can be used for an SSI based encoder.

#### I/O › Device (inverter)

##### `InvDeviceBase` (FUNCTION_BLOCK) — §6.6.1

**Description:** This class is the basis for inverter I/O elements.

##### `InvDeviceEc` (FUNCTION_BLOCK) — §6.6.2

**Description:** This class is the basis for inverters with EtherCAT status.

##### `InverterCoE402_CSVM` (FUNCTION_BLOCK) — §6.6.4

**Description:** This class can be used for DS402 inverters in CyclicSynchronousVelocityMode.

##### `InverterCoE402_VM` (FUNCTION_BLOCK) — §6.6.5

**Description:** This class can be used for DS402 inverters in VelocityMode.

##### `InverterAnalog` (FUNCTION_BLOCK) — §6.6.6

**Description:** This class is used for inverters with an analog interface.

#### I/O › LimitingOutput

##### `LimitingOutputCoE` (FUNCTION_BLOCK) — §6.7.1

**Description:** This class is part of the FB_AxisNcBase and transmits the current limit to a DS402-compatible CoE field device, such as an AX8000.

##### `LimitingOutputSoE` (FUNCTION_BLOCK) — §6.7.2

**Description:** This class is part of the FB_AxisNcBase and transmits the current limit to an SoE-capable field device, such as an AX5000.

#### (top level)

##### `FB_MdContainer` (FUNCTION_BLOCK) — §7.1

**Description:** This class is able to write data from a list of components to a file. The file format is binary and secured via a CRC checksum. This automatically makes it more difficult for unauthorized users to manipulate this file.

##### `FB_MdComponent` (FUNCTION_BLOCK) — §7.2

**Description:** The class represents the basis of a machine data component.

#### FB_MdComponent

##### `MdNextParameter()` (METHOD) — §7.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stMdata` | `REFERENCE TO` | Reference to a variable to which the component should write the ST_Mdata [} 267] information of the parameter. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MdNextParame BOOL` | `TRUE = stMdata contains a valid parameter.` | — |
| `ter` | `FALSE = No further data available.` | — |

**Description:** This method is used to get the parameters from the application in order to write them to the file.

##### `MdSetParameter()` (METHOD) — §7.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stMdata` | `REFERENCE TO` | Reference to a variable that contains the parameter loaded from the ST_Mdata [} 267] file. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MdSetParamet BOOL` | `TRUE = Parameter is applied.` | — |
| `er` | `FALSE = Parameter cannot be used.` | — |

**Description:** This method is used to set the parameters of the application that are read from the file.

#### Filter

##### `FB_FilterBase` (FUNCTION_BLOCK) — §8.1.1

**Description:** Base class for filter function blocks for filtering floating point numbers.

#### Filter › FB_FilterBase

##### `CyclicUpdate` (METHOD) → `LREAL` — §8.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bForce` | `BOOL` | The filtered value is updated with the input value regardless of filter |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CyclicUpdate` | `LREAL` | The filtered value. |

**Description:** Cyclic transfer of a new value.

#### Filter

##### `FB_FilterSlewRateLimit` (FUNCTION_BLOCK) — §8.1.3

**Description:** This class derived from FB_FilterBase implements a filter that limits the rate of rise (i.e. the ramp rate).

#### HRESULT

##### `F_SucceededHr` (FUNCTION) → `BOOL` — §8.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Source` | `HRESULT` | Error code to be checked. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Destination` | `HRESULT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_SucceededHr BOOL` | `TRUE if the check was successful.` | — |

**Description:** This method checks a HRESULT value for success and copies the result to a transferred variable if the check fails.

##### `F_HresultCreate` (FUNCTION) → `HRESULT` — §8.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFeedback` | `UDINT` | Error code to be integrated into HRESULT. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_HresultCreat HRESULT` | `Created HRESULT value. e` | — |

**Description:** This method is used to create a HRESULT value. Whether the value is interpreted as a success depends on the input parameter bSuccess.

##### `F_HresultFailure` (FUNCTION) → `HRESULT` — §8.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFeedback` | `UDINT` | Error code to be integrated into the HRESULT. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_HresultFailur HRESULT` | `Generated HRESULT value. e` | — |

**Description:** This method is used to generate a HRESULT value that is interpreted as an error.

##### `F_HresultSuccess` (FUNCTION) → `HRESULT` — §8.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFeedback` | `UDINT` | Error code to be integrated into the HRESULT. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `F_HresultSucc HRESULT` | `Generated HRESULT value. ess` | — |

**Description:** This method is used to generate a HRESULT value that is interpreted as a success.

#### Messages

##### `FB_Message` (FUNCTION_BLOCK) — §8.3.1

**Description:** This class is able to send messages via the EventLogger.

#### Messages › FB_Message

##### `SetTraceLevel()` (METHOD) → `HRESULT` — §8.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bOwnTraceLevel BOOL` | `Activates the fixed severity level.` | — |
| `eTraceLevel` | `TcEventSeverity` | Severity level |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetTraceLevel HRESULT` | `Return value with feedback on the success of the execution.` | — |

**Description:** This method fixes the severity of the events thrown by this instance.

#### Messages

##### `FB_XyzMessage` (FUNCTION_BLOCK) — §8.3.2

**Description:** These classes take into account various static parameters of the library in order to report messages in the

#### Messages › FB_XyzMessage

##### `SendMessage()` (METHOD) → `HRESULT` — §8.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEventEntry` | `TcEventEntry` | Message to be generated and sent. |
| `sString` | `STRING(255)` | Optional additional information as text. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendMessage HRESULT` | `Return value with feedback on the success of the execution.` | — |

**Description:** This method sends a message to the EventLogger.

#### Messages

##### `FB_MessageBase` (FUNCTION_BLOCK) — §8.3.3

**Description:** This class is the basic scope of the EventLogger interface of most axes and core functions.

#### Services

##### `FB_CoEService` (FUNCTION_BLOCK) — §8.4.1

**Description:** This class provides a read and write service for CoE-addressable EtherCAT devices. This can be used to read and/or write internal information from a field device, for example.

##### `FB_CoEParameter` (FUNCTION_BLOCK) — §8.4.2

**Description:** This class bundles the information of a CoE value in a field device.

##### `FB_CheckIoMapping` (FUNCTION_BLOCK) — §8.4.3

**Description:** This class checks for an I/O element that supports I_InputBase [} 218] or I_OutputBase [} 221] whether the ProcessValue (or, in the case of outputs, the ProcessFeedback) is cyclically synchronized via a TwinCAT mapping (e.g. with I/O).

#### Simulation › General simulation components

##### `FB_SimAxCommon` (FUNCTION_BLOCK) — §8.5.1.1

**Description:** This class is used as a common platform for carrying out various types of axis simulations. It is not intended to instantiate objects.

#### Simulation › General simulation components › FB_SimAxCommon

##### `GoFaultState()` (METHOD) → `BOOL` — §8.5.1.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GoFaultState` | `BOOL` | Reserved for future implementation |

**Description:** This method can be used to simulate an error situation on the simulated axis.

#### Simulation › General simulation components

##### `FB_Noise` (FUNCTION_BLOCK) — §8.5.1.2

**Description:** This FB is used to generate a pseudo-random signal that resembles a white noise disturbance. Noise signals may be used to simulate a common problem of analog sensors and interfaces.

#### Simulation › General simulation components › FB_Noise

##### `Cyclic` (METHOD) → `LREAL` — §8.5.1.2.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Cyclic` | `LREAL` | The new value of the noise signal |

**Description:** Method to be called cyclically to generate a new value.

##### `SetLineNoiseLevel` (METHOD) — §8.5.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fLevel` | `LREAL` | Level of an influence by a supply network. |

**Description:** Determination of the level of an influence by a supply network.

##### `SetSparkNoiseLevel` (METHOD) — §8.5.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fLevel` | `LREAL` | Level of an influence by static discharges. |

**Description:** Determination of the level of an influence by static discharges.

##### `SetWhiteNoiseLevel` (METHOD) — §8.5.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fLevel` | `LREAL` | Level of the noise signal. |

**Description:** Setting the level of the noise signal.

#### Simulation › Simulation of an EtherCAT based servo drive axis

##### `FB_SimCoE402_Servo` (FUNCTION_BLOCK) — §8.5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NcAdapt` | `FB_CoE402_ServoAdapt` | — |

**Description:** This class implements a simulation of a CoE402 servo drive. The FB offers mapping elements for CoE interfaces of servo drives. Mapping is done in the same way as for the real drive unit.

#### Simulation › Simulation of an EtherCAT based servo drive axis › FB_SimCoE402_Servo

##### `Gear()` (METHOD) → `I_SimAxGear` — §8.5.2.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GearBox` | `I_SimAxGear` | Gear interface on the axis to configure a chaining of gearboxes. |

**Description:** This method is used to start the chaining of gear factors.

##### `GearBox()` (METHOD) → `I_SimAxGear` — §8.5.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fRatioIn` | `LREAL` | Number of teeth of the gearbox input. |
| `fRatioOut` | `LREAL` | Number of teeth of the gearbox output. Mathematical relationship of the inputs |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GearBox` | `I_SimAxGear` | Gear interface on the axis to configure a chaining of gearboxes. |

**Description:** This method can be used to configure a rotary transmission factor of the gearbox.

##### `GearRackNPinion()` (METHOD) — §8.5.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fPinionRadius` | `LREAL` | Effective radius [m] of the pinion on the rack |

**Description:** This method is used to finalize a chaining of gears by means of a rack.

##### `GearScrew()` (METHOD) — §8.5.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fRadius` | `LREAL` | Radius of the spindle [m] |
| `fPitch` | `LREAL` | Pitch of the spindle [m] |

**Description:** This method is used to finalize a chaining of gears by means of a spindle.

#### Simulation › Simulation of an EtherCAT based servo drive axis › FB_SimTorqueLimitingCoE402_Servo

##### `ConnectToSim()` (METHOD) → `HRESULT` — §8.5.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipSim` | `I_SimCoE402_Servo` | The simulation block for the simulation of a CoE402 servo. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ConnectToSim HRESULT` | `Return value with feedback on the success of the execution.` | — |

**Description:** This method establishes a connection between the torque-limiting FB and the simulated servo drive. This connection replaces the EtherCAT communication with a real CoE402-based servo drive.

#### Simulation › Simulation of an inverter axis

##### `FB_SimCoE402_Inverter` (FUNCTION_BLOCK) — §8.5.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `EtC_Device` | `FB_EtC_SimDevice` | — |

**Description:** This simulation corresponds to an inverter axis and can be used to replace a real inverter drive by providing compatible mapping elements and emulating behavior very similar to that of a real axis.

##### `FB_SimCoE402_Inverter_VM` (FUNCTION_BLOCK) — §8.5.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IO` | `FB_CoE402_InverterAdapt` | — |

**Description:** This simulation corresponds to an inverter axis and can be used to replace a real inverter drive by providing compatible mapping elements and emulating behavior very similar to that of a real axis.

##### `FB_SimCoE402_Inverter_CSVM` (FUNCTION_BLOCK) — §8.5.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Outputs` | `ST_CoE402_CSVM_Outputs` | — |
| `Inputs` | `ST_CoE402_CSVM_Inputs` | — |

**Description:** This simulation corresponds to an inverter axis and can be used to replace a real inverter drive by providing compatible mapping elements and emulating behavior very similar to that of a real axis.

##### `FB_SimAnalog_Inverter` (FUNCTION_BLOCK) — §8.5.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Velocity` | `FB_SimAnalogOutputElTerminal4;   // mandatory` | — |
| `Actuals` | `FB_SimAnalogInputElTerminal4` | — |
| `PowerEnable` | `BOOL;   // mandatory` | — |
| `PositiveEnable` | `BOOL;    // optional: will be TRUE if no mapping` | — |
| `NegativeEnable` | `BOOL;    // optional: will be TRUE if no mapping` | — |
| `Reset` | `BOOL;   // mandatory` | — |
| `EnableFeedback` | `BOOL` | — |
| `Fault` | `BOOL` | — |

**Description:** This simulation corresponds to an inverter axis and can be used to replace a real inverter drive by providing compatible mapping elements and emulating behavior very similar to that of a real axis.

#### Simulation › I/O Simulation containers

##### `Simulation of an analog input terminal` (FUNCTION_BLOCK) — §8.5.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AnalogValue1` | `INT` | IN 0 The simulated input value of the 1st channel. |
| `AnalogValue2` | `INT` | IN 0 The simulated input value of the 2nd channel. |
| `AnalogValue3` | `INT` | IN 0 The simulated input value of the 3rd channel. |
| `AnalogValue4` | `INT` | IN 0 The simulated input value of the 4th channel. |
| `WcState` | `BOOL` | OUT FALSE The simulated Working Counter State. |
| `Toggle` | `BOOL` | OUT FALSE The simulated toggle bit. |
| `InfoDataState` | `UINT` | OUT 8 The simulated Device State. |
| `AdsAddr` | `AMSADDR` | OUT The simulated ADS address. |

**Description:** Simulation of a 4-channel analog input terminal. This FB only provides a mapping interface and does not contain an implementation.

##### `Simulation of an analog output terminal` (FUNCTION_BLOCK) — §8.5.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Output1` | `INT` | IN 0 The simulated output value of the 1st |

**Description:** Simulation of a 4-channel analog output terminal. This FB only provides a mapping interface and does not contain an implementation.

##### `Simulation of a CoE402 servo drive` (FUNCTION_BLOCK) — §8.5.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NcDriveIn` | `NCDRIVESTRUCT_IN2` | OUT For the DriveIn interface of the NC. |
| `NcDriveOut` | `NCDRIVESTRUCT_OUT2` | IN For the DriveOut interface of the NC. |
| `NcEncIn` | `NCENCODERSTRUCT_IN2B` | OUT For the EncoderIn interface of the NC. |
| `NcEncOut` | `NCENCODERSTRUCT_OUT2` | IN For the EncoderOut interface of the NC. |

**Description:** Simulation of a CoE408 servo drive. This FB only provides a mapping interface and does not contain an implementation.

##### `Simulation of an SSI input terminal` (FUNCTION_BLOCK) — §8.5.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `AnalogValue1` | `INT` | IN 0 The simulated input value of the 1st channel. |
| `AnalogValue2` | `INT` | IN 0 The simulated input value of the 2nd channel. |
| `AnalogValue3` | `INT` | IN 0 The simulated input value of the 3rd channel. |
| `AnalogValue4` | `INT` | IN 0 The simulated input value of the 4th channel. |
| `WcState` | `BOOL` | OUT FALSE The simulated Working Counter State. |
| `Toggle` | `BOOL` | OUT FALSE The simulated toggle bit. |
| `InfoDataState` | `UINT` | OUT 8 The simulated Device State. |
| `AdsAddr` | `AMSADDR` | OUT The simulated ADS address. |

**Description:** Simulation of a 1-channel SSI input terminal. This FB only provides a mapping interface and does not contain an implementation.

#### Simulation › Components of the hydraulic simulation

##### `FB_SimHydAx_Standard` (FUNCTION_BLOCK) — §8.5.5.1

**Description:** This FB is a simplified simulation of a universal hydraulic axis. A number of typical effects are supported.

##### `FB_SimValve` (FUNCTION_BLOCK) — §8.5.5.2

**Description:** This simplified simulation of an analog proportional or servo valve is used as part of a simulated hydraulic axis. It supports a range of typical variants of non-linearities. There is no dynamic behavior.

##### `FB_SimCylinder` (FUNCTION_BLOCK) — §8.5.5.3

**Description:** This simplified simulation of a universal cylinder is used as part of a simulated hydraulic axis. A number of typical effects are supported.

##### `FB_SimPressureTransducer` (FUNCTION_BLOCK) — §8.5.5.4

**Description:** This simplified simulation of a universal pressure transducer is used as component of a simulated hydraulic axis. A number of typical effects are supported.

##### `FB_SimUniversalEncoder` (FUNCTION_BLOCK) — §8.5.5.5

**Description:** This simplified simulation of a universal encoder is used as part of a simulated hydraulic axis. A number of typical effects are supported.

#### Simulation

##### `SimActuator` (FUNCTION_BLOCK) — §8.5.6

**Description:** This class simulates a digital linear actuator.

##### `Injection pressure simulation` (FUNCTION_BLOCK) — §8.5.7

**Description:** This class implements a rudimentary simulation of the melt pressure of an injection process. For this purpose, a pressure sensor is simulated, which is synchronized into the controller via an analog interface (preferably FB_InputAnalog16 [} 227]).

#### Debug

##### `FB_CheckDemoMode` (FUNCTION_BLOCK) — §8.6.1

**Description:** A FB of this type can be instantiated and used by the PLC application to scan the fieldbus configuration. It reports on fieldbuses with special features such as real-time performance or USB support.

##### `FB_ChangeDetector` (FUNCTION_BLOCK) — §8.6.2

**Description:** This class can recognize sudden changes in a value.

##### `FB_CoreDebug` (FUNCTION_BLOCK) — §8.6.3

**Description:** This class is used for better debugging of the core function chain.

#### (top level)

##### `PlcMcManager Support` (FUNCTION_BLOCK) — §8.7

**Description:** This class establishes the connection between TwinCAT 3 Plastic Technology Functions hydraulic axes and the PlcMcManager.

##### `ExternalHydAxisLibRef` (FUNCTION_BLOCK) — §8.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stAxisRef` | `AXIS_REF_BkPlcMc` | Reference to the hydraulic axis. Properties |
| `NextHydAxis` | `I_ExternalHydAxisLi Get, Set` | Interface to another hydraulic axis. bRef |
| `NextHydAxisChainl INT` | `Get` | Length of the chain of hydraulic axes. ength |
| `UseDatFile` | `BOOL` | Get, Set A TRUE here signals that the axis loads its |

**Description:** This class represents a standard hydraulic axis of the Tc2_Hydraulic library (without using an FB_AxisHydraulicBase). This allows standard hydraulic axes and TF8560-based hydraulic axes to communicate with the PlcMcManager in one application.

#### Camming

##### `SetGuidingValue()` (METHOD) → `HRESULT` — §10.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fGuidingValue` | `LREAL` | The guiding value that will be used in the next cycle. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetGuidingValue` | `HRESULT` | See below |

**Description:** Alternative: Camming [} 90].Pozna|Velo.GuidingValue

##### `SetLookupInterface()` (METHOD) → `HRESULT` — §10.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iLookUp` | `I_CammingLookUp [} 94] The camming table to use.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetLookupInterface` | `HRESULT` | See below |

**Description:** Alternative: Camming [} 90].Pozna|Velo.LookUp

#### PressureControl

##### `PID.EnableSwitchOver()` (METHOD) → `HRESULT` — §10.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | The enable for automatic activation. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EnableSwitchOver` | `HRESULT` | See below |

**Description:** Alternative: PID [} 162].AllowSwitchOver

##### `PID.GetActual()` (METHOD) → `HRESULT` — §10.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fActual` | `REFERENCE TO LREAL` | A reference to the variable to be updated with the actual value. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetActual` | `HRESULT` | See below |

**Description:** Alternative: PID [} 162].Actual

##### `PID.GetParams()` (METHOD) → `HRESULT` — §10.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iParameters` | `REFERENCE TO` | A reference to the variable to be updated with the |
| `I_PressureControlParams_PID` | `interface.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetParams` | `HRESULT` | See below |

**Description:** Alternative: PID [} 162].Parameters [} 166]

##### `PID.SetParams()` (METHOD) → `HRESULT` — §10.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iParameters` | `I_PressureControlParams_PID` | An interface to the parameter set. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParams` | `HRESULT` | See below |

**Description:** Alternative: PID [} 162].Parameters [} 166]

##### `PID.Setpoint()` (METHOD) → `HRESULT` — §10.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fValue` | `LREAL` | The setpoint. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Setpoint` | `HRESULT` | See below |

**Description:** Alternative: PID [} 162].Target

##### `FB_ProcessHandlerBase` (FUNCTION_BLOCK) — §10.2.7

**Description:** Alternative: PressureControl.Supervisor [} 171]

#### PressureControl › FB_ProcessHandlerBase

##### `GetControlParameter` (METHOD) → `I_PressureControlParams_PID` — §10.2.7.1

**Return type:** `I_PressureControlParams_PID`

**Description:** This method can be used to get access to the parameter container of the axis pressure controller.

##### `GetProcessValue` (METHOD) → `HRESULT` — §10.2.7.2

**Return type:** `HRESULT`

**Description:** This method is used to update the FB with a range of actual values (pressure, position, state) of the axis.

##### `GetSwitchEnable` (METHOD) → `HRESULT` — §10.2.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchOnPressure` | `REFERENCE TO BOOL` | A reference to the variable to be updated with the |

**Description:** This method is used to read back the rules for the switchover.

##### `GetSwitchParameter` (METHOD) → `HRESULT` — §10.2.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eSelect` | `E_SwitchoverParameter [} 336] The selection of the parameter.` | — |
| `fValue` | `REFERENCE TO LREAL` | A reference to the variable to be updated with the parameter. |

**Description:** This method may be used to read back parameters of the switchover rules.

##### `SetControlParameter` (METHOD) → `HRESULT` — §10.2.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iParams` | `I_PressureControlParams_PID` | An interface to a FB with a parameter set for a |

**Description:** This method can be used to assign a parameter container to the axis pressure controller.

##### `SetSwitchEnable` (METHOD) → `HRESULT` — §10.2.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSwitchOnPressure` | `BOOL` | A TRUE enables the switchover by exceeding the pressure threshold. |
| `bSwitchOnTravel` | `BOOL` | A TRUE enables the switchover by falling below a position threshold. |
| `bSwitchOnTime` | `BOOL` | A TRUE enables the switchover by reaching a time threshold. |

**Description:** This method is used to define the rules for the switchover. See below for more information.

##### `SetSwitchParameter` (METHOD) → `HRESULT` — §10.2.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eSelect` | `E_SwitchoverParameter [} 336] The selection of the parameter.` | — |
| `fValue` | `LREAL` | The value with which the parameter should be updated. |

**Description:** This method may be used to define parameters of the switchover rules.

#### PressureControl

##### `FB_ReadProcessValue` (FUNCTION_BLOCK) — §10.2.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fActualValue` | `LREAL` | The current actual value. Properties |
| `Offset` | `LREAL` | Get, Set The offset taken into account when calculating the actual |

**Description:** Alternative: PressureControl.Supervisor [} 171]

#### PressureControl › FB_ReadProcessValue

##### `Cyclic` (METHOD) — §10.2.8.1

**Description:** This method must be called by the application once per cycle. It checks the EtherCAT connection and updates the ProcessValue with a filtered actual value.

##### `DoReset` (METHOD) → `HRESULT` — §10.2.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | — |

**Description:** A call of this method with bExecute=TRUE resets the connected sensor interface and clears all local errors.

#### Ptp

##### `CheckPoint()` (METHOD) → `HRESULT` — §10.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stPoint` | `ST_LookUpPtpPoint [} 184]` | The point to be checked. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CheckPoint` | `HRESULT` | Return value with feedback on the validity of the point. |

**Description:** Alternative: / (INTERNAL)

##### `GetClampPoint()` (METHOD) → `HRESULT` — §10.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the point. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetClampPoint` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** Alternative: Ptp [} 173].ClampingPoint [} 176].Fetch(x)

##### `GetPoint()` (METHOD) → `HRESULT` — §10.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the interpolation point. |
| `stPoint` | `REFERENCE TO` | A reference to the variable that is to be updated with the |
| `ST_LookUpPtpPoint [} 184]` | `requested point.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetPoint` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** Alternative: Ptp [} 173].Point [} 182].Fetch(x)

##### `GetUpdatedPoint()` (METHOD) → `HRESULT` — §10.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the segment. |
| `stPoint` | `REFERENCE TO` | A reference to the variable that is to be updated with the |
| `ST_LookUpPtpPoint [} 184]` | `requested point.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetUpdatedPoint` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** Alternative: / (INTERNAL)

##### `SetClampPoint()` (METHOD) → `HRESULT` — §10.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the clamping point. |
| `stClampPoint` | `ST_LookUpClamping [} 178] The clamping point to be used.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetClampPoint` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** Alternative: Ptp [} 173].ClampingPoint [} 176].Set(x)

##### `SetPoint()` (METHOD) → `HRESULT` — §10.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the segment. |
| `stPoint` | `REFERENCE TO` | A reference to the variable that is to be updated. ST_LookUpPtpPoint [} 184] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetPoint` | `BOOL` | Return value with feedback on the success of the execution. |

**Description:** Alternative: Ptp [} 173].Point [} 182].Set(x)

##### `UpdatePosition()` (METHOD) → `HRESULT` — §10.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIdx` | `INT` | The index of the point to be updated. |
| `fPosition` | `LREAL` | The new target position. |
| `bSwap` | `BOOL` | With bSwap=TRUE the direction related enables (PositiveOnly, NegativeOnly) are |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `UpdatePosition` | `BOOL` | Return value with feedback on the success of the execution. |

**Description:** Alternative: / (INTERNAL)

#### Stop

##### `SetParameter()` (METHOD) → `HRESULT` — §10.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fDeceleration` | `LREAL` | The deceleration to be used. |
| `fJerk` | `LREAL` | The jerk to be used. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetParameter` | `HRESULT` | Return value with feedback on the success of the execution. |

**Description:** Alternative: Stop [} 194].Deceleration & Stop [} 194].Jerk

#### (top level)

##### `TeachPosition` (FUNCTION_BLOCK) — §10.5

**Description:** Alternative: Teach [} 196]

##### `TeachUpdate` (FUNCTION_BLOCK) — §10.6

**Description:** Alternative: Teach [} 196]

##### `TorqueLimiting` (FUNCTION_BLOCK) — §10.7

**Description:** Alternative: Limiting [} 145]
