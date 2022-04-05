package keeper

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	authkeeper "github.com/cosmos/cosmos-sdk/x/auth/keeper"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
)

func NameToAddress(name string) string {
	return authtypes.NewModuleAddress(name).String()
}

func NamesToAddresses(names ...string) []string {
	addresses := make([]string, len(names))
	for i, name := range names {
		addresses[i] = NameToAddress(name)
	}

	return addresses
}

func ActiveAddressesMap(addresses ...string) map[string]bool {
	addressesMap := make(map[string]bool, len(addresses))
	for _, address := range addresses {
		addressesMap[address] = true
	}

	return addressesMap
}

func TestAccountKeeper(tks *TestKeeperState, maccPerms map[string][]string) (*authkeeper.AccountKeeper, sdk.Context) {
	storeKey := sdk.NewKVStoreKey(authtypes.StoreKey)
	tks.StateStore.MountStoreWithDB(storeKey, sdk.StoreTypeIAVL, tks.TestDb)

	paramskeeper := tks.GetParamsKeeper()

	subspace := paramskeeper.Subspace(authtypes.ModuleName)
	accountKeeper := authkeeper.NewAccountKeeper(
		tks.EncodingConfig.Marshaler, storeKey, subspace, authtypes.ProtoBaseAccount, maccPerms,
	)

	tks.AccountKeeper = &accountKeeper

	return &accountKeeper, tks.Ctx
}
