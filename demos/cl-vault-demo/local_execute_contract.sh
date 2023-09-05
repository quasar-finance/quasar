#!/bin/sh

set -e

on_error() {
    echo "Some error occurred"

    afplay /System/Library/Sounds/Sosumi.aiff
    say -r 60 you suck
}

trap 'on_error' ERR

CHAIN_ID="osmosis"
TESTNET_NAME="osmosis"
FEE_DENOM="uosmo"
RPC="http://127.0.0.1:26679"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"
echo $NODE

# Alice: osmo1sqlsc5024sszglyh7pswk5hfpc5xtl77wcmrz0
# Bob: osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d
INIT='{"thesis":"hello world","name":"Distilled","admin":"osmo1sqlsc5024sszglyh7pswk5hfpc5xtl77wcmrz0","range_admin":"osmo1sqlsc5024sszglyh7pswk5hfpc5xtl77wcmrz0","pool_id":1,"config":{"performance_fee":"0.1","treasury":"osmo1sqlsc5024sszglyh7pswk5hfpc5xtl77wcmrz0","swap_max_slippage":"0.01"},"vault_token_subdenom":"test-cl-vault-1","initial_lower_tick":0,"initial_upper_tick":100000}'

cd ../../smart-contracts

# docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer-arm64:0.12.11

echo "Running store code"
RES=$(osmosisd tx wasm store artifacts/cl_vault-aarch64.wasm --from bob --keyring-backend test -y --output json -b block $TXFLAG)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT1=$(osmosisd tx wasm instantiate $CODE_ID "$INIT" --from bob --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin osmo1sqlsc5024sszglyh7pswk5hfpc5xtl77wcmrz0 $NODE --chain-id $CHAIN_ID)
ADDR1=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR1"

afplay /System/Library/Sounds/Funk.aiff
say "I want to die"

cd -
