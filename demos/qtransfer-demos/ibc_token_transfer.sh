#!/bin/bash

#osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 100uosmo --from alice --chain-id osmosis --home ~/.osmosis  --node tcp://localhost:26679 --keyring-backend test

echo "ibc token tx with some random memo - "
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 100uosmo --memo "\"{ \"wasm\": { \"contract\": \"osmo1contractAddr\", \"msg\": { \"execute_IBC_receive\": \"raw_message_data\"}}}" --from alice --chain-id osmosis --home ~/.osmosis  --node tcp://localhost:26679 --keyring-backend test
