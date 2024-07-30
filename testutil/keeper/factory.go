package keeper

import (
	"cosmossdk.io/store"
	dbm "github.com/cosmos/cosmos-db"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"

	"github.com/quasarlabs/quasarnode/app"
	"github.com/quasarlabs/quasarnode/app/params"
)

// KeeperFactory Structure holding storage context for initializing test keepers.
type KeeperFactory struct {
	DB             *dbm.MemDB
	StateStore     store.CommitMultiStore
	Ctx            sdk.Context
	EncodingConfig params.EncodingConfig
}

// NewKeeperFactory Creates with in memory database and default codecs.
func NewKeeperFactory(
	db *dbm.MemDB,
	stateStore store.CommitMultiStore,
	ctx sdk.Context,
	encodingConfig params.EncodingConfig,
) KeeperFactory {
	return KeeperFactory{
		DB:             db,
		StateStore:     stateStore,
		Ctx:            ctx,
		EncodingConfig: encodingConfig,
	}
}

// TestModuleAccountPerms returns module account permissions for testing.
func (kf KeeperFactory) TestModuleAccountPerms() map[string][]string {
	moduleAccPerms := app.ModuleAccountPermissions
	return moduleAccPerms
}

// BlockedModuleAccountAddrs returns all the app's module account addresses that are active.
func (kf KeeperFactory) BlockedModuleAccountAddrs(maccPerms map[string][]string) map[string]bool {
	modAccAddrs := make(map[string]bool)
	for acc := range maccPerms {
		modAccAddrs[authtypes.NewModuleAddress(acc).String()] = true
	}

	return modAccAddrs
}
