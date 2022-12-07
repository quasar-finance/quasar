OSMOICA=$(quasarnoded query intergamm interchain-account-from-address connection-2 --output json $ADDR | jq .interchain_account_address)
echo "found osmosis address for ICA $OSMOICA" 

osmosisd tx gamm create-pool --pool-file ./sample_pool.json --node tcp://localhost:26679 --from bob  --chain-id osmosis --gas 583610
POOLID=$(osmosisd query gamm pools --output json --node tcp://localhost:26679 | jq .pools[0].id)
POOLADDR=$(osmosisd query gamm pools --output json --node tcp://localhost:26679 | jq .pools[0].address)
echo "got poolID $POOLID with address $POOLADDR" 


