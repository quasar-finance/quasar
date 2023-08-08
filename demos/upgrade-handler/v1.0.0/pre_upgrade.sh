#!/bin/sh

set -e

on_error() {
    echo "Some error occurred"

    quasarnoded q wasm contract-state smart $ADDR1 '{"trapped_errors":{}}'

    afplay /System/Library/Sounds/Sosumi.aiff
}

trap 'on_error' ERR

# create a pool on osmosis to test against
osmosisd tx gamm create-pool --pool-file ./pools/sample_pool1.json --node http://127.0.0.1:26679 --from bob --keyring-backend test --home $HOME/.osmosis --chain-id osmosis -y --gas-prices 1uosmo -b block
osmosisd tx gamm create-pool --pool-file ./pools/sample_pool2.json --node http://127.0.0.1:26679 --from bob --keyring-backend test --home $HOME/.osmosis --chain-id osmosis -y --gas-prices 1uosmo -b block
osmosisd tx gamm create-pool --pool-file ./pools/sample_pool3.json --node http://127.0.0.1:26679 --from bob --keyring-backend test --home $HOME/.osmosis --chain-id osmosis -y --gas-prices 1uosmo -b block

echo "ibc transferring uosmo"
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 10000000000uosmo --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 1000002stake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 1000003fakestake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo
sleep 6

#     lock_period is  60 for 60 sec, in prod should be at least: 60 sec/min * 60 min/hr * 24hr * 14days "1209600"
#     pool_id is hardcoded to 1 for this testing setup, expected to be done by the instantiater on local/testnet
#     pool_denom should be looked up and hardcoded aswell
#     base_denom: base_denom should be the denom of the token on osmosos, for now uosmo
#     local_denom: the denom of the token used locally, in this testing case: the denom of the path transfer/channel-1/uosmo
#     quote_denom is the denom other denom in the pool, stake for now
INIT1='{"lock_period":6,"pool_id":1,"pool_denom":"gamm/pool/1","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"stake","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
INIT2='{"lock_period":6,"pool_id":3,"pool_denom":"gamm/pool/3","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"fakestake","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
INIT3='{"lock_period":6,"pool_id":2,"pool_denom":"gamm/pool/2","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"usdc","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'

# shellcheck disable=SC2164
cd ../../../smart-contracts

docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer-arm64:0.12.11

BINARY=quasarnoded-go-18
CHAIN_ID="quasar"
ACCOUNT_NAME="my_treasury"
ACCOUNT_ADDRESS="quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
RPC="http://127.0.0.1:26659"

$BINARY query bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --node "http://127.0.0.1:26659"

RES=$($BINARY tx wasm store artifacts/lp_strategy-aarch64.wasm --from $ACCOUNT_NAME --keyring-backend test -y -b block --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 --node $RPC)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got CODE_ID = $CODE_ID"

echo "Deploying primitives"
echo "Primitive 1"
OUT1=$($BINARY tx wasm instantiate $CODE_ID "$INIT1" --from $ACCOUNT_NAME --keyring-backend test --label "primitive-1" -b block -y --admin $ACCOUNT_ADDRESS --chain-id $CHAIN_ID --gas 7000000 --fees 10000uqsr --node $RPC)
ADDR1=$($BINARY query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[0]')
echo "Got address of primitive 1 contract = $ADDR1"

rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

echo "Primitive 2"
OUT2=$($BINARY tx wasm instantiate $CODE_ID "$INIT2" --from $ACCOUNT_NAME --keyring-backend test --label "primitive-1" -b block -y --admin $ACCOUNT_ADDRESS --chain-id $CHAIN_ID --gas 7000000 --fees 10000uqsr --node $RPC)
ADDR2=$($BINARY query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[1]')
echo "Got address of primitive 2 contract = $ADDR2"

rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

echo "Primitive 3"
OUT3=$($BINARY tx wasm instantiate $CODE_ID "$INIT3" --from $ACCOUNT_NAME --keyring-backend test --label "primitive-1" -b block -y --admin $ACCOUNT_ADDRESS --chain-id $CHAIN_ID --gas 7000000 --fees 10000uqsr --node $RPC)
ADDR3=$($BINARY query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[2]')
echo "Got address of primitive 3 contract = $ADDR3"

rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

echo "Store vault rewards code"
RES=$($BINARY tx wasm store artifacts/vault_rewards-aarch64.wasm --from $ACCOUNT_NAME --keyring-backend test -y -b block --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 --node $RPC)
VR_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got vault rewards code ID = $VR_CODE_ID"

echo "Running store code for vault"
RES=$($BINARY tx wasm store artifacts/basic_vault-aarch64.wasm --from $ACCOUNT_NAME --keyring-backend test -y -b block --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 --node $RPC)
VAULT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got basic vault CODE_ID = $VAULT_CODE_ID"

VAULT_INIT='{"total_cap":"200000000000","thesis":"test vault","vault_rewards_code_id":'$VR_CODE_ID',"reward_token":{"native":"uqsr"},"reward_distribution_schedules":[],"decimals":6,"symbol":"OPRO","min_withdrawal":"1","name":"OPRO","deposit_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","primitives":[{"address":"'$ADDR1'","weight":"0.333333333333","init":{"l_p":'$INIT1'}},{"address":"'$ADDR2'","weight":"0.333333333333","init":{"l_p":'$INIT2'}},{"address":"'$ADDR3'","weight":"0.333333333333","init":{"l_p":'$INIT3'}}]}'
OUT=$($BINARY tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from $ACCOUNT_NAME --keyring-backend test --label "vault 1" -b block -y --admin $ACCOUNT_ADDRESS --chain-id $CHAIN_ID --gas 7000000 --fees 10000uqsr --node $RPC)
VAULT_ADDR=$($BINARY query wasm list-contract-by-code $VAULT_CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed vault contract address = $VAULT_ADDR"

echo "setting depositor"
$BINARY tx wasm execute $ADDR1 '{"set_depositor":{"depositor":"'$VAULT_ADDR'"}}' --from $ACCOUNT_NAME --keyring-backend test -y -b block --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000
$BINARY tx wasm execute $ADDR2 '{"set_depositor":{"depositor":"'$VAULT_ADDR'"}}' --from $ACCOUNT_NAME --keyring-backend test -y -b block --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000
$BINARY tx wasm execute $ADDR3 '{"set_depositor":{"depositor":"'$VAULT_ADDR'"}}' --from $ACCOUNT_NAME --keyring-backend test -y -b block --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000

echo "perform a bond action"
$BINARY tx wasm execute $VAULT_ADDR '{"bond":{}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 --amount 10000000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518 --node $RPC

sleep 120

echo "query balance of the bonding account on vault"
$BINARY query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}'

echo "perform an unbond action right befor chain upgrade"
$BINARY tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"9000000"}}' --from $ACCOUNT_NAME --keyring-backend test -y -b block --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 --node $RPC

