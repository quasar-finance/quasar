#!/bin/sh

OUT=$(quasarnoded keys list --keyring-backend test --output json)
addresses=$(echo $OUT | jq -r '.[].address')

BINARY=quasarnoded
HOME_QSR=$HOME/.quasarnode
CHAIN_ID=quasar

echo "balance before airdropping"
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node http://127.0.0.1:26679

osmosisd tx bank send osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq osmo1hslpzt2j3t3tqg2ezcxj2nm6rhpfzscjulk9g3mw4dzj7us7qucs9vx4rc 100000000fakestake --from alice --chain-id osmosis --node http://127.0.0.1:26679 --keyring-backend test --home $HOME/.osmosis --fees 100000uosmo

echo "balance after airdropping"
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node http://127.0.0.1:26679

quasarnoded tx wasm execute quasar14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sy9numu '{"transfer_airdrop":{"destination_address":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq", "amounts":[{"amount":"100000000", "denom":"fakestake"}]}}' --from alice --keyring-backend test --gas auto --fees 10000uqsr -b block  --chain-id quasar

echo "balance after transferring airdrop"
osmosisd q bank balances osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --node http://127.0.0.1:26679