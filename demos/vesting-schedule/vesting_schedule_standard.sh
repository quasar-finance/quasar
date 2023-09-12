#!/bin/bash
set -o xtrace
# Kill existing quasarnoded processes
echo ">>> Killing existing quasarnoded processes..."
pkill quasarnoded || true

# Entry point to run quasar_localnet.sh
./quasar_localnet.sh

# Define variables
CHAIN_ID=quasar
unamestr=$(uname)
if [ "$unamestr" = 'Linux' ]; then
  START_TIME_OK=$(date -d "+5 seconds" +%s)
  END_TIME_OK=$(date -d "+60 seconds" +%s)
  START_TIME_KO=$(date -d "+900 seconds" +%s)
  END_TIME_KO=$(date -d "+1800 seconds" +%s)
elif [ "$unamestr" = 'Darwin' ]; then
  START_TIME_OK=$(date -v+5S +%s)
  END_TIME_OK=$(date -v+60S +%s)
  START_TIME_KO=$(date -v+900S +%s)
  END_TIME_KO=$(date -v+1800S +%s)
fi

# Use the my_treasury address created in quasar_localnet.sh
MY_TREASURY=$(quasarnoded keys show my_treasury -a --keyring-backend test)

echo ">>> Add keys vester_continuous_ok and vester_continuous_ko for further vesting account creation"
# Create vester accounts in the local node for testing
quasarnoded keys add vester_continuous_ok --keyring-backend test
quasarnoded keys add vester_continuous_ko --keyring-backend test
VC_OK_ADDRESS=$(quasarnoded keys show vester_continuous_ok -a --keyring-backend test)
VC_KO_ADDRESS=$(quasarnoded keys show vester_continuous_ko -a --keyring-backend test)

echo ">>> Creating OK accounts"
# Create vesting account executing as my_treasury for vester_continuous_ok
quasarnoded tx qvesting create-vesting-account $VC_OK_ADDRESS 1000uqsr $START_TIME_OK $END_TIME_OK --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5/35 seconds to elapse 5/35 before querying accounts to check vesting balances and start times"
sleep 5

echo ">>> Creating KO accounts"
# Create vesting account executing as my_treasury for vester_continuous_ok
quasarnoded tx qvesting create-vesting-account $VC_KO_ADDRESS 1000uqsr $START_TIME_KO $END_TIME_KO --from my_treasury --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5/35 seconds to elapse 10/35 before querying accounts to check vesting balances and start times"
sleep 5

# Check that the vesting account has been created successfully
echo ">>> Query accounts"
quasarnoded query auth account $VC_OK_ADDRESS
quasarnoded query auth account $VC_KO_ADDRESS

echo ">>> Sleeping 25/35 seconds to elapse 35/35 seconds"
sleep 25

# Transfer vesting tokens from vester1 to vester2 before the time
echo ">>> Testing OK accounts to be able to send vesting schedule tokens with amount 500uqsr that is half of vesting total"
quasarnoded tx bank send $VC_OK_ADDRESS $MY_TREASURY 500uqsr --from vester_continuous_ok --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5 seconds after sending bank tx from OK case"
#sleep 5

echo ">>> Testing again OK account to NOT be able to send vesting schedule tokens with amount 250uqsr that is a quarter of vesting total and should be locked already"
quasarnoded tx bank send $VC_OK_ADDRESS $MY_TREASURY 250uqsr --from vester_continuous_ok --chain-id $CHAIN_ID --keyring-backend test -y
echo ">>> Testing KO accounts to be NOT able to send vesting schedule tokens, even 1 should fail as vesting is in the future"
quasarnoded tx bank send $VC_KO_ADDRESS $MY_TREASURY 1uqsr --from vester_continuous_ko --chain-id $CHAIN_ID --keyring-backend test -y

echo ">>> Sleeping 5 seconds..."
sleep 5

echo ">>> Expecting OK balances equals to 500uqsr"
quasarnoded query bank balances $VC_OK_ADDRESS
echo ">>> Expecting balances equals to 1000uqsr"
quasarnoded query bank balances $VC_KO_ADDRESS

