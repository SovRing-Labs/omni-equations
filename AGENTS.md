# AGENTS.md — omni-equations

For coding assistants (and people) setting this repository up. Plain steps; every command runs from the repo root.

**Status: pre-alpha, under development.** Interfaces, numbers and file layout can change without notice. Read `STATUS.md` first: what works today (each line names its test and evidence file), what is not yet tested, the known failures and the corrections.

## What it is
Stage: about 70 % (under development, pre-alpha).
Our equation library: 67 concepts as dependency-free Rust, with their definitions, laws and recorded spec rows; free for the community.

## Install and check (3 commands)
```bash
git clone --depth 1 https://github.com/SovRing-Labs/omni-equations && cd omni-equations
# nothing to install beyond: python3 (3.9 or newer), rustc (stable, 2021 edition)
python3 -I tools/verify.py . --only hamming,fib,gcd,mod_inverse,ntt | tail -1   # expect: "pass": true
```
Needs: python3 (3.9 or newer), rustc (stable, 2021 edition).
Whole suite instead: `bash install.sh --no-gui` (python check → venv → every runnable `suite.json` test, headless; without `--no-gui` it then opens the local results page on 127.0.0.1).

GUI: `bash install.sh   # suite page on http://127.0.0.1 (localhost only)`

## What the tests mean
`suite.json` lists every claim this repository makes with its verify command, the expected output, what passing means, how the number was obtained and how to use it in your own work (5 tests, in `suite.json` at the repository root). `STATUS.md` says what works today, what is not yet tested and the known failures; the umbrella's `STATUS.md` has one row per repository.

## Where the evidence is
- `README.md`
- `TECHNICAL-DISCLOSURE.md` — the method, as dated prior art
- `CITATION.cff`

## Honest limits (from STATUS.md)
28 of 67 concepts are not re-verifiable from this repository and have no cemented law yet: `shuffle_mul`, `shuffle_div`, `shuffle_pow`, `shuffle_pow_frac` (log-table arithmetic); `rd_step`, `rd_packet` (the Opus range decoder); `celt_energy_pred`, `silk_lpc_synth`; `ring_exp`, `ring_ln`, `ring_softmax`; and the VP9 concepts `vp9_butterfly`, `idct4/8/16/32`, `iadst4/8/16`, `inv_txfm_2d`, `vp9_intra_dc/tm/v/h`, `vp9_lf4/8/16`, `vp9_subpel8`. The other 39 have rows and laws. Table in `README.md`. One wrong form of `sign_detect` agreed with all 400 probe rows when the equation was built and was …

## Licence
Apache-2.0 (code) + CC BY 4.0 (documents).

## Guided setup through MCP
The SovRing umbrella ships an MCP server that explains, checks readiness, clones and runs these tests: https://github.com/SovRing-Labs/SovRing (README → "Paste this to your assistant"). Zero-install docs endpoint: https://gitmcp.io/SovRing-Labs/omni-equations
