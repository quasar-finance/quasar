OSMOICA=$(quasarnoded query intergamm interchain-account-from-address connection-2 --output json $ADDR | jq .interchain_account_address)
echo "found osmosis address for ICA $OSMOICA" 

osmosisd tx gamm create-pool --pool-file ./sample_pool.json --node tcp://localhost:26679 --from bob  --chain-id osmosis --gas 583610
POOLID=$(osmosisd query gamm pools --output json --node tcp://localhost:26679 | jq .pools[0].id)
POOLADDR=$(osmosisd query gamm pools --output json --node tcp://localhost:26679 | jq .pools[0].address)
echo "got poolID $POOLID with address $POOLADDR" 

JOINPOOLMSG='{"join_single_pool":{"connection_id":"connection-2","pool_id":"1", "share_out_min_amount":1,"token_in":{"denom":"uosmo", "amount":"12345"}}}'
echo "sending join pool from quasar"
quasarnoded tx wasm execute quasar17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgsstgms7 '{"join_single_pool":{"connection_id":"connection-2","pool_id":"1", "share_out_min_amount":1,"token_in":{"denom":"uosmo", "amount":"12345"}}}' --from alice --gas auto --node http://0.0.0.0:26659 --chain-id quasar

