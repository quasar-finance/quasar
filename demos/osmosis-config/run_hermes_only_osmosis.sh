#!/bin/bash

project_dir="$(cd "$(dirname "${0}")/../.." ; pwd)" # Absolute path to project dir
this_dir="${project_dir}/demos/osmosis-config"
this_script="${this_dir}/$(basename "${0}")"

echo "$project_dir"
echo "$this_dir"
echo "$this_script"


rm -r ~/.hermes
mkdir ~/.hermes
cd $this_dir
cp ./hermes_config.toml ~/.hermes/config.toml

hermes keys add --chain quasar --mnemonic-file quasar_key.txt
# SUCCESS Restored key 'quasarkey' (quasar143wwmxhsd8nkwu7j8gzpv9ca503g8j55h059ew) on chain quasar

hermes keys add --chain osmosis --mnemonic-file osmosis_key.txt
# SUCCESS Restored key 'osmosiskey' (osmo194580p9pyxakf3y3nqqk9hc3w9a7x0yrnv7wcz) on chain osmosis

# hermes keys add --chain $BANDCHAIN --mnemonic-file bandchain_key.txt --hd-path "m/44'/494'/0'/0/0"

# Checking balance
quasard q bank balances quasar143wwmxhsd8nkwu7j8gzpv9ca503g8j55h059ew --node tcp://localhost:26659
osmosisd q bank balances osmo194580p9pyxakf3y3nqqk9hc3w9a7x0yrnv7wcz --node tcp://localhost:26679

# Create connections
#hermes create connection --a-chain quasar --b-chain $BANDCHAIN
hermes create connection --a-chain quasar --b-chain osmosis

# Create channel
#hermes create channel --a-chain quasar --a-connection connection-0 --a-port qbandchainoracle --b-port oracle --channel-version bandchain-1
hermes create channel --a-chain quasar --a-connection connection-0 --a-port qosmosisoracle --b-port icqhost --channel-version icq-1

# start
hermes start