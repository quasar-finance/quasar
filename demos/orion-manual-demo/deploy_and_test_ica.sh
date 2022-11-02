#!/bin/sh

CHAIN_ID="quasar"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
# STAKE_DENOM="urock"
BECH32_HRP="quas"
WASMD_VERSION="v0.23.0"
CONFIG_DIR=".wasmd"
BINARY="wasmd"
COSMJS_VERSION="v0.27.1"
GENESIS_URL="https://raw.githubusercontent.com/CosmWasm/testnets/master/cliffnet-1/config/genesis.json"
RPC="http://127.0.0.1:26659"
# RPC="https://rpc.cliffnet.cosmwasm.com:443"
LCD="https://lcd.cliffnet.cosmwasm.com"
FAUCET="https://faucet.cliffnet.cosmwasm.com"
# https://rpc-edgenet.dev-osmosis.zone/
# https://lcd-edgenet.dev-osmosis.zone/
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"
 
INIT="{}"
MSG='{"register_interchain_account":{"connection_id":"connection-1"}}'

cd ../../smart-contracts

# docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.12.6

echo "Running store code"
RES=$(quasarnoded tx wasm store artifacts/intergamm_bindings_test.wasm --from alice -y --output json -b block $TXFLAG) 
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value') 
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT" --from alice --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --no-admin $NODE --chain-id $CHAIN_ID)
ADDR=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR"

echo "Executing register ica message... ('$MSG')"
quasarnoded tx wasm execute $ADDR "$MSG" --from alice --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID

cd -