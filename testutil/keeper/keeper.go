package keeper

import (
	"testing"

	epochskeeper "github.com/abag/quasarnode/x/epochs/keeper"
	oriontypes "github.com/abag/quasarnode/x/orion/types"
	qbankkeeper "github.com/abag/quasarnode/x/qbank/keeper"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"github.com/stretchr/testify/require"
)

// Structure holding keepers that will be used for testing
type TestKeepers struct {
	T             testing.TB
	Ctx           sdk.Context
	ParamsKeeper  paramskeeper.Keeper
	EpochsKeeper  *epochskeeper.Keeper
	AccountKeeper authkeeper.AccountKeeper
	BankKeeper    bankkeeper.Keeper
	QbankKeeper   qbankkeeper.Keeper
}

// return module account permissions for testing
func testModuleAccountPerms() map[string][]string {
	return map[string][]string{
		qbanktypes.ModuleName: {authtypes.Minter, authtypes.Burner, authtypes.Staking},
		oriontypes.ModuleName: {authtypes.Minter, authtypes.Burner, authtypes.Staking},
		oriontypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Days_7):   nil,
		oriontypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Days_21):  nil,
		oriontypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Months_1): nil,
		oriontypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Months_3): nil,
	}
}

// blockModuleAccountAddrs returns all the app's module account addresses that are active
func blockModuleAccountAddrs(maccPerms map[string][]string) map[string]bool {
	modAccAddrs := make(map[string]bool)
	for acc := range maccPerms {
		modAccAddrs[authtypes.NewModuleAddress(acc).String()] = true
	}

	return modAccAddrs
}

// Create and initialize all the keepers for testing purposes
func NewTestSetup(t testing.TB) TestKeepers {
	initializer := newInitializer()

	maccPerms := testModuleAccountPerms()
	blockedMaccAddresses := blockModuleAccountAddrs(maccPerms)

	paramsKeeper := initializer.ParamsKeeper()
	epochsKeeper := initializer.EpochsKeeper(paramsKeeper)
	accountKeeper := initializer.AccountKeeper(paramsKeeper, maccPerms)
	bankKeeper := initializer.BankKeeper(paramsKeeper, accountKeeper, blockedMaccAddresses)
	qbankkeeper := initializer.QbankKeeper(paramsKeeper, bankKeeper)

	require.NoError(t, initializer.StateStore.LoadLatestVersion())

	return TestKeepers{
		T:             t,
		Ctx:           initializer.Ctx,
		ParamsKeeper:  paramsKeeper,
		EpochsKeeper:  epochsKeeper,
		AccountKeeper: accountKeeper,
		BankKeeper:    bankKeeper,
		QbankKeeper:   qbankkeeper,
	}
}

func (tk TestKeepers) GetEpochsKeeper() (sdk.Context, *epochskeeper.Keeper) {
	return tk.Ctx, tk.EpochsKeeper
}

func (tk TestKeepers) GetQbankKeeper() (sdk.Context, qbankkeeper.Keeper) {
	return tk.Ctx, tk.QbankKeeper
}
