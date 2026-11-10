# Tc2_MPBus

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_MPBus_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 44
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** MP-Bus: serial/DALI-2 variant master.

---

#### POUs

##### `Function blocks` — §4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `KL6771_IN` | `ARRAY OF BYTE` | Is linked with the MP-Bus master terminal's input process |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `KL6771_OUT` | `ARRAY OF BYTE` | Is linked with the MP-Bus master terminal's output process |

**Description:** 24

#### POUs › Function blocks

##### `MP_Addressing` — §4.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | The MP address that is to be used for the addressing or for |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block allows an MP-Bus slave to be addressed. It is also possible to use this function block to read the serial number of the slave. MP_Address is used to transfer the address of the slave that is addressed or whose serial number is to be read. The serial number is read out on a rising edge of bReadAddr. It is output through MP_Serial_Numer_Out. A rising edge of bAddrAuto addresses a slave whose address is MP_Address. The function block waits for the time set in TMOUT for the transmission of the slave with its serial number. Transmission of the serial number is initiated differently from one slave to another. Please see the documentation for the MP-Bus device for how the serial number can be sent (in most cases there is a switch on the drive which will trigger this action when pressed). No telegrams are sent to the slaves during the time specified by TMOUT.

##### `MP_CMV` — §4.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to control and monitor a volume flow regulator.

##### `MP_DamperLinearActuator` — §4.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `ActValue` | `WORD` | Contains the current position of the actuator (0...100 %). |
| `bMP_Sensor_Digi` | `BOOL` | If a digital sensor is connected, its state is indicated |

**Description:** This function block is used to control and monitor a drive of a damper and of a globe valve.

##### `MP_EnergyValveV4_2_Configuration` — §4.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `bRead_Write` | `BOOL` | If FALSE then READ only; if TRUE then READ and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure the Energy Valve actuators EV..R2+.. with firmware 4.2.0 or higher. For more information please visit www.belimo.com.

##### `MP_EnergyValveV4_2_Process` — §4.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is suitable for Energy Valve actuators EV..R2+.. with firmware 4.2.0 or higher. For more information please visit www.belimo.com.

##### `MP_EnergyValveV4_Configuration` — §4.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `bRead_Write` | `BOOL` | If FALSE then READ only; if TRUE then READ and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure the Energy Valve actuators EV..R2+.. with firmware 4.0.0 or higher. For more information please visit www.belimo.com.

##### `MP_EnergyValveV4_Process` — §4.1.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is suitable for Energy Valve actuators EV..R2+.. with firmware 4.0.0 or higher. For more information please visit www.belimo.com.

##### `MP_EPIV` — §4.1.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to control a pressure-independent control ball valve. For more information please visit www.belimo.com.

##### `MP_EPIV_R6_Parameter` — §4.1.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to parameterize drives of series EP..R-R6+BAC.

##### `MP_EPIV_R6_V4_Configuration` — §4.1.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `bRead_Write` | `BOOL` | If FALSE then READ only; if TRUE then READ and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure the 6-way EPIV V4 DN 15…25 EP..R6 + BAC-HHx. For more information please visit www.belimo.com.

##### `MP_EPIV_R6_V4_Process` — §4.1.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is suitable for 6-way EPIV V4 DN 15…25 EP..R6 + BAC-HHx. For more information please visit www.belimo.com.

##### `MP_EPIV_V4_Configuration` — §4.1.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `bRead_Write` | `BOOL` | If FALSE then READ only; if TRUE then READ and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure the 2-way EPIV V4 DN 15..50 EP..R2+(K)BAC. For more information please visit www.belimo.com.

##### `MP_EPIV_V4_Process` — §4.1.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is suitable for 2-way EPIV V4 DN 15..50 EP..R2+(K)BAC. For more information please visit www.belimo.com.

##### `MP_EV` — §4.1.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to control a control ball valve of series P6..W..EV-BAC. For more information please visit www.belimo.com.

##### `MP_EV_Parameter` — §4.1.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to parameterize drives.

##### `MP_FlowMeter_Configuration` — §4.1.1.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `bRead_Write` | `BOOL` | If FALSE then READ only; if TRUE then READ and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure the flow meters (FM). For more information please visit www.belimo.com.

##### `MP_FlowMeter_Process` — §4.1.1.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is suitable for the flow meters (FM). For more information please visit www.belimo.com.

##### `MP_JR_Actuator_Configuration` — §4.1.1.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `bRead_Write` | `BOOL` | If FALSE then READ only; if TRUE then READ and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure the rotary actuators for butterfly valves JR. Further information can be found at www.belimo.com.

##### `MP_JR_Actuator_Process` — §4.1.1.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is suitable for the rotary actuators for butterfly valves JR. Further information can be found at www.belimo.com.

##### `MP_MPX` — §4.1.1.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** For BELIMO room sensor MS24A-R..-MPX

##### `MP_OperatingUnit` — §4.1.1.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `strDataKL6771` | `DataKL6771 [} 134]` | The data structure with which the KL6771() [} 26] function |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to read room sensors of types P-22Rxx-1900x-1. It is compatible with 22Rxx-19-1 room sensors produced as of May 2022 (before May 2022, see MP_RoomSensor [} 87]). For more information, see www.belimo.com.

##### `MP_OperatingUnit_ConfigurationDisplay` — §4.1.1.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `strDataKL6771` | `DataKL6771 [} 134]` | The data structure with which the KL6771() [} 26] function |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure room sensors of types P-22Rxx-1900x-1. It is compatible with 22Rxx-19-1 room sensors produced as of May 2022 (before May 2022, see MP_RoomSensor_Parameter [} 89]). For more information, see www.belimo.com.

##### `MP_OperatingUnit_ConfigurationOffsetValues` — §4.1.1.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `strDataKL6771` | `DataKL6771 [} 134]` | The data structure with which the KL6771() [} 26] function |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure room sensors of types P-22Rxx-1900x-1. It is compatible with 22Rxx-19-1 room sensors produced as of May 2022 (before May 2022, see MP_RoomSensor_Parameter [} 89]). For more information, see www.belimo.com.

##### `MP_OperatingUnit_ConfigurationStatusIcons` — §4.1.1.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `strDataKL6771` | `DataKL6771 [} 134]` | The data structure with which the KL6771() [} 26] function |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure room sensors of types P-22Rxx-1900x-1. It is compatible with 22Rxx-19-1 room sensors produced as of May 2022 (before May 2022, see MP_RoomSensor_Parameter [} 89]). For more information, see www.belimo.com.

##### `MP_OperatingUnit_ConfigurationTemp` — §4.1.1.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `strDataKL6771` | `DataKL6771 [} 134]` | The data structure with which the KL6771() [} 26] function |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure room sensors of types P-22Rxx-1900x-1. It is compatible with 22Rxx-19-1 room sensors produced as of May 2022 (before May 2022, see MP_RoomSensor_Parameter [} 89]). For more information, see www.belimo.com.

##### `MP_OperatingUnit_ConfigurationVentilation` — §4.1.1.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT := 1` | — |
| `bStart` | `BOOL` | — |
| `strDataKL6771` | `DataKL6771` | — |
| `fVentilationStage` | `LREAL` | — |
| `nSystemOperationMode` | `E_MP_SystemOperationMode` | — |
| `bVentilationStageControlMode` | `BOOL` | — |
| `nDisplayModeVentilationStage` | `E_MP_DisplayModeVentilationStage := MPBus_DisplayModeVentilationSta` | — |
| `bVentilationControlMode` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure room sensors of types P-22Rxx-1900x-1. It is compatible with 22Rxx-19-1 room sensors produced as of May 2022 (before May 2022, see MP_RoomSensor_Parameter [} 89]). For more information, see www.belimo.com.

##### `MP_PTH` — §4.1.1.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to control and monitor a PTH sensor.

##### `MP_RoomSensor` — §4.1.1.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to read out room sensors of types 22Rxx-19-1 that were produced before May 2022 (as of May 2022, see MP_OperatingUnit [} 75]). For more information, see www.belimo.com.

##### `MP_RoomSensor_Parameter` — §4.1.1.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bWrite` | `BOOL` | A positive edge starts the function block and writes the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to parameterize room sensors of types 22Rxx-19-1 that were produced before May 2022 (as of May 2022, see MP_OperatingUnit_Configuration). For more information, see www.belimo.com.

##### `MP_Smoker` — §4.1.1.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to control and monitor a fire damper.

##### `MP_SR_actuator_Configuration` — §4.1.1.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `bRead_Write` | `BOOL` | If FALSE then READ only; if TRUE then READ and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure SR actuators. For more information please visit www.belimo.com.

##### `MP_SR_actuator_Process` — §4.1.1.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is suitable for SR actuators. For more information please visit www.belimo.com.

##### `MP_TEM_Configuration` — §4.1.1.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `bRead_Write` | `BOOL` | If FALSE then READ only; if TRUE then READ and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure the Thermal Energy Meter type 22PE-.. and 22PEM-.. . For more information please visit www.belimo.com.

##### `MP_TEM_Process` — §4.1.1.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is suitable for the Thermal Energy Meter type 22PE-.. and 22PEM-.. . For more information please visit www.belimo.com.

##### `MP_VAV` — §4.1.1.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to control and monitor a volume flow regulator.

##### `MP_VRU_Configuration` — §4.1.1.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. |
| `bRead_Write` | `BOOL` | If FALSE then READ only; if TRUE then READ and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to configure the VAV actuators VRU-D3-BAC, VRU-M1-BAC and VRU-M1R-BAC (max. 8 slaves). For more information please visit www.belimo.com.

##### `MP_VRU_Process` — §4.1.1.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is suitable for VAV actuators VRU-D3-BAC, VRU-M1-BAC and VRU-M1R-BAC (max. 8 slaves). For more information please visit www.belimo.com.

##### `MP_Window` — §4.1.1.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block is used to control and monitor a window ventilation system (FLS).

##### `MPL_DamperLinearActuator` — §4.1.1.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Address` | `USINT` | MP-Bus address of the slave. |
| `bStart` | `BOOL` | A positive edge starts the function block. If this remains |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This bit is set for as long as the function block is active. |
| `ActValue` | `WORD` | Contains the current position (0...100 %) of the actuator. |
| `iErrorID` | `MP_ERROR [} 131]` | This output outputs an error code in the event of an error. |

**Description:** This function block is used to control and monitor a drive of a damper and of a globe valve.

#### POUs › Functions

##### `NI1000_LuS_TO_INT: INT` — §4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Value` | `WORD` | — |
| `MP_Value` | `Input for an ohmic NI1000 L&S sensor` | — |

**Description:** This function calculates a temperature from the value of an NI1000 L&S resistor.

##### `NI1000_TO_INT: INT` — §4.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Value` | `WORD` | — |
| `MP_Value` | `Input for an ohmic NI1000 sensor` | — |

**Description:** This function calculates a temperature from the value of an NI1000 resistor.

##### `NTC_TO_INT: INT` — §4.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Value` | `WORD` | — |
| `MP_Value` | `Input for an ohmic NTC sensor` | — |

**Description:** This function calculates a temperature from the value of an NTC resistor.

##### `PT1000_TO_INT: INT` — §4.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `MP_Value` | `WORD` | — |
| `MP_Value` | `Input for an ohmic PT1000 sensor` | — |

**Description:** This function calculates a temperature from the value of a PT1000 resistor.
