# Crypto authentication for updates

> As-built reference. Updates (`PROG_PREPARE`, `PROG_SWAP`,
> `SHADOW_LOAD`) accept a package only when its execution image
> carries a valid ed25519 signature under the applicable policy.
> Boot is governed separately by `--sig-policy`.

## Trust domains

| domain | flag(s) | governs |
|---|---|---|
| boot | `--sig-policy`, `--trust-dir` | the `--package`/`--module` loaded at start |
| update | `--update-auth`, `--update-auth-map`, `--trust-dir` | every ctl update op |

The two compose and neither weakens the other: a permissive boot
(`open`) can still demand strict updates, and a strict boot does not
relax update checks.

## Update policies

`--update-auth open` (default): update ops use `--sig-policy`
semantics — `open`, `require`, or `require-trusted`.

`--update-auth strict`: every update op resolves to
`require-trusted` regardless of `--sig-policy` — the package must be
signed by a non-revoked key in the applicable trust store. No flag
or config weakens strict; a weaker per-channel entry is a startup
error.

## Per-channel policy

`--update-auth-map key=policy[@trust-dir]` (repeatable) overrides the
update policy for one ctl channel:

- `key` is a ctl role name (`observer`, `engineer`, `commissioner`)
  or a decimal peer uid (kernel-attested via `SO_PEERCRED`).
- Resolution order: uid entry > role entry > global update policy.
- `policy` is `open`, `require`, or `require-trusted`.
- `@trust-dir` selects a different trust root for that channel —
  e.g. a safety-backed signing authority whose keys are not in the
  global store.

Under `--update-auth strict` every entry must itself be
`require-trusted`; weaker entries and malformed specs fail startup
with exit code 2.

## Signature coverage

The `signature` section (`0x60`, 132 bytes: version, key id,
pubkey, ed25519 signature) signs `signing_hash` — the non-signature
TOC entries with offset fields zeroed plus the non-signature
payloads. That covers `target_obj` (the execution image), symmap,
layout descriptor, migrate descriptor, and manifest — authenticating
the package authenticates the image; no second signature layer
exists or is needed while image and package stay one container.

`key_id = sha256(pubkey)`; the trust store is
`<trust-dir>/<hex-keyid>.pub` files (raw 32-byte pubkeys).

## Key lifecycle

- **Rotation** is additive: the trust dir may hold several
  `<keyid>.pub` files; a package is accepted if any trusted,
  non-revoked key signed it.
- **Revocation**: `<trust-dir>/revoked` lists hex key-ids
  (whitespace-separated). Checked before the store lookup, so a
  revoked signer is rejected with `TPLC_E_SIG_UNTRUSTED` even while
  its `.pub` remains.

## Enforcement points

`Package::verifySignature(policy, trustDir)` runs on every load
path after `Package::open`:

- boot (`--package`): `--sig-policy` + `--trust-dir`;
- `PROG_PREPARE`, `PROG_SWAP`, `SHADOW_LOAD`: the resolved update
  policy (global or per-channel) — followed by `validateForLoad`;
- rollback restores an already-authenticated package, so no
  re-verification is needed.

A raw module path (`.so`, not `.tplc`) carries no signature section:
update ops reject it with `TPLC_E_SIG_REQUIRED` unless the resolved
policy is `open`.

## Errors

| code | meaning |
|---|---|
| `TPLC_E_SIG_REQUIRED` (-207) | policy requires a signature, package has none |
| `TPLC_E_SIG_INVALID` (-208) | malformed section, bad key_id, bad signature, or no crypto backend |
| `TPLC_E_SIG_UNTRUSTED` (-209) | valid signature but signer revoked or absent from the applicable trust store |

## Tooling

`tplcc sign keygen --dir D --name N` creates `N.key` + `N.pub`;
`tplcc sign sign PKG --key K -o OUT` appends the signature;
`tplcc sign verify PKG --trust-dir D` checks offline.

## Known gaps

- **MCU PIC-blob verification**: when the MCU mini-loader lands
  (see `mcu-path.md`), signature verification of the relocated image
  belongs in the loader, not the module — the ABI resolves functions
  through a table, so the loader is the trust boundary.
- **Per-channel identity is uid/role only**: there is no per-package
  or per-program policy dimension; all updates to one runtime share
  the channel rules above.
- **Trust anchor is the file store**: no PKI/CA, HSM, or online key
  fetch by design.

## Coverage

`tests/scan/update_auth.sh` (ctest `update_auth`, 13 checks):
strict rejection of unsigned/untrusted packages on swap and
shadow-load, trusted acceptance, key rotation, revocation,
per-channel trust roots, startup rejection of weaker/invalid map
entries, and the raw-`.so` rejection. `sig_policy` covers the boot
domain.
