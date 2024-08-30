#!/bin/bash

# TODO: Before running this script go to the main branch, execute a "git checkout v2.0.1" and "make install" the binary in order to start from the mainnet version.
# TODO: Now checkout the upgrade branch and you are able to execute this test script.

# Kill existing quasarnodedv1 processes
echo ">>> Killing existing quasarnoded processes..."
pkill quasarnoded || true
pkill quasard || true

echo ">>> Killing existing osmosisd processes..."
pkill osmosisd || true

echo ">>> Killing existing rly processes..."
pkill rly || true

rm -rf ./logs
mkdir ./logs

# Entry point to run quasar_localnet.sh and osmosis_localnet.sh
./quasar_localnet.sh
sleep 5
./osmosis_localnet.sh
sleep 5

# starting a relayer between the two
./setup_go_relayer.sh
sleep 20

# pre upgrade test
./pre_upgrade.sh

# Define variables
CHAIN_ID=quasar
UPGRADE_HEIGHT=80

echo ">>> Sleeping 10 seconds to create some initial blocks"
sleep 5

# Submit governance proposal for software-upgrade to v2
echo ">>> Submitting proposal for software-upgrade"
quasarnoded tx gov submit-proposal draft_proposal.json --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y --fees 100000uqsr --gas 1000000

echo ">>> Sleeping 10 seconds after submitting proposal"
sleep 6

echo ">>> Voting yes to proposal"
quasarnoded tx gov vote 1 yes --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y  --fees 100000uqsr --gas 1000000

# Wait for the block height to reach 50, cosmovisor should handle the upgrade
echo ">>> Waiting for the block height to reach $UPGRADE_HEIGHT"
while true; do
  CURRENT_HEIGHT=$(quasarnoded status | jq -r '.SyncInfo.latest_block_height')
  echo "Current height: "$CURRENT_HEIGHT
  if [ "$CURRENT_HEIGHT" -ge "$UPGRADE_HEIGHT" ]; then
    break
  fi
  sleep 5
done

# Check if the upgrade has been successful
quasarnoded query gov proposal 1 --chain-id $CHAIN_ID --output json

sleep 10

# Kill existing quasarnoded processes for new version to start
echo ">>> Killing existing quasarnoded processes..."
pkill quasarnoded || true

quasard start --home $HOME/.quasarnode  > ./logs/quasar_post_upgrade.log 2>&1 &

sleep 10
# run ibc send manually and monitor
./post_upgrade.sh