# State

`qoracle` maintain the state of osmosis pool data as state variables in its prefixed KV store.

Below are the states maintain in the qoracle. 

## Pool Info
`PoolInfo` object is stored in the KV store with pool id as the key.
```
Key = PoolInfoKeyPrefix + {PoolId}
Value = PoolInfo 
```

Where `PoolInfo` is defined as below protobuf struct.
```
message PoolInfo {
  string poolId = 1; 
  osmosis.gamm.poolmodels.BalancerPool info = 2; 
  uint64 lastUpdatedTime = 3; 
  string creator = 4;
}
```

## Pool Positions
Pool position is the current position of the osmosis pool as present in osmosis dex.
```
Key = types.PoolPositionKeyPrefix + {PoolId}
Value = PoolInfo 
```
Where `PoolPosition` is defined as below protobuf struct.
```
message PoolPosition {
  string poolId = 1; 
  PoolMetrics metrics = 2; 
  uint64 lastUpdatedTime = 3; 
  string creator = 4;
}
```

## Pool Ranking
Pool ranking is the ranking of pool id based on apy and tvl.
```
Key = types.PoolRankingKey + {PoolId}
Value = {PoolRanking} 
```
Where `PoolRanking` is defined as below protobuf struct.
```
message PoolRanking {
  repeated string poolIdsSortedByAPY = 1; 
  repeated string poolIdsSortedByTVL = 2; 
  uint64 lastUpdatedTime = 3; 
  string creator = 4;
}
```

## Pool spot prices
Pool spot price is the spot price of any pool as present in the osmosis.
```
Key = types.PoolSpotPriceKeyPrefix + {PoolId} + {denomIN} + {denomOut} 
Value =  {PoolSpotPrice} 
```
Where `PoolSpotPrice` is defined as below protobuf struct.
```
message PoolSpotPrice {
  string poolId = 1; 
  string denomIn = 2; 
  string denomOut = 3; 
  string price = 4; 
  uint64 lastUpdatedTime = 5; 
  string creator = 6;
}
```

## Stable price
Stable price is the US dollar equivalent value of any denom.
```
Key = types.types.StablePriceKBP + {"denom"}
Value = {sdk.Dec} // Price 
```