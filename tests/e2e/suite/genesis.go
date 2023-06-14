package suite

import (
	"context"
	"encoding/json"
	"time"

	"github.com/cosmos/cosmos-sdk/crypto/keyring"
	"github.com/cosmos/cosmos-sdk/types"
	"github.com/icza/dyno"
	"github.com/strangelove-ventures/interchaintest/v4"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
)

var (
	ICQAllowedQueries = []string{
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
		"/osmosis.gamm.v1beta1.Query/PoolType",
		"/osmosis.gamm.v2.Query/SpotPrice",
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
		"/osmosis.downtimedetector.v1beta1.Query/RecoveredSinceDowntimeOfLength",
	}
	ICAAllowedMessages = []string{
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
		"/osmosis.superfluid.MsgSuperfluidUnbondLock",
	}
)

// genesisModifiers takes genesis as an unmarshaled "any" value and, modifies it and returns the "any" back.
type genesisModifiers func(gen any) (any, error)

// modifyGenesis is a helper function that chains multiple genesisModifiers funcs in order to make
// chain configuration easier and a lot cleaner.
func modifyGenesis(mods ...genesisModifiers) func(cc ibc.ChainConfig, genbz []byte) ([]byte, error) {
	return func(cc ibc.ChainConfig, genbz []byte) ([]byte, error) {
		var gen any
		err := json.Unmarshal(genbz, &gen)
		if err != nil {
			return nil, err
		}

		for _, mod := range mods {
			gen, err = mod(gen)
			if err != nil {
				return nil, err
			}
		}

		return json.Marshal(gen)
	}
}

// modifyGenesisSetVotingPeriod sets the governance module voting period to the given duration.
func modifyGenesisSetVotingPeriod(period time.Duration) genesisModifiers {
	return func(gen any) (any, error) {
		err := dyno.Set(gen, VotingPeriod.String(), "app_state", "gov", "voting_params", "voting_period")
		return gen, err
	}
}

// modifyGenesisICAModule sets the params of ICA module.
func modifyGenesisICAModule(enabled bool, allowMsgs []string, Port string) genesisModifiers {
	return func(gen any) (any, error) {
		v := map[string]any{
			//"active_channels":     map[string]any{},
			//"interchain_accounts": map[string]any{},
			"port": Port,
			"params": map[string]any{
				"host_enabled":   enabled,
				"allow_messages": allowMsgs,
			},
		}
		err := dyno.Set(gen, v, "app_state", "interchainaccounts", "host_genesis_state")
		return gen, err
	}
}

// modifyGenesisICQModule sets the params of ICQ module.
func modifyGenesisICQModule(enabled bool, allowQueries []string, hostPort string) genesisModifiers {
	return func(gen any) (any, error) {
		v := map[string]any{
			"host_port": hostPort,
			"params": map[string]any{
				"host_enabled":  enabled,
				"allow_queries": allowQueries,
			},
		}
		err := dyno.Set(gen, v, "app_state", "interchainquery")
		return gen, err
	}
}

func modifyMintModule() genesisModifiers {
	return func(gen any) (any, error) {
		v := map[string]any{
			"minter": map[string]any{
				"epoch_provisions": "0.000000000000000000",
			},
			"params": map[string]any{
				"distribution_proportions": map[string]any{
					"community_pool":    "0.100000000000000000",
					"developer_rewards": "0.200000000000000000",
					"pool_incentives":   "0.300000000000000000",
					"staking":           "0.400000000000000000",
				},
				"epoch_identifier":                         "day",
				"genesis_epoch_provisions":                 "5000000.000000000000000000",
				"mint_denom":                               "uosmo",
				"minting_rewards_distribution_start_epoch": "0",
				"reduction_factor":                         "0.500000000000000000",
				"reduction_period_in_epochs":               "156",
				"weighted_developer_rewards_receivers":     []string{},
			},
			"reduction_started_epoch": "0",
		}
		err := dyno.Set(gen, v, "app_state", "mint")
		return gen, err
	}
}

func modifyIncentivesModule() genesisModifiers {
	return func(gen any) (any, error) {
		v := map[string]any{
			"last_gauge_id": "0",
			"lockable_durations": []string{
				"1s",
				"120s",
				"180s",
				"240s",
			},
			"params": map[string]any{
				"distr_epoch_identifier": "day",
			},
		}
		err := dyno.Set(gen, v, "app_state", "incentives")
		return gen, err
	}
}

func modifyPoolIncentivesModule() genesisModifiers {
	return func(gen any) (any, error) {
		v := map[string]any{
			"distr_info": map[string]any{
				"records": []any{
					map[string]any{
						"gauge_id": "0",
						"weight":   "10000",
					},
					map[string]any{
						"gauge_id": "1",
						"weight":   "1000",
					},
					map[string]any{
						"gauge_id": "2",
						"weight":   "100",
					},
				},
				"total_weight": "11100",
			},
			"lockable_durations": []string{
				"120s",
				"180s",
				"240s",
			},
			"params": map[string]any{
				"minted_denom": "uosmo",
			},
			"pool_to_gauges": nil,
		}
		err := dyno.Set(gen, v, "app_state", "poolincentives")
		return gen, err
	}
}

func quasarPreGenesis(ctx context.Context, val *cosmos.ChainNode) (Accounts, error) {
	chainCfg := val.Chain.Config()

	kr := keyring.NewInMemory()

	accTreasury := interchaintest.BuildWallet(kr, AuthorityKeyName, chainCfg)
	err := val.RecoverKey(ctx, AuthorityKeyName, accTreasury.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}

	genesisCoins := []types.Coin{
		{
			Denom:  chainCfg.Denom,
			Amount: types.NewIntFromUint64(100_000_000_000_000_000),
		},
		{
			Denom:  "uayy",
			Amount: types.NewIntFromUint64(100_000_000_000_000_000),
		},
	}

	err = val.AddGenesisAccount(ctx, accTreasury.Address, genesisCoins)
	if err != nil {
		return Accounts{}, err
	}

	return Accounts{
		Treasury: accTreasury,
	}, nil
}

func osmosisPreGenesis(ctx context.Context, val *cosmos.ChainNode) (Accounts, error) {
	chainCfg := val.Chain.Config()

	kr := keyring.NewInMemory()

	accTreasury := interchaintest.BuildWallet(kr, AuthorityKeyName, chainCfg)
	err := val.RecoverKey(ctx, AuthorityKeyName, accTreasury.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}

	genesisCoins := []types.Coin{
		{
			Denom:  "fakestake",
			Amount: types.NewIntFromUint64(100_000_000_000_000_000),
		},
		{
			Denom:  "stake1",
			Amount: types.NewIntFromUint64(100_000_000_000_000_000),
		},
		{
			Denom:  "usdc",
			Amount: types.NewIntFromUint64(100_000_000_000_000_000),
		},
		{
			Denom:  chainCfg.Denom,
			Amount: types.NewIntFromUint64(100_000_000_000_000_000),
		},
	}

	err = val.AddGenesisAccount(ctx, accTreasury.Address, genesisCoins)
	if err != nil {
		return Accounts{}, err
	}

	return Accounts{
		Treasury: accTreasury,
	}, nil
}

func addPreGenesis(ctx context.Context, val *cosmos.ChainNode, genesisCoins types.Coins) (AccountsNew, error) {
	// todo implement a general pre genesis function
	chainCfg := val.Chain.Config()

	kr := keyring.NewInMemory()

	accTreasury := interchaintest.BuildWallet(kr, AuthorityKeyName, chainCfg)
	err := val.RecoverKey(ctx, AuthorityKeyName, accTreasury.Mnemonic)
	if err != nil {
		return nil, err
	}

	err = val.AddGenesisAccount(ctx, accTreasury.Address, genesisCoins)
	if err != nil {
		return nil, err
	}

	accounts := make(map[string]*ibc.Wallet)
	accounts[AuthorityKeyName] = &accTreasury

	return accounts, nil
}
