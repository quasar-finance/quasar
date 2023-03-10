#!/bin/bash

./quasar_localnet.sh &

./osmo_localnet.sh &

sleep 15

./run_hermes_only_osmosis.sh