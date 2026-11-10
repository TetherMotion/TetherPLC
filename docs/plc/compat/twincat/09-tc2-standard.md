# Tc2_Standard

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_Standard_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T0 · **API entries:** 31
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** IEC standard functions (string, time, math, memory): pure-ST compatibility library, no runtime services.

---

#### Function blocks › Bistable

##### `RS` — §3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SET` | `BOOL` | Set input - Q1 is set to TRUE on a rising edge. |
| `RESET1` | `BOOL` | Reset input (dominant) - Q1 is set to FALSE on rising edge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q1` | `BOOL` | Output |

**Description:** The RS function block is bistable and RESET is dominant.

##### `SR` — §3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SET1` | `BOOL` | Set input (dominant) - Q1 is set to TRUE on a rising edge |
| `RESET` | `BOOL` | Reset input - Q1 is set to FALSE on a rising edge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q1` | `BOOL` | Output |

**Description:** The SR function block is bistable and SET is dominant.

#### Function blocks › Counter

##### `CTD` — §3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CD` | `BOOL` | On a rising edge: count down by one |
| `LOAD` | `BOOL` | TRUE: Set counter variable CV to the start value PV |
| `PV` | `Word` | Start value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `10 Function blocks` | `` | — |
| `Q` | `BOOL` | TRUE if the counter value CV is 0. |
| `CV` | `WORD` | Counter variable (current counter value) |

**Description:** The CTD function block is a down counter.

##### `CTU` — §3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CU` | `BOOL` | On a rising edge: count up by one |
| `RESET` | `BOOL` | TRUE: Reset counter variable CV to the value 0 |
| `PV` | `WORD` | Counter limit |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | TRUE if the counter value CV is greater than or equal to |

**Description:** The CTU function block is an up counter.

##### `CTUD` — §3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CU` | `BOOL` | On a rising edge: count up by one |
| `CD` | `BOOL` | On a rising edge: count down by one |
| `RESET` | `BOOL` | TRUE: Reset counter variable CV to the value 0 |
| `LOAD` | `BOOL` | TRUE: Set counter variable CV to the start value PV |
| `PV` | `WORD` | Start value/counter limit |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `QU` | `BOOL` | TRUE if the counter value CV is greater than or equal to |

**Description:** Function blocks

#### Function blocks › Timer

##### `TOF` — §3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IN` | `BOOL` | Falling edge: Start timer Rising edge: Reset timer |
| `PT` | `TIME` | Delay time (time that elapses before Q is reset) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | TRUE if the IN input is TRUE. |

**Description:** The TOF function block implements a timer with a switch-off delay.

##### `TON` — §3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IN` | `BOOL` | Rising edge: Start timer Falling edge: Reset timer |
| `PT` | `TIME` | Delay time (time elapsed before Q is set) |
| `PT` | `TIME` | Delay time (time elapsed before Q is set) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | FALSE if the IN input is FALSE. |

**Description:** The TON function block implements a timer with a switch-on delay.

##### `TP` — §3.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IN` | `BOOL` | Rising edge: Start pulse timer and set Q to TRUE |
| `PT` | `TIME` | Pulse duration (length of the high signal) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | TRUE after a rising edge at the IN input for the pulse |

**Description:** The TP function block, a pulse generator, can be used to generate pulses with a defined pulse duration.

#### Function blocks › Timer (LTIME)

##### `LTOF` — §3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IN` | `BOOL` | Falling edge: Start timer Rising edge: Reset timer |
| `PT` | `TIME` | Delay time (time that elapses before Q is reset) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | TRUE if the IN input is TRUE. |

**Description:** The LTOF function block implements a timer with switch-off delay with 64-bit time data type (LTIME).

##### `LTON` — §3.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IN` | `BOOL` | Rising edge: Start timer Falling edge: Reset timer |
| `PT` | `TIME` | Delay time (time elapsed before Q is set) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | FALSE if the IN input is FALSE. |

**Description:** The LTON function block implements a timer with a switch-on delay with a 64-bit time data type (LTIME).

##### `LTP` — §3.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `IN` | `BOOL` | Rising edge: Start pulse timer and set Q to TRUE |
| `PT` | `TIME` | Pulse duration (length of the high signal) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | TRUE after a rising edge at the IN input for the pulse |

**Description:** The LTP function block is a pulse generator with 64-bit time data type (LTIME). This function block can be used to generate pulses with a defined pulse duration.

#### Function blocks › Trigger

##### `F_TRIG` — §3.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CLK` | `BOOL` | Boolean signal to be checked |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | TRUE for a falling edge at the CLK input (for one cycle) |

**Description:** The F_TRIG function block is a detector for a falling edge.

##### `R_TRIG` — §3.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `CLK` | `BOOL` | Boolean signal to be checked |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | TRUE for a rising edge at the CLK input (for one cycle) |

**Description:** The R_TRIG function block is a rising edge detector.

#### String functions

##### `CONCAT` (FUNCTION) → `STRING` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR1` | `STRING(255)` | — |
| `STR2` | `STRING(255)` | — |

**Description:** Concatenation (combination) of two strings.

##### `DELETE` (FUNCTION) → `STRING` — §4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR` | `STRING(255)` | — |
| `LEN` | `INT` | — |
| `POS` | `INT` | — |

**Description:** The function DELETE removes a partial string from a larger string at a defined position. The input variable STR is type STRING, LEN and POS are type INT, the return value of the function is type STRING. DELETE (STR, LEN, POS) means: Delete LEN characters from STR beginning with the character in the POS.

##### `FIND` (FUNCTION) → `INT` — §4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR1` | `STRING(255)` | — |
| `STR2` | `STRING(255)` | — |

**Description:** The function FIND searches for a partial string within a string. FIND (STR1, STR2) means: Find the position of the first character where STR2 appears in STR1 for the first time. If STR2 is not found in STR1, then OUT:=0.

##### `INSERT` (FUNCTION) → `STRING` — §4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR1` | `STRING(255)` | — |
| `STR2` | `STRING(255)` | — |
| `POS` | `INT` | — |

**Description:** The function INSERT inserts a string into another string at a defined point. INSERT (STR1, STR2, POS) means: Insert STR2 into STR1 after position POS.

##### `LEFT` (FUNCTION) → `STRING` — §4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR` | `STRING(255)` | — |
| `SIZE` | `INT` | — |

**Description:** The function LEFT returns the left, initial string for a given string. LEFT (STR, SIZE) means: Take the first SIZE character from the left in the string STR.

##### `LEN` (FUNCTION) → `INT` — §4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR` | `STRING(255)` | — |
| `Var1` | `= LEN ('SUSI')` | — |

**Description:** The function LEN returns the length of a string.

##### `MID` (FUNCTION) → `STRING` — §4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR` | `STRING(255)` | — |
| `LEN` | `INT` | — |
| `POS` | `INT` | — |

**Description:** The function MID returns a partial string from within a string. MID (STR, LEN, POS) means: Retrieve LEN characters from the STR string beginning with the character at position POS.

##### `REPLACE` (FUNCTION) → `STRING` — §4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR1` | `STRING(255)` | — |
| `STR2` | `STRING(255)` | — |
| `L` | `INT` | — |
| `P` | `INT` | — |

**Description:** The function REPLACE replaces a partial string from a larger string with a third string. REPLACE (STR1, STR2, L, P) means: Replace L characters from STR1 with STR2 beginning with the character in the P position.

##### `RIGHT` (FUNCTION) → `STRING` — §4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR` | `STRING(255)` | — |
| `SIZE` | `INT` | — |

**Description:** The RIGHT function returns a right-hand start string of a string. RIGHT (STR, SIZE) means: take the first SIZE characters from the right in the STR string.

#### String functions (WSTRING)

##### `WCONCAT` (FUNCTION) → `WSTRING` — §5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR1` | `WSTRING(255); (*Head part of the concatenated result*)` | — |
| `STR2` | `WSTRING(255); (*Tail part of the concatenated result*)` | — |

**Description:** Concatenation (combination) of two WSTRINGs.

##### `WDELETE` (FUNCTION) → `WSTRING` — §5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR1` | `WSTRING(255)` | — |
| `LEN` | `INT` | — |
| `POS` | `INT` | — |

**Description:** The function WDELETE deletes part of a WSTRING from a certain point. The input STR is of type WSTRING. LEN and POS are of type INT. The return value of the function is of type WSTRING. WDELETE (STR, LEN, POS) means: Delete LEN characters from STR beginning with the POSth character.

##### `WFIND` (FUNCTION) → `INT` — §5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR1` | `WSTRING(255)` | — |
| `STR2` | `WSTRING(255)` | — |

**Description:** The function WFIND searches for a part in WSTRING. WFIND (STR1, STR2) means: Find the position of the first character of first occurrence of STR2 in STR1. If STR2 does not occur in STR1, then OUT := 0 applies.

##### `WINSERT` (FUNCTION) → `WSTRING` — §5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR1` | `WSTRING(255)` | — |
| `STR2` | `WSTRING(255)` | — |
| `POS` | `INT` | — |

**Description:** The function WINSERT adds a WString in another WString from a particular location. WINSERT (STR1, STR2, POS) means: Add STR2 in STR1 after the POSth position.

##### `WLEFT` (FUNCTION) → `WSTRING` — §5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR` | `WSTRING(255)` | — |
| `SIZE` | `INT` | — |

**Description:** The function WLEFT supplies a left-hand start string for a WSTRING. WLEFT (STR, SIZE) means: Take the first SIZE characters from the left in WString STR.

##### `WLEN` (FUNCTION) → `INT` — §5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR` | `WSTRING(255)` | — |

**Description:** The function WLEN outputs the length of a WSTRING.

##### `WMID` (FUNCTION) → `WSTRING` — §5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR` | `WSTRING(255)` | — |
| `LEN` | `INT` | — |
| `POS` | `INT` | — |

**Description:** String functions (WSTRING)

##### `WREPLACE` (FUNCTION) → `WSTRING` — §5.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR1` | `WSTRING(255)` | — |
| `STR2` | `WSTRING(255)` | — |
| `L` | `INT` | — |
| `P` | `INT` | — |

**Description:** The function WREPLACE replaces a substring of a WSTRING with another WSTRING. WREPLACE (STR1, STR2, L, P) means: Replace L characters from STR1 with STR2 beginning with the Pth character.

##### `WRIGHT` (FUNCTION) → `WSTRING` — §5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR` | `WSTRING(255)` | — |
| `SIZE` | `INT` | — |

**Description:** 28
