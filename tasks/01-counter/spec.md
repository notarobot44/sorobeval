# Task 01 — Counter

Write a Soroban smart contract in Rust, in a single file `lib.rs`, using
`soroban-sdk` 29.

The contract exposes two functions:

- `increment(env: Env) -> u32` — increases a persistent counter by one and
  returns the new value. The counter starts at 0.
- `get(env: Env) -> u32` — returns the current value without changing it.

Requirements:
- `#![no_std]`; use `#[contract]` and `#[contractimpl]`.
- Store the counter in **instance** storage under a short symbol key.
- Extend the instance TTL on every `increment`.
- The contract struct must be named `Counter`.

Return only the Rust code for `lib.rs` inside a single ```rust block. Do not
include tests; they are supplied separately.
