# 00 — Plan index (residual)

> The durable content of this chapter (system overview, planes,
> conventions, glossary, error-code registry) has migrated to the
> as-built layer:
>
> - **`docs/plc/architecture.md`** — concepts, component map, wire
>   surfaces, glossary, design invariants.
> - **`docs/plc/error-codes.md`** — the consolidated `TPLC_E_*`
>   registry.
>
> This file remains only to index the still-open plan chapters.

## Remaining work-item chapters

| chapter | scope | deletion condition |
|---|---|---|
| `17-milestones.md` | phased roadmap + P10 release sweep | v1 tag; remainder re-plans under iteration 2 |
| `18-mcu-path.md` | no-`dlopen` MCU targets (designed-for material) | `docs/plc/mcu-path.md` exists or first MCU loader lands |
| `20-crypto-auth.md` | updates only from signed execution images | `docs/plc/crypto-auth.md` exists |

Chapters 01–16 and 19 are retired; their as-built specs live in
`docs/plc/` (see `architecture.md` §7 for the map). Acceptance
status for v1 is tracked in `docs/plc/acceptance-matrix.md`.
