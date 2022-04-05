package keeper

import (
	"testing"

	osmolpvtypes "github.com/abag/quasarnode/x/osmolpv/types"
	qbankkeeper "github.com/abag/quasarnode/x/qbank/keeper"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	bankkeeper "github.com/cosmos/cosmos-sdk/x/bank/keeper"
	paramskeeper "github.com/cosmos/cosmos-sdk/x/params/keeper"
	"github.com/stretchr/testify/require"
)

type TestKeepers struct {
	T             testing.TB
	Ctx           sdk.Context
	ParamsKeeper  paramskeeper.Keeper
	AccountKeeper authkeeper.AccountKeeper
	BankKeeper    bankkeeper.Keeper
	QBankKeeper   qbankkeeper.Keeper
}

func moduleAccountPerms() map[string][]string {
	return map[string][]string{
		qbanktypes.ModuleName: {authtypes.Minter, authtypes.Burner, authtypes.Staking},
		osmolpvtypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Days_7):   nil,
		osmolpvtypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Days_21):  nil,
		osmolpvtypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Months_1): nil,
		osmolpvtypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Months_3): nil,
	}
}

func NameToAddress(name string) string {
	return authtypes.NewModuleAddress(name).String()
}

// BlockModuleAccountAddrs returns all the app's module account addresses.
func BlockModuleAccountAddrs(maccPerms map[string][]string) map[string]bool {
	modAccAddrs := make(map[string]bool)
	for acc := range maccPerms {
		modAccAddrs[NameToAddress(acc)] = true
	}

	return modAccAddrs
}

func NewTestSetup(t testing.TB) TestKeepers {
	initializer := newInitializer()

	maccPerms := moduleAccountPerms()
	blockedMaccAddresses := BlockModuleAccountAddrs(maccPerms)

	paramsKeeper := initializer.ParamsKeeper()
	accountKeeper := initializer.AccountKeeper(paramsKeeper, maccPerms)
	bankKeeper := initializer.BankKeeper(paramsKeeper, accountKeeper, blockedMaccAddresses)
	qbankkeeper := initializer.QbankKeeper(paramsKeeper, bankKeeper)

	require.NoError(t, initializer.StateStore.LoadLatestVersion())

	return TestKeepers{
		T:             t,
		Ctx:           initializer.Ctx,
		ParamsKeeper:  paramsKeeper,
		AccountKeeper: accountKeeper,
		BankKeeper:    bankKeeper,
		QBankKeeper:   qbankkeeper,
	}
}

func (tk TestKeepers) QbankKeeper() (sdk.Context, qbankkeeper.Keeper) {
	return tk.Ctx, tk.QBankKeeper
}
