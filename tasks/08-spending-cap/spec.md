# Task 08 — Spending-cap treasury (security invariants)

Write a Soroban smart contract in Rust, single file `lib.rs`, `soroban-sdk` 27.

Contract struct: `Treasury`. The treasury holds a token balance (sent to the
contract address by anyone) and lets an admin pay whitelisted recipients,
subject to a hard, immutable lifetime cap. Functions:

- `init(env, admin: Address, token: Address, cap: i128) -> Result<(), Error>`
  — stores admin, token and cap. `Err(AlreadyInitialized)` (1) on a second
  call; `Err(InvalidAmount)` (2) if `cap <= 0`.
- `whitelist(env, recipient: Address) -> Result<(), Error>` — admin only
  (`require_auth`); adds the recipient.
- `spend(env, to: Address, amount: i128) -> Result<(), Error>` — admin only.
  Fails with `Err(InvalidAmount)` (2) if `amount <= 0`;
  `Err(NotWhitelisted)` (3) if `to` is not whitelisted;
  `Err(CapExceeded)` (4) if `spent + amount > cap`. Otherwise transfers
  `amount` of the token from the contract's own address to `to` and adds
  `amount` to the running total.
- `spent(env) -> i128` — running total, 0 before any spend.
- `cap(env) -> i128` — the cap set at init.

Invariants a reviewer will test: cumulative outflow never exceeds `cap`;
no function changes `cap` after `init`; no transfer to a non-whitelisted
address; no function other than `spend` moves tokens; all state-changing
functions require the admin's authorization.

Requirements: `#![no_std]`; `#[contracterror]` enum `Error` with
`AlreadyInitialized = 1, InvalidAmount = 2, NotWhitelisted = 3,
CapExceeded = 4, NotInitialized = 5`; use `soroban_sdk::token::Client` for
the transfer; the contract's own address is `env.current_contract_address()`.

Return only the Rust code for `lib.rs` inside a single ```rust block. Do not
include tests.
