# E2E Tests

## Structure

### `e2e` Package

This `e2e` package defines an integration test suite used for full end-to-end testing functionality. It initializes chains 
based on the test triggered. The current design allows you to test the vault contracts end-to-end. It sets up and deploys the
necessary contracts on the Quasar chain and the pools on Osmosis side with the desired denoms.

The file e2e_wasmd_contract_test.go defines the testing suite and contains the core bootstrapping logic that creates a testing
environment via docker containers (a setup of 2 chains with 1 validator each and an IBC relayer).

## How It Works

- Build Quasar Docker Image :
   - An E2ETestSuiteBuilder is initialised with Quasar chain and 1 IBC relayer configuration. The quasar chain can either be from the current branch or any other specific branch to be tested.
   - If current, we run the DockerFile setup which is a little different from the regular Docker build in make file. As the build needs to have `bash` installed in order to perform certain actions of creating new accounts during genesis with any denom.
   - Use the specific Docker files present in the DockerFiles directory in order to build the required Quasar Docker image.
   - Replace the Dockerfile content in the Root directory with the ones in `tests/e2e/dockerfiles/quasar.Dockerfile` and run `make docker-build` in the root directory to build the desired image for running the e2e locally. 


- Build Osmosis Docker Image :
   - Clone the Osmosis repo, and checkout the desired version (any tag v15.0.0 or after) and replace the contents of the Dockerfile of Osmosis repo with the ones in `tests/e2e/dockerfiles/osmosis.Dockerfile`.
   - Once the contents are replaced, build the Osmosis Docker image using `make docker-build` in the root directory of Osmosis.
   

- Build Smart Contracts : 
   - In the quasar repo, change the directory to `smart-contracts`
   - Within that directory, run the following commands to build the artifacts for testing out the contracts code.
   - For MacM1 users : `docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer-arm64:0.12.11`
   - For other users : `docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.11`


