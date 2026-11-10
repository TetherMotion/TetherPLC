# Tc2_SMS+Tc2_SMTP

- **Manual:** `TF6350_TC3_SMS_SMTP_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 7
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** SMS needs GSM modem (serial); SMTP needs TCP socket + TLS.

---

#### TwinCAT SMTP › PLC API

##### `Function blocks` — §4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | AmsNetID on which the TwinCAT SMTP Server is running. |
| `sSmtpServer` | `T_MaxString` | Name or IP of the SMTP server to be reached |
| `sUsername` | `T_MaxString` | User name of the SMTP server to be reached |
| `sPassword` | `T_MaxString` | Password of the SMTP server to be reached |
| `nEncryption` | `UDINT` | SMTP encryption type: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output variable remains TRUE until the function block |

**Description:** sPassword T_MaxString nEncryption UDINT sFrom T_MaxString sTo T_MaxString sCc T_MaxString sBcc T_MaxString sSubject T_MaxString pMessage Pointer To STRING(80) cbMessage UDINT bExecute BOOL tTimeout TIME

#### TwinCAT SMTP › PLC API › Function blocks

##### `FB_SmtpV3_Full` — §4.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | AmsNetID on which the TwinCAT SMTP Server is running. |
| `sSmtpServer` | `T_MaxString` | Name or IP of the SMTP server to be reached |
| `sUsername` | `T_MaxString` | User name of the SMTP server to be reached |
| `sPassword` | `T_MaxString` | Password of the SMTP server to be reached |
| `nEncryption` | `UDINT` | SMTP encryption type: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output remains TRUE until the function block has |

**Description:** sPassword T_MaxString nEncryption UDINT sFrom T_MaxString sTo T_MaxString sCc T_MaxString sBcc T_MaxString sDispositionNotification T_MaxString sReturnReceipt T_MaxString nPriority UDINT nSensitivity UDINT nPort UDINT nContentType UDINT sSubject T_MaxString pMessage Pointer To STRING(80) cbMessage UDINT

#### TwinCAT SMTP › PLC API › Function blocks › [obsolete functions]

##### `FB_Smtp` — §4.2.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | AmsNetID, on which the TwinCAT mail server runs. |
| `sSmtpServer` | `T_MaxString` | Name or IP of the SMTP server |
| `sFrom` | `T_MaxString` | A string containing the email address of the sender. A sender |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | This output remains TRUE until the function block has |

**Description:** The function block FB_Smtp sends a byte stream to an ADS remote device via ADS. The TwinCAT ADS SMTP service must be running on the remote ADS device, so that the byte stream can be received and processed into an email. The email is then sent after bytesteam processing. Please note that the SMTP server does not support password verification, as the TwinCAT ADS SMTP service does not log in to the server via password verification.

##### `FB_SmtpFull` — §4.2.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | AmsNetID on which the TwinCAT SMTP Server is running. |
| `sSmtpServer` | `T_MaxString` | Name or IP of the SMTP server |
| `sUsername` | `T_MaxString` | User name of the SMTP server |
| `sPassword` | `T_MaxString` | Password for the SMTP server |
| `nAuth` | `UDINT` | Smtp Auth Type: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output remains TRUE until the function block has |

**Description:** The function block FB_SmtpFull communicates with the TwinCAT SMTP Server via ADS. It offers very extensive email functionalities, such as prioritizing emails from the PLC. The individual parameters are explained in detail in this documentation.

##### `FB_SmtpAttach` — §4.2.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetId` | `T_AmsNetID` | AmsNetID on which the TwinCAT SMS server is running. |
| `sSmtpServer` | `T_MaxString` | Name or IP of the SMTP server |
| `sUsername` | `T_MaxString` | User name of the SMTP server |
| `sPassword` | `T_MaxString` | Password for the SMTP server |
| `nAuth` | `UDINT` | Smtp Auth Type: |
| `sNetId` | `T_AmsNetID` | AmsNetID on which the TwinCAT SMS server is running. |
| `sSmtpServer` | `T_MaxString` | Name or IP of the SMTP server |
| `sUsername` | `T_MaxString` | User name of the SMTP server |
| `sPassword` | `T_MaxString` | Password for the SMTP server |
| `nAuth` | `UDINT` | Smtp Auth Type: |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The output variable remains TRUE until the function block |
| `bBusy` | `BOOL` | The output variable remains TRUE until the function block |

**Description:** The function block FB_SmtpAttach sends a data stream to a remote ADS device via ADS. The TwinCAT ADS SMTP service must be running on the remote ADS device so that the data stream can be received and processed into an email. As soon as the data stream has been processed, the email is sent.

#### TwinCAT SMS › PLC API › Function blocks

##### `SendSMS` — §5.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Send` | `BOOL` | The function block is enabled by a positive edge at this |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RXbuffer` | `ComBuffer` | — |
| `TXbuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | This output is set when there is a rising edge at the Send |

**Description:** Text STRING(160)

#### TwinCAT SMS › PLC API › Functions

##### `FUNCTION Get_TcPlcSMS_Version` (FUNCTION) → `STRING` — §5.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bGet` | `BOOL` | This parameter has no significance for the function |

**Description:** This function can be used to read the version information of the PLC library.
