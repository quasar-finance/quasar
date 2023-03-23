#!/bin/sh

# trap ctrl-c and ctrl-d
cleanup() {
    kill $COSMOS_PID
    kill $OSMO_PID
    kill $QUASAR_PID
    kill $HERMES_PID
    kill $RLY_PID_1
    kill $RLY_PID_2
    kill $RLY_PID_3

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
osmosisd tx gamm create-pool --pool-file ./sample_pool1.json --node http://127.0.0.1:26679 --from bob --keyring-backend test --home $HOME/.osmosis --chain-id osmosis -y --gas-prices 1uosmo
sleep 6
osmosisd tx gamm create-pool --pool-file ./sample_pool2.json --node http://127.0.0.1:26679 --from bob --keyring-backend test --home $HOME/.osmosis --chain-id osmosis -y --gas-prices 1uosmo
sleep 6
osmosisd tx gamm create-pool --pool-file ./sample_pool3.json --node http://127.0.0.1:26679 --from bob --keyring-backend test --home $HOME/.osmosis --chain-id osmosis -y --gas-prices 1uosmo

# run hermes and save pid, run_hermes and setup_go_relayer might not relay over the same channel out of the box due to connection creation in both scripts
# ./run_hermes.sh  &

# Currently we're not using Hermes due to an issue with relaying new channels https://github.com/informalsystems/ibc-rs/issues/2608

# setup and run hermes
./run_hermes_v1.sh

# echo "starting hermes"
hermes start >>./logs/hermes_start.log 2>&1 &
HERMES_PID=$!

echo "setting up go relayer"
./setup_go_relayer.sh

echo "starting go relaying"
# run an instance of go relayer for each path, thus 3 in total
# rly start quasar_cosmos --debug-addr "localhost:7597" --time-threshold 300s -p events >>./logs/quasar_cosmos_rly.log 2>&1 &
# RLY_PID_1=$!

rly start quasar_osmosis --debug-addr "localhost:7598" -p events --time-threshold 300s >>./logs/quasar_osmosis.log 2>&1 &
RLY_PID_2=$!

# rly start cosmos_osmosis --debug-addr "localhost:7599" -p events >>./logs/cosmos_osmosis.log 2>&1 &
# RLY_PID_3=$!

echo "ibc transferring uosmo"
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 1000001uosmo --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 1000002stake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 1000003fakestake --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu 1000001uosmo --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo
sleep 6
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1zaavvzxez0elundtn32qnk9lkm8kmcszvnk6zf 1000001uosmo --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo

sleep 10

quasarnoded query bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec

echo "setup ready for use"
afplay /System/Library/Sounds/Funk.aiff
say -r 200 "setup ready"

wait
