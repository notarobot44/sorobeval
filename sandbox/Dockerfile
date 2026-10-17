# SorobEval sandbox — the ONLY toolchain any sample is ever built with.
# Every version below is pinned and printed into results.json by the runner.
# VERIFY the three ARGs against developers.stellar.org on first build, then
# never change them for the lifetime of v0.
ARG RUST_VERSION=1.93
FROM rust:${RUST_VERSION}-bookworm

ARG SOROBAN_WASM_TARGET=wasm32v1-none
ARG STELLAR_CLI_VERSION=28.1.0

ENV SOROBAN_WASM_TARGET=${SOROBAN_WASM_TARGET} \
    STELLAR_CLI_VERSION=${STELLAR_CLI_VERSION} \
    CARGO_TERM_COLOR=never \
    RUSTFLAGS=""

RUN apt-get update && apt-get install -y --no-install-recommends python3 \
 && rm -rf /var/lib/apt/lists/* \
 && rustup target add ${SOROBAN_WASM_TARGET} \
 && cargo install --locked stellar-cli --version ${STELLAR_CLI_VERSION} --no-default-features \
 && rustc --version && cargo --version && stellar --version

# No network for submissions: builds must work from the vendored registry
# cache warmed below. Warm it with the task-1 reference so a fresh sample
# never needs the network.
WORKDIR /warm
COPY tasks/01-counter/reference/ /warm/
RUN cargo fetch && cargo build --release --target ${SOROBAN_WASM_TARGET} \
 && cargo test --release && rm -rf /warm/target

WORKDIR /work
ENTRYPOINT ["/bin/bash", "-c"]
