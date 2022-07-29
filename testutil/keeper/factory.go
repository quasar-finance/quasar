package keeper

import (
	"github.com/quasarlabs/quasarnode/app"
	oriontypes "github.com/quasarlabs/quasarnode/x/orion/types"
	"github.com/cosmos/cosmos-sdk/store"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	"github.com/tendermint/starport/starport/pkg/cosmoscmd"
	tmdb "github.com/tendermint/tm-db"
)

// Structure holding storage context for initializing test keepers
type KeeperFactory struct {
	DB             *tmdb.MemDB
	StateStore     store.CommitMultiStore
	Ctx            sdk.Context
	EncodingConfig cosmoscmd.EncodingConfig
}

// Create an KeeperFactory with in memory database and default codecs
func NewKeeperFactory(
	db *tmdb.MemDB,
	stateStore store.CommitMultiStore,
	ctx sdk.Context,
	encodingConfig cosmoscmd.EncodingConfig,
) KeeperFactory {
	return KeeperFactory{
		DB:             db,
		StateStore:     stateStore,
		Ctx:            ctx,
		EncodingConfig: encodingConfig,
	}
}

// TestModuleAccountPerms returns module account permissions for testing
func (kf KeeperFactory) TestModuleAccountPerms() map[string][]string {
	moduleAccPerms := app.GetMaccPerms()
	moduleAccPerms[oriontypes.CreateOrionRewardGloablMaccName()] = []string{authtypes.Minter, authtypes.Burner, authtypes.Staking}
	return moduleAccPerms
}

// BlockedModuleAccountAddrs returns all the app's module account addresses that are active
func (kf KeeperFactory) BlockedModuleAccountAddrs(maccPerms map[string][]string) map[string]bool {
	modAccAddrs := make(map[string]bool)
	for acc := range maccPerms {
		modAccAddrs[authtypes.NewModuleAddress(acc).String()] = true
	}

	return modAccAddrs
}
