#!/bin/sh

hermes keys restore --mnemonic "jungle law popular reunion festival horn divorce quarter image gather october weird slide trend resource render abuse food tomorrow multiply price fun ask quarter" quasar

hermes keys restore --mnemonic "blade trap agent boy note critic jazz nuclear eight lion pipe fresh tourist make broken inquiry close agree usual human stock move remain swim" cosmos

hermes keys restore --mnemonic "act scale exhibit enough swamp vivid bleak eagle giggle brass desert debris network scrub hazard fame salon normal over between inform advance sick dinner" osmosis

## Checking balance
quasarnoded q bank balances quasar1tshnze3yrtv3hk9x536p7znpxeckd4v9ha0trg --node tcp://localhost:26659
gaiad q bank balances cosmos14ahzv9ldtfn7ktgnd0m8k70d6l080lvdlrrsth  --node tcp://localhost:26669
osmosisd q bank balances osmo139njd402zqj368sk65y753ppp4hxr9268w7wdp --node tcp://localhost:26679


# Create connection
hermes create connection quasar cosmos

hermes create connection quasar osmosis

hermes create connection osmosis cosmos

# Create channel

hermes create channel --port-a transfer --port-b transfer cosmos connection-0

hermes create channel --port-a transfer --port-b transfer cosmos connection-1

hermes create channel --port-a transfer --port-b transfer quasar connection-1

# start
hermes start
