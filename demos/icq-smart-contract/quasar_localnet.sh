#!/bin/sh

## This script helps to create a basic version of the quasar chain genesis file for development purposes.
## However it will need some manual modifications before you start the chain to incorporate the custom fields.

# Configure variables
BINARY=quasard
HOME_QSR=$HOME/.quasarnode
CHAIN_ID=quasar
ALICE="edge victory hurry slight dog exit company bike hill erupt shield aspect turkey retreat stairs summer sadness crush absorb draft viable orphan chuckle exhibit"
BOB="harvest ill mean warfare gospel slide tragic palace model excess surprise distance voyage change bus grant special artwork win width group dwarf today jar"
USER_1="guard cream sadness conduct invite crumble clock pudding hole grit liar hotel maid produce squeeze return argue turtle know drive eight casino maze host"
USER_2="fuel obscure melt april direct second usual hair leave hobby beef bacon solid drum used law mercy worry fat super must ritual bring faculty"
RELAYER_ACC="$(cat ./keys/qsr.key)"

ALICE_GENESIS_COINS=20000token,200000000stake,1000000000uqsr
BOB_GENESIS_COINS=10000token,100000000stake,1000000000uqsr
USER_1_GENESIS_COINS=10000000000stake,10000000000uqsr
USER_2_GENESIS_COINS=10000000000stake,10000000000uqsr
RELAYER_ACC_GENESIS_COINS=10000000uqsr,10000000000stake

# Remove previous setup
rm -rf $HOME_QSR
 
$BINARY init $CHAIN_ID --chain-id $CHAIN_ID

# Bootstrap the quasar local network with single node

echo $ALICE  | $BINARY keys add alice --keyring-backend test --recover
echo $BOB    | $BINARY keys add bob   --keyring-backend test --recover
echo $USER_1 | $BINARY keys add user1 --keyring-backend test --recover
echo $USER_2 | $BINARY keys add user2 --keyring-backend test --recover
echo $RELAYER_ACC | $BINARY keys add relayer_acc --keyring-backend test --recover
$BINARY add-genesis-account $($BINARY keys show alice --keyring-backend test -a) $ALICE_GENESIS_COINS
$BINARY add-genesis-account $($BINARY keys show bob   --keyring-backend test -a) $BOB_GENESIS_COINS
$BINARY add-genesis-account $($BINARY keys show user1 --keyring-backend test -a) $USER_1_GENESIS_COINS
$BINARY add-genesis-account $($BINARY keys show user2 --keyring-backend test -a) $USER_2_GENESIS_COINS
$BINARY add-genesis-account $($BINARY keys show relayer_acc --keyring-backend test -a) $RELAYER_ACC_GENESIS_COINS
$BINARY gentx alice 100000000uqsr --chain-id $CHAIN_ID --keyring-backend test
$BINARY collect-gentxs

# Check platform
platform='unknown'
unamestr=`uname`
if [ "$unamestr" = 'Linux' ]; then
   platform='linux'
elif [ "$unamestr" = 'Darwin' ]; then
  platform='mac'
fi

if [ $platform = 'linux' ]; then
	sed -i 's/enable = false/enable = true/g' $HOME_QSR/config/app.toml
	sed -i 's/swagger = false/swagger = true/g' $HOME_QSR/config/app.toml
	sed -i 's/minimum-gas-prices = ""/minimum-gas-prices = "0uqsr"/g' $HOME_QSR/config/app.toml
	sed -i 's+laddr = "tcp://127.0.0.1:26657"+laddr = "tcp://127.0.0.1:26659"+g' $HOME_QSR/config/config.toml
	sed -i 's+node = "tcp://localhost:26657"+node = "tcp://localhost:26659"+g' $HOME_QSR/config/client.toml
	sed -i 's+laddr = "tcp://0.0.0.0:26656"+laddr = "tcp://0.0.0.0:26661"+g' $HOME_QSR/config/config.toml
	sed -i 's+pprof_laddr = "localhost:6060"+pprof_laddr = "localhost:6061"+g' $HOME_QSR/config/config.toml
	sed -i 's+address = "0.0.0.0:9090"+address = "0.0.0.0:9095"+g' $HOME_QSR/config/app.toml
	sed -i 's+address = "0.0.0.0:9091"+address = "0.0.0.0:8091"+g' $HOME_QSR/config/app.toml
	sed -i 's+address = "tcp://0.0.0.0:1317"+address = "tcp://0.0.0.0:1311"+g' $HOME_QSR/config/app.toml
	sed -i 's+address = ":8080"+address = ":8081"+g' $HOME_QSR/config/app.toml
elif [ $platform = 'mac' ]; then
	sed -i'.original' -e 's/enable = false/enable = true/g' $HOME_QSR/config/app.toml
	sed -i'.original' -e 's/swagger = false/swagger = true/g' $HOME_QSR/config/app.toml
	sed -i'.original' -e 's/minimum-gas-prices = ""/minimum-gas-prices = "0uqsr"/g' $HOME_QSR/config/app.toml
	sed -i'.original' -e 's+laddr = "tcp://127.0.0.1:26657"+laddr = "tcp://127.0.0.1:26659"+g' $HOME_QSR/config/config.toml
	sed -i'.original' -e 's+node = "tcp://localhost:26657"+node = "tcp://localhost:26659"+g' $HOME_QSR/config/client.toml
	sed -i'.original' -e 's+laddr = "tcp://0.0.0.0:26656"+laddr = "tcp://0.0.0.0:26661"+g' $HOME_QSR/config/config.toml
	sed -i'.original' -e 's+pprof_laddr = "localhost:6060"+pprof_laddr = "localhost:6061"+g' $HOME_QSR/config/config.toml
	sed -i'.original' -e 's+address = "0.0.0.0:9090"+address = "0.0.0.0:9095"+g' $HOME_QSR/config/app.toml
	sed -i'.original' -e 's+address = "0.0.0.0:9091"+address = "0.0.0.0:8091"+g' $HOME_QSR/config/app.toml
	sed -i'.original' -e 's+address = "tcp://0.0.0.0:1317"+address = "tcp://0.0.0.0:1311"+g' $HOME_QSR/config/app.toml
	sed -i'.original' -e 's+address = ":8080"+address = ":8081"+g' $HOME_QSR/config/app.toml
else
	echo "only linux and mac platforms are supported, if you are using other platforms you should probably improve this script."
	exit 1
	sed -i '' 's/enable = false/enable = true/g' $HOME_QSR/config/app.toml
	sed -i '' 's/swagger = false/swagger = true/g' $HOME_QSR/config/app.toml
fi

cp $HOME_QSR/config/genesis.json $HOME_QSR/config/genesis_original.json
cat $HOME_QSR/config/genesis_original.json |
  jq '.app_state.crisis.constant_fee.denom="uqsr"' |
  jq '.app_state.staking.params.bond_denom="uqsr"' |
  jq '.app_state.mint.params.mint_denom="uqsr"' |
  jq '.app_state.gov.deposit_params.min_deposit=[{denom:"uqsr",amount:"1"}]' |
  jq '.app_state.gov.voting_params.voting_period="30s"' |
  jq '.app_state.gov.tally_params={quorum:"0.000000000000000001",threshold:"0.5",veto_threshold:"0.334"}' |
  jq ".app_state.qoracle.params.oracleAccounts=\"$($BINARY keys show alice --keyring-backend test -a)\"" |
  jq '.app_state.orion = {
      "lpPosition": null,
      "lpStat": null,
      "params": {
        "destination_chain_id": "osmosis",
        "enabled": true,
        "lp_epoch_id": "minute",
        "mgmt_fee_per": "0.003000000000000000",
        "osmosis_local_info": {
          "chain_id": "osmosis",
          "connection_id": "connection-1",
          "local_zone_id": "osmosis-1"
        },
        "perf_fee_per": "0.020000000000000000",
        "white_listed_pools": [
          1,
          2,
          3
        ]
      },
      "rewardCollection": null
    }' |
  jq '.app_state.intergamm = {
      "params": {
        "dest_to_intr_zone_map": {
          "osmosis-01": "cosmos"
        },
      "intr_rcvrs": [
          {
            "next_zone_route_map": {
              "osmosis-01": {
                "chain_id": "osmosis",
                "connection_id": "connection-1",
                "local_zone_id": "osmosis-1",
                "transfer_channel_id": "channel-1"
              },
              "osmosis-02": {
                "chain_id": "osmosis2",
                "connection_id": "connection-2",
                "local_zone_id": "osmosis-2",
                "transfer_channel_id": "channel-2"
              }
            },
            "rcvr_address": "cosmos1ppkxa0hxak05tcqq3338k76xqxy2qse96uelcu",
            "zone_info": {
              "chain_id": "cosmos",
              "connection_id": "connection-2"
            }
          }
        ],
        "osmo_token_transfer_channels": {
          "osmosis": "channel-1",
          "osmosis-test": "channel-1"
        }
      }
    }' |
  jq '.app_state.qbank = {
      "claimableRewards": [],
      "depositInfos": [],
      "params": {
        "enabled": true,
        "min_orion_epoch_denom_dollar_deposit": "100.000000000000000000",
        "orion_epoch_identifier": "minute",
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
    }' >  $HOME_QSR/config/genesis.json

# Start
$BINARY start --home $HOME_QSR >> ./logs/quasar_localnet.log 2>&1
