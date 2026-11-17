# PROFINET DCP + cyclic IO (`tplc-profinet`)

TetherPLC implements PROFINET **DCP** (Discovery and Configuration
Protocol) natively — this stack lives in TetherPLC, not in Tether.
DCP PDUs (12-byte header + option blocks) are spoken over per-port
UDP transports; on real hardware a Layer-2 Ethertype-0x8892
transport is the deployment path.

## Architecture

```
ST/FBD  FB_ProfinetMaster (libraries/profinet/profinet.st)
          └─ FB_Profinet (builtin, OP dispatch)
               └─ tplc-profinet host service  (TPLC_PROFINET_IO,
                  fallback TPLC_IOLINK_IO / TPLC_CANOPEN_IO /
                  TPLC_EC_IO for port 0)
                    └─ per-port UDP socket
                         └─ PROFINET device / adapter
```

`TPLC_PROFINET_IO` takes a comma-separated list of `host:port` or
`udp://host:port` endpoints (max 4 ports); `PORT` selects one.

## DCP wire format (implemented)

- **Header (12 B)**: FrameID u16 BE | ServiceID u8 | ServiceType u8 |
  XID u32 BE | ResponseDelay u16 | DCPDataLength u16.
  FrameID: `0xFEFD` get/set, `0xFEFE` identify request,
  `0xFEFF` identify response. ServiceID: 3=Get, 4=Set, 5=Identify.
  ServiceType: 0=request, 1=response success, 5=not supported.
- **Block**: Option u8 | Suboption u8 | DcpBlockLength u16 BE |
  BlockInfo/Status u16 BE | data | pad-to-even. Selector (Get)
  blocks carry blocklen=0; value blocks carry info+data.
- **Options**: 1=IP (sub 1=MAC, 2=IP param, 3=full IP),
  2=device properties (sub 1=vendor, 2=name, 3=device ID,
  4=role, 5=options, 6=alias, 7=instance, 8=OEM),
  3=DHCP, 5=control, 0xFF=all selector.
- **Identify** parses name, vendor/device ID, role, instance, OEM
  into the FB's scalar outputs; **Get/Set** round-trip arbitrary
  option/suboption data; block error statuses map to
  `TPLC_PN_E_DEV`.

## FB_Profinet ops

| OP | Operation | Notes |
|----|-----------|-------|
| 1  | open      | UDP socket + peer connect |
| 2  | close     | |
| 3  | state     | `STATUS=1` when ports configured |
| 4  | identify  | → NAME/VENDOR/DEVID/ROLE/DEVINST/OEM |
| 5  | dcp_get   | OPTION/SUB → OD/DLEN |
| 6  | dcp_set   | OPTION/SUB/OD/DLEN |
| 7  | io_exchange | IOOUT/DLEN → IOIN/DLEN |
| 8  | io_cr      | IOCR cyclic APDU exchange (see below) |

`STATUS` carries the service return code; `ERRID` its magnitude.

### OP 8 — IOCR cyclic exchange

`io_cr` frames cyclic data as an IEC 61158-6-10 IOCR APDU:

```
[FrameID u16 BE][C-SDU ≥40B]['PN'+len+record][cycle u16 BE]
[dataStatus u8][transferStatus u8]
```

- `XID` selects the FrameID; `0` keeps the configured value
  (default `0x8000`). A nonzero value updates the port's FrameID.
- The C-SDU carries one provider/consumer record in the same
  `'PN'`-tagged format as OP 7, zero-padded to the 40-octet minimum.
- The request trailer uses DataStatus `0x35`
  (primary-provider / RUN / data-valid / station-OK); the reply's
  `transferStatus` must be `0` (usable) or the call fails with
  `TPLC_PN_E_DEV`.
- Replies are validated: FrameID echo, ≥40-octet C-SDU, `'PN'` tag.
- `PNCYC` receives the device's cycle counter; `PNSTAT` the reply
  DataStatus byte. The controller keeps its own per-port cycle
  counter in the request trailer.

## Scope and honest limits

- **Transport**: UDP carrying the DCP PDU / IOCR APDU
  (post-Ethernet-header). Real PROFINET is Ethertype-0x8892
  Layer-2 — a raw-socket transport is the hardware path (staged).
- **Cyclic IO**: OP 8 implements IOCR APDU framing, but AR
  (Application Relationship) negotiation via DCE/RPC —
  Connect/Parameterization/PrmEnd/ApplicationReady, IOCR object
  dictionary layout, watchdogs, RT-class negotiation — is staged.
  The 'PN' single-record C-SDU is the seam that AR mapping builds
  on; real devices expect per-submodule IOPS/IOCS records.
- **AR transport (implemented)**: `runtime/host/src/Dcerpc.cpp`
  provides the MS-RPCE connection-oriented PDU codec used by
  PNIO-CM — bind/alter_context/auth3/shutdown/request encoders,
  response/fault/bind_ack/bind_nak decoders, PNIO interface UUIDs
  (dea00001–4), and `DcRpcLink`, a UDP transport (port 34964) with
  per-call_id fragment reassembly. Verified against impacket's
  MSRPC* structures and a loopback fake. The PNIO AR *content*
  (FParametersBlock / ARBlockReq / IOCRBlockReq / IODControl NDR
  marshaling and the Connect→PrmEnd→ApplicationReady opnum
  sequence, per scapy `pnio_rpc.py` / p-net) remains staged.
- **DCP coverage**: Identify/Get/Set for the property/IP/DHCP/
  control options; DHCP option subfields and control transactions
  (start/end/signal) are partially mapped.

## Test

`tests/profinet/e2e.sh` runs `tests/profinet/profinet_case.st`
against `tests/profinet/fake_profinet_device.py` — an in-process
UDP device answering DCP Identify/Get/Set, the legacy IO record
primitive, and IOCR APDU exchanges with a device cycle counter.
11/11 checks: open, state, identify (all properties),
get name, set+get alias, record exchange, two IOCR exchanges
(record transform + cycle counter 1→2 + DataStatus 0x35),
close, invalid op.
