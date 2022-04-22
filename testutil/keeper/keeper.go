package keeper

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/abag/quasarnode/app"
	epochskeeper "github.com/abag/quasarnode/x/epochs/keeper"
	intergammkeeper "github.com/abag/quasarnode/x/intergamm/keeper"
	orionkeeper "github.com/abag/quasarnode/x/orion/keeper"
	oriontypes "github.com/abag/quasarnode/x/orion/types"
	qbankkeeper "github.com/abag/quasarnode/x/qbank/keeper"
	qoraclekeeper "github.com/abag/quasarnode/x/qoracle/keeper"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	capabilitykeeper "github.com/cosmos/cosmos-sdk/x/capability/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	icacontrollerkeeper "github.com/cosmos/ibc-go/v3/modules/apps/27-interchain-accounts/controller/keeper"
)

// TestKeepers structure holds keepers that will be used for testing
type TestKeepers struct {
	T                testing.TB
	Ctx              sdk.Context
	ParamsKeeper     paramskeeper.Keeper
	EpochsKeeper     *epochskeeper.Keeper
	AccountKeeper    authkeeper.AccountKeeper
	BankKeeper       bankkeeper.Keeper
	CapabilityKeeper capabilitykeeper.Keeper
	QbankKeeper      qbankkeeper.Keeper
	QoracleKeeper    qoraclekeeper.Keeper
	InterGammKeeper  intergammkeeper.Keeper
	OrionKeeper      orionkeeper.Keeper
}

// testModuleAccountPerms returns module account permissions for testing
func testModuleAccountPerms() map[string][]string {
	moduleAccPerms := app.GetMaccPerms()
	moduleAccPerms[oriontypes.CreateOrionRewardGloablMaccName()] = []string{authtypes.Minter, authtypes.Burner, authtypes.Staking}
	return moduleAccPerms
}

// blockModuleAccountAddrs returns all the app's module account addresses that are active
func blockModuleAccountAddrs(maccPerms map[string][]string) map[string]bool {
	modAccAddrs := make(map[string]bool)
	for acc := range maccPerms {
		modAccAddrs[authtypes.NewModuleAddress(acc).String()] = true
	}

	return modAccAddrs
}

// NewTestSetup creates and initializes all the keepers for testing purposes
func NewTestSetup(t testing.TB) TestKeepers {
	initializer := newInitializer()

	maccPerms := testModuleAccountPerms()
	blockedMaccAddresses := blockModuleAccountAddrs(maccPerms)

	paramsKeeper := initializer.ParamsKeeper()
	epochsKeeper := initializer.EpochsKeeper(paramsKeeper)
	accountKeeper := initializer.AccountKeeper(paramsKeeper, maccPerms)
	bankKeeper := initializer.BankKeeper(paramsKeeper, accountKeeper, blockedMaccAddresses)
	capabilityKeeper := initializer.CapabilityKeeper()
	qbankKeeper := initializer.QbankKeeper(paramsKeeper, bankKeeper)
	qoracleKeeper := initializer.QoracleKeeper(paramsKeeper)
	intergammKeeper := initializer.IntergammKeeper(paramsKeeper, capabilityKeeper, icacontrollerkeeper.Keeper{})
	orionKeeper := initializer.OrionKeeper(paramsKeeper, accountKeeper, bankKeeper, qbankKeeper, qoracleKeeper, intergammKeeper)

	// Note: the relative order of LoadLatestVersion and Set*DefaultParams is important.
	// Setting params before loading stores causes store does not exist error.
	// LoadLatestVersion must not be called again after setting params, as reloading stores clears set params.

	require.NoError(t, initializer.StateStore.LoadLatestVersion())

	initializer.SetQbankDefaultParams(qbankKeeper)
	initializer.SetQoracleDefaultParams(qoracleKeeper)
	initializer.SetIntergammDefaultParams(intergammKeeper)
	initializer.SetOrionDefaultParams(orionKeeper)

	return TestKeepers{
		T:                t,
		Ctx:              initializer.Ctx,
		ParamsKeeper:     paramsKeeper,
		EpochsKeeper:     epochsKeeper,
		AccountKeeper:    accountKeeper,
		BankKeeper:       bankKeeper,
		CapabilityKeeper: capabilityKeeper,
		QbankKeeper:      qbankKeeper,
		QoracleKeeper:    qoracleKeeper,
		InterGammKeeper:  intergammKeeper,
		OrionKeeper:      orionKeeper,
	}
}

func (tk TestKeepers) GetEpochsKeeper() (sdk.Context, *epochskeeper.Keeper) {
	return tk.Ctx, tk.EpochsKeeper
}

func (tk TestKeepers) GetQbankKeeper() (sdk.Context, qbankkeeper.Keeper) {
	return tk.Ctx, tk.QbankKeeper
}

func (tk TestKeepers) GetQoracleKeeper() (sdk.Context, qoraclekeeper.Keeper) {
	return tk.Ctx, tk.QoracleKeeper
}

func (tk TestKeepers) GetInterGammKeeper() (sdk.Context, intergammkeeper.Keeper) {
	return tk.Ctx, tk.InterGammKeeper
}

func (tk TestKeepers) GetOrionKeeper() (sdk.Context, orionkeeper.Keeper) {
	return tk.Ctx, tk.OrionKeeper
}
