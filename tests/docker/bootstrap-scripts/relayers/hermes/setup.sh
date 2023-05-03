#!/bin/sh


# Copy config.toml
cp $HOME/config/config.toml $HOME/.hermes/config.toml

# restore the keys from the mnemomic phrases, same phrases as the hermes script
# COSMOSKEY="$(cat ./keys/gaia.key)"
OSMOKEY="$(cat $HOME/keys/osmo.key)"
QUASARKEY="$(cat $HOME/keys/qsr.key)"
hermes keys add --key-name keyquasar --chain quasar --key-file "$QUASARKEY"
hermes keys add --key-name keyosmosis --chain osmosis --key-file "$OSMOKEY"

# Create clients
hermes create client --host-chain quasar --reference-chain osmosis
hermes create client --host-chain osmosis --reference-chain quasar

#Create connection
hermes create connection --a-chain quasar --a-client 07-tendermint-0 --b-client 07-tendermint-0

# Create ICS20
hermes create channel --a-chain quasar --a-port ics20 --b-port ics20 --order ordered --channel-version ics20 --a-connection connection-0

# Create ICA (ICS27)
hermes create channel --a-chain quasar --a-port ics-27 --b-port icqhost --order unordered --channel-version '{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}' --a-connection connection-0

# Create ICQ (ICS32)
hermes create channel --a-chain quasar --a-port icq-1 --b-port icqhost --order unordered --channel-version icq-1 --a-connection connection-0