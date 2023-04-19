#!/bin/sh
# This script checks if the container is started for the first time.

set -e

APP_INIT_SCRIPT=app_init.sh
CONTAINER_FIRST_STARTUP_FLAG="CONTAINER_FIRST_STARTUP"

if [ ! -e $CONTAINER_FIRST_STARTUP_FLAG ]; then
    echo "Initializing the container config"
    touch $CONTAINER_FIRST_STARTUP_FLAG
    ./$APP_INIT_SCRIPT
else
    echo "Node already initialized"
    
fi

exec "$@"
