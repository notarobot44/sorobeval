# Task 06 — Events on state change

Write a Soroban smart contract in Rust, single file `lib.rs`, `soroban-sdk` 27.

Contract struct: `Recorder`. Functions:
- `record(env: Env, value: u32) -> u32` — stores `value` in instance storage
  under a short symbol key, publishes a contract event, and returns the
  number of records so far (1 for the first call).
- `last(env: Env) -> Option<u32>` — the most recently recorded value.

Event contract: define an event type `Record` with `#[contractevent]` whose
topics are the event name (`record`) and `count: u32` (marked `#[topic]`),
and whose data is `value: u32`. Publish it with `.publish(&env)`. One event
per `record` call, none from `last`. Do not use the deprecated
`env.events().publish`.

Requirements: `#![no_std]`.

Return only the Rust code for `lib.rs` inside a single ```rust block. Do not
include tests.
