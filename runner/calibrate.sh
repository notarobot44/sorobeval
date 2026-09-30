#!/usr/bin/env bash
set -e
for t in tasks/0*; do
  bytes=$(docker run --rm --network none -v "$PWD":/work sorobeval-sandbox \
    "python3 runner/sorobeval.py reference $t" | python3 -c 'import json,sys; r=json.load(sys.stdin); print(r["wasm_bytes"] if r["pass"] else 0)')
  if [ "$bytes" = "0" ]; then echo "$t: reference FAILED, budget untouched"; continue; fi
  max=$(( bytes * 3 / 2 ))
  sed -i "s/^max_wasm_bytes = .*/max_wasm_bytes = $max/" "$t/budget.toml"
  echo "$t: reference $bytes bytes -> max_wasm_bytes = $max"
done
