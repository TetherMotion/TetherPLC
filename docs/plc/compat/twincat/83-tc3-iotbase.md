# Tc3_IotBase

- **Manual:** `TF6701_TC3_IoT_Communication_MQTT_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 372
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** IoT base: MQTT/HTTP egress via socket service.

---

#### PLC API › Tc3_IotBase

##### `FB_IotMqttClient` (FUNCTION_BLOCK) — §5.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sClientId` | `STRING(255)` | The client ID can be specified individually and must be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError output is set. An |

**Description:** The function block enables communication to a message broker based on MQTT version 3.1.1.

#### PLC API › Tc3_IotBase › FB_IotMqttClient

##### `Execute` (METHOD) — §5.1.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConnect` | `BOOL` | The connection to the broker is established when bConnect is set to TRUE. |

**Description:** This method must be called cyclically in order to ensure the background communication with the MQTT broker.

##### `Publish` (METHOD) → `BOOL` — §5.1.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING` | Topic of the MQTT message |
| `pPayload` | `PVOID` | Address for the payload of the MQTT message nPayloadSize UDINT Size of the payload in bytes |
| `eQoS` | `TcIotMqttQos "Quality of Service"` | — |
| `bRetain` | `BOOL` | If bRetain is TRUE, the broker stores the message in order to make it available |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics are case` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Publish` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once, in order to send a message to the broker.

##### `Subscribe` (METHOD) → `BOOL` — §5.1.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eQoS` | `TcIotMqttQos "Quality of Service" Inputs/outputs` | — |
| `sTopic` | `STRING` | Topic of the MQTT message |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics are case` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Subscribe` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is used by the client to signal to the broker that it wants to receive all MQTT message with a particular topic. The method can subscribe an MQTT client instance to multiple topics.

##### `Unsubscribe` (METHOD) → `BOOL` — §5.1.1.1.4

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics are case` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Unsubscribe` | `BOOL` | The method returns the return value TRUE if the call was successful. Inputs/outputs |
| `sTopic` | `STRING` | Topic for which no further messages should be received. |

**Description:** This method is used by the client to signal to the broker that no further messages with the specified topic should be transferred to it.

##### `OnMqttMessage` (METHOD) → `HRESULT` — §5.1.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `payload` | `PVOID` | Address for the payload of the received MQTT message |
| `length` | `UDINT` | Size of the payload in bytes |
| `qos` | `TcIotMqttQos` | "Quality of Service" |
| `repeated` | `BOOL` | If the user did not respond with S_OK to the last OnMqttMessage() method call, |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `topic` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OnMqttMessage HRESULT` | `The return value of the method should be S_OK, if the message was` | — |

**Description:** Callback method

##### `ActivateExponentialBackoff` (METHOD) — §5.1.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tMqttBackoffMin TIME` | `Describes the initial delay value for the new connection attempt. Time` | — |
| `tMqttBackoffMax TIME` | `Describes the largest delay value. Once this value has been reached, all new` | — |
| `Time` | `connection attempts are made at these intervals.` | — |

**Description:** A feature referred to as "exponential backoff" can be used to avoid burdening the message broker with unnecessary connection requests in case of a connection error. In the event of a TLS connection error involving the message broker, the reconnect rate is adjusted multiplicatively. This function can be activated using the ActivateExponentialBackoff() [} 61] method. The parameters of the method specify the minimum and maximum time for the algorithm. The minimum time describes the initial delay value for the new connection attempt. The maximum time describes the highest delay value. The delay values are doubled until the maximum value is reached. Once a connection has been established, the backoff rate is reset to the original value. The DeactivateExponentialBackoff() [} 62] method can be used to deactivate this function programmatically.

##### `DeactivateExponentialBackoff` (METHOD) — §5.1.1.1.7

**Description:** A feature referred to as "exponential backoff" can be used to avoid burdening the message broker with unnecessary connection requests in case of a connection error. In the event of a TLS connection error involving the message broker, the reconnect rate is adjusted multiplicatively. This function can be activated using the ActivateExponentialBackoff() [} 61] method. The parameters of the method specify the minimum and maximum time for the algorithm. The minimum time describes the initial delay value for the new connection attempt. The maximum time describes the highest delay value. The delay values are doubled until the maximum value is reached. Once a connection has been established, the backoff rate is reset to the original value. The DeactivateExponentialBackoff() [} 62] method can be used to deactivate this function programmatically.

##### `GetTimeSinceLastBrokerMessage` (METHOD) → `UDINT` — §5.1.1.1.8

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetTimeSinc UDINT` | `The time in milliseconds since the last message from the message broker. eLastBroker Message` | — |

**Description:** This method specifies the time (in ms) since the last message from the message broker. The time is reset to 0 with every new message from the message broker. It does not matter whether these are ping messages or regular publish/subscribe messages.

#### PLC API › Tc3_IotBase › FB_IotMqttMessageQueue

##### `Dequeue` (METHOD) → `BOOL` — §5.1.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbMessage` | `REFERENCE TO FB_IotMqttMe For the message itself the reference is transferred to an` | — |
| `ssage` | `instance of type FB_IotMqttMessage [} 66].` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Dequeue` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** The method returns the return value TRUE if the removal of an MQTT message from the queue was successful. The number of MQTT messages currently in the queue (nQueuedMessages property) is reduced by one through the removal of a message.

##### `ResetQueue` (METHOD) → `BOOL` — §5.1.1.4.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ResetQueue BOOL` | `The method returns the return value TRUE if the call was successful.` | — |

**Description:** When this method is called, all accumulated MQTT messages are deleted from the queue.

#### PLC API › Tc3_IotBase › FB_IotMqttMessage

##### `CompareTopic` (METHOD) → `BOOL` — §5.1.1.5.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string with any length (attend that MQTT topics are case sensitive)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CompareTo BOOL` | `Is TRUE, if the specified topic is identical to the topic of the MQTT message in the` | — |
| `pic` | `function block. Inputs/outputs` | — |
| `sTopic` | `STRING` | — |

**Description:** This method returns TRUE, if the specified topic is identical to the topic of the MQTT message in the function block.

##### `GetTopic` (METHOD) → `BOOL` — §5.1.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTopic` | `POINTER TO` | The memory address for the buffer into which the topic is to be copied is |
| `STRING` | `specified here.` | — |
| `nTopicSize` | `UINT` | The maximum available buffer size (in bytes) is specified here. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetTopic` | `BOOL` | — |

**Description:** Syntax

##### `GetPayload` (METHOD) → `BOOL` — §5.1.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPayload` | `PVOID` | The memory address for the buffer into which the payload is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetPayload` | `BOOL PLC API` | — |

**Description:** Syntax

#### PLC API › Tc3_IotBase

##### `FB_IotMqtt5Client` (FUNCTION_BLOCK) — §5.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sClientId` | `STRING(255)` | The client ID can be specified individually and must be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError output is set. An |

**Description:** The function block enables communication to a message broker based on MQTT version 5.0.

#### PLC API › Tc3_IotBase › FB_IotMqtt5Client

##### `Execute` (METHOD) — §5.1.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConnect` | `BOOL` | The connection to the broker is established when bConnect is set to TRUE. |

**Description:** This method must be called cyclically in order to ensure the background communication with the MQTT broker.

##### `Publish` (METHOD) → `BOOL` — §5.1.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPayload` | `PVOID` | Address for the payload of the MQTT message nPayloadSize UDINT Size of the payload in bytes |
| `eQoS` | `TcIotMqttQos "Quality of Service"` | — |
| `bRetain` | `BOOL` | If bRetain is TRUE, the broker stores the message in order to make it available |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics are case` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Publish` | `BOOL PLC API` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once, in order to send a message to the broker.

##### `Subscribe` (METHOD) → `BOOL` — §5.1.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eQoS` | `TcIotMqttQos` | "Quality of Service" |
| `pProps` | `POINTER TO` | Pointer to Subscribe properties to be sent. |
| `MqttSubscribeProperties` | `The parameter is optional. The` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics are case se` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Subscribe` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is used by the client to signal to the broker that it wants to receive all MQTT message with a particular topic. The method can subscribe an MQTT client instance to multiple topics.

##### `Unsubscribe` (METHOD) → `BOOL` — §5.1.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pProps` | `POINTER TO` | Pointer to Unsubscribe properties to be sent. |
| `MqttUnsubscribeProperties` | `The parameter is optional. The` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics are case se` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Unsubscribe` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is used by the client to signal to the broker that no further messages with the specified topic should be transferred to it.

##### `ActivateExponentialBackoff` (METHOD) — §5.1.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tMqttBackoff TIME` | `Describes the initial delay value for the new connection attempt. MinTime` | — |
| `tMqttBackoff TIME` | `Describes the largest delay value. Once this value has been reached, all new` | — |
| `MaxTime` | `connection attempts are made at these intervals.` | — |

**Description:** A feature referred to as "exponential backoff" can be used to avoid burdening the message broker with unnecessary connection requests in case of a connection error. In the event of a TLS connection error involving the message broker, the reconnect rate is adjusted multiplicatively. This function can be activated using the ActivateExponentialBackoff() [} 61] method. The parameters of the method specify the minimum and maximum time for the algorithm. The minimum time describes the initial delay value for the new connection attempt. The maximum time describes the highest delay value. The delay values are doubled until the maximum value is reached. Once a connection has been established, the backoff rate is reset to the original value. The DeactivateExponentialBackoff() [} 62] method can be used to deactivate this function programmatically.

##### `DeactivateExponentialBackoff` (METHOD) — §5.1.2.1.6

**Description:** A feature referred to as "exponential backoff" can be used to avoid burdening the message broker with unnecessary connection requests in case of a connection error. In the event of a TLS connection error involving the message broker, the reconnect rate is adjusted multiplicatively. This function can be activated using the ActivateExponentialBackoff() [} 61] method. The parameters of the method specify the minimum and maximum time for the algorithm. The minimum time describes the initial delay value for the new connection attempt. The maximum time describes the highest delay value. The delay values are doubled until the maximum value is reached. Once a connection has been established, the backoff rate is reset to the original value. The DeactivateExponentialBackoff() [} 62] method can be used to deactivate this function programmatically.

##### `Request` (METHOD) → `BOOL` — §5.1.2.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPayload` | `PVOID` | Address for the payload of the MQTT message |
| `nPayloadSize` | `UDINT` | Size of the payload in bytes |
| `eQoS` | `TcIotMqttQos` | "Quality of Service" |
| `bRetain` | `BOOL` | If bRetain is TRUE, the broker stores the message in order to make it |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics a` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Request` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once, in order to send a Request to the broker.

##### `Response` (METHOD) → `BOOL` — §5.1.2.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPayload` | `PVOID` | Address for the payload of the MQTT message |
| `nPayloadSize` | `UDINT` | Size of the payload in bytes |
| `eQoS` | `TcIotMqttQos` | "Quality of Service" |
| `bRetain` | `BOOL` | If bRetain is TRUE, the broker stores the message in order to make it |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sResponseTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Response` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once to send a response to a request.

##### `GetTimeSinceLastBrokerMessage` (METHOD) → `UDINT` — §5.1.2.1.9

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetTimeSinc UDINT` | `The time in milliseconds since the last message from the message broker. eLastBroker Message` | — |

**Description:** This method specifies the time (in ms) since the last message from the message broker. The time is reset to 0 with every new message from the message broker. It does not matter whether these are ping messages or regular publish/subscribe messages.

#### PLC API › Tc3_IotBase

##### `FB_IotMqtt5ClientBase` (FUNCTION_BLOCK) — §5.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sClientId` | `STRING(255)` | The client ID can be specified individually and must be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError output is set. An |

**Description:** The function block enables communication to a message broker based on MQTT version 5.0.

#### PLC API › Tc3_IotBase › FB_IotMqtt5ClientBase

##### `Execute` (METHOD) — §5.1.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bConnect` | `BOOL` | The connection to the broker is established when bConnect is set to TRUE. |

**Description:** This method must be called cyclically in order to ensure the background communication with the MQTT broker.

##### `Publish` (METHOD) → `BOOL` — §5.1.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPayload` | `PVOID` | Address for the payload of the MQTT message nPayloadSize UDINT Size of the payload in bytes |
| `eQoS` | `TcIotMqttQos "Quality of Service"` | — |
| `bRetain` | `BOOL` | If bRetain is TRUE, the broker stores the message in order to make it available |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics are case` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Publish` | `BOOL PLC API` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once, in order to send a message to the broker.

##### `Subscribe` (METHOD) → `BOOL` — §5.1.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eQoS` | `TcIotMqttQos` | "Quality of Service" |
| `pProps` | `POINTER TO` | Pointer to Subscribe properties to be sent. |
| `MqttSubscribeProperties` | `The parameter is optional. The` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics are case se` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Subscribe` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is used by the client to signal to the broker that it wants to receive all MQTT message with a particular topic. The method can subscribe an MQTT client instance to multiple topics.

##### `Unsubscribe` (METHOD) → `BOOL` — §5.1.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pProps` | `POINTER TO` | Pointer to Unsubscribe properties to be sent. |
| `MqttUnsubscribeProperties` | `The parameter is optional. The` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics are case se` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Unsubscribe` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is used by the client to signal to the broker that no further messages with the specified topic should be transferred to it.

##### `OnMqtt5ConnAck` (METHOD) → `HRESULT` — §5.1.2.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pProps` | `POINTER TO MQTT properties received in case of CONNACK. MqttConnAck Properties` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OnMqtt5ConnAck` | `HRESULT` | — |

**Description:** Callback method

##### `OnMqtt5Disconnected` (METHOD) → `HRESULT` — §5.1.2.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pProps` | `POINTER TO MQTT properties received in case of Disconnect. MqttDisconne ctProperties` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OnMqtt5Discon HRESULT nect` | `` | — |

**Description:** Callback method

##### `OnMqtt5Message` (METHOD) → `HRESULT` — §5.1.2.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `payload` | `PVOID` | Address for the payload of the received MQTT message |
| `length` | `UDINT` | Size of the payload in bytes |
| `qos` | `TcIotMqttQos "Quality of Service"` | — |
| `pProps` | `POINTER TO MQTT properties that may have been received with a message. MqttPublishPr operties` | — |
| `pReq` | `POINTER TO MQTT Request Properties, which may have been received with a message. MqttRequestP roperties` | — |
| `repeated` | `BOOL` | If the user did not respond with S_OK to the last OnMqtt5Message() method |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `topic` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `OnMqtt5Messag HRESULT` | `The return value of the method should be S_OK, if the message was` | — |
| `e` | `accepted. If the message is to be issued again in the context of the next` | — |

**Description:** Callback method

##### `ActivateExponentialBackoff` (METHOD) — §5.1.2.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `tMqttBackoff TIME` | `Describes the initial delay value for the new connection attempt. MinTime` | — |
| `tMqttBackoff TIME` | `Describes the largest delay value. Once this value has been reached, all new` | — |
| `MaxTime` | `connection attempts are made at these intervals.` | — |

**Description:** A feature referred to as "exponential backoff" can be used to avoid burdening the message broker with unnecessary connection requests in case of a connection error. In the event of a TLS connection error involving the message broker, the reconnect rate is adjusted multiplicatively. This function can be activated using the ActivateExponentialBackoff() [} 61] method. The parameters of the method specify the minimum and maximum time for the algorithm. The minimum time describes the initial delay value for the new connection attempt. The maximum time describes the highest delay value. The delay values are doubled until the maximum value is reached. Once a connection has been established, the backoff rate is reset to the original value. The DeactivateExponentialBackoff() [} 62] method can be used to deactivate this function programmatically.

##### `DeactivateExponentialBackoff` (METHOD) — §5.1.2.2.9

**Description:** A feature referred to as "exponential backoff" can be used to avoid burdening the message broker with unnecessary connection requests in case of a connection error. In the event of a TLS connection error involving the message broker, the reconnect rate is adjusted multiplicatively. This function can be activated using the ActivateExponentialBackoff() [} 61] method. The parameters of the method specify the minimum and maximum time for the algorithm. The minimum time describes the initial delay value for the new connection attempt. The maximum time describes the highest delay value. The delay values are doubled until the maximum value is reached. Once a connection has been established, the backoff rate is reset to the original value. The DeactivateExponentialBackoff() [} 62] method can be used to deactivate this function programmatically.

##### `Request` (METHOD) → `BOOL` — §5.1.2.2.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPayload` | `PVOID` | Address for the payload of the MQTT message |
| `nPayloadSize` | `UDINT` | Size of the payload in bytes |
| `eQoS` | `TcIotMqttQos` | "Quality of Service" |
| `bRetain` | `BOOL` | If bRetain is TRUE, the broker stores the message in order to make it |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics a` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Request` | `BOOL PLC API` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once, in order to send a Request to the broker.

##### `Response` (METHOD) → `BOOL` — §5.1.2.2.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPayload` | `PVOID` | Address for the payload of the MQTT message |
| `nPayloadSize` | `UDINT` | Size of the payload in bytes |
| `eQoS` | `TcIotMqttQos` | "Quality of Service" |
| `bRetain` | `BOOL` | If bRetain is TRUE, the broker stores the message in order to make it |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sResponseTopic` | `STRING; // topic string (UTF-8) with any length (attend that MQTT topics` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Response` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** This method is called once to send a response to a request.

##### `GetTimeSinceLastBrokerMessage` (METHOD) → `UDINT` — §5.1.2.2.12

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetTimeSinc UDINT` | `The time in milliseconds since the last message from the message broker. eLastBroker Message` | — |

**Description:** This method specifies the time (in ms) since the last message from the message broker. The time is reset to 0 with every new message from the message broker. It does not matter whether these are ping messages or regular publish/subscribe messages.

#### PLC API › Tc3_IotBase › FB_IotMqtt5MessageQueue

##### `Dequeue` (METHOD) → `BOOL` — §5.1.2.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `fbMessage` | `REFERENCE TO FB_IotMqtt5M For the message itself the reference is transferred to an` | — |
| `essage` | `instance of type FB_IotMqtt5Message.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Dequeue` | `BOOL` | The method returns the return value TRUE if the call was successful. |

**Description:** The method returns the return value TRUE if the removal of an MQTT message from the queue was successful. The number of MQTT messages currently in the queue (nQueuedMessages property) is reduced by one through the removal of a message.

##### `ResetQueue` (METHOD) → `BOOL` — §5.1.2.7.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ResetQueue BOOL` | `The method returns the return value TRUE if the call was successful.` | — |

**Description:** When this method is called, all accumulated MQTT messages are deleted from the queue.

#### PLC API › Tc3_IotBase › FB_IotMqtt5Message

##### `CompareTopic` (METHOD) → `BOOL` — §5.1.2.8.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sTopic` | `STRING; // topic string with any length (attend that MQTT topics are case sensitive)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CompareTo BOOL` | `Is TRUE, if the specified topic is identical to the topic of the MQTT message in the` | — |
| `pic` | `function block. PLC API Inputs/outputs` | — |
| `sTopic` | `STRING` | — |

**Description:** This method returns TRUE, if the specified topic is identical to the topic of the MQTT message in the function block.

##### `GetTopic` (METHOD) → `BOOL` — §5.1.2.8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pTopic` | `POINTER TO` | The memory address for the buffer into which the topic is to be copied is |
| `STRING` | `specified here.` | — |
| `nTopicSize` | `UINT` | The maximum available buffer size (in bytes) is specified here. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetTopic` | `BOOL` | — |

**Description:** Syntax

##### `GetPayload` (METHOD) → `BOOL` — §5.1.2.8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPayload` | `PVOID` | The memory address for the buffer into which the payload is to be |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetPayload` | `BOOL` | — |

**Description:** Syntax

#### PLC API › Tc3_IotBase › MQTT5 Properties

##### `FB_IotMqtt5ConnAckProperties` (FUNCTION_BLOCK) — §5.1.2.9.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError output is set. An |

**Description:** The function block enables the reception of status information from the message broker directly after the Connection Acknowledgement, which is part of the connection setup. This allows the message broker to communicate information to the client about potential constraints, such as a maximum packet size that the message broker can handle.

##### `FB_IotMqtt5DisconnectProperties` (FUNCTION_BLOCK) — §5.1.2.9.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError output is set. An |

**Description:** The function block enables the reception of disconnection information.

##### `FB_IotMqtt5PublishProperties` (FUNCTION_BLOCK) — §5.1.2.9.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError output is set. An |

**Description:** The function block allows the definition of various properties that can be set when sending a message. This allows additional metadata to be attached to a message during transmission. The following metadata can be defined:

##### `FB_IotMqtt5SubscribeProperties` (FUNCTION_BLOCK) — §5.1.2.9.4

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError output is set. An |

**Description:** The function block allows the definition of different properties, which can be set when creating a subscription. This allows additional properties to be defined for a subscription and transferred from the client to the message broker.

##### `FB_IotMqtt5UnsubscribeProperties` (FUNCTION_BLOCK) — §5.1.2.9.5

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError output is set. An |

**Description:** The function block allows the definition of various properties that can be set when a subscription is unsubscribed. This allows the following metadata to be transmitted from the client to the message broker.

##### `FB_IotMqtt5UserProperties` (FUNCTION_BLOCK) — §5.1.2.9.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bError` | `BOOL` | Becomes TRUE as soon as an error situation occurs. |
| `hrErrorCode` | `HRESULT` | Returns an error code if the bError output is set. An |

**Description:** The function block enables the definition and handling of UserProperties. UserProperties are key/value pairs that describe optional metadata and can be used, for example, with PublishProperties.

#### PLC API › Tc3_JsonXml › Function blocks

##### `FB_JsonDomParser` (FUNCTION_BLOCK) — §5.2.1.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `initStatus` | `HRESULT PLC API Methods` | — |
| `AddArrayMember [} 119]` | `Adds an Array member to a JSON object.` | — |
| `AddBase64Member [} 120]` | `Adds a Base64 member to a JSON object.` | — |
| `AddBoolMember [} 121]` | `Adds a Bool member to a JSON object.` | — |
| `AddDateTimeMember [} 122]` | `Adds a DateTime member to a JSON object.` | — |
| `AddDcTimeMember [} 122]` | `Adds a DcTime member to a JSON object.` | — |
| `AddDoubleMember [} 123]` | `Adds a Double member to a JSON object.` | — |
| `AddFileTimeMember [} 124]` | `Adds a FileTime member to a JSON object.` | — |
| `AddHexBinaryMember [} 124]` | `Adds a HexBinary member to a JSON object.` | — |
| `AddInt64Member [} 125]` | `Adds an Int64 member to a JSON object.` | — |
| `AddIntMember [} 126]` | `Adds an Int member to a JSON object.` | — |
| `AddJsonMember [} 126]` | `Adds a JSON member to a JSON object.` | — |
| `AddNullMember [} 127]` | `Adds a NULL member to a JSON object.` | — |
| `AddObjectMember [} 128]` | `Adds an Object member to a JSON object.` | — |
| `AddStringMember [} 128]` | `Adds a String member to a JSON object.` | — |
| `AddUint64Member [} 129]` | `Adds an UInt64 member to a JSON object.` | — |
| `AddUintMember [} 130]` | `Adds an UInt member to a JSON object.` | — |
| `ArrayBegin [} 130]` | `Returns the first element of an array.` | — |
| `ArrayEnd [} 131]` | `Returns the last element of an array.` | — |
| `ClearArray [} 131]` | `Deletes the contents of an array.` | — |
| `CopyDocument [} 132]` | `Copies the contents of the DOM memory into a variable of data type` | — |

**Description:** This function block is derived from the same internal function block as FB_JsonDynDomParser [} 183] and thus offers the same interface.

#### PLC API › Tc3_JsonXml › Function blocks › FB_JsonDomParser

##### `LoadDocumentFromFile` (METHOD) → `BOOL` — §5.2.1.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExec` | `REFERENCE TO BOOL` | — |
| `/` | `Inputs/outputs` | — |
| `sFile` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sFile` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LoadDocumentFromFile` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method loads a JSON document from a file.

##### `AddArrayMember` (METHOD) → `SJsonValue` — §5.2.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `reserve` | `UDINT` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddArrayMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds an array member to a JSON object.

##### `AddBase64Member` (METHOD) → `SJsonValue` — §5.2.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `p` | `PVOID` | — |
| `n` | `DINT` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddBase64Member` | `SJsonValue (note possible invalidity) PLC API` | — |

**Description:** This method adds a Base64 member to a JSON object. A structure, for example, can be addressed as an input parameter. The corresponding Base64 coding is done by the method.

##### `AddBoolMember` (METHOD) → `SJsonValue` — §5.2.1.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `BOOL` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddBoolMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds a Bool member to a JSON object.

##### `AddDateTimeMember` (METHOD) → `SJsonValue` — §5.2.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `DATE_AND_TIME` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddDateTimeMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds a DateTime member to a JSON object.

##### `AddDcTimeMember` (METHOD) → `SJsonValue` — §5.2.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `DCTIME` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddDcTimeMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds a DcTime member to a JSON object.

##### `AddDoubleMember` (METHOD) → `SJsonValue` — §5.2.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `LREAL` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddDoubleMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds a Double member to a JSON object.

##### `AddFileTimeMember` (METHOD) → `SJsonValue` — §5.2.1.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `FILETIME` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddFileTimeMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds a FileTime member to a JSON object.

##### `AddHexBinaryMember` (METHOD) → `SJsonValue` — §5.2.1.1.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SjsonValue (note possible invalidity)` | — |
| `p` | `PVOID` | — |
| `n` | `DINT` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddHexBinaryMember` | `SjsonValue (note possible invalidity)` | — |

**Description:** This method adds a HexBinary member to a JSON object.

##### `AddInt64Member` (METHOD) → `SjsonValue` — §5.2.1.1.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `LINT` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddInt64Member` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds an Int64 member to a JSON object.

##### `AddIntMember` (METHOD) → `SJsonValue` — §5.2.1.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `DINT` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddIntMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds an Int member to a JSON object.

##### `AddJsonMember` (METHOD) → `SJsonValue` — §5.2.1.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |
| `rawJson` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |
| `rawJson` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddJsonMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds a JSON member to a JSON object.

##### `AddNullMember` (METHOD) → `SJsonValue` — §5.2.1.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddNullMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds a NULL member to a JSON object.

##### `AddObjectMember` (METHOD) → `SJsonValue` — §5.2.1.1.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddObjectMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds an Object member to a JSON object.

##### `AddStringMember` (METHOD) → `SJsonValue` — §5.2.1.1.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |
| `value` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |
| `value` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddStringMember` | `SJsonValue (note possible invalidity) PLC API` | — |

**Description:** This method adds a String member to a JSON object.

##### `AddUint64Member` (METHOD) → `SJsonValue` — §5.2.1.1.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `ULINT` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddUint64Member` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds an UInt64 member to a JSON object.

##### `AddUintMember` (METHOD) → `SJsonValue` — §5.2.1.1.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `UDINT` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddUintMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method adds an UInt member to a JSON object.

##### `ArrayBegin` (METHOD) → `SJsonAIterator` — §5.2.1.1.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ArrayBegin` | `SJsonAIterator PLC API` | — |

**Description:** This method returns the first element of an array and can be used together with the methods ArrayEnd() and NextArray() for iteration through a JSON array.

##### `ArrayEnd` (METHOD) → `SJsonAIterator` — §5.2.1.1.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ArrayEnd` | `SJsonAIterator` | — |

**Description:** This method returns the last element of an array and can be used together with the methods ArrayBegin() and NextArray() for iteration through a JSON array.

##### `ClearArray` (METHOD) → `BOOL` — §5.2.1.1.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `i` | `SJsonAIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ClearArray` | `BOOL` | — |

**Description:** This method deletes the content of an array.

##### `CopyDocument` (METHOD) → `UDINT` — §5.2.1.1.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDoc` | `DINT PLC API` | — |
| `/` | `Inputs/Outputs` | — |
| `pDoc` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pDoc` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyDocument` | `UDINT` | — |

**Description:** This method copies the contents of the DOM memory into a variable of data type STRING, which can have any length. The method returns the length of the string (including null termination). If the destination buffer is too small, it is emptied by a null termination and returned as length 0.

##### `CopyFrom` (METHOD) → `SJsonValue` — §5.2.1.1.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `w` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyFrom` | `SJsonValue (note possible invalidity)` | — |

**Description:** Syntax

##### `CopyJson` (METHOD) → `UDINT` — §5.2.1.1.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `pDoc` | `STRING` | — |
| `nDoc` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pDoc` | `STRING` | — |
| `nDoc` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyJson` | `UDINT` | — |

**Description:** This method extracts a JSON object from a key and stores it in a variable of data type STRING. This STRING can have any length. The method returns the length of the copied JSON object (including null termination). If the destination buffer is too small, it is emptied by a null termination and returned as length 0.

##### `CopyString` (METHOD) → `UDINT` — §5.2.1.1.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `pStr` | `STRING` | — |
| `nStr` | `UDINT` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pStr` | `STRING` | — |
| `nStr` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyString` | `UDINT PLC API` | — |

**Description:** This method copies the value of a key into a variable of the data type STRING, which can be of any length. The method returns the length of the copied string (including null termination). If the destination buffer is too small, it is emptied by a null termination and returned as length 0.

##### `ExceptionRaised` (METHOD) → `BOOL` — §5.2.1.1.25

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ExceptionRaised` | `BOOL` | — |

**Description:** Syntax

##### `FindMember` (METHOD) → `SJsonValue` — §5.2.1.1.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindMember` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method searches for a specific property in a JSON document and returns it. 0 is returned if no corresponding property is found.

##### `FindMemberPath` (METHOD) → `SJsonValue` — §5.2.1.1.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity) PLC API` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FindMemberPath` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method searches for a specific property in a JSON document and returns it. The property is specified according to its path in the document. 0 is returned if no corresponding property is found.

##### `GetArraySize` (METHOD) → `UDINT` — §5.2.1.1.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetArraySize` | `UDINT` | — |

**Description:** This method returns the number of elements in a JSON array.

##### `GetArrayValue` (METHOD) → `SJsonValue` — §5.2.1.1.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `i` | `SJsonAIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetArrayValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method returns the value at the current iterator position of an array.

##### `GetArrayValueByIdx` (METHOD) → `SJsonValue` — §5.2.1.1.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `idx` | `UDINT` | — |
| `v` | `SJsonValue (note possible invalidity)` | — |
| `p` | `PVOID` | — |
| `n` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetArrayValueByIdx` | `SJsonValue (note possible invalidity)` | — |
| `GetBase64` | `DINT` | — |

**Description:** This method returns the value of an array in a specified index.

##### `GetBool` (METHOD) → `BOOL` — §5.2.1.1.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SjsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetBool` | `BOOL` | — |

**Description:** This method returns the value of a property of the data type BOOL.

##### `GetDateTime` (METHOD) → `DATE_AND_TIME` — §5.2.1.1.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDateTime` | `DATE_AND_TIME` | — |

**Description:** This method returns the value of a property of the data type DATE_AND_TIME.

##### `GetDcTime` (METHOD) → `DCTIME` — §5.2.1.1.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDcTime` | `DCTIME` | — |

**Description:** This method returns the value of a property of the data type DCTIME.

##### `GetDocument` (METHOD) → `STRING` — §5.2.1.1.35

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDocument` | `STRING(255)` | — |

**Description:** This method returns the content of the DOM memory as the data type STRING(255). With longer strings, the method will return a NULL string. In this case the method CopyDocument [} 132]() must be used.

##### `GetDocumentLength` (METHOD) → `UDINT` — §5.2.1.1.36

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDocumentLength` | `UDINT` | — |

**Description:** This method returns the length of a JSON document in the DOM memory.

##### `GetDocumentRoot` (METHOD) → `SJsonValue` — §5.2.1.1.37

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDocumentRoot` | `SjsonValue (note possible invalidity)` | — |

**Description:** This method returns the root node of a JSON document in the DOM memory.

##### `GetDouble` (METHOD) → `LREAL` — §5.2.1.1.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SjsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDouble` | `LREAL` | — |

**Description:** This method returns the value of a property of the data type LREAL.

##### `GetFileTime` (METHOD) → `FILETIME` — §5.2.1.1.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetFileTime` | `FILETIME` | — |

**Description:** This method returns the value of a property of the data type DCTIME.

##### `GetHexBinary` (METHOD) → `DINT` — §5.2.1.1.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `p` | `PVOID` | — |
| `n` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetHexBinary` | `DINT` | — |

**Description:** This method decodes the HexBinary content of a property and writes it to a certain memory address, e.g. to a data structure.

##### `GetInt` (METHOD) → `DINT` — §5.2.1.1.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SjsonValue (note possible invalidity)` | — |
| `v` | `SjsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetInt` | `DINT` | — |
| `GetInt64` | `LINT` | — |

**Description:** This method returns the value of a property of the data type DINT.

##### `GetJson` (METHOD) → `STRING` — §5.2.1.1.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetJson` | `STRING(255)` | — |

**Description:** This method returns the value of a property as data type STRING(255), if this is a JSON document itself. With longer strings, the method will return a NULL string. In this case the method CopyJson [} 133]() must be used.

##### `GetJsonLength` (METHOD) → `UDINT` — §5.2.1.1.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetJsonLength` | `UDINT` | — |

**Description:** This method returns the length of a property if this is a JSON document.

##### `GetMaxDecimalPlaces` (METHOD) → `DINT` — §5.2.1.1.45

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetMaxDecimalPlaces` | `DINT` | — |

**Description:** This method returns the current setting for MaxDecimalPlaces. This influences the number of decimal places in the case of floating-point numbers.

##### `GetMemberName` (METHOD) → `STRING` — §5.2.1.1.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `i` | `SJsonIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetMemberName` | `STRING(255)` | — |

**Description:** This method returns the name of a JSON property member at the position of the current iterator, e.g. during the iteration of a child element of a JSON property with the methods MemberBegin(), MemberEnd() and NextMember().

##### `GetMemberValue` (METHOD) → `SJsonValue` — §5.2.1.1.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `i` | `SJsonIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetMemberValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method returns the value of a JSON property member at the position of the current iterator, e.g. during the iteration of a child element of a JSON property with the methods MemberBegin(), MemberEnd() and NextMember().

##### `GetString` (METHOD) → `STRING` — §5.2.1.1.48

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetString` | `STRING(255)` | — |

**Description:** This method returns the value of a property of the data type STRING(255). With longer strings, the method will return a NULL string. In this case the method CopyString [} 134]() must be used.

##### `GetStringLength` (METHOD) → `UDINT` — §5.2.1.1.49

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetStringLength` | `UDINT` | — |

**Description:** This method returns the length of a property if its value is a string.

##### `GetType` (METHOD) → `EJsonType` — §5.2.1.1.50

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetType` | `EJsonType PLC API` | — |

**Description:** This method returns the type of a property value. The return value can assume one of the values of the enum EJsonType.

##### `GetUint` (METHOD) → `UDINT` — §5.2.1.1.51

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SjsonValue (note possible invalidity)` | — |
| `v` | `SjsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetUint` | `UDINT` | — |
| `GetUint64` | `ULINT` | — |

**Description:** This method returns the value of a property of the data type UDINT.

##### `HasMember` (METHOD) → `BOOL` — §5.2.1.1.53

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `HasMember` | `BOOL` | — |

**Description:** This method checks whether a certain property is present in the DOM memory. If the property is present the method returns TRUE, otherwise it returns FALSE.

##### `IsArray` (METHOD) → `BOOL` — §5.2.1.1.54

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsArray` | `BOOL` | — |
| `IsBase64` | `BOOL` | — |

**Description:** This method checks whether a given property is an array. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsBool` (METHOD) → `BOOL` — §5.2.1.1.56

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsBool` | `BOOL` | — |

**Description:** This method checks whether the value of a given property is of the data type BOOL. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsDouble` (METHOD) → `BOOL` — §5.2.1.1.57

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsDouble` | `BOOL` | — |

**Description:** This method checks whether the value of a given property is of the data type Double (PLC: LREAL). If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsFalse` (METHOD) → `BOOL` — §5.2.1.1.58

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity) PLC API` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsFalse` | `BOOL` | — |

**Description:** This method checks whether the value of a given property is FALSE. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsHexBinary` (METHOD) → `BOOL` — §5.2.1.1.59

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsHexBinary` | `BOOL` | — |

**Description:** This method checks whether the value of a property is in the HexBinary format. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsInt` (METHOD) → `BOOL` — §5.2.1.1.60

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsInt` | `BOOL` | — |
| `IsInt64` | `BOOL` | — |

**Description:** This method checks whether the value of a given property is of the data type Integer (PLC: DINT). If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsISO8601TimeFormat` (METHOD) → `BOOL` — §5.2.1.1.62

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsISO8601TimeFormat` | `BOOL` | — |

**Description:** This method checks whether the value of a given property has a time format according to ISO8601. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsNull` (METHOD) → `BOOL` — §5.2.1.1.63

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsNull` | `BOOL` | — |

**Description:** This method checks whether the value of a given property is NULL. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsNumber` (METHOD) → `BOOL` — §5.2.1.1.64

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsNumber` | `BOOL` | — |

**Description:** This method checks whether the value of a given property is a numerical value. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsObject` (METHOD) → `BOOL` — §5.2.1.1.65

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsObject` | `BOOL` | — |

**Description:** This method checks whether the given property is a further JSON object. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsString` (METHOD) → `BOOL` — §5.2.1.1.66

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsString` | `BOOL` | — |

**Description:** This method checks whether the value of a given property is of the data type STRING. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsTrue` (METHOD) → `BOOL` — §5.2.1.1.67

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsTrue` | `BOOL PLC API` | — |

**Description:** This method checks whether the value of a given property is TRUE. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsUint` (METHOD) → `BOOL` — §5.2.1.1.68

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsUint` | `BOOL` | — |
| `IsUint64` | `BOOL` | — |

**Description:** This method checks whether the value of a given property is of the data type UDINT. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `MemberBegin` (METHOD) → `SJsonIterator` — §5.2.1.1.70

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MemberBegin` | `SJsonIterator` | — |

**Description:** This method returns the first child element below a JSON property and can be used by a JSON property together with the methods MemberEnd() and NextMember() for iteration.

##### `MemberEnd` (METHOD) → `SJsonIterator` — §5.2.1.1.71

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity) PLC API` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `MemberEnd` | `SJsonIterator` | — |

**Description:** This method returns the last child element below a JSON property and can be used by a JSON property together with the methods MemberBegin() and NextMember() for iteration.

##### `NewDocument` (METHOD) → `SJsonValue` — §5.2.1.1.72

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NewDocument` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method generates a new empty JSON document in the DOM memory.

##### `NextArray` (METHOD) → `SJsonAIterator` — §5.2.1.1.73

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `i` | `SJsonAlterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NextArray` | `SJsonAIterator` | — |

**Description:** Syntax

##### `NextMember` (METHOD) → `SJsonIterator` — §5.2.1.1.74

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `i` | `SJsonIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NextMember` | `SJsonIterator` | — |

**Description:** This method returns the next child element below a JSON property and can be used by a JSON property together with the methods MemberBegin [} 158]() and MemberEnd [} 158]() for iteration.

##### `ParseDocument` (METHOD) → `SJsonValue` — §5.2.1.1.75

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sJson` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sJson` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ParseDocument` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method loads a JSON object into the DOM memory for further processing. The JSON object takes the form of a string and is transferred to the method as an input. A reference to the JSON document in the DOM memory is returned to the caller.

##### `PushbackBase64Value` (METHOD) → `SJsonValue` — §5.2.1.1.76

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `sJsonValue (note possible invalidity)` | — |
| `p` | `PVOID` | — |
| `n` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackBase64Value` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a Base64 value to the end of an array. A structure, for example, can be addressed as an input parameter. The corresponding Base64 coding is done by the method.

##### `PushbackBoolValue` (METHOD) → `SJsonValue` — §5.2.1.1.77

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `sJsonValue (note possible invalidity)` | — |
| `value` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackBoolValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a value of the data type BOOL to the end of an array.

##### `PushbackCopyValue` (METHOD) → `SJsonValue` — §5.2.1.1.78

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `json` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackCopyValue` | `SjsonValue (note possible invalidity)` | — |

**Description:** This method adds a JSON document from the memory of a FB_JsonDomParser [} 113] to the end of an array.

##### `PushbackDateTimeValue` (METHOD) → `SJsonValue` — §5.2.1.1.79

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `sJsonValue (note possible invalidity)` | — |
| `value` | `DATE_AND_TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackDateTimeValue` | `SJsonValue (note possible invalidity) PLC API` | — |

**Description:** This method appends a value of the data type DATE_AND_TIME to the end of an array.

##### `PushbackDcTimeValue` (METHOD) → `SJsonValue` — §5.2.1.1.80

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `sJsonValue (note possible invalidity)` | — |
| `value` | `DCTIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackDcTimeValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a value of the data type DCTIME to the end of an array.

##### `PushbackDoubleValue` (METHOD) → `SJsonValue` — §5.2.1.1.81

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `sJsonValue (note possible invalidity)` | — |
| `value` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackDoubleValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a value of the data type Double to the end of an array.

##### `PushbackFileTimeValue` (METHOD) → `SJsonValue` — §5.2.1.1.82

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `sJsonValue (note possible invalidity)` | — |
| `value` | `FILETIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackFileTimeValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a value of the data type FILETIME to the end of an array.

##### `PushbackHexBinaryValue` (METHOD) → `SJsonValue` — §5.2.1.1.83

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `sJsonValue (note possible invalidity)` | — |
| `p` | `PVOID` | — |
| `n` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackHexBinaryValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a HexBinary value to the end of an array. The coding in the HexBinary format is executed by the method. A data structure, for example, can be used as the source.

##### `PushbackInt64Value` (METHOD) → `SJsonValue` — §5.2.1.1.84

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `sJsonValue (note possible invalidity)` | — |
| `value` | `LINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackInt64Value` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a value of the data type Int64 to the end of an array.

##### `PushbackIntValue` (METHOD) → `SJsonValue` — §5.2.1.1.85

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `sJsonValue (note possible invalidity)` | — |
| `value` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackIntValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a value of the data type INT to the end of an array.

##### `PushbackJsonValue` (METHOD) → `SJsonValue` — §5.2.1.1.86

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity) PLC API` | — |
| `/` | `Inputs/outputs` | — |
| `rawJson` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rawJson` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackJsonValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a JSON document that exists as a STRING in the PLC to the end of an array.

##### `PushbackNullValue` (METHOD) → `SJsonValue` — §5.2.1.1.87

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackNullValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a NULL value to the end of an array.

##### `PushbackStringValue` (METHOD) → `SJsonValue` — §5.2.1.1.88

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `value` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackStringValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a value of the data type DCTIME to the end of an array.

##### `PushbackUint64Value` (METHOD) → `SJsonValue` — §5.2.1.1.89

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `ULINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackUint64Value` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a value of the data type UInt64 to the end of an array.

##### `PushbackUintValue` (METHOD) → `SJsonValue` — §5.2.1.1.90

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PushbackUintValue` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method appends a value of the data type UInt to the end of an array.

##### `RemoveAllMembers` (METHOD) → `BOOL` — §5.2.1.1.91

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveAllMembers` | `BOOL` | — |

**Description:** This method removes all child elements from a given property.

##### `RemoveArray` (METHOD) → `BOOL` — §5.2.1.1.92

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `i` | `SJsonAIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveArray` | `BOOL` | — |

**Description:** This method deletes the value of the current array iterator.

##### `RemoveMember` (METHOD) → `BOOL` — §5.2.1.1.93

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `i` | `SJsonIterator` | — |
| `keepOrder` | `BOOL PLC API` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveMember` | `BOOL` | — |

**Description:** This method deletes the property at the current iterator.

##### `RemoveMemberByName` (METHOD) → `BOOL` — §5.2.1.1.94

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `keepOrder` | `BOOL` | — |
| `/` | `Inputs/outputs` | — |
| `member` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `member` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveMemberByName` | `BOOL` | — |

**Description:** This method removes a child element from a given property. The element is referenced by its name.

##### `SaveDocumentToFile` (METHOD) → `BOOL` — §5.2.1.1.95

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExec` | `REFERENCE TO BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `sFile` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sFile` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SaveDocumentToFile` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method saves a JSON document in a file.

##### `SetAdsProvider` (METHOD) → `BOOL` — §5.2.1.1.96

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `oid` | `OTCID` | Object ID of the task. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetAdsProvider` | `BOOL` | The method returns TRUE if the setting of the ADS provider was |

**Description:** This method sets the context in which the file accesses are processed when loading and saving the JSON documents. If no ADS provider is specified, the current task is used.

##### `SetArray` (METHOD) → `SJsonValue` — §5.2.1.1.97

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `v` | `SJsonValue (note possible invalidity)` | — |
| `p` | `PVOID` | — |
| `n` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetArray` | `SJsonValue (note possible invalidity)` | — |
| `SetBase64` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to the type "Array". New values can now be added to the array with the Pushback methods.

##### `SetBool` (METHOD) → `SJsonValue` — §5.2.1.1.99

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetBool` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to a value of the data type BOOL.

##### `SetDateTime` (METHOD) → `SJsonValue` — §5.2.1.1.100

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `DATE_AND_TIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetDateTime` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to a value of the data type DATE_AND_TIME.

##### `SetDcTime` (METHOD) → `SJsonValue` — §5.2.1.1.101

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `DCTIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetDcTime` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to a value of the data type DCTIME.

##### `SetDouble` (METHOD) → `SJsonValue` — §5.2.1.1.102

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetDouble` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to a value of the data type Double.

##### `SetFileTime` (METHOD) → `SJsonValue` — §5.2.1.1.103

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `FILETIME` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetFileTime` | `SJsonValue (note possible invalidity) PLC API` | — |

**Description:** This method sets the value of a property to a value of the data type FILETIME.

##### `SetHexBinary` (METHOD) → `SJsonValue` — §5.2.1.1.104

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `p` | `PVOID` | — |
| `n` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetHexBinary` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to a HexBinary-coded value. A data structure, for example, can be used as the source. Coding to the HexBinary format takes place inside the method.

##### `SetInt` (METHOD) → `SJsonValue` — §5.2.1.1.105

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `DINT` | — |
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `LINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetInt` | `SJsonValue (note possible invalidity)` | — |
| `SetInt64` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to a value of the data type INT.

##### `SetJson` (METHOD) → `SJsonValue` — §5.2.1.1.107

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `rawJson` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rawJson` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetJson` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method inserts a further JSON document into the value of a property.

##### `SetMaxDecimalPlaces` (METHOD) — §5.2.1.1.108

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `dp` | `DINT` | — |

**Description:** This function specifies the number of decimal places after which a floating-point number is rounded.

##### `SetNull` (METHOD) → `SJsonValue` — §5.2.1.1.109

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetNull` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to the value NULL.

##### `SetObject` (METHOD) → `SJsonValue` — §5.2.1.1.110

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetObject` | `SjsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to the type "Object". This enables the nesting of JSON documents.

##### `SetString` (METHOD) → `SJsonValue` — §5.2.1.1.111

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `/` | `Inputs/outputs` | — |
| `value` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetString` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to a value of the data type STRING.

##### `SetUint` (METHOD) → `SJsonValue` — §5.2.1.1.112

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `UDINT` | — |
| `v` | `SJsonValue (note possible invalidity)` | — |
| `value` | `ULINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetUint` | `SJsonValue (note possible invalidity)` | — |
| `SetUint64` | `SJsonValue (note possible invalidity)` | — |

**Description:** This method sets the value of a property to a value of the data type UInt.

##### `Swap` (METHOD) → `BOOL` — §5.2.1.1.114

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `v` | `SJsonValue (note possible invalidity)` | — |
| `w` | `SJsonValue (note possible invalidity) PLC API` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Swap` | `BOOL` | — |

**Description:** This method swaps the content of two pointers.

#### PLC API › Tc3_JsonXml › Function blocks

##### `FB_JsonDynDomParser` (FUNCTION_BLOCK) — §5.2.1.2

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `initStatus` | `HRESULT Methods` | — |

**Description:** This function block is derived from the same internal function block as FB_JsonDomParser [} 113] and thus offers the same interface.

##### `FB_JsonSaxReader` (FUNCTION_BLOCK) — §5.2.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sBase64` | `STRING` | — |
| `pBytes` | `POINTER TO BYTE` | — |
| `nBytes` | `REFERENCE TO DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `initStatus` | `HRESULT Methods` | — |
| `DecodeBase64 [} 184]` | `Converts a Base64-formated string to binary data.` | — |
| `DecodeDateTime [} 185]` | `Creates a PLC variable of the data type DATE_AND_TIME or DT from a` | — |
| `DecodeBase64` | `BOOL PLC API` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** Syntax

#### PLC API › Tc3_JsonXml › Function blocks › FB_JsonSaxReader

##### `DecodeDateTime` (METHOD) → `BOOL` — §5.2.1.3.2

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sDT` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DecodeDateTime` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `sDT` | `STRING` | — |
| `nDT` | `DATE_AND_TIME` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method enables the generation of a PLC variable of the type DATE_AND_TIME or DT from a standardized ISO8601 time format (e.g. YYYY-MM-DDThh:mm:ss). DT corresponds to the number of seconds starting from the date 01.01.1970. If the conversion was successful the method returns TRUE, otherwise it returns FALSE.

##### `DecodeDcTime` (METHOD) → `BOOL` — §5.2.1.3.3

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sDC` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DecodeDcTime` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `sDC` | `STRING` | — |
| `nDC` | `DCTIME` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method enables the generation of a PLC variable of the type DCTIME from a standardized ISO8601 time format (e.g. YYYY-MM-DDThh:mm:ss). DCTIME corresponds to the number of nanoseconds starting from the date 01.01.2000. If the conversion was successful the method returns TRUE, otherwise it returns FALSE.

##### `DecodeFileTime` (METHOD) → `BOOL` — §5.2.1.3.4

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sFT` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DecodeDateTime` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `sFT` | `STRING` | — |
| `nFT` | `FILETIME` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method enables the generation of a PLC variable of the type FILETIME from a standardized ISO8601 time format (e.g. YYYY-MM-DDThh:mm:ss). FILETIME corresponds to the number of nanoseconds starting from the date 01.01.1601 – measured in 100 nanoseconds. If the conversion was successful the method returns TRUE, otherwise it returns FALSE.

##### `DecodeHexBinary` (METHOD) → `BOOL` — §5.2.1.3.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBytes` | `POINTER TO BYTE` | — |
| `nBytes` | `REFERENCE TO DINT` | — |
| `/` | `Inputs/Outputs` | — |
| `sHex` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sHex` | `STRING` | — |
| `sBase64` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DecodeHexBinary` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |
| `IsBase64` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `sBase64` | `STRING` | — |

**Description:** This method converts a string containing hexadecimal values into binary data. If the conversion was successful the method returns TRUE, otherwise it returns FALSE.

##### `IsHexBinary` (METHOD) → `BOOL` — §5.2.1.3.7

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sHex` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsHexBinary` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `sHex` | `STRING` | — |

**Description:** This method checks whether the transferred string consists of hexadecimal values. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `IsISO8601TimeFormat` (METHOD) → `BOOL` — §5.2.1.3.8

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sDT` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsISO8601TimeFormat` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `sDT` | `STRING` | — |

**Description:** This method checks whether the transferred string corresponds to the standardized ISO8601 time format. If that is the case, the method returns TRUE, otherwise it returns FALSE.

##### `Parse` (METHOD) → `BOOL` — §5.2.1.3.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipHdl` | `ITcJsonSaxHandler` | — |
| `/` | `Inputs/Outputs` | — |
| `sJson` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sJson` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Parse` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method starts the SAX reader parsing procedure. The JSON object to be parsed and a reference to a function block, which was derived from the interface ITcJsonSaxHandler, are transferred as input parameters. This function block is then used for the callback methods of the SAX reader.

##### `ParseValues` (METHOD) → `BOOL` — §5.2.1.3.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `ipHdl` | `ITcJsonSaxValues` | — |
| `/` | `Inputs/Outputs` | — |
| `sJson` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sJson` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ParseValues` | `BOOL PLC API` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method starts the SAX reader parsing procedure. The JSON object to be parsed and a reference to a function block, which was derived from the interface ITcJsonSaxValues, are transferred as input parameters. This function block is then used for the callback methods of the SAX reader. What is special about this method is that exclusively values are taken into account in the callback methods, i.e. there are no OnKey() or OnStartObject() callbacks.

#### PLC API › Tc3_JsonXml › Function blocks

##### `FB_JsonSaxWriter` (FUNCTION_BLOCK) — §5.2.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBytes` | `POINTER TO BYTE` | — |
| `nBytes` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `initStatus` | `HRESULT` | — |
| `AddBase64 [} 194]` | `Adds a value of the data type Base64 to a property.` | — |
| `AddBool [} 194]` | `Adds a value of the data type BOOL to a property.` | — |
| `AddDateTime [} 195]` | `Adds a value of the data type DATE_AND_TIME to a property.` | — |
| `AddDcTime [} 195]` | `Adds a value of the data type DCTIME to a property.` | — |
| `AddDint [} 195]` | `Adds a value of the data type DINT to a property.` | — |
| `AddFileTime [} 196]` | `Adds a value of the data type FILETIME to a property.` | — |
| `AddHexBinary [} 196]` | `Adds a HexBinary value to a property.` | — |
| `AddKey [} 197]` | `Adds a new property key at the current position of the SAX Writer.` | — |
| `AddKeyBool [} 197]` | `Creates a new property key and at the same time a value of the data type` | — |

**Description:** Syntax

#### PLC API › Tc3_JsonXml › Function blocks › FB_JsonSaxWriter

##### `AddBool` (METHOD) — §5.2.1.4.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `BOOL` | — |

**Description:** This method adds a value of the data type BOOL to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddDateTime` (METHOD) — §5.2.1.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DATE_AND_TIME` | — |

**Description:** This method adds a value of the data type DATE_AND_TIME to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddDcTime` (METHOD) — §5.2.1.4.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DCTIME` | — |

**Description:** This method adds a value of the data type DCTIME to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddDint` (METHOD) — §5.2.1.4.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DINT` | — |

**Description:** This method adds a value of the data type DINT to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddFileTime` (METHOD) — §5.2.1.4.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `FILETIME` | — |

**Description:** This method adds a value of the data type FILETIME to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddHexBinary` (METHOD) — §5.2.1.4.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBytes` | `POINTER TO BYTE` | — |
| `nBytes` | `DINT` | — |

**Description:** This method adds a hex binary value to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddKey` (METHOD) — §5.2.1.4.8

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method adds a new property key at the current position of the SAX writer. The value of the new property is usually set afterwards. This can be done using one of the following methods, for example: AddBase64 [} 194], AddBool [} 194], AddDateTime [} 195], AddDcTime [} 195], AddDint [} 195], AddFileTime [} 196], AddHexBinary [} 196], AddLint [} 201], AddLreal [} 202], AddNull [} 202], AddRawArray [} 202], AddRawObject [} 203], AddReal [} 203], AddString [} 204], AddUdint [} 204], AddUlint [} 204].

##### `AddKeyBool` (METHOD) — §5.2.1.4.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type BOOL.

##### `AddKeyDateTime` (METHOD) — §5.2.1.4.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DATE_AND_TIME` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type DATE_AND_TIME.

##### `AddKeyDcTime` (METHOD) — §5.2.1.4.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DCTIME` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type DCTIME.

##### `AddKeyFileTime` (METHOD) — §5.2.1.4.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `FILETIME` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type FILETIME.

##### `AddKeyLreal` (METHOD) — §5.2.1.4.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `LREAL` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type LREAL.

##### `AddKeyNull` (METHOD) — §5.2.1.4.14

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and initializes its value with null.

##### `AddKeyNumber` (METHOD) — §5.2.1.4.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DINT PLC API` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type DINT.

##### `AddKeyString` (METHOD) — §5.2.1.4.16

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |
| `value` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type STRING.

##### `AddLint` (METHOD) — §5.2.1.4.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `LINT` | — |

**Description:** This method adds a value of the data type LINT to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddLreal` (METHOD) — §5.2.1.4.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `LREAL` | — |

**Description:** This method adds a value of the data type LREAL to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddNull` (METHOD) — §5.2.1.4.19

**Description:** This method adds the value null to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddRawArray` (METHOD) — §5.2.1.4.20

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rawJson` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddRawArray` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `rawJson` | `STRING` | — |

**Description:** This method adds a valid JSON array to a given property as a value. The array to be added must be in a valid JSON format and may only be added if the SAX Writer is at a correspondingly valid position, e.g. directly after a preceding AddKey() [} 197], StartArray() [} 208] or as the first call after a ResetDocument() [} 208].

##### `AddRawObject` (METHOD) — §5.2.1.4.21

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rawJson` | `STRING` | — |

**Description:** This method adds a valid JSON object to a given property as a value. The object to be added must be in a valid JSON format and may only be added if the SAX Writer is in an appropriately valid position, e.g. directly after a preceding AddKey() [} 197], StartArray() [} 208] or as the first call after a ResetDocument() [} 208].

##### `AddReal` (METHOD) — §5.2.1.4.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `REAL` | — |

**Description:** This method adds a value of the data type REAL to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddString` (METHOD) — §5.2.1.4.23

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `STRING` | — |

**Description:** This method adds a value of the data type STRING to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddUdint` (METHOD) — §5.2.1.4.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `UDINT` | — |

**Description:** This method adds a value of the data type UDINT to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `AddUlint` (METHOD) — §5.2.1.4.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `ULINT` | — |

**Description:** This method adds a value of the data type ULINT to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 197].

##### `CopyDocument` (METHOD) → `UDINT` — §5.2.1.4.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDoc` | `UDINT` | — |
| `/` | `Inputs/Outputs` | — |
| `pDoc` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pDoc` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyDocument` | `UDINT` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method copies the content of the current JSON object created with the SAX Writer to a target variable of the data type STRING, which can be of any length. The method returns the length of the string (including null termination). If the destination buffer is too small, it is emptied by a null termination and returned as length 0.

##### `EndArray` (METHOD) → `HRESULT` — §5.2.1.4.27

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EndArray` | `HRESULT` | — |

**Description:** This method generates the end of a started JSON array ("square closing bracket") and inserts it at the current position of the SAX writer.

##### `EndObject` (METHOD) → `HRESULT` — §5.2.1.4.28

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EndObject` | `HRESULT` | — |

**Description:** This method generates the end of a started JSON object ("curly closing bracket") and inserts it at the current position of the SAX writer.

##### `GetDocument` (METHOD) → `STRING` — §5.2.1.4.29

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDocument` | `STRING(255)` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method returns the content of the JSON object that is currently created with the SAX Writer and returns it as data type STRING(255).

##### `GetDocumentLength` (METHOD) → `UDINT` — §5.2.1.4.30

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDocumentLength` | `UDINT` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method returns the length of the JSON object that is currently created with the SAX Writer and returns it as data type UDINT.

##### `GetMaxDecimalPlaces` (METHOD) → `DINT` — §5.2.1.4.31

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetMaxDecimalPlaces` | `DINT` | The currently set number of digits after which a floating point number |

**Description:** This function queries the currently set number of decimal places after which a floating point number is truncated.

##### `ResetDocument` (METHOD) → `HRESULT` — §5.2.1.4.32

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ResetDocument` | `HRESULT` | — |

**Description:** This method resets the JSON object currently created with the SAX writer.

##### `SetMaxDecimalPlaces` (METHOD) → `HRESULT` — §5.2.1.4.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `decimalPla DINT` | `The number of decimal places to be set after which a floating-` | — |
| `ces` | `point number is truncated.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetMaxDecimalPlaces` | `HRESULT` | — |

**Description:** This function specifies the number of decimal places after which a floating-point number is rounded.

##### `StartArray` (METHOD) → `HRESULT` — §5.2.1.4.34

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartArray` | `HRESULT` | — |

**Description:** This method generates the start of a new JSON array ("square opening bracket") and inserts it at the current position of the SAX writer.

##### `StartObject` (METHOD) → `HRESULT` — §5.2.1.4.35

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartObject` | `HRESULT` | — |

**Description:** This method generates the start of a new JSON object ("curly opening bracket") and inserts it at the current position of the SAX writer.

#### PLC API › Tc3_JsonXml › Function blocks

##### `FB_JsonSaxPrettyWriter` (FUNCTION_BLOCK) — §5.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBytes` | `POINTER TO BYTE` | — |
| `nBytes` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `initStatus` | `HRESULT Methods` | — |
| `SetFormatOptions` | `Adjusts the general formatting options for an instance of the [<=>] [} 224]` | FB_JsonSaxPrettyWriter. SetIndent [} 225] Customizes the indentation. |

**Description:** This function block has a difference from FB_JsonSaxWriter [} 191]: it adds matching whitespaces for better readability of a JSON document.

#### PLC API › Tc3_JsonXml › Function blocks › FB_JsonSaxPrettyWriter

##### `AddBool` (METHOD) — §5.2.1.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `BOOL` | — |

**Description:** This method adds a value of the data type BOOL to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddDateTime` (METHOD) — §5.2.1.5.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DATE_AND_TIME` | — |

**Description:** This method adds a value of the data type DATE_AND_TIME to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddDcTime` (METHOD) — §5.2.1.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DCTIME` | — |

**Description:** This method adds a value of the data type DCTIME to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddDint` (METHOD) — §5.2.1.5.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DINT` | — |

**Description:** This method adds a value of the data type DINT to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddFileTime` (METHOD) — §5.2.1.5.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `FILETIME` | — |

**Description:** This method adds a value of the data type FILETIME to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddHexBinary` (METHOD) — §5.2.1.5.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pBytes` | `POINTER TO BYTE` | — |
| `nBytes` | `DINT` | — |

**Description:** This method adds a hex binary value to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddKey` (METHOD) — §5.2.1.5.8

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method adds a new property key at the current position of the SAX writer. The value of the new property is usually set afterwards. This can be done using one of the following methods, for example: AddBase64 [} 210], AddBool [} 210], AddDateTime [} 211], AddDcTime [} 211], AddDint [} 212], AddFileTime [} 212], AddHexBinary [} 212], AddLint [} 217], AddLreal [} 218], AddNull [} 218], AddRawArray [} 218], AddRawObject [} 219], AddReal [} 219], AddString [} 220], AddUdint [} 220], AddUlint [} 221].

##### `AddKeyBool` (METHOD) — §5.2.1.5.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type BOOL.

##### `AddKeyDateTime` (METHOD) — §5.2.1.5.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DATE_AND_TIME` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type DATE_AND_TIME.

##### `AddKeyDcTime` (METHOD) — §5.2.1.5.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DCTIME` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type DCTIME.

##### `AddKeyFileTime` (METHOD) — §5.2.1.5.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `FILETIME` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type FILETIME.

##### `AddKeyLreal` (METHOD) — §5.2.1.5.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `LREAL` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type LREAL.

##### `AddKeyNull` (METHOD) — §5.2.1.5.14

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and initializes its value with null.

##### `AddKeyNumber` (METHOD) — §5.2.1.5.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DINT` | — |
| `/` | `Inputs/Outputs` | — |
| `key` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type DINT.

##### `AddKeyString` (METHOD) — §5.2.1.5.16

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |
| `value` | `STRING` | — |

**Description:** This method creates a new property key and at the same time a value of the data type STRING.

##### `AddLint` (METHOD) — §5.2.1.5.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `LINT` | — |

**Description:** This method adds a value of the data type LINT to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddLreal` (METHOD) — §5.2.1.5.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `LREAL` | — |

**Description:** This method adds a value of the data type LREAL to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddNull` (METHOD) — §5.2.1.5.19

**Description:** This method adds the value null to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddRawArray` (METHOD) — §5.2.1.5.20

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rawJson` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddRawArray` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `rawJson` | `STRING` | — |

**Description:** This method adds a valid JSON array to a given property as a value. The array to be added must be in a valid JSON format and may only be added if the SAX Writer is at a correspondingly valid position, e.g. directly after a preceding AddKey() [} 213], StartArray() [} 226] or as the first call after a ResetDocument() [} 224].

##### `AddRawObject` (METHOD) — §5.2.1.5.21

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `rawJson` | `STRING` | — |

**Description:** This method adds a valid JSON object to a given property as a value. The object to be added must be in a valid JSON format and may only be added if the SAX Writer is in an appropriately valid position, e.g. directly after a preceding AddKey() [} 213], StartArray() [} 226] or as the first call after a ResetDocument() [} 224].

##### `AddReal` (METHOD) — §5.2.1.5.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `REAL` | — |

**Description:** This method adds a value of the data type REAL to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddString` (METHOD) — §5.2.1.5.23

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `STRING` | — |

**Description:** This method adds a value of the data type STRING to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddUdint` (METHOD) — §5.2.1.5.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `UDINT` | — |

**Description:** This method adds a value of the data type UDINT to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `AddUlint` (METHOD) — §5.2.1.5.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `ULINT` | — |

**Description:** This method adds a value of the data type ULINT to a property. Usually, a corresponding property was created beforehand with the method AddKey() [} 213].

##### `CopyDocument` (METHOD) → `UDINT` — §5.2.1.5.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDoc` | `UDINT` | — |
| `/` | `Inputs/Outputs` | — |
| `pDoc` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pDoc` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyDocument` | `UDINT` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method copies the content of the current JSON object created with the SAX Writer to a target variable of the data type STRING, which can be of any length. The method returns the length of the string (including null termination). If the destination buffer is too small, it is emptied by a null termination and returned as length 0.

##### `EndArray` (METHOD) → `HRESULT` — §5.2.1.5.27

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EndArray` | `HRESULT` | — |

**Description:** This method generates the end of a started JSON array ("square closing bracket") and inserts it at the current position of the SAX writer.

##### `EndObject` (METHOD) → `HRESULT` — §5.2.1.5.28

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `EndObject` | `HRESULT` | — |

**Description:** This method generates the end of a started JSON object ("curly closing bracket") and inserts it at the current position of the SAX writer.

##### `GetDocument` (METHOD) → `STRING` — §5.2.1.5.29

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDocument` | `STRING(255)` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method returns the content of the JSON object that is currently created with the SAX Writer and returns it as data type STRING(255).

##### `GetDocumentLength` (METHOD) → `UDINT` — §5.2.1.5.30

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDocumentLength` | `UDINT` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method returns the length of the JSON object that is currently created with the SAX Writer and returns it as data type UDINT.

##### `GetMaxDecimalPlaces` (METHOD) → `DINT` — §5.2.1.5.31

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetMaxDecimalPlaces` | `DINT` | The currently set number of digits after which a floating point number |

**Description:** This function queries the currently set number of decimal places after which a floating point number is truncated.

##### `ResetDocument` (METHOD) → `HRESULT` — §5.2.1.5.32

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ResetDocument` | `HRESULT` | — |

**Description:** This method resets the JSON object currently created with the SAX writer.

##### `SetFormatOptions` (METHOD) → `HRESULT` — §5.2.1.5.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `options` | `EJsonPrettyFormatOptions` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetFormatOptions` | `HRESULT` | — |

**Description:** This method is used to customize the general formatting options for an instance of the FB_JsonSaxPrettyWriter [} 209]. In addition to the default configuration, there is also the possibility that no line breaks are generated within an array. The array is then output in one line despite the inserted indentations.

##### `SetIndent` (METHOD) → `HRESULT` — §5.2.1.5.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `indentChar` | `SINT` | — |
| `indentCharCount` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetIndent` | `HRESULT` | — |

**Description:** This method is used to customize the indentation.

##### `SetMaxDecimalPlaces` (METHOD) → `HRESULT` — §5.2.1.5.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `decimalPla DINT` | `The number of decimal places to be set after which a floating-` | — |
| `ces` | `point number is truncated.` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetMaxDecimalPlaces` | `HRESULT` | — |

**Description:** This function specifies the number of decimal places after which a floating-point number is rounded.

##### `StartArray` (METHOD) → `HRESULT` — §5.2.1.5.36

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartArray` | `HRESULT` | — |

**Description:** This method generates the start of a new JSON array ("square opening bracket") and inserts it at the current position of the SAX writer.

##### `StartObject` (METHOD) → `HRESULT` — §5.2.1.5.37

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StartObject` | `HRESULT` | — |

**Description:** This method generates the start of a new JSON object ("curly opening bracket") and inserts it at the current position of the SAX writer.

#### PLC API › Tc3_JsonXml › Function blocks

##### `FB_JsonReadWriteDataType` — §5.2.1.6

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `initStatus` | `HRESULT` | — |
| `FB_JsonReadWriteDataType` | `Metadata can be added via PLC attributes to the JSON` | — |

**Description:** In order to use UTF-8 characters, e.g. in the automatic generation of metadata via the function block FB_JsonReadWriteDataType [} 226], the check box for the support of UTF-8 in the symbolism must be activated in the TwinCAT project. To do this, double-click on SYSTEM in the project tree, open the Settings tab and activate the corresponding check box.

#### PLC API › Tc3_JsonXml › Function blocks › FB_JsonReadWriteDataType

##### `AddJsonKeyPropertiesFromSymbol` (METHOD) → `BOOL` — §5.2.1.6.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fbWriter` | `FB_JsonSaxWriter` | — |
| `sKey` | `STRING` | — |
| `sDatatype` | `STRING` | — |
| `sProperties` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddJsonValueFromSymbol` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `fbWriter` | `FB_JsonSaxWriter` | — |
| `sKey` | `STRING` | — |
| `sDatatype` | `STRING` | — |
| `sProperties` | `STRING` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** With the aid of this method, metadata can be added via PLC attributes to the JSON representation of a PLC data structure on an FB_JsonSaxWriter [} 191] object. The method receives as its input parameters the instance of the FB_JsonSaxWriter function block, the desired name of the JSON property that is to contain the metadata, the data type name of the structure and a string variable sProperties, which contains a list of the PLC attributes to be extracted, separated by a cross bar.

##### `AddJsonKeyValueFromSymbol` (METHOD) → `BOOL` — §5.2.1.6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nData` | `UDINT` | — |
| `pData` | `PVOID` | — |
| `/` | `Inputs/Outputs` | — |
| `fbWriter` | `FB_JsonSaxWriter` | — |
| `sKey` | `STRING` | — |
| `sDatatype` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fbWriter` | `FB_JsonSaxWriter` | — |
| `sKey` | `STRING` | — |
| `sDatatype` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddJsonValueFromSymbol` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method generates the JSON representation of a PLC data structure on an FB_JsonSaxWriter [} 191] object. The method receives as its input parameters the instance of the FB_JsonSaxWriter function block, the data type name of the structure, and the address and size of the source structure instance. As a result, the FB_JsonSaxWriter instance contains a valid JSON representation of the structure. Unlike the method AddJsonValueFromSymbol() [} 230], the elements of the source structure are nested here in a JSON sub- object whose name can be specified via the input/output parameter sKey.

##### `AddJsonValueFromSymbol` (METHOD) → `BOOL` — §5.2.1.6.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nData` | `UDINT` | — |
| `pData` | `PVOID` | — |
| `/` | `Inputs/Outputs` | — |
| `fbWriter` | `FB_JsonSaxWriter` | — |
| `sDatatype` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `fbWriter` | `FB_JsonSaxWriter` | — |
| `sDatatype` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AddJsonValueFromSymbol` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method generates the JSON representation of a PLC data structure on an FB_JsonSaxWriter [} 191] object. The method receives as its input parameters the instance of the FB_JsonSaxWriter function block, the data type name of the structure, and the address and size of the source structure instance. As a result, the FB_JsonSaxWriter instance contains a valid JSON representation of the structure.

##### `CopyJsonStringFromSymbol` (METHOD) → `UDINT` — §5.2.1.6.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nData` | `UDINT` | — |
| `nDoc` | `UDINT` | — |
| `pData` | `PVOID` | — |
| `/` | `Inputs/Outputs` | — |
| `pDoc` | `STRING` | — |
| `sDatatype` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pDoc` | `STRING` | — |
| `sDatatype` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyJsonStringFromSymbol` | `UDINT` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method generates the JSON representation of a symbol and copies it into a variable of the data type STRING, which can be of any length. The method returns the length of the string (including null termination). If the destination buffer is too small, it is emptied by a null termination and returned as length 0.

##### `CopyJsonStringFromSymbolProperties` (METHOD) → `UDINT` — §5.2.1.6.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nDoc` | `UDINT` | — |
| `/` | `Inputs/Outputs` | — |
| `pDoc` | `STRING` | — |
| `sDatatype` | `STRING` | — |
| `sProperties` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `pDoc` | `STRING` | — |
| `sDatatype` | `STRING` | — |
| `sProperties` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyJsonStringFromSymbolProperties` | `UDINT` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method generates a corresponding JSON representation of PLC attributes on a symbol. In contrast to the AddJsonKeyPropertiesFromSymbol [} 228] method, the result is not written to an instance of the function block FB_JsonSaxWriter, but to a string variable. The method receives as its input parameters the data type name of the symbol and a string variable that represents a list of the PLC attributes to be extracted, separated by a cross bar.

##### `CopySymbolNameByAddress` (METHOD) → `UDINT` — §5.2.1.6.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nData` | `UDINT` | — |
| `pData` | `PVOID` | — |
| `nName` | `UDINT` | — |
| `/` | `Inputs/Outputs` | — |
| `sName` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sName` | `STRING;    // target string buffer where the symbol name should be copied to` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopySymbolNameByAddress` | `UDINT` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method returns the complete (ADS) symbol name of a transferred symbol. The method returns the size of the string (including null termination). If the destination buffer is too small, it is emptied by a null termination and returned as length 0.

##### `GetDataTypeNameByAddress` (METHOD) → `STRING` — §5.2.1.6.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nData` | `UDINT` | — |
| `pData` | `PVOID` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDataTypeNameByAddress` | `STRING` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method returns the data type name of a transferred symbol.

##### `GetJsonFromSymbol` (METHOD) → `BOOL` — §5.2.1.6.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nData` | `UDINT` | — |
| `pData` | `PVOID` | — |
| `nJson` | `REFERENCE TO UDINT` | — |
| `pJson` | `POINTER TO STRING` | — |
| `/` | `Inputs/Outputs` | — |
| `sDatatype` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sDatatype` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetJsonFromSymbol` | `BOOL` | — |
| `hrErrorCode` | `HRESULT Input parameter nJson` | — |

**Description:** This method generates the corresponding JSON representation of a symbol. In contrast to the AddJsonValueFromSymbol() [} 230] method, the result is not written to an instance of the function block FB_JsonSaxWriter, but to a string variable. The method receives as its input parameters the data type name of the symbol as well as the address and size of the source symbol, e.g. of a structure instance. The address and size of the destination buffer that contains the JSON representation of the symbol after the call are transferred as further input parameters.

##### `GetJsonStringFromSymbol` (METHOD) → `STRING` — §5.2.1.6.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nData` | `UDINT` | — |
| `pData` | `PVOID` | — |
| `/` | `Inputs/Outputs` | — |
| `sDatatype` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sDatatype` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetJsonStringFromSymbol` | `STRING(255) PLC API` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method generates the corresponding JSON representation of a symbol. In contrast to the AddJsonValueFromSymbol() [} 230] method, the result is not written to an instance of the function block FB_JsonSaxWriter, but to a string variable. The method receives as its input parameters the data type name of the symbol as well as the address and size of the source symbol of a structure instance, for example.

##### `GetJsonStringFromSymbolProperties` (METHOD) → `STRING` — §5.2.1.6.10

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sDatatype` | `STRING` | — |
| `sProperties` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetJsonStringFromSymbolProperties` | `STRING(255)` | — |
| `/` | `Inputs/Outputs` | — |
| `sDatatype` | `STRING` | — |
| `sProperties` | `STRING` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method generates a corresponding JSON representation of PLC attributes on a symbol. In contrast to the AddJsonKeyPropertiesFromSymbol [} 228] method, the result is not written to an instance of the function block FB_JsonSaxWriter, but to a string variable. The method receives as its input parameters the data type name of the symbol and a string variable that represents a list of the PLC attributes to be extracted, separated by a cross bar. The result is returned directly as the return value of the method.

##### `GetSizeJsonStringFromSymbol` (METHOD) → `UDINT` — §5.2.1.6.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nData` | `UDINT` | — |
| `pData` | `PVOID` | — |
| `/` | `Inputs/Outputs` | — |
| `sDatatype` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sDatatype` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetSizeJsonStringFromSymbol` | `UDINT` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method reads the size of the JSON representation of a symbol. The value is specified with null termination.

##### `GetSizeJsonStringFromSymbolProperties` (METHOD) → `UDINT` — §5.2.1.6.12

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sDatatype` | `STRING` | — |
| `sProperties` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetSizeJsonStringFromSymbolProperties` | `UDINT` | — |
| `/` | `Inputs/Outputs` | — |
| `sDatatype` | `STRING` | — |
| `sProperties` | `STRING` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method reads the size of the JSON representation of PLC attributes on a symbol. The value is specified with null termination.

##### `GetSymbolNameByAddress` (METHOD) → `STRING` — §5.2.1.6.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nData` | `UDINT` | — |
| `pData` | `PVOID` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetSymbolNameByAddress` | `STRING(255)` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method returns the complete (ADS) symbol name of a transferred symbol.

##### `SetSymbolFromJson` (METHOD) → `BOOL` — §5.2.1.6.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nData` | `UDINT` | — |
| `pData` | `PVOID PLC API` | — |
| `/` | `Inputs/Outputs` | — |
| `sJson` | `STRING` | — |
| `sDatatype` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sJson` | `STRING` | — |
| `sDatatype` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetSymbolFromJson` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method extracts a string containing a valid JSON message and attempts to save the contents of the JSON object to an equivalent data structure. The method receives as its input parameters the string with the JSON object, the data type name of the target structure, and the address and size of the target structure instance.

#### PLC API › Tc3_JsonXml › Function blocks

##### `FB_XmlDomParser` — §5.2.1.7

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `initStatus` | `HRESULT` | — |
| `AppendAttribute [} 245]` | `Appends a new attribute to an existing node.` | — |
| `AppendAttributeAsBool [} 246]` | `Appends a new attribute to an existing node (Boolean).` | — |
| `AppendAttributeAsDouble` | `Appends a new attribute to an existing node (Double). [} 247]` | — |

#### PLC API › Tc3_JsonXml › Function blocks › FB_XmlDomParser

##### `AppendAttribute` (METHOD) → `SXmlAttribute` — §5.2.1.7.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |
| `value` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |
| `value` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendAttribute` | `SXmlAttribute` | — |

**Description:** This method adds a new attribute to an existing node. The name and value of the new attribute and the existing XML node are transferred to the method as input parameters. The method returns a reference to the newly added attribute.

##### `AppendAttributeAsBool` (METHOD) → `SXmlAttribute` — §5.2.1.7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `BOOL PLC API` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendAttributeAsBool` | `SXmlAttribute` | — |

**Description:** This method adds a new attribute to an existing node. The value of the attribute has the data type Boolean. The name and value of the new attribute and the existing XML node are transferred to the method as input parameters. The method returns a reference to the newly added attribute.

##### `AppendAttributeAsDouble` (METHOD) → `SXmlAttribute` — §5.2.1.7.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `LREAL` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendAttributeAsDouble` | `SXmlAttribute` | — |

**Description:** This method adds a new attribute to an existing node. The value of the attribute has the data type Double. The name and value of the new attribute and the existing XML node are transferred to the method as input parameters. The method returns a reference to the newly added attribute.

##### `AppendAttributeAsFloat` (METHOD) → `SXmlAttribute` — §5.2.1.7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `REAL` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendAttributeAsFloat` | `SXmlAttribute` | — |

**Description:** This method adds a new attribute to an existing node. The value of the attribute has the data type Float. The name and value of the new attribute and the existing XML node are transferred to the method as input parameters. The method returns a reference to the newly added attribute.

##### `AppendAttributeAsInt` (METHOD) → `SXmlAttribute` — §5.2.1.7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `DINT` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendAttributeAsInt` | `SXmlAttribute` | — |

**Description:** This method adds a new attribute to an existing node. The value of the attribute has the data type Integer. The name and value of the new attribute and the existing XML node are transferred to the method as input parameters. The method returns a reference to the newly added attribute.

##### `AppendAttributeAsLint` (METHOD) → `SXmlAttribute` — §5.2.1.7.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `LINT` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendAttributeAsLint` | `SXmlAttribute` | — |

**Description:** This method adds a new attribute to an existing node. The value of the attribute has the data type Integer64. The name and value of the new attribute and the existing XML node are transferred to the method as input parameters. The method returns a reference to the newly added attribute.

##### `AppendAttributeAsUint` (METHOD) → `SXmlAttribute` — §5.2.1.7.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `UDINT` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendAttributeAsUint` | `SXmlAttribute` | — |

**Description:** This method adds a new attribute to an existing node. The value of the attribute has the data type Unsigned Integer. The name and value of the new attribute and the existing XML node are transferred to the method as input parameters. The method returns a reference to the newly added attribute.

##### `AppendAttributeAsUlint` (METHOD) → `SXmlAttribute` — §5.2.1.7.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `ULINT` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendAttributeAsUlint` | `SXmlAttribute` | — |

**Description:** This method adds a new attribute to an existing node. The value of the attribute has the data type Unsigned Integer64. The name and value of the new attribute and the existing XML node are transferred to the method as input parameters. The method returns a reference to the newly added attribute.

##### `AppendAttributeCopy` (METHOD) → `SXmlAttribute` — §5.2.1.7.9

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `copy` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendAttributeCopy` | `SXmlAttribute` | — |

**Description:** This method adds a new attribute to an existing node. The name and value of the new attribute are copied from an existing attribute. The existing attribute is transferred to the method as input parameter.

##### `AppendChild` (METHOD) → `SXmlNode` — §5.2.1.7.10

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `cdata` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |
| `value` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |
| `value` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendChild` | `SXmlNode` | — |

**Description:** This method inserts a new node below an existing node. The value of the new node is of the data type STRING. The name and the value of the new node as well as a reference to the existing node are transferred to the method as input parameters. The method returns a reference to the newly added node. The input parameter cdata indicates whether the value of the node is to be encapsulated in a CDATA function block, so that certain special characters such as "<" and ">" are allowed as values.

##### `AppendChildAsBool` (METHOD) → `SXmlNode` — §5.2.1.7.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendChildAsBool` | `SXmlNode` | — |

**Description:** This method inserts a new node below an existing node. The value of the new node has the data type Boolean. The name and value of the new node and a reference to the existing node are transferred to the method as input parameters. The method returns a reference to the newly added node.

##### `AppendChildAsDouble` (METHOD) → `SXmlNode` — §5.2.1.7.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `LREAL` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendChildAsDouble` | `SXmlNode` | — |

**Description:** This method inserts a new node below an existing node. The value of the new node has the data type Double. The name and value of the new node and a reference to the existing node are transferred to the method as input parameters. The method returns a reference to the newly added node.

##### `AppendChildAsFloat` (METHOD) → `SXmlNode` — §5.2.1.7.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `REAL` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendChildAsFloat` | `SXmlNode` | — |

**Description:** This method inserts a new node below an existing node. The value of the new node has the data type Float. The name and value of the new node and a reference to the existing node are transferred to the method as input parameters. The method returns a reference to the newly added node.

##### `AppendChildAsInt` (METHOD) → `SXmlNode` — §5.2.1.7.14

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `DINT` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendChildAsInt` | `SXmlNode` | — |

**Description:** This method inserts a new node below an existing node. The value of the new node has the data type Integer. The name and value of the new node and a reference to the existing node are transferred to the method as input parameters. The method returns a reference to the newly added node.

##### `AppendChildAsLint` (METHOD) → `SXmlNode` — §5.2.1.7.15

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `LINT` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendChildAsLint` | `SXmlNode` | — |

**Description:** This method inserts a new node below an existing node. The value of the new node has the data type Integer64. The name and value of the new node and a reference to the existing node are transferred to the method as input parameters. The method returns a reference to the newly added node.

##### `AppendChildAsUint` (METHOD) → `SXmlNode` — §5.2.1.7.16

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `UDINT PLC API` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendChildAsUint` | `SXmlNode` | — |

**Description:** This method inserts a new node below an existing node. The value of the new node has the data type Unsigned Integer. The name and value of the new node and a reference to the existing node are transferred to the method as input parameters. The method returns a reference to the newly added node.

##### `AppendChildAsUlint` (METHOD) → `SXmlNode` — §5.2.1.7.17

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `ULINT` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendChildAsUlint` | `SXmlNode` | — |

**Description:** This method inserts a new node below an existing node. The value of the new node has the data type Unsigned Integer64. The name and value of the new node and a reference to the existing node are transferred to the method as input parameters. The method returns a reference to the newly added node.

##### `AppendCopy` (METHOD) → `SXmlNode` — §5.2.1.7.18

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `copy` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendCopy` | `SXmlNode` | — |

**Description:** This method inserts a new node below an existing node. The name and value of the new node are copied from an existing node. The references to the existing nodes are transferred to the method as input parameters. The method returns a reference to the newly added node.

##### `AppendNode` (METHOD) → `SXmlNode` — §5.2.1.7.19

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING PLC API` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AppendNode` | `SXmlNode` | — |

**Description:** This method adds a new node to an existing node. The existing node and the name of the new node are transferred to the method as input parameters. The method returns a reference to the newly added node.

##### `Attributes` (METHOD) → `SXmlAttribute` — §5.2.1.7.20

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Attributes` | `SXmlAttribute` | — |

**Description:** This method can be used to read the attribute of a given XML node. The XML node and the name of the attribute are transferred to the method as input parameters. After the method has been called, further methods have to be called, for example to read the value of the attribute, e.g. AttributeAsInt().

##### `AttributeAsBool` (METHOD) → `BOOL` — §5.2.1.7.21

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeAsBool` | `BOOL` | — |

**Description:** This method returns the value of an attribute as data type Boolean. The attribute is transferred to the method as input parameter.

##### `AttributeAsDouble` (METHOD) → `LREAL` — §5.2.1.7.22

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeAsDouble` | `LREAL` | — |

**Description:** This method returns the value of an attribute as data type Double. The attribute is transferred to the method as input parameter.

##### `AttributeAsFloat` (METHOD) → `REAL` — §5.2.1.7.23

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeAsFloat` | `REAL` | — |

**Description:** This method returns the value of an attribute as data type Float. The attribute is transferred to the method as input parameter.

##### `AttributeAsInt` (METHOD) → `DINT` — §5.2.1.7.24

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeAsInt` | `DINT` | — |

**Description:** This method returns the value of an attribute as a data type Integer. The attribute is transferred to the method as input parameter.

##### `AttributeAsLint` (METHOD) → `LINT` — §5.2.1.7.25

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeAsLint` | `LINT` | — |

**Description:** This method returns the value of an attribute as a data type Integer64. The attribute is transferred to the method as input parameter.

##### `AttributeAsUint` (METHOD) → `UDINT` — §5.2.1.7.26

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeAsUint` | `UDINT` | — |

**Description:** This method returns the value of an attribute as data type Unsigned Integer. The attribute is transferred to the method as input parameter.

##### `AttributeAsUlint` (METHOD) → `ULINT` — §5.2.1.7.27

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeAsUlint` | `ULINT` | — |

**Description:** This method returns the value of an attribute as data type Unsigned Integer64. The attribute is transferred to the method as input parameter.

##### `AttributeBegin` (METHOD) → `SXmlIterator` — §5.2.1.7.28

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeBegin` | `SXmlIterator` | — |

**Description:** This method returns an iterator over all attributes of an XML node. The XML node is transferred to the method as input parameter.

##### `AttributeFromIterator` (METHOD) → `SXmlAttribute` — §5.2.1.7.29

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `it` | `SXmlIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeFromIterator` | `SXmlAttribute` | — |

**Description:** This method converts the current position of an iterator to an XML attribute object. The iterator is transferred to the method as input parameter.

##### `AttributeName` (METHOD) → `STRING` — §5.2.1.7.30

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeName` | `STRING` | — |

**Description:** This method returns the name of a given attribute. The attribute is transferred to the method as input parameter.

##### `Attributes` (METHOD) → `SXmlAttribute` — §5.2.1.7.31

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `it` | `REFERENCE TO SXmlIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Attributes` | `SXmlAttribute` | — |

**Description:** This method is used to navigate through the DOM and returns an iterator for all attributes found at an XML node. The iterator can then be used for further navigation through the elements that were found. The node and a reference to the iterator are transferred to the method as input parameters.

##### `AttributeText` (METHOD) → `STRING` — §5.2.1.7.32

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `AttributeText` | `STRING(255)` | — |

**Description:** This method returns the text of a given attribute. The attribute is transferred to the method as input parameter.

##### `Begin` (METHOD) → `SXmlIterator` — §5.2.1.7.33

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Begin` | `SXmlIterator` | — |

**Description:** This method returns an iterator over all child elements of an XML node, always starting from the first child element. The XML node is transferred to the method as input parameter.

##### `BeginByName` (METHOD) → `SXmlIterator` — §5.2.1.7.34

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BeginByName` | `SXmlIterator PLC API` | — |

**Description:** This method returns an iterator over all child elements of an XML node, starting at a particular element. The XML node is transferred to the method as input parameter.

##### `Child` (METHOD) → `SXmlNode` — §5.2.1.7.35

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**Description:** This method is used to navigate through the DOM. It returns a reference to the (first) child element of the current node. The start node is transferred to the method as input parameter.

##### `ChildByAttribute` (METHOD) → `SXmlNode` — §5.2.1.7.36

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `/` | `Inputs/Outputs` | — |
| `attr` | `STRING` | — |
| `value` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `attr` | `STRING` | — |
| `value` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ChildByAttribute` | `SXmlNode` | — |

**Description:** This method is used to navigate through the DOM. It returns a reference to a child element in the XML document. The start node and the name and value of the attribute are transferred to the method as input parameters.

##### `ChildByAttributeAndName` (METHOD) → `SXmlNode` — §5.2.1.7.37

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `/` | `Inputs/Outputs` | — |
| `attr` | `STRING` | — |
| `value` | `STRING` | — |
| `child` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `attr` | `STRING` | — |
| `value` | `STRING` | — |
| `child` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ChildByAttributeAndName` | `SXmlNode PLC API` | — |

**Description:** This method is used to navigate through the DOM. It returns a reference to a child element in the XML document. The start node, the name and value of the attribute, and the name of the child element are transferred to the method as input parameters.

##### `ChildByName` (METHOD) → `SXmlNode` — §5.2.1.7.38

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ChildByName` | `SXmlNode` | — |

**Description:** This method is used to navigate through the DOM. It returns a reference to a child element in the XML document. The start node and the name of the element to be returned are transferred to the method as input parameters.

##### `Children` (METHOD) → `SXmlNode` — §5.2.1.7.39

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Children` | `SXmlNode` | — |

**Description:** This method is used to navigate through the DOM. It returns an iterator for several child elements found in the XML document. The iterator can then be used for further navigation through the elements that were found. The start node and a reference to the iterator are transferred to the method as input parameters.

##### `ChildrenByName` (METHOD) → `SXmlNode` — §5.2.1.7.40

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `it` | `REFERENCE TO SXmlIterator PLC API` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ChildrenByName` | `SXmlNode` | — |

**Description:** This method is used to navigate through the DOM. It returns an iterator for several child elements found in the XML document. The iterator can then be used for further navigation through the elements that were found. The start node, the name of the child elements to be found and a reference to the iterator are transferred to the method as input parameters.

##### `ClearIterator` (METHOD) → `BOOL` — §5.2.1.7.41

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `it` | `SXmlIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ClearIterator` | `BOOL` | — |

**Description:** This method resets iterators that have already been used so that they can be used again the next time the program is run.

##### `Compare` (METHOD) → `BOOL` — §5.2.1.7.42

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `It1` | `SXmlIterator` | — |
| `It2` | `SXmlIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Compare` | `BOOL` | — |

**Description:** This method checks two iterators for equality.

##### `CopyAttributeText` (METHOD) → `UDINT` — §5.2.1.7.43

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |
| `nXml` | `UDINT PLC API` | — |
| `/` | `Inputs/Outputs` | — |
| `sXml` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sXml` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyAttributeText` | `UDINT` | — |

**Description:** This method reads the value of an XML attribute and writes it to a variable of data type String. The XML attribute as well as the target variable and the length to be written are passed to the method as input parameters. The method returns the actual size.

##### `CopyDocument` (METHOD) → `UDINT` — §5.2.1.7.44

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nXml` | `UDINT` | — |
| `/` | `Inputs/Outputs` | — |
| `sXml` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sXml` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyDocument` | `UDINT` | — |

**Description:** This method copies the contents of the DOM memory into a variable of data type String. The length to be written and the variable into which the resulting string is to be written are passed to the method as input parameters. The method returns the actually written length. Note that the size of the string variable is at least equal to the size of the XML document in the DOM.

##### `CopyNodeText` (METHOD) → `UDINT` — §5.2.1.7.45

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `nXml` | `UDINT` | — |
| `/` | `Inputs/Outputs` | — |
| `sXml` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sXml` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyNodeText` | `UDINT` | — |

**Description:** This method reads the value of an XML node and writes it to a variable of data type String. The XML node as well as the target variable and the length to be written are passed to the method as input parameters. The method returns the actual size.

##### `CopyNodeXml` (METHOD) → `UDINT` — §5.2.1.7.46

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlNode` | — |
| `nXml` | `UDINT` | — |
| `/` | `Inputs/Outputs` | — |
| `sXml` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sXml` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `CopyNodeXml` | `UDINT PLC API` | — |

**Description:** This method reads the XML structure of an XML node and writes it to a variable of data type String. The XML node as well as the target variable and the length to be written are passed to the method as input parameters. The method returns the actual size.

##### `FirstNodeByPath` (METHOD) → `SXmlNode` — §5.2.1.7.47

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `path` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `FirstNodeByPath` | `SXmlNode` | — |

**Description:** This method navigates through an XML document using a path that was transferred to the method. The path and the start node are transferred to the method as input parameters. The path is specified with "/" as separator. The method returns a reference to the XML node that was found.

##### `GetAttributeTextLength` (METHOD) → `UDINT` — §5.2.1.7.48

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetAttributeTextLength` | `UDINT` | — |

**Description:** This method returns the length of the value of an XML attribute. The XML attribute is transferred to the method as input parameter.

##### `GetDocumentLength` (METHOD) → `UDINT` — §5.2.1.7.49

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDocumentLength` | `UDINT` | — |

**Description:** This method returns the length of an XML document in bytes.

##### `GetDocumentNode` (METHOD) → `SXmlNode` — §5.2.1.7.50

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetDocumentNode` | `SXmlNode` | — |

**Description:** This method returns the root node of an XML document. This is not the same as the first XML node in the document (the method GetRootNode() should be used for this). The method can also be used to create an empty XML document in the DOM.

##### `GetNodeTextLength` (METHOD) → `UDINT` — §5.2.1.7.51

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetNodeTextLength` | `UDINT` | — |

**Description:** This method returns the length of the value of an XML node. The XML node is transferred to the method as input parameter.

##### `GetNodeXmlLength` (METHOD) → `UDINT` — §5.2.1.7.52

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetNodeXmlLength` | `UDINT` | — |

**Description:** This method returns the length of the XML structure of an XML node. The XML node is transferred to the method as input parameter.

##### `GetRootNode` (METHOD) → `SXmlNode` — §5.2.1.7.53

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `GetRootNode` | `SXmlNode` | — |

**Description:** This method returns a reference to the first XML node in the XML document.

##### `InsertAttributeCopy` (METHOD) → `SXmlAttribute` — §5.2.1.7.54

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `before` | `SXmlAttribute` | — |
| `copy` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InsertAttributeCopy` | `SXmlAttribute` | — |

**Description:** This method adds an attribute to an XML node. The name and value of an existing attribute are copied. The attribute can be placed at a specific position. The XML node, the position and a reference to the existing attribute object are transferred to the method as input parameters. The method returns a reference to the newly added attribute.

##### `InsertAttribute` (METHOD) → `SXmlAttribute` — §5.2.1.7.55

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `before` | `SXmlAttribute` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InsertAttribute` | `SXmlAttribute` | — |

**Description:** This method adds an attribute to an XML node. The attribute can be placed at a specific position. The XML node and the position and name of the new attribute are transferred to the method as input parameters. The method returns a reference to the newly added attribute. A value for the attribute can then be entered using the SetAttribute() method, for example.

##### `InsertChild` (METHOD) → `SXmlNode` — §5.2.1.7.56

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `before` | `SXmlAttribute` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InsertChild` | `SXmlNode` | — |

**Description:** This method adds a node to an existing XML node. The new node can be placed at a specific location. The existing XML node and the position and name of the new node are transferred to the method as input parameters. The method returns a reference to the newly added node. A value for the node can then be entered using the SetChild() method, for example.

##### `InsertCopy` (METHOD) → `SXmlNode` — §5.2.1.7.57

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `before` | `SXmlNode` | — |
| `copy` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `InsertCopy` | `SXmlNode` | — |

**Description:** This method adds a new node to an existing XML node and copies an existing node. The new node can be placed anywhere in the existing node. The XML node, the position and a reference to the existing node object are transferred to the method as input parameters. The method returns a reference to the newly added node.

##### `IsEnd` (METHOD) → `BOOL` — §5.2.1.7.58

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nit` | `SXmlIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `IsEnd` | `BOOL` | — |

**Description:** This method checks whether a given XML iterator is at the end of the iteration that is to be performed.

##### `LoadDocumentFromFile` (METHOD) → `BOOL` — §5.2.1.7.59

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExec` | `REFERENCE TO BOOL` | — |
| `/` | `Inputs/outputs` | — |
| `sFile` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sFile` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `LoadDocumentFromFile` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method loads an XML document from a file. The absolute path to the file is transferred to the method as input parameter.

##### `NewDocument` (METHOD) → `BOOL` — §5.2.1.7.60

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NewDocument` | `BOOL` | — |

**Description:** This method creates an empty XML document in the DOM memory.

##### `Next` (METHOD) → `SXmlIterator` — §5.2.1.7.61

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `it` | `SXmlIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Next` | `SXmlIterator` | — |

**Description:** This method sets an XML iterator for the next object that is to be processed.

##### `NextAttribute` (METHOD) → `SXmlAttribute` — §5.2.1.7.62

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NextAttribute` | `SXmlAttribute` | — |

**Description:** This method returns the next attribute for a given XML attribute.

##### `NextByName` (METHOD) → `SXmlIterator` — §5.2.1.7.63

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `it` | `SXmlIterator` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NextByName` | `SXmlIterator` | — |

**Description:** This method sets an XML iterator for the next object that is to be processed, which is identified by its name.

##### `NextSibling` (METHOD) → `SXmlNode` — §5.2.1.7.64

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NextSibling` | `SXmlNode` | — |

**Description:** This method returns the next direct node for a given XML node at the same XML level.

##### `NextSiblingByName` (METHOD) → `SXmlNode` — §5.2.1.7.65

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NextSiblingByName` | `SXmlNode` | — |

**Description:** This method returns the next direct node for a given XML node with a particular name at the same XML level.

##### `Node` (METHOD) → `SXmlNode` — §5.2.1.7.66

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `it` | `SXmlIterator` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Node` | `SXmlNode` | — |

**Description:** This method is used in conjunction with an iterator to navigate through the DOM. The iterator is transferred to the method as input parameter. The method then returns the current XML node as return value.

##### `NodeAsBool` (METHOD) → `BOOL` — §5.2.1.7.67

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeAsBool` | `BOOL` | — |

**Description:** This method returns the text of an XML node as data type Boolean. The XML node is transferred to the method as input parameter.

##### `NodeAsDouble` (METHOD) → `LREAL` — §5.2.1.7.68

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeAsDouble` | `LREAL PLC API` | — |

**Description:** This method returns the text of an XML node as data type Double. The XML node is transferred to the method as input parameter.

##### `NodeAsFloat` (METHOD) → `REAL` — §5.2.1.7.69

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeAsFloat` | `REAL` | — |

**Description:** This method returns the text of an XML node as data type Float. The XML node is transferred to the method as input parameter.

##### `NodeAsInt` (METHOD) → `DINT` — §5.2.1.7.70

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeAsInt` | `DINT` | — |

**Description:** This method returns the text of an XML node as a data type Integer. The XML node is transferred to the method as input parameter.

##### `NodeAsLint` (METHOD) → `LINT` — §5.2.1.7.71

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeAsLint` | `LINT` | — |

**Description:** This method returns the text of an XML node as a data type Integer64. The XML node is transferred to the method as input parameter.

##### `NodeAsUint` (METHOD) → `UDINT` — §5.2.1.7.72

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeAsUint` | `UDINT PLC API` | — |

**Description:** This method returns the text of an XML node as data type Unsigned Integer. The XML node is transferred to the method as input parameter.

##### `NodeAsUlint` (METHOD) → `ULINT` — §5.2.1.7.73

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeAsUlint` | `ULINT` | — |

**Description:** This method returns the text of an XML node as data type Unsigned Integer64. The XML node is transferred to the method as input parameter.

##### `NodeName` (METHOD) → `STRING` — §5.2.1.7.74

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeName` | `STRING` | — |

**Description:** This method returns the name of an XML node. A reference to the XML node is transferred to the method as input parameter.

##### `NodeText` (METHOD) → `STRING` — §5.2.1.7.75

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `NodeText` | `STRING(255)` | — |

**Description:** This method returns the text of an XML node. The XML node is transferred to the method as input parameter.

##### `ParseDocument` (METHOD) → `SXmlNode` — §5.2.1.7.76

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sXml` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ParseDocument` | `SXmlNode PLC API` | — |
| `/` | `Inputs/Outputs` | — |
| `sXml` | `STRING` | — |

**Description:** This method loads an XML document into the DOM memory for further processing. The XML document exists as a string and is transferred to the method as input parameter. A reference to the XML document in the DOM is returned to the caller.

##### `RemoveChild` (METHOD) → `BOOL` — §5.2.1.7.77

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `child` | `SXmlNode` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveChild` | `BOOL` | — |

**Description:** This method removes an XML child node from a given XML node. The two XML nodes are transferred to the method as input parameters. The method returns TRUE if the operation was successful and the XML node was removed.

##### `RemoveChildByName` (METHOD) → `BOOL` — §5.2.1.7.78

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `/` | `Inputs/Outputs` | — |
| `name` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `name` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RemoveChildByName` | `BOOL` | — |

**Description:** This method removes an XML child node from a given XML node. The node to be removed is addressed by its name. If there is more than one child node, the last child node is removed. The method returns TRUE if the operation was successful and the XML node was removed.

##### `SaveDocumentToFile` (METHOD) → `BOOL` — §5.2.1.7.79

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExec` | `REFERENCE TO BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `sFile` | `STRING PLC API` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sFile` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SaveDocumentToFile` | `BOOL` | — |
| `hrErrorCode` | `HRESULT` | — |

**Description:** This method saves the current XML document in a file. The absolute path to the file is transferred to the method as input parameter.

##### `SetAdsProvider` (METHOD) → `BOOL` — §5.2.1.7.80

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `oid` | `OTCID` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetAdsProvider` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `oid` | `OTCID` | — |

**Description:** Syntax

##### `SetAttribute` (METHOD) → `SXmlAttribute` — §5.2.1.7.81

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |
| `/` | `Inputs/Outputs` | — |
| `value` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetAttribute` | `SXmlAttribute` | — |

**Description:** This method sets the value of an attribute. The value has the data type String.

##### `SetAttributeAsBool` (METHOD) → `SXmlAttribute` — §5.2.1.7.82

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |
| `value` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetAttributeAsBool` | `SXmlAttribute` | — |

**Description:** This method sets the value of an attribute. The value has the data type Boolean.

##### `SetAttributeAsDouble` (METHOD) → `SXmlAttribute` — §5.2.1.7.83

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |
| `value` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetAttributeAsDouble` | `SXmlAttribute PLC API` | — |

**Description:** This method sets the value of an attribute. The value here has the data type Double.

##### `SetAttributeAsFloat` (METHOD) → `SXmlAttribute` — §5.2.1.7.84

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |
| `value` | `REAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetAttributeAsFloat` | `SXmlAttribute` | — |

**Description:** This method sets the value of an attribute. The value has the data type Float.

##### `SetAttributeAsInt` (METHOD) → `SXmlAttribute` — §5.2.1.7.85

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |
| `value` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetAttributeAsInt` | `SXmlAttribute` | — |

**Description:** This method sets the value of an attribute. The value has the data type Integer.

##### `SetAttributeAsLint` (METHOD) → `SXmlAttribute` — §5.2.1.7.86

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |
| `value` | `LINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetAttributeAsLint` | `SXmlAttribute` | — |

**Description:** This method sets the value of an attribute. The value has the data type Integer64.

##### `SetAttributeAsUint` (METHOD) → `SXmlAttribute` — §5.2.1.7.87

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |
| `value` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetAttributeAsUint` | `SXmlAttribute PLC API` | — |

**Description:** This method sets the value of an attribute. The value has the data type Unsigned Integer.

##### `SetAttributeAsUlint` (METHOD) → `SXmlAttribute` — §5.2.1.7.88

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `a` | `SXmlAttribute` | — |
| `value` | `ULINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetAttributeAsUlint` | `SXmlAttribute` | — |

**Description:** This method sets the value of an attribute. The value has the data type Unsigned Integer64.

##### `SetChild` (METHOD) → `SXmlNode` — §5.2.1.7.89

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `cdata` | `BOOL` | — |
| `/` | `Inputs/Outputs` | — |
| `value` | `STRING` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChild` | `SXmlNode` | — |

**Description:** This method sets the value of an XML node. The value is transferred to the method as input parameter of data type String. The input parameter cdata indicates whether the value of the node is to be encapsulated in a CDATA function block, so that certain special characters such as "<" and ">" are allowed as values.

##### `SetChildAsBool` (METHOD) → `SXmlNode` — §5.2.1.7.90

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `BOOL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChildAsBool` | `SXmlNode` | — |

**Description:** This method sets the value of an XML node. The value is transferred to the method as input parameter of data type Boolean.

##### `SetChildAsDouble` (METHOD) → `SXmlNode` — §5.2.1.7.91

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `LREAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChildAsDouble` | `SXmlNode` | — |

**Description:** This method sets the value of an XML node. The value is transferred to the method as input parameter of data type Double.

##### `SetChildAsFloat` (METHOD) → `SXmlNode` — §5.2.1.7.92

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `REAL` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChildAsFloat` | `SXmlNode` | — |

**Description:** This method sets the value of an XML node. The value is transferred to the method as input parameter of data type Float.

##### `SetChildAsInt` (METHOD) → `SXmlNode` — §5.2.1.7.93

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `DINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChildAsInt` | `SXmlNode` | — |

**Description:** This method sets the value of an XML node. The value is transferred to the method as input parameter of data type Integer.

##### `SetChildAsLint` (METHOD) → `SXmlNode` — §5.2.1.7.94

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `LINT PLC API` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChildAsLint` | `SXmlNode` | — |

**Description:** This method sets the value of an XML node. The value is transferred to the method as input parameter of data type Integer64.

##### `SetChildAsUint` (METHOD) → `SXmlNode` — §5.2.1.7.95

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `UDINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChildAsUint` | `SXmlNode` | — |

**Description:** This method sets the value of an XML node. The value is transferred to the method as input parameter of data type Unsigned Integer.

##### `SetChildAsUlint` (METHOD) → `SXmlNode` — §5.2.1.7.96

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `n` | `SXmlNode` | — |
| `value` | `ULINT` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SetChildAsUlint` | `SXmlNode` | — |

**Description:** This method sets the value of an XML node. The value is transferred to the method as input parameter of data type Unsigned Integer64.

#### PLC API › Tc3_JsonXml › Function blocks

##### `FB_JwtEncode` (FUNCTION_BLOCK) — §5.2.1.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `bExecute` | `BOOL` | A rising edge activates processing of the function block. |
| `sHeaderAlg` | `STRING(46)` | The algorithm to be used for the JWT header, e.g. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `sJwt` | `STRING` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `bBusy` | `BOOL` | Is TRUE as long as processing of the function block is in |

**Description:** The function block enables the creation and signing of a JSON Web Token (JWT).

#### PLC API › Tc3_JsonXml › Interfaces › ITcJsonSaxHandler

##### `OnBool` (METHOD) → `HRESULT` — §5.2.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `BOOL` | — |

**Description:** This callback method is triggered if a value of the data type BOOL was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnDint` (METHOD) → `HRESULT` — §5.2.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `DINT` | — |

**Description:** This callback method is triggered if a value of the data type DINT was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnEndArray` (METHOD) → `HRESULT` — §5.2.2.1.3

**Return type:** `HRESULT`

**Description:** This callback method is triggered if a square closing bracket, which corresponds to the JSON synonym for an ending array, was found at the position of the SAX reader. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnEndObject` (METHOD) → `HRESULT` — §5.2.2.1.4

**Return type:** `HRESULT`

**Description:** This callback method is triggered if a curly closing bracket, which corresponds to the JSON synonym for an ending object, was found at the position of the SAX reader. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnKey` (METHOD) → `HRESULT` — §5.2.2.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `len` | `UDINT` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `key` | `STRING` | — |

**Description:** This callback method is triggered if a property was found at the position of the SAX reader. The property name lies on the input/output parameter key and its length on the input parameter len. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnLint` (METHOD) → `HRESULT` — §5.2.2.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `LINT` | — |

**Description:** This callback method is triggered if a value of the data type LINT was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnLreal` (METHOD) → `HRESULT` — §5.2.2.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `LREAL` | — |

**Description:** This callback method is triggered if a value of the data type LREAL was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnNull` (METHOD) → `HRESULT` — §5.2.2.1.8

**Return type:** `HRESULT`

**Description:** This callback method is triggered if a NULL value was found at the position of the SAX reader. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnStartArray` (METHOD) → `HRESULT` — §5.2.2.1.9

**Return type:** `HRESULT`

**Description:** This callback method is triggered if a square opening bracket, which corresponds to the JSON synonym for a starting array, was found at the position of the SAX reader. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnStartObject` (METHOD) → `HRESULT` — §5.2.2.1.10

**Return type:** `HRESULT`

**Description:** This callback method is triggered if a curly opening bracket, which corresponds to the JSON synonym for a starting object, was found at the position of the SAX reader. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnString` (METHOD) → `HRESULT` — §5.2.2.1.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `len` | `UDINT` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `STRING` | — |

**Description:** This callback method is triggered if a value of the data type STRING was found at the position of the SAX reader. The In/Out parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnUdint` (METHOD) → `HRESULT` — §5.2.2.1.12

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `UDINT` | — |

**Description:** This callback method is triggered if a value of the data type UDINT was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnUlint` (METHOD) → `HRESULT` — §5.2.2.1.13

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `ULINT` | — |

**Description:** This callback method is triggered if a value of the data type ULINT was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

#### PLC API › Tc3_JsonXml › Interfaces › ITcJsonSaxValues

##### `OnBoolValue` (METHOD) → `HRESULT` — §5.2.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `level` | `UDINT` | — |
| `infos` | `POINTER TO TcJsonLevelInfo` | — |
| `value` | `BOOL` | — |

**Description:** This callback method is triggered if a value of the data type BOOL was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnDintValue` (METHOD) → `HRESULT` — §5.2.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `level` | `UDINT` | — |
| `infos` | `POINTER TO TcJsonLevelInfo` | — |
| `value` | `DINT` | — |

**Description:** This callback method is triggered if a value of the data type DINT was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnLintValue` (METHOD) → `HRESULT` — §5.2.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `level` | `UDINT` | — |
| `infos` | `POINTER TO TcJsonLevelInfo` | — |
| `value` | `LINT` | — |

**Description:** This callback method is triggered if a value of the data type LINT was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnLrealValue` (METHOD) → `HRESULT` — §5.2.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `level` | `UDINT` | — |
| `infos` | `POINTER TO TcJsonLevelInfo` | — |
| `value` | `LREAL` | — |

**Description:** This callback method is triggered if a value of the data type LREAL was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnNullValue` (METHOD) → `HRESULT` — §5.2.2.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `level` | `UDINT` | — |
| `infos` | `POINTER TO TcJsonLevelInfo` | — |

**Description:** This callback method is triggered if a NULL value was found at the position of the SAX reader. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnStringValue` (METHOD) → `HRESULT` — §5.2.2.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `len` | `UDINT` | — |
| `level` | `UDINT` | — |
| `infos` | `POINTER TO TcJsonLevelInfo` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `value` | `STRING` | — |

**Description:** This callback method is triggered if a value of the data type STRING was found at the position of the SAX reader. The input/output parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnUdintValue` (METHOD) → `HRESULT` — §5.2.2.2.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `level` | `UDINT` | — |
| `infos` | `POINTER TO TcJsonLevelInfo` | — |
| `value` | `UDINT` | — |

**Description:** This callback method is triggered if a value of the data type UDINT was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

##### `OnUlintValue` (METHOD) → `HRESULT` — §5.2.2.2.8

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `level` | `UDINT` | — |
| `infos` | `POINTER TO TcJsonLevelInfo` | — |
| `value` | `ULINT` | — |

**Description:** This callback method is triggered if a value of the data type ULINT was found at the position of the SAX reader. The input parameter value contains the value found. The SAX parsing procedure is aborted by setting the return value HRESULT to S_FALSE.

#### Samples

##### `IotMqttSampleUsingCallback` (FUNCTION_BLOCK) — §6.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `payload` | `PVOID` | — |
| `length` | `UDINT` | — |
| `qos` | `TcIotMqttQos` | — |
| `repeated` | `BOOL` | — |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `topic` | `STRING` | — |

**Description:** Sample for MQTT communication via a callback method

##### `IotMqttv5Sample` (FUNCTION) → `STRING` — §6.11

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Sensor1` | `REAL` | — |
| `Sensor2` | `DINT` | — |
| `Sensor3` | `BOOL` | — |

**Description:** Sample for MQTTv5 communication via a message queue
