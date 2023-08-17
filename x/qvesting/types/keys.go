package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
)

const (
	// ModuleName defines the module name
	ModuleName = "qvesting"

	// StoreKey defines the primary module store key
	StoreKey = ModuleName

	// RouterKey is the message route for slashing
	RouterKey = ModuleName

	// QuerierRoute defines the module's query routing key
	QuerierRoute = ModuleName

	// MemStoreKey defines the in-memory store key
	MemStoreKey = "mem_qvesting"
)

var (
	VestingAccountStoreKeyPrefix = []byte{0x01}
)

// VestingAccountStoreKey turn an address to key used to record it in the vesting store
func VestingAccountStoreKey(addr sdk.AccAddress) []byte {
	return append(VestingAccountStoreKeyPrefix, addr.Bytes()...)
}
