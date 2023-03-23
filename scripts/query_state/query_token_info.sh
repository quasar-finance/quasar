#!/bin/bash

VAULT_ADDR="quasar1xzqhz0q969plap7awdjpls6vvrq57efk5vlkwr7kj5rzw9sq8j6s6wnxaj"

# get_token_info VS token_info
output=$(quasarnoded q wasm contract-state smart $VAULT_ADDR '{"token_info":{}}' --node https://rpc-tst5.qsr.network:443)

decimals=$(echo "$output" | grep "decimals" | awk '{print $2}')
name=$(echo "$output" | grep "name" | awk '{print $2}')
symbol=$(echo "$output" | grep "symbol" | awk '{print $2}')
thesis=$(echo "$output" | grep "thesis" | awk '{print $2}')
total_supply=$(echo "$output" | grep "total_supply" | awk '{print $2}' | tr -d '"')

if [ ! -f token_info.txt ] || ! grep -q "Timestamp;decimals;name;symbol;thesis;total_supply" token_info.txt; then
    echo "Timestamp;decimals;name;symbol;thesis;total_supply" > token_info.txt
fi

echo "$(date -u +"%Y-%m-%dT%H:%M:%S");$decimals;$name;$symbol;$thesis;$total_supply" >> token_info.txt