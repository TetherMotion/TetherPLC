# 18 — MCU Path: No-`dlopen` Targets

> **Status:** work-item document; deleted when `docs/plc/mcu-path.md`
> exists OR when the first MCU loader lands (whichever comes first —
> this chapter is *designed-for* material; v1 ships none of it).
>
> **Satisfies:** D10 (design-for-now/implement-later), G2 (very weak
> hardware incl. MCUs), G1 (same IR → same artifact semantics).
> **Depends on:** 01 (package), 02 (ABI), 03 (image semantics),
> 07 (codegen constraints). **Informs:** 17 (post-v1 scheduling).

---

## 1. Why this chapter exists

DESIGN-GOALS commits to "100% performance on very VERY weak hardware,
even MCUs later". MCUs (and some RTOS/bare-metal targets) have no
`dlopen`, no MMU, often no filesystem — the Linux delivery mechanism
doesn't exist. What *does* exist and transfers unchanged:

- the canonical IR → native code for the MCU target (Cortex-M, RISC-V,
  Xtensa… — LLVM covers all Tether's ESP-IDF targets),
- the runtime ABI (doc 02 — deliberately loader-agnostic),
- the process-image *semantics* (doc 03 — latch/commit/consistency),
- the state-blob + migrate model (G4 — MCU memory constraints actually
  strengthen the case for one contiguous relocatable region).

What does not transfer: the delivery mechanism (`.so`+`dlopen`) and
the sidecar-process isolation model (no processes on bare metal —
isolation becomes a *link-time/address-space* question instead).

## 2. Target classes and delivery mechanisms

| class | example | delivery | isolation |
|---|---|---|---|
| MCU, monolithic firmware | STM32 bare metal | program linked into firmware image at build time; update = OTA firmware | none (same address space) — fault wall = watchdog+MPU |
| MCU, loadable module | ESP32 + ELF loader | position-independent `.o`/ELF blob loaded into reserved RAM/PSRAM region by a mini-loader | MPU regions where available |
| RTOS w/ real loader | Zephyr `llext`, ThreadX modules, NuttX ELF | native module loader | RTOS-level |
| hybrid coprocessor | Linux SoC + MCU core (AM64x R5F) | firmware blob to remoteproc; image over RPMsg/shared SRAM | physical — closest analog of the sidecar model! |

### 2.1 The PIC-blob + mini-loader (the interesting one)

Precedent: Flipper Zero `.fap` apps are position-independent ELFs
loaded by a small in-system loader; ESP32 community ELF loaders exist.
Our equivalent:

- `tplcc --target thumbv7em-none-eabihf` emits a `target_obj` variant:
  `aux=PIC_BLOB` — a `ET_REL`-style or flat PIC blob with a **fixup
  table** (GOT-like: the module's external references are exactly the
  `svc` table + rt helpers, resolved by the loader at load time —
  our ABI is already function-table-shaped, doc 02 §2.8, which is
  precisely what makes this easy).
- The mini-loader (part of MCU firmware, ~1–2 KB code): validate blob
  header/hash → place in reserved region → resolve svc table →
  allocate state blob in SRAM → call `tplc_init`. No general dynamic
  linking needed — the ABI's symbol surface is 6 functions.
- **Online change on MCU** is then just: stop task → load new blob →
  `tplc_migrate` → swap pointer — *the same protocol* (doc 08), because
  the hard part was always state layout, not ELF.

### 2.2 The hybrid coprocessor (the elegant one)

On heterogeneous SoCs (AM64x, i.MX8MP, STM32MP1): run the MCU-side
`tether-plc` equivalent on the coprocessor with the process image in
shared SRAM — the doc-03 seqlock protocol works verbatim across
shared memory with cache-coherence handling (the protocol needs only
atomics+fences; add cache maintenance ops for non-coherent memories).
This is the *truest* port of the sidecar model to silicon.

### 2.3 The monolithic fallback

`tplcc` emits a C-linkable object + blob placement descriptor; the
firmware build links it; "online change" degrades to "OTA the module
partition" (A/B slot + same migrate rules on reboot). Weakest
dynamism, still better than today — and identical ABI/artifact
metadata.

## 3. What must be designed NOW (the D10 hooks, already embedded)

| now-decision | where | why |
|---|---|---|
| ABI is C + function-table, never ELF-specific | doc 02 | enables all loaders |
| state blob is relocatable + pointer-free | doc 02 §3 | SRAM placement freedom + migrate |
| package carries `target_obj` with `aux` flag space | doc 01 §2.1 | PIC_BLOB payload without container v2 |
| codegen emits PIC-clean code (no absolute relocs) | doc 07 §4.1 | blob loads anywhere |
| image protocol is offset+seq, not shm-specific | doc 03 §10 | SRAM/RPMsg transports later |
| symmap/layout_desc are target-independent data | doc 01 | same debugging tooling on MCU |
| `svc` table is the only host surface | doc 02 §2.8 | MCU host implements the same 5 functions |
| no libc requirement in generated code | doc 10 §2.4 | bare metal has none |

The anti-pattern to avoid: *anything in generated code or the ABI
that assumes mmap/dlopen/files/threads exist*. The conformance suite's
"MCU-shim host" (doc 02 §11.7 — a static-linked test host with no
dynamic loading) is the standing proof.

## 4. MCU-side runtime (future component: `tplc-mcu`)

A no_std-ish embeddable runtime crate/C-lib implementing:

- the loader (per §2 mechanism),
- the scheduler (cooperative task loop on the RTOS/bare-metal tick —
  deadline model survives; `clock_nanosleep` → platform delay),
- `ImageBackend` for the local transport (SRAM window / RPMsg /
  dual-port; same latch/commit semantics),
- the `svc` table (logging → RTT/UART ring; retain → flash-sector
  store with wear-aware append — the dirty-hint API already exists),
- checkpoint/abort flags identical.

## 5. Constraints & honesty

- **RAM**: state blob + image + code must fit SRAM — package
  `requires` fields (doc 01 §3) become the gate; tiny targets will
  need genuinely small programs (the ABI scales down naturally; the
  IEC program model doesn't demand megabytes).
- **Flash writes**: retain on flash = wear management; v1 retain
  model maps to an append-log region; power-loss window semantics
  get *better* with NVRAM actually.
- **No process isolation**: fault wall = hardware watchdog + MPU
  where present; a hard-faulting program resets the module/MCU —
  the master-side safe-state logic (now the MCU's own image peer)
  still applies.
- **Debug**: the symmap-driven debug plane works over whatever
  transport carries the control surface (UART/SWD/RPMsg) — same
  ops, thinner pipe.

## 6. What this chapter deliberately does NOT decide

- specific MCU families/boards;
- whether the first MCU backend is PIC-blob vs monolithic vs
  coprocessor (the three exist because the *right* one depends on
  the deployment hardware — ABI supports all);
- MPU/isolation specifics per MCU;
- an MCU timeline (it's post-v1 per doc 17).

## 7. Acceptance criteria (for when this activates)

1. `tplcc` emits a PIC-blob or monolithic-object `target_obj` for one
   Cortex-M target; package validates.
2. A mini-loader on that target loads + runs the ABI conformance
   subset (init/cycle/migrate) in <N KB RAM.
3. The MCU-shim conformance host (doc 02 §11.7) stays green — ABI
   never regressed toward Linux-only assumptions.
4. Image semantics demonstrated over a non-SHM transport.
