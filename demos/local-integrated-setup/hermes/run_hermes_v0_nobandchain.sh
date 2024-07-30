#!/bin/sh

## This hermes script will establish connections among quasar, cosmos and osmosis local network.
## This script is using the hermes 0.x.x version.
## This script is not taking band chain into considerations.

v=`hermes --version`
STR="$v"
SUB='hermes 0'
echo $STR
if echo "$STR" | grep -q "$SUB"; then
    echo "hermes version is correct for this script."
    echo "continuing ..."
else
    echo "$STR"
    echo "$SUB"
    echo "hermes version is not correct for this script. Please use hermes 0.x.x version for this script"
    echo "exiting ..."
    exit 1
fi

../running_status.sh

is_processes_running=`echo $?`

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
cp v0/hermes_config_nobandchain.toml ~/.hermes/config.toml

hermes keys restore --mnemonic "old cinnamon boy hurry pipe upset exhibit title copy squirrel grit eye love toy cotton connect inhale cost quarter mistake ahead endless bless license" quasar
hermes keys restore --mnemonic "ready hundred phrase theme bar breeze zone system bitter double flush deposit sugar swap burger outside primary nature attend caught wire ticket depth cycle" cosmos
hermes keys restore --mnemonic "rabbit garlic monitor wish pony magic budget someone room torch celery empower word assume digital rack electric weapon urban foot sketch jelly wet myself" osmosis

## Checking balance
quasard q bank balances quasar143wwmxhsd8nkwu7j8gzpv9ca503g8j55h059ew --node tcp://localhost:26659
gaiad q bank balances cosmos1lrelhs37akgz2wht0y377uerxjm9fh33ke3ksc  --node tcp://localhost:26669
osmosisd q bank balances osmo194580p9pyxakf3y3nqqk9hc3w9a7x0yrnv7wcz --node tcp://localhost:26679


# Create connection
hermes create connection quasar cosmos
hermes create connection quasar osmosis
hermes create connection osmosis cosmos

# Create channel

hermes create channel --port-a transfer --port-b transfer cosmos connection-0
hermes create channel --port-a transfer --port-b transfer cosmos connection-1
hermes create channel --port-a transfer --port-b transfer quasar connection-1

# hermes start
hermes start > hermes.log 2>&1
