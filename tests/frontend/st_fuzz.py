#!/usr/bin/env python3
"""st_fuzz.py — ST parser robustness fuzz (doc 14 §5): mutate the
corpus sources (truncate, byte-flip, token-delete, junk-insert) and
feed every mutant to `tplcc check`. The frontend must always answer
with diagnostics — never crash. rc 0/1 is fine; anything else
(signal, panic=101) is a crasher and is copied to the workdir for
reproduction. Exit 77 skips when tplcc is missing.

usage: st_fuzz.py [--mutants N] [--seed S] [--workdir D]
"""
import os
import random
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
CORPUS = os.path.join(ROOT, "tools", "oracle", "corpus")
TPLCC = os.environ.get("TPLCC") or os.path.join(
    ROOT, "compiler", "target", "debug", "tplcc")
WORK = os.path.join(ROOT, "build", "st-fuzz-crashers")


def mutants(src, rng, n):
    b = bytearray(src)
    for _ in range(n):
        m = bytearray(b)
        kind = rng.randrange(4)
        if kind == 0 and m:
            del m[rng.randrange(len(m)):]          # truncate
        elif kind == 1 and m:
            i = rng.randrange(len(m))
            m[i] = rng.randrange(32, 127)           # flip a byte
        elif kind == 2:                             # delete a token span
            words = [i for i, c in enumerate(b) if chr(c).isalnum()]
            if words:
                i = rng.choice(words)
                del m[i:i + rng.randrange(1, 8)]
        else:                                       # insert junk
            junk = rng.choice([b"%%%", b"{{", b"END_PROGRAM END_PROGRAM",
                               b"\x00\x01", b";;;;", b"###"])
            m[rng.randrange(len(m) + 1):0] = junk
        yield bytes(m)


def main():
    n, seed = 40, 0
    a = sys.argv[1:]
    for i, x in enumerate(a):
        if x == "--mutants":
            n = int(a[i + 1])
        elif x == "--seed":
            seed = int(a[i + 1])
    if not os.path.isfile(TPLCC):
        print("SKIP st_fuzz: tplcc missing")
        return 77
    rng = random.Random(seed)
    files = [os.path.join(CORPUS, f) for f in os.listdir(CORPUS)
             if f.endswith(".st")]
    total, crashers = 0, []
    for f in files:
        src = open(f, "rb").read()
        for k, m in enumerate(mutants(src, rng, n)):
            total += 1
            mp = os.path.join("/tmp", f"stfz_{os.getpid()}_{k}.st")
            open(mp, "wb").write(m)
            r = subprocess.run([TPLCC, "check", mp],
                               capture_output=True, timeout=30)
            os.unlink(mp)
            if r.returncode not in (0, 1):
                crashers.append((f, k, r.returncode))
                os.makedirs(WORK, exist_ok=True)
                open(os.path.join(
                    WORK, f"{os.path.basename(f)}.m{k}.st"), "wb").write(m)
    if crashers:
        for f, k, rc in crashers[:10]:
            print(f"CRASHER {os.path.basename(f)} mutant {k} rc={rc}")
        print(f"== st_fuzz: {total} mutants, {len(crashers)} crashers "
              f"in {WORK}")
        return 1
    print(f"== st_fuzz: {total} mutants, 0 crashers")
    return 0


if __name__ == "__main__":
    sys.exit(main())
