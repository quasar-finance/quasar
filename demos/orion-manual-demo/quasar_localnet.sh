## This script helps to create a basic version of the quasar chain genesis file for development purposes.
## However it will need some manual modifications before you start the chain to incorporate the custom fields.


# Configure variables
export BINARY=quasarnoded
export HOME_QSR=$HOME/.quasarnode
export CHAIN_ID=quasar
export VALIDATOR_1="edge victory hurry slight dog exit company bike hill erupt shield aspect turkey retreat stairs summer sadness crush absorb draft viable orphan chuckle exhibit"
#export VALIDATOR_2="harvest ill mean warfare gospel slide tragic palace model excess surprise distance voyage change bus grant special artwork win width group dwarf today jar"
export USER_1="guard cream sadness conduct invite crumble clock pudding hole grit liar hotel maid produce squeeze return argue turtle know drive eight casino maze host"
export USER_2="fuel obscure melt april direct second usual hair leave hobby beef bacon solid drum used law mercy worry fat super must ritual bring faculty"
export VALIDATOR_1_GENESIS_COINS=10000000000stake,10000000000uqsr
#export VALIDATOR_2_GENESIS_COINS=10000000000stake,10000000000uqsr
export USER_1_GENESIS_COINS=10000000000stake,10000000000uatom
export USER_2_GENESIS_COINS=10000000000stake,10000000000uatom


# Remove previous setup
rm -rf $HOME_QSR
 
$BINARY init $CHAIN_ID --chain-id $CHAIN_ID

# Bootstrap the quasar local network with single node

#$BINARY init $CHAIN_ID --chain-id $CHAIN_ID
echo $VALIDATOR_1 | $BINARY keys add val1 --keyring-backend test --recover
#echo $VALIDATOR_2 | $BINARY keys add val2 --keyring-backend test --recover
echo $USER_1 | $BINARY keys add user1 --keyring-backend test --recover
echo $USER_2 | $BINARY keys add user2 --keyring-backend test --recover
$BINARY add-genesis-account $($BINARY keys show val1 --keyring-backend test -a) $VALIDATOR_1_GENESIS_COINS
#$BINARY add-genesis-account $($BINARY keys show val2 --keyring-backend test -a) $VALIDATOR_2_GENESIS_COINS
$BINARY add-genesis-account $($BINARY keys show user1 --keyring-backend test -a) $USER_1_GENESIS_COINS
$BINARY add-genesis-account $($BINARY keys show user2 --keyring-backend test -a) $USER_2_GENESIS_COINS
$BINARY gentx val1 100000000stake --chain-id $CHAIN_ID --keyring-backend test
# $BINARY gentx val2 100000000stake --chain-id $CHAIN_ID --keyring-backend test
$BINARY collect-gentxs

# Check platform
platform='unknown'
unamestr=`uname`
if [ "$unamestr" = 'Linux' ]; then
   platform='linux'
fi

if [ $platform = 'linux' ]; then
	sed -i 's/enable = false/enable = true/g' $HOME_QSR/config/app.toml
	sed -i 's/swagger = false/swagger = true/g' $HOME_QSR/config/app.toml
	sed -i 's+laddr = "tcp://127.0.0.1:26657"+laddr = "tcp://127.0.0.1:26650"+g' $HOME_QSR/config/config.toml
	sed -i 's+node = "tcp://localhost:26657"+node = "tcp://localhost:26650"+g' $HOME_QSR/config/client.toml
	sed -i 's+laddr = "tcp://0.0.0.0:26656"+laddr = "tcp://0.0.0.0:26651"+g' $HOME_QSR/config/config.toml
	sed -i 's+pprof_laddr = "localhost:6060"+pprof_laddr = "localhost:6050"+g' $HOME_QSR/config/config.toml
	sed -i 's+address = "0.0.0.0:9090"+address = "0.0.0.0:9050"+g' $HOME_QSR/config/app.toml
	sed -i 's+address = "0.0.0.0:9091"+address = "0.0.0.0:9051"+g' $HOME_QSR/config/app.toml
	sed -i 's+address = "tcp://0.0.0.0:1317"+address = "tcp://0.0.0.0:1350"+g' $HOME_QSR/config/app.toml
	sed -i 's+address = ":8080"+address = ":8050"+g' $HOME_QSR/config/app.toml
	sed -i 's%"amount": "10000000"%"amount": "1"%g' $HOME_QSR/config/genesis.json
	sed -i 's%"quorum": "0.334000000000000000",%"quorum": "0.000000000000000001",%g' $HOME_QSR/config/genesis.json
	sed -i 's%"threshold": "0.500000000000000000",%"threshold": "0.000000000000000001",%g' $HOME_QSR/config/genesis.json
	sed -i 's%"voting_period": "172800s"%"voting_period": "30s"%g' $HOME_QSR/config/genesis.json
else
	echo "only linux platforms are supported, if you are using other platforms you should probably improve this script."
	exit 1
	sed -i '' 's/enable = false/enable = true/g' $HOME_QSR/config/app.toml
	sed -i '' 's/swagger = false/swagger = true/g' $HOME_QSR/config/app.toml
 	sed -i '' 's%"amount": "10000000"%"amount": "1"%g' $HOME_QSR/config/genesis.json
	sed -i '' 's%"quorum": "0.334000000000000000",%"quorum": "0.000000000000000001",%g' $HOME_QSR/config/genesis.json
	sed -i '' 's%"threshold": "0.500000000000000000",%"threshold": "0.000000000000000001",%g' $HOME_QSR/config/genesis.json
	sed -i '' 's%"voting_period": "172800s"%"voting_period": "30s"%g' $HOME_QSR/config/genesis.json
fi

cp $HOME_QSR/config/genesis.json $HOME_QSR/config/genesis_original.json
cat $HOME_QSR/config/genesis.json | jq '.app_state.orion = {
      "lpPosition": null,
      "lpStat": null,
      "params": {
        "destination_chain_id": "osmosis",
        "enabled": false,
        "lp_epoch_id": "day",
        "mgmt_fee_per": "0.003000000000000000",
        "osmosis_local_info": {
          "chain_id": "osmosis",
          "connection_id": "connection-01",
          "local_zone_id": "osmosis-01"
        },
        "perf_fee_per": "0.020000000000000000",
        "white_listed_pools": [
          1,
          2,
          3
        ]
      },
      "rewardCollection": null
    }' > $HOME_QSR/config/genesis1.json

cat $HOME_QSR/config/genesis1.json | jq '.app_state.intergamm = {
      "params": {
        "dest_to_intr_zone_map": {
          "osmosis-01": "cosmos"
        },
      "intr_rcvrs": [
          {
            "next_zone_route_map": {
              "osmosis-01": {
                "chain_id": "osmosis",
                "connection_id": "connection-01",
                "local_zone_id": "osmosis-01",
                "transfer_channel_id": "channel-01"
              },
              "osmosis-02": {
                "chain_id": "osmosis2",
                "connection_id": "connection-02",
                "local_zone_id": "osmosis-02",
                "transfer_channel_id": "channel-02"
              }
            },
            "rcvr_address": "cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu",
            "zone_info": {
              "chain_id": "cosmos",
              "connection_id": "connection-02"
            }
          }
        ],
        "osmo_token_transfer_channels": {
          "osmosis": "channel-1",
          "osmosis-test": "channel-1"
        }
      }
    }' > $HOME_QSR/config/genesis2.json

cat $HOME_QSR/config/genesis2.json | jq '.app_state.qbank = {
      "claimableRewards": [],
      "depositInfos": [],
      "params": {
        "enabled": true,
        "min_orion_epoch_denom_dollar_deposit": "100.000000000000000000",
        "orion_epoch_identifier": "day",
        "white_listed_denoms_in_orion": [
          {
            "onehop_osmo": "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
            "onehop_quasar": "ibc/BE1BB42D4BE3C30D50B68D7C41DB4DFCE9678E8EF8C539F6E6A9345048894FCC",
            "origin_name": "uatom"
          }
        ]
      },
      "totalClaimedRewards": [],
      "totalDeposits": [],
      "totalWithdraws": [],
      "withdrawables": []
    }' > $HOME_QSR/config/genesis3.json

cp $HOME_QSR/config/genesis3.json $HOME_QSR/config/genesis.json

# Start
$BINARY start
