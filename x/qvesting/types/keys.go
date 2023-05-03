package types

import (
	sdk "github.com/cosmos/cosmos-sdk/types"
	"github.com/cosmos/cosmos-sdk/types/kv"
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

// AddressFromVestingAccountKey creates the address from VestingAccountKey
func AddressFromVestingAccountKey(key []byte) sdk.AccAddress {
	kv.AssertKeyAtLeastLength(key, 2)
	return key[1:] // remove prefix byte
}
