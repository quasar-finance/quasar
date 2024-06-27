#!/bin/sh

OUT=$(quasarnoded keys list --keyring-backend test --output json)
addresses=$(echo $OUT | jq -r '.[].address')

BINARY=quasarnoded
HOME_QSR=$HOME/.quasarnode
CHAIN_ID=quasar
VAULT_ADDR=quasar1unyuj8qnmygvzuex3dwmg9yzt9alhvyeat0uu0jedg2wj33efl5qtefy4k

list="["
i=0
for addr in $addresses; do
  if [ "$i" -ne 0 ] && [ "$i" -ne 2 ]; then
    BALANCE=$(quasarnoded query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"'$addr'"}}' --output json)
    BAL1=$(echo $BALANCE | jq -r '.data.balance')
    if [ "$BAL1" -ne 0 ]; then
      temp='"'$addr'"'
      list+=$temp","
    fi
    BAL=$(quasarnoded q bank balances $addr --output json)
    amount=$(echo "$BAL" | jq -r '.balances[0].amount')
    denom=$(echo "$BAL" | jq -r '.balances[0].denom')
    echo "Balance $addr $amount$denom $BAL1"
  fi
  ((i++))
  if [ "$i" -eq 12 ]; then
    break
  fi
done

modified_string=$(echo "$list" | sed 's/,$//')
modified_string+="]"
echo $modified_string

quasarnoded tx wasm execute $VAULT_ADDR '{"force_unbond":{"addresses":'$modified_string'}}' -y --from alice --keyring-backend test --gas 50000000 --fees 10000uqsr --chain-id $CHAIN_ID
sleep 10

rly transact flush
sleep 10
quasarnoded tx wasm execute $VAULT_ADDR '{"clear_cache":{}}' -y --from user1 --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
sleep 10

quasarnoded tx wasm execute $VAULT_ADDR '{"force_claim":{"addresses":'$modified_string'}}' -y --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
sleep 10

rly transact flush
sleep 10
quasarnoded tx wasm execute $VAULT_ADDR '{"clear_cache":{}}' -y --from user1 --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
sleep 90

for addr in $addresses; do
  BAL=$(quasarnoded q bank balances $addr --output json)
  amount=$(echo "$BAL" | jq -r '.balances[0].amount')
  denom=$(echo "$BAL" | jq -r '.balances[0].denom')
  echo "Balance $addr $amount$denom"
done
