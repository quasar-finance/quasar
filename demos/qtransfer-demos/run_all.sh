#!/bin/bash

## Run Quasar
./quasar_localnet.sh &
## Run Osmosis
./osmo_localnet.sh &

sleep 10
## Run Relayer
./run_hermes.sh