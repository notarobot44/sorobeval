# SorobEval v0 — leaderboard

Tasks: 8 · samples per cell: 3 · pass = build + tests; refused / truncated / no-code samples count as failures and are listed separately (wasm size reported, not gated in v0)

| model | condition | pass rate | pass@3 | build ok | tests ok | refused | truncated | no code |
|---|---|---|---|---|---|---|---|---|
| claude-fable-5.1 | skill | 100% | 100% | 100% | 100% | 0 | 0 | 0 |
| gpt-6.1-sol | bare | 100% | 100% | 100% | 100% | 0 | 0 | 0 |
| kimi-k3 | bare | 96% | 100% | 96% | 96% | 0 | 0 | 0 |
| muse-spark-1.3 | skill | 96% | 100% | 96% | 96% | 0 | 1 | 0 |
| kimi-k3 | skill | 92% | 100% | 92% | 92% | 0 | 1 | 0 |
| claude-fable-5.1 | bare | 88% | 88% | 88% | 88% | 3 | 0 | 0 |
| muse-spark-1.3 | bare | 88% | 100% | 88% | 88% | 0 | 3 | 0 |
| glm-5.3-flash | skill | 71% | 88% | 75% | 71% | 0 | 4 | 0 |
| gpt-6.1-sol | skill | 71% | 75% | 71% | 71% | 0 | 0 | 0 |
| glm-5.3-flash | bare | 50% | 75% | 71% | 50% | 0 | 5 | 0 |

Toolchain: {'rustc': 'rustc 1.93.1 (01f6ddf75 2026-02-11)', 'cargo': 'cargo 1.93.1 (083ac5135 2025-12-15)', 'target': 'wasm32v1-none'}
