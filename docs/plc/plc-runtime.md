# PLC runtime process — as-built

Status of `runtime/tether-plc` + `runtime/host` against
`plan/04-plc-runtime.md`.

## Process

`tether-plc --image <shm> --module <so> | --package <.tplc>
[--ctl <uds>|none] [--retain DIR] [--retain-flush-ms N] [--sim]`

- Attaches to the SHM process image (plc role), validates size.
- `--package` extracts the `target_obj` section to a temp `.so`, then
  the normal `dlopen` path + descriptor validation.
- Single-threaded cooperative scan: `Scheduler::tick` dispatches due
  tasks in priority order inside the main loop; latch/commit hooks go
  through the seqlock image protocol.
- `--ctl none` runs headless (fault injection / tests).

## Control channel (0x0— ops implemented)

`PING`, `RT_STATUS`, `RT_SHUTDOWN`, `PROG_STATUS`, `PROG_LIST`,
`PROG_START`, `PROG_STOP` (schedule gating only), `TASK_STATS`,
`IMAGE_STATS`, `RETAIN_STATUS`, `RETAIN_FLUSH`, `RETAIN_CLEAR`,
`RETAIN_DUMP`, plus `HELLO`/`SET_NOTIFY` session ops.

## Retain store (doc 04 §5)

- `RetainStore` mirrors `retain_region` to
  `<dir>/<program_uuid>.bin` — a second copy, not a blob mmap.
- 64 B header `{magic 'TRET', version, program_uuid,
  retain_layout_hash(lo8), generation, len, payload_crc32,
  header_crc32}` + payload.
- Restore: file magic/version/uuid/layout-hash/len + both CRCs must
  match; payload is copied into the blob before `tplc_init` and
  `ctx_flags.RETAIN_RESTORED` is set — generated init then skips the
  retain-region memset and retain literal inits (verified end-to-end
  across process restart).
- Flush triggers: `svc.retain_dirty` hints drained in the main loop,
  periodic `--retain-flush-ms` (default 1000), stop/shutdown.
  `stage()` memcpys the region into a staging buffer between ticks
  (task boundary); `flush()` writes + `fdatasync`.
- Stats surfaced via `RETAIN_STATUS` (attached/dirty/len/generation/
  flushes/restores/losses).
- v1 simplification: staging and flushing run on the main loop thread,
  not a dedicated flusher thread — bounded since the scan is on the
  same thread; the spec's separate flusher exists for a multi-thread
  scan, which does not exist yet.

## Module-side init (codegen note)

`tplc_init` zeroes the blob in two segments around the retain region;
the retain segment is zeroed + retain literal inits applied only when
`!CTXF_RETAIN_RESTORED`. (An earlier unconditional whole-blob memset
destroyed restored retain data — fixed at P2-01.)

## Not yet

- `PROG_PREPARE/KILL/RESET/UNLOAD` (P2/P3 lifecycle).
- Multi-program scheduling (descriptor supports it; driver is
  single-program).
- Fault policies/respawn (P2-03), runbook, catalog exporter, bridge
  (P2-05/06), watchdog/budget accounting (P2-02).
