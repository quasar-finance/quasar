# Local testnet

This demo tutorial demonstrates how to run a local testnet comprising an arbitrary number of validator nodes using docker.

## Prerequisites

* docker: [install guide](https://docs.docker.com/engine/install/)

## Setup

The default number of validator nodes is 4.
If you'd like to change that you can use the following command:
    
    export NUM_NODES=5

Then run the below command to initialize the configs for the local testnet:

    make local-testnet-init

Finally, you can start/stop the local testnet using:

    make local-testnet-start
    make local-testnet-stop

To check if the nodes are successfully running, you can check their logs:

    sudo docker logs -f quasarnode0

In case of success, you should see new blocks every few seconds.
The nodes are numbered from 0 to NUM_NODES-1.

## Running commands on the nodes

### Quasar command on already running testnet

To run a quasar command on a node while the testnet is running, use:

    make local-testnet-exec-quasar node=<node> cmd=<cmd>

For example:

    make local-testnet-exec-quasar node=0 cmd='query bank total --denom=uqsar'

### Quasar command on stopped testnet

To run a quasar command on a node while the testnet is not running, use:

    make local-testnet-run-quasar node=<node> cmd=<cmd>

For example:

    make local-testnet-run-quasar node=0 cmd='keys list --keyring-backend=test'

### bash command on already running testnet

To run a bash command on a node while the testnet is running, use:

    make local-testnet-exec-bash node=<node> cmd=<cmd>

For example:

    make local-testnet-exec-bash node=0 cmd='ls'

### bash command on stopped testnet

To run a bash command on a node while the testnet is not running, use:

    make local-testnet-run-bash node=<node> cmd=<cmd>

For example:

    make local-testnet-run-bash node=0 cmd='ls'


## Cleanup

In order to stop the current local testnet and remove its file, use:

    make clean-local-testnet

To do the same for all local testnets:

    make clean-all-local-testnet
