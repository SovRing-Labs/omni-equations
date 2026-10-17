#!/usr/bin/env python3
"""lawcheck.py - re-run the concept laws (laws/mathkb.json) on fresh random draws, standalone (python 3 only).
Each law is a PROPERTY of a concept (never its definition restated): `gen` draws inputs, `forms.python` is the property, `prelude` holds helper code.
Usage: python3 tools/lawcheck.py [REPO_ROOT] [--n 300] [--seed S]
The teeth of the laws (a deliberately broken subject must fail) are recorded in results/*/laws-teeth.json, made by the factory.
Prints one line per law and a JSON summary last; exit 0 = all pass."""
import json, math, os, random, sys
a = sys.argv[1:]; n = int(a[a.index("--n") + 1]) if "--n" in a else 300; seed = a[a.index("--seed") + 1] if "--seed" in a else "2026-10-16"
pos = [x for i, x in enumerate(a) if not x.startswith("--") and (i == 0 or a[i - 1] not in ("--n", "--seed"))]
root = os.path.abspath(pos[0] if pos else ".")
laws = json.load(open(f"{root}/laws/mathkb.json"))["axioms"]
tot_fail = 0; rows = []
for e in laws:
    R = random.Random(f"{e['id']}|{seed}")
    ns = {"math": math, "random": random}
    exec(e.get("prelude", "") + "\n" + e["gen"], ns)
    body = e.get("prelude", "") + f"\nimport math\ndef check({', '.join(e['vars'])}):\n    return bool({e['forms']['python']})\n"
    cs = {}; exec(body, cs)
    ok = 0
    for _ in range(n):
        args = list(ns["gen"](R, "base"))
        try: r = bool(cs["check"](*args))
        except Exception: r = False
        ok += r
    good = ok == n
    tot_fail += (not good)
    print(f"{e['id']}: {ok}/{n} {'ok' if good else 'FAIL'}")
res = {"laws": len(laws), "draws_each": n, "failed_laws": tot_fail, "mode": "laws", "pass": tot_fail == 0 and bool(laws)}
print(json.dumps(res)); sys.exit(0 if res["pass"] else 1)
