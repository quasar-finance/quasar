#!/bin/bash

# Kill existing quasarnoded processes
echo ">>> Killing existing quasarnoded processes..."
pkill quasarnoded || true

# Entry point to run quasar_localnet.sh
./quasar_localnet.sh

# Define variables
CHAIN_ID=quasar
UPGRADE_HEIGHT=100

#TODO run this script as main branch, pre build the new binary from the new branch and set it up manually on the cosmovisor folder

# Submit governance proposal for software-upgrade to v0.1.1
echo ">>> Submitting proposal for software-upgrade"
PROPOSAL_ID=$(quasarnoded tx gov submit-proposal software-upgrade "v0.1.1" --title "Software Upgrade to v0.1.1" --description "This software-upgrade v0.1.1 introduces QVesting module for continuous vesting schedule accounts creation" --upgrade-height $UPGRADE_HEIGHT --deposit 1uqsr --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y | jq -r '.logs[0].events[0].attributes[0].value')

echo ">>> Sleeping 5 seconds after submitting proposal"
sleep 5

echo ">>> Voting yes to proposal"
quasarnoded tx gov vote $PROPOSAL_ID yes --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5 seconds after voting proposal"
sleep 5

# Wait for the block height to reach 100, cosmovisor should handle the upgrade
echo ">>> Waiting for the block height to reach $UPGRADE_HEIGHT"
while true; do
  CURRENT_HEIGHT=$(quasarnoded status | jq -r '.SyncInfo.latest_block_height')
  if [ "$CURRENT_HEIGHT" -ge "$UPGRADE_HEIGHT" ]; then
    break
  fi
  sleep 5
done

# Check if the upgrade has been successful
UPGRADE_SUCCESS=$(quasarnoded query gov proposal $PROPOSAL_ID --chain-id $CHAIN_ID --output json | jq -r '.proposal.status')

if [ "$UPGRADE_SUCCESS" == "Passed" ]; then
  echo ">>> Software upgrade was successful"
else
  echo ">>> Software upgrade failed"
fi

echo ">>> Asking for the new binary version, it should be changed"
quasarnoded version