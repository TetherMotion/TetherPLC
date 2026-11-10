# Tc2_IoFunctions

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_IoFunctions_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T0 · **API entries:** 81
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** I/O helper functions over the process image: maps to %I/%Q/%M addressing.

---

#### Function blocks › General IO FBs

##### `IOF_DeviceReset` — §3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_DeviceReset function block resets an I/O device (e.g. a fieldbus card). The function corresponds to the online reset function from the TwinCAT->I/O->Devices->Device xyz context menu.

##### `IOF_GetBoxAddrByName` — §3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_GetBoxAddrByName function block determines the fieldbus address of a box (box = slave, module, station), knowing the box name and the device ID. If no fieldbus address is available, the function block returns a physical or a logical address. (In the case of the Beckhoff Lightbus, for instance, it is the physical box number in the optical fiber ring, while in Profibus it is the station address). The box identifier is passed as a string to the function block, and can be specified by the user during the configuration in the TwinCAT system.

##### `IOF_GetBoxAddrByNameEx` — §3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_GetBoxAddrByNameEx function block determines the fieldbus address of a box (box = slave, module, station), knowing the box name and the device name. If no fieldbus address is available, the function block returns a physical or a logical address. (In the case of the Beckhoff Lightbus, for instance, it is the physical box number in the fiber-optic ring, while in Profibus it is the station address). The box name and the device name are passed as strings to the function block, and can be specified by the user during the configuration in the TwinCAT system.

##### `IOF_GetBoxCount` — §3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `16 Function blocks` | `` | — |
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_GetBoxCount function block reads the number of configured and active boxes (box = slave, module, station) of an I/O device.

##### `IOF_GetBoxNameByAddr` — §3.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_GetBoxNameByAddr function block determines the box name, knowing the device ID and the fieldbus address of a box (box = slave, module, station). If no fieldbus address is available, a logical or physical address can be transferred to the function block as the fieldbus address (e.g. the physical box number in the fiber-optic ring for Beckhoff Lightbus). If successful the function block returns the box name configured in TwinCAT as a string.

##### `IOF_GetBoxNetId` — §3.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** Some boxes (slave modules) can be assigned an AmsNetId during the configuration in TwinCAT. The AmsNetId can then be used to execute firmware functions on the box. The IOF_GetBoxNetId function block determines the TwinCAT network address, knowing the device ID of the master and the fieldbus address or logical address in the fieldbus. The device IDs are specified by the TwinCAT system during the configuration and cannot be configured by the user.

##### `IOF_GetDeviceCount` — §3.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_GetDeviceCount function block reads the number of configured and active I/O devices.

##### `IOF_GetDeviceIDByName` — §3.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_GetDeviceIDByName function block determines the device ID of an I/O device, knowing the device name. If successful, the function block returns the device ID specified by the TwinCAT system during configuration. The device IDs cannot be configured by the user.

##### `IOF_GetDeviceIDs` — §3.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_GetDeviceIDs function block reads the device IDs of all configured and active I/O devices into a data buffer. The data buffer can be defined as an array of Word variables. If successful, the function block returns the total number of the device IDs that exist in the first data word, while the remaining data words contain the corresponding device IDs of the individual I/O devices. The device IDs are specified by the TwinCAT system during the configuration and cannot be configured by the user.

##### `IOF_GetDeviceInfoByName` — §3.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_GetDeviceInfoByName function block determines the device ID of an I/O device and its TwinCAT network address, knowing the device name. The device IDs cannot be configured by the user.

##### `IOF_GetDeviceName` — §3.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_GetDeviceName function block reads the device name of an I/O device. The device name can be specified by the user during the configuration in the TwinCAT system. This is then sent as a string to the I/O driver at system startup and can be read using the ADS commands. The DEVICEID input variable is used to specify the I/O device whose device name is to be read.

##### `IOF_GetDeviceNetId` — §3.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** Some I/O devices can be assigned a TwinCAT network address during the configuration in the TwinCAT system (e.g. FC310x Profibus card or CP9030 card). The network address can then be used to execute firmware functions on the device. The IOF_GetDeviceNetId function block determines the network address knowing the device ID. The device IDs are defined by the TwinCAT system during configuration and cannot be configured by the user.

##### `IOF_GetDeviceType` — §3.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The "IOF_GetDeviceType" function block determines the device type knowing the device ID. The device IDs are defined by the TwinCAT system during configuration and cannot be configured by the user.

#### Function blocks › ASI master terminal

##### `FB_ASI_Addressing` — §3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iOldAddress` | `BYTE` | Old address of the slave to be addressed (new slaves |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParameterBuffer` | `ST_ParameterBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

##### `FB_ASI_SlaveDiag` — §3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `iSlaveaddress` | `BYTE` | Slave address |
| `iCounter` | `INT` | 1:PhysicalFaultCounter, 2:TimeoutCounter, 3:ResponseCounter, |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParameterBuffer` | `ST_ParameterBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

##### `FB_ASI_ReadParameter` — §3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iParameterNumber` | `WORD` | Parameter number. |
| `bCycleMode` | `BOOL` | 0: Acyclic , 1:Cyclic (permanent Read/Write) If this bit is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParameterBuffer` | `ST_ParameterBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

##### `FB_ASI_WriteParameter` — §3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iParameterNumber` | `WORD` | Parameter number |
| `iParametervalue` | `DWORD` | Parameter value |
| `bStart` | `BOOL` | The corresponding order is executed with a positive edge |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParameterBuffer` | `ST_ParameterBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `32 Function blocks` | `` | — |
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

##### `FB_ASI_Processdata_digital` — §3.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iSlaveaddress` | `BYTE` | Slave address |
| `iSlavevalue` | `WORD` | Process value |
| `bParametermode` | `BOOL` | 0: Read, 1: Write |
| `bCycleMode` | `BOOL` | 0: Acyclic , 1:Cyclic (permanent Read/Write) If this bit is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParameterBuffer` | `ST_ParameterBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

##### `FB_ASI_ParameterControl` — §3.2.7

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParameterBuffer` | `ST_ParameterBuffer` | — |
| `stParameter_IN` | `ST_Parameter_IN` | — |
| `stParameter_OUT` | `ST_Parameter_OUT` | — |

**Description:** The FB_ASI_ParameterControl realizes the background communication between the ASI master terminal and the individual function blocks of the Lib.

##### `FB_ReadInput_analog` — §3.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iSlaveaddress` | `BYTE` | Slave address |
| `iChannel` | `BYTE` | Slave channel Function blocks |
| `bCycleMode` | `BOOL` | 0: Acyclic , 1:Cyclic (permanent Read/Write) If this bit is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParameterBuffer` | `ST_ParameterBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

##### `FB_WriteOutput_analog` — §3.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iSlaveaddress` | `BYTE` | Slave address |
| `iChannel` | `BYTE` | Slave channel |
| `iSlavevalue` | `WORD` | Data to be written |
| `bCycleMode` | `BOOL` | 0: Acyclic , 1:Cyclic (permanent Read/Write) If this bit is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParameterBuffer` | `ST_ParameterBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

#### Function blocks › AX200x Profibus

##### `FB_AX2000_Parameter` — §3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `iSlaveAddress` | `BYTE` | Station address |
| `iPnu` | `WORD` | Selection of the parameter to be read or written. List of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

**Description:** The function block FB_AX2000_Parameter is used to write/read the data for parameterizing the drive.

##### `FB_AX2000_AXACT` — §3.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iVelocity` | `DWORD` | The parameter contains the required velocity for a |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stPZDIN` | `ST_PZD_IN` | — |
| `stPZDOUT` | `ST_PZD_OUT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

**Description:** The FB_AX2000_AXACT function block is used to start axis actions (must always be called cyclically).

##### `FB_AX2000_JogMode` — §3.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | Start jog mode |
| `bStop` | `BOOL` | Stop jog mode |
| `iBasicVelo` | `INT` | Basic velocity; the actual velocity is derived from the basic |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stPZDIN` | `ST_PZD_IN` | — |
| `stPZDOUT` | `ST_PZD_OUT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

**Description:** The FB_AX2000_JogMode function block is used for jog mode.

##### `FB_AX2000_Reference` — §3.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bSetRefPoint` | `BOOL` | Setting the reference point. |
| `bCalibrStart` | `BOOL` | Start homing. |
| `bCalibrStop` | `BOOL` | Stop homing. |
| `iCalVelo` | `WORD` | Basic velocity for homing. The final velocity is composed of |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stPZDIN` | `ST_PZD_IN` | — |
| `stPZDOUT` | `ST_PZD_OUT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

**Description:** The FB_AX2000_Reference function block is used to set the reference point or start homing

##### `FB_AX200X_Profibus` — §3.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInit` | `BOOL` | Initialization of the drive. If bInit is TRUE then operation |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stPZDIN` | `ST_PZD_IN` | — |
| `stPZDOUT` | `ST_PZD_OUT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has executed a command. While Busy = TRUE, no new |

**Description:** The FB_AX200X_Profibus function block combines the three previous function blocks. It offers the complete interface to the AX2000 with access to all functions (except parameters).

#### Function blocks › Beckhoff Lightbus

##### `IOF_LB_BreakLocationTest` — §3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `46 Function blocks` | `` | — |
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_LB_BreakLocationTest function block carries out a break location test in a Beckhoff Lightbus fiber- optic ring and can locate possible break locations. If no break location is detected during the test, the output variable BOXNO returns the current number of Lightbus modules in the ring. If a break location is detected before the NN-th module in front of the receiver input, the BREAK flag is set and the module number is provided via the output variable BOXNO. If the BOXNO variable returns a value of 0xFF the break location is situated immediately in front of the receiver input, and cannot be located.

##### `IOF_LB_ParityCheck` — §3.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** Function blocks

##### `IOF_LB_ParityCheckWithReset` — §3.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_LB_ParityCheckWithReset function block reads the parity error counters of the Beckhoff Lightbus modules (e.g. BK2000). The counters are then reset. The master maintains an 8-bit error counter for each module. These counters work without overflow. A maximum of 256 bytes of data, and therefore 256 counters, can be read. The number of error counters to be read is specified by the input variables: LEN and DESTADDR. If, for instance, there are only 5 modules in the ring, then the DESTADDR parameter can be supplied with the address of a data buffer of 5 bytes, and the LEN parameter can be supplied with the value 5.

#### Function blocks › Beckhoff UPS (configured with Windows UPS Service

##### `FB_GetUPSStatus` — §3.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bValid` | `BOOL` | If this output is set, the data in the ST_UPSStatus structure |

#### Function blocks › Bus Terminal configuration

##### `FB_KL1501Config` — §3.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConfigurate` | `BOOL` | A rising edge starts the configuration sequence. First, the |

**Description:** The FB_KL1501Config function block is used to parameterize a KL1501: 1-channel counter terminal.

#### (top level)

##### `32-bit gated counter, gate input High disables the counter` — §3

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_KL1501InData` | — |
| `stOutData` | `ST_KL1501OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | As long as the read or configuration sequence is in |

#### Function blocks › Bus Terminal configuration

##### `FB_KL27x1Config` — §3.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConfigurate` | `BOOL` | A rising edge starts the configuration sequence. First, the |

**Description:** The FB_KL27x1Config function block is used to parameterize a KL2751 / KL2761: 1-channel dimmer terminal.

#### (top level)

##### `Rectifier mode, negative (negative half-wave with leading edge` — §4

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_KL1501InData` | — |
| `stOutData` | `ST_KL1501OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bBusy` | `BOOL` | As long as the read or configuration sequence is in |

#### Function blocks › Bus Terminal configuration

##### `FB_KL320xConfig` — §3.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConfigurate` | `BOOL` | A rising edge starts the configuration sequence. First, the |

**Description:** The FB_KL320xConfig function block is used to parameterize a KL3201, KL3202 or KL3204: input terminal for resistance sensors.

#### (top level)

##### `PT1000 - two-wire connection - not permitted when using a` — §9

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_KL1501InData` | — |
| `stOutData` | `ST_KL1501OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | As long as the read or configuration sequence is in |

#### Function blocks › Bus Terminal configuration

##### `FB_KL3208Config` — §3.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConfigurate` | `BOOL` | A rising edge starts the configuration sequence. First, the |

**Description:** The FB_KL3208Config function block is used to parameterize a KL3208-0010: 8-channel input terminal for resistance sensors.

#### (top level)

##### `NTC100K` — §18

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_KL1501InData` | — |
| `stOutData` | `ST_KL1501OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | As long as the read or configuration sequence is in |

#### Function blocks › Bus Terminal configuration

##### `FB_KL3228Config` — §3.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConfigurate` | `BOOL` | A rising edge starts the configuration sequence. First, the |

**Description:** The FB_KL3228Config function block is used to parameterize a KL3228: 8-channel input terminal for resistance sensors.

#### (top level)

##### `RSNI1000 (NI1000 according to Landis&Staefa characteristics:` — §2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_KL1501InData` | — |
| `stOutData` | `ST_KL1501OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `64 Function blocks` | `` | — |
| `bBusy` | `BOOL` | As long as the read or configuration sequence is in |

#### Function blocks › CANopen

##### `IOF_CAN_Layer2Command` — §3.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_CAN_Layer2Command function block sends a 10-byte command to layer 2 of a CAN master.

#### Function blocks › NOV/DP-RAM

##### `FB_NovRamReadWrite` — §3.8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDevId` | `UDINT` | The device ID of a NOVRAM card. The ID is used to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The following description refers to TwinCAT Version 2.8. From TwinCAT version 2.9 [Build 927], a function block for writing or reading PLC data to be saved in NOVRAM is no longer required.

##### `FB_NovRamReadWriteEx` — §3.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `nDevId` | `UDINT` | The device ID of a NOVRAM card. The ID is used to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The FB_NovRamReadWriteEx function block accesses the NOV-RAM (e.g. of the FCxxxx-0002 fieldbus cards, CX9000 NOVRAM, etc.) from a PLC program. A rising edge at the bRead or bWrite input activates the function block and a corresponding number of data bytes are read from the NOV-RAM or written to the NOVRAM. If both inputs: bRead and bWrite are set at the same time, the data is first written to the NOVRAM and then read back. The difference to the FB_NovRamReadWrite function block is that the address offset can be specified in the NOVRAM for write and read accesses. The function block also checks the permitted type of access to the NOVRAM memory and, if necessary, copies the data byte by byte into the NOVRAM memory instead of using a MEMCPY. The NOVRAM of the CX9000 only allows byte accesses, for example, and the FB_NovRamReadWrite function block would return an error in this case.

##### `FB_GetDPRAMInfo` — §3.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDevId` | `UDINT` | The device ID of a NOV/DPRAM card. The ID is used to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The FB_GetDPRAMInfo function block determines the address pointer and the configured size of the NOV/ DP-RAM of a fieldbus card. The address pointer can be used, for example, for direct write or read access of the NOV-RAM of the FCxxx-0002 (Beckhoff) cards or the DPRAM of cards that are not supported by TwinCAT (third-party manufacturers). First, the card has to be configured as general NOV/DP-RAM within the TwinCAT system. The MEMCPY, MEMSET or MEMCMP functions of the PLC program can then be used for write/read access to any memory offset.

##### `FB_GetDPRAMInfoEx` — §3.8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `nDevId` | `UDINT` | The device ID of a NOV/DPRAM card. The ID is used to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The FB_GetDPRAMInfoEx function block determines the address pointer and the configured size of the NOV/DP-RAM of a fieldbus card. The address pointer can be used, for example, for direct write or read access of the NOV-RAM of the Fcxxx-0002 (Beckhoff) cards or the DPRAM of cards that are not supported by TwinCAT (third-party manufacturers). First, the card has to be configured as general NOV/DP-RAM within the TwinCAT system.

#### Function blocks › Profibus DPV1 (Sinamics)

##### `F_CreateDpv1ReadReqPkg` (FUNCTION) → `USINT` — §3.9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDpv1ReqData` | `ARRAY OF BYTE` | Pointer to an array of 240 bytes containing the DPV1 read |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDpv1Parameter` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ParamAddrEx;(* list of parameters *)` | — |

**Description:** The F_CreateDpv1ReadReqPkg function generates a DPV1 telegram for a FB_Dpv1Read [} 78] of one or more parameters of a drive or the controller unit of a Sinamics Profidrive (Profidrive Specification 3.1). As Profidrives use the Motorola format and IPCs the Intel format, the function automatically swaps the byte order in the DPV1 telegram of parameters with data types with more than one byte.

##### `F_CreateDpv1WriteReqPkg` (FUNCTION) → `USINT` — §3.9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDpv1ReqData` | `ARRAY OF BYTE` | Pointer to an array of 240 bytes containing the DPV1 write |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDpv1Parameter` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ParamAddrEx;(* list of parameters *)` | — |
| `stDpv1ValueHeaderEx` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ValueHeaderEx;(* list of values *)` | — |

**Description:** The F_CreateDpv1WriteReqPkg function generates a DPV1 telegram for a FB_Dpv1Write [} 81] of one or more parameters of a drive or the controller unit of a Sinamics Profidrive (Profidrive Specification 3.1). As Profidrives use the Motorola format and IPCs the Intel format, the function automatically swaps the byte order in the DPV1 telegram of parameters with data types with more than one byte.

##### `F_SplitDpv1ReadResPkg` (FUNCTION) → `USINT` — §3.9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDpv1ResData` | `ARRAY OF BYTE` | Pointer to an array of 240 bytes containing the DPV1 read |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDpv1Parameter` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ParamAddrEx;(* list of parameters *)` | — |
| `stDpv1ValueHeaderEx` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ValueHeaderEx;(* list of values *)` | — |

**Description:** The F_SplitDpv1ReadResPkg function evaluates a DPV1 telegram of a FB_Dpv1Read [} 78] of one or more parameters of a drive or the controller unit of a Sinamics Profidrive (Profidrive Specification 3.1). As Profidrives use the Motorola format and IPCs the Intel format, the function automatically swaps the byte order in the DPV1 telegram of parameters with data types with more than one byte.

##### `F_SplitDpv1WriteResPkg` (FUNCTION) → `USINT` — §3.9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDpv1ResData` | `ARRAY OF BYTE` | Pointer to an array of 240 bytes containing the DPV1 write |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDpv1Parameter` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ParamAddrEx;(* list of parameters *)` | — |
| `stDpv1ValueHeaderEx` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ValueHeaderEx;(* list of values *)` | — |

**Description:** Function blocks

##### `FB_Dpv1Read` — §3.9.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `bBusy` | `BOOL` | This output goes TRUE as soon as the function block is |

**Description:** The FB_Dpv1Read function block reads one or more parameters of a Sinamics Profidrive via DPV1 (Profidrive Specification 3.1). The DPV1 read telegram must be created with F_CreateDpv1ReadReqPkg [} 75] before a rising edge is present at bExecute. The DPV1 answer telegram must be evaluated with F_SplitDpv1ReadResPkg [} 77] after a falling edge appears at bBusy.

##### `FB_Dpv1Write` — §3.9.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output goes TRUE as soon as the function block is |

**Description:** The FB_Dpv1Write function block writes one or more parameters of a Sinamics Profidrive via DPV1 (Profidrive Specification 3.1). The DPV1 write telegram must be created with F_CreateDpv1WriteReqPkg [} 76] before a rising edge is present at bExecute. The DPV1 answer telegram must be evaluated with F_SplitDpv1WriteResPkg [} 77] after a falling edge appears at bBusy.

#### Function blocks › Profinet DPV1 (Sinamics)

##### `F_CreateDpv1ReadReqPkgPNET` (FUNCTION) → `USINT` — §3.10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDpv1ReqData` | `POINTER TO ARRAY` | Pointer to an array of 240 bytes containing the DPV1 read telegram. This telegram must be created by the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDpv1Parameter` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ParamAddrEx;(* list of parameters *)` | — |

**Description:** The function F_CreateDpv1ReadReqPkg generates a DPV1 telegram for a FB_Dpv1ReadPNET [} 86] of one or more parameters of a drive or the controller unit of a Sinamics Profidrive (Profidrive Specification 3.1) that is connected via Profinet. As Profidrives use the Motorola format and IPCs the Intel format, the function automatically swaps the byte order in the DPV1 telegram of parameters with data types with more than one byte.

##### `F_CreateDpv1WriteReqPkgPNET` (FUNCTION) → `USINT` — §3.10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDpv1ReqData` | `ARRAY OF BYTE` | Pointer to an array of 240 bytes containing the DPV1 write |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDpv1Parameter` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ParamAddrEx;(* list of parameters *)` | — |
| `stDpv1ValueHeaderEx` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ValueHeaderEx;(* list of values *)` | — |

**Description:** The function F_CreateDpv1WriteReqPkgPNET generates a DPV1 telegram for one FB_Dpv1WritePNET [} 89] of one or more parameters of a drive or the controller unit of a Sinamics Profidrive (Profidrive Specification 3.1) that is connected via Profinet. As Profidrives use the Motorola format and IPCs the Intel format, the function automatically swaps the byte order in the DPV1 telegram of parameters with data types with more than one byte.

##### `F_SplitDpv1ReadResPkgPNET` (FUNCTION) → `USINT` — §3.10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDpv1ResData` | `POINTER TO ARRAY` | Pointer to an array of 240 bytes containing the DPV1 read |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDpv1Parameter` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ParamAddrEx;(* list of parameters *)` | — |
| `stDpv1ValueHeaderEx` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ValueHeaderEx;(* list of values *)` | — |

**Description:** The function F_SplitDpv1ReadResPkgPNET evaluates a DPV1 telegram of a FB_Dpv1ReadPNET [} 86] of one or more parameters of a drive or the controller unit of a Sinamics Profidrive (Profidrive Specification 3.1) that is connected via Profinet. As Profidrives use the Motorola format and IPCs the Intel format, the function automatically swaps the byte order in the DPV1 telegram of parameters with data types with more than one byte.

##### `F_SplitDpv1WriteResPkgPNET` (FUNCTION) → `USINT` — §3.10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDpv1ResData` | `POINTER TO ARRAY` | Pointer to an array of 240 bytes containing the DPV1 write |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDpv1Parameter` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ParamAddrEx;(* list of parameters *)` | — |
| `stDpv1ValueHeaderEx` | `ARRAY [1..iMAX_DPV1_PARAMS] OF ST_Dpv1ValueHeaderEx;(* list of values *)` | — |

**Description:** The function F_SplitDpv1WriteResPkgPNET evaluates a DPV1 telegram of a FB_Dpv1WritePNET [} 89] of one or more parameters of a drive or the controller unit of a Sinamics Profidrive (Profidrive Specification 3.1) that is connected via Profinet. As Profidrives use the Motorola format and IPCs the Intel format, the function automatically swaps the byte order in the DPV1 telegram of parameters with data types with more than one byte.

##### `FB_Dpv1ReadPNET` — §3.10.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output goes TRUE as soon as the function block is |

**Description:** 86

##### `FB_Dpv1WritePNET` — §3.10.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output goes TRUE as soon as the function block is |

**Description:** The function block FB_Dpv1WritePNET writes one or more parameters of a Sinamics Profidrive via DPV1 (Profidrive Specification 3.1) via Profinet. The DPV1 write telegram must be created with F_CreateDpv1WriteReqPkgPNET [} 84] before a rising edge is present at bExecute. The DPV1 answer telegram must be evaluated with F_SplitDpv1WriteResPkgPNET [} 86] after bBusy indicates a falling edge.

#### Function blocks › RAID Controller

##### `FB_RAIDFindCntlr` — §3.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNETID` | `T_AmsNetID` | Is a string containing the AMS network ID of the target |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stRAIDCntlrFound` | `ST_RAIDCntlrFound` | Contains the number of RAID controllers found and their [} 140] RAID controller IDs. |
| `nBytesRead` | `UDINT` | Number of successfully returned data bytes. |
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** This function returns the counter value of the RAID controller and the corresponding controller IDs.

##### `FB_RAIDGetInfo` — §3.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `92 Function blocks` | `` | — |
| `sNETID` | `T_AmsNetID` | Is a string containing the AMS network ID of the target |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stRAIDInfo` | `ST_RAIDInfo [} 141]` | Returns a RAID info containing the number of RAID |

**Description:** This function block returns a RAID info containing the number of RAID controller sets and the maximum number of drives per set.

##### `FB_RAIDGetStatus` — §3.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNETID` | `T_AmsNetID` | Is a string containing the AMS network ID of the target |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stRAIDStatusRes` | `ST_RAIDStatusRes [} 141] This structure contains the RAID status response, the` | — |

**Description:** This function block returns the RAID set index, the RAID type, the RAID status, the number of RAID drives and the status of the RAID drives.

#### Function blocks › SERCOS

##### `IOF_SER_GetPhase` — §3.12.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_SER_GetPhase function block determines the current communication phase on the SERCOS ring. The communication phases can have values from 0 to 4.

##### `IOF_SER_SaveFlash` — §3.12.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_SER_SaveFlash function block checks the system parameters located in the DPRAM memory. If no error is present it activates them and saves them in the EEPROM. The function block can adjust system parameters in the EEPROM of the controller to suit the application.

##### `IOF_SER_ResetErr` — §3.12.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_SER_ResetErr function block clears the following errors of a SERCOS master: • The errors in the existing drives; • The diagnostic status in the diagnostic channel of the existing drives; • The system error;

##### `IOF_SER_SetPhase` — §3.12.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_SER_SetPhase function block carries out the phase start-up in the SERCOS ring to a specific value.

##### `IOF_SER_IDN_Read` — §3.12.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `cbRead` | `UDINT` | Number of bytes read and copied to dwDestAddr. |
| `nAttribRd` | `DWORD` | Attribute of the parameter; can be saved for the next |

**Description:** The IOF_SER_IDN_Read function block allows a value to be read from an S or P parameter of a Sercos drive. Data type and size are determined automatically on the basis of the attribute.

##### `IOF_SER_IDN_Write` — §3.12.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nAttribRd` | `DWORD` | Attribute of the parameter; can be saved for the next |

**Description:** The IOF_SER_IDN_Write function block allows a value to be written to an S or P parameter of a Sercos drive. Data type and size are determined automatically on the basis of the attribute.

##### `IOF_SER_DRIVE_Backup` — §3.12.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bCheck` | `BOOL` | A positive edge at this input activates the CRC and |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_SER_DRIVE_Backup function block allows the backup and restore of the drive data (S and P parameters) of the PLC in a binary file. The list of S and P parameter data to be backed up is taken from the Sercos parameter IDN192 by default. Backup and restore require the SERCOS parameter mode (phase 2).

##### `IOF_SER_DRIVE_BackupEx` — §3.12.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bCheck` | `BOOL` | A positive edge at this input activates the CRC and |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrList` | `ST_SercosParamList` | — |
| `arrSkippedList` | `ST_SercosParamErrList` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_SER_DRIVE_BackupEx function block allows the drive data (S and P parameters) to be backed up and restored via the PLC to a binary file or back to the drive. The list of S and P parameter data to be backed up is taken from the Sercos parameter IDN192 by default. Backup and restore require the SERCOS parameter mode (phase 2).

##### `IOF_SER_DRIVE_Reset` — §3.12.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The IOF_SER_DRIVE_Reset function block resets a Sercos drive. Drive errors are cleared.

#### Function blocks › TcTouchLock

##### `FB_TcTouchLock_AcquireFocus` — §3.13.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEnable` | `BOOL` | TRUE = request focus, FALSE = release focus |
| `sSetID` | `STRING` | ID of the device |
| `tLEDTime` | `TIME` | The output LED flashes at the specified interval (100 ms – |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `108 Function blocks` | `` | — |
| `bAcquired` | `BOOL` | TRUE if the client has the focus; FALSE if the client loses |

**Description:** The FB_TcTouchLock_AcquireFocus function block is used to prevent parallel, interfering inputs via several multi-touch control panels connected to one IPC. To this end, the focus is placed on one of the connected control panels, while inputs from all other connected control panels are blocked. The FB_TcTouchLock_AcquireFocus function block can be used to request and release the focus.

#### Function blocks › Third party devices › Phoenix IBS SC/I-T

##### `SCIT_ActivateConfiguration` — §3.14.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the computer in which the card is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The SCIT_ActivateConfiguration function block is used as an auxiliary function block to perform an Activate_Configuration on the Interbus card, which is addressed with the NETID and the PORT. An Activate_Configuration sets the card in the ACTIVE [} 110] state.

##### `SCIT_DeactivateConfiguration` — §3.14.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the computer in which the card is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The SCIT_DeactivateConfiguration function block is used as an auxiliary function block to perform a Deactivate_Configuration on the Interbus card, which is addressed with the NETID and the PORT. A Deactivate_Configuration sets the card to the PAR_READY [} 110] state and resets all outputs.

##### `SCIT_StartDataTransfer` — §3.14.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the computer in which the card is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `114 Function blocks` | `` | — |
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The SCIT_StartDataTransfer function block is used as an auxiliary function block to carry out a Start_Data_Transfer on the Interbus card, which is addressed with the NETID and the PORT. A Start_Data_Transfer places the card into the RUN [} 110] state.

##### `SCIT_StopDataTransfer` — §3.14.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the computer in which the card is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The SCIT_StopDataTransfer function block is used as an auxiliary function block to perform a Stop_Data_Transfer on the Interbus card that is addressed with the NETID and the PORT. A Stop_Data_Transfer sets the card to the ACTIVE [} 110] state, the outputs are not reset.

##### `SCIT_AlarmStop` — §3.14.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the computer in which the card is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The SCIT_AlarmStop function block is used as an auxiliary function block to perform an Alarm_Stop on the Interbus card that is addressed with the NETID and the PORT. An Alarm_Stop sets the card to the PAR_READY [} 110] state and all outputs are reset.

##### `SCIT_ControlActiveConfiguration` — §3.14.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the computer in which the card is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The SCIT_ControlActiveConfiguration function block is used as an auxiliary function block to perform a Control_Active_Configuration on the Interbus card, which is addressed with the NETID and the PORT. A Control_Active_Configuration can be used to change the state of a device (or several if the specified device is part of a group).

##### `SCIT_GetErrorInfo` — §3.14.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the computer in which the card is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The SCIT_GetErrorInfo function block reads the exact error cause and the exact error location of a previously occurred bus error from the Interbus card, which is addressed with the NETID and the PORT.

##### `SCIT_ConfDevErrAll` — §3.14.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NETID` | `T_AmsNetId` | The network address of the computer in which the card is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `120 Function blocks` | `` | — |
| `BUSY` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The SCIT_ConfDevErrAll function block acknowledges periphery errors of all existing devices simultaneously. The Control_Device_Function of the Interbus card is called internally. The interbus card is addressed with the NETID and the PORT.

#### Function blocks › Third party devices › ads-tec

##### `FB_ReadAdsTecSysData` — §3.14.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | A string with the network address of the TwinCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** The FB_ReadAdsTecSysData function block reads the system data/diagnostic data of an ads-tec industrial PC. The function block is level-triggered, i.e. the system data is only read cyclically when the bEnable input is set. To keep the system load low, the read cycle is automatically repeated every ~100 ms (default value). When the bValid output is set, the most recently read data is valid (i.e. the last read cycle was carried out without error). If an error occurs, the bError output is set, and cyclic reading is stopped. With a new rising edge at the bEnable input, existing errors can be deleted and cyclical reading can be restarted.

#### 1s › [Obsolete]

##### `F_GetVersionTcIoFunctions` (FUNCTION) → `UINT` — §4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | Version element to be read. Possible parameters: |

**Description:** This function is obsolete and should not be used. Please use the global constant stLibVersion_Tc2_IoFunctions [} 150] in order to read version information from the PLC library.

##### `F_GetVersionRAIDController` (FUNCTION) → `UINT` — §4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | Version element to be read. Possible parameters: |

**Description:** This function is obsolete and should not be used. Please use the global constant: stLibVersion_Tc2_IoFunctions [} 150] to read version information from the PLC library.
