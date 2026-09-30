# Task 04 — Persistent storage with TTL extension

Write a Soroban smart contract in Rust, single file `lib.rs`, `soroban-sdk` 27.

Contract struct: `Persist`. Functions:
- `put(env: Env, key: u32, value: Bytes)` — stores `value` under `key` in
  **persistent** storage and extends the entry's TTL so that it lives at
  least 5,000 ledgers from now (threshold 1,000, extend to 5,000).
- `fetch(env: Env, key: u32) -> Option<Bytes>` — returns the value or `None`.
- `bump(env: Env, key: u32)` — extends the TTL of an existing entry with the
  same parameters; does nothing if the key does not exist.

Requirements: `#![no_std]`; persistent storage (not instance, not
temporary); keys are plain `u32`.

Return only the Rust code for `lib.rs` inside a single ```rust block. Do not
include tests.
