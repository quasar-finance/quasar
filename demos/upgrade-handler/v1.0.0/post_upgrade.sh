#!/bin/sh

## env variables
BINARY=quasarnoded
CHAIN_ID="quasar"
ACCOUNT_NAME="my_treasury"
RPC="http://127.0.0.1:26659"

# let the chain start after upgrade and produce some blocks
sleep 15

echo "perform a claim on the account an unbond before chain upgrade"
$BINARY tx wasm execute $VAULT_ADDR '{"claim":{}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id quasar --fees 10000uqsr --gas 7000000 --node $RPC

sleep 60

echo "query bank balance"
$BINARY query bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --node $RPC

# perform bonding again to see if it is working
echo "perform another bond after chain upgrade"
$BINARY tx wasm execute $VAULT_ADDR '{"bond":{}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 --amount 10000000ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518 --node $RPC

sleep 120

echo "query the balance after the bond"
$BINARY query wasm contract-state smart $VAULT_ADDR '{"balance":{"address":"quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"}}' --node $RPC


echo "perform an unbond action on new bond"
$BINARY tx wasm execute $VAULT_ADDR '{"unbond":{"amount":"9999999"}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id quasar --fees 10000uqsr --gas 7000000 --node $RPC

sleep 60

echo "perform a claim"
$BINARY tx wasm execute $VAULT_ADDR '{"claim":{}}' --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id quasar --fees 10000uqsr --gas 7000000 --node $RPC

sleep 60

echo "query bank balance"
$BINARY query bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --node $RPC