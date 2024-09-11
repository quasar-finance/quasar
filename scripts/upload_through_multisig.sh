#!/bin/bash

set -e

if [ "$#" -ne 2 ]; then
    echo "Too many arguments. Please provide a string containing the names of the json files of the signed transactions."
    echo "Usage: bash upload_through_multisig.sh <MULTISIG> \"<SIGNED_TX_1> <SIGNED_TX_2>\""
    exit 1
fi

MULTISIG=$1
SIGNED_TXS=$2
NODE=https://rpc.osmosis.zone:443
CHAIN=osmosis-1

osmosisd tx multisign tx.json ${MULTISIG} ${SIGNED_TXS} --chain-id ${CHAIN} --node ${NODE} --from ${MULTISIG} --output-document tx_ms.json
osmosisd tx broadcast tx_ms.json --chain-id ${CHAIN} --node ${NODE}
rm tx_ms.json