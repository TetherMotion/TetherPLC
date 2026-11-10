# Tc2_SMI

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_SMI_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 25
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** SMI (building management interface): fieldbus transport.

---

#### Programming › POUs › Base

##### `FB_KL6831KL6841Communication` — §4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `12 Programming` | `` | — |
| `bResetMaximumDe BOOL` | `A positive edge resets the stored value of the maximum` | — |
| `mandCounter` | `command buffer demand` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInData` | `ST_KL6831KL6841InData` | — |
| `stOutData` | `ST_KL6831KL6841OutData` | — |
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function blocks for the SMI commands do not directly access the process image of the KL6831/KL6841; instead, they place the individual SMI commands into three different buffers. The function block FB_KL6831KL6841Communication() sequentially reads the SMI commands from these three buffers and forwards the SMI commands to the KL6831/KL6841. This prevents multiple function blocks accessing the KL6831/KL6841 process image simultaneously. Each of these three buffers is processed with a different priority (high, medium or low). The parameter eCommandPriority, which is available for most function blocks, can be used to influence the priority with which the respective SMI command is processed by the function block FB_KL6831KL6841Communication().

##### `FB_KL6831KL6841Config` — §4.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConfigurate` | `BOOL` | Configuration of the bus terminal is started by a positive |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stInDataTerminal` | `ST_KL6831KL6841InData` | — |
| `stOutDataTerminal` | `ST_KL6831KL6841OutData` | — |
| `stInData` | `ST_KL6831KL6841InData` | — |
| `stOutData` | `ST_KL6831KL6841OutData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `16 Programming` | `` | — |
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_KL6831KL6841Config is used to configure the KL6831/KL6841. The configuration is executed when the PLC program starts, or it can be triggered by a positive edge at the input bConfigurate. The parameters are stored in the respective registers of the KL6831/KL6841 in a fail-safe manner. In addition, some general information, such as the firmware version, is read from the KL6831/KL6841.

##### `FB_SMISendSMICommand` — §4.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `18 Programming` | `` | — |
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMISendSMICommand is used for the general sending of an SMI command. The precise structure of an SMI command and the functioning of the KL6831/KL6841 must be known for this. The use of this function block is necessary only if an SMI command is to be sent that is not covered by the other PLC function blocks.

#### Programming › POUs › Basic commands

##### `FB_SMIDiagAll` — §4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIDiagAll can be used to determine in which direction the drives are moving, whether they are stopped or whether there is a motor error. The command can also be sent also to several SMI slaves. The states of all SMI slaves can thus be queried with a single command.

##### `FB_SMIDown` — §4.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIDown controls the motor run to the lower end position.

##### `FB_SMIDownStep` — §4.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIDownStep controls the motor run downwards by a specified angle (0-510 degrees).

##### `FB_SMIPos1Read` — §4.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `26 Programming` | `` | — |
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIPos1Read reads the fixed position Pos1 configured on the motor side. Pos1 can be changed with the function block FB_SMIPos1Write() [} 27].

##### `FB_SMIPos1Write` — §4.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes Programming |
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIPos1Write writes the fixed position Pos1 that can be configured on the motor side. Pos1 can be read with the function block FB_SMIPos1Read() [} 26].

##### `FB_SMIPos2Read` — §4.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIPos2Read reads the fixed position Pos2 configured on the motor side. Pos2 can be changed with the function block FB_SMIPos2Write() [} 31].

##### `FB_SMIPos2Write` — §4.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** Programming

##### `FB_SMIPosRead` — §4.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIPosRead reads the current position from the drive.

##### `FB_SMIPosWrite` — §4.1.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `34 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIPosWrite moves the drive to the specified position.

##### `FB_SMIStop` — §4.1.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `36 Programming` | `` | — |
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIStop stops the motor run.

##### `FB_SMISyn` — §4.1.2.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMISyn queries the manufacturer code and the drive type.

##### `FB_SMIUp` — §4.1.2.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIUp controls the motor run up to the upper end position.

##### `FB_SMIUpStep` — §4.1.2.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIUpStep controls the motor run upwards by a specified angle (0-510 degrees).

#### Programming › POUs › Addressing commands

##### `FB_SMIAddressing` — §4.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** This function block addresses the connected SMI devices according to the random principle. Which SMI device is assigned to which address is beyond the control of the user. The addresses are assigned in descending order, starting with the address specified by the parameter byHighestAddress.

##### `FB_SMIDiscoverySlaveId` — §4.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the search is started by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The first drive is sought that corresponds to the specified manufacturer code and has the address 0. This function block is used for addressing SMI devices and is used in the function block FB_SMIAddressing() [} 41].

##### `FB_SMISlaveAddrRead` — §4.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `44 Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMISlaveAddrRead reads the address (0-15) of a drive.

##### `FB_SMISlaveAddrWrite` — §4.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMISlaveAddrWrite writes the address (0-15) of one or more drives.

##### `FB_SMISlaveIdCompare` — §4.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMISlaveIdCompare compares a specified slave ID (32-bit key ID) with the slave ID (32-bit key ID) defined on the motor side for one or more drives. The command can also be sent also to several SMI slaves.

##### `FB_SMISlaveIdRead` — §4.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMISlaveIdRead reads the slave ID (32-bit key ID) from a drive.

#### Programming › POUs › System commands

##### `FB_SMIParValueReadByte` — §4.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `52 Programming` | `` | — |
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIParValueReadByte reads a byte parameter (1 byte) stored on the motor side. The meaning of the individual parameters is manufacturer-specific.

##### `FB_SMIParValueReadWord` — §4.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIParValueReadWord reads a word parameter (2 bytes) stored on the motor side. The meaning of the individual parameters is manufacturer-specific.

##### `FB_SMIParValueReadDWord` — §4.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStart` | `BOOL` | The function block is activated and the command is sent by |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stCommandBuffer` | `ST_SMICommandBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | This output is set as soon as the function block processes |

**Description:** The function block FB_SMIParValueReadDWord reads a DWord parameter (4 bytes) stored on the motor side. The meaning of the individual parameters is manufacturer-specific.
