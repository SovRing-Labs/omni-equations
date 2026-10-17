# DATA — what was kept out of this repository, and why
Available on request (open an issue titled "Data request").

| Path in the development tree | Bytes | Why not shipped | How to regenerate |
|---|---|---|---|
| `emit/bin/` (55 compiled binaries of the emitted files) | 249,625,008 | compiled artefacts, never published | `rustc -O --edition 2021 emit/<concept>.rs -o <concept>` (what `tools/verify.py` does into a temporary folder) |
| rows and black-box references of the 28 codec / range-decoder / ring-arithmetic / shuffle concepts | — | they belong to the factory (math-core) and its per-run probe files (17.2 MB) | the factory's reader; recorded counts are in `results/2026-10-16-factory-receipts.json` |
| the laws ledger (append-only record of cementing runs, seeds 11 and 12 x 1,000 draws) | — | runs on our fleet; the laws themselves ship as `laws/mathkb.json` | — |
