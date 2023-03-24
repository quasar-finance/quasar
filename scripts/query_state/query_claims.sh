#!/bin/bash

source values.sh
VAULT_ADDR="$VAULT_ADDR" 
RPC="$RPC"

OUTPUT=$(quasarnoded q wasm contract-state smart $VAULT_ADDR '{"get_tvl_info":{}}' --node https://quasar-rpc.polkachu.com:443 -o json | jq)

if [ ! -f claims.txt ] || ! grep -q "timestamp;base_denom;ica_address;lp_denom;d_unlocked_shares;locked_shares;w_unlocked_shares;quote_denom" claims.txt; then
    echo "timestamp;base_denom;ica_address;lp_denom;d_unlocked_shares;locked_shares;w_unlocked_shares;quote_denom" > claims.txt
fi

for primitive in $(echo "${output}" | jq -c '.data.primitives[]'); do
    BASE_DENOM=$(echo "${primitive}" | jq -r '.base_denom')
    ICA_ADDRESS=$(echo "${primitive}" | jq -r '.ica_address')
    LP_DENOM=$(echo "${primitive}" | jq -r '.lp_denom')
    D_UNLOCKED_SHARES=$(echo "${primitive}" | jq -r '.lp_shares.d_unlocked_shares')
    LOCKED_SHARES=$(echo "${primitive}" | jq -r '.lp_shares.locked_shares')
    W_UNLOCKED_SHARESs=$(echo "${primitive}" | jq -r '.lp_shares.w_unlocked_shares')
    QUOTE_DENOM=$(echo "${primitive}" | jq -r '.quote_denom')
    echo "$(date -u +"%Y-%m-%dT%H:%M:%S");$BASE_DENOM;$ICA_ADDRESS;$LP_DENOM;$D_UNLOCKED_SHARES;$LOCKED_SHARES;$W_UNLOCKED_SHARESs;$QUOTE_DENOM" >> claims.txt
done