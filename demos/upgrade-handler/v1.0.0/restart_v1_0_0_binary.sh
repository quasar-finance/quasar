#!/bin/bash

BINARY=quasard
HOME_QSR=$HOME/.quasarnode
CHAIN_ID=quasar


version=`quasard version`
if [ "$version" != "1.0.0" ]; then
  echo "You are having incorrect version $version"
  echo "Please install the current mainnet version v1.0.0"
  exit 1
fi
pkill quasard || true

echo "Starting the quasar $version "
$BINARY start --home $HOME_QSR > quasar_new.log 2>&1 &