#!/bin/bash

# Variables
NODE="http://127.0.0.1:26659"
CHAIN_ID="quasar"
KEYRING_BACKEND="test"
HOME_DIR="$HOME/.quasarnode"
SENDER="bob"
SENDER_ADDRESS="quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu"
GAS_PRICES="0.025uqsr"
CONTRACT_WASM_PATH="../artifacts/tag2.3.0/airdrop.wasm"
CONTRACT_LABEL="Test Contract"
CONTRACT_INIT_MSG='{"config": {"airdrop_amount": "11000000000","airdrop_asset": {"native": "uqsr"},"airdrop_title":"Test Title","airdrop_description": "Test description","end_height": 0,"start_height": 0,"total_claimed": "0"}}'
SLEEP_DURATION=6

# Function to print and execute commands
execute() {
  echo "Executing: $*"
  eval $*
}

# Upload Contract
echo "1. Upload Contract"
execute "quasardv1 tx wasm store $CONTRACT_WASM_PATH --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --gas 2000000"
sleep $SLEEP_DURATION

# Get the code ID
CODE_ID=$(quasardv1 query wasm list-code --node $NODE --output json | jq -r '.code_infos[-1].code_id')
echo "Contract Code ID: $CODE_ID"

# Instantiate Contract
echo "2. Instantiate Contract"
execute "quasardv1 tx wasm instantiate $CODE_ID '$CONTRACT_INIT_MSG' --from $SENDER --label '$CONTRACT_LABEL' --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --admin $SENDER_ADDRESS"
sleep $SLEEP_DURATION

# Get the contract address
CONTRACT_ADDRESS=$(quasardv1 query wasm list-contract-by-code $CODE_ID --node $NODE --output json | jq -r '.contracts[-1]')
echo "Contract Address: $CONTRACT_ADDRESS"

# IBC testing post upgrade
OSMOSIS_NODE="http://127.0.0.1:26679"
OSMOSIS_CHANNEL="channel-0"
OSMOSIS_SENDER="bob"
OSMOSIS_RECEIVER="quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
OSMOSIS_DENOM="uosmo"
OSMOSIS_AMOUNT="10000"
OSMOSIS_CHAIN_ID="osmosis"
OSMOSIS_HOME="$HOME/.osmosis"

QUASAR_NODE="http://127.0.0.1:26659"
QUASAR_CHANNEL="channel-0"
QUASAR_SENDER="my_treasury"
QUASAR_RECEIVER="osmo1ez43ye5qn3q2zwh8uvswppvducwnkq6wjqc87d"
QUASAR_DENOM="uqsr"
QUASAR_AMOUNT="10000"
QUASAR_CHAIN_ID="quasar"
QUASAR_HOME="$HOME/.quasarnode"
LOOP_COUNT=1
SLEEP_DURATION=6

echo "12. IBC testing"
# Execute and check balances
for ((i=1; i<=LOOP_COUNT; i++))
do
  echo "Iteration $i"

  # Osmosis to Quasar transfer
  echo "Transferring from Osmosis to Quasar"
  osmosisd tx ibc-transfer transfer transfer $OSMOSIS_CHANNEL $OSMOSIS_RECEIVER ${OSMOSIS_AMOUNT}${OSMOSIS_DENOM} --from $OSMOSIS_SENDER --keyring-backend test --home $OSMOSIS_HOME --node $OSMOSIS_NODE --chain-id $OSMOSIS_CHAIN_ID -y --gas-prices 1$OSMOSIS_DENOM
  sleep $SLEEP_DURATION
  rly transact flush
  sleep $SLEEP_DURATION
  quasardv1 query bank balances $OSMOSIS_RECEIVER

  # Quasar to Osmosis transfer
  echo "Transferring from Quasar to Osmosis"
  quasardv1 tx ibc-transfer transfer transfer $QUASAR_CHANNEL $QUASAR_RECEIVER ${QUASAR_AMOUNT}${QUASAR_DENOM} --from $QUASAR_SENDER --keyring-backend test --home $QUASAR_HOME --chain-id $QUASAR_CHAIN_ID -y --gas-prices 1$QUASAR_DENOM --node $QUASAR_NODE
  sleep $SLEEP_DURATION
  rly transact flush
  sleep $SLEEP_DURATION
  osmosisd query bank balances $QUASAR_RECEIVER --node $OSMOSIS_NODE
done

echo "Script completed."