# Quasar Upgrade Test

## Requirements

- Cosmovisor setup: Follow the Cosmovisor documentation to set up Cosmovisor for your Quasar node. Make sure the
  Cosmovisor is configured correctly to watch for upgrades and switch to the new binaries as required.

## Specifications

- To create new `.sh tests for future versions, follow the same pattern as shown in the provided test script. Make sure
  to update the proposal details, block height for the upgrade, and any additional checks or commands specific to the
  new version.

## Usage

1. Ensure that the Cosmovisor is set up and configured for your Quasar node.
2. Update the test script with the necessary details for the new software upgrade proposal, such as title, description,
   and upgrade height.
3. Run the test script. This will perform the following actions:
- Kill existing quasarnoded processes
- Start the local Quasar network using ./quasar_localnet.sh
- Submit a software upgrade proposal with the specified details
- Sleep for a short duration to allow for proposal submission
- Vote 'yes' on the proposal using the specified account
- Sleep for a short duration to allow for voting
- Wait for the block height to reach the specified upgrade height
- Verify that the upgrade has been successful

The test script will provide you with the necessary logs and information to verify that each step of the upgrade process
was successful. You can use this test suite to test future software upgrades, ensuring a smooth upgrade process for the
Quasar blockchain.