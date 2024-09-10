#!/bin/bash

set -e

if [ "$#" -ne 3 ]; then
    echo "Usage: bash generate_signed_upload_tx.sh <WASM_FILE> <DEPLOYER> <MULTISIG>"
    echo "DEPLOYER and MULTISIG refer to the names of keys that are registered with osmosisd"
    exit 1
fi

WASM_FILE=$1
DEPLOYER=$2
MULTISIG=$3

FEES=1000000uosmo
NODE=https://rpc.osmosis.zone:443
MULTISIG_ADDRESS=$(osmosisd keys show ${MULTISIG} | grep address | sed "s/- address: //g")
CHAIN=osmosis-1

osmosisd tx wasm store ${WASM_FILE} --from ${MULTISIG} --gas 25000000 --fees ${FEES} --chain-id ${CHAIN} --node ${NODE} --generate-only > tx.json
osmosisd tx sign tx.json --multisig=${MULTISIG_ADDRESS} --sign-mode amino-json --chain-id ${CHAIN} --node ${NODE} --from ${DEPLOYER} --output-document ${DEPLOYER}-signed-tx.json