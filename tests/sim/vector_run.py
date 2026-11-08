#!/usr/bin/env python3
"""doc 14 §3.1 vector runner: replay JSONL test vectors at two
levels for parity checking:

  L1  tplcc vectors run    -- MIR interpreter, one scan per cycle
  L2  tether-plc-sim --scan -- compiled package on the real runtime,
      one ctl TASK_STEP per cycle (same scan-per-cycle semantics)

Each vector line {id, program, cycles[]} is compiled with tplcc,
converted into a .steps scenario, and executed. Cycle fields:
  i             %I input bytes (hex, from image offset 0)
  t_ns          absolute sim clock for the release
  expect_q      expected %Q bytes after the scan (hex)
  expect_fields blob field expectations by name, e.g. "ton1.ET":"T#500ms"

Field names resolve to blob offsets via the package's SMAP section;
expected bytes are encoded per the field's TYPC tag (i64 ns for
TIME, little-endian ints for the rest). Exit 77 skips when the
build tree, tplcc, or LLVM is unavailable.

usage: vector_run.py [--vectors FILE] [--build DIR]
env:   BUILD (default ./build), TPLCC, LLVM_SYS_211_PREFIX
"""
import json
import os
import re
import struct
import subprocess
import sys
import tempfile

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(
    os.path.abspath(__file__))))
BUILD = os.environ.get("BUILD") or os.path.join(ROOT, "build")
VECTORS = os.path.join(ROOT, "tools", "oracle", "corpus",
                       "vectors.jsonl")
PLC = os.path.join(BUILD, "tether-plc")
SIM = os.path.join(BUILD, "tether-plc-sim")

# ---------------------------------------------------------------- pkg

def pkg_section(data, sec_type):
    """Extract one section payload from a .tplc container."""
    if data[:8] != b"TPLCPKG\x00":
        raise ValueError("bad package magic")
    toc_count = struct.unpack_from("<I", data, 16)[0]
    for i in range(toc_count):
        e = 160 + i * 32
        st, off, size = (struct.unpack_from("<I", data, e)[0],
                         struct.unpack_from("<Q", data, e + 8)[0],
                         struct.unpack_from("<Q", data, e + 16)[0])
        if st == sec_type:
            return data[off:off + size]
    return None


def smap_entries(smap):
    """name -> (blob_offset, type_ref, size) for every SMAP row."""
    if not smap or smap[:4] != b"SMAP":
        return {}
    cnt, stroff, strlen = struct.unpack_from("<III", smap, 8)
    out = {}
    for i in range(cnt):
        r = 32 + i * 64
        noff = struct.unpack_from("<I", smap, r)[0]
        tref = struct.unpack_from("<I", smap, r + 8)[0]
        off = struct.unpack_from("<Q", smap, r + 12)[0]
        size = struct.unpack_from("<I", smap, r + 20)[0]
        end = smap.find(b"\x00", stroff + noff)
        name = smap[stroff + noff:end].decode("utf-8", "replace")
        out[name] = (off, tref, size)
    return out


def typc_tags(typc):
    """type_ref -> tag (doc 01 §6.6: tag u8 at entry +0, 56 B rows)."""
    if not typc or typc[:4] != b"TYPC":
        return {}
    cnt = struct.unpack_from("<I", typc, 8)[0]
    return {i: typc[24 + i * 56] for i in range(cnt)
            if 24 + i * 56 + 56 <= len(typc)}

# ------------------------------------------------------- value codec

_INT_SIZE = {1: 1, 2: 2, 3: 4, 4: 8, 5: 1, 6: 2, 7: 4, 8: 8}
_TIME = re.compile(r"^T#(-?[\d.]+)(ns|us|ms|s|m|h)$")
_TIME_MUL = {"ns": 1, "us": 1_000, "ms": 1_000_000,
             "s": 1_000_000_000, "m": 60_000_000_000,
             "h": 3_600_000_000_000}


def encode_value(tag, raw):
    raw = raw.strip()
    if tag == 0:  # BOOL
        return b"\x01" if raw.upper() == "TRUE" else b"\x00"
    if tag == 11:  # TIME: stored as i64 nanoseconds
        m = _TIME.match(raw)
        if not m:
            raise ValueError(f"bad TIME literal {raw!r}")
        ns = int(float(m.group(1)) * _TIME_MUL[m.group(2)])
        return struct.pack("<q", ns)
    if tag in _INT_SIZE:
        n = int(raw, 0)
        sz = _INT_SIZE[tag]
        signed = tag <= 4
        return n.to_bytes(sz, "little", signed=signed)
    if tag == 9:
        return struct.pack("<f", float(raw))
    if tag == 10:
        return struct.pack("<d", float(raw))
    raise ValueError(f"unsupported field tag {tag}")


def to_hex_comma(b):
    return ",".join(f"{x:02x}" for x in b)

# ---------------------------------------------------------------- run


def llvm_env():
    env = dict(os.environ)
    if not env.get("LLVM_SYS_211_PREFIX"):
        for p in ("/home/uli/llvm21/usr", "/opt/llvm21",
                  "/usr/lib/llvm-21"):
            for cand in ("llvm-config-21", "llvm-config"):
                c = os.path.join(p, "bin", cand)
                if os.path.isfile(c):
                    env["LLVM_SYS_211_PREFIX"] = p
                    env.setdefault("LLVM_CONFIG", c)
                    env["PATH"] = os.path.join(p, "bin",
                                               os.pathsep + env["PATH"])
                    return env
    return env


def main():
    vec_path = VECTORS
    args = sys.argv[1:]
    for i, a in enumerate(args):
        if a == "--vectors" and i + 1 < len(args):
            vec_path = args[i + 1]

    tplcc = os.environ.get("TPLCC") or os.path.join(
        ROOT, "compiler", "target", "debug", "tplcc")
    for f in (tplcc, PLC, SIM):
        if not os.path.isfile(f):
            print(f"SKIP: missing {f}")
            return 77
    if not os.path.isfile(vec_path):
        print(f"SKIP: no vectors file {vec_path}")
        return 77

    env = llvm_env()
    corpus = os.path.dirname(vec_path)
    vectors = [json.loads(l) for l in open(vec_path) if l.strip()]
    work = tempfile.mkdtemp(prefix="tplc_vec_")

    # L1 leg: the same vectors through the MIR interpreter
    l1 = subprocess.run([tplcc, "vectors", "run", vec_path],
                        env=env, capture_output=True, text=True)
    l1_line = [l for l in l1.stdout.splitlines()
               if l.endswith(" fail")][-1:]
    l1_fail = l1.returncode != 0
    print(("PASS" if not l1_fail else "FAIL") +
          " L1 interp: " + (l1_line[0] if l1_line else
                            f"rc={l1.returncode}"))

    npass = nfail = 0
    for v in vectors:
        vid = v.get("id", "?")
        src = os.path.join(corpus, v["program"])
        pkg = os.path.join(work, vid.replace(".", "_") + ".tplc")
        rc = subprocess.run([tplcc, "build", src, "-o", pkg],
                            env=env, capture_output=True)
        if rc.returncode != 0 or not os.path.isfile(pkg):
            print(f"FAIL {vid}: compile rc={rc.returncode} "
                  f"{rc.stderr.decode()[:200]}")
            nfail += 1
            continue

        data = open(pkg, "rb").read()
        syms = smap_entries(pkg_section(data, 0x0020))
        tags = typc_tags(pkg_section(data, 0x0021))

        lines = []
        bad = None
        for k, cyc in enumerate(v["cycles"]):
            parts = ["step", str(k), f"time={cyc['t_ns']}"]
            if "i" in cyc:
                parts += ["in", "0=" + cyc["i"]]
            if "expect_q" in cyc:
                parts += ["expect", "0=" + cyc["expect_q"]]
            for name, val in cyc.get("expect_fields", {}).items():
                if name not in syms:
                    bad = f"unknown field {name}"
                    break
                off, tref, _ = syms[name]
                tag = tags.get(tref, -1)
                try:
                    enc = encode_value(tag, str(val))
                except ValueError as e:
                    bad = f"{name}: {e}"
                    break
                parts += ["expectf", f"{off}={to_hex_comma(enc)}"]
            if bad:
                break
            lines.append(" ".join(parts))
        if bad:
            print(f"FAIL {vid}: {bad}")
            nfail += 1
            continue

        steps = os.path.join(work, vid.replace(".", "_") + ".steps")
        open(steps, "w").write("\n".join(lines) + "\n")
        img = f"/tplc_vec_{os.getpid()}_{nfail + npass}"
        ctl = f"/tmp/tplc_vec_{os.getpid()}_{npass + nfail}.ctl"
        r = subprocess.run(
            [SIM, "--image", img, "--package", pkg, "--steps", steps,
             "--plc", PLC, "--ctl", ctl, "--settle-ms", "800",
             "--scan"],
            env=env, capture_output=True, text=True, timeout=120)
        if r.returncode == 0:
            print(f"PASS {vid}")
            npass += 1
        else:
            tail = "\n     ".join(
                [l for l in r.stderr.splitlines() if l][-6:])
            print(f"FAIL {vid}:\n     {tail}")
            nfail += 1

    print(f"== vectors: {npass} pass, {nfail} fail"
          + (", L1 failed" if l1_fail else ""))
    return 1 if (nfail or l1_fail) else 0


if __name__ == "__main__":
    sys.exit(main())
