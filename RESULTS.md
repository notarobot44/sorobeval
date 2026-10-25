# SorobEval v0 — results

## SDK 29 rerun (24–25 Oct 2026)

Same eight tasks, same scorer, toolchain moved from soroban-sdk 27 to 29.0.1 (stellar-cli 28.1.0, Rust 1.93; builds now via `stellar contract build`, which SDK 28+ requires), skill condition = stellar-dev-skill `main` at e20f352 (21 Oct, PR #143 merged). No cell fell by more than one sample. GPT-6.1-sol/skill rose from 71% to 100%: the task-7 failures reported in v0 were fixed upstream and the fix verified across an SDK major. Claude's three bare failures remain the provider's content filter on task 7. The skill's effect is now confined to the smallest model (GLM 33% → 75%); the frontier rows pass v0's tasks with or without it. Cost of the rerun: one afternoon, one toolchain change, $8.40 of API credit, one laptop overnight. Local rows (LM Studio, 32k context): bare 0 of 72 on both SDKs; with the skill qwen3-coder-30b 58% (v0 62%), devstral-small-2 38% (38%), gpt-oss-20b 33% (42%) — within one or two samples of v0, same picture. gpt-oss-20b's bare failures are invented SDK calls (`set_ttl`, `get_or_default`, `Symbol::native`…), i.e. a model with no Soroban in its training.

| cell | v0 (sdk 27) | sdk 29 |
|---|---|---|
| claude-fable-5.1 skill / bare | 100 / 88 | 100 / 88 |
| gpt-6.1-sol skill / bare | 71 / 100 | 100 / 100 |
| kimi-k3 skill / bare | 92 / 96 | 100 / 92 |
| muse-spark-1.3 skill / bare | 96 / 88 | 100 / 100 |
| glm-5.3-flash skill / bare | 71 / – | 75 / 33 |
| qwen3-coder-30b-local skill / bare | 62 / 0 | 58 / 0 |
| gpt-oss-20b-local skill / bare | 42 / 0 | 33 / 0 |
| devstral-small-2-local skill / bare | 38 / 0 | 38 / 0 |
| truncated samples, all rows | 9 | 0 |


Run: 29–30 September 2026 · 8 tasks · 5 models · 2 conditions · 3 samples per cell · 240 samples.
Toolchain (pinned, in `sandbox/Dockerfile`): rustc 1.93.1, soroban-sdk 27.0.6, stellar-cli 28.1.0, target `wasm32v1-none`.
Model ids as listed by OpenRouter on 29 Sep 2026; default settings, temperature 0.2, `max_tokens` 16000.
Skill condition: SDF `stellar-dev-skill`, `skills/smart-contracts/` (SKILL.md + development, security, testing), commit `73b609d5`, SHA-256 recorded per sample.

A sample passes when it **builds to wasm** and **passes the task's tests** (including negative tests). Wasm size is reported but not gated in v0 (see Notes).

## Leaderboard

| model | condition | pass rate | pass@3 |
|---|---|---|---|
| Claude Fable 5.1 | skill | 100% | 100% |
| GPT-6.1 Sol | bare | 100% | 100% |
| Kimi K3 | bare | 96% | 100% |
| Muse Spark 1.3 | skill | 96% | 100% |
| Kimi K3 | skill | 92% | 100% |
| Claude Fable 5.1 | bare | 88% | 88% |
| Muse Spark 1.3 | bare | 88% | 100% |
| GLM-5.3-Flash | skill | 71% | 88% |
| GPT-6.1 Sol | skill | 71% | 75% |
| GLM-5.3-Flash | bare | 50% | 75% |

*bare* = task spec only. *skill* = task spec + SDF stellar-dev-skill in the system prompt.
Pass rate = share of the 24 samples passing; pass@3 = share of the 8 tasks with at least one passing sample.

## Per task (passing samples of 3)

| model / condition | T1 | T2 | T3 | T4 | T5 | T6 | T7 | T8 |
|---|---|---|---|---|---|---|---|---|
| Claude Fable 5.1 / bare | 3 | 3 | 3 | 3 | 3 | 3 | 0 | 3 |
| Claude Fable 5.1 / skill | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 |
| GPT-6.1 Sol / bare | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 |
| GPT-6.1 Sol / skill | 2 | 3 | 3 | 3 | 3 | 3 | 0 | 0 |
| Kimi K3 / bare | 3 | 3 | 3 | 3 | 3 | 2 | 3 | 3 |
| Kimi K3 / skill | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 1 |
| Muse Spark 1.3 / bare | 3 | 3 | 3 | 3 | 3 | 2 | 3 | 1 |
| Muse Spark 1.3 / skill | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 2 |
| GLM-5.3-Flash / bare | 2 | 3 | 3 | 2 | 0 | 1 | 0 | 1 |
| GLM-5.3-Flash / skill | 3 | 3 | 2 | 0 | 3 | 3 | 1 | 2 |

T1 counter · T2 key-value store · T3 hello (host types) · T4 persistent storage + TTL · T5 admin-gated setter · T6 events (`#[contractevent]`) · T7 cross-contract token transfer · T8 spending-cap treasury (security invariants).

## Why samples failed

Every failure falls into exactly one of four causes. None is attributable to the harness (see Notes for what was fixed before this table was final).

| model / condition | refused by policy | no code produced | does not build | fails tests |
|---|---|---|---|---|
| Claude Fable 5.1 / bare | 3 | 0 | 0 | 0 |
| Claude Fable 5.1 / skill | 0 | 0 | 0 | 0 |
| GPT-6.1 Sol / bare | 0 | 0 | 0 | 0 |
| GPT-6.1 Sol / skill | 0 | 0 | 7 | 0 |
| Kimi K3 / bare | 0 | 0 | 1 | 0 |
| Kimi K3 / skill | 0 | 0 | 2 | 0 |
| Muse Spark 1.3 / bare | 0 | 0 | 3 | 0 |
| Muse Spark 1.3 / skill | 0 | 0 | 1 | 0 |
| GLM-5.3-Flash / bare | 0 | 4 | 3 | 5 |
| GLM-5.3-Flash / skill | 0 | 4 | 2 | 1 |

## Three findings

**1. SDF's skill is double-edged — and the mechanism is a sentence.** It raises GLM-5.3-Flash by 21 points, Claude by 12 and Muse by 8, and lowers GPT-6.1 Sol by 29, from a perfect 24/24 to 0/6 on the two token-transfer tasks (T7, T8). All seven GPT failures are the same compile error (`E0283`) on the token client's `transfer`. SDK 27 made the destination a `MuxedAddress`; the skill says an `Address` "converts into it" but never shows the call. GPT followed the prose literally and wrote `&to.into()`, which type inference cannot resolve; the other models wrote `&to`, which compiles. Without the skill GPT never learns about `MuxedAddress` and passes 3/3. A one-line documentation fix — show the call, say not to add `.into()` — is proposed upstream. For an SDK maintainer this is the actionable finding of the run: model behaviour tracks documentation *phrasing*, not only its content.

**2. Claude's only failures are policy refusals, not code.** In the bare condition, Anthropic's API returned `finish_reason: content_filter` three times out of three on T7 — a `pay` function that calls a token contract's `transfer` after `require_auth`. With the SDF skill in context the identical task passes 3/3. Every line of Soroban Claude actually wrote in this run compiled and passed its tests. The refusal is reported as a failure because the model did not deliver, and separately as its own category because the cause is different.

**3. Reasoning without a result.** GLM-5.3-Flash exhausted a 16,000-token output budget entirely in reasoning, with no code emitted, in 8 of 48 samples — all on the harder tasks. Raising the budget from 4,000 to 16,000 did not change this. It is the model's default behaviour and is reported as such; a constrained-reasoning setting is a v0.1 experiment.

Also visible: T5 and T7/T8 are where models separate; T1–T3 are solved by everyone and exist to catch gross SDK drift. The open-weight Kimi K3 is within one sample of the proprietary leaders in both conditions.

## Notes on method and its limits

- **n = 3 per cell.** Differences under ~10 points are not meaningful. This is a v0 to establish the pipeline and the task set, not a ranking to be quoted to the decimal.
- **One toolchain.** All samples are built with the pinned versions above. The first scoring pass used `-D warnings`; it was dropped on 17 Oct 2026 because it failed samples on warnings unrelated to SDK correctness, and every row was re-scored without it on 21 Oct — no score changed. Removed APIs (e.g. the old `env.events().publish` signature) still fail the build; the benchmark measures current-SDK Soroban.
- **Local rows.** `qwen3-coder-30b-local` (Q3_K_L), `devstral-small-2-local` (Q4_K_M) and `gpt-oss-20b-local` (native MXFP4, no requantisation; reasoning effort low) ran in LM Studio on a laptop (Intel Arc, Vulkan runtime), 32k context, `max_tokens` 6000, with the runner's request timeout raised to 3600 s. The runner sends no temperature, so each local model ran at its LM Studio preset; Devstral's is 0.15 and its three samples per cell came out byte-identical, so its pass@3 equals its pass rate; gpt-oss-20b's is 0.8, so its three samples differ and its pass@3 can exceed its pass rate.
- **Wasm-size budget not gated in v0.** The 1.5× reference budgets were calibrated on hand-minimised references; on T5 every correct sample landed 5–10% over. Sizes are recorded in every result and will be gated once budgets are recalibrated on a larger sample.
- **Harness fixes made before the final table:** output budget raised to 16,000; 19 samples that had returned empty due to transient network errors or output truncation were regenerated; refusals are detected from the API response rather than counted as "no code".
- **Author disclosure.** The benchmark was built with the assistance of Claude, and Claude Fable 5.1 is one of the models tested. All gates are mechanical (compile, test), the tasks, references, prompts and raw samples are published, and anyone can rerun the matrix; that, rather than trust, is the answer to the conflict.

## Reproduce

See `README.md`: build the sandbox, run `reference` on each task, `generate`, `score-all`, `leaderboard`, `breakdown`. Total API cost of this run: under $10.

Pseudonymous release · Copyright 2026 Sorobai LLC · Apache 2.0.
