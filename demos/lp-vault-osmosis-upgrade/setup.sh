#!/bin/sh

# trap ctrl-c and ctrl-d
cleanup() {
    kill $OSMO_PID
    kill $QUASAR_PID
    kill $HERMES_PID
    kill $RLY_PID_2
}

trap cleanup 1 2 3 6

# reset logs dir
rm -rf ./logs
mkdir ./logs

# run cosmos and save pid
# ./cosmos_localnet.sh &
# COSMOS_PID=$!

# run quasar and save pid
./quasar_localnet.sh &
QUASAR_PID=$!

#run osmo and save pid
./osmo_localnet.sh &
OSMO_PID=$!

# wait for chains to start
sleep 10

# create a pool on osmosis to test against
osmosisd tx gamm create-pool --pool-file ./sample_pool1.json --pool-type stableswap --node http://127.0.0.1:26679 --from bob --keyring-backend test --home $HOME/.osmosis --chain-id osmosis -y --gas 2000000 --gas-prices 1uosmo
sleep 6
osmosisd tx gamm create-pool --pool-file ./sample_pool2.json --node http://127.0.0.1:26679 --from bob --keyring-backend test --home $HOME/.osmosis --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx gamm create-pool --pool-file ./sample_pool3.json --node http://127.0.0.1:26679 --from bob --keyring-backend test --home $HOME/.osmosis --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6

echo "setting up go relayer"
./setup_go_relayer.sh

rly start quasar_osmosis --debug-addr "localhost:7598" -p events --time-threshold 300s >>./logs/quasar_osmosis.log 2>&1 &
RLY_PID_2=$!

echo "ibc transferring uosmo"
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 100000000001uosmo --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 100000000002stake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 100000000003fakestake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 100000000004usdc --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu 100000000001uosmo --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu 100000000002stake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu 100000000003fakestake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu 100000000004usdc --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf 100000000001uosmo --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf 100000000002stake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf 100000000003fakestake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf 100000000004usdc --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar185fflsvwrz0cx46w6qada7mdy92m6kx4xruj7p 100000000001uosmo --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar185fflsvwrz0cx46w6qada7mdy92m6kx4xruj7p 100000000002stake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar185fflsvwrz0cx46w6qada7mdy92m6kx4xruj7p 100000000003fakestake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar185fflsvwrz0cx46w6qada7mdy92m6kx4xruj7p 100000000004usdc --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo --gas 2000000

sleep 6

quasarnoded query bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec

# Check platform
platform='unknown'
unamestr=$(uname)
if [ "$unamestr" = 'Linux' ]; then
    platform='linux'
elif [ "$unamestr" = 'Darwin' ]; then
    platform='macos'
fi

if [ $platform = 'macos' ]; then
    say "setup ready"
fi

wait
