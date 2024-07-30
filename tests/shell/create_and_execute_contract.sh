#!/bin/bash

set -e

on_error() {
    echo "Some error occurred"

    quasard q wasm contract-state smart $ADDR1 '{"trapped_errors":{}}'

    # afplay /System/Library/Sounds/Sosumi.aiff
}

trap 'on_error' ERR

BINARY="docker exec localenv-quasar-1 quasard"

CHAIN_ID="quasar"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://127.0.0.1:26657"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"
echo $NODE
#     lock_period is  60 for 60 sec, in prod should be at least: 60 sec/min * 60 min/hr * 24hr * 14days "1209600"
#     pool_id is hardcoded to 1 for this testing setup, expected to be done by the instantiater on local/testnet
#     pool_denom should be looked up and hardcoded aswell
#     base_denom: base_denom should be the denom of the token on osmosos, for now uosmo
#     local_denom: the denom of the token used locally, in this testing case: the denom of the path transfer/channel-1/uosmo
#     quote_denom is the denom other denom in the pool, stake for now
INIT1='{"lock_period":6,"pool_id":1,"pool_denom":"gamm/pool/1","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"stake","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'

cd ../../smart-contracts

# ARM64 (Mac M1)
#docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer-arm64:0.12.11

# AMD64 (Linux)
docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.11

docker cp ./artifacts/ localenv-quasar-1:/quasar/artifacts

echo "Running store code"
RES=$($BINARY tx wasm store artifacts/lp_strategy.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract (lp-strategy)"
# swallow output
OUT1=$($BINARY tx wasm instantiate $CODE_ID "$INIT1" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec $NODE --chain-id $CHAIN_ID)
ADDR1=$($BINARY query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR1"

docker exec localenv-relayer-1 rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icqhost --order unordered --version icq-1 --override
docker exec localenv-relayer-1 rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override


RES=$($BINARY tx wasm store artifacts/vault_rewards.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)
VR_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')

VAULT_INIT='{"total_cap":"200000000000","thesis":"yurmom","vault_rewards_code_id":'$VR_CODE_ID',"reward_token":{"native":"uqsr"},"reward_distribution_schedules":[],"decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR1'","weight":"0.5","init":{"l_p":'$INIT1'}}]}'
echo $VAULT_INIT

echo "Running store code (vault)"
RES=$($BINARY tx wasm store artifacts/basic_vault.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)

VAULT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')

echo "Got CODE_ID = $VAULT_CODE_ID"

echo "Deploying contract (vault)"
# # swallow output
OUT=$($BINARY tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec $NODE --chain-id $CHAIN_ID)
VAULT_ADDR=$($BINARY query wasm list-contract-by-code $VAULT_CODE_ID --output json $NODE | jq -r '.contracts[0]')

echo "Got address of deployed contract = $VAULT_ADDR (vault)"

echo "setting depositor"
sleep 6
$BINARY tx wasm execute $ADDR1 '{"set_depositor":{"depositor":"'$VAULT_ADDR'"}}' --from alice --keyring-backend test $TXFLAG -y

echo "VAULT: $VAULT_ADDR"


cd -
