#!/bin/sh

# Configure variables
BINARY=osmosisd
HOME_OSMOSIS=$HOME/.osmosis
CHAIN_ID=osmosis
ALICE="cruise scene law sea push expose scorpion wire trick repair wave quote task dose inner denial alpha favorite certain blouse motion flash split lunch"
BOB="lizard garlic canyon winner cheese tent drip task because ecology clay bridge junk critic track artefact gather harsh deliver unit vacant earth diesel stool"
USER_1="guard cream sadness conduct invite crumble clock pudding hole grit liar hotel maid produce squeeze return argue turtle know drive eight casino maze host"
USER_2="fuel obscure melt april direct second usual hair leave hobby beef bacon solid drum used law mercy worry fat super must ritual bring faculty"
RELAYER_ACC="$(cat ./keys/osmo.key)"

ALICE_GENESIS_COINS=20000000uosmo,2000000000stake,1000000000000fakestake
BOB_GENESIS_COINS=10000000000000uosmo,1000000000stake,1000000000000fakestake,100000000000000usdc
USER_1_GENESIS_COINS=10000000000stake,10000000000uosmo
USER_2_GENESIS_COINS=10000000000stake,10000000000uosmo
RELAYER_ACC_GENESIS_COINS=10000000uosmo,10000000000stake

echo $HOME_OSMOSIS

rm -rf $HOME_OSMOSIS
# Bootstrap
$BINARY init $CHAIN_ID --chain-id $CHAIN_ID --home $HOME_OSMOSIS

echo $ALICE | $BINARY keys add alice --keyring-backend test --recover --home $HOME_OSMOSIS
echo $BOB | $BINARY keys add bob --keyring-backend test --recover --home $HOME_OSMOSIS
echo $USER_1 | $BINARY keys add user1 --keyring-backend test --recover --home $HOME_OSMOSIS
echo $USER_2 | $BINARY keys add user2 --keyring-backend test --recover --home $HOME_OSMOSIS
echo $RELAYER_ACC | $BINARY keys add relayer_acc --keyring-backend test --recover --home $HOME_OSMOSIS
$BINARY add-genesis-account $($BINARY keys show alice --keyring-backend test -a --home $HOME_OSMOSIS) $ALICE_GENESIS_COINS --home $HOME_OSMOSIS
$BINARY add-genesis-account $($BINARY keys show bob --keyring-backend test -a --home $HOME_OSMOSIS) $BOB_GENESIS_COINS --home $HOME_OSMOSIS
$BINARY add-genesis-account $($BINARY keys show user1 --keyring-backend test -a --home $HOME_OSMOSIS) $USER_1_GENESIS_COINS --home $HOME_OSMOSIS
$BINARY add-genesis-account $($BINARY keys show user2 --keyring-backend test -a --home $HOME_OSMOSIS) $USER_2_GENESIS_COINS --home $HOME_OSMOSIS
$BINARY add-genesis-account $($BINARY keys show relayer_acc --keyring-backend test -a --home $HOME_OSMOSIS) $RELAYER_ACC_GENESIS_COINS --home $HOME_OSMOSIS
$BINARY add-genesis-account osmo15td5pfjkmfn8d6l4t8dc67l3apgt9epw4ct298 $RELAYER_ACC_GENESIS_COINS --home $HOME_OSMOSIS
$BINARY gentx alice 10000000uosmo --chain-id $CHAIN_ID --keyring-backend test --home $HOME_OSMOSIS
$BINARY collect-gentxs --home $HOME_OSMOSIS

# Check platform
platform='unknown'
unamestr=$(uname)
if [ "$unamestr" = 'Linux' ]; then
  platform='linux'
elif [ "$unamestr" = 'Darwin' ]; then
  platform='macos'
fi

if [ $platform = 'linux' ]; then
  sed -i 's/enable = false/enable = true/g' $HOME_OSMOSIS/config/app.toml
  sed -i 's/swagger = false/swagger = true/g' $HOME_OSMOSIS/config/app.toml
  sed -i 's/minimum-gas-prices = ""/minimum-gas-prices = "0uosmo"/g' $HOME_OSMOSIS/config/app.toml
  sed -i 's+laddr = "tcp://127.0.0.1:26657"+laddr = "tcp://127.0.0.1:26679"+g' $HOME_OSMOSIS/config/config.toml
  sed -i 's+node = "tcp://localhost:26657"+node = "tcp://localhost:26679"+g' $HOME_OSMOSIS/config/client.toml
  sed -i 's+laddr = "tcp://0.0.0.0:26656"+laddr = "tcp://0.0.0.0:26662"+g' $HOME_OSMOSIS/config/config.toml
  sed -i 's+pprof_laddr = "localhost:6060"+pprof_laddr = "localhost:6062"+g' $HOME_OSMOSIS/config/config.toml
  sed -i 's+address = "0.0.0.0:9090"+address = "0.0.0.0:9096"+g' $HOME_OSMOSIS/config/app.toml
  sed -i 's+address = "0.0.0.0:9091"+address = "0.0.0.0:8092"+g' $HOME_OSMOSIS/config/app.toml
  sed -i 's+address = "tcp://0.0.0.0:1317"+address = "tcp://0.0.0.0:1312"+g' $HOME_OSMOSIS/config/app.toml
  sed -i 's+address = ":8080"+address = ":8082"+g' $HOME_OSMOSIS/config/app.toml
elif [ $platform = 'macos' ]; then
  sed -i'.original' -e 's/enable = false/enable = true/g' $HOME_OSMOSIS/config/app.toml
  sed -i'.original' -e 's/swagger = false/swagger = true/g' $HOME_OSMOSIS/config/app.toml
  sed -i'.original' -e 's/minimum-gas-prices = ""/minimum-gas-prices = "0uosmo"/g' $HOME_OSMOSIS/config/app.toml
  sed -i'.original' -e 's+laddr = "tcp://127.0.0.1:26657"+laddr = "tcp://127.0.0.1:26679"+g' $HOME_OSMOSIS/config/config.toml
  sed -i'.original' -e 's+node = "tcp://localhost:26657"+node = "tcp://localhost:26679"+g' $HOME_OSMOSIS/config/client.toml
  sed -i'.original' -e 's+laddr = "tcp://0.0.0.0:26656"+laddr = "tcp://0.0.0.0:26662"+g' $HOME_OSMOSIS/config/config.toml
  sed -i'.original' -e 's+pprof_laddr = "localhost:6060"+pprof_laddr = "localhost:6062"+g' $HOME_OSMOSIS/config/config.toml
  sed -i'.original' -e 's+address = "0.0.0.0:9090"+address = "0.0.0.0:9096"+g' $HOME_OSMOSIS/config/app.toml
  sed -i'.original' -e 's+address = "0.0.0.0:9091"+address = "0.0.0.0:8092"+g' $HOME_OSMOSIS/config/app.toml
  sed -i'.original' -e 's+address = "tcp://0.0.0.0:1317"+address = "tcp://0.0.0.0:1312"+g' $HOME_OSMOSIS/config/app.toml
  sed -i'.original' -e 's+address = ":8080"+address = ":8082"+g' $HOME_OSMOSIS/config/app.toml
else
  echo "only linux and macos platforms are supported, if you are using other platforms you should probably improve this script."

  exit 1
  sed -i '' 's/enable = false/enable = true/g' $HOME_OSMOSIS/config/app.toml
  sed -i '' 's/swagger = false/swagger = true/g' $HOME_OSMOSIS/config/app.toml
fi

cp $HOME_OSMOSIS/config/genesis.json $HOME_OSMOSIS/config/genesis_original.json
cat $HOME_OSMOSIS/config/genesis_original.json |

    jq '.app_state.crisis.constant_fee.denom="uosmo"' |
    jq '.app_state.staking.params.bond_denom="uosmo"' |
    jq '.app_state.txfees.basedenom="uosmo"' |
    jq '.app_state.gov.deposit_params.min_deposit=[{denom:"uosmo",amount:"1"}]' |
    jq '.app_state.gov.voting_params.voting_period="30s"' |
    jq '.app_state.gov.tally_params={quorum:"0.000000000000000001",threshold:"0.5",veto_threshold:"0.334"}' >$HOME_OSMOSIS/config/genesis.json


# Start
$BINARY start --home $HOME_OSMOSIS >>./logs/osmo_localnet.log 2>&1 &
#$BINARY start --home $HOME_OSMOSIS