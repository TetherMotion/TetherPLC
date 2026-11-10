# Tc2_SerialCom

- **Manual:** `TF6340_TC3_Serial_Communication_EN` (Beckhoff TwinCAT 3 PLC library
  documentation; copyright Beckhoff Automation)
- **Category:** Connectivity · **Tier:** T2 · **API entries:** 20
- **Catalog source:** `docs/plc/plan/21-twincat-libraries.md`
  §7 (complete recursive catalog)

**Integration:** Serial ports: needs a serial-port service/driver (termios on host, UART on MCU). Nothing in the runtime touches serial today — this is the canonical gap example.

---

#### PLC API › Function blocks › Send + Receive

##### `ReceiveByte` — §5.1.1.1

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RXBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ByteReceived` | `BOOL` | If ByteReceived=TRUE after the call, then the data byte |

**Description:** The function block ReceiveByte receives a single character from the interface corresponding to the input variable RxBuffer.

##### `ReceiveData` — §5.1.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pPrefix` | `POINTER TO BYTE` | pPrefix is the address of any kind of data structure that is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RXBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `DataReceived` | `BOOL` | DataReceived becomes TRUE as soon as the receive data |

**Description:** ReceiveData receives data of any type from the interface corresponding to the input variable RxBuffer, storing it in the ReceiveData variable. The start and end of the data stream are recognized by various mechanisms, which can be combined with one another: • Prefix If a variable is passed to the input variable prefix, the first characters of the received data must be the same as this prefix. Other characters are discarded. If no prefix is supplied (null), the receive data starts with the first received character. • Suffix If an input variable Suffix is supplied, the input data is read until the end of the receive data agrees with the suffix. If the received data reach the maximum length SizeReceiveData, a COMERROR_DATASIZEOVERRUN error is generated. • Block size If no suffix is specified, characters are received up to SizeReceiveData. • Timeout If a timeout is passed to the function block, characters are received until a correspondingly large time

##### `ReceiveString` — §5.1.1.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Prefix` | `STRING PLC API` | If a string is supplied to the input variable Prefix, the first |
| `Prefix` | `STRING` | If a string is supplied to the input variable Prefix, the first |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ReceivedString` | `STRING` | — |
| `RXBuffer` | `ComBuffer` | — |
| `ReceivedString` | `STRING(255)` | — |
| `RXBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `StringReceived` | `BOOL` | As soon as the StringReceived output becomes TRUE, the |
| `StringReceived` | `BOOL` | As soon as the StringReceived output becomes TRUE, the |

**Description:** The function block ReceiveString receives a string of characters from the interface corresponding to the input variable RxBuffer, storing it in the output variable ReceivedString. The start and end of the character string are recognized by various mechanisms, which can be combined with one another.

##### `SendByte` — §5.1.1.5

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SendByte` | `BYTE PLC API` | Das zu sendende Zeichen/Byte. Inputs/outputs |
| `TxBuffer` | `ComBuffer [} 64]` | Send data buffer that corresponds to the interface used. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TxBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | For as long as the output Busy=TRUE, the transmission is |

**Description:** SendByte sends a single character to the interface that corresponds to the input variable TxBuffer (of type ComBuffer [} 64]).

##### `SendData` — §5.1.1.6

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `pSendData` | `POINTER TO BYTE` | pSendData is the address of the send data, and is |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TxBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | For as long as the output Busy=TRUE, the transmission is |

**Description:** SendData sends the content of a variable of any type to the interface that corresponds to the input variable TxBuffer (of type ComBuffer [} 64]).

##### `SendString` — §5.1.1.7

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `SendString` | `STRING` | The character string to be sent. Inputs/outputs |
| `TxBuffer` | `ComBuffer [} 64]` | Send data buffer that corresponds to the interface used. |
| `SendString` | `STRING (255)` | The character string to be sent. Inputs/outputs |
| `TxBuffer` | `ComBuffer [} 64]` | Send data buffer that corresponds to the interface used. |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TxBuffer` | `ComBuffer` | — |
| `TxBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | For as long as the output Busy=TRUE, the transmission is |
| `Busy` | `BOOL` | For as long as the output Busy=TRUE, the transmission is |

**Description:** SendString sends a character string to the interface that corresponds to the input variable TxBuffer (of type ComBuffer [} 64]).

##### `ClearComBuffer` — §5.1.1.9

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Buffer` | `ComBuffer` | — |

**Description:** The internal PLC communication Buffer (of type ComBuffer [} 64]) is cleared.

#### PLC API › Function blocks › Configuration

##### `ComReset` — §5.1.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at the Execute input resets the connected |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | The Done output becomes TRUE when the function has |

**Description:** ComReset resets the connected serial hardware. This clears the internal hardware send and receive buffers. The function block supports a variety of serial hardware, such as the serial PC interface and the serial Bus

##### `KL6Configuration` — §5.1.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at the Execute input resets the connected |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | The Done output becomes TRUE when the function has |

**Description:** The function block KL6Configuration initializes and configures a serial Bus Terminal KL6xxx.

##### `KL6ReadRegisters` — §5.1.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at the Execute input starts the read process. |
| `FirstRegister` | `UINT` | Specifies the first register to be read. From this register number (which may range between 1 and 64), |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | The Done output becomes TRUE when the function has |

**Description:** The function block KL6ReadRegisters reads one or several registers of a serial Bus Terminal KL6xxx.

##### `KL6WriteRegisters` — §5.1.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Execute` | `BOOL` | A rising edge at the Execute input starts the read process. |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Done` | `BOOL` | The Done output becomes TRUE when the function has |

**Description:** The function block KL6WriteRegisters writes data to one or several registers of a serial Bus Terminal KL6xxx.

#### PLC API › Function blocks › Background Communication

##### `SerialLineControl` — §5.1.3.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Mode` | `ComSerialLineMode_t` | The Mode input specifies unambiguously which serial [} 72] hardware is being used. |
| `pComIn` | `POINTER TO BYTE` | Universal pointer to the input variable of the process data |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TxBuffer` | `ComBuffer` | — |
| `RxBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Error` | `BOOL` | This output Error goes TRUE as soon as an error occurs. |
| `ErrorID` | `ComError_t [} 70]` | The ErrorID output provides an error code when an error |

**Description:** The SerialLineControl function block looks after the communication between a serial interface (KL60xx, EL60xx or COM interface) and the PLC. The function block is called cyclically, and places received data into the RxBuffer. Data made available in the TxBuffer transmit buffer is sent to the interface at the same time.

##### `SerialLineControlADS` — §5.1.3.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `Connect` | `BOOL` | To initialize a connection to a serial port, Connect TRUE |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TxBuffer` | `ComBuffer` | — |
| `RxBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `PortOpened` | `BOOL` | This output indicates whether the selected serial port is |

**Description:** The function block SerialLineControlADS handles the communication between a virtual serial interface and the PLC. The function block is called cyclically, and places received data into the RxBuffer. Data made available in the transmit buffer TxBuffer is sent to the interface at the same time.

#### PLC API › Function blocks › 3964R + RK512 protocols

##### `3964R` — §5.1.4.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `TXstart` | `BOOL` | TXstart is edge-triggered, so that before a new attempt |
| `Send` | `BOOL` | With a positive edge at the Send input, data from the |

**VAR_IN_OUT / Inputs-Outputs**

| Parameter | Type | Description |
|---|---|---|
| `TXdata` | `P3964buffer;    (* application data (to send)*)` | — |
| `RXdata` | `P3964buffer;    (* application data (received)*)` | — |
| `TxBuffer` | `ComBuffer;      (* buffer between protocol software and terminal *)` | — |
| `RxBuffer` | `ComBuffer;      (* buffer between protocol software and terminal *)` | — |
| `TxBuffer` | `ComBuffer` | — |
| `RxBuffer` | `ComBuffer` | — |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `Busy` | `BOOL` | After the transfer, the function block switches to the basic (* send or fetch (active) mode *) (* receive (passive) mode only *) |
| `Busy` | `BOOL` | Busy becomes TRUE as soon as the function block |

**Description:** The function block P3964R handles the 3964R protocol. The background communication, which is handled by a separate function block SerialLineControl [} 51] decides on the hardware interface used. The associated data buffers TxBuffer and RxBuffer (of type ComBuffer [} 64]) are transferred to the function block P3964R.

#### PLC API › Functions › Help Functions

##### `ASC` (FUNCTION) → `BYTE` — §5.2.1.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `str` | `STRING` | — |

**Description:** The function Asc returns the ASCII code of the first character of the input sting as one byte.

##### `CHR` (FUNCTION) → `STRING` — §5.2.1.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `c` | `BYTE` | — |

**Description:** The function Chr returns the character corresponding to the ASCII code in the input variable c to a string.

#### PLC API › Functions › Conversion functions

##### `ComError_TO_TcEventEntry` — §5.2.2.1

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eErrorId` | `ComError_t [} 70]` | Error code to be converted. |
| `stEventEntry` | `REFERENCE TO` | Event definition as a result of the conversion TcEventEntry |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `ComError_TO_TcEv BOOL` | `If the conversion is successful, the function returns TRUE. entEntry` | — |

**Description:** The ComError_TO_TcEventEntry function converts errors that can occur at various function blocks(SerialLineControl [} 51], SendData [} 41], ReceiveData [} 34], ...) into an event definition.

##### `P3964RError_TO_TcEventEntry` — §5.2.2.2

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eErrorId` | `P3964R_Error_t` | Error code to be converted. |
| `stEventEntry` | `REFERENCE TO` | Event definition as a result of the conversion TcEventEntry |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `P3964RError_TO_T BOOL` | `If the conversion is successful, the function returns TRUE. cEventEntry` | — |

**Description:** The P3964RError_TO_TcEventEntry function converts errors that can occur at the P3964R [} 54] function block to an event definition.

##### `RK512Error_TO_TcEventEntry` — §5.2.2.3

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `eErrorId` | `RK512_Error_t` | Error code to be converted. |
| `stEventEntry` | `REFERENCE TO` | Event definition as a result of the conversion TcEventEntry |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `RK512Error_TO_Tc BOOL` | `If the conversion is successful, the function returns TRUE. EventEntry` | — |

**Description:** The RK512Error_TO_TcEventEntry function converts errors that can occur in the RK512 [} 57] function block to an event definition.

##### `SerialLineControlADSErr_TO_TcEventEntry` — §5.2.2.4

**VAR_INPUT / Inputs**

| Parameter | Type | Description |
|---|---|---|
| `nErrorId` | `UDINT` | Error code to be converted. |
| `stEventEntry` | `REFERENCE TO` | Event definition as a result of the conversion TcEventEntry |

**VAR_OUTPUT / Outputs**

| Parameter | Type | Description |
|---|---|---|
| `SerialLineControlAD BOOL` | `If the conversion is successful, the function returns TRUE. SErr_TO_TcEventE ntry` | — |

**Description:** The SerialLineControlADSErr_TO_TcEventEntry function converts errors that can occur in the SerialLineControlADS [} 52] function block to an event definition.
