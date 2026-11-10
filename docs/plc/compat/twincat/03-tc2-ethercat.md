# Tc2_EtherCAT

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_EtherCAT_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T2 · **API entries:** 97
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** EtherCAT mailbox/state access needs master service hooks.

---

#### EtherCAT Commands

##### `FB_EcPhysicalReadCmd` — §3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The function block FB_EcPhysicalReadCmd can be used to send an EtherCAT read command (FPRD, APRD, BRD) to a particular EtherCAT slave or to all EtherCAT slaves. This command can be used by the PLC to read a register or the DPRAM of the EtherCAT slave controller.

##### `FB_EcPhysicalWriteCmd` — §3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The function block FB_EcPhysicalWriteCmd can be used to send an EtherCAT write command (FPWR, APWR, BWR) to a particular EtherCAT slave or to all EtherCAT slaves. This command can be used by the PLC to write to a register or the DPRAM of the EtherCAT slave controller.

##### `FB_EcLogicalReadCmd` — §3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The master sends a logical EtherCAT write command (LRD) with the function block FB_EcLogicalReadCmd. In each slave, local address ranges (DPRAM) can be mapped to global logical address ranges. This command therefore addresses all EtherCAT slaves, which have mapping configured for the selected logical address range.

##### `FB_EcLogicalWriteCmd` — §3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `logAddr` | `UDINT` | Logical address |
| `len` | `UDINT` | Number of bytes to be written |
| `pSrcBuf` | `PVOID` | Address (pointer) to the transmit buffer |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this input. |
| `tTimeout` | `TIME` | Maximum time allowed for the execution of the function block. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The master sends a logical EtherCAT write command (LWR) with the function block FB_EcLogicalWriteCmd. In each slave, local address ranges (DPRAM) can be mapped to global logical address ranges. This command therefore addresses all EtherCAT slaves, which have mapping configured for the selected logical address range.

#### EtherCAT Diagnostic

##### `FB_EcGetAllMasters` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the IPC on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until |

**Description:** The function block FB_EcGetAllMasters can be used to read information from all EtherCAT masters. If the call is successful, the buffer transferred in the pAddrBuf parameter contains the information (Device ID, AMS Net ID, name) of all masters as an array of ST_EcDeviceInfo.

##### `FB_EcGetAllSlaveAbnormalStateChanges` — §4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId; (*AmsNetId of the EtherCAT master device*)` | — |
| `pAddrBuf` | `POINTER TO ARRAY [0..EC_MAX_SLAVES] OF UDINT` | — |
| `cbBufLen` | `UDINT; (*Size of the buffer pAddrBuf.      The size of the buffer must be at least nS` | — |
| `bExecute` | `BOOL; (*Function Block execution is triggered by a rising     edge at this input*)` | — |
| `tTimeout` | `TIME; (*States the time before the function is cancelled.*)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The function block FB_EcGetAllSlaveAbnormalStateChanges can be used to read the unexpected EtherCAT state changes of all the slaves connected to the master. If the call is successful, the buffer transferred in the parameter pBufAddr contains the number of unexpected state changes of all slaves as an array of UDINTs. EtherCAT state changes are unexpected if they were not requested by the EtherCAT master, e.g. if an EtherCAT slave spontaneously switches from OP state to SAFEOP state.

##### `FB_EcGetAllSlaveAddr` — §4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The FB_EcGetAllSlaveAddr function block allows the addresses of all the slaves connected to the master to be read. When the call is successful, the buffer passed in the parameter pAddrBuf contains the addresses of all the slaves as an array of UINTs.

##### `FB_EcGetAllSlaveCrcErrors` — §4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The FB_EcGetAllSlaveCrcErrors function block allows the CRC error counters of all the slaves connected to the master to be read. The CRC errors at the individual ports of a slave are added.

##### `FB_EcGetAllSlavePresentStateChanges` — §4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId; (*AmsNetId of the EtherCAT master device*)` | — |
| `pAddrBuf` | `POINTER TO ARRAY [0..EC_MAX_SLAVES] OF UDINT; (*Contains the address of the buffer` | — |
| `cbBufLen` | `UDINT; (*Size of the buffer pAddrBuf. The size of the buffer must be at least nSlav` | — |
| `bExecute` | `BOOL; (*Function Block execution is triggered by a rising edge at this input*)` | — |
| `tTimeout` | `TIME; (*States the time before the function is cancelled.*)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The function block FB_EcGetAllSlavePresentStateChanges can be used to read the EtherCAT state changes from state “slave is present” to “INIT_NO_COMM” of all slaves connected to the master. If the call is successful, the buffer transferred in the parameter pBufAddr contains the number of state changes of all slaves as an array of UDINTs. The EtherCAT state change from state “slave is present” to “INIT_NO_COMM” means that the connection to the slave has been interrupted. For example by disconnecting the EtherCAT cable.

##### `FB_EcGetAllSyncUnitSlaveAddr` — §4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | The network address of the TwinCAT computer on which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The FB_EcGetAllSyncUnitSlaveAddr function block allows the addresses of all the slaves connected to the master to be read. When the call is successful, the buffer passed in the parameter pAddrBuf contains the addresses of all the slaves as an array of UINTs.

##### `FB_EcGetConfSlaves` — §4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, |

**Description:** The function block FB_EcGetConfSlaves can be used to read a list of configured slaves from the EtherCAT master object directory.

##### `FB_EcGetLastProtErrInfo` — §4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The function block FB_EcGetLastProtErrInfo can be used to read additional error information relating to the most recent mailbox protocol error. An error-free mailbox command resets the last error every time.

##### `FB_EcGetMasterDevState` — §4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until |

**Description:** The function block FB_EcGetMasterDevState can be used to read the current state of the EtherCAT master.

##### `FB_EcGetScannedSlaves` — §4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ND_VAR` | `` | — |
| `bBusy` | `BOOL` | This output is set when the function block is activated, |

**Description:** The function block FB_EcGetScannedSlaves can be used to read a list of the currently available (scanned) slaves from the EtherCAT master object directory. To this end an online scan is executed, during which the EEPROMs of the EtherCAT slaves are read. The scanning process may take some time, depending on the number of connected slaves.

##### `FB_EcGetSlaveCount` — §4.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this input. |
| `tTimeout` | `TIME` | Maximum time allowed for the execution of the function block. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains |

**Description:** The function block FB_EcGetSlaveCount can be used to determine the number of slaves that are connected to the master.

##### `FB_EcGetSlaveCrcError` — §4.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Fixed address of the EtherCAT slave whose CRC error counter is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** The function block FB_EcGetSlaveCrcError allows the CRC error counters of the individual ports (A, B and C) of a slave to be read. If the call is successful, the output variable crcError, whose type is ST_EcCrcError, contains the requested CRC error counter.

##### `FB_EcGetSlaveCrcErrorEx` — §4.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains |

**Description:** The function block FB_EcGetSlaveCrcErrorEx allows the CRC error counters of the individual ports (A, D, B and C) of a slave to be read. If the call is successful, the output variable crcError, whose type is ST_EcCrcErrorEx, contains the requested CRC error counter.

##### `FB_EcGetSlaveIdentity` — §4.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The function block FB_EcGetSlaveIdentity can be used to read the CANopen identity of an individual EtherCAT slave device. If the call is successful, the output variable identity, whose type is ST_EcSlaveIdentity, contains the requested identity information.

##### `FB_EcGetSlaveTopologyInfo` — §4.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `the buffer the topology data are copied to.*) e * 64 Bytes*)` | `` | — |
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The function block FB_EcGetSlaveTopolgyInfo can be used to determine topology information.

##### `FB_EcMasterFrameCount` — §4.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, |

**Description:** The function block FB_EcMasterFrameCount can be used to determine the number of EtherCAT frames configured in the master.

##### `FB_EcMasterFrameStatistic` — §4.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, |

**Description:** The function block FB_EcMasterFrameStatistic can be used to read the frame statistics of the EtherCAT master. A distinction is made between cyclic and acyclic (queued) frames. Acyclic frames are used for the initialization or for parameter access to EtherCAT slaves. Frames are regarded as lost if they fail to return to the master or are invalid. The number of lost frames (i.e. lost or invalid cyclic frames), the number of cyclic frames per second, the number of lost queued frames (i.e. lost or invalid acyclic frames) and the number of queued frames per second is provided at the function block output.

##### `FB_EcMasterFrameStatisticClearCRC` — §4.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The function block FB_EcMasterFrameStatisticClearCRC can be used to delete the CRC error counters of all EtherCAT slaves.

##### `FB_EcMasterFrameStatisticClearFrames` — §4.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The function block FB_EcMasterFrameStatisticClearFrames can be used to delete the lost frame counters.

##### `FB_EcMasterFrameStatisticClearTxRxErr` — §4.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the CPU (PC). |
| `nEcMasterDevID INT` | `Device ID of the EtherCAT master.` | — |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this input. |
| `tTimeout` | `TIME 38` | Maximum time allowed for the execution of the function block. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The function block FB_EcMasterFrameStatisticClearTxRxErr can be used to delete the error counters of the miniport driver of the network card.

##### `FB_EcMasterObjectID` — §4.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, |

**Description:** The FB_EcMasterObjectID function block can be used to determine the object ID of the EtherCAT master device.

##### `F_CheckVendorId` (METHOD) → `BOOL` — §4.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stSlaveIdentity` | `ST_EcSlaveIdentity` | Slave Identity, which can be read with |

**Description:** The function F_CheckVendorId returns TRUE if the VendorID is Beckhoff, otherwise it returns FALSE.

##### `F_EcGetLinkedTaskOfSyncUnit` (METHOD) → `HRESULT` — §4.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `oidSyncUnit` | `OTCID` | The object ID of the Sync Unit is specified at this input. This can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sLinkedTask` | `STRING` | Returns the name of the linked task. |
| `oidLinkedTask` | `OTCID` | Returns the object ID of the linked task. |

**Description:** The name and object ID of the linked task of an EtherCAT Sync Unit can be read with this function. The return value of the function signals whether the call was successful and outputs the corresponding error code in case of an error.

##### `F_EcGetSyncUnitName` (METHOD) → `HRESULT` — §4.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `oidSyncUnit` | `OTCID` | The object ID of the Sync Unit is specified at this input. This can be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sSyncUnitName` | `STRING` | Returns the name of the sync unit. |

**Description:** The name of an EtherCAT Sync Unit can be read via its object ID using this function. The return value of the function signals whether the call was successful and outputs the relevant error code in the event of an error.

##### `F_EcGetMailboxGatewayAddr` (METHOD) → `HRESULT` — §4.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `oidEcMaster` | `OTCID` | The object ID of the EtherCAT master device is specified at this input. If |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aIpAddress` | `T_IPv4Addr Byte array of the (IPv4) Internet Protocol network address Arr` | — |
| `aMacAddress` | `ARRAY[0..5] Byte array of the (IPv4) MAC address OF BYTE` | — |

**Description:** This function can be used to determine the IP address and the MAC address of an EtherCAT master device via its object ID. The return value of the function signals whether the call was successful and outputs the relevant error code in the event of an error.

#### EtherCAT State Machine

##### `FB_EcGetAllSlaveStates` — §5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The FB_EcGetAllSlaveStates function block allows the EtherCAT status and the Link status of all the slaves connected to the master to be read. When the call is successful, the buffer passed in the parameter pStateBuf contains the requested status information as an array of ST_EcSlaveState.

##### `FB_EcGetAllExtSlaveStates` — §5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The FB_EcGetAllExtSlaveStates function block can be used to read the EtherCAT status, the extended EtherCAT status, and the link status of all slaves connected to the master. If the call is successful, the buffer passed in the pStateBuf parameter contains the requested status information as an array of type ST_EcExtendedSlaveState. The function block FB_EcGetAllExtSlaveStates is required when using the MX-System in order to detect empty MX slots (without an MX module inserted) in the PLC.

##### `FB_EcGetMasterState` — §5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this input. |
| `tTimeout` | `TIME` | Maximum time allowed for the execution of the function block. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until |

**Description:** The function block FB_EcGetMasterState can be used to read the EtherCAT state of the master. If the call is successful, the State output variable of type WORD contains the requested status information.

##### `FB_EcGetSlaveState` — §5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** The function block FB_EcGetSlaveState allows the EtherCAT status and the Link status of an individual EtherCAT slave to be read. If the call is successful, the output variable state, whose type is ST_EcSlaveState, contains the requested status information.

##### `FB_EcGetExtSlaveState` — §5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and |

**Description:** 48

##### `FB_EcReqMasterState` — §5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** With this function block the EtherCAT state of a master device can be requested and set. The requested EtherCAT state is transferred in the state variable. The function block becomes inactive as soon as it has requested the EtherCAT state. Unlike the function block FB_EcSetMasterState it does not wait until the new state is set.

##### `FB_EcReqSlaveState` — §5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Fixed address of the EtherCAT slave whose EtherCAT state is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** With this function block a slave can be set to a specified EtherCAT state. The requested EtherCAT state is transferred in the state variable. The function block becomes inactive as soon as it has sent the command to change state. Unlike the function block FB_EcSetSlaveState it does not wait until the EtherCAT slave has attained the new state.

##### `FB_EcSetMasterState` — §5.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this input. |
| `tTimeout` | `TIME` | Maximum time allowed for the execution of the function block. |
| `reqState` | `WORD (See reqState) reqState` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** 52

##### `FB_EcSetSlaveState` — §5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Fixed address of the EtherCAT slave whose EtherCAT state is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** With this function block a slave can be set to a specified EtherCAT state. The requested EtherCAT state is transferred with the reqState variable. The function block sends the command to change state and, unlike the function block FB_EcRegSlaveState, remains active until the EtherCAT slave has attained the new state or the maximum time tTimeout is exceeded. The current state is output in the currState variable.

#### ADS Interface

##### `FB_EcReadBIC` — §6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Fixed address of the EtherCAT slave to which the SDO upload command |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains |

**Description:** The function block FB_EcReadBIC can be used to read the BIC from the EEPROM of an EtherCAT slave via ADS. For this the slave must contain the BIC in the EEPROM.

##### `FB_EcReadBTN` — §6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Fixed address of the EtherCAT slave to which the SDO upload |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** The function block FB_EcReadBTN can be used to read the BTN from the EEPROM of an EtherCAT slave via ADS. For this the slave must contain the BIC in the EEPROM.

#### CoE interface

##### `FB_EcCoeSdoRead` — §7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until |

**Description:** The function block FB_EcCoeSdoRead allows data to be read from an object directory of an EtherCAT slave via SDO (Service Data Object) access. This requires the slave to have a mailbox and to support the “CANopen over EtherCAT” (CoE) protocol. The nSubIndex and nIndex parameters allow the object that is to be read to be selected. The function block FB_EcCoeSdoReadEx [} 60] must be used for access to the complete parameter, including subelements.

##### `FB_EcCoeSdoReadEx` — §7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `*) 60` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until |

**Description:** The function block FB_EcCoeSdoReadEx allows data to be read from an object directory of an EtherCAT slave via SDO (Service Data Object) access. This requires the slave to have a mailbox and to support the “CANopen over EtherCAT” (CoE) protocol. The nSubIndex and nIndex parameters allow the object that is to be read to be selected. Via bCompleteAccess := TRUE the parameter can be read with subelements.

##### `FB_EcCoeSdoWrite` — §7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_Ams` | String containing the AMS network ID of the EtherCAT master device. NetId |
| `nSlave` | `UINT` | Fixed address of the EtherCAT slave to which the SDO download command |
| `Addr` | `should be sent.` | — |
| `nSubIndex` | `BYTE` | Subindex of the object that is supposed to be written. |
| `nIndex` | `WORD` | Index of the object that is supposed to be written. |
| `pSrcBuf` | `PVOID` | Address (pointer) to the transmit buffer |
| `cbBufLen` | `UDINT` | Amount of data to be sent in bytes |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this input. |
| `tTimeout` | `TIME` | Maximum time allowed for the execution of the function block. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The FB_EcCoeSdoWrite function block permits an object from the object directory of an EtherCAT slave to be written by means of an SDO download. This requires the slave to have a mailbox and to support the “CANopen over EtherCAT” (CoE) protocol. The nSubIndex and nIndex parameters allow the object that is to be written to be selected. The function block FB_EcCoeSdoWriteEx [} 63] must be used for access to the complete parameter, including subelements.

##### `FB_EcCoeSdoWriteEx` — §7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `) *)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The FB_EcCoeSdoWriteEx function block permits an object from the object directory of an EtherCAT slave to be written by means of an SDO download. This requires the slave to have a mailbox and to support the “CANopen over EtherCAT” (CoE) protocol. The nSubIndex and nIndex parameters allow the object that is to be written to be selected. Via bCompleteAccess := TRUE the parameter can be written with sub- elements.

##### `FB_CoERead_ByDriveRef` — §7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `of EcDrive, either preset or read from NC*) P-0-0023*) 64 input*)` | `` | — |
| `stDriveRef` | `ST_DriveR Structure containing the AMS network ID of the EtherCAT master device` | — |
| `ef` | `and the address of the slave device. The reference to the drive can be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** The function block FB_CoERead_ByDriveRef can be used to read drive parameters by means of the "CANopen over EtherCAT (CoE)" protocol. This requires the slave to have a mailbox and to support the “CANopen over EtherCAT” (CoE) protocol. The nSubIndex and nIndex parameters allow the object that is to be read to be selected. Via bCompleteAccess := TRUE the parameter can be read with subelements.

##### `FB_CoEWrite_ByDriveRef` — §7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `EcDrive, either preset or read from NC*) P-0-0023*) input*)` | `` | — |
| `stDriveRef` | `ST_DriveRef Structure containing the AMS network ID of the EtherCAT master device` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** The function block FB_CoEWrite_ByDriveRef can be used to write drive parameters based on the “CANopen over EtherCAT (CoE)” protocol. This requires the slave to have a mailbox and to support the “CANopen over EtherCAT” (CoE) protocol. The nSubIndex and nIndex parameters allow the object that is to be written to be selected. Via bCompleteAccess := TRUE the parameter can be written with sub- elements.

##### `FB_EcCoeReadBIC` — §7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Fixed address of the EtherCAT slave to which the SDO upload command |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** The function block FB_EcCoeReadBIC can be used to read the BIC from the object directory of an EtherCAT slave via SDO (Service Data Object) access. For this the slave must have a mailbox and support the "CANopen over EtherCAT (CoE)" protocol and the object directory must contain an object 0x10E2:01 with the BIC.

##### `FB_EcCoeReadBTN` — §7.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Fixed address of the EtherCAT slave to which the SDO upload |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The function block FB_EcCoeReadBTN can be used to read the BTN from the object directory of an EtherCAT slave via SDO (Service Data Object) access. For this the slave must have a mailbox and support the "CANopen over EtherCAT (CoE)" protocol and the object directory must contain an object 0xF083 with the BTN.

##### `FB_EcCoESdoAbortCode` — §7.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId String containing the AMS network ID of the EtherCAT master device.` | — |
| `nSlave` | `UINT` | Fixed address of the EtherCAT slave to which the SDO download command |
| `Addr` | `should be sent.` | — |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this input. |
| `tTimeout` | `TIME` | Maximum time allowed for the execution of the function block. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until |

**Description:** The function block FB_EcCoESdoAbortCode reads the abort code from an EtherCAT sub device.

#### FoE interface

##### `FB_EcFoeAccess` — §8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hFoe` | `T_HFoe [} 139]` | "File access over EtherCAT" handle |
| `pBuffer` | `DWORD` | Contains the address of the buffer into which the data are to be read (read |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** This function block writes or reads data via the communication port of the “File access over EtherCAT” mailbox protocol.

##### `FB_EcFoeClose` — §8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hFoe` | `T_HFoe [} 139]` | "File access over EtherCAT" handle |
| `bExecute` | `BOOL` | The function block is activated by a positive edge at this input. |
| `tTimeout` | `TIME` | Maximum time allowed for the execution of the function block. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** This function block closes the communication port for the “File access over EtherCAT” mailbox protocol.

##### `FB_EcFoeLoad` — §8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Fixed address of the EtherCAT slave whose file is to be uploaded or |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** The function block FB_EcFoeLoad can be used to download or upload files to or from an EtherCAT device via the “File access over EtherCAT” mailbox protocol.

##### `FB_EcFoeOpen` — §8.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nPort` | `UINT` | Fixed address of the EtherCAT device |
| `sPathName` | `T_MaxString` | File path name ( e.g.: 'c:\TwinCAT\FOE\Data.fwp' ) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** This function block is used to open the communication port for the “File access over EtherCAT” mailbox protocol.

##### `FB_EcFoeReadFile` — §8.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sFSrvNetId` | `T_AmsNetId` | AMS network ID of the computer on which the file that was read is to be written. (Default: local computer) |
| `sFSrvPathNam T_MaxString` | `Contains the path and filename of the file to be written (e.g. 'C:` | — |
| `e` | `\Data\LogData.csv').` | — |
| `sEcNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Address of the EtherCAT slave |
| `sFoEPathNam` | `T_MaxString` | Name of the file on the EtherCAT slave (e.g. 'LogData') e |
| `dwPass` | `DWORD` | Password |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this input. |
| `tTimeout` | `TIME` | Maximum time allowed for the execution of the function block. (Default: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** The FB_EcFoeReadFile function block can be used to download files from an EtherCAT device to the local data carrier via the "File access over EtherCAT" mailbox protocol.

##### `FB_EcFoeWriteFile` — §8.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sFSrvNetId` | `T_AmsNetId` | AMS network ID of the computer from which the file to be written is to be read. (Default: local computer) |
| `sFSrvPathNam T_MaxString` | `Contains the path and filename of the file to be read (e.g.: 'C:` | — |
| `e` | `\Data\LogData.csv').` | — |
| `sEcNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Address of the EtherCAT slave |
| `sFoEPathNam` | `T_MaxString` | Name of the file on the EtherCAT slave (e.g. 'LogData') e |
| `dwPass` | `DWORD` | Password |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this input. |
| `tTimeout` | `TIME` | Maximum time allowed for the execution of the function block. (Default: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** The function block FB_EcFoeWriteFile can be used to write files from a local data carrier to an EtherCAT device via the "File access over EtherCAT" mailbox protocol.

#### SoE interface

##### `FB_EcSoeRead` — §9.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Fixed address of the EtherCAT slave to which the SoE read command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until |

**Description:** The function block FB_EcSoeRead can be used to read drive parameters by means of the “Sercos over EtherCAT (SoE)” protocol. To this end the slave must have a mailbox and support the SoE protocol. The drive parameter to be read is specified with the parameters nIdn (identification number), nElement and nDriveNo.

##### `FB_EcSoeWrite` — §9.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the AMS network ID of the EtherCAT master device. |
| `nSlaveAddr` | `UINT` | Fixed address of the EtherCAT slave to which the SoE write command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set until a |

**Description:** The function block FB_EcSoeWrite can be used to write drive parameters by means of the “Sercos over EtherCAT (SoE)” protocol. To this end the slave must have a mailbox and support the SoE protocol. The drive parameter to be written is specified with the parameters nIdn (identification number), nElement and nDriveNo.

##### `FB_SoERead_ByDriveRef` — §9.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rive, either preset or read from NC *)` | `` | — |
| `stDriveRef` | `ST_DriveRef` | The reference to the drive can be linked directly to the PLC in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** The FB_SoeRead_ByRef function block can be used to read drive parameters by means of the “Sercos over EtherCAT (SoE)” protocol. To this end the slave must have a mailbox and support the SoE protocol. The drive parameter to be read is specified with the parameters nIdn (identification number), nElement and stDriveRef.

##### `FB_SoEWrite_ByDriveRef` — §9.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `rive, either preset or read from NC *) 86` | `` | — |
| `stDriveRef` | `ST_DriveRef` | The reference to the drive can be linked directly to the PLC in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated, and remains set |

**Description:** The function block FB_SoeWrite_ByRef can be used to write drive parameters by means of the “Sercos over EtherCAT (SoE)” protocol. To this end the slave must have a mailbox and support the SoE protocol. The drive parameter to be written is specified with the parameters nIdn (identification number), nElement and stDriveRef.

#### (top level)

##### `F_ConvBK1120CouplerStateToString` — §10.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nState` | `WORD` | Coupler state; can be linked in the System Manager from the inputs of the BK1120/ |

**Description:** The function F_ConvBK1120CouplerStateToString returns the coupler state of the BK1120/BK1150/ BK1250 as string. For nState = 0 'No error' is returned, otherwise 'K-bus error' is returned, e.g. for nState = 1. If several errors are pending, they are separated by commas.

##### `F_ConvExtSlaveStateToBits` — §10.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEcExtSlaveState` | `ST_EcExtendedSlaveS Extended EtherCAT slave state structure (consisting of:` | — |
| `tate` | `deviceState : BYTE; linkState : BYTE; extendedState : WORD;)` | — |

**Description:** The function F_ConvExtSlaveStateToBits returns the extended EtherCAT slave state as the structure ST_EcExtendedSlaveStateBits [} 130].

##### `F_ConvExtSlaveStateToString` — §10.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `extState` | `ST_EcExtendedSlaveSta Extended EtherCAT slave state structure (consisting of:` | — |
| `te` | `deviceState : BYTE; linkState : BYTE; extendedState : WORD;)` | — |

**Description:** The function F_ConvExtSlaveStateToString returns the extended EtherCAT slave state as a string. To convert it to a string, see F_ConvExtStateToString [} 89].

##### `F_ConvExtStateToString` — §10.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nExtendedS DWORD` | `Extended EtherCAT slave state as a DWORD` | — |
| `tate` | `0x___1____ = 'EMPTY' 0x0000___1 = 'INIT' 0x0000___2 = 'PREOP' 0x0000___3 = 'BOOT' 0x0000___4 = 'SAFEOP' 0x0000___8 = 'OP' 0x0000001_ = 'Slave signals error'` | — |

**Description:** The F_ConvExtStateToString function returns the extended EtherCAT slave state as a string.

##### `F_ConvMasterDevStateToString` — §10.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nState` | `WORD` | Device state of the EtherCAT master; can be linked as DevState in the System |

**Description:** The function F_ConvMasterDevStateToString converts the device status of the EtherCAT master to string.

##### `F_ConvProductCodeToString` — §10.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stSlaveIdentity` | `ST_EcSlaveIdentity` | Slave Identity as it can be read with the |

**Description:** The function F_ConvProductToString returns the product code as string, e.g. 'EL6731-0000-0017'. From version 3.3.8.0 of the Tc2_EtherCAT library this function also supports ELM and EPP slaves such as 'EPP4374-0002-0018' and 'ELM3704-0001-0016'.

##### `F_ConvSlaveStateToString` — §10.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `state` | `ST_EcSlaveState` | EtherCAT slave state structure (consisting of: deviceState : BYTE; linkState : BYTE;) |

**Description:** The function F_ConvSlaveStateToString returns the EtherCAT slave state as string. For conversion to the string see F_ConvStateToString [} 92].

##### `F_ConvSlaveStateToBits` — §10.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEcSlaveState` | `ST_EcSlaveState` | EtherCAT slave state structure (consisting of: deviceState : BYTE; linkState : BYTE;) |

**Description:** The function F_ConvSlaveStateToBits returns the EtherCAT slave state as structure TYPE ST_EcSlaveStateBits [} 135].

##### `F_ConvSlaveStateToBitsEx` — §10.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stEcSlaveState` | `ST_EcSlaveState` | EtherCAT slave state structure (consisting of: deviceState : BYTE; linkState : BYTE;) |
| `nState` | `WORD` | EtherCAT slave state as WORD 0x___1 = 'INIT' 0x___2 = 'PREOP' 0x___3 = 'BOOT' 0x___4 = 'SAFEOP' 0x___8 = 'OP' 0x001_ = 'Slave signals error' |

**Description:** The function F_ConvSlaveStateToBitsEx returns the EtherCAT slave state as structureST_EcSlaveStateBitsEx [} 135].

##### `ConvertDcTimeToPos` — §11.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nAxisId` | `UDINT` | ID of the NC axis |
| `nSubIdx` | `UDINT` | This 32-bit input variable contains two different items of information, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPosition` | `LREAL` | Supplies the NC axis position corresponding to dcTime. This is an NC |

**Description:** This function block converts a 32-bit distributed clock system time variable of type T_DCTIME32 [} 138] to a corresponding NC axis position (i.e. the NC axis position at precisely this time).

##### `ConvertPosToDcTime` — §11.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nAxisId` | `UDINT` | ID of the NC axis |
| `nSubIdx` | `UDINT` | This 32-bit input magnitude is composed of two different items of |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `dcTime` | `T_DCTIME32` | Returns the 32-bit "Distributed Clock System Time" variable that |

**Description:** This function block converts an NC axis position to a corresponding 32-bit distributed clock system time variable of type T_DCTIME32 [} 138] (i.e. the time which precisely this NC axis position was or will be reached).

##### `ConvertDcTimeToPathPos` — §11.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGrpId` | `UDINT` | Group ID of the corresponding Nci channel |
| `nSubIdx` | `UDINT` | This 32-bit input variable contains two different items of information, and is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fPosition` | `LREAL` | Returns the relative Nci path distance on the contour that corresponds to |

**Description:** This function block converts a 32-bit distributed clock system time variable of type T_DCTIME32 [} 138] to a relative Nci path distance on the contour of the currently active Nci program (i.e. the function block returns a positive or negative relative interval, depending on the timing).

##### `ConvertPathPosToDcTime` (FUNCTION) → `T_DCTIME64` — §11.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nGrpId` | `UDINT` | Group ID of the corresponding Nci channel |
| `nSubIdx` | `UDINT` | This 32-bit input variable contains two different items of information, and is |
| `in` | `T_DCTIME` | The "Distributed Clock System Time" variable to be converted |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `dcTime` | `T_DCTIME32` | Returns the 32-bit "Distributed Clock System Time" variable that |

**Description:** This function block converts a relative Nci path distance to a corresponding 32-bit distributed clock system time variable of type T_DCTIME32 [} 138] (i.e. the time that corresponds or corresponded to the relative Nci path distance).

##### `DCTIME64_TO_DCTIME` (FUNCTION) → `T_DCTIME` — §11.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME64` | The "Distributed Clock System Time" variable to be converted |

**Description:** The function converts a distributed clock system time variable of type T_DCTIME [} 138] to a 64-bit distributed clock system time variable of type T_DCTIME64 [} 139].

##### `DCTIME64_TO_DCTIMESTRUCT` (FUNCTION) — §11.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME64` | The "Distributed Clock System Time" variable to be converted |
| `in` | `T_DCTIME64` | The "Distributed Clock System Time" variable to be converted |

**Description:** The function converts a 64-bit distributed clock system time variable of type T_DCTIME64 [} 138] to a structured variable of type DCTIMESTRUCT [} 137].

##### `DCTIME64_TO_STRING` (FUNCTION) → `STRING` — §11.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME64` | The "Distributed Clock System Time" variable to be converted |

**Description:** The function converts a 64-bit distributed clock system time variable of type T_DCTIME64 [} 138] to a string.

##### `DCTIME64_TO_SYSTEMTIME` (FUNCTION) → `T_DCTIME64` — §11.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME64` | The "Distributed Clock System Time" variable to be converted |
| `in` | `DCTIMESTRUCT` | The structured variable to be converted |
| `in` | `T_FILETIME64` | The "Windows File Time" variable to be converted |
| `in` | `STRING` | The string to be converted |
| `in` | `TIMESTRUCT` | The "Windows System Time" variable to be converted |
| `micro` | `WORD` | Microseconds |
| `nano` | `WORD` | Nanoseconds |
| `put` | `WORD` | Input parameter (currently not used) Inputs/outputs |
| `in` | `T_DCTIME64 [} 138]` | The "Distributed Clock System Time" variable to be converted |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME64` | — |

**Description:** The function converts a 64-bit distributed clock system time variable of type T_DCTIME64 [} 138] to a structured Windows system time variable of type TIMESTRUCT.

##### `DCTIME64 and ULINT` (FUNCTION) → `T_DCTIME64` — §11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ExtTime` | `T_DCTIME64 [} 138]` | External time in TwinCAT "Distributed Clock" system time format |
| `DcToExtTime` | `ULINT` | Time offset between the TwinCAT "Distributed Clock" system time |
| `Offset` | `and an external time` | — |
| `TcTime` | `T_DCTIME64 [} 138] TwinCAT system time in TwinCAT "Distributed Clock" system time format` | — |
| `DcToTcTimeOffs ULINT` | `Time offset between the TwinCAT "Distributed Clock" system time` | — |
| `et` | `and the TwinCAT system time 106 (*none*) (*none*) (*none*)` | — |
| `DcToExtTime` | `ULINT` | Time offset between the TwinCAT "Distributed Clock" system time |
| `Offset` | `and an external time` | — |
| `DcToTcTime` | `ULINT` | Time offset between the TwinCAT "Distributed Clock" system time |
| `Offset` | `and the TwinCAT system time` | — |
| `nSyncWindow` | `UDINT` | Time window within which the internal and external clock are |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DcToTcTimeOffset` | `ULINT` | — |
| `DcToExtTimeOffset` | `ULINT` | — |
| `ExtTime` | `T_DCTIME64` | — |
| `IntTime` | `T_DCTIME64` | — |
| `DcToTcTimeOffset` | `ULINT` | — |
| `DcToExtTimeOffset` | `ULINT` | — |
| `ExtTime` | `T_DCTIME64` | — |
| `IntTime` | `T_DCTIME64` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Offset = 0x7FFFFFFF*)` | `` | — |
| `bSynchronized` | `BOOL` | TRUE = external and internal clock are synchronous |
| `nTimeDiff` | `UDINT` | Current time difference between the two clocks |
| `nOffsetFrom` | `DINT` | Offset to sync master SyncMaster |

**Description:** The function F_ConvExtTimeToDcTime64 converts an external time to the TwinCAT distributed clock system time.

#### [obsolete] › [outdated DCTIME]

##### `DCTIME_TO_DCTIMESTRUCT` (FUNCTION) → `DCTIMESTRUCT` — §11.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME` | The "Distributed Clock System Time" variable to be converted. |

**Description:** The function converts a 64-bit distributed clock system time variable of type T_DCTIME [} 139] to a structured variable of type DCTIMESTRUCT [} 137].

##### `DCTIME_TO_FILETIME` (FUNCTION) → `T_FILETIME` — §11.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME` | The "Distributed Clock System Time" variable to be converted |

**Description:** The function converts a 64-bit distributed clock system time variable of type T_DCTIME [} 139] to a 64-bit Windows file time variable of type T_FILETIME.

##### `DCTIME_TO_STRING` (FUNCTION) → `STRING` — §11.4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME` | The "Distributed Clock System Time" variable to be converted |

**Description:** The function converts a string to a distributed clock system time variable of type T_DCTIME [} 139].

##### `DCTIME_TO_SYSTEMTIME` (FUNCTION) → `TIMESTRUCT` — §11.4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME` | The "Distributed Clock System Time" variable to be converted |

**Description:** The function converts a 64-bit distributed clock system time variable of type T_DCTIME [} 139] to a structured Windows system time variable of type TIMESTRUCT.

##### `DCTIMESTRUCT_TO_DCTIME` (FUNCTION) → `T_DCTIME` — §11.4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `DCTIMESTRUCT` | The structured variable to be converted |

**Description:** The function converts a structured variable of type DCTIMESTRUCT [} 137] to a 64-bit distributed clock system time variable of type T_DCTIME [} 139]. The structure components wDayofWeek is ignored in the conversion. The structure components wYear must be greater than or equal to 2000 and less than 2584. For invalid values of the structure components the function returns the value zero.

##### `FILETIME_TO_DCTIME` (FUNCTION) → `T_DCTIME` — §11.4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_FILETIME` | The "Windows File Time" variable to be converted |

**Description:** 114

##### `STRING_TO_DCTIME` (FUNCTION) → `T_DCTIME` — §11.4.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `STRING(29)` | — |

**Description:** The function converts a string to a distributed clock system time variable of type T_DCTIME [} 139].

##### `SYSTEMTIME_TO_DCTIME` (FUNCTION) → `T_DCTIME` — §11.4.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `TIMESTRUCT` | The "Windows System Time" variable to be converted |
| `Micro` | `WORD` | Microseconds |
| `nano` | `WORD` | Nanoseconds |

**Description:** The function converts a structured Windows system time variable of type TIMESTRUCT to a 64-bit distributed clock system time variable of type T_DCTIME [} 139]. In the event of a conversion error the function returns the value zero.

##### `FB_EcDcTimeCtrl` — §11.4.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `put` | `WORD` | Input parameter (currently not used) Inputs/outputs |
| `in` | `T_DCTIME` | TwinCAT "Distributed Clock System Time" variable |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | — |
| `get` | `WORD` | — |

**Description:** This function block can be used to read the individual components such as year, month, day etc. of a 64-bit "Distributed Clock System Time" variable of type T_DCTIME [} 139]. The function block has several A_GetXYZ actions. Once the required action has been called, the value of the XYZ component is available in the “get” output variable. The “put” input variable is currently not used.

#### [obsolete] › [outdated DCTIME and T_LARGE_INTEGER]

##### `F_ConvExtTimeToDcTime` (FUNCTION) → `T_DCTIME` — §11.4.2.1

**Return type:** `T_DCTIME`

**Description:** The function F_ConvExtTimeToDcTime converts an external time to the TwinCAT distributed clock system time.

##### `F_ConvTcTimeToDcTime` (FUNCTION) → `T_DCTIME` — §11.4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `TcTime` | `T_DCTIME` | TwinCAT system time in TwinCAT "Distributed Clock" system time format |
| `DcToTcTime` | `T_LARGE_` | Time offset between the TwinCAT "Distributed Clock" system time |
| `Offset` | `INTEGER` | and the TwinCAT system time |

**Description:** The function F_ConvTcTimeToDcTime64 converts the TwinCAT system time to the TwinCAT distributed clock system time.

##### `F_ConvTcTimeToExtTime` (FUNCTION) → `T_DCTIME` — §11.4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `TcTime` | `T_DCTIME` | TwinCAT system time in "Distributed Clock" format |
| `DcToTcTime` | `T_LARGE_` | Time offset between the TwinCAT "Distributed Clock" system time |
| `Offset` | `INTEGER` | and the TwinCAT system time |
| `DcToExtTime` | `T_LARGE_` | Time offset between the TwinCAT "Distributed Clock" system time |
| `Offset` | `INTEGER` | and an external time |

**Description:** The function F_ConcTcTimeToExtTime converts the TwinCAT distributed clock system time to an external time.

##### `F_GetActualDcTime` (FUNCTION) → `T_DCTIME` — §11.4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**Description:** This function returns the current time in TwinCAT distributed clock system time format (T_DCTIME [} 139]).

##### `F_GetCurDcTaskTime` (FUNCTION) → `T_DCTIME` — §11.4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**Description:** This function returns the task start time (time at which the task should start) in TwinCAT distributed clock system time format (T_DCTIME [} 139]). The function always returns the start time of the task in which it was called.

##### `F_GetCurDcTickTime` (FUNCTION) → `T_DCTIME` — §11.4.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**Description:** The function returns the time of the current (last) tick in TwinCAT distributed clock system time format (T_DCTIME [} 139]).

##### `F_GetCurExtTime` (FUNCTION) → `T_DCTIME` — §11.4.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DcToExtTime` | `T_LARGE_` | Time offset between the TwinCAT "Distributed Clock" system time |
| `Offset` | `INTEGER` | and an external time |
| `DcToTcTime` | `T_LARGE_` | Time offset between the TwinCAT "Distributed Clock" system time |
| `Offset` | `INTEGER` | and the TwinCAT system time |

**Description:** The function returns the external time in TwinCAT distributed clock system time format (T_DCTIME [} 139]).

##### `FB_EcExtSyncCalcTimeDiff` — §11.4.2.8

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DcToTcTimeOffset` | `T_LARGE_INTEGER` | — |
| `DcToExtTimeOffset` | `T_LARGE_INTEGER` | — |
| `ExtTime` | `T_DCTIME` | — |
| `IntTime` | `T_DCTIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `int Offset = 0x7FFFFFFF*)` | `` | — |
| `nTimeDiff` | `UDINT` | If the difference is less than 32 bits, the time difference is returned. |

**Description:** The function block FB_EcExtSyncCalcTimeDiff calculates the difference between external and internal time, taking into account the time offsets.

##### `FB_EcExtSyncCheck` — §11.4.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nSyncWindow` | `UDINT` | Time window within which the internal and external clock are |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DcToTcTimeOffset` | `T_LARGE_INTEGER` | — |
| `DcToExtTimeOffset` | `T_LARGE_INTEGER` | — |
| `ExtTime` | `T_DCTIME` | — |
| `IntTime` | `T_DCTIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bSynchronized` | `BOOL` | TRUE = external and internal clock are synchronous |
| `nTimeDiff` | `UDINT` | Current time difference between the two clocks |
| `nOffsetFrom` | `DINT` | Offset to sync master SyncMaster |

**Description:** The function block FB_EcExtSyncCheck checks whether the internal and external clocks are synchronous. See function block FB_EcExtSyncCalcTimeDiff [} 122].

#### [obsolete]

##### `DCTIME64_TO_FILETIME` (FUNCTION) → `T_FILETIME` — §11.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `in` | `T_DCTIME64` | The "Distributed Clock System Time" variable to be converted |
| `in` | `T_FILETIME` | The "Windows File Time" variable to be converted |

**Description:** The function converts a 64-bit distributed clock system time variable of type T_DCTIME64 [} 138] to a 64-bit Windows file time variable of type T_FILETIME.

#### (top level)

##### `F_GetVersionTcEtherCAT` (FUNCTION) → `UINT` — §12.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | Version element to be read. Possible parameters: |

**Description:** This function can be used to read PLC library version information.
