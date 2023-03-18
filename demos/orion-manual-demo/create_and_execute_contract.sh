#!/bin/sh

set -e

on_error() {
    echo "Some error occurred"

    quasarnoded q wasm contract-state smart $ADDR1 '{"trapped_errors":{}}'

    afplay /System/Library/Sounds/Sosumi.aiff
    afplay /System/Library/Sounds/Sosumi.aiff
}

trap 'on_error' ERR

CHAIN_ID="quasar"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://127.0.0.1:26659"
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
# INIT2='{"lock_period":6,"pool_id":2,"pool_denom":"gamm/pool/2","base_denom":"stake","local_denom":"ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878","quote_denom":"fakestake","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
# INIT3='{"lock_period":6,"pool_id":3,"pool_denom":"gamm/pool/3","base_denom":"fakestake","local_denom":"ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1","quote_denom":"uosmo","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'

cd ../../smart-contracts

docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer-arm64:0.12.11

echo "Running store code"
RES=$(quasarnoded tx wasm store artifacts/lp_strategy-aarch64.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT1=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT1" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec $NODE --chain-id $CHAIN_ID)
ADDR1=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR1"

rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# quasarnoded tx wasm execute $ADDR1 '{"bond":{"id": "my-id"}}' -y --from alice --keyring-backend test --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518 $TXFLAG
# sleep 6

# OUT2=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT2" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec $NODE --chain-id $CHAIN_ID)
# ADDR2=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[1]')
# echo "Got address of deployed contract = $ADDR2"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icqhost --order unordered --version icq-1 --override
# rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# sleep 6

# OUT3=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT3" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec $NODE --chain-id $CHAIN_ID)
# ADDR3=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[2]')
# echo "Got address of deployed contract = $ADDR3"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icqhost --order unordered --version icq-1 --override
# rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# echo "primitive contracts:\n$ADDR1\n$ADDR2\n$ADDR3\n"

# BOND=$(printf 'quasarnoded tx wasm execute %s "{\"bond\":{\"id\":\"my-id\"}}" -y --from alice --keyring-backend test --gas-prices 10uqsr --gas auto --gas-adjustment 1.3 --node http://127.0.0.1:26659 --chain-id quasar --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518\n' $ADDR)
# echo "bonding tokens, to replay: "
# echo $BOND
# $BOND

VAULT_INIT='{"decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR1'","weight":"0.5","init":{"l_p":'$INIT1'}}]}'
#,{"address":"'$ADDR2'","weight":"0.333333333333","init":{"l_p":'$INIT2'}},{"address":"'$ADDR3'","weight":"0.333333333333","init":{"l_p":'$INIT3'}}]}'
echo $VAULT_INIT

echo "Running store code (vault)"
RES=$(quasarnoded tx wasm store artifacts/basic_vault-aarch64.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)

VAULT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')

echo "Got CODE_ID = $VAULT_CODE_ID"

echo "Deploying contract (vault)"
# # swallow output
OUT=$(quasarnoded tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec $NODE --chain-id $CHAIN_ID)
VAULT_ADDR=$(quasarnoded query wasm list-contract-by-code $VAULT_CODE_ID --output json $NODE | jq -r '.contracts[0]')

echo "Got address of deployed contract = $VAULT_ADDR (vault)"

echo "bonding multiple bonds"
echo "Command: quasarnoded tx wasm execute $VAULT_ADDR '{\"bond\":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1"
quasarnoded tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1
sleep 5
echo "second bond"
quasarnoded tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1
sleep 5
echo "third bond"
quasarnoded tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518,1000ibc/C053D637CCA2A2BA030E2C5EE1B28A16F71CCB0E45E8BE52766DC1B241B77878,1000ibc/391EB817CD435CDBDFC5C85301E06E1512800C98C0232E9C00AD95C77A73BFE1
sleep 5

# echo "Sleeping for 80 seconds"
# sleep 80

echo "Querying alice balance"
quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}' --output json

echo "Running unbond, command: quasarnoded tx wasm execute $VAULT_ADDR '{\"unbond\":{\"amount\":\"100\"}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID"
quasarnoded tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"100"}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID
echo $(quasarnoded tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"100"}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID)

# echo ""

# echo "Sleeping 120"
# sleep 120
# echo "Running unbond again to get funds back"
# quasarnoded tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"0"}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID

# echo "Sleeping 60"
# echo "Alice balances after unbond step:"
# quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}' --output json
# quasarnoded q bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec

afplay /System/Library/Sounds/Funk.aiff
say "I want to cry"

cd -
