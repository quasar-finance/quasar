# Development Guidelines

## Pull requests
In order to allow fast and efficient reviews, please create small PRs per independent change instead of big ones for a full feature. 

**Rule of thumb**: It should be easy to formulate the commit message as a one-liner. If that is not the case, then the PR is probably to big.


## DoD -- Definition of Done

tbd


## Ready for review

tbd

## Unplanned work
In order to balance tracking of our progress and speed of development we use the following rules for unplanned works that gets identified during a sprint:
* fix takes less than 1h and you address it immediately: no ticket required
* fix takes more than 1h or you don't address it immediately: create a ticket

## Releases

### Smart contracts
The following steps are required to release smart contracts:
1. Create a new release:
    * Make sure you have the latest state: `git checkout main && git pull`
    * Create a tag: `git tag -a "<TAG>" -m"<DESCRIPTION>"` (TAG should be of the form v[0-9]+.[0-9]+.[0-9]+)
    * Push tag: `git push origin <TAG>`
2. Upload code through multisig (This can only be done by the multisig-members, please sync with them):
    * Create signed message: `bash scripts/generate_signed_upload_tx.sh <WASM_FILE> <DEPLOYER> <MULTISIG>`, where <DEPLOYER> and <MULTISIG> are the names of your key, resp. the multisig key, registered with `osmosisd`.
    * Collect signed messages from coworkers, when you have enough: `bash scripts/upload_through_multisig.sh "<SIGNED_TX_1> <SIGNED_TX_2>"`
3. Create proposal to instantiate or migrate contracts on [DAODAO](https://daodao.zone/dao/osmo12ry93err6s2ekg02ekslucwx8n3pxm3y7zxz3l6w8zuhex984k5ss4ltl6/proposals).
4. After the proposal did receive enough votes, it can be executed.
5. Please make sure that all instances of a contract are migrated. You can find a list of contracts [here](smart-contracts/contracts.json). The easiest way to propose the migration of all contracts in that list is by adjusting `scripts/propose_cl_vault_migration` and executing `node scripts/propose_cl_vault_migration.js` from the repo root directory. 
6. Update versions and deployment dates in the list of contracts. This can be done with the help of `scripts/update_contracts_list.js`.