#!/bin/sh

# Configure variables
BINARY=gaiad
HOME_COSMOSHUB=$HOME/.gaia
CHAIN_ID=cosmos
ALICE="blur service enlist into false certain replace arrow until fatal glory mule design into dilemma palm helmet upper behave gallery into afford candy exercise"
BOB="lucky surface version conduct ketchup cash unfair rival shoulder example demand upset deny tilt very clinic tribe rather renew skirt naive sweet box chicken"
USER_1="guard cream sadness conduct invite crumble clock pudding hole grit liar hotel maid produce squeeze return argue turtle know drive eight casino maze host"
USER_2="fuel obscure melt april direct second usual hair leave hobby beef bacon solid drum used law mercy worry fat super must ritual bring faculty"
RELAYER_ACC="$(cat ./keys/gaia.key)"

ALICE_GENESIS_COINS=200000000uatom,2000000000stake
BOB_GENESIS_COINS=10000000uatom,1000000000stake
USER_1_GENESIS_COINS=10000000000stake,10000000000uatom,10000000000uusd
USER_2_GENESIS_COINS=10000000000stake,10000000000uatom
RELAYER_ACC_GENESIS_COINS=10000000uatom,10000000000stake

# Remove previous setup
rm -rf $HOME_COSMOSHUB

# Bootstrap
$BINARY init $CHAIN_ID --chain-id $CHAIN_ID --home $HOME_COSMOSHUB
echo $ALICE | $BINARY keys add alice --keyring-backend test --recover
echo $BOB | $BINARY keys add bob --keyring-backend test --recover
echo $USER_1 | $BINARY keys add user1 --keyring-backend test --recover
echo $USER_2 | $BINARY keys add user2 --keyring-backend test --recover
echo $RELAYER_ACC | $BINARY keys add relayer_acc --keyring-backend test --recover
$BINARY add-genesis-account $($BINARY keys show alice --keyring-backend test -a) $ALICE_GENESIS_COINS --home $HOME_COSMOSHUB
$BINARY add-genesis-account $($BINARY keys show bob --keyring-backend test -a) $BOB_GENESIS_COINS --home $HOME_COSMOSHUB
$BINARY add-genesis-account $($BINARY keys show user1 --keyring-backend test -a) $USER_1_GENESIS_COINS --home $HOME_COSMOSHUB
$BINARY add-genesis-account $($BINARY keys show user2 --keyring-backend test -a) $USER_2_GENESIS_COINS --home $HOME_COSMOSHUB
$BINARY add-genesis-account $($BINARY keys show relayer_acc --keyring-backend test -a) $RELAYER_ACC_GENESIS_COINS --home $HOME_COSMOSHUB
$BINARY gentx alice 100000000uatom --chain-id $CHAIN_ID --keyring-backend test --home $HOME_COSMOSHUB
#$BINARY gentx bob 100000000uatom --chain-id $CHAIN_ID --keyring-backend test --home $HOME_COSMOSHUB
$BINARY collect-gentxs

# Check platform
platform='unknown'
unamestr=$(uname)
if [ "$unamestr" = 'Linux' ]; then
   	platform='linux'
elif [ "$unamestr" = 'Darwin' ]; then
	platform='macos'
fi

if [ $platform = 'linux' ]; then
	sed -i 's/enable = false/enable = true/g' $HOME_COSMOSHUB/config/app.toml
	sed -i 's/swagger = false/swagger = true/g' $HOME_COSMOSHUB/config/app.toml
	sed -i 's/minimum-gas-prices = ""/minimum-gas-prices = "0uatom"/g' $HOME_COSMOSHUB/config/app.toml
	sed -i 's+laddr = "tcp://127.0.0.1:26657"+laddr = "tcp://127.0.0.1:26669"+g' $HOME_COSMOSHUB/config/config.toml
	sed -i 's+node = "tcp://localhost:26657"+node = "tcp://localhost:26669"+g' $HOME_COSMOSHUB/config/client.toml
	sed -i 's+laddr = "tcp://0.0.0.0:26656"+laddr = "tcp://0.0.0.0:26663"+g' $HOME_COSMOSHUB/config/config.toml
	sed -i 's+pprof_laddr = "localhost:6060"+pprof_laddr = "localhost:6063"+g' $HOME_COSMOSHUB/config/config.toml
	sed -i 's+address = "0.0.0.0:9090"+address = "0.0.0.0:9097"+g' $HOME_COSMOSHUB/config/app.toml
	sed -i 's+address = "0.0.0.0:9091"+address = "0.0.0.0:8093"+g' $HOME_COSMOSHUB/config/app.toml
	sed -i 's+address = "tcp://0.0.0.0:1317"+address = "tcp://0.0.0.0:1313"+g' $HOME_COSMOSHUB/config/app.toml
	sed -i 's+address = ":8080"+address = ":8083"+g' $HOME_COSMOSHUB/config/app.toml
elif [ $platform = 'macos' ]; then
	sed -i'.original' -e 's/enable = false/enable = true/g' $HOME_COSMOSHUB/config/app.toml
	sed -i'.original' -e 's/swagger = false/swagger = true/g' $HOME_COSMOSHUB/config/app.toml
	sed -i'.original' -e 's/minimum-gas-prices = ""/minimum-gas-prices = "0uatom"/g' $HOME_COSMOSHUB/config/app.toml
	sed -i'.original' -e 's+laddr = "tcp://127.0.0.1:26657"+laddr = "tcp://127.0.0.1:26669"+g' $HOME_COSMOSHUB/config/config.toml
	sed -i'.original' -e 's+node = "tcp://localhost:26657"+node = "tcp://localhost:26669"+g' $HOME_COSMOSHUB/config/client.toml
	sed -i'.original' -e 's+laddr = "tcp://0.0.0.0:26656"+laddr = "tcp://0.0.0.0:26663"+g' $HOME_COSMOSHUB/config/config.toml
	sed -i'.original' -e 's+pprof_laddr = "localhost:6060"+pprof_laddr = "localhost:6063"+g' $HOME_COSMOSHUB/config/config.toml
	sed -i'.original' -e 's+address = "0.0.0.0:9090"+address = "0.0.0.0:9097"+g' $HOME_COSMOSHUB/config/app.toml
	sed -i'.original' -e 's+address = "0.0.0.0:9091"+address = "0.0.0.0:8093"+g' $HOME_COSMOSHUB/config/app.toml
	sed -i'.original' -e 's+address = "tcp://0.0.0.0:1317"+address = "tcp://0.0.0.0:1313"+g' $HOME_COSMOSHUB/config/app.toml
	sed -i'.original' -e 's+address = ":8080"+address = ":8083"+g' $HOME_COSMOSHUB/config/app.toml
else
	echo "only linux and macos platforms are supported, if you are using other platforms you should probably improve this script."

	exit 1
	sed -i '' 's/enable = false/enable = true/g' $HOME_COSMOSHUB/config/app.toml
	sed -i '' 's/swagger = false/swagger = true/g' $HOME_COSMOSHUB/config/app.toml
fi

cp $HOME_COSMOSHUB/config/genesis.json $HOME_COSMOSHUB/config/genesis_original.json
cat $HOME_COSMOSHUB/config/genesis_original.json |
	jq '.app_state.crisis.constant_fee.denom="uatom"' |
	jq '.app_state.staking.params.bond_denom="uatom"' |
	jq '.app_state.mint.params.mint_denom="uatom"' |
	jq '.app_state.liquidity.params.pool_creation_fee=[{denom:"uatom",amount:"1"}]' |
	jq '.app_state.gov.deposit_params.min_deposit=[{denom:"uatom",amount:"1"}]' |
	jq '.app_state.gov.voting_params.voting_period="30s"' |
	jq '.app_state.gov.tally_params={quorum:"0.000000000000000001",threshold:"0.5",veto_threshold:"0.334"}' \
		>$HOME_COSMOSHUB/config/genesis.json

# Start
$BINARY start --home $HOME_COSMOSHUB >> ./logs/cosmos_localnet.log 2>&1
