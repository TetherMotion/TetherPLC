# I/O Mapping — Process Image, Located Vars, Catalog Bridge

How PLC variables meet physical I/O: the SHM process image, `%`-located
variables, the bridge that publishes them to the Tether catalog, and
the monitor/force path the web IDE uses.

## 1. The process image

`tether-plc` does not read fieldbus data inside the scan. Instead a
shared-memory region (created by `tplc-image create <shm> <in> <out>`,
backed by `shm_open` — the path is a POSIX shm name like `/tplc_prod`,
not a filesystem path) holds a fixed-layout image:

```
[header][%I inputs][%Q outputs][%M markers][image latch]
```

The EtherCAT/master side writes `%I` before a scan and reads `%Q`
after; the PLC side does the inverse. Synchronization is a seqlock —
readers retry while a writer's sequence counter is odd
(`TPLC_E_IMG_SEQ_EXHAUSTED` bounds retries). The *image latch* is an
atomic snapshot taken at task start so a cycle sees a consistent image
even if the bus writer lands mid-scan (process-image.md).

## 2. Located variables (`%`-addresses)

An ST `VAR … AT %I0.3 : BOOL` or an FBD variable block with an `at`
field is *located*: it does not own storage, it aliases a byte in the
image. `tplc_codegen` lowers reads/writes of located vars into
loads/stores on the image base + offset; the binding itself is recorded
in `io_bindings` (exported via the LAYD/SYMMAP sections).

The image areas follow IEC convention: `%I` inputs, `%Q` outputs,
`%M` markers (flat, non-persistent scratch), `%R` retain is *not* an
image area — retain lives in the state blob + retain store (see
`plc-runtime.md`).

## 3. The catalog bridge (`runtime/bridge`)

The bridge is the piece that makes PLC state visible to the rest of
Tether and therefore to the web UI. It registers PLC values into the
`tether_io` catalog under two name families:

```
plc.<program>.<varName>            param entries (read + maybe write)
plc.image.<I|Q|M>.ch.<name>        signal entries (the located channels)
```

- Regular program vars appear as **param** entries; retain vars are
  read-only (writable flag `0x02` cleared) — the runtime owns their
  persistence, ad-hoc writes would defeat it.
- Located vars appear **both** ways: `plc.<prog>.<name>` for the
  variable view and `plc.image.<I|Q|M>.ch.<name>` for the channel view.
  The compiler also emits an `__io_<…>` shadow name per located var so
  the IDE can distinguish "the program variable" from "the image bit".
- Probe taps: every FBD wire marked `probe` lowers to a hidden
  `__probe_<wireId>` var and therefore publishes as
  `plc.<pou>.__probe_<id>` — the canvas overlays read exactly this key.

Schemas ride in catalog metadata (`schema.name`); kind-11 alias nodes
resolve to the underlying scalar kind so a writer knows the real width
and signedness before encoding.

## 4. Monitor path (IDE watch + canvas overlays)

1. `DataHub.bind(keys)` marks catalog keys bound;
   `pollBoundValues()` resolves each key over signals *and* params
   (params added for PLC support; signals win collisions) and issues
   `client.get`/`getTyped` reads on the poll tick.
2. The IDE binds every `plc.*` param + signal on connect, and
   `mon.ts`'s `probeKeys`/`varKeys` helpers produce the exact catalog
   names from a `Pou` + block/wire — one module owns the naming
   contract (`src/plc/mon.ts`).
3. `EV_PROG_TRANSITION`/`EV_FAULT` over `/events` trigger a coalesced
   status refresh, so a swap is reflected without waiting for a poll.

## 5. Force path (scalar writes)

`writeScalar(client, catalog, key, text)`:

1. Find the catalog entry; require flag `0x02` (writable). Structs,
   strings, and arrays are rejected — force is scalars only.
2. Resolve the entry's schema slot; follow kind-11 aliases to the
   terminal node; require scalar kind (1 or 4) for `scalarType`.
3. `encodeScalarArgument(scalarType, text)` produces the raw bytes at
   the correct width/signedness; `client.setParameter` writes them.

Force therefore writes *through the catalog* — the same path any Tether
tool uses — rather than a PLC-private op. A forced `%Q` var lands in
the output image and is driven on the next scan boundary exactly like
program-computed output (there is no write-lock masking yet; the force
is the last writer before the latch).

## 6. The map validator

`tplc-image` ships `dump`/`unlink` alongside `create`; the compiler's
`io_bindings` + the deployed `.tplc` LAYD let tooling answer "which
program var owns `%I0.3`" without symbol guesswork — the basis for the
map-validation CI step and for the IDE's `plc.image.*.ch.<name>` signal
naming.
