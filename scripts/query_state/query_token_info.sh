#!/bin/bash

source values.sh
VAULT_ADDR="$VAULT_ADDR" 
RPC="$RPC"

output=$(quasarnoded q wasm contract-state smart $VAULT_ADDR '{"token_info":{}}' --node $RPC)

decimals=$(echo "$output" | grep "decimals" | awk '{print $2}')
name=$(echo "$output" | grep "name" | awk '{print $2}')
symbol=$(echo "$output" | grep "symbol" | awk '{print $2}')
thesis=$(echo "$output" | grep "thesis" | awk '{print $2}')
total_supply=$(echo "$output" | grep "total_supply" | awk '{print $2}' | tr -d '"')

if [ ! -f token_info.txt ] || ! grep -q "timestamp;decimals;name;symbol;thesis;total_supply" token_info.txt; then
    echo "timestamp;decimals;name;symbol;thesis;total_supply" > token_info.txt
fi

echo "$(date -u +"%Y-%m-%dT%H:%M:%S");$decimals;$name;$symbol;$thesis;$total_supply" >> token_info.txt