#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: bash generate_signed_upload_tx.sh <WASM_FILE> <DEPLOYER>"
    exit 1
fi

WASM_FILE=$1
DEPLOYER=$2

FEES=1000000uosmo
NODE=https://rpc.osmosis.zone:443
MULTISIG=osmo1vxq5h3encfyguulqeh26l8dlw9lavl3e2zw7n8
CHAIN=osmosis-1

set -e

osmosisd tx wasm store ${WASM_FILE} --from contract-upload --gas 25000000 --fees ${FEES} --chain-id ${CHAIN} --node ${NODE} --generate-only > tx.json
osmosisd tx sign tx.json --multisig=${MULTISIG} --sign-mode amino-json --chain-id ${CHAIN} --node ${NODE} --from ${DEPLOYER} --output-document ${DEPLOYER}-signed-tx.json