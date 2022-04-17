# Parameters 

`qoracle` module contains the below parameters. 

# OracleAccounts

`OracleAccounts` parameter stores the whitelisted bech32 string of the oracle client

Proto message for the OracleAccounts parameter.

## One hop IBC denom
`OneHopIbcDenomMapping` define the mapping of origin denom in `quasar` and `osmosis`. As any origin denom will be representation as different ibc dex hash value in both the chain. 
This param will help to relate the representations in different chains. 

```
// Params defines the parameters for the module.
message OneHopIbcDenomMapping {
  // option (gogoproto.goproto_stringer) = false;

  string originName = 1 [(gogoproto.moretags) = "yaml:\"origin_name\""]; // Original denom name i.e. uatom
  string quasar = 2  [(gogoproto.moretags) = "yaml:\"quasar\""]; // one hop ibc denom representation in quasar
  string osmo = 3 [(gogoproto.moretags) = "yaml:\"osmo\""]; // one hop ibc denom representation in osmo
  // Representation in the other chains can be added in the future.
}
```

## Stable Denoms
`StableDenoms` is a list of ibc stable denoms present in the osmosis dex or any other dex in future. This is used to calculate the current market value of any other denoms. 

```
// Params defines the parameters for the module.
message Params {
  option (gogoproto.goproto_stringer) = false;
  
  string oracleAccounts = 1 [(gogoproto.moretags) = "yaml:\"oracle_accounts\""];
  repeated  string stableDenoms = 2 [(gogoproto.moretags) = "yaml:\"stable_denoms\""];
  repeated OneHopIbcDenomMapping oneHopDenomMap = 3 [(gogoproto.moretags) = "yaml:\"onehop_ibcdenoms\""];
}
```

## Example genesis params section 

```
"params": {
        "oneHopDenomMap": [
          {
            "originName": "uatom",
            "osmo": "IBC/TESTOSMO",
            "quasar": "IBC/TESTATOM"
          },
          {
            "originName": "uosmo",
            "osmo": "uosmo",
            "quasar": "IBC/TESTOSMO"
          }
        ],
        "oracleAccounts": "oracle_accounts",
        "stableDenoms": [
          "IBC/UST",
          "USTTESTA"
        ]
      }
```