#!/usr/bin/env python3
# license manifest + gate (doc 15 §6): enumerate shipped dependencies,
# record their licenses, fail on GPL-family in shipped paths.
#   rust deps : cargo metadata, non-dev deps reachable from workspace
#   c++ deps  : tools/cpp_deps.txt inventory (name|version|license|note)
#   waivers   : tools/license_waivers.txt (name|license|justification)
# usage: license_manifest.py [--manifest OUT.json] [--check]
import json, re, subprocess, sys, os

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

ALLOWLIST = {
    "apache-2.0", "apache-2.0 with llvm-exception", "mit", "mit-0",
    "bsd-2-clause", "bsd-3-clause", "mpl-2.0", "isc", "zlib",
    "unicode-3.0", "unicode-dfs-2016", "cc0-1.0", "bsl-1.0",
    "openssl", "0bsd", "bsd-1-clause",
}
STRONG = ("agpl", "gpl-", "gplv", "sspl", "eupl")   # never waived


def and_terms(expr):
    """one AND-set (already an OR-alternative) → normalized token list"""
    e = re.sub(r"[()]", " ", expr.lower())
    e = re.sub(r"\bwith\b.*", "", e)                # drop exceptions
    e = re.sub(r"\band\b", " ", e).replace("/", " ")
    return [t for t in e.split() if t != "or"]


def alternatives(expr):
    """SPDX OR-expression → list of AND-term lists"""
    if not expr:
        return [["unlicensed"]]
    alts = re.split(r"\bor\b", expr.lower())
    return [and_terms(a) for a in alts]


def cls(token):
    if any(token.startswith(g) or token == g.strip("-") for g in STRONG):
        return "gpl"
    if token.startswith("lgpl"):
        return "lgpl"
    if token in ALLOWLIST:
        return "ok"
    return "unknown"


def classify(expr, source):
    """entry → gpl|lgpl|warn|ok (best alternative wins under OR)"""
    sev = {"ok-sys": 0, "warn": 1, "lgpl": 2, "gpl": 3}
    best = None
    for terms in alternatives(expr):
        cs = {cls(t) for t in terms}
        if cs <= {"ok"}:
            return "ok"
        if "gpl" in cs:
            v = "gpl"
        elif "lgpl" in cs:
            v = "lgpl" if source != "system" else "ok-sys"
        else:
            v = "warn"
        if best is None or sev[v] < sev[best]:
            best = v
    return best or "warn"


def load_waivers():
    path = os.path.join(ROOT, "tools", "license_waivers.txt")
    w = []
    if os.path.exists(path):
        for line in open(path):
            line = line.strip()
            if line and not line.startswith("#"):
                w.append(line.split("|"))
    return w


def waived(name, lic, waivers):
    for pat, wlic, *_ in waivers:
        if pat == name and (not wlic or wlic in lic.lower()):
            return True
    return False


def rust_deps():
    for flags in (["--offline"], []):
        out = subprocess.run(
            ["cargo", "metadata", "--format-version", "1"] + flags,
            cwd=os.path.join(ROOT, "compiler"),
            capture_output=True, text=True)
        if out.returncode == 0:
            break
    else:
        sys.stderr.write("SKIP: cargo metadata unusable: "
                         + out.stderr[:400])
        sys.exit(77)
    meta = json.loads(out.stdout)
    pkgs = {p["id"]: p for p in meta["packages"]}
    nodes = {n["id"]: n for n in meta["resolve"]["nodes"]}
    roots = set(meta["workspace_members"])
    seen, stack = set(), list(roots)
    while stack:
        pid = stack.pop()
        if pid in seen:
            continue
        seen.add(pid)
        for dep in nodes.get(pid, {}).get("deps", []):
            if all(k["kind"] == "dev" for k in dep["dep_kinds"]):
                continue
            if dep["pkg"] not in seen:
                stack.append(dep["pkg"])
    return sorted(
        ({"name": p["name"], "version": p["version"],
          "license": p.get("license") or "",
          "source": "path" if pid in roots else "crates.io"}
         for pid in seen for p in [pkgs[pid]]),
        key=lambda r: r["name"])


def cpp_deps():
    path = os.path.join(ROOT, "tools", "cpp_deps.txt")
    res = []
    if os.path.exists(path):
        for line in open(path):
            line = line.strip()
            if line and not line.startswith("#"):
                name, ver, lic, *note = line.split("|")
                res.append({"name": name.strip(),
                            "version": ver.strip(),
                            "license": lic.strip(), "source": "system",
                            "note": note[0].strip() if note else ""})
    return res


def main():
    args = sys.argv[1:]
    manifest, check = "LICENSES.json", "--check" in args
    if "--manifest" in args:
        manifest = args[args.index("--manifest") + 1]

    waivers = load_waivers()
    entries = rust_deps() + cpp_deps()
    bad, review, warn = [], [], []
    for e in entries:
        v = classify(e["license"], e["source"])
        e["verdict"] = v
        if v == "gpl":
            bad.append(e)                      # never waivable
        elif v == "lgpl":
            if waived(e["name"], e["license"].lower(), waivers):
                e["verdict"] = "waived"
                e["waiver"] = True
            else:
                review.append(e)
        elif v == "warn" and not waived(
                e["name"], e["license"].lower(), waivers):
            warn.append(e)

    doc = {"generator": "tools/license_manifest.py",
           "policy": "GPL-family fails; LGPL needs an entry in "
                     "license_waivers.txt; others should be within the "
                     "Apache-2.0/MPL-2.0/MIT/BSD/ISC/Zlib allowlist",
           "entries": entries}
    if not check:
        with open(manifest, "w") as f:
            json.dump(doc, f, indent=2)
        print(f"wrote {manifest} ({len(entries)} deps)")

    for e in bad:
        print(f"FAIL GPL-family: {e['name']} {e['version']} "
              f"[{e['license']}]")
    for e in review:
        print(f"REVIEW LGPL (needs waiver): {e['name']} {e['version']} "
              f"[{e['license']}]")
    for e in warn:
        print(f"WARN non-allowlist: {e['name']} {e['version']} "
              f"[{e['license'] or 'none'}]")
    if bad or review:
        sys.exit(1)
    print(f"license gate: {len(entries)} deps, 0 GPL, "
          f"{len(review)} unwaivered LGPL")


if __name__ == "__main__":
    main()
