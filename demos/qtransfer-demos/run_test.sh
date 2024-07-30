#!/bin/bash

pkill quasard
pkill osmosisd

## Run Quasar
./quasar_localnet.sh &
## Run Osmosis
./osmo_localnet.sh &

sleep 10
## Run Relayer
./run_hermes.sh &

sleep 30

./balance_check.sh

sleep 5

./ibc_token_transfer.sh &

sleep 10

./balance_check.sh

sleep 5

./ibc_token_transfer-r.sh

sleep 10
./balance_check.sh


