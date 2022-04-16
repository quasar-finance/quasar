# State 

Orion used certain KV store key-value pairs to maintain its state for its different use cases like strategy position management, reward mechanism, fee collection, and refund.

## Collection of LP fund
When strategy exits the osmosis pool; it will add the exit amount using `AddEpochExitAmt` method which add the exit amount on denom and epoch basis.

### Denom wise withdrawable amount on any epoch
```
Key - {types.ExitKBP} + {epochday} +  {":"} + {denom}
Value = sdk.Coin 
```

### Balance change 
On successful exit from the pool orion module will add the collected exited amount to its module account.
- Balance of the module account increased by the amount of the exited tokens.
  
Note - Also look into the concept section for the detail mechanism of refund calculation logic.

When the actual refund is done on withdrable amount the state of the balance changed based on the amount of tokens refunded. This happens when users explicitly request for the withdraw to qbank. 

- Balance of the orion module account reduced by the refunded tokens.
- Balance of the users increases by the amount of tokens refunded.
  
## Distribution of refund 
At the end of epoch day; it will run its refund logic `DistributeEpochLockupFunds` method which calculate the refund tokens for every depositors. And add the book keeping of calculation in the qbank Withdrawable KV store using qbanks `AddActualWithdrawableAmt`.

- Balance of the module account Orion reserve account `types.OrionReserveMaccName` increased by the minted Quasar native tokens.
- Balance of the management fee collector account `types.MgmtFeeCollectorMaccName` increased by the fee amount charged. 

Note - Also look into the qbank concept and state section for the details. 
As a result of the distribution logic - below state change happends in the Orion module.


## Reward Distribution 
Similar to the refund, reward distribution is done at the end of every epoch. 
### Reward collection 

- All the reward collected will first be added to the Orion global reward collector account `types.CreateOrionRewardGloablMaccName()`  during the claim from osmosis  dex.

- They will be added in the KV store or book keeping. 
```
Key = types.RewardCollectionKBP + {epoch day}
Value = types.RewardCollection 
```

### Reward distribution 
After the reward collection; orion will run its reward distribution logic `RewardDistribution` method.

As part of this, Performance fee gets deducted from the orion global reward collector account `types.CreateOrionRewardGloablMaccName()` And added to the `types.PerfFeeCollectorMaccName`.

As a result of the logic execution below changes happen in the state.
- claim reward amount is being added in the qbanks Claim reward KV store. [ Check the qbanks state document for details ]
 
- Balance of `types.PerfFeeCollectorMaccName` increased by the amount of the performance fee calculated using `DeductVaultFees`
- Balance of the orion global reward collector account `types.CreateOrionRewardGloablMaccName()` gets reduced by the amount of performance fee.
