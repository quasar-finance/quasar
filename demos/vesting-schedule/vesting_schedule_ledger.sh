#!/bin/bash

# Kill existing quasard processes
echo ">>> Killing existing quasard processes..."
pkill quasard || true

# Entry point to run quasar_localnet.sh
./quasar_localnet.sh

# Define variables
CHAIN_ID=quasar
unamestr=$(uname)
if [ "$unamestr" = 'Linux' ]; then
  START_TIME_OK=$(date -d "+10 seconds" +%s)
  END_TIME_OK=$(date -d "+60 seconds" +%s)
elif [ "$unamestr" = 'Darwin' ]; then
  START_TIME_OK=$(date -v+10S +%s)
  END_TIME_OK=$(date -v+60S +%s)
fi

# Use the my_treasury address created in quasar_localnet.sh
MY_TREASURY=$(quasard keys show my_treasury -a --keyring-backend test)

echo ">>> Add keys vester_continuous_ok and vester_continuous_ko for further vesting account creation"
# Create vester accounts in the local node for testing
quasard keys add vester_continuous_ok --keyring-backend test
VC_OK_ADDRESS=$(quasard keys show vester_continuous_ok -a --keyring-backend test)

echo ">>> Create multi-sig account using signer1, signer2, and signer3"
quasard keys add multisig_account --multisig-threshold 2 --multisig "signer1,signer2,signer3" --keyring-backend test
SIGNER1_ADDRESS=$(quasard keys show signer1 -a --ledger)
SIGNER2_ADDRESS=$(quasard keys show signer2 -a --ledger)
SIGNER3_ADDRESS=$(quasard keys show signer3 -a --ledger)
MULTISIG_ADDRESS=$(quasard keys show multisig_account -a --keyring-backend test)
echo "Signer 1: $SIGNER1_ADDRESS"
echo "Signer 2: $SIGNER2_ADDRESS"
echo "Signer 3: $SIGNER3_ADDRESS"
echo "Multisig: $MULTISIG_ADDRESS"

echo ">>> Fund multisig and signer accounts"
quasard tx bank send $MY_TREASURY $MULTISIG_ADDRESS 1000uqsr --from $MY_TREASURY --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5/35 seconds to elapse 5/35 before querying multisig account"
sleep 5

echo ">>> Create create-vesting-account transaction using multisig as the signer"
quasard tx qvesting create-vesting-account $VC_OK_ADDRESS 1000uqsr $START_TIME_OK $END_TIME_OK --from $MULTISIG_ADDRESS --chain-id $CHAIN_ID --keyring-backend test --generate-only > tx.json

echo ">>> Sign the transaction with each signer"
quasard tx sign tx.json --from signer1 --multisig $MULTISIG_ADDRESS --chain-id $CHAIN_ID --ledger --output-document tx_signed1.json
quasard tx sign tx.json --from signer2 --multisig $MULTISIG_ADDRESS --chain-id $CHAIN_ID --ledger --output-document tx_signed2.json
quasard tx sign tx.json --from signer3 --multisig $MULTISIG_ADDRESS --chain-id $CHAIN_ID --ledger --output-document tx_signed3.json

echo ">>> Assemble the signatures and broadcast the transaction"
quasard tx multisign tx.json multisig_account tx_signed1.json tx_signed2.json tx_signed3.json --chain-id $CHAIN_ID --keyring-backend test > tx_multisig.json
quasard tx broadcast tx_multisig.json --chain-id $CHAIN_ID -y

echo ">>> Sleeping 30/35 seconds to elapse 30/35 before sending 500uqsr from Vester to MyTreasury"
sleep 30

# Transfer vesting tokens from vester1 to vester2 before the time
echo ">>> Testing OK accounts to be able to send vesting schedule tokens with amount 500uqsr that is half of vesting total"
quasard tx bank send $VC_OK_ADDRESS $MY_TREASURY 500uqsr --from vester_continuous_ok --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5 seconds after sending bank tx from OK case"
sleep 5

echo ">>> Testing again OK account to NOT be able to send vesting schedule tokens with amount 250uqsr that is a quarter of vesting total and should be locked already"
quasard tx bank send $VC_OK_ADDRESS $MY_TREASURY 250uqsr --from vester_continuous_ok --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5 more seconds..."
sleep 5

echo ">>> Expecting OK balances equals to 500uqsr"
quasard query bank balances $VC_OK_ADDRESS

# Remove all the generated .json filed related to transaction signatures
rm -rf tx.json tx_signed1.json tx_signed2.json tx_signed3.json tx_multisig.json
