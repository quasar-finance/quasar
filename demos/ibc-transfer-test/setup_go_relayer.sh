#!/bin/sh

# remove any old configs
RELAYER_CONF="$HOME/.relayer"
rm -rf $RELAYER_CONF &> /dev/null

rly config init

# add configs for each chain, 
rly chains add-dir ./go-relayer-config/chains

# restore the keys from the mnemomic phrases, same phrases as the hermes script
OSMOKEY="$(cat ./keys/osmo.key)"
QUASARKEY="$(cat ./keys/qsr.key)"

rly keys restore quasar quasarkey "$QUASARKEY"
rly keys restore osmosis osmokey "$OSMOKEY"

rly q balance quasar
rly q balance osmosis

rly paths add-dir ./go-relayer-config/paths
rly tx link  quasar_osmosis --debug  >> ./logs/rly_qo_setup.log 2>&1

