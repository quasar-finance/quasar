#!/bin/sh

set -e

CHAIN_ID="qsr-questnet-04"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
RPC="http://node3.tst4.qsr.network:26657/"
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"

INIT1='{"lock_period":300,"pool_id":1,"pool_denom":"gamm/pool/1","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"ibc/6BEEEE6CC17BA0EE37857A1E2AF6EC53C50DB6B6F89463A3933D0859C4CF6087","return_source_channel":"channel-0","transfer_channel":"channel-0"}'
INIT2='{"lock_period":300,"pool_id":2,"pool_denom":"gamm/pool/2","base_denom":"ibc/DA3CEF7DEAF6983032E061030C63E13262957D223E9EDBBB7AF9B69F8F8BA090","local_denom":"uayy","quote_denom":"uosmo","return_source_channel":"channel-0","transfer_channel":"channel-0"}'
INIT3='{"lock_period":300,"pool_id":3,"pool_denom":"gamm/pool/3","base_denom":"ibc/6BEEEE6CC17BA0EE37857A1E2AF6EC53C50DB6B6F89463A3933D0859C4CF6087","local_denom":"uqsr","quote_denom":"ibc/DA3CEF7DEAF6983032E061030C63E13262957D223E9EDBBB7AF9B69F8F8BA090","return_source_channel":"channel-0","transfer_channel":"channel-0"}'

cd ../smart-contracts

docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.11

echo "Running store code"
RES=$(quasarnoded tx wasm store artifacts/lp_strategy.wasm --from laurens-2 --keyring-backend test -y --output json -b block $TXFLAG) 
echo $RES
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value') 
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
ADDR1=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')OUT1=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT1" --from laurens-2 --keyring-backend test -y  --label "primitive-1" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1wzdhlvurmav577eu7n3z329eg5ykaez050az8l"  $NODE --chain-id $CHAIN_ID)

echo "Got address of deployed contract = $ADDR1"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icqhost --order unordered --version icq-1 --override
# rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# sleep 6

OUT2=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT2" --from laurens-2 --keyring-backend test --label "primitive-2" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1wzdhlvurmav577eu7n3z329eg5ykaez050az8l" $NODE --chain-id $CHAIN_ID)
ADDR2=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[1]')
echo "Got address of deployed contract = $ADDR2"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icqhost --order unordered --version icq-1 --override
# rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# sleep 6

OUT3=$(quasarnoded tx wasm instantiate $CODE_ID "$INIT3" --from laurens-2 --keyring-backend test --label "primitive-3" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1wzdhlvurmav577eu7n3z329eg5ykaez050az8l" $NODE --chain-id $CHAIN_ID)
ADDR3=$(quasarnoded query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[2]')
echo "Got address of deployed contract = $ADDR3"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icqhost --order unordered --version icq-1 --override
# rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

echo "primitive contracts:\n$ADDR1\n$ADDR2\n$ADDR3\n"

VAULT_INIT='{"decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR1'","weight":"0.333333333333","init":{"l_p":'$INIT1'}},{"address":"'$ADDR2'","weight":"0.333333333333","init":{"l_p":'$INIT2'}},{"address":"'$ADDR3'","weight":"0.333333333333","init":{"l_p":'$INIT3'}}]}'
echo $VAULT_INIT

echo "Running store code (vault)"
RES=$(quasarnoded tx wasm store artifacts/basic_vault.wasm --from laurens-2 --keyring-backend test -y --output json -b block $TXFLAG)

VAULT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')

echo "Got CODE_ID = $VAULT_CODE_ID"

echo "Deploying contract (vault)"
# swallow output
OUT=$(quasarnoded tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from laurens-2 --keyring-backend test --label "vault-contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1wzdhlvurmav577eu7n3z329eg5ykaez050az8l" $NODE --chain-id $CHAIN_ID)
VAULT_ADDR=$(quasarnoded query wasm list-contract-by-code $VAULT_CODE_ID --output json $NODE | jq -r '.contracts[0]')

echo "Got address of deployed contract = $VAULT_ADDR (vault)"