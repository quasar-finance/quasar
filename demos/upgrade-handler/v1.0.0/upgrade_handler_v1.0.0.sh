#!/bin/bash

# Pre-requisites
# Before running this script go to the main branch, execute a "git checkout v0.1.1" and "make install"
# the binary in order to start from the mainnet version.

version=`quasard version`
if [ "$version" != "0.1.1" ]; then
  echo "You are having incorrect version $version"
  echo "Please install the current mainnet version 0.1.1"
  exit 1
fi

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
current_block=$(quasard status | jq -r '.SyncInfo.latest_block_height')
echo "current block - $current_block"

if [ $((current_block)) -lt 5 ]; then
  echo "current block - $current_block is less than 5 block, sleep more"
  sleep 5 # sleep more
fi

# Submit governance proposal for software-upgrade to v0.1.1
echo ">>> Submitting proposal for software-upgrade"
quasard tx gov submit-proposal software-upgrade "v1" --title "Software Upgrade to v1" --description "This software-upgrade v1 introduces qvesting, token factory and authx module" --upgrade-height $UPGRADE_HEIGHT --deposit 100uqsr --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y

#sleep 5


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

echo "Check if the upgrade proposal works."
quasard query gov proposal 1 --chain-id $CHAIN_ID --output json

## Post chain halt status -
### Check the binary log, and see if "UPGRADE "v1.0.0" NEEDED at height: 30" is available.
### If , yes chain has been halted and not producing blocks.
### `quasard status | jq -r '.SyncInfo.latest_block_height'` command will be returning 30 , and
### will not update the heights.

### Compile the binary using below commands to get the expected version in place.
### go install -mod=readonly -tags "netgo ledger" -ldflags '-X github.com/cosmos/cosmos-sdk/version.Name=quasar -X github.com/cosmos/cosmos-sdk/version.AppName=quasard -X github.com/cosmos/cosmos-sdk/version.Version=1.0.0 -X github.com/cosmos/cosmos-sdk/version.Commit=00df969376c46d124bb35435aba71160c1def817 -X "github.com/cosmos/cosmos-sdk/version.BuildTags=netgo ledger," -w -s' -trimpath  ./cmd/quasard
