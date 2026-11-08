# As-Built: Live Attach over `tether_io` (`tplc-io`)

> Status: implemented (doc 16 §7 iteration-2, partial). A standalone
> `tplc-io` process bridges a live `tether-plc` `ctl.sock` into a
> `tether_io` server over TCP. WebSocket/browser transport is not
> included (Tether's WS transport is client-side); a browser gateway
> remains future work.

## Architecture

```
tether-plc ── ctl.sock ── tplc-io ── TCP (SLIP-framed) ── tether_io clients
 (scan loop, ownership)   (provider)                    (CLI, probe, app)
```

- `tether-plc` keeps authoritative execution and control ownership.
  `tplc-io` never touches the scan loop or the process image; every
  op is a `ctl.sock` request round-trip, mutex-serialized so the
  multi-threaded `Server` sessions cannot interleave frames.
- `tplc-io` compiles Tether's `tether_io` sources into a local
  static lib (`tplc_tether_io`): `Protocol`, `Registry`, `Session*`,
  `Server`, `TcpTransport`, `TetherIOClient`, logging, BLAKE3,
  libSLIPspeed. `TETHER_DIR` points at a Tether checkout; the
  provider and tests are skipped when unset. `WsTransport` and the
  serial transport are compiled out (`TETHER_IO_TRANSPORT_WS=0`,
  `TETHER_IO_TRANSPORT_SERIAL=0`).
- Wire framing is SLIP-over-TCP (`SlipFramedTransport` clientside),
  the same framing `tether_io_client` uses for `tcp://` URLs.

## Surface

**Functions** (group `plc`, ~38 entries). Every ctl op is exposed as
a typed function; complex replies are JSON strings:

- lifecycle: `prog.start/stop/kill/reset/unload/prepare/rollback/
  status` — strict transitions propagate `ctl 501 current=N …`
- debug: `dbg.state`, `dbg.fields`, `dbg.state_diff`, `dbg.resolve`,
  `dbg.read/write`, `dbg.force/unforce`, `dbg.forces`,
  `dbg.watch_list`, `dbg.breaks`, `dbg.break_add`, `dbg.break_expr`,
  `dbg.break_clear`
- tasks: `task.suspend/resume/step`
- watches: `watch.add/get/rm`
- trace: `trace.conf/read/clear/status`
- retain: `retain.status/flush/dump`
- management: `journal.tail`, `map.get`, `image.stats`, `blob`

ctl errors surface as `remote error 13: ctl <code>: <msg>` — the
provider never masks control-plane failures.

**Signals** (group `plc`): `status.state` (U8), `status.cycles` (U64),
`status.epoch` (U64), `status.fault_code` (I32), `status.force_count`
(U32), `status.swap_stage` (U8), `blob` (Binary, whole program blob,
`maxValueSize` advertised). Signals read through `PROG_STATUS` /
`DBG_READ` on demand; there is no server-side cache today.

## Schema model

One scalar `SchemaNode` per used `ValueType` is installed in a
global `SchemaGraph` + `SchemaCatalog` (V6). Schema keys use
`0x50` (`'P'`) in byte 0 and the `ValueType` tag in byte 1. All
function parameters, returns, and signals carry `schema` +
`schemaSlot` so V6 clients decode by slot.

Value encoding follows the schema codec in both directions:

- returns are produced via `encodeNative` — a `present=false`
  function returns truly empty bytes (`okVoid`)
- arguments arrive schema-wire encoded and are decoded via
  `decodeSchemaValue` with a raw-LE fallback for flat clients
- the `blob` signal's `varReadFn` emits `varint len + bytes` —
  the session validates the value against the schema wire format

## Security boundary

- The ctl authz check runs on the **provider's** peer credentials:
  every tether_io call inherits the role of the uid that runs
  `tplc-io`. Remote-client identity cannot reach `SO_PEERCRED` —
  per-remote-user roles need a separate tether_io-side policy and
  are not implemented.
- Default bind is `127.0.0.1`; `--bind` exposes other addresses.
  TCP tether_io has no transport authentication — non-loopback
  exposure must sit behind a network boundary.
- All ctl authorization failures (`401 insufficient role`) and
  audit journaling behave exactly as for local clients — denials
  are journaled with the provider's uid.

## Determinism

The provider holds no locks into the scan loop; `ctl.sock` op
dispatch is bounded work per request, identical to local `tplc-ctl`
invocations. `maxClients = 8` caps concurrent sessions.

## Limits

- No per-entry catalog removal — program swaps keep the same
  function/signal surface; stale symbols fail at `dbg.resolve`
  time (`ctl 410`).
- `mgmt_epoch` is bridged onto the tether_io catalog: the provider
  polls `PROG_STATUS` and, on change, re-installs the schema
  catalog (epoch bump → V6 stale-epoch rejection) and registers an
  `epoch.<N>` marker signal (registry revision bump →
  `CatalogChanged` push). The marker signals accumulate one entry
  per program-set change — bounded by deploy frequency.
- `dbg.state_diff` returns the committed-swap migration rules as
  JSON; it errors `ctl 416 no committed swap` until a swap lands.

## Verification

`tplc-io-probe` (in-repo tether_io client) drives the acceptance
sequence: handshake, catalog listing, live signal decode, JSON
function returns, symbol resolve + error path, blob read/write,
force handle lifecycle, task suspend/step/resume, watch lifecycle,
and strict-transition rejections. `ctest tplc_io_attach` compiles a
fixture package, boots `tether-plc`, starts the provider, and runs
the probe end-to-end.
