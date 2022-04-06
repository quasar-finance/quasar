package keeper

import (
	"testing"

	osmolpvtypes "github.com/abag/quasarnode/x/osmolpv/types"
	"github.com/abag/quasarnode/x/qbank/keeper"
	qbanktypes "github.com/abag/quasarnode/x/qbank/types"
	storetypes "github.com/cosmos/cosmos-sdk/store/types"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
)

const QbankMaccName = qbanktypes.ModuleName

func QbankKeeperExistingState(tks *TestKeeperState) (*keeper.Keeper, sdk.Context) {
	TestParamsKeeper(tks)

	maccPerms := map[string][]string{
		qbanktypes.ModuleName: {authtypes.Minter, authtypes.Burner, authtypes.Staking},
		osmolpvtypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Days_7):   nil,
		osmolpvtypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Days_21):  nil,
		osmolpvtypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Months_1): nil,
		osmolpvtypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Months_3): nil,
	}
	TestAccountKeeper(tks, maccPerms)

	activeModuleAccAddresses := ActiveAddressesMap(NamesToAddresses(qbanktypes.ModuleName, osmolpvtypes.CreateOrionStakingMaccName(qbanktypes.LockupTypes_Days_21))...)
	TestBankKeeper(tks, activeModuleAccAddresses)

	return TestQbankKeeper(tks)
}

func QbankKeeper(t *testing.T) (*keeper.Keeper, sdk.Context) {
	tks := NewTestKeeperState(t)
	defer tks.LoadKVStores()

	return QbankKeeperExistingState(tks)
}

func TestQbankKeeper(tks *TestKeeperState) (*keeper.Keeper, sdk.Context) {
	storeKey := sdk.NewKVStoreKey(qbanktypes.StoreKey)
	memStoreKey := storetypes.NewMemoryStoreKey(qbanktypes.MemStoreKey)
	tks.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, tks.TestDb)
	tks.StateStore.MountStoreWithDB(memStoreKey, sdk.StoreTypeMemory, nil)

	bankKeeper := tks.GetBankKeeper()
	paramsKeeper := tks.GetParamsKeeper()

	qbankSubspace := paramsKeeper.Subspace(qbanktypes.ModuleName)
	qbankKeeper := keeper.NewKeeper(
		tks.EncodingConfig.Marshaler,
		storeKey,
		memStoreKey,
		qbankSubspace,
		bankKeeper,
	)
	// Initialize params
	qbankKeeper.SetParams(tks.Ctx, qbanktypes.DefaultParams())

	return qbankKeeper, tks.Ctx
}
