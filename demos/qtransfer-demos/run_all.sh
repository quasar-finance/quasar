#!/bin/bash

## Run Quasar
./quasar_localnet.sh &
## Run Osmosis
./osmo_localnet.sh &

echo "Waiting 15 sec"
sleep 15

echo "Starting run hermes"
## Run Relayer
#./run_hermes.sh
