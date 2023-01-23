#!/bin/sh

## This hermes script will establish connections among quasar, cosmos and osmosis local network.
## This script is using the hermes 1.x.x version.
## This script is not taking band chain into considerations.
 
v=`hermes --version`
STR="$v"
SUB='hermes 1'
echo $STR
if echo "$STR" | grep -q "$SUB"; then
	echo "hermes version is correct for this script."
	echo "continuing ..."
else 
	echo "$STR"
	echo "$SUB"
	echo "hermes version is not correct for this script. Please use hermes 1.x.x version for this script"
	echo "exiting ..."
	exit 1
fi 

../running_status.sh

is_processes_running=`echo $?`

echo "hello $is_processes_running"

if [ "$is_processes_running" = "0" ]
then
    echo "All processes are running."
    echo "Continuing..."
else
    echo "All processes are not running."
    echo "Exiting..."
    exit 1
fi


mkdir -p ~/.hermes/ 
pwd 
cp v1/hermes_config_with_bandchain.toml ~/.hermes/config.toml


hermes  keys add --chain quasar --mnemonic-file quasar.seeds
hermes  keys add --chain osmosis --mnemonic-file osmosis.seeds
hermes  keys add --chain cosmos --mnemonic-file cosmos.seeds

BANDCHAIN="band-laozi-testnet6"
hermes keys add --chain band-laozi-testnet6 --mnemonic-file band.seeds  --hd-path "m/44'/494'/0'/0/0"

## Checking balance
quasarnoded q bank balances quasar143wwmxhsd8nkwu7j8gzpv9ca503g8j55h059ew --node tcp://localhost:26659
gaiad q bank balances cosmos1lrelhs37akgz2wht0y377uerxjm9fh33ke3ksc  --node tcp://localhost:26669
osmosisd q bank balances osmo194580p9pyxakf3y3nqqk9hc3w9a7x0yrnv7wcz --node tcp://localhost:26679
bandd q bank balances band1cjx30d7n4k4pedgqkeqztz90q2l465gqrcymgf --node https://rpc.laozi-testnet5.bandchain.org:443


# Create connection


hermes create connection --a-chain quasar --b-chain cosmos
hermes create connection --a-chain quasar --b-chain osmosis
hermes create connection --a-chain osmosis --b-chain cosmos
hermes create connection --a-chain quasar --b-chain $BANDCHAIN

# Create channel

hermes create channel --a-chain cosmos --a-connection connection-0 --a-port transfer --b-port transfer 
hermes create channel --a-chain cosmos --a-connection connection-1 --a-port transfer --b-port transfer 
hermes create channel --a-chain quasar --a-connection connection-1 --a-port transfer --b-port transfer
hermes create channel --a-chain quasar --a-connection connection-2 --a-port qoracle --b-port oracle --channel-version bandchain-1

# start
hermes start > hermes.log 2>&1 
