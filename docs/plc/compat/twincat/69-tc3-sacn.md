# Tc3_sACN

- **Manual:** `TF6310_SACN_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Industry · **Tier:** T2 · **API entries:** 28
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** sACN (E1.31) lighting: UDP multicast socket service — the canonical multicast case; depends on the TCP/IP socket service.

---

#### I_sAcnCommunicator

##### `OnDisableSacnSource` (Method) — §6.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sAcnSource` | `I_sAcnSource` | The instance of the sACN source (implementing I_sAcnSource) requesting |

**Description:** This method is called by an I_sAcnSource instance when it is being disabled. The communicator should then perform necessary actions, such as unregistering the source, stopping packet transmission for its universe, and releasing any allocated network resources (e.g., socket handles). It should also notify the source that its socket is no longer prepared if applicable.

##### `OnEnableSacnSource` (Method) — §6.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sAcnSource` | `I_sAcnSource` | The instance of the sACN source (implementing I_sAcnSource) requesting |

**Description:** This method is called by an I_sAcnSource instance when it is being enabled. The communicator is responsible for registering this source, setting up the necessary network resources (e.g., preparing a UDP socket), and then calling OnSocketIsPrepared() on the source to provide it with the socket handle FOR sending packets.

#### I_sAcnReceiver

##### `OnMulticastAddedToSocket` (Method) — §6.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Result` | `BOOL` | A boolean flag indicating the outcome of the multicast group operation; |

**Description:** This method is called by the I_sAcnCommunicator to inform the receiver about the outcome of a request to join a multicast group. Result indicates whether the operation was successful. The receiver should update its internal state (isWaitingMulticastToAdd) accordingly.

##### `OnMulticastDroppedFromSocket` (Method) — §6.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Result` | `BOOL` | A boolean flag indicating the outcome of the multicast group operation; |

**Description:** This method is called by the I_sAcnCommunicator to inform the receiver about the outcome of a request to leave a multicast group. Result indicates whether the operation was successful. The receiver should update its internal state (isWaitingMulticastToDrop) accordingly.

##### `OnSACNSyncPacketReceived` (Method) — §6.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRemoteHost` | `T_IPv4Addr` | The IPv4 address of the remote device (sender) that originated the sACN packet |

**Description:** This method is called by the I_sAcnCommunicator when an sACN Synchronization Packet is received. This packet typically signals timing information from a source, allowing receivers to synchronize their DMX output.

##### `OnSAcnDataPacketReceived` (Method) — §6.2.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRemoteHost` | `T_IPv4Addr` | The IPv4 address of the remote device (sender) that originated the sACN packet |

**Description:** This method is called by the I_sAcnCommunicator when a valid sACN Data Packet (containing DMX values) is received for the universe this receiver is monitoring. The Packet contains the parsed sACN data, and sRemoteHost provides the sender's IP address.

##### `OnSAcnUniDiscoveryPacketReceived` (Method) — §6.2.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRemoteHost` | `T_IPv4Addr` | The IPv4 address of the remote device (sender) that originated the sACN |

**Description:** This method is called by the I_sAcnCommunicator when an sACN Universe Discovery Packet is received. These packets are sent by sACN sources to announce their presence and the universes they are serving, allowing receivers to discover available sACN streams.

#### I_sAcnSource

##### `OnSocketIsPrepared` (Method) — §6.3.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Socket` | `T_HSOCKET` | Externally provided socket handle |

**Description:** This method is called by the I_sAcnCommunicator to provide the sACN source with a valid UDP socket handle (T_HSOCKET). The source should then store this handle and prepare itself for sending sACN packets.

#### Function Blocks

##### `FB_sACN_Receiver` — §7.1

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `isListening` | `BOOL` | Indicates whether the multicast address is added to the |

**Description:** The FB_sACN_Receiver functional block is designed to receive streaming Architecture for Control Networks (sACN) protocol data packets.

#### Function Blocks › FB_sACN_Receiver

##### `BindExternalDmxArray` (Method) — §7.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DmxArray` | `ARRAY [*] OF USINT` | External variable to bind to |

**Description:** Binds an external DMX array to the receiver

##### `OnMulticastAddedToSocket` (Method) — §7.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Result` | `BOOL` | A boolean flag indicating the outcome of the multicast group operation; |

**Description:** Updates the listening state and logs the event when a multicast address is successfully added to the socket.

##### `OnMulticastDroppedFromSocket` (Method) — §7.1.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Result` | `BOOL` | A boolean flag indicating the outcome of the multicast group operation; |

**Description:** This method is called by the I_sAcnCommunicator to inform the receiver about the outcome of a request to leave a multicast group. Result indicates whether the operation was successful. The receiver should update its internal state (isWaitingMulticastToDrop) accordingly.

##### `OnSACNSyncPacketReceived` (Method) — §7.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRemoteHost` | `T_IPv4Addr` | The IPv4 address of the remote device (sender) that originated the sACN packet |

**Description:** This method is called by the I_sAcnCommunicator when an sACN Synchronization Packet is received. This packet typically signals timing information from a source, allowing receivers to synchronize their DMX output.

##### `OnSAcnDataPacketReceived` (Method) — §7.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRemoteHost` | `T_IPv4Addr` | The IPv4 address of the remote device (sender) that originated the sACN |

**Description:** This method is called by the I_sAcnCommunicator when a valid sACN Data Packet (containing DMX values) is received for the universe this receiver is monitoring. The Packet contains the parsed sACN data, and sRemoteHost provides the sender's IP address.

##### `OnSAcnUniDiscoveryPacketReceived` (Method) — §7.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRemoteHost` | `T_IPv4Addr` | The IPv4 address of the remote device (sender) that originated the sACN |

**Description:** This method is called by the I_sAcnCommunicator when an sACN Universe Discovery Packet is received. These packets are sent by sACN sources to announce their presence and the universes they are serving, allowing receivers to discover available sACN streams.

#### Function Blocks

##### `FB_sACN_ReceiverAndMerger` — §7.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Enable` | `BOOL` | Boolean flag to enable or disable the receiver. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `isListening` | `BOOL` | Indicates whether the multicast address is added to the |

**Description:** The FB_sACN_ReceiverAndMerger functional block extends the capabilities of FB_sACN_Receiver by adding functionality to merge DMX values from multiple sources according to specified merge rules.

#### Function Blocks › FB_sACN_ReceiverAndMerger

##### `OnSAcnDataPacketReceived` (Method) — §7.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sRemoteHost` | `T_IPv4Addr` | The IPv4 address of the remote device (sender) that originated the sACN packet |

**Description:** Processes received sACN data packets.

#### Function Blocks

##### `FB_sACN_Source` — §7.3

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SentPackets` | `UINT` | Number of packets successfully sent by the source. |

**Description:** The FB_sACN_Source is a specialized TwinCAT 3 FUNCTION block designed TO generate AND transmit sACN (streaming ACN) data FOR a specific DMX universe. It automatically formats sACN packets with essential details like source name, unique ID, DMX channel values, AND sequence numbers, sending them

#### Function Blocks › FB_sACN_Source

##### `OnSocketIsPrepared` (Method) — §7.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Socket` | `T_HSOCKET` | Externally provided socket handle |

**Description:** Handles the event when the socket is prepared and ready. Sets the provided socket handle and marks the socket as no longer waiting. Logs the receipt of the socket for the specified universe.

#### Function Blocks

##### `FB_sACN_SourceExtDmx` — §7.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sAcnCommunicator` | `I_sAcnCommunicator Interface to the sACN communicator that handles network` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SentPackets` | `UINT` | Number of packets successfully sent by the source. |

**Description:** FB_sACN_SourceExtDmx is an sACN (streaming ACN) source, similar to FB_sACN_Source, but with a key enhancement: it allows binding to an external DMX ARRAY via pointers, offering more flexible data handling without copying large arrays. It manages the generation and transmission of sACN data packets for a specific DMX universe and interacts with an I_sAcnCommunicator for network operations.

#### Function Blocks › FB_sACN_SourceExtDmx

##### `BindExternalDmxArray` (Method) — §7.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `DmxArray` | `ARRAY [*] OF USINT External variable to bind to` | — |

**Description:** Binds an external DMX array to the function block.

##### `OnSocketIsPrepared` (Method) — §7.4.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Socket` | `T_HSOCKET` | Externally provided socket handle |

**Description:** Handles the event when the socket is prepared and ready. Sets the provided socket handle and marks the socket as no longer waiting. Logs the receipt of the socket for the specified universe.

#### Function Blocks

##### `FB_sACN_UDP_Communicator` — §7.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `NetworkAdapte USINT` | `Selects the network adapter index to use for communication. rNum` | — |
| `ReconnectTim TIME` | `Delay before attempting to reopen the socket after a failure. e` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `BindedToNetw BOOL` | `Indicates whether the function block is currently bound to the selected` | — |
| `orkAdapter` | `network adapter.` | — |

**Description:** The FB_sACN_UDP_Communicator continuously tracks active sACN receivers and sources and manages a shared UDP socket for sACN traffic. • Socket management: Automatically opens the UDP socket when at least one receiver or source is enabled and closes it when all are disabled. If opening fails, reconnection attempts are scheduled. • Multicast group control: For sACN receivers (implementing I_sAcnReceiver), joins and leaves multicast groups based on the receivers internal state (e.g., a waiting-to-add/waiting-to-drop flag). • Packet reception: Continuously listens for UDP packets. A packet is treated as sACN only if the ACN Packet Identifier "ASC-E1.17" and the relevant root/framing vectors match. Valid sACN packets (Data, Synchronization, Universe Discovery) are dispatched to the matching I_sAcnReceiver by universe.

#### Function Blocks › FB_sACN_UDP_Communicator

##### `OnDisableSacnSource` (Method) — §7.5.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sAcnSource` | `I_sAcnSource` | The instance of the sACN source (implementing I_sAcnSource) requesting |

**Description:** Handles the disabling of an sACN source by removing it from the queue.

##### `OnEnableSacnSource` (Method) — §7.5.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `sAcnSource` | `I_sAcnSource` | The instance of the sACN source (implementing I_sAcnSource) requesting |

**Description:** Handles the enabling of an sACN source by adding it to the queue.

#### (top level)

##### `F_HighLowArr_TO_UINT` — §8.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Value` | `ARRAY [1..2] OF BYTE` | The 2-byte array to be converted (High byte first). Returns |
| `UINT` | `The resulting 16-bit unsigned integer.` | — |

**Description:** This function converts a byte array consisting of two bytes (represented in Big-endian order) into a 16-bit unsigned integer (UINT). It is primarily used to decode network-ordered data, such as universe numbers or packet lengths, into a format compatible with the TwinCAT environment.

##### `F_UINT_TO_HighLowArr` — §8.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Value` | `UINT` | The 16-bit integer to be converted. Returns |
| `ARRAY [1..2]` | `A byte array where index 1 is the High byte and index 2 is the Low byte. OF BYTE` | — |

**Description:** This function performs the inverse operation by converting a 16-bit unsigned integer (UINT) into a 2-byte array in Big-endian (High-Low) format. This conversion is essential for packing sACN header fields into the correct network byte order before transmission.

##### `F_UNIVERSE_TO_MulticastAdr` — §8.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Universe` | `UINT` | The sACN universe number (1–63999) for which the multicast address is |

**Description:** This function automatically calculates the sACN multicast IPv4 address associated with a specific universe number according to the ANSI E1.31 standard. The address is generated using the pattern **239.255.H.L**, where H and L correspond to the high and low bytes of the universe ID respectively.
