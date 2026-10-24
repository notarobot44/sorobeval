# Task 05 — Admin-gated setter

Write a Soroban smart contract in Rust, single file `lib.rs`, `soroban-sdk` 29.

Contract struct: `Gated`. Functions:
- `init(env: Env, admin: Address) -> Result<(), Error>` — stores the admin.
  Returns `Err(Error::AlreadyInitialized)` (code 1) if called twice.
- `set_value(env: Env, value: u32) -> Result<(), Error>` — requires the
  stored admin's authorization (`require_auth`), then stores the value.
  Returns `Err(Error::NotInitialized)` (code 2) if `init` was never called.
- `value(env: Env) -> u32` — returns the stored value, 0 if never set.

Requirements: `#![no_std]`; define `Error` with `#[contracterror]`,
`#[repr(u32)]`, variants `AlreadyInitialized = 1`, `NotInitialized = 2`;
store admin and value in instance storage.

Return only the Rust code for `lib.rs` inside a single ```rust block. Do not
include tests.
