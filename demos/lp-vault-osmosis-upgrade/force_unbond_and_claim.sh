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
sleep 60

quasarnoded tx wasm execute $VAULT_ADDR '{"force_claim":{"addresses":["quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu","quasar1kv76l2rr4pwl9equ95chvzr2f4393kr785gqn5","quasar1q2ax2t7d4jha704zwwkw0cse4n5r9aa097lcqh","quasar1egg2pgtt8f0mquwnw4jncarhjr2am9hn27lv5m","quasar1ww4u2vd68yyly57ll3t5hchfun3ta5388zjglg","quasar1cqmxrv2jla6q8vmvtar7uv5p9uxzwulsyyqx2z","quasar1sk0jcnlfft96n62r2ehqwt4dcvk4nkeryez5ed","quasar1qcjzqavsnvaz93eldq8e5uwjfws9hmmy4a2e0d","quasar1m5j2xjakdu8hv5zz0l3nf6rc8gr475r4xrc3sh","quasar1ragdn3ry57exlgy80c79xlqnhet9h4u23wys0p"]}}' -y --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
sleep 10

rly transact flush
sleep 10
quasarnoded tx wasm execute $VAULT_ADDR '{"clear_cache":{}}' -y --from user1 --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID
sleep 60

for addr in $addresses; do
  BAL=$(quasarnoded q bank balances $addr --output json)
  amount=$(echo "$BAL" | jq -r '.balances[0].amount')
  denom=$(echo "$BAL" | jq -r '.balances[0].denom')
  echo "Balance $addr $amount$denom"
done

