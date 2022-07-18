# Configure variables
export BINARY=gaiad
export HOME_COSMOSHUB=$HOME/.gaiad
export CHAIN_ID=cosmoshub
export VALIDATOR_1="struggle panic room apology luggage game screen wing want lazy famous eight robot picture wrap act uphold grab away proud music danger naive opinion"
#export VALIDATOR_2="guard damp hub tomorrow rotate upgrade unable sail similar royal cave heavy shield license note glimpse include submit spell burst viable duty they curtain"
export USER_1="guard cream sadness conduct invite crumble clock pudding hole grit liar hotel maid produce squeeze return argue turtle know drive eight casino maze host"
export USER_2="fuel obscure melt april direct second usual hair leave hobby beef bacon solid drum used law mercy worry fat super must ritual bring faculty"
export VALIDATOR_1_GENESIS_COINS=10000000000stake,10000000000uatom,10000000000uusd
#export VALIDATOR_2_GENESIS_COINS=10000000000stake,10000000000uatom,10000000000uusd
export USER_1_GENESIS_COINS=10000000000stake,10000000000uatom,10000000000uusd
export USER_2_GENESIS_COINS=10000000000stake,10000000000uatom

# Remove previous setup
rm -rf $HOME_COSMOSHUB

# Bootstrap
$BINARY init $CHAIN_ID --chain-id $CHAIN_ID
echo $VALIDATOR_1 | $BINARY keys add val1 --keyring-backend test --recover
#echo $VALIDATOR_2 | $BINARY keys add val2 --keyring-backend test --recover
echo $USER_1 | $BINARY keys add user1 --keyring-backend test --recover
echo $USER_2 | $BINARY keys add user2 --keyring-backend test --recover
$BINARY add-genesis-account $($BINARY keys show val1 --keyring-backend test -a) $VALIDATOR_1_GENESIS_COINS
#$BINARY add-genesis-account $($BINARY keys show val2 --keyring-backend test -a) $VALIDATOR_2_GENESIS_COINS
$BINARY add-genesis-account $($BINARY keys show user1 --keyring-backend test -a) $USER_1_GENESIS_COINS
$BINARY add-genesis-account $($BINARY keys show user2 --keyring-backend test -a) $USER_2_GENESIS_COINS
$BINARY gentx val1 100000000stake --chain-id $CHAIN_ID --keyring-backend test
#$BINARY gentx val2 100000000stake --chain-id $CHAIN_ID --keyring-backend test
$BINARY collect-gentxs

# Check platform
platform='unknown'
unamestr=`uname`
if [ "$unamestr" = 'Linux' ]; then
   platform='linux'
fi

if [ $platform = 'linux' ]; then
	sed -i 's/enable = false/enable = true/g' $HOME_COSMOSHUB/config/app.toml
	sed -i 's/swagger = false/swagger = true/g' $HOME_COSMOSHUB/config/app.toml
	sed -i 's%"amount": "10000000"%"amount": "1"%g' $HOME_COSMOSHUB/config/genesis.json
	sed -i 's%"quorum": "0.334000000000000000",%"quorum": "0.000000000000000001",%g' $HOME_COSMOSHUB/config/genesis.json
	sed -i 's%"threshold": "0.500000000000000000",%"threshold": "0.000000000000000001",%g' $HOME_COSMOSHUB/config/genesis.json
	sed -i 's%"voting_period": "172800s"%"voting_period": "30s"%g' $HOME_COSMOSHUB/config/genesis.json
else
	sed -i '' 's/enable = false/enable = true/g' $HOME_COSMOSHUB/config/app.toml
	sed -i '' 's/swagger = false/swagger = true/g' $HOME_COSMOSHUB/config/app.toml
	sed -i '' 's%"amount": "10000000"%"amount": "1"%g' $HOME_COSMOSHUB/config/genesis.json
	sed -i '' 's%"quorum": "0.334000000000000000",%"quorum": "0.000000000000000001",%g' $HOME_COSMOSHUB/config/genesis.json
	sed -i '' 's%"threshold": "0.500000000000000000",%"threshold": "0.000000000000000001",%g' $HOME_COSMOSHUB/config/genesis.json
	sed -i '' 's%"voting_period": "172800s"%"voting_period": "30s"%g' $HOME_COSMOSHUB/config/genesis.json
fi

# Start
$BINARY start
