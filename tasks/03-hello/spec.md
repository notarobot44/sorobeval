# Task 03 — Hello with host types

Write a Soroban smart contract in Rust, single file `lib.rs`, `soroban-sdk` 29.

Contract struct: `Hello`. Function:
- `hello(env: Env, to: Symbol) -> Vec<Symbol>` — returns a two-element vector:
  the symbol `Hello` followed by `to`.

Requirements: `#![no_std]`; use Soroban host types (`Vec`, `Symbol` from
`soroban_sdk`), not `std` or `alloc` types. Build the `Vec` with the
environment.

Return only the Rust code for `lib.rs` inside a single ```rust block. Do not
include tests.
