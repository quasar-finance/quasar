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

// TODO need to find better place for this whole file (didn't use it from config dir as it was creating import cycles

const (
	// DefaultNumValidators is the number of validator nodes deployed for each chain
	DefaultNumValidators = 1
	// DefaultNumNodes Number of full nodes deployed for each chain
	DefaultNumNodes = 0

	// VotingPeriod is the duration in which proposals in gov module are open for voting
	VotingPeriod = time.Second * 10

	// Default relayer path names for quasar <-> cosmos link
	Quasar2CosmosPath = "quasar-cosmos"
	// Default relayer path names for cosmos <-> osmosis link
	Cosmos2OsmosisPath = "cosmos-osmosis"
	// Default relayer path names for quasar <-> osmosis link
	Quasar2OsmosisPath = "quasar-osmosis"
)

const (
	authorityKeyName = "authority"

	ownerKeyName        = "owner"
	ownerKeyName1       = "pppppppppppppp"
	newOwnerKeyName     = "new_owner"
	masterMinterKeyName = "masterminter"
	bondTestKeyName     = "bond_test"
	bondTestKeyName1    = "bond_test_1"
	bondTestKeyName2    = "bond_test_2"
	bondTestKeyName3    = "bond_test_3"
	bondTestKeyName4    = "bond_test_4"
	bondTestKeyName5    = "bond_test_5"
	bondTestKeyName6    = "bond_test_6"
	bondTestKeyName7    = "bond_test_7"
)

// genesisModifiers takes genesis as an unmarshaled "any" value and, modifies it and returns the "any" back.
type genesisModifiers func(gen any) (any, error)
type preGenesis func(gen any) (any, error)

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

	authority := interchaintest.BuildWallet(kr, authorityKeyName, chainCfg)
	masterMinter := interchaintest.BuildWallet(kr, masterMinterKeyName, chainCfg)
	owner := interchaintest.BuildWallet(kr, ownerKeyName, chainCfg)
	newOwner := interchaintest.BuildWallet(kr, newOwnerKeyName, chainCfg)
	bondTest := interchaintest.BuildWallet(kr, bondTestKeyName, chainCfg)
	bondTest1 := interchaintest.BuildWallet(kr, bondTestKeyName1, chainCfg)
	bondTest2 := interchaintest.BuildWallet(kr, bondTestKeyName2, chainCfg)
	bondTest3 := interchaintest.BuildWallet(kr, bondTestKeyName3, chainCfg)
	bondTest4 := interchaintest.BuildWallet(kr, bondTestKeyName4, chainCfg)
	bondTest5 := interchaintest.BuildWallet(kr, bondTestKeyName5, chainCfg)
	bondTest6 := interchaintest.BuildWallet(kr, bondTestKeyName6, chainCfg)
	bondTest7 := interchaintest.BuildWallet(kr, bondTestKeyName7, chainCfg)

	err := val.RecoverKey(ctx, authorityKeyName, authority.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, ownerKeyName, owner.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, newOwnerKeyName, newOwner.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, masterMinterKeyName, masterMinter.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, bondTestKeyName, bondTest.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, bondTestKeyName1, bondTest1.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, bondTestKeyName2, bondTest2.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, bondTestKeyName3, bondTest3.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, bondTestKeyName4, bondTest4.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, bondTestKeyName5, bondTest5.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, bondTestKeyName6, bondTest6.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, bondTestKeyName7, bondTest7.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}

	genesisWallets := []ibc.WalletAmount{
		{
			Address: authority.Address,
			Denom:   chainCfg.Denom,
			Amount:  10_000_000_000_000_000,
		},
		{
			Address: owner.Address,
			Denom:   "uayy",
			Amount:  10_000_000_000_000_000,
		},
		{
			Address: newOwner.Address,
			Denom:   chainCfg.Denom,
			Amount:  10_000_000_000_000_000,
		},
		{
			Address: masterMinter.Address,
			Denom:   chainCfg.Denom,
			Amount:  10_000_000_000_000_000,
		},
		{
			Address: bondTest.Address,
			Denom:   "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			Amount:  100_000_000_000,
		},
		{
			Address: bondTest1.Address,
			Denom:   "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			Amount:  100_000_000_000,
		},
		{
			Address: bondTest2.Address,
			Denom:   "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			Amount:  100_000_000_000,
		},
		{
			Address: bondTest3.Address,
			Denom:   "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			Amount:  100_000_000_000,
		},
		{
			Address: bondTest4.Address,
			Denom:   "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			Amount:  100_000_000_000,
		},
		{
			Address: bondTest5.Address,
			Denom:   "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			Amount:  100_000_000_000,
		},
		{
			Address: bondTest6.Address,
			Denom:   "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			Amount:  100_000_000_000,
		},
		{
			Address: bondTest7.Address,
			Denom:   "ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518",
			Amount:  100_000_000_000,
		},
	}

	for _, wallet := range genesisWallets {
		err = val.AddGenesisAccount(ctx, wallet.Address, []types.Coin{types.NewCoin(wallet.Denom, types.NewIntFromUint64(uint64(wallet.Amount)))})
		if err != nil {
			return Accounts{}, err
		}
	}
	return Accounts{
		Authority:    authority,
		Owner:        owner,
		NewOwner:     newOwner,
		MasterMinter: masterMinter,
		BondTest:     bondTest,
		BondTest1:    bondTest1,
		BondTest2:    bondTest2,
		BondTest3:    bondTest3,
		BondTest4:    bondTest4,
		BondTest5:    bondTest5,
		BondTest6:    bondTest6,
		BondTest7:    bondTest7,
	}, nil
}

func osmosisPreGenesis(ctx context.Context, val *cosmos.ChainNode) (Accounts, error) {
	chainCfg := val.Chain.Config()

	kr := keyring.NewInMemory()

	authority := interchaintest.BuildWallet(kr, authorityKeyName, chainCfg)

	masterMinter := interchaintest.BuildWallet(kr, masterMinterKeyName, chainCfg)
	owner := interchaintest.BuildWallet(kr, ownerKeyName1, chainCfg)
	newOwner := interchaintest.BuildWallet(kr, newOwnerKeyName, chainCfg)

	err := val.RecoverKey(ctx, authorityKeyName, authority.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, ownerKeyName1, owner.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, newOwnerKeyName, newOwner.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	err = val.RecoverKey(ctx, masterMinterKeyName, masterMinter.Mnemonic)
	if err != nil {
		return Accounts{}, err
	}
	genesisWallets := []ibc.WalletAmount{
		{
			Address: authority.Address,
			Denom:   "fakestake",
			Amount:  10_000_000_000_000_000,
		},
		{
			Address: owner.Address,
			Denom:   "stake1",
			Amount:  10_000_000_000_000_000,
		},
		{
			Address: newOwner.Address,
			Denom:   "usdc",
			Amount:  10_000_000_000_000_000,
		},
		{
			Address: masterMinter.Address,
			Denom:   chainCfg.Denom,
			Amount:  10_000_000_000_000_000,
		},
	}

	for _, wallet := range genesisWallets {
		err = val.AddGenesisAccount(ctx, wallet.Address, []types.Coin{types.NewCoin(wallet.Denom, types.NewIntFromUint64(uint64(wallet.Amount)))})
		if err != nil {
			return Accounts{}, err
		}
	}
	return Accounts{
		Authority:    authority,
		Owner:        owner,
		NewOwner:     newOwner,
		MasterMinter: masterMinter,
	}, nil
}
