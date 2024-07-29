#!/bin/sh

set -e

CHAIN_ID="quasar"
TESTNET_NAME="quasar"
FEE_DENOM="uqsr"
# STAKE_DENOM="urock"
BECH32_HRP="quas"
WASMD_VERSION="v0.23.0"
CONFIG_DIR=".wasmd"
BINARY="wasmd"
COSMJS_VERSION="v0.27.1"
GENESIS_URL="https://raw.githubusercontent.com/CosmWasm/testnets/master/cliffnet-1/config/genesis.json"
RPC="http://127.0.0.1:26659"
# RPC="https://rpc.cliffnet.cosmwasm.com:443"
LCD="https://lcd.cliffnet.cosmwasm.com"
FAUCET="https://faucet.cliffnet.cosmwasm.com"
# https://rpc-edgenet.dev-osmosis.zone/
# https://lcd-edgenet.dev-osmosis.zone/
NODE="--node $RPC"
TXFLAG="$NODE --chain-id $CHAIN_ID --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3"

# Prep quasar & osmo chains

read -p "Should we run setup ? (Y/n)" -n 1 -r
echo   
if [[ $REPLY =~ ^[Yy]$ ]]
then
    # Get Oracle Prices
    read -p "Do we have oracle prices? Refresh here until yes: http://localhost:1311/quasarlabs/quasarnode/qoracle/oracle_prices (Y/n)" -n 1 -r
    echo   
    if [[ ! $REPLY =~ ^[Yy]$ ]]
    then
        echo "Exiting deploy_and_exec_contract.sh."
        exit 1 || return 1 # handle exits from shell or function but don't exit interactive shell
    fi

    # Update osmo chain params
    echo
    echo "Updating osmosis chain params on quasar"
    quasard tx qoracle update-osmosis-chain-params --node tcp://localhost:26659 --from alice --home ~/.quasarnode --chain-id quasar --output json --keyring-backend test
    echo

    read -p "Do we have chain params? Refresh here until yes (epochs info should not be empty): http://localhost:1311/quasarlabs/quasarnode/qoracle/osmosis/chain_params (Y/n)" -n 1 -r
    echo   
    if [[ ! $REPLY =~ ^[Yy]$ ]]
    then
        echo "Exiting deploy_and_exec_contract.sh."
        exit 1 || return 1 # handle exits from shell or function but don't exit interactive shell
    fi

    # Deploy Osmo pool
    echo
    echo "Deploying Osmosis Pool..."
    osmosisd tx gamm create-pool --pool-fi\le demo_pool.json --home ~/.osmosis --chain-id osmosis --node=http://localhost:26679 --from alice --gas=300000 --output json --keyring-backend test
    echo
    echo "Does quasar have the osmosis pool (maximum 1 minute)?\nOsmosis link:http://localhost:1312/osmosis/gamm/v1beta1/pools \nQuasar link: http://localhost:1311/quasarlabs/quasarnode/qoracle/osmosis/pools\n"

    read -p "(Y/n)" -n 1 -r
    echo   
    if [[ ! $REPLY =~ ^[Yy]$ ]]
    then
        echo "Exiting deploy_and_exec_contract.sh."
        exit 1 || return 1 # handle exits from shell or function but don't exit interactive shell
    fi
fi


INIT="{}"
MSG1='{"demo_osmosis_pools":{}}'
MSG2='{"demo_osmosis_pool_info":{}}'
MSG3='{"demo_oracle_prices":{}}'

cd ../../smart-contracts

# docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.12.6
RUSTFLAGS='-C link-arg=-s' cargo wasm

echo "Running store code"
RES=$(quasard tx wasm store target/wasm32-unknown-unknown/release/qoracle_bindings_test.wasm --from alice -y --output json -b block $TXFLAG)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value') 
echo "Got CODE_ID = $CODE_ID"

echo "Deploying contract"
# swallow output
OUT=$(quasard tx wasm instantiate $CODE_ID "$INIT" --from alice --label "my first contract" --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 -b block -y --no-admin $NODE --chain-id $CHAIN_ID)
ADDR=$(quasard query wasm list-contract-by-code $CODE_ID --output json $NODE | jq -r '.contracts[0]')
echo "Got address of deployed contract = $ADDR"

echo "Executing message... ('$MSG1')"
quasard tx wasm execute $ADDR "$MSG1" --from alice --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --log_level trace

sleep 5 # keep getting account sequence mismatch
echo 

echo "Executing message... ('$MSG2')"
quasard tx wasm execute $ADDR "$MSG2" --from alice --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --log_level trace

sleep 5 # keep getting account sequence mismatch
echo

echo "Executing message... ('$MSG3')"
quasard tx wasm execute $ADDR "$MSG3" --from alice --gas-prices 10$FEE_DENOM --gas auto --gas-adjustment 1.3 $NODE --chain-id $CHAIN_ID --log_level trace

cd -