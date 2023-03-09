package suite

import (
	"context"
	"encoding/json"
	"github.com/cosmos/cosmos-sdk/crypto/keyring"
	"github.com/cosmos/cosmos-sdk/types"
	"github.com/strangelove-ventures/interchaintest/v4"
	"github.com/strangelove-ventures/interchaintest/v4/chain/cosmos"
	"time"

	"github.com/icza/dyno"
	"github.com/strangelove-ventures/interchaintest/v4/ibc"
)

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
func modifyGenesisICAModule(enabled bool, allowMsgs []string) genesisModifiers {
	return func(gen any) (any, error) {
		v := map[string]any{
			"host_enabled":   enabled,
			"allow_messages": allowMsgs,
		}
		err := dyno.Set(gen, v, "app_state", "interchainaccounts", "host_genesis_state", "params")
		return gen, err
	}
}

// modifyGenesisICQModule sets the params of ICQ module.
func modifyGenesisICQModule(enabled bool, allowQueries []string) genesisModifiers {
	return func(gen any) (any, error) {
		v := map[string]any{
			"host_enabled":  enabled,
			"allow_queries": allowQueries,
		}
		err := dyno.Set(gen, v, "app_state", "interchainquery", "params")
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
	genesisWallets := []ibc.WalletAmount{
		{
			Address: authority.Address,
			Denom:   "uoro",
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
			Denom:   chainCfg.Denom,
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
