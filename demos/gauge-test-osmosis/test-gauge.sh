#!/bin/sh

osmosisd tx gamm create-pool --pool-file ./sample_pool1.json --pool-type balancer --node http://127.0.0.1:26679 --from bob --keyring-backend test --chain-id osmosis -y --fees 10000uosmo --gas 2000000 -b block
osmosisd tx gamm join-pool --max-amounts-in 10000000000000fakestake --max-amounts-in 10000000000000stake --pool-id 1 --share-amount-out 10000000000000000000 --node http://127.0.0.1:26679 --from user1 --keyring-backend test --chain-id osmosis -y --fees 10000uosmo --gas 2000000 -b block
osmosisd tx gamm join-pool --max-amounts-in 10000000000000fakestake --max-amounts-in 10000000000000stake --pool-id 1 --share-amount-out 10000000000000000000 --node http://127.0.0.1:26679 --from user2 --keyring-backend test --chain-id osmosis -y --fees 10000uosmo --gas 2000000 -b block
osmosisd tx lockup lock-tokens 10000000000000000000gamm/pool/1 --duration 60s --node http://127.0.0.1:26679 --from user1 --keyring-backend test --chain-id osmosis -y --fees 10000uosmo --gas 2000000 -b block
osmosisd tx lockup lock-tokens 9999999999999999990gamm/pool/1 --duration 120s --node http://127.0.0.1:26679 --from user2 --keyring-backend test --chain-id osmosis -y --fees 10000uosmo --gas 2000000 -b block
osmosisd tx lockup lock-tokens 10000000000000000000gamm/pool/1 --duration 180s --node http://127.0.0.1:26679 --from bob --keyring-backend test --chain-id osmosis -y --fees 10000uosmo --gas 2000000 -b block
osmosisd tx incentives create-gauge "gamm/pool/1" 100000000000000usdc 0 --duration 180s --start-time "$(date +%s)" --epochs 2 --from bob --keyring-backend test --chain-id osmosis -y --node http://127.0.0.1:26679 --fees 10000uosmo -b block

echo "after gauge balance of the gauge creator - check for usdc amount to be zero"
osmosisd q bank balances osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d

sleep 30

echo "after the first unlock the user that created the gauge has a balance of 50_000_000_000usdc, other users that bonded for lesser time never receives usdc"

BOB_USDC_BALANCE=$(osmosisd q bank balances osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d --output json | jq -r '.balances[2]')
echo "bob balance in usdc: $BOB_USDC_BALANCE"

USER1_USDC_BALANCE=$(osmosisd q bank balances osmo1zaavvzxez0elundtn32qnk9lkm8kmcsz2tlhe7 --output json | jq -r '.balances[3]')
echo "user 1 balance in usdc: $USER1_USDC_BALANCE"

USER2_USDC_BALANCE=$(osmosisd q bank balances osmo185fflsvwrz0cx46w6qada7mdy92m6kx4qm4l9k --output json | jq -r '.balances[3]')
echo "user 2 balance in usdc: $USER2_USDC_BALANCE"

sleep 60

echo "after the first unlock the user that created the gauge has a balance of 100_000_000_000usdc, other users that bonded for lesser time never receives usdc"

BOB_USDC_BALANCE=$(osmosisd q bank balances osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d --output json | jq -r '.balances[2]')
echo "user 2 balance in usdc: $BOB_USDC_BALANCE"

USER1_USDC_BALANCE=$(osmosisd q bank balances osmo1zaavvzxez0elundtn32qnk9lkm8kmcsz2tlhe7 --output json | jq -r '.balances[3]')
echo "user 2 balance in usdc: $USER1_USDC_BALANCE"

USER2_USDC_BALANCE=$(osmosisd q bank balances osmo185fflsvwrz0cx46w6qada7mdy92m6kx4qm4l9k --output json | jq -r '.balances[3]')
echo "user 2 balance in usdc: $USER2_USDC_BALANCE"
