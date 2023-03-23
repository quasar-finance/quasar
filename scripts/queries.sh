#!/bin/sh

VAULT_ADDR=quasar1xzqhz0q969plap7awdjpls6vvrq57efk5vlkwr7kj5rzw9sq8j6s6wnxaj
PRIM_ADDR="quasar1ck0qyewrpeqaqyyz3pp0qjsuseglukvqeuec37vgy4nah2hf0gfqyrt5vw"
USER_ADDR="quasar10d6fe5kzrmxzrjkhqr5wccqsqscnqy7a9uqp9x"

USER_BALANCE=$(~/go/bin/quasarnoded q wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"'$USER_ADDR'"}}' --node https://rpc-tst5.qsr.network:443 | grep 'balance:' | cut -d '"' -f 2)

PRIM_ADDR_1=$(~/go/bin/quasarnoded q wasm contract-state smart $VAULT_ADDR '{"investment":{}}' --node https://rpc-tst5.qsr.network:443 | grep 'address' | grep -o 'quasar[^\"]*' | sed -n '1p')
PRIM_ADDR_2=$(~/go/bin/quasarnoded q wasm contract-state smart $VAULT_ADDR '{"investment":{}}' --node https://rpc-tst5.qsr.network:443 | grep 'address' | grep -o 'quasar[^\"]*' | sed -n '2p')

ICA_BALANCE_1=$(~/go/bin/quasarnoded q wasm contract-state smart $PRIM_ADDR_1 '{"ica_balance": {}}' --node https://rpc-tst5.qsr.network:443 | grep 'amount: '| cut -d '"' -f 2)
ICA_BALANCE_2=$(~/go/bin/quasarnoded q wasm contract-state smart $PRIM_ADDR_2 '{"ica_balance": {}}' --node https://rpc-tst5.qsr.network:443 | grep 'amount: '| cut -d '"' -f 2)

TOTAL_SUPPLY=$(~/go/bin/quasarnoded q wasm contract-state smart $VAULT_ADDR '{"token_info":{}}' --node https://rpc-tst5.qsr.network:443 | grep 'total_supply: '| cut -d '"' -f 2)

LOCK=$(~/go/bin/quasarnoded q wasm contract-state smart $PRIM_ADDR_1 '{"lock":{}}' --node https://rpc-tst5.qsr.network:443)
VALUES=($(echo "$LOCK" | sed 's/data: lock: //'))

BOND="${VALUES[3]}"
RECOVERY="${VALUES[5]}"
START_UNBOND="${VALUES[7]}"
UNBOND="${VALUES[9]}"

LOCK=$(~/go/bin/quasarnoded q wasm contract-state smart $PRIM_ADDR_1 '{"lock":{}}' --node https://rpc-tst5.qsr.network:443)
VALUES=($(echo "$LOCK" | sed 's/data: lock: //'))

BOND="${VALUES[3]}"
RECOVERY="${VALUES[5]}"
START_UNBOND="${VALUES[7]}"
UNBOND="${VALUES[9]}"

echo "USER_ADDR     = $USER_ADDR"
echo "USER_BALANCE  = $USER_BALANCE\n"

echo "PRIM_ADDR_1   = $PRIM_ADDR_1"
echo "ICA_BALANCE_1 = $ICA_BALANCE_1\n"
echo "PRIM_ADDR_2   = $PRIM_ADDR_2"
echo "ICA_BALANCE_2 = $ICA_BALANCE_2\n"

echo "TOTAL_SUPPLY  = $TOTAL_SUPPLY\n"

echo "BOND          = $BOND"
echo "RECOVERY      = $RECOVERY"
echo "START_UNBOND  = $START_UNBOND"
echo "UNBOND        = $UNBOND"



