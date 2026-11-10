# Tc2_ModbusRTU

- **Manual:** `TF6255_TC3_Modbus_RTU_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 11
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Modbus RTU: serial-port service + RTU framing (CRC16, T3.5 gap timing).

---

#### PLC API › Function blocks › [obsolete]

##### `ModbusRtuMaster_PcCOM` — §5.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `UnitID` | `UINT` | Modbus station address [} 41] (1..247). The Modbus |

**Description:** The function block ModbusRtuMaster_PcCOM implements a Modbus master that communicates via a serial PC interface (COM port). The function block ModbusRtuMaster_KL6x5B [} 18] is available for communication via a serial Bus Terminal KL6001, KL6011 or KL6021.

##### `ModbusRtuMaster_KL6x5B` — §5.1.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that the function block is active. Busy becomes |

**Description:** The function block ModbusRtuMaster_KL6x5B realizes a Modbus master, which communicates via a serial Bus Terminal KL6001, KL6011 or KL6021.

##### `ModbusRtuMaster_KL6x22B` — §5.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `UnitID` | `UINT` | Modbus station address [} 41] (1..247). The Modbus |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BUSY` | `BOOL` | — |
| `Error` | `BOOL` | — |
| `ErrorId` | `MODBUS_ERRORS` | — |
| `cbRead` | `UINT` | — |

**Description:** The function block ModbusRtuMaster_KL6x22B realizes a Modbus master, which communicates via a serial Bus Terminal KL6031 or KL6041. The function block ModbusRtuMaster_PcCOM [} 16] is available for communication via a serial PC interface (COM port).

#### PLC API › Function blocks

##### `ModbusRtuMasterV2_PcCOM` — §5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `UnitID` | `BYTE` | — |
| `Quantity` | `WORD` | — |
| `MBAddr` | `WORD` | — |
| `cbLength` | `UINT` | — |
| `pMemoryAddr` | `POINTER TO BYTE` | — |
| `AuxQuantity` | `WORD` | — |
| `AuxMBAddr` | `WORD` | — |
| `AuxcbLength` | `UINT` | — |
| `pAuxMemoryAddr` | `POINTER TO BYTE` | — |
| `Execute` | `BOOL` | — |
| `Timeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that the function block is active. Busy becomes |

**Description:** The function block ModbusRtuMasterV2_PcCOM implements a Modbus master that communicates via a serial PC interface (COM port). Additional function blocks are available for communication via a serial bus terminal.

##### `ModbusRtuMasterV2_KL6x22B` — §5.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that the function block is active. Busy becomes |

**Description:** The function block ModbusRtuMasterV2_KL6x22B realizes a Modbus master, which communicates via a serial Bus Terminal KL6031 or KL6041. Serial EtherCAT Terminals with 22 bytes of data process image are also supported. The function block ModbusRtuMasterV2_PcCOM [} 23] is available for communication via a serial PC interface (COM port).

##### `ModbusRtuMasterV2_KL6x5B` — §5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `UnitID` | `BYTE` | — |
| `Quantity` | `WORD` | — |
| `MBAddr` | `WORD` | — |
| `cbLength` | `UINT` | — |
| `pMemoryAddr` | `POINTER TO BYTE` | — |
| `AuxQuantity` | `WORD` | — |
| `AuxMBAddr` | `WORD` | — |
| `AuxcbLength` | `UINT` | — |
| `pAuxMemoryAddr` | `POINTER TO BYTE` | — |
| `Execute` | `BOOL` | — |
| `Timeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that the function block is active. Busy becomes |

**Description:** The function block ModbusRtuMasterV2_KL6x5B realizes a Modbus master, which communicates via a serial Bus Terminal KL6001, KL6011 or KL6021. The function block ModbusRtuMasterV2_PcCOM [} 23] is available for communication via a serial PC interface (COM port).

##### `ModbusRtuMasterV2_Generic` — §5.1.5

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RxBuffer` | `ComBuffer` | — |
| `TxBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | Indicates that the function block is active. Busy becomes |

**Description:** The function block ModbusRtuMasterV2_Generic implements a Modbus master that communicates via various serial interfaces (COM port, virtual COM port, EtherCAT Terminals, ...).

##### `ModbusRtuSlave_PcCOM` — §5.1.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ErrorId` | `MODBUS_ERRORS` | Indicates an error number [} 41] in the event of disturbed |

**Description:** The function block ModbusRtuSlave_PcCOM implements a Modbus slave that communicates via a serial PC interface (COM port). The function block ModbusRtuSlave_KL6x5B [} 38] is available for communication via a serial Bus Terminal KL6001, KL6011 or KL6021.

##### `ModbusRtuSlave_KL6x22B` — §5.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `UnitID` | `UINT` | Modbus station address [} 41] (1..247). The Modbus |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ErrorId` | `MODBUS_ERRORS` | Indicates an error number [} 41] in the event of disturbed |

**Description:** The function block ModbusRtuSlave_KL6x22B realizes a Modbus slave, which communicates via a serial Bus Terminal KL6031 or KL6041. Serial EtherCAT Terminals with 22 bytes of data process image are also supported. The function block ModbusRtuSlave_PcCOM [} 35] is available for communication via a serial PC interface (COM port).

##### `ModbusRtuSlave_KL6x5B` — §5.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `UnitID` | `UINT` | Modbus station address [} 41] (1..247). The Modbus |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ErrorId` | `MODBUS_ERRORS` | Indicates an error number [} 41] in the event of disturbed |

**Description:** The function block ModbusRTUslave_KL6x5B realizes a Modbus slave, which communicates via a serial Bus Terminal KL6001, KL6011 or KL6021. The function block ModbusRtuSlave_PcCOM [} 35] is available for communication via a serial PC interface (COM port).

##### `ModbusRtuSlave_Generic` — §5.1.9

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RxBuffer` | `ComBuffer` | — |
| `TxBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ErrorId` | `MODBUS_ERRORS` | Indicates an error number [} 41] in the event of disturbed |

**Description:** The function block ModbusRtuSlave_Generic implements a Modbus slave that communicates via various serial interfaces (COM port, virtual COM port, EtherCAT Terminals, ...).
