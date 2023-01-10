#!/bin/sh

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
#     denom: denom should be the denom of the token on osmosos, for now uosmo
#     local_denom: the denom of the token used locally, in this testing case: the denom of the path transfer/channel-1/uosmo
INIT='{"lock_period":"1209600","pool_id":1,"pool_denom":"gamm/pool/1","denom":"uosmo","local_denom":"ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B"}'

cd ../../smart-contracts

docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.12.6

echo "Running store code"
RES=$(quasarnoded tx wasm store artifacts/lp_strategy.wasm --from alice --keyring-backend test -y --output json -b block $TXFLAG) 
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value') 
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT" --from alice --keyring-backend test --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --no-admin $NODE --chain-id $CHAIN_ID)
ADDR=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR" --dst-port icqhost --order unordered --version icq-1 --override
rly transact channel quasar_osmosis --src-port "wasm.$ADDR" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-1","host_connection_id":"connection-0"}' --override

QMSG='{"channels": {}}'
CADDR=$(quasarnoded query wasm contract-state smart $ADDR "$QMSG" --output json | jq '.data.channels[] | select(.counterparty_endpoint.port_id=="icahost").channel_type.ica.counter_party_address')
CCHAN=$(quasarnoded query wasm contract-state smart $ADDR "$QMSG" --output json | jq '.data.channels[] | select(.counterparty_endpoint.port_id=="icahost").id')

MSG= $(printf '{"transfer":{"channel":"channel-0", "to_address": "cosmos1twes4wv4c28r0x6dnczgda5sm36khlv7ve8m89|transfer/channel-1:%s"}}' $CADDR)

echo "transferring funds over ibc message... to replay: \"quasarnoded tx wasm execute $ADDR "$MSG" -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1000uqsr\""
quasarnoded tx wasm execute $ADDR "$MSG" -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --amount 1000uqsr


AMOUNT="100000stake"
HOME_OSMOSIS=$HOME/.osmosis
echo $CADDR
echo "preloading the ICA address with $AMOUNT to play around with"
BANKTX=$(printf 'osmosisd tx bank send bob %s %s -y --keyring-backend test --node tcp://localhost:26679  --chain-id osmosis --gas 583610 --home %s' $CADDR $AMOUNT $HOME_OSMOSIS)
echo $BANKTX
$BANKTX

echo "joining pool and locking all lp tokens using preloaded funds"

JOINMSG=$(printf '{
  "deposit_and_lock_tokens": {
    "amount": "1000",
    "channel": %s,
    "denom": "uosmo",
    "pool_id": 1,
    "share_out_min_amount": "1"
  }
}' $CCHAN)

echo "joining pool, to replay: \"quasarnoded tx wasm execute $ADDR '$JOINMSG' -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID\""
quasarnoded tx wasm execute $ADDR "$JOINMSG" -y --from alice --keyring-backend test --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID
cd -