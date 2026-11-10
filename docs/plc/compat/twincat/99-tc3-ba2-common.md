# Tc3_BA2_Common

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_BA2_Common_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Building · **Tier:** T0 · **API entries:** 66
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Building-automation common types/helpers: shared type layer for the BA libraries.

---

#### Programming › POUs › Functions › Compare

##### `F_BA_CompareVersion` (FUNCTION) → `BOOL` — §4.3.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stVersion1,` | `ST_BA_Version [} 38] Version numbers to be compared. stVersion2 50 Programming` | — |
| `eCompare` | `E_BA_CompareMode Comparison operation to be performed. [} 8]` | — |
| `nLimit` | `UINT(1 ... 4)` | Number of digits to be compared, starting with "Major". |

**Description:** The function F_BA_CompareVersion of return type BOOL compares two version numbers stVersion1 and stVersion2, each of type ST_BA_Version [} 38]. The input variable nLimit specifies how many digits are to be compared, starting with 1 = "Major". The enumeration eCompare specifies the comparison operation. The function return value changes to TRUE if the comparison is fulfilled.

#### Programming › POUs › Functions › Memory

##### `F_BA_ByteCmp` (FUNCTION) → `DINT` — §4.3.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pValue` | `PVOID` | Pointer to the beginning of the memory area to be examined. |
| `nSize` | `UXINT` | Length of the memory area. |
| `nCompare` | `BYTE` | Comparison byte. |

**Description:** The function F_BA_ByteCmp of return type DINT compares the contents of a memory area in byte steps with a compare byte nCompare.

##### `F_BA_Cmp` (FUNCTION) → `DINT` — §4.3.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pValue` | `PVOID` | Pointer to the beginning of the memory area to be examined. |
| `pCompare` | `PVOID` | Pointer to the beginning of the memory area to be compared. |
| `nSize` | `UXINT` | Length of the memory area. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nEqualBytes` | `UINT` | Display how many bytes were equal before the comparison |

**Description:** The function F_BA_Cmp of return type DINT compares two memory areas of the same size. Here pValue defines the beginning of the observation area and pCompare the beginning of the area to be compared. It is compared byte by byte. As soon as a byte is found in the observation area which is smaller than the one of the area to be compared, the function aborts the comparison and takes the return value "-1". If a byte is found in the observation area that is larger than that of the area to be compared, the function also aborts and takes the return value "1". If, on the other hand, no difference is found, i.e. all bytes of the observation area are identical to those of the area to be compared, the function assumes the value "0" at the end of the comparison.

##### `F_BA_GetUsedEntryCount` (FUNCTION) → `DINT` — §4.3.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pArray` | `PVOID` | Pointer to the beginning of the memory area where the array to |

**Description:** 52

##### `F_BA_MemSet` (FUNCTION) → `UDINT` — §4.3.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDestAddr` | `PVOID` | Destination address of the write operation. Programming |
| `xValue` | `ANY` | Value to be written. |

**Description:** The function F_BA_MemSet of return type UDINT describes the address range starting with pDestAddr with the value of a variable xValue of any type.

##### `F_BA_MemSetEx` (FUNCTION) → `UDINT` — §4.3.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pDestAddr` | `PVOID` | Destination address of the filling area. |
| `nDestSize` | `UDINT` | Size of the filling area. |
| `xValue` | `ANY` | Filling value |

**Description:** The function F_BA_MetSetEx of return type UDINT fills the address range starting with pDestAddr and length nDestSize with the value of a variable xValue of any type.

##### `F_BA_OffsetPtr` (FUNCTION) → `PVOID` — §4.3.1.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pAddr` | `PVOID` | Base address |
| `nOffset` | `DINT` | Offset |

**Description:** The function F_BA_OffestPtr of return type PVOID adds an offset nOffset to the entered address pAddr and returns the result as return value of the function. This value then in turn represents an address.

##### `F_BA_DiffPtr` (FUNCTION) → `UDINT` — §4.3.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pAddr1` | `PVOID` | Address 1 |
| `pAddr2` | `PVOID` | Address 2 |

**Description:** The function F_BA_DiffPtr of return type UDINT specifies the difference between two memory addresses as an absolute value: internally the smaller address is subtracted from the larger one.

#### Programming › POUs › Functions › Types

##### `AnyLEN` (FUNCTION) → `INT` — §4.3.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `STR` | `ANY` | Variable to be examined. In principle, any simple variable type |

**Description:** The function AnyLEN with the return type INT determines the described length of a string variable. This is applied to input STR of type ANY and can thus be analyzed.

#### Programming › POUs › Functions › Types › ClassValue

##### `F_BA_BVal` (FUNCTION) → `U_BA_ClassValue` — §4.3.1.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bValue` | `BOOL` | This value is assigned to iVal in the output structure. |

**Description:** The return value of the F_BA_BVal function is created by the structure of type U_BA_ClassValue [} 40]. The input value bValue is written to the variable bVal of this structure. Due to the data type UNION, whose elements all start from the same memory address, all other values of this structure change as well.

##### `F_BA_ByteVal` (FUNCTION) → `U_BA_ClassValue` — §4.3.1.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nByte1` | `BYTE` | This value is assigned _nByteVal[1]_to the output structure. |
| `nByte2` | `BYTE` | This value is assigned _nByteVal[2]_to the output structure. |
| `nByte3` | `BYTE` | This value is assigned _nByteVal[3]_to the output structure. |
| `nByte4` | `BYTE` | This value is assigned _nByteVal[4]_to the output structure. |

**Description:** The return value of the F_BA_ByteVal function is created by the structure of type U_BA_ClassValue [} 40]. The input values nByte1, nByte2, nByte3 and nByte4 are assigned in this structure on the respective values nByteVal[1], nByteVal[2], nByteVal[3] and nByteVal[4]. Due to the data type UNION, whose elements all start from the same memory address, all other values of this structure change as well.

##### `F_BA_IVal` (FUNCTION) → `U_BA_ClassValue` — §4.3.1.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nValue` | `INT` | This value is assigned to iVal in the output structure. Programming |

**Description:** The return value of the F_BA_IVal function is created by the structure of type U_BA_ClassValue [} 40]. The input value nValue is written to the variable iVal of this structure. Due to the data type UNION, whose elements all start from the same memory address, all other values of this structure change as well.

##### `F_BA_RVal` (FUNCTION) → `U_BA_ClassValue` — §4.3.1.3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fValue` | `REAL` | This value is assigned to rVal in the output structure. |

**Description:** The return value of the F_BA_RVal function is created by the structure of type U_BA_ClassValue [} 40]. The input value fValue is written to the variable rVal of this structure. Due to the data type UNION, whose elements all start from the same memory address, all other values of this structure change as well.

##### `F_BA_UDIVal` (FUNCTION) → `U_BA_ClassValue` — §4.3.1.3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nValue` | `UINT` | This value is assigned to udiVal in the output structure. |

**Description:** The return value of the F_BA_UDIVal function is created by the structure of type U_BA_ClassValue [} 40]. The input value nValue is written to the variable udiVal of this structure. Due to the data type UNION, whose elements all start from the same memory address, all other values of this structure change as well.

#### Programming › POUs › Functions › Types › Date and Time › Check

##### `F_BA_DateHasPlaceholder` (FUNCTION) → `BOOL` — §4.3.1.3.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDate` | `ST_BA_Date [} 33]` | Observed date-time entry. |

**Description:** The function F_BA_DateHasPlaceholder of return type BOOL returns a TRUE if from the input structure the year, month, day or weekday component holds the placeholder value 16#FF.

##### `F_BA_DateUnspecified` (FUNCTION) → `BOOL` — §4.3.1.3.3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDate` | `ST_BA_Date [} 33]` | Observed date-time entry. |

**Description:** The function F_BA_DateUnspecified of return type BOOL returns a TRUE if the year, month, day and weekday components of the input structure have the placeholder value 16#FF.

##### `F_BA_IsLeapYear` (FUNCTION) → `BOOL` — §4.3.1.3.3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nYear` | `UDINT` | Year to be studied. |

**Description:** The function F_BA_IsLeapYear of return type BOOL returns a TRUE if the entered year is a leap year.

##### `F_BA_TimeHasPlaceholder` (FUNCTION) → `BOOL` — §4.3.1.3.3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stTime` | `ST_BA_Time [} 34]` | Observed day-time entry. |

**Description:** The function F_BA_TimeHasPlaceholder of return type BOOL returns a TRUE if from the input structure the hour, minute or second component holds the placeholder value 16#FF.

##### `F_BA_TimeUnspecified` (FUNCTION) → `BOOL` — §4.3.1.3.3.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stTime` | `ST_BA_Time [} 34]` | Observed day-time entry. |

**Description:** 60

#### Programming › POUs › Functions › Types › Date and Time › Convert

##### `F_BA_TimeStruct_TO_DateTime` (FUNCTION) → `ST_BA_DateTime` — §4.3.1.3.3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tValue` | `TIMESTRUCT` | Time input value to be converted. |

**Description:** The function F_BA_TIMESTRUCT_TO_DateTime of return type ST_BA_DateTime [} 34] converts a time value tValue of type TIMESTRUCT into a time value of type ST_BA_DateTime [} 34].

##### `F_BA_TimeStruct_TO_Time` (FUNCTION) → `ST_BA_Time` — §4.3.1.3.3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tValue` | `TIMESTRUCT` | Time input value to be converted. |

**Description:** The function F_BA_TIMESTRUCT_TO_Time of return type ST_BA_Time [} 34] converts a time value tValue of type TIMESTRUCT into a time value of type ST_BA_Time [} 34].

##### `F_BA_To100msDate` (FUNCTION) → `UDINT` — §4.3.1.3.3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stBACnetDate` | `ST_BA_Date [} 33]` | Date input value to be converted. |

**Description:** The function F_BA_To100msDate of return type UDINT converts a date stBACnetDate of type ST_BA_Date [} 33] into a tenths of a second value related to 01.01.1900 00:00:00.

##### `F_BA_To100msTime` (FUNCTION) → `UDINT` — §4.3.1.3.3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stBACnetTime` | `ST_BA_Time [} 34]` | Time input value to be converted. |

**Description:** The function F_BA_To100msTime of return type UDINT converts a time of day stBACnetTime of type ST_BA_Time [} 34] into tenths of a second starting from 00:00:00.

##### `F_BA_ToDate` (FUNCTION) → `DATE` — §4.3.1.3.3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDate` | `ST_BA_Date [} 33]` | Date input value to be converted. |

**Description:** The function F_BA_ToDATE of return type DATE converts a date stDate of type ST_BA_Date [} 33] into a date of type DATE.

##### `F_BA_ToDT` (FUNCTION) → `DATE` — §4.3.1.3.3.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDate` | `ST_BA_Date [} 33]` | Date input value to be converted. |

**Description:** The function F_BA_ToDATE of return type DATE_AND_TIME converts a date stDate of type ST_BA_Date [} 33] into a date of type DATE_AND_TIME.

##### `F_BA_ToSTDate` (FUNCTION) → `ST_BA_Date` — §4.3.1.3.3.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dValue` | `DATE` | Date input value to be converted. |

**Description:** The function F_BA_ToSTDate of return type ST_BA_Date [} 33] converts a date dValue of type DATE into a date of type ST_BA_Date [} 33].

##### `F_BA_ToSTDateTime` (FUNCTION) → `ST_BA_DateTime` — §4.3.1.3.3.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dtDateTime` | `DT 64` | Time input value to be converted. Programming |

**Description:** The function F_BA_ToSTDateTime of return type ST_BA_DateTime [} 34] converts a time value dtDateTime of type DT (DATE_AND_TIME) into a time value of type ST_BA_DateTime [} 34].

##### `F_BA_ToSTTime` (FUNCTION) → `ST_BA_Time` — §4.3.1.3.3.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tTime` | `TIME` | Time input value to be converted. |

**Description:** The function F_BA_ToSTTime of return type ST_BA_Time [} 34] converts a time value tTime of type TIME into a time value of type ST_BA_Time [} 34].

##### `F_BA_ToTime` (FUNCTION) → `TIME` — §4.3.1.3.3.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stTime` | `ST_BA_Time [} 34]` | Time input value to be converted. |

**Description:** The function F_BA_ToTIME of return type TIME converts a time value stTime of type ST_BA_Time [} 34] into a time value of type TIME.

#### Programming › POUs › Functions › Types › Date and Time

##### `F_BA_CountLeapYears` (FUNCTION) → `UDINT` — §4.3.1.3.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nYear` | `UDINT` | End of the period under consideration. |
| `nStartYear` | `UDINT` | Start of the period under consideration. |

**Description:** The function F_BA_CountLeapYears of return type UDINT calculates the number of leap years from the entered start year nStartYear to the end of the period under consideration nYear (inclusive).

##### `F_BA_DateMerge` (FUNCTION) → `ST_BA_Date` — §4.3.1.3.3.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDate1` | `ST_BA_Date [} 33]` | The input date, which is checked if parts need to be replaced. |
| `stDate2` | `ST_BA_Date [} 33]` | Input date, which is used to replace stDate1. |

**Description:** The function F_BA_DateMerge of the return type ST_BA_Date [} 33] replaces those subcomponents of input date stDate1 which are evaluated as not specified (16#FF) with the corresponding components of input date stDate2.

##### `F_BA_DateTimeString` (FUNCTION) → `STRING` — §4.3.1.3.3.5

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stDateTime` | `ST_BA_DateTime` | — |

**Description:** The function F_BA_DateTimeString of return type STRING(19) converts a date stDateTime into a STRING of format DD.MM.YYYY hh:mm:ss.

##### `F_BA_DayOfWeek` (FUNCTION) → `E_BA_Weekday` — §4.3.1.3.3.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stDate` | `ST_BA_Date [} 33]` | Date from which the day of the week is determined. |

**Description:** The function F_BA_DayOfWeek of return type E_BA_Weekday [} 14] determines the corresponding weekday from a given date stDate.

##### `F_BA_DaysInMonth` (FUNCTION) → `UDINT` — §4.3.1.3.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stBACnetDate` | `ST_BA_Date [} 33]` | Date from whose month the number of days is to be determined. |

**Description:** The function F_BA_DaysInMonth of return type UDINT determines from a given date stBACnetDate the number of days of the given month, provided that the month and the year are not evaluated as unspecified, i.e. with 16#FF. In case of missing specification the return value of the function is 16#FFFFFF.

##### `F_BA_TimeMerge` (FUNCTION) → `ST_BA_Time` — §4.3.1.3.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stTime1` | `ST_BA_Time [} 34]` | Input timestamp, which is checked. |
| `stTime2` | `ST_BA_Time [} 34]` | Input timestamp which is used to replace unspecified parts of |

**Description:** The function F_BA_TimeMerge of return type ST_BA_Time [} 34] replaces those subcomponents of the input timestamp stTime1 that are evaluated as unspecified (16#FF) with the corresponding components of the input timestamp stTime2.

##### `F_BA_TimeString` (FUNCTION) → `STRING` — §4.3.1.3.3.11

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stTime` | `ST_BA_Time` | — |

**Description:** The function F_BA_TimeString of return type STRING(12) converts a timestamp stTime into a STRING in the format hh:mm:ss:**. The last two digits of the hundredths of a second are ignored and filled with "*". Decimal values smaller than 10 are preceded by a "0", e.g. the ninth hour in the morning is represented with "09" instead of simply "9".

#### Programming › POUs › Functions › Types › DateValue

##### `F_BA_DateRangeVal` (FUNCTION) → `U_BA_DateVal` — §4.3.1.3.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nFromYear` | `UINT` | Starting year of the time range. |
| `eFromMonth` | `E_BA_Month [} 12]` | Starting month of the time range. |
| `nFromDay` | `UINT` | Start day of the time range. |
| `nToYear` | `UINT` | End year of the time range. |
| `eToMonth` | `E_BA_Month [} 12]` | End month of the time range. |
| `nToDay` | `UINT` | End tag of the time range. |

**Description:** The function F_BA_DateRangeVal of return type U_BA_DateVal [} 35] uses the input variables nFromYear, eFromMonth, nFromDay, nToYear, eToMonth and nToDay fills the subcomponent ST_BA_DateRange [} 34] of the function return U_BA_DateVal [} 35]. This defines a time range.

##### `F_BA_DateVal` (FUNCTION) → `U_BA_DateVal` — §4.3.1.3.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nYear` | `UINT` | Entry of the year. |
| `eMonth` | `E_BA_Month [} 12]` | Entry of the month. |
| `nDay` | `UINT` | Entry of the day. |

**Description:** The function F_BA_DateVal of the return type U_BA_DateVal [} 35] fills the subcomponent ST_BA_Date [} 33] of the function return U_BA_DateVal [} 35] using the input variables nYear, eMonth and nDay. This defines a date.

##### `F_BA_WeekNDayVal` (FUNCTION) → `U_BA_DateVal` — §4.3.1.3.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eWeekday` | `E_BA_Weekday [} 14]` | Entry of the day of the week. |
| `eWeekOfMonth` | `E_BA_Week [} 13]` | Entry of the week within the month. |
| `eMonth` | `E_BA_Month [} 12]` | Entry of the month. |

**Description:** The function F_BA_WeekNDayVal of the return type U_BA_DateVal [} 35] fills the subcomponent ST_BA_WeekNDay [} 35] of the function return U_BA_DateVal [} 35] using the input variables eWeekday, eWeekOfMonth and eMonth. This defines a day of the week within a month.

#### Programming › POUs › Functions › Types › Scheduler

##### `F_BA_SetSchedulerEntry` (FUNCTION) → `BOOL` — §4.3.1.3.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tEntryTime` | `TIME` | Input of the time. |
| `uEntryValue` | `U_BA_ClassValue [} 40]` | Input of the value. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stEntry` | `ST_BA_SchedEntry` | — |

**Description:** The function F_BA_SetSchedulerEntry of return type BOOL first checks whether the input structure uEntryValue is undefined, i.e. filled with 16#FF values. If this is the case, the value E_BA_SchedEntryState.eNull is assigned to the status variable eState of the input-output structure stEntry, in the opposite case the value E_BA_SchedEntryState.eValue.

#### Programming › POUs › Functions › Types › Trend

##### `F_BA_TrendBufferSize` (FUNCTION) → `DINT` — §4.3.1.3.6.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aBuffer` | `ARRAY [*] OF ST_BA_TrendEntry` | — |

**Description:** The function F_BA_TrendBufferSize of return type DINT determines the size of a trend, which is defined as ARRAY of type ST_BA_TrendEntry [} 38].

#### Programming › POUs › Functions › Types

##### `F_BA_IsDisturbed` (FUNCTION) → `BOOL` — §4.3.1.3.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stStateFlags` | `ST_BA_StatusFlags [} 37]` | Status flags that are checked. |

**Description:** The function F_BA_IsDisturbed of return type BOOL checks the subcomponents of the input structure stStateFlags.

#### Programming › POUs › Functions › Universal › AuxiliaryCalculation

##### `F_BA_RemMsTof` (FUNCTION) → `UDINT` — §4.3.1.4.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tofTimer` | `TOF` | Input of the considered switch-off delay of type TOF. |

**Description:** The function F_BA_RemMsTof of return type UDINT displays the elapsed time of a TOF function (switch-off delay) in milliseconds.

##### `F_BA_RemMsTon` (FUNCTION) → `UDINT` — §4.3.1.4.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tonTimer` | `TON` | Input of the considered switch-on delay of type TON. |

**Description:** The function F_BA_RemMsTon of return type UDINT displays the elapsed time of a TON function (switch-on delay) in milliseconds.

##### `F_BA_RemMsTp` (FUNCTION) → `UDINT` — §4.3.1.4.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tpTimer` | `TP` | Input of the considered pulse function of type TP. |

**Description:** The function F_BA_RemMsTp of return type UDINT displays the elapsed time of a TP function (pulse function) in milliseconds.

##### `F_BA_RemSecsTof` (FUNCTION) → `UDINT` — §4.3.1.4.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tofTimer` | `TOF` | Input of the considered switch-off delay of type TOF. |

**Description:** The function F_BA_RemSecsTof of return type UDINT displays the elapsed time of a TOF function block (switch-off delay) in seconds.

##### `F_BA_RemSecsTone` (FUNCTION) → `UDINT` — §4.3.1.4.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tonTimer` | `TON 76` | Input of the considered switch-on delay of type TON. Programming |

**Description:** The function F_BA_RemSecsTon of return type UDINT displays the elapsed time of a TON function (switch- on delay) in seconds. The switch-on delay to be considered must be applied to input tonTimer.

##### `F_BA_RemSecsTp` (FUNCTION) → `UDINT` — §4.3.1.4.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tpTimer` | `TP` | Input of the considered pulse function of type TP. |

**Description:** The function F_BA_RemSecsTp of return type UDINT displays the elapsed time of a TP function (pulse function) in seconds.

#### Programming › POUs › Functions › Universal

##### `F_BA_CheckEnum` (FUNCTION) → `BOOL` — §4.3.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nIndex` | `INT` | Index to be examined. Inputs/outputs |
| `aInfo` | `ST_BA_EnumInfo [} 37]` | List in which the element with the index nIndex is to be found. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aInfo` | `ARRAY [*] OF ST_BA_EnumInfo` | — |

**Description:** The function F_BA_CheckEnum of return type BOOL determines whether the value nIndex of an enumeration exists within a predefined list aInfo of type ARRAY [*] OF ST_BA_EnumInfo [} 37].

#### Programming › POUs › Functions › Universal › TcLog

##### `F_BA_LogMessage` (FUNCTION) — §4.3.1.4.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nLogType` | `DWORD` | Log type, which can be set as a mask. The message is then |
| `nLogType` | `DWORD` | Log type, which can be set as a mask. The message is then |
| `nLogType` | `DWORD` | Log type, which can be set as a mask. The message is then |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `80 Programming` | `` | — |

**Description:** Internally, the function F_BA_LogMessage works with the function block FB_FormatString. In the first run, a message string is generated from sLogText. The text sLogText needs a valid format specification in addition to the message text. The resulting text is preceded by a text argument in square brackets in the second run of FB_FormatString.

#### Programming › POUs › Functions › ValidationFunctions

##### `F_BA_IsDateValChoiceValid` (FUNCTION) → `BOOL` — §4.3.1.5.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eChoice` | `E_BA_DateValChoice Enumeration to be checked. [} 11]` | — |

**Description:** The F_BA_IsDateValChoiceValid function of return type BOOL checks whether the value of the enumeration applied to input eChoice is within the limits First and Last as defined at E_BA_DateValChoice [} 11].

##### `F_BA_IsLoggingTypeValid` (FUNCTION) → `BOOL` — §4.3.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eType` | `E_BA_LoggingType [} 16]` | Enumeration to be checked. |

**Description:** The function F_BA_IsLoggingTypeValid of return type BOOL checks whether the value of the enumeration applied to input eType is within the limits First and Last as defined under E_BA_LoggingType [} 16]. Syntax

##### `F_BA_IsUnitValid` (FUNCTION) → `BOOL` — §4.3.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eUnit` | `E_BA_Unit [} 17]` | Enumeration to be checked. |

**Description:** The function F_BA_IsUnitValid of return type BOOL checks whether the value of the enumeration applied to the input eUnit is within the limits First and Last as defined at E_BA_Unit [} 17].

##### `F_BA_IsMeasuringElementValid` (FUNCTION) → `BOOL` — §4.3.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eElement` | `E_BA_MeasuringEle` | Enumeration to be checked. ment [} 29] |

**Description:** The F_BA_IsMeasuringElementValid function of return type BOOL checks whether the value of the enumeration applied to input eElement is within the limits First and Last as defined at E_BA_MeasuringElement [} 29].

##### `F_BA_IsDataClassValid` (FUNCTION) → `BOOL` — §4.3.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eClass` | `E_BA_DataClass [} 14]` | Enumeration to be checked. |

**Description:** The F_BA_IsDataClassValid function of return type BOOL checks whether the value of the enumeration applied to input eClass is within the limits First and Last as defined at E_BA_DataClass [} 14].

##### `F_BA_IsDataTypeValid` (FUNCTION) → `BOOL` — §4.3.1.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eType` | `E_BA_DataType [} 15]` | Enumeration to be checked. |

**Description:** The F_BA_IsDataTypeValid function of return type BOOL checks whether the value of the enumeration applied to input eType is within the limits First and Last as defined at E_BA_DataType [} 15].

##### `F_BA_IsWeekdayValid` (FUNCTION) → `BOOL` — §4.3.1.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eDay` | `E_BA_Weekday [} 14]` | Enumeration to be checked. |

**Description:** The F_BA_IsWeekdayValid function of return type BOOL checks whether the value of the enumeration applied to input eDay is within the limits First and Last as defined at E_BA_Weekday [} 14].

#### (top level)

##### `Anti-Reset-Windup Controller enabled - bEn see Anti-Reset-Windup` — §5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable of the controller. |
| `fW` | `REAL` | Setpoint of the controlled system. |
| `fX` | `REAL` | Actual value of the controlled system. |
| `eActn` | `E_BA_Action [} 29]` | Control direction of the controller. |
| `fYMax` | `REAL` | Upper output limit of the controller [%]. |
| `fYMin` | `REAL` | Lower output limit of the controller [%]. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fY` | `REAL` | Control value. Range limited by fYMin and fYMax. |
| `fE` | `REAL` | Control deviation (The calculation depends on the control |

**Description:** Neutral zone

#### Programming › POUs › FunctionBlocks › Universal › I/O

##### `FB_BA_KL32xx` (FUNCTION_BLOCK) — §4.3.2.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConfigurate` | `BOOL` | A rising edge starts the configuration of the bus terminal. |
| `bReadConfig` | `BOOL` | A rising edge starts the reading of the bus terminal. |
| `eSensor` | `E_BA_MeasuringEle Selection of the sensor type. ment Programming` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nRawState` | `USINT` | — |
| `nRawDataIn` | `INT` | — |
| `nRawCtrl` | `USINT` | — |
| `nRawDataOut` | `INT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nState` | `USINT` | Output of the present terminal status. |
| `nData` | `INT` | Output of the present process data. |
| `fVal` | `REAL` | Scaled output value. |
| `bErr` | `BOOL` | Error in the terminal configuration. |
| `bWireBreak` | `BOOL` | A TRUE indicates a wire break at the sensor. |
| `bShortCircuit` | `BOOL` | A TRUE indicates a short circuit at the sensor. Inputs/outputs |
| `nRawState` | `USINT` | Linking with the corresponding status byte of the bus terminal in |

**Description:** The function block FB_BA_KL32xx is for the configuration of Bus Terminals of the types KL3208_0010, Kl3201, KL3202 and Kl3204.

#### Programming › POUs › FunctionBlocks › Universal › Trigger

##### `FB_BA_ATrigCOV` (FUNCTION_BLOCK) — §4.3.2.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `xValue` | `ANY` | Value to be monitored. It must not exceed 4 bytes. |
| `bForce` | `BOOL` | A rising edge at this input forces a trigger pulse at the output. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bReady` | `BOOL` | Switches to TRUE if the variable applied to xValue does not |

**Description:** The function block FB_BA_ATrigCOV is used to detect a value change of a variable xValue of any type.

##### `FB_BA_RFTrig` (FUNCTION_BLOCK) — §4.3.2.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bValue` | `BOOL` | Value to be monitored. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Q` | `BOOL` | TRUE for one cycle if a rising or falling edge was detected |

**Description:** The function block FB_BA_RFTrig is used to detect a rising or falling edge on a boolean variable.

#### Programming › POUs › FunctionBlocks › Universal

##### `Ramps filters` (FUNCTION_BLOCK) — §4.3.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fIn` | `REAL` | Input signal |
| `nDampConst` | `UDINT` | Filter time constant [s]. Internally limited to values between |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `REAL` | Filtered output signal |

**Description:** The function block FB_BA_FltrPT1 is a first order filter.

#### Programming › POUs › FunctionBlocks › Universal › Ramps filters

##### `FB_BA_RampLmt` (FUNCTION_BLOCK) — §4.3.2.2.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | Enable function block if FALSE, then fOut = 0.0. |
| `bEnRamp` | `BOOL` | Enable ramp limitation, if FALSE, then fOut = fIn. |
| `fIn` | `REAL` | Input value of the ramp function Programming |
| `fHi` | `REAL` | Upper interpolation point for calculating the ramps |
| `fLo` | `REAL` | Lower interpolation point for calculating the ramps. fHi |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fOut` | `REAL` | Output signal, slope-limited through the ramps. |

**Description:** The function block FB_BA_RampLmt limits the increase or decrease speed of an input signal. When fIn rises, the output fOut is limited to the slope of (fHi-fLo)/nTiUp. When fIn falls, the output fOut is limited to the slope of (fHi-fLo)/nTiDwn.

#### Programming › POUs › FunctionBlocks › Universal › Hysteresis 2-Point-Control

##### `FB_BA_Swi2P` (FUNCTION_BLOCK) — §4.3.2.2.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the function block. |
| `fIn` | `REAL` | Input value |
| `fOn` | `REAL` | Switch-on point |
| `fOff` | `REAL` | Switch-off point |
| `nDlyOn` | `UDINT` | Start-up delay [s] |
| `nDlyOff` | `UDINT` | Switch-off delay [s] |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Calculated output value of the characteristic curve. |
| `nRemTiDlyOn` | `UDINT` | Remaining time of the start-up delay [s]. |
| `nRemTiDlyOff` | `UDINT` | Remaining time of the switch-off delay [s]. |

**Description:** The function block FB_BA_Swi2P is a two-point switch with one switch-on point and one switch-off point.

##### `FB_BA_SwiHys2P` — §4.3.2.2.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bEn` | `BOOL` | General enable of the function block. |
| `fIn` | `REAL` | Input value |
| `fSp` | `REAL` | Setpoint input |
| `fHys` | `REAL` | Hysteresis |
| `fHysOffs` | `REAL` | Hysteresis offset |
| `nDlyOn` | `UDINT` | Start-up delay [s] |
| `nDlyOff` | `UDINT` | Switch-off delay [s] |
| `bActn` | `BOOL` | Control direction |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bQ` | `BOOL` | Output |
| `fSwiHi` | `REAL` | Upper switching point |
| `fSwiLow` | `REAL` | Lower switching point |
| `nRemTiDlyOn` | `UDINT` | Remaining time of the start-up delay [s]. |
| `nRemTiDlyOff` | `UDINT` | Remaining time of the switch-off delay [s]. |

**Description:** The function block FB_BA_SwiHys2P is a two-point switch with adjustable hysteresis and hysteresis offset.

#### Programming › POUs › FunctionBlocks › System › Persistent Data

##### `FB_BA_PersistentDataHandler` (FUNCTION_BLOCK) — §4.3.2.3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bStt` | `BOOL` | A rising edge at this input starts the function block if it is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | The function block is being executed. |
| `nRemTiInitSttDly` | `UDINT` | Countdown of the set start-up phase. |
| `bErr` | `BOOL` | This output is switched to TRUE if the parameters entered |

**Description:** Programming
