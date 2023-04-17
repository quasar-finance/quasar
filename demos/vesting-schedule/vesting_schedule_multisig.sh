#!/bin/bash

# Kill existing quasarnoded processes
echo ">>> Killing existing quasarnoded processes..."
pkill quasarnoded || true

# Entry point to run quasar_localnet.sh
./quasar_localnet.sh

# Define variables
CHAIN_ID=quasar
START_TIME_OK=$(date +%s)
unamestr=$(uname)
if [ "$unamestr" = 'Linux' ]; then
  END_TIME_OK=$(date -d "+60 seconds" +%s)
elif [ "$unamestr" = 'Darwin' ]; then
  END_TIME_OK=$(date -v+60S +%s)
fi

# Use the my_treasury address created in quasar_localnet.sh
MY_TREASURY=$(quasarnoded keys show my_treasury -a --keyring-backend test)

echo ">>> Add keys vester_continuous_ok and vester_continuous_ko for further vesting account creation"
# Create vester accounts in the local node for testing
quasarnoded keys add vester_continuous_ok --keyring-backend test
VC_OK_ADDRESS=$(quasarnoded keys show vester_continuous_ok -a --keyring-backend test)

echo ">>> Create signer1, signer2, and signer3 keys"
quasarnoded keys add signer1 --keyring-backend test
quasarnoded keys add signer2 --keyring-backend test
quasarnoded keys add signer3 --keyring-backend test
echo ">>> Create multi-sig account using signer1, signer2, and signer3"
quasarnoded keys add multisig_account --multisig-threshold 2 --multisig "signer1,signer2,signer3" --keyring-backend test
SIGNER1_ADDRESS=$(quasarnoded keys show signer1 -a --keyring-backend test)
SIGNER2_ADDRESS=$(quasarnoded keys show signer2 -a --keyring-backend test)
SIGNER3_ADDRESS=$(quasarnoded keys show signer3 -a --keyring-backend test)
MULTISIG_ADDRESS=$(quasarnoded keys show multisig_account -a --keyring-backend test)
echo "Signer 1: $SIGNER1_ADDRESS"
echo "Signer 2: $SIGNER2_ADDRESS"
echo "Signer 3: $SIGNER3_ADDRESS"
echo "Multisig: $MULTISIG_ADDRESS"


echo ">>> Fund multisig and signer accounts"
quasarnoded tx bank send $MY_TREASURY $MULTISIG_ADDRESS 1000uqsr --from $MY_TREASURY --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5/30 seconds to elapse 5/30 before querying multisig account"
sleep 5

echo ">>> Create create-vesting-account transaction using multisig as the signer"
quasarnoded tx qvesting create-vesting-account $VC_OK_ADDRESS 1000uqsr $START_TIME_OK $END_TIME_OK --from $MULTISIG_ADDRESS --chain-id $CHAIN_ID --keyring-backend test --generate-only > tx.json

echo ">>> Sign the transaction with each signer"
quasarnoded tx sign tx.json --from signer1 --multisig $MULTISIG_ADDRESS --chain-id $CHAIN_ID --keyring-backend test --output-document tx_signed1.json
quasarnoded tx sign tx.json --from signer2 --multisig $MULTISIG_ADDRESS --chain-id $CHAIN_ID --keyring-backend test --output-document tx_signed2.json
quasarnoded tx sign tx.json --from signer3 --multisig $MULTISIG_ADDRESS --chain-id $CHAIN_ID --keyring-backend test --output-document tx_signed3.json

echo ">>> Assemble the signatures and broadcast the transaction"
quasarnoded tx multisign tx.json multisig_account tx_signed1.json tx_signed2.json tx_signed3.json --chain-id $CHAIN_ID --keyring-backend test > tx_multisig.json
quasarnoded tx broadcast tx_multisig.json --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 25/30 seconds to elapse 30/30 before sending 500uqsr from Vester to MyTreasury"
sleep 25

# Transfer vesting tokens from vester1 to vester2 before the time
echo ">>> Testing OK accounts to be able to send vesting schedule tokens with amount 500uqsr that is half of vesting total"
quasarnoded tx bank send $VC_OK_ADDRESS $MY_TREASURY 500uqsr --from vester_continuous_ok --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5 seconds after sending bank tx from OK case"
sleep 5

echo ">>> Testing again OK account to NOT be able to send vesting schedule tokens with amount 250uqsr that is a quarter of vesting total and should be locked already"
quasarnoded tx bank send $VC_OK_ADDRESS $MY_TREASURY 250uqsr --from vester_continuous_ok --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5 more seconds..."
sleep 5

echo ">>> Expecting OK balances equals to 500uqsr"
quasarnoded query bank balances $VC_OK_ADDRESS

# Remove all the generated .json filed related to transaction signatures
rm -rf tx.json tx_signed1.json tx_signed2.json tx_signed3.json tx_multisig.json
