# Tc2_Math

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_Math_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T0 · **API entries:** 8
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Mathematical functions: pure-ST compatibility library.

---

#### Functions

##### `CEIL` (FUNCTION) → `LREAL` — §3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `lr_in` | `LREAL` | Function parameters of type LREAL Comparison with other rounding functions |

**Description:** The CEIL function determines an integer value from a floating point number that is lower than or equal to that number. It is always rounded up to the larger number.

##### `FLOOR` (FUNCTION) → `LREAL` — §3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `lr_in` | `LREAL 10` | Function parameters of type LREAL Functions Comparison with other rounding functions |

**Description:** The FLOOR function determines an integer value from a floating point number that is lower than or equal to that number. It is always rounded down to the lower number.

##### `FRAC` (FUNCTION) → `LREAL` — §3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `lr_in` | `LREAL` | Function parameters of type LREAL |

**Description:** The FRAC function determines the decimal component of a floating point number.

##### `LMOD` (FUNCTION) → `LREAL` — §3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `lr_Value` | `LREAL` | Input value |
| `lr_Arg` | `LREAL` | Modulo range |

**Description:** 12

##### `LTRUNC` (FUNCTION) → `LREAL` — §3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `lr_in` | `LREAL` | Function parameters of type LREAL Comparison with other rounding functions |

**Description:** The LTRUNC function determines the integer part of a floating point number.

##### `MODABS` (FUNCTION) → `LREAL` — §3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `lr_val` | `LREAL` | Input value |
| `lr_mod` | `LREAL` | Modulo range Functions |

**Description:** The MODABS function performs a modulo division and determines the unsigned modulo value within the modulo range.

##### `MODTURNS` (FUNCTION) → `DINT` — §3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `lr_Value` | `LREAL` | Input value |
| `lr_Arg` | `LREAL 16` | Modulo range Functions |

**Description:** The MODTURNS function carries out a modulo division and determines the signed integer component (modulo periods, modulo rotations).

#### [obsolete functions]

##### `F_GetVersionTcMath` (FUNCTION) → `UINT` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nVersionElement` | `INT` | Version element to be read. Possible parameters: |

**Description:** This function can be used to read PLC library version information.
