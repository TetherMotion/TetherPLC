# Tc2_FTP+Tc3_FTP

- **Manual:** `TF6300_TC3_FTP_Client_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 41
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** FTP client needs socket service + filesystem credentials.

---

#### Technical introduction

##### `Quick start` (FUNCTION_BLOCK) — §4.1

**Description:** This documentation article is intended to allow you an initial, quick start in how to use this product. You can also download the program code below at Samples [} 66]. The program code for a quick start with older product versions can also be found there. Following successful installation and licensing [} 11], perform the following steps in order to establish a connection to an FTP/SFTP server and configure variables for the read/write access.

#### PLC libraries › Tc3_FTP › Function blocks

##### `FB_FtpConnection` — §5.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeout` | `TIME` | Indicates the ADS timeout to be used between PLC runtime and the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `ipResultMessage` | `I_TcMessage` | Output with extended error and diagnostic information based on the TwinCAT 3 |

**Description:** This function block can be used to manage a connection to an FTP/SFTP server. Different methods allow you to create a profile, connect to the server, and disconnect from the server.

#### PLC libraries › Tc3_FTP › Function blocks › FB_FtpConnection

##### `AddProfile` (METHOD) → `BOOL` — §5.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stFtpConfigProfile` | `ST_FtpConfigProfile [} 43]` | Structure with further information on the profile to be created. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddProfile` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Adds a new connection profile.

##### `Connect` (METHOD) → `BOOL` — §5.1.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sFtpConfigProfileFileNa STRING` | `Indicates the name of the connection profile. me` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Connect` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Starts connecting to a server by specifying the appropriate connection profile.

##### `Disconnect` (METHOD) → `BOOL` — §5.1.1.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Disconnect` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Disconnects from a server.

#### PLC libraries › Tc3_FTP › Function blocks

##### `FB_FtpExplorer` — §5.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeout` | `TIME` | Indicates the ADS timeout to be used between PLC runtime and the FTP |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `ipResultMessage` | `I_TcMessage` | Output with extended error and diagnostic information based on the TwinCAT 3 |

**Description:** This function block can be used to perform file and directory operations, e.g. creating new directories on the FTP server or reading existing directories. The corresponding operations are available as methods on the function block.

#### PLC libraries › Tc3_FTP › Function blocks › FB_FtpExplorer

##### `DirCreate` (METHOD) → `BOOL` — §5.1.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection [} 27]` | References the server connection to be used for the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DirCreate` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Creates a new directory on the server.

##### `DirExists` (METHOD) → `BOOL` — §5.1.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection [} 27]` | References the server connection to be used for the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DirExists` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Checks whether a specified directory already exists on the server.

##### `DirMove` (METHOD) → `BOOL` — §5.1.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection [} 27]` | References the server connection to be used |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DirMove` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Moves a directory on the server.

##### `DirRemove` (METHOD) → `BOOL` — §5.1.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection` | References the server connection to be used for the [} 27] operation. |
| `sDirectory` | `STRING` | Path to the directory. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DirRemove` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Removes a directory on the server.

##### `DirRename` (METHOD) → `BOOL` — §5.1.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection` | References the server connection to be used for the [} 27] operation. |
| `sDirectory` | `STRING` | Path to the directory to be renamed. |
| `sNewName` | `STRING` | New directory name. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DirRename` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Renames an existing directory on the server.

##### `FileExists` (METHOD) → `BOOL` — §5.1.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection [} 27] References the server connection to be used for the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FileExists` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Checks if a given file exists on the server.

##### `FileMove` (METHOD) → `BOOL` — §5.1.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection [} 27] References the server connection to be used for the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FileMove` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Moves a file on the server.

##### `FileRemove` (METHOD) → `BOOL` — §5.1.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection [} 27] References the server connection to be used for the` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FileRemove` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Removes a file from the server.

##### `FileRename` (METHOD) → `BOOL` — §5.1.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection` | References the server connection to be used for the [} 27] operation. |
| `sFilePath` | `STRING` | Path to the file on the server. |
| `sNewName` | `STRING` | New filename. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FileRename BOOL` | `Returns TRUE when the method has completed. Whether the action was` | — |

**Description:** Renames a file on the server.

#### PLC libraries › Tc3_FTP › Function blocks

##### `FB_FtpTransfer` — §5.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimeout` | `TIME` | Indicates the ADS timeout to be used between PLC runtime and the FTP |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | TRUE if an error has occurred. |
| `ipResultMessage` | `I_TcMessage` | Output with extended error and diagnostic information based on the TwinCAT 3 |

**Description:** This function block can be used to initiate and manage upload and download operations.

#### PLC libraries › Tc3_FTP › Function blocks › FB_FtpTransfer

##### `Abort` (METHOD) → `BOOL` — §5.1.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection` | References the server connection to be used for the [} 27] operation. |
| `nTransferHandle` | `UDINT` | TransferHandle of the transfer operation that is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Abort` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Aborts an existing transfer.

##### `Download` (METHOD) → `BOOL` — §5.1.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection` | References the server connection to be used for the [} 27] operation. |
| `sServerSourcePath` | `STRING` | Path to the source on the server. Can be a file path or a |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Download` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Starts a download operation.

##### `GetActive` (METHOD) → `BOOL` — §5.1.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection` | References the server connection to be used for the [} 27] operation. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetActive` | `BOOL` | Returns TRUE when the method has completed. Whether the action was successful can be read |

**Description:** PLC libraries

##### `GetActiveVerbose` (METHOD) → `BOOL` — §5.1.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection` | References the server connection to be used for the [} 27] operation. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetActiveVe BOOL` | `Returns TRUE if the currently active transfer` | — |
| `rbose` | `operations have been successfully returned.` | — |
| `stTransfers ST_FtpTransfersActive [} 44]` | `Contains the number of transfer operations` | — |

**Description:** Provides detailed information about transfer operations currently in progress.

##### `GetState` (METHOD) → `BOOL` — §5.1.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection [} 27]` | References the server connection to be used for the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetState` | `BOOL` | Returns TRUE when the method has completed. Whether the |

**Description:** Returns the current state of a specific transfer operation. This method can be used, for example, to check the progress of a file upload/download.

##### `GetStateVerbose` (METHOD) → `BOOL` — §5.1.1.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection [} 27]` | References the server connection to be used for the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetStateVer BOOL` | `Returns TRUE when the method has completed. Whether the` | — |
| `bose` | `action was successful can be read from the bError variable of` | — |

**Description:** Returns detailed information on a specific transfer operation. This method can be used, for example, to check the progress of a file upload/download.

##### `Upload` (METHOD) → `BOOL` — §5.1.1.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbFtpConnection` | `FB_FtpConnection` | References the server connection to be used for the [} 27] operation. |
| `sLocalSourcePath` | `STRING` | Path to the local source. Can be a file path or a directory |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Upload` | `BOOL` | Returns TRUE when the method has completed. Whether the action was |

**Description:** Starts an upload operation.

#### PLC libraries › Tc2_FTP › Function Blocks

##### `FB_FTP_HostResolve` — §5.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new |

**Description:** Host names of FTP servers can be resolved with the function block FB_FTP_HostResolve.

##### `FB_FTP_Open` — §5.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new |

**Description:** With the function block FB_FTP_Open a connection to an FTP server can be established. The handle returned can then be used to perform further actions on the FTP server. The connection type used is

##### `FB_FTP_OpenEX` — §5.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be accepted as long as |

**Description:** With the function block FB_FTP_OpenEx a connection to an FTP server can be established. The handle returned can then be used to perform further actions on the FTP server. Additionally, the connection type - Passive or Active FTP - can be selected.

##### `FB_FTP_Close` — §5.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the ADS |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be |

**Description:** An existing connection to an FTP server can be closed with the function block FB_FTP_Close.

##### `FB_FTP_CloseAll` — §5.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `PLC libraries` | `` | — |
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the ADS |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new |

**Description:** All existing connections to an FTP server can be closed with the function block FB_FTP_CloseAll.

##### `FB_FTP_Info` — §5.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be accepted as long as |

**Description:** Information about the created FTP server connections can be retrieved with the function block FB_FTP_Info.

##### `FB_FTP_FileUpload` — §5.2.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PLC libraries` | `` | — |
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new |

**Description:** Files can be transferred from the ADS-Device to an FTP server with the function block FB_FTP_FileUpload. This function block uses an already opened connection to an FTP server

##### `FB_FTP_FileUploadEx` — §5.2.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command |

**Description:** Files can be transferred from the ADS-Device to an FTP server with the function block FB_FTP_FileUploadEx. The connection type used is Passive FTP.

##### `FB_FTP_FileDownload` — §5.2.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PLC libraries` | `` | — |
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new |

**Description:** Files can be transferred from the FTP server to an ADS-Device with the function block FB_FTP_FileDownload. This function block uses an already open connection to an FTP server.

##### `FB_FTP_FileDownloadEx` — §5.2.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command |

**Description:** Files can be transferred from an FTP server to a ADS-Device with the function block FB_FTP_FileDownloadEx. The connection type used is Passive FTP.

##### `FB_FTP_DirCreate` — §5.2.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be |

**Description:** Directories can be created on an FTP server with the function block FB_FTP_DirCreate. This function block uses an already open connection to an FTP server.

##### `FB_FTP_DirRemove` — §5.2.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId := ''` | — |
| `hFTP` | `T_HFTP` | — |
| `sDirectory` | `T_MaxString:= ''` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME        := T#15s` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** Directories can be deleted from the FTP server with the function block FB_FTP_DirRemove. This function block uses an already open connection to an FTP server.

##### `FB_FTP_FileList` — §5.2.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PLC libraries` | `` | — |
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be accepted as long as |

**Description:** File and folder names can be read or searched from the FTP server with the function block FB_FTP_FileList. This function block uses an already opened connection to an FTP server

##### `FB_FTP_FileListEx` — §5.2.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be |

**Description:** File and folder names can be read or searched from the FTP server with the function block FB_FTP_FileListEx. This function block uses an already open connection to an FTP server. In contrast to the function block FB_FTP_FileList, details of the files/folders are also returned.

##### `FB_FTP_FileExist` — §5.2.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will |

**Description:** It can be checked whether a certain file exists on the FTP server with the function block FB_FTP_FileExist. This function block uses an already open connection to an FTP server.

##### `FB_FTP_FileRemove` — §5.2.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be |

**Description:** Files on an FTP server can be deleted with the function block FB_FTP_FileRemove. This function block uses an already open connection to an FTP server.

##### `FB_FTP_FileRename` — §5.2.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | A string containing the AMS network ID of the target device, at |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command will be |

**Description:** Files on an FTP server can be renamed with the function block FB_FTP_FileRename. This function block uses an already open connection to an FTP server.

##### `FB_GetStateTcFTPClient` — §5.2.1.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | A string containing the AMS network ID of the target device, at which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The command is in the process of being transmitted by ADS. No new command |

**Description:** The current state of the FTP client can be queried with the function block FB_GetStateTcFTPClient.
