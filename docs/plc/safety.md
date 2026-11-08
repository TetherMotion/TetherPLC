# Safety boundary (as-built)

> "A soft-PLC containing user code can never be the safety layer."

Safety functions live in FSoE/safety-rated components under
certified configuration. User logic participates via **read-only
safety state and certified interlock inputs**. This page is the
structural statement of that boundary, not a disclaimer.

## Rules

1. **Safety outputs are not programmable.** Safety-classified PDOs
   never map to `%Q` — enforced at map build (`MAP_SAFETY` in
   io-mapping.md), at deploy, and by the bridge at runtime as
   defense-in-depth.
2. **Safety state is readable, never mutable.** Safety inputs, FSoE
   connection status, and safety-terminal states map to `%I` for
   interlock logic. A program cannot write, force, or mask them:
   `forceable=false` is hard-coded for the safety class, and the
   package's `DEPLOY_MAP` section carries `safety <key>` lines whose
   backing blob ranges are denied at the debug resolver —
   `DBG_WRITE` → -415, `DBG_FORCE` → -414 (`safety_boundary` test).
3. **Safe-state policy belongs to the trusted side.** Per-channel
   safe values are a bridge/map property applied by the master; a
   faulted or hostile PLC cannot veto them. The runtime's part is
   honest reporting: watchdog trip → `TPLC_IMG_PLC_FAULT` +
   `last_fault` in the image header, supervisor `_exit` at ~2x
   hang-timeout for respawn (`watchdog_escalation` test).
4. **No watchdog delegation.** Safety-relevant watchdogs (FSoE
   connection supervision, safety-terminal watchdogs) run in the
   certified layer, never via user tasks. A hardware watchdog, if
   fitted, is held by the bridge side, not the PLC.

## What user logic MAY do

- Interlock logic on non-safety outputs informed by safety `%I`.
- Read-only `SAFETY_*` status signals for diagnostics/HMI.
- Alarm/derived signals feeding the event journal.

## What user logic must never be asked to guarantee

E-stop chains, guard locking, safe speed, two-hand control, safe
muting — anything IEC 61508/62061-rated. The UI must present the
boundary structurally (safety channels visually distinct, force UI
refuses with explanation, mapping picker cannot select them), not
as a footnote.

## Honesty notes

- The boundary is enforced at map/deploy/bridge/resolver layers —
  none of them is the certified layer; they are defense-in-depth
  around it.
- A safety-channel read is a *sample*, not a guaranteed-current
  interlock: hard interlocks must live on the trusted side.
