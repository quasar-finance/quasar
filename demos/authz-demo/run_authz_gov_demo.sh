#!/bin/sh
pkill quasard

./quasar_localnet.sh

sleep 10
./authz_gov_demo.sh > log.log 2>&1
