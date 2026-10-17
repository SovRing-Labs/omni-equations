> **Status: under development — pre-alpha.** Interfaces, numbers and file layout can change without notice.

# STATUS — omni-equations 0.1.0 (2026-10-16)
Stage: about 70 % (the author, 2026-10-16: the math core and its equation library are at about 70 %; everything else about 30 %).

## Works today (each line = a suite test that passed on 2026-10-16, with its evidence)
- **Q1** 39 emitted equations compile from this repository and reproduce all 3,245 recorded spec rows: 39 passed, 0 failed — `results/2026-10-16-verify.log`
- **Q2** negative control: with every recorded output changed, all 39 programs fail as they must — `results/2026-10-16-verify-teeth.log`
- **Q3** 39 concept laws hold on 300 fresh seeded draws each, 0 failed — `results/2026-10-16-lawcheck.log`
- **Q4** all 67 emitted files compile (rustc -C opt-level=0), `emit/index.json` and `emit/*.rs` name the same 67 concepts, equations balanced — `results/2026-10-16-compile.log`
- **Q5** the verifier lists the 28 concepts that have no rows here as NOT-RE-VERIFIABLE-HERE (four checked: idct4, vp9_lf4, ring_ln, rd_step) instead of counting them — `results/2026-10-16-not-reverifiable.log`

## Not yet tested
- Speed of any emitted program (nothing here measures it).
- The 28 concepts without rows here: their rows and black-box references are in the development tree. Receipts record that each was verified at build on 570 to 2,084 rows
  (`results/2026-10-16-factory-receipts.json`), not re-run here.
- Behaviour outside the recorded rows: the rows are the ones recorded when each equation was built, not an exhaustive test. Emitted programs use wrapping 64-bit
  arithmetic and may panic on inputs outside the concept's domain (for example `% 0`); the domain of each concept is in its definition (`CONCEPTS.md`, comments in the
  development reference, not shipped here).

## Known failures and honest negatives
- 28 of 67 concepts are not re-verifiable from this repository and have no cemented law yet: `shuffle_mul`, `shuffle_div`, `shuffle_pow`, `shuffle_pow_frac` (log-table
  arithmetic); `rd_step`, `rd_packet` (the Opus range decoder); `celt_energy_pred`, `silk_lpc_synth`; `ring_exp`, `ring_ln`, `ring_softmax`; and the VP9 concepts
  `vp9_butterfly`, `idct4/8/16/32`, `iadst4/8/16`, `inv_txfm_2d`, `vp9_intra_dc/tm/v/h`, `vp9_lf4/8/16`, `vp9_subpel8`. The other 39 have rows and laws. Table in `README.md`.
- One wrong form of `sign_detect` agreed with all 400 probe rows when the equation was built and was not refused (`results/2026-10-16-factory-receipts.json`); the
  law for `sign_detect` is in `laws/mathkb.json` and its recorded teeth caught the broken subject (187 of 1000 fail, `results/2026-10-16-laws-teeth-r3.json`).
- Pollard rho (`pollard_rho`) is Brent's batched form at 16 steps and finds a factor on only 22 % of the semiprimes tried; `ntt` / `intt` / `polymul_negacyclic` are
  specified for lengths up to 8; `mm_combine` and `solve_mod` for small fixed sizes.
- No big integers: 255-bit field arithmetic cannot be expressed in the equation language yet.

## Corrections
None recorded against this repository (first release).

## How to re-check
`python3 sovring-suite.pyz --run omni-equations` or `bash install.sh` (runs Q1–Q5 with their explanations), or the three commands in `README.md`.

## Defensive publication
`TECHNICAL-DISCLOSURE.md` (the library form: equation language, emitted-program shape, rows and laws as the verification contract), dated 2026-10-16.
DOI: minted by Zenodo on the first GitHub release; `CITATION.cff` carries it after that.
