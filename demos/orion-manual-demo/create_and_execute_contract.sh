#!/bin/sh

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
INIT='{"lock_period":60,"pool_id":1,"pool_denom":"gamm/pool/1","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"stake","return_source_channel":"channel-0","transfer_channel":"channel-0"}'

cd ../../smart-contracts

docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.11
echo "Running store code"
RES=$(quasarnoded tx wasm store artifacts/lp_strategy.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG) 
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value') 
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --no-admin $NODE --chain-id $CHAIN_ID)
ADDR=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR"

rly transact channel quasar_osmosis --src-port "wasm.$ADDR" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

QMSG='{"channels": {}}'
CADDR=$(quasarnoded query wasm contract-state smart $ADDR "$QMSG" --output json | jq '.data.channels[] | select(.counterparty_endpoint.port_id=="icahost").channel_type.ica.counter_party_address')
CCHAN=$(quasarnoded query wasm contract-state smart $ADDR "$QMSG" --output json | jq '.data.channels[] | select(.counterparty_endpoint.port_id=="icahost").id')


AMOUNT="100000uosmo"
HOME_OSMOSIS=$HOME/.osmosis
# echo $CADDR
# echo "preloading the ICA address with $AMOUNT to play around with"
BANKTX=$(printf 'osmosisd tx bank send bob %s %s -y --keyring-backend test --node tcp://localhost:26679  --chain-id osmosis --gas 583610 --home %s' $CADDR $AMOUNT $HOME_OSMOSIS)
echo $BANKTX
# $BANKTX

# BONDMSG='{"bond": {"id": "my-id"}}'
# BOND=$(printf 'quasarnoded tx wasm execute %s '%s' -y --from alice --keyring-backend test --gas-prices 10uqsr --gas auto --gas-adjustment 1.3 --node http://127.0.0.1:26659 --chain-id quasar --amount 1000ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817\n' $ADDR $BONDMSG)

# echo "bonding tokens, to replay: "
# echo $BOND
# $BOND

VAULT_INIT='{"decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR'","weight":"1.0","init":{"l_p":'$INIT'}}]}'
echo $VAULT_INIT

echo "Running store code (vault)"
RES=$(quasarnoded tx wasm store target/wasm32-unknown-unknown/release/basic_vault.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)

VAULT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')

echo "Got CODE_ID = $VAULT_CODE_ID"

echo "Deploying contract (vault)"
# swallow output
OUT=$(quasarnoded tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --no-admin $NODE --chain-id $CHAIN_ID)
VAULT_ADDR=$(quasarnoded query wasm list-contract-by-code $VAULT_CODE_ID --output json $NODE | jq -r '.contracts[0]')

echo "Got address of deployed contract = $VAULT_ADDR (vault)"

echo "Command: quasarnoded tx wasm execute $VAULT_ADDR '{\"bond\":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 100ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518"
quasarnoded tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 100ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518

cd -