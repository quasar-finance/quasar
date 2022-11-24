#!/bin/sh

# restore the keys from the mnemomic phrases, same phrases as the hermes script
rly keys restore quasar quasarkey "ready hundred phrase theme bar breeze zone system bitter double flush deposit sugar swap burger outside primary nature attend caught wire ticket depth cycle"

rly q balance quasar

rly chains add -f go-relayer-quasar.json quasar

