#!/bin/sh

# restore the keys from the mnemomic phrases, same phrases as the hermes script
rly keys restore cosmos cosmokey "old cinnamon boy hurry pipe upset exhibit title copy squirrel grit eye love toy cotton connect inhale cost quarter mistake ahead endless bless license"
rly keys restore quasar quasarkey "ready hundred phrase theme bar breeze zone system bitter double flush deposit sugar swap burger outside primary nature attend caught wire ticket depth cycle"
rly keys restore osmosis osmokey "rabbit garlic monitor wish pony magic budget someone room torch celery empower word assume digital rack electric weapon urban foot sketch jelly wet myself"

rly q balance quasar
rly q balance cosmos
rly q balance osmosis

rly chains add -f go-relayer-quasar.json quasar
rly chains add -f go-relayer-cosmoshub.json cosmos
rly chains add -f go-relayer-osmosis.json osmosis

