# omni-equations — our equation library: 67 concepts as dependency-free Rust

[![status: pre-alpha](https://img.shields.io/badge/status-pre--alpha%20(under%20development)-orange)](STATUS.md)
[![defensive publication](https://img.shields.io/badge/defensive%20publication-TECHNICAL--DISCLOSURE.md-blue)](TECHNICAL-DISCLOSURE.md)

Status: **under development — pre-alpha** (`STATUS.md`). Defensive publication: `TECHNICAL-DISCLOSURE.md` (dated 2026-10-16; the DOI is
minted by Zenodo on the first GitHub release).

An equation library: each concept (a greatest common divisor, a Montgomery product, a number-theoretic transform, a video inverse transform, an
audio range decoder) is one small Rust file with no dependencies, the definition it was built from (an s-expression), the property law that
concept must satisfy, and recorded input/output rows. You can use an equation without the factory that made it: copy the one `fn f` line, or
run the file on rows. Free for the community, like our research corpus (Apache-2.0 code, CC BY 4.0 documents). The factory that produced the
equations, **math-core**, stays source-available under PolyForm; its outputs are free.

## Run it (Linux or macOS; Python 3 and `rustc`)
```bash
python3 -I tools/verify.py .                 # compile 39 concepts and run each on its recorded rows (about 11 s)
python3 -I tools/verify.py . --compile       # all 67 files compile, index and files agree
python3 -I tools/lawcheck.py .               # the 39 concept laws on fresh random draws
```
Or run the whole suite with its explanations: `bash install.sh` (opens a local page; `--no-gui` for the terminal only).

## Use one equation
Every `emit/<concept>.rs` has the same shape: `fn f(a0: ..., a1: ...) -> ...` (the equation; integers are `i64` with wrapping arithmetic, floats `f32`,
lists `&[i64]` / `Vec<i64>`), a `close` helper for float comparison, and a `main` that reads rows from a file (one row per line: the arguments, then the
expected output; a list is written `n v1 ... vn`) and asserts `f(args) == expected`. To check your own port, feed it the rows in `specs/specs.jsonl`.
`emit/index.json` lists each concept's argument types and its equation as an s-expression (the language is described in `TECHNICAL-DISCLOSURE.md`).

## What it does today (one laptop, Intel i5-8365U, rustc 1.98.0; files in `results/`)
| Test | Result |
|---|---|
| Q1 emitted equations vs recorded spec rows | 39 concepts, 3,245 rows, 39 passed, 0 failed (`results/2026-10-16-verify.log`) |
| Q2 negative control | tampered rows: all 39 programs fail as they must (`results/2026-10-16-verify-teeth.log`) |
| Q3 laws | 39 laws x 300 fresh draws, 0 failed (`results/2026-10-16-lawcheck.log`) |
| Q4 structure | 67 files compile, index and files agree, equations balanced (`results/2026-10-16-compile.log`) |
| Q5 honest negative | 28 concepts have no rows here and are listed as not re-verifiable, not counted (`results/2026-10-16-not-reverifiable.log`) |

Not shown here: the factory (math-core), the compiled binaries (239 MB, never published), the laws ledger, and the rows of the 28 codec / range-decoder /
ring-arithmetic / shuffle concepts. See `STATUS.md` and `DATA.md`.

## Index of concepts
Rows verified = rows this repository re-checks (Q1). The other 28 were checked at build against a black-box reference and are recorded in
`results/2026-10-16-factory-receipts.json`; they have no cemented law yet.

| Concept | What it computes | Law (property) | Rows verified from this repository |
|---|---|---|---|
| `absmean_quant` | BitNet b1.58 weight quantisation: s = max(mean absolute weight, 1e-5); q = clamp(round(w/s), -1, 1) | `absmean.ternary_sign` | 84 rows (run it: tools/verify.py) |
| `clock_read` | residue clock read: t in [0, 105) from its residues on rings 3, 5, 7 | `clock.roundtrip` | 84 rows (run it: tools/verify.py) |
| `crt2` | Chinese remainder theorem, two coprime moduli: the x in [0, m1·m2) with x ≡ r1 (m1), x ≡ r2 (m2) | `crt.bijection` | 55 rows (run it: tools/verify.py) |
| `mod_barrett` | a mod m (the definition; the Barrett form is the authored equation, checked against it) | `barrett.equals_mod` | 84 rows (run it: tools/verify.py) |
| `hamming` | Hamming distance of two non-negative integers = bits that differ | `hamming.triangle` | 84 rows (run it: tools/verify.py) |
| `mixed_radix` | mixed-radix numeral (d0, d1, d2) in bases (m0, m1, m2) → its value | `mixed_radix.roundtrip` | 82 rows (run it: tools/verify.py) |
| `isqrt` | integer square root: the largest r with r·r ≤ n | `isqrt.newton_brackets` | 84 rows (run it: tools/verify.py) |
| `softmax` | numerically stable softmax: exp(x − max) / Σ exp(x − max) | `softmax.normalised_shift_invariant` | 84 rows (run it: tools/verify.py) |
| `period` | smallest period p ≥ 1 of a sequence: v[i] = v[i+p] for every valid i (len if none shorter) | `period.reproduces` | 84 rows (run it: tools/verify.py) |
| `fib` | Fibonacci F(n), F(0)=0, F(1)=1 (the golden-ratio recurrence) | `fib.cassini` | 84 rows (run it: tools/verify.py) |
| `mont_mul` | Montgomery product (Montgomery 1985), R = 2^16: a·b·R⁻¹ mod m, m odd, 0 ≤ a, b < m | `montgomery.domain_homomorphism` | 84 rows (run it: tools/verify.py) |
| `mod_inverse` | modular inverse: the x in [0, m) with a·x ≡ 1 (mod m), defined when gcd(a, m) = 1 | `mod_inverse.involution` | 84 rows (run it: tools/verify.py) |
| `modexp` | modular exponentiation: b^e mod m | `modexp.exponent_and_base_homomorphism` | 84 rows (run it: tools/verify.py) |
| `base_extension` | residue number system base extension: x from its residues on coprime m1, m2 (x < m1·m2), read mod p | `base_extension.order_free_and_exact` | 84 rows (run it: tools/verify.py) |
| `rns_mul` | residues of a·b (or a + b) mod M = ∏ m_i, from the residues of a and b on coprime m_i | `rns.ring_homomorphism` | 84 rows (run it: tools/verify.py) |
| `rns_extend` | RNS base extension: x from its residues on coprime ms (x < ∏ ms), read on the target moduli ps | `rns.extension_is_the_same_number` | 84 rows (run it: tools/verify.py) |
| `rns_mont_mul` | RNS Montgomery product (Posch & Posch 1995): q = −x·y·N⁻¹ mod MA, r = (x·y + q·N) / MA, given on base B | `rns_montgomery.domain_homomorphism` | 84 rows (run it: tools/verify.py) |
| `ct_reduce` | Barrett reduction with mask-select corrections (no secret-dependent branch): a mod m | `constant_time.trace_invariant` | 84 rows (run it: tools/verify.py) |
| `rns_sq_extend` | residues of x**2 (mod prod ms) read on the target moduli ps — a composition's reference | `rns.square_then_extend_is_the_square` | 84 rows (run it: tools/verify.py) |
| `rns_mont_mul_a` | the RNS Montgomery product read back on base A (mont gives it on B; base extension carries it) | `rns_montgomery.base_a_agrees_with_base_b` | 84 rows (run it: tools/verify.py) |
| `matmul_mod` | square matrix product mod m: C[i][j] = sum_k A[i][k]·B[k][j] mod m, row-major n×n, entries in [0, m) | `matmul_mod.associative_distributive` | 84 rows (run it: tools/verify.py) |
| `det_mod` | determinant mod m of an n×n matrix, n in 1..3 (Leibniz/cofactor expansion), entries in [0, m) | `det_mod.multiplicative` | 84 rows (run it: tools/verify.py) |
| `mr_digits` | mixed-radix digits (Garner) of the integer named by its residues, lowest radix first | `mr_digits.horner_roundtrip` | 84 rows (run it: tools/verify.py) |
| `rns_compare` | order (-1, 0, +1) of two integers given as residue vectors, without leaving residue form | `rns_compare.total_order` | 84 rows (run it: tools/verify.py) |
| `sign_detect` | sign of a residue-vector integer in the centred range: 0 for zero, +1 up to (M-1)/2, -1 above | `sign_detect.odd_under_negation` | 84 rows (run it: tools/verify.py) |
| `exact_scale` | floor(X / product of the first q moduli), read on the remaining moduli, from X's residues | `exact_scale.undoes_multiplication` | 84 rows (run it: tools/verify.py) |
| `newton_reciprocal` | floor(2^31 / d) for d >= 1 by Newton iteration in integers | `newton_reciprocal.within_one_ulp` | 84 rows (run it: tools/verify.py) |
| `rational_reconstruction` | Wang 1981: (n, d) with n = a·d (mod m), /n/ and d bounded by sqrt(m/2) | `rational_reconstruction.recovers_the_fraction` | 84 rows (run it: tools/verify.py) |
| `range_plan` | fewest leading rings (moduli sorted large to small) whose whole-bit count reaches bits+1 | `range_plan.rings_suffice` | 84 rows (run it: tools/verify.py) |
| `gcd` | greatest common divisor (Euclid, Elements VII.2): the largest d dividing both, a,b >= 0 not both 0 | `gcd.divides_and_homogeneous` | 84 rows (run it: tools/verify.py) |
| `mm_combine` | multi-modular combine (Garner/CRT per entry): the entries x in [0, m1·m2) with x ≡ r1 (m1), x ≡ r2 (m2), coprime moduli | `mm_combine.crt_per_entry` | 84 rows (run it: tools/verify.py) |
| `ext_gcd` | extended Euclid (Bezout's identity): [g, x, y] with a·x + b·y = g = gcd(a, b); the standard back-substitution representative | `ext_gcd.bezout` | 84 rows (run it: tools/verify.py) |
| `solve_mod` | solve A x = b over Z_p for n = 2 or 3 by Cramer's rule (unique solution iff det A != 0 mod p), p prime | `solve_mod.satisfies_the_system` | 84 rows (run it: tools/verify.py) |
| `ntt` | number-theoretic transform over Z_p (Cooley-Tukey definition): X_k = sum_j x_j w^(jk) | `ntt.inverse_and_convolution` | 84 rows (run it: tools/verify.py) |
| `intt` | inverse number-theoretic transform over Z_p, back from the frequency side | `intt.round_trip_from_the_frequency_side` | 84 rows (run it: tools/verify.py) |
| `polymul_negacyclic` | product in Z_p[x]/(x^n + 1), the ring of lattice cryptography (FIPS 203 sec. 2.4): c_k = sum a_i b_j - sum a_i b_j (wrap-around) | `polymul_negacyclic.shift_and_commute` | 84 rows (run it: tools/verify.py) |
| `miller_rabin` | strong probable-prime test to base a (Miller 1976 / Rabin 1980): n odd > 4, 2 <= a <= n-2; 1 = passes, 0 = a is a witness (n composite) | `miller_rabin.primes_always_pass` | 84 rows (run it: tools/verify.py) |
| `crt_general` | two congruences x ≡ a (mod m), x ≡ b (mod n), moduli NOT necessarily coprime (Euclid/CRT, "when two cycles meet"): the least x in [0, lcm(m,n)), or -1 when gcd(m,n) does not divide a-b | `crt_general.iff_gcd_divides` | 84 rows (run it: tools/verify.py) |
| `pollard_rho` | Pollard's rho (Pollard 1975) with Brent's batched gcd (Brent 1980): f(x) = x^2 + 1 mod n from x = y = 2; 16 steps of Floyd's walk (x -> f(x), y -> f(f(y))), q = product of /x - y/ mod n, then ONE gcd(q, n) (1 = none found, n = the walk collapsed, oth | `pollard_rho.returns_a_divisor` | 84 rows (run it: tools/verify.py) |
| `shuffle_mul` | x*y on Z/p by per-ring log / antilog table reads | none yet | none here (not re-verifiable); 252 rows checked at build, per the receipt |
| `shuffle_div` | x/y on Z/p by per-ring log / antilog table reads | none yet | none here (not re-verifiable); 202 rows checked at build, per the receipt |
| `shuffle_pow` | x^e on Z/p by per-ring log / antilog table reads | none yet | none here (not re-verifiable); 332 rows checked at build, per the receipt |
| `rd_step` | one range-decoder symbol (RFC 6716 sec. 4.1.2): decision, state update, renormalisation | none yet | none here (not re-verifiable); 570 rows checked at build, per the receipt |
| `rd_packet` | range decoder over a packet: init (RFC 6716 sec. 4.1.1) then one symbol per script entry; the frame's final range | none yet | none here (not re-verifiable); 321 rows checked at build, per the receipt |
| `vp9_butterfly` | the VP9 inverse-transform butterfly B(a, b, angle) with Round2 at 14 bits (VP9 bitstream spec v0.6 sec. 8.7.1.1) | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |
| `idct4` | VP9 inverse DCT, length 4 (spec v0.6 sec. 8.7.1.2-3) | none yet | none here (not re-verifiable); 1194 rows checked at build, per the receipt |
| `idct8` | VP9 inverse DCT, length 8 (sec. 8.7.1.2-3) | none yet | none here (not re-verifiable); 306 rows checked at build, per the receipt |
| `idct16` | VP9 inverse DCT, length 16 (sec. 8.7.1.2-3) | none yet | none here (not re-verifiable); 84 rows checked at build, per the receipt |
| `idct32` | VP9 inverse DCT, length 32 (sec. 8.7.1.2-3) | none yet | none here (not re-verifiable); 84 rows checked at build, per the receipt |
| `iadst4` | VP9 inverse ADST, length 4 (sec. 8.7.1.9) | none yet | none here (not re-verifiable); 1194 rows checked at build, per the receipt |
| `iadst8` | VP9 inverse ADST, length 8 (sec. 8.7.1.9) | none yet | none here (not re-verifiable); 306 rows checked at build, per the receipt |
| `iadst16` | VP9 inverse ADST, length 16 (sec. 8.7.1.9) | none yet | none here (not re-verifiable); 84 rows checked at build, per the receipt |
| `inv_txfm_2d` | VP9 2D inverse transform (sec. 8.7.2, lossless = 0): rows, then columns, then the final rounding shift; 4 transform types | none yet | none here (not re-verifiable); 172 rows checked at build, per the receipt |
| `vp9_intra_dc` | VP9 intra DC prediction of a block from the above row and left column (sec. 8.5.1) | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |
| `vp9_intra_tm` | VP9 intra TrueMotion prediction (sec. 8.5.1) | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |
| `vp9_intra_v` | VP9 intra vertical prediction (sec. 8.5.1) | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |
| `vp9_intra_h` | VP9 intra horizontal prediction (sec. 8.5.1) | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |
| `vp9_lf4` | VP9 loop filter across one edge, 4-tap (sec. 8.8.5) | none yet | none here (not re-verifiable); 306 rows checked at build, per the receipt |
| `vp9_lf8` | VP9 loop filter across one edge, 8-tap (sec. 8.8.5) | none yet | none here (not re-verifiable); 306 rows checked at build, per the receipt |
| `vp9_lf16` | VP9 loop filter across one edge, 16-tap (sec. 8.8.5) | none yet | none here (not re-verifiable); 84 rows checked at build, per the receipt |
| `vp9_subpel8` | VP9 8-tap sub-pixel block inter prediction, unscaled (sec. 8.5.2.4) | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |
| `shuffle_pow_frac` | x^(a/b) on Z/p: the unique b-th root of x^a, as x^(a * b^-1 mod (p-1)) | none yet | none here (not re-verifiable); 278 rows checked at build, per the receipt |
| `ring_exp` | e^(-w) for fixed-point w = W/S >= 0 in integers: [mantissa m, exponent z] with e^(-w) ~ m/S * 2^(-z) | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |
| `ring_ln` | ln(x) for fixed-point x in [1, 2^20] in integers: (k+1) ln 2 + 2 atanh((t-1)/(t+1)) after range reduction | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |
| `celt_energy_pred` | Opus CELT coarse-energy prediction filter (RFC 6716 sec. 4.3.2.1) | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |
| `silk_lpc_synth` | Opus SILK LPC synthesis in Q36 integers (RFC 6716 sec. 4.2.7.9.2) | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |
| `ring_softmax` | softmax of fixed-point inputs in integers: output [P_0..P_n-1, E_0..E_n-1], p_i = P_i/S * 2^(-E_i) | none yet | none here (not re-verifiable); 2084 rows checked at build, per the receipt |

## Files
| Path | What |
|---|---|
| `emit/*.rs` | the 67 emitted concept files |
| `emit/index.json` | per concept: argument types and the equation (the definition, as an s-expression) |
| `specs/specs.jsonl` | the recorded spec rows (examples, holdout, validate) of the 39 verified concepts |
| `laws/mathkb.json` | the 39 property laws (statement, variables, Python form, input generator) |
| `CONCEPTS.md` | each concept's signature and equation |
| `tools/verify.py`, `tools/lawcheck.py` | the standalone checks (Python 3 + rustc) |
| `results/` | logs of the 2026-10-16 runs, the factory receipts, the recorded law teeth |

## Licence
Code (`emit/*.rs`, `tools/`, `install.sh`, the code in `laws/mathkb.json`): **Apache-2.0** (`LICENSE`). Documents (`README.md`, `STATUS.md`, `CONCEPTS.md`,
`TECHNICAL-DISCLOSURE.md`, `PROVENANCE.md`, `DATA.md`, the statements in `laws/` and `specs/`, `results/`): **CC BY 4.0** (`LICENSE-CC-BY-4.0.md`). Free for the
community, like our research corpus. The factory that produced them is a separate source-available repository. Details: `LICENSING.md`.

Stage: about 70 %. Part of [SovRing-Labs](https://github.com/SovRing-Labs/SovRing). Cite: `CITATION.cff`.
