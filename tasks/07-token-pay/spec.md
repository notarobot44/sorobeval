# Task 07 — Cross-contract token transfer

Write a Soroban smart contract in Rust, single file `lib.rs`, `soroban-sdk` 27.

Contract struct: `Payer`. Function:
- `pay(env: Env, from: Address, to: Address, token: Address, amount: i128)
  -> Result<(), Error>` — requires `from`'s authorization, then transfers
  `amount` of `token` from `from` to `to` by calling the token contract.
  Returns `Err(Error::InvalidAmount)` (code 1) if `amount <= 0`, without
  calling the token.

Requirements: `#![no_std]`; use `soroban_sdk::token::Client` (or
`TokenClient`) for the transfer; define `Error` with `#[contracterror]`,
`#[repr(u32)]`, variant `InvalidAmount = 1`.

Return only the Rust code for `lib.rs` inside a single ```rust block. Do not
include tests.
