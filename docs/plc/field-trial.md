# Field-trial checklist

Pre-deployment checks for a first on-machine trial of tether-plc.
This is a checklist, not a promise: the soft PLC is not a safety
layer (threat-model-signoff.md).

## Before you start

- [ ] Safety functions (e-stop, guard locking, safe speed) are wired
      through certified components — never through this runtime.
- [ ] The machine can be stopped by removing power; the trial is on
      a machine whose outputs cannot hurt anyone if PLC output goes
      to the worst case.
- [ ] `sig_policy` trust store contains only the keys you intend to
      accept artifacts from.

## Deploy envelope (recommended systemd unit)

The runtime ships no cgroup/namespace isolation (in-process seccomp
+ rlimits only). A unit file should supply:

```ini
[Service]
User=tplc
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
PrivateTmp=yes
RestrictNamespaces=yes
TasksMax=64
MemoryMax=256M
Restart=on-failure
RestartSec=200ms
```

- [ ] dedicated `tplc` user exists (not root, not the master user)
- [ ] `--authz <file>` maps ctl uids to roles; verify observer
      cannot force (`tplc-ctl ... force` → `error 401`)
- [ ] `--hang-timeout-ms` set deliberately (default bounds the
      stall→fault→_exit window at ~2x)
- [ ] `--retain <store>` configured if the program uses RETAIN;
      `retain flush` verified before first power cycle
- [ ] journal/audit path is on persistent storage, not tmpfs

## Attach / debug

- [ ] `tplc-io --ctl <sock> --port 43770` binds 127.0.0.1 only;
      any remote attach goes through an SSH tunnel or a
      network-boundary gateway — never expose the TCP port.
- [ ] remember: the provider's own ctl role applies to all its
      clients; run a commissioner provider only on a box where
      every local user is trusted.
- [ ] debug load on a production scan: `debug_stress` shows ops shed
      before tasks slip, but forcing a safety-adjacent channel is
      still a real write — treat force as an operator action
      (journaled).

## During the trial

- [ ] `tplc-image dump` shows `plc_status run`, heartbeats advancing
- [ ] `tplc-ctl prog` shows expected epoch, no growing
      `max_late`/`overruns` trend
- [ ] swap workflow rehearsed on a non-critical instance first:
      stop→prepare→start, or hot `swap` with lineage check
- [ ] journal tail reviewed after any denied op — denials are
      recorded with uid
- [ ] collect `tplc-ctl task` stats + evidence.sh output for the
      trial record
