# Quasar Upgrade Tests for v0.1.1

## Requirements

The script should be run by the upgrade branch already containing the new code

Before the execution:

- Go to the main branch from another cloned instance of the chain repository
- Execute a "git checkout v0.1.0"
- Execute a "make install" in order to ensure starting from the correct previous mainnet version.
- Go back to the upgrade branch and run the test script

After the execution:

- Now that the governance proposal has been success the chain should have been halted. You can check the quasar.log file
  expecting to find: UPGRADE "v0.1.1" NEEDED at height: 30: CONSENSUS FAILURE!!!
- You can now "make install" the new version and check that blocks are produced as expected by running "quasarnoded
  start". It should start producing blocks from height 31.

## Usage

1. Run the test script after setting it as executable:
```bash
cd demos/upgrade-handler/v0.1.1
chmod +x upgrade_handler_v0_1_1.sh
./upgrade_handler_v0_1_1.sh
```