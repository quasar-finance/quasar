#!/bin/sh

BANDCHAIN="band-laozi-testnet5"

rm -r ~/.hermes
mkdir ~/.hermes

cp ./hermes_config.toml ~/.hermes/config.toml

hermes keys add --chain quasar --mnemonic-file quasar_key.txt
hermes keys add --chain osmosis --mnemonic-file osmosis_key.txt
hermes keys add --chain $BANDCHAIN --mnemonic-file bandchain_key.txt --hd-path "m/44'/494'/0'/0/0"

# Checking balance
quasard q bank balances quasar143wwmxhsd8nkwu7j8gzpv9ca503g8j55h059ew --node tcp://localhost:26659
osmosisd q bank balances osmo194580p9pyxakf3y3nqqk9hc3w9a7x0yrnv7wcz --node tcp://localhost:26679

# Create connections
hermes create connection --a-chain quasar --b-chain $BANDCHAIN
hermes create connection --a-chain quasar --b-chain osmosis

# Create channel
hermes create channel --a-chain quasar --a-connection connection-0 --a-port qoracle --b-port oracle --channel-version bandchain-1
hermes create channel --a-chain quasar --a-connection connection-1 --a-port qoracle --b-port icqhost --channel-version icq-1

# start
hermes start
