#!/bin/bash

# ./query_all_strategies.sh <contract_address>

CONTRACT=$1
NODE_URL=${2:-tcp://localhost:26679}

QUERY_MSG='{"get_all_strategies": {}}'

osmosisd q wasm contract-state smart $CONTRACT "$QUERY_MSG" --node $NODE_URL --output json

