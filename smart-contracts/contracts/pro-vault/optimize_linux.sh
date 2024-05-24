docker run --rm -v "$(pwd)":/code  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry  --env CARGO_BUILD_JOBS=1  --platform linux/amd64 cosmwasm/workspace-optimizer:0.15.0

