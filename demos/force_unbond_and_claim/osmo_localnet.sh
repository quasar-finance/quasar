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

ALICE_GENESIS_COINS=1000000000000000000000uosmo,2000000000stake,1000000000000fakestake
BOB_GENESIS_COINS=10000000000000uosmo,10000000000000stake,10000000000000fakestake,100000000000000usdc
USER_1_GENESIS_COINS=10000000000stake,10000000000uosmo
USER_2_GENESIS_COINS=10000000000stake,10000000000uosmo
RELAYER_ACC_GENESIS_COINS=1000000000000000000000uosmo,10000000000stake

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
  jq '.app_state.mint = {
      minter: {
        epoch_provisions: "0.000000000000000000"
      },
      params: {
        distribution_proportions: {
          community_pool: "0.100000000000000000",
          developer_rewards: "0.200000000000000000",
          pool_incentives: "0.300000000000000000",
          staking: "0.400000000000000000"
        },
        epoch_identifier: "day",
        genesis_epoch_provisions: "5000000.000000000000000000",
        mint_denom: "uosmo",
        minting_rewards_distribution_start_epoch: "0",
        reduction_factor: "0.500000000000000000",
        reduction_period_in_epochs: "156",
        weighted_developer_rewards_receivers: []
    }
  }' |
  jq '.app_state.poolincentives = {
    "any_pool_to_internal_gauges": null,
    "concentrated_pool_to_no_lock_gauges": null,
    "distr_info": {
      "records": [],
      "total_weight": "0"
    },
    "lockable_durations": [
      "3600s",
      "10800s",
      "25200s"
    ],
    "params": {
      "minted_denom": "stake"
    }
  }' |
  jq '.app_state.txfees.basedenom="uosmo"' |
  jq '.app_state.gov.deposit_params.min_deposit=[{denom:"uosmo",amount:"1"}]' |
  jq '.app_state.gov.voting_params.voting_period="30s"' |
  jq '.app_state.gov.tally_params={quorum:"0.000000000000000001",threshold:"0.5",veto_threshold:"0.334"}' |
  jq '.app_state.interchainaccounts = {
    host_genesis_state: {
      port: "icahost",
      params: {
        host_enabled: true,
        allow_messages: [
          "/ibc.applications.transfer.v1.MsgTransfer",
          "/cosmos.bank.v1beta1.MsgSend",
          "/cosmos.staking.v1beta1.MsgDelegate",
          "/cosmos.staking.v1beta1.MsgBeginRedelegate",
          "/cosmos.staking.v1beta1.MsgCreateValidator",
          "/cosmos.staking.v1beta1.MsgEditValidator",
          "/cosmos.staking.v1beta1.MsgUndelegate",
          "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward",
          "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress",
          "/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission",
          "/cosmos.distribution.v1beta1.MsgFundCommunityPool",
          "/cosmos.gov.v1beta1.MsgVote",
          "/osmosis.gamm.v1beta1.MsgJoinPool",
          "/osmosis.gamm.v1beta1.MsgExitPool",
          "/osmosis.gamm.v1beta1.MsgSwapExactAmountIn",
          "/osmosis.gamm.v1beta1.MsgSwapExactAmountOut",
          "/osmosis.gamm.v1beta1.MsgJoinSwapExternAmountIn",
          "/osmosis.gamm.v1beta1.MsgJoinSwapShareAmountOut",
          "/osmosis.gamm.v1beta1.MsgExitSwapExternAmountOut",
          "/osmosis.gamm.v1beta1.MsgExitSwapShareAmountIn",
          "/osmosis.lockup.MsgBeginUnlocking",
          "/osmosis.lockup.MsgLockTokens",
          "/osmosis.superfluid.MsgSuperfluidUnbondLock"
       ]
      }
    }
  }' |
  jq '.app_state.interchainquery = {
    host_port: "icqhost",
    params: {
      host_enabled: true,
      allow_queries: [
          "/ibc.applications.transfer.v1.Query/DenomTrace",
          "/cosmos.auth.v1beta1.Query/Account",
          "/cosmos.auth.v1beta1.Query/Params",
          "/cosmos.bank.v1beta1.Query/Balance",
          "/cosmos.bank.v1beta1.Query/DenomMetadata",
          "/cosmos.bank.v1beta1.Query/Params",
          "/cosmos.bank.v1beta1.Query/SupplyOf",
          "/cosmos.distribution.v1beta1.Query/Params",
          "/cosmos.distribution.v1beta1.Query/DelegatorWithdrawAddress",
          "/cosmos.distribution.v1beta1.Query/ValidatorCommission",
          "/cosmos.gov.v1beta1.Query/Deposit",
          "/cosmos.gov.v1beta1.Query/Params",
          "/cosmos.gov.v1beta1.Query/Vote",
          "/cosmos.slashing.v1beta1.Query/Params",
          "/cosmos.slashing.v1beta1.Query/SigningInfo",
          "/cosmos.staking.v1beta1.Query/Delegation",
          "/cosmos.staking.v1beta1.Query/Params",
          "/cosmos.staking.v1beta1.Query/Validator",
          "/osmosis.epochs.v1beta1.Query/EpochInfos",
          "/osmosis.epochs.v1beta1.Query/CurrentEpoch",
          "/osmosis.gamm.v1beta1.Query/NumPools",
          "/osmosis.gamm.v1beta1.Query/TotalLiquidity",
          "/osmosis.gamm.v1beta1.Query/Pool",
          "/osmosis.gamm.v1beta1.Query/PoolParams",
          "/osmosis.gamm.v1beta1.Query/TotalPoolLiquidity",
          "/osmosis.gamm.v1beta1.Query/TotalShares",
          "/osmosis.gamm.v1beta1.Query/CalcJoinPoolShares",
          "/osmosis.gamm.v1beta1.Query/CalcExitPoolCoinsFromShares",
          "/osmosis.gamm.v1beta1.Query/CalcJoinPoolNoSwapShares",
          "/osmosis.gamm.v2.Query/SpotPrice",
          "/osmosis.gamm.v1beta1.Query/PoolType",
          "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountIn",
          "/osmosis.gamm.v1beta1.Query/EstimateSwapExactAmountOut",
          "/osmosis.incentives.Query/ModuleToDistributeCoins",
          "/osmosis.incentives.Query/LockableDurations",
          "/osmosis.lockup.Query/ModuleBalance",
          "/osmosis.lockup.Query/ModuleLockedAmount",
          "/osmosis.lockup.Query/AccountUnlockableCoins",
          "/osmosis.lockup.Query/AccountUnlockingCoins",
          "/osmosis.lockup.Query/LockedDenom",
          "/osmosis.lockup.Query/LockedByID",
          "/osmosis.lockup.Query/NextLockID",
          "/osmosis.mint.v1beta1.Query/EpochProvisions",
          "/osmosis.mint.v1beta1.Query/Params",
          "/osmosis.poolincentives.v1beta1.Query/GaugeIds",
          "/osmosis.superfluid.Query/Params",
          "/osmosis.superfluid.Query/AssetType",
          "/osmosis.superfluid.Query/AllAssets",
          "/osmosis.superfluid.Query/AssetMultiplier",
          "/osmosis.poolmanager.v1beta1.Query/NumPools",
          "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn",
          "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut",
          "/osmosis.txfees.v1beta1.Query/FeeTokens",
          "/osmosis.txfees.v1beta1.Query/DenomSpotPrice",
          "/osmosis.txfees.v1beta1.Query/DenomPoolId",
          "/osmosis.txfees.v1beta1.Query/BaseDenom",
          "/osmosis.tokenfactory.v1beta1.Query/Params",
          "/osmosis.tokenfactory.v1beta1.Query/DenomAuthorityMetadata",
          "/osmosis.twap.v1beta1.Query/ArithmeticTwap",
          "/osmosis.twap.v1beta1.Query/ArithmeticTwapToNow",
          "/osmosis.twap.v1beta1.Query/GeometricTwap",
          "/osmosis.twap.v1beta1.Query/GeometricTwapToNow",
          "/osmosis.twap.v1beta1.Query/Params",
          "/osmosis.downtimedetector.v1beta1.Query/RecoveredSinceDowntimeOfLength"
      ]
    }
  }' \
    >$HOME_OSMOSIS/config/genesis.json

# Start
$BINARY start --home $HOME_OSMOSIS >>./logs/osmo_localnet.log 2>&1