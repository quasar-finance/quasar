# State

qbank manages the state of fund by using different combinations of prefixed keys.

qbank module implements three major functionalities,
1. Deposit 
2. Withdraw
3. Claim reward 

For each functionality it maintain states in its KV store. 

## Deposit 
1. The deposit transaction allows the users to do the deposit transactions in the quasar chain. This use case is primarily used to deposit IBC transferred tokens from other IBC-enabled chains. To do so users first need to ibc transfer the other chain native tokens such as cosmos-hub atom, Juno network Juno, crypto.com CRO, etc. to quasar chain to users' own quasar address.
2. At the time of deposit; users will have the option to select the vault, tokens amount, and lockup periods. In phase #1, only the Orion vault is available. The lockup periods option allows users to decide on the number of days/weeks/months they want to deposit funds. 
3. On successful acceptance of the deposit transaction by the network, the qbank module message server processes the transaction message, it will bank transfer the tokens to the vault account and do the bookkeeping of the deposit done in its prefixed KV store.
4. Once it is available to the vault; the vault will do its decision-making based on the strategies it is running. 
5. The deposit bookkeeping information stored in the vault will be used by the vaults later to calculate the withdrawable amount and reward calculation. 
6. Before doing deposits users should be aware of the Orion vault mechanism [ to be documented on the Orion vault document ] on a high level. As it is not a reversible transaction. 

### Deposit transaction message is defined as - 
```
message MsgRequestDeposit {
 string creator = 1;
 string riskProfile = 2; // Reserved for future use.
 string vaultID = 3;
 cosmos.base.v1beta1.Coin coin = 4 [ (gogoproto.nullable) = false ];
 LockupTypes lockupPeriod  = 5;
}
```

### Deposit state transition - 
1. With this deposit transaction, the amount will be transferred to the specified vault module account lockup container.  The lockup container is the deposit fund collector based on the lockup period defined in the transaction message. The lockup container help to build a better accounting mechanism. 
2. Users’ balance will be reduced and vault fund collector balance will be increased.
3. Deposit bookkeeping information will be updated to reflect the current state of the user's deposit.  The Kv store for the deposit betters explains the state of qbank keeper. 

### KV store design - 

#### Total deposit so far by a user - 
```
Key = types.UserDepositKBP + {UserAcc }
Value = Value =  types.QCoins [ Wrapper of sdk.Coins, Total deposit so far irrespective of the withdrawal ] 
```
#### Total denom deposit so far by a user -
```
Key = types.UserDenomDepositKBP + {useraccount} + {":"} + {denom}
Value = sdk.Coin [ Total denom deposit so far irrespective of the withdrawal ] 
```
#### Total epoch lockup denom deposit by a user - 
This is to maintain the denom deposit done by a user on any given epoch day for any lockup periods. 
```
Key = {EpochLockupUserDenomDepositKBP} + {epochday} + ":" + {lockupString} + ":" + {uid} + ":" + {denom}
Value = sdk.Coin 
```

## Withdraw 

1. Withdraw transactions allow users to withdraw the currently available withdrawable funds for a specific vault.
2. As Orion vault is providing liquidity to the osmosis liquidity pools, users should be aware that the withdrawable amount can be very well different than the original deposit amount due to the nature of liquidity pool dynamics. The difference between the original deposit denom amount is covered by the equivalent amount of Orion vault tokens.  
3. Users' withdrawable amount will be added to the withdrawable prefixed KV store by vault.

### Withdraw transaction message is defined as - 

```
message MsgRequestWithdraw {
 string creator = 1;
 string riskProfile = 2;
 string vaultID = 3;
 cosmos.base.v1beta1.Coin coin = 4 [ (gogoproto.nullable) = false ];
}
```

### Withdraw State transition - 
1. Balance in the Vault module account will be reduced by the withdrawal amount. 
2. Balance of the user's account will be increased by the withdrawal amount.
3. Withdrawal bookkeeping will be updated to reflect the current available withdrawal amount. If it is zero; the associated key will be deleted.
4. Total withdraw amount will be increased by the withdraw amount in this transaction.

### Withdrawal KV store design - 
```
Key = types.ActualWithdrawableKeyKBP + {userAcc} + ":" + {denom}
Value =  sdk.Coin [ Current withdrawal amount of given denom ] 
```

### Total withdraw amount
```
Key = types.TotalWithdrawKeyKBP + {uid} + ":" + {vaultID}
Value =  types.QCoins [ Wrapper of sdk.Coins ] 
```

## Claim rewards

1. Claim reward transaction allows users to claim all the accumulated rewards for the user.
2. Orion Vault collects the rewards from osmosis dex and does the distribution calculation to add in the claim bookkeeping in the qbank kv store.
3. On successfully processing the claim reward transaction message, the claim amount will be bank transferred to the depositor account. And claim kv store will become empty for the requested user.
4. Total claimed amount also increased by the amount claimed in this transaction.

### Claim transaction message is defined as - 

```
message MsgClaimRewards {
 string creator = 1;
 string vaultID = 2;
}
```

### Claim State transition - 
1. User balance will be increased by the claimed amount.
2. Vault Reward module account will be reduced by the claimed amount.

### Claim KV store design - 
``` 
Key - types.UserClaimKBP + {userAccount} + {":"} + {VaultID}
Value =  types.QCoins [ Wrapper of sdk.Coins ] 
```  

### Total claimed amount 
Total claimed amount represent the total token rewards claimed by the user so far.
```
Key - types.UserClaimedKBP + {userAccount} + {":"} + {VaultID}
Value = types.QCoins [ Wrapper of sdk.Coins ]
```