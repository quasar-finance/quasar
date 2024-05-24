#!/bin/bash

#./execute_create_strategy.sh <contract_address> "example_strategy" "example_description"


CONTRACT=$1
STRATEGY_NAME=$2
STRATEGY_DESCRIPTION=$3
NODE_URL=${4:-tcp://localhost:26679}
FROM_ACCOUNT=${5:-alice}
KEYRING_BACKEND=${6:-test}
HOME_DIR=${7:-~/.osmosis}
CHAIN_ID=${8:-osmosis}
FEES=${9:-300000uosmo}
GAS=${10:-7000000}

#EXECUTE_MSG="{\"vault_extension\": {\"create_strategy\": {\"name\": \"$STRATEGY_NAME\", \"description\": \"$STRATEGY_DESCRIPTION\"}}}"
EXECUTE_MSG="{\"vault_extension\": {\"pro_extension\": {\"create_strategy\": {\"name\": \"$STRATEGY_NAME\", \"description\": \"$STRATEGY_DESCRIPTION\"}}}}"

#EXECUTE_MSG="{\"create_strategy\": {\"name\": \"$STRATEGY_NAME\", \"description\": \"$STRATEGY_DESCRIPTION\"}}"

osmosisd tx wasm execute $CONTRACT "$EXECUTE_MSG" --from $FROM_ACCOUNT --keyring-backend $KEYRING_BACKEND --home $HOME_DIR --chain-id $CHAIN_ID --fees $FEES --gas $GAS --node $NODE_URL --yes


