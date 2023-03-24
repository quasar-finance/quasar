#!/bin/bash

source values.sh
VAULT_ADDR="$VAULT_ADDR" 
RPC="$RPC"

output=$(quasarnoded q wasm contract-state smart $VAULT_ADDR '{"investment":{}}' --node $RPC)

min_withdrawal=$(echo "$output" | grep "min_withdrawal" | awk '{print $2}' | tr -d '"')
owner=$(echo "$output" | grep "owner" | awk '{print $2}' | tr -d '"')
addresses=$(echo "$output" | grep "address:" | awk '{print $3}')
weights=$(echo "$output" | grep "weight" | awk '{print $2}' | tr -d '"')

if [ ! -f log_investment.txt ] || ! grep -q "timestamp;vault_address;min_withdrawal;owner" log_investment.txt || ! grep -q "timestamp;address;base_denom;pool_id;local_denom;expected_connection;lock_period;pool_denom;quote_denom;return_source_channel;transfer_channel;weight" log_investment.txt; then
    echo "timestamp;vault_address;min_withdrawal;owner" > log_investment.txt
    echo "timestamp;address;base_denom;pool_id;local_denom;expected_connection;lock_period;pool_denom;quote_denom;return_source_channel;transfer_channel;weight" >> log_investment.txt
fi

echo "$(date -u +"%Y-%m-%dT%H:%M:%S");$VAULT_ADDR;$min_withdrawal;$owner" >> log_investment.txt

for address in $addresses; do
    base_denom=$(echo "$output" | grep -A 12 "address: $address" | grep "base_denom" | awk '{print $2}')
    expected_connection=$(echo "$output" | grep -A 12 "address: $address" | grep "expected_connection" | awk '{print $2}')
    local_denom=$(echo "$output" | grep -A 12 "address: $address" | grep "local_denom" | awk '{print $2}' | tr -d '"')
    lock_period=$(echo "$output" | grep -A 12 "address: $address" | grep "lock_period" | awk '{print $2}')
    pool_denom=$(echo "$output" | grep -A 12 "address: $address" | grep "pool_denom" | awk '{print $2}')
    pool_id=$(echo "$output" | grep -A 12 "address: $address" | grep "pool_id" | awk '{print $2}')
    quote_denom=$(echo "$output" | grep -A 12 "address: $address" | grep "quote_denom" | awk '{print $2}')
    return_source_channel=$(echo "$output" | grep -A 12 "address: $address" | grep "return_source_channel" | awk '{print $2}')
    transfer_channel=$(echo "$output" | grep -A 12 "address: $address" | grep "transfer_channel" | awk '{print $2}')
    weight=$(echo "$output" | grep -A 12 "address: $address" | grep "weight" | awk '{print $2}' | tr -d '"')
    echo "$(date -u +"%Y-%m-%dT%H:%M:%S");$address;$base_denom;$expected_connection;$local_denom;$lock_period;$pool_denom;$pool_id;$quote_denom;$return_source_channel;$transfer_channel;$weight" >> log_investment.txt
done