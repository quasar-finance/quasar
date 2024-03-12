#!/bin/sh

# check whether the compiled artifact is compatible
docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.15.0
cosmwasm-check artifacts/intergamm_bindings_test.wasm 