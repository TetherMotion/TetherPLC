#!/usr/bin/env python3
"""diff_fuzz.py — differential fuzz: our L1 interpreter vs the matiec
iec2c oracle (GPL, subprocess only, downloaded on demand by
ensure_matiec.sh into a cache outside the repo — never linked,
never shipped; its emitted C is compiled into a throwaway runner whose
trace is data).

Per case a random PROGRAM in the common ST subset is generated once and
rendered twice, because the two engines' surface syntax differs:

  ours   : single VAR block, fb outputs read as `inst.PIN` members
  matiec : located vars need their own VAR block, `inst.PIN` member
           access does not exist — outputs are wired `PIN => tmp`,
           and CTU/CTD expose Q/CV where we expose QU/QD/CV

The same random cycle stream (i bytes + t_ns, one scan per cycle) is
fed to a compiled matiec reference runner; its %Q trace becomes the
expect_q of a vector line which `tplcc vectors run` then replays at L1.
A FAIL is a semantic divergence between the engines; fingerprints are
the vector ids `mfuzz.<seed>.c<k>` and are triaged in
known_divergences.txt (each line documented — unknown ids fail).

usage: diff_fuzz.py [--seed N] [--cases N] [--cycles M] [--l2]
                    [--workdir D] [--update-known]
exit: 0 all agree (or only known divergences), 1 new divergence/error,
      77 skipped (matiec or tplcc unavailable)
"""
import json
import os
import random
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
PIN_SHA = "01a3a6c0c29b9aa0322ab6aba6258ebef2e36155"
MATIEC_HOME = os.environ.get("MATIEC_HOME") or os.path.join(
    os.path.expanduser("~"), ".cache", "tplc-oracle", "matiec")
IEC2C = os.path.join(MATIEC_HOME, "src", "build", "bin", "iec2c")
LIB = os.path.join(MATIEC_HOME, "src", "lib")
LIBC = os.path.join(LIB, "C")
TPLCC = os.environ.get("TPLCC") or os.path.join(
    ROOT, "compiler", "target", "debug", "tplcc")
WORK = os.environ.get("MATIEC_FUZZ_WORK") or os.path.join(
    ROOT, "build", "matiec-fuzz")
KNOWN = os.path.join(HERE, "known_divergences.txt")

# fb name -> (input params, outputs as (ours_pin, matiec_pin, kind))
FBS = {
    "CTU": ([("CU", "b"), ("R", "b"), ("PV", "int")],
            [("QU", "Q", "b"), ("CV", "CV", "i")]),
    "CTD": ([("CD", "b"), ("LD", "b"), ("PV", "int")],
            [("QD", "Q", "b"), ("CV", "CV", "i")]),
    "CTUD": ([("CU", "b"), ("CD", "b"), ("R", "b"), ("LD", "b"), ("PV", "int")],
             [("QU", "QU", "b"), ("QD", "QD", "b"), ("CV", "CV", "i")]),
    "R_TRIG": ([("CLK", "b")], [("Q", "Q", "b")]),
    "F_TRIG": ([("CLK", "b")], [("Q", "Q", "b")]),
    "SR": ([("S1", "b"), ("R", "b")], [("Q1", "Q1", "b")]),
    "RS": ([("S", "b"), ("R1", "b")], [("Q1", "Q1", "b")]),
    "TON": ([("IN", "b"), ("PT", "t")], [("Q", "Q", "b"), ("ET", "ET", "t")]),
    "TOF": ([("IN", "b"), ("PT", "t")], [("Q", "Q", "b"), ("ET", "ET", "t")]),
    "TP": ([("IN", "b"), ("PT", "t")], [("Q", "Q", "b"), ("ET", "ET", "t")]),
}


class Gen:
    """One random program; render() produces both dialects."""

    def __init__(self, rng):
        self.ins = [f"i{k}" for k in range(rng.randint(2, 5))]
        self.outs = [f"o{k}" for k in range(rng.randint(1, 5))]
        self.tb = [f"x{k}" for k in range(rng.randint(1, 4))]
        self.ti = [f"n{k}" for k in range(rng.randint(0, 3))]
        self.fbs = []
        for j, t in enumerate(rng.sample(list(FBS), rng.randint(1, 3))):
            self.fbs.append({"inst": f"{t.lower()}{j}", "ty": t})
        self.stmts = []
        self._fill(rng)

    # expression trees -------------------------------------------------
    def bexpr(self, rng, depth):
        r = rng.random()
        if depth <= 0 or r < 0.45:
            pool = [("var", v) for v in self.ins + self.tb]
            pool += [("fb", f["inst"], p[0]) for f in self.fbs
                     for p in FBS[f["ty"]][1] if p[2] == "b"]
            if self.ti and rng.random() < 0.4:
                return ("cmp", rng.choice(["=", "<>", "<", ">", "<=", ">="]),
                        self.iexpr(rng, 1), self.iexpr(rng, 1))
            return rng.choice(pool) if pool else ("litb", rng.random() < .5)
        if r < 0.75:
            return (rng.choice(["and", "or", "xor"]),
                    self.bexpr(rng, depth - 1), self.bexpr(rng, depth - 1))
        return ("not", self.bexpr(rng, depth - 1))

    def iexpr(self, rng, depth):
        if depth <= 0 or rng.random() < 0.5:
            if self.ti and rng.random() < 0.6:
                return ("var", rng.choice(self.ti))
            fb_i = [("fb", f["inst"], p[0]) for f in self.fbs
                    for p in FBS[f["ty"]][1] if p[2] == "i"]
            if fb_i and rng.random() < 0.4:
                return rng.choice(fb_i)
            return ("liti", rng.randint(0, 20))
        return (rng.choice(["add", "sub", "mul"]),
                self.iexpr(rng, depth - 1), self.iexpr(rng, depth - 1))

    def _fill(self, rng):
        # temp init assignments
        for v in self.tb:
            self.stmts.append(("ass", v, self.bexpr(rng, 2)))
        for v in self.ti:
            self.stmts.append(("ass", v, self.iexpr(rng, 1)))
        # fb calls (inputs wired to bool exprs, constants for PV/PT)
        for f in self.fbs:
            ins, _outs = FBS[f["ty"]]
            wires, consts = [], {}
            for pname, kind in ins:
                if kind == "b":
                    wires.append((pname, self.bexpr(rng, 2)))
                elif kind == "int":
                    consts[pname] = ("liti", rng.randint(2, 5))
                else:
                    consts[pname] = ("tlit", rng.choice([200, 300, 500, 800]))
            self.stmts.append(("call", f, wires, consts))
        # a couple of dependent temp updates and one IF block
        for _ in range(rng.randint(0, 3)):
            self.stmts.append(("ass", rng.choice(self.tb), self.bexpr(rng, 2)))
        if rng.random() < 0.7:
            arms = [(self.bexpr(rng, 1),
                     [("ass", rng.choice(self.tb), self.bexpr(rng, 1))])]
            els = [("ass", rng.choice(self.tb), self.bexpr(rng, 1))]
            self.stmts.append(("if", arms, els))
        # outputs last
        for o in self.outs:
            self.stmts.append(("ass", o, self.bexpr(rng, 2)))

    # renderers ---------------------------------------------------------
    def e_ours(self, e):
        k = e[0]
        if k == "var":
            return e[1]
        if k == "fb":
            return f"{e[1]}.{e[2]}"
        if k == "litb":
            return "TRUE" if e[1] else "FALSE"
        if k == "liti":
            return str(e[1])
        if k == "tlit":
            return f"T#{e[1]}ms"
        if k == "not":
            return f"NOT ({self.e_ours(e[1])})"
        if k in ("and", "or", "xor"):
            return f"({self.e_ours(e[1])} {k.upper()} {self.e_ours(e[2])})"
        if k == "cmp":
            return f"({self.e_ours(e[2])} {e[1]} {self.e_ours(e[3])})"
        return f"({self.e_ours(e[1])} {dict(add='+', sub='-', mul='*')[k]} {self.e_ours(e[2])})"

    def e_mat(self, e):
        k = e[0]
        if k == "fb":
            return f"{e[1]}_{e[2]}"
        if k == "not":
            return f"NOT ({self.e_mat(e[1])})"
        if k in ("and", "or", "xor"):
            return f"({self.e_mat(e[1])} {k.upper()} {self.e_mat(e[2])})"
        if k == "cmp":
            return f"({self.e_mat(e[2])} {e[1]} {self.e_mat(e[3])})"
        if k in ("add", "sub", "mul"):
            return f"({self.e_mat(e[1])} {dict(add='+', sub='-', mul='*')[k]} {self.e_mat(e[2])})"
        return self.e_ours(e)

    def s_ours(self, s, ind):
        k = s[0]
        if k == "ass":
            return [ind + f"{s[1]} := {self.e_ours(s[2])};"]
        if k == "call":
            f, wires, consts = s[1], s[2], s[3]
            args = [f"{p} := {self.e_ours(e)}" for p, e in wires]
            args += [f"{p} := {self.e_ours(e)}" for p, e in consts.items()]
            return [ind + f"{f['inst']}({', '.join(args)});"]
        lines = []
        for i, (c, ss) in enumerate(s[1]):
            kw = "IF" if i == 0 else "ELSIF"
            lines.append(ind + f"{kw} {self.e_ours(c)} THEN")
            lines += sum((self.s_ours(x, ind + "  ") for x in ss), [])
        if s[2]:
            lines.append(ind + "ELSE")
            lines += sum((self.s_ours(x, ind + "  ") for x in s[2]), [])
        lines.append(ind + "END_IF;")
        return lines

    def s_mat(self, s, ind):
        k = s[0]
        if k == "ass":
            return [ind + f"{s[1]} := {self.e_mat(s[2])};"]
        if k == "call":
            f, wires, consts = s[1], s[2], s[3]
            args = [f"{p} := {self.e_mat(e)}" for p, e in wires]
            args += [f"{p} := {self.e_mat(e)}" for p, e in consts.items()]
            # wire only the outputs the program actually reads
            used = {p for p, mp, _k in FBS[f["ty"]][1] if self.uses(f, p)}
            args += [f"{mp} => {f['inst']}_{p}" for p, mp, _k in
                     FBS[f["ty"]][1] if p in used]
            return [ind + f"{f['inst']}({', '.join(args)});"]
        lines = []
        for i, (c, ss) in enumerate(s[1]):
            kw = "IF" if i == 0 else "ELSIF"
            lines.append(ind + f"{kw} {self.e_mat(c)} THEN")
            lines += sum((self.s_mat(x, ind + "  ") for x in ss), [])
        if s[2]:
            lines.append(ind + "ELSE")
            lines += sum((self.s_mat(x, ind + "  ") for x in s[2]), [])
        lines.append(ind + "END_IF;")
        return lines

    def uses(self, fb, pin):
        """is (fb.inst, pin) read anywhere in the program?"""
        def walk(e):
            if e[0] == "fb":
                return e[1] == fb["inst"] and e[2] == pin
            return any(walk(x) for x in e[1:] if isinstance(x, tuple))
        def in_expr(e):
            return any(walk(x) for x in e[1:] if isinstance(x, tuple))
        for s in self.stmts:
            if s[0] == "ass" and walk(s[2]):
                return True
            if s[0] == "call":
                if any(walk(e) for _p, e in s[2]):
                    return True
            if s[0] == "if":
                if any(walk(c) or any(walk(a[2]) for a in ss)
                       for c, ss in s[1]):
                    return True
                if any(walk(a[2]) for a in s[2]):
                    return True
        return False

    def render(self, matiec):
        """Full ST text; matiec=True renders the oracle dialect."""
        e = self.e_mat if matiec else self.e_ours
        s = self.s_mat if matiec else self.s_ours
        decls = [f"  {v} : BOOL;" for v in self.tb] \
            + [f"  {v} : INT;" for v in self.ti] \
            + [f"  {f['inst']} : {f['ty']};" for f in self.fbs]
        if matiec:
            decls += [f"  {f['inst']}_{p} : "
                      + ("INT" if k2 == "i" else "TIME" if k2 == "t" else "BOOL")
                      + ";"
                      for f in self.fbs for p, _m, k2 in FBS[f["ty"]][1]
                      if self.uses(f, p)]
        loc = [f"  {n} AT %IX0.{b} : BOOL;" for b, n in enumerate(self.ins)]
        loc += [f"  {n} AT %QX0.{b} : BOOL;" for b, n in enumerate(self.outs)]
        body = sum((s(x, "") for x in self.stmts), [])
        if matiec:
            return ("PROGRAM FUZZ\nVAR\n" + "\n".join(decls) +
                    "\nEND_VAR\nVAR\n" + "\n".join(loc) +
                    "\nEND_VAR\n" + "\n".join(body) + "\nEND_PROGRAM\n")
        return ("PROGRAM FUZZ\nVAR\n" + "\n".join(decls + loc) +
                "\nEND_VAR\n" + "\n".join(body) + "\nEND_PROGRAM\n")


def gen_cycles(rng, m):
    t = 0
    out = []
    for _ in range(m):
        t += rng.choice([0, 50, 100, 200, 400, 800]) * 1000000
        out.append((rng.randint(0, 255), t))
    return out


def run(cmd, **kw):
    return subprocess.run(cmd, capture_output=True, text=True,
                          timeout=120, **kw)


def matiec_trace(case_dir, mtxt, cycles):
    """Compile the matiec rendering, run cycles, return %Q hex list."""
    src = os.path.join(case_dir, "case_m.st")
    with open(src, "w") as f:
        f.write(mtxt)
    gen = os.path.join(case_dir, "gen")
    os.makedirs(gen, exist_ok=True)
    r = run([IEC2C, src, "-T", gen, "-I", LIB])
    if not os.path.exists(os.path.join(gen, "POUS.c")):
        return None, "iec2c: " + (r.stdout + r.stderr).strip()[:300]
    r = run(["gcc", "-O1", "-DPROGNAME=FUZZ", "-I", LIBC, "-I", gen,
             os.path.join(HERE, "ref_runner.c"), "-o",
             os.path.join(case_dir, "ref")])
    if r.returncode != 0:
        return None, "gcc: " + r.stderr.strip()[:300]
    feed = "".join(f" i={i:02x} t={t}\n" for i, t in cycles)
    r = run([os.path.join(case_dir, "ref")], input=feed)
    if r.returncode != 0:
        return None, "ref run: " + r.stderr.strip()[:300]
    return [l.split("=", 1)[1].strip() for l in r.stdout.splitlines()
            if l.startswith("q=")], None


def load_known():
    out = set()
    if os.path.exists(KNOWN):
        for l in open(KNOWN):
            l = l.split("#")[0].strip()
            if l:
                out.add(l)
    return out


def main():
    seed, nc, mc, do_l2 = 0, 12, 25, False
    upd_known = False
    a = sys.argv[1:]
    i = 0
    while i < len(a):
        if a[i] == "--seed":
            seed = int(a[i + 1]); i += 1
        elif a[i] == "--cases":
            nc = int(a[i + 1]); i += 1
        elif a[i] == "--cycles":
            mc = int(a[i + 1]); i += 1
        elif a[i] == "--l2":
            do_l2 = True
        elif a[i] == "--update-known":
            upd_known = True
        elif a[i] == "--workdir":
            global WORK; WORK = a[i + 1]; i += 1
        i += 1

    if not (os.path.exists(IEC2C) and os.path.exists(TPLCC)):
        print("SKIP matiec_fuzz: iec2c or tplcc missing "
              "(run tools/oracle-matiec/ensure_matiec.sh, cargo build)")
        return 77
    rng = random.Random(seed)
    os.makedirs(WORK, exist_ok=True)
    known = load_known()
    vec_lines, meta = [], []
    errors = 0

    for k in range(nc):
        vid = f"mfuzz.s{seed}.c{k}"
        cdir = os.path.join(WORK, f"s{seed}", f"c{k}")
        os.makedirs(cdir, exist_ok=True)
        g = Gen(rng)
        cycles = gen_cycles(rng, mc)
        trace, err = matiec_trace(cdir, g.render(True), cycles)
        if err:
            print(f"ORACLE-ERR {vid}: {err}")
            errors += 1
            continue
        with open(os.path.join(cdir, "case_ours.st"), "w") as f:
            f.write(g.render(False))
        cyc = [{"i": f"{i_:02x}", "t_ns": t, "expect_q": q}
               for (i_, t), q in zip(cycles, trace)]
        vec_lines.append(json.dumps(
            {"id": vid, "program": f"c{k}/case_ours.st",
             "source": f"matiec-{PIN_SHA[:8]}",
             "cycles": cyc}))
        meta.append((vid, cdir))

    vfile = os.path.join(WORK, f"s{seed}", "vectors.jsonl")
    os.makedirs(os.path.dirname(vfile), exist_ok=True)
    with open(vfile, "w") as f:
        f.write("\n".join(vec_lines) + "\n")

    r = run([TPLCC, "vectors", "run", vfile])
    print(r.stdout.strip())
    new_div, known_div = [], []
    for l in r.stdout.splitlines():
        if not l.startswith("FAIL "):
            continue
        m = re.match(r"FAIL (\S+) cycle (\d+): %Q want (\S+) got (\S+)", l)
        if not m:
            new_div.append(l)
            continue
        vid, cyc, w, g = m.groups()
        if vid in known:
            known_div.append(f"{vid} c{cyc} want {w} got {g} (known)")
        else:
            new_div.append(f"{vid} c{cyc} want {w} got {g}")
            if upd_known:
                with open(KNOWN, "a") as f:
                    f.write(f"{vid}\n")

    for d in known_div:
        print("KNOWN-DIV", d)
    for d in new_div:
        print("DIVERGENCE", d)

    if do_l2 and not new_div:
        vr = os.path.join(ROOT, "tests", "sim", "vector_run.py")
        r = run([sys.executable, vr, "--vectors", vfile])
        print(r.stdout.strip())
        if r.returncode not in (0,):
            print("L2 leg failed (see above)")
            return 1

    if errors:
        print(f"== {errors} oracle errors")
        return 1
    if new_div:
        print(f"== {len(new_div)} NEW divergences — triage into "
              "known_divergences.txt with the semantic reason")
        return 1
    print(f"== diff_fuzz: {len(vec_lines)} cases, "
          f"{len(known_div)} known divergences, 0 new")
    return 0


if __name__ == "__main__":
    sys.exit(main())
