#!/bin/sh

# trap ctrl-c and ctrl-d
function cleanup()
{
    kill $COSMOS_PID
    kill $OSMO_PID
    kill $QUASAR_PID
    # kill $HERMES_PID
}

trap cleanup EXIT

# reset logs dir
rm -rf ./logs
mkdir ./logs

# run cosmos and save pid
./cosmos_localnet.sh  &
COSMOS_PID=$!

#run osmo and save pid
./osmo_localnet.sh  &
OSMO_PID=$!

# run quasar and save pid
./quasar_localnet.sh  &
QUASAR_PID=$!

# # wait for chains to start
# sleep 10

# # run hermes and save pid
# ./run_hermes.sh  &
# HERMES_PID=$!

wait

