# CANopen / CoE object-dictionary access (`tplc-canopen`)

TetherPLC reaches the CANopen **object dictionary** (CiA 301) through
Tether's EtherCAT **CoE** (CANopen over EtherCAT) mailbox — the same
protocol family CANopen uses on CAN, carried here over the EtherCAT
master's SDO mailbox. This is *not* a CAN-bus master: there is no
SocketCAN interface, no NMT/SYNC/EMCY/heartbeat on a CAN wire. PDO
mapping is a separate surface (`machine.pdo.map` on the same server).

## Architecture

```
ST/FBD  FB_CoSdo (libraries/canopen/canopen.st)
          └─ FB_CanOpen (builtin, OP dispatch)
               └─ tplc-canopen host service  (TPLC_CANOPEN_IO,
                  fallback TPLC_EC_IO / TPLC_MOTION_IO)
                    └─ Tether IO  machine.sdo.read / .write
                         └─ MachineSdoService → EtherCatSdoAccess
                              → EtherCAT CoE SDO mailbox
```

The service is a thin synchronous adapter: it discovers
`machine.sdo.read`/`machine.sdo.write` in the Tether IO function
catalog at connect time and encodes the tagged `SdoRequestV1` /
`SdoWriteRequestV1` structs by hand (varint field-count, then
`[key varint][len varint][bytes]` — identical to
`tether::io::machine::detail::encodeTagged`).

## Server side (Tether)

`tether_io_ec_server` exposes the surface in two modes:

- `--simulate` (default): in-memory CiA 402 fleet +
  `SimulatedSdoAccess` dictionary — no hardware needed.
- `--master-iface` / `--master-udp`: live CoE wrappers
  (`EtherCatSdoAccess`, `EtherCatPdoAccess`,
  `EtherCatSupervisorAccess`, `EtherCatDiagnosticsAdapter`) against a
  real EtherCAT master.

**Authentication:** `machine.sdo.read` requires an authenticated
session and `machine.sdo.write` requires technician-or-above (audited,
rate-limited to one write per 50 ms). Plain TCP/SLIP carries no
credential handshake, so the server gains `--session-role ROLE`
(`observer|operator|technician|engineer|admin`) and
`--session-actor NAME`: when set, every accepted session is assigned
that authenticated identity via the new `ServerConfig::sessionInitFn`
hook. This is opt-in — dedicated PLC-bridge listeners only; WebSocket
deployments keep bearer-token auth.

## PLC side

`FB_CanOpen` (builtin, no library needed) — synchronous OP dispatch,
call once per operation (each call is an IO round-trip, **not** a
realtime-cyclic operation):

| Member | Dir | Meaning |
|---|---|---|
| `OP` | in | 1=sdo_read, 2=sdo_write, 3=state |
| `SLAVE`/`INDEX`/`SUB` | in | EtherCAT slave position, object index, subindex |
| `VERIFY` | in | write: request server-side post-write readback |
| `DLEN` | in/out | write: payload length; read: cap hint (0→256) / bytes read |
| `DATA` | in/out | `ARRAY[0..255] OF BYTE` payload buffer |
| `STATUS`/`ERRID` | out | service return code / magnitude |
| `ABORT` | out | CoE SDO abort code (0 on success) |

`libraries/canopen/canopen.st` provides `FB_CoSdo`, the FBD-catalog
wrapper: `EXEC` rising edge → one op, `DONE` pulse, `WDATA`/`RDATA`
payload arrays, `RLEN`/`ABORT`/`STATUS`/`ERRID` outputs.

## Return codes

`0` ok · `-1` bad params · `-2` transport/server error ·
`-3` no endpoint/catalog · `-4` slave CoE abort (`ABORT` set, e.g.
`0x06020000` object-does-not-exist) · `-5` malformed reply ·
`-6` service unavailable.

## Tests

`tests/canopen/e2e.sh` — builds the case, starts
`tether_io_ec_server --session-role technician`, replays
`canopen_sdo_case.st` through `tether-plc-sim` with
`TPLC_CANOPEN_IO`: state query, seeded-object read (`0x6041/0` →
`{0x50,0x02}`), verified write + readback (`0x6060/0`), abort
propagation on a missing object, unverified write, invalid-op
rejection. The two writes are spaced by a scan-count delay because
the server rate-limits writes to one per 50 ms of *real* time.

## Limitations / not yet covered

- No CAN physical layer: NMT, SYNC, EMCY, heartbeat/guarding, and
  cyclic PDO exchange on a CAN bus are out of scope (US-022's full
  acceptance criteria). PDO *mapping* exists server-side via
  `machine.pdo.map` and could get a `tplc-*` wrapper next.
- Synchronous calls block the calling scan for the IO round-trip —
  invoke from a slow task or on demand, not a fast cyclic task.
- `machine.sdo.write` inherits the server's technician+ role,
  audit journal, and 50 ms rate limit — PLC logic cannot bypass them.
