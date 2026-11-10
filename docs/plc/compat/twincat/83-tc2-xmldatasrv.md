# Tc2_XmlDataSrv

- **Manual:** `TF6421_TC3_XML_Server_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 8
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** XML data server needs HTTP/socket service + XML store.

---

#### PLC libraries › Function blocks

##### `FB_XmlSrvRead` — §4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT 3 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated and |

**Description:** pSymAddr PVOID cbSymSize UDINT sFilePath T_MaxString sXPath T_MaxString bExecute BOOL tTimeout TIME

##### `FB_XmlSrvWrite` — §4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT 3 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated and |

**Description:** pSymAddr PVOID cbSymSize UDINT sFilePath T_MaxString sXPath T_MaxString bExecute BOOL tTimeout TIME

##### `FB_XmlSrvReadByName` — §4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT 3 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated and |

**Description:** sSymName T_MaxString sFilePath T_MaxString sXPath T_MaxString bExecute BOOL tTimeout TIME

##### `FB_XmlSrvWriteByName` — §4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `PLC libraries` | `` | — |
| `sNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT 3 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated and |

**Description:** sSymName T_MaxString sFilePath T_MaxString sXPath T_MaxString bExecute BOOL tTimeout TIME

##### `FB_XmlSrvEnableObfuscation` — §4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT 3 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated and |

**Description:** The FB_XmlSrvEnableObfuscation function block enables the obfuscation mode. XML files are then only stored on the hard disk with AES-256 encryption.

##### `FB_XmlSrvDisableObfuscation` — §4.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT 3 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated and |

**Description:** The FB_XmlSrvDisableObfuscation function block disables the obfuscation mode.

##### `FB_XmlSrvIsObfuscationEnabled` — §4.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT 3 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated and |

**Description:** The FB_XmlSrvIsObfuscationEnabled function block queries whether the obfuscation mode is enabled or disabled.

##### `FB_XmlSrvObfuscationExport` — §4.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `PLC libraries` | `` | — |
| `sNetId` | `T_AmsNetId` | String containing the network address of the TwinCAT 3 |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output is set when the function block is activated and |

**Description:** The FB_XmlSrvObfuscationExport function block decrypts an obfuscated XML file and stores it unencrypted on the hard disk.
