# Tc3_DALI

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_DALI_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 405
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** DALI-2: dedicated serial master service.

---

#### Error while reading the variable minLevel [} 163]. › POUs › Applications › Part 102 (control gears) › Addressing

##### `FB_DALI102Addressing` — §4.1.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block addresses the DALI control gears at random. The user has no influence on which DALI control gear is assigned which short address. Short addresses are allocated in ascending order.

#### (top level)

##### `nOldAddress and nNewAddress have the same values` — §6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChangeAddressList` | `ARRAY [0..63] OF ST_DALIChangeAddressList` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

#### Error while reading the variable minLevel [} 163]. › POUs › Applications › Part 102 (control gears) › Power control

##### `FB_DALI102ConstantLightControl` — §4.1.1.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | A positive edge at this input writes the values of all |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The FB_DALI102ConstantLightControl function block provides the basic functions for implementing constant light regulation.

##### `FB_DALI102Dimmer1Switch` — §4.1.1.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | A positive edge at this input writes the values of all |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI102Dimmer1Switch provides the main functions for implementing a push- dimmer.

##### `FB_DALI102Dimmer2Switch` — §4.1.1.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI102Dimmer2Switch provides the main functions for implementing a push- dimmer.

#### (top level)

##### `Error while reading the variable physicalMinLevel [} 167].` — §15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrDALIDeviceSettings` | `ARRAY [0..63] of ST_DALIControlGearSettings` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

##### `Lamp failure (see lampFailure [} 166])` — §1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL := FALSE` | — |
| `bStartVerification` | `BOOL` | — |
| `tCycle` | `TIME := T#1M` | — |
| `nOptions` | `DWORD := GVL.cOpticalFeedback OR GVL.cCheckControlGearFailure OR GVL.cCheckL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** Some error states can only be detected when the lamp is switched on.

##### `Error while writing the variable scene0-scene15 [} 167].` — §11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrDALIDeviceSettings` | `ARRAY [0..63] of ST_DALIControlGearSettings` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

#### Error while reading the variable minLevel [} 163]. › POUs › Applications › Part 102 (control gears) › Settings

##### `FB_DALI102ReadMemoryBank` — §4.1.1.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The specified memory bank of the DALI control gear is read (see Memory banks [} 16]). The memory bank is specified by the parameter nMemoryBank. The range to be read can be limited with the help of the parameters nSubRangeStart and nSubRangeEnd.

#### Error while reading the variable minLevel [} 163]. › POUs › Applications › Part 103 (control devices) › Addressing

##### `FB_DALI103Addressing` — §4.1.1.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block addresses the DALI control devices at random. The user has no influence on which control device is assigned which short address. Short addresses are allocated in ascending order.

#### (top level)

##### `nOldAddress and nNewAddress have the same values` — §6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aChangeAddressList` | `ARRAY [0..63] OF ST_DALIChangeAddressList` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

#### Error while reading the variable minLevel [} 163]. › POUs › Applications › Part 103 (control devices) › Settings

##### `FB_DALI103ReadMemoryBank` — §4.1.1.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The specified memory bank of the DALI control device is read (see Memory banks [} 16]). The memory bank is specified by the parameter nMemoryBank. The range to be read can be limited with the help of the parameters nSubRangeStart and nSubRangeEnd.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 102 (control gears) › Application extended commands

##### `FB_DALI102QueryExtendedVersionNumber` — §4.1.2.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the version number for the implemented, application extended commands from the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 102 (control gears) › Configuration commands

##### `FB_DALI102AddToGroup` — §4.1.2.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block adds the DALI control gear to the nGroup group.

##### `FB_DALI102IdentifyDevice` — §4.1.2.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block starts the identification routine for the DALI control gear.

##### `FB_DALI102RemoveFromGroup` — §4.1.2.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block removes the DALI control gear from the group nGroup.

##### `FB_DALI102RemoveFromScene` — §4.1.2.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block removes the DALI control gear from the scene nScene.

##### `FB_DALI102Reset` — §4.1.2.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block resets all variables of the DALI control gear to their reset values.

##### `FB_DALI102ResetMemoryBank` — §4.1.2.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The values of the selected memory bank of the DALI control gear are set to their respective default values by the function block (see Memory banks [} 16]).

##### `FB_DALI102SavePersistentVariables` — §4.1.2.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block immediately triggers the saving of variables in the persistent memory by the DALI command SAVE PRESISTENT VARIABLES.

##### `FB_DALI102SetExtendedFadeTime` — §4.1.2.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | — |
| `nAddress` | `BYTE` | — |
| `eAddressType` | `E_DALIAddressType := E_DALIAddressType.Short` | — |
| `eCommandPriority` | `E_DALICommandPriority := E_DALICommandPriority.MiddleLow` | — |
| `eExtendedFadeTimeBase` | `E_DALIExtendedFadeTimeBase := E_DALIExtendedFadeTimeBase.Base01` | — |
| `eExtendedFadeTimeMultiplier` | `E_DALIExtendedFadeTimeMultiplier := E_DALIExtendedFadeTimeMultiplier` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block writes the values eExtendedFadeTimeBase and eExtendedFadeTimeMultiplier to the variables extendedFadeTimeBase [} 165] and extendedFadeTimeMultiplier [} 165].

##### `FB_DALI102SetFadeRate` — §4.1.2.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block sets the fadeRate [} 163] to the value eFadeRate.

##### `FB_DALI102SetFadeTime` — §4.1.2.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block sets the fadeTime [} 164] to the value eFadeTime.

##### `FB_DALI102SetMaxLevel` — §4.1.2.1.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nMaxLevel in the variable maxLevel [} 163] of the DALI control gear.

##### `FB_DALI102SetMinLevel` — §4.1.2.1.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nMinLevel in the variable minLevel [} 163] of the DALI control gear.

##### `FB_DALI102SetPowerOnLevel` — §4.1.2.1.2.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nPowerOnLevel in the variable powerOnLevel [} 163] of the DALI control gear.

##### `FB_DALI102SetScene` — §4.1.2.1.2.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nSceneLevel in the specified scene nScene of the DALI control gear.

##### `FB_DALI102SetShortAddress` — §4.1.2.1.2.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the new short address nNewShortAddress in the shortAddress [} 166] variable of the DALI control gear.

##### `FB_DALI102SetSystemFailureLevel` — §4.1.2.1.2.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL 84` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nSystemFailureLevel in the variable systemFailureLevel [} 163] of the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 102 (control gears) › Control commands

##### `FB_DALI102DirectArcPowerControl` — §4.1.2.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block switches the DALI control gear to the default output value nArcPowerLevel.

##### `FB_DALI102Down` — §4.1.2.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output value of the DALI control gear is reduced over the specified period of 200 ms.

##### `FB_DALI102GoToLastActiveLevel` — §4.1.2.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** If the output value of the DALI control gear is 0, it is set to the last active output value (minLevel...maxLevel) that was used before switching off. This function block is used to reset DALI control gears that have been switched off to their last output value (switched on).

##### `FB_DALI102GoToScene` — §4.1.2.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output value of the DALI control gear is set to the value stored in nScene.

##### `FB_DALI102Off` — §4.1.2.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The DALI control gear is switched off immediately.

##### `FB_DALI102OnAndStepUp` — §4.1.2.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output value of the DALI control gear is increased by one step.

##### `FB_DALI102RecallMaxLevel` — §4.1.2.1.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output value of the DALI control gear is set to maxLevel [} 163].

##### `FB_DALI102RecallMinLevel` — §4.1.2.1.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output value of the DALI control gear is set to minLevel [} 163] value.

##### `FB_DALI102StepDown` — §4.1.2.1.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output value of the DALI control gear is reduced by one step.

##### `FB_DALI102StepDownAndOff` — §4.1.2.1.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output value of the DALI control gear is reduced by one step.

##### `FB_DALI102StepUp` — §4.1.2.1.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output value of the DALI control gear is increased by one step.

##### `FB_DALI102Up` — §4.1.2.1.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output value of the DALI control gear is increased over the specified period of 200 ms.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 102 (control gears) › Query commands

##### `FB_DALI102QueryActualLevel` — §4.1.2.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL 106` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL 108` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the current output value of the DALI control gear (see actualLevel [} 163]).

##### `FB_DALI102QueryControlGearFailure` — §4.1.2.1.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block checks whether there is a defect in one or more DALI control gears.

##### `FB_DALI102QueryControlGearPresent` — §4.1.2.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates whether the required DALI control gear can be reached via its short address.

##### `FB_DALI102QueryDeviceTypes` — §4.1.2.1.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block returns a list of all device types supported by the DALI control gear.

##### `FB_DALI102QueryExtendedFadeTime` — §4.1.2.1.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block reads the variables extendedFadeTimeBase [} 165] and extendedFadeTimeMultiplier [} 165] from the DALI control gear. The Extended Fade Time (100 ms to 16 min) can be calculated from the product of the two variables.

##### `FB_DALI102QueryFadeTimeFadeRate` — §4.1.2.1.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variables fadeTime [} 164] and fadeRate [} 163] from the DALI control gear.

##### `FB_DALI102QueryGroups` — §4.1.2.1.4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable gearGroups [} 167] from the DALI control gear.

##### `FB_DALI102QueryLightSourceTypes` — §4.1.2.1.4.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block returns a list of all light source types supported by the DALI control gear.

##### `FB_DALI102QueryLampFailure` — §4.1.2.1.4.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block checks whether there is a defect in the lamp of a DALI control gear.

##### `FB_DALI102QueryMaxLevel` — §4.1.2.1.4.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable maxLevel [} 163] from the DALI control gear.

##### `FB_DALI102QueryMinLevel` — §4.1.2.1.4.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable minLevel [} 163] from the DALI control gear.

##### `FB_DALI102QueryMissingShortAddress` — §4.1.2.1.4.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | — |
| `ipResultMessage` | `I_TcMessage` | — |
| `bBusy` | `BOOL` | — |

**Description:** The function block checks whether one or more DALI control gears do not have a short address.

##### `FB_DALI102QueryPhysicalMinimum` — §4.1.2.1.4.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable physicalMinimum [} 167] from the DALI control gear.

##### `FB_DALI102QueryPowerOnLevel` — §4.1.2.1.4.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable powerOnLevel [} 163] from the DALI control gear.

##### `FB_DALI102QueryRandomAddress` — §4.1.2.1.4.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable randomAddress [} 166] from the DALI control gear.

##### `FB_DALI102QuerySceneLevel` — §4.1.2.1.4.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the output value from the DALI control gear for the specified scene nScene.

#### (top level)

##### `The power supply was enabled (see powerCycleSeen [} 167])` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output nStatus only contains a valid value if the function block was executed without errors (bError = FALSE).

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 102 (control gears) › Query commands

##### `FB_DALI102QuerySystemFailureLevel` — §4.1.2.1.4.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable systemFailureLevel [} 163] from the DALI control gear.

##### `FB_DALI102ReadMemoryLocation` — §4.1.2.1.4.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** One byte is read from the memory bank of the DALI control gear. The memory bank is specified by the parameter nMemoryBank and the address within the memory bank by the parameter nOffset.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 102 (control gears) › Special commands

##### `FB_DALI102EnableDeviceType` — §4.1.2.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block must be called before an application extended command is called. This means that only the DALI control gears belonging to the corresponding device type respond.

##### `FB_DALI102Initialise` — §4.1.2.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block prepares the addressing of the DALI control gears.

##### `FB_DALI102ProgramShortAddress` — §4.1.2.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nShortAddress as a short address in the variable shortAddress [} 166] for all selected DALI control gears.

##### `FB_DALI102QueryShortAddress` — §4.1.2.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the short address (shortAddress [} 166]) of the selected DALI control gear.

##### `FB_DALI102Randomise` — §4.1.2.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL 146` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block generates a new random address for all selected DALI control gears and writes it to the randomAddress [} 166] variable.

##### `FB_DALI102SetSearchAddress` — §4.1.2.1.5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nSearchAddress in the searchAddress [} 166] variable for all selected DALI control gears.

##### `FB_DALI102Terminate` — §4.1.2.1.5.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block terminates addressing for all DALI control gears. The selection of the DALI control gears to be addressed by the FB_DALI102Initialise [} 138] function block is also canceled.

##### `FB_DALI102VerifyShortAddress` — §4.1.2.1.5.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block checks whether the variable shortAddress [} 166] matches the value nShortAddress for all selected DALI control gears.

##### `FB_DALI102WriteMemoryLocation` — §4.1.2.1.5.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The value nValue is written in the memory bank of the DALI control gear. The corresponding memory bank is specified by the input variable nMemoryBank, the address within the memory bank by the input variable nOffset.

##### `FB_DALI102WriteMemoryLocationNoReply` — §4.1.2.1.5.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The value nValue is written in the memory bank of the DALI control gear. The corresponding memory bank is specified by the input variable nMemoryBank, the address within the memory bank by the input variable nOffset.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 102 (control gears)

##### `FB_DALI102SendCommand` — §4.1.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The FB_DALI102SendCommand function block sends the specified DALI command to the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 103 (control devices) › Configuration commands

##### `FB_DALI103DisableApplicationController` — §4.1.2.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block deactivates the application controller (see applicationActive [} 246]).

##### `FB_DALI103DisableInstance` — §4.1.2.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block disables the selected instance of the DALI control device (see instanceActive [} 248]).

##### `FB_DALI103DisablePowerCycleNotification` — §4.1.2.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block locks the event Power Cycle Notification.

##### `FB_DALI103EnableApplicationController` — §4.1.2.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block enables the application controller (see applicationActive [} 246]).

##### `FB_DALI103EnableInstance` — §4.1.2.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block enables the selected instance of the DALI control device (see instanceActive [} 248]).

##### `FB_DALI103EnablePowerCycleNotification` — §4.1.2.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block enables the event Power Cycle Notification.

##### `FB_DALI103IdentifyDevice` — §4.1.2.2.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block starts the identification routine for a DALI control device.

##### `FB_DALI103Reset` — §4.1.2.2.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block resets all device and instance variables of the DALI control device to their reset values (see resetState [} 247]).

##### `FB_DALI103ResetMemoryBank` — §4.1.2.2.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The values of the selected memory bank of the DALI control device are set to their respective default values by the function block (see Memory banks [} 16]).

##### `FB_DALI103SavePersistentVariables` — §4.1.2.2.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block immediately triggers the saving of variables in the persistent memory by the DALI command SAVE PRESISTENT VARIABLES.

##### `FB_DALI103SetEventFilter` — §4.1.2.2.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block sets the event filter (see eventFilter [} 248]) for the corresponding instance of the DALI control device.

##### `FB_DALI103SetEventPriority` — §4.1.2.2.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block sets the event priority (see eventPriority [} 249]) for the corresponding instance of the DALI control device.

##### `FB_DALI103SetEventScheme` — §4.1.2.2.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block determines the event scheme (see eventScheme [} 249]) for the events of the respective instance of the DALI control device.

##### `FB_DALI103SetOperatingMode` — §4.1.2.2.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block sets the operating mode of the DALI control device (see operatingMode [} 246]).

##### `FB_DALI103SetShortAddress` — §4.1.2.2.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the new short address nNewShortAddress in the DALI control device (see shortAddress [} 245]).

##### `FB_DALI103StartQuiescentMode` — §4.1.2.2.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block enables the Quiescent Mode [} 246] for approx. 15 minutes.

##### `FB_DALI103StopQuiescentMode` — §4.1.2.2.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block interrupts the Quiescent mode [} 246] prematurely.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 103 (control devices)

##### `Query commands` — §4.1.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL 198` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the contents of DTR0 [} 245] (Data Transfer Register 0) from the DALI control device.

#### (top level)

##### `Unused (default value is 0)` — §7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output nDeviceStatus only contains a valid value if the function block was executed without errors (bError = FALSE).

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 103 (control devices) › Query commands

##### `FB_DALI103QueryEventFilter` — §4.1.2.2.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the event filter (see eventFilter [} 248]) for the selected instance of the DALI control device.

##### `FB_DALI103QueryEventPriority` — §4.1.2.2.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block reads the event priority (see eventPriority [} 249]) for the selected instance of the DALI control device.

##### `FB_DALI103QueryEventScheme` — §4.1.2.2.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the event scheme (see eventScheme [} 249]) for the events of the selected DALI control device instance.

##### `FB_DALI103QueryInputDeviceError` — §4.1.2.2.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads more detailed error information from the DALI control device (see inputDeviceError [} 247]).

##### `FB_DALI103QueryInputValue` — §4.1.2.2.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block queries the input value (see inputValue [} 248]) of the selected instance of the DALI control device.

##### `FB_DALI103QueryInstanceEnabled` — §4.1.2.2.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block queries whether the specified instance of the DALI control device is active (see instanceActive [} 248]).

##### `FB_DALI103QueryInstanceError` — §4.1.2.2.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads more detailed error information for the selected instance from the DALI control device (see instanceError [} 249]).

#### (top level)

##### `Events are enabled for the instance (see instanceActive [} 248])` — §1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The output nInstanceStatus only contains a valid value if the function block was executed without errors (bError = FALSE).

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 103 (control devices) › Query commands

##### `FB_DALI103QueryInstanceType` — §4.1.2.2.2.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the instance type (see instanceType [} 248]) for the selected instance from the DALI control device.

##### `FB_DALI103QueryNumberOfInstances` — §4.1.2.2.2.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the number of existing instances from the DALI control device (see numberOfInstances [} 246]).

##### `FB_DALI103QueryOperatigMode` — §4.1.2.2.2.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable operatingMode [} 246] from the DALI control device.

##### `FB_DALI103QueryPowerCycleNotification` — §4.1.2.2.2.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block outputs whether the Power Cycle Notification event of a DALI control device is enabled.

##### `FB_DALI103QueryRandomAddress` — §4.1.2.2.2.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable randomAddress [} 245] from the DALI control device.

##### `FB_DALI103QueryResolution` — §4.1.2.2.2.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the resolution of the input value for the selected instance from the DALI control device (see resolution [} 248]).

##### `FB_DALI103ReadMemoryLocation` — §4.1.2.2.2.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The value nValue is read from the memory bank of the DALI control device. The corresponding memory bank is specified by the input variable nMemoryBank, the address within the memory bank by the input variable nOffset.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 103 (control devices) › Special commands

##### `FB_DALI103Initialise` — §4.1.2.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block prepares the addressing of the DALI control devices.

##### `FB_DALI103ProgramShortAddress` — §4.1.2.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nShortAddress as short address in the variable shortAddress [} 245] for all selected DALI control devices.

##### `FB_DALI103Randomise` — §4.1.2.2.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL 232` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block generates a new random address for all selected DALI control devices and writes it to the randomAddress variable. DALI control devices are selected if the DALI command INITIALISE (see FB_DALI103Initialise [} 226]) was executed previously.

##### `FB_DALI103SetSearchAddress` — §4.1.2.2.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nSearchAddress in the searchAddress [} 245] variable for all selected DALI control devices.

##### `FB_DALI103Terminate` — §4.1.2.2.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block terminates addressing in all DALI control devices. The selection of the devices to be addressed by the function block FB_DALI103Initialise [} 226] is also canceled.

##### `FB_DALI103VerifyShortAddress` — §4.1.2.2.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block checks whether the variable shortAddress [} 245] matches the value nShortAddress for all selected DALI control devices.

##### `FB_DALI103WriteMemoryLocation` — §4.1.2.2.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The value nValue is written in the memory bank of the DALI control device. The corresponding memory bank is specified by the input variable nMemoryBank, the address within the memory bank by the input variable nOffset.

##### `FB_DALI103WriteMemoryLocationNoReply` — §4.1.2.2.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The value nValue is written in the memory bank of the DALI control device. The corresponding memory bank is specified by the input variable nMemoryBank, the address within the memory bank by the input variable nOffset.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 202 (emergency lighting) › Configuration commands

##### `FB_DALI202SetDurationTestInterval` — §4.1.2.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nDurationTestInterval in the variable durationTestInterval [} 298] of the DALI control gear.

##### `FB_DALI202SetEmergencyLevel` — §4.1.2.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nEmergencyLevel in the variable emergencyLevel [} 296] of the DALI control gear.

##### `FB_DALI202SetFunctionTestInterval` — §4.1.2.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nFunctionTestInterval in the variable functionTestInterval [} 297] of the DALI control gear.

##### `FB_DALI202SetProlongTime` — §4.1.2.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nProlongTime in the variable prolongTime [} 297] of the DALI control gear.

##### `FB_DALI202SetTestDelayTime` — §4.1.2.3.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nDelayTestTime in the variable testDelayTime [} 297] of the DALI control gear.

##### `FB_DALI202SetTestExecutionTimeout` — §4.1.2.3.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nTestExecutionTimeout in the variable testExecutionTimeout [} 298] of the DALI control gear.

##### `FB_DALI202StartIdentification` — §4.1.2.3.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block starts or extends the identification routine of the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 202 (emergency lighting) › Control commands

##### `FB_DALI202Inhibit` — §4.1.2.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block prevents the DALI control gear from switching to emergency mode for 15 minutes.

#### (top level)

##### `All variables are reset to their respective reset values` — §0

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** 262

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 202 (emergency lighting) › Control commands

##### `FB_DALI202ReLightResetInhibit` — §4.1.2.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block is used to switch the DALI control gear back to emergency mode in the absence of mains voltage.

##### `FB_DALI202ResetDurationTestDoneFlag` — §4.1.2.3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block resets bit 2 of the variable emergencyStatus [} 300].

##### `FB_DALI202ResetFunctionTestDoneFlag` — §4.1.2.3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block resets bit 1 of the variable emergencyStatus [} 300].

##### `FB_DALI202ResetLampTime` — §4.1.2.3.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block sets the variables lampEmergencyTime [} 298] and lampTotalOperationTime [} 299] to 0.

##### `FB_DALI202Rest` — §4.1.2.3.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block is used to switch off the lamp when the DALI control gear is in emergency mode.

##### `FB_DALI202StartDurationTest` — §4.1.2.3.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block starts a duration test.

##### `FB_DALI202StartFunctionTest` — §4.1.2.3.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block starts a function test.

##### `FB_DALI202StopTest` — §4.1.2.3.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stops a function or duration test.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 202 (emergency lighting) › Query commands

##### `FB_DALI202QueryBatteryCharge` — §4.1.2.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable batteryCharge [} 298] from the DALI control gear.

##### `FB_DALI202QueryDurationTestResult` — §4.1.2.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable durationTestResult [} 298] from the DALI control gear.

##### `FB_DALI202QueryEmergencyLevel` — §4.1.2.3.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable emergencyLevel [} 296] from the DALI control gear.

##### `FB_DALI202QueryEmergencyMaxLevel` — §4.1.2.3.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable emergencyMaxLevel [} 297] from the DALI control gear.

##### `FB_DALI202QueryEmergencyMinLevel` — §4.1.2.3.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable emergencyMinLevel [} 297] from the DALI control gear.

##### `FB_DALI202QueryEmergencyMode` — §4.1.2.3.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable emergencyMode [} 299] from the DALI control gear.

##### `FB_DALI202QueryEmergencyStatus` — §4.1.2.3.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable emergencyStatus [} 300] from the DALI control gear.

##### `FB_DALI202QueryFailureStatus` — §4.1.2.3.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable failureStatus [} 300] from the DALI control gear.

##### `FB_DALI202QueryFeatures` — §4.1.2.3.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the implemented properties from the DALI control gear (see features [} 299]).

##### `FB_DALI202QueryLampEmergencyTime` — §4.1.2.3.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable lampEmergencyTime [} 298] from the DALI control gear.

##### `FB_DALI202QueryLampTotalOperationTime` — §4.1.2.3.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable lampTotalOperationTime [} 299] from the DALI control gear.

##### `FB_DALI202QueryRatedDuration` — §4.1.2.3.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable ratedDuration [} 299] from the DALI control gear.

##### `FB_DALI202QueryTestTiming` — §4.1.2.3.3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** Depending on the input eTestTiming, this function block reads out the settings for the function and duration test from the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 205 (incandescent lamps) › Configuration commands

##### `FB_DALI205SelectDimmingCurve` — §4.1.2.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Var_INPUT` | `` | — |
| `bStart` | `: BOOL;` | — |
| `nAddress` | `: BYTE;` | — |
| `eAddressType` | `: E_DALIAddressType := E_DALIAddressType.Short;` | — |
| `eCommandPriority` | `: E_DALICommandPriority := E_DALICommandPriority.MiddleLow;` | — |
| `eDimmingCurve` | `: E_DALIDimmingCurve := E_DALIDimmingCurve.Standard;` | — |
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value eDimmingCurve in the variable dimmingCurve [} 319] of the DALI control gear. This also changes the value of the Delay Time.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 205 (incandescent lamps) › Query commands

##### `FB_DALI205QueryDimmerStatus` — §4.1.2.4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the current operation mode from the DALI control gear (see dimmerStatus [} 319]).

##### `FB_DALI205QueryDimmerTemperature` — §4.1.2.4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the current operating temperature from the DALI control gear (see dimmerTemperature [} 321]).

##### `FB_DALI205QueryDimmingCurve` — §4.1.2.4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the set dimming curve from the DALI control gear (see dimmingCurve [} 319]).

##### `FB_DALI205QueryFailureStatus` — §4.1.2.4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the failure status from the DALI control gear (see failureStatus [} 320]).

##### `FB_DALI205QueryFeatures` — §4.1.2.4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the implemented properties from the DALI control gear (see dimmerFeatures [} 320]).

##### `FB_DALI205QueryLoadRating` — §4.1.2.4.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the maximum permissible root mean square of the load current from the DALI control gear (see loadRating [} 322]).

##### `FB_DALI205QueryRealLoadPower` — §4.1.2.4.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the current active power of the load from the DALI control gear (see realLoadPower [} 322]).

##### `FB_DALI205QueryRMSLoadCurrent` — §4.1.2.4.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the root mean square of the load current from the DALI control gear (rmsLoadCurrent [} 322]).

##### `FB_DALI205QueryRMSLoadVoltage` — §4.1.2.4.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the root mean square of the load voltage from the DALI control gear (see rmsLoadVoltage [} 321]).

##### `FB_DALI205QueryRMSSupplyVoltage` — §4.1.2.4.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads out the root mean square of the supply voltage from the DALI control gear (see rmsSupplyVoltage [} 321]).

##### `FB_DALI205QuerySupplyFrequency` — §4.1.2.4.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the current frequency of the supply voltage from the DALI control gear (see supplyFrequency [} 321]).

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 207 (LED modules) › Configuration commands

##### `FB_DALI207ReferenceSystemPower` — §4.1.2.5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The DALI control gear measures and stores the performance level of the system, in order to detect load increase and decrease.

##### `FB_DALI207SelectDimmingCurve` — §4.1.2.5.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value eDimmingCurve in the variable dimmingCurve [} 347] of the DALI control gear.

##### `FB_DALI207SetFastFadeTime` — §4.1.2.5.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value eFastFadeTime in the variable fastFadeTime [} 346] of the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 207 (LED modules) › Query commands

##### `FB_DALI207QueryControlGearType` — §4.1.2.5.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable controlGearType [} 346] from the DALI control gear.

##### `FB_DALI207QueryDimmingCurve` — §4.1.2.5.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the set dimming curve from the DALI control gear (see dimmingCurve [} 347]).

##### `FB_DALI207QueryFailureStatus` — §4.1.2.5.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the failure status from the DALI control gear (see failureStatus [} 347]).

##### `FB_DALI207QueryFastFadeTime` — §4.1.2.5.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable fastFadeTime [} 346] from the DALI control gear.

##### `FB_DALI207QueryFeatures` — §4.1.2.5.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable features [} 346] from the DALI control gear.

##### `FB_DALI207QueryLoadDecrease` — §4.1.2.5.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block queries whether a significant load decrease (in comparison with the reference power of the system) has been detected.

##### `FB_DALI207QueryLoadIncrease` — §4.1.2.5.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block queries whether a significant load increase (in comparison with the reference power of the system) has been detected.

##### `FB_DALI207QueryMinFastFadeTime` — §4.1.2.5.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable minFastFadeTime [} 346] from the DALI control gear.

##### `FB_DALI207QueryReferenceMeasurementFailed` — §4.1.2.5.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block queries whether a started reference measurement has failed.

##### `FB_DALI207QueryReferenceRunning` — §4.1.2.5.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block queries whether a reference measurement of the system power is activated.

##### `FB_DALI207QueryThermalOverload` — §4.1.2.5.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block queries whether there is a thermal overload with reduction of the output level.

##### `FB_DALI207QueryThermalShutdown` — §4.1.2.5.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block queries whether a thermal shutdown has taken place.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 208 (device for switching functions) › Configuration commands

##### `FB_DALI208SetDownSwitchOffThreshold` — §4.1.2.6.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nDownSwitchOffThreshold in the variable downSwitchOffThreshold [} 367] of the DALI control gear.

##### `FB_DALI208SetDownSwitchOnThreshold` — §4.1.2.6.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nDownSwitchOnThreshold in the variable downSwitchOnThreshold [} 367] of the DALI control gear.

##### `FB_DALI208SetUpSwitchOffThreshold` — §4.1.2.6.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nUpSwitchOffThreshold in the variable upSwitchOffThreshold [} 367] of the DALI control gear.

##### `FB_DALI208SetUpSwitchOnThreshold` — §4.1.2.6.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nUpSwitchOnThreshold in the variable upSwitchOnThreshold [} 367] of the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 208 (device for switching functions) › Query commands

##### `FB_DALI208QueryDownSwitchOffThreshold` — §4.1.2.6.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable downSwitchOffThreshold [} 367] from the DALI control gear.

##### `FB_DALI208QueryDownSwitchOnThreshold` — §4.1.2.6.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable downSwitchOnThreshold [} 367] from the DALI control gear.

##### `FB_DALI208QueryFeatures` — §4.1.2.6.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable features [} 368] from the DALI control gear.

##### `FB_DALI208QueryGearType` — §4.1.2.6.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable gearType [} 368] from the DALI control gear.

##### `FB_DALI208QuerySwitchStatus` — §4.1.2.6.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable switchStatus [} 368] from the DALI control gear.

##### `FB_DALI208QueryUpSwitchOffThreshold` — §4.1.2.6.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable upSwitchOffThreshold [} 367] from the DALI control gear.

##### `FB_DALI208QueryUpSwitchOnThreshold` — §4.1.2.6.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable upSwitchOnThreshold [} 367] from the DALI control gear.

#### (top level)

##### `Freely selectable color` — §6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 209 (color/color temperature control) › Configuration commands

##### `FB_DALI209SetPowerOnValues` — §4.1.2.7.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the values nPowerOnLevel and eColourType in the variables powerOnLevel [} 163] and powerOnColourType [} 438] of the DALI control gear. In addition, depending on the value of eColourType, the values of the inputs are written to the corresponding variables.

##### `FB_DALI209SetScene` — §4.1.2.7.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block can be used to assign a specific lighting value to a scene.

##### `FB_DALI209SetSystemFailureValues` — §4.1.2.7.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block writes the color representation and light intensity (nSystemFailureLevel) of the light source to be used in the case of a system failure to the DALI control gear.

##### `FB_DALI209StartAutoCalibration` — §4.1.2.7.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The calibration procedure is started in order to measure the x-coordinate, the y-coordinate and the TY value of all supported primary colors.

##### `FB_DALI209StoreColourTemperatureTcLimit` — §4.1.2.7.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** Depending on the specification of eSelectLimitValue, the function block saves the value nColourTemperatureTcLimit in the variables colourTemperatureTcCoolest, colourTemperatureTcWarmest, colourTemperatureTcPhysicalCoolest or colourTemperatureTcPhysicalWarmest of the DALI control gear.

##### `FB_DALI209StoreGearFeaturesStatus` — §4.1.2.7.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nGearFeaturesStatus in the variable gearFeatures/Status [} 439] of the DALI control gear.

##### `FB_DALI209StoreTYPrimaryN` — §4.1.2.7.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nTYPrimaryN in the variable tyPrimaryN [} 438] of the DALI control gear.

##### `FB_DALI209StoreXyCoordinatePrimaryN` — §4.1.2.7.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block copies the value from the variables temporaryXCoordinate [} 437] and temporaryYCoordinate [} 438] to the variables xCoordinatePrimaryN [} 438] and yCoordinatePrimaryN [} 438].

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 209 (color/color temperature control) › Control commands

##### `FB_DALI209Activate` — §4.1.2.7.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block causes the buffered values of the temporary registers to be transferred to the lamp.

##### `FB_DALI209ColourTemperatureTcStepCooler` — §4.1.2.7.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** Due to the function block the value colourTemperatureTc [} 438] is reduced by 1 Mirek without cross-fading. If the colourTemperatureTc [} 438] value already has the same value as colourTemperatureTcCoolest [} 438], no change takes place.

##### `FB_DALI209ColourTemperatureTcStepWarmer` — §4.1.2.7.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** Due to the function block the value colourTemperatureTc [} 438] is increased by 1 Mirek without cross- fading. If the colourTemperatureTc [} 438] value already has the same value as colourTemperatureTcWarmest [} 438], no change takes place.

##### `FB_DALI209CopyReportToTemporary` — §4.1.2.7.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block copies the contents of the variables with the color settings to the variables for the temporary color settings.

##### `FB_DALI209SetColourTemperatureTc` — §4.1.2.7.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nColourTemperatureTc in the variable colourTemperatureTc [} 438] of the DALI control gear and causes the set color temperature to be applied to the lamp by executing the ACTIVATE command.

##### `FB_DALI209SetTemporaryColourTemperatureTc` — §4.1.2.7.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nTemporaryColourTemperatureTc in the variable temporaryTemperatureTc [} 438] of the DALI control gear.

##### `FB_DALI209SetTemporaryPrimaryNDimLevel` — §4.1.2.7.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nTemporaryPrimaryNDimLevel in the variable temporaryPrimaryNDimLevel [} 438] of the DALI control gear.

##### `FB_DALI209SetTemporaryRGBDimLevel` — §4.1.2.7.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the values nTemporaryRedDimLevel, nTemporaryGreenDimLevel and nTemporaryBlueDimLevel in the variables temporaryRedDimLevel [} 438], temporaryGreenDimLevel [} 438] and temporaryBlueDimLevel [} 438] of the DALI control gear.

##### `FB_DALI209SetTemporaryRGBWAFControl` — §4.1.2.7.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nTemporaryRGBWAFControl in the variable temporaryRGBWAFControl [} 439] of the DALI control gear.

##### `FB_DALI209SetTemporaryWAFDimLevel` — §4.1.2.7.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the values nTemporaryWhiteDimLevel, nTemporaryAmberDimLevel and nTemporaryFreeColourDimLevel in the variables temporaryWhiteDimLevel [} 438], temporaryAmberDimLevel [} 438] and temporaryFreeColourDimLevel [} 438] of the DALI control gear.

##### `FB_DALI209SetTemporaryXCoordinate` — §4.1.2.7.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nTemporaryXCoordinate in the variable temporaryXCoordinate [} 437] of the DALI control gear.

##### `FB_DALI209SetTemporaryYCoordinate` — §4.1.2.7.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nTemporaryYCoordinate in the variable temporaryYCoordinate [} 438] of the DALI control gear.

##### `FB_DALI209XCoordinateStepDown` — §4.1.2.7.3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reduces the variable xCoordinate [} 437] by 256 steps (256 / 65536) without cross-fading.

##### `FB_DALI209XCoordinateStepUp` — §4.1.2.7.3.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block increases the variable xCoordinate [} 437] by 256 steps (256 / 65536) without cross- fading.

##### `FB_DALI209YCoordinateStepDown` — §4.1.2.7.3.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reduces the variable yCoordinate [} 438] by 256 steps (256 / 65536) without cross-fading.

##### `FB_DALI209YCoordinateStepUp` — §4.1.2.7.3.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block increases the variable yCoordinate [} 438] by 256 steps (256 / 65536) without cross- fading.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 209 (color/color temperature control) › Query commands

##### `FB_DALI209QueryAssignedColour` — §4.1.2.7.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during the |

**Description:** The function block reads the variable assignedColour [} 438] from the DALI control gear.

##### `FB_DALI209QueryColourStatus` — §4.1.2.7.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable colourStatus [} 439] from the DALI control gear.

##### `FB_DALI209QueryColourTypeFeatures` — §4.1.2.7.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable colourTypeFeatures [} 439] from the DALI control gear.

##### `FB_DALI209QueryColourValue` — §4.1.2.7.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the specified variable (color value) from the DALI control gear. The value to be read is defined by eColourValue.

##### `FB_DALI209QueryGearFeaturesStatus` — §4.1.2.7.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable gearFeatures/Status [} 439] from the DALI control gear.

##### `FB_DALI209QueryPowerOnValues` — §4.1.2.7.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variables powerOnLevel [} 163] and the supported values of the color settings from the DALI control gear.

##### `FB_DALI209QueryRGBWAFControl` — §4.1.2.7.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during the |

**Description:** The function block reads the variable RGBWAFControl [} 439] from the DALI control gear.

##### `FB_DALI209QuerySceneValues` — §4.1.2.7.4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the output values from the DALI control gear for the specified scene nScene.

##### `FB_DALI209QuerySystemFailureValues` — §4.1.2.7.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable systemFailureLevel [} 163] and the corresponding color/color temperature settings from the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 209 (color/color temperature control) › Help Functions

##### `KELVIN_TO_MIREK` — §4.1.2.7.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fKelvin` | `LREAL` | Color temperature in Kelvin |

**Description:** Conversion of the color temperature from Kelvin to Mirek.

##### `MIREK_TO_KELVIN` — §4.1.2.7.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nMirek` | `UINT` | Color temperature in Mirek |

**Description:** Conversion of the color temperature from Mirek to Kelvin.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 251 (memory bank 1 extension)

##### `FB_DALI251QueryColourRenderingIndex` — §4.1.2.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the color rendering index from the DALI control gear.

##### `FB_DALI251QueryCorrelatedColourTemperature` — §4.1.2.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the correlated color temperature from the DALI control gear.

##### `FB_DALI251QueryLuminaireColour` — §4.1.2.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the luminaire color from the DALI control gear.

##### `FB_DALI251QueryLuminaireDateOfManufacture` — §4.1.2.8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the year and week of production from the DALI control gear.

##### `FB_DALI251QueryLuminaireIdentification` — §4.1.2.8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the identification designation of the luminaire from the DALI control gear.

##### `FB_DALI251QueryNominalInputPower` — §4.1.2.8.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the nominal input power of the luminaire from the DALI control gear.

##### `FB_DALI251QueryNominalLightOutput` — §4.1.2.8.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the nominal light output of the luminaire from the DALI control gear.

##### `FB_DALI251QueryNominalMaximumACMainsVoltage` — §4.1.2.8.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the maximum nominal input voltage of the DALI control gear.

##### `FB_DALI251QueryNominalMinimumACMainsVoltage` — §4.1.2.8.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the minimum nominal input voltage of the DALI control gear.

##### `FB_DALI251QueryPowerAtMinimumDimLevel` — §4.1.2.8.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the power consumption at the minimum dimming value of the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 252 (energy reporting)

##### `FB_DALI252QueryActiveEnergy` — §4.1.2.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the active energy from the DALI control gear.

##### `FB_DALI252QueryActiveEnergyLoadside` — §4.1.2.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the load-side active energy from the DALI control gear.

##### `FB_DALI252QueryActivePower` — §4.1.2.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the active power from the DALI control gear.

##### `FB_DALI252QueryActivePowerLoadside` — §4.1.2.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the load-side active power from the DALI control gear.

##### `FB_DALI252QueryApparentEnergy` — §4.1.2.9.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the apparent energy from the DALI control gear.

##### `FB_DALI252QueryApparentPower` — §4.1.2.9.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the apparent power from the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 253 (diagnostics and maintenance)

##### `FB_DALI253QueryControlGearExternalSupplyOvervoltage` — §4.1.2.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates whether an overvoltage has been detected in the external supply voltage of the DALI control gear.

##### `FB_DALI253QueryControlGearExternalSupplyOvervoltageCounter` — §4.1.2.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter for the detected overvoltage events of the external supply voltage of the DALI control gear.

##### `FB_DALI253QueryControlGearExternalSupplyUndervoltage` — §4.1.2.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates whether an undervoltage has been detected in the external supply voltage of the DALI control gear.

##### `FB_DALI253QueryControlGearExternalSupplyUndervoltageCounter` — §4.1.2.10.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter for the detected undervoltage events of the external supply voltage of the DALI control gear.

##### `FB_DALI253QueryControlGearExternalSupplyVoltage` — §4.1.2.10.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the effective value of the external supply voltage from the DALI control gear.

##### `FB_DALI253QueryControlGearExternalSupplyVoltageFrequency` — §4.1.2.10.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the frequency of the external supply voltage from the DALI control gear.

##### `FB_DALI253QueryControlGearOperatingTime` — §4.1.2.10.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the operating time of the DALI control gear.

##### `FB_DALI253QueryControlGearOutputCurrentPercent` — §4.1.2.10.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the output current as a percentage of the nominal current from the DALI control gear.

##### `FB_DALI253QueryControlGearOutputPowerLimitation` — §4.1.2.10.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates whether the limit value of the output power of the DALI control gear was exceeded.

##### `FB_DALI253QueryControlGearOutputPowerLimitationCounter` — §4.1.2.10.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter of the number of exceedances of the output power limit value at the DALI control gear.

##### `FB_DALI253QueryControlGearOverallFailureCondition` — §4.1.2.10.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block outputs the status of controlGearFailure [} 166] at the DALI control gear.

##### `FB_DALI253QueryControlGearOverallFailureConditionCounter` — §4.1.2.10.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter of the error events that have occurred in the DALI control gear.

##### `FB_DALI253QueryControlGearPowerFactor` — §4.1.2.10.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block outputs the power factor of the DALI control gear.

##### `FB_DALI253QueryControlGearStartCounter` — §4.1.2.10.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block outputs the number of starts of the DALI control gear. The value is incremented after switching on the external power supply.

##### `FB_DALI253QueryControlGearTemperature` — §4.1.2.10.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block outputs the internal temperature of the DALI control gear.

##### `FB_DALI253QueryControlGearThermalDerating` — §4.1.2.10.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates whether the internal temperature is higher than the specified limit value in the DALI control gear.

##### `FB_DALI253QueryControlGearThermalDeratingCounter` — §4.1.2.10.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter that counts the number of times the temperature limit value inside the DALI control gear is exceeded.

##### `FB_DALI253QueryControlGearThermalShutdown` — §4.1.2.10.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates whether there is a thermal shutdown of the DALI control gear.

##### `FB_DALI253QueryControlGearThermalShutdownCounter` — §4.1.2.10.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter of the number of thermal shutdowns of the DALI control gear.

##### `FB_DALI253QueryInternalControlGearReferenceTemperature` — §4.1.2.10.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block outputs the internal reference temperature of the DALI control gear.

##### `FB_DALI253QueryLightSourceCurrent` — §4.1.2.10.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the current consumption of the light source.

##### `FB_DALI253QueryLightSourceOnTime` — §4.1.2.10.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the non-resettable operating hour counter of the light source. If the light source is switched on, the non-resettable operating hour counter is incremented by one every second.

##### `FB_DALI253QueryLightSourceOnTimeResettable` — §4.1.2.10.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the resettable operating hour counter of the light source. If the light source is switched on, the resettable operating hour counter is incremented by one every second. The resettable operating hours counter can be changed by the function block FB_DALI253SetLightSourceOnTimeResettable [} 526].

##### `FB_DALI253QueryLightSourceOpenCircuit` — §4.1.2.10.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates whether there is an open circuit to the light source.

##### `FB_DALI253QueryLightSourceOpenCircuitCounter` — §4.1.2.10.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter of the number of open circuits to the light source.

##### `FB_DALI253QueryLightSourceOverallFailureCondition` — §4.1.2.10.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block outputs the status of lampFailure [} 166] at the DALI control gear.

##### `FB_DALI253QueryLightSourceOverallFailureConditionCounter` — §4.1.2.10.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter of the light source error events.

##### `FB_DALI253QueryLightSourceShortCircuit` — §4.1.2.10.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates whether there is a short-circuit at the light source.

##### `FB_DALI253QueryLightSourceShortCircuitCounter` — §4.1.2.10.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter that counts the number of light source short-circuits.

##### `FB_DALI253QueryLightSourceStartCounter` — §4.1.2.10.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the non-resettable counter for the switching on of the light source. The non- resettable counter is incremented by one each time the light source changes from the Off state to the On state. The non-resettable counter cannot be reset or changed.

##### `FB_DALI253QueryLightSourceStartCounterResettable` — §4.1.2.10.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the resettable counter for the switching on of the light source. The resettable counter is incremented by one each time the light source changes from the Off state to the On state.

##### `FB_DALI253QueryLightSourceTemperature` — §4.1.2.10.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates the temperature of the light source.

##### `FB_DALI253QueryLightSourceThermalDerating` — §4.1.2.10.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates whether the light source temperature is higher than the specified limit value.

##### `FB_DALI253QueryLightSourceThermalDeratingCounter` — §4.1.2.10.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter that counts the number of times the light source temperature exceeded the limit value.

##### `FB_DALI253QueryLightSourceThermalShutdown` — §4.1.2.10.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates whether there is a thermal shutdown on the DALI control gear due to the light source.

##### `FB_DALI253QueryLightSourceThermalShutdownCounter` — §4.1.2.10.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the counter that counts the number of thermal shutdowns of the light source.

##### `FB_DALI253QueryLightSourceVoltage` — §4.1.2.10.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the output voltage at the DALI control gear.

##### `FB_DALI253QueryRatedMedianUsefulLifeOfLuminaire` — §4.1.2.10.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates the estimated average lifetime of the complete luminaire, including all individual components.

##### `FB_DALI253QueryRatedMedianUsefulLightSourceStarts` — §4.1.2.10.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block indicates the average number of possible starts of the complete luminaire, including all individual components.

##### `FB_DALI253SetInternalControlGearReferenceTemperature` — §4.1.2.10.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | — |
| `nAddress` | `BYTE` | — |
| `eAddressType` | `E_DALIAddressType := E_DALIAddressType.Short` | — |
| `eCommandPriority` | `E_DALICommandPriority := E_DALICommandPriority.MiddleLo` | — |
| `nInternalControlGearReferenceTemperature` | `INT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block writes the internal reference temperature to the DALI control gear.

##### `FB_DALI253SetLightSourceOnTimeResettable` — §4.1.2.10.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block sets the resettable operating hour counter of the light source to any desired value. If the light source is switched on, the resettable operating hour counter is incremented by one every second.

##### `FB_DALI253SetLightSourceStartCounterResettable` — §4.1.2.10.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block sets the resettable counter for the switching on of the light source to any desired value. The resettable counter is incremented by one each time the light source changes from the Off state to the On state.

##### `FB_DALI253SetRatedMedianUsefulLifeOfLuminaire` — §4.1.2.10.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block writes the estimated average lifetime of the complete luminaire to the DALI control gear.

##### `FB_DALI253SetRatedMedianUsefulLightSourceStarts` — §4.1.2.10.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block writes the average number of possible starts of the complete luminaire to the DALI control gear.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 301 (push button)

##### `FB_DALI301QueryDoubleTimer` — §4.1.2.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tDouble [} 553].

##### `FB_DALI301QueryDoubleTimerMin` — §4.1.2.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tDoubleMin [} 553].

##### `FB_DALI301QueryRepeatTimer` — §4.1.2.11.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tRepeat [} 553].

##### `FB_DALI301QueryShortTimer` — §4.1.2.11.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tShort [} 553].

##### `FB_DALI301QueryShortTimerMin` — §4.1.2.11.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tShortMin [} 553].

##### `FB_DALI301QueryStuckTimer` — §4.1.2.11.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tStuck [} 554].

##### `FB_DALI301SetDoubleTimer` — §4.1.2.11.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nDouble in the variable tDouble [} 553] of the DALI control device.

##### `FB_DALI301SetRepeatTimer` — §4.1.2.11.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nRepeat in the variable tRepeat [} 553] of the DALI control device.

##### `FB_DALI301SetShortTimer` — §4.1.2.11.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nShort in the variable tShort [} 553] of the DALI control device.

##### `FB_DALI301SetStuckTimer` — §4.1.2.11.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nStuck in the variable tStuck [} 554] of the DALI control device.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 302 (absolute input device)

##### `FB_DALI302QueryDeadtimeTimer` — §4.1.2.12.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tDeadtime [} 564].

##### `FB_DALI302QueryReportTimer` — §4.1.2.12.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tReport [} 564].

##### `FB_DALI302QuerySwitch` — §4.1.2.12.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block queries whether the instance represents a switch.

##### `FB_DALI302SetDeadtimeTimer` — §4.1.2.12.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nDeadtime in the variable tDeadtime [} 564] of the DALI control device.

##### `FB_DALI302SetReportTimer` — §4.1.2.12.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nReport in the variable tReport [} 564] of the DALI control device.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 303 (occupancy sensor)

##### `FB_DALI303CancelHoldTimer` — §4.1.2.13.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block ends the Hold Timer prematurely if the latter was implemented in the DALI control device.

##### `FB_DALI303CatchMovement` — §4.1.2.13.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** Catching is activated for the DALI control devices by this function block. If the DALI control device detects motion, an event is sent once and the Catching is reset. The Event Filter [} 587] should be set so that the Movement event is disabled. This ensures that the event is only sent once as soon as Catching is active and movement is detected.

##### `FB_DALI303QueryCatching` — §4.1.2.13.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block queries whether Catching is active.

##### `FB_DALI303QueryDeadtimeTimer` — §4.1.2.13.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tDeadtime [} 587].

##### `FB_DALI303QueryDetectionSensitivity` — §4.1.2.13.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the detectionSensitivity [} 587] variable.

##### `FB_DALI303QueryHoldTimer` — §4.1.2.13.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tHold [} 587].

##### `FB_DALI303QueryReportTimer` — §4.1.2.13.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tReport [} 587].

##### `FB_DALI303SetDeadtimeTimer` — §4.1.2.13.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nDeadtime in the variable tDeadtime [} 587] of the DALI control device.

##### `FB_DALI303SetDetectionSensitivity` — §4.1.2.13.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value of nDetectionSensitivity in the detectionSensitivity [} 587] variable of the DALI control device.

##### `FB_DALI303SetHoldTimer` — §4.1.2.13.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nHold in the variable tHold [} 587] of the DALI control device.

##### `FB_DALI303SetReportTimer` — §4.1.2.13.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nReport in the variable tReport [} 587] of the DALI control device.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 304 (light sensors)

##### `FB_DALI304QueryDeadtimeTimer` — §4.1.2.14.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tDeadtime [} 605].

##### `FB_DALI304QueryHysteresis` — §4.1.2.14.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable hysteresis [} 605].

##### `FB_DALI304QueryHysteresisMin` — §4.1.2.14.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable hysteresisMin [} 605].

##### `FB_DALI304QueryReportTimer` — §4.1.2.14.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tReport [} 605].

##### `FB_DALI304SetDeadtimeTimer` — §4.1.2.14.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nDeadtime in the variable tDeadtime [} 605] of the DALI control device.

##### `FB_DALI304SetHysteresis` — §4.1.2.14.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nHysteresis in the variable hysteresis [} 605] of the DALI control device.

##### `FB_DALI304SetHysteresisMin` — §4.1.2.14.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nHysteresisMin in the variable hysteresisMin [} 605] of the DALI control device.

##### `FB_DALI304SetReportTimer` — §4.1.2.14.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nReport in the variable tReport [} 605] of the DALI control device.

#### Error while reading the variable minLevel [} 163]. › POUs › Commands › Part 306 (general purpose sensors)

##### `FB_DALI306QueryAlarmReportTimer` — §4.1.2.15.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tAlarmReport.

##### `FB_DALI306QueryDeadtimeTimer` — §4.1.2.15.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tDeadtime [} 637].

##### `FB_DALI306QueryHysteresis` — §4.1.2.15.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable hysteresis [} 637].

##### `FB_DALI306QueryHysteresisMin` — §4.1.2.15.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable hysteresisMin [} 637].

##### `FB_DALI306QueryMeasurementVariable` — §4.1.2.15.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads a measured value defined by the eMeasurementVariable parameter from the DALI control device and outputs the value at the output nMeasurementVariable.

##### `FB_DALI306QueryReportTimer` — §4.1.2.15.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block reads the variable tReport [} 636].

##### `FB_DALI306SetAlarm` — §4.1.2.15.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nAlarmLevel in the DALI control device. The input eAlarmNumber specifies for which alarm (Alarm 0 ... Alarm 3) the alarm level is to be saved.

##### `FB_DALI306SetAlarmHysteresis` — §4.1.2.15.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nAlarmHysteresis in the DALI control device. The input eAlarmNumber specifies for which alarm (Alarm 0 ... Alarm 3) the alarm hysteresis is to be saved.

##### `FB_DALI306SetAlarmReportTimer` — §4.1.2.15.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nAlarmReport in the variable tAlarmReport of the DALI control device.

##### `FB_DALI306SetAlarmType` — §4.1.2.15.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value eAlarmNType in the variable alarmType [} 638] of the DALI control device.

##### `FB_DALI306SetDeadtimeTimer` — §4.1.2.15.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nDeadtime in the variable tDeadtime [} 637] of the DALI control device.

##### `FB_DALI306SetHysteresis` — §4.1.2.15.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nHysteresis in the variable hysteresis [} 637] of the DALI control device.

##### `FB_DALI306SetHysteresisMin` — §4.1.2.15.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value nHysteresisMin in the variable hysteresisMin [} 637] of the DALI control device.

##### `FB_DALI306SetMagnitude` — §4.1.2.15.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block saves the value nMagnitude in the variable magnitude [} 637] of the DALI control device.

##### `FB_DALI306SetReportTimer` — §4.1.2.15.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Execution of the DALI commands is triggered via a positive |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block stores the value of nReport in the variable tReport [} 636] of the DALI control device.

#### Error while reading the variable minLevel [} 163]. › POUs › Communication

##### `FB_EL6821Communication` — §4.1.3.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_EL6821InData` | — |
| `stOutData` | `ST_KL6821OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function blocks for the DALI commands do not directly access the process image of the EL6821, but place the individual DALI commands in a command buffer. The function block FB_EL6821Communication sequentially reads the DALI commands from the command buffer and forwards them to the EL6821. This prevents multiple function blocks accessing the EL6821 process image at the same time.

##### `FB_KL6811Communication` — §4.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetMaximumDe BOOL` | `A positive edge resets the stored value for the maximum` | — |
| `mandCounter` | `utilization of the command buffer,` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_KL6811InData` | — |
| `stOutData` | `ST_KL6811OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function blocks for the DALI commands do not directly access the process image of the KL6811, but place the individual DALI commands in a command buffer. The function block FB_KL6811Communication sequentially reads the DALI commands from the command buffer and forwards them to the KL6811. This prevents multiple function blocks accessing the KL6811 process image at the same time.

##### `FB_KL6821Communication` — §4.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetMaximumDemandCounter` | `BOOL` | — |
| `bResetOverflowCounter` | `BOOL` | — |
| `nOptions` | `DWORD := 0` | — |
| `bResetInactiveProcessImage` | `BOOL` | — |
| `bInitialise` | `BOOL := FALSE` | — |
| `eCommandKBusWatchdog` | `E_DALIConfigurationCommand := E_DALIConfigurationCommand.DoNothing` | — |
| `eCommandDI1RisingEdge` | `E_DALIConfigurationCommand := E_DALIConfigurationCommand.Off` | — |
| `eCommandDI1FallingEdge` | `E_DALIConfigurationCommand := E_DALIConfigurationCommand.DoNothing` | — |
| `eCommandDI2RisingEdge` | `E_DALIConfigurationCommand := E_DALIConfigurationCommand.RecallMaxLev` | — |
| `eCommandDI2FallingEdge` | `E_DALIConfigurationCommand := E_DALIConfigurationCommand.DoNothing` | — |
| `bDoNotLockProcessImage` | `BOOL := FALSE` | — |
| `bDisableInternalPowerSupply` | `BOOL := FALSE` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_KL6821InData` | — |
| `stOutData` | `ST_KL6821OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function blocks for the DALI commands do not directly access the process image of the KL6821, but place the individual DALI commands in a command buffer. The function block FB_KL6821Communication sequentially reads the DALI commands from the command buffer and forwards them to the KL6821. This prevents multiple function blocks accessing the KL6821 process image at the same time.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices

##### `B.E.G.` — §4.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the Indoor 180-BMS DALI-2 DALI sensor from B.E.G..

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › B.E.G.

##### `FB_DALI_BEG_PD11_BMS_V8_Flat` — §4.1.4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the PD11-BMS-FLAT DALI-2 DALI sensor from B.E.G.

##### `FB_DALI_BEG_PD2N_BMS_V8_FC` — §4.1.4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the PD2N-BMS-FC DALI-2 DALI sensor from B.E.G..

##### `FB_DALI_BEG_PD2N_BMS_V8_FM` — §4.1.4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the PD2N-BMS-V8-FM DALI-2 DALI sensor from B.E.G..

##### `FB_DALI_BEG_PD4_BMS_V8_GH_SM` — §4.1.4.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the PD4-BMS-GH-SM DALI-2 DALI sensor from B.E.G..

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Esylux

##### `FB_DALI_Esylux_PD_180_Basic_BMS` — §4.1.4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the PD 180/16 BASIC BMS DALI-2 DALI sensor from Esylux.

##### `FB_DALI_Esylux_PD_C_360_BMS` — §4.1.4.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_Esylux_PD_C_360_BMS represents the DALI sensors PD-C 360/8 BMS DALI-2, PD-C 360/24 BMS DALI-2 and PD-C 360/32 BMS DALI-2 from Esylux.

##### `FB_DALI_Esylux_PD_C_360_Mini_BMS` — §4.1.4.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_Esylux_PD_C_360_Mini_BMS represents the DALI sensor PD-C 360/8 mini BMS DALI-2 from Esylux.

##### `FB_DALI_Esylux_PD_FLAT_360_BMS` — §4.1.4.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_Esylux_PD_FLAT_360_BMS represents the DALI sensors PD-FLAT 360/8 RW BMS DALI-2, PD-FLAT 360/8 SW BMS DALI-2 and PD-FLAT-L 360/8 RW BMS DALI-2 from Esylux.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Glamox

##### `FB_DALI_Glamox_LMS_E2D_6InputModule` — §4.1.4.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** Functional description

##### `FB_DALI_Glamox_LMS_PB8_SwitchPanel` — §4.1.4.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the LMS DALI-2 PB8 Switch Panel DALI push button interface from Glamox.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Helvar

##### `FB_DALI_Helvar_2ButtonModule` — §4.1.4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the 2 Button DALI push button interface module from Helvar.

##### `FB_DALI_Helvar_4ButtonModule` — §4.1.4.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the 4 Button DALI push button interface module from Helvar.

##### `FB_DALI_Helvar_5ButtonModule` — §4.1.4.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the 5 Button DALI push button interface module from Helvar.

##### `FB_DALI_Helvar_6ButtonModule` — §4.1.4.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the 6 Button DALI push button interface module from Helvar.

##### `FB_DALI_Helvar_7ButtonModule` — §4.1.4.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the 7 Button DALI push button interface module from Helvar.

##### `FB_DALI_Helvar_8ButtonModule` — §4.1.4.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the 8 Button DALI push button interface module from Helvar.

##### `FB_DALI_Helvar_Multisensor_R60_HighBay` — §4.1.4.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the Multisensor High Bay DALI-2 R60.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices

##### `Loytec` — §4.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the LDALI-BM2 DALI push button interface from Loytec.

#### (top level)

##### `Push button blocked.` — §8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_ToPushButtonState [} 926] can be used to convert the state of a push button into a BOOL variable.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Loytec

##### `FB_DALI_Loytec_LDALI_MS4_BT` — §4.1.4.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_Loytec_LDALI_MS4 represents the LDALI-MS4-BT DALI sensor from Loytec.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Lunatone

##### `FB_DALI_Lunatone_CS` — §4.1.4.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_Lunatone_CS represents the DALI-2 sensor CS Integration from Lunatone.

##### `FB_DALI_Lunatone_CS_THP` — §4.1.4.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_Lunatone_CS_THP represents the DALI-2 sensor CS Integration THP from Lunatone.

##### `FB_DALI_Lunatone_CS_THP_AQ` — §4.1.4.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_Lunatone_CS_THP_AQ represents the DALI-2 Sensor CS Integration THP-AQ from Lunatone.

##### `FB_DALI_Lunatone_MC` — §4.1.4.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_Lunatone_MC represents the MC DALI push button interface from Lunatone.

##### `FB_DALI_Lunatone_MC4L` — §4.1.4.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the MC4L DALI push button interface from Lunatone.

##### `FB_DALI_Lunatone_Cross_Switch` — §4.1.4.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the Cross Switch DALI push button interface from Lunatone.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Niko

##### `FB_DALI_Niko_P46LR` — §4.1.4.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the DALI sensors P46LR for surface mounting, P46LR for SnapFit flush mounting and P46LR for flush-mounting box from Niko.

##### `FB_DALI_Niko_P46MR` — §4.1.4.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the DALI sensors P46MR for surface mounting, P46MR for SnapFit flush mounting and P46MR for flush-mounting box from Niko.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices

##### `Osram` — §4.1.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the DALI COUPLER E G2 DALI sensor from Osram.

#### (top level)

##### `Push button blocked.` — §8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_ToPushButtonState [} 926] can be used to convert the state of a push button into a BOOL variable.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices

##### `Siteco` — §4.1.4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the DALI sensor Connect 31 from Siteco (this device is registered in the DiiA product database under the company LEXIKOS).

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Smart Device

##### `FB_DALI_SmartDevice_SD360_8_Basic` — §4.1.4.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the SD360-8 Basic DALI sensor from SMART DEVICE.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Steinel

##### `FB_DALI_Steinel_ControlProDualHF` — §4.1.4.12.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the ControlPro Dual HF DALI sensor from Steinel.

##### `FB_DALI_Steinel_ControlProDualTech` — §4.1.4.12.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the ControlPro Dual Tech US Dali sensor from Steinel.

##### `FB_DALI_Steinel_ControlProDualUS` — §4.1.4.12.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the ControlPro Dual US DALI sensor from Steinel.

##### `FB_DALI_Steinel_ControlProIRQuattroHD` — §4.1.4.12.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the ControlPro IR Quattro HD DALI sensor from Steinel.

##### `FB_DALI_Steinel_ControlProSingleUS` — §4.1.4.12.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the DALI ControlPro Single US DALI sensor from Steinel.

##### `FB_DALI_Steinel_DualLightSensor` — §4.1.4.12.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the Dual Light Sensor AP/UP DALI sensor from Steinel.

##### `FB_DALI_Steinel_Hallway_IPD` — §4.1.4.12.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the Hallway IPD DALI sensor from Steinel.

##### `FB_DALI_Steinel_Hallway_IPD_ECO` — §4.1.4.12.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the Hallway IPD ECO DALI sensor from Steinel.

##### `FB_DALI_Steinel_HF360_II_IPD` — §4.1.4.12.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the HF 360 II IPD DALI sensor from Steinel.

##### `FB_DALI_Steinel_HF360_II_IPD_ECO` — §4.1.4.12.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the HF 360 II IPD ECO DALI sensor from Steinel.

##### `FB_DALI_Steinel_IRMicro` — §4.1.4.12.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the IR Micro DALI sensor from Steinel.

##### `FB_DALI_Steinel_IRQuattroMicro` — §4.1.4.12.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** ☺The function block represents the IR Quattro Micro DALI sensor from Steinel.

##### `FB_DALI_Steinel_IRQuattroSlimXS` — §4.1.4.12.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the IR Quattro Slim XS DALI sensor from Steinel.

##### `FB_DALI_Steinel_IS345MX` — §4.1.4.12.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the IS 345 MX AP/UP DALI sensor from Steinel.

##### `FB_DALI_Steinel_IS3360MX` — §4.1.4.12.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the IS 3360 MX AP/UP DALI sensor from Steinel.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Sunricher

##### `FB_DALI_Sunricher_PushButtonCoupler` — §4.1.4.13.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the Push Button Coupler DALI push button interface from Sunricher.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices

##### `Theben` — §4.1.4.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the PlanoSpot 360 DALI sensor from Theben.

#### (top level)

##### `Push button blocked.` — §8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_ToPushButtonState [} 926] can be used to convert the state of a push button into a BOOL variable.

##### `Push button blocked.` — §8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_ToPushButtonState [} 926] can be used to convert the state of a push button into a BOOL variable.

##### `Push button blocked.` — §8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_ToPushButtonState [} 926] can be used to convert the state of a push button into a BOOL variable.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Theben

##### `FB_DALI_Theben_TA_4_S` — §4.1.4.14.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** This function block represents the DALI push button interface TA 4 S DALI-2 from Theben.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Tridonic

##### `FB_DALI_Tridonic_MSensorG3_SFI_30_PIR` — §4.1.4.15.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the MSensor G3 SFI 30 PIR 5DPI, MSensor G3 SFI 30 PIR 10DPI or MSensor G3 SFI 30 PIR 16DPI DALI sensor from Tridonic.

##### `FB_DALI_Tridonic_MSensorG3_SRC_30_PIR` — §4.1.4.15.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the MSensor G3 SRC 30 PIR 5DPI DALI sensor from Tridonic.

#### (top level)

##### `Push button blocked.` — §8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALI_ToPushButtonState [} 926] can be used to convert the state of a push button into a BOOL variable.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Tridonic

##### `FB_DALI_Tridonic_PCell_SSI_31_PC_DA2_SA` — §4.1.4.15.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the DALI light sensor PCell SSI 31 PC DA2 SA from Tridonic.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices › Zencontrol

##### `FB_DALI_Zencontrol_PIR` — §4.1.4.16.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the PIR DALI-2 sensor from Zencontrol.

##### `FB_DALI_Zencontrol_Scenepanel_Switch` — §4.1.4.16.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the Scenepanel Switch DALI push button interface from Zencontrol.

##### `FB_DALI_Zencontrol_Switch_1Button` — §4.1.4.16.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents the Switch 1 Button DALI push button interface from Zencontrol.

#### Error while reading the variable minLevel [} 163]. › POUs › Devices

##### `FB_DALI103ControlDevice` — §4.1.4.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block is used for the configuration and operation of a DALI control device that supports Part 103 (control devices).

##### `FB_DALI300GenericInstance` — §4.1.4.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block is used for the direct reading of generic values of an instance of a DALI control device that supports Part 103 (instance type 0).

##### `FB_DALI301PushButton` — §4.1.4.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block is used for the configuration and operation of an instance of a DALI control device that supports Part 301 (push buttons).

##### `FB_DALI302AbsoluteInputDevice` — §4.1.4.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block is used for the configuration and operation of an instance of a DALI control device that supports Part 302 (absolute encoder).

##### `FB_DALI303OccupancySensor` — §4.1.4.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block is used for the configuration and operation of an instance of a DALI control device that supports Part 303 (occupancy sensor).

##### `FB_DALI304LightSensor` — §4.1.4.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block is used for the configuration and operation of an instance of a DALI control device that supports Part 304 (light sensor).

##### `FB_DALI306GeneralPurposeSensor` — §4.1.4.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitialize` | `BOOL` | The configuration of the DALI control device is started by a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block is used for the configuration and operation of an instance of a DALI control device that supports Part 306 (general purpose sensor).

##### `FB_DALI_ToPushButtonState` — §4.1.4.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nPushButtonEvent` | `UINT` | This variable contains the individual events of the push |

**Description:** Determines the static state from the events of a button.

#### (top level)

##### `Push button blocked.` — §8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nPushButton` | `BOOL` | This variable reflects the static state of the push button. |

**Description:** The function block FB_DALI_ToPushButtonState [} 926] can be used to convert the state of a push button into a BOOL variable.

#### Error while reading the variable minLevel [} 163]. › POUs › Events

##### `FB_DALIGetInputNotification` — §4.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | If this input is TRUE, all received Input Notification Events |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block filters all received Input Notification Events according to certain filter criteria.

##### `FB_DALIGetPowerCycleNotification` — §4.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | If this input is TRUE, all received Power Cycle Notification |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block FB_DALIGetPowerCycleNotification filters out all received Power Cycle Notification

#### (top level)

##### `100 %` — §254

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDALI` | `BYTE` | Output value for the DALI control gear (0…254). |

#### Error while reading the variable minLevel [} 163]. › POUs › Helper

##### `PERCENT_TO_DALI` — §4.1.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fPercent` | `LREAL` | Output value for the DALI control gear (0…100%). |

**Description:** Conversion of the output value of a DALI control gear from 0...100 % to 0...254.

#### Error while reading the variable minLevel [} 163]. › POUs › Simulation

##### `FB_DALIVirtualCommunication` — §4.1.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetMaximumDe BOOL` | `A positive edge resets the stored value for the maximum` | — |
| `mandCounter` | `utilization of the command buffer,` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The individual DALI commands are temporarily stored in a command buffer within the Tc3_DALI library before further processing. The FB_DALIVirtualCommunication function block sequentially reads the DALI commands from this command buffer and forwards the DALI commands to the associated instances of FB_DALIVirtualControlGear [} 935].

##### `FB_DALIVirtualControlGear` — §4.1.7.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | This output is switched to TRUE if an error occurs during |

**Description:** The function block represents a DALI control gear on a virtual DALI line.
