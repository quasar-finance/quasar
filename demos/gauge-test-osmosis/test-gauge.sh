#!/bin/sh

osmosisd tx gamm create-pool --pool-file ./sample_pool1.json --pool-type balancer --node http://127.0.0.1:26679 --from bob --keyring-backend test --chain-id osmosis -y --fees 10000uosmo --gas 2000000 -b block
osmosisd tx lockup lock-tokens 100000000000000000000gamm/pool/1 --duration 180s --node http://127.0.0.1:26679 --from bob --keyring-backend test --chain-id osmosis -y --fees 10000uosmo --gas 2000000 -b block
osmosisd tx incentives create-gauge "gamm/pool/1" 1000000000000uosmo 0 --duration 60s --start-time "$(date +%s)" --epochs 2 --from bob --keyring-backend test --chain-id osmosis -y --node http://127.0.0.1:26679 --fees 10000uosmo -b block

echo "display balances after every 60s and notice the balance increase for the user"
osmosisd q bank balances osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d
sleep 60
osmosisd q bank balances osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d
sleep 60
osmosisd q bank balances osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d
sleep 60
osmosisd q bank balances osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d