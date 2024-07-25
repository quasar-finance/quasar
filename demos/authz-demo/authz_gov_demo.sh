#!/bin/sh 
set -o xtrace
BINARY=quasard
HOME_QSR=$HOME/.quasarnode
CHAIN_ID=quasar
NODE="--node tcp://localhost:26659"


ALICE=$($BINARY keys show alice --keyring-backend test -a --home $HOME_QSR)
BOB=$($BINARY keys show bob --keyring-backend test -a --home $HOME_QSR)
TX_FLAG="--chain-id=quasar --gas-prices 0.1uqsr --gas auto --gas-adjustment 1.3 -b block --node tcp://localhost:26659 --keyring-backend test  --home $HOME_QSR --output json -y"
QUERY_FLAG="--node tcp://localhost:26659 --output json"

# alice to create a text based gov proposal on quasar chain, that alice wants to get voted by bob on behalf of alice.

echo "Alice creating gov proposal on chain."
$BINARY tx gov submit-proposal --title="Test Authorization" --description="Is Bob authorized to vote?" --type="Text" --deposit="10000000uqsr" --from $ALICE  $TX_FLAG

echo "Alice grant Vote auth to bob."
$BINARY tx authz grant $BOB generic --msg-type /cosmos.gov.v1beta1.MsgVote --from $ALICE $TX_FLAG

echo "Query auth grant status." 
$BINARY query authz grants $ALICE $BOB /cosmos.gov.v1beta1.MsgVote $QUERY_FLAG 

echo "Alice generate unsinged tx., the expected sequence number is 3 for the below tx." # NEED TO CHECK WHY?
# If you are running multiple tests, and runs. the expected sequence number might got changed accordingly you need to use the expected sequence number.
$BINARY tx gov vote 1 yes --from $ALICE --generate-only $TX_FLAG --sequence 3 > tx.json

echo "bob is signing tx on behalf on alice"
$BINARY tx authz exec tx.json --from bob $TX_FLAG

echo "query if the vote was done by alice"
$BINARY query gov vote 1 $ALICE $QUERY_FLAG


sleep 3

echo "Revoke the auth"
$BINARY tx authz revoke $BOB /cosmos.gov.v1beta1.MsgVote --from alice $TX_FLAG

echo "query if the auth revoke was successful. it should show below error. \nError: rpc error: code = NotFound desc = rpc error: code = NotFound desc = no authorization found for /cosmos.gov.v1beta1.MsgVote type: key not found"
$BINARY query authz grants $ALICE $BOB /cosmos.gov.v1beta1.MsgVote $QUERY_FLAG


