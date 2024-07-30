#!/bin/bash
# Configure variables
BINARY=quasard
HOME_QSR=$HOME/.quasarnode
CHAIN_ID=quasar

send_tx_get_output() {
  $BINARY tx $@ --from alice --chain-id $CHAIN_ID --home "$HOME_QSR"  --node tcp://localhost:26659 --keyring-backend test -y -b block -o json
}

SUBMIT_PROPOSAL_OUTPUT=$(send_tx_get_output gov submit-proposal param-change quasar_denom_to_native_zone_id_map-proposal.json)
if [ "$(echo "$SUBMIT_PROPOSAL_OUTPUT" | jq '.code')" != 0 ]
then
  echo "error: $BINARY returned non-zero code"
  echo "raw log: $(echo "$SUBMIT_PROPOSAL_OUTPUT" | jq '.raw_log')"
fi

PROPOSAL_ID=$(echo "$SUBMIT_PROPOSAL_OUTPUT" | jq '.logs[0].events[]|select(.type=="submit_proposal").attributes[]|select(.key=="proposal_id").value' -r)
VOTE_OUTPUT=$(send_tx_get_output gov vote "$PROPOSAL_ID" yes)
if [ "$(echo "$VOTE_OUTPUT" | jq '.code')" != 0 ]
then
  echo "error: $BINARY returned non-zero code"
  echo "raw log: $(echo "$VOTE_OUTPUT" | jq '.raw_log')"
fi

SUBMIT_PROPOSAL_OUTPUT=$(send_tx_get_output gov submit-proposal param-change osmosis_denom_to_quasar_denom_map-proposal.json)
if [ "$(echo "$SUBMIT_PROPOSAL_OUTPUT" | jq '.code')" != 0 ]
then
  echo "error: $BINARY returned non-zero code"
  echo "raw log: $(echo "$SUBMIT_PROPOSAL_OUTPUT" | jq '.raw_log')"
fi

PROPOSAL_ID=$(echo "$SUBMIT_PROPOSAL_OUTPUT" | jq '.logs[0].events[]|select(.type=="submit_proposal").attributes[]|select(.key=="proposal_id").value' -r)
VOTE_OUTPUT=$(send_tx_get_output gov vote "$PROPOSAL_ID" yes)
if [ "$(echo "$VOTE_OUTPUT" | jq '.code')" != 0 ]
then
  echo "error: $BINARY returned non-zero code"
  echo "raw log: $(echo "$VOTE_OUTPUT" | jq '.raw_log')"
fi

SUBMIT_PROPOSAL_OUTPUT=$(send_tx_get_output gov submit-proposal param-change complete_zone_info_map-proposal.json --gas 1000000)
if [ "$(echo "$SUBMIT_PROPOSAL_OUTPUT" | jq '.code')" != 0 ]
then
  echo "error: $BINARY returned non-zero code"
  echo "raw log: $(echo "$SUBMIT_PROPOSAL_OUTPUT" | jq '.raw_log')"
fi

PROPOSAL_ID=$(echo "$SUBMIT_PROPOSAL_OUTPUT" | jq '.logs[0].events[]|select(.type=="submit_proposal").attributes[]|select(.key=="proposal_id").value' -r)
VOTE_OUTPUT=$(send_tx_get_output gov vote "$PROPOSAL_ID" yes)
if [ "$(echo "$VOTE_OUTPUT" | jq '.code')" != 0 ]
then
  echo "error: $BINARY returned non-zero code"
  echo "raw log: $(echo "$VOTE_OUTPUT" | jq '.raw_log')"
fi
