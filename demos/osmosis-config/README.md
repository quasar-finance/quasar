# Prerequisites
1. `go` v1.19
2. `hermes` v1.0.0
3.  osmosis repo with async-icq or quasar repo sdk45 ( feature/sdk45_chainupdates or main-sdk45)

# Setup
Install the main binary of this source code with the following command:
```bash
make install
```
Clone osmosis v15 release candidate branch. and install the osmosisd binary with the following commands:
```bash

clone the tag/v15.0.0-rc0 , https://github.com/osmosis-labs/osmosis/releases/tag/v15.0.0-rc0

cd ./osmosis

make install
```

# Starting Quasar node
Run the following commands to start a single node of quasar in local machine with the preset of parameters needed for this demo:
```bash
cd ./demos/osmosis-config

./startup.sh
```
After this you should see block logs written in the stdout of your terminal.

# Starting Osmosis node
Run the following commands to start a single node of osmosis in local machine with the preset of parameters needed for this demo:
```bash
cd ./demos/osmosis-config

./startup.sh
```
After this you should see block logs written in the stdout of your terminal.

# Config and Start the Hermes Relayer

Before running the relayer checkout http://localhost:1311/quasarlabs/quasarnode/qoracle/osmosis/state by  the response should be in the below format. 
Values must be zero initially.

```json
{
  "params_request_state": {
    "packet_sequence": "14",
    "acknowledged": true,
    "failed": false,
    "updated_at_height": "99"
  },
  "incentivized_pools_state": {
    "packet_sequence": "13",
    "acknowledged": true,
    "failed": false,
    "updated_at_height": "99"
  },
  "pools_state": {
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
## Update the chain params of osmosis in quasar run the following command. This will be happening automatically in the epoch hooks on every configured interval.

- For testing this interval is set to 1 minute. In the live scenario it could be done once per day. 

 
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
Note that the `apy` and `tvl will be zero at this point. As in this version of the codebase we don't have integrated any stable price oracle yet.


## TO DO QUICK TESTING for param update ; just run , run_test.sh and verify  chain_param after 2-3 minutes
`
curl  http://localhost:1311/quasarlabs/quasarnode/qoracle/osmosis/chain_params
` 
- Values will not be zero. 