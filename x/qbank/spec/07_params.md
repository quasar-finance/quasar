### Params

`qbank` module has following parameters.

## Enabled 
enabled param is used to enable or disable qbanks transactions.

## OrionEpochIdentifier

## MinOrionEpochDenomDollarDeposit
minOrionEpochDenomDollarDeposit defines the minimum amount of dollar equivalent to be deposited 
in the current epoch for any denom.

## WhiteListedDenomsInOrion
WhiteListedDenomsInOrion defines the list of whitelisted ibc denoms as defined by the `WhiteListedDenomInOrion`.

### One hop IBC denom
`WhiteListedDenomInOrion` define the mapping of origin denom in `quasar` and `osmosis`. As any origin denom will be representation as different ibc dex hash value in both the chain. 
This param will help to relate the representations in different chains. 

```protobuf
message WhiteListedDenomInOrion {
  // option (gogoproto.goproto_stringer) = false;

  string originName = 1 [(gogoproto.moretags) = "yaml:\"origin_name\""]; // Original denom name i.e. uatom
  string onehopQuasar = 2  [(gogoproto.moretags) = "yaml:\"onehop_quasar\""]; // one hop ibc denom representation in quasar
  string onehopOsmo = 3 [(gogoproto.moretags) = "yaml:\"onehop_osmo\""]; // one hop ibc denom representation in osmo
  // Representation in the other chains can be added in the future.
}
```
## Example params field in genesis file - 

```json
 "qbank": {
      "params": {
        "enabled": true,
        "min_orion_epoch_denom_dollar_deposit": "100.000000000000000000",
        "orion_epoch_identifier": "minute",
        "white_listed_denoms_in_orion": [
          {
            "onehop_osmo": "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
            "onehop_quasar": "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
            "origin_name": "uatom"
          },
          {
            "onehop_osmo": "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
            "onehop_quasar": "uqsar",
            "origin_name": "uqsar"
          }
        ]
      }
    }
```

## Param spec  

| Key                              | Type                    | Example                    |
| -------------------------------- | ------------------------| -------------------------- |
| Enabled                          | bool                    | true/false                 |
| MinOrionEpochDenomDollarDeposit  | string (sdk.Dec)        | "100.000000000000000000"   |
| KeyOrionEpochIdentifier          | string                  | "day"                      |
| WhiteListedDenomsInOrion         | WhiteListedDenomInOrion | Refer Example.             |
