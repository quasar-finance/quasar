package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/address"
)

const (
	// ModuleName is the name of the module
	ModuleName = "qtransfer"

	// StoreKey is string representation of the store key for qtransfer
	StoreKey = ModuleName

	// QuerierRoute is the querier route for the qtransfer module
	QuerierRoute = ModuleName
)

var (
	// IntermediateAccountAddress is the address of the intermediate account
	IntermediateAccountAddress sdk.AccAddress = address.Module(ModuleName, []byte("wasm-hooks intermediate account"))
)
