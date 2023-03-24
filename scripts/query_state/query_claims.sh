#!/bin/bash

source values.sh
VAULT_ADDR="$VAULT_ADDR" 
RPC="$RPC"
USER_ADDR_LIST="$USER_ADDR_LIST"

for USER_ADDR in "${USER_ADDR_LIST[@]}"; do
    OUTPUT=$(quasarnoded q wasm contract-state smart $VAULT_ADDR '{"claims":{"address": "'"$USER_ADDR"'"}}' --node $RPC -o json | jq)
    echo "$(date -u +"%Y-%m-%dT%H:%M:%S");$USER_ADDR;$OUTPUT" >> log_claims.txt
done

# need claims to not be empty to parse the data
