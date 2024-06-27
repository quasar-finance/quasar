#!/bin/sh

OUT=$(quasarnoded keys list --keyring-backend test --output json)
addresses=$(echo $OUT | jq -r '.[].address')

BINARY=quasarnoded
HOME_QSR=$HOME/.quasarnode
CHAIN_ID=quasar
ALICE_ACCOUNT=quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec
BOB_ACCOUNT=quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu
VR_ADDR=quasar1hulx7cgvpfcvg83wk5h96sedqgn72n026w6nl47uht554xhvj9nscd7f47

echo "adding funds to vault rewards contract"
quasarnoded tx bank send $ALICE_ACCOUNT $VR_ADDR 1000000000uqsr -y --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID -b block

echo "query bob account balance for visual verification before auto claim"
quasarnoded q bank balances $BOB_ACCOUNT
for addr in $addresses; do
    quasarnoded query wasm contract-state smart $VAULT_ADDR '{"pending_rewards":"'$addr'"}' --output json
done

sleep 6
echo "querying accounts for auto claiming accounts"
quasarnoded query wasm contract-state smart $VR_ADDR '{"all_users":{}}' --output json
OUT=$(quasarnoded query wasm contract-state smart $VR_ADDR '{"all_users":{"start_after": 0, "limit": 100}}' --output json)
ACCOUNTS=$(echo $OUT | jq -r '.data.users_and_rewards')
echo "auto claiming queried accounts"
quasarnoded tx wasm execute $VR_ADDR '{"admin":{"auto_claim":{"user_addresses":'$ACCOUNTS'}}}' -y --from alice --keyring-backend test --gas auto --fees 10000uqsr --chain-id $CHAIN_ID -b block
sleep 6

echo "query bob account balance for visual verification after auto claim"
quasarnoded q bank balances $BOB_ACCOUNT
for addr in $addresses; do
    quasarnoded query wasm contract-state smart $VAULT_ADDR '{"pending_rewards":"'$addr'"}' --output json
done