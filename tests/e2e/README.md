# End-to-End (E2E) Tests

## Overview

This section details the E2E testing functionality available in the `e2e` package. These tests are specifically designed
to facilitate end-to-end testing of vault contracts, using the Quasar and Osmosis chains. The structure of the package
allows for efficient setup and deployment of necessary contracts on the Quasar chain and pools on the Osmosis chain, as
per the specific test cases.

Our E2E testing implementation is built upon the robust core of the Strangelove interchaintest framework. This framework
provides the necessary backbone to test inter-blockchain communication effectively and efficiently, enhancing the
overall reliability and performance of our system. It aids in setting up different chains, deploying contracts, and
initiating transactions across chains, which is integral to our end-to-end testing. For more information on the
interchaintest framework, please refer to the
official [Strangelove Ventures GitHub repository](https://github.com/strangelove-ventures/interchaintest).

## Package Structure

The `e2e` package comprises the following components:

- `/cases`: This directory contains the end-to-end test cases.
    - `/_helpers`: This subdirectory houses test helpers and generalized functions.
    - `/_utils`: This subdirectory contains files, payloads, and other resources helpful for mocking entities during
      testing.
- `/dockerfiles`: This directory contains Dockerfiles for any chain that needs to be tested.
- `/dockerutil`: This directory holds utility resources for Docker.
- `/suite`: This directory includes the core package files for end-to-end testing.

## Docker Setup

Our testing framework relies on Docker to instantiate local chains. Prior to executing tests, ensure that you build the
chain Docker images. Currently, the framework supports two chains: Quasar and Osmosis. You can find the respective
Dockerfiles in the `tests/e2e/dockerfiles` directory.

To build a Docker image, use the following command from the repository
root: `make docker-e2e-build [chain1 chain2 ...]`. Here, `chain1` refers to the name of the Dockerfile, excluding the
file extension.

Example: `make docker-e2e-build quasar`
Example: `make docker-e2e-build quasar osmosis`

## Smart Contracts

To create end-to-end tests that involve Wasm contracts, first compile the contracts using the following steps:

1. Navigate to the `smart-contracts` directory.
2. Execute the appropriate command below to build the artifacts needed for testing the contracts code:

For Mac Silicon users:

```bash
docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer-arm64:0.12.11
```

For other users:

```bash
docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.11
```

## Testing

This folder's `Makefile` contains a command that enables the execution of end-to-end (E2E) tests. This provides the
flexibility to either run the complete test suite or execute specific test cases as needed.

### Running the Full Test Suite

To execute all test cases, simply run the following command:

```bash
cd tests/e2e
make test-e2e
```

This command will iterate over all test folders found within `./tests/e2e/cases`, excluding those beginning with an
underscore. For each of these folders, it will execute the `go test ./...` command.

### Running Specific Test Cases

If you want to run specific test cases instead of the entire suite, you can do so by using an environment variable named
`CASES` to specify the test case folders. For example, to run the test cases `case1`, `case2`, and `case3`, you would
use the following command:

```bash
cd tests/e2e
CASES="case1 case2 case3" make test-e2e
```

This command will only execute the `go test ./...` command in the directories `case1`, `case2`, and `case3`.

---

## Technical References

Refer to the following resources for further technical insights:

- [Learn IBC Test](https://github.com/strangelove-ventures/interchaintest/blob/v4/examples/ibc/learn_ibc_test.go)
- [Cosmos Chain Expedited Proposal Test](https://github.com/strangelove-ventures/interchaintest/blob/v4/examples/osmosis/cosmos_chain_expedited_proposal_test.go)
- [Write Custom Tests](https://github.com/strangelove-ventures/interchaintest/blob/main/docs/writeCustomTests.md)
