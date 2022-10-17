#!/bin/sh

CHAIN_ID="quasar"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://127.0.0.1:26659"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"
echo $NODE
# the callback_address is the address of the orion module
INIT='{"default_timeout": 1000}'

cd ../../smart-contracts

docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.12.6

echo "Running store code"
RES=$(quasarnoded tx wasm store artifacts/ica.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG) 
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value') 
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --no-admin $NODE --chain-id $CHAIN_ID)
ADDR=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR"

echo "setting up channel"
rly transact channel quasar_osmosis --src-port "wasm.$ADDR" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

CHANNEL=$(rly q channels quasar | jq  -s --arg ADDR $ADDR '.[] | select(.port_id=="wasm." + $ADDR).channel_id')
SENDER=$(rly q channels quasar | jq  -s --arg ADDR $ADDR '.[] | select(.port_id=="wasm." + $ADDR).version | fromjson | .address')
MSG='{"join_pool":{"channel": '$CHANNEL', "sender": '$SENDER', "pool_id": "1", "share_out_amount": "1", "token_in_maxs":[{"denom": "uosmo", "amount": "1"}]}}'

# echo "Executing register ica message... ('$MSG')"
quasarnoded tx wasm execute $ADDR "$MSG" -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID

cd -