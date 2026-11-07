# 15 — Safety & Security: Boundaries, Sandboxing, Provenance, Audit

> **Status:** work-item document; deleted when `docs/plc/safety.md` +
> `docs/plc/security.md` exist and controls below are implemented and
> pen-tested at the stated level.
>
> **Satisfies:** G11 (the chapter), G6 (containment), G12 (hygiene).
> **Depends on:** all prior. **Informs:** 16, 17.

---

## 1. Threat/safety model

TetherPLC executes **machine-generated and human-authored code on the
control plane of a real machine**. Assets and adversaries:

| asset | threat | primary control |
|---|---|---|
| machine motion/outputs | buggy or malicious user logic drives unsafe state | FSoE boundary (§2), safe-state (doc 03), watchdog (doc 04) |
| the fieldbus/master | PLC process fault or compromise | process isolation (G6), image protocol (doc 03 §9), sandbox (§3) |
| other programs/tenants | cross-program interference | per-instance isolation (doc 04), map ownership (doc 09) |
| program integrity | tampered artifact executes | signing + verification (§4), hash-pinned runbook |
| configuration integrity | map/task edits bypass policy | authority model + audit (§5) |
| credentials/access | unauthorized deploy/force/debug | tether_io auth + channel authZ (§5) |
| supply chain | compromised deps/toolchain | pinning, license manifest, reproducible builds (§6) |

Explicit non-goals v1: IEC 61508/62061-rated user logic; runtime
attestation; multi-tenant hostile-user hardening beyond §3's level;
encrypted artifacts (obfuscation is not a goal); network-facing
exposure of the PLC control channel (loopback/UDS only).

### 1.1 Honesty statement (docs contract)

"A soft-PLC containing user code can never be the safety layer."
Safety functions live in FSoE/safety-rated components under certified
configuration; user logic participates via **read-only safety state
and certified interlock inputs**. Documentation must present this
boundary as a feature, not a footnote — including the §2.3 list of
what user logic must never be asked to guarantee (e-stop chains,
guard locking, safe speed).

## 2. The safety boundary (G11 realization)

### 2.1 Rules

1. **Safety outputs are not programmable.** PDOs flagged
   safety-classified may never map to `%Q` (doc 09 `MAP_SAFETY`) —
   enforced at map build, deploy, and (defense-in-depth) by the
   bridge at runtime.
2. **Safety state is readable**: safety inputs, FSoE connection
   status, safety-terminal states map to `%I` for interlock logic —
   read-only channels; a program cannot write, force, or mask them
   (channel `forceable=false` hard-coded for safety class — the UI
   cannot express it, the resolver denies it).
3. **Safe-state policy belongs to the trusted side**: per-channel
   safe values are bridge/map property (doc 09 §4.3), applied by the
   master — a faulted or hostile PLC cannot veto safe outputs.
4. **No watchdog delegation**: safety-relevant watchdogs (FSoE
   conn supervision, safety terminal watchdogs) run in the certified
   layer, never via user tasks.

### 2.2 What user logic MAY do (designed affordances)

- Interlock logic on non-safety outputs informed by safety `%I`.
- `SAFETY_*` status FBs (read-only catalog entries) for diagnostics
  and HMI display.
- Alarm/derived signals feeding the event journal.

### 2.3 Documentation obligations

Ship a "safety boundary" doc page + in-IDE affordances (safety
channels visually distinct, force UI refuses with explanation, mapping
picker can't select them). A user who never reads docs still cannot
wire a safety output — the constraint is structural.

## 3. Process sandboxing (`tether-plc`)

Default-on (`--no-sandbox` is a debug escape, journaled):

| layer | control |
|---|---|
| user | dedicated `tplc` user, `tether` group (SHM/ctl access only) |
| namespaces | optional mount/netns profile (container mode gets it free) |
| seccomp | syscall whitelist: rt-sleep/futex/epoll/shm/signal/file-i/o-on-allowed-fds + dlopen needs (mmap/mprotect during load only — tightened post-load? — v1: static list incl. load-time calls; a post-load stricter profile is the documented improvement path) |
| capabilities | none (no CAP_*; RT scheduling needs `CAP_SYS_NICE` or RTKit — deploy as `CAP_SYS_NICE` only via AmbientCapabilities, documented) |
| rlimits | `RLIMIT_NPROC`, `RLIMIT_FSIZE` (retain), `RLIMIT_MEMLOCK` bounded, `RLIMIT_NOFILE` bounded |
| FS | read-only root; writable only: retain dir, runbook dir, tmp staging (noexec) |
| network | **no network** — the process has UDS only (ctl + image paths); seccomp denies socket() entirely where feasible |
| watchdog | supervisor respawn (doc 04 §7) + hardware watchdog optional (`/dev/watchdog` hook by the *bridge*, not the PLC — trusted side holds it) |

A compromised/broken program's blast radius: it can corrupt `%Q`
(bounded by ownership — it owns outputs anyway) and its own process.
It cannot touch the fieldbus, filesystem beyond retain, network, or
other programs' memory.

For generated code inside the process: the sandbox boundary is the
**process**, not the module — a malicious .so with arbitrary native
code could attempt syscalls directly; mitigations: artifacts come
through the deploy path (signed/audited), codegen emits no syscall
instructions (verified by a post-link scanner rejecting `syscall`
`int 0x80` outside whitelisted rt stubs — cheap static check, §4.3),
and seccomp bounds the residue. Documented residual risk honestly:
native-code payloads are trusted-code execution by design — the trust
anchor is artifact provenance (§4), not in-process containment.

## 4. Artifact provenance & integrity

### 4.1 Signing (doc 01 §11 mechanism, policy here)

Deploy policies (config `signature.policy`):

- `open` (dev): accept unsigned, warn loudly in UI/log.
- `require` (default): package must carry a valid `signature` section
  (ed25519); unsigned → `TPLC_E_SIG_REQUIRED`.
- `require-trusted`: signature + key must be in the station trust
  store (`/etc/tether/trust/*.pub`); unknown signer → reject.

Key management v1: file-based trust store + `tplcc sign` (keygen/
sign/verify subcommands). Rotation documented; HSM/KMS integration
out of scope (the sig block carries `key_id` so a KMS upgrade later
is format-compatible).

### 4.2 Runbook integrity

`runbook/` entries carry package sha256 + signature status; on
respawn the runtime re-verifies hashes before reload — a tampered
on-disk package cannot auto-resume (it goes ERROR + alarm, never
"runs something else").

### 4.3 Post-link code scanner

`libtplc-pkg` verification includes a static scan of `target_obj`:
disasm-lite pass asserting **no `syscall`/`int`/`sysenter` opcodes**
outside a whitelist (none in v1), no PLT calls outside libc/libm
whitelist, no RWX segments, relocations resolvable. It runs on every
deploy (cheap, one pass) — defense-in-depth against a hand-crafted
`.so` bypassing codegen invariants (a signed package from a malicious
toolchain still can't smuggle syscalls past policy B+).

## 5. Access control & audit

- Control channel: UDS peer creds → OS-level identity; bridge proxies
  to `tether_io` sessions with role mapping: `observer` /
  `engineer` / `commissioner` (doc 11 §8) — deploy/force/suspend/
  clear-require commissioner.
- All mutating ops journal `{timestamp, identity, op, args-hash,
  result}` into the existing `EventJournal` (durable sink) — the
  audit story reuses proven machinery instead of inventing one.
- Rate limits on debug ops + control ops (brute-force/dos surface).
- Session binding: online-change commit and force ops may require
  fresh authority token (step-up) — config `authz.stepup_ops`.

## 6. Supply-chain hygiene

- Pinned toolchain versions (rustc, LLVM, tplc-rt) recorded in the
  `toolchain` package section; reproducibility gate (doc 01 §13)
  makes "this binary came from that source" checkable.
- License manifest generated per release (cargo-about / reuse):
  shipped set ⊆ Apache-2.0/MPL-2.0/MIT/BSD; GPL presence anywhere in
  shipped paths fails CI (G12) — `tools/oracle-*` excluded by
  construction (dev-only, flag-gated).
- Dependencies: no runtime network deps; vendored/pinned where
  feasible; advisory monitoring (cargo-audit, dependabot) on the
  Rust side; C++ side inherits Tether's existing dep discipline.
- The `fbd-model`/map/map schemas versioned and validated —
  malformed inputs are attack surface too (fuzz coverage, doc 14).

## 7. Privacy/data notes

Programs and traces can contain process know-how: package sources are
optional-embedded (doc 01 §9 — IP choice); journal entries record
hash not content; remote compile mode leaves source on the
engineering host by design (device sees only artifacts) — the
IP-sensitive path is the default, not an option you forget.

## 8. Iteration-2 hooks

- Signed *source* provenance (sigstore-style) slots into the
  signature section meta without format change.
- SELinux/AppArmor profiles documented as deploy-optional.
- Safety FB library (certified subset) is the *only* allowed path to
  safety-relevant user-visible function blocks — its plan would live
  here (v2+; needs the certification conversation first, per G11).

## 9. Error codes (owned by this chapter)

`TPLC_E_SIG_* (required/invalid/untrusted), _SEC_SANDBOX,
_SEC_POLICY, _SEC_AUDIT, _PKG_SYSCALL (scanner reject)`

## 10. Acceptance criteria

1. Sandbox profile enforced: runtime runs unprivileged with seccomp
   profile loaded (verifiable via /proc + a syscall-attempt test
   binary being killed).
2. Sign/verify cycle; all three policies enforced; tampered package
   rejected pre-dlopen; trust-store rotation documented+tested.
3. Safety boundary: every attempt to map/force/write a safety
   channel rejected at the stated layer with the stated code.
4. Post-link scanner rejects a hand-modified `.so` containing a
   syscall; accepts the shipped corpus.
5. Audit: every mutating op journaled with identity; journal
   survives restart (durable sink).
6. License scan CI gate: no GPL in shipped artifacts; manifest
   published.
7. Threat-model review signed off: the §1 table's "primary control"
   column is all green-tested.
