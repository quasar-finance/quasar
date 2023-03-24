#!/bin/bash

source values.sh
VAULT_ADDR="$VAULT_ADDR" 
RPC="$RPC"
USER_ADDR_LIST="$USER_ADDR_LIST"

if [ ! -f log_balance.txt ] || ! grep -q "timestamp;base_denom;ica_address;lp_denom;d_unlocked_shares;locked_shares;w_unlocked_shares;quote_denom" log_balance.txt; then
    echo "timestamp;base_denom;ica_address;lp_denom;d_unlocked_shares;locked_shares;w_unlocked_shares;quote_denom" > log_balance.txt
fi

for USER_ADDR in "${USER_ADDR_LIST[@]}"; do
    OUTPUT=$(quasarnoded q wasm contract-state smart $VAULT_ADDR '{"balance":{"address": "'"$USER_ADDR"'"}}' --node $RPC -o json | jq)
    BALANCE=$(echo "$OUTPUT" | jq -r '.balance')
    echo "$(date -u +"%Y-%m-%dT%H:%M:%S");$USER_ADDR;$BALANCE" #>> log_balance.txt
done