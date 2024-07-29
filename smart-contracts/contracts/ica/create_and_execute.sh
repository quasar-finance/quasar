#!/bin/sh

CHAIN_ID="quasar"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://127.0.0.1:26659"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"
echo $NODE
INIT='{"default_timeout": 600}'
# quasar <-> osmosis channel is connection 2 in current setup
# MSG='{"register_interchain_account":{"connection_id":"connection-0"}}'

cd ../../

docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.12.6

echo "Running store code"
RES=$(quasard tx wasm store artifacts/icq.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value') 
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT=$(quasard tx wasm instantiate $CODE_ID "$INIT" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --no-admin $NODE --chain-id $CHAIN_ID)
ADDR=$(quasard query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR"

# echo "Executing register ica message... ('$MSG')"
# quasard tx wasm execute $ADDR "$MSG" -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID

cd -