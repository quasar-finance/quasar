#!/bin/bash

rm -rf ~/.hermes/keys
rm ~/.hermes/config.toml
cp ./hermes_config.toml ~/.hermes/config.toml

hermes keys add --chain quasar --mnemonic-file quasar_key.txt
# SUCCESS Restored key 'quasarkey' (quasar143wwmxhsd8nkwu7j8gzpv9ca503g8j55h059ew) on chain quasar

hermes keys add --chain osmosis --mnemonic-file osmosis_key.txt
# SUCCESS Restored key 'osmosiskey' (osmo194580p9pyxakf3y3nqqk9hc3w9a7x0yrnv7wcz) on chain osmosis

# Checking balance
quasard q bank balances quasar143wwmxhsd8nkwu7j8gzpv9ca503g8j55h059ew --node tcp://localhost:26659
osmosisd q bank balances osmo194580p9pyxakf3y3nqqk9hc3w9a7x0yrnv7wcz --node tcp://localhost:26679

# Create connections
hermes create connection --a-chain quasar --b-chain osmosis

# Create channel
hermes create channel --a-chain quasar --a-connection connection-0 --a-port transfer --b-port transfer --channel-version ics20-1

# start
hermes start > hermes.log 2>&1 &
