#!/bin/bash

BINARY=quasarnoded
HOME_QSR=$HOME/.quasarnode
CHAIN_ID=quasar


version=`quasarnoded version`
if [ "$version" != "1.0.0" ]; then
  echo "You are having incorrect version $version"
  echo "Please install the current mainnet version v1.0.0"
  exit 1
fi
pkill quasarnoded || true

echo "Starting the quasar $version "
$BINARY start --home $HOME_QSR > quasar_new.log 2>&1 &