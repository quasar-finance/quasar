# State

Qoracle maintain the state of osmosis pool data as state variables in its prefixed KV store.

Below are the states maintain in the qoracle. 

## Pool Info
PoolInfo object is stored in the KV store with pool id as the key.

Byte Prefix = PoolInfoKeyPrefix
Key = byte converted value of  PoolId
Value = byte converted value of PoolInfo 

Where PoolInfo is defined as below protobuf struct.

message PoolInfo {
  string poolId = 1; 
  osmosis.gamm.poolmodels.BalancerPool info = 2; 
  uint64 lastUpdatedTime = 3; 
  string creator = 4;
}


## Pool Positions
Byte Prefix = types.PoolPositionKeyPrefix
Key = byte converted value of  PoolId
Value = Value = byte converted value of PoolInfo 

Where PoolPosition is defined as below protobuf struct.

message PoolPosition {
  string poolId = 1; 
  PoolMetrics metrics = 2; 
  uint64 lastUpdatedTime = 3; 
  string creator = 4;
}


## Pool Ranking 
Byte Prefix = types.PoolRankingKey
Key = byte converted value of  PoolId
Value = Value = byte converted value of PoolRanking 

Where PoolRanking is defined as below protobuf struct.

message PoolRanking {
  repeated string poolIdsSortedByAPY = 1; 
  repeated string poolIdsSortedByTVL = 2; 
  uint64 lastUpdatedTime = 3; 
  string creator = 4;
}

## Pool spot prices
Byte Prefix = types.PoolSpotPriceKeyPrefix
Key = byte converted value of  {PoolId} + {denomIN} + {denomOut} 
Value =  byte converted value of PoolRanking 

Where PoolRanking is defined as below protobuf struct.

message PoolSpotPrice {
  string poolId = 1; 
  string denomIn = 2; 
  string denomOut = 3; 
  string price = 4; 
  uint64 lastUpdatedTime = 5; 
  string creator = 6;
}

