# Tc2_ModbusSrv

- **Manual:** `TF6250_TC3_Modbus_TCP_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 20
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Modbus TCP server: socket service + holding/register image mapping.

---

#### PLC API › Tc2_ModbusSrv library › Function blocks

##### `FB_MBReadCoils (Modbus function 1)` — §6.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING(15)` | — |
| `nTCPPort` | `UINT:= MODBUS_TCP_PORT` | — |
| `nUnitID` | `BYTE:=16#FF` | — |
| `nQuantity` | `WORD` | — |
| `nMBAddr` | `WORD` | — |
| `cbLength` | `UDINT` | — |
| `pDestAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBAddr WORD cbLength UDINT pDestAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBReadInputs (Modbus function 2)` — §6.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING(15)` | — |
| `nTCPPort` | `UINT:= MODBUS_TCP_PORT` | — |
| `nUnitID` | `BYTE:=16#FF` | — |
| `nQuantity` | `WORD` | — |
| `nMBAddr` | `WORD` | — |
| `cbLength` | `UDINT` | — |
| `pDestAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBAddr WORD cbLength UDINT pDestAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBReadRegs (Modbus function 3)` — §6.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING(15)` | — |
| `nTCPPort` | `UINT:= MODBUS_TCP_PORT` | — |
| `nUnitID` | `BYTE:=16#FF` | — |
| `nQuantity` | `WORD` | — |
| `nMBAddr` | `WORD` | — |
| `cbLength` | `UDINT` | — |
| `pDestAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBAddr WORD cbLength UDINT pDestAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBReadInputRegs (Modbus function 4)` — §6.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING(15)` | — |
| `nTCPPort` | `UINT:= MODBUS_TCP_PORT` | — |
| `nUnitID` | `BYTE:=16#FF` | — |
| `nQuantity` | `WORD` | — |
| `nMBAddr` | `WORD` | — |
| `cbLength` | `UDINT` | — |
| `pDestAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBAddr WORD cbLength UDINT pDestAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBWriteSingleCoil (Modbus function 5)` — §6.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING(15)` | — |
| `nTCPPort` | `UINT:= MODBUS_TCP_PORT` | — |
| `nUnitID` | `BYTE:=16#FF` | — |
| `nMBAddr` | `WORD` | — |
| `nValue` | `WORD` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBAddr WORD nValue WORD bExecute BOOL tTimeout TIME

##### `FB_MBWriteSingleReg (Modbus function 6)` — §6.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING (15)` | Is a string containing the IP address of the target device. |
| `nTCPPort` | `UINT` | Port number of the target device. |
| `nUnitID` | `BYTE` | Identification number of a serial sub-network device. If a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBAddr WORD nValue WORD bExecute BOOL tTimeout TIME

##### `FB_MBWriteCoils (Modbus function 15)` — §6.1.2.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nQuantity WORD nMBAddr WORD cbLength UDINT pSrcAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBWriteRegs (Modbus function 16)` — §6.1.2.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nQuantity WORD nMBAddr WORD cbLength UDINT pSrcAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBReadWriteRegs (Modbus-Funktion 23)` — §6.1.2.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBReadAddr WORD nWriteQuantity WORD nMBWriteAddr WORD cbDestLength UDINT pDestAddr Pointer To BYTE cbSrcLength UDINT pSrcAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBDiagnose (Modbus function 8)` — §6.1.2.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nWriteData WORD bExecute BOOL tTimeout TIME

#### PLC API › Tc2_ModbusSrv library › Function blocks › UDP

##### `FB_MBUdpReadCoils (Modbus function 1)` — §6.1.2.11.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING (15)` | Is a string containing the IP address of the target device. |
| `nTCPPort` | `UINT` | Port number of the target device. |
| `nUnitID` | `BYTE` | Identification number of a serial sub-network device. If a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBAddr WORD cbLength UDINT pDestAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBUdpReadInputs (Modbus function 2)` — §6.1.2.11.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING(15)` | — |
| `nTCPPort` | `UINT:= MODBUS_TCP_PORT` | — |
| `nUnitID` | `BYTE:=16#FF` | — |
| `nQuantity` | `WORD` | — |
| `nMBAddr` | `WORD` | — |
| `cbLength` | `UDINT` | — |
| `pDestAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBAddr WORD cbLength UDINT pDestAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBUdpReadRegs (Modbus function 3)` — §6.1.2.11.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING (15)` | Is a string containing the IP address of the target device. |
| `nTCPPort` | `UINT` | Port number of the target device. |
| `nUnitID` | `BYTE` | Identification number of a serial sub-network device. If a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBAddr WORD cbLength UDINT pDestAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBUdpReadInputRegs (Modbus function 4)` — §6.1.2.11.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING (15)` | Is a string containing the IP address of the target device. |
| `nTCPPort` | `UINT` | Port number of the target device. |
| `nUnitID` | `BYTE` | Identification number of a serial sub-network device. If a |

**Description:** nMBAddr WORD cbLength UDINT pDestAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBUdpWriteSingleCoil (Modbus function 5)` — §6.1.2.11.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING (15)` | Is a string containing the IP address of the target device. |
| `nTCPPort` | `UINT` | Port number of the target device. |
| `nUnitID` | `BYTE` | Identification number of a serial sub-network device. If a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBUSY` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrId` | `UDINT` | — |

**Description:** nMBAddr WORD nValue WORD bExecute BOOL tTimeout TIME

##### `FB_MBUdpWriteSingleReg (Modbus function 6)` — §6.1.2.11.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING (15)` | Is a string containing the IP address of the target device. |
| `nTCPPort` | `UINT` | Port number of the target device. |
| `nUnitID` | `BYTE` | Identification number of a serial sub-network device. If a |

**Description:** nMBAddr WORD nValue WORD bExecute BOOL tTimeout TIME

##### `FB_MBUdpWriteCoils (Modbus function 15)` — §6.1.2.11.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING(15)` | — |
| `nTCPPort` | `UINT:= MODBUS_TCP_PORT` | — |
| `nUnitID` | `BYTE:=16#FF` | — |
| `nQuantity` | `WORD` | — |
| `nMBAddr` | `WORD` | — |
| `cbLength` | `UDINT` | — |
| `pSrcAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nQuantity WORD nMBAddr WORD cbLength UDINT pSrcAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBUdpWriteRegs (Modbus function 16)` — §6.1.2.11.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING (15)` | Is a string containing the IP address of the target device. |
| `nTCPPort` | `UINT` | Port number of the target device. |
| `nUnitID` | `BYTE` | Identification number of a serial sub-network device. If a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nQuantity WORD nMBAddr WORD cbLength UDINT pSrcAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBUdpReadWriteRegs (Modbus function 23)` — §6.1.2.11.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nMBReadAddr WORD nWriteQuantity WORD nMBWriteAddr WORD cbDestLength UDINT pDestAddr Pointer To BYTE cbSrcLength UDINT pSrcAddr Pointer To BYTE bExecute BOOL tTimeout TIME

##### `FB_MBUdpDiagnose (Modbus function 8)` — §6.1.2.11.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIPAddr` | `STRING (15)` | Is a string containing the IP address of the target device. |
| `nTCPPort` | `UINT` | Port number of the target device. |
| `nUnitID` | `BYTE` | Identification number of a serial sub-network device. If a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | When the function block is enabled, this output is set and |

**Description:** nWriteData WORD bExecute BOOL tTimeout TIME
