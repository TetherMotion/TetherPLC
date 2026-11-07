# 09 — IO Mapping: Image ↔ Fieldbus Binding & the Bridge

> **Status:** work-item document; deleted when `docs/plc/io-mapping.md`
> exists and the bridge drives the doc-03 protocol against real PDO
> mappings.
>
> **Satisfies:** G5 (image decoupling realized), G11 (safety-channel
> exclusion), G8 (symmap→catalog export), D11.
> **Depends on:** 03 (protocol), 02 (bindings), 04 (runtime), 05 (AT
> syntax). **Informs:** 11 (mapping UI), 12, 14, 16.

---

## 1. What the IO map is

The IO map is **station configuration** that binds three namespaces:

```
fieldbus PDO space          %I/%Q image space          program AT space
(slave, sync unit,          (bytes/bits — doc 03)       (%I*/%Q* names — doc 05)
 index.subindex, bitlen)
        └────── map entry ──────┘────── binding ──────┘
```

A program is **image-agnostic** (doc 05: wildcard AT, or fixed AT —
fixed forms must match the map); the map is what ties it to *this*
machine. One package can deploy to different stations with different
maps — the portability story.

Owned by the **station config**, consumed by:

- `tether-plc-bridge` (master side): drives %I writes from PDO inputs,
  %Q reads into PDO outputs, per doc 03.
- `tether-plc` runtime: validates `descriptor.requires` + AT bindings,
  learns ownership ranges.
- Toolchain/web IDE: resolution of `AT %I*` wildcards, watch labels,
  cross-reference ("which PLC var touches this terminal?").

## 2. Map model

### 2.1 Entities

```
IoMap (station artifact; versioned file + served over control channel)
 ├─ meta {name, version, machine_id, created_by, hash}
 ├─ image {in_size, out_size}                      // doc 03 sizing
 ├─ regions[]:
 │    {name, space: I|Q, offset, size,
 │     owner: {program_uuid, task_name} | "master" | "unmapped",
 │     purpose?}
 ├─ channels[]:                                    // the bindings
 │    {id, dir: in|out,
 │     image: {space, byte_off, bit_off?, type},   // typed view
 │     pdo: {slave_id, sm, entry: {index, sub, bitlen}, name?,
 │           transport: ethercat|modbus|sim|virtual, dev_key?},
 │     safe: {policy: zero|last|defined, value?},  // §4
 │     forceable: bool,                            // §12 plumbing
 │     unit?, label?, tags?}
 ├─ program_bindings[]:                            // AT resolution
 │    {program_uuid, at_name?, pattern: "%I*", var_name, → channel_id}
 └─ policy {input_loss: hold_last|zero|stop, hold_max_cycles,
            safe_override_audit: bool}
```

### 2.2 Invariants (validated at map build AND at deploy)

| rule | code |
|---|---|
| every %Q byte has exactly ONE writer (program task \| master) — no overlaps | `TPLC_E_MAP_OWNER` |
| %I bytes may be read by many; written only by bridge | `TPLC_E_MAP_DIR` |
| channel image range ⊂ declared image sizes | `TPLC_E_MAP_RANGE` |
| channel type size ⊂ its image range | `TPLC_E_MAP_TYPE` |
| PDO endpoints exist in the actual bus config (validated against master's slave info) | `TPLC_E_MAP_PDO` |
| **no channel may bind a safety-classified PDO to %Q written by user logic** (FSoE/safety-terminal flag on PDO descriptor; G11) | `TPLC_E_MAP_SAFETY` |
| program AT fixed addresses must land inside a region owned by that program+task | `TPLC_E_MAP_AT` |
| wildcard bindings resolve to channels of matching type | `TPLC_E_MAP_BIND` |
| safe.policy defined for every %Q channel | `TPLC_E_MAP_SAFE` |

The map is **total**: unmapped %Q bytes get implicit
`owner:"unmapped"` entries with `safe=zero` — there is no floating
output space.

## 3. Deterministic packing

Channel → image offsets are assigned either explicitly (authored) or by
the packing algorithm (default):

```
inputs:  sort channels by (slave_id, index, sub) → first-fit by align
outputs: same; then partition owner ranges by (program, task) order
```

Deterministic → the same map builds identical images on any tool —
`map.hash` is a content hash the runtime and bridge both verify (map
skew = `TPLC_E_MAP_EPOCH`).

## 4. The bridge (`tether-plc-bridge`, in the Tether app)

The master-side half of doc 03 — a library the Tether app instantiates:

```
Bridge { ImageServer (shm create/publish),
         MapRunner (per-cycle: PDO→%I, %Q→PDO+safe),
         ProgramSupervisor (spawn/respawn tether-plc per doc 04 §7),
         CatalogExporter (symmap+channels→tether_io schema),
         ControlProxy (UDS client → runtime ctl.sock; curated ops) }
```

### 4.1 Cycle integration

The map runner hooks the master's existing cyclic exchange (DS402/
generic PDO path — same place `CyclicTaskScheduler` motion tasks live;
phase: **after input copy, before output copy**, with the doc-03 §5.1
sequence). At 1 kHz the added cost is two bounded memcpys + seq ops —
budgeted and measured (§10).

### 4.2 Multi-transport

`pdo.transport` abstraction: EtherCAT PDOs (primary), Modbus registers
(mapping to register addresses), `sim` (sim harness writes/reads the
same layout — G7), `virtual` (UI-driven test IO for commissioning).
Each transport contributes its own endpoint descriptor; the image
doesn't care.

### 4.3 Safety gating (G11 realization)

- PDO descriptors carry `safety_class` from the slave/safety config —
  the map *cannot* express user-logic ownership of a safety output
  (build error `MAP_SAFETY`); safety channels may still map to `%I`
  **read-only** for interlocks/monitoring.
- The bridge exposes `safe` policy per channel (doc 03 §6.2
  substitution happens here, in the trusted process — the policy lives
  where it can't be corrupted by PLC faults).

## 5. Wildcard AT resolution (doc 05 §9 end of the story)

- Compile: `AT %I* : INT` records an *unresolved* binding in the
  package (`io_map` section + symmap flag).
- Deploy: the map's `program_bindings` (or an auto-binder matching by
  `{var_name ↔ channel label}` + type) resolves each wildcard →
  concrete image offset → runtime patches its `io_bindings` table at
  program prepare (bindings are data, not code — late binding is
  **deployment config**, not recompilation — key property).
- Fixed AT (`AT %IX0.0`) skips resolution; the map must already grant
  that byte to the program+task (else `MAP_AT`).
- Unresolved wildcard at start = deploy error listing the unbound
  names (never run with holes).

## 6. Symbol export → `tether_io` catalog

`CatalogExporter` converts symmap + channels into `tether_io`
parameter/signal schemas so the web UI sees PLC state natively:

```
plc.<prog>.<qualified.var>     parameter (rw where allowed)
plc.<prog>.task.<name>.stats   signal group (exec/jitter counters)
plc.image.<I|Q>.ch.<label>     signal (raw channel monitor)
plc.<prog>.retain.*            parameter (ro)
plc.runtime.*                  signals (doc 04 counters)
```

- Types map 1:1 from `type_catalog`; structured values expose fields.
- Epoch discipline: catalog epoch bumps on every program swap/reload
  (stale-name handling doc 12); clients resync per V6 semantics.
- **This is the mechanism that makes watch-in-diagram, force UI, and
  scope traces configuration-free** (G8) — no bespoke per-PLC plumbing.

## 7. Map authoring and storage

- File: `iomap.v1.json` (JSON for tooling; a PLCopen-friendly XML view
  can be generated for exchange but JSON is authoritative — consistent
  with doc 06's working/exchange split).
- Authoring paths: (a) hand-written (integrators — full syntax), (b)
  **auto-mapper**: scan bus config → propose channels + labels, human
  approves (web UI flow, doc 11), (c) generated by machine-config
  tooling (Tether `IoConfig`-adjacent).
- Distribution: station config → bridge at startup; runtime gets it via
  control channel (`map.get`/`map.verify`); toolchain fetch for AT
  resolution.
- Versioned + hash-pinned: bridge rejects deploys whose program AT
  references a different map hash than loaded (unless `--map-tolerant`
  override for dev).

## 8. Diagnostics & counters (bridge exports)

`plc.image.*`: latch/commit counts, seq retries, safe-state
substitutions (per channel + total), stale-heartbeat durations,
map epoch. Channel-level: `forced` flags, last-write timestamps.
Structured alarms: `MAP_SAFETY`, `OWNER`, `PLC_LOST` — journaled with
channel context.

## 9. Iteration-2 hooks

- SFC: steps may bind "action enable" bits — channels already typed;
  no model change.
- LD: same %I/%Q surface.
- Redundant/dual-channel IO (future safety-adjacent): `channels[].redundant_with`
  reserved field.
- Hot-plug bus segments: map `regions[].dynamic` reserved; v1 maps
  are static per bridge start (bus topology changes → controlled
  re-map sequence, a documented operator flow not auto-magic).

## 10. Acceptance criteria

1. Map schema + validator: every §2.2 rule has positive+negative
   fixtures; safety-channel binding attempt → `MAP_SAFETY`.
2. Bridge runs doc-03 protocol against real EtherCAT PDO exchange at
   1 kHz: added latency measured ≤ budget; jitter accounted.
3. Wildcard AT: unresolved package + map → bound at prepare; unbound
   names produce the deploy error; same package on two different maps
   verified.
4. Catalog export: UI-visible parameters for a reference program
   appear with correct types; epoch bumps on swap.
5. Safe-state: PLC killed → per-channel policies applied; audit event.
6. Determinism: same map + config ⇒ same image layout + hash on any
   host.
7. `virtual` transport lets a map run a program against simulated
   channels end-to-end in CI.

---

## Appendix J — `iomap.v1.json` schema (normative)

```jsonc
{ "$schema": "https://tether.dev/schemas/iomap.v1.json",
  "schema": "iomap.v1",
  "meta": {"name":"press_line_2","version":"2026-11-07.3",
           "machine_id":"ml-014","created_by":"u.koehler"},
  "image": {"in_size": 1024, "out_size": 1024},

  "regions": [
    {"name":"plc_main_in","space":"I","offset":0,"size":256,
     "owner":{"program":"*"}, "purpose":"plc-visible inputs"},
    {"name":"plc_main_out","space":"Q","offset":0,"size":256,
     "owner":{"program_uuid":"3f9c…","task":"MAIN"}},
    {"name":"master_pt","space":"Q","offset":256,"size":256,
     "owner":"master","purpose":"master pass-through outputs"}
  ],

  "channels": [
    {"id":"ch_start_btn","dir":"in",
     "image":{"space":"I","byte_off":0,"bit_off":0,"type":"BOOL"},
     "pdo":{"transport":"ethercat","slave_id":3,"sm":2,
            "entry":{"index":"0x6000","sub":"0x01","bitlen":1},
            "name":"EL1008 ch1"},
     "label":"start_button","tags":["operator","estop_loop"],
     "unit":null,"safe":{"policy":"zero"},
     "forceable":true,"safety_class":false},
    {"id":"ch_motor_contactor","dir":"out",
     "image":{"space":"Q","byte_off":0,"bit_off":0,"type":"BOOL"},
     "pdo":{"transport":"ethercat","slave_id":3,"sm":3,
            "entry":{"index":"0x7010","sub":"0x01","bitlen":1},
            "name":"EL2008 ch1"},
     "label":"motor_contactor","safe":{"policy":"defined","value":false},
     "forceable":true,"safety_class":false},
    {"id":"ch_pressure_raw","dir":"in",
     "image":{"space":"I","byte_off":8,"type":"UINT"},
     "pdo":{"transport":"ethercat","slave_id":4,"sm":2,
            "entry":{"index":"0x6000","sub":"0x11","bitlen":16}},
     "label":"pressure_raw","unit":"counts"},
    {"id":"ch_axis_enable","dir":"out",
     "image":{"space":"Q","byte_off":16,"type":"BOOL","bit_off":1},
     "pdo":{"transport":"ethercat","slave_id":7,"sm":3,
            "entry":{"index":"0x6040","sub":"0x00","bitlen":16,
                     "bit_pos":1},
            "name":"axis1 controlword"},
     "label":"axis_enable","safe":{"policy":"last"},
     "forceable":false,
     "safety_class":true,
     "note":"safety-classed: map→%Q REJECTED (G11); shown as the
            canonical MAP_SAFETY fixture"}
  ],

  "program_bindings": [
    {"program_uuid":"3f9c…","var":"PROG.start_btn","pattern":"%I*",
     "to_channel":"ch_start_btn","by":"label"},
    {"program_uuid":"3f9c…","var":"PROG.motor","pattern":"%Q*",
     "to_channel":"ch_motor_contactor"}
  ],

  "policy": {"input_loss":"hold_last","hold_max_cycles":1000,
             "force_on_stop":"clear","safe_override_audit":true}
}
```

### J.1 Type table for `image.type`

`BOOL` (bit_off required), `SINT..LINT`, `USINT..ULINT`,
`BYTE..LWORD` (bit-prefixed variants allowed via bit_off+bitlen),
`REAL/LREAL`, `TIME`, `STRING(n)`, `RAW(n)` (opaque bytes —
arrays/structs pass through as raw; typed view comes from symmap).

### J.2 PDO endpoint descriptors per transport

- `ethercat`: `{slave_id|slave_name, sm, entry:{index,sub,bitlen,
  bit_pos?}}` — bit_pos for bit-level PDO entry packing.
- `modbus`: `{unit, table:coil|di|hr|ir, addr, bitlen, wordorder,
  bitorder}`.
- `sim`: `{plant_id, signal}` (simulated-plant channel name).
- `virtual`: `{vkey}` — UI-writable test channels for commissioning.

### J.3 Map hash canonicalization

`map.hash` = SHA-256 over canonical JSON (sorted keys, no whitespace,
LF) of `{image, regions, channels, program_bindings, policy}` —
meta excluded (cosmetic). Both bridge and runtime compute it on load;
mismatch = `TPLC_E_MAP_EPOCH`.
