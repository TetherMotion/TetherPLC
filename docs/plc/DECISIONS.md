# Implementation decisions & deviations from `docs/plc/plan/`

Running log: every place the implementation deviates from a plan doc, or
resolves an ambiguity the plan left open. Append-only; each entry names the
affected plan doc. Plan docs are corrected in place only when the deviation
is a spec bug (already done for the ones below marked "spec fixed").

## D-IMPL-1 — `TplcImageHeader` atomics (doc 03)

**Plan:** all header fields `_Atomic uN`.
**Implemented:** plain integer fields; all access through
`tplc_img_load/store_u32/u64` which apply `__atomic_*` builtins with
explicit memory orders. Identical semantics (seqlock protocol unchanged),
portable across C and C++, no `std::atomic`/`stdatomic.h` layout concerns.
Plan doc Appendix B updated to match.

## D-IMPL-2 — `TplcTaskDesc` size: 64 B, not 56 B (doc 02, spec bug — fixed)

The field list (name[32] + 4×u8 → 36, u64-align → 40, 2×u64 → 56, 2×u32 →
64) totals 64 B; the doc's `/* 56 B */` comment and `_Static_assert(...==56)`
were wrong. Assert now reads 64 in both the header and the plan doc.

## D-IMPL-3 — `TPLC_IMAGE_MAGIC` (doc 02 App B vs doc 03 §3, spec bug — fixed)

Appendix B defined `'TIMG'` u32-with-version-in-high-bits; doc 03 §3's file
layout has an 8-byte magic `TPLCIMAG` at offset 0. Implemented
`0x47414D49434C5054` ('TPLCIMAG' LE) as a plain u64 magic with a separate
`version` u32 field, matching §3's byte offsets. Appendix B updated.

## D-IMPL-4 — Missing error codes added (doc 02 §12 → App A)

`TPLC_E_ABI_MISMATCH`, `TPLC_E_ABI_EXT_UNMET`, `TPLC_PROG_FAULT` are owned by
doc 02 §12 but were absent from the normative listing. Defined as `-64`,
`-65`, and event code `16` respectively; header and plan doc updated.

## D-IMPL-5 — Image-layer error range (doc 03 §12)

`TPLC_E_IMG_*` defined at `-100..-107` (MAGIC/VERSION/TOO_SMALL/
SEQ_EXHAUSTED/EPOCH/RANGE/IO/STATE) — inside the image header, disjoint
from ABI codes. Event codes `TPLC_IMG_PLC_LOST`/`MASTER_LOST` = 1/2
(non-negative; never return values).

## D-IMPL-6 — `task_local` pointer source deferred (doc 02)

`TplcTaskCtx.task_local` is set to `nullptr` in P0. The ABI descriptor
carries only `task_local_size`; region offsets live in `layout_desc` which
the pure-ABI host doesn't parse yet. Resolution arrives with the package
reader (P1/P2); modules needing task-local storage declare
`task_local_size=0` meanwhile (conformance stubs do).

## D-IMPL-7 — `%Q` commit granularity (doc 03 §5.3)

`Program::executeTask` commits via a caller-supplied hook that receives the
program's `image_out` buffer + declared `image_out_bytes`. Ownership-range
enforcement (per-task disjoint %Q ranges, `TPLC_E_MAP_OWNER`) is a doc-09
map concern — not yet wired; single-program commits are correct today.

## D-IMPL-8 — C dialect: C11, not C23 (doc 02 §2)

Headers are C11 (`_Static_assert`, `__atomic` builtins) with a
`#define _Static_assert static_assert` bridge for C++ — the normative form
from the plan doc is preserved verbatim in source.

## D-IMPL-9 — Host-side error range

Host-only load/lifecycle errors use `-300..` (`TPLC_E_HOST_*` in
`host/Status.hpp`), disjoint from ABI codes (`-1..-65`), image codes
(`-100..`), and module-defined (`-999/-1999`).

## Open items carried forward (not deviations — just not done yet)

- `bad_*` module series (doc 02 §11 fixtures 2): §3.3 prohibition
  enforcement needs the post-link symbol scanner (plan doc 15/P8-04);
  loader-level checks beyond symbol presence aren't implementable at
  dlopen granularity alone.
- Fault-injection harness (P0-09), control channel (P0-06), tether-plc
  binary (P0-07): next in WBS order.
- `_Static_assert` in `TplcImageHeader`: `reserved[112]` + `in_off` field
  alignment verified by the 256 B size assert.
