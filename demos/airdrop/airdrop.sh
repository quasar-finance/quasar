#!/bin/sh

BINARY=quasarnoded
CHAIN_ID="quasar"
ACCOUNT_NAME="alice"
ACCOUNT_ADDRESS="quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"

cd ../../smart-contracts

#docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.11

RES=$($BINARY tx wasm store artifacts/airdrop-aarch64.wasm --from $ACCOUNT_NAME --keyring-backend test -y -b block --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got airdrop CODE_ID = $CODE_ID"

INIT='{"config": {"airdrop_amount": "11000000000","airdrop_asset": {"native": "uqsr"},"airdrop_title":"Test Title","airdrop_description": "Test description","end_height": 0,"start_height": 0,"total_claimed": "0"}}'

echo "Deploying airdrop contract"
OUT1=$($BINARY tx wasm instantiate $CODE_ID "$INIT" --from $ACCOUNT_NAME --keyring-backend test --label "primitive-1" -b block -y --admin $ACCOUNT_ADDRESS --chain-id $CHAIN_ID --gas 7000000 --fees 10000uqsr)
ADDR1=$($BINARY query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[0]')
echo "Got address of primitive 1 contract = $ADDR1"

quasarnoded query wasm contract-state smart $ADDR1 '{"airdrop_config_query":{}}'

echo "Should not fail"
quasarnoded tx wasm execute $ADDR1 '{"admin": {"add_users": {"users": [{"address": "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec", "amount": "2500000000"}, {"address": "quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu", "amount": "2500000000"}, {"address": "quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf", "amount": "2500000000"}, {"address": "quasar185fflsvwrz0cx46w6qada7mdy92m6kx4xruj7p", "amount": "2500000000"}]}}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block

quasarnoded q wasm contract-state smart $ADDR1 '{"contract_state_query":{}}'

echo "Set alice amount to a higher amount so that it overflows and it should fail"
quasarnoded tx wasm execute $ADDR1 '{"admin": {"set_users": {"users": [{"address": "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec", "amount": "4500000000"}]}}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block

echo "Update set new users to update airdrop amount for alice and bob and it should work"
quasarnoded tx wasm execute $ADDR1 '{"admin": {"set_users": {"users": [{"address": "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec", "amount": "1500000000"}, {"address": "quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu", "amount": "3500000000"}]}}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block

quasarnoded q wasm contract-state smart $ADDR1 '{"contract_state_query":{}}'

echo "Remove alice from the airdrop eligibility"
quasarnoded tx wasm execute $ADDR1 '{"admin": {"remove_users":["quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"]}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block

quasarnoded q wasm contract-state smart $ADDR1 '{"contract_state_query":{}}'

echo "Add alcie to the airdrop eligibility"
quasarnoded tx wasm execute $ADDR1 '{"admin": {"add_users": {"users": [{"address": "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec", "amount": "1500000000"}]}}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block

echo "funding contract account"
quasarnoded tx bank send $ACCOUNT_ADDRESS $ADDR1 11000000000uqsr --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block

quasarnoded q bank balances $ADDR1

AIRDROP_START_HEIGHT=15
AIRDROP_END_HEIGHT=25
echo "starting airdrop (but it will start form the given height in future)"
quasarnoded tx wasm execute $ADDR1 '{ "admin" :{"update_airdrop_config": {"airdrop_amount": "11000000000","airdrop_asset": {"native": "uqsr"},"airdrop_title":"Test Title","airdrop_description": "Test description","end_height": '$AIRDROP_END_HEIGHT',"start_height": '$AIRDROP_START_HEIGHT',"total_claimed": "0"}}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block

quasarnoded q wasm contract-state smart $ADDR1 '{"contract_state_query":{}}'

echo ">>> Waiting for the block height to reach $AIRDROP_START_HEIGHT"
while true; do
  CURRENT_HEIGHT=$(quasarnoded status | jq -r '.SyncInfo.latest_block_height')
  echo "Current height: "$CURRENT_HEIGHT
  if [ "$CURRENT_HEIGHT" -ge "$AIRDROP_START_HEIGHT" ]; then
    break
  fi
  sleep 5
done

quasarnoded query wasm contract-state smart $ADDR1 '{"airdrop_config_query":{}}'
quasarnoded tx wasm execute $ADDR1 '{"claim_airdrop":[]}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block
quasarnoded tx wasm execute $ADDR1 '{"claim_airdrop":[]}' --from bob --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block

quasarnoded query wasm contract-state smart $ADDR1 '{"airdrop_config_query":{}}'
quasarnoded tx wasm execute $ADDR1 '{"claim_airdrop":[]}' --from user1 --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block
quasarnoded tx wasm execute $ADDR1 '{"claim_airdrop":[]}' --from user2 --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block

quasarnoded query wasm contract-state smart quasar14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sy9numu '{"contract_state_query":{}}'

echo ">>> Waiting for the block height to reach $AIRDROP_END_HEIGHT"
while true; do
  CURRENT_HEIGHT=$(quasarnoded status | jq -r '.SyncInfo.latest_block_height')
  echo "Current height: "$CURRENT_HEIGHT
  if [ "$CURRENT_HEIGHT" -ge "$AIRDROP_END_HEIGHT" ]; then
    break
  fi
  sleep 5
done

quasarnoded q bank balances $ADDR1
quasarnoded tx wasm execute $ADDR1 '{"admin": {"withdraw_funds":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 -b block
quasarnoded q bank balances $ADDR1