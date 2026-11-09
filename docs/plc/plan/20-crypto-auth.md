# 20 — Crypto authentication for updates

> Status: planned (iteration 2). Owner directive: the runtime accepts
> updates **only** from signed execution images.
> Deletion condition: `docs/plc/crypto-auth.md` (as-built) exists and
> the acceptance rows below are green.

## 1. Requirement

Every path that can change the running program — `DEPLOY_*`,
`PREPARE`, `SWAP`, `SHADOW_LOAD`, and any rollback target — accepts a
package only when its **execution image** carries a valid signature
from a trusted key. There is no unsigned or untrusted escape hatch on
an update path; `open`-policy warning behavior is a development
convenience, not an update authorization.

"Execution image" here means the deployable payload: the `target_obj`
module (`.so` on hosted targets, relocatable/PIC object on MCU
targets) plus the metadata the signature already binds. Because the
package signature covers the whole container (TOC + all non-signature
payloads), authenticating the package authenticates the image — no
second signature layer is needed unless a deployment ever splits
image from package (see §4.3).

## 2. What exists today

- `signature` section (`0x60`): ed25519 over `signing_hash` — the
  non-signature TOC entries with `offset` fields zeroed plus the
  non-signature payloads. Layout-independent, so appending the
  signature does not invalidate it; covers `target_obj`, symmap,
  layout, migrate descriptor, everything.
- `tplcc sign keygen | sign | verify`; file-based trust store
  (`<dir>/<hex-keyid>.pub`, `key_id = sha256(pubkey)`).
- `--sig-policy open | require | require-trusted` + `--trust-dir`,
  enforced by `Package::verifySignature` on every package load path
  (boot, deploy, prepare, swap, shadow load).
- Errors: `TPLC_E_SIG_REQUIRED` (-207), `TPLC_E_SIG_INVALID` (-208),
  `TPLC_E_SIG_UNTRUSTED` (-209).

## 3. Gaps to close

1. **Policy strength on update paths.** `open` is the default and only
   logs a warning. **Landed:** `--update-auth strict` upgrades every
   update op (prepare, swap, shadow-load) to `require-trusted`
   semantics regardless of `--sig-policy`; boot keeps its own policy.
   Open question kept: whether `require` (signed, any key) should
   ever be acceptable for updates, or trust is always mandatory.
2. **Key management.** **Landed:** rotation works today (trust-dir
   holds several `<keyid>.pub`; any trusted key signs) and revocation
   is implemented — `<trust-dir>/revoked` lists hex key-ids, checked
   before the store lookup so a revoked signer is rejected even while
   its `.pub` remains. Still open: per-channel policy (safety-backed
   channels may demand a different trust root).
3. **Boot vs update.** State whether first-boot package load also
   requires trust, or whether trust is an update-time control only.
4. **MCU path.** When `18-mcu-path` lands PIC blobs, the signature
   must cover the relocated image or the loader needs its own
   verification step — the ABI shim already resolves functions
   through a table, so signature verification belongs in the mini-
   loader, not the module.
5. **Non-goals.** No PKI/CA, no HSM, no online key fetch, no per-scan
   re-authentication. The trust anchor stays the file trust store.

## 4. Design notes

- Reuse `verifySignature` + trust store; the work is policy plumbing
  and key lifecycle, not new crypto.
- Revocation is a denylist file next to the trust store
  (`<dir>/revoked`); checked before trust-dir lookup so a revoked key
  is rejected even if its `.pub` remains.
- Rotation is additive: trust-dir may hold several keys; a package is
  accepted if *any* trusted, non-revoked key signed it.

## 5. Acceptance criteria

1. With update-auth strict, every update op rejects unsigned or
   untrusted packages with `SIG_*` codes; no flag or config weakens it.
   **Green:** `tests/scan/update_auth.sh` (ctest `update_auth`).
2. Two trusted keys coexist during rotation; packages signed by
   either are accepted. **Green:** same test.
3. A revoked key id is rejected even while its `.pub` is present.
   **Green:** same test.
4. `error_paths`-style coverage for policy × op combinations;
   `security.md` and `threat-model-signoff.md` updated; as-built doc
   `crypto-auth.md` written and this chapter deleted.
   **Partial:** `security.md` updated; per-channel policy and the
   as-built doc remain.
