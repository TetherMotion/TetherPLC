# Security model (as-built)

The envelope around running machine-authored code on the control
plane. Threat model and per-control test signoff:
`threat-model-signoff.md`.

## Process isolation & sandbox

- `tether-plc` is a sidecar process; the IO plane is a SHM process
  image, the control plane a UDS `ctl.sock`. No network sockets —
  the seccomp whitelist permits `socket()` only for AF_UNIX.
- In-process sandbox (`installSandbox` before module load):
  seccomp syscall whitelist (rt-sleep/futex/epoll/shm/signal/
  file-io on allowed fds + dlopen-time mmap/mprotect), rlimits
  (NOFILE 256, FSIZE 64 MiB, MEMLOCK 0, CORE 0), `no_new_privs`.
  Verified by `sandbox_enforce`; `--no-sandbox` escape is warned
  and journaled.
- **Not included**: cgroup/namespace/FS isolation — deploy-level
  (systemd unit/container); `field-trial.md` ships a reference unit.
- Residual risk, honestly: `.so` payloads are trusted native code by
  design; the trust anchor is artifact signing + the post-link
  scanner, and seccomp bounds what remains.

## Artifact provenance

- `.tplc` packages may carry a signature section; `--sig-policy`:
  - `open` (default): signature ignored, warning logged;
  - `require`: must be signed, any key;
  - `require-trusted`: signed + signer key in `--trust-dir`
    (file-based store, `tplcc sign keygen/sign`).
  - `--update-auth strict` upgrades every *update* op (prepare,
    swap, shadow-load) to `require-trusted` regardless of
    `--sig-policy` — the update-authorization control. Boot keeps
    `--sig-policy`; updates are authenticated separately.
    The trust store supports rotation (multiple `<keyid>.pub` files;
    any trusted key signs) and revocation (`<trust-dir>/revoked`
    lists hex key-ids, checked before the store lookup).
  - `--update-auth-map key=policy[@trust-dir]` overrides the update
    policy per channel: `key` is a ctl role name or a decimal peer
    uid, resolved uid-entry > role-entry > global. `@dir` selects a
    different trust root for that channel (e.g. a safety-backed
    signing authority). Under `strict` every entry is floored at
    `require-trusted`; a weaker entry is a startup error.
  - A raw module path (`.so`, not `.tplc`) carries no signature
    section: update ops reject it with `SIG_REQUIRED` unless the
    resolved policy is `open`.
- Verification happens before `dlopen` (`sig_policy` test covers
  all three policies + tamper paths; `update_auth` covers strict,
  rotation, revocation, per-channel roots, and the raw-module
  rejection).
- Post-link scanner (`ElfScan`): rejects `.so` payloads containing
  raw `syscall`/`int80` instructions or forbidden imports
  (`module_scan` fixtures: evil syscall/int80/import/inet, good
  immediates false-positive check).
- `TOOLCHAIN` section records `{tplcc, rustc, llvm, host}` in every
  package for provenance.

## Authority model & audit

- ctl ops carry a minimum role (`op_min_role`): **observer** =
  read-only; **engineer** adds debug-state mutation that cannot
  move the program (watch sets); **commissioner** owns deploy,
  lifecycle, suspend, write, force, break, trace-arm, retain,
  shutdown. Unknown ops → commissioner (fail-safe).
- Peer identity is `SO_PEERCRED` on the UDS; `--authz <file>` maps
  `uid role` (observer/engineer/commissioner), default
  commissioner, file can downgrade.
- Every mutating op is journaled to a durable audit log with uid +
  op + result (`authz_audit` test: observer → -401, denial
  recorded).
- `tplc-io` (tether_io attach) is itself a ctl peer: its own role
  covers all its clients — remote identity does not reach
  `SO_PEERCRED`. TCP binds loopback; anything wider needs a
  boundary (tether-io-attach.md).

## Supply chain

- `tools/license_manifest.py` — SPDX-aware gate over
  `cargo metadata` (non-dev graph) + C++ inventory; GPL-family
  licenses hard-fail; documented waivers live in
  `license_waivers.txt` (`license_manifest` ctest).
- matiec oracle is **not linked** — downloaded at build time by a
  separate adapter, used only to produce test vectors
  (`gpl_hygiene` test scans for contamination).

## Honest limits

- No runtime attestation, no artifact encryption (obfuscation is
  not a goal), no network-facing ctl exposure by design.
- `TPLC_E_MGMT_AUTHZ` (-502) declared but unemitted — all denials
  surface as `CTL_AUTHZ` (-401).
