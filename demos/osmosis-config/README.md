# Prerequisites
1. `go` v1.19
2. `hermes` v1.0.0
3.  osmosis repo with async-icq or quasar repo sdk45 ( feature/sdk45_chainupdates or main-sdk45)

# Setup
Install the main binary of this source code with the following command:
```bash
make install
```
Clone either the forked version of `osmosis` or osmosis v15 release candidate branch. and install the osmosisd binary with the following commands:
```bash

git clone https://github.com/quasar-finance/osmosis.git -b v12.0.0-icq --depth 1

OR
clone the tag/v15.0.0-rc0 , https://github.com/osmosis-labs/osmosis/releases/tag/v15.0.0-rc0

cd ./osmosis

make install
```

# Starting Quasar node
Run the following commands to start a single node of quasar in local machine with the preset of parameters needed for this demo:
```bash
cd ./demos/osmosis-config

./quasar_localnet.sh
```
After this you should see block logs written in the stdout of your terminal.

# Starting Osmosis node
Run the following commands to start a single node of osmosis in local machine with the preset of parameters needed for this demo:
```bash
cd ./demos/osmosis-config

./osmo_localnet.sh
```
After this you should see block logs written in the stdout of your terminal.

# Config and Start the Hermes Relayer

Before running the relayer checkout http://localhost:1311/quasarlabs/quasarnode/qoracle/osmosis/state by default the response should be:
```json
{
  "coin_rates_state": {
    "call_data": null,
    "request_packet_sequence": "0",
    "oracle_request_id": "0",
    "result_packet_sequence": "0",
    "result": null,
    "failed": false,
    "updated_at_height": "0"
  },
  "osmosis_params_request_state": {
    "packet_sequence": "0",
    "acknowledged": false,
    "failed": false,
    "updated_at_height": "0"
  },
  "osmosis_incentivized_pools_state": {
    "packet_sequence": "0",
    "acknowledged": false,
    "failed": false,
    "updated_at_height": "0"
  },
  "osmosis_pools_state": {
    "packet_sequence": "0",
    "acknowledged": false,
    "failed": false,
    "updated_at_height": "0"
  }
}
```
Simply run the following commands to config and start the hermes relayer. 
## Note - This demo does not intend to run bandchain part of the qoracle. 

```bash
cd ./demos/osmosis-config

### Run hermes relayer and create necessary connections using below commands.

[run_hermes_only_osmosis.sh](./run_hermes_only_osmosis.sh)
```
### NOTE - Ignore bandchain 
### For this demo of using the qoracle you can also avoid bandchain part; as bandchain part is anyway expected to be disabled or deprecated. We will consider 
### bandchain for future enhancement. 

Now after hermes starts to run you should see changes in `"packet_sequence"` field of `"coin_rates_state"` (which is about fetching coin prices from bandchain) and `"osmosis_incentivized_pools_state"` (which is about fetching list of incentivized pools from osmosis) This means that quasar sent the packets.

After a while if the bandchain response with a successful receive a response like below from http://localhost:1311/quasarlabs/quasarnode/qoracle/oracle_prices
```json
{
  "prices": [
    {
      "denom": "ATOM",
      "amount": "14.338800000000000000"
    },
    {
      "denom": "BNB",
      "amount": "267.529999000000000000"
    },
    {
      "denom": "BTC",
      "amount": "19251.452500000000000000"
    },
    {
      "denom": "OSMO",
      "amount": "1.255382000000000000"
    }
  ],
  "updated_at_height": "112"
}
```

# Updating Osmosis Chain Params
Before updating the osmosis chain params http://localhost:1311/quasarlabs/quasarnode/qoracle/osmosis/chain_params returns empty result like:
```json
{
  "epochs_info": [
  ],
  "lockable_durations": [
  ],
  "mint_params": {
    "mint_denom": "",
    "genesis_epoch_provisions": "0.000000000000000000",
    "epoch_identifier": "",
    "reduction_period_in_epochs": "0",
    "reduction_factor": "0.000000000000000000",
    "distribution_proportions": {
      "staking": "0.000000000000000000",
      "pool_incentives": "0.000000000000000000",
      "developer_rewards": "0.000000000000000000",
      "community_pool": "0.000000000000000000"
    },
    "weighted_developer_rewards_receivers": [
    ],
    "minting_rewards_distribution_start_epoch": "0"
  },
  "mint_epoch_provisions": "\u003cnil\u003e",
  "distr_info": {
    "total_weight": "0",
    "records": [
    ]
  }
}
```
To Update the chain params of osmosis in quasar run the following command:
```bash
quasarnoded tx qoracle osmosis update-osmosis-chain-params --node tcp://localhost:26659 --from alice --home ~/.quasarnode --chain-id quasar --output json --keyring-backend test
```
After hermes relayed the acknowledgement the result of http://localhost:1311/quasarlabs/quasarnode/qoracle/osmosis/chain_params will change to:
```json
{
  "epochs_info": [
    {
      "identifier": "day",
      "start_time": "2022-09-21T17:11:05.191976Z",
      "duration": "86400s",
      "current_epoch": "1",
      "current_epoch_start_time": "2022-09-21T17:11:05.191976Z",
      "epoch_counting_started": true,
      "current_epoch_start_height": "1"
    },
    {
      "identifier": "hour",
      "start_time": "2022-09-21T17:11:05.191976Z",
      "duration": "3600s",
      "current_epoch": "1",
      "current_epoch_start_time": "2022-09-21T17:11:05.191976Z",
      "epoch_counting_started": true,
      "current_epoch_start_height": "1"
    },
    {
      "identifier": "week",
      "start_time": "2022-09-21T17:11:05.191976Z",
      "duration": "604800s",
      "current_epoch": "1",
      "current_epoch_start_time": "2022-09-21T17:11:05.191976Z",
      "epoch_counting_started": true,
      "current_epoch_start_height": "1"
    }
  ],
  "lockable_durations": [
    "120s",
    "180s",
    "240s"
  ],
  "mint_params": {
    "mint_denom": "uosmo",
    "genesis_epoch_provisions": "5000000.000000000000000000",
    "epoch_identifier": "day",
    "reduction_period_in_epochs": "156",
    "reduction_factor": "0.500000000000000000",
    "distribution_proportions": {
      "staking": "0.400000000000000000",
      "pool_incentives": "0.300000000000000000",
      "developer_rewards": "0.200000000000000000",
      "community_pool": "0.100000000000000000"
    },
    "weighted_developer_rewards_receivers": [
    ],
    "minting_rewards_distribution_start_epoch": "0"
  },
  "mint_epoch_provisions": "5000000.000000000000000000",
  "distr_info": {
    "total_weight": "11100",
    "records": [
      {
        "gauge_id": "0",
        "weight": "10000"
      },
      {
        "gauge_id": "1",
        "weight": "1000"
      },
      {
        "gauge_id": "2",
        "weight": "100"
      }
    ]
  }
}
```

## NOTE - If you are ignoring bandchain protocol part ; then you should also ignore the APY/TVL Calculations at the moment. Without the stable prices 
## that part is not feasible. Initially we can use qoracle only for the saving osmosis pool states locally.

# Creating a Pool in Osmosis
To create a pool in osmosis simply run the following command which will create a simple pool with `uosmo` and dummy `uatom` tokens:
```bash
cd ./demos/osmosis-config

osmosisd tx gamm create-pool --pool-file demo_pool.json --home ~/.osmosis --chain-id osmosis --node=http://localhost:26679 --from alice --gas=300000 --output json --keyring-backend test
```
On successful execution of tx you should see the pool in response of http://localhost:1312/osmosis/gamm/v1beta1/pools
```json
{
  "pools": [
    {
      "@type": "/osmosis.gamm.v1beta1.Pool",
      "address": "osmo1mw0ac6rwlp5r8wapwk3zs6g29h8fcscxqakdzw9emkne6c8wjp9q0t3v8t",
      "id": "1",
      "pool_params": {
        "swap_fee": "0.001000000000000000",
        "exit_fee": "0.001000000000000000",
        "smooth_weight_change_params": null
      },
      "future_pool_governor": "",
      "total_shares": {
        "denom": "gamm/pool/1",
        "amount": "100000000000000000000"
      },
      "pool_assets": [
        {
          "token": {
            "denom": "uatom",
            "amount": "2000"
          },
          "weight": "2147483648"
        },
        {
          "token": {
            "denom": "uosmo",
            "amount": "1000"
          },
          "weight": "1073741824"
        }
      ],
      "total_weight": "3221225472"
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```
And after about a minute (maximum) quasar should be updated as well so checking the http://localhost:1311/quasarlabs/quasarnode/qoracle/osmosis/pools should response the following results:
```json
{
  "pools": [
    {
      "pool_info": {
        "address": "osmo1mw0ac6rwlp5r8wapwk3zs6g29h8fcscxqakdzw9emkne6c8wjp9q0t3v8t",
        "id": "1",
        "poolParams": {
          "swapFee": "0.001000000000000000",
          "exitFee": "0.001000000000000000",
          "smoothWeightChangeParams": null
        },
        "future_pool_governor": "",
        "totalShares": {
          "denom": "gamm/pool/1",
          "amount": "100000000000000000000"
        },
        "poolAssets": [
          {
            "token": {
              "denom": "uatom",
              "amount": "2000"
            },
            "weight": "2147483648"
          },
          {
            "token": {
              "denom": "uosmo",
              "amount": "1000"
            },
            "weight": "1073741824"
          }
        ],
        "totalWeight": "3221225472"
      },
      "metrics": {
        "apy": "228679.752403732192854600",
        "tvl": "0.029779015000000000"
      }
    }
  ],
  "pagination": {
    "next_key": null,
    "total": "1"
  }
}
```
Note that the `apy` and `tvl` values may be different in your case depending on the prices fetched from bandchain.