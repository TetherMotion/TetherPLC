# Threat-model signoff

Maps the doc-15 §1 threat table's "primary control" column to the
concrete tests that exercise each control. This is the acceptance
record for the safety/security chapter; keep it in sync with ctest.

| threat | primary control | covering test(s) | status |
|---|---|---|---|
| buggy/malicious user logic drives unsafe machine state | FSoE boundary: safety-classified channels non-forceable, writes/forces denied at resolver | `safety_boundary` (DEPLOY_MAP `safety` lines → force→-414, write→-415) | covered |
| same | safe-state on fault: image `plc_status`/`last_fault` flags are the master-visible commit | `watchdog_escalation`, `fault_injection` (scenarios A/B), `tplc-image dump` | covered |
| PLC process fault/compromise reaches fieldbus/master | process isolation via SHM image; master unaffected by PLC crash | `fault_injection` scenario B (kill -9, image stays consistent, reattach works) | covered |
| same | in-process sandbox: seccomp syscall whitelist, AF_UNIX-only socket, no_new_privs | `sandbox_enforce` (probe syscall denials), module scanner `module_scan` | covered |
| cross-program/tenant interference | per-instance isolation (own ctl.sock, image, journal); map ownership | `mgmt_surface` (per-instance ctl), `online_change_e2e` (blob/lineage isolation) | partial — multi-instance contention not load-tested |
| tampered artifact executes | artifact signing + trust policy; post-link syscall scan | `sig_policy` (required/invalid/untrusted paths), `module_scan` (evil syscall/import fixtures) | covered |
| configuration edits bypass policy | authority model + durable audit journal | `authz_audit` (denials journaled with uid), `mgmt_surface` (role-gated ops) | covered |
| unauthorized deploy/force/debug | channel authZ via SO_PEERCRED roles; tether_io inherits ctl role | `authz_audit` (observer→-401), `tplc_io_attach` (provider role applies end-to-end) | covered |
| supply chain compromise | pinned toolchain (TOOLCHAIN section), SPDX license manifest gate | `license_manifest` (GPL hard-fail, waiver mechanism), `sig_policy` (hash-pinned artifact) | covered |

## Explicit v1 non-goals (restated from doc 15 §1)

- IEC 61508/62061-rated user logic — the soft PLC is not a safety
  layer; safety functions live in certified components.
- Runtime attestation, artifact encryption, network-facing ctl
  exposure (loopback/UDS only; tether_io binds 127.0.0.1).
- Multi-tenant hostile hardening beyond the §3 sandbox level —
  hence "partial" on cross-tenant interference above.

## Residual gaps worth noting

- The sandbox has no cgroup/namespace/FS isolation — those layers
  are deploy-level (systemd unit/container) and are not shipped
  yet. A reference unit is a P10 field-trial item.
- `TPLC_E_MGMT_AUTHZ` (-502) is declared but never emitted; all
  denials surface as `CTL_AUTHZ` (-401). Either wire it for
  management ops or drop the reservation.
