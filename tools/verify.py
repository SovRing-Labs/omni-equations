#!/usr/bin/env python3
"""verify.py - re-verify the emitted Rust equations on their spec rows, from a clean clone (python 3 + rustc only; no factory needed).
For each concept in emit/index.json that has recorded spec rows (specs/specs.jsonl: examples + holdout + validate), compile emit/<concept>.rs with rustc
into a temp directory and run it on those rows; the program asserts f(args) == recorded output on every row and exits non-zero if any differs.
Usage: python3 tools/verify.py [REPO_ROOT] [--only a,b,c] [--teeth] [--opt N] [--compile]
--teeth: negative control - the rows' recorded outputs are changed first; every program must then FAIL (exit 0 only if all do).
--compile: the structural check - emit/index.json and emit/*.rs name exactly the same concepts, every recorded equation has balanced parentheses, and EVERY
emitted file (also the ones with no recorded rows) compiles with rustc.
Concepts without recorded rows (the codec / range-decoder / ring-NL ones) are listed as NOT-RE-VERIFIABLE-HERE, never as passed."""
import json, os, subprocess, sys, tempfile
a = sys.argv[1:]
only = a[a.index("--only") + 1].split(",") if "--only" in a else None
opt = a[a.index("--opt") + 1] if "--opt" in a else "1"
pos = [x for i, x in enumerate(a) if not x.startswith("--") and (i == 0 or a[i - 1] not in ("--only", "--opt"))]
root = os.path.abspath(pos[0] if pos else "."); TEETH = "--teeth" in a
idx = json.load(open(f"{root}/emit/index.json"))
specs = [json.loads(l) for l in open(f"{root}/specs/specs.jsonl") if l.strip()]
def enc(v): return f"{len(v)} " + " ".join(repr(float(x)) if isinstance(x, float) else str(x) for x in v) if isinstance(v, list) else (repr(float(v)) if isinstance(v, float) else str(v))
bump = lambda w: ([w[0] + 1] + list(w[1:]) if w else [1]) if isinstance(w, list) else w + 1
tmp = tempfile.mkdtemp(prefix="eqverify-"); ok = bad = skipped = 0; rows_total = 0
if "--compile" in a:
    files = {f[:-3] for f in os.listdir(f"{root}/emit") if f.endswith(".rs")}
    problems = []
    if files != set(idx): problems.append(f"index/files differ: {sorted(files ^ set(idx))}")
    for c, info in idx.items():
        e = info["equation"]
        if e.count("(") != e.count(")"): problems.append(f"{c}: unbalanced parentheses in the recorded equation")
        r = subprocess.run(["rustc", "-C", "opt-level=0", "--edition", "2021", "-o", f"{tmp}/{c}", f"{root}/emit/{c}.rs"], capture_output=True, text=True)
        if r.returncode != 0: problems.append(f"{c}: COMPILE-FAIL {r.stderr[:120]!r}")
    print("\n".join(problems) or "all emitted files compile; index and files agree; equations balanced")
    print(json.dumps({"mode": "compile", "concepts": len(idx), "files": len(files), "problems": len(problems), "pass": not problems})); sys.exit(1 if problems else 0)
for c, info in idx.items():
    if only and c not in only: continue
    rows = [r for s in specs if s.get("equation") == info["equation"] for r in s.get("examples", []) + s.get("holdout", []) + s.get("validate", [])]
    if not rows:
        skipped += 1; print(f"{c}: NOT-RE-VERIFIABLE-HERE (no recorded spec rows in this repository)"); continue
    if TEETH: rows = [(x, bump(w)) for x, w in rows]
    rf = f"{tmp}/{c}.rows"; open(rf, "w").write("".join(" ".join(enc(x) for x in args) + " " + enc(w) + "\n" for args, w in rows))
    b = f"{tmp}/{c}"
    r = subprocess.run(["rustc", "-C", f"opt-level={opt}", "--edition", "2021", "-o", b, f"{root}/emit/{c}.rs"], capture_output=True, text=True)
    if r.returncode != 0:
        bad += 1; print(f"{c}: COMPILE-FAIL {r.stderr[:200]!r}"); continue
    run = subprocess.run([b, rf], capture_output=True, text=True, timeout=300)
    passed = (run.returncode != 0) if TEETH else (run.returncode == 0)
    ok += passed; bad += (not passed); rows_total += len(rows)
    print(f"{c}: {len(rows)} rows, {'PASS' if passed else 'FAIL'}{' (teeth: tampered rows were rejected)' if TEETH and passed else ''}")
print(json.dumps({"mode": "teeth" if TEETH else "verify", "passed": ok, "failed": bad, "not_reverifiable": skipped, "rows": rows_total, "pass": bad == 0 and ok > 0}))
sys.exit(0 if bad == 0 and ok > 0 else 1)
