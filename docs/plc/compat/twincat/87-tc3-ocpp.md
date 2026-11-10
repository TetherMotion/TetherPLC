# Tc3_OCPP

- **Manual:** `TF6771_TC3_IoT_OCPP_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 175
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** OCPP EV-charging protocol: WebSocket/TLS client via socket service.

---

#### PLC API › POUs

##### `FB_OCPP1_Client` (FUNCTION) — §5.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL` | The interface to the driver in the |

**Description:** This function block represents an OCPP client that can be connected to an OCPP server. There are two directions of communication.

#### PLC API › POUs › FB_OCPP1_Client

##### `Execute` (METHOD) → `BOOL` — §5.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConnect` | `BOOL` | If a connection exists, FALSE is |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method can be used to establish a connection or to disconnect a connection. No permanent call is required, only changes are transferred from bConnect to the driver.

##### `FB_Exit` (METHOD) → `BOOL` — §5.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInCopyCode` | `BOOL` | If TRUE, the Exit method is called to exit an instance, which is then |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_Exit` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** Syntax

##### `FB_Init` (METHOD) → `BOOL` — §5.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitRetains` | `BOOL` | If TRUE, the Retain variables are |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_Init` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** Syntax

##### `Init` (METHOD) → `BOOL` — §5.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stParam` | `REFERENCE TO` | Parameters for the WebSockets |
| `ST_OCPP1_Client_Param [} 199]` | `connection of the OCPP client. PLC API` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method is called when the function block is initialized in the FB_Init [} 23] method and specifies the parameters of the WebSockets connection. If the connection parameters are to be changed afterwards, this method can be called again while the application is running.

##### `InitBootInfo` (METHOD) → `BOOL` — §5.1.1.5

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stBootInfo` | `ST_OCPP1_Client_BootInfo` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InitBootInfo` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method can be used to set the Boot Notification sent by the OCPP client when an OCPP connection is established. If this method is not called after starting the application, default values are used. More detailed information can be found in ST_OCPP1_Client_BootInfo [} 198].

##### `InitOptions` (METHOD) → `BOOL` — §5.1.1.6

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stOptions` | `ST_OCPP1_Client_Options` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InitOptions` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method can be used to set various options for the OCPP client. If this method is not called after starting the application, default values are used. More detailed information can be found in ST_OCPP1_Client_Options [} 198].

##### `InitSettings` (METHOD) → `BOOL` — §5.1.1.7

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `stSettings` | `ST_OCPP1_Client_Settings` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InitSettings` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method can be used to make various settings for the OCPP client. If this method is not called after starting the application, default values are used. More detailed information can be found in ST_OCPP1_Client_Settings [} 200].

##### `PollRequest` (METHOD) → `BOOL` — §5.1.1.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PollRequest` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eAction` | `E_OCPP1_Action [} 189]` | Type of OCPP request received |

**Description:** This method must be called in order to receive incoming requests from the OCPP server. The outputs of the method return both the message ID of the received message and the information about which OCPP request it is.

##### `RecvCancelReservation` (METHOD) → `BOOL` — §5.1.1.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvCancelReservation` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Cancel Reservation request from the corresponding OCPP server. To respond to the request, the method RespCancelReservation [} 47] must be called.

##### `RecvChangeAvailability` (METHOD) → `BOOL` — §5.1.1.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvChangeAvailability` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Change Availability request from the corresponding OCPP server. To respond to the request, the method RespChangeAvailability [} 48] must be called.

##### `RecvChangeConfiguration` (METHOD) → `BOOL` — §5.1.1.11

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvChangeConfiguration` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Change Configuration request from the corresponding OCPP server. To respond to the request, the method RespChangeConfiguration [} 49] must be called.

##### `RecvClearCache` (METHOD) → `BOOL` — §5.1.1.12

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvClearCache` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Clear Cache request from the corresponding OCPP server. To respond to the request, the method RespClearCache [} 50] must be called.

##### `RecvClearChargingProfile` (METHOD) → `BOOL` — §5.1.1.13

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvClearChargingProfile` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Clear Charging Profile request from the corresponding OCPP server. To respond to the request, the method RespClearChargingProfile [} 51] must be called.

##### `RecvDataTransfer` (METHOD) → `BOOL` — §5.1.1.14

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sData` | `T_OCPP1_DataTransferData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvDataTransfer` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Data Transfer request from the corresponding OCPP server. To respond to the request, the method RespDataTransfer [} 52] must be called.

##### `RecvGetCompositeSchedule` (METHOD) → `BOOL` — §5.1.1.15

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvGetCompositeSchedule` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Get Composite Schedule request from the corresponding OCPP server. To respond to the request, the method RespGetCompositeSchedule [} 53] must be called.

##### `RecvGetConfiguration` (METHOD) → `BOOL` — §5.1.1.16

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvGetConfiguration` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Get Configuration request from the corresponding OCPP server. To respond to the request, the method RespGetConfiguration [} 54] must be called.

##### `RecvGetDiagnostics` (METHOD) → `BOOL` — §5.1.1.17

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvGetDiagnostics` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Get Diagnostics request from the corresponding OCPP server. To respond to the request, the method RespGetDiagnostics [} 55] must be called.

##### `RecvGetLocalListVersion` (METHOD) → `BOOL` — §5.1.1.18

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvGetLocalListVersion` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Get Local List Version request from the corresponding OCPP server. To respond to the request, the method RespGetLocalListVersion [} 56] must be called.

##### `RecvLocalList` (METHOD) → `BOOL` — §5.1.1.19

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aAuthList` | `ARRAY[*] OF ST_OCPP1_AuthorizationData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvLocalList` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Send Local List request from the corresponding OCPP server. To respond to the request, the method RespLocalList [} 57] must be called.

##### `RecvRemoteStartTransaction` (METHOD) → `BOOL` — §5.1.1.20

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvRemoteStartTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Remote Start Transaction request from the corresponding OCPP server. To respond to the request, the method RespRemoteStartTransaction [} 58] must be called.

##### `RecvRemoteStopTransaction` (METHOD) → `BOOL` — §5.1.1.21

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvRemoteStopTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Remote Stop Transaction request from the corresponding OCPP server. To respond to the request, the method RespRemoteStopTransaction [} 59] must be called.

##### `RecvReserveNow` (METHOD) → `BOOL` — §5.1.1.22

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvReserveNow` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Reserve Now request from the corresponding OCPP server. To respond to the request, the method RespReserveNow [} 60] must be called.

##### `RecvReset` (METHOD) → `BOOL` — §5.1.1.23

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvReset` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Reset request from the corresponding OCPP server. To respond to the request, the method RespReset [} 61] must be called.

##### `RecvSetChargingProfile` (METHOD) → `BOOL` — §5.1.1.24

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvSetChargingProfile` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Set Charging Profile request from the corresponding OCPP server. To respond to the request, the method RespSetChargingProfile [} 62] must be called.

##### `RecvTriggerMessage` (METHOD) → `BOOL` — §5.1.1.25

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvTriggerMessage` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives a Trigger Message request from the corresponding OCPP server. To respond to the request, the method RespTriggerMessage [} 63] must be called.

##### `RecvUnlockConnector` (METHOD) → `BOOL` — §5.1.1.26

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvUnlockConnector` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives an Unlock Connector request from the corresponding OCPP server. To respond to the request, the method RespUnlockConnector [} 64] must be called.

##### `RecvUpdateFirmware` (METHOD) → `BOOL` — §5.1.1.27

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvUpdateFirmware` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP client receives an Update Firmware request from the corresponding OCPP server. To respond to the request, the method RespUpdateFirmware [} 64] must be called.

##### `Reset` (METHOD) → `BOOL` — §5.1.1.28

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method is used to reset the last error present on the function block.

##### `RespCancelReservation` (METHOD) → `BOOL` — §5.1.1.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespCancelReservation` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Cancel Reservation request from the corresponding OCPP server.

##### `RespChangeAvailability` (METHOD) → `BOOL` — §5.1.1.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespChangeAvailability` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Change Availability request from the corresponding OCPP server.

##### `RespChangeConfiguration` (METHOD) → `BOOL` — §5.1.1.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespChangeAvailability` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Change Configuration request from the corresponding OCPP server.

##### `RespClearCache` (METHOD) → `BOOL` — §5.1.1.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespClearCache` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Clear Cache request from the corresponding OCPP server.

##### `RespClearChargingProfile` (METHOD) → `BOOL` — §5.1.1.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespClearChargingProfile` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Clear Charging Profile request from the corresponding OCPP server.

##### `RespDataTransfer` (METHOD) → `BOOL` — §5.1.1.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespDataTransfer` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Data Transfer request from the corresponding OCPP server.

##### `RespGetCompositeSchedule` (METHOD) → `BOOL` — §5.1.1.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespGetCompositeSchedule` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Get Composite Schedule request from the corresponding OCPP server.

##### `RespGetConfiguration` (METHOD) → `BOOL` — §5.1.1.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `aKeys` | `ARRAY[*] OF ST_OCPP1_ConfigKeyValue` | — |
| `aUnknown` | `ARRAY[*] OF T_OCPP1_ConfigKey` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespGetConfiguration` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Get Configuration request from the corresponding OCPP server.

##### `RespGetDiagnostics` (METHOD) → `BOOL` — §5.1.1.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespGetDiagnostics` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Get Diagnostics request from the corresponding OCPP server.

##### `RespGetLocalListVersion` (METHOD) → `BOOL` — §5.1.1.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespGetLocalListVersion` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Get Local List Version request from the corresponding OCPP server.

##### `RespLocalList` (METHOD) → `BOOL` — §5.1.1.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespLocalList` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Send Local List request from the corresponding OCPP server.

##### `RespRemoteStartTransaction` (METHOD) → `BOOL` — §5.1.1.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespRemoteStartTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Remote Start Transaction request from the corresponding OCPP server.

##### `RespRemoteStopTransaction` (METHOD) → `BOOL` — §5.1.1.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespRemoteStopTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Remote Stop Transaction request from the corresponding OCPP server.

##### `RespReserveNow` (METHOD) → `BOOL` — §5.1.1.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespReserveNow` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Reserve Now request from the corresponding OCPP server.

##### `RespReset` (METHOD) → `BOOL` — §5.1.1.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespReset` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Reset request from the corresponding OCPP server.

##### `RespSetChargingProfile` (METHOD) → `BOOL` — §5.1.1.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespSetChargingProfile` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Set Charging Profile request from the corresponding OCPP server.

##### `RespTriggerMessage` (METHOD) → `BOOL` — §5.1.1.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespTriggerMessage` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to a Trigger Message request from the corresponding OCPP server.

##### `RespUnlockConnector` (METHOD) → `BOOL` — §5.1.1.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespUnlockConnector` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to an Unlock Connector request from the corresponding OCPP server.

##### `RespUpdateFirmware` (METHOD) → `BOOL` — §5.1.1.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespUpdateFirmware` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client responds to an Update Firmware request from the corresponding OCPP server.

##### `SendAuthorize` (METHOD) → `BOOL` — §5.1.1.48

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIdTag` | `T_OCPP1_IdToken [} 209]` | ID token with which the Charge Point wants to be authorized on the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendAuthorize` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_AuthorizationStatus` | Status of the authorization as a |
| `[} 190]` | `response from the Central System.` | — |

**Description:** With this method, an OCPP client sends an Authorize request to the corresponding OCPP server. The response from the OCPP server is processed directly within the method.

##### `SendDataTransfer` (METHOD) → `BOOL` — §5.1.1.49

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sVendorId` | `STRING(255)` | Identifier for the vendor, which identifies the vendor-specific |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sData` | `T_OCPP1_DataTransferData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendDataTransfer` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_DataTransferStatus` | Status of the Data Transfer as a |
| `[} 193]` | `response from the Central System.` | — |

**Description:** With this method, an OCPP client sends a Data Transfer request to the corresponding OCPP server. The response from the OCPP server is processed directly within the method.

##### `SendDiagnosticsStatusNotification` (METHOD) → `BOOL` — §5.1.1.50

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eStatus` | `E_OCPP1_DiagnosticsStatus [} 193] Contains the status of the upload of` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendDiagnosticsStatusNotification BOOL` | `The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed` | — |

**Description:** With this method, an OCPP client sends a Diagnostics Status Notification to the corresponding OCPP server. The response from the OCPP server is processed directly within the method.

##### `SendFirmwareStatusNotification` (METHOD) → `BOOL` — §5.1.1.51

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eStatus` | `E_OCPP1_FirmwareStatus [} 193]` | Contains the progress of the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendFirmwareStatusNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client sends a Firmware Status Notification to the corresponding OCPP server. The response from the OCPP server is processed directly within the method.

##### `SendHeartbeat` (METHOD) → `BOOL` — §5.1.1.52

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendHeartbeat` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client sends a Heartbeat to the corresponding OCPP server. The client already sends a Heartbeat internally, which can be configured via the HeartbeatInterval Property. This method would configure another Heartbeat in parallel if necessary.

##### `SendMeterValue` (METHOD) → `BOOL` — §5.1.1.53

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrSampleValues` | `ARRAY[*] OF ST_OCPP1_SampledValue` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendMeterValue` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client sends Meter Values to the corresponding OCPP server. The response from the OCPP server is processed directly within the method.

##### `SendSecurityEventNotification` (METHOD) → `BOOL` — §5.1.1.54

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTimestamp` | `ULINT` | Date and time when the Security |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendDiagnosticsStatusNotification BOOL` | `The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed` | — |

**Description:** With this method, an OCPP client sends a Security Event Notification to the corresponding OCPP server. The response from the OCPP server is processed directly within the method.

##### `SendStartTransaction` (METHOD) → `BOOL` — §5.1.1.55

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIdTag` | `T_OCPP1_IdToken [} 209]` | ID token with which the transaction |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendStartTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_AuthorizationStatus` | Status of the authorization in |
| `[} 190]` | `response from the OCPP server.` | — |

**Description:** With this method, an OCPP client sends a Start Transaction request to the corresponding OCPP server. The response from the OCPP server is processed directly within the method. The Transaction ID is managed internally so that the user does not have to pay attention to the administration.

##### `SendStatusNotification` (METHOD) → `BOOL` — §5.1.1.56

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendStatusNotificationEx` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP client sends a Status Notification to the corresponding OCPP server. The response from the OCPP server is processed directly within the method, but in this case it does not contain any separate variables anyway. The timestamp is set internally when the request is sent and cannot be set manually when the method is called.

##### `SendStopTransaction` (METHOD) → `BOOL` — §5.1.1.57

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIdTag` | `T_OCPP1_IdToken [} 209]` | ID token for which the transaction |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendStopTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_AuthorizationStatus` | Status of the authorization in |
| `[} 190]` | `response from the OCPP server.` | — |

**Description:** With this method, an OCPP client sends a Stop Transaction request to the corresponding OCPP server. The response from the OCPP server is processed directly within the method.

#### PLC API › POUs

##### `FB_OCPP1_Server` (FUNCTION) — §5.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL` | The interface to the driver in the |

**Description:** This function block represents an OCPP server to which any number of OCPP clients can be connected. An OCPP client is identified via an ID. If a 1-to-1 relationship between OCPP client and OCPP server is required, the function block FB_OCPP1_Station [} 131] can be used instead of this function block.

#### PLC API › POUs › FB_OCPP1_Server

##### `CreateStation` (METHOD) → `HRESULT` — §5.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIdent` | `T_OCPP_Identity [} 209]` | Identity of the OCPP client to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CreateStation` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**Description:** This method is used to create a new station in the OCPP server.

##### `FB_Exit` (METHOD) → `BOOL` — §5.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInCopyCode` | `BOOL` | If TRUE, the Exit method is called to exit an instance, which is then |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_Exit` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** Syntax

##### `FB_Init` (METHOD) → `BOOL` — §5.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitRetains` | `BOOL` | If TRUE, the Retain variables are |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_Init` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** Syntax

##### `GetStationId` (METHOD) → `BOOL` — §5.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIdent` | `T_OCPP_Identity [} 209]` | Identity of the OCPP client. Inputs/outputs |
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetStationId` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method can be used to query the StationId of an OCPP client in the instance of the OCPP server.

##### `Init` (METHOD) → `BOOL` — §5.1.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `stParam` | `REFERENCE TO` | Connection parameters for the ST_OCPP1_ServerParam [} 201] OCPP server. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method is called when the function block is initialized in the FB_Init [} 23] method and specifies the parameters of the WebSockets server. If the connection parameters are to be changed afterwards, this method can be called again while the application is running.

##### `PollRequest` (METHOD) → `BOOL` — §5.1.2.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PollRequest` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**Description:** This method must be called in order to receive incoming requests from OCPP clients.

##### `RecvAuthorize` (METHOD) → `BOOL` — §5.1.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvAuthorize` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives an Authorize request from an OCPP client. To respond to the request, the method RespAuthorize [} 96] must be called.

##### `RecvBootNotification` (METHOD) → `BOOL` — §5.1.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvBootNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Boot Notification from an OCPP client. To respond to the Boot Notification, the method RespBootNotification [} 97] must be called.

##### `RecvDataTransfer` (METHOD) → `BOOL` — §5.1.2.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sData` | `T_OCPP1_DataTransferData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvDataTransfer` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Data Transfer request from an OCPP client. To respond to the request, the method RespDataTransfer [} 98] must be called.

##### `RecvDiagnosticsStatusNotification` (METHOD) → `BOOL` — §5.1.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvDiagnosticsStatusNotification BOOL` | `The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed` | — |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Diagnostics Status Notification from an OCPP client. To respond to the Diagnostics Status Notification, the method RespDiagnosticsStatusNotification [} 99] must be called.

##### `RecvFirmwareStatusNotification` (METHOD) → `BOOL` — §5.1.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvFirmwareStatusNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Firmware Status Notification from an OCPP client. To respond to the Firmware Status Notification, the method RespFirmwareStatusNotification [} 101] must be called.

##### `RecvHeartbeat` (METHOD) → `BOOL` — §5.1.2.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvHeartbeat` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Heartbeat from an OCPP client. To respond to the Heartbeat, the method RespHeartbeat [} 102] must be called.

##### `RecvMeterValue` (METHOD) → `BOOL` — §5.1.2.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrSampleValues` | `ARRAY[*] OF ST_OCPP1_SampledValue` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvMeterValue` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives Meter Values from an OCPP client. To respond to the receipt of the Meter Values, the method RespMeterValue [} 103] must be called.

##### `RecvSecurityEventNotification` (METHOD) → `BOOL` — §5.1.2.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvSecurityEventNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Security Event Notification from an OCPP client. To respond to the Security Event Notification, the method RespSecurityEventNotification [} 104] must be called.

##### `RecvStartTransaction` (METHOD) → `BOOL` — §5.1.2.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvStartTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Start Transaction request from an OCPP client. To respond to the request, the method RespStartTransaction [} 105] must be called.

##### `RecvStatusNotification` (METHOD) → `BOOL` — §5.1.2.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvStatusNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Status Notification from an OCPP client. To respond to the Status Notification, the method RespStatusNotification [} 106] must be called.

##### `RecvStopTransaction` (METHOD) → `BOOL` — §5.1.2.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvStopTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Stop Transaction request from an OCPP client. To respond to the request, the method RespStopTransaction [} 106] must be called.

##### `ReleaseStation` (METHOD) → `HRESULT` — §5.1.2.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIdent` | `T_OCPP_Identity [} 209]` | Identity of the OCPP client to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReleaseStation` | `HRESULT` | — |
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**Description:** This method is used to remove a connected station from the OCPP server.

##### `Reset` (METHOD) → `BOOL` — §5.1.2.19

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method is used to reset the last error present on the function block.

##### `RespAuthorize` (METHOD) → `BOOL` — §5.1.2.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespAuthorize` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to an Authorize request from an OCPP client.

##### `RespBootNotification` (METHOD) → `BOOL` — §5.1.2.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespBootNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Boot Notification from an OCPP client.

##### `RespDataTransfer` (METHOD) → `BOOL` — §5.1.2.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespDataTransfer` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Data Transfer request from an OCPP client.

##### `RespDiagnosticsStatusNotification` (METHOD) → `BOOL` — §5.1.2.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespDiagnosticsStatusNotification BOOL` | `The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed` | — |

**Description:** With this method, an OCPP server responds to a Diagnostics Status Notification from an OCPP client.

##### `RespError` (METHOD) → `BOOL` — §5.1.2.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespError` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to an OCPP client if the sent request has triggered an internal error. This may be the case, for example, if a requested action is not available.

##### `RespFirmwareStatusNotification` (METHOD) → `BOOL` — §5.1.2.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespFirmwareStatusNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Firmware Status Notification from an OCPP client.

##### `RespHeartbeat` (METHOD) → `BOOL` — §5.1.2.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespHeartbeat` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Heartbeat from an OCPP client. The timestamp is set internally and does not need to be set separately in the PLC.

##### `RespMeterValue` (METHOD) → `BOOL` — §5.1.2.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespMeterValue` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to the sending of Meter Values from an OCPP client.

##### `RespSecurityEventNotification` (METHOD) → `BOOL` — §5.1.2.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespSecurityEventNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Security Event Notification from an OCPP client.

##### `RespStartTransaction` (METHOD) → `BOOL` — §5.1.2.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespStartTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Start Transaction request from an OCPP client.

##### `RespStatusNotification` (METHOD) → `BOOL` — §5.1.2.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespStatusNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Status Notification from an OCPP client.

##### `RespStopTransaction` (METHOD) → `BOOL` — §5.1.2.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespStopTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Stop Transaction request from an OCPP client.

##### `SendCancelReservation` (METHOD) → `BOOL` — §5.1.2.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendCancelReservation` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_CancelReservationStatus The status shows whether the` | — |
| `[} 190]` | `reservation was successfully` | — |

**Description:** With this method, an OCPP server sends a Cancel Reservation request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendChangeAvailability` (METHOD) → `BOOL` — §5.1.2.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendChangeAvailability` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_AvailabilityStatus [} 190] Response whether the Charge Point was able to implement the` | — |

**Description:** With this method, an OCPP server sends a Change Availability request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendChangeConfiguration` (METHOD) → `BOOL` — §5.1.2.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendChangeConfiguration` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ConfigurationStatus` | The status indicates whether the |
| `[} 192]` | `configuration change has been` | — |

**Description:** With this method, an OCPP server sends a Change Configuration request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendChargingProfile` (METHOD) → `BOOL` — §5.1.2.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendChargingProfile` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ChargingProfileStatus` | The status indicates whether the |
| `[} 191]` | `Charging Profile has been` | — |

**Description:** With this method, an OCPP server sends a Charging Profile to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendClearCache` (METHOD) → `BOOL` — §5.1.2.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendClearCache` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ClearCacheStatus [} 192] The status indicates whether the clearing of the Cache Profile has been accepted by the Charge` | — |

**Description:** With this method, an OCPP server sends a Clear Cache request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendClearChargingProfile` (METHOD) → `BOOL` — §5.1.2.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendClearChargingProfile` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ClearChargingProfileStatu The status indicates whether the` | — |
| `s [} 192]` | `Charging Profile could be cleared` | — |

**Description:** With this method, an OCPP server sends a Clear Charging Profile request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendDataTransfer` (METHOD) → `BOOL` — §5.1.2.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sData` | `T_OCPP1_DataTransferData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendDataTransfer` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_DataTransferStatus [} 193]` | Status of the Data Transfer. |

**Description:** With this method, an OCPP server sends a Data Transfer request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendGetCompositeSchedule` (METHOD) → `BOOL` — §5.1.2.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendGetCompositeSchedule` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_GetCompositeScheduleSt The status shows whether the atus [} 194]` | schedule request could be |

**Description:** With this method, an OCPP server sends a Get Composite Schedule request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendGetConfiguration` (METHOD) → `BOOL` — §5.1.2.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrKeys` | `ARRAY[*] OF ST_OCPP1_ConfigKeyValue` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendGetConfiguration` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `nKeysCount` | `UDINT` | Number of the following |

**Description:** With this method, an OCPP server sends a Get Configuration request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendGetDiagnostics` (METHOD) → `BOOL` — §5.1.2.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendGetDiagnostics` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `sFileName` | `STRING(255)` | Contains the name of the |

**Description:** With this method, an OCPP server sends a Get Diagnostics request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendGetLocalListVersion` (METHOD) → `BOOL` — §5.1.2.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendGetLocalListVersion` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `nListVersion` | `UDINT` | Contains the version number of the Local Authorization List currently |

**Description:** With this method, an OCPP server sends a Get Local List Version request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendLocalList` (METHOD) → `BOOL` — §5.1.2.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrAuthList` | `ARRAY[*] OF ST_OCPP1_AuthorizationData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendLocalList` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_UpdateStatus [} 197]` | The status indicates whether the Charge Point has received and carried out the Local Authorization |

**Description:** With this method, an OCPP server sends a Local List request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendRemoteStartTransaction` (METHOD) → `BOOL` — §5.1.2.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendRemoteStartTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_RemoteStartStopStatus` | The status shows whether the |
| `[} 196]` | `Charge Point has accepted the` | — |

**Description:** With this method, an OCPP server sends a Remote Start Transaction request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendRemoteStartTransactionProfile` (METHOD) → `BOOL` — §5.1.2.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendRemoteStartTransactionProfil BOOL` | `The method returns the return` | — |
| `e` | `value TRUE if the call was successful. A method call is also considered successfully completed` | — |
| `eStatus` | `E_OCPP1_RemoteStartStopStatus` | The status shows whether the |
| `[} 196]` | `Charge Point has accepted the` | — |

**Description:** With this method, an OCPP server sends a Remote Start Transaction request including a Charging Profile to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendRemoteStopTransaction` (METHOD) → `BOOL` — §5.1.2.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendRemoteStopTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_RemoteStartStopStatus` | The status indicates whether the |
| `[} 196]` | `Charge Point has accepted the` | — |

**Description:** With this method, an OCPP server sends a Remote Stop Transaction request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendReserveNow` (METHOD) → `BOOL` — §5.1.2.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendReserveNow` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ReservationStatus [} 196] The status indicates whether the` | — |

**Description:** With this method, an OCPP server sends a Reserve Now request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendReset` (METHOD) → `BOOL` — §5.1.2.48

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendReset` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ResetStatus [} 196]` | The status shows whether the Charge Point was able to accept |

**Description:** With this method, an OCPP server sends a Reset Request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendTriggerMessage` (METHOD) → `BOOL` — §5.1.2.49

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendTriggerMessage` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_TriggerMessageStatus` | The status indicates whether the |
| `[} 196]` | `Charge Point will send the` | — |

**Description:** With this method, an OCPP server sends a Trigger Message request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendUnlockConnector` (METHOD) → `BOOL` — §5.1.2.50

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendUnlockConnector` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_UnlockStatus [} 197]` | The status shows whether the |

**Description:** With this method, an OCPP server sends an Unlock Connector request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendUpdateFirmware` (METHOD) → `BOOL` — §5.1.2.51

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hStationId` | `UDINT` | Identifier of the OCPP client in the |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendUpdateFirmware` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server sends an Update Firmware request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

#### PLC API › POUs

##### `FB_OCPP1_Station` (FUNCTION) — §5.1.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bValid` | `BOOL` | The interface to the driver in the |

**Description:** Syntax

#### PLC API › POUs › FB_OCPP1_Station

##### `FB_Exit` (METHOD) → `BOOL` — §5.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInCopyCode` | `BOOL` | If TRUE, the Exit method is called to exit an instance, which is then |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_Exit` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** Syntax

##### `FB_Init` (METHOD) → `BOOL` — §5.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bInitRetains` | `BOOL` | If TRUE, the Retain variables are |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FB_Init` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** Syntax

##### `Init` (METHOD) → `BOOL` — §5.1.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sIdent` | `T_OCPP_Identity [} 209]` | Identity of the OCPP client to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Init` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method is called when the function block is initialized in the FB_Init [} 132] method and specifies the parameters of the WebSockets server. If the connection parameters are to be changed afterwards, this method can be called again while the application is running.

##### `PollRequest` (METHOD) → `BOOL` — §5.1.3.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PollRequest` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eAction` | `E_OCPP1_Action [} 189]` | Type of OCPP request received |

**Description:** This method must be called to receive incoming requests from the connected OCPP client.

##### `RecvAuthorize` (METHOD) → `BOOL` — §5.1.3.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvAuthorize` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives an Authorize request from the connected OCPP client. To respond to the request, the RespAuthorize [} 147] method must be called.

##### `RecvBootNotification` (METHOD) → `BOOL` — §5.1.3.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvBootNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Boot Notification from the connected OCPP client. To respond to the boot notification, the RespBootNotification [} 148] method must be called.

##### `RecvDataTransfer` (METHOD) → `BOOL` — §5.1.3.7

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sData` | `T_OCPP1_DataTransferData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvDataTransfer` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Data Transfer request from the connected OCPP client. To respond to the request, the RespDataTransfer [} 149] method must be called.

##### `RecvDiagnosticsStatusNotification` (METHOD) → `BOOL` — §5.1.3.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvDiagnosticsStatusNotification BOOL` | `The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed` | — |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Diagnostics Status Notification from the connected OCPP client. To respond to the Diagnostics Status Notification, the RespDiagnosticsStatusNotification [} 150] method must be called.

##### `RecvFirmwareStatusNotification` (METHOD) → `BOOL` — §5.1.3.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvFirmwareStatusNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Firmware Status Notification from the connected OCPP client. To respond to the Firmware Status Notification, the RespFirmwareStatusNotification [} 151] method must be called.

##### `RecvHeartbeat` (METHOD) → `BOOL` — §5.1.3.10

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvHeartbeat` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Heartbeat from the connected OCPP client. To respond to the Heartbeat, the RespHeartbeat [} 152] method must be called.

##### `RecvMeterValue` (METHOD) → `BOOL` — §5.1.3.11

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrSampleValues` | `ARRAY[*] OF ST_OCPP1_SampledValue` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvMeterValue` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives Meter Values from the connected OCPP client. To respond to the receipt of the Meter Values, the RespMeterValue [} 153] method must be called.

##### `RecvSecurityEventNotification` (METHOD) → `BOOL` — §5.1.3.12

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvSecurityEventNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Security Event Notification from the connected OCPP client. To respond to the Security Event Notification, the RespSecurityEventNotification [} 154] method must be called.

##### `RecvStartTransaction` (METHOD) → `BOOL` — §5.1.3.13

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvStartTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Start Transaction request from the connected OCPP client. To respond to the request, the RespStartTransaction [} 155] method must be called.

##### `RecvStatusNotification` (METHOD) → `BOOL` — §5.1.3.14

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvStatusNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Status Notification from the connected OCPP client. To respond to the Status Notification, the RespStatusNotification [} 156] method must be called.

##### `RecvStopTransaction` (METHOD) → `BOOL` — §5.1.3.15

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvStopTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**Description:** With this method, an OCPP server receives a Stop Transaction request from the connected OCPP client. To respond to the request, the RespStopTransaction [} 157] method must be called.

##### `Reset` (METHOD) → `BOOL` — §5.1.3.16

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Reset` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** This method is used to reset the last error present on the function block.

##### `RespAuthorize` (METHOD) → `BOOL` — §5.1.3.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespAuthorize` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to an Authorize request from the connected OCPP client.

##### `RespBootNotification` (METHOD) → `BOOL` — §5.1.3.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespBootNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Boot Notification from the connected OCPP client.

##### `RespDataTransfer` (METHOD) → `BOOL` — §5.1.3.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespDataTransfer` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Data Transfer request from the connected OCPP client.

##### `RespDiagnosticsStatusNotification` (METHOD) → `BOOL` — §5.1.3.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespDiagnosticsStatusNotification BOOL` | `The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed` | — |

**Description:** With this method, an OCPP server responds to a Status Notification from the connected OCPP client.

##### `RespError` (METHOD) → `BOOL` — §5.1.3.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespError` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to the connected OCPP client if the sent request has triggered an internal error. This may be the case, for example, if a requested action is not available.

##### `RespFirmwareStatusNotification` (METHOD) → `BOOL` — §5.1.3.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespFirmwareStatusNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Firmware Status Notification from the connected OCPP client.

##### `RespHeartbeat` (METHOD) → `BOOL` — §5.1.3.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespHeartbeat` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Heartbeat from the connected OCPP client.

##### `RespMeterValue` (METHOD) → `BOOL` — §5.1.3.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespMeterValue` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to the sending of Meter Values from the connected OCPP client.

##### `RespSecurityEventNotification` (METHOD) → `BOOL` — §5.1.3.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespSecurityEventNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Security Event Notification from the connected OCPP client.

##### `RespStartTransaction` (METHOD) → `BOOL` — §5.1.3.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespStartTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Start Transaction request from the connected OCPP client.

##### `RespStatusNotification` (METHOD) → `BOOL` — §5.1.3.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespStatusNotification` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Status Notification from the connected OCPP client.

##### `RespStopTransaction` (METHOD) → `BOOL` — §5.1.3.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `hMessageId` | `T_OCPP_MessageId [} 209]` | MessageId of the received |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RespStopTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server responds to a Stop Transaction request from the connected OCPP client.

##### `SendCancelReservation` (METHOD) → `BOOL` — §5.1.3.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nReservationId` | `UDINT` | ID of the reservation to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendCancelReservation` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_CancelReservationStatus The status shows whether the` | — |
| `[} 190]` | `reservation was successfully` | — |

**Description:** With this method, an OCPP server sends a Cancel Reservation request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendChangeAvailability` (METHOD) → `BOOL` — §5.1.3.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendChangeAvailability` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_AvailabilityStatus [} 190] Response whether the Charge Point was able to implement the` | — |

**Description:** With this method, an OCPP server sends a Change Availability request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendChangeConfiguration` (METHOD) → `BOOL` — §5.1.3.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sKey` | `T_OCPP1_ConfigKey [} 208]` | The name of the configuration |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendChangeConfiguration` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ConfigurationStatus` | The status indicates whether the |
| `[} 192]` | `configuration change has been` | — |

**Description:** With this method, an OCPP server sends a Change Configuration request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendChargingProfile` (METHOD) → `BOOL` — §5.1.3.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendChargingProfile` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ChargingProfileStatus` | The status indicates whether the |
| `[} 191]` | `Charging Profile has been` | — |

**Description:** With this method, an OCPP server sends a Charging Profile to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendClearCache` (METHOD) → `BOOL` — §5.1.3.33

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendClearCache` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ClearCacheStatus [} 192] The status indicates whether the clearing of the Cache Profile has been accepted by the Charge` | — |

**Description:** With this method, an OCPP server sends a Clear Cache request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendClearChargingProfile` (METHOD) → `BOOL` — §5.1.3.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nProfileId` | `UDINT` | Identifier of the Charging Profile to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendClearChargingProfile` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ClearChargingProfileStatu The status indicates whether the` | — |
| `s [} 192]` | `Charging Profile could be cleared` | — |

**Description:** With this method, an OCPP server sends a Clear Charging Profile request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendDataTransfer` (METHOD) → `BOOL` — §5.1.3.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sVendorId` | `STRING(255)` | Identifier for the vendor, which identifies the vendor-specific |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sData` | `T_OCPP1_DataTransferData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendDataTransfer` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_DataTransferStatus [} 193]` | Status of the Data Transfer. |

**Description:** With this method, an OCPP server sends a Data Transfer request to the corresponding OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendGetCompositeSchedule` (METHOD) → `BOOL` — §5.1.3.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendGetCompositeSchedule` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_GetCompositeScheduleSt The status shows whether the atus [} 194]` | schedule request could be |

**Description:** With this method, an OCPP server sends a Get Composite Schedule request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendGetConfiguration` (METHOD) → `BOOL` — §5.1.3.37

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrKeys` | `ARRAY[*] OF ST_OCPP1_ConfigKeyValue` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendGetConfiguration` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `nKeysCount` | `UDINT` | Number of the following |

**Description:** With this method, an OCPP server sends a Get Configuration request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendGetDiagnostics` (METHOD) → `BOOL` — §5.1.3.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sLocation` | `STRING(255)` | Directory to which the diagnostic |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendGetDiagnostics` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `sFileName` | `STRING(255)` | Contains the name of the |

**Description:** With this method, an OCPP server sends a Get Diagnostics request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendGetLocalListVersion` (METHOD) → `BOOL` — §5.1.3.39

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendGetLocalListVersion` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `nListVersion` | `UDINT` | Contains the version number of the Local Authorization List currently |

**Description:** With this method, an OCPP server sends a Get Local List Version request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendLocalList` (METHOD) → `BOOL` — §5.1.3.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nListVersion` | `UDINT` | Version of the Local Authorization |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `arrAuthList` | `ARRAY[*] OF ST_OCPP1_AuthorizationData` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendLocalList` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_UpdateStatus [} 197]` | The status indicates whether the Charge Point has received and carried out the Local Authorization |

**Description:** With this method, an OCPP server sends a Send Local List request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendRemoteStartTransaction` (METHOD) → `BOOL` — §5.1.3.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendRemoteStartTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_RemoteStartStopStatus` | The status shows whether the |
| `[} 196]` | `Charge Point has accepted the` | — |

**Description:** With this method, an OCPP server sends a Remote Start Transaction request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendRemoteStartTransactionProfile` (METHOD) → `BOOL` — §5.1.3.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendRemoteStartTransactionProfil BOOL` | `The method returns the return` | — |
| `e` | `value TRUE if the call was successful. A method call is also considered successfully completed` | — |
| `eStatus` | `E_OCPP1_RemoteStartStopStatus` | The status shows whether the |
| `[} 196]` | `Charge Point has accepted the` | — |

**Description:** With this method, an OCPP server sends a Remote Start Transaction request including a Charging Profile to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendRemoteStopTransaction` (METHOD) → `BOOL` — §5.1.3.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nTransactionId` | `UDINT` | Identifier of the transaction to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendRemoteStopTransaction` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_RemoteStartStopStatus` | The status indicates whether the |
| `[} 196]` | `Charge Point has accepted the` | — |

**Description:** With this method, an OCPP server sends a Remote Stop Transaction request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendReserveNow` (METHOD) → `BOOL` — §5.1.3.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendReserveNow` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ReservationStatus [} 196] The status indicates whether the` | — |

**Description:** With this method, an OCPP server sends a Reserve Now request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendReset` (METHOD) → `BOOL` — §5.1.3.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eType` | `E_OCPP1_ResetType [} 196]` | Type of reset that the Charge Point |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendReset` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_ResetStatus [} 196]` | The status shows whether the Charge Point was able to accept |

**Description:** With this method, an OCPP server sends a Reset request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendTriggerMessage` (METHOD) → `BOOL` — §5.1.3.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eRequestedMessage` | `E_OCPP1_MessageTrigger [} 194]` | Type of message to be triggered. |
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendTriggerMessage` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_TriggerMessageStatus` | The status indicates whether the |
| `[} 196]` | `Charge Point will send the` | — |

**Description:** With this method, an OCPP server sends a Trigger Message request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendUnlockConnector` (METHOD) → `BOOL` — §5.1.3.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendUnlockConnector` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |
| `eStatus` | `E_OCPP1_UnlockStatus [} 197]` | The status shows whether the |

**Description:** With this method, an OCPP server sends an Unlock Connector request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

##### `SendUpdateFirmware` (METHOD) → `BOOL` — §5.1.3.48

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sLocation` | `STRING(255)` | URI from which the firmware is to |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SendUpdateFirmware` | `BOOL` | The method returns the return value TRUE if the call was successful. A method call is also considered successfully completed |

**Description:** With this method, an OCPP server sends an Update Firmware request to the connected OCPP client. The response from the OCPP client is processed directly within the method.

#### PLC API › POUs

##### `FB_OCPP1_ChargingProfile` (FUNCTION) — §5.1.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `nScheduleTime` | `ULINT` | The current timestamp used to |

**Description:** This function block can be used to query the currently available Charging Profiles.

#### PLC API › POUs › FB_OCPP1_ChargingProfile

##### `Execute` (METHOD) — §5.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `arrCharging` | `ARRAY [1..Param_OCPP` | Per connector within a Charge |
| `[} 209].nConnectorCount] OF` | `Point, the information as to whether` | — |
| `BOOL` | `charging is in progress or not.` | — |
| `fbClient` | `REFERENCE TO FB_OCPP1_Client Instance of the OCPP client. [} 21] PLC API` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | The return value of the method is not currently assigned and always |

**Description:** This method must be called cyclically if the function block should be used. The instance of FB_OCPP1_Client [} 21] must be transferred. In addition, information is required on which connector is currently charging. This allows the starting point of the Charging Schedule to be calculated.

##### `GetCurrentPeriod` (METHOD) — §5.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `mPeriod` | `ST_OCPP1_ChargingSchedulePeriod` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetCurrentPeriod` | `BOOL` | The return value of the method is not currently assigned and always |

**Description:** This method can be used to read the current Charging Schedule Period for a specific Connector of a Charge Point.

##### `GetCurrentSchedule` (METHOD) — §5.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nConnectorId` | `UDINT` | ID of the Connector of a Charge |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `mChargingSchedule` | `ST_OCPP1_ChargingScheduleMax` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetCurrentSchedule` | `BOOL` | The return value of the method is not currently assigned and always |

**Description:** This method can be used to read the current Charging Schedule for a specific Connector of a Charge Point.

#### PLC API › POUs

##### `FB_OCPP1_Configuration` (FUNCTION) — §5.1.5

**Description:** This function block can be used to manage the Configuration Keys of an instance of FB_OCPP1_Client [} 21].

#### PLC API › POUs › FB_OCPP1_Configuration

##### `GetChanged` (METHOD) → `BOOL` — §5.1.5.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChanged` | `BOOL` | The method returns TRUE if the |
| `eKey` | `E_OCPP1_ConfigKey [} 192]` | Config Key for which the last |

**Description:** This method can be used to get the last change to the Config Keys that are part of the enumeration E_OCPP1_ConfigKey [} 192].

##### `GetChangedEx` (METHOD) → `BOOL` — §5.1.5.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetChanged` | `BOOL` | The method returns TRUE if the |
| `sKey` | `T_OCPP1_ConfigKey [} 208]` | Config Key for which the last |

**Description:** This method can be used to get the last change to the Config Keys that are not part of the enumeration E_OCPP1_ConfigKey [} 192].

##### `GetValue` (METHOD) → `T_OCPP1_ConfigValue` — §5.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eKey` | `E_OCPP1_ConfigKey [} 192]` | Config Key for which the current |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetValue` | `T_OCPP1_ConfigValue [} 208]` | Current value of the queried Config |

**Description:** This method can be used to get the current value of a Config Key that is part of the enumeration E_OCPP1_ConfigKey [} 192].

##### `GetValueEx` (METHOD) → `T_OCPP1_ConfigValue` — §5.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sKey` | `T_OCPP1_ConfigKey [} 208]` | Config Key for which the current |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetValue` | `T_OCPP1_ConfigValue [} 208]` | Current value of the queried Config |

**Description:** This method can be used to query the current value of a Config Key.

##### `InitKeyValue` (METHOD) → `UDINT` — §5.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eKey` | `E_OCPP1_ConfigKey [} 192]` | Configuration Key to be overwritten |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InitKeyValue` | `UDINT` | The assigned index of the Config Key. At present, however, this index is only used internally and does not need to be taken into |

**Description:** This method can be used to add new Key/Value pairs for Configuration Keys that are part of the enumeration E_OCPP1_ConfigKey [} 192]. If a Key is transferred that is already known in the OCPP client, the value is overwritten. The method InitKeyValueEx [} 185] can be used to add user-defined Keys.

##### `InitKeyValueEx` (METHOD) → `UDINT` — §5.1.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sKey` | `T_OCPP1_ConfigKey [} 208]` | Configuration Key to be overwritten |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InitKeyValueEx` | `UDINT` | The assigned index of the Config Key. At present, however, this index is only used internally and does not need to be taken into |

**Description:** This method can be used to add new Key/Value pairs for Configuration Keys. If a Key is transferred that is already known in the OCPP client, the value is overwritten. The method can be used to add user-defined Keys that are not part of the enumeration E_OCPP1_ConfigKey [} 192].

##### `InitStandard` (METHOD) → `BOOL` — §5.1.5.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InitStandard` | `BOOL` | The return value of the method is not currently assigned and always |

**Description:** This method is used to set default values for some of the Configuration Keys described in the OCPP specification. The list of Configuration Keys and their default assignment can be found in the appendix [} 211].

##### `SetValue` (METHOD) → `BOOL` — §5.1.5.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eKey` | `E_OCPP1_ConfigKey [} 192]` | Config Key that is to be rewritten. Inputs/outputs |
| `sValue` | `T_OCPP1_ConfigValue [} 208]` | New value for the Config Key to be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sValue` | `T_OCPP1_ConfigValue` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetValue` | `BOOL` | The method returns TRUE if the |

**Description:** This method can be used to write a new value for a Config Key from the enumeration E_OCPP1_ConfigKey [} 192].

##### `SetValueEx` (METHOD) → `BOOL` — §5.1.5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sKey` | `T_OCPP1_ConfigKey [} 208]` | Config Key that is to be rewritten. Inputs/outputs |
| `sValue` | `T_OCPP1_ConfigValue [} 208]` | New value for the Config Key to be |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sValue` | `T_OCPP1_ConfigValue` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetValueEx` | `BOOL` | The method returns TRUE if the |

**Description:** This method can be used to set a new value for a Config Key.

##### `RecvChangeConfiguration` (METHOD) → `BOOL` — §5.1.5.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbClient` | `REFERENCE TO FB_OCPP1_Client` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvChangeConfiguration` | `BOOL` | Not currently implemented. In future versions: if the method call is successful, the method returns |

**Description:** This method is an alternative way of receiving and responding to a Change Configuration Request from the corresponding OCPP server in the OCPP client. In contrast to the RecvChangeConfiguration [} 29] method at FB_OCPP1_Client [} 21], the response is sent automatically to the OCPP server.

##### `RecvGetConfiguration` (METHOD) → `BOOL` — §5.1.5.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbClient` | `REFERENCE TO FB_OCPP1_Client` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RecvGetConfiguration` | `BOOL` | Not currently implemented. In future versions: if the method call is successful, the method returns |

**Description:** This method is an alternative way of receiving and responding to a Get Configuration Request from the corresponding OCPP server in the OCPP client. In contrast to the RecvGetConfiguration [} 34] method at FB_OCPP1_Client [} 21], the response is sent automatically to the OCPP server. Depending on the request, it contains all or specifically requested Configuration Keys.
