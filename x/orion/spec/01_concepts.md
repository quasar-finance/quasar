# Concepts 

Orion vault is first vault in the quasar chain which is specifically design 
for doing Lping activity and yield aggregation in the osmosis dex. Orion module will collect the funds deposited by the users in the qbank and execute the defined strategies.

Orion has majorly three parts internally. 
1. Strategy 
2. Reward Distribution
3. Refund Distribution 
4. Treasury build-up

# Strategy 
Orion module will have multiple strategies implemented within it. At the moment it has only one strategy called meissa strategy.

## Meissa strategy

Meissa strategy is one of the strategies available in the Quasar Orion vault. Meissa is a lockup-based strategy specifically designed to do LPing on Osmosis dex. Meissa aims to Lp in osmosis pools for a period based on the lockup interval mentioned by depositors and will also lock the LP tokens with the pool. 

## Algotithm
The high level algotithm of the meissa strategy can be summarized as below steps.
- Get the list of pools with APY ranks from the oracle module. 
- Iterate apy_ranked_pools with highest apy pool picked first
- Get the list of denoms from the current pool - Denom1, Denom2, and pool denom ratio.
- Collect the max possible amount from both denom 1 and denom 2 from the Orion module staking pool.
- Do the IBC token transfer using IBC call to osmosis from the quasar using multihop operations via denom source chains.
- Provide liquidity to osmosis via IBC Interchain Account for this pool.
- Upon positive acknowlegement, update chain state to reduce staking pool amount for both the denom.
- Update the amount deployed on osmosis in the appropriate KV store. 
- Go to the next pool and repeat.


# Reward Distribution 
Reward distribution is done by the Orion module based on the users weighted contribution on a particular denom. The algorithm collect the rewards every day at the end of epochday. And calculates the reward associated with each of the denom contribution. And then it takes the weight of each users on the particular denom deposit amounts. 

# Refund Distribution and assurance mechanism using Orion vault token

This section explains the initial mechanism of Orion token design and how the Orion vault covers depositor funds and assures them that they will get the equivalent withdrawable value during withdrawal.

Orion module mint vault native Orion which is covered by stable coins equivalent to values of other tokens like QSR which is based on the current market value. 

When a user Alice deposits 1000 atoms, the Orion module will calculate the equivalent amount of Orion. It won’t mint any Orions at that point in time, but will just keep a record of it in the KV store.  

Orion native token will be used to cover the difference between the deposited and withdrawable amount. As the Orion module is doing Lping on osmosis pools; it is normal to have a difference between the exit and join tokens amounts.  In such cases, Orion wants to provide assurance to the depositors that they will at least get the same equivalent market value at the time of withdrawal. 

Before the withdrawal, allocated orions are not transferred to the depositors. They exist in a form of bookkeeping. It will be minted and transferred only during the withdrawal activity when there is a difference between the deposit and withdrawable. Once transferred it will be owned by the users as sdk.Coin with “Orion” denom. 

Allocated and Owned Orion shares will also represent the Orion governance power in the Orion module decision-making.  Governace will be implemented in future.

Owned Orions are ibc enabled like any other sdk.Coin. 
At some point of time in the future when Orion Treasury collects a sufficient amount in the treasury of other tokens, it can decide to mint “Orion” shares as sdk.Coin from the treasury for other purposes based on the community decisions. 

## The assurance gurantee is provided by the below example logic -

- Orion calculate the current market value of the diff ( Withdrawable - Deposit ) 
- Mint and Lock equivalent Quasar native tokens in the Orion Module.
- Mint Orions and allocate them to users.  This way Orion will back the user's deposit diff and Orion itself is backed by Quasar. 
- Ref methods to check is `MintAndAllocateOrions` and `DistributeEpochLockupFunds`
- The locked Quasar can be reused by the Quasar to secure the network with internal bonding to Validators. [ Phase 2 ] 


# Treasury build up 
Orion module has two type of fees to build its treasury. 
1. Management Fee - 
   Orion module charge a fixed configurable percentage of the deposit amount.  Management fee is collected by the `types.MgmtFeeCollectorMaccName` account.
2. Performance Fee - 
   Orion module charge a fixed configurable percentage on the reward collected from osmosis dex on each day. Performance fee is collected by the `types.PerfFeeCollectorMaccName`


# Connection with the intergamm module
Orion module call the intergamm module keeper methods for sending ibc messages using interchain account to osmosis dex. 
