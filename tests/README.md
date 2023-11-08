
# Test Utilities for Quasar

- [Test Utilities for Quasar](#test-utilities-for-quasar)
    - [Overview](#overview)
    - [Folder Structure](#folder-structure)
    - [Simple Quick Bootup](#simple-quick-bootup)
    - [Docker Images Offered](#docker-images-offered)
    - [How to Use This Test Environment](#how-to-use-this-test-environment)
    - [Shell Folder](#shell-folder)
    - [Troubleshooting](#troubleshooting)
    - [Forwarded Ports](#forwarded-ports)
    - [End-to-End Testing](#end-to-end-testing)



---

## Overview

Within this directory, you will find testing utilities designed for the Quasar platform. 

Included are recipes for building a Docker-based local environment that features the Quasar chain, a local copy of Osmosis, and a Go relayer. 

This environment is designed to be easily disposable, allowing for quick recreation as needed. 

We kindly ask that you add your required baseline configurations to the `bootstrap-scripts` in order to maintain a common base for all tests.

Additionally, the folder contains utilities for integration testing and local manual functional testing and demos. 


## Folder Structure

`tests` folder contains:

- **docker**: Contains the necessary files to create a local environment with Quasar, Osmosis, and a Go relayer. It includes the Dockerfile for each application and the Docker Compose file to orchestrate the containers. It also includes bootstrap scripts to initialize each node with the corresponding configuration file.
  - **bootstrap-scripts**: Contains the initialization scripts for each node and the entrypoint script for the Docker container.
  - **keys**: Contains the private keys for each node.
  - **go-relayer-config**: Contains the relayer configuration files to create the channels between chains.
- **shell**: Contains a set of bash scripts that define different testing scenarios that can be executed on the local environment. The `create_and_execute_contract.sh` script automates the process of creating and executing a smart contract on the Quasar chain.
- **e2e**: Contains a test suite written in golang using the [interchaintest](https://github.com/strangelove-ventures/interchaintest) framework developed by Strangelove

In the root of the repo you have:
- **Makefile**: Defines a set of commands to orchestrate the Docker environment, such as starting, stopping, rebuilding the containers, and connecting to the nodes. It also includes a set of commands to execute the testing scenarios defined in the `shell` folder.
- **Dockerfile**: The main Dockerfile for the Quasar application that defines the dependencies and configurations needed to run the node.


## Simple Quick Bootup

To quickly set up a new environment, run the following command:

```
make docker-compose-up
```

## Docker Images Offered

The test environment uses the following Docker images:

| Image    | Version       | Tag  |
| -------- | ------------- | ---- |
| Quasar   | local dir     | dev  |
| Osmosis  | 15.0.0-alpine | dev  |
| Relayer  | latest        | dev  |

The Osmosis and Go Relayer images are built patching upstream images
to include sample genesis configurations and the relayer configuration. The Quasar image is built from the local repository.

The Go Relayer image is pre-configured with a simple IBC "transfer" channel.
If you need to add more channels, you can either modify the
`relayer_localnet.sh` script or add them at runtime in your shell script.

## How to Use This Test Environment

To create a new environment with default settings, run the following command:
```
make docker-compose-up
```

To destroy (stop and delete) the current environment, use the following command:
```
make docker-compose-down
```

To rebuild the Docker images, run:
```
make docker-compose-build
```

To rebuild the Docker images and restart the containers, run:
```
make docker-compose-rebuild
```

To stop the containers without deleting them, use:
```
make docker-compose-stop
```

To access the command prompt within the Quasar, Osmosis, and Go Relayer nodes, use the following commands:
```
make docker-attach-quasar
```
```
make docker-attach-osmosis
```
```
make docker-attach-relayer
```

## Shell Folder

The `shell` folder contains testing scenarios built using shell scripts. It's a simple "flow" borrowed from the developers' demos, and you can add new flows in this folder without reconfiguring the chain each time.

## Troubleshooting

###To view running containers:

```
docker ps
```

To view all containers (running and stopped):

```
docker ps -a
```

To view Docker images:

```
docker images
```

To view all Docker images (including intermediate images):

```
docker images -a
```

To remove a specific container:

```
docker rm CONTAINER_ID
```

To remove a specific image:

```
docker rmi IMAGE_ID
```

To check the status of the Docker Compose environment, use the following command:


```
docker compose -p localenv -f tests/docker/docker-compose.yml ps
```


## Forwarded Ports

The Docker Compose environment forwards the following ports from the Quasar and Osmosis nodes to your local machine, allowing you to access their RPC and LCD endpoints:

- Quasar:
  - RPC: 26657
  - LCD: 1317

- Osmosis:
  - RPC: 26757
  - LCD: 1417

You can interact with these endpoints using tools like `curl` or HTTP clients in your programming language of choice.

## End-to-End Testing

To run end-to-end tests using the provided `create_and_execute_contract.sh` script, execute the following Makefile target:

```
make docker-test-e2e
```

