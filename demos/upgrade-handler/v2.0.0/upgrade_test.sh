#!/bin/bash

# TODO: Before running this script go to the main branch, execute a "git checkout v0.1.0" and "make install" the binary in order to start from the mainnet version.
# TODO: Now checkout the upgrade branch and you are able to execute this test script.

# Kill existing quasarnodedv1 processes
echo ">>> Killing existing quasarnodedv1 processes..."
pkill quasarnodedv1 || true
pkill quasarnoded || true

echo ">>> Killing existing osmosisd processes..."
pkill osmosisd || true

echo ">>> Killing existing rly processes..."
pkill rly || true

rm -rf ./logs
mkdir ./logs

# Entry point to run quasar_localnet.sh a.ðƒnd osmosis_localnet.sh
./quasar_localnet.sh
sleep 5
./osmosis_localnet.sh
sleep 5

# starting a relayer between the two
./setup_go_relayer.sh
sleep 20

# IBC testing before upgrade
echo "ibc transferring uosmo"
osmosisd tx ibc-transfer transfer transfer channel-0 quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec 1000001uosmo --from bob --keyring-backend test --home $HOME/.osmosis --node http://127.0.0.1:26679 --chain-id osmosis -y --gas-prices 1uosmo
sleep 10
quasarnodedv1 query bank balances quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec


# Define variables
CHAIN_ID=quasar
UPGRADE_HEIGHT=50

echo ">>> Sleeping 10 seconds to create some initial blocks"
sleep 10

# Submit governance proposal for software-upgrade to v2
echo ">>> Submitting proposal for software-upgrade"
quasarnodedv1 tx gov submit-proposal software-upgrade "v2" --title "Software Upgrade to v2" --description "This software-upgrade v2." --upgrade-height $UPGRADE_HEIGHT --deposit 1uqsr --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 10 seconds after submitting proposal"
sleep 10

echo ">>> Voting yes to proposal"
quasarnodedv1 tx gov vote 1 yes --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5 seconds after voting proposal"
sleep 5


# Wait for the block height to reach 100, cosmovisor should handle the upgrade
echo ">>> Waiting for the block height to reach $UPGRADE_HEIGHT"
while true; do
  CURRENT_HEIGHT=$(quasarnodedv1 status | jq -r '.SyncInfo.latest_block_height')
  echo "Current height: "$CURRENT_HEIGHT
  if [ "$CURRENT_HEIGHT" -ge "$UPGRADE_HEIGHT" ]; then
    break
  fi
  sleep 5
done

# Check if the upgrade has been successful
quasarnodedv1 query gov proposal 1 --chain-id $CHAIN_ID --output json

sleep 10

# Kill existing quasarnodedv1 processes for new version to start
echo ">>> Killing existing quasarnodedv1 processes..."
pkill quasarnodedv1 || true

quasarnoded start --home $HOME/.quasarnode  > quasar.log 2>&1 &

# run ibc send manually and monitor
