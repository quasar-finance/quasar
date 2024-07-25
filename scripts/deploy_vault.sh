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

ADDR1="quasar1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqu5prdk"
ADDR2="quasar1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqgsk7e6"
ADDR3="quasar1xr3rq8yvd7qplsw5yx90ftsr2zdhg4e9z60h5duusgxpv72hud3syhezhc"

cd ../smart-contracts

# docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.11


VAULT_INIT='{"decimals":6,"symbol":"ORN","min_withdrawal":"1","name":"ORION","primitives":[{"address":"'$ADDR1'","weight":"0.333333333333","init":{"l_p":'$INIT1'}},{"address":"'$ADDR2'","weight":"0.333333333333","init":{"l_p":'$INIT2'}},{"address":"'$ADDR3'","weight":"0.333333333333","init":{"l_p":'$INIT3'}}]}'
echo $VAULT_INIT

echo "Running store code (vault)"
RES=$(quasard tx wasm store artifacts/basic_vault.wasm --from test-laurens --keyring-backend os -y --output json -b block $TXFLAG)

VAULT_CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')

echo "Got CODE_ID = $VAULT_CODE_ID"

echo "Deploying contract (vault)"
# swallow output
OUT=$(quasard tx wasm instantiate $VAULT_CODE_ID "$VAULT_INIT" --from test-laurens --keyring-backend os --label "vault-contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --admin "quasar1wzdhlvurmav577eu7n3z329eg5ykaez050az8l" $NODE --chain-id $CHAIN_ID)
VAULT_ADDR=$(quasard query wasm list-contract-by-code $VAULT_CODE_ID --output json $NODE | jq -r '.contracts[0]')

echo "Got address of deployed contract = $VAULT_ADDR (vault)"