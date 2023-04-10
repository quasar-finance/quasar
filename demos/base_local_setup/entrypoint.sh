#!/bin/sh
# This script checks if the container is started for the first time.

set -e  
CONTAINER_FIRST_STARTUP="CONTAINER_FIRST_STARTUP"

if [ ! -e $CONTAINER_FIRST_STARTUP ]; then
    echo "Initializing the container config"
    touch $CONTAINER_FIRST_STARTUP
    src/quasar/demos/base_local_setup/quasar_localnet.sh
else
    echo "Node already initialized"
    
fi

quasarnoded start 