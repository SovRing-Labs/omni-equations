# TECHNICAL DISCLOSURE — omni-equations: a library of verified equations as dependency-free Rust

Author: Juan Carlos De Santiago, with the Sovereign Raptor fleet (AI-assisted). Date: 2026-10-16. Version 0.1.0 (pre-alpha).
This document describes the library's form so that a skilled reader could rebuild it. It is published as dated prior art: the author claims no patent on what it
describes and asks that none be granted on it. The DOI is minted by Zenodo on the first GitHub release.

## 1. Problem
Exact arithmetic building blocks (modular and residue-number-system arithmetic, transforms, codec steps) are copied between projects by hand, and each copy
silently loses its proof. We want a library in which every block carries, next to the code, its definition, a property that must hold, and rows that anyone can
re-run without the tool that produced the block.

## 2. The equation language
A concept's definition is an s-expression over 64-bit integers and 32-bit floats. Arguments are `a0 ... an` of type `int` (i64), `list` (list of i64) or `f32list`.
Operators: arithmetic and bit operators (`+ - * / % & | ^ >> <<`), comparisons, `and`, `or`, `sel` (conditional: `(sel c x y)`), `abs`, `max`, `min`, `round`, `exp`;
sequences `(range a b)`, `(map seq body)`, `(fold seq init body)`, `idx`, `len`, `sum`, `maxall`, `push`; tuples `(tup ...)` with `(get0 t)` .. `(get4 t)`; loops with state
`(while0.<d> init cond step)`. Inside a loop of depth d the item is `item<d>`, the accumulator `acc<d>.k`, the state `st<d>.k`. There is no `let`; a loop runs at most 10,000
ticks and a `range` has at most 256 items in the interpreter. Example (greatest common divisor, Euclid):
`(get0 (while0.1 (tup a0 a1) (> st1.1 0) (tup st1.1 (% st1.0 st1.1))))`.

## 3. The emitted program
Each equation is emitted as one Rust file with no dependencies and a fixed shape: `fn f(a0: i64, a1: &[i64], ...) -> ...` (integer arithmetic uses `wrapping_*`,
shifts mask the count with 63, a fold becomes a `for` loop over an `i64` range with a mutable accumulator, a `while0` becomes a `loop` over a state tuple, `map` becomes an
iterator `map`), a `close` function for floats (equal when both NaN or within 1e-5 + 1e-4 of the larger magnitude), and a `main` that reads a rows file and asserts
`f(args) == expected` on each row, printing `ok <rows>`. Row format: arguments then expected output separated by spaces; a list is `n v1 ... vn`; floats are printed with a
decimal point. Example (Fibonacci): `fn f(a0: i64) -> i64 { { let mut st = ((0i64), (1i64), a0); loop { let (s1_0, s1_1, s1_2) = st; if ((s1_2 > (0i64)) as i64) == 0 { break; } st = (s1_1, s1_0.wrapping_add(s1_1), s1_2.wrapping_sub((1i64))); } st }.0 }`.

## 4. The verification contract shipped with each concept
1. **Definition**: the equation in `emit/index.json`, with argument types.
2. **Rows**: `specs/specs.jsonl` records, per equation, `examples`, `holdout` and `validate` lists of `[[args], output]` (most concepts 84 rows). The expected outputs
   were produced by the concept's published reference definition when the equation was built.
3. **Law**: `laws/mathkb.json` holds a property per concept (for example: for coprime m1, m2 the map x -> (x mod m1, x mod m2) on [0, m1 m2) hits every pair exactly once)
   as a Python expression with a seeded input generator. A law is a property, never the definition restated.
4. **Re-verification** (`tools/verify.py`): compile each `emit/<concept>.rs` into a temporary folder with `rustc`, write that concept's rows in the row format, run the program;
   a non-zero exit means a row differs. A negative control (`--teeth`) changes every expected output first and requires every program to fail. Concepts without recorded
   rows are listed as not re-verifiable, never counted as passed.
5. **Laws re-check** (`tools/lawcheck.py`): for each law run the generator on a seeded random source and evaluate the property on each draw.

## 5. How the equations were made (summary; the factory is a separate repository)
Each equation was authored from a published definition and accepted only if it agreed, as a black box, with a reference program on 100 to 400 probe rows drawn from the concept's
real domain, while deliberately wrong forms were refused (`results/2026-10-16-factory-receipts.json`); it was then emitted and re-run on its recorded rows (1,369 to 2,084 rows per
concept in the receipts) and paired with a law that fails when the code is broken on purpose (`results/2026-10-16-laws-teeth-*.json`, 15 laws recorded). The method is described
in the math-core disclosure.

## 6. Measured (files; machine: Intel i5-8365U laptop, rustc 1.98.0, 2026-10-16)
- 39 emitted programs on 3,245 recorded rows: 39 passed, 0 failed; tampered rows: 39 of 39 fail (`results/2026-10-16-verify.log`, `-verify-teeth.log`).
- 39 laws x 300 seeded draws: 0 failed (`results/2026-10-16-lawcheck.log`).
- 67 files compile; index and files name the same concepts (`results/2026-10-16-compile.log`).

## 7. What is not claimed
No claim that the 28 concepts without rows here are re-checked by this repository; no speed claim; no claim about inputs outside the recorded rows (wrapping arithmetic can
overflow, and some concepts panic outside their domain); no big-integer support; `pollard_rho` is a 16-step Brent-batched form that finds factors of only some semiprimes (22 % of those tried).

## 8. Field sources (ideas, no code taken)
Euclid (gcd); Montgomery 1985; Posch and Posch 1995; Garner 1959; Wang 1981; Miller 1976 and Rabin 1980; Pollard 1975; Brent 1980; Harvey 2012 (number-theoretic transforms);
NIST FIPS 203 (negacyclic ring); Ma et al. 2024 (BitNet b1.58 quantisation); the VP9 bitstream and decoding process specification v0.6; IETF RFC 6716 (Opus).
