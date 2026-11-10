# Tc3_Module

- **Manual:** `TwinCAT_3_PLC_Lib_Tc3_Module_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Standard · **Tier:** T1 · **API entries:** 33
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Module/config access needs a device/module description model in the package manifest.

---

#### Function blocks

##### `TcBaseModuleRegistered` (FUNCTION_BLOCK) — §3.1

**Description:** VAR

#### Function blocks › TcBaseModuleRegistered

##### `TcAddRef` — §3.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcAddRef` | `UDINT` | The resulting reference count value is returned. |

**Description:** The TcAddRef() method increments the reference counter and returns the new value.

##### `TcGetObjectId` — §3.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `objId` | `REFERENCE TO OTCID` | Reference to OTCID value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcGetObjectId` | `HRESULT` | Gives information about success of the OTCID query. |

**Description:** The method TcGetObjectId saves the object ID with the help of the given OTCID reference.

##### `TcGetObjectName` — §3.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `objName` | `POINTER TO SI The name to be set NT` | — |
| `nameLen` | `UDINT 10` | The maximum length of the name to be written Function blocks |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcGetObjectName` | `DINT` | Gives information about success of the name query. |

**Description:** The method TcGetObjectName saves the object names in the buffer with the given length.

##### `TcGetObjPara` — §3.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pid` | `PTCID` | Parameter-ID of the object parameter |
| `nData` | `REFERENCE TO UDINT` | Maximum length of the data |
| `pData` | `REFERENCE TO PVOID` | Pointer to the data |
| `Pgp` | `PTCGP` | Reserved for future expansion. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcGetObjP HRESULT` | `Gives information about success of the object parameter query. ara` | — |

**Description:** The TcGetObjPara method queries an object parameter identified by means of its PTCID.

##### `TcGetObjState` — §3.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pState` | `POINTER TO TCOM_STA Pointer to the state TE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `TcGetObjState` | `HRESULT` | Gives information about success of the state query. |

**Description:** The TcGetObjState method queries the current state of the object.

##### `TcQueryInterface` — §3.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iid` | `REFERENCE TO IID Interface ID` | — |
| `pipItf` | `POINTER TO PVOID Pointer to interface pointer. Is set when the requested` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcQueryInterface` | `HRESULT` | Informs about success of the interface query. |

**Description:** The method queries the reference at an implemented interface over the ID.

##### `TcRelease` — §3.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcRelease` | `UDINT` | The resulting reference count value is returned. |

**Description:** The TcRelease() method decrements the reference counter and returns the new value. If the reference counter is 0, the object deletes itself.

##### `TcSetObjId` — §3.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `objId` | `OTCID` | The OTCID to be set |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcSetObjId` | `HRESULT` | Gives information about success of the ID change. |

**Description:** The TcSetObjectId method sets the object ID of the object to the given OTCID.

##### `TcSetObjectName` — §3.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `objName` | `POINTER TO SINT` | The name to be set of the object |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcSetObjectNam HRESULT` | `Gives information about the success of the name change. e` | — |

**Description:** The TcSetObjectName method sets the object name of the object.

##### `TcSetObjPara` — §3.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `14 Function blocks` | `` | — |
| `pid` | `PTCID` | Parameter-ID of the object parameter |
| `nData` | `UDINT` | Maximum length of the data |
| `pData` | `PVOID` | Pointer to the data |
| `pgp` | `PTCGPkl` | Reserved for future expansion, pass NULL. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcSetObjPara` | `HRESULT` | Gives information about success of the parameter |

**Description:** The TcSetObjPara method sets an object parameter identified by means of its PTCID.

##### `TcSetObjState` (FUNCTION_BLOCK) — §3.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `state` | `TCOM_STATE` | Displays the new state. |
| `ipSrv` | `ITComObjServer` | Object description |
| `pInitData` | `POINTER TO TComInitDataH Points to a list of parameters (optional). dr` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcSetObjState` | `HRESULT` | Gives information about success of the state change. |

**Description:** The TcSetObjState method initializes a transition to the given state.

#### Function blocks

##### `TcAddRef` — §3.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcAddRef` | `UDINT` | The resulting reference count value is returned. |

**Description:** The TcAddRef() method increments the reference counter and returns the new value.

##### `TcGetObjectId` — §3.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `objId` | `REFERENCE TO OTCID` | Reference to OTCID value |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcGetObjectId` | `HRESULT` | Gives information about success of the OTCID query. |

**Description:** The method TcGetObjectId saves the object ID with the help of the given OTCID reference.

##### `TcGetObjectName` — §3.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `objName` | `POINTER TO SI The name to be set NT` | — |
| `nameLen` | `UDINT` | The maximum length of the name to be written |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcGetObjectName` | `DINT` | Gives information about success of the name query. |

**Description:** The method TcGetObjectName saves the object names in the buffer with the given length.

##### `TcGetObjPara` — §3.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pid` | `PTCID` | Parameter-ID of the object parameter |
| `nData` | `REFERENCE TO UDINT` | Maximum length of the data |
| `pData` | `REFERENCE TO PVOID` | Pointer to the data |
| `Pgp` | `PTCGP` | Reserved for future expansion. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcGetObjP HRESULT` | `Gives information about success of the object parameter query. ara` | — |

**Description:** The TcGetObjPara method queries an object parameter identified by means of its PTCID.

##### `TcGetObjState` — §3.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pState` | `POINTER TO TCOM_STA Pointer to the state TE` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcGetObjState` | `HRESULT 18` | Gives information about success of the state query. Function blocks |

**Description:** The TcGetObjState method queries the current state of the object.

##### `TcQueryInterface` — §3.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `iid` | `REFERENCE TO IID Interface ID` | — |
| `pipItf` | `POINTER TO PVOID Pointer to interface pointer. Is set when the requested` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcQueryInterface` | `HRESULT` | Informs about success of the interface query. |

**Description:** The method queries the reference at an implemented interface over the ID.

##### `TcRelease` — §3.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `(*none*)` | `` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcRelease` | `UDINT` | The resulting reference count value is returned. |

**Description:** The TcRelease() method decrements the reference counter and returns the new value. If the reference counter is 0, the object deletes itself.

##### `TcSetObjId` — §3.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `objId` | `OTCID` | The OTCID to be set |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcSetObjId` | `HRESULT` | Gives information about success of the ID change. |

**Description:** The TcSetObjectId method sets the object ID of the object to the given OTCID.

##### `TcSetObjectName` — §3.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `objName` | `POINTER TO SINT` | The name to be set of the object |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcSetObjectNam HRESULT` | `Gives information about the success of the name change. e` | — |

**Description:** The TcSetObjectName method sets the object name of the object.

##### `TcSetObjPara` — §3.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Function blocks` | `` | — |
| `pid` | `PTCID` | Parameter-ID of the object parameter |
| `nData` | `UDINT` | Maximum length of the data |
| `pData` | `PVOID` | Pointer to the data |
| `pgp` | `PTCGPkl` | Reserved for future expansion, pass NULL. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcSetObjPara` | `HRESULT` | Gives information about success of the parameter |

**Description:** The TcSetObjPara method sets an object parameter identified by means of its PTCID.

##### `TcSetObjState` — §3.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `state` | `TCOM_STATE` | Displays the new state. |
| `ipSrv` | `ITComObjServer` | Object description |
| `pInitData` | `POINTER TO TComInitDataH Points to a list of parameters (optional). dr` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TcSetObjState` | `HRESULT` | Gives information about success of the state change. |

**Description:** The TcSetObjState method initializes a transition to the given state.

#### Functions

##### `FW_ObjMgr_CreateAndInitInstance` — §4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Functions` | `` | — |
| `clsId` | `CLSID` | Specifies the class from which an object should be |

**Description:** This function generates an instance of the class specified by means of Class-ID and at the same time returns an interface pointer to this object. In addition the object name and state into which the object is to be put can be specified.

##### `FW_ObjMgr_CreateInstance` — §4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `24 Functions` | `` | — |
| `clsId` | `CLSID` | Specifies the class from which an object should be |

**Description:** This function generates an instance of the class specified by means of Class-ID and at the same time returns an interface pointer to this object.

##### `FW_ObjMgr_DeleteInstance` — §4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pipUnk` | `POINTER TO ITcUnknown` | Specifies the address of the interface pointer to the |

**Description:** This function puts the object in the Init state. After that the reference counter of the object is decremented, analogous to ITcUnknown.TcRelease(), and the interface pointer is set to zero at the same time.

##### `FW_ObjMgr_GetObjectInstance` — §4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `oid` | `OTCID` | Object ID |
| `iid` | `IID` | Specifies the interface ID to which an interface pointer |

**Description:** This function returns an interface pointer to an object instance specified by means of object ID.

##### `FW_SafeRelease` (METHOD) → `BOOL` — §4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pipUnk` | `POINTER TO ITcUknown` | Specifies the address of the interface pointer to the |

**Description:** This function decrements the reference counter of the object, analogous to ITcUnknown.TcRelease(), and at the same time sets the interface pointer to zero.

##### `FAILED` — §4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hr` | `DINT` | Specification of the error code or status code of type |

**Description:** Error codes or status codes of the type HRESULT are checked with this function for invalidity.

##### `SUCCEEDED` — §4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hr` | `DINT` | Specification of the error code or status code of type |

**Description:** Error codes or status codes of the type HRESULT are checked with this function for validity.

##### `ITCUNKNOWN_TO_PVOID` — §4.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `itcUnknown` | `ITcUknown` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ITCUNKNOWN_TO_PVOID : PVOID` | `` | — |

**Description:** This conversion function converts an interface pointer of the type ITcUnknown to a pointer to VOID.

##### `PVOID_TO_ITCUNKNOWN` — §4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pVoid` | `PVOID` | — |

**Description:** This conversion function converts a pointer to VOID to an interface pointer of the type ITcUnknown.

##### `GuidsEqual` — §4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Functions` | `` | — |
| `pGuidA` | `POINTER TO GUID` | Pointer to GUID object |
| `pGuidB` | `POINTER TO GUID` | Pointer to GUID object |

**Description:** The function GuidsEqual checks two GUID objects for their equality to one another.
