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
| kimi-k3 | bare | 92% | 100% | 96% | 92% | 0 | 0 | 0 |
| claude-fable-5.1 | bare | 88% | 88% | 88% | 88% | 3 | 0 | 0 |
| glm-5.3-flash | skill | 75% | 88% | 88% | 75% | 0 | 0 | 0 |
| qwen3-coder-30b-local | skill | 58% | 75% | 75% | 58% | 0 | 0 | 0 |
| devstral-small-2-local | skill | 38% | 38% | 50% | 38% | 0 | 0 | 0 |
| glm-5.3-flash | bare | 33% | 50% | 62% | 33% | 0 | 0 | 0 |
| gpt-oss-20b-local | skill | 33% | 75% | 38% | 33% | 0 | 0 | 0 |
| qwen3-coder-30b-local | bare | 0% | 0% | 0% | 0% | 0 | 0 | 0 |
| gpt-oss-20b-local | bare | 0% | 0% | 0% | 0% | 0 | 0 | 0 |
| devstral-small-2-local | bare | 0% | 0% | 0% | 0% | 0 | 0 | 0 |

Toolchain: {'rustc': 'rustc 1.93.1 (01f6ddf75 2026-02-11)', 'cargo': 'cargo 1.93.1 (083ac5135 2025-12-15)', 'stellar': 'stellar 28.1.0 (c0f4d0da891bbf214c08b8c5035ae6db80e9a3bd)\nstellar-xdr 28.0.0 (d0f1330e43c3a2c0c616da30698b24140d4eea72)\nxdr (9c9c145953e80990d6ff1ae3a6a973a0ce6d0694)', 'target': 'wasm32v1-none'}
