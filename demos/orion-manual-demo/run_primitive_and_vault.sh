#!/bin/sh

set -e

CHAIN_ID="quasar"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://127.0.0.1:26659"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"
echo $NODE
#     duration is  60 sec/min * 60 min/hr * 24hr * 14days "1209600"
#     pool_id is hardcoded to 1 for this testing setup, expected to be done by the instantiater on local/testnet
#     pool_denom should be looked up and hardcoded aswell
#     base_denom: base_denom should be the denom of the token on osmosos, for now uosmo
#     local_denom: the denom of the token used locally, in this testing case: the denom of the path transfer/channel-1/uosmo
#     quote_denom is the denom other denom in the pool, stake for now
INIT='{"lock_period":60,"pool_id":1,"pool_denom":"gamm/pool/1","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"stake","return_source_channel":"channel-0","transfer_channel":"channel-0"}'

cd ../../smart-contracts/contracts/lp-strategy

RUSTFLAGS='-C link-arg=-s' cargo wasm
# docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.12.6

cd ../..

echo "Running store code"
RES=$(quasard tx wasm store target/wasm32-unknown-unknown/release/lp_strategy.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT=$(quasard tx wasm instantiate $CODE_ID "$INIT" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --no-admin $NODE --chain-id $CHAIN_ID)
ADDR=$(quasard query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR"

rly transact channel quasar_osmosis --src-port "wasm.$ADDR" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

QMSG='{"channels": {}}'
CADDR=$(quasard query wasm contract-state smart $ADDR "$QMSG" --output json | jq '.data.channels[] | select(.counterparty_endpoint.port_id=="icahost").channel_type.ica.counter_party_address')
CCHAN=$(quasard query wasm contract-state smart $ADDR "$QMSG" --output json | jq '.data.channels[] | select(.counterparty_endpoint.port_id=="icahost").id')

echo $CADDR

# trim CADDR by one character on both sides
CADDR2=$(echo $CADDR | cut -c 2- | rev | cut -c 2- | rev)

AMOUNT="100000uosmo"
HOME_OSMOSIS=$HOME/.osmosis
# echo $CADDR
# echo "preloading the ICA address with $AMOUNT to play around with"
BANTX=$(osmosisd tx bank send bob $CADDR2 $AMOUNT -y --keyring-backend test --node tcp://localhost:26679 --chain-id osmosis --gas 583610 --home $HOME_OSMOSIS)
# BANKTX=$(printf 'osmosisd tx bank send bob %s %s -y --keyring-backend test --node tcp://localhost:26679  --chain-id osmosis --gas 583610 --home %s' $CADDR $AMOUNT $HOME_OSMOSIS)
# echo $BANTX
# $BANKTX

echo "joining pool and locking all lp tokens using preloaded funds"

JOINMSG=$(printf '{
  "deposit_and_lock_tokens": {
    "amount": "1000",
    "denom": "uosmo",
    "pool_id": 1,
    "share_out_min_amount": "1"
  }
}')

echo "joining pool, to replay: \"quasard tx wasm execute $ADDR '$JOINMSG' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID\""
quasard tx wasm execute $ADDR "$JOINMSG" -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID

## MULTI ASSET VAULT ZONE
echo "Starting multi-asset vault init"

VAULT_INIT='{"decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR'","weight":"1.0","init":{"l_p":'$INIT'}}]}'
echo $VAULT_INIT

RUSTFLAGS='-C link-arg=-s' cargo wasm
# docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer-arm64:0.12.10
# sleep 4

echo "Running store code (vault)"
RES=$(quasard tx wasm store target/wasm32-unknown-unknown/release/basic_vault.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG)

VAULT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')

echo "Got CODE_ID = $VAULT_CODE_ID"

echo "Deploying contract (vault)"
# swallow output
OUT=$(quasard tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --no-admin $NODE --chain-id $CHAIN_ID)
VAULT_ADDR=$(quasard query wasm list-contract-by-code $VAULT_CODE_ID --output json $NODE | jq -r '.contracts[0]')

echo "Got address of deployed contract = $VAULT_ADDR (vault)"

# echo "Running a primitive deposit manually to circumvent the cold start issue with primitives"
# quasard tx wasm execute $ADDR '{"bond": {"id": "test"}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 10ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518
# sleep 4

echo "Running deposit (vault)"
echo "Command: quasard tx wasm execute $VAULT_ADDR '{\"bond\":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 100ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518"
quasard tx wasm execute $VAULT_ADDR '{"bond":{}}' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 100ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518

cd -
