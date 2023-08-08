#!/bin/sh

# trap ctrl-c and ctrl-d
cleanup() {
  kill $OSMO_PID
  kill $QUASAR_PID
  kill $RLY_PID_1
}

pkill quasarnoded
pkill osmosisd
pkill rly

sleep 3

trap cleanup 1 2 3 6

# reset logs dir
rm -rf ./logs
mkdir ./logs

# run quasar and save pid
./quasar_localnet.sh &
QUASAR_PID=$!

#run osmo and save pid
./osmo_localnet.sh &
OSMO_PID=$!

# wait for chains to start
sleep 10

echo "setting up go relayer"
./go_relayer_setup.sh

echo "starting go relaying"
# run an instance of go relayer for each path, thus 3 in total
rly start quasar_osmosis --debug-addr "localhost:7598" -p events --time-threshold 300s >>./logs/quasar_osmosis.log 2>&1 &
RLY_PID_1=$!

quasarnoded status
osmosisd status

# run pre upgrade actions like pools creation, contract deployments and bonding actions
. ./pre_upgrade.sh

# run upgrade part that performs a chain upgrade
. ./upgrade.sh

# run post upgrade actions like new bonds, unbond and claim.
. ./post_upgrade.sh

# Check platform
platform='unknown'
unamestr=$(uname)
if [ "$unamestr" = 'Linux' ]; then
  platform='linux'
elif [ "$unamestr" = 'Darwin' ]; then
  platform='macos'
fi

if [ $platform = 'macos' ]; then
  say "test finished"
fi

## wait is added so that the all the processes are not killed.
## please perform all the other queries after the announcement of test finished in order to check all the actions are working properly
wait
