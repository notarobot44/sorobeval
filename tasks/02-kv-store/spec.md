# Task 02 — Key-value store (instance storage)

Write a Soroban smart contract in Rust, single file `lib.rs`, `soroban-sdk` 29.

Contract struct: `Store`. Functions:
- `set(env: Env, key: Symbol, value: i128)` — stores the value under the key.
- `get(env: Env, key: Symbol) -> Option<i128>` — returns the stored value, or
  `None` if the key was never set.
- `remove(env: Env, key: Symbol)` — deletes the key; `get` returns `None`
  afterwards.

Requirements: `#![no_std]`; use **instance** storage; keys are the `Symbol`
values passed in, not wrapped in any enum.

Return only the Rust code for `lib.rs` inside a single ```rust block. Do not
include tests.
