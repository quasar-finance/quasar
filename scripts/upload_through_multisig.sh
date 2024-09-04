#!/bin/bash

set -e

if [ "$#" -ne 1 ]; then
    echo "Too many arguments. Please provide a string containing the names of the json files of the signed transactions."
    echo "Usage: bash upload_through_multisig.sh \"<SIGNED_TX_1> <SIGNED_TX_2>\""
    exit 1
fi

SIGNED_TXS=$1
NODE=https://osmosis-rpc.publicnode.com:443
CHAIN=osmosis-1

osmosisd tx multisign tx.json contract-upload ${SIGNED_TXS} --chain-id ${CHAIN} --node ${NODE} --from contract-upload --output-document tx_ms.json
osmosisd tx broadcast tx_ms.json --chain-id ${CHAIN} --node ${NODE}
rm tx_ms.json