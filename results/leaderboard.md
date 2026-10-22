# SorobEval v0 — leaderboard

Tasks: 8 · samples per cell: 3 · pass = build + tests; refused / truncated / no-code samples count as failures and are listed separately (wasm size reported, not gated in v0)

| model | condition | pass rate | pass@3 | build ok | tests ok | refused | truncated | no code |
|---|---|---|---|---|---|---|---|---|
| claude-fable-5.1 | skill | 100% | 100% | 100% | 100% | 0 | 0 | 0 |
| kimi-k3 | skill | 100% | 100% | 100% | 100% | 0 | 0 | 0 |
| gpt-6.1-sol | skill | 100% | 100% | 100% | 100% | 0 | 0 | 0 |
| gpt-6.1-sol | bare | 100% | 100% | 100% | 100% | 0 | 0 | 0 |
| muse-spark-1.3 | skill | 100% | 100% | 100% | 100% | 0 | 0 | 0 |
| muse-spark-1.3 | bare | 100% | 100% | 100% | 100% | 0 | 0 | 0 |
| kimi-k3 | bare | 96% | 100% | 96% | 96% | 0 | 0 | 0 |
| claude-fable-5.1 | bare | 88% | 88% | 88% | 88% | 3 | 0 | 0 |
| glm-5.3-flash | skill | 71% | 88% | 79% | 71% | 0 | 0 | 0 |
| glm-5.3-flash | bare | 42% | 62% | 67% | 42% | 0 | 0 | 0 |

Toolchain: {'rustc': 'rustc 1.93.1 (01f6ddf75 2026-02-11)', 'cargo': 'cargo 1.93.1 (083ac5135 2025-12-15)', 'target': 'wasm32v1-none'}
