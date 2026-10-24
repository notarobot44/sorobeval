# SorobEval

> **Note (24 Oct 2026):** the v0 results in this repository were produced on soroban-sdk 27 (stellar-cli 28.1.0, Rust 1.93) and are dated. A full rerun on soroban-sdk 29 is in progress on branch `sdk-29`.

A reproducible benchmark for AI-generated Soroban smart contracts.
Eight tasks, mechanical gates (build, tests), two context conditions (with and without SDF's official stellar-dev-skill), any OpenAI-compatible model.
Independent project by Sorobai LLC; not affiliated with the Stellar Development Foundation.

**Results:** see [RESULTS.md](RESULTS.md) for the v0 run (29–30 Sep 2026): leaderboard, per-task grid, failure causes and three findings.

## What is measured

Each task is a one-file Soroban contract specified in `tasks/NN-name/spec.md`. A model's answer is scored inside a pinned Docker sandbox, offline:

1. **build** — compiles to wasm (`wasm32v1-none`) with `-D warnings`, so deprecated SDK calls fail
2. **tests** — passes the task's tests, which the model never sees, including negative tests on T5–T8
3. **size** — wasm bytes, recorded (gated from v0.1 once budgets are recalibrated)

Three samples per task, model and condition; `pass rate` is the share of passing samples, `pass@3` the share of tasks with at least one pass.

## Reproduce the v0 run

```bash
docker build -f sandbox/Dockerfile -t sorobeval-sandbox .          # pinned toolchain, ~30 min
for t in tasks/0*; do docker run --rm --network none -v "$PWD":/work sorobeval-sandbox \
  "python3 runner/sorobeval.py reference $t"; done                    # every reference must pass
export OPENROUTER_API_KEY=...                                          # host side, needs network
python3 runner/sorobeval.py generate                                   # 240 samples, resumable
docker run --rm --network none -v "$PWD":/work sorobeval-sandbox \
  "python3 runner/sorobeval.py score-all; chown -R $(id -u):$(id -g) /work/results /work/samples"
python3 runner/sorobeval.py leaderboard                                # results/leaderboard.md
python3 runner/breakdown.py                                            # per-task grid, failure causes
```

Model ids live in `runner/models.toml` (as listed by OpenRouter on the run date; versioned ids, no aliases). The skill condition needs `context/stellar-dev-skill.md`: the `skills/smart-contracts/` files of https://github.com/stellar/stellar-dev-skill concatenated; the commit used is in `context/stellar-dev-skill.commit` and each sample records the file's SHA-256.

Pinned for v0: rustc 1.93, soroban-sdk 27, stellar-cli 28.1.0. If the first build fails, it is one of these three lines in `sandbox/Dockerfile` or the SDK version in `tasks/*/reference/Cargo.toml`.

## Layout

- `sandbox/` — the only toolchain samples are built with; pinned, offline
- `tasks/NN-name/` — `spec.md` (the prompt), `reference/` (a passing crate), `tests/test.rs` (copied into any crate under test), `budget.toml`
- `runner/` — stdlib-only Python: `reference` / `score` (one crate), `generate`, `score-all`, `leaderboard`; `breakdown.py`; `calibrate.sh`
- `context/` — system prompt and the SDF skill for the `skill` condition
- `samples/`, `results/` — raw model outputs and scores for the published run

## Disclosure

This benchmark was built with the assistance of Claude, and Claude Fable 5.1 is one of the models tested. Everything that decides a score is mechanical and published: tasks, references, tests, prompts, raw samples, toolchain. Rerun it.

## Contributing

Open an issue for a task you believe is wrong (a reference that shouldn't pass, a test that's too strict) or a model that should be added. Pull requests are reviewed as text; nothing from a PR is executed before review.

Pseudonymous release · Copyright 2026 Sorobai LLC · Apache 2.0
