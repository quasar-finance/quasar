#!/bin/sh
    
# trap ctrl-c and ctrl-d
cleanup()
{
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
./cosmos_localnet.sh  &
COSMOS_PID=$!

# run quasar and save pid
./quasar_localnet.sh  &
QUASAR_PID=$!

#run osmo and save pid
./osmo_localnet.sh  &
OSMO_PID=$!

# wait for chains to start
sleep 10

# create a pool on osmosis to test against
osmosisd tx gamm create-pool --pool-file ./sample_pool.json --node http://127.0.0.1:26679 --from bob --keyring-backend test --home $HOME/.osmosis --chain-id osmosis -y

# run hermes and save pid, run_hermes and setup_go_relayer might not relay over the same channel out of the box due to connection creation in both scripts
# ./run_hermes.sh  &

# Currently we're not using Hermes due to an issue with relaying new channels https://github.com/informalsystems/ibc-rs/issues/2608
# starting hermes
# echo "starting hermes"
# hermes start >> ./logs/hermes_start.log 2>&1
# HERMES_PID=$!

echo "setting up go relayer"
./setup_go_relayer.sh

echo "starting go relaying"
# # run an instance of go relayer for each path, thus 3 in total
rly start quasar_cosmos --debug-addr "localhost:7597" --time-threshold 30s -p events >> ./logs/quasar_cosmos_rly.log 2>&1  & 
RLY_PID_1=$!

rly start quasar_osmosis --debug-addr "localhost:7598" -p events --time-threshold 30s >> ./logs/quasar_osmosis.log 2>&1 &
RLY_PID_2=$!

rly start cosmos_osmosis --debug-addr "localhost:7599" -p events >> ./logs/cosmos_osmosis.log 2>&1  &
RLY_PID_3=$!

wait