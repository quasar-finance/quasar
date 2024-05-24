#!/bin/bash

CODE_ID=1
osmosisd tx wasm instantiate $CODE_ID '{"thesis":"example thesis","name":"example vault","provault_config":{"max_deposit_cap":"1000000","deposit_denom":"uosmo","share_denom":"ushares","max_strategy_inst":"10","admin":"osmo1address"}}' --amount 300uosmo --label "test-01" --node tcp://localhost:26679 --from alice --keyring-backend test --home ~/.osmosis --chain-id osmosis --fees 300000uosmo --gas 7000000 --admin "osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq" --trace
