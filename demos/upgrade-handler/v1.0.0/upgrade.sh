#!/bin/sh

## run chain upgrade procedure
echo "RUNNING CHAIN UPGRADE PROCEDURE"
BINARY=quasard-go-18
CHAIN_ID="quasar"
ACCOUNT_NAME="my_treasury"
RPC="http://127.0.0.1:26659"
CURRENT_BLOCK=$($BINARY status | jq -r '.SyncInfo.latest_block_height')
CHAIN_ID=quasar
NUM1=20
UPGRADE_HEIGHT=$(expr $NUM1 + $CURRENT_BLOCK)
echo $UPGRADE_HEIGHT

# sleep for a few seconds after the previous file transaction
sleep 6

# Submit governance proposal for software-upgrade to v1
echo ">>> Submitting proposal for software-upgrade"
$BINARY tx gov submit-proposal software-upgrade "v1" --title "Software Upgrade to v1" --description "This software-upgrade v1 introduces qvesting, token factory and authz module" --upgrade-height $UPGRADE_HEIGHT --deposit 100uqsr --from $ACCOUNT_NAME --keyring-backend test -y --output json --chain-id $CHAIN_ID --fees 10000uqsr --gas 7000000 --node $RPC

sleep 20

echo ">>> Voting yes to proposal"
$BINARY tx gov vote 1 yes --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5 seconds after voting proposal"
sleep 5

# Wait for the block height to reach the upgrade height
echo ">>> Waiting for the block height to reach $UPGRADE_HEIGHT"
while true; do
  CURRENT_HEIGHT=$($BINARY status | jq -r '.SyncInfo.latest_block_height')
  echo "Current height: ""$CURRENT_HEIGHT"
  if [ "$CURRENT_HEIGHT" -ge "$UPGRADE_HEIGHT" ]; then
    break
  fi
  sleep 5
done

echo "Check if the upgrade proposal works."
$BINARY query gov proposal 1 --chain-id $CHAIN_ID --output json

sleep 30

echo "killing the old quasar instance to start the new one"
pkill quasard
rm ./logs/quasar.log

# run chain upgrade
BINARY=quasard
HOME_QSR=$HOME/.quasarnode

echo "starting with new binary"
$BINARY start --home $HOME_QSR >>./logs/quasar.log 2>&1 &
