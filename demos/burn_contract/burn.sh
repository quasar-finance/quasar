#!/bin/sh

BINARY=quasard
CHAIN_ID="quasar"
ACCOUNT_NAME="alice"
ACCOUNT_ADDRESS="quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"

cd ../../smart-contracts

#docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.11

RES=$($BINARY tx wasm store artifacts/burn_coins-aarch64.wasm --from $ACCOUNT_NAME --keyring-backend test -y -b block --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got airdrop CODE_ID = $CODE_ID"

INIT='{}'

echo "Deploying airdrop contract"
OUT1=$($BINARY tx wasm instantiate $CODE_ID "{}" --from $ACCOUNT_NAME --keyring-backend test --label "burn coins contract" -b block -y --admin $ACCOUNT_ADDRESS --chain-id $CHAIN_ID --gas 7000000 --fees 10000uqsr)
ADDR1=$($BINARY query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[0]')
echo "Got address of burn coin contract = $ADDR1"

quasard q bank total

echo "Should not fail"
quasard tx wasm execute $ADDR1 '{"burn":{}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block --amount 90000000000000000uqsr,200000000stake,20000token

quasard q wasm contract-state smart $ADDR1 '{"total_burnt_query":{}}'

quasard q bank total


