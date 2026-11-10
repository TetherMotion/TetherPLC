# Tc2_Coupler

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_Coupler_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T2 · **API entries:** 6
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Mailbox coupling between images needs an ADS-like transport.

---

#### Function blocks

##### `ReadWriteTerminalReg` — §3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STATE` | `BYTE` | Status byte of the terminal channel. |
| `DATAIN` | `WORD` | Data input word of the terminal channel. |
| `REGNO` | `BYTE` | Number of the register to read or write. |
| `READ` | `BOOL` | A positive edge at this input activates the function block, |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `CTRL` | `BYTE` | Control byte of the terminal channel. |
| `DATAOUT` | `WORD` | Data output word of the terminal channel. |
| `BUSY` | `BOOL` | When the function block is activated, this output is set and |

**Description:** The function block ReadWriteTerminalReg enables convenient access to the terminal's registers via the control/status byte of the terminal channel (register communication). In the standard operation mode, the data inputs and outputs of the intelligent terminals (e.g. an analog output terminal) are used to exchange the analog output data. A handshake via the control/status byte permits register access. The data input and output variables are used here to transfer the register values. A positive edge at the READ or WRITE input causes the register with number REGNO to be read or written to. Write protection of the register is disabled by the function block for a write access and enabled once more afterwards. With a write access to a register, the new register value is written first and then read. The value read is available at the output CURRREGVALUE. If changes made to the register values are to be stored permanently, the power supply to the coupler must be interrupted. The variables STATE, DATAIN, CTRL and DATAOUT must be linked in the TwinCAT System Manager to the corresponding I/O variables in the terminal channel.

##### `CouplerReset` — §3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STATE` | `PLCINTFSTRUCT [} 18]` | Status word of the 2-byte PLC interface. |
| `START` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `CONTROL` | `PLCINTFSTRUCT [} 18]` | Control word of the 2-byte PLC interface. |
| `BUSY` | `BOOL` | When the function block is activated, this output is set and |

**Description:** The function block CouplerReset can be used to execute a reset of the coupler via the 2-byte PLC interface. During a coupler reset, for example, the current terminal configuration is read in again by the coupler via the K-bus (terminal bus) and communication on the K-bus is reinitialized. Existing K-bus error messages for the coupler are reset. The STATE and CONTROL variables are used to perform a handshake with the coupler during the execution of the function block. These variables must therefore be linked to the Control/Status variables of the 2-byte PLC interface in the TwinCAT System Manager.

##### `FB_ReadCouplerDiag` — §3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stState` | `PLCINTFSTRUCT [} 18]` | Status word of the 2-byte PLC interface. |
| `bExecute` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCtrl` | `PLCINTFSTRUCT [} 18]` | Control word of the 2-byte PLC interface. |
| `bBusy` | `BOOL` | When the function block is activated, this output is set and |

**Description:** The function block FB_ReadCouplerDiag allows reading of the first and second flashing sequences of the error LED on the coupler when a terminal bus or coupler error occurs. The data is transferred to the PLC via the 2-byte PLC interface. This only works if communication is maintained via the fieldbus. It must be possible to transfer data from the coupler to the PLC via the fieldbus without errors. To detect a coupler error that has occurred, the status byte of the coupler can be queried cyclically in the PLC and the function block is activated if an error occurs.

##### `FB_ReadCouplerRegs` — §3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stState` | `PLCINTFSTRUCT [} 18]` | Status word of the 2-byte PLC interface. |
| `nTerminal` | `BYTE` | Terminal number whose table register is to be accessed. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `14 Function blocks` | `` | — |
| `stCtrl` | `PLCINTFSTRUCT [} 18]` | Control word of the 2-byte PLC interface. |
| `bBusy` | `BOOL` | When the function block is activated, this output is set and |

**Description:** This function block FB_ReadCouplerRegs provides read access to the table register in the coupler and the registers of the intelligent terminals. The coupler itself is referred to as terminal 0 (null). All the other terminals in the terminal block, with the exception of passive terminals (such as power supply terminals), are counted in ascending sequence (beginning with 1). All registers or only a subrange (between nStartReg and nEndReg) can be read. Several seconds are required to read all registers (0.255) of a coupler table. Register values that have been successfully read are found in the structure stCouplerTable. The structure is an array of high and low bytes. Each array element corresponds to a register value (e.g.: stCouplerTable[ 5 ] == Register 5).

##### `FB_WriteCouplerRegs` — §3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `stState` | `PLCINTFSTRUCT [} 18]` | Status word of the 2-byte PLC interface. |
| `nTerminal` | `BYTE` | Terminal number whose table register is to be accessed. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCtrl` | `PLCINTFSTRUCT [} 18]` | Control word of the 2-byte PLC interface. |
| `bBusy` | `BOOL` | When the function block is activated, this output is set and |

**Description:** The function block FB_WriteCouplerRegs can be used for write access to the coupler's table registers and the registers of the intelligent terminals. The coupler itself is referred to as terminal 0 (null). All the other terminals in the terminal block, with the exception of passive terminals (such as power supply terminals), are counted in ascending sequence (beginning with 1). All registers or only a subrange (between nStartReg and nEndReg) can be written to. Several seconds are required to write all registers (0.255) of a coupler table. The register values to be written are located in the structure stCouplerTable. The structure is an array of high and low bytes. Each array element corresponds to a register value (e.g.: stCouplerTable[ 5 ] == Register 5).

#### [obsolete functions]

##### `F_GetVersionTcPlcCoupler` (FUNCTION) → `UINT` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | — |

**Description:** This function is obsolete and should not be used any longer. Use the global constant stLibVersion_Tc2_Coupler [} 20] to read version information from the plc library.
