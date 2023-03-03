#!/bin/bash

#osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 100uosmo --from alice --chain-id osmosis --home ~/.osmosis  --node tcp://localhost:26679 --keyring-backend test
#osmosisd keys show alice -a   --home ~/.osmosis  --keyring-backend test
#osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq

echo "ibc token tx with some random memo - "
quasarnoded tx ibc-transfer transfer transfer channel-0 osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq 100uqsr  --from alice --chain-id quasar --home ~/.quasarnode/  --node tcp://localhost:26659 --keyring-backend test
