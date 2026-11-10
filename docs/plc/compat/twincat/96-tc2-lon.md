# Tc2_LON

- **Manual:** `TwinCAT_3_PLC_Lib_Tc2_LON_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T2 · **API entries:** 371
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** LonWorks: needs a LonWorks/IP-852 stack.

---

#### SNVT_angle_vel Read [} 36] Send [} 245]

##### `POUs` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bResetMaxBuffer` | `BOOL` | Deletes the value rMaxBuffer for the maximum utilization of |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stParameter_IN` | `ST_LON_Parameter_IN_36B` | — |
| `stParameter_OUT` | `ST_LON_Parameter_OUT_36B` | — |
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReady` | `BOOL` | The initialization is complete. |
| `bBusy` | `BOOL` | The output is TRUE as long as values are sent. |
| `bTimeOut` | `BOOL` | Timeout during initialization. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** Read

#### SNVT_angle_vel Read [} 36] Send [} 245] › POUs › Read

##### `FB_READ_001_SNVT_amp` — §4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3276.8 / Max: 3276.7. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_002_SNVT_amp_mil` — §4.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3276.8 / Max: 3276.7. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_003_SNVT_angle` — §4.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 65,535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_004_SNVT_angle_vel` — §4.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3276.8 / Max: 3276.7. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_005_SNVT_btu_kilo` — §4.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_006_SNVT_btu_mega` — §4.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_007_SNVT_char_ascii` — §4.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sValue` | `STRING` | STRING(1). |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_008_SNVT_count` — §4.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** 40

##### `FB_READ_009_SNVT_count_inc` — §4.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `iValue` | `INT` | Min: -32768 / Max: 32767. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_011_SNVT_date_day` — §4.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_days_of_week_t [} 593]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_013_SNVT_elec_kwh` — §4.1.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_014_SNVT_elec_whr` — §4.1.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. Programming |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_015_SNVT_flow` — §4.1.2.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `46 Programming` | `` | — |
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_016_SNVT_flow_mil` — §4.1.2.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_017_SNVT_length` — §4.1.2.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_018_SNVT_length_kilo` — §4.1.2.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_019_SNVT_length_micr` — §4.1.2.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_020_SNVT_length_mil` — §4.1.2.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_021_SNVT_lev_cont` — §4.1.2.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `52 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 100. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_023_SNVT_mass` — §4.1.2.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_024_SNVT_mass_kilo` — §4.1.2.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_025_SNVT_mass_mega` — §4.1.2.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_026_SNVT_mass_mil` — §4.1.2.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_027_SNVT_power` — §4.1.2.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** Programming

##### `FB_READ_028_SNVT_power_kilo` — §4.1.2.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_029_SNVT_ppm` — §4.1.2.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_030_SNVT_press` — §4.1.2.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3276.8 / Max: 3276.7. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_031_SNVT_res` — §4.1.2.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_032_SNVT_res_kilo` — §4.1.2.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_033_SNVT_sound_db` — §4.1.2.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -327.68 / Max: 327.67. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_034_SNVT_speed` — §4.1.2.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_035_SNVT_speed_mil` — §4.1.2.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 65,535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_036_SNVT_str_asc` — §4.1.2.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sValue` | `STRING` | STRING(31) |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_037_SNVT_str_int` — §4.1.2.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_str_int [} 695]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_038_SNVT_telcom` — §4.1.2.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_telcom_states_t [} 627]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_039_SNVT_temp` — §4.1.2.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL 70` | Min: 0 / Max: 6553.5. Programming |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_041_SNVT_vol` — §4.1.2.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_042_SNVT_vol_kilo` — §4.1.2.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `72 Programming` | `` | — |
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_043_SNVT_vol_mil` — §4.1.2.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_044_SNVT_volt` — §4.1.2.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3276.8 / Max: 3276.7. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_045_SNVT_volt_dbmv` — §4.1.2.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -327.68 / Max: 327.67. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_046_SNVT_volt_kilo` — §4.1.2.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3276.8 / Max: 3276.7. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_047_SNVT_volt_mil` — §4.1.2.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3276.8 / Max: 3276.7. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_048_SNVT_amp_f` — §4.1.2.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_049_SNVT_angle_f` — §4.1.2.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_050_SNVT_angle_vel_f` — §4.1.2.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_051_SNVT_count_f` — §4.1.2.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_052_SNVT_count_inc_f` — §4.1.2.48

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** 82

##### `FB_READ_053_SNVT_flow_f` — §4.1.2.49

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_054_SNVT_length_f` — §4.1.2.50

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_055_SNVT_lev_cont_f` — §4.1.2.51

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 100. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_056_SNVT_mass_f` — §4.1.2.52

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_057_SNVT_power_f` — §4.1.2.53

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_058_SNVT_ppm_f` — §4.1.2.54

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_059_SNVT_press_f` — §4.1.2.55

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_060_SNVT_res_f` — §4.1.2.56

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_061_SNVT_sound_db_f` — §4.1.2.57

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_062_SNVT_speed_f` — §4.1.2.58

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_063_SNVT_temp_f` — §4.1.2.59

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -273.17 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_064_SNVT_time_f` — §4.1.2.60

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_065_SNVT_vol_f` — §4.1.2.61

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_066_SNVT_volt_f` — §4.1.2.62

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_067_SNVT_btu_f` — §4.1.2.63

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_068_SNVT_elec_whr_f` — §4.1.2.64

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_069_SNVT_config_src` — §4.1.2.65

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_config_source_t [} 589]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_070_SNVT_color` — §4.1.2.66

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_color [} 674]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_071_SNVT_grammage` — §4.1.2.67

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL 102` | Min: 0 / Max: 6553.5. Programming |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_072_SNVT_grammage_f` — §4.1.2.68

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_073_SNVT_file_req` — §4.1.2.69

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `104 Programming` | `` | — |
| `stValue` | `ST_LON_SNVT_file_req [} 680]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_074_SNVT_file_status` — §4.1.2.70

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_file_status` | Structure of the data that was received. If stValue.eStatus |
| `[} 681]` | `= eLON_FS_LOOKUP_OK (1), then the structure stValue.stAddr.stDescriptor is valid, otherwise` | — |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_075_SNVT_freq_f` — §4.1.2.71

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_076_SNVT_freq_hz` — §4.1.2.72

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_077_SNVT_freq_kilohz` — §4.1.2.73

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_078_SNVT_freq_milhz` — §4.1.2.74

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_079_SNVT_lux` — §4.1.2.75

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_081_SNVT_lev_percent` — §4.1.2.76

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -163.84 / Max: 163.835 |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_082_SNVT_multiplier` — §4.1.2.77

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 32.7675 |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_083_SNVT_state` — §4.1.2.78

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrValue` | `ARRAY OF BOOL` | 0–15 bit |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_084_SNVT_time_stamp` — §4.1.2.79

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `TIMESTRUCT` | Structure of the data that was received. The structure |

**Description:** 114

##### `FB_READ_085_SNVT_zerospan` — §4.1.2.80

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_zerospan [} 698]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_086_SNVT_magcard` — §4.1.2.81

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrValue` | `ARRAY OF BYTE` | 1–40 BYTE. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_087_SNVT_elapsed_tm` — §4.1.2.82

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_elapsed_tm Structure of the data that was received. [} 677] 118 Programming` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_088_SNVT_alarm` — §4.1.2.83

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `stValue` | `ST_LON_SNVT_alarm [} 669]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_089_SNVT_currency` — §4.1.2.84

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `120 Programming` | `` | — |
| `stValue` | `ST_LON_SNVT_currency [} 675]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_090_SNVT_file_pos` — §4.1.2.85

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_file_pos [} 680]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_091_SNVT_muldiv` — §4.1.2.86

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_muldiv [} 685]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_092_SNVT_obj_request` — §4.1.2.87

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_obj_reques Structure of the data that was received. t [} 686]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_093_SNVT_obj_status` — §4.1.2.88

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `124 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_obj_status Structure of the data that was received. [} 686]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_094_SNVT_preset` — §4.1.2.89

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_preset [} 688]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_095_SNVT_switch` — §4.1.2.90

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_switch [} 695]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_096_SNVT_trans_table` — §4.1.2.91

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_trans_table Structure of the data that was received. [} 698]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_097_SNVT_override` — §4.1.2.92

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_override_t [} 611]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_098_SNVT_pwr_fact` — §4.1.2.93

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -1 / Max: 1. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_099_SNVT_pwr_fact_f` — §4.1.2.94

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -1 / Max: 1. Programming |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_100_SNVT_density` — §4.1.2.95

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `132 Programming` | `` | — |
| `rValue` | `REAL` | Min: 0 / Max: 32767.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_101_SNVT_density_f` — §4.1.2.96

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_102_SNVT_rpm` — §4.1.2.97

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_103_SNVT_hvac_emerg` — §4.1.2.98

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_emerg_t [} 596]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_104_SNVT_angle_deg` — §4.1.2.99

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -359.98 / Max: 360. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_105_SNVT_temp_p` — §4.1.2.100

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -273.17 / Max: 327.67. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_106_SNVT_temp_setpt` — §4.1.2.101

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `138 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_temp_setpt Structure of the data that was received. [} 696]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_107_SNVT_time_sec` — §4.1.2.102

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 6553.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_108_SNVT_hvac_mode` — §4.1.2.103

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_hvac_t [} 607]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_109_SNVT_occupancy` — §4.1.2.104

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_occup_t [} 611].` | Enum to be sent. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_110_SNVT_area` — §4.1.2.105

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 13,107. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_111_SNVT_hvac_overid` — §4.1.2.106

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_hvac_overi Structure of the data that was received. d [} 681]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** Programming

##### `FB_READ_112_SNVT_hvac_status` — §4.1.2.107

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_hvac_status Structure of the data that was received. [} 683]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_113_SNVT_press_p` — §4.1.2.108

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `iValue` | `INT` | Min: -32768 / Max: 32767. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_114_SNVT_address` — §4.1.2.109

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 16384 / Max: 64767. Programming |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_115_SNVT_scene` — §4.1.2.110

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `148 Programming` | `` | — |
| `stValue` | `ST_LON_SNVT_scene [} 694]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_116_SNVT_scene_cfg` — §4.1.2.111

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_scene_cfg [} 694]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_117_SNVT_setting` — §4.1.2.112

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_setting [} 695]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_118_SNVT_evap_state` — §4.1.2.113

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_evap_t [} 599]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_119_SNVT_therm_mode` — §4.1.2.114

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_therm_mode_t [} 628]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_120_SNVT_defr_mode` — §4.1.2.115

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_defrost_mode_t [} 593]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_121_SNVT_defr_term` — §4.1.2.116

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_defrost_term_t [} 594]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_122_SNVT_defr_state` — §4.1.2.117

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_defrost_state_t [} 593]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_123_SNVT_time_min` — §4.1.2.118

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_124_SNVT_time_hour` — §4.1.2.119

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** Programming

##### `FB_READ_125_SNVT_ph` — §4.1.2.120

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -32,768 / Max: 32,767. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_126_SNVT_ph_f` — §4.1.2.121

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -3.40E+38 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_127_SNVT_chlr_status` — §4.1.2.122

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_chlr_status Structure of the data that was received. [} 670]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_128_SNVT_tod_event` — §4.1.2.123

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_tod_event Structure of the data that was received. [} 697]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_129_SNVT_smo_obscur` — §4.1.2.124

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 5. Programming |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_130_SNVT_fire_test` — §4.1.2.125

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `164 Programming` | `` | — |
| `eValue` | `E_LON_fire_test_t [} 602]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_131_SNVT_temp_ror` — §4.1.2.126

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `rValue` | `REAL` | Min: -16384 / Max: 16383.5. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_132_SNVT_fire_init` — §4.1.2.127

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_fire_initiator_t [} 601]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_133_SNVT_fire_indcte` — §4.1.2.128

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_fire_indicator_t [} 601]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_134_SNVT_time_zone` — §4.1.2.129

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_time_zone Structure of the data that was received. [} 697]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_135_SNVT_earth_pos` — §4.1.2.130

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_earth_pos [} 677]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_136_SNVT_reg_val` — §4.1.2.131

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_reg_val [} 693]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_137_SNVT_reg_val_ts` — §4.1.2.132

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_reg_val_ts [} 693]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_138_SNVT_volt_ac` — §4.1.2.133

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_139_SNVT_amp_ac` — §4.1.2.134

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** Programming

##### `FB_READ_143_SNVT_turbidity` — §4.1.2.135

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 65,535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_144_SNVT_turbidity_f` — §4.1.2.136

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 3.40E+38. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_145_SNVT_hvac_type` — §4.1.2.137

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_hvac_hvt_t [} 604] Enum to be received.` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_146_SNVT_elec_kwh_l` — §4.1.2.138

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IrValue` | `LREAL` | Min: -214748364.8 / Max: 214748364.7. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_147_SNVT_temp_diff_p` — §4.1.2.139

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -327.68 / Max: 327.67. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_148_SNVT_ctrl_req` — §4.1.2.140

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_ctrl_req [} 674]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_149_SNVT_ctrl_resp` — §4.1.2.141

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_ctrl_resp [} 675]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_150_SNVT_ptz` — §4.1.2.142

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_ptz [} 689] Structure of the data that was received. 182 Programming` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_151_SNVT_privacyzone` — §4.1.2.143

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `stValue` | `ST_LON_SNVT_privacyzon Structure of the data that was received. e [} 689]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_152_SNVT_pos_ctrl` — §4.1.2.144

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_pos_ctrl [} 688]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_153_SNVT_enthalpy` — §4.1.2.145

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -327.68 / Max: 327.67. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_154_SNVT_gfci_status` — §4.1.2.146

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_gfci_status_t [} 603]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_155_SNVT_motor_state` — §4.1.2.147

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_motor_state_t [} 609]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_156_SNVT_pumpset_mn` — §4.1.2.148

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_pumpset_` | Structure of the data that was received. mn [} 690] |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_157_SNVT_ex_control` — §4.1.2.149

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_ex_control Structure of the data that was received. [} 680]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_158_SNVT_pumpset_sn` — §4.1.2.150

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_pumpset_s Structure of the data that was received. n [} 691]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_159_SNVT_pump_sensor` — §4.1.2.151

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_pump_sens Structure of the data that was received. or [} 689]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_160_SNVT_abs_humid` — §4.1.2.152

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: 0 / Max: 655.35. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_161_SNVT_flow_p` — §4.1.2.153

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL 194` | Min: 0 / Max: 655.35. Programming |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_162_SNVT_dev_c_mode` — §4.1.2.154

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `eValue` | `E_LON_device_c_mode_t [} 595]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_163_SNVT_valve_mode` — §4.1.2.155

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |
| `stLON_Com` | `ST_LON_Communication` | — |
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `196 Programming` | `` | — |
| `eValue` | `E_LON_valve_mode_t [} 629]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |
| `stValue` | `ST_LON_SNVT_alarm_2 [} 670]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |
| `arrValue` | `ARRAY OF BOOL` | 0–63 bit. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_166_SNVT_nv_type` — §4.1.2.158

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_nv_type [} 685]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_168_SNVT_ent_opmode` — §4.1.2.159

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `200 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_ent_opmode_cmd_ Enum to be received. t [} 598]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_169_SNVT_ent_state` — §4.1.2.160

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_ent_cmd_t [} 597] Enum to be received.` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_170_SNVT_ent_status` — §4.1.2.161

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_ent_status Structure of the data that was received. [} 678]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_171_SNVT_flow_dir` — §4.1.2.162

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `eValue` | `E_LON_flow_direction_t [} 603]` | Enum to be received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_172_SNVT_hvac_satsts` — §4.1.2.163

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_hvac_satsts Structure of the data that was received. [} 682]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_173_SNVT_dev_status` — §4.1.2.164

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_dev_status Structure of the data that was received. [} 676] 206 Programming` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_174_SNVT_dev_fault` — §4.1.2.165

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `stValue` | `ST_LON_SNVT_dev_fault [} 676]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_175_SNVT_dev_maint` — §4.1.2.166

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `208 Programming` | `` | — |
| `stValue` | `ST_LON_SNVT_dev_maint Structure of the data that was received. [} 676]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_176_SNVT_date_event` — §4.1.2.167

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_date_event Structure of the data that was received. [} 675]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_177_SNVT_sched_val` — §4.1.2.168

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `byValue` | `BYTE` | Min: 0 / Max: 255. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_180_SNVT_sblnd_state` — §4.1.2.169

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_sblnd_state Structure of the data that was received. [} 694]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_181_SNVT_rac_ctrl` — §4.1.2.170

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `212 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_rac_ctrl [} 692]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_182_SNVT_rac_req` — §4.1.2.171

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_rac_req [} 692]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |
| `udiValue` | `UDINT` | Min: 0 / Max: 4294967294. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_184_SNVT_clothes_w_c` — §4.1.2.173

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_clothes_w_ Structure of the data that was received. c [} 672]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_185_SNVT_clothes_w_m` — §4.1.2.174

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_clothes_w_ Structure of the data that was received. m [} 673]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** 216

##### `FB_READ_186_SNVT_clothes_w_s` — §4.1.2.175

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_clothes_w_ Structure of the data that was received. s [} 673]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_187_SNVT_clothes_w_a` — §4.1.2.176

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_clothes_w_ Structure of the data that was received. a [} 671]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_188_SNVT_multiplier_s` — §4.1.2.177

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |
| `stLON_Com` | `ST_LON_Communication` | — |
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL 220` | Min: 0 / Max: 2.54. Programming |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has Programming |
| `stValue` | `ST_LON_SNVT_switch_2 [} 696]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |
| `stValue` | `ST_LON_SNVT_color_2 [} 674]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_191_SNVT_log_status` — §4.1.2.180

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_log_status Structure of the data that was received. [} 684]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_192_SNVT_time_stamp_p` — §4.1.2.181

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `TIMESTRUCT` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_193_SNVT_log_fx_request` — §4.1.2.182

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_log_fx_req Structure of the data that was received. uest [} 683]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_194_SNVT_log_fx_status` — §4.1.2.183

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_log_fx_stat Structure of the data that was received. us [} 684]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_195_SNVT_log_request` — §4.1.2.184

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_196_SNVT_enthalpy_d` — §4.1.2.185

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rValue` | `REAL` | Min: -327.68 / Max: 327.66. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** 228

##### `FB_READ_197_SNVT_amp_ac_mil` — §4.1.2.186

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `uiValue` | `UINT` | Min: 0 / Max: 65535. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** Application

##### `FB_READ_198_SNVT_time_hour_p` — §4.1.2.187

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `udiValue` | `UDINT` | Min: 0 / Max: 4294967294. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_199_SNVT_lamp_status` — §4.1.2.188

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stValue` | `ST_LON_SNVT_lamp_statu Structure of the data that was received. s [} 683] 232 Programming` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_200_SNVT_environment` — §4.1.2.189

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `stValue` | `ST_LON_SNVT_environme Structure of the data that was received. nt [} 679]` | — |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

##### `FB_READ_201_SNVT_geo_loc` — §4.1.2.190

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This is required for binding the LON nodes. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `234 Programming` | `` | — |
| `stValue` | `ST_LON_SNVT_geo_loc [} 681]` | Structure of the data that was received. |
| `bNewData` | `BOOL` | Becomes TRUE for 1 cycle when the function block has |

**Description:** This function block receives the following LON input variable (nvi):

#### SNVT_angle_vel Read [} 36] Send [} 245] › POUs › Send

##### `FB_SEND_001_SNVT_amp` — §4.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `240 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_002_SNVT_amp_mil` — §4.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_003_SNVT_angle` — §4.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `244 Programming` | `` | — |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_004_SNVT_angle_vel` — §4.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_005_SNVT_btu_kilo` — §4.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_006_SNVT_btu_mega` — §4.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_007_SNVT_char_ascii` — §4.1.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_008_SNVT_count` — §4.1.3.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL 252` | The bBusy output is TRUE as long as values are sent. Programming |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_009_SNVT_count_inc` — §4.1.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_011_SNVT_date_day` — §4.1.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_013_SNVT_elec_kwh` — §4.1.3.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `256 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_014_SNVT_elec_whr` — §4.1.3.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_015_SNVT_flow` — §4.1.3.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `260 Programming` | `` | — |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_016_SNVT_flow_mil` — §4.1.3.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_017_SNVT_length` — §4.1.3.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_018_SNVT_length_kilo` — §4.1.3.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_019_SNVT_length_micr` — §4.1.3.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_020_SNVT_length_mil` — §4.1.3.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_021_SNVT_lev_cont` — §4.1.3.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_023_SNVT_mass` — §4.1.3.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_024_SNVT_mass_kilo` — §4.1.3.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_025_SNVT_mass_mega` — §4.1.3.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_026_SNVT_mass_mil` — §4.1.3.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_027_SNVT_power` — §4.1.3.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_028_SNVT_power_kilo` — §4.1.3.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_029_SNVT_ppm` — §4.1.3.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_030_SNVT_press` — §4.1.3.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_031_SNVT_res` — §4.1.3.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_032_SNVT_res_kilo` — §4.1.3.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_033_SNVT_sound_db` — §4.1.3.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_034_SNVT_speed` — §4.1.3.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_035_SNVT_speed_mil` — §4.1.3.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_036_SNVT_str_asc` — §4.1.3.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_037_SNVT_str_int` — §4.1.3.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `294 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_038_SNVT_telcom` — §4.1.3.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_039_SNVT_temp` — §4.1.3.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_041_SNVT_vol` — §4.1.3.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_042_SNVT_vol_kilo` — §4.1.3.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_043_SNVT_vol_mil` — §4.1.3.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_044_SNVT_volt` — §4.1.3.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_045_SNVT_volt_dbmv` — §4.1.3.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_046_SNVT_volt_kilo` — §4.1.3.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_047_SNVT_volt_mil` — §4.1.3.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_048_SNVT_amp_f` — §4.1.3.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_049_SNVT_angle_f` — §4.1.3.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_050_SNVT_angle_vel_f` — §4.1.3.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_051_SNVT_count_f` — §4.1.3.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_052_SNVT_count_inc_f` — §4.1.3.48

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_053_SNVT_flow_f` — §4.1.3.49

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_054_SNVT_length_f` — §4.1.3.50

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_055_SNVT_lev_cont_f` — §4.1.3.51

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_056_SNVT_mass_f` — §4.1.3.52

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_057_SNVT_power_f` — §4.1.3.53

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_058_SNVT_ppm_f` — §4.1.3.54

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_059_SNVT_press_f` — §4.1.3.55

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** Application

##### `FB_SEND_060_SNVT_res_f` — §4.1.3.56

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_061_SNVT_sound_db_f` — §4.1.3.57

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_062_SNVT_speed_f` — §4.1.3.58

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_063_SNVT_temp_f` — §4.1.3.59

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_064_SNVT_time_f` — §4.1.3.60

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_065_SNVT_vol_f` — §4.1.3.61

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_066_SNVT_volt_f` — §4.1.3.62

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_067_SNVT_btu_f` — §4.1.3.63

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_068_SNVT_elec_whr_f` — §4.1.3.64

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_069_SNVT_config_src` — §4.1.3.65

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_070_SNVT_color` — §4.1.3.66

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_071_SNVT_grammage` — §4.1.3.67

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `348 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_072_SNVT_grammage_f` — §4.1.3.68

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_073_SNVT_file_req` — §4.1.3.69

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_074_SNVT_file_status` — §4.1.3.70

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_075_SNVT_freq_f` — §4.1.3.71

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** 354

##### `FB_SEND_076_SNVT_freq_hz` — §4.1.3.72

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_077_SNVT_freq_kilohz` — §4.1.3.73

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_078_SNVT_freq_milhz` — §4.1.3.74

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_079_SNVT_lux` — §4.1.3.75

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_081_SNVT_lev_percent` — §4.1.3.76

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_082_SNVT_multiplier` — §4.1.3.77

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_083_SNVT_state` — §4.1.3.78

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_084_SNVT_time_stamp` — §4.1.3.79

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_085_SNVT_zerospan` — §4.1.3.80

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_086_SNVT_magcard` — §4.1.3.81

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_087_SNVT_elapsed_tm` — §4.1.3.82

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_088_SNVT_alarm` — §4.1.3.83

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_089_SNVT_currency` — §4.1.3.84

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_090_SNVT_file_pos` — §4.1.3.85

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_091_SNVT_muldiv` — §4.1.3.86

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_092_SNVT_obj_request` — §4.1.3.87

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_093_SNVT_obj_status` — §4.1.3.88

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_094_SNVT_preset` — §4.1.3.89

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_095_SNVT_switch` — §4.1.3.90

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_096_SNVT_trans_table` — §4.1.3.91

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `386 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_097_SNVT_override` — §4.1.3.92

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_098_SNVT_pwr_fact` — §4.1.3.93

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_099_SNVT_pwr_fact_f` — §4.1.3.94

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_100_SNVT_density` — §4.1.3.95

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_101_SNVT_density_f` — §4.1.3.96

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_102_SNVT_rpm` — §4.1.3.97

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_103_SNVT_hvac_emerg` — §4.1.3.98

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_104_SNVT_angle_deg` — §4.1.3.99

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_105_SNVT_temp_p` — §4.1.3.100

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_106_SNVT_temp_setpt` — §4.1.3.101

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_107_SNVT_time_sec` — §4.1.3.102

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `404 Programming` | `` | — |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** Programming

##### `FB_SEND_108_SNVT_hvac_mode` — §4.1.3.103

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_109_SNVT_occupancy` — §4.1.3.104

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_110_SNVT_area` — §4.1.3.105

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `408 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_111_SNVT_hvac_overid` — §4.1.3.106

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_112_SNVT_hvac_status` — §4.1.3.107

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_113_SNVT_press_p` — §4.1.3.108

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_114_SNVT_address` — §4.1.3.109

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_115_SNVT_scene` — §4.1.3.110

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_116_SNVT_scene_cfg` — §4.1.3.111

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL 418` | The bBusy output is TRUE as long as values are sent. Programming |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_117_SNVT_setting` — §4.1.3.112

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_118_SNVT_evap_state` — §4.1.3.113

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_119_SNVT_therm_mode` — §4.1.3.114

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `422 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_120_SNVT_defr_mode` — §4.1.3.115

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_121_SNVT_defr_term` — §4.1.3.116

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_122_SNVT_defr_state` — §4.1.3.117

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_123_SNVT_time_min` — §4.1.3.118

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `428 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_124_SNVT_time_hour` — §4.1.3.119

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_125_SNVT_ph` — §4.1.3.120

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `432 Programming` | `` | — |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_126_SNVT_ph_f` — §4.1.3.121

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_127_SNVT_chlr_status` — §4.1.3.122

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_128_SNVT_tod_event` — §4.1.3.123

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `436 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_129_SNVT_smo_obscur` — §4.1.3.124

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_130_SNVT_fire_test` — §4.1.3.125

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_131_SNVT_temp_ror` — §4.1.3.126

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_132_SNVT_fire_init` — §4.1.3.127

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `442 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_133_SNVT_fire_indcte` — §4.1.3.128

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_134_SNVT_time_zone` — §4.1.3.129

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_135_SNVT_earth_pos` — §4.1.3.130

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_136_SNVT_reg_val` — §4.1.3.131

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `448 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_137_SNVT_reg_val_ts` — §4.1.3.132

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_138_SNVT_volt_ac` — §4.1.3.133

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_139_SNVT_amp_ac` — §4.1.3.134

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_143_SNVT_turbidity` — §4.1.3.135

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_144_SNVT_turbidity_f` — §4.1.3.136

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_145_SNVT_hvac_type` — §4.1.3.137

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_146_SNVT_elec_kwh_l` — §4.1.3.138

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_147_SNVT_temp_diff_p` — §4.1.3.139

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_148_SNVT_ctrl_req` — §4.1.3.140

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_149_SNVT_ctrl_resp` — §4.1.3.141

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_150_SNVT_ptz` — §4.1.3.142

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_151_SNVT_privacyzone` — §4.1.3.143

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_152_SNVT_pos_ctrl` — §4.1.3.144

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_153_SNVT_enthalpy` — §4.1.3.145

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_154_SNVT_gfci_status` — §4.1.3.146

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_155_SNVT_motor_state` — §4.1.3.147

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_156_SNVT_pumpset_mn` — §4.1.3.148

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_157_SNVT_ex_control` — §4.1.3.149

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_158_SNVT_pumpset_sn` — §4.1.3.150

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_159_SNVT_pump_sensor` — §4.1.3.151

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_160_SNVT_abs_humid` — §4.1.3.152

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. Programming |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_161_SNVT_flow_p` — §4.1.3.153

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_162_SNVT_dev_c_mode` — §4.1.3.154

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_163_SNVT_valve_mode` — §4.1.3.155

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |
| `wNVIndex` | `WORD 488` | Unique index. This index is required for binding the LON Programming |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |
| `stLON_Com` | `ST_LON_Communication` | — |
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_166_SNVT_nv_type` — §4.1.3.158

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_168_SNVT_ent_opmode` — §4.1.3.159

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_169_SNVT_ent_state` — §4.1.3.160

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_170_SNVT_ent_status` — §4.1.3.161

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `494 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_171_SNVT_flow_dir` — §4.1.3.162

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_172_SNVT_hvac_satsts` — §4.1.3.163

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_173_SNVT_dev_status` — §4.1.3.164

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_174_SNVT_dev_fault` — §4.1.3.165

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `500 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_175_SNVT_dev_maint` — §4.1.3.166

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_176_SNVT_date_event` — §4.1.3.167

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_177_SNVT_sched_val` — §4.1.3.168

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_180_SNVT_sblnd_state` — §4.1.3.169

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `506 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_181_SNVT_rac_ctrl` — §4.1.3.170

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_182_SNVT_rac_req` — §4.1.3.171

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_184_SNVT_clothes_w_c` — §4.1.3.173

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `512 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_185_SNVT_clothes_w_m` — §4.1.3.174

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_186_SNVT_clothes_w_s` — §4.1.3.175

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_187_SNVT_clothes_w_a` — §4.1.3.176

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_188_SNVT_multiplier_s` — §4.1.3.177

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `518 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON Programming |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |
| `stLON_Com` | `ST_LON_Communication` | — |
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_191_SNVT_log_status` — §4.1.3.180

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_192_SNVT_time_stamp_p` — §4.1.3.181

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `524 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_193_SNVT_log_fx_request` — §4.1.3.182

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_194_SNVT_log_fx_status` — §4.1.3.183

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_195_SNVT_log_request` — §4.1.3.184

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_196_SNVT_enthalpy_d` — §4.1.3.185

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_197_SNVT_amp_ac_mil` — §4.1.3.186

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_198_SNVT_time_hour_p` — §4.1.3.187

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_199_SNVT_lamp_status` — §4.1.3.188

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_200_SNVT_environment` — §4.1.3.189

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):

##### `FB_SEND_201_SNVT_geo_loc` — §4.1.3.190

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `538 Programming` | `` | — |
| `wNVIndex` | `WORD` | Unique index. This index is required for binding the LON |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stLON_Com` | `ST_LON_Communication` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The bBusy output is TRUE as long as values are sent. |
| `bError` | `BOOL` | This output goes TRUE as soon as an error occurs. This |

**Description:** This function block sends the following LON output variable (nvo):
