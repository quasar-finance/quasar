# Concepts 

Orion vault is one of the first vault in the quasar chain which is specifically design 
for doing Lping activity and yield aggregation in the osmosis dex. 
Orion module will collect the funds deposited by the users in the qbank and execute the defined strategies.

Orion module will have multiple strategies implemented within it. At the moment it has only one strategy called meissa strategy.

## Meissa strategy

Meissa strategy is one of the strategies available in the Quasar Orion vault. Meissa is a lockup-based strategy specifically designed to do LPing on Osmosis dex. Meissa aims to lock the fund in osmoses dex for 7 days, 21 days, 1 month, 3 months, 6 months, etc., and will also lock the assigned LP tokens back to the pool to increase its earning potential. 

Algotithm - 
The high level algotithm of the meissa strategy can be summarized as below steps.
- Get the list of pools with APY ranks from the oracle module. 
- Iterate apy_ranked_pools with highest apy pool picked first
- Get the list of denoms from the current pool - Denom1, Denom2, and pool denom ratio.
- Collect the max possible amount from both denom 1 and denom 2 from the Orion module staking pool.
- Send the coins using IBC call to osmosis from the quasar custom sender module account ( intergamm module.) 
- Provide liquidity to osmosis via IBC for this pool.
- Update chain state to reduce staking pool amount for both the denom.
- Update the amount deployed on osmosis in the appropriate KV store.
- Go to the next pool and repeat [A - F] 
