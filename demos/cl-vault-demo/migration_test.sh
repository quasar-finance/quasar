#!/bin/sh

osmosisd tx concentratedliquidity create-pool uatom uosmo 100 0.001 --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y
osmosisd tx concentratedliquidity create-position 1 "[-5000000]" 500000 1000000000000uatom,1000000000000uosmo 0 0 --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y

osmosisd tx wasm store ../../../quasar_copy/smart-contracts/artifacts/cl_vault-aarch64.wasm --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y
INIT='{"thesis":"hello world","name":"Distilled","admin":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq","range_admin":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq","pool_id":1,"config":{"performance_fee":"0.1","treasury":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq","swap_max_slippage":"0.01"},"vault_token_subdenom":"test-cl-vault-1","initial_lower_tick":-5000000,"initial_upper_tick":500000}'
osmosisd tx wasm instantiate 1 "$INIT" --label "my first contract" --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y --admin osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq --amount 1000uatom,1000uosmo

CONTRACT_ADDRESS=osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9

osmosisd tx wasm execute $CONTRACT_ADDRESS '{"exact_deposit":{}}' --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -y --amount 5000000uatom,5000000uosmo
osmosisd tx wasm execute $CONTRACT_ADDRESS '{"exact_deposit":{}}' --from bob --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -y --amount 5000000uatom,5000000uosmo
osmosisd tx wasm execute $CONTRACT_ADDRESS '{"exact_deposit":{}}' --from user1 --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -y --amount 5000000uatom,5000000uosmo
osmosisd tx wasm execute $CONTRACT_ADDRESS '{"exact_deposit":{}}' --from user2 --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -y --amount 5000000uatom,5000000uosmo

osmosisd tx poolmanager swap-exact-amount-in 20000000000uosmo 1 --swap-route-pool-ids 1 --swap-route-denoms uatom --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y
osmosisd tx poolmanager swap-exact-amount-in 20000000000uosmo 1 --swap-route-pool-ids 1 --swap-route-denoms uatom --from bob --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y
osmosisd tx poolmanager swap-exact-amount-in 20000000000uatom 1 --swap-route-pool-ids 1 --swap-route-denoms uosmo --from user1 --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y
osmosisd tx poolmanager swap-exact-amount-in 20000000000uatom 1 --swap-route-pool-ids 1 --swap-route-denoms uosmo --from user2 --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y


osmosisd q wasm contract-state smart $CONTRACT_ADDRESS '{"vault_extension":{"balances":{"user_shares_balance":{"user":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq"}}}}' --node http://127.0.0.1:26679
osmosisd q wasm contract-state smart $CONTRACT_ADDRESS '{"vault_extension":{"balances":{"user_shares_balance":{"user":"osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d"}}}}' --node http://127.0.0.1:26679
osmosisd q wasm contract-state smart $CONTRACT_ADDRESS '{"vault_extension":{"balances":{"user_shares_balance":{"user":"osmo1zaavvzxez0elundtn32qnk9lkm8kmcsz2tlhe7"}}}}' --node http://127.0.0.1:26679
osmosisd q wasm contract-state smart $CONTRACT_ADDRESS '{"vault_extension":{"balances":{"user_shares_balance":{"user":"osmo185fflsvwrz0cx46w6qada7mdy92m6kx4qm4l9k"}}}}' --node http://127.0.0.1:26679

osmosisd q wasm contract-state smart $CONTRACT_ADDRESS '{"vault_extension":{"balances":{"user_assets_balance":{"user":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq"}}}}' --node http://127.0.0.1:26679
osmosisd tx wasm execute $CONTRACT_ADDRESS '{"vault_extension":{"distribute_rewards":{}}}' --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -y
osmosisd q wasm contract-state smart $CONTRACT_ADDRESS '{"vault_extension":{"balances":{"user_assets_balance":{"user":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq"}}}}' --node http://127.0.0.1:26679

osmosisd q wasm contract-state smart $CONTRACT_ADDRESS '{"vault_extension":{"balances":{"user_rewards":{"user":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq"}}}}' --node http://127.0.0.1:26679
osmosisd q wasm contract-state smart $CONTRACT_ADDRESS '{"vault_extension":{"balances":{"user_rewards":{"user":"osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d"}}}}' --node http://127.0.0.1:26679
osmosisd q wasm contract-state smart $CONTRACT_ADDRESS '{"vault_extension":{"balances":{"user_rewards":{"user":"osmo1zaavvzxez0elundtn32qnk9lkm8kmcsz2tlhe7"}}}}' --node http://127.0.0.1:26679
osmosisd q wasm contract-state smart $CONTRACT_ADDRESS '{"vault_extension":{"balances":{"user_rewards":{"user":"osmo185fflsvwrz0cx46w6qada7mdy92m6kx4qm4l9k"}}}}' --node http://127.0.0.1:26679

osmosisd tx wasm execute $CONTRACT_ADDRESS '{"vault_extension":{"distribute_rewar":{}}}' --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -y

osmosisd tx wasm store ../../../cw-dex-router/artifacts/cw_dex_router.wasm --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y
osmosisd tx wasm instantiate 2 "{}" --label "my first dex contract" --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y --admin osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq
DEX_ROUTER=osmo1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqvlx82r

osmosisd tx wasm store ../../smart-contracts/artifacts/cl_vault-aarch64.wasm --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y
osmosisd tx wasm migrate osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 3 '{"dex_router":"osmo1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqvlx82r","auto_compound_admin":"osmo1t8eh66t2w5k67kwurmn5gqhtq6d2ja0vp7jmmq"}' --from alice --keyring-backend test --chain-id osmosis --node http://127.0.0.1:26679 --gas 8000000 --fees 100000uosmo --home $HOME/.osmosis -b sync -y
