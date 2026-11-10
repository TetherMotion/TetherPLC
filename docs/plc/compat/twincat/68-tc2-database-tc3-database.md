# Tc2_Database+Tc3_Database

- **Manual:** `TF6420_TC3_Database_Server_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 153
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** SQL access needs DB drivers (ODBC/SQLite-class) via a database service.

---

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › Configure mode

##### `FB_ConfigTcDBSrvEvt` (FUNCTION_BLOCK) — §6.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for creating, reading and deleting configuration entries for the TwinCAT Database Server.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › Configure mode › FB_ConfigTcDBSrvEvt

##### `Create` (METHOD) → `BOOL` — §6.1.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTcDBSrvConfig` | `POINTER TO BYTE` | Pointer of the configuration structure to be created. |
| `cbTcDBSrvConfig` | `UDINT` | Length of the configuration structure |
| `bTemporary` | `BOOL` | Indicates whether the configuration is to be stored in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Create` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method creates new entries in the XML configuration file for the TwinCAT Database Server. Optionally the TwinCAT Database Server can use a new entry on a temporary basis. In this case no data is written to the XML file.

##### `Read` (METHOD) → `BOOL` — §6.1.1.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDBConfig` | `POINTER TO ARRAY [1..MA Pointer address of the array into which the database` | — |
| `X_CONFIGURATIONS` | `configurations are to be written. [} 247]] OF ST_ConfigDB [} 229]` | — |
| `cbDBConfig` | `UDINT` | Length of the database configuration array |
| `pAutoLogGrpConfig` | `POINTER TO ARRAY[1..MAX Pointer address of the array into which the` | — |
| `_CONFIGURATIONS` | `AutoLogGrp configurations are to be written. [} 247]] OF ST_ConfigAutoLogGrp [} 228]` | — |
| `cbAutoLogGrpConfig UDINT` | `Length of the AutoLogGrp configuration array` | — |
| `pDBCount` | `POINTER TO UDINT` | Pointer address for storing the number of database |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to read the current configurations of the TwinCAT Database Server. Any temporary configurations that may be included are marked accordingly.

##### `Delete` (METHOD) → `BOOL` — §6.1.1.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eTcDBSrvConfigType` | `E_TcDBSrvConfigType` | Type of the configuration to be deleted (database / AutoLog group) |
| `hConfigID` | `UDINT` | ID of the configuration to be deleted (hDBID or hAutoLogGrpID) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Delete` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to delete databases and AutoLog groups from the configuration of the

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › Configure mode

##### `FB_PLCDBAutoLogEvt` (FUNCTION_BLOCK) — §6.1.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is directed. |
| `tTimeout` | `TIME` | Indicates the time before the function is cancelled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function |

**Description:** Function block with four methods for starting and stopping of defined AutoLog groups and for reading of the corresponding group status.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › Configure mode › FB_PLCDBAutoLogEvt

##### `RunOnce` (METHOD) → `BOOL` — §6.1.1.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hAutoLogGrpID` | `UDINT` | ID of the AutoLog group to be executed once. |
| `bAll` | `BOOL` | If TRUE, all AutoLog groups are executed once. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RunOnce` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method can be used to execute an AutoLog group once, for example based on an event in the controller.

##### `Start` (METHOD) → `BOOL` — §6.1.1.1.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Start` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method starts the AutoLog mode with the corresponding configured AutoLog groups.

##### `Status` (METHOD) → `BOOL` — §6.1.1.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tCheckCycle` | `TIME` | Interval time at which the status array is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Status` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method can be used to query the status of the AutoLog groups. A separate busy flag is provided in the body of the function block for this method, since it can be called independently of the other methods of the function block: bBusy_Status.

##### `Stop` (METHOD) → `BOOL` — §6.1.1.1.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Stop` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method stops the AutoLog mode.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode

##### `FB_ConfigTcDBSrvEvt` (FUNCTION_BLOCK) — §6.1.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for creating, reading and deleting configuration entries for the TwinCAT Database Server.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode › FB_ConfigTcDBSrvEvt

##### `Create` (METHOD) → `BOOL` — §6.1.1.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTcDBSrvConfig` | `POINTER TO BYTE` | Pointer of the configuration structure to be created. |
| `cbTcDBSrvConfig` | `UDINT` | Length of the configuration structure |
| `bTemporary` | `BOOL` | Indicates whether the configuration is to be stored in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Create` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method creates new entries in the XML configuration file for the TwinCAT Database Server. Optionally the TwinCAT Database Server can use a new entry on a temporary basis. In this case no data is written to the XML file.

##### `Read` (METHOD) → `BOOL` — §6.1.1.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDBConfig` | `POINTER TO ARRAY [1..MA Pointer address of the array into which the database` | — |
| `X_CONFIGURATIONS` | `configurations are to be written. [} 247]] OF ST_ConfigDB [} 229]` | — |
| `cbDBConfig` | `UDINT` | Length of the database configuration array |
| `pAutoLogGrpConfig` | `POINTER TO ARRAY[1..MAX Pointer address of the array into which the` | — |
| `_CONFIGURATIONS` | `AutoLogGrp configurations are to be written. [} 247]] OF ST_ConfigAutoLogGrp [} 228]` | — |
| `cbAutoLogGrpConfig UDINT` | `Length of the AutoLogGrp configuration array` | — |
| `pDBCount` | `POINTER TO UDINT` | Pointer address for storing the number of database |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to read the current configurations of the TwinCAT Database Server. Any temporary configurations that may be included are marked accordingly.

##### `Delete` (METHOD) → `BOOL` — §6.1.1.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eTcDBSrvConfigType` | `E_TcDBSrvConfigType` | Type of the configuration to be deleted (database / AutoLog group) |
| `hConfigID` | `UDINT` | ID of the configuration to be deleted (hDBID or hAutoLogGrpID) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Delete` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to delete databases and AutoLog groups from the configuration of the

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode

##### `FB_PLCDBAutoLogEvt` (FUNCTION_BLOCK) — §6.1.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is directed. |
| `tTimeout` | `TIME` | Indicates the time before the function is cancelled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function |

**Description:** Function block with four methods for starting and stopping of defined AutoLog groups and for reading of the corresponding group status.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode › FB_PLCDBAutoLogEvt

##### `RunOnce` (METHOD) → `BOOL` — §6.1.1.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hAutoLogGrpID` | `UDINT` | ID of the AutoLog group to be executed once. |
| `bAll` | `BOOL` | If TRUE, all AutoLog groups are executed once. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RunOnce` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method can be used to execute an AutoLog group once, for example based on an event in the controller.

##### `Start` (METHOD) → `BOOL` — §6.1.1.2.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Start` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method starts the AutoLog mode with the corresponding configured AutoLog groups.

##### `Status` (METHOD) → `BOOL` — §6.1.1.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tCheckCycle` | `TIME` | Interval time at which the status array is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Status` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method can be used to query the status of the AutoLog groups. A separate busy flag is provided in the body of the function block for this method, since it can be called independently of the other methods of the function block: bBusy_Status.

##### `Stop` (METHOD) → `BOOL` — §6.1.1.2.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Stop` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method stops the AutoLog mode.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode

##### `FB_PLCDBCreateEvt` (FUNCTION_BLOCK) — §6.1.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block with two methods. One method can be used to create databases from the PLC on a database server specified in the PLC. The other method can be used to generate a new table in a specified database.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode › FB_PLCDBCreateEvt

##### `Database` (METHOD) → `BOOL` — §6.1.1.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDatabaseConfig` | `POINTER TO BYTE` | Address of the database configuration structure [} 230] |
| `cbDatabaseConfig` | `UDINT` | Length of the database configuration structure |
| `bCreateXMLConfig` | `BOOL` | Indicates whether the newly created database should be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Database` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method creates a new database. Optionally you can specify whether the created database should also be used for the configuration of the TwinCAT Database Server.

##### `Table` (METHOD) → `BOOL` — §6.1.1.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `MaxString` | Name of the table to be created. |
| `pTableCfg` | `POINTER TO` | Indicates the pointer address of the table structure |
| `ARRAY[0..MAX_DBCOLUMNS` | `array. The individual columns are written in this array. [} 247]] OF ST_ColumnInfo [} 244]` | — |
| `cbTableCfg` | `UDINT` | Indicates the length of the array in which the columns |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Table` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method creates a new table with a structure that is defined through an array with x elements or x columns in the PLC.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode

##### `FB_PLCDBReadEvt` (FUNCTION_BLOCK) — §6.1.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for reading records from a database.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode › FB_PLCDBReadEvt

##### `Read` (METHOD) → `BOOL` — §6.1.1.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `T_MaxString` | Name of the table that is to be read. |
| `sDBSymbolName` | `T_MaxString` | Symbol name to be read from the standard table |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method execution is |

**Description:** This method reads a specified number of records from a database table with the standard table structure specified by Beckhoff. The standard table structure is used in AutoLog mode and in the FB_DBWriteEvt function block, for example.

##### `ReadStruct` (METHOD) → `BOOL` — §6.1.1.2.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `T_MaxString` | Name of the table that is to be read. |
| `pColumnNames` | `POINTER TO ARRAY [0..MAX_D Address of the array containing the column name BCOLUMNS] OF STRING(50)` | to be read. |
| `cbColumnNames` | `UDINT` | Length of the column name array |
| `sOrderByColumn` | `STRING(50)` | Name the sorting column |
| `eOrderType` | `E_OrderType` | Sorting direction (ASC or DESC) |
| `nStartIndex` | `UDINT` | Indicates the index of the first record to be read. |
| `nRecordCount` | `UDINT` | Indicates the number of records to be read. |
| `pData` | `POINTER TO BYTE` | Address of the structure array into which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReadStruct BOOL` | `Displays the status of the method. Returns TRUE as soon as the method execution is` | — |

**Description:** This method reads a specified number of records from a database table with any table structure.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode

##### `FB_PLCDBWriteEvt` (FUNCTION_BLOCK) — §6.1.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for writing of records into a database.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode › FB_PLCDBWriteEvt

##### `Write` (METHOD) → `BOOL` — §6.1.1.2.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `T_MaxString` | Name of the table that is to be read. |
| `pValue` | `POINTER TO BYTE` | Address of the variable to be logged in the standard table |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Write` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method creates a record in the standard table structure specified by Beckhoff.

##### `WriteBySymbol` (METHOD) → `BOOL` — §6.1.1.2.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `T_MaxString` | Name of the table that is to be read. |
| `stADSDevice` | `ST_ADSDevice` | ADS device from which a symbol is to be logged in the standard |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WriteBySymbol` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method reads the value of a specified ADS symbol and saves it in the standard table structure specified by Beckhoff. ADS symbols from other ADS devices can also be read.

##### `WriteStruct` (METHOD) → `BOOL` — §6.1.1.2.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `T_MaxString` | Name of the table that is to be read. |
| `pRecord` | `POINTER TO BYTE` | Address of a structure that is to be logged in a freely |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WriteStruct` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method creates a record with a freely selectable table structure.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode

##### `FB_PLCDBCmdEvt` (FUNCTION_BLOCK) — §6.1.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block with two methods. Users can define and transfer their own SQL commands. Placeholders in the SQL command can correlate with structures in the PLC, which reflect the table structure. The database server enters the current data of the structure into the SQL command.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › PLC Expert mode › FB_PLCDBCmdEvt

##### `Execute` (METHOD) → `BOOL` — §6.1.1.2.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | — |
| `pExpression` | `POINTER TO BYTE` | — |
| `cbExpression` | `UDINT` | — |
| `pData` | `POINTER TO BYTE` | — |
| `cbData` | `UDINT` | — |
| `pParameter` | `POINTER TO ARRAY[0..MAX_DBCOLUMNS] OF ST_ExpParameter` | — |
| `cbParameter` | `UDINT` | — |

**Description:** This method can be used to send SQL commands to the database. The database connection is opened with each call and then closed again. It is possible to define placeholders in the command, which are replaced by the TwinCAT Database Server with the corresponding values before the execution. Returned records cannot be read.

##### `ExecuteDataReturn` (METHOD) → `BOOL` — §6.1.1.2.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `pExpression` | `POINTER TO BYTE` | Address of the string variable with the SQL command |
| `cbExpression` | `UDINT` | Length of the string variable with the SQL command |
| `pData` | `POINTER TO BYTE` | Address of the structure with the parameter values |
| `cbData` | `UDINT` | Length of the structure with the parameter values |
| `pParameter` | `POINTER TO ARRAY[0..MAX_ Address of the structure array with the parameter DBCOLUMNS] OF ST_ExpPar information ameter` | — |
| `cbParameter` | `UDINT` | Length of the structure array with the parameter information |
| `nStartIndex` | `UDINT` | Indicates the index of the first record to be read. |
| `nRecordCount` | `UDINT` | Indicates the number of records to be read. |
| `pReturnData` | `POINTER TO BYTE` | Address of the structure array into which the records are |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ExecuteDataReturn` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method can be used to send SQL commands to the database. The database connection is opened with each call and then closed again. It is possible to define placeholders in the command, which are replaced by the TwinCAT Database Server with the corresponding values before the execution. A specified number of records can be read.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › SQL Expert mode

##### `FB_ConfigTcDBSrvEvt` (FUNCTION_BLOCK) — §6.1.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for creating, reading and deleting configuration entries for the TwinCAT Database Server.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › SQL Expert mode › FB_ConfigTcDBSrvEvt

##### `Create` (METHOD) → `BOOL` — §6.1.1.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTcDBSrvConfig` | `POINTER TO BYTE` | Pointer of the configuration structure to be created. |
| `cbTcDBSrvConfig` | `UDINT` | Length of the configuration structure |
| `bTemporary` | `BOOL` | Indicates whether the configuration is to be stored in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Create` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method creates new entries in the XML configuration file for the TwinCAT Database Server. Optionally the TwinCAT Database Server can use a new entry on a temporary basis. In this case no data is written to the XML file.

##### `Read` (METHOD) → `BOOL` — §6.1.1.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDBConfig` | `POINTER TO ARRAY [1..MA Pointer address of the array into which the database` | — |
| `X_CONFIGURATIONS` | `configurations are to be written. [} 247]] OF ST_ConfigDB [} 229]` | — |
| `cbDBConfig` | `UDINT` | Length of the database configuration array |
| `pAutoLogGrpConfig` | `POINTER TO ARRAY[1..MAX Pointer address of the array into which the` | — |
| `_CONFIGURATIONS` | `AutoLogGrp configurations are to be written. [} 247]] OF ST_ConfigAutoLogGrp [} 228]` | — |
| `cbAutoLogGrpConfig UDINT` | `Length of the AutoLogGrp configuration array` | — |
| `pDBCount` | `POINTER TO UDINT` | Pointer address for storing the number of database |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to read the current configurations of the TwinCAT Database Server. Any temporary configurations that may be included are marked accordingly.

##### `Delete` (METHOD) → `BOOL` — §6.1.1.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eTcDBSrvConfigType` | `E_TcDBSrvConfigType` | Type of the configuration to be deleted (database / AutoLog group) |
| `hConfigID` | `UDINT` | ID of the configuration to be deleted (hDBID or hAutoLogGrpID) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Delete` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to delete databases and AutoLog groups from the configuration of the

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › SQL Expert mode

##### `FB_SQLDatabaseEvt` (FUNCTION) — §6.1.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for opening, closing and managing a database connection.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › SQL Expert mode › FB_SQLDatabaseEvt

##### `Connect` (METHOD) → `BOOL` — §6.1.1.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT PLC API` | Indicates the ID of the database to be used. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Connect` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method opens a connection to a declared database.

##### `CreateCmd` (METHOD) → `BOOL` — §6.1.1.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSQLCommand` | `POINTER TO FB_SQLCommand` | Returns a new instance of the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateCmd` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method is used to initialize an instance of the function block FB_SQLCommand with the already open database connection of the function block FB_SQLDatabase. The function block FB_SQLCommand only uses the database connection it was assigned via the CreateCmd method. Several instances of the function block FB_SQLCommand can be initialized with the same database connection.

##### `CreateSP` (METHOD) → `BOOL` — §6.1.1.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sProcedureName T_MaxString` | `Indicates the name of the procedure to be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateSP` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method is used to initialize an instance of the function block FB_SQLStoredProcedureEvt with the already open database connection of the function block FB_SQLDatabaseEvt. The function block FB_SQLStoredProcedureEvt only uses the database connection it was assigned via the CreateCmd method. Several instances of the function block FB_SQLStoredProcedureEvt can be initialized with the same database connection.

##### `Disconnect` (METHOD) → `BOOL` — §6.1.1.3.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Disconnect` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method closes the connection to the database that was opened by this function block instance.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › SQL Expert mode

##### `FB_SQLCommandEvt` (FUNCTION) — §6.1.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for executing SQL commands. Before it can be used it has to be initialized with the function block FB_SQLDatabaseEvt.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › SQL Expert mode › FB_SQLCommandEvt

##### `Execute` (METHOD) → `BOOL` — §6.1.1.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSQLCmd` | `POINTER TO BYTE Indicates the pointer address of a string variable with the SQL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `POINTER TO BYTE Displays the status of the method. Returns TRUE as soon as the` | — |

**Description:** This method sends the specified SQL command to the database via the database connection already opened by the function block FB_SQLDatabase.

##### `ExecuteDataReturn` (METHOD) → `BOOL` — §6.1.1.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSQLCmd` | `POINTER TO BYTE` | Indicates the pointer address of a string variable with |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ExecuteDataReturn` | `POINTER TO BYTE` | Displays the status of the method. Returns TRUE as soon |

**Description:** This method sends the specified SQL command to the database via the database connection already opened by the function block FB_SQLDatabase. An instance of the function block FB_SQLResult can be transferred for reading the returned records.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › SQL Expert mode

##### `FB_SQLResultEvt` (FUNCTION) — §6.1.1.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** The function block is used for reading the cached records.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › SQL Expert mode › FB_SQLResultEvt

##### `Read` (METHOD) → `BOOL` — §6.1.1.3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nStartIndex` | `UDINT` | Indicates the index of the first record to be read. |
| `nRecordCount` | `UDINT` | Indicates the number of records to be read. |
| `pData` | `POINTER TO BYTE` | Address of the structure array into which the records are to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method reads a specified number of records from the result data cached in the

##### `Release` (METHOD) → `BOOL` — §6.1.1.3.4.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Release` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to release data cached by the TwinCAT Database Server.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › SQL Expert mode

##### `FB_SQLStoredProcedureEvt` (FUNCTION) — §6.1.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for executing stored procedures of the database. Before it can be used it has to be initialized with the function block FB_SQLDatabaseEvt.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › SQL Expert mode › FB_SQLStoredProcedureEvt

##### `Execute` (METHOD) → `BOOL` — §6.1.1.3.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pParameterStrc` | `POINTER TO BYTE Pointer address to the parameter structure that is transferred to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method sends the call of the specified stored procedure to the database via the database connection already opened by the function block FB_SQLDatabaseEvt.

##### `ExecuteDataReturn` (METHOD) → `BOOL` — §6.1.1.3.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pParameterStrc` | `POINTER TO BYTE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method sends the call of the specified stored procedure to the database via the database connection already opened by the function block FB_SQLDatabase. An instance of the FB_SQLResult function block can be transferred for reading the returned records.

##### `Release` (METHOD) → `BOOL` — §6.1.1.3.5.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Release` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method releases the parameter information of the stored procedure, which was transferred during initialization.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › NoSQL Expert Mode › Query Builder

##### `FB_NoSQLQueryBuilder_DocumentDB` (FUNCTION) — §6.1.1.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eQueryType` | `E_DocumentDb Type of query sent to the database. QueryType [} 234]` | — |
| `sCollectionNa T_` | `Name of the collection that is the target of the query.` | — |
| `me` | `MAXSTRING` | — |
| `pQueryOption POINTER TO` | `Specifies the address for the query options [} 235].` | — |
| `s` | `BYTE cbQueryOptio UDINT` | Length of the query options. nsr Methods tion |
| `Build [} 207]` | `Local` | [optional] This method generates a query for the function block |

**Description:** Function block for defining a query for the database. The query is sent with FB_NoSQLQueryEvt [} 209]. It is not necessary to call the Build method.

##### `FB_NoSQLQueryBuilder_TimeSeriesDB` (FUNCTION) — §6.1.1.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pQueryOption POINTER TO` | `Specifies the address for the query options [} 237].` | — |
| `s` | `BYTE` | — |
| `cbQueryOptio UDINT` | `Length of the query options. ns Syntax` | — |

**Description:** Function block for defining a query for a TimeSeries database. The query is sent with FB_NoSQLQueryEvt [} 209]. It is not necessary to call the Build method. Data structures [} 358] can be described with attributes to affect individual settings.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › NoSQL Expert Mode

##### `FB_NoSQLQueryEvt` (FUNCTION) — §6.1.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for executing NoSQL database queries. A QueryBuilder function block that describes the query is used as the input parameter for the methods.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › NoSQL Expert Mode › FB_NoSQLQueryEvt

##### `Execute` (METHOD) → `BOOL` — §6.1.1.4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | ID of the set database configuration |
| `iNoSSQLQuery` | `I_NoSQLQueryBuilde Pre-parameterized QueryBuilder function block. This varies` | — |
| `Builder` | `r` | depending on the database. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method sends a query to the NoSQL database, which was previously set with the I_NoSQLQueryBuilder [} 206] function block.

##### `ExecuteDataReturn` (METHOD) → `BOOL` — §6.1.1.4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | ID of the set database configuration |
| `iNoSQLQueryBuil I_NoSQLQueryBuilder` | `Preconfigured QueryBuilder function block that defines` | — |
| `der` | `the query to be sent.` | — |
| `pNoSQLResult` | `POINTER TO` | Specifies the address for the FB_NoSQLResultEvt |
| `FB_NoSSQLResultEvt` | `function block, which can be used to read the results.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ExecuteDataReturn` | `BOOL` | Displays the status of the method. Returns TRUE as soon |

**Description:** This method executes a query to a NoSQL database that was previously set using the I_NoSQLQueryBuilder function block. The transferred instance of type FB_NoSQLResultEvt is filled with return values.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › NoSQL Expert Mode

##### `FB_NoSQLResultEvt` (FUNCTION) — §6.1.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for reading buffered records.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › NoSQL Expert Mode › FB_NoSQLResultEvt

##### `ReadAsString` (METHOD) → `BOOL` — §6.1.1.4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nStartIndex` | `UDINT` | Indicates the index of the first record to be read. |
| `nRecordCount` | `UDINT` | Indicates the number of records to be read. |
| `pData` | `POINTER TO BYTE` | Indicates the address of the string array into which the records |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReadAsString` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method reads a specified number of records from the result data cached in the TwinCAT Database Server. An array of strings is specified into which this data is to be copied as JSON.

##### `ReadAsStruct` (METHOD) → `BOOL` — §6.1.1.4.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nStartIndex` | `UDINT` | Indicates the index of the first record to be read. |
| `nRecordCount` | `UDINT` | Indicates the number of records to be read. |
| `pData` | `POINTER TO BYTEAddress of the structure array into which the records are to be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReadAsStruct` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method reads a specified number of records from the buffered result data. A structure or an array of a structure is specified in which the data is to be written. The data type schema of this structure should correspond as closely as possible to that of the read data. The variable names are compared with those of the record. A validation makes it possible to detect deviations and respond to them.

##### `Release` (METHOD) → `BOOL` — §6.1.1.4.3.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Release` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to release data cached by the TwinCAT Database Server.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › NoSQL Expert Mode

##### `FB_NoSQLValidationEvt` (FUNCTION) — §6.1.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for reading the validation events and results that occurred when reading the data with FB_NoSQLResultEvt [} 212]. This function block is initialized via the CreateValidation method of the NoSQLResult. It refers to the last call of the ReadAsStruct [} 214] method.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › NoSQL Expert Mode › FB_NoSQLValidationEvt

##### `GetIssues` (METHOD) → `BOOL` — §6.1.1.4.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pData` | `POINTER TO BYTE` | Address of the array of type T_MAXSTRING in which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReadAsString` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** Reads the validation events that occurred into an array of type T_MAXSTRING. These contain information about remaining or unassigned records or which elements of the PLC structure were not filled.

##### `GetRemainingData` (METHOD) → `BOOL` — §6.1.1.4.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pData` | `POINTER TO BYTE` | Indicates the address of the string array into which the records |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetRemainingDat BOOL` | `Displays the status of the method. Returns TRUE as soon as the` | — |
| `a` | `method execution is finished, even in the event of an error.` | — |

**Description:** This method can be used to read the remaining data in JSON format after the validation. This includes records which could not be assigned to the PLC structure, for example.

##### `Release` (METHOD) → `BOOL` — §6.1.1.4.4.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Release` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** Releases the validation results in the memory.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › NoSQL Expert Mode › Helper

##### `FB_NoSQLObjectId_MongoDB` (FUNCTION_BLOCK) — §6.1.1.4.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ObjectId` | `T_ObjectId_Mongo 12-byte data type for describing the ObjectId. DB Properties` | — |
| `eTraceLevel` | `TcEventSeverity` | Specifies the weighting of the events. Only events with a weighting |

**Description:** The function block for parsing the ObjectId from the MongoDB. In the PLC it is described by the data type T_ObjectId_MongoDB [} 234].

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › Support of the Tc3_Eventlogger › I_TcEventBase

##### `EqualsTo` (METHOD) → `BOOL` — §6.1.1.5.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipOther` | `I_TcEventBase` | Event to be compared |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsTo` | `BOOL` | Returns TRUE if the events match. |

**Description:** This method carries out a comparison with another event specified at the input.

##### `EqualsToEventClass` (METHOD) → `BOOL` — §6.1.1.5.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `OtherEventClass` | `GUID` | Event class to be compared. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsToEventClass` | `BOOL` | Returns TRUE if the event classes match. |

**Description:** This method carries out a comparison with another event class specified at the input.

##### `EqualsToEventEntry` (METHOD) → `BOOL` — §6.1.1.5.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `OtherEventClass` | `GUID` | Event class of the event to be compared. |
| `nOtherEventID` | `UDINT` | Event ID of the event to be compared. |
| `eOtherSeverity` | `TcEventSeverity PLC API` | Event severity of the event to be compared. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsToEventEntry` | `BOOL` | Returns TRUE if the events match. |

**Description:** This method carries out a comparison with another event specified at the input.

##### `EqualsToEventEntryEx` (METHOD) → `BOOL` — §6.1.1.5.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stOther` | `TcEventEntry` | Event to be compared. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EqualsToEventEntryEx` | `BOOL` | Returns TRUE if the events match. |

**Description:** This method carries out a comparison with another event specified at the input.

##### `GetJsonAttribute` (METHOD) → `HRESULT` — §6.1.1.5.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sJsonAttribute` | `REFERENCE TO STRING` | Reference to a variable of the type String |
| `nJsonAttribute` | `UDINT` | Length of the String variable |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetJsonAttribute` | `HRESULT` | Returns S_OK if the method call was successful. |

**Description:** This method returns the Json attribute.

##### `RequestEventClassName` (METHOD) → `BOOL` — §6.1.1.5.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLangId` | `DINT …` | Specifies the language ID English (en-US) = 1033 German (de-DE) = 1031 |
| `sResult` | `REFERENCE TO STRING` | Reference to a variable of the type String |
| `nResultSize` | `UDINT` | Size of the String variable in bytes |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestEventClassName` | `BOOL` | Returns TRUE as soon as the request has been terminated. |
| `bError` | `BOOL` | Returns FALSE if the method call was successful. Returns TRUE if |

**Description:** This method returns the name of the event class.

##### `RequestEventText` (METHOD) → `BOOL` — §6.1.1.5.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLangId` | `DINT …` | Specifies the language ID English (en-US) = 1033 German (de-DE) = 1031 |
| `sResult` | `REFERENCE TO STRING` | Reference to a variable of the type String |
| `nResultSize` | `UDINT` | Size of the String variable in bytes |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RequestEventText` | `BOOL` | Returns TRUE as soon as the request has been terminated. |
| `bError` | `BOOL` | Returns FALSE if the method call was successful. Returns TRUE if |

**Description:** This method returns the event text.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Function blocks › Support of the Tc3_Eventlogger › I_TcMessage

##### `Send` (METHOD) → `HRESULT` — §6.1.1.5.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimeStamp` | `ULINT` | 0: Current time stamp is used |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Send` | `FB_ HRESULT` | Returns S_OK if the method call was successful, otherwise HRESULT as error code |

**Description:** This method sends the message.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › Configure mode

##### `FB_ConfigTcDBSrv` (FUNCTION_BLOCK) — §6.1.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is directed. |
| `tTimeout` | `TIME` | Indicates the time before the function is cancelled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function |

**Description:** Function block for creating, reading and deleting configuration entries for the TwinCAT Database Server.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › Configure mode › FB_ConfigTcDBSrv

##### `Create` (METHOD) → `BOOL` — §6.1.4.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTcDBSrvConfig` | `POINTER TO BYTE` | Pointer of the configuration structure to be created. |
| `cbTcDBSrvConfig` | `UDINT` | Length of the configuration structure |
| `bTemporary` | `BOOL` | Indicates whether the configuration is to be stored in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Create` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method creates new entries in the XML configuration file for the TwinCAT Database Server. Optionally the TwinCAT Database Server can use a new entry on a temporary basis. In this case no data is written to the XML file.

##### `Read` (METHOD) → `BOOL` — §6.1.4.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDBConfig` | `POINTER TO ARRAY [1..MA Pointer address of the array into which the database` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to read the current configurations of the TwinCAT Database Server. Any temporary configurations that may be included are marked accordingly.

##### `Delete` (METHOD) → `BOOL` — §6.1.4.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eTcDBSrvConfigType` | `E_TcDBSrvConfigType` | Type of the configuration to be deleted (database / AutoLog group) |
| `hConfigID` | `UDINT` | ID of the configuration to be deleted (hDBID or hAutoLogGrpID) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Delete` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to delete databases and AutoLog groups from the configuration of the

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › Configure mode

##### `FB_PLCDBAutoLog` (FUNCTION_BLOCK) — §6.1.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is directed. |
| `tTimeout` | `TIME` | Indicates the time before the function is cancelled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function |

**Description:** Function block with four methods for starting and stopping of defined AutoLog groups and for reading of the corresponding group status.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › Configure mode › FB_PLCDBAutoLog

##### `RunOnce` (METHOD) → `BOOL` — §6.1.4.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hAutoLogGrpID` | `UDINT` | ID of the AutoLog group to be executed once. |
| `bAll` | `BOOL` | If TRUE, all AutoLog groups are executed once. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RunOnce` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method can be used to execute an AutoLog group once, for example based on an event in the controller.

##### `Start` (METHOD) → `BOOL` — §6.1.4.1.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Start` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method starts the AutoLog mode with the corresponding configured AutoLog groups.

##### `Status` (METHOD) → `BOOL` — §6.1.4.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tCheckCycle` | `TIME` | Interval time at which the status array is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Status` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method can be used to query the status of the AutoLog groups. A separate busy flag is provided in the body of the function block for this method, since it can be called independently of the other methods of the function block: bBusy_Status.

##### `Stop` (METHOD) → `BOOL` — §6.1.4.1.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Stop` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method stops the AutoLog mode.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode

##### `FB_ConfigTcDBSrv` (FUNCTION_BLOCK) — §6.1.4.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is directed. |
| `tTimeout` | `TIME` | Indicates the time before the function is cancelled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function |

**Description:** Function block for creating, reading and deleting configuration entries for the TwinCAT Database Server.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode › FB_ConfigTcDBSrv

##### `Create` (METHOD) → `BOOL` — §6.1.4.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTcDBSrvConfig` | `POINTER TO BYTE` | Pointer of the configuration structure to be created. |
| `cbTcDBSrvConfig` | `UDINT` | Length of the configuration structure |
| `bTemporary` | `BOOL` | Indicates whether the configuration is to be stored in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Create` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method creates new entries in the XML configuration file for the TwinCAT Database Server. Optionally the TwinCAT Database Server can use a new entry on a temporary basis. In this case no data is written to the XML file.

##### `Read` (METHOD) → `BOOL` — §6.1.4.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDBConfig` | `POINTER TO ARRAY [1..MA Pointer address of the array into which the database` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to read the current configurations of the TwinCAT Database Server. Any temporary configurations that may be included are marked accordingly.

##### `Delete` (METHOD) → `BOOL` — §6.1.4.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eTcDBSrvConfigType` | `E_TcDBSrvConfigType` | Type of the configuration to be deleted (database / AutoLog group) |
| `hConfigID` | `UDINT` | ID of the configuration to be deleted (hDBID or hAutoLogGrpID) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Delete` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to delete databases and AutoLog groups from the configuration of the

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode

##### `FB_PLCDBAutoLog` (FUNCTION_BLOCK) — §6.1.4.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is directed. |
| `tTimeout` | `TIME` | Indicates the time before the function is cancelled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function |

**Description:** Function block with four methods for starting and stopping of defined AutoLog groups and for reading of the corresponding group status.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode › FB_PLCDBAutoLog

##### `RunOnce` (METHOD) → `BOOL` — §6.1.4.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hAutoLogGrpID` | `UDINT` | ID of the AutoLog group to be executed once. |
| `bAll` | `BOOL PLC API` | If TRUE, all AutoLog groups are executed once. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RunOnce` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method can be used to execute an AutoLog group once, for example based on an event in the controller.

##### `Start` (METHOD) → `BOOL` — §6.1.4.2.2.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Start` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method starts the AutoLog mode with the corresponding configured AutoLog groups.

##### `Status` (METHOD) → `BOOL` — §6.1.4.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tCheckCycle` | `TIME` | Interval time at which the status array is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Status` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method can be used to query the status of the AutoLog groups. A separate busy flag is provided in the body of the function block for this method, since it can be called independently of the other methods of the function block: bBusy_Status.

##### `Stop` (METHOD) → `BOOL` — §6.1.4.2.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Stop` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method stops the AutoLog mode.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode

##### `FB_PLCDBCreate` (FUNCTION_BLOCK) — §6.1.4.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block is |

**Description:** Function block with two methods. One method can be used to create databases from the PLC on a database server specified in the PLC. The other method can be used to generate a new table in a specified database.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode › FB_PLCDBCreate

##### `Database` (METHOD) → `BOOL` — §6.1.4.2.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDatabaseConfig` | `POINTER TO BYTE` | Address of the database configuration structure [} 230] |
| `cbDatabaseConfig` | `UDINT` | Length of the database configuration structure |
| `bCreateXMLConfig` | `BOOL` | Indicates whether the newly created database should be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Database` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method creates a new database. Optionally you can specify whether the created database should also be used for the configuration of the TwinCAT Database Server.

##### `Table` (METHOD) → `BOOL` — §6.1.4.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `MaxString` | Name of the table to be created. |
| `pTableCfg` | `POINTER TO` | Indicates the pointer address of the table structure |
| `ARRAY[0..MAX_DBCOLUMNS]` | `array. The individual columns are written in this array. OF ST_ColumnInfo [} 244]` | — |
| `cbTableCfg` | `UDINT` | Indicates the length of the array in which the columns |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Table` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method creates a new table with a structure that is defined through an array with x elements or x columns in the PLC.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode

##### `FB_PLCDBRead` (FUNCTION_BLOCK) — §6.1.4.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for reading records from a database.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode › FB_PLCDBRead

##### `Read` (METHOD) → `BOOL` — §6.1.4.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `T_MaxString` | Name of the table that is to be read. |
| `sDBSymbolName` | `T_MaxString` | Symbol name to be read from the standard table |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method execution is |

**Description:** This method reads a specified number of records from a database table with the standard table structure specified by Beckhoff. (The standard table structure is used in AutoLog mode and in the FB_DBWrite function block, for example).

##### `ReadStruct` (METHOD) → `BOOL` — §6.1.4.2.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `T_MaxString` | Name of the table that is to be read. |
| `pColumnNames` | `POINTER TO ARRAY [0..MAX_D Address of the array containing the column name BCOLUMNS] OF STRING(50)` | to be read. |
| `cbColumnNames` | `UDINT` | Length of the column name array |
| `sOrderByColumn` | `STRING(50)` | Name the sorting column |
| `eOrderType` | `E_OrderType` | Sorting direction (ASC or DESC) |
| `nStartIndex` | `UDINT` | Indicates the index of the first record to be read. |
| `nRecordCount` | `UDINT` | Indicates the number of records to be read. |
| `pData` | `POINTER TO BYTE` | Address of the structure array into which the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReadStruct BOOL` | `Displays the status of the method. Returns TRUE as soon as the method execution is` | — |

**Description:** This method reads a specified number of records from a database table with any table structure.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode

##### `FB_PLCDBWrite` (FUNCTION_BLOCK) — §6.1.4.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block |

**Description:** Function block for writing of records into a database.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode › FB_PLCDBWrite

##### `Write` (METHOD) → `BOOL` — §6.1.4.2.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `T_MaxString` | Name of the table that is to be read. |
| `pValue` | `POINTER TO BYTE` | Address of the variable to be logged in the standard table |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Write` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method creates a record in the standard table structure specified by Beckhoff.

##### `WriteBySymbol` (METHOD) → `BOOL` — §6.1.4.2.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `T_MaxString` | Name of the table that is to be read. |
| `stADSDevice` | `ST_ADSDevice` | ADS device from which a symbol is to be logged in the standard |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WriteBySymbol` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method reads the value of a specified ADS symbol and saves it in the standard table structure specified by Beckhoff. ADS symbols from other ADS devices can also be read.

##### `WriteStruct` (METHOD) → `BOOL` — §6.1.4.2.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `sTableName` | `T_MaxString` | Name of the table that is to be read. |
| `pRecord` | `POINTER TO BYTE` | Address of a structure that is to be logged in a freely |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `WriteStruct` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method creates a record with a freely selectable table structure.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode

##### `FB_PLCDBCmd` (FUNCTION_BLOCK) — §6.1.4.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is directed. |
| `tTimeout` | `TIME PLC API` | Indicates the time before the function is cancelled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function |

**Description:** Function block with two methods. Users can define and transfer their own SQL commands. Placeholders in the SQL command can correlate with structures in the PLC, which reflect the table structure. The Database Server ultimately enters the current data of the structure into the SQL command.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › PLC Expert mode › FB_PLCDBCmd

##### `Execute` (METHOD) → `BOOL` — §6.1.4.2.6.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | — |
| `pExpression` | `POINTER TO BYTE` | — |
| `cbExpression` | `UDINT` | — |
| `pData` | `POINTER TO BYTE` | — |
| `cbData` | `UDINT` | — |
| `pParameter` | `POINTER TO ARRAY[0..MAX_DBCOLUMNS] OF ST_ExpParameter` | — |
| `cbParameter` | `UDINT` | — |

**Description:** This method can be used to send SQL commands to the database. The database connection is opened with each call and then closed again. It is possible to define placeholders in the command, which are then replaced by the TwinCAT Database Server with the corresponding values before the execution. Returned records cannot be read.

##### `ExecuteDataReturn` (METHOD) → `BOOL` — §6.1.4.2.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |
| `pExpression` | `POINTER TO BYTE` | Address of the string variable with the SQL command. |
| `cbExpression` | `UDINT` | Length of the string variable with the SQL command. |
| `pData` | `POINTER TO BYTE` | Address of the structure with the parameter values |
| `cbData` | `UDINT` | Length of the structure with the parameter values |
| `pParameter` | `POINTER TO ARRAY[0..MAX_ Address of the structure array with the parameter` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ExecuteDataReturn` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method can be used to send SQL commands to the database. The database connection is opened with each call and then closed again. It is possible to define placeholders in the command, which are then replaced by the TwinCAT Database Server with the corresponding values before the execution. A specified number of records can be read.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › SQL Expert mode

##### `FB_ConfigTcDBSrv` (FUNCTION_BLOCK) — §6.1.4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is directed. |
| `tTimeout` | `TIME` | Indicates the time before the function is cancelled. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function |

**Description:** Function block for creating, reading and deleting configuration entries for the TwinCAT Database Server.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › SQL Expert mode › FB_ConfigTcDBSrv

##### `Create` (METHOD) → `BOOL` — §6.1.4.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTcDBSrvConfig` | `POINTER TO BYTE` | Pointer of the configuration structure to be created. |
| `cbTcDBSrvConfig` | `UDINT` | Length of the configuration structure |
| `bTemporary` | `BOOL` | Indicates whether the configuration is to be stored in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Create` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method creates new entries in the XML configuration file for the TwinCAT Database Server. Optionally the TwinCAT Database Server can use a new entry on a temporary basis. In this case no data is written to the XML file.

##### `Read` (METHOD) → `BOOL` — §6.1.4.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDBConfig` | `POINTER TO ARRAY [1..MA Pointer address of the array into which the database` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to read the current configurations of the TwinCAT Database Server. Any temporary configurations that may be included are marked accordingly.

##### `Delete` (METHOD) → `BOOL` — §6.1.4.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eTcDBSrvConfigType` | `E_TcDBSrvConfigType` | Type of the configuration to be deleted (database / AutoLog group) |
| `hConfigID` | `UDINT` | ID of the configuration to be deleted (hDBID or hAutoLogGrpID) |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Delete` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to delete databases and AutoLog groups from the configuration of the

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › SQL Expert mode

##### `FB_SQLDatabase` (FUNCTION) — §6.1.4.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block is active. |
| `bError` | `BOOL` | TRUE when an error occurs. |
| `ipTcResultEvent` | `Tc3_EventLogger.I_TcResult` | Result interface with detailed information on the return |
| `Event` | `value. Methods tion` | — |
| `Connect [} 281]` | `Local` | Opens a connection to a declared database. |
| `CreateCmd [} 282]` | `Local` | Initializes an instance of the function block FB_SQLCommand |

**Description:** Function block for opening, closing and managing a database connection.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › SQL Expert mode › FB_SQLDatabase

##### `Connect` (METHOD) → `BOOL` — §6.1.4.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hDBID` | `UDINT` | Indicates the ID of the database to be used. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Connect` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method opens a connection to a declared database.

##### `CreateCmd` (METHOD) → `BOOL` — §6.1.4.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSQLCommand` | `POINTER TO FB_SQLCommand` | Returns a new instance of the function block |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateCmd` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method is used to initialize an instance of the function block FB_SQLCommand with the already open database connection of the function block FB_SQLDatabase. The function block FB_SQLCommand only uses the database connection it was assigned via the CreateCmd method. Several instances of the function block FB_SQLCommand can be initialized with the same database connection.

##### `CreateSP` (METHOD) → `BOOL` — §6.1.4.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sProcedureName T_MaxString` | `Indicates the name of the procedure to be` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateSP` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method is used to initialize an instance of the function block FB_SQLStoredProcedure with the already open database connection of the function block FB_SQLDatabase. The function block FB_SQLStoredProcedure only uses the database connection it was assigned via the CreateCmd method. Several instances of the function block FB_SQLStoredProcedure can be initialized with the same database connection.

##### `Disconnect` (METHOD) → `BOOL` — §6.1.4.3.2.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Disconnect` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the method |

**Description:** This method closes the connection to the database that was opened by this function block instance.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › SQL Expert mode

##### `FB_SQLCommand` (FUNCTION) — §6.1.4.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block is active. |
| `bError` | `BOOL` | TRUE when an error occurs. |
| `ipTcResultEvent` | `Tc3_EventLogger.I_TcResult` | Result interface with detailed information on the return |
| `Event` | `value. PLC API Methods tion` | — |
| `Execute [} 285]` | `Local` | Sends the specified SQL command to the database via the |

**Description:** Function block for executing SQL commands. Before it can be used it has to be initialized with the function block FB_SQLDatabase.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › SQL Expert mode › FB_SQLCommand

##### `Execute` (METHOD) → `BOOL` — §6.1.4.3.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSQLCmd` | `POINTER TO BYTE Indicates the pointer address of a string variable with the SQL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `POINTER TO BYTE Displays the status of the method. Returns TRUE as soon as the` | — |

**Description:** This method sends the specified SQL command to the database via the database connection already opened by the function block FB_SQLDatabase.

##### `ExecuteDataReturn` (METHOD) → `BOOL` — §6.1.4.3.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSQLCmd` | `POINTER TO BYTE` | Indicates the pointer address of a string variable with |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ExecuteDataReturn` | `POINTER TO BYTE` | Displays the status of the method. Returns TRUE as soon |

**Description:** This method sends the specified SQL command to the database via the database connection already opened by the function block FB_SQLDatabase. An instance of the function block FB_SQLResult can be transferred for reading the returned records.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › SQL Expert mode

##### `FB_SQLResult` (FUNCTION) — §6.1.4.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block is active. |
| `bError` | `BOOL` | TRUE when an error occurs. |
| `ipTcResultEvent` | `Tc3_EventLogger.I_TcRes Result interface with detailed information on the return value. ultEvent Methods tion` | — |
| `Read [} 287]` | `Local` | Reads a specified number of records from the result data |

**Description:** The function block is used for reading the cached records.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › SQL Expert mode › FB_SQLResult

##### `Read` (METHOD) → `BOOL` — §6.1.4.3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nStartIndex` | `UDINT` | Indicates the index of the first record to be read. |
| `nRecordCount` | `UDINT` | Indicates the number of records to be read. |
| `pData` | `POINTER TO BYTE` | Address of the structure array into which the records are to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method reads a specified number of records from the result data cached in the

##### `Release` (METHOD) → `BOOL` — §6.1.4.3.4.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Release` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method can be used to release data cached by the TwinCAT Database Server.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › SQL Expert mode

##### `FB_SQLStoredProcedure` (FUNCTION) — §6.1.4.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | AMS network ID of the target device at which the ADS command is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | TRUE as soon as a method of the function block is active. |
| `bError` | `BOOL` | TRUE when an error occurs. |
| `ipTcResultEvent` | `Tc3_EventLogger.I_TcR Result interface with detailed information on the return value. esultEvent` | — |
| `Execute [} 290]` | `Local` | Sends the call of the specified stored procedure to the |

**Description:** Function block for executing stored procedures of the database. Before it can be used it has to be initialized with the function block "FB_SQLDatabase".

#### AutoLog groups List of configured AutoLog groups on the target system › Tc3_Database › Obsolete › SQL Expert mode › FB_SQLStoredProcedure

##### `Execute` (METHOD) → `BOOL` — §6.1.4.3.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pParameterStrc` | `POINTER TO BYTE Pointer address to the parameter structure that is transferred to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method sends the call of the specified stored procedure to the database via the database connection already opened by the function block FB_SQLDatabase.

##### `ExecuteDataReturn` (METHOD) → `BOOL` — §6.1.4.3.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pParameterStrc` | `POINTER TO BYTE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Read` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method sends the call of the specified stored procedure to the database via the database connection already opened by the function block FB_SQLDatabase. An instance of the FB_SQLResult function block can be transferred for reading the returned records.

##### `Release` (METHOD) → `BOOL` — §6.1.4.3.5.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Release` | `BOOL` | Displays the status of the method. Returns TRUE as soon as the |

**Description:** This method releases the parameter information of the stored procedure, which was transferred during initialization.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc2_Database › Function blocks

##### `FB_GetStateTcDatabase` — §6.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `nAdsState` | `UINT` | — |
| `nDevState` | `UINT` | — |

**Description:** The FB_GetStateTcDatabase function block can be used to query the current status of the database server.

##### `FB_DBReloadConfig` — §6.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** The XML configuration file can be reloaded using the FB_DBReloadConfig function block. If changes have been made to the XML configuration file, the changes must be made known to the Database Server using FB_DBReloadConfig.

##### `FB_DBConnectionAdd` — §6.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `eDBType` | `E_DBTypes` | — |
| `eDBValueType` | `E_DBValueType` | — |
| `sDBServer` | `T_MaxString` | — |
| `sDBProvider` | `T_MaxString` | — |
| `sDBUrl` | `T_MaxString` | — |
| `sDBSystemDB` | `T_MaxString` | — |
| `sDBUserId` | `T_MaxString` | — |
| `sDBPassword` | `T_MaxString` | — |
| `sDBTable` | `T_MaxString` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `bErrID` | `UDINT` | — |
| `hDBID` | `UDINT` | — |

**Description:** The FB_DBConnectionAdd function block permits additional database connections to be added to the XML configuration file.

##### `FB_DBOdbcConnectionAdd` — §6.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `eDBType` | `E_DBTypes` | — |
| `eDBValueType` | `E_DBValueType` | — |
| `sDBDriver` | `T_MaxString` | — |
| `sDBServer` | `T_MaxString` | — |
| `sDBDatabase` | `T_MaxString` | — |
| `nDBPort` | `UDINT` | — |
| `sDBProtocol` | `T_MaxString` | — |
| `sDBUserId` | `T_MaxString` | — |
| `sDBPassword` | `T_MaxString` | — |
| `sDBScheme` | `T_MaxString` | — |
| `sDBSequence` | `T_MaxString` | — |
| `sDBClientDll` | `T_MaxString` | — |
| `sDBTable` | `T_MaxString` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `bErrID` | `UDINT` | — |
| `hDBID` | `UDINT` | — |

**Description:** The FB_DBOdbcConnectionAdd function block can be used to add further ODBC database connections to the XML configuration file.

##### `FB_AdsDeviceConnectionAdd` — §6.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | — |
| `sADSDevNetID` | `T_AmsNetID` | — |
| `nADSDevPort` | `UINT` | — |
| `tADSDevTimeout` | `TIME` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `hAdsId` | `UDINT` | — |

**Description:** AdsDevices can be declared in the XML configuration file using the FB_AdsDeviceConnectionAdd function block.

##### `FB_GetDBXMLConfig` — §6.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `cbDBCfg` | `UDINT` | — |
| `pDBCfg` | `POINTER TO ARRAY [0.. MAX_XML_DECLARATIONS] OF ST_DBXMLCfg` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** The FB_GetDBXMLConfig function block can be used to read all databases that are declared in the XML configuration file.

##### `FB_GetAdsDevXMLConfig` — §6.2.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `cbAdsDevCfg` | `UDINT` | — |
| `pAdsDevCfg` | `POINTER TO ARRAY [0.. MAX_XML_DECLARATIONS] OF ST_ADSDevXMLCfg` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** The FB_GetAdsDevXMLConfig function block can be used to read all ADS devices that are declared in the XML configuration file.

##### `FB_DBConnectionOpen` — §6.2.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `hDBID` | `DINT` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |

**Description:** The FB_DBConnectionOpen function block can be used to open connections to databases. This can accelerate the read/write access with the function blocks FB_DBWrite, FB_DBRead, FB_DBRecordInsert and FB_FBRecordSelect.

##### `FB_DBConnectionClose` — §6.2.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `hDBID` | `DINT` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** The FB_DBConnectionClose function block can be used to close connections to databases. If a connection with a database was opened previously, it must be closed again.

##### `FB_DBCreate` — §6.2.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | — |
| `sPathName` | `T_MaxString` | — |
| `sDBName` | `T_MaxString` | — |
| `eDBType` | `E_DBTypes` | — |
| `sSystemDB` | `T_MaxString` | — |
| `sUserID` | `T_MaxString` | — |
| `sPassword` | `T_MaxString` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** The FB_DBCreate function block allows databases to be created.

##### `FB_DBTableCreate` — §6.2.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | — |
| `hDBID` | `UDINT` | — |
| `sTableName` | `T_MaxString` | — |
| `cbTableCfg` | `UDINT` | — |
| `pTableCfg` | `POINTER TO ARRAY[0..MAX_DB_TABLE_COLUMNS] OF ST_DBColumnCfg` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |

**Description:** The FB_DBTableCreate function block can be used to create tables of any structure in databases.

##### `FB_DBCyclicRdWrt` — §6.2.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |

**Description:** The FB_DBCyclicRdWrt function block can be used to start or stop the cyclic logging \ writing of variables.

##### `FB_DBRead` — §6.2.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `hDBID` | `DINT` | — |
| `sDBVarName` | `STRING(80)` | — |
| `cbReadLen` | `UDINT` | — |
| `pDestAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |

**Description:** The FB_DBRead function block allows values to be read from a database. The function block searches for the specified sDBVarName in the "Name" column of the database table and then outputs the corresponding value from the "Value" column. If the searched sDBVarName exists several times in the database table, the first record found is output.

##### `FB_DBWrite` — §6.2.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | — |
| `hDBID` | `UDINT` | — |
| `hAdsID` | `UDINT` | — |
| `sVarName` | `T_MaxString` | — |
| `nIGroup` | `UDINT` | — |
| `nIOffset` | `UDINT` | — |
| `nVarSize` | `UDINT` | — |
| `sVarType` | `T_MaxString` | — |
| `sDBVarName` | `T_MaxString` | — |
| `eDBWriteMode` | `E_DBWriteModes` | — |
| `tRingBufferTime` | `TIME` | — |
| `nRingBufferCount` | `UDINT` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |

**Description:** The FB_DBWrite function block can be used to write the values of individual variables to databases. The table structure must have the columns "Timestamp", "Name" and "Value" (see "SQL Compact Database" [} 128]). To use the function block, the database to which the variable is to be written and the ADS device from which the variable is to be read must be declared in the XML configuration file.

##### `FB_DBRecordDelete` — §6.2.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `hDBID` | `UDINT` | — |
| `cbCmdSize` | `UDINT` | — |
| `pCmdAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |

**Description:** The FB_DBRecordDelete function block can be used to delete individual records from a database. This function block allows SQL DELETE commands with up to 10,000 characters to be executed. To use the function block, the database from which records are to be deleted must be declared in the XML configuration file.

##### `FB_DBRecordInsert_EX` — §6.2.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `hDBID` | `UDINT` | — |
| `cbCmdSize` | `UDINT` | — |
| `pCmdAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |

**Description:** The FB_DBRecordInsert_EX function block can be used to write individual records with any structure into a database. This function block allows SQL INSERT commands with up to 10,000 characters to be executed. To use the function block, the database to which the data is to be written must be declared in the XML configuration file.

##### `FB_DBRecordArraySelect` — §6.2.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | — |
| `hDBID` | `UDINT` | — |
| `cbCmdSize` | `UDINT` | — |
| `pCmdAddr` | `UDINT` | — |
| `nStartIndex` | `UDINT` | — |
| `nRecordCount` | `UDINT` | — |
| `cbRecordArraySize` | `UDINT` | — |
| `pDestAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |
| `nRecords` | `UDINT` | — |

**Description:** The FB_DBRecordArraySelect function block can be used to read several records with any structure from the database. This function block allows SQL SELECT commands with up to 10,000 characters to be executed. This function block is not compatible with ASCII files.

##### `FB_DBStoredProcedures` — §6.2.1.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID     :=''` | — |
| `hDBID` | `UDINT          :=1` | — |
| `sProcedureName` | `T_MaxString    :=''` | — |
| `cbParameterList` | `UDINT` | — |
| `pParameterList` | `POINTER TO ARRAY[0..MAX_STORED_PROCEDURES_PARAMETERS] OF ST_DBParameter` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME           := T#15s` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |

**Description:** The FB_DBStoredProcedures function block can be used to call stored procedures. Parameters can be transferred that are used in the stored procedures.

##### `FB_DBStoredProceduresRecordArray` — §6.2.1.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID           :=''` | — |
| `hDBID` | `UDINT                :=1` | — |
| `sProcedureName` | `T_MaxString          :=''` | — |
| `cbParameterList` | `UDINT` | — |
| `pParameterList` | `POINTER TO ARRAY[0..MAX_STORED_PROCEDURES_PARAMETERS] OF ST_DBParameter` | — |
| `nStartIndex` | `UDINT` | — |
| `nRecordCount` | `UDINT` | — |
| `cbRecordArraySize` | `UDINT` | — |
| `pDesAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME                  := T#15s` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |
| `nRecords` | `UDINT` | — |

**Description:** The FB_DBStoredProceduresRecordArray function block can be used to call stored procedures that return records. In contrast to the FB_DBStoredProceduresRecordReturn function block, this function block can be used to return several records with a single call. Parameters can be transferred that are used in the stored procedures.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc2_Database › Function blocks › Obsolete

##### `FB_DBAuthentificationAdd` — §6.2.1.20.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | — |
| `hDBID` | `DINT` | — |
| `sDBSystemDB` | `T_MaxString` | — |
| `sDBUserId` | `T_MaxString` | — |
| `sDBPassword` | `T_MaxString` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |

**Description:** The FB_DBAuthentificationAdd function block can be used to add authentication information to already declared databases or to overwrite existing authentication information.

##### `FB_DBRecordInsert` — §6.2.1.20.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetId` | — |
| `hDBID` | `UDINT` | — |
| `sInsertCmd` | `T_MaxString` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |

**Description:** The FB_DBRecordInsert function block can be used to write individual records with any structure into a database. To use the function block it is necessary to declare the database to which the records are to be written in the XML configuration file.

##### `FB_DBRecordSelect` — §6.2.1.20.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | — |
| `hDBID` | `UDINT` | — |
| `sSelectCmd` | `T_MaxString` | — |
| `nRecordIndex` | `UDINT` | — |
| `cbRecordSize` | `UDINT` | — |
| `pDestAddr` | `DWORD` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |
| `nRecords` | `UDINT` | — |

**Description:** The FB_DBRecordSelect function block can be used to read individual records with any structure from a database. This function block is not compatible with ASCII files.

##### `FB_DBRecordSelect_EX` — §6.2.1.20.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID` | — |
| `Hdbid` | `UDINT` | — |
| `cbCmdSize` | `UDINT` | — |
| `pCmdAddr` | `UDINT` | — |
| `nRecordIndex` | `UDINT` | — |
| `cbRecordSize` | `UDINT` | — |
| `pDestAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |
| `nRecords` | `UDINT` | — |

**Description:** The FB_DBRecordSelect_EX function block can be used to read individual records with any structure from the database. This function block allows SQL SELECT commands with up to 10000 characters to be executed. This function block is not compatible with ASCII files.

##### `FB_DBStoredProceduresRecordReturn` — §6.2.1.20.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sNetID` | `T_AmsNetID     :=''` | — |
| `hDBID` | `UDINT          :=1` | — |
| `sProcedureName` | `T_MaxString    :=''` | — |
| `cbParameterList` | `UDINT` | — |
| `pParameterList` | `POINTER TO ARRAY[0..MAX_STORED_PROCEDURES_PARAMETERS] OF ST_DBParameter` | — |
| `nRecordIndex` | `UDINT` | — |
| `cbRecordSize` | `UDINT` | — |
| `pRecordAddr` | `POINTER TO BYTE` | — |
| `bExecute` | `BOOL` | — |
| `tTimeout` | `TIME           := T#15s` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | — |
| `bError` | `BOOL` | — |
| `nErrID` | `UDINT` | — |
| `sSQLState` | `ST_DBSQLError` | — |
| `nRecords` | `UDINT` | — |

**Description:** The FB_DBStoredProceduresRecordReturn function block can be used to call stored procedures that return a record. Parameters can be transferred that are used in the stored procedures.

#### AutoLog groups List of configured AutoLog groups on the target system › Tc2_Database › Data types

##### `ST_DBColumnCfg` — §6.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sColumnName` | `STRING(59)` | — |
| `sColumnProperty` | `STRING(59)` | — |
| `eColumnType` | `E_DbColumnTypes` | — |
| `sColumnName` | `Contains the name of the column to be created.` | — |
| `sColumnProperty` | `Contains certain column properties.` | — |
| `eColumnType` | `Gives the type of column.` | — |

##### `ST_DBXMLCfg` — §6.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sDBName` | `STRING` | — |
| `sDBTable` | `STRING` | — |
| `nDBID` | `DINT` | — |
| `eDBType` | `E_DBTypes` | — |
| `sDBName` | `Contains the name of the database.` | — |
| `sDBTable` | `Contains the name of the table.` | — |
| `nDBID` | `Returns the ID of the database.` | — |
| `eDBType` | `Gives the type of database.` | — |

##### `ST_ADSDevXMLCfg` — §6.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sAdsDevNetID` | `T_AmsNetID` | — |
| `tAdsDevTimeout` | `TIME` | — |
| `nAdsDevID` | `DINT` | — |
| `nAdsDevPort` | `UINT` | — |
| `sAdsDevNetID` | `String containing the AMS network ID of the ADS device.` | — |
| `tAdsDevTimeout` | `Indicates the timeout time of the ADS device.` | — |
| `nAdsDevID` | `Returns the ID of the ADS device.` | — |
| `nAdsDevPort` | `Indicates the port of the ADS device.` | — |

##### `ST_DBSQLError` — §6.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sSQLState` | `STRING(5)` | — |
| `nSQLErrorCode` | `DINT` | — |
| `sSQLState` | `Contains the 5-character error code, which is based on the SQL ANSI standard.` | — |
| `nSQLErrorCode` | `Returns a database-specific error code.` | — |
| `sSQLState` | `= '00000'` | — |
| `nSQLErrorCode` | `= 0` | — |

##### `ST_DBParameter` — §6.2.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sParameterName` | `STRING(59)` | — |
| `cbParameterValue` | `UDINT` | — |
| `pParameterValue` | `UDINT` | — |
| `eParameterDataType` | `E_DBColumnTypes` | — |
| `eParameterType` | `E_DBParameterTypes` | — |
| `sParameterName` | `Indicates the name of the parameter.` | — |
| `cbParameterValue` | `Contains the size of the variable to be used in bytes.` | — |
| `pParameterValue` | `Contains the address of the variable to be used.` | — |
| `eParameterDataType` | `Indicates the data type of the parameter (E_DBColumnTypes [} 323]).` | — |
| `eParameterType` | `Indicates the parameter type (E_DBParameterTypes [} 324]).` | — |

#### Symbols List of configured symbols for the selected AutoLog group › Tc3_Database › Scenario examples

##### `Message logger` (METHOD) → `BOOL` — §7.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sDBName` | `T_MaxString` | — |
| `tTimestamp` | `DT` | — |
| `eSeverity` | `E_Severity` | — |
| `nErrCode` | `UDINT` | — |
| `sMessage` | `T_MaxString` | — |
| `tStartTimestamp` | `DT` | — |
| `tEndTimestamp` | `DT` | — |
| `nStartIndex` | `UDINT` | — |
| `nLangId` | `INT := 1033` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `tTimestamp` | `DT` | — |
| `sSeverity` | `STRING(10)` | — |
| `nErrCode` | `UDINT` | — |
| `sMessage` | `T_MaxString` | — |
| `nErrorCount` | `UDINT` | — |
| `arrErrors` | `ARRAY [0..10] OF ST_ErrorEntry` | — |

**Description:** This scenario example illustrates the PLC Expert Mode for a Message Logger in the PLC. In the sample program the function blocks of the TwinCAT Database Server are used to create a function block, which provides various methods for generating and reading messages. The database in which the messages are stored is created from the PLC. A sample application of the created function block is implemented in the MAIN program. A new database file is created every 7 days. Three different messages can be sent. In addition it is possible to call up the last message or all messages from a particular interval.

##### `Production recipe` (METHOD) → `BOOL` — §7.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTypeNum` | `DINT` | — |
| `nTypeNum` | `DINT` | — |
| `sTestNum` | `STRING(8)` | — |
| `nTypeNum` | `DINT` | — |
| `sTimestamp` | `STRING` | — |
| `sTester` | `STRING` | — |
| `sResult` | `STRING` | — |
| `nLangId` | `INT := 1033` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stConfig` | `ST_Config` | — |
| `sTestNum` | `STRING(8)` | — |
| `stTestPara` | `ST_TestParameter` | — |

**Description:** This scenario example illustrates how the TwinCAT Database Server handles XML files with any structure. The production recipe for building the product is read from an XML file. The corresponding test parameters are read from a different file. In addition, the test results are written into an existing XML file.
