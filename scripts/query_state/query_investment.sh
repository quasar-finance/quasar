#!/bin/bash

VAULT_ADDR="quasar1xzqhz0q969plap7awdjpls6vvrq57efk5vlkwr7kj5rzw9sq8j6s6wnxaj"

output=$(quasarnoded q wasm contract-state smart $VAULT_ADDR '{"investment":{}}' --node https://rpc-tst5.qsr.network:443)

min_withdrawal=$(echo "$output" | grep "min_withdrawal" | awk '{print $2}' | tr -d '"')
owner=$(echo "$output" | grep "owner" | awk '{print $2}' | tr -d '"')
addresses=$(echo "$output" | grep "address:" | awk '{print $3}')
weights=$(echo "$output" | grep "weight" | awk '{print $2}' | tr -d '"')

if [ ! -f investment.txt ] || ! grep -q "vault_address;min_withdrawal;owner" investment.txt || ! grep -q "Timestamp;Address;Local Denom;Pool ID;Base Denom;Expected Connection;Lock Period;Pool Denom;Quote Denom;Return Source Channel;Transfer Channel;Weight" investment.txt; then
    echo "Timestamp;vault_address;min_withdrawal;owner" > investment.txt
    echo "Timestamp;Address;Base Denom;Pool ID;Local Denom;Expected Connection;Lock Period;Pool Denom;Quote Denom;Return Source Channel;Transfer Channel;Weight" >> investment.txt
fi

echo "$(date -u +"%Y-%m-%dT%H:%M:%S");$VAULT_ADDR;$min_withdrawal;$owner" >> investment.txt

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
    echo "$(date -u +"%Y-%m-%dT%H:%M:%S");$address;$base_denom;$expected_connection;$local_denom;$lock_period;$pool_denom;$pool_id;$quote_denom;$return_source_channel;$transfer_channel;$weight" >> investment.txt
done