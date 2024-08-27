#!/bin/bash

# Variables
NODE="http://127.0.0.1:26659"

# Function to check if a value is true
check_true() {
  local key="$1"
  local value="$2"
  if [ "$value" == "true" ]; then
    echo "$key is true."
  else
    echo "$key is not true."
    exit 1
  fi
}

# Function to check if a value is greater than a threshold
check_greater() {
  local key="$1"
  local value="$2"
  local threshold="$3"
  if (( $(echo "$value > $threshold" | bc -l) )); then
    echo "$key is greater than $threshold."
  else
    echo "$key is not greater than $threshold."
    exit 1
  fi
}

# Function to check if a value is equal to an expected value
check_equal() {
  local key="$1"
  local value="$2"
  local expected="$3"
  if [ "$value" == "$expected" ]; then
    echo "$key is $expected."
  else
    echo "$key is not $expected. Got: $value"
    exit 1
  fi
}

# Check ibc-transfer params
IBC_TRANSFER_PARAMS=$(quasard q ibc-transfer params --output json --node $NODE)
SEND_ENABLED=$(echo $IBC_TRANSFER_PARAMS | jq -r '.send_enabled')
RECEIVE_ENABLED=$(echo $IBC_TRANSFER_PARAMS | jq -r '.receive_enabled')
check_true "send_enabled" "$SEND_ENABLED"
check_true "receive_enabled" "$RECEIVE_ENABLED"

# Check ibc-wasm checksums
IBC_WASM_CHECKSUMS=$(quasard q ibc-wasm checksums --output json --node $NODE)
CHECKSUMS_LENGTH=$(echo $IBC_WASM_CHECKSUMS | jq -r '.checksums | length')
check_equal "checksums length" "$CHECKSUMS_LENGTH" "0"

# Check interchain-accounts controller params
ICA_CONTROLLER_PARAMS=$(quasard q interchain-accounts controller params --output json --node $NODE)
CONTROLLER_ENABLED=$(echo $ICA_CONTROLLER_PARAMS | jq -r '.controller_enabled')
check_true "controller_enabled" "$CONTROLLER_ENABLED"

# Check interchain-accounts host params
ICA_HOST_PARAMS=$(quasard q interchain-accounts host params --output json --node $NODE)
HOST_ENABLED=$(echo $ICA_HOST_PARAMS | jq -r '.host_enabled')
ALLOW_MESSAGES_LENGTH=$(echo $ICA_HOST_PARAMS | jq -r '.allow_messages | length')
check_true "host_enabled" "$HOST_ENABLED"
check_equal "allow_messages length" "$ALLOW_MESSAGES_LENGTH" "1"

# Check mint params
MINT_PARAMS=$(quasard q mint params --output json --node $NODE)
MINT_DENOM=$(echo $MINT_PARAMS | jq -r '.params.mint_denom')
INFLATION_RATE_CHANGE=$(echo $MINT_PARAMS | jq -r '.params.inflation_rate_change')
check_equal "mint_denom" "$MINT_DENOM" "uqsr"
check_greater "inflation_rate_change" "$INFLATION_RATE_CHANGE" "0"

# Check params subspace slashing SignedBlocksWindow
#SLASHING_PARAMS=$(quasard q params subspace slashing SignedBlocksWindow --output json --node $NODE)
#SIGNED_BLOCKS_WINDOW=$(echo $SLASHING_PARAMS | jq -r '.value' | tr -d '"')
#check_greater "SignedBlocksWindow" "$SIGNED_BLOCKS_WINDOW" "0"

# Check slashing params
SLASHING_PARAMS=$(quasard q slashing params --output json --node $NODE)
SIGNED_BLOCKS_WINDOW=$(echo $SLASHING_PARAMS | jq -r '.params.signed_blocks_window')
check_greater "signed_blocks_window" "$SIGNED_BLOCKS_WINDOW" "99"

# Check staking params
STAKING_PARAMS=$(quasard q staking params --output json --node $NODE)
UNBONDING_TIME=$(echo $STAKING_PARAMS | jq -r '.params.unbonding_time' | sed 's/s//')
MAX_VALIDATORS=$(echo $STAKING_PARAMS | jq -r '.params.max_validators')
check_equal "unbonding_time" "$UNBONDING_TIME" "504h0m0"
check_equal "max_validators" "$MAX_VALIDATORS" "100"

# Check tendermint-validator-set
VALIDATOR_SET=$(quasard q tendermint-validator-set --output json --node $NODE)
VALIDATORS_COUNT=$(echo $VALIDATOR_SET | jq -r '.validators | length')
check_greater "validators count" "$VALIDATORS_COUNT" "0"

# Check tokenfactory params
TOKENFACTORY_PARAMS=$(quasard q tokenfactory params --output json --node $NODE)
DENOM_CREATION_FEE_AMOUNT=$(echo $TOKENFACTORY_PARAMS | jq -r '.params.denom_creation_fee[] | select(.denom=="uqsr") | .amount')
check_greater "denom_creation_fee_amount" "$DENOM_CREATION_FEE_AMOUNT" "0"

# Check upgrade module_versions
UPGRADE_MODULE_VERSIONS=$(quasard q upgrade module_versions --output json --node $NODE)
MODULE_VERSIONS_COUNT=$(echo $UPGRADE_MODULE_VERSIONS | jq -r '.module_versions | length')
check_greater "module_versions count" "$MODULE_VERSIONS_COUNT" "0"

# Check wasm params
WASM_PARAMS=$(quasard q wasm params --output json --node $NODE)
CODE_UPLOAD_ACCESS=$(echo $WASM_PARAMS | jq -r '.code_upload_access.permission')
check_equal "code_upload_access permission" "$CODE_UPLOAD_ACCESS" "Everybody"

# Check account info
ACCOUNT_INFO=$(quasard q auth account quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec --output json --node $NODE)
ACCOUNT_NUMBER=$(echo $ACCOUNT_INFO | jq -r '.account.value.account_number')
check_greater "account_number" "$ACCOUNT_NUMBER" "0"

# Check auth params
AUTH_PARAMS=$(quasard q auth params --output json --node $NODE)
MAX_MEMO_CHARACTERS=$(echo $AUTH_PARAMS | jq -r '.params.max_memo_characters')
TX_SIG_LIMIT=$(echo $AUTH_PARAMS | jq -r '.params.tx_sig_limit')
check_greater "max_memo_characters" "$MAX_MEMO_CHARACTERS" "0"
check_greater "tx_sig_limit" "$TX_SIG_LIMIT" "0"

# Check bank total
BANK_TOTAL=$(quasard q bank total --output json --node $NODE)
UQSR_AMOUNT=$(echo $BANK_TOTAL | jq -r '.supply[] | select(.denom=="uqsr") | .amount')
check_greater "uqsr amount" "$UQSR_AMOUNT" "100000000"

# Check distribution params
DISTRIBUTION_PARAMS=$(quasard q distribution params --output json --node $NODE)
COMMUNITY_TAX=$(echo $DISTRIBUTION_PARAMS | jq -r '.params.community_tax')
check_greater "community_tax" "$COMMUNITY_TAX" "0"

# Check epochs info
EPOCHS_INFO=$(quasard q epochs epoch-infos --output json --node $NODE)
CURRENT_EPOCHS=$(echo $EPOCHS_INFO | jq -r '.epochs[].current_epoch')
ALL_VALID=true
for EPOCH in $CURRENT_EPOCHS; do
  if [ "$EPOCH" -lt 1 ]; then
    ALL_VALID=false
    break
  fi
done
if [ "$ALL_VALID" = true ]; then
  echo "All current epochs are greater than or equal to 1."
else
  echo "One or more current epochs are less than 1."
  exit 1
fi

# Check evidence
EVIDENCE=$(quasard q evidence list --height 21 --output json --node $NODE)
EVIDENCE_COUNT=$(echo $EVIDENCE | jq -r '.pagination | length')
check_equal "evidence count" "$EVIDENCE_COUNT" "0"

# Check gov params
GOV_PARAMS=$(quasard q gov params --output json --node $NODE)
MIN_DEPOSIT_AMOUNT=$(echo $GOV_PARAMS | jq -r '.params.min_deposit[] | select(.denom=="uqsr") | .amount')
MAX_DEPOSIT_PERIOD=$(echo $GOV_PARAMS | jq -r '.params.max_deposit_period')
check_equal "min_deposit_amount" "$MIN_DEPOSIT_AMOUNT" "1"
check_equal "max_deposit_period" "$MAX_DEPOSIT_PERIOD" "48h0m0s"

echo "qwuery testing finished"



# Variables
NODE="http://127.0.0.1:26659"
CHAIN_ID="quasar"
KEYRING_BACKEND="test"
HOME_DIR="$HOME/.quasarnode"
SENDER="bob"
RECEIVER="my_treasury"
SENDER_ADDRESS="quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu"
RECEIVER_ADDRESS="quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
VALIDATOR="quasarvaloper1sqlsc5024sszglyh7pswk5hfpc5xtl77v7z632"
DENOM="uqsr"
GAS_PRICES="0.1uqsr"
CONTRACT_WASM_PATH="./artifacts/airdrop.wasm"
CONTRACT_LABEL="Test Contract"
CONTRACT_INIT_MSG='{"config": {"airdrop_amount": "11000000000","airdrop_asset": {"native": "uqsr"},"airdrop_title":"Test Title","airdrop_description": "Test description","end_height": 0,"start_height": 0,"total_claimed": "0"}}'
SLEEP_DURATION=6

# Function to print and execute commands
execute() {
  echo "Executing: $*"
  eval $*
}

OLD_CONTRACT_ADDRESS="quasar14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sy9numu"
echo "Interact with old Contract"
CONTRACT_EXECUTE_MSG='{"admin": {"add_users": {"users": [{"address": "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec", "amount": "2500000000"}]}}}'
execute "quasard tx wasm execute $OLD_CONTRACT_ADDRESS '$CONTRACT_EXECUTE_MSG' --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --gas-adjustment 1.5 --gas auto"
sleep $SLEEP_DURATION

# Query Contract State
echo "Query old Contract State"
CONTRACT_QUERY_MSG='{"airdrop_config_query": {}}'
execute "quasard query wasm contract-state smart $OLD_CONTRACT_ADDRESS '$CONTRACT_QUERY_MSG' --node $NODE --output json"

# Bank Send
echo "1. Bank Send"
execute "quasard query bank balances $RECEIVER_ADDRESS --node $NODE"
execute "quasard tx bank send $SENDER_ADDRESS $RECEIVER_ADDRESS 10000000$DENOM --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --gas auto --gas-adjustment 1.5"
sleep $SLEEP_DURATION
execute "quasard query bank balances $RECEIVER_ADDRESS --node $NODE"

# Staking
echo "2. Staking"
execute "quasard tx staking delegate $VALIDATOR 1000000$DENOM --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --gas auto --gas-adjustment 1.5"
sleep $SLEEP_DURATION
execute "quasard query staking delegations $SENDER_ADDRESS --node $NODE"

# Claim Rewards
echo "3. Claim Rewards"
execute "quasard tx distribution withdraw-rewards $VALIDATOR --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --gas --gas-adjustment 1.5"
sleep $SLEEP_DURATION
execute "quasard query distribution rewards-by-validator $SENDER_ADDRESS $VALIDATOR --node $NODE"

# Upload Contract
echo "4. Upload Contract"
execute "quasard tx wasm store $CONTRACT_WASM_PATH --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --gas auto --gas-adjustment 1.5"
sleep $SLEEP_DURATION

# Get the code ID
CODE_ID=$(quasard query wasm list-code --node $NODE --output json | jq -r '.code_infos[-1].code_id')
echo "Contract Code ID: $CODE_ID"

# Instantiate Contract
echo "5. Instantiate Contract"
execute "quasard tx wasm instantiate $CODE_ID '$CONTRACT_INIT_MSG' --from $SENDER --label '$CONTRACT_LABEL' --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --admin $SENDER_ADDRESS --gas auto --gas-adjustment 1.5"
sleep $SLEEP_DURATION

# Get the contract address
CONTRACT_ADDRESS=$(quasard query wasm list-contract-by-code $CODE_ID --node $NODE --output json | jq -r '.contracts[-1]')
echo "Contract Address: $CONTRACT_ADDRESS"

# Interact with Contract
echo "6. Interact with Contract"
CONTRACT_EXECUTE_MSG='{"admin": {"add_users": {"users": [{"address": "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec", "amount": "2500000000"}]}}}'
execute "quasard tx wasm execute $CONTRACT_ADDRESS '$CONTRACT_EXECUTE_MSG' --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --gas auto --gas-adjustment 1.5"
sleep $SLEEP_DURATION

# Query Contract State
echo "7. Query Contract State"
CONTRACT_QUERY_MSG='{"airdrop_config_query": {}}'
execute "quasard query wasm contract-state smart $CONTRACT_ADDRESS '$CONTRACT_QUERY_MSG' --node $NODE --output json"

echo "8. Submit Governance Proposal"
PROPOSAL_TITLE="Test Proposal"
PROPOSAL_DESCRIPTION="This is a test proposal."
PROPOSAL_TYPE="Text"
PROPOSAL_DEPOSIT="1$DENOM"
execute "quasard tx gov submit-legacy-proposal --title '$PROPOSAL_TITLE' --description '$PROPOSAL_DESCRIPTION' --type '$PROPOSAL_TYPE' --deposit $PROPOSAL_DEPOSIT --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --gas auto --gas-adjustment 1.5"
sleep $SLEEP_DURATION

# Get the proposal ID
PROPOSAL_ID=$(quasard query gov proposals --output json --node $NODE | jq -r '.proposals[-1].id')
echo "Proposal ID: $PROPOSAL_ID"

# Deposit to the Proposal
echo "9. Deposit to the Proposal"
execute "quasard tx gov deposit $PROPOSAL_ID 5000000$DENOM --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --gas auto --gas-adjustment 1.5"
sleep $SLEEP_DURATION

# Vote on the Proposal
echo "10. Vote on the Proposal"
execute "quasard tx gov vote $PROPOSAL_ID yes --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y --gas auto --gas-adjustment 1.5"
sleep $SLEEP_DURATION

# Query Proposal Status
echo "11. Query Proposal Status"
execute "quasard query gov proposal $PROPOSAL_ID --node $NODE --output json"

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
LOOP_COUNT=15
SLEEP_DURATION=10

SENDER_ADDRESS="quasar1828z63g9wp3qwyn4p64adc3ungsv56ux5aacmu"
RECEIVER_ADDRESS="quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec"
VALIDATOR="quasarvaloper1sqlsc5024sszglyh7pswk5hfpc5xtl77v7z632"

echo "12. IBC testing"
# Execute and check balances
for ((i=1; i<=LOOP_COUNT; i++))
do
  echo "Iteration $i"

  # Osmosis to Quasar transfer
  echo "Transferring from Osmosis to Quasar"
  INITIAL_BALANCE=$(quasard query bank balances $OSMOSIS_RECEIVER --output json | jq -r --arg denom "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518" '.balances[] | select(.denom==$denom) | .amount')
  osmosisd tx ibc-transfer transfer transfer $OSMOSIS_CHANNEL $OSMOSIS_RECEIVER ${OSMOSIS_AMOUNT}${OSMOSIS_DENOM} --from $OSMOSIS_SENDER --keyring-backend test --home $OSMOSIS_HOME --node $OSMOSIS_NODE --chain-id $OSMOSIS_CHAIN_ID -y --gas-prices 1$OSMOSIS_DENOM
  sleep $SLEEP_DURATION
  rly transact flush
  sleep $SLEEP_DURATION
  FINAL_BALANCE=$(quasard query bank balances $OSMOSIS_RECEIVER --output json | jq -r --arg denom "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518" '.balances[] | select(.denom==$denom) | .amount')
  check_greater "Balance" "$FINAL_BALANCE" "$INITIAL_BALANCE"


  # Quasar to Osmosis transfer
  echo "Transferring from Quasar to Osmosis"
  INITIAL_BALANCE=$(osmosisd query bank balances $QUASAR_RECEIVER --node $OSMOSIS_NODE --output json | jq -r --arg denom "ibc/6BEEEE6CC17BA0EE37857A1E2AF6EC53C50DB6B6F89463A3933D0859C4CF6087" '.balances[] | select(.denom==$denom) | .amount')
  quasard tx ibc-transfer transfer transfer $QUASAR_CHANNEL $QUASAR_RECEIVER ${QUASAR_AMOUNT}${QUASAR_DENOM} --from $QUASAR_SENDER --keyring-backend test --home $QUASAR_HOME --chain-id $QUASAR_CHAIN_ID -y --gas-prices 1$QUASAR_DENOM --node $QUASAR_NODE
  sleep $SLEEP_DURATION
  rly transact flush
  sleep $SLEEP_DURATION
  FINAL_BALANCE=$(osmosisd query bank balances $QUASAR_RECEIVER --node $OSMOSIS_NODE --output json | jq -r --arg denom "ibc/6BEEEE6CC17BA0EE37857A1E2AF6EC53C50DB6B6F89463A3933D0859C4CF6087" '.balances[] | select(.denom==$denom) | .amount')
  check_greater "Balance" "$FINAL_BALANCE" "$INITIAL_BALANCE"

  # other module actions
  INITIAL_BALANCE=$(quasard query bank balances $RECEIVER_ADDRESS --output json | jq -r --arg denom "$QUASAR_DENOM" '.balances[] | select(.denom==$denom) | .amount')
  quasard tx bank send $SENDER_ADDRESS $RECEIVER_ADDRESS 10000000$DENOM --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y
  sleep $SLEEP_DURATION
  FINAL_BALANCE=$(quasard query bank balances $RECEIVER_ADDRESS --output json | jq -r --arg denom "$QUASAR_DENOM" '.balances[] | select(.denom==$denom) | .amount')
  check_greater "Balance" "$FINAL_BALANCE" "$INITIAL_BALANCE"

  REWARDS=$(quasard query distribution rewards-by-validator $SENDER_ADDRESS $VALIDATOR --output json)
  INITIAL_REWARDS=$(echo $REWARDS | jq -r '.rewards[] | sub("uqsr$"; "") | tonumber' )
  quasard tx distribution withdraw-rewards $VALIDATOR --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y
  sleep $SLEEP_DURATION
  REWARDS=$(quasard query distribution rewards-by-validator $SENDER_ADDRESS $VALIDATOR --output json)
  FINAL_REWARDS=$(echo $REWARDS | jq -r '.rewards[] | sub("uqsr$"; "") | tonumber' )
  check_greater "Rewards" "$INITIAL_REWARDS" "$FINAL_REWARDS"

  INITIAL_STAKING=$(quasard query staking delegations $SENDER_ADDRESS --output json)
  INITIAL_STAKING_AMOUNT=$(echo $INITIAL_STAKING | jq -r '.delegation_responses[0].balance.amount')
  quasard tx staking delegate $VALIDATOR 1000000$DENOM --from $SENDER --chain-id $CHAIN_ID --gas-prices $GAS_PRICES --keyring-backend $KEYRING_BACKEND --home $HOME_DIR -y
  sleep $SLEEP_DURATION
  POST_STAKING=$(quasard query staking delegations $SENDER_ADDRESS --output json)
  POST_STAKING_AMOUNT=$(echo $POST_STAKING | jq -r '.delegation_responses[0].balance.amount')
  check_greater "Staked Amount" "$POST_STAKING_AMOUNT" "$INITIAL_STAKING_AMOUNT"

done

echo "Script completed."
