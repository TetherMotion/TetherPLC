# IO-Link master data-link (`tplc-iolink`)

TetherPLC implements the **IO-Link master** data-link protocol
(IEC 61131-9 / SDCI) natively — this stack lives in TetherPLC, not in
Tether. It speaks the real wire protocol: wakeup bytes, MC/CKT/CKS
frames, the A.1.6 XOR-seeded 8→6 checksum, page-channel
identification, type-2_V cyclic process data, and ISDU on-request
data.

## Architecture

```
ST/FBD  FB_IoLinkMaster (libraries/iolink/iolink.st)
          └─ FB_IoLink (builtin, OP dispatch)
               └─ tplc-iolink host service  (TPLC_IOLINK_IO,
                  fallback TPLC_CANOPEN_IO / TPLC_EC_IO /
                  TPLC_MOTION_IO for port 0)
                    └─ per-port TCP byte transport
                         └─ IO-Link device (or PHY adapter)
```

Each configured endpoint is one IO-Link port; the `PORT` index
selects it. `TPLC_IOLINK_IO` takes a comma-separated list of
`tcp://host:port` endpoints (max 4 ports).

## Wire protocol (implemented)

- **Wakeup**: burst of `0x55` bytes on open puts a sleeping device on
  COM1; inert on always-listening devices.
- **Master frame**: `[MC][CKT][PD-out][OD]` — MC bit7 = read/write,
  bits6-5 = channel (PROCESS `0x00`, PAGE `0x20`, DIAGNOSIS `0x40`,
  ISDU `0x60`), bits4-0 = address/FlowCTRL. CKT bits7-6 = M-sequence
  type, bits5-0 = checksum.
- **Device frame**: `[PD-in][OD][CKS]` — CKS bit7 = event pending,
  bit6 = PD-in invalid, bits5-0 = checksum.
- **Checksum A.1.6**: XOR all octets (seed `0x52`) with the checksum
  octet itself entering with bits 0-5 zeroed, then fold 8→6 bits.
  Verified on every device reply (`TPLC_IL_E_CKS` on mismatch).
- **Identification**: 16 page-channel reads return the DPP1 record
  (PD-in/out descriptors, vendor/device IDs, revision, M-seq caps).
- **Cyclic PD**: type-2_V exchange — PD-out octets up, PD-in octets
  down, CKS flag bits surfaced via `CKSFLAGS`.
- **ISDU**: single-octet on-request read/write per exchange
  (segmented multi-octet ISDU is staged, not yet implemented).

## FB_IoLink ops

| OP | Operation | Notes |
|----|-----------|-------|
| 1  | open      | connect + wakeup; `STATUS=0` on success |
| 2  | close     | |
| 3  | state     | `STATUS=1` when ports configured |
| 4  | ident     | → `IDENT[0..15]` DPP1, `DLEN=16` |
| 5  | pd_exchange | `MSEQ`/`PDOUT`/`DLEN` → `PDIN`/`DLEN`/`CKSFLAGS` |
| 6  | isdu_read | `ADDR` → `OD`/`DLEN` |
| 7  | isdu_write| `ADDR`/`OD`/`DLEN` |

`STATUS` carries the service return code (0 ok, -2 transport,
-5 checksum, -6 malformed, -7 device error); `ERRID` its magnitude.

## Scope and honest limits

- **Transport**: TCP per port. Real IO-Link needs 24 V-class
  signaling — deploy behind a PHY-equipped adapter; a UART/PHY
  transport is the hardware path (staged).
- **M-sequence**: the codec emits type-2_V for cyclic PD; other
  negotiated types are staged. AL negotiation (cycle-time/M-seq
  capability handshake during PREOPERATE) is not yet automated —
  the port operates with caller-supplied lengths.
- **ISDU**: single-octet services only; segmentation/flow control
  for multi-octet parameter access is staged.
- **SIO mode**, diagnosis-channel decoding, and event-driven
  wakeup are staged.

## Test

`tests/iolink/e2e.sh` runs `tests/iolink/iolink_case.st` against
`tests/iolink/fake_iolink_device.py` — an in-process device speaking
the device side of the protocol (page record, deterministic PD-in
pattern, ISDU param table, A.1.6 checksums). 8/8 checks: open,
service state, DPP1 ident, cyclic PD exchange, ISDU write+readback,
close, invalid-op rejection.
