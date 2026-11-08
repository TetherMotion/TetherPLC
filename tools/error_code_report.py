#!/usr/bin/env python3
"""error_code_report.py — error-code coverage map.

Enumerates every TPLC_E_*/TPLC_TASK_E_* code declared in the public
headers, finds where each is emitted in runtime/tools, and where tests
exercise it (by symbol or numeric literal, e.g. `ctl 415` / `-501`).

Usage: tools/error_code_report.py [--json] [--gaps]
Exit 0 always; --gaps prints only codes with no test hit.
"""

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

CODE_HEADERS = [
    "include/tplc/tplc_abi.h",
    "include/tplc/tplc_ctl.h",
    "include/tplc/tplc_image.h",
    "include/tplc/tplc_pkg.h",
    "runtime/host/include/tplc/host/Status.hpp",
    "runtime/bridge/include/tplc/bridge/MapRunner.hpp",
]

# NAME = VALUE   or   #define NAME (VALUE)   or   #define NAME VALUE
# Only TPLC_E_*/TPLC_TASK_E_* are error codes — TPLC_IMAGE_*,
# TPLC_SEC_* etc. are constants that happen to contain "E_".
DECL_RE = re.compile(
    r"(?:^\s*(?:constexpr\s+int\s+)?(TPLC_(?:TASK_)?E_\w+)\s*=\s*(-?\d+)|"
    r"^\s*#\s*define\s+(TPLC_(?:TASK_)?E_\w+)\s*\(?(-?\d+)\)?)",
    re.M)

SENTINELS = {"TPLC_E_MODULE_MIN", "TPLC_E_MODULE_MAX"}
NONERROR = {"TPLC_OK"}

EMIT_DIRS = ["runtime", "tools", "compiler"]
# shell/C++ e2e tests plus Rust unit-test dirs (compiler/*/tests) —
# both legitimately exercise error paths
TEST_DIRS = ["tests"] + sorted(
    str(p.relative_to(ROOT))
    for p in ROOT.glob("compiler/*/tests") if p.is_dir())


def declarations():
    codes = {}
    for rel in CODE_HEADERS:
        text = (ROOT / rel).read_text()
        for m in DECL_RE.finditer(text):
            name = m.group(1) or m.group(3)
            value = int(m.group(2) or m.group(4))
            if name in SENTINELS or name in NONERROR:
                continue
            codes[name] = {"value": value, "declared": rel}
    return codes


def rg_files(pattern, dirs):
    try:
        out = subprocess.run(
            ["rg", "-l", "--no-messages", pattern] + dirs,
            capture_output=True, text=True, cwd=ROOT)
    except FileNotFoundError:
        return []
    return sorted(x for x in out.stdout.splitlines() if x)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--gaps", action="store_true",
                    help="print only codes with no test coverage")
    args = ap.parse_args()

    codes = declarations()
    hdrs = set(CODE_HEADERS)

    for name, c in codes.items():
        v = c["value"]
        # emit sites: symbol referenced outside the declaring headers
        c["emit"] = [f for f in rg_files(rf"\b{name}\b", EMIT_DIRS)
                     if f not in hdrs]
        # test hits: symbol name (strong) OR a numeric rendering with
        # enough context to be a code: "-501", "ctl 415", "code=415",
        # quoted "414", "error 401" — shell tests grep these forms.
        a = -v
        numeric = rf"({v}\b|ctl {a}\b|code[= ]{a}\b|\"{a}\"|error {a}\b)"
        c["tests"] = sorted(set(
            rg_files(rf"\b{name}\b", TEST_DIRS) +
            rg_files(numeric, TEST_DIRS)))

    if args.json:
        print(json.dumps(codes, indent=2))
        return 0

    rows = [(n, c["value"], len(c["emit"]), len(c["tests"]))
            for n, c in codes.items()]
    covered = sum(1 for _, _, _, t in rows if t)
    emitted = sum(1 for _, _, e, _ in rows if e)

    if args.gaps:
        for n, v, e, t in rows:
            if not t:
                print(f"{v:>6} {n:<28} emit_sites={e}")
        print(f"{len(rows)-covered} code(s) with no test hit", file=sys.stderr)
        return 0

    print(f"{'code':>6} {'name':<28} {'emit':>4} {'test':>4}  sites")
    for n, v, e, t in rows:
        sites = ",".join(Path(f).name for f in codes[n]["tests"][:3])
        print(f"{v:>6} {n:<28} {e:>4} {t:>4}  {sites}")
    print(f"\n{len(rows)} codes | {emitted} emitted in runtime/tools | "
          f"{covered} hit by tests ({100*covered//len(rows)}%)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
