#!/bin/bash

# TODO: Before running this script go to the main branch, execute a "git checkout v0.1.0" and "make install" the binary in order to start from the mainnet version.
# TODO: Now checkout the upgrade branch and you are able to execute this test script.

# Kill existing quasard processes
echo ">>> Killing existing quasard processes..."
pkill quasard || true

# Entry point to run quasar_localnet.sh
../quasar_localnet.sh

# Define variables
CHAIN_ID=quasar
UPGRADE_HEIGHT=30

echo ">>> Sleeping 10 seconds to create some initial blocks"
sleep 10

# Submit governance proposal for software-upgrade to v0.1.1
echo ">>> Submitting proposal for software-upgrade"
quasard tx gov submit-proposal software-upgrade "v0.1.1" --title "Software Upgrade to v0.1.1" --description "This software-upgrade v0.1.1 introduces QVesting module for continuous vesting schedule accounts creation" --upgrade-height $UPGRADE_HEIGHT --deposit 1uqsr --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 60 seconds after submitting proposal"
sleep 60

echo ">>> Voting yes to proposal"
quasard tx gov vote 1 yes --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5 seconds after voting proposal"
sleep 5

# Wait for the block height to reach 100, cosmovisor should handle the upgrade
echo ">>> Waiting for the block height to reach $UPGRADE_HEIGHT"
while true; do
  CURRENT_HEIGHT=$(quasard status | jq -r '.SyncInfo.latest_block_height')
  echo "Current height: "$CURRENT_HEIGHT
  if [ "$CURRENT_HEIGHT" -ge "$UPGRADE_HEIGHT" ]; then
    break
  fi
  sleep 1
done

# Check if the upgrade has been successful
quasard query gov proposal 1 --chain-id $CHAIN_ID --output json

# TODO: Now that the governance proposal has been success the chain should have been halted. You can check the quasar.log file expecting to find: UPGRADE "v0.1.1" NEEDED at height: 30: CONSENSUS FAILURE!!!
# TODO: You can now "make install" the new version and check that blocks are produced as expected by running "quasard start". It should start producing blocks from height 31.
