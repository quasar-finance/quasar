# Params 

Orion module has below parameters to control its activities. 

1. PerfFeePer - 
    PerfFeePer defines the vault performance fee as percentage of the reward collected from osmosis 
2. MgmtFeePer -
    MgmtFeePer defines the account mangement fees and is calculated based on the percentage of users deposit amount.
3. LpEpochId  -
     LpEpochId is the epoch identifier subscribed by the orion module for its Lping, reward collection, reward distribution, refund and fee collection interval. If it is set as day, it  will execute the strategy on per epoch day. 
4. Enabled  - 
    Enabled defines the parameter whether the orion module is enabled or disabled. 

## Example params field in genesis file - 
```
"params": {
        "enabled": true,
        "lp_epoch_id": "minute",
        "mgmt_fee_per": "0.005000000000000000",
        "perf_fee_per": "0.030000000000000000"
      }
```

## Param spec  

| Key                 | Type             | Example                    |
| ------------------- | -----------------| -------------------------- |
| PerfFeePer          | string (sdk.Dec) | "0.005000000000000000"     |
| MgmtFeePer          | string (sdk.Dec) | "0.030000000000000000"     |
| LpEpochId           | string           | "day"                      |
| Enabled             | bool             |  true                      |
