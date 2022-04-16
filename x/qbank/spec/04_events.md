## Events

qbank module emits follwoing events and events attributes during deposit, withdraw and claim.

## Event types
```
TypeEvtDeposit      = "deposit"
TypeEvtWithdraw     = "withdraw"
TypeEvtWithdrawAll  = "withdraw_all"
TypeEvtClaimRewards = "claim_rewards"
```
## Event attributes
```
AttributeKeyDepositCoin         = "deposit_coin"
AttributeKeyDepositLockupPeriod = "deposit_lockup_period"
AttributeKeyDepositEpoch        = "deposit_epoch"
AttributeKeyWithdrawCoin        = "withdraw_coin"
AttributeKeyWithdrawVaultId     = "withdraw_vault_id"
AttributeKeyWithdrawRiskProfile = "withdraw_risk_profile"
AttributeKeyWithdrawAllVaultId  = "withdraw_all_vault_id"
AttributeKeyClaimRewardsVaultId = "claim_rewards_vault_id"
```

## Event attribute table


| Type                | Attribute Key                     | Attribute Value         |
|---------------------|-----------------------------------|-------------------------|
| TypeEvtDeposit      | AttributeKeyDepositCoin           | {deposit coin}          |
| TypeEvtDeposit      | AttributeKeyDepositLockupPeriod   | {deposit lockup period} |
| TypeEvtDeposit      | AttributeKeyDepositEpoch          | {deposit epoch day}     |
| TypeEvtWithdraw     | AttributeKeyWithdrawCoin          | {withdraw coin}         |
| TypeEvtWithdraw     | AttributeKeyWithdrawVaultId       | {withdrawal vault}      |
| TypeEvtWithdraw     | AttributeKeyWithdrawRiskProfile   | {risk profile}          |
| TypeEvtWithdrawAll  | AttributeKeyWithdrawAllVaultId    | {withdrawal vault}      |
| TypeEvtClaimRewards | AttributeKeyClaimRewardsVaultId   | {claim vault}           |

