#!/bin/bash

CONTRACT=$1
AMOUNT=$2
DENOM=${3:-uosmo}
NODE_URL=${4:-tcp://localhost:26679}
FROM_ACCOUNT=${5:-alice}
KEYRING_BACKEND=${6:-test}
HOME_DIR=${7:-~/.osmosis}
CHAIN_ID=${8:-osmosis}
FEES=${9:-300000uosmo}
GAS=${10:-7000000}

EXECUTE_MSG="{\"deposit\": {\"amount\": \"$AMOUNT\"}}"

osmosisd tx wasm execute $CONTRACT "$EXECUTE_MSG" --amount ${AMOUNT}${DENOM} --from $FROM_ACCOUNT --keyring-backend $KEYRING_BACKEND --home $HOME_DIR --chain-id $CHAIN_ID --fees $FEES --gas $GAS --node $NODE_URL --yes

