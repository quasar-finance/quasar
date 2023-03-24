#!/bin/bash

source values.sh
VAULT_ADDR="$VAULT_ADDR" 
RPC="$RPC"

output=$(quasarnoded q wasm contract-state smart $VAULT_ADDR '{"get_tvl_info":{}}' --node https://quasar-rpc.polkachu.com:443)

ica_address=$(echo "$output" | grep "ica_address" | awk '{print $2}')

if [ ! -f tvl_info.txt ] || ! grep -q "timestamp;base_denom;ica_address;lp_denom;d_unlocked_shares;locked_shares;w_unlocked_shares;quote_denom" tvl_info.txt; then
    echo "timestamp;base_denom;ica_address;lp_denom;d_unlocked_shares;locked_shares;w_unlocked_shares;quote_denom" > tvl_info.txt
fi

for ica_address in $ica_address; do
    base_denom=$(echo "$output" | grep -A 6 -B 1 "ica_address: $ica_address" | grep "base_denom" | awk '{print $3}')
    lp_denom=$(echo "$output" | grep -A 6 "ica_address: $ica_address" | grep "lp_denom" | awk '{print $2}')
    d_unlocked_shares=$(echo "$output" | grep -A 6 "ica_address: $ica_address" | grep "d_unlocked_shares" | awk '{print $2}'| tr -d '"')
    locked_shares=$(echo "$output" | grep -A 6 "ica_address: $ica_address" | grep " locked_shares" | awk '{print $2}' | tr -d '"')
    w_unlocked_shares=$(echo "$output" | grep -A 6 "ica_address: $ica_address" | grep "w_unlocked_shares" | awk '{print $2}' | tr -d '"')
    quote_denom=$(echo "$output" | grep -A 6 "ica_address: $ica_address" | grep "quote_denom" | awk '{print $2}')
    echo "$(date -u +"%Y-%m-%dT%H:%M:%S");$base_denom;$ica_address;$lp_denom;$d_unlocked_shares;$locked_shares;$w_unlocked_shares;$quote_denom" >> tvl_info.txt
done