# Quasar Upgrade Tests for v1.0.0 from the current mainnet version v.0.1.1

## Requirements

The script should be run by the upgrade branch already containing the new code

Before the execution:

- Go to the main branch from another cloned instance of the chain repository
- Execute a "git checkout v0.1.1"
- Execute a "make install" in order to ensure starting from the correct previous mainnet version.
- Go back to the upgrade branch and run the test script, Before running the test script, open and check the script for 
- some pre-requisites and other instructions. 

After the execution:

- Now that the governance proposal has been success the chain should have been halted. You can check the quasar.log file
  expecting to find: UPGRADE "v0.1.1" NEEDED at height: 30: CONSENSUS FAILURE!!!
- You can now "make install" the new version and check that blocks are produced as expected by running "quasarnoded
  start". It should start producing blocks from height 31.

## Usage

1. Run the test script after setting it as executable:
```bash
cd demos/upgrade-handler/v1.0.0
chmod +x upgrade_handler_v1_0_0.sh
./upgrade_handler_v1_0_0.sh
```

2. After installing the new version binary with v.1.0.0 tag, restart the binary.
```bash
chmod +x restart_v1_0_0_binary.sh
./restart_v1_0_0_binary.sh
```

# How to test upgrade with Contracts in action

Testing a software upgrade on a Cosmos-based chain with contracts involves several steps to ensure a smooth transition and verify that the upgrade process works correctly. Below is a detailed breakdown of each step:

1. **Set Up Chains: Quasar and Osmosis**
   Set up two chains, Quasar and Osmosis, using the appropriate network and node infrastructure. Ensure you have the required configurations and access to deploy contracts and perform transactions.

2. **Set Up Relayer**
   Configure and set up a relayer that can handle IBC (Inter-Blockchain Communication) transactions between the two chains. The relayer will play a crucial role in moving data between the chains during the upgrade process.

3. **Deploy Pools on Osmosis**
   Deploy liquidity pools or any other necessary contracts on the Osmosis chain. Ensure these contracts are functioning as expected before proceeding.

4. **Perform IBC Transactions**
   Initiate IBC transactions from Osmosis to Quasar's treasury account. This tests the cross-chain communication mechanism and verifies that funds can be moved securely.

5. **Deploy Primitives**
   Deploy any additional smart contracts or primitives required for your upgrade. These could be contract templates, modules, or any other logic needed for your application.

6. **Deploy Vault**
   Deploy a contract that represents the vault on the chain. This vault could be related to tokens, assets, or any other functionality your application requires.

7. **Perform Bonding**
   Perform a bond action to lock up a certain amount of tokens in the vault. This tests the bond mechanism and ensures that tokens are securely locked.

8. **Query Bond Account Balance**
   Use queries to check the balance of the bonded account in the vault. This verifies that the bond action was successful and the balance reflects the tokens that were bonded.

9. **Post Software Upgrade Proposal**
   Initiate a software upgrade proposal that includes the necessary upgrade parameters, including the new binary, upgrade plan, and any other relevant information.

10. **Wait for Blocks**
    Allow time for the network to process the software upgrade proposal. Wait for a sufficient number of blocks to pass to ensure the proposal is properly propagated and considered by validators.

11. **Perform Binary Change**
    Once the upgrade proposal passes the governance process, perform the binary change by applying the new version of the software to the chain's nodes. This step should be executed carefully to ensure a smooth transition.

12. **Wait for Blocks to Pass**
    After the binary change, wait for a few blocks to pass to ensure the new version of the software is functioning correctly and has been adopted by the majority of validators.

13. **Perform Bond and Unbond**
    Perform bonding and unbonding actions to test the functionality of the vault and ensure that tokens can be bonded and unbonded as intended.

14. **Perform Claim**
    Perform a claim action to retrieve any rewards or benefits from the vault. This ensures that the vault's claim mechanism is working as expected.

Each of these steps involves both technical and functional testing to verify that the upgrade process is successful and the new software version functions correctly. It's important to have proper testing environments, test cases, and coordination with the network participants to ensure a successful software upgrade.

## Steps

- Download v0.1.1 from Quasar-preview repository
- Put this in your path and name it as `quasarnoded-go-18`
- Checkout to the v1 of Quasar and install it locally as `quasarnoded`
- Once the binaries are in place : 
  - Run :
  ```shell
  cd demos/upgrade-handler/v1.0.0
  sh test_upgrade_with_contracts.sh
  ```
- Once this start all the actions would pe performed automatically. Make sure to keep the contracts artifacts at the latest version in order to perform a good testing.
- Please perform all the other queries after the announcement of `test finished` in order to check all the actions are working properly
