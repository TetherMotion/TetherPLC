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

## D-IMPL-10 — PROG_STOP/PROG_START gate scheduling only (doc 04)

P0 control ops: `prog.stop` halts task scheduling, `prog.start` resumes
it — module state and blob are untouched (IEC stop≠reset). A re-initing
`reset` op (cold/warm) is P2. The ctl op surface for `prepare`/`kill`/
`reset`/`unload`/`deploy.*`/`retain.*`/`dbg.*`/`trace.*` is declared in
`tplc_ctl.h` but unimplemented until their phases land.

## D-IMPL-11 — ctl wire details (doc 04 App I)

- Error frames: same `msg_type` as the request, flag bit15
  (`TPLC_CTL_F_ERROR`), payload `{code u32 (positive magnitude),
  subcode u32, str msg}`.
- `hello`: built into the server (pre-dispatch gate); `proto_ver`
  negotiated as `min(client, server)`; `authz` currently always 1
  (full control) — real authz arrives with P8-03.
- Strings on the wire: `u16 len + bytes`, no NUL.

## D-IMPL-12 — ctl is optional at runtime

`tether-plc` keeps running headless if the ctl socket can't be opened
(logs a warning) — matches the spec's "control channel is a service
plane concern; the scan path must not depend on it." Passing
`--ctl none` skips the socket deliberately (used by the fault harness);
an empty `--ctl` value falls back to `<image>.ctl` as before.

## Open items carried forward (not deviations — just not done yet)

- `bad_*` module series (doc 02 §11 fixtures 2): §3.3 prohibition
  enforcement needs the post-link symbol scanner (plan doc 15/P8-04);
  loader-level checks beyond symbol presence aren't implementable at
  dlopen granularity alone.
- Fault-injection harness (P0-09), control channel (P0-06), tether-plc
  binary (P0-07): next in WBS order.
- `_Static_assert` in `TplcImageHeader`: `reserved[112]` + `in_off` field
  alignment verified by the 256 B size assert.

## D-IMPL-13 — `content_hash` covers the TOC (doc 01 §2 amended)

Original spec hashed only section payloads; the TOC's `section_type`,
`flags`, `aux`, `section_version` bytes were then integrity-uncovered —
a tamper could flip an optional section to required or redirect
`aux`→`targets[]` undetected. Amended: `content_hash` = SHA-256 over
the raw TOC bytes followed by all payloads in canonical order.
Implemented in both `tplc-pkg` (Rust) and `tplc::Package` (C++) and
locked by `toc_tamper_detected_by_content_hash` tests on both sides.

## D-IMPL-14 — package_uuid is a 64-byte ASCII field

Doc 01 §2 reserves 64 B for `package_uuid`; implementations store the
UUID's ASCII text (36-char hyphenated UUIDv7) NUL-padded — keeps the
field self-describing in hex dumps and forward-compatible with longer
encodings.

## D-IMPL-15 — PkgError block -200..-215, `TPLC_E_PKG_SECTION_FORMAT` added

`tplc_pkg.h` defines the container error block as C macros; -213 is
named `TPLC_E_PKG_ABI_MISMATCH` (abi.h already owns
`TPLC_E_ABI_MISMATCH = -64`). -215 `SECTION_FORMAT` fills an unassigned
slot for malformed section-internal payloads (bad LAYD magic etc.).
