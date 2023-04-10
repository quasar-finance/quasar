#!/bin/sh
# This script checks if the container is started for the first time.

CONTAINER_FIRST_STARTUP="CONTAINER_FIRST_STARTUP"
ls /tmp

if [ ! -e /tmp/$CONTAINER_FIRST_STARTUP ]; then
    touch /tmp/$CONTAINER_FIRST_STARTUP
    echo "Initializing the container config"
    src/quasar/demos/base_local_setup/quasar_localnet.sh
else
    echo "Node already initialized"
    
fi

wait 20
quasarnoded start 