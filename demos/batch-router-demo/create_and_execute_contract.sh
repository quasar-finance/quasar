#!/bin/sh

set -e

on_error() {
    echo "Some error occurred"

    afplay /System/Library/Sounds/Sosumi.aiff
    # say -r 10 you suck
}

trap 'on_error' ERR

CHAIN_ID="osmosis"
TESTNET_NAME="osmosis"
FEE_DENOM="uosmo"
RPC="http://127.0.0.1:26679"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"

cd ../../smart-contracts/osmosis-contracts

docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer-arm64:0.12.11

echo "store swaprouter"
RES=$(osmosisd tx wasm store artifacts/swaprouter-aarch64.wasm --from bob --keyring-backend test -y --output json -b block $TXFLAG)
SWAPROUTER_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got swaprouter CODE_ID = $SWAPROUTER_CODE_ID"

echo "deploy swaprouter"
# swallow output
OUT=$(osmosisd tx wasm instantiate $SWAPROUTER_CODE_ID '{"owner": "osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d"}' --from bob --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d $NODE --chain-id $CHAIN_ID)
SWAPROUTER_ADDR=$(osmosisd query wasm list-contract-by-code $SWAPROUTER_CODE_ID --output json $NODE | jq -r '.contracts[0]')

echo "store batch-crosschain-swaps"
RES=$(osmosisd tx wasm store artifacts/batch_crosschain_swaps-aarch64.wasm --from bob --keyring-backend test -y --output json -b block $TXFLAG)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got bccs CODE_ID = $CODE_ID"

BATCH_CCS_INIT='{"governor":"osmo1sqlsc5024sszglyh7pswk5hfpc5xtl77wcmrz0","swap_contract":"'$SWAPROUTER_ADDR'"}'

echo "deploy batch-crosschain-swaps"
# swallow output
OUT=$(osmosisd tx wasm instantiate $CODE_ID "$BATCH_CCS_INIT" --from bob --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin osmo1sqlsc5024sszglyh7pswk5hfpc5xtl77wcmrz0 $NODE --chain-id $CHAIN_ID)
BATCH_CCS_ADDR=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got bccs address = $BATCH_CCS_ADDR"

# create routes through the pools we setup
echo "adding routes to swaprouter"
# swallow output
# OUT=$(osmosisd tx wasm execute $SWAPROUTER_ADDR '{"set_route":{"input_denom":"uosmo","output_denom":"stake","pool_route":[{"pool_id":"1","token_out_denom":"stake"}]}}' --from bob --keyring-backend test -y --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block $NODE --chain-id $CHAIN_ID)
# OUT=$(osmosisd tx wasm execute $SWAPROUTER_ADDR '{"set_route":{"input_denom":"uosmo","output_denom":"usdc","pool_route":[{"pool_id":"2","token_out_denom":"usdc"}]}}' --from bob --keyring-backend test -y --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block $NODE --chain-id $CHAIN_ID)

# trigger the batch crosschain swaps contract to swap through the route
echo "triggering batch crosschain swaps contract"
osmosisd tx wasm execute $BATCH_CCS_ADDR '{"batch_osmosis_swap":{"output_denoms":["stake","usdc"],"output_weights":["1.0","3.0"],"receiver":"osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d","slippage":{"twap":{"window_seconds":90,"slippage_percentage":"5"}},"on_failed_delivery":"do_nothing"}}' --from bob --keyring-backend test -y --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block $NODE --chain-id $CHAIN_ID --amount 100uosmo

cd -

afplay /System/Library/Sounds/Funk.aiff
# say "I want to die"
