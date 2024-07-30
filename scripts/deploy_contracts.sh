#!/bin/sh

set -e

CHAIN_ID="qsr-questnet-05"
TESTNET_NAME="quasar"
FEE_DENOM="ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518"
RPC="https://rpc-tst5.qsr.network:443/"
NODE="--node $RPC"
FEE="0.000021$FEE_DENOM"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices $FEE --gas auto --gas-adjustment 1.3"

INIT1='{"lock_period":300,"pool_id":7,"pool_denom":"gamm/pool/7","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"ibc/32B1C34EB2C19DB8BC26CC14DE8E08D2C8A7CC4158FE60ED4EF6B2FB06AE9141","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
INIT2='{"lock_period":300,"pool_id":8,"pool_denom":"gamm/pool/8","base_denom":"uosmo","local_denom":"ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518","quote_denom":"ibc/3D346D544AB66D6872D81A69794BF17473D94B601BD503DA9F92E368167FAB0C","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
# INIT2='{"lock_period":300,"pool_id":2,"pool_denom":"gamm/pool/2","base_denom":"ibc/DA3CEF7DEAF6983032E061030C63E13262957D223E9EDBBB7AF9B69F8F8BA090","local_denom":"uayy","quote_denom":"uosmo","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'
# INIT3='{"lock_period":300,"pool_id":3,"pool_denom":"gamm/pool/3","base_denom":"ibc/6BEEEE6CC17BA0EE37857A1E2AF6EC53C50DB6B6F89463A3933D0859C4CF6087","local_denom":"uqsr","quote_denom":"ibc/DA3CEF7DEAF6983032E061030C63E13262957D223E9EDBBB7AF9B69F8F8BA090","return_source_channel":"channel-0","transfer_channel":"channel-0","expected_connection":"connection-0"}'

cd ../smart-contracts

# docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer-arm64:0.12.11

echo "Running store code"
RES=$(quasard tx wasm store artifacts/lp_strategy-aarch64.wasm --from testnet-relayer --keyring-backend test -y --output json -b block $TXFLAG)
echo $RES
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
# quasard tx wasm instantiate $CODE_ID "$INIT1" --from testnet-relayer --keyring-backend test -y --label "primitive-1" --gas-prices $FEE --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1wzdhlvurmav577eu7n3z329eg5ykaez050az8l" $NODE --chain-id $CHAIN_ID
OUT1=$(quasard tx wasm instantiate $CODE_ID "$INIT1" --from testnet-relayer --keyring-backend test -y --label "primitive-1" --gas-prices $FEE --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1wzdhlvurmav577eu7n3z329eg5ykaez050az8l" $NODE --chain-id $CHAIN_ID)
ADDR1=$(quasard query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR1"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icqhost --order unordered --version icq-1 --override
# rly transact channel quasar_osmosis --src-port "wasm.$ADDR1" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# sleep 6

OUT2=$(quasard tx wasm instantiate $CODE_ID "$INIT2" --from testnet-relayer --keyring-backend test --label "primitive-2" --gas-prices $FEE --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1wzdhlvurmav577eu7n3z329eg5ykaez050az8l" $NODE --chain-id $CHAIN_ID)
# quasard query wasm list-contract-by-code $CODE_ID --output json $NODE
ADDR2=$(quasard query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[1]')
echo "Got address of deployed contract = $ADDR2"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icqhost --order unordered --version icq-1 --override
# rly transact channel quasar_osmosis --src-port "wasm.$ADDR2" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

# sleep 6

# OUT3=$(quasard tx wasm instantiate $CODE_ID "$INIT3" --from testnet-relayer --keyring-backend test --label "primitive-3" --gas-prices $FEE --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1wzdhlvurmav577eu7n3z329eg5ykaez050az8l" $NODE --chain-id $CHAIN_ID)
# ADDR3=$(quasard query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[2]')
# echo "Got address of deployed contract = $ADDR3"

# rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icqhost --order unordered --version icq-1 --override
# rly transact channel quasar_osmosis --src-port "wasm.$ADDR3" --dst-port icahost --order ordered --version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --override

echo "primitive contracts:\n$ADDR1\n$ADDR2\n$ADDR3\n"

VAULT_INIT='{"thesis":"yurmum","decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR1'","weight":"0.333333333333","init":{"l_p":'$INIT1'}},{"address":"'$ADDR2'","weight":"0.333333333333","init":{"l_p":'$INIT2'}}]}'
# VAULT_INIT='{"thesis":"yurmum","decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR1'","weight":"0.333333333333","init":{"l_p":'$INIT1'}},{"address":"'$ADDR2'","weight":"0.333333333333","init":{"l_p":'$INIT2'}},{"address":"'$ADDR3'","weight":"0.333333333333","init":{"l_p":'$INIT3'}}]}'
echo $VAULT_INIT

echo "Running store code (vault)"
RES=$(quasard tx wasm store artifacts/basic_vault-aarch64.wasm --from testnet-relayer --keyring-backend test -y --output json -b block $TXFLAG)

VAULT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[1].value')

echo "Got CODE_ID = $VAULT_CODE_ID"

echo "Deploying contract (vault)"
# swallow output
OUT=$(quasard tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from testnet-relayer --keyring-backend test --label "vault-contract" --gas-prices $FEE --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1wzdhlvurmav577eu7n3z329eg5ykaez050az8l" $NODE --chain-id $CHAIN_ID)
VAULT_ADDR=$(quasard query wasm list-contract-by-code $VAULT_CODE_ID --output json $NODE | jq -r '.contracts[0]')

echo "Got address of deployed contract = $VAULT_ADDR (vault)"
