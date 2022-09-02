ADDR="quasar14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sy9numu"
OSMOICA=$(quasarnoded query intergamm interchain-account-from-address connection-1 --output json $ADDR | jq .interchain_account_address)
echo "found osmosis address for ICA $OSMOICA" 

osmosisd tx gamm create-pool --pool-file ./sample_pool.json --node tcp://localhost:26679 --from bob  --chain-id osmosis --gas 583610
POOLID=$(osmosisd query gamm pools --output json --node tcp://localhost:26679 | jq '.pools[0].id')
POOLADDR=$(osmosisd query gamm pools --output json --node tcp://localhost:26679 | jq '.pools[0].address')
echo "got poolID $POOLID with address $POOLADDR" 

JOINPOOLMSG='{"join_single_pool":{"connection_id":"connection-1","pool_id":"1", "share_out_min_amount":1,"token_in":{"denom":"uosmo", "amount":"12345"}}}'
echo "sending join pool from quasar"
quasarnoded tx wasm execute $ADDR '{"join_single_pool":{"connection_id":"connection-1","pool_id":"1", "share_out_min_amount":1,"token_in":{"denom":"uosmo", "amount":"12345"}}}' --from alice --gas auto --node http://0.0.0.0:26659 --chain-id quasar

